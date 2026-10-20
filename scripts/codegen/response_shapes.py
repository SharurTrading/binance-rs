# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Offline dispatch and completeness ledger for independently sourced wire fixtures."""
import json
import subprocess
import re


class EvidenceChecks:
    """Generate structural unknown-evidence checks from the emitted Rust types."""
    def __init__(self, g):
        self.defs = {}
        self.functions = {}
        self.rendered = []
        for path in (g.ROOT/'src').rglob('*.rs'):
            scope = 'binance_client::'+'::'.join(path.relative_to(g.ROOT/'src').with_suffix('').parts)
            source = path.read_text()
            for match in re.finditer(r'pub (struct|enum) (\w+)(?:<[^\n]+>)? \{\n(.*?)^\}', source, re.M|re.S):
                kind, name, body = match.groups()
                if kind == 'struct':
                    members = [(name, ' '.join(inner.split())) for name, inner in re.findall(r'^    pub (\w+): (.*?),$', body, re.M|re.S)]
                else:
                    members = [(name, ' '.join(inner.split()).removesuffix(',')) for name, inner in re.findall(r'^    (\w+)\((.*?)\),$', body, re.M|re.S)]
                self.defs[scope+'::'+name] = (kind, members, scope)
            for name, typ in re.findall(r'^pub type (\w+) = (.+);$', source, re.M):
                self.defs[scope+'::'+name] = ('alias', typ, scope)

    @staticmethod
    def absolute(typ, scope):
        typ = typ.replace('crate::', 'binance_client::')
        for product in ['spot','usdm','coinm','options','margin','wallet','convert']:
            typ = typ.replace('binance_client::'+product+'::', 'binance_client::core_trading::'+product+'::')
        while typ.startswith('super::'):
            scope = scope.rsplit('::', 1)[0]
            typ = typ.removeprefix('super::')
        if not typ.startswith('binance_client::'):
            typ = scope+'::'+typ
        return typ

    def expression(self, typ, value, scope=''):
        typ = typ.strip()
        for wrapper in ['Vec','Option','Box']:
            if typ.startswith(wrapper+'<') and typ.endswith('>'):
                inner = self.expression(typ[len(wrapper)+1:-1], 'item', scope)
                if inner == 'false': return 'false'
                predicate = (inner[:-6] if re.fullmatch(r'unknown_\d+\(item\)', inner) and not typ[len(wrapper)+1:].startswith('Box<') else '|item|'+inner)
                receiver = value.removeprefix('&')
                if wrapper == 'Vec': return f'{receiver}.iter().any({predicate})'
                if wrapper == 'Option': return f'{receiver}.as_ref().is_some_and({predicate})'
                return self.expression(typ[4:-1], value, scope)
        if 'BatchResult<' in typ:
            outer, inner = typ.split('<', 1)
            outer = self.absolute(outer, scope)
            check = self.expression(inner[:-1], 'item', scope)
            if check == 'false': return f'matches!({value}, {outer}::Unknown(_))'
            return (f'match {value} {{ {outer}::Unknown(_)=>true, '
                    + (f'{outer}::Success(item)=>{check}, ' if check != 'false' else '')
                    + '_=>false }')
        typ = self.absolute(typ, scope)
        if typ in self.functions:
            name = self.functions[typ]
            return 'false' if name is None else f'{name}({value})'
        definition = self.defs.get(typ)
        if definition is None: return 'false'
        kind, members, local = definition
        if kind == 'alias': return self.expression(members, value, local)
        self.functions[typ] = None
        if kind == 'struct':
            checks = [self.expression(field_type, '&value.'+name, local) for name,field_type in members]
            expression = ' || '.join(check for check in checks if check != 'false') or 'false'
        else:
            arms = []
            for variant, inner in members:
                check = 'true' if variant == 'Unknown' and inner.endswith('UnknownMessage') else self.expression(inner, 'item', local)
                if check != 'false': arms.append(f'{typ}::{variant}('+('_' if check == 'true' else 'item')+f')=>{check},')
            expression = (f'matches!(value, {typ}::Unknown(_))' if len(arms) == 1 and arms[0].endswith('::Unknown(_)=>true,')
                          else 'match value {'+' '.join(arms)+' _=>false }' if arms else 'false')
        if expression == 'false': return 'false'
        name = f'unknown_{len(self.rendered)}'
        self.functions[typ] = name
        self.rendered.append(f'fn {name}(value:&{typ})->bool{{{expression}}}\n')
        return f'{name}({value})'

    def predicate(self, typ):
        expression = self.expression(typ, 'decoded')
        if expression == 'false': return '|_|false'
        if re.fullmatch(r'unknown_\d+\(decoded\)', expression): return expression[:-9]
        return '|decoded|'+expression

    def code(self):
        return '\n'.join(self.rendered)


