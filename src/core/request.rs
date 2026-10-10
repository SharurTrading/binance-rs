// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{Cost, Error};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Security {
    Public,
    Key,
    Signed,
    /// No authoritative authentication contract: dispatch must refuse unsent.
    Unresolved,
}

#[derive(Clone, Copy)]
pub(crate) struct Operation {
    pub name: &'static str,
    pub path: &'static str,
    pub method: &'static str,
    pub security: Security,
    pub mutation: bool,
    pub weight: u64,
    /// Documented per-second request cap for the rare endpoint whose provider
    /// page annotates one (for example Wallet `withdrawHistory`); admission
    /// derives its bucket from this pinned fact instead of a local constant.
    pub requests_per_second: Option<u64>,
    /// Additional documented per-IP minute request cap, independent of UID weight.
    pub requests_per_minute: Option<u64>,
    pub success_weight: Option<u64>,
    pub partial: Option<fn(u16, &Value) -> Option<super::error::PartialOperation>>,
    pub definitive: fn(u16, &Value) -> bool,
    pub validate_time: fn(&BTreeMap<String, Value>, u64) -> Result<(), Error>,
}

pub(crate) trait Request: Serialize + Send + Sync {
    type Response: DeserializeOwned + Send + 'static;
    const OP: Operation;
    /// Only explicit provider no-data contracts permit a successful empty body.
    const EMPTY_RESPONSE: bool = false;
    fn validate(&self) -> Result<(), Error>;
    fn validate_authority(&self, _now: u64) -> Result<(), Error> {
        Ok(())
    }
    fn cost(&self) -> Result<Cost, Error>;
}

pub(crate) fn parameters<T: Serialize>(request: &T) -> Result<BTreeMap<String, Value>, Error> {
    let value = serde_json::to_value(request).map_err(|_| Error::Validation("request encoding"))?;
    let map = value
        .as_object()
        .ok_or(Error::Validation("request must be an object"))?;
    Ok(map
        .iter()
        .filter(|(_, v)| !v.is_null())
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect())
}

pub(crate) fn order_ids(p: &BTreeMap<String, Value>) -> BTreeMap<String, String> {
    fn collect(path: &str, value: &Value, ids: &mut BTreeMap<String, String>) {
        match value {
            Value::Object(object) => {
                for (key, value) in object {
                    let path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{path}.{key}")
                    };
                    collect(&path, value, ids);
                }
            }
            Value::Array(array) => {
                for (index, value) in array.iter().enumerate() {
                    collect(&format!("{path}[{index}]"), value, ids);
                }
            }
            Value::String(id) => {
                let field = path.rsplit('.').next().unwrap_or(path);
                if field.ends_with("ClientOrderId")
                    || field.ends_with("ClientAlgoId")
                    || field == "clientAlgoId"
                    || field == "withdrawOrderId"
                    || field == "clientId"
                    || field == "clientOrderId"
                    || field.starts_with("clientOrderIds[")
                    || field == "blockOrderMatchingKey"
                    || field == "quoteId"
                    || field == "orderId"
                    || field.starts_with("orderIds[")
                    || field.starts_with("origClientOrderIdList[")
                {
                    ids.insert(path.to_owned(), id.clone());
                }
            }
            Value::Number(id)
                if path
                    .rsplit('.')
                    .next()
                    .is_some_and(|field| field == "orderId" || field.starts_with("orderIds[")) =>
            {
                ids.insert(path.to_owned(), id.to_string());
            }
            _ => (),
        }
    }
    let mut ids = BTreeMap::new();
    for (key, value) in p {
        collect(key, value, &mut ids);
    }
    ids
}

pub(crate) fn text(value: &Value) -> Result<String, Error> {
    match value {
        Value::String(s) => Ok(s.clone()),
        _ => serde_json::to_string(value).map_err(|_| Error::Validation("parameter encoding")),
    }
}

