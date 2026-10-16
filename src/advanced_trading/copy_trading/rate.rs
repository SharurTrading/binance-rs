// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Error,
    core::{Cost, Operation},
};
use serde_json::Value;
use std::collections::BTreeMap;
pub(crate) fn cost(op: Operation, _p: &BTreeMap<String, Value>) -> Result<Cost, Error> {
    let weight = op.weight;
    if weight == 0 {
        return Err(Error::Configuration("missing native endpoint weight"));
    }
    Ok(Cost {
        sapi: Some(crate::core::sapi::SapiCost {
            endpoint: op.path,
            uid: false,
            weight,
            requests_per_second: op.requests_per_second,
            requests_per_minute: op.requests_per_minute,
        }),
        weight: 0,
        rolling: None,
        ..Cost::default()
    })
}
