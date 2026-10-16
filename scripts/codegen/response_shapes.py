# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Offline dispatch and completeness ledger for independently sourced wire fixtures."""
import json
import subprocess


def binary_dispatch(g):
    corpus = json.loads((g.ROOT/'tests/fixtures/binary-shapes.json').read_text())
    models = sorted({check['model'] for case in corpus['cases'] for check in case['native_checks']})
    code = '''//! Generated native binary response checks, independent wire corpus.
use serde_json::Value;
fn typed<T:serde::de::DeserializeOwned+std::fmt::Debug>(value:Value,expectation:&str)->Result<(),String>{
    let decoded=serde_json::from_value::<T>(value);
    if expectation=="refused_missing_subscription_identity" {
        return if decoded.is_err(){Ok(())}else{Err("null subscription identity accepted".into())};
    }
    let decoded=decoded.map_err(|e|e.to_string())?;
    let unknown=format!("{decoded:?}").contains("Unknown(UnknownMessage");
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
            code += f'        {g.lit(model)}=>typed::<{model}>(value,expectation),\n'
        code += (f'        _=>check_{i+1}(model,value,expectation),\n' if i+1<len(chunks)
                 else '        _=>Err("missing binary native model".into()),\n')
        code += '    }\n}\n'
    path = g.ROOT/'tests/support/binary_shape_dispatch.rs'
    g.write(path, g.HEADER+code)
    subprocess.run(['rustfmt','--edition','2024',str(path)],check=True)


def stream_dispatch(g):
    corpus = json.loads((g.ROOT/'tests/fixtures/stream-shapes.json').read_text())
    models = sorted({case['model'] for case in corpus['cases']})
    code = '''//! Generated native event checks, independent stream corpus.
use serde_json::Value;
fn known<T:std::fmt::Debug>(decoded:T)->Result<(),String>{
    if format!("{decoded:?}").starts_with("Unknown(") {
        return Err("documented event became unknown".into());
    }
    Ok(())
}
fn typed<T:serde::de::DeserializeOwned+std::fmt::Debug>(value:Value)->Result<(),String>{
    known(serde_json::from_value::<T>(value).map_err(|e|e.to_string())?)
}
pub(super) fn check(model:&str,value:Value)->Result<(),String>{check_0(model,value)}
'''
    chunks = [models[i:i+8] for i in range(0,len(models),8)]
    for i, chunk in enumerate(chunks):
        code += f'fn check_{i}(model:&str,value:Value)->Result<(),String>{{\n    match model{{\n'
        for model in chunk:
            expr = ('known(binance_client::margin::event_payloads::user_payload(value).map_err(|e|e.to_string())?)'
                    if model=='binance_client::margin::event_payloads::UserPayload' else f'typed::<{model}>(value)')
            code += f'        {g.lit(model)}=>{expr},\n'
        code += (f'        _=>check_{i+1}(model,value),\n' if i+1<len(chunks)
                 else '        _=>Err("missing native stream model".into()),\n')
        code += '    }\n}\n'
    path = g.ROOT/'tests/support/stream_shape_dispatch.rs'
    g.write(path, g.HEADER+code)
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
    code = '''//! Generated response-fixture dispatch; fixtures come from independent references.
use serde_json::Value;
fn typed<T:serde::de::DeserializeOwned+std::fmt::Debug>(value:Value)->Result<(),String>{
    let decoded:T=serde_json::from_value(value).map_err(|e|e.to_string())?;
    if format!("{decoded:?}").contains("Unknown(UnknownMessage") {
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
                code += f'        {patterns}=>typed::<{model}>(value),\n'
            code += (f'        _=>{product}_{transport}_{i+1}(operation,value),\n' if i+1<len(chunks)
                     else '        _=>Err("response fixture dispatch missing operation".into()),\n')
            code += '    }\n}\n'
    path = g.ROOT/'tests/support/response_shape_dispatch.rs'
    g.write(path, g.HEADER+code)
    subprocess.run(['rustfmt','--edition','2024',str(path)],check=True)
    streams = stream_dispatch(g)
    g.write(g.ROOT/'schema/response-shape-coverage.json',json.dumps({
        'checked':corpus['checked'], 'fixture':'tests/fixtures/response-shapes.json',
        'operations':ledger, 'count':len(ledger), 'cases':len(corpus['cases']),
        'streams':streams, 'binary':{'fixture':'tests/fixtures/binary-shapes.json'}},indent=2)+'\n')
    binary_dispatch(g)
