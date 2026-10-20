// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{
    Error,
    advanced_trading::validation::{Parameters, common, range, text},
};
pub(crate) fn validate(op: &str, p: &Parameters) -> Result<(), Error> {
    common(p)?;
    if let Some(value) = p.get("marginCallLevel") {
        let level = super::wire::parse_decimal(value)
            .map_err(|_| Error::Validation("Portfolio Margin Pro margin-call level"))?;
        if level < crate::Decimal::new(11, 1) || level > crate::Decimal::from(2) {
            return Err(Error::Validation(
                "Portfolio Margin Pro margin-call level range",
            ));
        }
    }
    if op == "fundCollectionByAsset" && text(p, "asset") == Some("BNB") {
        return Err(Error::Validation(
            "Portfolio Margin Pro BNB uses its dedicated transfer endpoint",
        ));
    }
    if op == "queryPortfolioMarginProBankruptcyLoanRepayHistory" {
        range(p, 360 * 86_400_000, false)?;
    }
    Ok(())
}
pub(crate) use crate::advanced_trading::validation::validate_time;

// Product-local validation/authentication refusals; unknown and 5xx remain ambiguous.
pub(crate) fn definitive(status: u16, value: &serde_json::Value) -> bool {
    // Native refusal codes pinned in schema/portfolio-margin-pro-error-codes.json.
    crate::advanced_trading::validation::definitive(
        status,
        value,
        &[
            -2015, -2014, -1131, -1130, -1128, -1127, -1121, -1111, -1106, -1105, -1104, -1103,
            -1102, -1101, -1100, -1022, -1021, -1020, -1002,
        ],
    )
}

pub(crate) fn response_failed(_op: &str, value: &serde_json::Value) -> bool {
    value.get("code").is_some() || value.get("success") == Some(&serde_json::Value::Bool(false))
}
