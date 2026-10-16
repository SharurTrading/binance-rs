// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated response-fixture dispatch; fixtures come from independent references.
use serde_json::Value;
fn typed<T: serde::de::DeserializeOwned + std::fmt::Debug>(value: Value) -> Result<(), String> {
    let decoded: T = serde_json::from_value(value).map_err(|e| e.to_string())?;
    if format!("{decoded:?}").contains("Unknown(UnknownMessage") {
        return Err("documented member decoded as unknown evidence".into());
    }
    Ok(())
}
pub(super) fn check(
    product: &str,
    transport: &str,
    operation: &str,
    value: Value,
) -> Result<(), String> {
    match (product, transport) {
        ("algo", "rest") => algo_rest_0(operation, value),
        ("alpha", "rest") => alpha_rest_0(operation, value),
        ("coinm", "rest") => coinm_rest_0(operation, value),
        ("coinm", "ws") => coinm_ws_0(operation, value),
        ("convert", "rest") => convert_rest_0(operation, value),
        ("copy_trading", "rest") => copy_trading_rest_0(operation, value),
        ("margin", "rest") => margin_rest_0(operation, value),
        ("options", "rest") => options_rest_0(operation, value),
        ("portfolio_margin_pro", "rest") => portfolio_margin_pro_rest_0(operation, value),
        ("portfolio_margin", "rest") => portfolio_margin_rest_0(operation, value),
        ("spot", "rest") => spot_rest_0(operation, value),
        ("spot", "ws") => spot_ws_0(operation, value),
        ("stocks", "rest") => stocks_rest_0(operation, value),
        ("usdm", "rest") => usdm_rest_0(operation, value),
        ("usdm", "ws") => usdm_ws_0(operation, value),
        ("wallet", "rest") => wallet_rest_0(operation, value),
        ("spot", "session") => spot_session_0(operation, value),
        ("usdm", "session") => usdm_session_0(operation, value),
        ("coinm", "session") => coinm_session_0(operation, value),
        ("margin", "ws") => margin_ws_0(operation, value),
        _ => Err("response fixture dispatch missing product/transport".into()),
    }
}
fn algo_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "cancelAlgoOrderFutureAlgo"=>typed::<binance_client::advanced_trading::algo::rest_models::CancelAlgoOrderFutureAlgoResponse>(value),
        "queryCurrentAlgoOpenOrdersFutureAlgo"=>typed::<binance_client::advanced_trading::algo::rest_models::QueryCurrentAlgoOpenOrdersFutureAlgoResponse>(value),
        "queryHistoricalAlgoOrdersFutureAlgo"=>typed::<binance_client::advanced_trading::algo::rest_models::QueryHistoricalAlgoOrdersFutureAlgoResponse>(value),
        "querySubOrdersFutureAlgo"=>typed::<binance_client::advanced_trading::algo::rest_models::QuerySubOrdersFutureAlgoResponse>(value),
        "timeWeightedAveragePriceFutureAlgo"=>typed::<binance_client::advanced_trading::algo::rest_models::TimeWeightedAveragePriceFutureAlgoResponse>(value),
        "volumeParticipationFutureAlgo"=>typed::<binance_client::advanced_trading::algo::rest_models::VolumeParticipationFutureAlgoResponse>(value),
        "cancelAlgoOrderSpotAlgo"=>typed::<binance_client::advanced_trading::algo::rest_models::CancelAlgoOrderSpotAlgoResponse>(value),
        "queryCurrentAlgoOpenOrdersSpotAlgo"=>typed::<binance_client::advanced_trading::algo::rest_models::QueryCurrentAlgoOpenOrdersSpotAlgoResponse>(value),
        _=>algo_rest_1(operation,value),
    }
}
fn algo_rest_1(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "queryHistoricalAlgoOrdersSpotAlgo"=>typed::<binance_client::advanced_trading::algo::rest_models::QueryHistoricalAlgoOrdersSpotAlgoResponse>(value),
        "querySubOrdersSpotAlgo"=>typed::<binance_client::advanced_trading::algo::rest_models::QuerySubOrdersSpotAlgoResponse>(value),
        "timeWeightedAveragePriceSpotAlgo"=>typed::<binance_client::advanced_trading::algo::rest_models::TimeWeightedAveragePriceSpotAlgoResponse>(value),
        "volumeParticipation"=>typed::<binance_client::advanced_trading::algo::rest_models::VolumeParticipationResponse>(value),
        _=>Err("response fixture dispatch missing operation".into()),
    }
}
fn alpha_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "aggregatedTrades" => typed::<
            binance_client::advanced_trading::alpha::rest_models::AggregatedTradesResponse,
        >(value),
        "fullDepth" => {
            typed::<binance_client::advanced_trading::alpha::rest_models::FullDepthResponse>(value)
        }
        "getExchangeInfo" => typed::<
            binance_client::advanced_trading::alpha::rest_models::GetExchangeInfoResponse,
        >(value),
        "klines" => {
            typed::<binance_client::advanced_trading::alpha::rest_models::KlinesResponse>(value)
        }
        "ticker" => {
            typed::<binance_client::advanced_trading::alpha::rest_models::TickerResponse>(value)
        }
        "tokenList" => {
            typed::<binance_client::advanced_trading::alpha::rest_models::TokenListResponse>(value)
        }
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn coinm_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "accountInformation"=>typed::<binance_client::core_trading::coinm::rest_models::AccountInformationResponse>(value),
        "futuresAccountBalance"=>typed::<binance_client::core_trading::coinm::rest_models::FuturesAccountBalanceResponse>(value),
        "getCurrentPositionMode"=>typed::<binance_client::core_trading::coinm::rest_models::GetCurrentPositionModeResponse>(value),
        "changePositionMode"=>typed::<binance_client::core_trading::coinm::rest_models::ChangePositionModeResponse>(value),
        "getDownloadIdForFuturesOrderHistory"=>typed::<binance_client::core_trading::coinm::rest_models::GetDownloadIdForFuturesOrderHistoryResponse>(value),
        "getDownloadIdForFuturesTradeHistory"=>typed::<binance_client::core_trading::coinm::rest_models::GetDownloadIdForFuturesTradeHistoryResponse>(value),
        "getDownloadIdForFuturesTransactionHistory"=>typed::<binance_client::core_trading::coinm::rest_models::GetDownloadIdForFuturesTransactionHistoryResponse>(value),
        "getFuturesOrderHistoryDownloadLinkById"=>typed::<binance_client::core_trading::coinm::rest_models::GetFuturesOrderHistoryDownloadLinkByIdResponse>(value),
        _=>coinm_rest_1(operation,value),
    }
}
fn coinm_rest_1(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "getFuturesTradeDownloadLinkById"=>typed::<binance_client::core_trading::coinm::rest_models::GetFuturesTradeDownloadLinkByIdResponse>(value),
        "getFuturesTransactionHistoryDownloadLinkById"=>typed::<binance_client::core_trading::coinm::rest_models::GetFuturesTransactionHistoryDownloadLinkByIdResponse>(value),
        "getIncomeHistory"=>typed::<binance_client::core_trading::coinm::rest_models::GetIncomeHistoryResponse>(value),
        "notionalBracketForPair"=>typed::<binance_client::core_trading::coinm::rest_models::NotionalBracketForPairResponse>(value),
        "notionalBracketForSymbol"=>typed::<binance_client::core_trading::coinm::rest_models::NotionalBracketForSymbolResponse>(value),
        "userCommissionRate"=>typed::<binance_client::core_trading::coinm::rest_models::UserCommissionRateResponse>(value),
        "basis"=>typed::<binance_client::core_trading::coinm::rest_models::BasisResponse>(value),
        "checkServerTime"=>typed::<binance_client::core_trading::coinm::rest_models::CheckServerTimeResponse>(value),
        _=>coinm_rest_2(operation,value),
    }
}
fn coinm_rest_2(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "compressedAggregateTradesList"=>typed::<binance_client::core_trading::coinm::rest_models::CompressedAggregateTradesListResponse>(value),
        "continuousContractKlineCandlestickData"=>typed::<binance_client::core_trading::coinm::rest_models::ContinuousContractKlineCandlestickDataResponse>(value),
        "exchangeInformation"=>typed::<binance_client::core_trading::coinm::rest_models::ExchangeInformationResponse>(value),
        "getFundingRateHistoryOfPerpetualFutures"=>typed::<binance_client::core_trading::coinm::rest_models::GetFundingRateHistoryOfPerpetualFuturesResponse>(value),
        "fundingInfo"=>typed::<binance_client::core_trading::coinm::rest_models::FundingInfoResponse>(value),
        "indexPriceAndMarkPrice"=>typed::<binance_client::core_trading::coinm::rest_models::IndexPriceAndMarkPriceResponse>(value),
        "indexPriceKlineCandlestickData"=>typed::<binance_client::core_trading::coinm::rest_models::IndexPriceKlineCandlestickDataResponse>(value),
        "klineCandlestickData"=>typed::<binance_client::core_trading::coinm::rest_models::KlineCandlestickDataResponse>(value),
        _=>coinm_rest_3(operation,value),
    }
}
fn coinm_rest_3(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "longShortRatio" => {
            typed::<binance_client::core_trading::coinm::rest_models::LongShortRatioResponse>(value)
        }
        "markPriceKlineCandlestickData" => typed::<
            binance_client::core_trading::coinm::rest_models::MarkPriceKlineCandlestickDataResponse,
        >(value),
        "oldTradesLookup" => typed::<
            binance_client::core_trading::coinm::rest_models::OldTradesLookupResponse,
        >(value),
        "openInterest" => {
            typed::<binance_client::core_trading::coinm::rest_models::OpenInterestResponse>(value)
        }
        "openInterestStatistics" => typed::<
            binance_client::core_trading::coinm::rest_models::OpenInterestStatisticsResponse,
        >(value),
        "orderBook" => {
            typed::<binance_client::core_trading::coinm::rest_models::OrderBookResponse>(value)
        }
        "premiumIndexKlineData" => typed::<
            binance_client::core_trading::coinm::rest_models::PremiumIndexKlineDataResponse,
        >(value),
        "queryIndexPriceConstituents" => typed::<
            binance_client::core_trading::coinm::rest_models::QueryIndexPriceConstituentsResponse,
        >(value),
        _ => coinm_rest_4(operation, value),
    }
}
fn coinm_rest_4(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "recentTradesList"=>typed::<binance_client::core_trading::coinm::rest_models::RecentTradesListResponse>(value),
        "symbolOrderBookTicker"=>typed::<binance_client::core_trading::coinm::rest_models::SymbolOrderBookTickerResponse>(value),
        "symbolPriceTicker"=>typed::<binance_client::core_trading::coinm::rest_models::SymbolPriceTickerResponse>(value),
        "takerBuySellVolume"=>typed::<binance_client::core_trading::coinm::rest_models::TakerBuySellVolumeResponse>(value),
        "testConnectivity"=>typed::<binance_client::core_trading::coinm::rest_models::TestConnectivityResponse>(value),
        "ticker24hrPriceChangeStatistics"=>typed::<binance_client::core_trading::coinm::rest_models::Ticker24hrPriceChangeStatisticsResponse>(value),
        "topTraderLongShortRatioAccounts"=>typed::<binance_client::core_trading::coinm::rest_models::TopTraderLongShortRatioAccountsResponse>(value),
        "topTraderLongShortRatioPositions"=>typed::<binance_client::core_trading::coinm::rest_models::TopTraderLongShortRatioPositionsResponse>(value),
        _=>coinm_rest_5(operation,value),
    }
}
fn coinm_rest_5(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "accountTradeList" => typed::<
            binance_client::core_trading::coinm::rest_models::AccountTradeListResponse,
        >(value),
        "allOrders" => {
            typed::<binance_client::core_trading::coinm::rest_models::AllOrdersResponse>(value)
        }
        "autoCancelAllOpenOrders" => typed::<
            binance_client::core_trading::coinm::rest_models::AutoCancelAllOpenOrdersResponse,
        >(value),
        "cancelAllOpenOrders" => typed::<
            binance_client::core_trading::coinm::rest_models::CancelAllOpenOrdersResponse,
        >(value),
        "cancelMultipleOrders" => typed::<
            binance_client::core_trading::coinm::rest_models::CancelMultipleOrdersResponse,
        >(value),
        "modifyMultipleOrders" => typed::<
            binance_client::core_trading::coinm::rest_models::ModifyMultipleOrdersResponse,
        >(value),
        "placeMultipleOrders" => typed::<
            binance_client::core_trading::coinm::rest_models::PlaceMultipleOrdersResponse,
        >(value),
        "cancelOrder" => {
            typed::<binance_client::core_trading::coinm::rest_models::CancelOrderResponse>(value)
        }
        _ => coinm_rest_6(operation, value),
    }
}
fn coinm_rest_6(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "modifyOrder"=>typed::<binance_client::core_trading::coinm::rest_models::ModifyOrderResponse>(value),
        "newOrder"=>typed::<binance_client::core_trading::coinm::rest_models::NewOrderResponse>(value),
        "queryOrder"=>typed::<binance_client::core_trading::coinm::rest_models::QueryOrderResponse>(value),
        "changeInitialLeverage"=>typed::<binance_client::core_trading::coinm::rest_models::ChangeInitialLeverageResponse>(value),
        "changeMarginType"=>typed::<binance_client::core_trading::coinm::rest_models::ChangeMarginTypeResponse>(value),
        "currentAllOpenOrders"=>typed::<binance_client::core_trading::coinm::rest_models::CurrentAllOpenOrdersResponse>(value),
        "getOrderModifyHistory"=>typed::<binance_client::core_trading::coinm::rest_models::GetOrderModifyHistoryResponse>(value),
        "getPositionMarginChangeHistory"=>typed::<binance_client::core_trading::coinm::rest_models::GetPositionMarginChangeHistoryResponse>(value),
        _=>coinm_rest_7(operation,value),
    }
}
fn coinm_rest_7(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "modifyIsolatedPositionMargin" => typed::<
            binance_client::core_trading::coinm::rest_models::ModifyIsolatedPositionMarginResponse,
        >(value),
        "positionAdlQuantileEstimation" => typed::<
            binance_client::core_trading::coinm::rest_models::PositionAdlQuantileEstimationResponse,
        >(value),
        "positionInformation" => typed::<
            binance_client::core_trading::coinm::rest_models::PositionInformationResponse,
        >(value),
        "queryCurrentOpenOrder" => typed::<
            binance_client::core_trading::coinm::rest_models::QueryCurrentOpenOrderResponse,
        >(value),
        "usersForceOrders" => typed::<
            binance_client::core_trading::coinm::rest_models::UsersForceOrdersResponse,
        >(value),
        "closeUserDataStream" => typed::<
            binance_client::core_trading::coinm::rest_models::CloseUserDataStreamResponse,
        >(value),
        "keepaliveUserDataStream" => typed::<
            binance_client::core_trading::coinm::rest_models::KeepaliveUserDataStreamResponse,
        >(value),
        "startUserDataStream" => typed::<
            binance_client::core_trading::coinm::rest_models::StartUserDataStreamResponse,
        >(value),
        _ => coinm_rest_8(operation, value),
    }
}
fn coinm_rest_8(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "newAlgoOrder" => {
            typed::<binance_client::core_trading::coinm::rest_models::NewAlgoOrderResponse>(value)
        }
        "cancelAlgoOrder" => typed::<
            binance_client::core_trading::coinm::rest_models::CancelAlgoOrderResponse,
        >(value),
        "openAlgoOrders" => {
            typed::<binance_client::core_trading::coinm::rest_models::OpenAlgoOrdersResponse>(value)
        }
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn coinm_ws_0(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "accountInformation" => typed::<
            binance_client::core_trading::coinm::ws_models::AccountInformationResponse,
        >(value),
        "futuresAccountBalance" => typed::<
            binance_client::core_trading::coinm::ws_models::FuturesAccountBalanceResponse,
        >(value),
        "cancelOrder" => {
            typed::<binance_client::core_trading::coinm::ws_models::CancelOrderResponse>(value)
        }
        "modifyOrder" => {
            typed::<binance_client::core_trading::coinm::ws_models::ModifyOrderResponse>(value)
        }
        "newOrder" => {
            typed::<binance_client::core_trading::coinm::ws_models::NewOrderResponse>(value)
        }
        "positionInformation" => typed::<
            binance_client::core_trading::coinm::ws_models::PositionInformationResponse,
        >(value),
        "queryOrder" => {
            typed::<binance_client::core_trading::coinm::ws_models::QueryOrderResponse>(value)
        }
        "closeUserDataStream" => typed::<
            binance_client::core_trading::coinm::ws_models::CloseUserDataStreamResponse,
        >(value),
        _ => coinm_ws_1(operation, value),
    }
}
fn coinm_ws_1(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "keepaliveUserDataStream" => typed::<
            binance_client::core_trading::coinm::ws_models::KeepaliveUserDataStreamResponse,
        >(value),
        "startUserDataStream" => typed::<
            binance_client::core_trading::coinm::ws_models::StartUserDataStreamResponse,
        >(value),
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn convert_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "listAllConvertPairs"=>typed::<binance_client::core_trading::convert::rest_models::ListAllConvertPairsResponse>(value),
        "queryOrderQuantityPrecisionPerAsset"=>typed::<binance_client::core_trading::convert::rest_models::QueryOrderQuantityPrecisionPerAssetResponse>(value),
        "acceptQuote"=>typed::<binance_client::core_trading::convert::rest_models::AcceptQuoteResponse>(value),
        "cancelLimitOrder"=>typed::<binance_client::core_trading::convert::rest_models::CancelLimitOrderResponse>(value),
        "getConvertTradeHistory"=>typed::<binance_client::core_trading::convert::rest_models::GetConvertTradeHistoryResponse>(value),
        "orderStatus"=>typed::<binance_client::core_trading::convert::rest_models::OrderStatusResponse>(value),
        "placeLimitOrder"=>typed::<binance_client::core_trading::convert::rest_models::PlaceLimitOrderResponse>(value),
        "queryLimitOpenOrders"=>typed::<binance_client::core_trading::convert::rest_models::QueryLimitOpenOrdersResponse>(value),
        _=>convert_rest_1(operation,value),
    }
}
fn convert_rest_1(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "sendQuoteRequest" => typed::<
            binance_client::core_trading::convert::rest_models::SendQuoteRequestResponse,
        >(value),
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn copy_trading_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "getFuturesLeadTraderStatus"=>typed::<binance_client::advanced_trading::copy_trading::rest_models::GetFuturesLeadTraderStatusResponse>(value),
        "getFuturesLeadTradingSymbolWhitelist"=>typed::<binance_client::advanced_trading::copy_trading::rest_models::GetFuturesLeadTradingSymbolWhitelistResponse>(value),
        _=>Err("response fixture dispatch missing operation".into()),
    }
}
fn margin_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "adjustCrossMarginMaxLeverage"=>typed::<binance_client::core_trading::margin::rest_models::AdjustCrossMarginMaxLeverageResponse>(value),
        "disableIsolatedMarginAccount"=>typed::<binance_client::core_trading::margin::rest_models::DisableIsolatedMarginAccountResponse>(value),
        "enableIsolatedMarginAccount"=>typed::<binance_client::core_trading::margin::rest_models::EnableIsolatedMarginAccountResponse>(value),
        "queryIsolatedMarginAccountInfo"=>typed::<binance_client::core_trading::margin::rest_models::QueryIsolatedMarginAccountInfoResponse>(value),
        "getBnbBurnStatus"=>typed::<binance_client::core_trading::margin::rest_models::GetBnbBurnStatusResponse>(value),
        "getSummaryOfMarginAccount"=>typed::<binance_client::core_trading::margin::rest_models::GetSummaryOfMarginAccountResponse>(value),
        "queryCrossIsolatedMarginCapitalFlow"=>typed::<binance_client::core_trading::margin::rest_models::QueryCrossIsolatedMarginCapitalFlowResponse>(value),
        "queryCrossMarginAccountDetails"=>typed::<binance_client::core_trading::margin::rest_models::QueryCrossMarginAccountDetailsResponse>(value),
        _=>margin_rest_1(operation,value),
    }
}
fn margin_rest_1(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "queryCrossMarginFeeData"=>typed::<binance_client::core_trading::margin::rest_models::QueryCrossMarginFeeDataResponse>(value),
        "queryEnabledIsolatedMarginAccountLimit"=>typed::<binance_client::core_trading::margin::rest_models::QueryEnabledIsolatedMarginAccountLimitResponse>(value),
        "queryIsolatedMarginFeeData"=>typed::<binance_client::core_trading::margin::rest_models::QueryIsolatedMarginFeeDataResponse>(value),
        "getFutureHourlyInterestRate"=>typed::<binance_client::core_trading::margin::rest_models::GetFutureHourlyInterestRateResponse>(value),
        "getInterestHistory"=>typed::<binance_client::core_trading::margin::rest_models::GetInterestHistoryResponse>(value),
        "marginAccountBorrowRepay"=>typed::<binance_client::core_trading::margin::rest_models::MarginAccountBorrowRepayResponse>(value),
        "queryBorrowRepayRecordsInMarginAccount"=>typed::<binance_client::core_trading::margin::rest_models::QueryBorrowRepayRecordsInMarginAccountResponse>(value),
        "queryMarginInterestRateHistory"=>typed::<binance_client::core_trading::margin::rest_models::QueryMarginInterestRateHistoryResponse>(value),
        _=>margin_rest_2(operation,value),
    }
}
fn margin_rest_2(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "queryMaxBorrow" => typed::<
            binance_client::core_trading::margin::rest_models::QueryMaxBorrowResponse,
        >(value),
        "crossMarginCollateralRatio" => typed::<
            binance_client::core_trading::margin::rest_models::CrossMarginCollateralRatioResponse,
        >(value),
        "getAllCrossMarginPairs" => typed::<
            binance_client::core_trading::margin::rest_models::GetAllCrossMarginPairsResponse,
        >(value),
        "getAllIsolatedMarginSymbol" => typed::<
            binance_client::core_trading::margin::rest_models::GetAllIsolatedMarginSymbolResponse,
        >(value),
        "getAllMarginAssets" => typed::<
            binance_client::core_trading::margin::rest_models::GetAllMarginAssetsResponse,
        >(value),
        "getDelistSchedule" => typed::<
            binance_client::core_trading::margin::rest_models::GetDelistScheduleResponse,
        >(value),
        "getLimitPricePairs" => typed::<
            binance_client::core_trading::margin::rest_models::GetLimitPricePairsResponse,
        >(value),
        "getListSchedule" => typed::<
            binance_client::core_trading::margin::rest_models::GetListScheduleResponse,
        >(value),
        _ => margin_rest_3(operation, value),
    }
}
fn margin_rest_3(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "getMarginAssetRiskBasedLiquidationRatio"=>typed::<binance_client::core_trading::margin::rest_models::GetMarginAssetRiskBasedLiquidationRatioResponse>(value),
        "getMarginRestrictedAssets"=>typed::<binance_client::core_trading::margin::rest_models::GetMarginRestrictedAssetsResponse>(value),
        "queryIsolatedMarginTierData"=>typed::<binance_client::core_trading::margin::rest_models::QueryIsolatedMarginTierDataResponse>(value),
        "queryMarginAvailableInventory"=>typed::<binance_client::core_trading::margin::rest_models::QueryMarginAvailableInventoryResponse>(value),
        "queryMarginPriceindex"=>typed::<binance_client::core_trading::margin::rest_models::QueryMarginPriceindexResponse>(value),
        "closeUserDataStream"=>typed::<binance_client::core_trading::margin::rest_models::CloseUserDataStreamResponse>(value),
        "keepaliveUserDataStream"=>typed::<binance_client::core_trading::margin::rest_models::KeepaliveUserDataStreamResponse>(value),
        "startUserDataStream"=>typed::<binance_client::core_trading::margin::rest_models::StartUserDataStreamResponse>(value),
        _=>margin_rest_4(operation,value),
    }
}
fn margin_rest_4(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "createSpecialKey"=>typed::<binance_client::core_trading::margin::rest_models::CreateSpecialKeyResponse>(value),
        "deleteSpecialKey"=>typed::<binance_client::core_trading::margin::rest_models::DeleteSpecialKeyResponse>(value),
        "querySpecialKey"=>typed::<binance_client::core_trading::margin::rest_models::QuerySpecialKeyResponse>(value),
        "editIpForSpecialKey"=>typed::<binance_client::core_trading::margin::rest_models::EditIpForSpecialKeyResponse>(value),
        "exitSpecialKeyMode"=>typed::<binance_client::core_trading::margin::rest_models::ExitSpecialKeyModeResponse>(value),
        "getForceLiquidationRecord"=>typed::<binance_client::core_trading::margin::rest_models::GetForceLiquidationRecordResponse>(value),
        "getSmallLiabilityExchangeCoinList"=>typed::<binance_client::core_trading::margin::rest_models::GetSmallLiabilityExchangeCoinListResponse>(value),
        "smallLiabilityExchange"=>typed::<binance_client::core_trading::margin::rest_models::SmallLiabilityExchangeResponse>(value),
        _=>margin_rest_5(operation,value),
    }
}
fn margin_rest_5(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "getSmallLiabilityExchangeHistory"=>typed::<binance_client::core_trading::margin::rest_models::GetSmallLiabilityExchangeHistoryResponse>(value),
        "marginAccountCancelAllOpenOrdersOnASymbol"=>typed::<binance_client::core_trading::margin::rest_models::MarginAccountCancelAllOpenOrdersOnASymbolResponse>(value),
        "queryMarginAccountsOpenOrders"=>typed::<binance_client::core_trading::margin::rest_models::QueryMarginAccountsOpenOrdersResponse>(value),
        "marginAccountCancelOco"=>typed::<binance_client::core_trading::margin::rest_models::MarginAccountCancelOcoResponse>(value),
        "queryMarginAccountsOco"=>typed::<binance_client::core_trading::margin::rest_models::QueryMarginAccountsOcoResponse>(value),
        "marginAccountCancelOrder"=>typed::<binance_client::core_trading::margin::rest_models::MarginAccountCancelOrderResponse>(value),
        "marginAccountNewOrder"=>typed::<binance_client::core_trading::margin::rest_models::MarginAccountNewOrderResponse>(value),
        "queryMarginAccountsOrder"=>typed::<binance_client::core_trading::margin::rest_models::QueryMarginAccountsOrderResponse>(value),
        _=>margin_rest_6(operation,value),
    }
}
fn margin_rest_6(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "marginAccountNewOco"=>typed::<binance_client::core_trading::margin::rest_models::MarginAccountNewOcoResponse>(value),
        "marginAccountNewOto"=>typed::<binance_client::core_trading::margin::rest_models::MarginAccountNewOtoResponse>(value),
        "marginAccountNewOtoco"=>typed::<binance_client::core_trading::margin::rest_models::MarginAccountNewOtocoResponse>(value),
        "marginManualLiquidation"=>typed::<binance_client::core_trading::margin::rest_models::MarginManualLiquidationResponse>(value),
        "queryCurrentMarginOrderCountUsage"=>typed::<binance_client::core_trading::margin::rest_models::QueryCurrentMarginOrderCountUsageResponse>(value),
        "queryMarginAccountsAllOco"=>typed::<binance_client::core_trading::margin::rest_models::QueryMarginAccountsAllOcoResponse>(value),
        "queryMarginAccountsAllOrders"=>typed::<binance_client::core_trading::margin::rest_models::QueryMarginAccountsAllOrdersResponse>(value),
        "queryMarginAccountsOpenOco"=>typed::<binance_client::core_trading::margin::rest_models::QueryMarginAccountsOpenOcoResponse>(value),
        _=>margin_rest_7(operation,value),
    }
}
fn margin_rest_7(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "queryMarginAccountsOpenOtootocoOrderLists"=>typed::<binance_client::core_trading::margin::rest_models::QueryMarginAccountsOpenOtootocoOrderListsResponse>(value),
        "queryMarginAccountsTradeList"=>typed::<binance_client::core_trading::margin::rest_models::QueryMarginAccountsTradeListResponse>(value),
        "queryPreventedMatches"=>typed::<binance_client::core_trading::margin::rest_models::QueryPreventedMatchesResponse>(value),
        "querySpecialKeyList"=>typed::<binance_client::core_trading::margin::rest_models::QuerySpecialKeyListResponse>(value),
        "queryLiquidationLoan"=>typed::<binance_client::core_trading::margin::rest_models::QueryLiquidationLoanResponse>(value),
        "liquidationLoanRepay"=>typed::<binance_client::core_trading::margin::rest_models::LiquidationLoanRepayResponse>(value),
        "queryLiquidationLoanRepayHistory"=>typed::<binance_client::core_trading::margin::rest_models::QueryLiquidationLoanRepayHistoryResponse>(value),
        "getCrossMarginTransferHistory"=>typed::<binance_client::core_trading::margin::rest_models::GetCrossMarginTransferHistoryResponse>(value),
        _=>margin_rest_8(operation,value),
    }
}
fn margin_rest_8(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "queryMaxTransferOutAmount" => typed::<
            binance_client::core_trading::margin::rest_models::QueryMaxTransferOutAmountResponse,
        >(value),
        "createUserListenToken" => typed::<
            binance_client::core_trading::margin::rest_models::CreateUserListenTokenResponse,
        >(value),
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn options_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "accountFundingFlow"=>typed::<binance_client::core_trading::options::rest_models::AccountFundingFlowResponse>(value),
        "optionMarginAccountInformation"=>typed::<binance_client::core_trading::options::rest_models::OptionMarginAccountInformationResponse>(value),
        "checkServerTime"=>typed::<binance_client::core_trading::options::rest_models::CheckServerTimeResponse>(value),
        "exchangeInformation"=>typed::<binance_client::core_trading::options::rest_models::ExchangeInformationResponse>(value),
        "historicalExerciseRecords"=>typed::<binance_client::core_trading::options::rest_models::HistoricalExerciseRecordsResponse>(value),
        "indexPrice"=>typed::<binance_client::core_trading::options::rest_models::IndexPriceResponse>(value),
        "klineCandlestickData"=>typed::<binance_client::core_trading::options::rest_models::KlineCandlestickDataResponse>(value),
        "openInterest"=>typed::<binance_client::core_trading::options::rest_models::OpenInterestResponse>(value),
        _=>options_rest_1(operation,value),
    }
}
fn options_rest_1(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "optionMarkPrice"=>typed::<binance_client::core_trading::options::rest_models::OptionMarkPriceResponse>(value),
        "orderBook"=>typed::<binance_client::core_trading::options::rest_models::OrderBookResponse>(value),
        "recentBlockTradesList"=>typed::<binance_client::core_trading::options::rest_models::RecentBlockTradesListResponse>(value),
        "recentTradesList"=>typed::<binance_client::core_trading::options::rest_models::RecentTradesListResponse>(value),
        "testConnectivity"=>typed::<binance_client::core_trading::options::rest_models::TestConnectivityResponse>(value),
        "ticker24hrPriceChangeStatistics"=>typed::<binance_client::core_trading::options::rest_models::Ticker24hrPriceChangeStatisticsResponse>(value),
        "acceptBlockTradeOrder"=>typed::<binance_client::core_trading::options::rest_models::AcceptBlockTradeOrderResponse>(value),
        "queryBlockTradeDetails"=>typed::<binance_client::core_trading::options::rest_models::QueryBlockTradeDetailsResponse>(value),
        _=>options_rest_2(operation,value),
    }
}
fn options_rest_2(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "accountBlockTradeList" => typed::<
            binance_client::core_trading::options::rest_models::AccountBlockTradeListResponse,
        >(value),
        "cancelBlockTradeOrder" => typed::<
            binance_client::core_trading::options::rest_models::CancelBlockTradeOrderResponse,
        >(value),
        "extendBlockTradeOrder" => typed::<
            binance_client::core_trading::options::rest_models::ExtendBlockTradeOrderResponse,
        >(value),
        "newBlockTradeOrder" => typed::<
            binance_client::core_trading::options::rest_models::NewBlockTradeOrderResponse,
        >(value),
        "queryBlockTradeOrder" => typed::<
            binance_client::core_trading::options::rest_models::QueryBlockTradeOrderResponse,
        >(value),
        "autoCancelAllOpenOrders" => typed::<
            binance_client::core_trading::options::rest_models::AutoCancelAllOpenOrdersResponse,
        >(value),
        "getAutoCancelAllOpenOrders" => typed::<
            binance_client::core_trading::options::rest_models::GetAutoCancelAllOpenOrdersResponse,
        >(value),
        "setAutoCancelAllOpenOrders" => typed::<
            binance_client::core_trading::options::rest_models::SetAutoCancelAllOpenOrdersResponse,
        >(value),
        _ => options_rest_3(operation, value),
    }
}
fn options_rest_3(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "getMarketMakerProtectionConfig"=>typed::<binance_client::core_trading::options::rest_models::GetMarketMakerProtectionConfigResponse>(value),
        "resetMarketMakerProtectionConfig"=>typed::<binance_client::core_trading::options::rest_models::ResetMarketMakerProtectionConfigResponse>(value),
        "setMarketMakerProtectionConfig"=>typed::<binance_client::core_trading::options::rest_models::SetMarketMakerProtectionConfigResponse>(value),
        "accountTradeList"=>typed::<binance_client::core_trading::options::rest_models::AccountTradeListResponse>(value),
        "cancelAllOptionOrdersByUnderlying"=>typed::<binance_client::core_trading::options::rest_models::CancelAllOptionOrdersByUnderlyingResponse>(value),
        "cancelAllOptionOrdersOnSpecificSymbol"=>typed::<binance_client::core_trading::options::rest_models::CancelAllOptionOrdersOnSpecificSymbolResponse>(value),
        "cancelMultipleOptionOrders"=>typed::<binance_client::core_trading::options::rest_models::CancelMultipleOptionOrdersResponse>(value),
        "placeMultipleOrders"=>typed::<binance_client::core_trading::options::rest_models::PlaceMultipleOrdersResponse>(value),
        _=>options_rest_4(operation,value),
    }
}
fn options_rest_4(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "cancelOptionOrder"=>typed::<binance_client::core_trading::options::rest_models::CancelOptionOrderResponse>(value),
        "newOrder"=>typed::<binance_client::core_trading::options::rest_models::NewOrderResponse>(value),
        "querySingleOrder"=>typed::<binance_client::core_trading::options::rest_models::QuerySingleOrderResponse>(value),
        "optionPositionInformation"=>typed::<binance_client::core_trading::options::rest_models::OptionPositionInformationResponse>(value),
        "queryCurrentOpenOptionOrders"=>typed::<binance_client::core_trading::options::rest_models::QueryCurrentOpenOptionOrdersResponse>(value),
        "queryOptionOrderHistory"=>typed::<binance_client::core_trading::options::rest_models::QueryOptionOrderHistoryResponse>(value),
        "tradfiOptionsContract"=>typed::<binance_client::core_trading::options::rest_models::TradfiOptionsContractResponse>(value),
        "userCommission"=>typed::<binance_client::core_trading::options::rest_models::UserCommissionResponse>(value),
        _=>options_rest_5(operation,value),
    }
}
fn options_rest_5(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "userExerciseRecord" => typed::<
            binance_client::core_trading::options::rest_models::UserExerciseRecordResponse,
        >(value),
        "closeUserDataStream" => typed::<
            binance_client::core_trading::options::rest_models::CloseUserDataStreamResponse,
        >(value),
        "keepaliveUserDataStream" => typed::<
            binance_client::core_trading::options::rest_models::KeepaliveUserDataStreamResponse,
        >(value),
        "startUserDataStream" => typed::<
            binance_client::core_trading::options::rest_models::StartUserDataStreamResponse,
        >(value),
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn portfolio_margin_pro_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "bnbTransfer"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::BnbTransferResponse>(value),
        "changeAutoRepayFuturesStatus"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::ChangeAutoRepayFuturesStatusResponse>(value),
        "getAutoRepayFuturesStatus"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::GetAutoRepayFuturesStatusResponse>(value),
        "fundAutoCollection"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::FundAutoCollectionResponse>(value),
        "fundCollectionByAsset"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::FundCollectionByAssetResponse>(value),
        "getDeltaModeStatus"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::GetDeltaModeStatusResponse>(value),
        "switchDeltaMode"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::SwitchDeltaModeResponse>(value),
        "getPortfolioMarginProAccountBalance"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::GetPortfolioMarginProAccountBalanceResponse>(value),
        _=>portfolio_margin_pro_rest_1(operation,value),
    }
}
fn portfolio_margin_pro_rest_1(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "getPortfolioMarginProAccountInfo"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::GetPortfolioMarginProAccountInfoResponse>(value),
        "getPortfolioMarginProSpanAccountInfo"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::GetPortfolioMarginProSpanAccountInfoResponse>(value),
        "getTransferableEarnAssetBalanceForPortfolioMargin"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::GetTransferableEarnAssetBalanceForPortfolioMarginResponse>(value),
        "portfolioMarginProBankruptcyLoanRepay"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::PortfolioMarginProBankruptcyLoanRepayResponse>(value),
        "queryPortfolioMarginProBankruptcyLoanAmount"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::QueryPortfolioMarginProBankruptcyLoanAmountResponse>(value),
        "queryPortfolioMarginProBankruptcyLoanRepayHistory"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::QueryPortfolioMarginProBankruptcyLoanRepayHistoryResponse>(value),
        "queryPortfolioMarginProNegativeBalanceInterestHistory"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::QueryPortfolioMarginProNegativeBalanceInterestHistoryResponse>(value),
        "repayFuturesNegativeBalance"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::RepayFuturesNegativeBalanceResponse>(value),
        _=>portfolio_margin_pro_rest_2(operation,value),
    }
}
fn portfolio_margin_pro_rest_2(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "transferLdusdtRwusdForPortfolioMargin"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::TransferLdusdtRwusdForPortfolioMarginResponse>(value),
        "getMarginCallLevel"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::GetMarginCallLevelResponse>(value),
        "setMarginCallLevel"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::SetMarginCallLevelResponse>(value),
        "deleteMarginCallLevel"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::DeleteMarginCallLevelResponse>(value),
        "getPortfolioMarginAssetLeverage"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::GetPortfolioMarginAssetLeverageResponse>(value),
        "portfolioMarginCollateralRate"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::PortfolioMarginCollateralRateResponse>(value),
        "portfolioMarginProTieredCollateralRate"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::PortfolioMarginProTieredCollateralRateResponse>(value),
        "queryPortfolioMarginAssetIndexPrice"=>typed::<binance_client::advanced_trading::portfolio_margin_pro::rest_models::QueryPortfolioMarginAssetIndexPriceResponse>(value),
        _=>Err("response fixture dispatch missing operation".into()),
    }
}
fn portfolio_margin_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "accountBalance"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::AccountBalanceResponse>(value),
        "accountInformation"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::AccountInformationResponse>(value),
        "bnbTransfer"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::BnbTransferResponse>(value),
        "changeAutoRepayFuturesStatus"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::ChangeAutoRepayFuturesStatusResponse>(value),
        "getAutoRepayFuturesStatus"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetAutoRepayFuturesStatusResponse>(value),
        "changeCmInitialLeverage"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::ChangeCmInitialLeverageResponse>(value),
        "changeCmPositionMode"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::ChangeCmPositionModeResponse>(value),
        "getCmCurrentPositionMode"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetCmCurrentPositionModeResponse>(value),
        _=>portfolio_margin_rest_1(operation,value),
    }
}
fn portfolio_margin_rest_1(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "changeUmInitialLeverage"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::ChangeUmInitialLeverageResponse>(value),
        "changeUmPositionMode"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::ChangeUmPositionModeResponse>(value),
        "getUmCurrentPositionMode"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetUmCurrentPositionModeResponse>(value),
        "cmNotionalAndLeverageBrackets"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CmNotionalAndLeverageBracketsResponse>(value),
        "fundAutoCollection"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::FundAutoCollectionResponse>(value),
        "fundCollectionByAsset"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::FundCollectionByAssetResponse>(value),
        "getCmAccountDetail"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetCmAccountDetailResponse>(value),
        "getCmIncomeHistory"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetCmIncomeHistoryResponse>(value),
        _=>portfolio_margin_rest_2(operation,value),
    }
}
fn portfolio_margin_rest_2(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "getDownloadIdForUmFuturesOrderHistory"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetDownloadIdForUmFuturesOrderHistoryResponse>(value),
        "getDownloadIdForUmFuturesTradeHistory"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetDownloadIdForUmFuturesTradeHistoryResponse>(value),
        "getDownloadIdForUmFuturesTransactionHistory"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetDownloadIdForUmFuturesTransactionHistoryResponse>(value),
        "getMarginBorrowLoanInterestHistory"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetMarginBorrowLoanInterestHistoryResponse>(value),
        "getUmAccountDetail"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetUmAccountDetailResponse>(value),
        "getUmAccountDetailV2"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetUmAccountDetailV2Response>(value),
        "getUmFuturesOrderDownloadLinkById"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetUmFuturesOrderDownloadLinkByIdResponse>(value),
        "getUmFuturesTradeDownloadLinkById"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetUmFuturesTradeDownloadLinkByIdResponse>(value),
        _=>portfolio_margin_rest_3(operation,value),
    }
}
fn portfolio_margin_rest_3(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "getUmFuturesTransactionDownloadLinkById"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetUmFuturesTransactionDownloadLinkByIdResponse>(value),
        "getUmIncomeHistory"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetUmIncomeHistoryResponse>(value),
        "getUserCommissionRateForCm"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetUserCommissionRateForCmResponse>(value),
        "getUserCommissionRateForUm"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetUserCommissionRateForUmResponse>(value),
        "marginMaxBorrow"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::MarginMaxBorrowResponse>(value),
        "portfolioMarginUmTradingQuantitativeRulesIndicators"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::PortfolioMarginUmTradingQuantitativeRulesIndicatorsResponse>(value),
        "queryCmPositionInformation"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryCmPositionInformationResponse>(value),
        "queryMarginLoanRecord"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryMarginLoanRecordResponse>(value),
        _=>portfolio_margin_rest_4(operation,value),
    }
}
fn portfolio_margin_rest_4(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "queryMarginMaxWithdraw"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryMarginMaxWithdrawResponse>(value),
        "queryMarginRepayRecord"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryMarginRepayRecordResponse>(value),
        "queryPortfolioMarginNegativeBalanceInterestHistory"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryPortfolioMarginNegativeBalanceInterestHistoryResponse>(value),
        "queryUmPositionInformation"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryUmPositionInformationResponse>(value),
        "queryUserNegativeBalanceAutoExchangeRecord"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryUserNegativeBalanceAutoExchangeRecordResponse>(value),
        "queryUserRateLimit"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryUserRateLimitResponse>(value),
        "repayFuturesNegativeBalance"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::RepayFuturesNegativeBalanceResponse>(value),
        "umFuturesAccountConfiguration"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::UmFuturesAccountConfigurationResponse>(value),
        _=>portfolio_margin_rest_5(operation,value),
    }
}
fn portfolio_margin_rest_5(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "umFuturesSymbolConfiguration"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::UmFuturesSymbolConfigurationResponse>(value),
        "umNotionalAndLeverageBrackets"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::UmNotionalAndLeverageBracketsResponse>(value),
        "testConnectivity"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::TestConnectivityResponse>(value),
        "cancelAllCmOpenConditionalOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CancelAllCmOpenConditionalOrdersResponse>(value),
        "cancelAllCmOpenOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CancelAllCmOpenOrdersResponse>(value),
        "cancelAllUmOpenOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CancelAllUmOpenOrdersResponse>(value),
        "cancelCmConditionalOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CancelCmConditionalOrderResponse>(value),
        "newCmConditionalOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::NewCmConditionalOrderResponse>(value),
        _=>portfolio_margin_rest_6(operation,value),
    }
}
fn portfolio_margin_rest_6(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "cancelCmOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CancelCmOrderResponse>(value),
        "modifyCmOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::ModifyCmOrderResponse>(value),
        "newCmOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::NewCmOrderResponse>(value),
        "queryCmOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryCmOrderResponse>(value),
        "cancelMarginAccountAllOpenOrdersOnASymbol"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CancelMarginAccountAllOpenOrdersOnASymbolResponse>(value),
        "cancelMarginAccountOcoOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CancelMarginAccountOcoOrdersResponse>(value),
        "queryMarginAccountsOco"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryMarginAccountsOcoResponse>(value),
        "cancelMarginAccountOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CancelMarginAccountOrderResponse>(value),
        _=>portfolio_margin_rest_7(operation,value),
    }
}
fn portfolio_margin_rest_7(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "newMarginOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::NewMarginOrderResponse>(value),
        "queryMarginAccountOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryMarginAccountOrderResponse>(value),
        "cancelUmOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CancelUmOrderResponse>(value),
        "modifyUmOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::ModifyUmOrderResponse>(value),
        "newUmOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::NewUmOrderResponse>(value),
        "queryUmOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryUmOrderResponse>(value),
        "cmAccountTradeList"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CmAccountTradeListResponse>(value),
        "cmPositionAdlQuantileEstimation"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CmPositionAdlQuantileEstimationResponse>(value),
        _=>portfolio_margin_rest_8(operation,value),
    }
}
fn portfolio_margin_rest_8(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "getUmFuturesBnbBurnStatus"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::GetUmFuturesBnbBurnStatusResponse>(value),
        "toggleBnbBurnOnUmFuturesTrade"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::ToggleBnbBurnOnUmFuturesTradeResponse>(value),
        "marginAccountBorrow"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::MarginAccountBorrowResponse>(value),
        "marginAccountNewOco"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::MarginAccountNewOcoResponse>(value),
        "marginAccountRepay"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::MarginAccountRepayResponse>(value),
        "marginAccountRepayDebt"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::MarginAccountRepayDebtResponse>(value),
        "marginAccountTradeList"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::MarginAccountTradeListResponse>(value),
        "queryAllCmConditionalOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryAllCmConditionalOrdersResponse>(value),
        _=>portfolio_margin_rest_9(operation,value),
    }
}
fn portfolio_margin_rest_9(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "queryAllCmOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryAllCmOrdersResponse>(value),
        "queryAllCurrentCmOpenConditionalOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryAllCurrentCmOpenConditionalOrdersResponse>(value),
        "queryAllCurrentCmOpenOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryAllCurrentCmOpenOrdersResponse>(value),
        "queryAllCurrentUmOpenOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryAllCurrentUmOpenOrdersResponse>(value),
        "queryAllMarginAccountOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryAllMarginAccountOrdersResponse>(value),
        "queryAllUmOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryAllUmOrdersResponse>(value),
        "queryCmConditionalOrderHistory"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryCmConditionalOrderHistoryResponse>(value),
        "queryCmModifyOrderHistory"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryCmModifyOrderHistoryResponse>(value),
        _=>portfolio_margin_rest_10(operation,value),
    }
}
fn portfolio_margin_rest_10(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "queryCurrentCmOpenConditionalOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryCurrentCmOpenConditionalOrderResponse>(value),
        "queryCurrentCmOpenOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryCurrentCmOpenOrderResponse>(value),
        "queryCurrentMarginOpenOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryCurrentMarginOpenOrderResponse>(value),
        "queryCurrentUmOpenOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryCurrentUmOpenOrderResponse>(value),
        "queryMarginAccountsAllOco"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryMarginAccountsAllOcoResponse>(value),
        "queryMarginAccountsOpenOco"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryMarginAccountsOpenOcoResponse>(value),
        "queryUmModifyOrderHistory"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryUmModifyOrderHistoryResponse>(value),
        "queryUsersCmForceOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryUsersCmForceOrdersResponse>(value),
        _=>portfolio_margin_rest_11(operation,value),
    }
}
fn portfolio_margin_rest_11(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "queryUsersMarginForceOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryUsersMarginForceOrdersResponse>(value),
        "queryUsersUmForceOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryUsersUmForceOrdersResponse>(value),
        "umAccountTradeList"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::UmAccountTradeListResponse>(value),
        "umPositionAdlQuantileEstimation"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::UmPositionAdlQuantileEstimationResponse>(value),
        "newUmAlgoOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::NewUmAlgoOrderResponse>(value),
        "cancelUmAlgoOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CancelUmAlgoOrderResponse>(value),
        "cancelAllUmAlgoOpenOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CancelAllUmAlgoOpenOrdersResponse>(value),
        "queryCurrentUmOpenAlgoOrder"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryCurrentUmOpenAlgoOrderResponse>(value),
        _=>portfolio_margin_rest_12(operation,value),
    }
}
fn portfolio_margin_rest_12(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "queryAllCurrentUmOpenAlgoOrders"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryAllCurrentUmOpenAlgoOrdersResponse>(value),
        "queryUmAlgoOrderHistory"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::QueryUmAlgoOrderHistoryResponse>(value),
        "futuresTradfiPerpsContract"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::FuturesTradfiPerpsContractResponse>(value),
        "closeUserDataStream"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::CloseUserDataStreamResponse>(value),
        "keepaliveUserDataStream"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::KeepaliveUserDataStreamResponse>(value),
        "startUserDataStream"=>typed::<binance_client::advanced_trading::portfolio_margin::rest_models::StartUserDataStreamResponse>(value),
        _=>Err("response fixture dispatch missing operation".into()),
    }
}
fn spot_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "accountCommission" => typed::<
            binance_client::core_trading::spot::rest_models::AccountCommissionResponse,
        >(value),
        "allOrders" => {
            typed::<binance_client::core_trading::spot::rest_models::AllOrdersResponse>(value)
        }
        "getAccount" => {
            typed::<binance_client::core_trading::spot::rest_models::GetAccountResponse>(value)
        }
        "getOpenOrders" => {
            typed::<binance_client::core_trading::spot::rest_models::GetOpenOrdersResponse>(value)
        }
        "deleteOpenOrders" => typed::<
            binance_client::core_trading::spot::rest_models::DeleteOpenOrdersResponse,
        >(value),
        "getOrder" => {
            typed::<binance_client::core_trading::spot::rest_models::GetOrderResponse>(value)
        }
        "deleteOrder" => {
            typed::<binance_client::core_trading::spot::rest_models::DeleteOrderResponse>(value)
        }
        "newOrder" => {
            typed::<binance_client::core_trading::spot::rest_models::NewOrderResponse>(value)
        }
        _ => spot_rest_1(operation, value),
    }
}
fn spot_rest_1(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "myTrades" => {
            typed::<binance_client::core_trading::spot::rest_models::MyTradesResponse>(value)
        }
        "rateLimitOrder" => {
            typed::<binance_client::core_trading::spot::rest_models::RateLimitOrderResponse>(value)
        }
        "exchangeInfo" => {
            typed::<binance_client::core_trading::spot::rest_models::ExchangeInfoResponse>(value)
        }
        "executionRules" => {
            typed::<binance_client::core_trading::spot::rest_models::ExecutionRulesResponse>(value)
        }
        "ping" => typed::<binance_client::core_trading::spot::rest_models::PingResponse>(value),
        "time" => typed::<binance_client::core_trading::spot::rest_models::TimeResponse>(value),
        "aggTrades" => {
            typed::<binance_client::core_trading::spot::rest_models::AggTradesResponse>(value)
        }
        "avgPrice" => {
            typed::<binance_client::core_trading::spot::rest_models::AvgPriceResponse>(value)
        }
        _ => spot_rest_2(operation, value),
    }
}
fn spot_rest_2(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "depth" => typed::<binance_client::core_trading::spot::rest_models::DepthResponse>(value),
        "getTrades" => {
            typed::<binance_client::core_trading::spot::rest_models::GetTradesResponse>(value)
        }
        "historicalTrades" => typed::<
            binance_client::core_trading::spot::rest_models::HistoricalTradesResponse,
        >(value),
        "historicalBlockTrades" => typed::<
            binance_client::core_trading::spot::rest_models::HistoricalBlockTradesResponse,
        >(value),
        "klines" => typed::<binance_client::core_trading::spot::rest_models::KlinesResponse>(value),
        "ticker" => typed::<binance_client::core_trading::spot::rest_models::TickerResponse>(value),
        "ticker24hr" => {
            typed::<binance_client::core_trading::spot::rest_models::Ticker24hrResponse>(value)
        }
        "tickerBookTicker" => typed::<
            binance_client::core_trading::spot::rest_models::TickerBookTickerResponse,
        >(value),
        _ => spot_rest_3(operation, value),
    }
}
fn spot_rest_3(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "tickerPrice" => {
            typed::<binance_client::core_trading::spot::rest_models::TickerPriceResponse>(value)
        }
        "tickerTradingDay" => typed::<
            binance_client::core_trading::spot::rest_models::TickerTradingDayResponse,
        >(value),
        "uiKlines" => {
            typed::<binance_client::core_trading::spot::rest_models::UiKlinesResponse>(value)
        }
        "referencePrice" => {
            typed::<binance_client::core_trading::spot::rest_models::ReferencePriceResponse>(value)
        }
        "referencePriceCalculation" => typed::<
            binance_client::core_trading::spot::rest_models::ReferencePriceCalculationResponse,
        >(value),
        "orderTest" => {
            typed::<binance_client::core_trading::spot::rest_models::OrderTestResponse>(value)
        }
        "allOrderList" => {
            typed::<binance_client::core_trading::spot::rest_models::AllOrderListResponse>(value)
        }
        "getOrderList" => {
            typed::<binance_client::core_trading::spot::rest_models::GetOrderListResponse>(value)
        }
        _ => spot_rest_4(operation, value),
    }
}
fn spot_rest_4(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "deleteOrderList" => {
            typed::<binance_client::core_trading::spot::rest_models::DeleteOrderListResponse>(value)
        }
        "myAllocations" => {
            typed::<binance_client::core_trading::spot::rest_models::MyAllocationsResponse>(value)
        }
        "myFilters" => {
            typed::<binance_client::core_trading::spot::rest_models::MyFiltersResponse>(value)
        }
        "myPreventedMatches" => typed::<
            binance_client::core_trading::spot::rest_models::MyPreventedMatchesResponse,
        >(value),
        "openOrderList" => {
            typed::<binance_client::core_trading::spot::rest_models::OpenOrderListResponse>(value)
        }
        "orderAmendments" => {
            typed::<binance_client::core_trading::spot::rest_models::OrderAmendmentsResponse>(value)
        }
        "orderAmendKeepPriority" => typed::<
            binance_client::core_trading::spot::rest_models::OrderAmendKeepPriorityResponse,
        >(value),
        "orderCancelReplace" => typed::<
            binance_client::core_trading::spot::rest_models::OrderCancelReplaceResponse,
        >(value),
        _ => spot_rest_5(operation, value),
    }
}
fn spot_rest_5(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "orderListOco" => {
            typed::<binance_client::core_trading::spot::rest_models::OrderListOcoResponse>(value)
        }
        "orderListOpo" => {
            typed::<binance_client::core_trading::spot::rest_models::OrderListOpoResponse>(value)
        }
        "orderListOpoco" => {
            typed::<binance_client::core_trading::spot::rest_models::OrderListOpocoResponse>(value)
        }
        "orderListOto" => {
            typed::<binance_client::core_trading::spot::rest_models::OrderListOtoResponse>(value)
        }
        "orderListOtoco" => {
            typed::<binance_client::core_trading::spot::rest_models::OrderListOtocoResponse>(value)
        }
        "orderOco" => {
            typed::<binance_client::core_trading::spot::rest_models::OrderOcoResponse>(value)
        }
        "sorOrder" => {
            typed::<binance_client::core_trading::spot::rest_models::SorOrderResponse>(value)
        }
        "sorOrderTest" => {
            typed::<binance_client::core_trading::spot::rest_models::SorOrderTestResponse>(value)
        }
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn spot_ws_0(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "accountCommission" => {
            typed::<binance_client::core_trading::spot::ws_models::AccountCommissionResponse>(value)
        }
        "accountRateLimitsOrders" => typed::<
            binance_client::core_trading::spot::ws_models::AccountRateLimitsOrdersResponse,
        >(value),
        "accountStatus" => {
            typed::<binance_client::core_trading::spot::ws_models::AccountStatusResponse>(value)
        }
        "allOrders" => {
            typed::<binance_client::core_trading::spot::ws_models::AllOrdersResponse>(value)
        }
        "myTrades" => {
            typed::<binance_client::core_trading::spot::ws_models::MyTradesResponse>(value)
        }
        "openOrdersStatus" => {
            typed::<binance_client::core_trading::spot::ws_models::OpenOrdersStatusResponse>(value)
        }
        "orderStatus" => {
            typed::<binance_client::core_trading::spot::ws_models::OrderStatusResponse>(value)
        }
        "exchangeInfo" => {
            typed::<binance_client::core_trading::spot::ws_models::ExchangeInfoResponse>(value)
        }
        _ => spot_ws_1(operation, value),
    }
}
fn spot_ws_1(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "executionRules" => {
            typed::<binance_client::core_trading::spot::ws_models::ExecutionRulesResponse>(value)
        }
        "ping" => typed::<binance_client::core_trading::spot::ws_models::PingResponse>(value),
        "time" => typed::<binance_client::core_trading::spot::ws_models::TimeResponse>(value),
        "avgPrice" => {
            typed::<binance_client::core_trading::spot::ws_models::AvgPriceResponse>(value)
        }
        "depth" => typed::<binance_client::core_trading::spot::ws_models::DepthResponse>(value),
        "klines" => typed::<binance_client::core_trading::spot::ws_models::KlinesResponse>(value),
        "ticker" => typed::<binance_client::core_trading::spot::ws_models::TickerResponse>(value),
        "ticker24hr" => {
            typed::<binance_client::core_trading::spot::ws_models::Ticker24hrResponse>(value)
        }
        _ => spot_ws_2(operation, value),
    }
}
fn spot_ws_2(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "tickerBook" => {
            typed::<binance_client::core_trading::spot::ws_models::TickerBookResponse>(value)
        }
        "tickerPrice" => {
            typed::<binance_client::core_trading::spot::ws_models::TickerPriceResponse>(value)
        }
        "tickerTradingDay" => {
            typed::<binance_client::core_trading::spot::ws_models::TickerTradingDayResponse>(value)
        }
        "tradesAggregate" => {
            typed::<binance_client::core_trading::spot::ws_models::TradesAggregateResponse>(value)
        }
        "tradesHistorical" => {
            typed::<binance_client::core_trading::spot::ws_models::TradesHistoricalResponse>(value)
        }
        "blockTradesHistorical" => typed::<
            binance_client::core_trading::spot::ws_models::BlockTradesHistoricalResponse,
        >(value),
        "tradesRecent" => {
            typed::<binance_client::core_trading::spot::ws_models::TradesRecentResponse>(value)
        }
        "uiKlines" => {
            typed::<binance_client::core_trading::spot::ws_models::UiKlinesResponse>(value)
        }
        _ => spot_ws_3(operation, value),
    }
}
fn spot_ws_3(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "referencePrice" => {
            typed::<binance_client::core_trading::spot::ws_models::ReferencePriceResponse>(value)
        }
        "referencePriceCalculation" => typed::<
            binance_client::core_trading::spot::ws_models::ReferencePriceCalculationResponse,
        >(value),
        "openOrdersCancelAll" => typed::<
            binance_client::core_trading::spot::ws_models::OpenOrdersCancelAllResponse,
        >(value),
        "orderCancel" => {
            typed::<binance_client::core_trading::spot::ws_models::OrderCancelResponse>(value)
        }
        "orderPlace" => {
            typed::<binance_client::core_trading::spot::ws_models::OrderPlaceResponse>(value)
        }
        "orderTest" => {
            typed::<binance_client::core_trading::spot::ws_models::OrderTestResponse>(value)
        }
        "sessionSubscriptions" => typed::<
            binance_client::core_trading::spot::ws_models::SessionSubscriptionsResponse,
        >(value),
        "userDataStreamSubscribe" => typed::<
            binance_client::core_trading::spot::ws_models::UserDataStreamSubscribeResponse,
        >(value),
        _ => spot_ws_4(operation, value),
    }
}
fn spot_ws_4(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "userDataStreamSubscribeSignature" => typed::<
            binance_client::core_trading::spot::ws_models::UserDataStreamSubscribeSignatureResponse,
        >(value),
        "userDataStreamUnsubscribe" => typed::<
            binance_client::core_trading::spot::ws_models::UserDataStreamUnsubscribeResponse,
        >(value),
        "allOrderLists" => {
            typed::<binance_client::core_trading::spot::ws_models::AllOrderListsResponse>(value)
        }
        "myAllocations" => {
            typed::<binance_client::core_trading::spot::ws_models::MyAllocationsResponse>(value)
        }
        "myFilters" => {
            typed::<binance_client::core_trading::spot::ws_models::MyFiltersResponse>(value)
        }
        "myPreventedMatches" => typed::<
            binance_client::core_trading::spot::ws_models::MyPreventedMatchesResponse,
        >(value),
        "openOrderListsStatus" => typed::<
            binance_client::core_trading::spot::ws_models::OpenOrderListsStatusResponse,
        >(value),
        "orderAmendments" => {
            typed::<binance_client::core_trading::spot::ws_models::OrderAmendmentsResponse>(value)
        }
        _ => spot_ws_5(operation, value),
    }
}
fn spot_ws_5(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "orderListStatus" => {
            typed::<binance_client::core_trading::spot::ws_models::OrderListStatusResponse>(value)
        }
        "orderAmendKeepPriority" => typed::<
            binance_client::core_trading::spot::ws_models::OrderAmendKeepPriorityResponse,
        >(value),
        "orderCancelReplace" => typed::<
            binance_client::core_trading::spot::ws_models::OrderCancelReplaceResponse,
        >(value),
        "orderListCancel" => {
            typed::<binance_client::core_trading::spot::ws_models::OrderListCancelResponse>(value)
        }
        "orderListPlace" => {
            typed::<binance_client::core_trading::spot::ws_models::OrderListPlaceResponse>(value)
        }
        "orderListPlaceOco" => {
            typed::<binance_client::core_trading::spot::ws_models::OrderListPlaceOcoResponse>(value)
        }
        "orderListPlaceOpo" => {
            typed::<binance_client::core_trading::spot::ws_models::OrderListPlaceOpoResponse>(value)
        }
        "orderListPlaceOpoco" => typed::<
            binance_client::core_trading::spot::ws_models::OrderListPlaceOpocoResponse,
        >(value),
        _ => spot_ws_6(operation, value),
    }
}
fn spot_ws_6(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "orderListPlaceOto" => {
            typed::<binance_client::core_trading::spot::ws_models::OrderListPlaceOtoResponse>(value)
        }
        "orderListPlaceOtoco" => typed::<
            binance_client::core_trading::spot::ws_models::OrderListPlaceOtocoResponse,
        >(value),
        "sorOrderPlace" => {
            typed::<binance_client::core_trading::spot::ws_models::SorOrderPlaceResponse>(value)
        }
        "sorOrderTest" => {
            typed::<binance_client::core_trading::spot::ws_models::SorOrderTestResponse>(value)
        }
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn stocks_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "exchangeInfo" => typed::<
            binance_client::advanced_trading::stocks::rest_models::ExchangeInfoResponse,
        >(value),
        "tokenizedAssets" => typed::<
            binance_client::advanced_trading::stocks::rest_models::TokenizedAssetsResponse,
        >(value),
        "latestQuote" => typed::<
            binance_client::advanced_trading::stocks::rest_models::LatestQuoteResponse,
        >(value),
        "placeEquityOrder" => typed::<
            binance_client::advanced_trading::stocks::rest_models::PlaceEquityOrderResponse,
        >(value),
        "cancelEquityOrder" => typed::<
            binance_client::advanced_trading::stocks::rest_models::CancelEquityOrderResponse,
        >(value),
        "cancelAllEquityOrders" => typed::<
            binance_client::advanced_trading::stocks::rest_models::CancelAllEquityOrdersResponse,
        >(value),
        "currentOpenOrders" => typed::<
            binance_client::advanced_trading::stocks::rest_models::CurrentOpenOrdersResponse,
        >(value),
        "equityOrderHistory" => typed::<
            binance_client::advanced_trading::stocks::rest_models::EquityOrderHistoryResponse,
        >(value),
        _ => stocks_rest_1(operation, value),
    }
}
fn stocks_rest_1(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "equityOrderDetail" => typed::<
            binance_client::advanced_trading::stocks::rest_models::EquityOrderDetailResponse,
        >(value),
        "equityTradeHistory" => typed::<
            binance_client::advanced_trading::stocks::rest_models::EquityTradeHistoryResponse,
        >(value),
        "tokenizedMint" => typed::<
            binance_client::advanced_trading::stocks::rest_models::TokenizedMintResponse,
        >(value),
        "tokenizedRedeem" => typed::<
            binance_client::advanced_trading::stocks::rest_models::TokenizedRedeemResponse,
        >(value),
        "tokenizedConvertStatus" => typed::<
            binance_client::advanced_trading::stocks::rest_models::TokenizedConvertStatusResponse,
        >(value),
        "tokenizedConvertHistory" => typed::<
            binance_client::advanced_trading::stocks::rest_models::TokenizedConvertHistoryResponse,
        >(value),
        "signUsEquityDisclaimer" => typed::<
            binance_client::advanced_trading::stocks::rest_models::SignUsEquityDisclaimerResponse,
        >(value),
        "createRenewListenKey" => typed::<
            binance_client::advanced_trading::stocks::rest_models::CreateRenewListenKeyResponse,
        >(value),
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn usdm_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "accountInformationV2"=>typed::<binance_client::core_trading::usdm::rest_models::AccountInformationV2Response>(value),
        "accountInformationV3"=>typed::<binance_client::core_trading::usdm::rest_models::AccountInformationV3Response>(value),
        "futuresAccountBalanceV2"=>typed::<binance_client::core_trading::usdm::rest_models::FuturesAccountBalanceV2Response>(value),
        "futuresAccountBalanceV3"=>typed::<binance_client::core_trading::usdm::rest_models::FuturesAccountBalanceV3Response>(value),
        "futuresAccountConfiguration"=>typed::<binance_client::core_trading::usdm::rest_models::FuturesAccountConfigurationResponse>(value),
        "futuresTradingQuantitativeRulesIndicators"=>typed::<binance_client::core_trading::usdm::rest_models::FuturesTradingQuantitativeRulesIndicatorsResponse>(value),
        "getBnbBurnStatus"=>typed::<binance_client::core_trading::usdm::rest_models::GetBnbBurnStatusResponse>(value),
        "toggleBnbBurnOnFuturesTrade"=>typed::<binance_client::core_trading::usdm::rest_models::ToggleBnbBurnOnFuturesTradeResponse>(value),
        _=>usdm_rest_1(operation,value),
    }
}
fn usdm_rest_1(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "getCurrentMultiAssetsMode"=>typed::<binance_client::core_trading::usdm::rest_models::GetCurrentMultiAssetsModeResponse>(value),
        "changeMultiAssetsMode"=>typed::<binance_client::core_trading::usdm::rest_models::ChangeMultiAssetsModeResponse>(value),
        "getCurrentPositionMode"=>typed::<binance_client::core_trading::usdm::rest_models::GetCurrentPositionModeResponse>(value),
        "changePositionMode"=>typed::<binance_client::core_trading::usdm::rest_models::ChangePositionModeResponse>(value),
        "getDownloadIdForFuturesOrderHistory"=>typed::<binance_client::core_trading::usdm::rest_models::GetDownloadIdForFuturesOrderHistoryResponse>(value),
        "getDownloadIdForFuturesTradeHistory"=>typed::<binance_client::core_trading::usdm::rest_models::GetDownloadIdForFuturesTradeHistoryResponse>(value),
        "getDownloadIdForFuturesTransactionHistory"=>typed::<binance_client::core_trading::usdm::rest_models::GetDownloadIdForFuturesTransactionHistoryResponse>(value),
        "getFuturesOrderHistoryDownloadLinkById"=>typed::<binance_client::core_trading::usdm::rest_models::GetFuturesOrderHistoryDownloadLinkByIdResponse>(value),
        _=>usdm_rest_2(operation,value),
    }
}
fn usdm_rest_2(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "getFuturesTradeDownloadLinkById"=>typed::<binance_client::core_trading::usdm::rest_models::GetFuturesTradeDownloadLinkByIdResponse>(value),
        "getFuturesTransactionHistoryDownloadLinkById"=>typed::<binance_client::core_trading::usdm::rest_models::GetFuturesTransactionHistoryDownloadLinkByIdResponse>(value),
        "getIncomeHistory"=>typed::<binance_client::core_trading::usdm::rest_models::GetIncomeHistoryResponse>(value),
        "notionalAndLeverageBrackets"=>typed::<binance_client::core_trading::usdm::rest_models::NotionalAndLeverageBracketsResponse>(value),
        "queryUserRateLimit"=>typed::<binance_client::core_trading::usdm::rest_models::QueryUserRateLimitResponse>(value),
        "symbolConfiguration"=>typed::<binance_client::core_trading::usdm::rest_models::SymbolConfigurationResponse>(value),
        "userCommissionRate"=>typed::<binance_client::core_trading::usdm::rest_models::UserCommissionRateResponse>(value),
        "acceptTheOfferedQuote"=>typed::<binance_client::core_trading::usdm::rest_models::AcceptTheOfferedQuoteResponse>(value),
        _=>usdm_rest_3(operation,value),
    }
}
fn usdm_rest_3(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "listAllConvertPairs"=>typed::<binance_client::core_trading::usdm::rest_models::ListAllConvertPairsResponse>(value),
        "orderStatus"=>typed::<binance_client::core_trading::usdm::rest_models::OrderStatusResponse>(value),
        "sendQuoteRequest"=>typed::<binance_client::core_trading::usdm::rest_models::SendQuoteRequestResponse>(value),
        "adlRisk"=>typed::<binance_client::core_trading::usdm::rest_models::AdlRiskResponse>(value),
        "basis"=>typed::<binance_client::core_trading::usdm::rest_models::BasisResponse>(value),
        "checkServerTime"=>typed::<binance_client::core_trading::usdm::rest_models::CheckServerTimeResponse>(value),
        "compositeIndexSymbolInformation"=>typed::<binance_client::core_trading::usdm::rest_models::CompositeIndexSymbolInformationResponse>(value),
        "compressedAggregateTradesList"=>typed::<binance_client::core_trading::usdm::rest_models::CompressedAggregateTradesListResponse>(value),
        _=>usdm_rest_4(operation,value),
    }
}
fn usdm_rest_4(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "continuousContractKlineCandlestickData"=>typed::<binance_client::core_trading::usdm::rest_models::ContinuousContractKlineCandlestickDataResponse>(value),
        "exchangeInformation"=>typed::<binance_client::core_trading::usdm::rest_models::ExchangeInformationResponse>(value),
        "getFundingRateHistory"=>typed::<binance_client::core_trading::usdm::rest_models::GetFundingRateHistoryResponse>(value),
        "getFundingRateInfo"=>typed::<binance_client::core_trading::usdm::rest_models::GetFundingRateInfoResponse>(value),
        "indexPriceKlineCandlestickData"=>typed::<binance_client::core_trading::usdm::rest_models::IndexPriceKlineCandlestickDataResponse>(value),
        "klineCandlestickData"=>typed::<binance_client::core_trading::usdm::rest_models::KlineCandlestickDataResponse>(value),
        "longShortRatio"=>typed::<binance_client::core_trading::usdm::rest_models::LongShortRatioResponse>(value),
        "markPrice"=>typed::<binance_client::core_trading::usdm::rest_models::MarkPriceResponse>(value),
        _=>usdm_rest_5(operation,value),
    }
}
fn usdm_rest_5(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "markPriceKlineCandlestickData"=>typed::<binance_client::core_trading::usdm::rest_models::MarkPriceKlineCandlestickDataResponse>(value),
        "assetIndex"=>typed::<binance_client::core_trading::usdm::rest_models::AssetIndexResponse>(value),
        "oldTradesLookup"=>typed::<binance_client::core_trading::usdm::rest_models::OldTradesLookupResponse>(value),
        "openInterest"=>typed::<binance_client::core_trading::usdm::rest_models::OpenInterestResponse>(value),
        "openInterestStatistics"=>typed::<binance_client::core_trading::usdm::rest_models::OpenInterestStatisticsResponse>(value),
        "orderBook"=>typed::<binance_client::core_trading::usdm::rest_models::OrderBookResponse>(value),
        "premiumIndexKlineData"=>typed::<binance_client::core_trading::usdm::rest_models::PremiumIndexKlineDataResponse>(value),
        "quarterlyContractSettlementPrice"=>typed::<binance_client::core_trading::usdm::rest_models::QuarterlyContractSettlementPriceResponse>(value),
        _=>usdm_rest_6(operation,value),
    }
}
fn usdm_rest_6(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "queryIndexPriceConstituents"=>typed::<binance_client::core_trading::usdm::rest_models::QueryIndexPriceConstituentsResponse>(value),
        "queryInsuranceFundBalanceSnapshot"=>typed::<binance_client::core_trading::usdm::rest_models::QueryInsuranceFundBalanceSnapshotResponse>(value),
        "recentTradesList"=>typed::<binance_client::core_trading::usdm::rest_models::RecentTradesListResponse>(value),
        "rpiOrderBook"=>typed::<binance_client::core_trading::usdm::rest_models::RpiOrderBookResponse>(value),
        "symbolOrderBookTicker"=>typed::<binance_client::core_trading::usdm::rest_models::SymbolOrderBookTickerResponse>(value),
        "symbolPriceTicker"=>typed::<binance_client::core_trading::usdm::rest_models::SymbolPriceTickerResponse>(value),
        "symbolPriceTickerV2"=>typed::<binance_client::core_trading::usdm::rest_models::SymbolPriceTickerV2Response>(value),
        "takerBuySellVolume"=>typed::<binance_client::core_trading::usdm::rest_models::TakerBuySellVolumeResponse>(value),
        _=>usdm_rest_7(operation,value),
    }
}
fn usdm_rest_7(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "testConnectivity"=>typed::<binance_client::core_trading::usdm::rest_models::TestConnectivityResponse>(value),
        "ticker24hrPriceChangeStatistics"=>typed::<binance_client::core_trading::usdm::rest_models::Ticker24hrPriceChangeStatisticsResponse>(value),
        "topTraderLongShortRatioAccounts"=>typed::<binance_client::core_trading::usdm::rest_models::TopTraderLongShortRatioAccountsResponse>(value),
        "topTraderLongShortRatioPositions"=>typed::<binance_client::core_trading::usdm::rest_models::TopTraderLongShortRatioPositionsResponse>(value),
        "tradingSchedule"=>typed::<binance_client::core_trading::usdm::rest_models::TradingScheduleResponse>(value),
        "classicPortfolioMarginAccountInformation"=>typed::<binance_client::core_trading::usdm::rest_models::ClassicPortfolioMarginAccountInformationResponse>(value),
        "accountTradeList"=>typed::<binance_client::core_trading::usdm::rest_models::AccountTradeListResponse>(value),
        "allOrders"=>typed::<binance_client::core_trading::usdm::rest_models::AllOrdersResponse>(value),
        _=>usdm_rest_8(operation,value),
    }
}
fn usdm_rest_8(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "autoCancelAllOpenOrders" => typed::<
            binance_client::core_trading::usdm::rest_models::AutoCancelAllOpenOrdersResponse,
        >(value),
        "cancelAlgoOrder" => {
            typed::<binance_client::core_trading::usdm::rest_models::CancelAlgoOrderResponse>(value)
        }
        "newAlgoOrder" => {
            typed::<binance_client::core_trading::usdm::rest_models::NewAlgoOrderResponse>(value)
        }
        "queryAlgoOrder" => {
            typed::<binance_client::core_trading::usdm::rest_models::QueryAlgoOrderResponse>(value)
        }
        "cancelAllAlgoOpenOrders" => typed::<
            binance_client::core_trading::usdm::rest_models::CancelAllAlgoOpenOrdersResponse,
        >(value),
        "cancelAllOpenOrders" => typed::<
            binance_client::core_trading::usdm::rest_models::CancelAllOpenOrdersResponse,
        >(value),
        "cancelMultipleOrders" => typed::<
            binance_client::core_trading::usdm::rest_models::CancelMultipleOrdersResponse,
        >(value),
        "modifyMultipleOrders" => typed::<
            binance_client::core_trading::usdm::rest_models::ModifyMultipleOrdersResponse,
        >(value),
        _ => usdm_rest_9(operation, value),
    }
}
fn usdm_rest_9(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "placeMultipleOrders" => typed::<
            binance_client::core_trading::usdm::rest_models::PlaceMultipleOrdersResponse,
        >(value),
        "cancelOrder" => {
            typed::<binance_client::core_trading::usdm::rest_models::CancelOrderResponse>(value)
        }
        "modifyOrder" => {
            typed::<binance_client::core_trading::usdm::rest_models::ModifyOrderResponse>(value)
        }
        "newOrder" => {
            typed::<binance_client::core_trading::usdm::rest_models::NewOrderResponse>(value)
        }
        "queryOrder" => {
            typed::<binance_client::core_trading::usdm::rest_models::QueryOrderResponse>(value)
        }
        "changeInitialLeverage" => typed::<
            binance_client::core_trading::usdm::rest_models::ChangeInitialLeverageResponse,
        >(value),
        "changeMarginType" => typed::<
            binance_client::core_trading::usdm::rest_models::ChangeMarginTypeResponse,
        >(value),
        "currentAllAlgoOpenOrders" => typed::<
            binance_client::core_trading::usdm::rest_models::CurrentAllAlgoOpenOrdersResponse,
        >(value),
        _ => usdm_rest_10(operation, value),
    }
}
fn usdm_rest_10(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "currentAllOpenOrders" => typed::<
            binance_client::core_trading::usdm::rest_models::CurrentAllOpenOrdersResponse,
        >(value),
        "futuresTradfiPerpsContract" => typed::<
            binance_client::core_trading::usdm::rest_models::FuturesTradfiPerpsContractResponse,
        >(value),
        "getOrderModifyHistory" => typed::<
            binance_client::core_trading::usdm::rest_models::GetOrderModifyHistoryResponse,
        >(value),
        "getPositionMarginChangeHistory" => typed::<
            binance_client::core_trading::usdm::rest_models::GetPositionMarginChangeHistoryResponse,
        >(value),
        "modifyIsolatedPositionMargin" => typed::<
            binance_client::core_trading::usdm::rest_models::ModifyIsolatedPositionMarginResponse,
        >(value),
        "positionAdlQuantileEstimation" => typed::<
            binance_client::core_trading::usdm::rest_models::PositionAdlQuantileEstimationResponse,
        >(value),
        "positionInformationV2" => typed::<
            binance_client::core_trading::usdm::rest_models::PositionInformationV2Response,
        >(value),
        "positionInformationV3" => typed::<
            binance_client::core_trading::usdm::rest_models::PositionInformationV3Response,
        >(value),
        _ => usdm_rest_11(operation, value),
    }
}
fn usdm_rest_11(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "queryAllAlgoOrders" => typed::<
            binance_client::core_trading::usdm::rest_models::QueryAllAlgoOrdersResponse,
        >(value),
        "queryCurrentOpenOrder" => typed::<
            binance_client::core_trading::usdm::rest_models::QueryCurrentOpenOrderResponse,
        >(value),
        "testOrder" => {
            typed::<binance_client::core_trading::usdm::rest_models::TestOrderResponse>(value)
        }
        "usersForceOrders" => typed::<
            binance_client::core_trading::usdm::rest_models::UsersForceOrdersResponse,
        >(value),
        "closeUserDataStream" => typed::<
            binance_client::core_trading::usdm::rest_models::CloseUserDataStreamResponse,
        >(value),
        "keepaliveUserDataStream" => typed::<
            binance_client::core_trading::usdm::rest_models::KeepaliveUserDataStreamResponse,
        >(value),
        "startUserDataStream" => typed::<
            binance_client::core_trading::usdm::rest_models::StartUserDataStreamResponse,
        >(value),
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn usdm_ws_0(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "accountInformation" => typed::<
            binance_client::core_trading::usdm::ws_models::AccountInformationResponse,
        >(value),
        "accountInformationV2" => typed::<
            binance_client::core_trading::usdm::ws_models::AccountInformationV2Response,
        >(value),
        "futuresAccountBalance" => typed::<
            binance_client::core_trading::usdm::ws_models::FuturesAccountBalanceResponse,
        >(value),
        "futuresAccountBalanceV2" => typed::<
            binance_client::core_trading::usdm::ws_models::FuturesAccountBalanceV2Response,
        >(value),
        "orderBook" => {
            typed::<binance_client::core_trading::usdm::ws_models::OrderBookResponse>(value)
        }
        "symbolOrderBookTicker" => typed::<
            binance_client::core_trading::usdm::ws_models::SymbolOrderBookTickerResponse,
        >(value),
        "symbolPriceTicker" => {
            typed::<binance_client::core_trading::usdm::ws_models::SymbolPriceTickerResponse>(value)
        }
        "cancelAlgoOrder" => {
            typed::<binance_client::core_trading::usdm::ws_models::CancelAlgoOrderResponse>(value)
        }
        _ => usdm_ws_1(operation, value),
    }
}
fn usdm_ws_1(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "cancelOrder" => {
            typed::<binance_client::core_trading::usdm::ws_models::CancelOrderResponse>(value)
        }
        "modifyOrder" => {
            typed::<binance_client::core_trading::usdm::ws_models::ModifyOrderResponse>(value)
        }
        "newAlgoOrder" => {
            typed::<binance_client::core_trading::usdm::ws_models::NewAlgoOrderResponse>(value)
        }
        "newOrder" => {
            typed::<binance_client::core_trading::usdm::ws_models::NewOrderResponse>(value)
        }
        "positionInformation" => typed::<
            binance_client::core_trading::usdm::ws_models::PositionInformationResponse,
        >(value),
        "positionInformationV2" => typed::<
            binance_client::core_trading::usdm::ws_models::PositionInformationV2Response,
        >(value),
        "queryOrder" => {
            typed::<binance_client::core_trading::usdm::ws_models::QueryOrderResponse>(value)
        }
        "closeUserDataStream" => typed::<
            binance_client::core_trading::usdm::ws_models::CloseUserDataStreamResponse,
        >(value),
        _ => usdm_ws_2(operation, value),
    }
}
fn usdm_ws_2(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "keepaliveUserDataStream" => typed::<
            binance_client::core_trading::usdm::ws_models::KeepaliveUserDataStreamResponse,
        >(value),
        "startUserDataStream" => typed::<
            binance_client::core_trading::usdm::ws_models::StartUserDataStreamResponse,
        >(value),
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn wallet_rest_0(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "accountApiTradingStatus" => typed::<
            binance_client::core_trading::wallet::rest_models::AccountApiTradingStatusResponse,
        >(value),
        "accountInfo" => {
            typed::<binance_client::core_trading::wallet::rest_models::AccountInfoResponse>(value)
        }
        "accountStatus" => {
            typed::<binance_client::core_trading::wallet::rest_models::AccountStatusResponse>(value)
        }
        "dailyAccountSnapshot" => typed::<
            binance_client::core_trading::wallet::rest_models::DailyAccountSnapshotResponse,
        >(value),
        "getApiKeyPermission" => typed::<
            binance_client::core_trading::wallet::rest_models::GetApiKeyPermissionResponse,
        >(value),
        "assetDetail" => {
            typed::<binance_client::core_trading::wallet::rest_models::AssetDetailResponse>(value)
        }
        "assetDividendRecord" => typed::<
            binance_client::core_trading::wallet::rest_models::AssetDividendRecordResponse,
        >(value),
        "dustConvert" => {
            typed::<binance_client::core_trading::wallet::rest_models::DustConvertResponse>(value)
        }
        _ => wallet_rest_1(operation, value),
    }
}
fn wallet_rest_1(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "dustConvertibleAssets"=>typed::<binance_client::core_trading::wallet::rest_models::DustConvertibleAssetsResponse>(value),
        "dustlog"=>typed::<binance_client::core_trading::wallet::rest_models::DustlogResponse>(value),
        "dustTransfer"=>typed::<binance_client::core_trading::wallet::rest_models::DustTransferResponse>(value),
        "fundingWallet"=>typed::<binance_client::core_trading::wallet::rest_models::FundingWalletResponse>(value),
        "getSpotAssetTags"=>typed::<binance_client::core_trading::wallet::rest_models::GetSpotAssetTagsResponse>(value),
        "getAssetsThatCanBeConvertedIntoBnb"=>typed::<binance_client::core_trading::wallet::rest_models::GetAssetsThatCanBeConvertedIntoBnbResponse>(value),
        "getCloudMiningPaymentAndRefundHistory"=>typed::<binance_client::core_trading::wallet::rest_models::GetCloudMiningPaymentAndRefundHistoryResponse>(value),
        "getOpenSymbolList"=>typed::<binance_client::core_trading::wallet::rest_models::GetOpenSymbolListResponse>(value),
        _=>wallet_rest_2(operation,value),
    }
}
fn wallet_rest_2(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "queryUserDelegationHistory"=>typed::<binance_client::core_trading::wallet::rest_models::QueryUserDelegationHistoryResponse>(value),
        "queryUserUniversalTransferHistory"=>typed::<binance_client::core_trading::wallet::rest_models::QueryUserUniversalTransferHistoryResponse>(value),
        "userUniversalTransfer"=>typed::<binance_client::core_trading::wallet::rest_models::UserUniversalTransferResponse>(value),
        "queryUserWalletBalance"=>typed::<binance_client::core_trading::wallet::rest_models::QueryUserWalletBalanceResponse>(value),
        "toggleBnbBurnOnSpotTradeAndMarginInterest"=>typed::<binance_client::core_trading::wallet::rest_models::ToggleBnbBurnOnSpotTradeAndMarginInterestResponse>(value),
        "tradeFee"=>typed::<binance_client::core_trading::wallet::rest_models::TradeFeeResponse>(value),
        "userAsset"=>typed::<binance_client::core_trading::wallet::rest_models::UserAssetResponse>(value),
        "allCoinsInformation"=>typed::<binance_client::core_trading::wallet::rest_models::AllCoinsInformationResponse>(value),
        _=>wallet_rest_3(operation,value),
    }
}
fn wallet_rest_3(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "depositAddress"=>typed::<binance_client::core_trading::wallet::rest_models::DepositAddressResponse>(value),
        "depositHistory"=>typed::<binance_client::core_trading::wallet::rest_models::DepositHistoryResponse>(value),
        "fetchDepositAddressListWithNetwork"=>typed::<binance_client::core_trading::wallet::rest_models::FetchDepositAddressListWithNetworkResponse>(value),
        "fetchWithdrawQuota"=>typed::<binance_client::core_trading::wallet::rest_models::FetchWithdrawQuotaResponse>(value),
        "oneClickArrivalDepositApply"=>typed::<binance_client::core_trading::wallet::rest_models::OneClickArrivalDepositApplyResponse>(value),
        "withdrawHistory"=>typed::<binance_client::core_trading::wallet::rest_models::WithdrawHistoryResponse>(value),
        "getSymbolsDelistScheduleForSpot"=>typed::<binance_client::core_trading::wallet::rest_models::GetSymbolsDelistScheduleForSpotResponse>(value),
        "systemStatus"=>typed::<binance_client::core_trading::wallet::rest_models::SystemStatusResponse>(value),
        _=>wallet_rest_4(operation,value),
    }
}
fn wallet_rest_4(operation: &str, value: Value) -> Result<(), String> {
    match operation{
        "checkQuestionnaireRequirements"=>typed::<binance_client::core_trading::wallet::rest_models::CheckQuestionnaireRequirementsResponse>(value),
        "getCountryList"=>typed::<binance_client::core_trading::wallet::rest_models::GetCountryListResponse>(value),
        "depositHistoryTravelRule"=>typed::<binance_client::core_trading::wallet::rest_models::DepositHistoryTravelRuleResponse>(value),
        "depositHistoryV2"=>typed::<binance_client::core_trading::wallet::rest_models::DepositHistoryV2Response>(value),
        "getRegionList"=>typed::<binance_client::core_trading::wallet::rest_models::GetRegionListResponse>(value),
        "submitDepositQuestionnaire"=>typed::<binance_client::core_trading::wallet::rest_models::SubmitDepositQuestionnaireResponse>(value),
        "submitDepositQuestionnaireTravelRule"=>typed::<binance_client::core_trading::wallet::rest_models::SubmitDepositQuestionnaireTravelRuleResponse>(value),
        "submitDepositQuestionnaireV2"=>typed::<binance_client::core_trading::wallet::rest_models::SubmitDepositQuestionnaireV2Response>(value),
        _=>wallet_rest_5(operation,value),
    }
}
fn wallet_rest_5(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "vaspList" => {
            typed::<binance_client::core_trading::wallet::rest_models::VaspListResponse>(value)
        }
        "withdrawHistoryV1" => typed::<
            binance_client::core_trading::wallet::rest_models::WithdrawHistoryV1Response,
        >(value),
        "withdrawHistoryV2" => typed::<
            binance_client::core_trading::wallet::rest_models::WithdrawHistoryV2Response,
        >(value),
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn spot_session_0(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "session.status" | "session.logout" | "session.logon" => {
            typed::<binance_client::core_trading::spot::event_payloads::SessionStatus>(value)
        }
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn usdm_session_0(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "session.logon" | "session.status" | "session.logout" => {
            typed::<binance_client::core_trading::usdm::event_payloads::SessionStatus>(value)
        }
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn coinm_session_0(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "session.logon" | "session.status" | "session.logout" => {
            typed::<binance_client::core_trading::coinm::event_payloads::SessionStatus>(value)
        }
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
fn margin_ws_0(operation: &str, value: Value) -> Result<(), String> {
    match operation {
        "userDataStream.subscribe.listenToken" => {
            typed::<binance_client::margin::Subscription>(value)
        }
        _ => Err("response fixture dispatch missing operation".into()),
    }
}
