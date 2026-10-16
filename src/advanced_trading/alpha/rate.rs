// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Error,
    core::{Cost, Operation},
};
use serde_json::Value;
use std::collections::BTreeMap;
#[allow(
    clippy::unnecessary_wraps,
    reason = "Alpha documents no numeric weight; retain the generated request cost boundary"
)]
pub(crate) fn cost(op: Operation, _p: &BTreeMap<String, Value>) -> Result<Cost, Error> {
    let weight = op.weight;
    Ok(Cost {
        weight,
        raw_requests: 1,
        rolling: None,
        ..Cost::default()
    })
}
