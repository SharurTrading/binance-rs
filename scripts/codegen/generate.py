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
CORE_TRADING = ROOT/'src'/'core_trading'
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


# Venue enumerations named in a product's schemas: name -> (values, sources, summary).
OPEN_ENUMS = {}


def open_enum_variant(value):
    variant = ''.join(part[:1].upper()+part[1:].lower() for part in value.split('_'))
    if not re.fullmatch(r'[A-Z][A-Za-z0-9]*', variant) or variant == 'Unknown':
        raise ValueError(('enum value has no variant spelling', PRODUCT, value))
    return variant


def render_open_enum(name, values, sources, summary, wire_type='string', names=()):
    variants = list(names) if names else [open_enum_variant(v) for v in values]
    if wire_type == 'integer':
        return render_integer_enum(name, values, sources, summary, variants)
    if len(set(variants)) != len(variants) or any(not re.fullmatch(r'[A-Z][A-Za-z0-9]*', v) or v == 'Unknown' for v in variants):
        raise ValueError(('enum values share an invalid variant spelling', PRODUCT, name))
    return '\n'.join([f'/// {summary}', '///', '/// One variant per value documented at:', '///',
        *[f'/// - <{url}>' for url in sources], '///',
        '/// A value the venue sends that is not documented there decodes to `Unknown`',
        '/// exactly as sent, and encodes back unchanged.',
        '#[derive(Clone, Debug, PartialEq, Eq, Hash)]', '#[non_exhaustive]', f'pub enum {name} {{',
        *[f'    /// Venue `{v}`.\n    {variants[i]},' for i, v in enumerate(values)],
        '    /// A value the source documentation does not list, kept exactly as the venue sent it.',
        '    Unknown(String),', '}',
        f'impl {name} {{', '    /// The exact venue spelling.', '    #[must_use]',
        '    pub fn as_str(&self) -> &str {', '        match self {',
        *[f'            Self::{variants[i]} => {lit(v)},' for i, v in enumerate(values)],
        '            Self::Unknown(value) => value,', '        }', '    }', '}',
        f'impl serde::Serialize for {name} {{',
        '    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {',
        '        serializer.serialize_str(self.as_str())', '    }', '}',
        f"impl<'de> serde::Deserialize<'de> for {name} {{",
        "    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {",
        "        let value = <String as serde::Deserialize>::deserialize(deserializer)?;",
        '        Ok(match value.as_str() {',
        *[f'            {lit(v)} => Self::{variants[i]},' for i, v in enumerate(values)],
        '            _ => Self::Unknown(value),', '        })', '    }', '}'])


def render_integer_enum(name, values, sources, summary, variants):
    if len(set(variants)) != len(variants) or any(not re.fullmatch(r'[A-Z][A-Za-z0-9]*', v) or v == 'Unknown' for v in variants):
        raise ValueError(('invalid integer enum variants', PRODUCT, name))
    return '\n'.join([f'/// {summary}', '///', '/// Documented at:',
        *[f'/// - <{url}>' for url in sources],
        '#[derive(Clone, Debug, PartialEq, Eq, Hash)]', '#[non_exhaustive]', f'pub enum {name} {{',
        *[f'    /// Venue code `{value}`.\n    {variants[i]},' for i, value in enumerate(values)],
        '    /// Future native integer code, retained exactly.', '    Unknown(i64),', '}',
        f'impl {name} {{', '    /// The exact native integer code.', '    #[must_use]',
        '    pub fn value(&self) -> i64 { match self {',
        *[f'        Self::{variants[i]} => {value},' for i, value in enumerate(values)],
        '        Self::Unknown(value) => *value,', '    } }', '}',
        f'impl serde::Serialize for {name} {{',
        '    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {',
        '        serializer.serialize_i64(self.value())', '    }', '}',
        f"impl<'de> serde::Deserialize<'de> for {name} {{",
        "    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {",
        "        let value = <i64 as serde::Deserialize>::deserialize(deserializer)?;",
        '        Ok(match value {',
        *[f'            {value} => Self::{variants[i]},' for i, value in enumerate(values)],
        '            _ => Self::Unknown(value),', '        })', '    }', '}'])


