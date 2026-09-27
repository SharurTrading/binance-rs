// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Error,
    core::{Cost, Operation, sapi::SapiCost},
};
use serde_json::Value;
use std::collections::BTreeMap;
pub(crate) fn cost(op: Operation, _p: &BTreeMap<String, Value>) -> Result<Cost, Error> {
    if op.weight == 0 {
        return Err(Error::Configuration("missing Convert quota evidence"));
    }
    Ok(Cost {
        sapi: Some(SapiCost {
            endpoint: op.path,
            uid: !matches!(
                op.name,
                "listAllConvertPairs" | "queryOrderQuantityPrecisionPerAsset"
            ),
            weight: op.weight,
            requests_per_second: None,
        }),
        ..Cost::default()
    })
}
