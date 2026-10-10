// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated venue enumerations; regenerate with scripts/codegen/generate.py.

/// Native `AccountUpdateReason` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/user-data-streams>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AccountUpdateReason {
    /// Venue `DEPOSIT`.
    Deposit,
    /// Venue `WITHDRAW`.
    Withdraw,
    /// Venue `ORDER`.
    Order,
    /// Venue `FUNDING_FEE`.
    FundingFee,
    /// Venue `ADJUSTMENT`.
    Adjustment,
    /// Venue `INSURANCE_CLEAR`.
    InsuranceClear,
    /// Venue `ADMIN_DEPOSIT`.
    AdminDeposit,
    /// Venue `ADMIN_WITHDRAW`.
    AdminWithdraw,
    /// Venue `MARGIN_TRANSFER`.
    MarginTransfer,
    /// Venue `MARGIN_TYPE_CHANGE`.
    MarginTypeChange,
    /// Venue `ASSET_TRANSFER`.
    AssetTransfer,
    /// Venue `COIN_SWAP_DEPOSIT`.
    CoinSwapDeposit,
    /// Venue `COIN_SWAP_WITHDRAW`.
    CoinSwapWithdraw,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl AccountUpdateReason {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Deposit => "DEPOSIT",
            Self::Withdraw => "WITHDRAW",
            Self::Order => "ORDER",
            Self::FundingFee => "FUNDING_FEE",
            Self::Adjustment => "ADJUSTMENT",
            Self::InsuranceClear => "INSURANCE_CLEAR",
            Self::AdminDeposit => "ADMIN_DEPOSIT",
            Self::AdminWithdraw => "ADMIN_WITHDRAW",
            Self::MarginTransfer => "MARGIN_TRANSFER",
            Self::MarginTypeChange => "MARGIN_TYPE_CHANGE",
            Self::AssetTransfer => "ASSET_TRANSFER",
            Self::CoinSwapDeposit => "COIN_SWAP_DEPOSIT",
            Self::CoinSwapWithdraw => "COIN_SWAP_WITHDRAW",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AccountUpdateReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AccountUpdateReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "DEPOSIT" => Self::Deposit,
            "WITHDRAW" => Self::Withdraw,
            "ORDER" => Self::Order,
            "FUNDING_FEE" => Self::FundingFee,
            "ADJUSTMENT" => Self::Adjustment,
            "INSURANCE_CLEAR" => Self::InsuranceClear,
            "ADMIN_DEPOSIT" => Self::AdminDeposit,
            "ADMIN_WITHDRAW" => Self::AdminWithdraw,
            "MARGIN_TRANSFER" => Self::MarginTransfer,
            "MARGIN_TYPE_CHANGE" => Self::MarginTypeChange,
            "ASSET_TRANSFER" => Self::AssetTransfer,
            "COIN_SWAP_DEPOSIT" => Self::CoinSwapDeposit,
            "COIN_SWAP_WITHDRAW" => Self::CoinSwapWithdraw,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `AlgoStatus` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/Important-CM-UM-Integration-Notice>
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-usds-futures/user-data-streams>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AlgoStatus {
    /// Venue `NEW`.
    New,
    /// Venue `CANCELED`.
    Canceled,
    /// Venue `TRIGGERING`.
    Triggering,
    /// Venue `TRIGGERED`.
    Triggered,
    /// Venue `FINISHED`.
    Finished,
    /// Venue `REJECTED`.
    Rejected,
    /// Venue `EXPIRED`.
    Expired,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl AlgoStatus {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::New => "NEW",
            Self::Canceled => "CANCELED",
            Self::Triggering => "TRIGGERING",
            Self::Triggered => "TRIGGERED",
            Self::Finished => "FINISHED",
            Self::Rejected => "REJECTED",
            Self::Expired => "EXPIRED",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AlgoStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlgoStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "NEW" => Self::New,
            "CANCELED" => Self::Canceled,
            "TRIGGERING" => Self::Triggering,
            "TRIGGERED" => Self::Triggered,
            "FINISHED" => Self::Finished,
            "REJECTED" => Self::Rejected,
            "EXPIRED" => Self::Expired,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `AlgoType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/Important-CM-UM-Integration-Notice>
/// - <https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-usd-s-m-futures/api/rest-api/trade#new-algo-order>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum AlgoType {
    /// Venue `CONDITIONAL`.
    Conditional,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl AlgoType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Conditional => "CONDITIONAL",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for AlgoType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlgoType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "CONDITIONAL" => Self::Conditional,
            _ => Self::Unknown(value),
        })
    }
}

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

/// Native `ExecutionType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/user-data-streams>
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
    /// Venue `CALCULATED`.
    Calculated,
    /// Venue `EXPIRED`.
    Expired,
    /// Venue `TRADE`.
    Trade,
    /// Venue `AMENDMENT`.
    Amendment,
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
            Self::Calculated => "CALCULATED",
            Self::Expired => "EXPIRED",
            Self::Trade => "TRADE",
            Self::Amendment => "AMENDMENT",
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
            "CALCULATED" => Self::Calculated,
            "EXPIRED" => Self::Expired,
            "TRADE" => Self::Trade,
            "AMENDMENT" => Self::Amendment,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `FilterType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum FilterType {
    /// Venue `PRICE_FILTER`.
    PriceFilter,
    /// Venue `LOT_SIZE`.
    LotSize,
    /// Venue `MARKET_LOT_SIZE`.
    MarketLotSize,
    /// Venue `MAX_NUM_ORDERS`.
    MaxNumOrders,
    /// Venue `PERCENT_PRICE`.
    PercentPrice,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl FilterType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::PriceFilter => "PRICE_FILTER",
            Self::LotSize => "LOT_SIZE",
            Self::MarketLotSize => "MARKET_LOT_SIZE",
            Self::MaxNumOrders => "MAX_NUM_ORDERS",
            Self::PercentPrice => "PERCENT_PRICE",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for FilterType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FilterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "PRICE_FILTER" => Self::PriceFilter,
            "LOT_SIZE" => Self::LotSize,
            "MARKET_LOT_SIZE" => Self::MarketLotSize,
            "MAX_NUM_ORDERS" => Self::MaxNumOrders,
            "PERCENT_PRICE" => Self::PercentPrice,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `KlineInterval` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum KlineInterval {
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

/// Native `OrderSide` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
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
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/user-data-streams>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OrderStatus {
    /// Venue `NEW`.
    New,
    /// Venue `PARTIALLY_FILLED`.
    PartiallyFilled,
    /// Venue `FILLED`.
    Filled,
    /// Venue `CANCELED`.
    Canceled,
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
            Self::PartiallyFilled => "PARTIALLY_FILLED",
            Self::Filled => "FILLED",
            Self::Canceled => "CANCELED",
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
            "PARTIALLY_FILLED" => Self::PartiallyFilled,
            "FILLED" => Self::Filled,
            "CANCELED" => Self::Canceled,
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
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/user-data-streams>
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
    /// Venue `STOP`.
    Stop,
    /// Venue `STOP_MARKET`.
    StopMarket,
    /// Venue `TAKE_PROFIT`.
    TakeProfit,
    /// Venue `TAKE_PROFIT_MARKET`.
    TakeProfitMarket,
    /// Venue `TRAILING_STOP_MARKET`.
    TrailingStopMarket,
    /// Venue `LIQUIDATION`.
    Liquidation,
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
            Self::Stop => "STOP",
            Self::StopMarket => "STOP_MARKET",
            Self::TakeProfit => "TAKE_PROFIT",
            Self::TakeProfitMarket => "TAKE_PROFIT_MARKET",
            Self::TrailingStopMarket => "TRAILING_STOP_MARKET",
            Self::Liquidation => "LIQUIDATION",
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
            "STOP" => Self::Stop,
            "STOP_MARKET" => Self::StopMarket,
            "TAKE_PROFIT" => Self::TakeProfit,
            "TAKE_PROFIT_MARKET" => Self::TakeProfitMarket,
            "TRAILING_STOP_MARKET" => Self::TrailingStopMarket,
            "LIQUIDATION" => Self::Liquidation,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `PositionSide` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PositionSide {
    /// Venue `BOTH`.
    Both,
    /// Venue `LONG`.
    Long,
    /// Venue `SHORT`.
    Short,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl PositionSide {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Both => "BOTH",
            Self::Long => "LONG",
            Self::Short => "SHORT",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PositionSide {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PositionSide {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "BOTH" => Self::Both,
            "LONG" => Self::Long,
            "SHORT" => Self::Short,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `PriceMatch` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PriceMatch {
    /// Venue `NONE`.
    None,
    /// Venue `OPPONENT`.
    Opponent,
    /// Venue `OPPONENT_5`.
    Opponent5,
    /// Venue `OPPONENT_10`.
    Opponent10,
    /// Venue `OPPONENT_20`.
    Opponent20,
    /// Venue `QUEUE`.
    Queue,
    /// Venue `QUEUE_5`.
    Queue5,
    /// Venue `QUEUE_10`.
    Queue10,
    /// Venue `QUEUE_20`.
    Queue20,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl PriceMatch {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "NONE",
            Self::Opponent => "OPPONENT",
            Self::Opponent5 => "OPPONENT_5",
            Self::Opponent10 => "OPPONENT_10",
            Self::Opponent20 => "OPPONENT_20",
            Self::Queue => "QUEUE",
            Self::Queue5 => "QUEUE_5",
            Self::Queue10 => "QUEUE_10",
            Self::Queue20 => "QUEUE_20",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for PriceMatch {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PriceMatch {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "NONE" => Self::None,
            "OPPONENT" => Self::Opponent,
            "OPPONENT_5" => Self::Opponent5,
            "OPPONENT_10" => Self::Opponent10,
            "OPPONENT_20" => Self::Opponent20,
            "QUEUE" => Self::Queue,
            "QUEUE_5" => Self::Queue5,
            "QUEUE_10" => Self::Queue10,
            "QUEUE_20" => Self::Queue20,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `RateLimitInterval` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RateLimitInterval {
    /// Venue `MINUTE`.
    Minute,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl RateLimitInterval {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Minute => "MINUTE",
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
            "MINUTE" => Self::Minute,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `RateLimitType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
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
            _ => Self::Unknown(value),
        })
    }
}

