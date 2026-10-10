// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated venue enumerations; regenerate with scripts/codegen/generate.py.

/// Native `AllocationType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AllocationType {
    /// Venue `SOR`.
    Sor,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl AllocationType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Sor => "SOR",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AllocationType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AllocationType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "SOR" => Self::Sor,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `ContingencyType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ContingencyType {
    /// Venue `OCO`.
    Oco,
    /// Venue `OTO`.
    Oto,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl ContingencyType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Oco => "OCO",
            Self::Oto => "OTO",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ContingencyType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ContingencyType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "OCO" => Self::Oco,
            "OTO" => Self::Oto,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `ExecutionType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ExecutionType {
    /// Venue `NEW`.
    New,
    /// Venue `CANCELED`.
    Canceled,
    /// Venue `REPLACED`.
    Replaced,
    /// Venue `REJECTED`.
    Rejected,
    /// Venue `TRADE`.
    Trade,
    /// Venue `EXPIRED`.
    Expired,
    /// Venue `TRADE_PREVENTION`.
    TradePrevention,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl ExecutionType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::New => "NEW",
            Self::Canceled => "CANCELED",
            Self::Replaced => "REPLACED",
            Self::Rejected => "REJECTED",
            Self::Trade => "TRADE",
            Self::Expired => "EXPIRED",
            Self::TradePrevention => "TRADE_PREVENTION",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ExecutionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "NEW" => Self::New,
            "CANCELED" => Self::Canceled,
            "REPLACED" => Self::Replaced,
            "REJECTED" => Self::Rejected,
            "TRADE" => Self::Trade,
            "EXPIRED" => Self::Expired,
            "TRADE_PREVENTION" => Self::TradePrevention,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `KlineInterval` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/web-socket-streams.md#klinecandlestick-streams>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum KlineInterval {
    /// Venue `1s`.
    Second1,
    /// Venue `1m`.
    Minute1,
    /// Venue `3m`.
    Minute3,
    /// Venue `5m`.
    Minute5,
    /// Venue `15m`.
    Minute15,
    /// Venue `30m`.
    Minute30,
    /// Venue `1h`.
    Hour1,
    /// Venue `2h`.
    Hour2,
    /// Venue `4h`.
    Hour4,
    /// Venue `6h`.
    Hour6,
    /// Venue `8h`.
    Hour8,
    /// Venue `12h`.
    Hour12,
    /// Venue `1d`.
    Day1,
    /// Venue `3d`.
    Day3,
    /// Venue `1w`.
    Week1,
    /// Venue `1M`.
    Month1,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl KlineInterval {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Second1 => "1s",
            Self::Minute1 => "1m",
            Self::Minute3 => "3m",
            Self::Minute5 => "5m",
            Self::Minute15 => "15m",
            Self::Minute30 => "30m",
            Self::Hour1 => "1h",
            Self::Hour2 => "2h",
            Self::Hour4 => "4h",
            Self::Hour6 => "6h",
            Self::Hour8 => "8h",
            Self::Hour12 => "12h",
            Self::Day1 => "1d",
            Self::Day3 => "3d",
            Self::Week1 => "1w",
            Self::Month1 => "1M",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for KlineInterval {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KlineInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "1s" => Self::Second1,
            "1m" => Self::Minute1,
            "3m" => Self::Minute3,
            "5m" => Self::Minute5,
            "15m" => Self::Minute15,
            "30m" => Self::Minute30,
            "1h" => Self::Hour1,
            "2h" => Self::Hour2,
            "4h" => Self::Hour4,
            "6h" => Self::Hour6,
            "8h" => Self::Hour8,
            "12h" => Self::Hour12,
            "1d" => Self::Day1,
            "3d" => Self::Day3,
            "1w" => Self::Week1,
            "1M" => Self::Month1,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `ListOrderStatus` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ListOrderStatus {
    /// Venue `EXECUTING`.
    Executing,
    /// Venue `ALL_DONE`.
    AllDone,
    /// Venue `REJECT`.
    Reject,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl ListOrderStatus {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Executing => "EXECUTING",
            Self::AllDone => "ALL_DONE",
            Self::Reject => "REJECT",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListOrderStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListOrderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "EXECUTING" => Self::Executing,
            "ALL_DONE" => Self::AllDone,
            "REJECT" => Self::Reject,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `ListStatusType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ListStatusType {
    /// Venue `RESPONSE`.
    Response,
    /// Venue `EXEC_STARTED`.
    ExecStarted,
    /// Venue `UPDATED`.
    Updated,
    /// Venue `ALL_DONE`.
    AllDone,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl ListStatusType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Response => "RESPONSE",
            Self::ExecStarted => "EXEC_STARTED",
            Self::Updated => "UPDATED",
            Self::AllDone => "ALL_DONE",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for ListStatusType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListStatusType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "RESPONSE" => Self::Response,
            "EXEC_STARTED" => Self::ExecStarted,
            "UPDATED" => Self::Updated,
            "ALL_DONE" => Self::AllDone,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `OrderSide` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OrderSide {
    /// Venue `BUY`.
    Buy,
    /// Venue `SELL`.
    Sell,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl OrderSide {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Buy => "BUY",
            Self::Sell => "SELL",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OrderSide {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrderSide {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "BUY" => Self::Buy,
            "SELL" => Self::Sell,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `OrderStatus` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OrderStatus {
    /// Venue `NEW`.
    New,
    /// Venue `PENDING_NEW`.
    PendingNew,
    /// Venue `PARTIALLY_FILLED`.
    PartiallyFilled,
    /// Venue `FILLED`.
    Filled,
    /// Venue `CANCELED`.
    Canceled,
    /// Venue `PENDING_CANCEL`.
    PendingCancel,
    /// Venue `REJECTED`.
    Rejected,
    /// Venue `EXPIRED`.
    Expired,
    /// Venue `EXPIRED_IN_MATCH`.
    ExpiredInMatch,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl OrderStatus {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::New => "NEW",
            Self::PendingNew => "PENDING_NEW",
            Self::PartiallyFilled => "PARTIALLY_FILLED",
            Self::Filled => "FILLED",
            Self::Canceled => "CANCELED",
            Self::PendingCancel => "PENDING_CANCEL",
            Self::Rejected => "REJECTED",
            Self::Expired => "EXPIRED",
            Self::ExpiredInMatch => "EXPIRED_IN_MATCH",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OrderStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "NEW" => Self::New,
            "PENDING_NEW" => Self::PendingNew,
            "PARTIALLY_FILLED" => Self::PartiallyFilled,
            "FILLED" => Self::Filled,
            "CANCELED" => Self::Canceled,
            "PENDING_CANCEL" => Self::PendingCancel,
            "REJECTED" => Self::Rejected,
            "EXPIRED" => Self::Expired,
            "EXPIRED_IN_MATCH" => Self::ExpiredInMatch,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `OrderType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OrderType {
    /// Venue `LIMIT`.
    Limit,
    /// Venue `MARKET`.
    Market,
    /// Venue `STOP_LOSS`.
    StopLoss,
    /// Venue `STOP_LOSS_LIMIT`.
    StopLossLimit,
    /// Venue `TAKE_PROFIT`.
    TakeProfit,
    /// Venue `TAKE_PROFIT_LIMIT`.
    TakeProfitLimit,
    /// Venue `LIMIT_MAKER`.
    LimitMaker,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl OrderType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Limit => "LIMIT",
            Self::Market => "MARKET",
            Self::StopLoss => "STOP_LOSS",
            Self::StopLossLimit => "STOP_LOSS_LIMIT",
            Self::TakeProfit => "TAKE_PROFIT",
            Self::TakeProfitLimit => "TAKE_PROFIT_LIMIT",
            Self::LimitMaker => "LIMIT_MAKER",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OrderType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrderType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "LIMIT" => Self::Limit,
            "MARKET" => Self::Market,
            "STOP_LOSS" => Self::StopLoss,
            "STOP_LOSS_LIMIT" => Self::StopLossLimit,
            "TAKE_PROFIT" => Self::TakeProfit,
            "TAKE_PROFIT_LIMIT" => Self::TakeProfitLimit,
            "LIMIT_MAKER" => Self::LimitMaker,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `Permission` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Permission {
    /// Venue `SPOT`.
    Spot,
    /// Venue `MARGIN`.
    Margin,
    /// Venue `LEVERAGED`.
    Leveraged,
    /// Venue `TRD_GRP_002`.
    TrdGrp002,
    /// Venue `TRD_GRP_003`.
    TrdGrp003,
    /// Venue `TRD_GRP_004`.
    TrdGrp004,
    /// Venue `TRD_GRP_005`.
    TrdGrp005,
    /// Venue `TRD_GRP_006`.
    TrdGrp006,
    /// Venue `TRD_GRP_007`.
    TrdGrp007,
    /// Venue `TRD_GRP_008`.
    TrdGrp008,
    /// Venue `TRD_GRP_009`.
    TrdGrp009,
    /// Venue `TRD_GRP_010`.
    TrdGrp010,
    /// Venue `TRD_GRP_011`.
    TrdGrp011,
    /// Venue `TRD_GRP_012`.
    TrdGrp012,
    /// Venue `TRD_GRP_013`.
    TrdGrp013,
    /// Venue `TRD_GRP_014`.
    TrdGrp014,
    /// Venue `TRD_GRP_015`.
    TrdGrp015,
    /// Venue `TRD_GRP_016`.
    TrdGrp016,
    /// Venue `TRD_GRP_017`.
    TrdGrp017,
    /// Venue `TRD_GRP_018`.
    TrdGrp018,
    /// Venue `TRD_GRP_019`.
    TrdGrp019,
    /// Venue `TRD_GRP_020`.
    TrdGrp020,
    /// Venue `TRD_GRP_021`.
    TrdGrp021,
    /// Venue `TRD_GRP_022`.
    TrdGrp022,
    /// Venue `TRD_GRP_023`.
    TrdGrp023,
    /// Venue `TRD_GRP_024`.
    TrdGrp024,
    /// Venue `TRD_GRP_025`.
    TrdGrp025,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl Permission {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Spot => "SPOT",
            Self::Margin => "MARGIN",
            Self::Leveraged => "LEVERAGED",
            Self::TrdGrp002 => "TRD_GRP_002",
            Self::TrdGrp003 => "TRD_GRP_003",
            Self::TrdGrp004 => "TRD_GRP_004",
            Self::TrdGrp005 => "TRD_GRP_005",
            Self::TrdGrp006 => "TRD_GRP_006",
            Self::TrdGrp007 => "TRD_GRP_007",
            Self::TrdGrp008 => "TRD_GRP_008",
            Self::TrdGrp009 => "TRD_GRP_009",
            Self::TrdGrp010 => "TRD_GRP_010",
            Self::TrdGrp011 => "TRD_GRP_011",
            Self::TrdGrp012 => "TRD_GRP_012",
            Self::TrdGrp013 => "TRD_GRP_013",
            Self::TrdGrp014 => "TRD_GRP_014",
            Self::TrdGrp015 => "TRD_GRP_015",
            Self::TrdGrp016 => "TRD_GRP_016",
            Self::TrdGrp017 => "TRD_GRP_017",
            Self::TrdGrp018 => "TRD_GRP_018",
            Self::TrdGrp019 => "TRD_GRP_019",
            Self::TrdGrp020 => "TRD_GRP_020",
            Self::TrdGrp021 => "TRD_GRP_021",
            Self::TrdGrp022 => "TRD_GRP_022",
            Self::TrdGrp023 => "TRD_GRP_023",
            Self::TrdGrp024 => "TRD_GRP_024",
            Self::TrdGrp025 => "TRD_GRP_025",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for Permission {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Permission {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "SPOT" => Self::Spot,
            "MARGIN" => Self::Margin,
            "LEVERAGED" => Self::Leveraged,
            "TRD_GRP_002" => Self::TrdGrp002,
            "TRD_GRP_003" => Self::TrdGrp003,
            "TRD_GRP_004" => Self::TrdGrp004,
            "TRD_GRP_005" => Self::TrdGrp005,
            "TRD_GRP_006" => Self::TrdGrp006,
            "TRD_GRP_007" => Self::TrdGrp007,
            "TRD_GRP_008" => Self::TrdGrp008,
            "TRD_GRP_009" => Self::TrdGrp009,
            "TRD_GRP_010" => Self::TrdGrp010,
            "TRD_GRP_011" => Self::TrdGrp011,
            "TRD_GRP_012" => Self::TrdGrp012,
            "TRD_GRP_013" => Self::TrdGrp013,
            "TRD_GRP_014" => Self::TrdGrp014,
            "TRD_GRP_015" => Self::TrdGrp015,
            "TRD_GRP_016" => Self::TrdGrp016,
            "TRD_GRP_017" => Self::TrdGrp017,
            "TRD_GRP_018" => Self::TrdGrp018,
            "TRD_GRP_019" => Self::TrdGrp019,
            "TRD_GRP_020" => Self::TrdGrp020,
            "TRD_GRP_021" => Self::TrdGrp021,
            "TRD_GRP_022" => Self::TrdGrp022,
            "TRD_GRP_023" => Self::TrdGrp023,
            "TRD_GRP_024" => Self::TrdGrp024,
            "TRD_GRP_025" => Self::TrdGrp025,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `RateLimitInterval` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/sbe/schemas/spot_3_4.xml>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RateLimitInterval {
    /// Venue `SECOND`.
    Second,
    /// Venue `MINUTE`.
    Minute,
    /// Venue `HOUR`.
    Hour,
    /// Venue `DAY`.
    Day,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl RateLimitInterval {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Second => "SECOND",
            Self::Minute => "MINUTE",
            Self::Hour => "HOUR",
            Self::Day => "DAY",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RateLimitInterval {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "SECOND" => Self::Second,
            "MINUTE" => Self::Minute,
            "HOUR" => Self::Hour,
            "DAY" => Self::Day,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `RateLimitType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/sbe/schemas/spot_3_4.xml>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RateLimitType {
    /// Venue `REQUEST_WEIGHT`.
    RequestWeight,
    /// Venue `ORDERS`.
    Orders,
    /// Venue `RAW_REQUESTS`.
    RawRequests,
    /// Venue `CONNECTIONS`.
    Connections,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl RateLimitType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::RequestWeight => "REQUEST_WEIGHT",
            Self::Orders => "ORDERS",
            Self::RawRequests => "RAW_REQUESTS",
            Self::Connections => "CONNECTIONS",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for RateLimitType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RateLimitType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "REQUEST_WEIGHT" => Self::RequestWeight,
            "ORDERS" => Self::Orders,
            "RAW_REQUESTS" => Self::RawRequests,
            "CONNECTIONS" => Self::Connections,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `SelfTradePreventionMode` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SelfTradePreventionMode {
    /// Venue `NONE`.
    None,
    /// Venue `EXPIRE_MAKER`.
    ExpireMaker,
    /// Venue `EXPIRE_TAKER`.
    ExpireTaker,
    /// Venue `EXPIRE_BOTH`.
    ExpireBoth,
    /// Venue `DECREMENT`.
    Decrement,
    /// Venue `TRANSFER`.
    Transfer,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl SelfTradePreventionMode {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "NONE",
            Self::ExpireMaker => "EXPIRE_MAKER",
            Self::ExpireTaker => "EXPIRE_TAKER",
            Self::ExpireBoth => "EXPIRE_BOTH",
            Self::Decrement => "DECREMENT",
            Self::Transfer => "TRANSFER",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SelfTradePreventionMode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SelfTradePreventionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "NONE" => Self::None,
            "EXPIRE_MAKER" => Self::ExpireMaker,
            "EXPIRE_TAKER" => Self::ExpireTaker,
            "EXPIRE_BOTH" => Self::ExpireBoth,
            "DECREMENT" => Self::Decrement,
            "TRANSFER" => Self::Transfer,
            _ => Self::Unknown(value),
        })
    }
}

/// Symbol status (`status` in exchange information).
///
/// One variant per value documented at:
///
/// - <https://github.com/binance/binance-spot-api-docs/blob/master/enums.md#symbol-status-status>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SymbolStatus {
    /// Venue `TRADING`.
    Trading,
    /// Venue `END_OF_DAY`.
    EndOfDay,
    /// Venue `HALT`.
    Halt,
    /// Venue `BREAK`.
    Break,
    /// Venue `CANCEL_ONLY`.
    CancelOnly,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl SymbolStatus {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Trading => "TRADING",
            Self::EndOfDay => "END_OF_DAY",
            Self::Halt => "HALT",
            Self::Break => "BREAK",
            Self::CancelOnly => "CANCEL_ONLY",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SymbolStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SymbolStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "TRADING" => Self::Trading,
            "END_OF_DAY" => Self::EndOfDay,
            "HALT" => Self::Halt,
            "BREAK" => Self::Break,
            "CANCEL_ONLY" => Self::CancelOnly,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `TimeInForce` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TimeInForce {
    /// Venue `GTC`.
    Gtc,
    /// Venue `IOC`.
    Ioc,
    /// Venue `FOK`.
    Fok,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl TimeInForce {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Gtc => "GTC",
            Self::Ioc => "IOC",
            Self::Fok => "FOK",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for TimeInForce {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TimeInForce {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "GTC" => Self::Gtc,
            "IOC" => Self::Ioc,
            "FOK" => Self::Fok,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `WorkingFloor` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://raw.githubusercontent.com/binance/binance-spot-api-docs/master/enums.md>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WorkingFloor {
    /// Venue `EXCHANGE`.
    Exchange,
    /// Venue `SOR`.
    Sor,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl WorkingFloor {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exchange => "EXCHANGE",
            Self::Sor => "SOR",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for WorkingFloor {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WorkingFloor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "EXCHANGE" => Self::Exchange,
            "SOR" => Self::Sor,
            _ => Self::Unknown(value),
        })
    }
}
