// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{
    Fields, Header, Role, Value,
    codec::{Dictionary, dictionary, validate_required, validate_value, wire_value},
};
use crate::core_trading::spot::sbe::schema::Node;
use crate::{Decimal, Error};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
};
use zeroize::Zeroizing;

/// Documented client application messages. Authentication/control traffic is
/// owned by the session driver, so callers cannot inject a second Logon.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum RequestKind {
    /// Place one native Spot order.
    NewOrderSingle,
    /// Place OCO/OTO/OTOCO/OPO/OPOCO with caller-owned leg/list IDs.
    NewOrderList,
    /// Cancel an order/list by native identity.
    CancelOrder,
    /// Cancel and place; both legs have independent outcomes.
    CancelReplace,
    /// Cancel every account order on a symbol.
    CancelAll,
    /// Reduce order quantity, preserving priority when the venue permits.
    AmendKeepPriority,
    /// Query current native quotas.
    Limits,
    /// Query native instrument metadata and filters.
    Instruments,
    /// Subscribe/unsubscribe native market data.
    MarketData,
}
impl RequestKind {
    pub(super) fn code(self) -> &'static str {
        match self {
            Self::NewOrderSingle => "D",
            Self::NewOrderList => "E",
            Self::CancelOrder => "F",
            Self::CancelReplace => "XCN",
            Self::CancelAll => "q",
            Self::AmendKeepPriority => "XAK",
            Self::Limits => "XLQ",
            Self::Instruments => "x",
            Self::MarketData => "V",
        }
    }
    pub(super) fn mutation(self) -> bool {
        matches!(
            self,
            Self::NewOrderSingle
                | Self::NewOrderList
                | Self::CancelOrder
                | Self::CancelReplace
                | Self::CancelAll
                | Self::AmendKeepPriority
        )
    }
}
/// Validated provider request. No method assigns or changes a caller order ID.
#[derive(Clone, Debug)]
pub struct Request {
    pub(super) kind: RequestKind,
    pub(super) fields: Fields,
}
/// Native dictionary builder. More than two optional settings remain explicit;
/// financial fields must use their typed Decimal variants.
#[derive(Clone, Debug)]
#[must_use = "build and validate the request before dispatch"]
pub struct RequestBuilder {
    kind: RequestKind,
    fields: Fields,
}
impl Request {
    /// Start a builder for the intended native session role.
    pub fn builder(role: Role, kind: RequestKind) -> RequestBuilder {
        RequestBuilder {
            kind,
            fields: Fields::new(role),
        }
    }
    /// Native message kind.
    #[must_use]
    pub fn kind(&self) -> RequestKind {
        self.kind
    }
    /// Borrow the exact validated fields.
    #[must_use]
    pub fn fields(&self) -> &Fields {
        &self.fields
    }
    /// Encode one checksum/body-length-correct FIX frame using a caller-supplied
    /// header. Authentication and send-time authority belong to the session driver.
    /// The returned bytes are redacted by ownership discipline and zeroized on drop.
    ///
    /// # Errors
    /// Refuses invalid header identities/timestamps or unrepresentable wire values.
    pub fn encode(&self, header: &Header) -> Result<Zeroizing<Vec<u8>>, Error> {
        encode(
            self.fields.role,
            self.kind.code(),
            &self.fields,
            header,
            None,
        )
    }
    /// Number of native order placements charged by this request.
    ///
    /// # Errors
    /// Refuses an invalid list group.
    pub fn order_count(&self) -> Result<u64, Error> {
        match self.kind {
            RequestKind::NewOrderSingle | RequestKind::CancelReplace => Ok(1),
            RequestKind::NewOrderList => match self.fields.get("NoOrders") {
                Some(Value::Group(entries)) => {
                    u64::try_from(entries.len()).map_err(|_| Error::Validation("FIX list count"))
                }
                _ => Err(Error::Validation("FIX list orders")),
            },
            _ => Ok(0),
        }
    }
    /// Caller identities by native field path, retained for outcome attribution.
    #[must_use]
    pub fn identities(&self) -> BTreeMap<String, String> {
        identities(&self.fields)
    }
}
impl RequestBuilder {
    /// Set a native field without normalizing assets, quantities or identities.
    ///
    /// # Errors
    /// Refuses unknown names, duplicate fields and wrong wire value types.
    pub fn field(mut self, name: &str, value: Value) -> Result<Self, Error> {
        self.fields = self.fields.with(name, value)?;
        Ok(self)
    }
    /// Validate required/conditional wire behavior before network admission.
    /// Price and trigger price retain their exact sign; the venue's symbol filters
    /// decide admissibility. Quantity, quote spend and iceberg size remain positive.
    ///
    /// # Errors
    /// Refuses role mismatch, missing caller identities, contradictory quantities,
    /// unsupported enum values, malformed list/trigger relationships and invalid money.
    pub fn build(self) -> Result<Request, Error> {
        if self.kind.mutation() && self.fields.role != Role::OrderEntry {
            return Err(Error::Validation("FIX execution requires order entry role"));
        }
        if matches!(
            self.kind,
            RequestKind::Instruments | RequestKind::MarketData
        ) && self.fields.role != Role::MarketData
        {
            return Err(Error::Validation("FIX market role required"));
        }
        let dict = dictionary(self.fields.role)?;
        let node = dict
            .messages
            .get(self.kind.code())
            .ok_or(Error::Validation("FIX request unavailable for role"))?;
        let rules = dict.rules(&node.children, true)?;
        validate_fields(dict, &rules, &self.fields)?;
        validate_contract(self.kind, &self.fields)?;
        let ids = identities(&self.fields);
        let unique: BTreeSet<_> = ids.values().collect();
        if unique.len() != ids.len() {
            return Err(Error::Validation(
                "FIX list/leg identities must be distinct",
            ));
        }
        Ok(Request {
            kind: self.kind,
            fields: self.fields,
        })
    }
}
pub(super) fn validate_fields(
    dict: &Dictionary,
    rules: &[Node],
    fields: &Fields,
) -> Result<(), Error> {
    validate_required(rules, fields)?;
    for field in &fields.values {
        let name = field
            .name
            .as_deref()
            .ok_or(Error::Validation("unknown outbound FIX tag"))?;
        let rule = rules
            .iter()
            .find(|r| r.name == name)
            .ok_or(Error::Validation("field does not belong to FIX request"))?;
        let meta = dict
            .fields
            .get(name)
            .ok_or(Error::Configuration("FIX field definition"))?;
        validate_value(meta, &field.value, true)?;
        if let Value::Group(entries) = &field.value {
            if rule.kind != "group" {
                return Err(Error::Validation("unexpected FIX group"));
            }
            let children = dict.rules(&rule.children, true)?;
            for entry in entries {
                validate_fields(dict, &children, entry)?;
            }
        }
    }
    Ok(())
}
fn code<'a>(fields: &'a Fields, name: &str) -> Option<&'a str> {
    match fields.get(name) {
        Some(Value::Code(c)) => Some(c),
        _ => None,
    }
}
fn positive(fields: &Fields, name: &str) -> Result<(), Error> {
    match fields.get(name) {
        Some(Value::Price(v) | Value::Quantity(v) | Value::Amount(v) | Value::Decimal(v))
            if *v > Decimal::ZERO =>
        {
            Ok(())
        }
        _ => Err(Error::Validation("FIX positive financial field required")),
    }
}
fn require(fields: &Fields, names: &[&str]) -> Result<(), Error> {
    if names.iter().any(|n| fields.get(n).is_none()) {
        return Err(Error::Validation("missing conditional FIX field"));
    }
    Ok(())
}
fn order(fields: &Fields) -> Result<(), Error> {
    let base = fields.get("OrderQty").is_some();
    let quote = fields.get("CashOrderQty").is_some();
    if base == quote {
        return Err(Error::Validation(
            "FIX base quantity xor quote spend required",
        ));
    }
    positive(fields, if base { "OrderQty" } else { "CashOrderQty" })?;
    if fields.get("MaxFloor").is_some() {
        positive(fields, "MaxFloor")?;
    }
    match code(fields, "OrdType") {
        Some("1") => (),
        Some("2") => {
            if !base {
                return Err(Error::Validation("FIX limit quantity uses base asset"));
            }
            require(fields, &["Price"])?;
            if code(fields, "ExecInst") != Some("6") {
                require(fields, &["TimeInForce"])?;
            }
        }
        Some("3" | "4") => {
            if !base {
                return Err(Error::Validation("FIX contingent quantity uses base asset"));
            }
            if fields.get("TriggerPrice").is_none()
                && fields.get("TriggerTrailingDeltaBips").is_none()
            {
                return Err(Error::Validation(
                    "FIX trigger price or trailing delta required",
                ));
            }
            if code(fields, "TriggerType") != Some("4")
                || code(fields, "TriggerAction") != Some("1")
                || code(fields, "TriggerPriceType") != Some("2")
            {
                return Err(Error::Validation("FIX trigger constants required"));
            }
            if fields.get("TriggerPrice").is_some() {
                require(fields, &["TriggerPriceDirection"])?;
            }
            if code(fields, "OrdType") == Some("4") {
                require(fields, &["Price", "TimeInForce"])?;
            }
        }
        Some("P") => {
            if !base {
                return Err(Error::Validation("FIX pegged quantity uses base asset"));
            }
            require(fields, &["PegPriceType", "PegMoveType"])?;
        }
        _ => return Err(Error::Validation("unsupported FIX order type")),
    }
    if let Some(Value::Integer(v)) = fields.get("TriggerTrailingDeltaBips")
        && *v <= 0
    {
        return Err(Error::Validation("FIX trailing delta must be positive"));
    }
    if let Some(Value::Integer(v)) = fields.get("TargetStrategy")
        && *v < 1_000_000
    {
        return Err(Error::Validation("FIX target strategy minimum"));
    }
    Ok(())
}
fn validate_contract(kind: RequestKind, f: &Fields) -> Result<(), Error> {
    match kind {
        RequestKind::NewOrderSingle => order(f)?,
        RequestKind::CancelReplace => {
            order(f)?;
            require(f, &["CancelClOrdID"])?;
            if f.get("OrderID").is_none() && f.get("OrigClOrdID").is_none() {
                return Err(Error::Validation(
                    "FIX cancel/replace original identity required",
                ));
            }
        }
        RequestKind::CancelOrder
            if ["OrderID", "OrigClOrdID", "ListID", "OrigClListID"]
                .iter()
                .all(|n| f.get(n).is_none()) =>
        {
            return Err(Error::Validation("FIX cancel original identity required"));
        }
        RequestKind::AmendKeepPriority => {
            positive(f, "OrderQty")?;
            if f.get("OrderID").is_none() && f.get("OrigClOrdID").is_none() {
                return Err(Error::Validation("FIX amend original identity required"));
            }
        }
        RequestKind::NewOrderList => {
            let Some(Value::Group(entries)) = f.get("NoOrders") else {
                return Err(Error::Validation("FIX list orders required"));
            };
            if !matches!(entries.len(), 2 | 3)
                || (code(f, "ContingencyType") == Some("1") && entries.len() != 2)
            {
                return Err(Error::Validation("documented FIX list size"));
            }
            for e in entries {
                order(e)?;
            }
            validate_list(entries)?;
        }
        RequestKind::MarketData => {
            if code(f, "SubscriptionRequestType") == Some("1") {
                require(f, &["NoRelatedSym", "NoMDEntryTypes"])?;
            }
            for name in ["NoRelatedSym", "NoMDEntryTypes"] {
                if let Some(Value::Group(entries)) = f.get(name)
                    && entries.is_empty()
                {
                    return Err(Error::Validation("empty FIX market group"));
                }
            }
        }
        _ => (),
    }
    Ok(())
}
fn validate_list(entries: &[Fields]) -> Result<(), Error> {
    for (index, e) in entries.iter().enumerate() {
        if let Some(Value::Group(triggers)) = e.get("NoListTriggeringInstructions") {
            for t in triggers {
                let Some(Value::Integer(other)) = t.get("ListTriggerTriggerIndex") else {
                    return Err(Error::Validation("FIX list trigger index"));
                };
                let other = usize::try_from(*other)
                    .map_err(|_| Error::Validation("FIX list trigger index"))?;
                if other >= entries.len() || other == index {
                    return Err(Error::Validation("FIX list trigger references another leg"));
                }
            }
        }
    }
    Ok(())
}
pub(super) fn identities(fields: &Fields) -> BTreeMap<String, String> {
    fn visit(fields: &Fields, prefix: &str, out: &mut BTreeMap<String, String>) {
        for field in &fields.values {
            let Some(name) = &field.name else {
                continue;
            };
            let path = format!("{prefix}{name}");
            if matches!(
                name.as_str(),
                "ClOrdID" | "CancelClOrdID" | "ClListID" | "MDReqID" | "ReqID" | "InstrumentReqID"
            ) && let Ok(text) = wire_value(&field.value)
            {
                out.insert(path.clone(), text);
            }
            if let Value::Group(entries) = &field.value {
                for (i, e) in entries.iter().enumerate() {
                    visit(e, &format!("{path}[{i}]."), out);
                }
            }
        }
    }
    let mut out = BTreeMap::new();
    visit(fields, "", &mut out);
    out
}
fn fields_wire(
    dict: &Dictionary,
    rules: &[Node],
    fields: &Fields,
    output: &mut Zeroizing<String>,
) -> Result<(), Error> {
    for rule in rules {
        let Some(value) = fields.get(&rule.name) else {
            continue;
        };
        let tag = dict
            .fields
            .get(&rule.name)
            .and_then(|m| m.attr("number"))
            .ok_or(Error::Configuration("FIX field tag"))?;
        let text = Zeroizing::new(wire_value(value)?);
        output.push_str(tag);
        output.push('=');
        output.push_str(&text);
        output.push('\x01');
        if let Value::Group(entries) = value {
            let nested = dict.rules(&rule.children, true)?;
            for e in entries {
                fields_wire(dict, &nested, e, output)?;
            }
        }
    }
    Ok(())
}
pub(super) fn encode(
    role: Role,
    kind: &str,
    fields: &Fields,
    header: &Header,
    recv_window: Option<Decimal>,
) -> Result<Zeroizing<Vec<u8>>, Error> {
    let sender = header
        .sender
        .as_ref()
        .ok_or(Error::Validation("outbound FIX sender required"))?;
    let target = header
        .target
        .as_ref()
        .ok_or(Error::Validation("outbound FIX target required"))?;
    if target.as_str() != "SPOT" {
        return Err(Error::Validation("FIX target must be SPOT"));
    }
    let dict = dictionary(role)?;
    let node = dict
        .messages
        .get(kind)
        .ok_or(Error::Validation("unsupported outbound FIX message"))?;
    let rules = dict.rules(&node.children, true)?;
    validate_fields(dict, &rules, fields)?;
    let mut body = Zeroizing::new(format!(
        "35={kind}\x0134={}\x0149={}\x0152={}\x0156={}\x01",
        header.sequence,
        sender.as_str(),
        header.sending_time.wire()?,
        target.as_str()
    ));
    if let Some(window) = recv_window {
        write!(body, "25000={window}\x01")
            .map_err(|_| Error::Validation("FIX receive window encoding"))?;
    }
    fields_wire(dict, &rules, fields, &mut body)?;
    let mut output = Zeroizing::new(format!("8=FIX.4.4\x019={}\x01", body.len()).into_bytes());
    output.extend_from_slice(body.as_bytes());
    let checksum = output.iter().fold(0_u8, |sum, b| sum.wrapping_add(*b));
    output.extend_from_slice(format!("10={checksum:03}\x01").as_bytes());
    Ok(output)
}
