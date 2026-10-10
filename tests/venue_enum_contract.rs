// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Venue enum-valued status and type fields decode to typed enums that keep the
//! venue's exact spelling, and keep a value the venue has not documented as it
//! was sent instead of refusing or guessing it.

#![allow(clippy::unwrap_used, reason = "synthetic contract assertions")]

use binance_client::{coinm, spot, usdm};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::json;
use std::fmt::Debug;

fn decodes_and_reencodes<T>(spelling: &str, expected: &T)
where
    T: DeserializeOwned + Serialize + PartialEq + Debug,
{
    let decoded: T = serde_json::from_value(json!(spelling)).unwrap();
    assert_eq!(&decoded, expected, "{spelling}");
    assert_eq!(serde_json::to_value(&decoded).unwrap(), json!(spelling));
}

#[test]
fn usdm_contract_status_reads_every_documented_value() {
    use usdm::enums::ContractStatus as S;
    let documented = [
        ("PENDING_TRADING", S::PendingTrading),
        ("TRADING", S::Trading),
        ("PRE_DELIVERING", S::PreDelivering),
        ("DELIVERING", S::Delivering),
        ("DELIVERED", S::Delivered),
        ("PRE_SETTLE", S::PreSettle),
        ("SETTLING", S::Settling),
        ("CLOSE", S::Close),
        ("TRADING_HALT", S::TradingHalt),
        ("TRADING_CANCEL_ONLY", S::TradingCancelOnly),
    ];
    for (spelling, status) in &documented {
        decodes_and_reencodes(spelling, status);
        assert_eq!(status.as_str(), *spelling);
    }
}

#[test]
fn coinm_contract_status_reads_every_documented_value() {
    use coinm::enums::ContractStatus as S;
    let documented = [
        ("PENDING_TRADING", S::PendingTrading),
        ("TRADING", S::Trading),
        ("PRE_DELIVERING", S::PreDelivering),
        ("DELIVERING", S::Delivering),
        ("DELIVERED", S::Delivered),
        ("TRADING_HALT", S::TradingHalt),
        ("TRADING_CANCEL_ONLY", S::TradingCancelOnly),
    ];
    for (spelling, status) in &documented {
        decodes_and_reencodes(spelling, status);
        assert_eq!(status.as_str(), *spelling);
    }
}

#[test]
fn usdm_contract_type_reads_every_documented_value() {
    use usdm::enums::ContractType as T;
    let documented = [
        ("PERPETUAL", T::Perpetual),
        ("CURRENT_MONTH", T::CurrentMonth),
        ("NEXT_MONTH", T::NextMonth),
        ("CURRENT_QUARTER", T::CurrentQuarter),
        ("NEXT_QUARTER", T::NextQuarter),
        ("PERPETUAL_DELIVERING", T::PerpetualDelivering),
        ("TRADIFI_PERPETUAL", T::TradifiPerpetual),
    ];
    for (spelling, kind) in &documented {
        decodes_and_reencodes(spelling, kind);
        assert_eq!(kind.as_str(), *spelling);
    }
}

#[test]
fn coinm_contract_type_reads_every_documented_value() {
    use coinm::enums::ContractType as T;
    let documented = [
        ("PERPETUAL", T::Perpetual),
        ("CURRENT_QUARTER", T::CurrentQuarter),
        ("NEXT_QUARTER", T::NextQuarter),
        ("CURRENT_QUARTER_DELIVERING", T::CurrentQuarterDelivering),
        ("NEXT_QUARTER_DELIVERING", T::NextQuarterDelivering),
        ("PERPETUAL_DELIVERING", T::PerpetualDelivering),
    ];
    for (spelling, kind) in &documented {
        decodes_and_reencodes(spelling, kind);
        assert_eq!(kind.as_str(), *spelling);
    }
}

