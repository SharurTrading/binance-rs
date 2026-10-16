// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated native event checks, independent stream corpus.
use serde_json::Value;
fn known<T: std::fmt::Debug>(decoded: T) -> Result<(), String> {
    if format!("{decoded:?}").starts_with("Unknown(") {
        return Err("documented event became unknown".into());
    }
    Ok(())
}
fn typed<T: serde::de::DeserializeOwned + std::fmt::Debug>(value: Value) -> Result<(), String> {
    known(serde_json::from_value::<T>(value).map_err(|e| e.to_string())?)
}
pub(super) fn check(model: &str, value: Value) -> Result<(), String> {
    check_0(model, value)
}
fn check_0(model: &str, value: Value) -> Result<(), String> {
    match model{
        "Vec<binance_client::core_trading::coinm::stream_models::AllMarketMiniTickersStreamEventItem>"=>typed::<Vec<binance_client::core_trading::coinm::stream_models::AllMarketMiniTickersStreamEventItem>>(value),
        "Vec<binance_client::core_trading::coinm::stream_models::AllMarketTickersStreamsEventItem>"=>typed::<Vec<binance_client::core_trading::coinm::stream_models::AllMarketTickersStreamsEventItem>>(value),
        "Vec<binance_client::core_trading::coinm::stream_models::MarkPriceOfAllSymbolsOfAPairEventItem>"=>typed::<Vec<binance_client::core_trading::coinm::stream_models::MarkPriceOfAllSymbolsOfAPairEventItem>>(value),
        "Vec<binance_client::core_trading::usdm::stream_models::AllMarketMiniTickersStreamEventItem>"=>typed::<Vec<binance_client::core_trading::usdm::stream_models::AllMarketMiniTickersStreamEventItem>>(value),
        "Vec<binance_client::core_trading::usdm::stream_models::AllMarketTickersStreamsEventItem>"=>typed::<Vec<binance_client::core_trading::usdm::stream_models::AllMarketTickersStreamsEventItem>>(value),
        "Vec<binance_client::core_trading::usdm::stream_models::AssetIndexEventItem>"=>typed::<Vec<binance_client::core_trading::usdm::stream_models::AssetIndexEventItem>>(value),
        "Vec<binance_client::core_trading::usdm::stream_models::AssetIndexForAssetEventItem>"=>typed::<Vec<binance_client::core_trading::usdm::stream_models::AssetIndexForAssetEventItem>>(value),
        "Vec<binance_client::core_trading::usdm::stream_models::MarkPriceStreamForAllMarketEventItem>"=>typed::<Vec<binance_client::core_trading::usdm::stream_models::MarkPriceStreamForAllMarketEventItem>>(value),
        _=>check_1(model,value),
    }
}
fn check_1(model: &str, value: Value) -> Result<(), String> {
    match model {
        "Vec<binance_client::options::stream_models::IndexPriceStreamsEventItem>" => {
            typed::<Vec<binance_client::options::stream_models::IndexPriceStreamsEventItem>>(value)
        }
        "Vec<binance_client::options::stream_models::OpenInterestEventItem>" => {
            typed::<Vec<binance_client::options::stream_models::OpenInterestEventItem>>(value)
        }
        "Vec<binance_client::options::stream_models::OptionMarkPriceEventItem>" => {
            typed::<Vec<binance_client::options::stream_models::OptionMarkPriceEventItem>>(value)
        }
        "Vec<binance_client::spot::stream_models::AllMarketRollingWindowTickerEventItem>" => {
            typed::<Vec<binance_client::spot::stream_models::AllMarketRollingWindowTickerEventItem>>(
                value,
            )
        }
        "Vec<binance_client::spot::stream_models::AllMiniTickerEventItem>" => {
            typed::<Vec<binance_client::spot::stream_models::AllMiniTickerEventItem>>(value)
        }
        "binance_client::advanced_trading::alpha::stream_models::AggregateTradeStreamEvent" => {
            typed::<binance_client::advanced_trading::alpha::stream_models::AggregateTradeStreamEvent>(
                value,
            )
        }
        "binance_client::advanced_trading::alpha::stream_models::AllBookTickerStreamEvent" => {
            typed::<binance_client::advanced_trading::alpha::stream_models::AllBookTickerStreamEvent>(
                value,
            )
        }
        "binance_client::advanced_trading::alpha::stream_models::AllMiniTickerStreamEvent" => {
            typed::<binance_client::advanced_trading::alpha::stream_models::AllMiniTickerStreamEvent>(
                value,
            )
        }
        _ => check_2(model, value),
    }
}
fn check_2(model: &str, value: Value) -> Result<(), String> {
    match model{
        "binance_client::advanced_trading::alpha::stream_models::AllTickerStreamEvent"=>typed::<binance_client::advanced_trading::alpha::stream_models::AllTickerStreamEvent>(value),
        "binance_client::advanced_trading::alpha::stream_models::AllTokens24hTickerStreamEvent"=>typed::<binance_client::advanced_trading::alpha::stream_models::AllTokens24hTickerStreamEvent>(value),
        "binance_client::advanced_trading::alpha::stream_models::BookTickerStreamEvent"=>typed::<binance_client::advanced_trading::alpha::stream_models::BookTickerStreamEvent>(value),
        "binance_client::advanced_trading::alpha::stream_models::ContractKlineStreamEvent"=>typed::<binance_client::advanced_trading::alpha::stream_models::ContractKlineStreamEvent>(value),
        "binance_client::advanced_trading::alpha::stream_models::FullDepthStreamEvent"=>typed::<binance_client::advanced_trading::alpha::stream_models::FullDepthStreamEvent>(value),
        "binance_client::advanced_trading::alpha::stream_models::KlineStreamEvent"=>typed::<binance_client::advanced_trading::alpha::stream_models::KlineStreamEvent>(value),
        "binance_client::advanced_trading::alpha::stream_models::MiniTickerStreamEvent"=>typed::<binance_client::advanced_trading::alpha::stream_models::MiniTickerStreamEvent>(value),
        "binance_client::advanced_trading::alpha::stream_models::PartialDepthStreamEvent"=>typed::<binance_client::advanced_trading::alpha::stream_models::PartialDepthStreamEvent>(value),
        _=>check_3(model,value),
    }
}
fn check_3(model: &str, value: Value) -> Result<(), String> {
    match model{
        "binance_client::advanced_trading::alpha::stream_models::TickerStreamEvent"=>typed::<binance_client::advanced_trading::alpha::stream_models::TickerStreamEvent>(value),
        "binance_client::advanced_trading::alpha::stream_models::TradeStreamEvent"=>typed::<binance_client::advanced_trading::alpha::stream_models::TradeStreamEvent>(value),
        "binance_client::advanced_trading::portfolio_margin::stream_models::AccountConfigUpdateEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::AccountConfigUpdateEvent>(value),
        "binance_client::advanced_trading::portfolio_margin::stream_models::AccountUpdateEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::AccountUpdateEvent>(value),
        "binance_client::advanced_trading::portfolio_margin::stream_models::AlgoOrderUpdateEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::AlgoOrderUpdateEvent>(value),
        "binance_client::advanced_trading::portfolio_margin::stream_models::BalanceUpdateEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::BalanceUpdateEvent>(value),
        "binance_client::advanced_trading::portfolio_margin::stream_models::ConditionalOrderTradeUpdateEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::ConditionalOrderTradeUpdateEvent>(value),
        "binance_client::advanced_trading::portfolio_margin::stream_models::ExecutionReportEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::ExecutionReportEvent>(value),
        _=>check_4(model,value),
    }
}
fn check_4(model: &str, value: Value) -> Result<(), String> {
    match model{
        "binance_client::advanced_trading::portfolio_margin::stream_models::LiabilityChangeEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::LiabilityChangeEvent>(value),
        "binance_client::advanced_trading::portfolio_margin::stream_models::ListenKeyExpiredEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::ListenKeyExpiredEvent>(value),
        "binance_client::advanced_trading::portfolio_margin::stream_models::OpenOrderLossEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::OpenOrderLossEvent>(value),
        "binance_client::advanced_trading::portfolio_margin::stream_models::OrderTradeUpdateEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::OrderTradeUpdateEvent>(value),
        "binance_client::advanced_trading::portfolio_margin::stream_models::OutboundAccountPositionEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::OutboundAccountPositionEvent>(value),
        "binance_client::advanced_trading::portfolio_margin::stream_models::RiskLevelChangeEvent"=>typed::<binance_client::advanced_trading::portfolio_margin::stream_models::RiskLevelChangeEvent>(value),
        "binance_client::advanced_trading::portfolio_margin_pro::stream_models::PmProAccountUpdateEvent"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::stream_models::PmProAccountUpdateEvent>(value),
        "binance_client::advanced_trading::portfolio_margin_pro::stream_models::RisklevelchangeEvent"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::stream_models::RisklevelchangeEvent>(value),
        _=>check_5(model,value),
    }
}
fn check_5(model: &str, value: Value) -> Result<(), String> {
    match model {
        "binance_client::advanced_trading::stocks::stream_models::CalendarStreamEvent" => {
            typed::<binance_client::advanced_trading::stocks::stream_models::CalendarStreamEvent>(
                value,
            )
        }
        "binance_client::advanced_trading::stocks::stream_models::KlineStreamEvent" => {
            typed::<binance_client::advanced_trading::stocks::stream_models::KlineStreamEvent>(
                value,
            )
        }
        "binance_client::advanced_trading::stocks::stream_models::OrderReportStreamEvent" => {
            typed::<binance_client::advanced_trading::stocks::stream_models::OrderReportStreamEvent>(
                value,
            )
        }
        "binance_client::advanced_trading::stocks::stream_models::PriceStreamEvent" => {
            typed::<binance_client::advanced_trading::stocks::stream_models::PriceStreamEvent>(
                value,
            )
        }
        "binance_client::advanced_trading::stocks::stream_models::QuoteStreamEvent" => {
            typed::<binance_client::advanced_trading::stocks::stream_models::QuoteStreamEvent>(
                value,
            )
        }
        "binance_client::advanced_trading::stocks::stream_models::TradabilityStreamEvent" => {
            typed::<binance_client::advanced_trading::stocks::stream_models::TradabilityStreamEvent>(
                value,
            )
        }
        "binance_client::advanced_trading::stocks::stream_models::TradingStatusStreamEvent" => {
            typed::<binance_client::advanced_trading::stocks::stream_models::TradingStatusStreamEvent>(
                value,
            )
        }
        "binance_client::core_trading::coinm::stream_models::AccountConfigUpdateEvent" => {
            typed::<binance_client::core_trading::coinm::stream_models::AccountConfigUpdateEvent>(
                value,
            )
        }
        _ => check_6(model, value),
    }
}
fn check_6(model: &str, value: Value) -> Result<(), String> {
    match model{
        "binance_client::core_trading::coinm::stream_models::AccountUpdateEvent"=>typed::<binance_client::core_trading::coinm::stream_models::AccountUpdateEvent>(value),
        "binance_client::core_trading::coinm::stream_models::AggregateTradeStreamsEvent"=>typed::<binance_client::core_trading::coinm::stream_models::AggregateTradeStreamsEvent>(value),
        "binance_client::core_trading::coinm::stream_models::AllBookTickersStreamEvent"=>typed::<binance_client::core_trading::coinm::stream_models::AllBookTickersStreamEvent>(value),
        "binance_client::core_trading::coinm::stream_models::AllMarketLiquidationOrderStreamsEvent"=>typed::<binance_client::core_trading::coinm::stream_models::AllMarketLiquidationOrderStreamsEvent>(value),
        "binance_client::core_trading::coinm::stream_models::ContinuousContractKlineCandlestickStreamsEvent"=>typed::<binance_client::core_trading::coinm::stream_models::ContinuousContractKlineCandlestickStreamsEvent>(value),
        "binance_client::core_trading::coinm::stream_models::ContractInfoStreamEvent"=>typed::<binance_client::core_trading::coinm::stream_models::ContractInfoStreamEvent>(value),
        "binance_client::core_trading::coinm::stream_models::DiffBookDepthStreamsEvent"=>typed::<binance_client::core_trading::coinm::stream_models::DiffBookDepthStreamsEvent>(value),
        "binance_client::core_trading::coinm::stream_models::GridUpdateEvent"=>typed::<binance_client::core_trading::coinm::stream_models::GridUpdateEvent>(value),
        _=>check_7(model,value),
    }
}
fn check_7(model: &str, value: Value) -> Result<(), String> {
    match model{
        "binance_client::core_trading::coinm::stream_models::IndexKlineCandlestickStreamsEvent"=>typed::<binance_client::core_trading::coinm::stream_models::IndexKlineCandlestickStreamsEvent>(value),
        "binance_client::core_trading::coinm::stream_models::IndexPriceStreamEvent"=>typed::<binance_client::core_trading::coinm::stream_models::IndexPriceStreamEvent>(value),
        "binance_client::core_trading::coinm::stream_models::IndividualSymbolBookTickerStreamsEvent"=>typed::<binance_client::core_trading::coinm::stream_models::IndividualSymbolBookTickerStreamsEvent>(value),
        "binance_client::core_trading::coinm::stream_models::IndividualSymbolMiniTickerStreamEvent"=>typed::<binance_client::core_trading::coinm::stream_models::IndividualSymbolMiniTickerStreamEvent>(value),
        "binance_client::core_trading::coinm::stream_models::IndividualSymbolTickerStreamsEvent"=>typed::<binance_client::core_trading::coinm::stream_models::IndividualSymbolTickerStreamsEvent>(value),
        "binance_client::core_trading::coinm::stream_models::KlineCandlestickStreamsEvent"=>typed::<binance_client::core_trading::coinm::stream_models::KlineCandlestickStreamsEvent>(value),
        "binance_client::core_trading::coinm::stream_models::ListenKeyExpiredEvent"=>typed::<binance_client::core_trading::coinm::stream_models::ListenKeyExpiredEvent>(value),
        "binance_client::core_trading::coinm::stream_models::MarginCallEvent"=>typed::<binance_client::core_trading::coinm::stream_models::MarginCallEvent>(value),
        _=>check_8(model,value),
    }
}
fn check_8(model: &str, value: Value) -> Result<(), String> {
    match model{
        "binance_client::core_trading::coinm::stream_models::MarkPriceKlineCandlestickStreamsEvent"=>typed::<binance_client::core_trading::coinm::stream_models::MarkPriceKlineCandlestickStreamsEvent>(value),
        "binance_client::core_trading::coinm::stream_models::MarkPriceStreamEvent"=>typed::<binance_client::core_trading::coinm::stream_models::MarkPriceStreamEvent>(value),
        "binance_client::core_trading::coinm::stream_models::MarketLiquidationOrderStreamsEvent"=>typed::<binance_client::core_trading::coinm::stream_models::MarketLiquidationOrderStreamsEvent>(value),
        "binance_client::core_trading::coinm::stream_models::OrderTradeUpdateEvent"=>typed::<binance_client::core_trading::coinm::stream_models::OrderTradeUpdateEvent>(value),
        "binance_client::core_trading::coinm::stream_models::PartialBookDepthStreamsEvent"=>typed::<binance_client::core_trading::coinm::stream_models::PartialBookDepthStreamsEvent>(value),
        "binance_client::core_trading::coinm::stream_models::StrategyUpdateEvent"=>typed::<binance_client::core_trading::coinm::stream_models::StrategyUpdateEvent>(value),
        "binance_client::core_trading::usdm::stream_models::AccountConfigUpdateEvent"=>typed::<binance_client::core_trading::usdm::stream_models::AccountConfigUpdateEvent>(value),
        "binance_client::core_trading::usdm::stream_models::AccountUpdateEvent"=>typed::<binance_client::core_trading::usdm::stream_models::AccountUpdateEvent>(value),
        _=>check_9(model,value),
    }
}
fn check_9(model: &str, value: Value) -> Result<(), String> {
    match model{
        "binance_client::core_trading::usdm::stream_models::AggregateTradeStreamsEvent"=>typed::<binance_client::core_trading::usdm::stream_models::AggregateTradeStreamsEvent>(value),
        "binance_client::core_trading::usdm::stream_models::AlgoUpdateEvent"=>typed::<binance_client::core_trading::usdm::stream_models::AlgoUpdateEvent>(value),
        "binance_client::core_trading::usdm::stream_models::AllBookTickersStreamEvent"=>typed::<binance_client::core_trading::usdm::stream_models::AllBookTickersStreamEvent>(value),
        "binance_client::core_trading::usdm::stream_models::AllMarketLiquidationOrderStreamsEvent"=>typed::<binance_client::core_trading::usdm::stream_models::AllMarketLiquidationOrderStreamsEvent>(value),
        "binance_client::core_trading::usdm::stream_models::CompositeIndexSymbolInformationStreamsEvent"=>typed::<binance_client::core_trading::usdm::stream_models::CompositeIndexSymbolInformationStreamsEvent>(value),
        "binance_client::core_trading::usdm::stream_models::ConditionalOrderTriggerRejectEvent"=>typed::<binance_client::core_trading::usdm::stream_models::ConditionalOrderTriggerRejectEvent>(value),
        "binance_client::core_trading::usdm::stream_models::ContinuousContractKlineCandlestickStreamsEvent"=>typed::<binance_client::core_trading::usdm::stream_models::ContinuousContractKlineCandlestickStreamsEvent>(value),
        "binance_client::core_trading::usdm::stream_models::ContractInfoStreamEvent"=>typed::<binance_client::core_trading::usdm::stream_models::ContractInfoStreamEvent>(value),
        _=>check_10(model,value),
    }
}
fn check_10(model: &str, value: Value) -> Result<(), String> {
    match model{
        "binance_client::core_trading::usdm::stream_models::DiffBookDepthStreamsEvent"=>typed::<binance_client::core_trading::usdm::stream_models::DiffBookDepthStreamsEvent>(value),
        "binance_client::core_trading::usdm::stream_models::GridUpdateEvent"=>typed::<binance_client::core_trading::usdm::stream_models::GridUpdateEvent>(value),
        "binance_client::core_trading::usdm::stream_models::IndividualSymbolBookTickerStreamsEvent"=>typed::<binance_client::core_trading::usdm::stream_models::IndividualSymbolBookTickerStreamsEvent>(value),
        "binance_client::core_trading::usdm::stream_models::IndividualSymbolMiniTickerStreamEvent"=>typed::<binance_client::core_trading::usdm::stream_models::IndividualSymbolMiniTickerStreamEvent>(value),
        "binance_client::core_trading::usdm::stream_models::IndividualSymbolTickerStreamsEvent"=>typed::<binance_client::core_trading::usdm::stream_models::IndividualSymbolTickerStreamsEvent>(value),
        "binance_client::core_trading::usdm::stream_models::KlineCandlestickStreamsEvent"=>typed::<binance_client::core_trading::usdm::stream_models::KlineCandlestickStreamsEvent>(value),
        "binance_client::core_trading::usdm::stream_models::LiquidationOrderStreamsEvent"=>typed::<binance_client::core_trading::usdm::stream_models::LiquidationOrderStreamsEvent>(value),
        "binance_client::core_trading::usdm::stream_models::ListenKeyExpiredEvent"=>typed::<binance_client::core_trading::usdm::stream_models::ListenKeyExpiredEvent>(value),
        _=>check_11(model,value),
    }
}
fn check_11(model: &str, value: Value) -> Result<(), String> {
    match model {
        "binance_client::core_trading::usdm::stream_models::MarginCallEvent" => {
            typed::<binance_client::core_trading::usdm::stream_models::MarginCallEvent>(value)
        }
        "binance_client::core_trading::usdm::stream_models::MarkPriceStreamEvent" => {
            typed::<binance_client::core_trading::usdm::stream_models::MarkPriceStreamEvent>(value)
        }
        "binance_client::core_trading::usdm::stream_models::OrderTradeUpdateEvent" => {
            typed::<binance_client::core_trading::usdm::stream_models::OrderTradeUpdateEvent>(value)
        }
        "binance_client::core_trading::usdm::stream_models::PartialBookDepthStreamsEvent" => {
            typed::<binance_client::core_trading::usdm::stream_models::PartialBookDepthStreamsEvent>(
                value,
            )
        }
        "binance_client::core_trading::usdm::stream_models::RpiDiffBookDepthStreamsEvent" => {
            typed::<binance_client::core_trading::usdm::stream_models::RpiDiffBookDepthStreamsEvent>(
                value,
            )
        }
        "binance_client::core_trading::usdm::stream_models::StrategyUpdateEvent" => {
            typed::<binance_client::core_trading::usdm::stream_models::StrategyUpdateEvent>(value)
        }
        "binance_client::core_trading::usdm::stream_models::TradeLiteEvent" => {
            typed::<binance_client::core_trading::usdm::stream_models::TradeLiteEvent>(value)
        }
        "binance_client::core_trading::usdm::stream_models::TradingSessionStreamEvent" => {
            typed::<binance_client::core_trading::usdm::stream_models::TradingSessionStreamEvent>(
                value,
            )
        }
        _ => check_12(model, value),
    }
}
fn check_12(model: &str, value: Value) -> Result<(), String> {
    match model {
        "binance_client::margin::event_payloads::UserPayload" => known(
            binance_client::margin::event_payloads::user_payload(value)
                .map_err(|e| e.to_string())?,
        ),
        "binance_client::margin::stream_models::BalanceUpdateEvent" => {
            typed::<binance_client::margin::stream_models::BalanceUpdateEvent>(value)
        }
        "binance_client::margin::stream_models::ExecutionReportEvent" => {
            typed::<binance_client::margin::stream_models::ExecutionReportEvent>(value)
        }
        "binance_client::margin::stream_models::LiabilityChangeEvent" => {
            typed::<binance_client::margin::stream_models::LiabilityChangeEvent>(value)
        }
        "binance_client::margin::stream_models::ListStatusEvent" => {
            typed::<binance_client::margin::stream_models::ListStatusEvent>(value)
        }
        "binance_client::margin::stream_models::ListenKeyExpiredEvent" => {
            typed::<binance_client::margin::stream_models::ListenKeyExpiredEvent>(value)
        }
        "binance_client::margin::stream_models::MarginLevelChangeEvent" => {
            typed::<binance_client::margin::stream_models::MarginLevelChangeEvent>(value)
        }
        "binance_client::margin::stream_models::MarginLevelStatusChangeEvent" => {
            typed::<binance_client::margin::stream_models::MarginLevelStatusChangeEvent>(value)
        }
        _ => check_13(model, value),
    }
}
fn check_13(model: &str, value: Value) -> Result<(), String> {
    match model {
        "binance_client::margin::stream_models::OutboundAccountPositionEvent" => {
            typed::<binance_client::margin::stream_models::OutboundAccountPositionEvent>(value)
        }
        "binance_client::margin::stream_models::UserLiabilityChangeEvent" => {
            typed::<binance_client::margin::stream_models::UserLiabilityChangeEvent>(value)
        }
        "binance_client::options::stream_models::AccountUpdateEvent" => {
            typed::<binance_client::options::stream_models::AccountUpdateEvent>(value)
        }
        "binance_client::options::stream_models::BalancePositionUpdateEvent" => {
            typed::<binance_client::options::stream_models::BalancePositionUpdateEvent>(value)
        }
        "binance_client::options::stream_models::DiffBookDepthStreamsEvent" => {
            typed::<binance_client::options::stream_models::DiffBookDepthStreamsEvent>(value)
        }
        "binance_client::options::stream_models::GreekUpdateEvent" => {
            typed::<binance_client::options::stream_models::GreekUpdateEvent>(value)
        }
        "binance_client::options::stream_models::Hour24TickerEvent" => {
            typed::<binance_client::options::stream_models::Hour24TickerEvent>(value)
        }
        "binance_client::options::stream_models::IndividualSymbolBookTickerStreamsEvent" => {
            typed::<binance_client::options::stream_models::IndividualSymbolBookTickerStreamsEvent>(
                value,
            )
        }
        _ => check_14(model, value),
    }
}
fn check_14(model: &str, value: Value) -> Result<(), String> {
    match model {
        "binance_client::options::stream_models::KlineCandlestickStreamsEvent" => {
            typed::<binance_client::options::stream_models::KlineCandlestickStreamsEvent>(value)
        }
        "binance_client::options::stream_models::ListenKeyExpiredEvent" => {
            typed::<binance_client::options::stream_models::ListenKeyExpiredEvent>(value)
        }
        "binance_client::options::stream_models::NewSymbolInfoEvent" => {
            typed::<binance_client::options::stream_models::NewSymbolInfoEvent>(value)
        }
        "binance_client::options::stream_models::OrderTradeUpdateEvent" => {
            typed::<binance_client::options::stream_models::OrderTradeUpdateEvent>(value)
        }
        "binance_client::options::stream_models::PartialBookDepthStreamsEvent" => {
            typed::<binance_client::options::stream_models::PartialBookDepthStreamsEvent>(value)
        }
        "binance_client::options::stream_models::RiskLevelChangeEvent" => {
            typed::<binance_client::options::stream_models::RiskLevelChangeEvent>(value)
        }
        "binance_client::options::stream_models::TradeStreamsEvent" => {
            typed::<binance_client::options::stream_models::TradeStreamsEvent>(value)
        }
        "binance_client::spot::stream_models::AggTradeEvent" => {
            typed::<binance_client::spot::stream_models::AggTradeEvent>(value)
        }
        _ => check_15(model, value),
    }
}
fn check_15(model: &str, value: Value) -> Result<(), String> {
    match model {
        "binance_client::spot::stream_models::AvgPriceEvent" => {
            typed::<binance_client::spot::stream_models::AvgPriceEvent>(value)
        }
        "binance_client::spot::stream_models::BalanceUpdateEvent" => {
            typed::<binance_client::spot::stream_models::BalanceUpdateEvent>(value)
        }
        "binance_client::spot::stream_models::BlockTradeEvent" => {
            typed::<binance_client::spot::stream_models::BlockTradeEvent>(value)
        }
        "binance_client::spot::stream_models::BookTickerEvent" => {
            typed::<binance_client::spot::stream_models::BookTickerEvent>(value)
        }
        "binance_client::spot::stream_models::DiffBookDepthEvent" => {
            typed::<binance_client::spot::stream_models::DiffBookDepthEvent>(value)
        }
        "binance_client::spot::stream_models::EventStreamTerminatedEvent" => {
            typed::<binance_client::spot::stream_models::EventStreamTerminatedEvent>(value)
        }
        "binance_client::spot::stream_models::ExecutionReportEvent" => {
            typed::<binance_client::spot::stream_models::ExecutionReportEvent>(value)
        }
        "binance_client::spot::stream_models::ExternalLockUpdateEvent" => {
            typed::<binance_client::spot::stream_models::ExternalLockUpdateEvent>(value)
        }
        _ => check_16(model, value),
    }
}
fn check_16(model: &str, value: Value) -> Result<(), String> {
    match model {
        "binance_client::spot::stream_models::KlineEvent" => {
            typed::<binance_client::spot::stream_models::KlineEvent>(value)
        }
        "binance_client::spot::stream_models::KlineOffsetEvent" => {
            typed::<binance_client::spot::stream_models::KlineOffsetEvent>(value)
        }
        "binance_client::spot::stream_models::ListStatusEvent" => {
            typed::<binance_client::spot::stream_models::ListStatusEvent>(value)
        }
        "binance_client::spot::stream_models::MiniTickerEvent" => {
            typed::<binance_client::spot::stream_models::MiniTickerEvent>(value)
        }
        "binance_client::spot::stream_models::OutboundAccountPositionEvent" => {
            typed::<binance_client::spot::stream_models::OutboundAccountPositionEvent>(value)
        }
        "binance_client::spot::stream_models::PartialBookDepthEvent" => {
            typed::<binance_client::spot::stream_models::PartialBookDepthEvent>(value)
        }
        "binance_client::spot::stream_models::ReferencePriceEvent" => {
            typed::<binance_client::spot::stream_models::ReferencePriceEvent>(value)
        }
        "binance_client::spot::stream_models::RollingWindowTickerEvent" => {
            typed::<binance_client::spot::stream_models::RollingWindowTickerEvent>(value)
        }
        _ => check_17(model, value),
    }
}
fn check_17(model: &str, value: Value) -> Result<(), String> {
    match model {
        "binance_client::spot::stream_models::TickerEvent" => {
            typed::<binance_client::spot::stream_models::TickerEvent>(value)
        }
        "binance_client::spot::stream_models::TradeEvent" => {
            typed::<binance_client::spot::stream_models::TradeEvent>(value)
        }
        _ => Err("missing native stream model".into()),
    }
}
