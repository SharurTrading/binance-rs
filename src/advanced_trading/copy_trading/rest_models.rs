// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated rest response DTOs; regenerate with scripts/codegen/generate.py.

use crate::Symbol;
use serde::{Deserialize, Serialize};

/// Provider-native `GetFuturesLeadTraderStatusResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFuturesLeadTraderStatusResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: String,
    /// Exact `message` wire field.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Exact `data` wire field.
    #[serde(rename = "data")]
    pub data: GetFuturesLeadTraderStatusResponseData,
    /// Exact `success` wire field.
    #[serde(rename = "success")]
    pub success: bool,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetFuturesLeadTraderStatusResponseData` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFuturesLeadTraderStatusResponseData {
    /// Exact `isLeadTrader` wire field.
    #[serde(
        rename = "isLeadTrader",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_lead_trader: Option<bool>,
    /// Exact `time` wire field.
    #[serde(rename = "time", default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetFuturesLeadTradingSymbolWhitelistResponse` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFuturesLeadTradingSymbolWhitelistResponse {
    /// Exact `code` wire field.
    #[serde(rename = "code")]
    pub code: String,
    /// Exact `message` wire field.
    #[serde(rename = "message", default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Exact `data` wire field.
    #[serde(rename = "data")]
    pub data: Vec<GetFuturesLeadTradingSymbolWhitelistResponseDataItem>,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}

/// Provider-native `GetFuturesLeadTradingSymbolWhitelistResponseDataItem` payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GetFuturesLeadTradingSymbolWhitelistResponseDataItem {
    /// Exact `symbol` wire field.
    #[serde(rename = "symbol")]
    pub symbol: Symbol,
    /// Exact `baseAsset` wire field.
    #[serde(rename = "baseAsset")]
    pub base_asset: crate::Asset,
    /// Exact `quoteAsset` wire field.
    #[serde(rename = "quoteAsset")]
    pub quote_asset: crate::Asset,
    /// Unknown future wire fields, retained without inventing defaults; avoid logging.
    #[serde(flatten)]
    pub extra: super::event_payloads::UnknownMessage,
}
