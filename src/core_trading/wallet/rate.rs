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
    // Per-second caps come only from pinned schema evidence
    // (`x-requests-per-second`, snapshot-checked by codegen): the
    // withdrawHistory endpoint page annotates its UID weight 18000 as
    // "10 requests per second", corroborated by the 2023-09-04 Wallet
    // change log. General info's independent 180,000/minute UID budget
    // implies 10/minute for an 18000-weight call; the client conservatively
    // enforces both documented budgets rather than choosing between them.
    let requests_per_second = op.requests_per_second;
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