pub(crate) fn encode(params: &BTreeMap<String, Value>) -> Result<String, Error> {
    let mut form = url::form_urlencoded::Serializer::new(String::new());
    for (name, value) in params {
        form.append_pair(name, &text(value)?);
    }
    Ok(form.finish())
}

pub(crate) fn validate_parameters(
    p: &BTreeMap<String, Value>,
    required: &[&str],
    enums: &[(&str, &[&str])],
    bounds: &[(&str, i64, i64)],
) -> Result<(), Error> {
    for name in required {
        if !p.contains_key(*name) {
            return Err(Error::Validation("required request parameter"));
        }
    }
    for (name, choices) in enums {
        if let Some(value) = p.get(*name) {
            let value = text(value)?;
            if !choices.contains(&value.as_str()) {
                return Err(Error::Validation("request enum"));
            }
        }
    }
    for (name, min, max) in bounds {
        if let Some(value) = p.get(*name) {
            let value = text(value)?
                .parse::<i64>()
                .map_err(|_| Error::Validation("integer parameter"))?;
            if value < *min || value > *max {
                return Err(Error::Validation("integer range"));
            }
        }
    }
    if let Some(value) = p.get("recvWindow") {
        let n = text(value)?
            .parse::<rust_decimal::Decimal>()
            .map_err(|_| Error::Validation("recvWindow"))?;
        if n <= rust_decimal::Decimal::ZERO
            || n > rust_decimal::Decimal::new(60_000, 0)
            || n.scale() > 3
        {
            return Err(Error::Validation("recvWindow"));
        }
    }
    for name in [
        "quantity",
        "price",
        "triggerPrice",
        "stopPrice",
        "activationPrice",
        "activatePrice",
        "callbackRate",
        "fromAmount",
        "toAmount",
        "amount",
    ] {
        if let Some(value) = p.get(name) {
            let value = text(value)?
                .parse::<rust_decimal::Decimal>()
                .map_err(|_| Error::Validation("decimal parameter"))?;
            if value <= rust_decimal::Decimal::ZERO {
                return Err(Error::Validation("positive financial magnitude"));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reconciliation_ids_include_nested_orders_and_cancellation_lists() {
        let p = BTreeMap::from([
            (
                "batchOrders".into(),
                serde_json::json!([{ "newClientOrderId":"a" }, { "origClientOrderId":"b" }]),
            ),
            (
                "origClientOrderIdList".into(),
                serde_json::json!(["c", "d"]),
            ),
            ("apiKey".into(), serde_json::json!("synthetic-secret")),
        ]);
        assert_eq!(
            order_ids(&p),
            BTreeMap::from([
                ("batchOrders[0].newClientOrderId".into(), "a".into()),
                ("batchOrders[1].origClientOrderId".into(), "b".into()),
                ("origClientOrderIdList[0]".into(), "c".into()),
                ("origClientOrderIdList[1]".into(), "d".into()),
            ])
        );
    }

    #[test]
    fn options_reconciliation_preserves_native_client_ids_and_cancel_lists() {
        let p = BTreeMap::from([
            ("clientOrderId".into(), serde_json::json!("options-caller")),
            (
                "clientOrderIds".into(),
                serde_json::json!(["first", "second"]),
            ),
            (
                "blockOrderMatchingKey".into(),
                serde_json::json!("matching-key"),
            ),
            (
                "orderIds".into(),
                serde_json::json!([9_007_199_254_740_993_i64, "0007"]),
            ),
        ]);
        assert_eq!(
            order_ids(&p),
            BTreeMap::from([
                ("clientOrderId".into(), "options-caller".into()),
                ("clientOrderIds[0]".into(), "first".into()),
                ("clientOrderIds[1]".into(), "second".into()),
                ("blockOrderMatchingKey".into(), "matching-key".into()),
                ("orderIds[0]".into(), "9007199254740993".into()),
                ("orderIds[1]".into(), "0007".into()),
            ])
        );
    }
}
