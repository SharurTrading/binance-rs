// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{ClientId, CompId, Timestamp, WireId};
use crate::spot::sbe::schema::Node;
use crate::{Asset, Decimal, Error, Outcome, SensitiveString, Symbol};
use serde::Deserialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    sync::OnceLock,
};

/// Provider session role, with separate messages and limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[non_exhaustive]
pub enum Role {
    /// Order entry and execution delivery.
    OrderEntry,
    /// Execution delivery, delayed one second at the venue.
    DropCopy,
    /// Market subscriptions and instrument metadata.
    MarketData,
}
/// Native FIX value; financial meanings remain separate and unrounded.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Value {
    /// Price in quote asset units.
    Price(Decimal),
    /// Quantity in the field's documented asset units.
    Quantity(Decimal),
    /// Native amount (including commission).
    Amount(Decimal),
    /// Other exact fixed-point provider value.
    Decimal(Decimal),
    /// Signed provider integer.
    Integer(i64),
    /// Unsigned count or sequence.
    Unsigned(u64),
    /// Native FIX SBE receive-window duration; ASCII FIX uses decimal milliseconds.
    DurationMicros(u64),
    /// Native boolean.
    Boolean(bool),
    /// Provider UTC timestamp.
    Timestamp(Timestamp),
    /// Caller-owned client/list/subscription ID.
    ClientId(ClientId),
    /// Provider-assigned opaque identity.
    Id(WireId),
    /// Native symbol; assets come from instrument definitions.
    Symbol(Symbol),
    /// Native asset, including commission currency.
    Asset(Asset),
    /// Native enumeration wire code, including future response codes.
    Code(String),
    /// Free-form text is redacted and zeroized.
    Text(SensitiveString),
    /// Repeating group entries in source order.
    Group(Vec<Fields>),
}
/// A native tag and decoded value, preserving original field order.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Field {
    /// Exact native tag number, including future tags.
    pub tag: u32,
    /// Official field name, absent for unknown tags.
    pub name: Option<String>,
    /// Typed value. Unknown future field text remains redacted.
    pub value: Value,
}
/// Validated fields; callers cannot bypass dictionary/type validation.
#[derive(Clone, Debug, PartialEq)]
pub struct Fields {
    pub(super) role: Role,
    pub(super) values: Vec<Field>,
}
impl Fields {
    /// Start an empty set for the intended native session role.
    #[must_use]
    pub fn new(role: Role) -> Self {
        Self {
            role,
            values: Vec::new(),
        }
    }
    /// Set a known native field. Groups own their entries and preserve order.
    ///
    /// # Errors
    /// Refuses unknown names, duplicate tags, wrong types, invalid enums and
    /// characters that would alter framing. Required fields are checked by `build`.
    pub fn with(mut self, name: &str, value: Value) -> Result<Self, Error> {
        let dict = dictionary(self.role)?;
        let meta = dict
            .fields
            .get(name)
            .ok_or(Error::Validation("unknown FIX field"))?;
        let tag = tag(meta)?;
        if self.values.iter().any(|f| f.tag == tag) {
            return Err(Error::Validation("duplicate FIX field"));
        }
        validate_value(meta, &value, true)?;
        self.values.push(Field {
            tag,
            name: Some(name.into()),
            value,
        });
        Ok(self)
    }
    /// Inspect a named native value without converting quantities or assets.
    #[must_use]
    pub fn field_value(&self, name: &str) -> Option<&Value> {
        self.values
            .iter()
            .find(|f| f.name.as_deref() == Some(name))
            .map(|f| &f.value)
    }
    /// Alias for response/group lookup.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Value> {
        self.field_value(name)
    }
    /// Lookup a named native response field.
    #[must_use]
    pub fn field(&self, name: &str) -> Option<&Value> {
        self.get(name)
    }
    /// All accepted fields in original source order.
    #[must_use]
    pub fn fields(&self) -> &[Field] {
        &self.values
    }
}
/// Native header; sequence numbers roll over to zero at the documented maximum.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Header {
    /// Source session component.
    pub sender: Option<CompId>,
    /// Destination session component.
    pub target: Option<CompId>,
    /// Exact native sequence.
    pub sequence: u32,
    /// Sending timestamp in microseconds.
    pub sending_time: Timestamp,
}
impl Header {
    /// Build a native client header targeting the Spot FIX service.
    ///
    /// # Errors
    /// Returns an error if the fixed provider target cannot be validated.
    pub fn request(sender: CompId, sequence: u32, sending_time: Timestamp) -> Result<Self, Error> {
        Ok(Self {
            sender: Some(sender),
            target: Some(CompId::new("SPOT")?),
            sequence,
            sending_time,
        })
    }
}
/// Native decoded response and its exact header.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Message {
    /// Provider session header.
    pub header: Header,
    /// Native message kind (35), including future kinds.
    pub kind: WireId,
    /// Source-ordered fields. Free-form unknown evidence is redacted.
    pub body: Fields,
}
impl Message {
    /// Lookup a native field.
    #[must_use]
    pub fn field(&self, name: &str) -> Option<&Value> {
        self.body.get(name)
    }
    /// Execution evidence from this record only. Partial cancel/replace operations
    /// still require both legs; no single receipt completes the other leg.
    #[must_use]
    pub fn outcome(&self) -> Outcome {
        let code = match self.field("ErrorCode") {
            Some(Value::Integer(v)) => Some(*v),
            _ => None,
        };
        if code.is_some_and(|v| !crate::spot::validation::definitive_code(v)) {
            return Outcome::Unknown;
        }
        if self.kind.as_str() == "8" {
            let status = match self.field("OrdStatus") {
                Some(Value::Code(s)) => s.as_str(),
                _ => return Outcome::Unknown,
            };
            let execution = match self.field("ExecType") {
                Some(Value::Code(s)) => s.as_str(),
                _ => return Outcome::Unknown,
            };
            return match (execution, status) {
                ("0", "0") | ("4", "4") | ("5", "0" | "1") | ("F", "1" | "2") => Outcome::Accepted,
                ("8", "8") => Outcome::Rejected,
                _ => Outcome::Unknown,
            };
        }
        match (self.kind.as_str(), self.field("ExecType")) {
            ("8", Some(Value::Code(code))) if matches!(code.as_str(), "0" | "4" | "5" | "F") => {
                Outcome::Accepted
            }
            ("8", Some(Value::Code(code))) if code == "8" => Outcome::Rejected,
            _ => Outcome::Unknown,
        }
    }
}
#[derive(Deserialize)]
struct Snapshot {
    schema: Node,
}
pub(super) struct Dictionary {
    pub fields: BTreeMap<String, Node>,
    pub by_tag: BTreeMap<u32, String>,
    pub messages: BTreeMap<String, Node>,
    components: BTreeMap<String, Node>,
}
impl Dictionary {
    fn load(raw: &str) -> Result<Self, Error> {
        let root = serde_json::from_str::<Snapshot>(raw)
            .map_err(|_| Error::Configuration("FIX dictionary"))?
            .schema;
        let mut d = Self {
            fields: BTreeMap::new(),
            by_tag: BTreeMap::new(),
            messages: BTreeMap::new(),
            components: BTreeMap::new(),
        };
        for n in root.children {
            for child in n.children {
                match n.kind.as_str() {
                    "fields" => {
                        d.by_tag.insert(tag(&child)?, child.name.clone());
                        d.fields.insert(child.name.clone(), child);
                    }
                    "messages" => {
                        d.messages.insert(
                            child
                                .attr("msgtype")
                                .ok_or(Error::Configuration("FIX kind"))?
                                .into(),
                            child,
                        );
                    }
                    "components" => {
                        d.components.insert(child.name.clone(), child);
                    }
                    _ => (),
                }
            }
        }
        Ok(d)
    }
    pub(super) fn rules(&self, nodes: &[Node], required: bool) -> Result<Vec<Node>, Error> {
        let mut result = Vec::new();
        for node in nodes {
            let required = required && node.attr("required") == Some("Y");
            if node.kind == "component" {
                let c = self
                    .components
                    .get(&node.name)
                    .ok_or(Error::Configuration("FIX component"))?;
                result.extend(self.rules(&c.children, required)?);
            } else {
                let mut node = node.clone();
                node.attrs
                    .insert("required".into(), if required { "Y" } else { "N" }.into());
                result.push(node);
            }
        }
        Ok(result)
    }
}
static OE: OnceLock<Result<Dictionary, Error>> = OnceLock::new();
static MD: OnceLock<Result<Dictionary, Error>> = OnceLock::new();
pub(super) fn dictionary(role: Role) -> Result<&'static Dictionary, Error> {
    let (owner, raw) = if role == Role::MarketData {
        (&MD, include_str!("../../../schema/spot/fix-md.json"))
    } else {
        (&OE, include_str!("../../../schema/spot/fix-oe.json"))
    };
    owner
        .get_or_init(|| Dictionary::load(raw))
        .as_ref()
        .map_err(|_| Error::Configuration("FIX dictionary snapshot"))
}
fn tag(n: &Node) -> Result<u32, Error> {
    n.attr("number")
        .and_then(|v| v.parse().ok())
        .ok_or(Error::Configuration("FIX tag"))
}
fn failure(tag: Option<u32>, offset: usize, reason: &'static str) -> Error {
    Error::FixDecode {
        tag,
        offset,
        reason,
    }
}