def binary_dispatch(g):
    corpus = json.loads((g.ROOT/'tests/fixtures/binary-shapes.json').read_text())
    models = sorted({check['model'] for case in corpus['cases'] for check in case['native_checks']})
    evidence = EvidenceChecks(g)
    code = '''//! Generated native binary response checks, independent wire corpus.
use serde_json::Value;
fn typed<T:serde::de::DeserializeOwned>(value:Value,expectation:&str,unknown:impl FnOnce(&T)->bool)->Result<(),String>{
    let decoded=serde_json::from_value::<T>(value);
    if expectation=="refused_missing_subscription_identity" {
        return if decoded.is_err(){Ok(())}else{Err("null subscription identity accepted".into())};
    }
    let decoded=decoded.map_err(|e|e.to_string())?;
    let unknown=unknown(&decoded);
    if unknown != (expectation=="decoded_unknown_evidence") {
        return Err("native known/unknown evidence disposition mismatch".into());
    }
    Ok(())
}
pub(super) fn check(model:&str,value:Value,expectation:&str)->Result<(),String>{
    check_0(model,value,expectation)
}
'''
    chunks = [models[i:i+8] for i in range(0,len(models),8)]
    for i, chunk in enumerate(chunks):
        code += f'fn check_{i}(model:&str,value:Value,expectation:&str)->Result<(),String>{{\n    match model{{\n'
        for model in chunk:
            code += f'        {g.lit(model)}=>typed::<{model}>(value,expectation,{evidence.predicate(model)}),\n'
        code += (f'        _=>check_{i+1}(model,value,expectation),\n' if i+1<len(chunks)
                 else '        _=>Err("missing binary native model".into()),\n')
        code += '    }\n}\n'
    path = g.ROOT/'tests/support/binary_shape_dispatch.rs'
    g.write(path, g.HEADER+code+'\n'+evidence.code())
    subprocess.run(['rustfmt','--edition','2024',str(path)],check=True)


def stream_dispatch(g):
    corpus = json.loads((g.ROOT/'tests/fixtures/stream-shapes.json').read_text())
    models = sorted({case['model'] for case in corpus['cases']})
    evidence = EvidenceChecks(g)
    code = '''//! Generated native event checks, independent stream corpus.
use serde_json::Value;
fn known<T>(decoded:&T,unknown:impl FnOnce(&T)->bool)->Result<(),String>{
    if unknown(decoded) { return Err("documented event became unknown".into()); }
    Ok(())
}
fn typed<T:serde::de::DeserializeOwned>(value:Value,unknown:impl FnOnce(&T)->bool)->Result<(),String>{
    known(&serde_json::from_value::<T>(value).map_err(|e|e.to_string())?,unknown)
}
pub(super) fn check(model:&str,value:Value)->Result<(),String>{check_0(model,value)}
'''
    chunks = [models[i:i+8] for i in range(0,len(models),8)]
    for i, chunk in enumerate(chunks):
        code += f'fn check_{i}(model:&str,value:Value)->Result<(),String>{{\n    match model{{\n'
        for model in chunk:
            expr = ('known(&binance_client::margin::event_payloads::user_payload(value).map_err(|e|e.to_string())?, '+evidence.predicate(model)+')'
                    if model=='binance_client::margin::event_payloads::UserPayload' else f'typed::<{model}>(value,{evidence.predicate(model)})')
            code += f'        {g.lit(model)}=>{expr},\n'
        code += (f'        _=>check_{i+1}(model,value),\n' if i+1<len(chunks)
                 else '        _=>Err("missing native stream model".into()),\n')
        code += '    }\n}\n'
    path = g.ROOT/'tests/support/stream_shape_dispatch.rs'
    g.write(path, g.HEADER+code+'\n'+evidence.code())
    subprocess.run(['rustfmt','--edition','2024',str(path)],check=True)
    return {'fixture':'tests/fixtures/stream-shapes.json',
            'operations':[{**op,'cases':[c['id'] for c in corpus['cases']
                           if (c['product'],c['operation'])==(op['product'],op['operation'])]}
                          for op in corpus['operations']], 'cases':len(corpus['cases'])}


