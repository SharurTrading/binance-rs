# SPDX-FileCopyrightText: 2026 Kevin Monaghan
# SPDX-License-Identifier: MIT-0

"""Generate Advanced Trading stream DTOs and source-local payload dispatch."""
import json
import re

STREAM_PRODUCTS = ['portfolio_margin', 'portfolio_margin_pro', 'alpha', 'stocks']


def generate(g):
    product = g.PRODUCT
    snap = json.loads((g.ROOT/'schema'/f'{g.fact_name(product)}-streams.json').read_text())
    models = g.Models(snap['components']['schemas'])
    variants = []
    constructors = []
    coverage = []
    for op in snap['operations']:
        name = g.pascal(op['operationId'])
        schema = op['responses']['Raw Stream']['content']['application/json']['schema']
        typ = models.type(schema, name+'Event')
        variants.append((name, typ, op['operationId']))
        coverage.append(dict(name=op['operationId'], path=op['path'], source=op['source'], private=op['operationId']=='orderReportStream'))
        if op['operationId']=='orderReportStream':
            continue  # User data connects only through the dedicated sensitive-token API.
        params = [p for p in op.get('parameters', []) if '{'+p['name']+'}' in op['path']]
        args = []
        replacements = []
        for param in params:
            key = param['name']; field = g.snake(key)
            typ = '&crate::Symbol' if key=='symbol' else '&str'
            args.append(f'{field}: {typ}')
            if typ=='&str':
                replacements.append(f'if {field}.is_empty()||{field}.contains([\'/\',\'?\',\'&\',\'#\']){{return Err(crate::Error::Validation("native stream parameter"));}}')
            val = f'{field}.as_str()' if key=='symbol' else field
            if key=='symbol' and product=='alpha':
                val=f'&{field}.as_str().to_lowercase()'
            if key=='symbol' and product=='stocks':
                replacements.append(f'if !{field}.as_str().chars().all(|c|c.is_ascii_uppercase()||matches!(c,\'.\'|\'-\')){{return Err(crate::Error::Validation("native stock ticker"));}}')
            choices = param.get('schema',{}).get('enum')
            if choices:
                replacements.append(f'if ![{", ".join(g.lit(str(x)) for x in choices)}].contains(&{field}){{return Err(crate::Error::Validation("native stream parameter"));}}')
            replacements.append(f'name=name.replace({g.lit("{"+key+"}")},{val});')
        constructors.append('\n'.join([f'    /// Native `{op["operationId"]}` stream; see <{op["source"]}>.',
            '    ///', '    /// # Errors', '    /// Refuses invalid native parameters or stream-path delimiters.',
            f'    pub fn {g.snake(op["operationId"])}({", ".join(args)})->Result<Self,crate::Error>{{',
            f'        let {"mut " if params else ""}name={g.lit(op["path"].lstrip("/"))}.to_owned();',
            *['        '+v for v in replacements],
            f'        Self::new(name,{g.lit(op["operationId"])})','    }']))
    user = []
    for name, schema in snap['components']['schemas'].items():
        typ=models.type(schema,g.pascal(name)+'Event')
        event=schema.get('properties',{}).get('e',{}).get('enum')
        if event:
            variants.append((g.pascal(name),typ,name))
            user.append((event[0],g.pascal(name)))
    model_text='\n\n'.join(models.defs.values())
    imports=['use serde::{Serialize,Deserialize};']
    for typ in ['Decimal','Symbol','ClientOrderId','SensitiveString']:
        if re.search(r'(?<!::)\b'+typ+r'\b',model_text):imports.append(f'use crate::{typ};')
    for typ in ['PriceLevel','Kline']:
        if re.search(r'\b'+typ+r'\b',model_text):imports.append(f'use super::wire::{typ};')
    g.write(g.product_root(product)/'stream_models.rs',g.HEADER+'//! Generated native stream DTOs.\n\n'+'\n'.join(imports)+'\n\n'+model_text+'\n')
    event_text='''//! Native stream dispatch with retained future-event evidence.
pub use crate::core_trading::wallet::event_payloads::UnknownMessage;
/// A native event, without cross-product account or instrument normalization.
#[derive(Clone,Debug,PartialEq)]
#[non_exhaustive]
pub enum Payload {
'''
    for name,typ,key in variants:
        event_text+=f'    /// Native `{key}` evidence.\n    {name}(Box<super::stream_models::{typ}>),\n'
    event_text+='''    /// Future native event or stream shape, retained with redacted diagnostics.
    Unknown(UnknownMessage),
}
/// Decode one named market stream, or a user event when `kind` is absent.
///
/// # Errors
/// Returns a gap for malformed known evidence, including financial parse failures.
pub fn decode(kind:Option<&str>,value:serde_json::Value)->Result<Payload,crate::Error>{
'''
    if user:
        event_text+='''    if kind.is_none(){
        let event=value.get("e").and_then(serde_json::Value::as_str).ok_or(crate::Error::Gap("native user event type required"))?;
'''
        if product=='portfolio_margin':
            event_text+='''        if event=="ACCOUNT_UPDATE" && value.get("a").is_none_or(|a|a.get("B").is_none()&&a.get("P").is_none()) {return Err(crate::Error::Gap("native account update has no balance or position evidence"));}
        let commission=match event {"ORDER_TRADE_UPDATE"=>value.get("o"),"executionReport"=>Some(&value),_=>None};
        if let Some(commission)=commission {
            if let Some(amount)=commission.get("n") {
                let amount=super::wire::parse_decimal(amount).map_err(|_|crate::Error::Gap("native commission amount"))?;
                if !amount.is_zero() && commission.get("N").is_none_or(serde_json::Value::is_null) {return Err(crate::Error::Gap("native commission asset required"));}
            } else if commission.get("N").is_some_and(|asset|!asset.is_null()) {return Err(crate::Error::Gap("native commission amount required"));}
        }
'''
        event_text+='''
        return match event{
'''
        for event,name in user:
            event_text+=f'            {g.lit(event)}=>decode_value(value,Payload::{name}),\n'
        event_text+='            _=>Ok(Payload::Unknown(value.into())),\n        };\n    }\n'
    if product=='stocks':
        event_text+='''    let kind=kind.or_else(||match value.get("e").and_then(serde_json::Value::as_str){
'''
        for op in snap['operations']:
            event=op['operationId'].removesuffix('Stream')
            event_text+=f'        Some({g.lit(event)})=>Some({g.lit(op["operationId"])}),\n'
        event_text+='        _=>None,\n    });\n'
    event_text+='    match kind{\n'
    for name,typ,key in variants:
        operation = next((op for op in snap['operations'] if op['operationId']==key), None)
        guard = ''
        if operation and operation.get('x-event-type'):
            guard = g.lit(operation['x-event-type'])
        event_text+=f'        Some({g.lit(key)})=>decode_market(value,{guard or "None" if not guard else "Some("+guard+")"},Payload::{name}),\n'
    event_text+='        _=>Ok(Payload::Unknown(value.into())),\n    }\n}\n'
    event_text+='''
fn decode_value<T:serde::de::DeserializeOwned>(value:serde_json::Value,wrap:fn(Box<T>)->Payload)->Result<Payload,crate::Error>{
    serde_json::from_value(value).map(|value|wrap(Box::new(value))).map_err(|_|crate::Error::Gap("malformed native stream event"))
}
fn decode_market<T:serde::de::DeserializeOwned>(value:serde_json::Value,event:Option<&str>,wrap:fn(Box<T>)->Payload)->Result<Payload,crate::Error>{
    if event.is_some_and(|expected|value.get("e").and_then(serde_json::Value::as_str).is_some_and(|actual|actual!=expected)){return Ok(Payload::Unknown(value.into()));}
    decode_value(value,wrap)
}
'''
    g.write(g.product_root(product)/'event_payloads.rs',g.HEADER+event_text)
    g.write(g.product_root(product)/'stream_names.rs',g.HEADER+'//! Generated native market stream names.\n\nimpl crate::advanced_trading::streams::Stream<super::Protocol>{\n'+'\n\n'.join(constructors)+'\n}\n')
    return coverage