pub(super) fn scalar(meta: &Node, text: &str) -> Result<Value, Error> {
    let invalid = || failure(tag(meta).ok(), 0, "malformed native FIX value");
    let number = || {
        let digits = text
            .strip_prefix('-')
            .or_else(|| text.strip_prefix('+'))
            .unwrap_or(text);
        if digits.is_empty()
            || digits.bytes().filter(|b| *b == b'.').count() > 1
            || !digits.bytes().all(|b| b.is_ascii_digit() || b == b'.')
        {
            return Err(invalid());
        }
        Decimal::from_str_exact(text).map_err(|_| invalid())
    };
    if matches!(meta.name.as_str(), "MiscFeeAmt" | "CashOrderQty") {
        return number().map(Value::Amount);
    }
    if matches!(
        meta.name.as_str(),
        "ClOrdID" | "OrigClOrdID" | "MDReqID" | "ClListID" | "OrigClListID" | "CancelClOrdID"
    ) {
        return ClientId::new(text)
            .map(Value::ClientId)
            .map_err(|_| invalid());
    }
    if meta.name == "Symbol" || meta.name == "CounterSymbol" {
        return Symbol::new(text).map(Value::Symbol).map_err(|_| invalid());
    }
    if meta.attr("type") == Some("CURRENCY") || meta.name == "MiscFeeCurr" {
        return Asset::new(text).map(Value::Asset).map_err(|_| invalid());
    }
    if meta.attr("type") == Some("BOOLEAN") {
        return match text {
            "Y" => Ok(Value::Boolean(true)),
            "N" => Ok(Value::Boolean(false)),
            _ => Err(invalid()),
        };
    }
    if !meta.children.is_empty() {
        if meta.attr("type") == Some("CHAR") && (text.len() != 1 || !text.is_ascii()) {
            return Err(invalid());
        }
        if meta.attr("type") == Some("INT") && text.parse::<i64>().is_err() {
            return Err(invalid());
        }
        return Ok(Value::Code(text.into()));
    }
    match meta.attr("type").unwrap_or("STRING") {
        "PRICE" => number().map(Value::Price),
        "QTY" => number().map(Value::Quantity),
        "AMT" => number().map(Value::Amount),
        "FLOAT" => number().map(Value::Decimal),
        "INT" => text.parse().map(Value::Integer).map_err(|_| invalid()),
        "LENGTH" | "SEQNUM" | "NUMINGROUP" => {
            text.parse().map(Value::Unsigned).map_err(|_| invalid())
        }
        "BOOLEAN" => match text {
            "Y" => Ok(Value::Boolean(true)),
            "N" => Ok(Value::Boolean(false)),
            _ => Err(invalid()),
        },
        "UTCTIMESTAMP" => Timestamp::parse(text)
            .map(Value::Timestamp)
            .map_err(|_| invalid()),
        _ if meta.name.ends_with("ID")
            && !matches!(meta.name.as_str(), "SenderCompID" | "TargetCompID") =>
        {
            WireId::new(text).map(Value::Id).map_err(|_| invalid())
        }
        _ => Ok(Value::Text(SensitiveString::new(text))),
    }
}
pub(super) fn validate_value(meta: &Node, value: &Value, outbound: bool) -> Result<(), Error> {
    if matches!(value, Value::Group(_)) && meta.attr("type") == Some("NUMINGROUP") {
        return Ok(());
    }
    let text = zeroize::Zeroizing::new(wire_value(value)?);
    if text.contains('\x01') || text.chars().any(char::is_control) {
        return Err(Error::Validation("FIX field framing"));
    }
    let parsed = scalar(meta, &text)?;
    if std::mem::discriminant(&parsed) != std::mem::discriminant(value) {
        return Err(Error::Validation("FIX field value type"));
    }
    if outbound
        && !meta.children.is_empty()
        && !meta
            .children
            .iter()
            .any(|v| v.attr("enum") == Some(text.as_str()))
    {
        return Err(Error::Validation("FIX enum value"));
    }
    Ok(())
}
pub(super) fn wire_value(v: &Value) -> Result<String, Error> {
    match v {
        Value::Price(v) | Value::Quantity(v) | Value::Amount(v) | Value::Decimal(v) => {
            Ok(v.to_string())
        }
        Value::Integer(v) => Ok(v.to_string()),
        Value::Unsigned(v) | Value::DurationMicros(v) => Ok(v.to_string()),
        Value::Boolean(v) => Ok(if *v { "Y" } else { "N" }.into()),
        Value::Timestamp(v) => v.wire(),
        Value::ClientId(v) => Ok(v.as_str().into()),
        Value::Id(v) => Ok(v.as_str().into()),
        Value::Symbol(v) => Ok(v.as_str().into()),
        Value::Asset(v) => Ok(v.as_str().into()),
        Value::Code(v) => Ok(v.clone()),
        Value::Text(v) => Ok(v.as_str().into()),
        Value::Group(v) => Ok(v.len().to_string()),
    }
}
struct Token<'a> {
    tag: u32,
    text: &'a str,
    offset: usize,
}
fn tokens(bytes: &[u8]) -> Result<Vec<Token<'_>>, Error> {
    let mut output = Vec::new();
    let mut offset = 0;
    for raw in bytes.split(|b| *b == 1) {
        if raw.is_empty() {
            continue;
        }
        let text = std::str::from_utf8(raw).map_err(|_| failure(None, offset, "FIX UTF-8"))?;
        let (t, text) = text
            .split_once('=')
            .ok_or_else(|| failure(None, offset, "FIX field separator"))?;
        if t.is_empty() || !t.bytes().all(|b| b.is_ascii_digit()) {
            return Err(failure(None, offset, "FIX tag digits"));
        }
        let tag = t
            .parse()
            .map_err(|_| failure(None, offset, "FIX tag digits"))?;
        output.push(Token { tag, text, offset });
        offset += raw.len() + 1;
    }
    Ok(output)
}
/// Determine complete frame length without a fixed buffer capacity.
pub(super) fn frame_length(bytes: &[u8]) -> Result<Option<usize>, Error> {
    let prefix = b"8=FIX.4.4\x019=";
    let compare = bytes.len().min(prefix.len());
    if bytes[..compare] != prefix[..compare] {
        return Err(failure(None, 0, "FIX begin/header order"));
    }
    if bytes.len() < prefix.len() {
        return Ok(None);
    }
    let Some(end) = bytes[prefix.len()..]
        .iter()
        .position(|b| *b == 1)
        .map(|n| n + prefix.len())
    else {
        return Ok(None);
    };
    let count = std::str::from_utf8(&bytes[prefix.len()..end])
        .ok()
        .filter(|v| !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()))
        .and_then(|v| v.parse::<usize>().ok())
        .ok_or_else(|| failure(Some(9), prefix.len(), "FIX body length"))?;
    let total = end
        .checked_add(1)
        .and_then(|v| v.checked_add(count))
        .and_then(|v| v.checked_add(7))
        .ok_or_else(|| failure(Some(9), end, "FIX frame overflow"))?;
    Ok(Some(total))
}
/// Decode exactly one FIX 4.4 frame using the role's official dictionary.
///
/// # Errors
/// Refuses incorrect body length/checksum/header order, duplicates, malformed
/// financial fields/groups, and missing required evidence. Unknown future tags
/// remain redacted fields; unknown outcome codes never become success.
pub fn decode(role: Role, bytes: &[u8]) -> Result<Message, Error> {
    let total =
        frame_length(bytes)?.ok_or_else(|| failure(None, bytes.len(), "truncated FIX header"))?;
    if bytes.len() != total || total < 7 {
        return Err(failure(Some(9), bytes.len(), "FIX frame length mismatch"));
    }
    let trailer = &bytes[total - 7..];
    let checksum = std::str::from_utf8(&trailer[3..6])
        .ok()
        .and_then(|v| v.parse::<u8>().ok());
    if !trailer.starts_with(b"10=")
        || trailer[6] != 1
        || checksum
            != Some(
                bytes[..total - 7]
                    .iter()
                    .fold(0_u8, |sum, b| sum.wrapping_add(*b)),
            )
    {
        return Err(failure(Some(10), total - 7, "FIX checksum"));
    }
    let fields = tokens(bytes)?;
    if fields.get(2).map(|f| f.tag) != Some(35) {
        return Err(failure(Some(35), 0, "FIX message type order"));
    }
    let mut header = BTreeMap::new();
    let mut body = Vec::new();
    for t in fields {
        if matches!(t.tag, 8 | 9 | 10 | 35 | 49 | 56 | 34 | 52 | 25000) {
            if header.insert(t.tag, t.text).is_some() {
                return Err(failure(Some(t.tag), t.offset, "duplicate FIX header"));
            }
        } else {
            body.push(t);
        }
    }
    let h = |tag| {
        header
            .get(&tag)
            .copied()
            .ok_or_else(|| failure(Some(tag), 0, "missing FIX header"))
    };
    let kind = WireId::new(h(35)?)?;
    if !h(34)?.bytes().all(|b| b.is_ascii_digit()) {
        return Err(failure(Some(34), 0, "FIX sequence digits"));
    }
    let header = Header {
        sender: Some(CompId::new(h(49)?)?),
        target: Some(CompId::new(h(56)?)?),
        sequence: h(34)?
            .parse()
            .map_err(|_| failure(Some(34), 0, "FIX sequence range"))?,
        sending_time: Timestamp::parse(h(52)?)?,
    };
    let dict = dictionary(role)?;
    let rules = dict
        .messages
        .get(kind.as_str())
        .map(|m| dict.rules(&m.children, true))
        .transpose()?
        .unwrap_or_default();
    let mut cursor = 0;
    let body = parse_fields(role, dict, &rules, &body, &mut cursor, None)?;
    validate_required(&rules, &body)?;
    Ok(Message { header, kind, body })
}
fn parse_fields(
    role: Role,
    dict: &Dictionary,
    rules: &[Node],
    tokens: &[Token<'_>],
    cursor: &mut usize,
    delimiter: Option<u32>,
) -> Result<Fields, Error> {
    let allowed: BTreeSet<_> = rules
        .iter()
        .filter_map(|r| dict.fields.get(&r.name).and_then(|m| tag(m).ok()))
        .collect();
    let mut fields = Fields::new(role);
    while let Some(t) = tokens.get(*cursor) {
        if !fields.values.is_empty()
            && (delimiter == Some(t.tag)
                || (delimiter.is_some()
                    && dict.by_tag.contains_key(&t.tag)
                    && !allowed.contains(&t.tag)))
        {
            break;
        }
        if fields.values.iter().any(|f| f.tag == t.tag) {
            return Err(failure(Some(t.tag), t.offset, "duplicate FIX field"));
        }
        *cursor += 1;
        let name = dict.by_tag.get(&t.tag).cloned();
        let meta = name.as_ref().and_then(|n| dict.fields.get(n));
        let group = rules
            .iter()
            .find(|r| r.kind == "group" && Some(&r.name) == name.as_ref());
        let value = if let Some(group) = group {
            let count = t
                .text
                .parse::<usize>()
                .map_err(|_| failure(Some(t.tag), t.offset, "FIX group count"))?;
            if count > tokens.len().saturating_sub(*cursor) {
                return Err(failure(Some(t.tag), t.offset, "truncated FIX group"));
            }
            let children = dict.rules(&group.children, true)?;
            let first = children
                .first()
                .and_then(|n| dict.fields.get(&n.name))
                .map(tag)
                .transpose()?
                .ok_or(Error::Configuration("FIX group delimiter"))?;
            let mut entries = Vec::new();
            for _ in 0..count {
                if tokens.get(*cursor).map(|t| t.tag) != Some(first) {
                    return Err(failure(
                        Some(first),
                        t.offset,
                        "missing FIX group delimiter",
                    ));
                }
                let entry = parse_fields(role, dict, &children, tokens, cursor, Some(first))?;
                validate_required(&children, &entry)?;
                entries.push(entry);
            }
            Value::Group(entries)
        } else if let Some(meta) = meta {
            scalar(meta, t.text)?
        } else {
            Value::Text(SensitiveString::new(t.text))
        };
        fields.values.push(Field {
            tag: t.tag,
            name,
            value,
        });
    }
    Ok(fields)
}
pub(super) fn validate_required(rules: &[Node], fields: &Fields) -> Result<(), Error> {
    for r in rules {
        if r.attr("required") == Some("Y") && fields.get(&r.name).is_none() {
            return Err(failure(None, 0, "missing required FIX field"));
        }
    }
    Ok(())
}
/// Decode one little-endian SOFH-framed FIX SBE schema 1:1 response.
///
/// # Errors
/// Refuses wrong SOFH length/encoding, unknown schema/templates, truncation,
/// malformed native field values and financial overflow. No checksum is invented.
pub fn decode_sbe(role: Role, bytes: &[u8]) -> Result<Message, Error> {
    let prefix: [u8; 6] = bytes
        .get(..6)
        .ok_or_else(|| failure(None, 0, "truncated SOFH"))?
        .try_into()
        .map_err(|_| failure(None, 0, "SOFH width"))?;
    let size = u32::from_le_bytes([prefix[0], prefix[1], prefix[2], prefix[3]]);
    if usize::try_from(size).ok() != Some(bytes.len())
        || u16::from_le_bytes([prefix[4], prefix[5]]) != 0xeb50
    {
        return Err(failure(None, 0, "SOFH length/encoding"));
    }
    let mut value = BinaryBuffer(crate::spot::sbe::schema::decode_fix_value(&bytes[6..])?);
    let map = value
        .0
        .as_object_mut()
        .ok_or_else(|| failure(None, 6, "FIX SBE body"))?;
    let sequence = map
        .remove("MsgSeqNum")
        .and_then(|v| v.as_u64())
        .and_then(|v| u32::try_from(v).ok())
        .ok_or_else(|| failure(Some(34), 6, "FIX SBE sequence"))?;
    let sending_time = Timestamp::from_micros(
        map.remove("SendingTime")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| failure(Some(52), 6, "FIX SBE sending time"))?,
    )?;
    let kind = WireId::new(
        map.remove("MsgType")
            .and_then(|v| v.as_str().map(str::to_owned))
            .ok_or_else(|| failure(Some(35), 6, "FIX SBE message type"))?,
    )?;
    let template = u16::from_le_bytes(
        bytes
            .get(8..10)
            .ok_or_else(|| failure(None, 8, "FIX SBE template"))?
            .try_into()
            .map_err(|_| failure(None, 8, "FIX SBE template width"))?,
    );
    let node = crate::spot::sbe::schema::fix_node(template)?;
    let body = project_sbe(role, &node.children, map)?;
    // SBE headers omit component IDs. Their authority comes from the authenticated
    // session; standalone decoding explicitly records this omission as None later.
    let header = Header {
        sender: None,
        target: None,
        sequence,
        sending_time,
    };
    Ok(Message { header, kind, body })
}
fn project_sbe(
    role: Role,
    nodes: &[Node],
    map: &serde_json::Map<String, serde_json::Value>,
) -> Result<Fields, Error> {
    let dict = dictionary(role)?;
    let mut fields = Fields::new(role);
    for node in nodes {
        let name = &node.name;
        let Some(value) = map.get(name) else {
            continue;
        };
        if value.is_null() {
            continue;
        }
        let native_tag = node
            .attr("id")
            .and_then(|id| id.parse::<u32>().ok())
            .ok_or(Error::Configuration("FIX SBE field tag"))?;
        let meta = dict
            .by_tag
            .get(&native_tag)
            .and_then(|name| dict.fields.get(name));
        let decoded = if let Some(entries) = value.as_array() {
            Value::Group(
                entries
                    .iter()
                    .map(|entry| {
                        project_sbe(
                            role,
                            &node.children,
                            entry
                                .as_object()
                                .ok_or_else(|| failure(None, 0, "FIX SBE group entry"))?,
                        )
                    })
                    .collect::<Result<_, Error>>()?,
            )
        } else if node.attr("type") == Some("utcTimestampUs") {
            Value::Timestamp(Timestamp::from_micros(
                value
                    .as_i64()
                    .ok_or_else(|| failure(Some(native_tag), 0, "FIX SBE timestamp"))?,
            )?)
        } else if node.attr("type") == Some("durationUs") {
            Value::DurationMicros(
                value
                    .as_u64()
                    .ok_or_else(|| failure(Some(native_tag), 0, "FIX SBE duration"))?,
            )
        } else if let Some(meta) = meta {
            if let Some(boolean) = value.as_bool() {
                let text = if meta.attr("type") == Some("BOOLEAN") {
                    if boolean { "Y" } else { "N" }
                } else if boolean {
                    "1"
                } else {
                    "0"
                };
                scalar(meta, text)?
            } else {
                let text = zeroize::Zeroizing::new(
                    value
                        .as_str()
                        .map_or_else(|| value.to_string(), str::to_owned),
                );
                scalar(meta, &text)?
            }
        } else if node.attr("exponent").is_some() {
            Value::Decimal(
                Decimal::from_str_exact(
                    value
                        .as_str()
                        .ok_or_else(|| failure(Some(native_tag), 0, "FIX SBE decimal text"))?,
                )
                .map_err(|_| failure(Some(native_tag), 0, "FIX SBE financial overflow"))?,
            )
        } else {
            Value::Text(SensitiveString::new(
                value
                    .as_str()
                    .map_or_else(|| value.to_string(), str::to_owned),
            ))
        };
        fields.values.push(Field {
            tag: native_tag,
            name: Some(meta.map_or(name.as_str(), |m| m.name.as_str()).into()),
            value: decoded,
        });
    }
    Ok(fields)
}

// FIX binary Logon contains API key/signature fields. Zeroize the projection as
// well as the raw framing buffer, including early errors while projecting it.
struct BinaryBuffer(serde_json::Value);
impl Drop for BinaryBuffer {
    fn drop(&mut self) {
        fn clear(value: &mut serde_json::Value) {
            use zeroize::Zeroize as _;
            match value {
                serde_json::Value::String(text) => text.zeroize(),
                serde_json::Value::Array(entries) => entries.iter_mut().for_each(clear),
                serde_json::Value::Object(entries) => entries.values_mut().for_each(clear),
                _ => (),
            }
        }
        clear(&mut self.0);
    }
}
