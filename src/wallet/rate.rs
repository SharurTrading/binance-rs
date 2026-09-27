// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Error,
    core::{Cost, Operation, sapi::SapiCost},
};
use serde_json::Value;
use std::collections::BTreeMap;
pub(crate) fn cost(op: Operation, _p: &BTreeMap<String, Value>) -> Result<Cost, Error> {
    let uid = matches!(
        op.name,
        "dustConvert"
            | "dustTransfer"
            | "getCloudMiningPaymentAndRefundHistory"
            | "brokerWithdraw"
            | "submitDepositQuestionnaire"
            | "submitDepositQuestionnaireTravelRule"
            | "submitDepositQuestionnaireV2"
            | "withdrawTravelRule"
            | "userUniversalTransfer"
            | "withdraw"
            | "withdrawHistory"
    );
    let weight = op.weight;
    let requests_per_second = if op.name == "withdrawHistory" {
        Some(10)
    } else {
        None
    };
    if weight == 0 {
        return Err(Error::Configuration("missing Wallet quota evidence"));
    }
    Ok(Cost {
        sapi: Some(SapiCost {
            endpoint: op.path,
            uid,
            weight,
            requests_per_second,
        }),
        ..Cost::default()
    })
}
