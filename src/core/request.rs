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
}

#[derive(Clone, Copy)]
pub(crate) struct Operation {
    pub name: &'static str,
    pub path: &'static str,
    pub method: &'static str,
    pub security: Security,
    pub mutation: bool,
    pub weight: u64,
    pub validate_time: fn(&BTreeMap<String, Value>, u64) -> Result<(), Error>,
}

pub(crate) trait Request: Serialize + Send + Sync {
    type Response: DeserializeOwned + Send + 'static;
    const OP: Operation;
    fn validate(&self) -> Result<(), Error>;
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
            .parse::<u64>()
            .map_err(|_| Error::Validation("recvWindow"))?;
        if n == 0 || n > 60_000 {
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
                return Err(Error::Validation("positive Futures magnitude"));
            }
        }
    }
    Ok(())
}
