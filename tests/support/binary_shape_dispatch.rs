// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated native binary response checks, independent wire corpus.
use serde_json::Value;
fn typed<T: serde::de::DeserializeOwned + std::fmt::Debug>(
    value: Value,
    expectation: &str,
) -> Result<(), String> {
    let decoded = serde_json::from_value::<T>(value);
    if expectation == "refused_missing_subscription_identity" {
        return if decoded.is_err() {
            Ok(())
        } else {
            Err("null subscription identity accepted".into())
        };
    }
    let decoded = decoded.map_err(|e| e.to_string())?;
    let unknown = format!("{decoded:?}").contains("Unknown(UnknownMessage");
    if unknown != (expectation == "decoded_unknown_evidence") {
        return Err("native known/unknown evidence disposition mismatch".into());
    }
    Ok(())
}
pub(super) fn check(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    check_0(model, value, expectation)
}
fn check_0(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::event_payloads::SessionStatus" => {
            typed::<binance_client::spot::event_payloads::SessionStatus>(value, expectation)
        }
        "binance_client::spot::rest_models::AccountCommissionResponse" => typed::<
            binance_client::spot::rest_models::AccountCommissionResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::rest_models::AggTradesResponse" => {
            typed::<binance_client::spot::rest_models::AggTradesResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::AllOrderListResponse" => {
            typed::<binance_client::spot::rest_models::AllOrderListResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::AllOrdersResponse" => {
            typed::<binance_client::spot::rest_models::AllOrdersResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::AvgPriceResponse" => {
            typed::<binance_client::spot::rest_models::AvgPriceResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::DeleteOpenOrdersResponse" => {
            typed::<binance_client::spot::rest_models::DeleteOpenOrdersResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::DeleteOrderListResponse" => {
            typed::<binance_client::spot::rest_models::DeleteOrderListResponse>(value, expectation)
        }
        _ => check_1(model, value, expectation),
    }
}
fn check_1(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::rest_models::DeleteOrderResponse" => {
            typed::<binance_client::spot::rest_models::DeleteOrderResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::DepthResponse" => {
            typed::<binance_client::spot::rest_models::DepthResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::ExchangeInfoResponse" => {
            typed::<binance_client::spot::rest_models::ExchangeInfoResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::ExecutionRulesResponse" => {
            typed::<binance_client::spot::rest_models::ExecutionRulesResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::ExecutionRulesResponseSymbolRulesItemRulesItem" => {
            typed::<binance_client::spot::rest_models::ExecutionRulesResponseSymbolRulesItemRulesItem>(
                value,
                expectation,
            )
        }
        "binance_client::spot::rest_models::GetAccountResponse" => {
            typed::<binance_client::spot::rest_models::GetAccountResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::GetOpenOrdersResponse" => {
            typed::<binance_client::spot::rest_models::GetOpenOrdersResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::GetOrderListResponse" => {
            typed::<binance_client::spot::rest_models::GetOrderListResponse>(value, expectation)
        }
        _ => check_2(model, value, expectation),
    }
}
fn check_2(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::rest_models::GetOrderResponse" => {
            typed::<binance_client::spot::rest_models::GetOrderResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::GetTradesResponse" => {
            typed::<binance_client::spot::rest_models::GetTradesResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::HistoricalBlockTradesResponse" => typed::<
            binance_client::spot::rest_models::HistoricalBlockTradesResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::rest_models::HistoricalTradesResponse" => {
            typed::<binance_client::spot::rest_models::HistoricalTradesResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::KlinesResponse" => {
            typed::<binance_client::spot::rest_models::KlinesResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::MyAllocationsResponse" => {
            typed::<binance_client::spot::rest_models::MyAllocationsResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::MyFiltersResponse" => {
            typed::<binance_client::spot::rest_models::MyFiltersResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::MyFiltersResponseAssetFiltersItem" => {
            typed::<binance_client::spot::rest_models::MyFiltersResponseAssetFiltersItem>(
                value,
                expectation,
            )
        }
        _ => check_3(model, value, expectation),
    }
}
fn check_3(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::rest_models::MyFiltersResponseExchangeFiltersItem" => {
            typed::<binance_client::spot::rest_models::MyFiltersResponseExchangeFiltersItem>(
                value,
                expectation,
            )
        }
        "binance_client::spot::rest_models::MyFiltersResponseSymbolFiltersItem" => {
            typed::<binance_client::spot::rest_models::MyFiltersResponseSymbolFiltersItem>(
                value,
                expectation,
            )
        }
        "binance_client::spot::rest_models::MyPreventedMatchesResponse" => typed::<
            binance_client::spot::rest_models::MyPreventedMatchesResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::rest_models::MyTradesResponse" => {
            typed::<binance_client::spot::rest_models::MyTradesResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::NewOrderResponse" => {
            typed::<binance_client::spot::rest_models::NewOrderResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::OrderAmendKeepPriorityResponse" => typed::<
            binance_client::spot::rest_models::OrderAmendKeepPriorityResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::rest_models::OrderAmendmentsResponse" => {
            typed::<binance_client::spot::rest_models::OrderAmendmentsResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::OrderCancelReplaceResponse" => typed::<
            binance_client::spot::rest_models::OrderCancelReplaceResponse,
        >(
            value, expectation
        ),
        _ => check_4(model, value, expectation),
    }
}
fn check_4(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::rest_models::OrderListOcoResponse" => {
            typed::<binance_client::spot::rest_models::OrderListOcoResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::OrderListOtoResponse" => {
            typed::<binance_client::spot::rest_models::OrderListOtoResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::OrderListOtocoResponse" => {
            typed::<binance_client::spot::rest_models::OrderListOtocoResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::OrderTestResponse" => {
            typed::<binance_client::spot::rest_models::OrderTestResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::PingResponse" => {
            typed::<binance_client::spot::rest_models::PingResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::RateLimitOrderResponse" => {
            typed::<binance_client::spot::rest_models::RateLimitOrderResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::ReferencePriceCalculationResponse" => {
            typed::<binance_client::spot::rest_models::ReferencePriceCalculationResponse>(
                value,
                expectation,
            )
        }
        "binance_client::spot::rest_models::ReferencePriceResponse" => {
            typed::<binance_client::spot::rest_models::ReferencePriceResponse>(value, expectation)
        }
        _ => check_5(model, value, expectation),
    }
}
fn check_5(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::rest_models::SorOrderResponse" => {
            typed::<binance_client::spot::rest_models::SorOrderResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::SorOrderTestResponse" => {
            typed::<binance_client::spot::rest_models::SorOrderTestResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::Ticker24hrResponse" => {
            typed::<binance_client::spot::rest_models::Ticker24hrResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::TickerBookTickerResponse" => {
            typed::<binance_client::spot::rest_models::TickerBookTickerResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::TickerPriceResponse" => {
            typed::<binance_client::spot::rest_models::TickerPriceResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::TickerResponse" => {
            typed::<binance_client::spot::rest_models::TickerResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::TickerTradingDayResponse" => {
            typed::<binance_client::spot::rest_models::TickerTradingDayResponse>(value, expectation)
        }
        "binance_client::spot::rest_models::TimeResponse" => {
            typed::<binance_client::spot::rest_models::TimeResponse>(value, expectation)
        }
        _ => check_6(model, value, expectation),
    }
}
fn check_6(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::rest_models::UiKlinesResponse" => {
            typed::<binance_client::spot::rest_models::UiKlinesResponse>(value, expectation)
        }
        "binance_client::spot::stream_models::BalanceUpdateEvent" => {
            typed::<binance_client::spot::stream_models::BalanceUpdateEvent>(value, expectation)
        }
        "binance_client::spot::stream_models::EventStreamTerminatedEvent" => typed::<
            binance_client::spot::stream_models::EventStreamTerminatedEvent,
        >(
            value, expectation
        ),
        "binance_client::spot::stream_models::ExecutionReportEvent" => {
            typed::<binance_client::spot::stream_models::ExecutionReportEvent>(value, expectation)
        }
        "binance_client::spot::stream_models::ExternalLockUpdateEvent" => typed::<
            binance_client::spot::stream_models::ExternalLockUpdateEvent,
        >(
            value, expectation
        ),
        "binance_client::spot::stream_models::ListStatusEvent" => {
            typed::<binance_client::spot::stream_models::ListStatusEvent>(value, expectation)
        }
        "binance_client::spot::stream_models::OutboundAccountPositionEvent" => typed::<
            binance_client::spot::stream_models::OutboundAccountPositionEvent,
        >(
            value, expectation
        ),
        "binance_client::spot::ws_models::AccountCommissionResponse" => {
            typed::<binance_client::spot::ws_models::AccountCommissionResponse>(value, expectation)
        }
        _ => check_7(model, value, expectation),
    }
}
fn check_7(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::ws_models::AccountRateLimitsOrdersResponse" => typed::<
            binance_client::spot::ws_models::AccountRateLimitsOrdersResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::ws_models::AccountStatusResponse" => {
            typed::<binance_client::spot::ws_models::AccountStatusResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::AllOrdersResponse" => {
            typed::<binance_client::spot::ws_models::AllOrdersResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::AvgPriceResponse" => {
            typed::<binance_client::spot::ws_models::AvgPriceResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::BlockTradesHistoricalResponse" => typed::<
            binance_client::spot::ws_models::BlockTradesHistoricalResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::ws_models::DepthResponse" => {
            typed::<binance_client::spot::ws_models::DepthResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::ExchangeInfoResponse" => {
            typed::<binance_client::spot::ws_models::ExchangeInfoResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::ExecutionRulesResponse" => {
            typed::<binance_client::spot::ws_models::ExecutionRulesResponse>(value, expectation)
        }
        _ => check_8(model, value, expectation),
    }
}
fn check_8(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::ws_models::ExecutionRulesResponseSymbolRulesItemRulesItem" => {
            typed::<binance_client::spot::ws_models::ExecutionRulesResponseSymbolRulesItemRulesItem>(
                value,
                expectation,
            )
        }
        "binance_client::spot::ws_models::KlinesResponse" => {
            typed::<binance_client::spot::ws_models::KlinesResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::MyAllocationsResponse" => {
            typed::<binance_client::spot::ws_models::MyAllocationsResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::MyFiltersResponse" => {
            typed::<binance_client::spot::ws_models::MyFiltersResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::MyFiltersResponseAssetFiltersItem" => {
            typed::<binance_client::spot::ws_models::MyFiltersResponseAssetFiltersItem>(
                value,
                expectation,
            )
        }
        "binance_client::spot::ws_models::MyFiltersResponseExchangeFiltersItem" => {
            typed::<binance_client::spot::ws_models::MyFiltersResponseExchangeFiltersItem>(
                value,
                expectation,
            )
        }
        "binance_client::spot::ws_models::MyFiltersResponseSymbolFiltersItem" => {
            typed::<binance_client::spot::ws_models::MyFiltersResponseSymbolFiltersItem>(
                value,
                expectation,
            )
        }
        "binance_client::spot::ws_models::MyPreventedMatchesResponse" => {
            typed::<binance_client::spot::ws_models::MyPreventedMatchesResponse>(value, expectation)
        }
        _ => check_9(model, value, expectation),
    }
}
fn check_9(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::ws_models::MyTradesResponse" => {
            typed::<binance_client::spot::ws_models::MyTradesResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::OpenOrderListsStatusResponse" => typed::<
            binance_client::spot::ws_models::OpenOrderListsStatusResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::ws_models::OpenOrdersCancelAllResponse" => typed::<
            binance_client::spot::ws_models::OpenOrdersCancelAllResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::ws_models::OpenOrdersStatusResponse" => {
            typed::<binance_client::spot::ws_models::OpenOrdersStatusResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::OrderAmendKeepPriorityResponse" => typed::<
            binance_client::spot::ws_models::OrderAmendKeepPriorityResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::ws_models::OrderAmendmentsResponse" => {
            typed::<binance_client::spot::ws_models::OrderAmendmentsResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::OrderCancelReplaceResponse" => {
            typed::<binance_client::spot::ws_models::OrderCancelReplaceResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::OrderCancelResponse" => {
            typed::<binance_client::spot::ws_models::OrderCancelResponse>(value, expectation)
        }
        _ => check_10(model, value, expectation),
    }
}
fn check_10(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::ws_models::OrderListCancelResponse" => {
            typed::<binance_client::spot::ws_models::OrderListCancelResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::OrderListPlaceOcoResponse" => {
            typed::<binance_client::spot::ws_models::OrderListPlaceOcoResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::OrderListPlaceOtoResponse" => {
            typed::<binance_client::spot::ws_models::OrderListPlaceOtoResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::OrderListPlaceOtocoResponse" => typed::<
            binance_client::spot::ws_models::OrderListPlaceOtocoResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::ws_models::OrderListStatusResponse" => {
            typed::<binance_client::spot::ws_models::OrderListStatusResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::OrderPlaceResponse" => {
            typed::<binance_client::spot::ws_models::OrderPlaceResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::OrderStatusResponse" => {
            typed::<binance_client::spot::ws_models::OrderStatusResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::OrderTestResponse" => {
            typed::<binance_client::spot::ws_models::OrderTestResponse>(value, expectation)
        }
        _ => check_11(model, value, expectation),
    }
}
fn check_11(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::ws_models::PingResponse" => {
            typed::<binance_client::spot::ws_models::PingResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::ReferencePriceCalculationResponse" => {
            typed::<binance_client::spot::ws_models::ReferencePriceCalculationResponse>(
                value,
                expectation,
            )
        }
        "binance_client::spot::ws_models::ReferencePriceResponse" => {
            typed::<binance_client::spot::ws_models::ReferencePriceResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::SessionSubscriptionsResponse" => typed::<
            binance_client::spot::ws_models::SessionSubscriptionsResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::ws_models::SorOrderPlaceResponse" => {
            typed::<binance_client::spot::ws_models::SorOrderPlaceResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::SorOrderTestResponse" => {
            typed::<binance_client::spot::ws_models::SorOrderTestResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::Ticker24hrResponse" => {
            typed::<binance_client::spot::ws_models::Ticker24hrResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::TickerBookResponse" => {
            typed::<binance_client::spot::ws_models::TickerBookResponse>(value, expectation)
        }
        _ => check_12(model, value, expectation),
    }
}
fn check_12(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::ws_models::TickerPriceResponse" => {
            typed::<binance_client::spot::ws_models::TickerPriceResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::TickerResponse" => {
            typed::<binance_client::spot::ws_models::TickerResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::TickerTradingDayResponse" => {
            typed::<binance_client::spot::ws_models::TickerTradingDayResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::TimeResponse" => {
            typed::<binance_client::spot::ws_models::TimeResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::TradesAggregateResponse" => {
            typed::<binance_client::spot::ws_models::TradesAggregateResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::TradesHistoricalResponse" => {
            typed::<binance_client::spot::ws_models::TradesHistoricalResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::TradesRecentResponse" => {
            typed::<binance_client::spot::ws_models::TradesRecentResponse>(value, expectation)
        }
        "binance_client::spot::ws_models::UiKlinesResponse" => {
            typed::<binance_client::spot::ws_models::UiKlinesResponse>(value, expectation)
        }
        _ => check_13(model, value, expectation),
    }
}
fn check_13(model: &str, value: Value, expectation: &str) -> Result<(), String> {
    match model {
        "binance_client::spot::ws_models::UserDataStreamSubscribeResponse" => typed::<
            binance_client::spot::ws_models::UserDataStreamSubscribeResponse,
        >(
            value, expectation
        ),
        "binance_client::spot::ws_models::UserDataStreamSubscribeSignatureResponse" => {
            typed::<binance_client::spot::ws_models::UserDataStreamSubscribeSignatureResponse>(
                value,
                expectation,
            )
        }
        "binance_client::spot::ws_models::UserDataStreamUnsubscribeResponse" => {
            typed::<binance_client::spot::ws_models::UserDataStreamUnsubscribeResponse>(
                value,
                expectation,
            )
        }
        _ => Err("missing binary native model".into()),
    }
}
