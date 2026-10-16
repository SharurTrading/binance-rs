// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Error,
    core::{Cost, Operation},
};
use serde_json::Value;
use std::collections::BTreeMap;
pub(crate) fn cost(op: Operation, p: &BTreeMap<String, Value>) -> Result<Cost, Error> {
    let weight = match op.name {
        "queryPortfolioMarginAssetIndexPrice" => {
            if p.contains_key("asset") {
                1
            } else {
                50
            }
        }
        _ => op.weight,
    };
    if weight == 0 {
        return Err(Error::Configuration("missing native endpoint weight"));
    }
    Ok(Cost {
        sapi: Some(crate::core::sapi::SapiCost {
            endpoint: op.path,
            uid: matches!(
                op.name,
                "getPortfolioMarginProAccountInfo"
                    | "portfolioMarginProBankruptcyLoanRepay"
                    | "queryPortfolioMarginProBankruptcyLoanAmount"
                    | "transferLdusdtRwusdForPortfolioMargin"
            ),
            weight,
            requests_per_second: op.requests_per_second,
            requests_per_minute: op.requests_per_minute,
        }),
        weight: 0,
        rolling: match op.name {
            "bnbTransfer" => Some(crate::core::rate::RollingLimit {
                endpoint: op.path,
                window_millis: 600_000,
                requests: 2,
            }),
            "fundAutoCollection" => Some(crate::core::rate::RollingLimit {
                endpoint: op.path,
                window_millis: 3_600_000,
                requests: 500,
            }),
            _ => None,
        },
        ..Cost::default()
    })
}
