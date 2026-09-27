# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Deterministically generate typed API bindings from pinned protocol facts.

No downloads or package dependencies. Financial types are Decimal; responses
preserve unknown fields, request builders validate required fields before sending.
"""
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PRODUCT = 'usdm'

def write(path, text):
    path.write_text(text, encoding='utf-8', newline='\n')

HEADER = '// SPDX-FileCopyrightText: 2026 Kevin Monaghan\n// SPDX-License-Identifier: MIT-0\n\n'
RESERVED = {'type', 'match', 'ref', 'self', 'mod', 'loop', 'in', 'return', 'fn', 'use', 'enum'}


def snake(value):
    if value == 'newOrderRespType':
        return 'response_type'
    if value.isupper():
        return 'upper_' + value.lower()
    value = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', value)
    value = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', value)
    value = re.sub(r'[^A-Za-z0-9]+', '_', value).strip('_').lower()
    if value in RESERVED:
        value += '_value'
    if not value or value[0].isdigit():
        value = 'value_' + value
    return value


def pascal(value):
    return ''.join(x[:1].upper()+x[1:] for x in snake(value).split('_'))


def lit(value):
    return json.dumps(value, ensure_ascii=False)


class Models:
    def __init__(self, components):
        self.components = components
        self.defs = {}

    def resolve(self, schema):
        if '$ref' in schema:
            return self.resolve(self.components[schema['$ref'].split('/')[-1]])
        return schema

    def type(self, schema, name, response=True):
        schema = self.resolve(schema)
        if schema.get('x-batch-member'):
            base = dict(schema)
            base.pop('x-batch-member')
            return 'super::wire::BatchResult<'+self.type(base, name, response)+'>'
        if schema.get('x-rust-type'):
            return schema['x-rust-type']
        if schema.get('x-decimal'):
            return 'Decimal'
        if 'oneOf' in schema or 'anyOf' in schema:
            variants = schema.get('oneOf', schema.get('anyOf'))
            if len(variants) == 1:
                return self.type(variants[0], name, response)
            if name not in self.defs:
                self.defs[name] = ''
                types = [self.type(x, name+'Variant'+str(i+1), response) for i, x in enumerate(variants)]
                tags = [self.resolve(x).get('properties', {}).get('filterType', {}).get('enum', []) for x in variants]
                if tags and all(len(t) == 1 for t in tags):
                    for x in variants:
                        resolved = self.resolve(x)
                        resolved['required'] = [k for k in resolved.get('properties', {}) if not k.endswith('Exponent')]
                    # Regenerate variants with their mandatory filter evidence.
                    for t in types:
                        self.defs.pop(t, None)
                    types = [self.type(x, name+'Variant'+str(i+1), response) for i, x in enumerate(variants)]
                    variant_names = [''.join(piece.title() for piece in str(tag[0]).split('_')) for tag in tags]
                    self.defs[name] = '\n'.join([
                        f'/// Provider filters with explicit discriminator dispatch and unknown retention.',
                        '#[derive(Clone, Debug, PartialEq, Serialize)]', '#[non_exhaustive]', '#[serde(untagged)]', f'pub enum {name} {{',
                        *[f'    /// Provider `{tags[i][0]}` filter.\n    {variant_names[i]}(Box<{t}>),' for i,t in enumerate(types)],
                        '    /// Future filter facts, retained with redacted Debug.\n    Unknown(super::event_payloads::UnknownMessage),', '}',
                        f"impl<'de> Deserialize<'de> for {name} {{",
                        "    fn deserialize<D: serde::Deserializer<'de>>(d:D)->Result<Self,D::Error> {",
                        '        let value=serde_json::Value::deserialize(d)?;',
                        '        match value.get("filterType").and_then(serde_json::Value::as_str) {',
                        *[f'            Some({lit(tags[i][0])}) => serde_json::from_value(value).map(|v|Self::{variant_names[i]}(Box::new(v))).map_err(serde::de::Error::custom),' for i,t in enumerate(types)],
                        '            Some(_) => Ok(Self::Unknown(value.into())),',
                        '            None => Err(serde::de::Error::custom("filter type required")),',
                        '        }', '    }', '}'])
                    return name
                self.defs[name] = '\n'.join([
                    f'/// Provider alternatives for `{name}`; no member is discarded.',
                    '#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]',
                    '#[non_exhaustive]', '#[serde(untagged)]', f'pub enum {name} {{',
                    *[f'    /// Wire alternative {i+1}.\n    Variant{i+1}(Box<{t}>),' for i,t in enumerate(types)], '}'])
            return name
        typ = schema.get('type', 'object')
        if typ == 'string':
            if name.lower().endswith(('listenkey','downloadurl','msg')):
                return 'SensitiveString'
            if name.lower().endswith('symbol'):
                return 'Symbol'
            if name.lower().endswith('clientorderid') or name.lower().endswith('clientalgoid'):
                return 'ClientOrderId'
            return 'String'
        if typ == 'integer':
            return 'i64'
        if typ == 'boolean':
            return 'bool'
        if typ == 'number':
            return 'Decimal'
        if typ == 'array':
            prefix = schema.get('x-prefixItems', [])
            if prefix:
                prefix_types = [self.type(x, name+'Value'+str(i), response) for i,x in enumerate(prefix)]
                if prefix_types == ['Decimal','Decimal']:
                    return 'PriceLevel'
                if len(prefix) == 12:
                    return 'Kline'
                return 'Vec<serde_json::Value>'
            return 'Vec<'+self.type(schema.get('items', {}), name+'Item', response)+'>'
        props = schema.get('properties', {})
        if response and 'code' in props and 'orderId' in props:
            success = dict(schema)
            success['properties'] = {k:v for k,v in props.items() if k not in ['code','msg']}
            success['required'] = ['orderId']
            return 'super::wire::BatchResult<'+self.type(success,name+'Success',response)+'>'
        if not response and props:
            if name not in self.defs:
                self.defs[name] = ''
                fields=[]; setters=[]
                for key,field_schema in props.items():
                    typ=request_type(self,field_schema,name+pascal(key),key)
                    field=snake(key)
                    fields += [f'    #[serde(rename = {lit(key)}, skip_serializing_if = "Option::is_none")]',f'    {field}: Option<{typ}>,']
                    arg='impl Into<String>' if typ=='String' else typ
                    value='value.into()' if typ=='String' else 'value'
                    setters += [f'    /// Set `{key}`.', '    #[must_use]',f'    pub fn {field}(mut self,value:{arg})->Self {{ self.{field}=Some({value});self }}']
                op='newOrder' if name.startswith('PlaceMultipleOrders') else 'modifyOrder' if name.startswith('ModifyMultipleOrders') else 'nestedInput'
                self.defs[name]='\n'.join([f'/// Validated nested request builder for `{name}`.', '#[derive(Clone, Debug, Default, Serialize)]', f'pub struct {name} {{',*fields,'}', f'impl {name} {{', '    /// Start this nested request builder.', '    #[must_use]', '    pub fn new()->Self {Self::default()}',*setters,'    /// Validate required and conditional provider parameters.', '    ///', '    /// # Errors', '    /// Refuses missing or contradictory input.',f'    pub fn build(self)->Result<Self,crate::Error> {{ super::validation::validate({lit(op)}, &crate::core::parameters(&self)?)?; Ok(self) }}','}'])
            return name
        if not props:
            additional = schema.get('additionalProperties')
            if isinstance(additional, dict):
                return 'BTreeMap<String, '+self.type(additional, name+'Value', response)+'>'
            return 'BTreeMap<String, serde_json::Value>'
        if name not in self.defs:
            self.defs[name] = ''
            fields = []
            required = set(schema.get('required', []))
            # A discriminator is required for alternatives, so an error object cannot
            # accidentally deserialize as an all-optional successful order.
            if 'code' in props:
                required.add('code')
            if 'orderId' in props and name!='TestOrderResponse':
                required.add('orderId')
            if PRODUCT != 'usdm' and 'orderListId' in props and 'orders' in props:
                required.update(['orderListId', 'orders'])
            if 'algoId' in props:
                required.add('algoId')
            if 'e' in props:
                required.add('e')
            for key,s in props.items():
                t = self.type(s, name+pascal(key), response)
                optional = key not in required
                if not optional and t == 'Decimal':
                    serde_attr = f'#[serde(rename = {lit(key)}, deserialize_with = "super::wire::decimal")]'
                elif optional and t == 'Decimal':
                    serde_attr = f'#[serde(rename = {lit(key)}, default, deserialize_with = "super::wire::decimal_option", skip_serializing_if = "Option::is_none")]'
                elif optional:
                    serde_attr = f'#[serde(rename = {lit(key)}, default, skip_serializing_if = "Option::is_none")]'
                else:
                    serde_attr = f'#[serde(rename = {lit(key)})]'
                fields += [f'    /// Exact `{key}` wire field.', '    '+serde_attr,
                    f'    pub {snake(key)}: '+(f'Option<{t}>' if optional else t)+',']
            extra = 'BTreeMap<String, serde_json::Value>' if PRODUCT == 'usdm' else 'super::event_payloads::UnknownMessage'
            fields += ['    /// Unknown future wire fields, retained without inventing defaults; avoid logging.',
                '    #[serde(flatten)]', f'    pub extra: {extra},'] if PRODUCT != 'usdm' else ['    /// Unknown future wire fields, retained without inventing defaults.',
                '    #[serde(flatten)]', '    pub extra: BTreeMap<String, serde_json::Value>,']
            self.defs[name] = '\n'.join([f'/// Provider-native `{name}` payload.',
                '#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]', '#[non_exhaustive]',
                f'pub struct {name} {{', *fields, '}'])
        return name


def request_fields(op, kind):
    if kind == 'ws':
        content = next(iter(op.get('requestBody', {}).get('content', {}).values()), {})
        s = content.get('schema', {}).get('properties', {}).get('params', {})
        props = s.get('properties', {}).copy()
        required = set(s.get('required', []))
    else:
        props = {p['name']:p.get('schema', {}) for p in op.get('parameters', []) if p['in']=='query'}
        required = {p['name'] for p in op.get('parameters', []) if p['in']=='query' and p.get('required')}
        content = next(iter(op.get('requestBody', {}).get('content', {}).values()), {})
        s = content.get('schema', {})
        props.update(s.get('properties', {}))
        required.update(s.get('required', []))
    for key in ['apiKey', 'signature', 'timestamp']:
        props.pop(key, None)
        required.discard(key)
    if op['operationId'] in ['newOrder', 'testOrder', 'orderPlace', 'orderTest']:
        required.add('newClientOrderId')
    if op['operationId'] == 'newAlgoOrder':
        required.add('clientAlgoId')
    return props, required


def request_type(models, schema, name, field):
    if schema.get('enum'):
        # Open response enums are strings; outgoing enum values are validated.
        return 'String'
    if field in ['symbol', 'pair']:
        return 'Symbol'
    if field.lower().endswith('clientorderid') or field in ['clientAlgoId','origClientAlgoId']:
        return 'ClientOrderId'
    if field == 'listenKey':
        return 'SensitiveString'
    return models.type(schema, name, response=False)


def generate(kind):
    snap = json.loads((ROOT/'schema'/f'{PRODUCT}-{kind}.json').read_text())
    models = Models(snap.get('components', {}).get('schemas', {}))
    requests = []
    methods = []
    coverage = []
    for op in snap['operations']:
        name = pascal(op['operationId'])
        response = next(iter(op.get('responses', {}).values()), {})
        content = next(iter(response.get('content', {}).values()), {})
        rs = content.get('schema', {})
        if kind == 'ws':
            if 'oneOf' in rs or 'anyOf' in rs:
                alternatives = rs.get('oneOf', rs.get('anyOf'))
                rs = {'oneOf': [models.resolve(v)['properties']['result'] for v in alternatives]}
            else:
                rs = rs.get('properties', {}).get('result', {})
        response_type = models.type(rs, name+'Response')
        if response_type != name+'Response':
            models.defs[name+'ResponseAlias'] = f'/// Exact response for `{op["operationId"]}`.\npub type {name}Response = {response_type};'
            response_type = name+'Response'
        props,required = request_fields(op,kind)
        if not required <= set(props):
            raise ValueError((op['operationId'],required-set(props)))
        fields=[]
        setters=[]
        enums=[]
        bounds=[]
        for key,s in props.items():
            typ = request_type(models,s,name+pascal(key)+'Input',key)
            field = snake(key)
            fields += [f'    #[serde(rename = {lit(key)}, skip_serializing_if = "Option::is_none")]',f'    {field}: Option<{typ}>,']
            arg = 'impl Into<String>' if typ=='String' else typ
            val = 'value.into()' if typ=='String' else 'value'
            setters += [f'    /// Set the provider `{key}` parameter.',
                '    #[must_use]',f'    pub fn {field}(mut self, value: {arg}) -> Self {{ self.{field} = Some({val}); self }}']
            if s.get('enum'):
                enums.append('('+lit(key)+', &['+', '.join(lit(str(v)) for v in s['enum'])+'])')
            if typ=='i64' and ('minimum' in s or 'maximum' in s):
                bounds.append('('+lit(key)+', '+format(s.get('minimum',-9223372036854775808),'_')+', '+format(s.get('maximum',9223372036854775807),'_')+')')
        security = 'Signed' if op.get('x-signed') else 'Key' if op.get('x-security-type') in ['MARKET_DATA','USER_STREAM'] else 'Public'
        mutation = op['method']!='GET' if kind=='rest' else op['path'] in ['/order.place','/order.modify','/order.cancel','/algoOrder.place','/algoOrder.cancel','/userDataStream.start','/userDataStream.stop','/userDataStream.ping']
        if PRODUCT == 'spot' and kind == 'ws':
            mutation = op['tags'][0] == 'trade' or op['operationId'] in ['userDataStreamSubscribe','userDataStreamSubscribeSignature','userDataStreamUnsubscribe']
        if op['operationId'] in ['testOrder','orderTest','sorOrderTest']:mutation=False
        # Generating download jobs has side effects despite the HTTP GET method.
        if op['operationId'].startswith('getDownloadId'):
            mutation=True
        success_weight = 'Some(0)' if PRODUCT == 'spot' and op['operationId'] in ['newOrder','deleteOrder','deleteOpenOrders','orderPlace','orderCancel','openOrdersCancelAll'] else 'None'
        partial = 'Some(super::validation::partial)' if op.get('x-partial-result') else 'None'
        op_expr = f'Operation {{ name: {lit(op["operationId"])}, path: {lit(op["path"])}, method: {lit(op["method"])}, security: Security::{security}, mutation: {str(mutation).lower()}, weight: {op.get("x-ip-weight",0)}, validate_time: super::validation::validate_time, definitive: super::validation::definitive, success_weight: {success_weight}, partial: {partial} }}'
        required_rust='&['+', '.join(lit(v) for v in sorted(required))+']'
        validation=f'let p = parameters(self)?; validate_parameters(&p, {required_rust}, &[{", ".join(enums)}], &[{", ".join(bounds)}])?; super::validation::validate({lit(op["operationId"])}, &p)'
        requests.append('\n'.join([f'/// Validated request builder for [`{op["operationId"]}`]({op["source"]}).',
            '#[derive(Clone, Debug, Default, Serialize)]',f'pub struct {name} {{',*fields,'}',
            f'impl {name} {{','    /// Start a request builder. Required inputs are checked by `build` and by dispatch.',
            '    #[must_use]', '    pub fn new() -> Self { Self::default() }',*setters,
            '    /// Validate this request before dispatch.', '    ///', '    /// # Errors',
            '    /// Refuses missing, invalid, or contradictory provider parameters.',
            '    pub fn build(self) -> Result<Self, Error> { self.validate()?; Ok(self) }','}',
            f'impl Request for {name} {{', f'    type Response = super::{kind}_models::{response_type};',
            f'    const OP: Operation = {op_expr};',f'    fn validate(&self) -> Result<(), Error> {{ {validation} }}',
            '    fn cost(&self) -> Result<crate::core::Cost, Error> { super::rate::cost(Self::OP, &parameters(self)?) }','}']))
        method=snake(op['operationId'])
        if kind=='rest':
            args=f'&self, request: &{name}, deadline: tokio::time::Instant'
            call='self.inner.execute(request, deadline).await'
        else:
            args=f'&self, request: &{name}, id: crate::RequestId, deadline: tokio::time::Instant'
            call='self.execute(request, id, deadline).await'
        methods.append('\n'.join([f'    /// [{op["operationId"]}]({op["source"]}).',
            '    ///', '    /// # Errors', '    /// Returns input/admission errors before sending, or typed venue/transport evidence.',
            f'    pub async fn {method}({args}) -> Result<crate::Response<super::{kind}_models::{response_type}>, Error> {{ {call} }}']))
        coverage.append({'name':op['operationId'],'method':op['method'],'path':op['path'],'source':op['source']})
    common='use std::collections::BTreeMap;\nuse serde::{Serialize, Deserialize};\nuse crate::{Decimal, Symbol, ClientOrderId, SensitiveString};\nuse super::wire::{PriceLevel, Kline};\n'
    # Output only necessary imports to keep strict lint gates unchanged.
    model_text='\n\n'.join(models.defs.values())
    imports=[]
    for line,token in [('use std::collections::BTreeMap;','BTreeMap'),('use serde::{Serialize, Deserialize};','derive'),('use crate::Decimal;','Decimal'),('use crate::Symbol;','Symbol'),('use crate::ClientOrderId;','ClientOrderId'),('use crate::SensitiveString;','SensitiveString'),('use super::wire::PriceLevel;','PriceLevel'),('use super::wire::Kline;','Kline')]:
        if re.search(r'(?<!::)\b'+re.escape(token)+r'\b',model_text):imports.append('use super::ClientOrderId;' if PRODUCT == 'spot' and token == 'ClientOrderId' else line)
    write((ROOT/'src'/PRODUCT/f'{kind}_models.rs'), HEADER+f'//! Generated {kind} response DTOs; regenerate with scripts/codegen/generate.py.\n\n'+'\n'.join(imports)+'\n\n'+model_text+'\n')
    request_text='\n\n'.join(requests)
    imports=['use serde::Serialize;','use crate::Error;','use crate::core::{Request, Operation, Security, parameters, validate_parameters};']
    for token in ['Symbol','ClientOrderId','SensitiveString','Decimal']:
        if token in request_text:imports.append(f'use super::{token};' if PRODUCT == 'spot' and token == 'ClientOrderId' else f'use crate::{token};')
    # Nested input objects are emitted into the models module and imported here.
    nested=[n for n in models.defs if 'Input' in n and re.search(r'\b'+n+r'\b',request_text)]
    if nested: imports.append(f'use super::{kind}_models::{{'+', '.join(nested)+'};')
    client='RestClient' if kind=='rest' else 'WsClient'
    write((ROOT/'src'/PRODUCT/f'{kind}_requests.rs'), HEADER+f'//! Generated {kind} request builders.\n\n'+'\n'.join(imports)+'\n\n'+request_text+'\n\n'+f'impl super::{client} {{\n'+'\n\n'.join(methods)+'\n}\n')
    return coverage


def generate_streams():
    snap=json.loads((ROOT/'schema'/f'{PRODUCT}-streams.json').read_text())
    models=Models(snap['components']['schemas'])
    names=[]
    methods=[]
    for op in snap['operations']:
        name=pascal(op['operationId'])
        response=op['responses'].get('Raw Stream',next(iter(op['responses'].values())))
        schema=next(iter(response['content'].values()))['schema']
        typ=models.type(schema,name+'Event')
        names.append((name,op['path'],op['tags'][0],typ))
        params=[p for p in op.get('parameters',[]) if '{'+p['name']+'}' in op['path']]
        args=[]; replacements=[]
        for param in params:
            key=param['name'];field=snake(key)
            typ='&Symbol' if key in ['symbol','pair'] else '&str'
            args.append(f'{field}: {typ}')
            value=f'{field}.as_str()' if typ=='&Symbol' else field
            if key in ['symbol','pair']: value+=' .to_lowercase().as_str()'
            replacements.append(f'name=name.replace({lit("{"+key+"}")}, {value});')
            choices=param.get('schema',{}).get('enum')
            if choices:
                replacements.insert(0,f'if ![{", ".join(lit(str(x)) for x in choices)}].contains(&{field}) {{ return Err(Error::Validation("stream parameter")); }}')
        methods.append('\n'.join([f'    /// [{op["operationId"]}]({op["source"]}).',
            '    ///', '    /// # Errors', '    /// Refuses an invalid documented stream parameter.',
            f'    pub fn {snake(op["operationId"])}({", ".join(args)}) -> Result<Self,Error> {{',
            f'        let mut name={lit(op["path"].lstrip("/"))}.to_owned();' if params else f'        let name={lit(op["path"].lstrip("/"))}.to_owned();',
            *['        '+r for r in replacements],
            f'        Self::new(name, Route::{pascal(op["tags"][0])}, {lit(op["operationId"])})','    }']))
    for name,s in snap['components']['schemas'].items():
        if name!='User Data Stream Events' and 'e' in s.get('properties',{}):models.type(s,pascal(name)+'Event')
    text='\n\n'.join(models.defs.values())
    imports=['use serde::{Serialize, Deserialize};']
    for line,token in [('use std::collections::BTreeMap;','BTreeMap'),('use crate::Decimal;','Decimal'),('use crate::Symbol;','Symbol'),('use crate::ClientOrderId;','ClientOrderId'),('use crate::SensitiveString;','SensitiveString'),('use super::wire::PriceLevel;','PriceLevel'),('use super::wire::Kline;','Kline')]:
        if re.search(r'(?<!::)\b'+re.escape(token)+r'\b',text):imports.append('use super::ClientOrderId;' if PRODUCT == 'spot' and token == 'ClientOrderId' else line)
    write((ROOT/'src'/PRODUCT/'stream_models.rs'), HEADER+'//! Generated market and user-data event payloads.\n\n'+'\n'.join(imports)+'\n\n'+text+'\n')
    write((ROOT/'src'/PRODUCT/'stream_names.rs'), HEADER+'//! Generated constructors for every documented market stream.\n\nuse crate::{Error, Symbol};\nuse super::streams::{Stream, Route};\n\nimpl Stream {\n'+'\n\n'.join(methods)+'\n}\n')
    return [{'name':n,'path':p,'route':r,'type':t} for n,p,r,t in names]


def generate_events(coverage):
    ws=coverage['ws']
    text='''//! Typed payload dispatch for API and market/user-data events.
    
    use crate::{Error,SensitiveString};
    use serde_json::Value;
    
    /// Unknown future payloads are retained, with redacted Debug output.
    #[derive(Clone,PartialEq,Eq,serde::Serialize,serde::Deserialize)]
    #[serde(transparent)]
    pub struct UnknownMessage(Value);
    impl UnknownMessage {
        /// Explicit provider payload access; do not log sensitive account data.
        #[must_use]
        pub fn as_value(&self)->&Value {&self.0}
    }
    impl std::fmt::Debug for UnknownMessage {
        fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result {f.write_str("UnknownMessage([REDACTED])")}
    }
    impl From<Value> for UnknownMessage {fn from(v:Value)->Self {Self(v)}}
    
    /// A late WebSocket API response, decoded using its original operation.
    #[derive(Clone,Debug,PartialEq)]
    #[non_exhaustive]
    pub enum ApiPayload {
    '''
    for o in ws:
     n=pascal(o['name']);text+=f'    /// `{o["name"]}` response.\n    {n}(Box<super::ws_models::{n}Response>),\n'
    text+='''    /// Session authentication/status response.
        Session(SessionStatus),
    }
    
    /// Provider session evidence, never an assertion about order truth.
    #[derive(Clone,Debug,PartialEq,serde::Deserialize)]
    #[non_exhaustive]
    pub struct SessionStatus {
        /// Connection timestamp.
        #[serde(rename="connectedSince")]
        pub connected_since:Option<i64>,
        /// Authentication timestamp.
        #[serde(rename="authorizedSince")]
        pub authorized_since:Option<i64>,
        /// Current server timestamp.
        #[serde(rename="serverTime")]
        pub server_time:Option<i64>,
        /// Authenticated API key; redacted by default.
        #[serde(rename="apiKey")]
        pub api_key:Option<SensitiveString>,
        /// Unknown future session fields.
        #[serde(flatten)]
        pub extra:UnknownMessage,
    }
    
    pub(crate) fn api_payload(operation:&str,value:Value)->Result<ApiPayload,Error> {
        match operation {
    '''
    chunks=[ws[i:i+20] for i in range(0,len(ws),20)]
    for i,chunk in enumerate(chunks):
        text+='        '+'|'.join(lit(o['name']) for o in chunk)+f'=>return api_payload_{i}(operation,value),\n'
    text+='''        "sessionLogon"|"sessionStatus"|"sessionLogout"=>serde_json::from_value(value).map(ApiPayload::Session),
            _=>return Err(Error::Gap("unrecognized correlated API operation")),
        }.map_err(|_|Error::Gap("malformed correlated API response"))
    }
    
    '''
    for i,chunk in enumerate(chunks):
        text+=f'fn api_payload_{i}(operation:&str,value:Value)->Result<ApiPayload,Error> {{\n    match operation {{\n'
        for o in chunk:
            n=pascal(o['name']);text+=f'        "{o["name"]}"=>serde_json::from_value(value).map(|v|ApiPayload::{n}(Box::new(v))),\n'
        text+='''        _=>return Err(Error::Gap("unrecognized correlated API operation")),
        }.map_err(|_|Error::Gap("malformed correlated API response"))
    }
    '''
    text+='''    /// Every documented market-stream payload, with provider distinctions intact.
    #[derive(Clone,Debug,PartialEq)]
    #[non_exhaustive]
    pub enum MarketPayload {
    '''
    for o in coverage['streams']:
     t=o['type'].replace('Vec<','Vec<super::stream_models::') if o['type'].startswith('Vec<') else 'super::stream_models::'+o['type']
     text+=f'    /// `{o["name"]}` events.\n    {o["name"]}(Box<{t}>),\n'
    text+='''}
    
    pub(crate) fn market_payload(kind:&str,value:Value)->Result<MarketPayload,Error> {
        match kind {
    '''
    for o in coverage['streams']:
     text+=f'        "{o["name"][0].lower()+o["name"][1:]}"=>serde_json::from_value(value).map(|v|MarketPayload::{o["name"]}(Box::new(v))),\n'
    text+='''        _=>return Err(Error::Gap("unrecognized subscribed stream")),
        }.map_err(|_|Error::Gap("malformed market payload"))
    }
    
    /// Every documented user-data event, plus an explicit unknown-future alternative.
    #[derive(Clone,Debug,PartialEq)]
    #[non_exhaustive]
    pub enum UserPayload {
    '''
    ss=json.loads((ROOT/'schema'/f'{PRODUCT}-streams.json').read_text())['components']['schemas']
    for n,s in ss.items():
     if n=='User Data Stream Events' or 'e' not in s.get('properties',{}):continue
     name=pascal(n);text+=f'    /// Provider `{n}` event.\n    {name}(Box<super::stream_models::{name}Event>),\n'
    text+='''    /// A future event type, preserved for explicit consumer handling.
        Unknown(UnknownMessage),
    }
    
    pub(crate) fn user_payload(value:Value)->Result<UserPayload,Error> {
        let event=value.get("e").and_then(Value::as_str).ok_or(Error::Gap("user event type"))?;
        if event=="ACCOUNT_UPDATE" && value.get("a").is_none_or(|a|a.get("B").is_none() && a.get("P").is_none()) {return Err(Error::Gap("account event has no balance/position evidence"));}
        if event=="ACCOUNT_CONFIG_UPDATE" && value.get("ac").is_none() && value.get("ai").is_none() {return Err(Error::Gap("account configuration event has no evidence"));}
        match event {
    '''
    for n,s in ss.items():
     if n=='User Data Stream Events' or 'e' not in s.get('properties',{}):continue
     name=pascal(n);event=s['properties']['e']['enum'][0]
     text+=f'        "{event}"=>serde_json::from_value(value).map(|v|UserPayload::{name}(Box::new(v))),\n'
    text+='''        _=>return Ok(UserPayload::Unknown(value.into())),
        }.map_err(|_|Error::Gap("malformed execution/account event"))
    }
    '''
    write((ROOT/'src'/PRODUCT/'event_payloads.rs'), HEADER+text)

def main():
    global PRODUCT
    check='--check' in sys.argv
    files=['rest_models.rs','rest_requests.rs','ws_models.rs','ws_requests.rs','stream_models.rs','stream_names.rs','event_payloads.rs']
    paths=[*(ROOT/'src'/p/f for p in ['usdm','spot','coinm'] for f in files), ROOT/'schema/coverage.json', ROOT/'schema/spot-coverage.json', ROOT/'schema/coinm-coverage.json']
    before={p:p.read_bytes() if p.exists() else None for p in paths}
    for PRODUCT in ['usdm','spot','coinm']:
        coverage={kind:generate(kind) for kind in ['rest','ws']}
        coverage['streams']=generate_streams()
        filename='coverage.json' if PRODUCT == 'usdm' else f'{PRODUCT}-coverage.json'
        write((ROOT/'schema'/filename), json.dumps(coverage,indent=2)+'\n')
        generate_events(coverage)
        subprocess.run(['rustfmt','--edition','2024',*[str(ROOT/'src'/PRODUCT/f) for f in files]],check=True)
        print(PRODUCT+': '+', '.join(f'{len(v)} {k}' for k,v in coverage.items())+'.')
    if check:
        changed=[p for p,v in before.items() if p.read_bytes()!=v]
        for p in changed:
            if before[p] is None:p.unlink()
            else:p.write_bytes(before[p])
        if changed:raise SystemExit('Stale generated bindings: '+', '.join(str(p.relative_to(ROOT)) for p in changed))


if __name__=='__main__':
    main()