#[test]
fn spot_symbol_status_reads_every_documented_value() {
    use spot::enums::SymbolStatus as S;
    let documented = [
        ("TRADING", S::Trading),
        ("END_OF_DAY", S::EndOfDay),
        ("HALT", S::Halt),
        ("BREAK", S::Break),
        ("CANCEL_ONLY", S::CancelOnly),
    ];
    for (spelling, status) in &documented {
        decodes_and_reencodes(spelling, status);
        assert_eq!(status.as_str(), *spelling);
    }
}

#[test]
fn usdm_funding_rate_type_reads_every_documented_value() {
    use usdm::enums::FundingRateType as T;
    for (spelling, kind) in &[("Regular", T::Regular), ("Special", T::Special)] {
        decodes_and_reencodes(spelling, kind);
        assert_eq!(kind.as_str(), *spelling);
    }
}

#[test]
fn undocumented_values_are_kept_exactly_and_reencode_unchanged() {
    decodes_and_reencodes(
        "CURRENT_QUARTER DELIVERING",
        &coinm::enums::ContractType::Unknown("CURRENT_QUARTER DELIVERING".to_owned()),
    );
    decodes_and_reencodes(
        "CURRENT_WEEK",
        &usdm::enums::ContractType::Unknown("CURRENT_WEEK".to_owned()),
    );
    decodes_and_reencodes(
        "TRADIFI_PERPETUAL",
        &coinm::enums::ContractType::Unknown("TRADIFI_PERPETUAL".to_owned()),
    );
    decodes_and_reencodes(
        "SETTLING",
        &coinm::enums::ContractStatus::Unknown("SETTLING".to_owned()),
    );
    decodes_and_reencodes(
        "AUCTION_MATCH",
        &usdm::enums::ContractStatus::Unknown("AUCTION_MATCH".to_owned()),
    );
    decodes_and_reencodes(
        "AUCTION_MATCH",
        &spot::enums::SymbolStatus::Unknown("AUCTION_MATCH".to_owned()),
    );
    decodes_and_reencodes(
        "Dividend",
        &usdm::enums::FundingRateType::Unknown("Dividend".to_owned()),
    );
    // Spelling is exact: a case variant of a documented value is not that value.
    decodes_and_reencodes(
        "trading",
        &spot::enums::SymbolStatus::Unknown("trading".to_owned()),
    );
    assert_eq!(
        usdm::enums::ContractType::Unknown("NEXT_WEEK".to_owned()).as_str(),
        "NEXT_WEEK"
    );
}

#[test]
fn usdm_exchange_information_symbol_types_its_status_and_contract_type() {
    // Official example response, Exchange Information (USD-M market data).
    let metadata: usdm::rest_models::ExchangeInformationResponse = serde_json::from_value(json!({
        "exchangeFilters": [], "rateLimits": [], "serverTime": 1_565_613_908_500_i64,
        "assets": [],
        "symbols": [{
            "symbol": "BLZUSDT", "pair": "BLZUSDT", "contractType": "PERPETUAL",
            "deliveryDate": 4_133_404_800_000_i64, "onboardDate": 1_598_252_400_000_i64,
            "status": "TRADING", "maintMarginPercent": "2.5000",
            "requiredMarginPercent": "5.0000", "baseAsset": "BLZ", "quoteAsset": "USDT",
            "marginAsset": "USDT", "pricePrecision": 5, "quantityPrecision": 0,
            "baseAssetPrecision": 8, "quotePrecision": 8, "underlyingType": "COIN",
            "underlyingSubType": ["STORAGE"], "settlePlan": 0, "triggerProtect": "0.15",
            "filters": [], "orderTypes": ["LIMIT"], "timeInForce": ["GTC"],
            "liquidationFee": "0.010000", "marketTakeBound": "0.30"
        }],
        "timezone": "UTC"
    }))
    .unwrap();
    let symbol = &metadata.symbols.unwrap()[0];
    assert_eq!(symbol.status, Some(usdm::enums::ContractStatus::Trading));
    assert_eq!(
        symbol.contract_type,
        Some(usdm::enums::ContractType::Perpetual)
    );
}

