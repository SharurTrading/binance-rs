// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Offline schema interpreter. Only documented versions/templates are accepted.
use crate::{Decimal, Error};
use serde::{Deserialize, de::DeserializeOwned};
use serde_json::{Map, Value};
use std::{collections::BTreeMap, sync::OnceLock};

#[derive(Deserialize)]
struct Snapshot {
    schema: Node,
}
#[derive(Clone, Deserialize)]
pub(crate) struct Node {
    pub kind: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub children: Vec<Node>,
    #[serde(flatten)]
    pub attrs: BTreeMap<String, String>,
}
impl Node {
    pub fn attr(&self, name: &str) -> Option<&str> {
        self.attrs.get(name).map(String::as_str)
    }
}
pub(crate) struct Schema {
    pub id: u16,
    pub version: u16,
    pub types: BTreeMap<String, Node>,
    pub messages: BTreeMap<u16, Node>,
}
impl Schema {
    fn load(raw: &str) -> Result<Self, Error> {
        let root = serde_json::from_str::<Snapshot>(raw)
            .map_err(|_| Error::Configuration("binary schema snapshot"))?
            .schema;
        let id = root
            .attr("id")
            .and_then(|s| s.parse().ok())
            .ok_or(Error::Configuration("schema ID"))?;
        let version = root
            .attr("version")
            .and_then(|s| s.parse().ok())
            .ok_or(Error::Configuration("schema version"))?;
        let mut types = BTreeMap::new();
        let mut messages = BTreeMap::new();
        for n in root.children {
            if n.kind == "types" {
                types.extend(n.children.into_iter().map(|t| (t.name.clone(), t)));
            } else if n.kind == "message" {
                let id = n
                    .attr("id")
                    .and_then(|s| s.parse().ok())
                    .ok_or(Error::Configuration("template ID"))?;
                messages.insert(id, n);
            }
        }
        Ok(Self {
            id,
            version,
            types,
            messages,
        })
    }
    pub(crate) fn type_node(&self, name: &str) -> Result<Node, Error> {
        if let Some(t) = self.types.get(name) {
            return Ok(t.clone());
        }
        if primitive_size(name).is_some() {
            return Ok(Node {
                name: name.into(),
                kind: "type".into(),
                children: vec![],
                attrs: BTreeMap::from([("primitiveType".into(), name.into())]),
            });
        }
        Err(Error::Configuration("unresolved binary type"))
    }
    pub(crate) fn size(&self, name: &str) -> Result<usize, Error> {
        let n = self.type_node(name)?;
        match n.kind.as_str() {
            "type" => primitive_size(
                n.attr("primitiveType")
                    .ok_or(Error::Configuration("primitive type"))?,
            )
            .and_then(|v| v.checked_mul(n.attr("length").and_then(|s| s.parse().ok()).unwrap_or(1)))
            .ok_or(Error::Configuration("primitive width")),
            "enum" | "set" => self.size(
                n.attr("encodingType")
                    .ok_or(Error::Configuration("encoding type"))?,
            ),
            "composite" => n.children.iter().try_fold(0_usize, |total, c| {
                let size = if c.kind == "ref" {
                    self.size(
                        c.attr("type")
                            .ok_or(Error::Configuration("reference type"))?,
                    )?
                } else {
                    primitive_size(
                        c.attr("primitiveType")
                            .ok_or(Error::Configuration("composite type"))?,
                    )
                    .and_then(|v| {
                        v.checked_mul(c.attr("length").and_then(|s| s.parse().ok()).unwrap_or(1))
                    })
                    .ok_or(Error::Configuration("composite width"))?
                };
                total
                    .checked_add(size)
                    .ok_or(Error::Configuration("composite overflow"))
            }),
            _ => Err(Error::Configuration("binary type kind")),
        }
    }
}
fn primitive_size(name: &str) -> Option<usize> {
    match name {
        "char" | "int8" | "uint8" => Some(1),
        "int16" | "uint16" => Some(2),
        "int32" | "uint32" => Some(4),
        "int64" | "uint64" => Some(8),
        _ => None,
    }
}
static API_SCHEMA: OnceLock<Result<Schema, Error>> = OnceLock::new();
static FIX_SCHEMA: OnceLock<Result<Schema, Error>> = OnceLock::new();
pub(super) fn api_schema() -> Result<&'static Schema, Error> {
    API_SCHEMA
        .get_or_init(|| Schema::load(include_str!("../../../../schema/spot/api-sbe.json")))
        .as_ref()
        .map_err(|_| Error::Configuration("API binary schema"))
}
pub(crate) fn fix_schema() -> Result<&'static Schema, Error> {
    FIX_SCHEMA
        .get_or_init(|| Schema::load(include_str!("../../../../schema/spot/fix-sbe.json")))
        .as_ref()
        .map_err(|_| Error::Configuration("FIX binary schema"))
}