def generate(g):
    fixture = g.ROOT/'tests/fixtures/response-shapes.json'
    corpus = json.loads(fixture.read_text())
    operations = corpus['operations']
    seen = set()
    for op in operations:
        key = (op['product'], op['transport'], op['operation'])
        if key in seen:
            raise ValueError(('duplicate response operation', key))
        seen.add(key)
    cases = {}
    for case in corpus['cases']:
        key = (case['product'], case['transport'], case['operation'])
        if key not in seen:
            raise ValueError(('response fixture without operation', case['id']))
        cases.setdefault(key, []).append(case['id'])
    if seen != set(cases):
        raise ValueError(('operation without response fixtures', sorted(seen-set(cases))))
    groups = {}
    ledger = []
    for op in operations:
        key = (op['product'], op['transport'], op['operation'])
        groups.setdefault(key[:2], []).append(op)
        ledger.append({**op, 'cases': cases[key]})
    evidence = EvidenceChecks(g)
    code = '''//! Generated response-fixture dispatch; fixtures come from independent references.
use serde_json::Value;
fn typed<T:serde::de::DeserializeOwned>(value:Value,unknown:impl FnOnce(&T)->bool)->Result<(),String>{
    let decoded:T=serde_json::from_value(value).map_err(|e|e.to_string())?;
    if unknown(&decoded) {
        return Err("documented member decoded as unknown evidence".into());
    }
    Ok(())
}
pub(super) fn check(product:&str,transport:&str,operation:&str,value:Value)->Result<(),String>{
    match (product,transport){
'''
    for product, transport in groups:
        code += f'        ({g.lit(product)},{g.lit(transport)})=>{product}_{transport}_0(operation,value),\n'
    code += '        _=>Err("response fixture dispatch missing product/transport".into()),\n    }\n}\n'
    for (product, transport), rows in groups.items():
        chunks = [rows[i:i+8] for i in range(0,len(rows),8)]
        for i, chunk in enumerate(chunks):
            code += f'fn {product}_{transport}_{i}(operation:&str,value:Value)->Result<(),String>{{\n    match operation{{\n'
            arms = {}
            for op in chunk:
                arms.setdefault(op['model'], []).append(op['operation'])
            for model, names in arms.items():
                patterns = '|'.join(g.lit(name) for name in names)
                code += f'        {patterns}=>typed::<{model}>(value,{evidence.predicate(model)}),\n'
            code += (f'        _=>{product}_{transport}_{i+1}(operation,value),\n' if i+1<len(chunks)
                     else '        _=>Err("response fixture dispatch missing operation".into()),\n')
            code += '    }\n}\n'
    path = g.ROOT/'tests/support/response_shape_dispatch.rs'
    g.write(path, g.HEADER+code+'\n'+evidence.code())
    subprocess.run(['rustfmt','--edition','2024',str(path)],check=True)
    streams = stream_dispatch(g)
    g.write(g.ROOT/'schema/response-shape-coverage.json',json.dumps({
        'checked':corpus['checked'], 'fixture':'tests/fixtures/response-shapes.json',
        'operations':ledger, 'count':len(ledger), 'cases':len(corpus['cases']),
        'streams':streams, 'binary':{'fixture':'tests/fixtures/binary-shapes.json'}},indent=2)+'\n')
    binary_dispatch(g)
