// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated venue enumerations; regenerate with scripts/codegen/generate.py.

/// Contract status (`contractStatus` in exchange information, `cs` on the contract info stream).
///
/// One variant per value documented at:
///
/// - <https://developers.binance.info/docs/derivatives/coin-margined-futures/common-definition>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ContractStatus {
    /// Venue `PENDING_TRADING`.
    PendingTrading,
    /// Venue `TRADING`.
    Trading,
    /// Venue `PRE_DELIVERING`.
    PreDelivering,
    /// Venue `DELIVERING`.
    Delivering,
    /// Venue `DELIVERED`.
    Delivered,
    /// Venue `TRADING_HALT`.
    TradingHalt,
    /// Venue `TRADING_CANCEL_ONLY`.
    TradingCancelOnly,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl ContractStatus {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::PendingTrading => "PENDING_TRADING",
            Self::Trading => "TRADING",
            Self::PreDelivering => "PRE_DELIVERING",
            Self::Delivering => "DELIVERING",
            Self::Delivered => "DELIVERED",
            Self::TradingHalt => "TRADING_HALT",
            Self::TradingCancelOnly => "TRADING_CANCEL_ONLY",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ContractStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ContractStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "PENDING_TRADING" => Self::PendingTrading,
            "TRADING" => Self::Trading,
            "PRE_DELIVERING" => Self::PreDelivering,
            "DELIVERING" => Self::Delivering,
            "DELIVERED" => Self::Delivered,
            "TRADING_HALT" => Self::TradingHalt,
            "TRADING_CANCEL_ONLY" => Self::TradingCancelOnly,
            _ => Self::Unknown(value),
        })
    }
}

/// Contract type (`contractType`, and `ct` on the contract info and continuous kline streams).
///
/// One variant per value documented at:
///
/// - <https://developers.binance.info/docs/derivatives/coin-margined-futures/common-definition>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ContractType {
    /// Venue `PERPETUAL`.
    Perpetual,
    /// Venue `CURRENT_QUARTER`.
    CurrentQuarter,
    /// Venue `NEXT_QUARTER`.
    NextQuarter,
    /// Venue `CURRENT_QUARTER_DELIVERING`.
    CurrentQuarterDelivering,
    /// Venue `NEXT_QUARTER_DELIVERING`.
    NextQuarterDelivering,
    /// Venue `PERPETUAL_DELIVERING`.
    PerpetualDelivering,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl ContractType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Perpetual => "PERPETUAL",
            Self::CurrentQuarter => "CURRENT_QUARTER",
            Self::NextQuarter => "NEXT_QUARTER",
            Self::CurrentQuarterDelivering => "CURRENT_QUARTER_DELIVERING",
            Self::NextQuarterDelivering => "NEXT_QUARTER_DELIVERING",
            Self::PerpetualDelivering => "PERPETUAL_DELIVERING",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ContractType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ContractType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "PERPETUAL" => Self::Perpetual,
            "CURRENT_QUARTER" => Self::CurrentQuarter,
            "NEXT_QUARTER" => Self::NextQuarter,
            "CURRENT_QUARTER_DELIVERING" => Self::CurrentQuarterDelivering,
            "NEXT_QUARTER_DELIVERING" => Self::NextQuarterDelivering,
            "PERPETUAL_DELIVERING" => Self::PerpetualDelivering,
            _ => Self::Unknown(value),
        })
    }
}