/// Native `SelfTradePreventionMode` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
/// - <https://developers.binance.com/en/docs/catalog/core-trading-derivatives-trading-coin-m-futures/api/rest-api/trade#new-order>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SelfTradePreventionMode {
    /// Venue `NONE`.
    None,
    /// Venue `EXPIRE_TAKER`.
    ExpireTaker,
    /// Venue `EXPIRE_BOTH`.
    ExpireBoth,
    /// Venue `EXPIRE_MAKER`.
    ExpireMaker,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl SelfTradePreventionMode {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::None => "NONE",
            Self::ExpireTaker => "EXPIRE_TAKER",
            Self::ExpireBoth => "EXPIRE_BOTH",
            Self::ExpireMaker => "EXPIRE_MAKER",
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
            "EXPIRE_TAKER" => Self::ExpireTaker,
            "EXPIRE_BOTH" => Self::ExpireBoth,
            "EXPIRE_MAKER" => Self::ExpireMaker,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `TimeInForce` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
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
    /// Venue `GTX`.
    Gtx,
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
            Self::Gtx => "GTX",
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
            "GTX" => Self::Gtx,
            _ => Self::Unknown(value),
        })
    }
}

/// Native `WorkingType` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/derivatives-trading-coin-futures/common-definition>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum WorkingType {
    /// Venue `MARK_PRICE`.
    MarkPrice,
    /// Venue `CONTRACT_PRICE`.
    ContractPrice,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl WorkingType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::MarkPrice => "MARK_PRICE",
            Self::ContractPrice => "CONTRACT_PRICE",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for WorkingType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WorkingType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "MARK_PRICE" => Self::MarkPrice,
            "CONTRACT_PRICE" => Self::ContractPrice,
            _ => Self::Unknown(value),
        })
    }
}
