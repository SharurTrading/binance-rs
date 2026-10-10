// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated venue enumerations; regenerate with scripts/codegen/generate.py.

/// Contract status (`status` in exchange information, `cs` on the contract info stream).
///
/// One variant per value documented at:
///
/// - <https://developers.binance.info/docs/derivatives/usds-margined-futures/common-definition>
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
    /// Venue `PRE_SETTLE`.
    PreSettle,
    /// Venue `SETTLING`.
    Settling,
    /// Venue `CLOSE`.
    Close,
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
            Self::PreSettle => "PRE_SETTLE",
            Self::Settling => "SETTLING",
            Self::Close => "CLOSE",
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
            "PRE_SETTLE" => Self::PreSettle,
            "SETTLING" => Self::Settling,
            "CLOSE" => Self::Close,
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
/// - <https://developers.binance.info/docs/derivatives/usds-margined-futures/common-definition>
/// - <https://developers.binance.info/docs/derivatives/usds-margined-futures/market-data/rest-api/Continuous-Contract-Kline-Candlestick-Data>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ContractType {
    /// Venue `PERPETUAL`.
    Perpetual,
    /// Venue `CURRENT_MONTH`.
    CurrentMonth,
    /// Venue `NEXT_MONTH`.
    NextMonth,
    /// Venue `CURRENT_QUARTER`.
    CurrentQuarter,
    /// Venue `NEXT_QUARTER`.
    NextQuarter,
    /// Venue `PERPETUAL_DELIVERING`.
    PerpetualDelivering,
    /// Venue `TRADIFI_PERPETUAL`.
    TradifiPerpetual,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl ContractType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Perpetual => "PERPETUAL",
            Self::CurrentMonth => "CURRENT_MONTH",
            Self::NextMonth => "NEXT_MONTH",
            Self::CurrentQuarter => "CURRENT_QUARTER",
            Self::NextQuarter => "NEXT_QUARTER",
            Self::PerpetualDelivering => "PERPETUAL_DELIVERING",
            Self::TradifiPerpetual => "TRADIFI_PERPETUAL",
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
            "CURRENT_MONTH" => Self::CurrentMonth,
            "NEXT_MONTH" => Self::NextMonth,
            "CURRENT_QUARTER" => Self::CurrentQuarter,
            "NEXT_QUARTER" => Self::NextQuarter,
            "PERPETUAL_DELIVERING" => Self::PerpetualDelivering,
            "TRADIFI_PERPETUAL" => Self::TradifiPerpetual,
            _ => Self::Unknown(value),
        })
    }
}

/// Funding rate type (`rateType` in funding rate history).
///
/// One variant per value documented at:
///
/// - <https://developers.binance.info/docs/derivatives/usds-margined-futures/market-data/rest-api/Get-Funding-Rate-History>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FundingRateType {
    /// Venue `Regular`.
    Regular,
    /// Venue `Special`.
    Special,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl FundingRateType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Regular => "Regular",
            Self::Special => "Special",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FundingRateType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FundingRateType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "Regular" => Self::Regular,
            "Special" => Self::Special,
            _ => Self::Unknown(value),
        })
    }
}