def generate_enums():
    text = '\n\n'.join(render_open_enum(name, *OPEN_ENUMS[name]) for name in sorted(OPEN_ENUMS))
    write(CORE_TRADING/PRODUCT/'enums.rs', HEADER+'//! Generated venue enumerations; regenerate with scripts/codegen/generate.py.\n\n'+text+'\n')


class Models:
    def __init__(self, components):
        self.components = components
        self.defs = {}

    def resolve(self, schema):
        if '$ref' in schema:
            return self.resolve(self.components[schema['$ref'].split('/')[-1]])
        return schema

    def open_enum(self, schema):
        # A string component with an `enum` list is a venue enumeration, typed
        # once per product; inline lists stay strings checked on the way out.
        name = schema.get('$ref', '').split('/')[-1]
        target = self.components.get(name, {}) if name else {}
        if target.get('type') not in ['string', 'integer'] or not target.get('enum'):
            return None
        if not target.get('x-sources') or not target.get('description'):
            raise ValueError(('venue enum needs its sources and summary', PRODUCT, name))
        wire_type = target['type']
        names = target.get('x-rust-variants', {})
        if wire_type == 'integer' and not names:
            raise ValueError(('integer enum needs native variant names', PRODUCT, name))
        variants = tuple(names[str(v)] if str(v) in names else open_enum_variant(v) for v in target['enum'])
        facts = (tuple(target['enum']), tuple(target['x-sources']), target['description'], wire_type, variants)
        if OPEN_ENUMS.setdefault(name, facts) != facts:
            raise ValueError(('venue enum defined differently within one product', PRODUCT, name))
        return 'super::enums::'+name

    def type(self, schema, name, response=True):
        enum = self.open_enum(schema) if response else None
        if enum:
            return enum
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
            tags = [self.resolve(x).get('properties', {}).get('filterType', {}).get('enum', []) for x in variants]
            if len(variants) == 1 and not all(len(t) == 1 for t in tags):
                return self.type(variants[0], name, response)
            if name not in self.defs:
                self.defs[name] = ''
                types = [self.type(x, name+'Variant'+str(i+1), response) for i, x in enumerate(variants)]
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
                validation=f'super::validation::validate({lit(op)}, &crate::core::parameters(&self)?)?;'
                if PRODUCT in ['margin','options']:
                    required='&['+', '.join(lit(k) for k in sorted(schema.get('required',[])))+']'
                    enums='&['+', '.join('('+lit(k)+', &['+', '.join(lit(str(v)) for v in s['enum'])+'])' for k,s in props.items() if s.get('enum'))+']'
                    bounds='&['+', '.join('('+lit(k)+', '+format(s.get('minimum',-9223372036854775808),'_')+', '+format(s.get('maximum',9223372036854775807),'_')+')' for k,s in props.items() if s.get('type')=='integer' and ('minimum' in s or 'maximum' in s))+']'
                    validation=f'let p=crate::core::parameters(&self)?;crate::core::validate_parameters(&p,{required},{enums},{bounds})?;super::validation::validate({lit(op)},&p)?;'
                self.defs[name]='\n'.join([f'/// Validated nested request builder for `{name}`.', '#[derive(Clone, Debug, Default, Serialize)]', f'pub struct {name} {{',*fields,'}', f'impl {name} {{', '    /// Start this nested request builder.', '    #[must_use]', '    pub fn new()->Self {Self::default()}',*setters,'    /// Validate required and conditional provider parameters.', '    ///', '    /// # Errors', '    /// Refuses missing or contradictory input.',f'    pub fn build(self)->Result<Self,crate::Error> {{ {validation} Ok(self) }}','}'])
            return name
        if not props:
            additional = schema.get('additionalProperties')
            if isinstance(additional, dict):
                return 'BTreeMap<'+schema.get('x-rust-key-type','String')+', '+self.type(additional, name+'Value', response)+'>'
            if response and PRODUCT in ['wallet','convert','margin','options']:
                self.defs[name]='\n'.join([f'/// Provider empty object receipt with retained future fields.', '#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]', '#[non_exhaustive]',f'pub struct {name} {{', '    /// Future fields; never logged implicitly.', '    #[serde(flatten)]','    pub extra:super::event_payloads::UnknownMessage,','}'])
                return name
            return 'BTreeMap<String, serde_json::Value>'
        if name not in self.defs:
            self.defs[name] = ''
            fields = []
            required = set(schema.get('required', []))
            if PRODUCT in ['wallet','convert']:
                required.update(k for k in props if k in ['asset','coin','fromAsset','toAsset','baseAsset','quoteAsset','free','locked','amount','fromAmount','toAmount','baseAmount','quoteAmount'])
            # A discriminator is required for alternatives, so an error object cannot
            # accidentally deserialize as an all-optional successful order.
            if 'code' in props:
                required.add('code')
            if 'orderId' in props and name!='TestOrderResponse' and not props['orderId'].get('nullable'):
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
                if s.get('x-deserialize-with'):
                    optional_attrs = ', default, skip_serializing_if = \"Option::is_none\"' if optional else ''
                    serde_attr = f'#[serde(rename = {lit(key)}{optional_attrs}, deserialize_with = {lit(s["x-deserialize-with"])})]'
                elif not optional and t == 'Decimal':
                    serde_attr = f'#[serde(rename = {lit(key)}, deserialize_with = "super::wire::decimal")]'
                elif not optional and t == 'Vec<Vec<Decimal>>':
                    serde_attr = f'#[serde(rename = {lit(key)}, deserialize_with = "super::wire::decimal_rows")]'
                elif optional and t == 'Decimal':
                    sentinel = s.get('x-decimal-unavailable')
                    decoder = {'': 'decimal_option_empty', 'null': 'decimal_option_null_string'}.get(sentinel, 'decimal_option')
                    serde_attr = f'#[serde(rename = {lit(key)}, default, deserialize_with = "super::wire::{decoder}", skip_serializing_if = "Option::is_none")]'
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
            bool_exception=['#[allow(clippy::struct_excessive_bools, reason = "independent provider wire flags must retain their native meaning")]'] if PRODUCT in ['margin','options'] and sum(self.type(v,name+pascal(k),response)=='bool' for k,v in props.items()) > 3 else []
            self.defs[name] = '\n'.join([f'/// Provider-native `{name}` payload.',
                '#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]', '#[non_exhaustive]', *bool_exception,
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
    auth_fields = ['signature', 'timestamp'] if PRODUCT == 'margin' and kind == 'rest' else ['apiKey', 'signature', 'timestamp']
    for key in auth_fields:
        props.pop(key, None)
        required.discard(key)
    if op['operationId'] in ['newOrder', 'testOrder', 'orderPlace', 'orderTest']:
        required.add('clientOrderId' if PRODUCT == 'options' else 'newClientOrderId')
    if op['operationId'] == 'newAlgoOrder':
        required.add('clientAlgoId')
    return props, required


def request_type(models, schema, name, field):
    if PRODUCT in ['margin','options'] and schema.get('x-rust-type'):
        return schema['x-rust-type']
    if schema.get('enum'):
        # Open response enums are strings; outgoing enum values are validated.
        return 'String'
    if field in ['symbol', 'pair', 'fromSymbol', 'toSymbol']:
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
            setter = snake(key)
            field = setter
            if PRODUCT == 'wallet' and name == 'Withdraw' and key == 'withdrawOrderId': field = 'caller_id'
            if PRODUCT == 'wallet' and name == 'OneClickArrivalDepositApply':
                field = {'depositId':'deposit','txId':'transaction','subAccountId':'sub_account','subUserId':'sub_user'}[key]
            fields += [f'    #[serde(rename = {lit(key)}, skip_serializing_if = "Option::is_none")]',f'    {field}: Option<{typ}>,']
            arg = 'impl Into<String>' if typ=='String' else typ
            val = 'value.into()' if typ=='String' else 'value'
            setters += [f'    /// Set the provider `{key}` parameter.',
                '    #[must_use]',f'    pub fn {setter}(mut self, value: {arg}) -> Self {{ self.{field} = Some({val}); self }}']
            if s.get('enum'):
                enums.append('('+lit(key)+', &['+', '.join(lit(str(v)) for v in s['enum'])+'])')
            if typ=='i64' and ('minimum' in s or 'maximum' in s):
                bounds.append('('+lit(key)+', '+format(s.get('minimum',-9223372036854775808),'_')+', '+format(s.get('maximum',9223372036854775807),'_')+')')
        margin_open_orders = PRODUCT == 'margin' and op['operationId'] == 'queryMarginAccountsOpenOrders'
        if margin_open_orders:
            fields += ['    #[serde(skip)]', '    trading_symbol_count: Option<super::TradingSymbolCount>,']
            setters += ['    /// Supply current venue trading-symbol count authority for all-symbol admission.',
                '    /// This local authority and its expiry are never sent to Binance.',
                '    #[must_use]', '    pub fn trading_symbol_count(mut self, value: super::TradingSymbolCount) -> Self { self.trading_symbol_count = Some(value); self }']
        cost_fn = 'super::rate::open_orders_cost(Self::OP, &parameters(self)?, self.trading_symbol_count)' if margin_open_orders else 'super::rate::cost(Self::OP, &parameters(self)?)'
        authority_fn = ['    fn validate_authority(&self, now: u64) -> Result<(), Error> { self.cost()?.validate_authority(Self::OP.name, now) }'] if margin_open_orders else []
        security = 'Signed' if op.get('x-signed') else 'Key' if op.get('x-security-type') in ['MARKET_DATA','USER_STREAM'] else 'Public'
        mutation = op['method']!='GET' if kind=='rest' else op['path'] in ['/order.place','/order.modify','/order.cancel','/algoOrder.place','/algoOrder.cancel','/userDataStream.start','/userDataStream.stop','/userDataStream.ping']
        if PRODUCT == 'spot' and kind == 'ws':
            mutation = op['tags'][0] == 'trade' or op['operationId'] in ['userDataStreamSubscribe','userDataStreamSubscribeSignature','userDataStreamUnsubscribe']
        if op['operationId'] in ['testOrder','orderTest','sorOrderTest']:mutation=False
        # Generating download jobs has side effects despite the HTTP GET method.
        if 'x-mutation' in op: mutation = op['x-mutation']
        if op['operationId'].startswith('getDownloadId'):
            mutation=True
        success_weight = 'Some(0)' if PRODUCT == 'spot' and op['operationId'] in ['newOrder','deleteOrder','deleteOpenOrders','orderPlace','orderCancel','openOrdersCancelAll'] else 'None'
        partial = 'Some(super::validation::partial)' if op.get('x-partial-result') else 'None'
        rps = op.get('x-requests-per-second')
        requests_per_second = f'Some({rps})' if rps is not None else 'None'
        rpm = op.get('x-requests-per-minute')
        requests_per_minute = f'Some({rpm})' if rpm is not None else 'None'
        time_validator = op.get('x-time-validator','super::validation::validate_time')
        op_expr = f'Operation {{ name: {lit(op["operationId"])}, path: {lit(op["path"])}, method: {lit(op["method"])}, security: Security::{security}, mutation: {str(mutation).lower()}, weight: {op.get("x-ip-weight",op.get("x-uid-weight",0))}, requests_per_second: {requests_per_second}, requests_per_minute: {requests_per_minute}, validate_time: {time_validator}, definitive: super::validation::definitive, success_weight: {success_weight}, partial: {partial} }}'
        required_rust='&['+', '.join(lit(v) for v in sorted(required))+']'
        validation=f'let p = parameters(self)?; validate_parameters(&p, {required_rust}, &[{", ".join(enums)}], &[{", ".join(bounds)}])?; super::validation::validate({lit(op["operationId"])}, &p)'
        # The weight admission charges, reported before sending; the venue's pools only.
        weight_fn = ['    /// The weight admission charges this request against its pool\'s minute weight',
            '    /// window, as the venue documents it for these parameters; read before sending.',
            '    ///', '    /// # Errors',
            '    /// Refuses a request dispatch would refuse before admission.',
            '    pub fn weight(&self) -> Result<u64, Error> { self.validate()?; Ok(self.cost()?.request_weight()) }',
            ] if kind == 'rest' and PRODUCT in ['spot', 'usdm', 'coinm'] else []
        if PRODUCT == 'convert' and op['operationId']=='acceptQuote':
            requests.append(f'/// Canonical quote acceptance operation facts.\npub(crate) const ACCEPT_QUOTE_OPERATION:Operation={op_expr};\npub use super::quote::AcceptQuote;')
        else:
            empty_response = ['    const EMPTY_RESPONSE: bool = true;'] if op.get('x-empty-response') else []
            requests.append('\n'.join([f'/// Validated request builder for [`{op["operationId"]}`]({op["source"]}).',
                '#[derive(Clone, Debug, Default, Serialize)]',f'pub struct {name} {{',*fields,'}',
                f'impl {name} {{','    /// Start a request builder. Required inputs are checked by `build` and by dispatch.',
                '    #[must_use]', '    pub fn new() -> Self { Self::default() }',*setters,
                '    /// Validate this request before dispatch.', '    ///', '    /// # Errors',
                '    /// Refuses missing, invalid, or contradictory provider parameters.',
                '    pub fn build(self) -> Result<Self, Error> { self.validate()?; Ok(self) }',*weight_fn,'}',
                f'impl Request for {name} {{', f'    type Response = super::{kind}_models::{response_type};',*empty_response,
                f'    const OP: Operation = {op_expr};',f'    fn validate(&self) -> Result<(), Error> {{ {validation} }}',
                *authority_fn, f'    fn cost(&self) -> Result<crate::core::Cost, Error> {{ {cost_fn} }}','}']))
        method=snake(op['operationId'])
        if kind=='rest':
            args=f'&self, request: &{name}, deadline: tokio::time::Instant'
            call='self.inner.execute(request, deadline).await'
        else:
            args=f'&self, request: &{name}, id: crate::RequestId, deadline: tokio::time::Instant'
            call='self.execute(request, id, deadline).await'
        return_type = f'super::{kind}_models::{response_type}'
        # Exchange information states the venue's own IP limits; its pool adopts them.
        if kind == 'rest' and (PRODUCT, op['operationId']) in [('spot', 'exchangeInfo'), ('usdm', 'exchangeInformation'), ('coinm', 'exchangeInformation'), ('options', 'exchangeInformation')]:
            call='let response=self.inner.execute(request,deadline).await?;super::rate::adopt_stated_limits(&self.inner,&response.data)?;Ok(response)'
        if kind == 'ws' and (PRODUCT, op['operationId']) == ('spot', 'exchangeInfo'):
            call='let response=self.execute(request,id,deadline).await?;super::rate::adopt_stated_ws_limits(self,&response.data)?;Ok(response)'
        if PRODUCT == 'wallet' and op['operationId'] in ['queryUserWalletBalance','dustConvert','dustConvertibleAssets']:
            context,field,wrapper = {'queryUserWalletBalance':('quote_asset','wallets','QuotedWalletBalance'), 'dustConvert':('target_asset','receipt','DustConversion'), 'dustConvertibleAssets':('target_asset','assets','ConvertibleDust')}[op['operationId']]
            return_type='super::'+wrapper
            call=f'let {context}=request.{context}.clone().ok_or(Error::Validation("asset provenance required"))?; let response=self.inner.execute(request,deadline).await?; Ok(crate::Response{{data:super::{wrapper}{{{context},{field}:response.data}},meta:response.meta}})'
        if PRODUCT == 'convert' and op['operationId'] == 'sendQuoteRequest':
            return_type='super::Quotation'
            call='let from_asset=request.from_asset.clone().ok_or(Error::Validation("source asset required"))?;let to_asset=request.to_asset.clone().ok_or(Error::Validation("target asset required"))?;let response=self.inner.execute(request,deadline).await?;Ok(crate::Response{data:super::Quotation{from_asset,to_asset,wallet_type:request.wallet_type.clone(),receipt:response.data},meta:response.meta})'
        if PRODUCT == 'convert' and op['operationId'] == 'acceptQuote':
            return_type='super::Acceptance'
            call='let response=self.inner.execute(request,deadline).await?;Ok(crate::Response{data:super::Acceptance{quotation:request.quotation().clone(),receipt:response.data},meta:response.meta})'
        if PRODUCT == 'margin' and op['operationId'] in ['queryMaxBorrow','queryMaxTransferOutAmount']:
            wrapper, field = ('BorrowCapacity','capacity') if op['operationId']=='queryMaxBorrow' else ('TransferCapacity','available')
            return_type='super::'+wrapper
            call=f'let asset=request.asset.clone().ok_or(Error::Validation("asset provenance required"))?;let response=self.inner.execute(request,deadline).await?;Ok(crate::Response{{data:super::{wrapper}{{asset,isolated_symbol:request.isolated_symbol.clone(),{field}:response.data}},meta:response.meta}})'
        if PRODUCT == 'margin' and op['operationId'] == 'createUserListenToken':
            return_type='super::ListenToken'
            call='request.validate()?;let scope=if request.is_isolated==Some(true){super::AccountScope::Isolated(request.symbol.clone().ok_or(Error::Validation("isolated token symbol required"))?)}else{super::AccountScope::Cross};let response=self.inner.execute(request,deadline).await?;Ok(crate::Response{data:super::ListenToken{scope,receipt:response.data},meta:response.meta})'
        limit_docs = ['    /// Adopts counted IP limits for every client sharing this pool.'] if kind == 'ws' and (PRODUCT, op['operationId']) == ('spot', 'exchangeInfo') else []
        limit_errors = ['    /// A counted limit without a positive value returns [`Error::Gap`] and leaves',
            '    /// the pool\'s limits unchanged; the attempt remains charged.'] if limit_docs else []
        methods.append('\n'.join([f'    /// [{op["operationId"]}]({op["source"]}).', *limit_docs,
            '    ///', '    /// # Errors', '    /// Returns input/admission errors before sending, or typed venue/transport evidence.',
            *limit_errors,
            f'    pub async fn {method}({args}) -> Result<crate::Response<{return_type}>, Error> {{ {call} }}']))
        coverage_entry={'name':op['operationId'],'method':op['method'],'path':op['path'],'source':op['source']}
        if rps is not None:
            coverage_entry['requests_per_second']=rps
        if rpm is not None:
            coverage_entry['requests_per_minute']=rpm
        if op.get('x-effective-from') is not None:
            coverage_entry['effective_from']=op['x-effective-from']
        coverage.append(coverage_entry)
    common='use std::collections::BTreeMap;\nuse serde::{Serialize, Deserialize};\nuse crate::{Decimal, Symbol, ClientOrderId, SensitiveString};\nuse super::wire::{PriceLevel, Kline};\n'
    # Output only necessary imports to keep strict lint gates unchanged.
    model_text='\n\n'.join(models.defs.values())
    imports=[]
    for line,token in [('use std::collections::BTreeMap;','BTreeMap'),('use serde::{Serialize, Deserialize};','derive'),('use crate::Decimal;','Decimal'),('use crate::Symbol;','Symbol'),('use crate::ClientOrderId;','ClientOrderId'),('use crate::SensitiveString;','SensitiveString'),('use super::wire::PriceLevel;','PriceLevel'),('use super::wire::Kline;','Kline')]:
        if re.search(r'(?<!::)\b'+re.escape(token)+r'\b',model_text):imports.append('use super::ClientOrderId;' if PRODUCT == 'spot' and token == 'ClientOrderId' else line)
    write((CORE_TRADING/PRODUCT/f'{kind}_models.rs'), HEADER+f'//! Generated {kind} response DTOs; regenerate with scripts/codegen/generate.py.\n\n'+'\n'.join(imports)+'\n\n'+model_text+'\n')
    request_text='\n\n'.join(requests)
    imports=['use serde::Serialize;','use crate::Error;','use crate::core::{Request, Operation, Security, parameters, validate_parameters};']
    for token in ['Symbol','ClientOrderId','SensitiveString','Decimal']:
        if re.search(r'(?<!::)\b'+token+r'\b',request_text):imports.append(f'use super::{token};' if PRODUCT == 'spot' and token == 'ClientOrderId' else f'use crate::{token};')
    # Nested input objects are emitted into the models module and imported here.
    nested=[n for n in models.defs if 'Input' in n and re.search(r'\b'+n+r'\b',request_text)]
    if nested: imports.append(f'use super::{kind}_models::{{'+', '.join(nested)+'};')
    client='RestClient' if kind=='rest' else 'WsClient'
    write((CORE_TRADING/PRODUCT/f'{kind}_requests.rs'), HEADER+f'//! Generated {kind} request builders.\n\n'+'\n'.join(imports)+'\n\n'+request_text+'\n\n'+f'impl super::{client} {{\n'+'\n\n'.join(methods)+'\n}\n')
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
            typ='&Symbol' if key in ['symbol','pair'] else '&crate::Symbol' if PRODUCT == 'options' and key == 'underlying' else '&str'
            args.append(f'{field}: {typ}')
            value=f'{field}.as_str()' if typ in ['&Symbol','&crate::Symbol'] else field
            if key in ['symbol','pair','underlying']: value+=' .to_lowercase().as_str()'
            if PRODUCT == 'options' and typ == '&str' and param.get('required',False):
                replacements.append(f'if {field}.is_empty() {{ return Err(Error::Validation("required stream parameter")); }}')
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
    write((CORE_TRADING/PRODUCT/'stream_models.rs'), HEADER+'//! Generated market and user-data event payloads.\n\n'+'\n'.join(imports)+'\n\n'+text+'\n')
    stream_imports = 'use crate::Error;\nuse super::Symbol;' if PRODUCT == 'options' else 'use crate::{Error, Symbol};'
    write((CORE_TRADING/PRODUCT/'stream_names.rs'), HEADER+'//! Generated constructors for every documented market stream.\n\n'+stream_imports+'\nuse super::streams::{Stream, Route};\n\nimpl Stream {\n'+'\n\n'.join(methods)+'\n}\n')
    return [{'name':n,'path':p,'route':r,'type':t} for n,p,r,t in names]


def generate_margin_events():
    """Margin uses execution/risk sockets, without invented market stream names."""
    snap=json.loads((ROOT/'schema/margin-streams.json').read_text())
    models=Models(snap['components']['schemas'])
    for name,schema in snap['components']['schemas'].items():
        models.type(schema,pascal(name)+'Event')
    text='\n\n'.join(models.defs.values())
    imports=['use serde::{Serialize, Deserialize};']
    for line,token in [('use crate::Decimal;','Decimal'),('use crate::Symbol;','Symbol'),('use crate::ClientOrderId;','ClientOrderId'),('use crate::SensitiveString;','SensitiveString')]:
        if re.search(r'(?<!::)\b'+token+r'\b',text):imports.append(line)
    write(CORE_TRADING/'margin/stream_models.rs',HEADER+'//! Generated native Margin risk and execution payloads.\n\n'+'\n'.join(imports)+'\n\n'+text+'\n')


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
    text+='''    /// A future stream name or kind this build does not model, preserved
    /// with redacted Debug for explicit consumer handling.
    Unknown(UnknownMessage),
}

pub(crate) fn market_payload(kind:&str,value:Value)->Result<MarketPayload,Error> {
    match kind {
'''
    for o in coverage['streams']:
     text+=f'        "{o["name"][0].lower()+o["name"][1:]}"=>serde_json::from_value(value).map(|v|MarketPayload::{o["name"]}(Box::new(v))),\n'
    text+='''        // An unrecognized stream kind is retained evidence, not a continuity
        // break; `Error::Gap` is reserved for malformed payloads of known kinds.
        _=>return Ok(MarketPayload::Unknown(value.into())),
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
    # Evidence guards are emitted only for events this product's own pinned
    # stream schema documents. A foreign product's event name is never borrowed
    # into a local continuity failure; the catch-all retains it as unknown.
    documented={s['properties']['e']['enum'][0] for s in ss.values()
        if isinstance(s,dict) and 'e' in s.get('properties',{}) and 'enum' in s['properties'].get('e',{})}
    guards=''
    for event,probe,message in [
        ('ACCOUNT_UPDATE','value.get("a").is_none_or(|a|a.get("B").is_none() && a.get("P").is_none())','account event has no balance/position evidence'),
        ('ACCOUNT_CONFIG_UPDATE','value.get("ac").is_none() && value.get("ai").is_none()','account configuration event has no evidence'),
    ]:
        if event in documented:
            guards+=f'        if event=="{event}" && {probe} {{return Err(Error::Gap("{message}"));}}\n'
    text+='''    /// A future event type, preserved for explicit consumer handling.
        Unknown(UnknownMessage),
    }
    
    pub(crate) fn user_payload(value:Value)->Result<UserPayload,Error> {
        let event=value.get("e").and_then(Value::as_str).ok_or(Error::Gap("user event type"))?;
'''+guards+'''        match event {
    '''
    for n,s in ss.items():
     if n=='User Data Stream Events' or 'e' not in s.get('properties',{}):continue
     name=pascal(n);event=s['properties']['e']['enum'][0]
     text+=f'        "{event}"=>serde_json::from_value(value).map(|v|UserPayload::{name}(Box::new(v))),\n'
    text+='''        _=>return Ok(UserPayload::Unknown(value.into())),
        }.map_err(|_|Error::Gap("malformed execution/account event"))
    }
    '''
    write((CORE_TRADING/PRODUCT/'event_payloads.rs'), HEADER+text)

def main():
    global PRODUCT
    check='--check' in sys.argv
    files=['rest_models.rs','rest_requests.rs','ws_models.rs','ws_requests.rs','stream_models.rs','stream_names.rs','event_payloads.rs','enums.rs']
    paths=[*(CORE_TRADING/p/f for p in ['usdm','spot','coinm'] for f in files), ROOT/'schema/coverage.json', ROOT/'schema/spot-coverage.json', ROOT/'schema/coinm-coverage.json']
    before={p:p.read_bytes() if p.exists() else None for p in paths}
    rest_products = ['wallet','convert','margin','options']
    paths += [*(CORE_TRADING/p/f for p in rest_products for f in ['rest_models.rs','rest_requests.rs']),*(CORE_TRADING/p/'enums.rs' for p in rest_products),*(ROOT/'schema'/f'{p}-coverage.json' for p in rest_products),*(CORE_TRADING/'options'/f for f in ['stream_models.rs','stream_names.rs']),CORE_TRADING/'margin/stream_models.rs']
    before.update({p:p.read_bytes() if p.exists() else None for p in paths if p not in before})
    for PRODUCT in ['usdm','spot','coinm',*rest_products]:
        OPEN_ENUMS.clear()
        if PRODUCT in rest_products:
            coverage={'rest':generate('rest')}
            product_files=['rest_models.rs','rest_requests.rs']
            if PRODUCT == 'options':
                coverage['streams']=generate_streams()
                product_files += ['stream_models.rs','stream_names.rs']
            if PRODUCT == 'margin':
                generate_margin_events()
                product_files += ['stream_models.rs']
            generate_enums()
            product_files += ['enums.rs']
            write(ROOT/'schema'/f'{PRODUCT}-coverage.json',json.dumps(coverage,indent=2)+'\n')
            subprocess.run(['rustfmt','--edition','2024',*[str(CORE_TRADING/PRODUCT/f) for f in product_files]],check=True)
            print(PRODUCT+': '+', '.join(f'{len(v)} {k}' for k,v in coverage.items())+'.')
            continue
        coverage={kind:generate(kind) for kind in ['rest','ws']}
        coverage['streams']=generate_streams()
        filename='coverage.json' if PRODUCT == 'usdm' else f'{PRODUCT}-coverage.json'
        write((ROOT/'schema'/filename), json.dumps(coverage,indent=2)+'\n')
        generate_events(coverage)
        generate_enums()
        subprocess.run(['rustfmt','--edition','2024',*[str(CORE_TRADING/PRODUCT/f) for f in files]],check=True)
        print(PRODUCT+': '+', '.join(f'{len(v)} {k}' for k,v in coverage.items())+'.')
    if check:
        changed=[p for p,v in before.items() if p.read_bytes()!=v]
        for p in changed:
            if before[p] is None:p.unlink()
            else:p.write_bytes(before[p])
        if changed:raise SystemExit('Stale generated bindings: '+', '.join(str(p.relative_to(ROOT)) for p in changed))


if __name__=='__main__':
    main()