struct Reader<'a> {
    bytes: &'a [u8],
    offset: usize,
    schema: &'a Schema,
    template: Option<u16>,
    ancestors: Vec<u16>,
}
impl<'a> Reader<'a> {
    fn error(&self, reason: &'static str) -> Error {
        Error::BinaryDecode {
            schema_id: Some(self.schema.id),
            version: Some(self.schema.version),
            template_id: self.template,
            offset: self.offset,
            reason,
        }
    }
    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = self
            .offset
            .checked_add(n)
            .ok_or_else(|| self.error("offset overflow"))?;
        let bytes = self
            .bytes
            .get(self.offset..end)
            .ok_or_else(|| self.error("truncated binary field"))?;
        self.offset = end;
        Ok(bytes)
    }
    fn integer(&mut self, primitive: &str, width: usize) -> Result<i128, Error> {
        let mut bytes = [0; 16];
        let input = self.take(width)?;
        let signed = primitive.starts_with("int");
        if signed && input.last().is_some_and(|b| b & 0x80 != 0) {
            bytes.fill(0xff);
        }
        bytes
            .get_mut(..width)
            .ok_or(Error::Configuration("integer width"))?
            .copy_from_slice(input);
        Ok(i128::from_le_bytes(bytes))
    }
    fn value(&mut self, node: &Node, scope: &BTreeMap<String, Value>) -> Result<Value, Error> {
        let name = node
            .attr("type")
            .ok_or(Error::Configuration("field type"))?;
        let ty = self.schema.type_node(name)?;
        if node.attr("presence") == Some("constant") {
            let reference = node
                .attr("valueRef")
                .ok_or(Error::Configuration("constant reference"))?;
            let (_, label) = reference
                .split_once('.')
                .ok_or(Error::Configuration("constant name"))?;
            let v = ty
                .children
                .iter()
                .find(|v| v.name == label)
                .ok_or(Error::Configuration("constant value"))?;
            return enum_value(name, v).ok_or(Error::Configuration("constant encoding"));
        }
        let primitive = ty
            .attr("primitiveType")
            .or_else(|| ty.attr("encodingType"))
            .ok_or(Error::Configuration("scalar encoding"))?;
        let width = self.schema.size(name)?;
        let value = self.integer(
            if name == "mantissa128" {
                "int128"
            } else {
                primitive
            },
            width,
        )?;
        let null = if primitive == "char" {
            0
        } else if primitive.starts_with("int") || name == "mantissa128" {
            if width == 16 {
                i128::MIN
            } else {
                -(1_i128 << (width * 8 - 1))
            }
        } else {
            (1_i128 << (width * 8)) - 1
        };
        if value == null {
            if node.attr("presence") == Some("optional") {
                return Ok(Value::Null);
            }
            return Err(self.error("null required binary field"));
        }
        if ty.kind == "enum" {
            let v = ty
                .children
                .iter()
                .find(|n| {
                    n.attr("value").is_some_and(|s| {
                        if primitive == "char" {
                            s.as_bytes()
                                .first()
                                .is_some_and(|b| i128::from(*b) == value)
                        } else {
                            s.parse::<i128>().ok() == Some(value)
                        }
                    })
                })
                .ok_or_else(|| self.error("unknown binary enum value"))?;
            if self.schema.id == 1 && name != "boolEnum" {
                return v
                    .attr("value")
                    .map(|v| Value::String(v.into()))
                    .ok_or_else(|| self.error("FIX enum code"));
            }
            return enum_value(name, v).ok_or_else(|| self.error("enum representation"));
        }
        if ty.kind == "set" {
            return self.set_value(&ty, value);
        }
        if let Some(exponent) = node.attr("exponent") {
            let exponent = scope
                .get(exponent)
                .and_then(Value::as_i64)
                .ok_or_else(|| self.error("missing mantissa exponent"))?;
            let exponent = i8::try_from(exponent).map_err(|_| self.error("exponent range"))?;
            return exact_decimal(value, exponent)
                .map(|v| Value::String(v.to_string()))
                .ok_or_else(|| self.error("decimal cannot be represented exactly"));
        }
        if primitive.starts_with("uint") {
            u64::try_from(value)
                .map(Value::from)
                .map_err(|_| self.error("unsigned integer range"))
        } else {
            i64::try_from(value)
                .map(Value::from)
                .map_err(|_| self.error("signed integer range"))
        }
    }
    fn set_value(&self, ty: &Node, value: i128) -> Result<Value, Error> {
        let mut names = Vec::new();
        let mut known = 0_i128;
        for choice in &ty.children {
            let bit = choice
                .attr("value")
                .and_then(|v| v.parse::<u32>().ok())
                .ok_or(Error::Configuration("set bit"))?;
            let mask = 1_i128
                .checked_shl(bit)
                .ok_or(Error::Configuration("set mask"))?;
            known |= mask;
            if value & mask != 0 {
                names.push(Value::String(
                    choice.attr("jsonValue").unwrap_or(&choice.name).into(),
                ));
            }
        }
        if value & !known != 0 {
            return Err(self.error("unknown binary set bits"));
        }
        Ok(Value::Array(names))
    }
    fn body(
        &mut self,
        nodes: &[Node],
        block: usize,
        parent: &BTreeMap<String, Value>,
    ) -> Result<Value, Error> {
        let start = self.offset;
        let mut scope = parent.clone();
        let mut output = Value::Object(Map::new());
        for field in nodes.iter().filter(|n| n.kind == "field") {
            let value = self.value(field, &scope)?;
            scope.insert(field.name.clone(), value.clone());
            if field.attr("type") != Some("exponent8") {
                insert(&mut output, field, value)?;
            }
        }
        if self.offset - start != block {
            return Err(self.error("unexpected fixed block length"));
        }
        for field in nodes.iter().filter(|n| n.kind != "field") {
            let value = match field.kind.as_str() {
                "group" => self.group(field, &scope)?,
                "data" => self.data(field)?,
                _ => return Err(self.error("unknown schema field kind")),
            };
            insert(&mut output, field, value)?;
        }
        Ok(output)
    }
    fn group(&mut self, field: &Node, scope: &BTreeMap<String, Value>) -> Result<Value, Error> {
        let dimensions = self
            .schema
            .type_node(field.attr("dimensionType").unwrap_or("groupSizeEncoding"))?;
        let mut values = BTreeMap::new();
        for d in dimensions.children {
            let primitive = d
                .attr("primitiveType")
                .ok_or(Error::Configuration("group dimension"))?;
            let width = primitive_size(primitive).ok_or(Error::Configuration("dimension width"))?;
            values.insert(d.name.clone(), self.integer(primitive, width)?);
        }
        let block = values
            .get("blockLength")
            .and_then(|v| usize::try_from(*v).ok())
            .ok_or_else(|| self.error("group block width"))?;
        let count = values
            .get("numInGroup")
            .and_then(|v| usize::try_from(*v).ok())
            .ok_or_else(|| self.error("group count"))?;
        if count > (self.bytes.len() - self.offset) / block.max(1) {
            return Err(self.error("truncated group entries"));
        }
        (0..count)
            .map(|_| self.body(&field.children, block, scope))
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array)
    }
    fn data(&mut self, field: &Node) -> Result<Value, Error> {
        let name = field
            .attr("type")
            .ok_or(Error::Configuration("data type"))?;
        let ty = self.schema.type_node(name)?;
        let primitive = ty
            .children
            .first()
            .and_then(|n| n.attr("primitiveType"))
            .ok_or(Error::Configuration("data length type"))?;
        let width = primitive_size(primitive).ok_or(Error::Configuration("length width"))?;
        let length = self.integer(primitive, width)?;
        let length = usize::try_from(length).map_err(|_| self.error("data length range"))?;
        if length == 0 && name.starts_with("optional") {
            return Ok(Value::Null);
        }
        let bytes = self.take(length)?;
        if name.to_ascii_lowercase().contains("messagedata") {
            decode_nested(bytes, self.schema, false, self.ancestors.clone())
        } else {
            std::str::from_utf8(bytes)
                .map(|s| Value::String(s.into()))
                .map_err(|_| self.error("data UTF-8"))
        }
    }
}
fn enum_value(name: &str, value: &Node) -> Option<Value> {
    if name == "boolEnum" {
        match value.attr("value")? {
            "0" => Some(Value::Bool(false)),
            "1" => Some(Value::Bool(true)),
            _ => None,
        }
    } else {
        Some(Value::String(
            value.attr("jsonValue").unwrap_or(&value.name).into(),
        ))
    }
}
pub(super) fn exact_decimal(mantissa: i128, exponent: i8) -> Option<Decimal> {
    if mantissa == 0 {
        return Some(Decimal::ZERO);
    }
    let mut mantissa = mantissa;
    let mut scale = u32::from(exponent.unsigned_abs());
    if exponent > 0 {
        return Decimal::try_from_i128_with_scale(mantissa, 0)
            .ok()?
            .checked_mul(Decimal::try_from_i128_with_scale(10_i128.checked_pow(scale)?, 0).ok()?);
    }
    while scale > 0 && mantissa % 10 == 0 {
        mantissa /= 10;
        scale -= 1;
    }
    Decimal::try_from_i128_with_scale(mantissa, scale).ok()
}
fn insert(output: &mut Value, node: &Node, mut value: Value) -> Result<(), Error> {
    if value.is_null()
        && let Some(default) = node.attr("jsonDefaultValue")
    {
        value = serde_json::from_str(default).unwrap_or_else(|_| Value::String(default.into()));
    }
    let path = node.attr("jsonPath").unwrap_or(&node.name);
    if path == ".." {
        *output = value;
        return Ok(());
    }
    if path == "[]" {
        if output.is_object() {
            *output = Value::Array(Vec::new());
        }
        output
            .as_array_mut()
            .ok_or(Error::Configuration("schema array projection"))?
            .push(value);
        return Ok(());
    }
    let mut object = output;
    let components: Vec<_> = path.split('.').collect();
    for (i, key) in components.iter().enumerate() {
        let map = object
            .as_object_mut()
            .ok_or(Error::Configuration("schema object projection"))?;
        if i == components.len() - 1 {
            map.insert((*key).into(), value);
            return Ok(());
        }
        object = map
            .entry((*key).to_owned())
            .or_insert_with(|| Value::Object(Map::new()));
    }
    Ok(())
}
pub(super) fn decode_value(bytes: &[u8], schema: &Schema, fix: bool) -> Result<Value, Error> {
    decode_nested(bytes, schema, fix, vec![])
}
fn decode_nested(
    bytes: &[u8],
    schema: &Schema,
    fix: bool,
    ancestors: Vec<u16>,
) -> Result<Value, Error> {
    let mut r = Reader {
        bytes,
        offset: 0,
        schema,
        template: None,
        ancestors,
    };
    let block = usize::try_from(r.integer("uint16", 2)?).map_err(|_| r.error("root width"))?;
    let template = u16::try_from(r.integer("uint16", 2)?).map_err(|_| r.error("template range"))?;
    let id = u16::try_from(r.integer("uint16", 2)?).map_err(|_| r.error("schema range"))?;
    let version = u16::try_from(r.integer("uint16", 2)?).map_err(|_| r.error("version range"))?;
    r.template = Some(template);
    if r.ancestors.contains(&template) {
        return Err(r.error("recursive binary envelope"));
    }
    r.ancestors.push(template);
    if id != schema.id || version != schema.version {
        return Err(Error::BinaryDecode {
            schema_id: Some(id),
            version: Some(version),
            template_id: Some(template),
            offset: 0,
            reason: "unsupported binary schema",
        });
    }
    let node = schema
        .messages
        .get(&template)
        .ok_or_else(|| r.error("unknown binary template"))?;
    if node.name == "NonRepresentableMessage" {
        return Err(r.error("venue non-representable response"));
    }
    let (seq, time) = if fix {
        (Some(r.integer("uint32", 4)?), Some(r.integer("int64", 8)?))
    } else {
        (None, None)
    };
    let mut value = r.body(&node.children, block, &BTreeMap::new())?;
    if r.offset != bytes.len() {
        return Err(r.error("trailing binary bytes"));
    }
    if fix {
        let map = value
            .as_object_mut()
            .ok_or_else(|| r.error("FIX message shape"))?;
        map.insert(
            "MsgSeqNum".into(),
            Value::from(
                u32::try_from(seq.ok_or_else(|| r.error("missing sequence"))?)
                    .map_err(|_| r.error("sequence range"))?,
            ),
        );
        map.insert(
            "SendingTime".into(),
            Value::from(
                i64::try_from(time.ok_or_else(|| r.error("missing timestamp"))?)
                    .map_err(|_| r.error("timestamp range"))?,
            ),
        );
        map.insert(
            "MsgType".into(),
            Value::String(
                node.attr("semanticType")
                    .ok_or(Error::Configuration("FIX message kind"))?
                    .into(),
            ),
        );
    } else if node.name == "WebSocketResponse"
        && value
            .get("status")
            .and_then(Value::as_u64)
            .is_some_and(|s| s >= 400)
    {
        let map = value
            .as_object_mut()
            .ok_or_else(|| r.error("WebSocket response shape"))?;
        if let Some(error) = map.remove("result") {
            map.insert("error".into(), error);
        }
    }
    if !fix {
        api_projection(&node.name, &mut value)?;
    }
    Ok(value)
}
fn api_projection(name: &str, value: &mut Value) -> Result<(), Error> {
    if name == "KlinesResponse" {
        let rows = value.as_array_mut().ok_or(Error::Gap("SBE kline shape"))?;
        for row in rows {
            row.as_array_mut()
                .ok_or(Error::Gap("SBE kline row"))?
                .push(Value::Null);
        }
    }
    let event = match name {
        "AllocationReportEvent" => "allocationReport",
        "BalanceUpdateEvent" => "balanceUpdate",
        "EventStreamTerminatedEvent" => "eventStreamTerminated",
        "ExecutionReportEvent" => "executionReport",
        "ExternalLockUpdateEvent" => "externalLockUpdate",
        "ListStatusEvent" => "listStatus",
        "OutboundAccountPositionEvent" => "outboundAccountPosition",
        "ServerShutdownEvent" => "serverShutdown",
        _ => return Ok(()),
    };
    let map = value.as_object_mut().ok_or(Error::Gap("SBE event shape"))?;
    map.insert("e".into(), Value::String(event.into()));
    if let Some(subscription) = map.remove("subscriptionId") {
        *value = serde_json::json!({"subscriptionId":subscription,"event":value.take()});
    }
    Ok(())
}
pub(crate) fn decode_api_value(bytes: &[u8]) -> Result<Value, Error> {
    decode_value(bytes, api_schema()?, false)
}
pub(crate) fn decode_fix_value(bytes: &[u8]) -> Result<Value, Error> {
    decode_value(bytes, fix_schema()?, true)
}
pub(crate) fn fix_node(template: u16) -> Result<&'static Node, Error> {
    fix_schema()?
        .messages
        .get(&template)
        .ok_or(Error::Configuration("FIX binary template"))
}

/// Decode production Spot API schema 3:4 into a native response model. All wire
/// decimals are projected as exact decimal strings before model deserialization;
/// timestamps retain the SBE schema's microseconds. No float is used.
///
/// # Errors
/// Refuses malformed/truncated records, unknown enums/schema/templates,
/// non-representable responses, numeric overflow, and native model mismatch.
pub fn decode_api<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, Error> {
    serde_json::from_value(decode_api_value(bytes)?)
        .map_err(|_| Error::Gap("SBE native API model mismatch"))
}