#[test]
fn coinm_exchange_information_symbol_types_its_status_and_contract_type() {
    // Official example response, Exchange Information (COIN-M market data).
    let metadata: coinm::rest_models::ExchangeInformationResponse = serde_json::from_value(json!({
        "exchangeFilters": [], "rateLimits": [], "serverTime": 1_565_613_908_500_i64,
        "symbols": [{
            "filters": [], "orderTypes": ["LIMIT"], "timeInForce": ["GTC"],
            "liquidationFee": "0.010000", "marketTakeBound": "0.30",
            "symbol": "BTCUSD_200925", "pair": "BTCUSD", "contractType": "CURRENT_QUARTER",
            "deliveryDate": 1_601_020_800_000_i64, "onboardDate": 1_590_739_200_000_i64,
            "contractStatus": "TRADING", "contractSize": 100, "quoteAsset": "USD",
            "baseAsset": "BTC", "marginAsset": "BTC", "pricePrecision": 1,
            "quantityPrecision": 0, "baseAssetPrecision": 8, "quotePrecision": 8,
            "equalQtyPrecision": 4, "triggerProtect": "0.0500",
            "maintMarginPercent": "2.5000", "requiredMarginPercent": "5.0000",
            "underlyingType": "COIN"
        }],
        "timezone": "UTC"
    }))
    .unwrap();
    let symbol = &metadata.symbols.unwrap()[0];
    assert_eq!(
        symbol.contract_status,
        Some(coinm::enums::ContractStatus::Trading)
    );
    assert_eq!(
        symbol.contract_type,
        Some(coinm::enums::ContractType::CurrentQuarter)
    );
}

#[test]
fn spot_exchange_information_types_symbol_status_over_rest_and_websocket() {
    // Official example symbol, Exchange information (Spot REST and WebSocket API).
    let symbol = json!({
        "symbol": "ETHBTC", "status": "TRADING", "baseAsset": "ETH",
        "baseAssetPrecision": 8, "quoteAsset": "BTC", "quotePrecision": 8,
        "quoteAssetPrecision": 8, "baseCommissionPrecision": 8,
        "quoteCommissionPrecision": 8,
        "orderTypes": ["LIMIT", "LIMIT_MAKER", "MARKET", "STOP_LOSS", "STOP_LOSS_LIMIT",
            "TAKE_PROFIT", "TAKE_PROFIT_LIMIT"],
        "icebergAllowed": true, "ocoAllowed": true, "otoAllowed": true, "opoAllowed": true,
        "quoteOrderQtyMarketAllowed": true, "allowTrailingStop": false,
        "cancelReplaceAllowed": false, "amendAllowed": false, "pegInstructionsAllowed": true,
        "isSpotTradingAllowed": true, "isMarginTradingAllowed": true, "filters": [],
        "permissions": [], "permissionSets": [["SPOT", "MARGIN"]],
        "defaultSelfTradePreventionMode": "NONE", "allowedSelfTradePreventionModes": ["NONE"]
    });
    let reply = json!({
        "timezone": "UTC", "serverTime": 1_565_246_363_776_i64, "rateLimits": [],
        "exchangeFilters": [], "symbols": [symbol]
    });
    let rest: spot::rest_models::ExchangeInfoResponse =
        serde_json::from_value(reply.clone()).unwrap();
    assert_eq!(
        rest.symbols.unwrap()[0].status,
        Some(spot::enums::SymbolStatus::Trading)
    );
    let ws: spot::ws_models::ExchangeInfoResponse = serde_json::from_value(reply).unwrap();
    assert_eq!(
        ws.symbols.unwrap()[0].status,
        Some(spot::enums::SymbolStatus::Trading)
    );
}

#[test]
fn futures_market_data_rows_type_their_contract_type() {
    // Official example responses: Basis (both markets), and COIN-M Open Interest,
    // Open Interest Statistics and Taker Buy/Sell Volume. The USD-M example's empty
    // `annualizedBasisRate` is left out: its decoding is tracked in issue #79.
    let basis: usdm::rest_models::BasisResponse = serde_json::from_value(json!([{
        "indexPrice": "34400.15945055", "contractType": "PERPETUAL", "basisRate": "0.0004",
        "futuresPrice": "34414.10", "basis": "13.94054945",
        "pair": "BTCUSDT", "timestamp": 1_698_742_800_000_i64
    }]))
    .unwrap();
    assert_eq!(
        basis[0].contract_type,
        Some(usdm::enums::ContractType::Perpetual)
    );

    let basis: coinm::rest_models::BasisResponse = serde_json::from_value(json!([{
        "indexPrice": "29269.93972727", "contractType": "CURRENT_QUARTER", "basisRate": "0.0024",
        "futuresPrice": "29341.3", "annualizedBasisRate": "0.0283", "basis": "71.36027273",
        "pair": "BTCUSD", "timestamp": 1_653_381_600_000_i64
    }]))
    .unwrap();
    assert_eq!(
        basis[0].contract_type,
        Some(coinm::enums::ContractType::CurrentQuarter)
    );

    let open_interest: coinm::rest_models::OpenInterestResponse = serde_json::from_value(json!({
        "symbol": "BTCUSD_200626", "pair": "BTCUSD", "openInterest": "15004",
        "contractType": "CURRENT_QUARTER", "time": 1_591_261_042_378_i64
    }))
    .unwrap();
    assert_eq!(
        open_interest.contract_type,
        Some(coinm::enums::ContractType::CurrentQuarter)
    );

    let history: coinm::rest_models::OpenInterestStatisticsResponse =
        serde_json::from_value(json!([{
            "pair": "BTCUSD", "contractType": "CURRENT_QUARTER", "sumOpenInterest": "20403",
            "sumOpenInterestValue": "176196512.23400000", "timestamp": 1_591_261_042_378_i64
        }]))
        .unwrap();
    assert_eq!(
        history[0].contract_type,
        Some(coinm::enums::ContractType::CurrentQuarter)
    );

    let volume: coinm::rest_models::TakerBuySellVolumeResponse = serde_json::from_value(json!([{
        "pair": "BTCUSD", "contractType": "CURRENT_QUARTER", "takerBuyVol": "387",
        "takerSellVol": "248", "takerBuyVolValue": "2342.1220",
        "takerSellVolValue": "4213.9800", "timestamp": 1_591_261_042_378_i64
    }]))
    .unwrap();
    assert_eq!(
        volume[0].contract_type,
        Some(coinm::enums::ContractType::CurrentQuarter)
    );
}

#[test]
fn usdm_funding_rate_history_types_its_rate_type() {
    // Official example response, Get Funding Rate History (USD-M).
    let rows: usdm::rest_models::GetFundingRateHistoryResponse = serde_json::from_value(json!([{
        "symbol": "BTCUSDT", "fundingRate": "-0.03750000", "fundingTime": 1_570_608_000_000_i64,
        "markPrice": "34287.54619963", "rateType": "Regular"
    }]))
    .unwrap();
    assert_eq!(
        rows[0].rate_type,
        Some(usdm::enums::FundingRateType::Regular)
    );
}

#[test]
fn contract_info_streams_type_status_and_contract_type_in_both_markets() {
    // Official example payloads, Contract Info Stream (USD-M and COIN-M).
    let usdm_event: usdm::stream_models::ContractInfoStreamEvent = serde_json::from_value(json!({
        "e": "contractInfo", "E": 1_669_356_423_908_i64, "s": "IOTAUSDT", "ct": "PERPETUAL",
        "dt": 4_133_404_800_000_i64, "ot": 1_569_398_400_000_i64, "cs": "TRADING",
        "bks": [{"bs": 1, "bnf": 0, "bnc": 5000, "mmr": 0.01, "cf": 0, "mi": 21, "ma": 50}],
        "st": 1
    }))
    .unwrap();
    assert_eq!(usdm_event.cs, usdm::enums::ContractStatus::Trading);
    assert_eq!(usdm_event.ct, usdm::enums::ContractType::Perpetual);

    let coinm_event: coinm::stream_models::ContractInfoStreamEvent =
        serde_json::from_value(json!({
            "e": "contractInfo", "E": 1_669_647_330_375_i64, "s": "APTUSD_PERP", "ps": "APTUSD",
            "ct": "PERPETUAL", "dt": 4_133_404_800_000_i64, "ot": 1_666_594_800_000_i64,
            "cs": "TRADING",
            "bks": [{"bs": 1, "bnf": 0, "bnc": 500_000, "mmr": 0.0065, "cf": 75, "mi": 51, "ma": 75}],
            "st": 1
        }))
        .unwrap();
    assert_eq!(coinm_event.cs, coinm::enums::ContractStatus::Trading);
    assert_eq!(coinm_event.ct, coinm::enums::ContractType::Perpetual);
}

#[test]
fn continuous_kline_streams_type_their_contract_type_in_both_markets() {
    // Official example payloads, Continuous Contract Kline/Candlestick Streams.
    let usdm_event: usdm::stream_models::ContinuousContractKlineCandlestickStreamsEvent =
        serde_json::from_value(json!({
            "e": "continuous_kline", "E": 1_607_443_058_651_i64, "ps": "BTCUSDT",
            "ct": "PERPETUAL",
            "k": {"t": 1_607_443_020_000_i64, "T": 1_607_443_079_999_i64, "i": "1m",
                "f": 116_467_658_886_i64, "L": 116_468_012_423_i64, "o": "18787.00",
                "c": "18804.04", "h": "18804.04", "l": "18786.54", "v": "197.664", "n": 543,
                "x": false, "q": "3715253.19494", "V": "184.769", "Q": "3472925.84746",
                "B": "0"}
        }))
        .unwrap();
    assert_eq!(usdm_event.ct, Some(usdm::enums::ContractType::Perpetual));

    let coinm_event: coinm::stream_models::ContinuousContractKlineCandlestickStreamsEvent =
        serde_json::from_value(json!({
            "e": "continuous_kline", "E": 1_591_261_542_539_i64, "ps": "BTCUSD",
            "ct": "NEXT_QUARTER",
            "k": {"t": 1_591_261_500_000_i64, "T": 1_591_261_559_999_i64, "i": "1m",
                "f": 606_400, "L": 606_430, "o": "9638.9", "c": "9639.8", "h": "9639.8",
                "l": "9638.6", "v": "156", "n": 31, "x": false, "q": "1.61836886",
                "V": "73", "Q": "0.75731156", "B": "0"}
        }))
        .unwrap();
    assert_eq!(coinm_event.ct, coinm::enums::ContractType::NextQuarter);
}

#[test]
fn an_undocumented_contract_status_on_the_stream_is_retained_not_refused() {
    let event: usdm::stream_models::ContractInfoStreamEvent = serde_json::from_value(json!({
        "e": "contractInfo", "E": 1, "s": "BTCUSDT_261225", "ct": "CURRENT_QUARTER DELIVERING",
        "dt": 1_798_185_600_000_i64, "ot": 1_782_979_200_000_i64, "cs": "AUCTION_MATCH"
    }))
    .unwrap();
    assert_eq!(
        event.cs,
        usdm::enums::ContractStatus::Unknown("AUCTION_MATCH".to_owned())
    );
    assert_eq!(
        event.ct,
        usdm::enums::ContractType::Unknown("CURRENT_QUARTER DELIVERING".to_owned())
    );
}
