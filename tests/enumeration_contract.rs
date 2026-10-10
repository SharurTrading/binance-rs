// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Documented response enumerations retain known variants and exact future spellings.
#![allow(clippy::unwrap_used, reason = "synthetic protocol assertions")]

use binance_client::{coinm, convert, spot, usdm, wallet};
use serde_json::json;

macro_rules! strings {
    ($ty:path, $values:expr) => {{
        use $ty as Enum;
        for spelling in $values {
            let decoded: Enum = serde_json::from_value(json!(spelling)).unwrap();
            assert!(!matches!(decoded, Enum::Unknown(_)), "documented {spelling}");
            assert_eq!(decoded.as_str(), spelling);
            assert_eq!(serde_json::to_value(&decoded).unwrap(), json!(spelling));
        }
        for spelling in ["VENUE_FUTURE_VALUE", "future value", ""] {
            let decoded: Enum = serde_json::from_value(json!(spelling)).unwrap();
            assert!(matches!(&decoded, Enum::Unknown(value) if value == spelling));
            assert_eq!(serde_json::to_value(decoded).unwrap(), json!(spelling));
        }
        assert!(serde_json::from_value::<Enum>(json!(null)).is_err());
        assert!(serde_json::from_value::<Enum>(json!(3)).is_err());
    }};
}

#[test]
fn futures_documented_order_values_round_trip_without_borrowing_other_markets() {
    strings!(usdm::enums::OrderSide, ["BUY", "SELL"]);
    strings!(coinm::enums::OrderSide, ["BUY", "SELL"]);
    strings!(
        usdm::enums::OrderStatus,
        [
            "NEW",
            "PARTIALLY_FILLED",
            "FILLED",
            "CANCELED",
            "REJECTED",
            "EXPIRED",
            "EXPIRED_IN_MATCH"
        ]
    );
    strings!(
        coinm::enums::OrderStatus,
        [
            "NEW",
            "PARTIALLY_FILLED",
            "FILLED",
            "CANCELED",
            "EXPIRED",
            "EXPIRED_IN_MATCH"
        ]
    );
    strings!(
        usdm::enums::OrderType,
        [
            "LIMIT",
            "MARKET",
            "STOP",
            "STOP_MARKET",
            "TAKE_PROFIT",
            "TAKE_PROFIT_MARKET",
            "TRAILING_STOP_MARKET",
            "LIQUIDATION"
        ]
    );
    strings!(
        coinm::enums::OrderType,
        [
            "LIMIT",
            "MARKET",
            "STOP",
            "STOP_MARKET",
            "TAKE_PROFIT",
            "TAKE_PROFIT_MARKET",
            "TRAILING_STOP_MARKET",
            "LIQUIDATION"
        ]
    );
    strings!(usdm::enums::PositionSide, ["BOTH", "LONG", "SHORT"]);
    strings!(coinm::enums::PositionSide, ["BOTH", "LONG", "SHORT"]);
    strings!(
        usdm::enums::TimeInForce,
        ["GTC", "IOC", "FOK", "GTX", "GTD", "RPI"]
    );
    strings!(coinm::enums::TimeInForce, ["GTC", "IOC", "FOK", "GTX"]);
    strings!(usdm::enums::WorkingType, ["MARK_PRICE", "CONTRACT_PRICE"]);
    strings!(coinm::enums::WorkingType, ["MARK_PRICE", "CONTRACT_PRICE"]);
    strings!(
        usdm::enums::PriceMatch,
        [
            "NONE",
            "OPPONENT",
            "OPPONENT_5",
            "OPPONENT_10",
            "OPPONENT_20",
            "QUEUE",
            "QUEUE_5",
            "QUEUE_10",
            "QUEUE_20"
        ]
    );
    strings!(
        coinm::enums::PriceMatch,
        [
            "NONE",
            "OPPONENT",
            "OPPONENT_5",
            "OPPONENT_10",
            "OPPONENT_20",
            "QUEUE",
            "QUEUE_5",
            "QUEUE_10",
            "QUEUE_20"
        ]
    );
}

#[test]
fn spot_order_and_exchange_information_values_round_trip() {
    strings!(spot::enums::OrderSide, ["BUY", "SELL"]);
    strings!(
        spot::enums::OrderType,
        [
            "LIMIT",
            "MARKET",
            "STOP_LOSS",
            "STOP_LOSS_LIMIT",
            "TAKE_PROFIT",
            "TAKE_PROFIT_LIMIT",
            "LIMIT_MAKER"
        ]
    );
    strings!(
        spot::enums::OrderStatus,
        [
            "NEW",
            "PENDING_NEW",
            "PARTIALLY_FILLED",
            "FILLED",
            "CANCELED",
            "PENDING_CANCEL",
            "REJECTED",
            "EXPIRED",
            "EXPIRED_IN_MATCH"
        ]
    );
    strings!(spot::enums::TimeInForce, ["GTC", "IOC", "FOK"]);
    strings!(
        spot::enums::SelfTradePreventionMode,
        [
            "NONE",
            "EXPIRE_MAKER",
            "EXPIRE_TAKER",
            "EXPIRE_BOTH",
            "DECREMENT",
            "TRANSFER"
        ]
    );
    strings!(
        spot::enums::ListStatusType,
        ["RESPONSE", "EXEC_STARTED", "UPDATED", "ALL_DONE"]
    );
    strings!(
        spot::enums::ListOrderStatus,
        ["EXECUTING", "ALL_DONE", "REJECT"]
    );
    strings!(spot::enums::ContingencyType, ["OCO", "OTO"]);
    strings!(
        spot::enums::ExecutionType,
        [
            "NEW",
            "CANCELED",
            "REPLACED",
            "REJECTED",
            "TRADE",
            "EXPIRED",
            "TRADE_PREVENTION"
        ]
    );
}

#[test]
fn kline_month_and_minute_intervals_remain_distinct() {
    let minute: usdm::enums::KlineInterval = serde_json::from_value(json!("1m")).unwrap();
    let month: usdm::enums::KlineInterval = serde_json::from_value(json!("1M")).unwrap();
    assert_eq!(minute, usdm::enums::KlineInterval::Minute1);
    assert_eq!(month, usdm::enums::KlineInterval::Month1);
    assert_ne!(minute, month);
    strings!(
        usdm::enums::KlineInterval,
        [
            "1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d",
            "1w", "1M"
        ]
    );
    strings!(
        coinm::enums::KlineInterval,
        [
            "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d", "1w",
            "1M"
        ]
    );
}

#[test]
fn futures_order_models_type_known_and_future_order_evidence() {
    let wire = json!({"orderId":1,"symbol":"BTCUSDT","side":"BUY","positionSide":"BOTH","status":"NEW","type":"LIMIT","origType":"LIMIT","timeInForce":"GTC","workingType":"CONTRACT_PRICE","priceMatch":"NONE","selfTradePreventionMode":"VENUE_FUTURE_VALUE"});
    let rest: usdm::rest_models::NewOrderResponse = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(rest.side, Some(usdm::enums::OrderSide::Buy));
    assert_eq!(rest.status, Some(usdm::enums::OrderStatus::New));
    assert!(
        matches!(rest.self_trade_prevention_mode, Some(usdm::enums::SelfTradePreventionMode::Unknown(ref value)) if value == "VENUE_FUTURE_VALUE")
    );
    let ws: usdm::ws_models::NewOrderResponse = serde_json::from_value(wire).unwrap();
    assert_eq!(ws.side, Some(usdm::enums::OrderSide::Buy));
    assert_eq!(serde_json::to_value(rest).unwrap()["side"], "BUY");
}

#[test]
fn unknown_contract_spellings_without_evidence_stay_unknown() {
    for spelling in ["CURRENT_WEEK", "NEXT_WEEK", "CURRENT_QUARTER DELIVERING"] {
        let value: usdm::enums::ContractType = serde_json::from_value(json!(spelling)).unwrap();
        assert!(matches!(&value, usdm::enums::ContractType::Unknown(v) if v == spelling));
        assert_eq!(serde_json::to_value(value).unwrap(), json!(spelling));
    }
    for spelling in ["CURRENT_QUARTER DELIVERING", "TRADIFI_PERPETUAL"] {
        let value: coinm::enums::ContractType = serde_json::from_value(json!(spelling)).unwrap();
        assert!(matches!(&value, coinm::enums::ContractType::Unknown(v) if v == spelling));
        assert_eq!(serde_json::to_value(value).unwrap(), json!(spelling));
    }
    for spelling in ["SETTLING", "PRE_SETTLE", "CLOSE"] {
        let value: coinm::enums::ContractStatus = serde_json::from_value(json!(spelling)).unwrap();
        assert!(matches!(&value, coinm::enums::ContractStatus::Unknown(v) if v == spelling));
        assert_eq!(serde_json::to_value(value).unwrap(), json!(spelling));
    }
}

#[test]
fn convert_order_status_retains_typed_native_receipt() {
    let receipt: convert::rest_models::AcceptQuoteResponse = serde_json::from_value(json!({"orderId":"933256278426274426","createTime":1_623_381_330_472_i64,"orderStatus":"SUCCESS"})).unwrap();
    assert_eq!(receipt.order_status, convert::enums::OrderStatus::Success);
    assert_eq!(
        serde_json::to_value(receipt).unwrap()["orderStatus"],
        "SUCCESS"
    );
}

macro_rules! integers {
    ($ty:path, $values:expr) => {{
        use $ty as Enum;
        for code in $values {
            let decoded: Enum = serde_json::from_value(json!(code)).unwrap();
            assert!(!matches!(decoded, Enum::Unknown(_)), "documented code {code}");
            assert_eq!(decoded.value(), code);
            assert_eq!(serde_json::to_value(decoded).unwrap(), json!(code));
        }
        for code in [-100_i64, 999] {
            let decoded: Enum = serde_json::from_value(json!(code)).unwrap();
            assert!(matches!(decoded, Enum::Unknown(value) if value == code));
            assert_eq!(serde_json::to_value(decoded).unwrap(), json!(code));
        }
        assert!(serde_json::from_value::<Enum>(json!("1")).is_err());
        assert!(serde_json::from_value::<Enum>(json!(1.5)).is_err());
    }};
}

#[test]
fn usdm_remaining_documented_values_keep_native_shape() {
    strings!(
        usdm::enums::FilterType,
        [
            "PRICE_FILTER",
            "LOT_SIZE",
            "MARKET_LOT_SIZE",
            "MAX_NUM_ORDERS",
            "MAX_NUM_ALGO_ORDERS",
            "PERCENT_PRICE",
            "MIN_NOTIONAL"
        ]
    );
    strings!(
        usdm::enums::ExecutionType,
        [
            "NEW",
            "CANCELED",
            "CALCULATED",
            "EXPIRED",
            "TRADE",
            "AMENDMENT"
        ]
    );
    strings!(
        usdm::enums::AlgoStatus,
        [
            "NEW",
            "CANCELED",
            "TRIGGERING",
            "TRIGGERED",
            "FINISHED",
            "REJECTED",
            "EXPIRED"
        ]
    );
    strings!(usdm::enums::AlgoType, ["CONDITIONAL"]);
    strings!(
        usdm::enums::SelfTradePreventionMode,
        ["NONE", "EXPIRE_TAKER", "EXPIRE_BOTH", "EXPIRE_MAKER"]
    );
}

#[test]
fn coinm_remaining_documented_values_keep_native_shape() {
    strings!(
        coinm::enums::FilterType,
        [
            "PRICE_FILTER",
            "LOT_SIZE",
            "MARKET_LOT_SIZE",
            "MAX_NUM_ORDERS",
            "PERCENT_PRICE"
        ]
    );
    strings!(
        coinm::enums::ExecutionType,
        [
            "NEW",
            "CANCELED",
            "CALCULATED",
            "EXPIRED",
            "TRADE",
            "AMENDMENT"
        ]
    );
    strings!(
        coinm::enums::AlgoStatus,
        [
            "NEW",
            "CANCELED",
            "TRIGGERING",
            "TRIGGERED",
            "FINISHED",
            "REJECTED",
            "EXPIRED"
        ]
    );
    strings!(coinm::enums::AlgoType, ["CONDITIONAL"]);
    strings!(
        coinm::enums::SelfTradePreventionMode,
        ["NONE", "EXPIRE_TAKER", "EXPIRE_BOTH", "EXPIRE_MAKER"]
    );
}

#[test]
fn spot_remaining_documented_values_keep_native_shape() {
    strings!(
        spot::enums::Permission,
        [
            "SPOT",
            "MARGIN",
            "LEVERAGED",
            "TRD_GRP_002",
            "TRD_GRP_003",
            "TRD_GRP_004",
            "TRD_GRP_005",
            "TRD_GRP_006",
            "TRD_GRP_007",
            "TRD_GRP_008",
            "TRD_GRP_009",
            "TRD_GRP_010",
            "TRD_GRP_011",
            "TRD_GRP_012",
            "TRD_GRP_013",
            "TRD_GRP_014",
            "TRD_GRP_015",
            "TRD_GRP_016",
            "TRD_GRP_017",
            "TRD_GRP_018",
            "TRD_GRP_019",
            "TRD_GRP_020",
            "TRD_GRP_021",
            "TRD_GRP_022",
            "TRD_GRP_023",
            "TRD_GRP_024",
            "TRD_GRP_025"
        ]
    );
    strings!(spot::enums::AllocationType, ["SOR"]);
    strings!(spot::enums::WorkingFloor, ["EXCHANGE", "SOR"]);
    strings!(
        spot::enums::KlineInterval,
        [
            "1s", "1m", "3m", "5m", "15m", "30m", "1h", "2h", "4h", "6h", "8h", "12h", "1d", "3d",
            "1w", "1M"
        ]
    );
}

#[test]
fn wallet_remaining_documented_values_keep_native_shape() {
    integers!(wallet::enums::DepositStatus, [0, 1, 2, 6, 7, 8]);
    integers!(wallet::enums::WithdrawStatus, [0, 2, 3, 4, 6]);
    integers!(wallet::enums::TransferDirection, [0, 1]);
    integers!(wallet::enums::WalletType, [0, 1]);
    integers!(wallet::enums::DepositTravelRuleStatus, [0, 1]);
    integers!(wallet::enums::SystemStatus, [0, 1]);
    strings!(
        wallet::enums::TravelRuleVerificationStatus,
        ["PASSED", "PENDING", "REJECTED"]
    );
    integers!(wallet::enums::CloudMiningPaymentType, [248, 249]);
    strings!(wallet::enums::CloudMiningStatus, ["S"]);
}

#[test]
fn convert_remaining_documented_values_keep_native_shape() {
    strings!(
        convert::enums::OrderStatus,
        ["PROCESS", "ACCEPT_SUCCESS", "SUCCESS", "FAIL"]
    );
}

#[test]
fn exchange_information_fields_are_typed_in_rest_and_websocket_models() {
    let futures = json!({"symbol":"BTCUSDT","orderTypes":["LIMIT","FUTURE_TYPE"],"timeInForce":["GTC","FUTURE_TIME"],"filters":[{"filterType":"PRICE_FILTER"},{"filterType":"FUTURE_FILTER"}]});
    let value: usdm::rest_models::ExchangeInformationResponseSymbolsItem =
        serde_json::from_value(futures.clone()).unwrap();
    assert_eq!(
        value.order_types.as_ref().unwrap()[0],
        usdm::enums::OrderType::Limit
    );
    assert!(
        matches!(&value.order_types.as_ref().unwrap()[1], usdm::enums::OrderType::Unknown(v) if v=="FUTURE_TYPE")
    );
    assert_eq!(
        value.time_in_force.as_ref().unwrap()[0],
        usdm::enums::TimeInForce::Gtc
    );
    assert_eq!(
        value.filters.as_ref().unwrap()[0].filter_type,
        Some(usdm::enums::FilterType::PriceFilter)
    );
    assert_eq!(
        serde_json::to_value(value).unwrap()["filters"],
        futures["filters"]
    );
    let coin: coinm::rest_models::ExchangeInformationResponseSymbolsItem =
        serde_json::from_value(futures).unwrap();
    assert_eq!(
        coin.order_types.as_ref().unwrap()[0],
        coinm::enums::OrderType::Limit
    );
    let spot_wire = json!({"symbol":"BTCUSDT","status":"TRADING","orderTypes":["LIMIT"],"permissions":["SPOT","FUTURE_PERMISSION"],"permissionSets":[["MARGIN","FUTURE_PERMISSION"]],"defaultSelfTradePreventionMode":"NONE","allowedSelfTradePreventionModes":["NONE","FUTURE_STP"]});
    let rest: spot::rest_models::ExchangeInfoResponseSymbolsItem =
        serde_json::from_value(spot_wire.clone()).unwrap();
    assert_eq!(
        rest.permissions.as_ref().unwrap()[0],
        spot::enums::Permission::Spot
    );
    assert!(
        matches!(&rest.permission_sets.as_ref().unwrap()[0][1],spot::enums::Permission::Unknown(v) if v=="FUTURE_PERMISSION")
    );
    assert_eq!(
        serde_json::to_value(rest).unwrap()["permissionSets"],
        spot_wire["permissionSets"]
    );
    let ws: spot::ws_models::ExchangeInfoResponseSymbolsItem =
        serde_json::from_value(spot_wire).unwrap();
    assert_eq!(
        ws.default_self_trade_prevention_mode,
        Some(spot::enums::SelfTradePreventionMode::None)
    );
}

#[test]
fn native_order_and_kline_events_preserve_typed_evidence_and_financial_values() {
    let wire = json!({"e":"ORDER_TRADE_UPDATE","E":1,"T":1,"o":{"s":"BTCUSDT","c":"native-order","i":42,"q":"1","l":"1","z":"1","L":"1","T":1,"t":1,"S":"SELL","o":"LIMIT","f":"GTC","x":"TRADE","X":"FILLED","ps":"LONG","wt":"CONTRACT_PRICE","V":"NONE","pm":"NONE","p":"-0.123456789123456789"}});
    let event: usdm::stream_models::OrderTradeUpdateEvent =
        serde_json::from_value(wire.clone()).unwrap();
    let order = &event.o;
    assert_eq!(order.upper_s, usdm::enums::OrderSide::Sell);
    assert_eq!(order.upper_x, usdm::enums::OrderStatus::Filled);
    assert_eq!(order.x, usdm::enums::ExecutionType::Trade);
    assert_eq!(order.p.to_string(), "-0.123456789123456789");
    assert_eq!(serde_json::to_value(event).unwrap()["o"]["S"], "SELL");
    let coin: coinm::stream_models::OrderTradeUpdateEvent = serde_json::from_value(wire).unwrap();
    assert_eq!(coin.o.upper_x, coinm::enums::OrderStatus::Filled);
    let event: spot::stream_models::KlineEvent = serde_json::from_value(
        json!({"e":"kline","E":1,"s":"BTCUSDT","k":{"i":"1M","o":"1","c":"2","h":"3","l":"0"}}),
    )
    .unwrap();
    assert_eq!(event.k.i, Some(spot::enums::KlineInterval::Month1));
    let event: usdm::stream_models::AllMarketLiquidationOrderStreamsEvent = serde_json::from_value(
        json!({"e":"forceOrder","E":1,"o":{"S":"SELL","o":"LIMIT","f":"IOC","X":"FILLED"}}),
    )
    .unwrap();
    assert_eq!(
        event.o.as_ref().unwrap().f,
        Some(usdm::enums::TimeInForce::Ioc)
    );
}

#[test]
fn wallet_status_codes_remain_integer_and_future_codes_are_preserved() {
    let value:wallet::rest_models::DepositHistoryResponseItem=serde_json::from_value(json!({"coin":"BTC","amount":"0.001","status":1,"transferType":0,"walletType":0,"travelRuleStatus":1})).unwrap();
    assert_eq!(value.status, Some(wallet::enums::DepositStatus::Success));
    assert_eq!(value.wallet_type, Some(wallet::enums::WalletType::Spot));
    assert_eq!(serde_json::to_value(value).unwrap()["status"], 1);
    let value: wallet::rest_models::WithdrawHistoryResponseItem = serde_json::from_value(
        json!({"coin":"BTC","amount":"0.001","status":5,"transferType":1,"walletType":1}),
    )
    .unwrap();
    assert_eq!(
        value.status,
        Some(wallet::enums::WithdrawStatus::Unknown(5))
    );
    assert_eq!(serde_json::to_value(value).unwrap()["status"], 5);
}

#[test]
fn spot_sbe_exchange_information_decodes_into_the_same_typed_public_model() {
    // Official spot_3_4.xml, template 103: no fixed root fields; native groups.
    let mut wire = [0_u16, 103, 3, 4]
        .into_iter()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    wire.extend_from_slice(&11_u16.to_le_bytes());
    wire.extend_from_slice(&1_u32.to_le_bytes());
    wire.extend_from_slice(&[2, 1, 1]); // REQUEST_WEIGHT, MINUTE, intervalNum
    wire.extend_from_slice(&6000_i64.to_le_bytes());
    for (width, count) in [(0_u16, 0_u32), (19, 1)] {
        wire.extend_from_slice(&width.to_le_bytes());
        wire.extend_from_slice(&count.to_le_bytes());
    }
    wire.extend_from_slice(&[0, 8, 8, 8, 8]); // TRADING and precisions
    wire.extend_from_slice(&3_u16.to_le_bytes()); // MARKET and LIMIT bits
    wire.extend_from_slice(&[1; 9]); // native boolEnum fields
    wire.extend_from_slice(&[1, 3, 1]); // NONE; allowed NONE + EXPIRE_TAKER; peg bool
    for (width, count) in [(0_u16, 0_u32), (0, 1), (0, 2)] {
        wire.extend_from_slice(&width.to_le_bytes());
        wire.extend_from_slice(&count.to_le_bytes());
    }
    for text in ["SPOT", "FUTURE_PERMISSION", "BTCUSDT", "BTC", "USDT"] {
        wire.push(u8::try_from(text.len()).unwrap());
        wire.extend_from_slice(text.as_bytes());
    }
    wire.extend_from_slice(&0_u16.to_le_bytes());
    wire.extend_from_slice(&0_u32.to_le_bytes()); // sors
    let result: spot::rest_models::ExchangeInfoResponse = spot::sbe::decode_api(&wire).unwrap();
    assert_eq!(
        result.rate_limits.as_ref().unwrap()[0].rate_limit_type,
        Some(spot::enums::RateLimitType::RequestWeight)
    );
    assert_eq!(
        result.rate_limits.as_ref().unwrap()[0].interval,
        Some(spot::enums::RateLimitInterval::Minute)
    );
    let symbol = &result.symbols.as_ref().unwrap()[0];
    assert_eq!(symbol.status, Some(spot::enums::SymbolStatus::Trading));
    assert_eq!(
        symbol.order_types.as_ref().unwrap(),
        &[
            spot::enums::OrderType::Market,
            spot::enums::OrderType::Limit
        ]
    );
    assert_eq!(
        symbol.default_self_trade_prevention_mode,
        Some(spot::enums::SelfTradePreventionMode::None)
    );
    assert!(
        matches!(&symbol.permission_sets.as_ref().unwrap()[0][1],spot::enums::Permission::Unknown(v) if v=="FUTURE_PERMISSION")
    );
    assert_eq!(
        serde_json::to_value(result).unwrap()["symbols"][0]["permissionSets"][0][1],
        "FUTURE_PERMISSION"
    );
}

/// Explicitly invoked, public production GETs only; no credentials or mutations.
/// Writes only dated public contract facts and safe transport evidence, never an
/// account payload. Lack of an observed spelling does not prove it impossible.
#[tokio::test]
#[ignore = "explicit operator invocation required: public production exchangeInfo GETs"]
async fn record_public_exchange_information_enum_evidence() -> Result<(), Box<dyn std::error::Error>>
{
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    let run_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let um = usdm::RestClient::new(usdm::Config::new(usdm::Environment::Production)?)?;
    let cm = coinm::RestClient::new(coinm::Config::new(coinm::Environment::Production)?)?;
    let um_result = um
        .exchange_information(
            &usdm::rest_requests::ExchangeInformation::new().build()?,
            tokio::time::Instant::now() + Duration::from_secs(10),
        )
        .await;
    let cm_result = cm
        .exchange_information(
            &coinm::rest_requests::ExchangeInformation::new().build()?,
            tokio::time::Instant::now() + Duration::from_secs(10),
        )
        .await;
    let um_evidence = match um_result {
        Ok(response) => {
            json!({"httpStatus":response.meta.status,"symbols":response.data.symbols.map(|symbols|symbols.into_iter().map(|symbol|json!({"symbol":symbol.symbol,"contractType":symbol.contract_type,"status":symbol.status})).collect::<Vec<_>>())})
        }
        Err(error) => json!({"safeFailure":error.to_string()}),
    };
    let cm_evidence = match cm_result {
        Ok(response) => {
            json!({"httpStatus":response.meta.status,"symbols":response.data.symbols.map(|symbols|symbols.into_iter().map(|symbol|json!({"symbol":symbol.symbol,"contractType":symbol.contract_type,"status":symbol.contract_status})).collect::<Vec<_>>())})
        }
        Err(error) => json!({"safeFailure":error.to_string()}),
    };
    let evidence = json!({"run":"nonrate-contract-issues-exchange-info-2026-10-11","observedAtUnixSeconds":run_time,"credentialed":false,"usdm":{"endpoint":"https://fapi.binance.com/fapi/v1/exchangeInfo","evidence":um_evidence},"coinm":{"endpoint":"https://dapi.binance.com/dapi/v1/exchangeInfo","evidence":cm_evidence}});
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/public-exchange-info-2026-10-11.json");
    std::fs::create_dir_all(path.parent().ok_or("fixture path has no parent")?)?;
    std::fs::write(&path, serde_json::to_vec_pretty(&evidence)?)?;
    println!("Public enum evidence written to {}", path.display());
    Ok(())
}

#[test]
fn dated_public_exchange_information_facts_decode_in_their_own_market() {
    let evidence: serde_json::Value = serde_json::from_str(include_str!(
        "fixtures/public-exchange-info-2026-10-11.json"
    ))
    .unwrap();
    assert_eq!(evidence["credentialed"], false);
    for market in ["usdm", "coinm"] {
        assert_eq!(evidence[market]["evidence"]["httpStatus"], 200);
        let symbols = evidence[market]["evidence"]["symbols"].as_array().unwrap();
        assert!(!symbols.is_empty());
        for symbol in symbols {
            binance_client::Symbol::new(symbol["symbol"].as_str().unwrap()).unwrap();
            let contract = symbol["contractType"].clone();
            let status = symbol["status"].clone();
            if market == "usdm" {
                let decoded: usdm::enums::ContractType =
                    serde_json::from_value(contract.clone()).unwrap();
                assert_eq!(serde_json::to_value(decoded).unwrap(), contract);
                let decoded: usdm::enums::ContractStatus =
                    serde_json::from_value(status.clone()).unwrap();
                assert_eq!(serde_json::to_value(decoded).unwrap(), status);
            } else {
                let decoded: coinm::enums::ContractType =
                    serde_json::from_value(contract.clone()).unwrap();
                assert_eq!(serde_json::to_value(decoded).unwrap(), contract);
                let decoded: coinm::enums::ContractStatus =
                    serde_json::from_value(status.clone()).unwrap();
                assert_eq!(serde_json::to_value(decoded).unwrap(), status);
            }
        }
    }
}

#[test]
fn wallet_universal_transfer_directions_keep_native_spelling() {
    strings!(
        wallet::enums::UniversalTransferType,
        [
            "MAIN_UMFUTURE",
            "MAIN_CMFUTURE",
            "MAIN_MARGIN",
            "UMFUTURE_MAIN",
            "UMFUTURE_MARGIN",
            "CMFUTURE_MAIN",
            "CMFUTURE_MARGIN",
            "MARGIN_MAIN",
            "MARGIN_UMFUTURE",
            "MARGIN_CMFUTURE",
            "ISOLATEDMARGIN_MARGIN",
            "MARGIN_ISOLATEDMARGIN",
            "ISOLATEDMARGIN_ISOLATEDMARGIN",
            "MAIN_FUNDING",
            "FUNDING_MAIN",
            "FUNDING_UMFUTURE",
            "UMFUTURE_FUNDING",
            "MARGIN_FUNDING",
            "FUNDING_MARGIN",
            "FUNDING_CMFUTURE",
            "CMFUTURE_FUNDING",
            "MAIN_OPTION",
            "OPTION_MAIN",
            "UMFUTURE_OPTION",
            "OPTION_UMFUTURE",
            "MARGIN_OPTION",
            "OPTION_MARGIN",
            "FUNDING_OPTION",
            "OPTION_FUNDING",
            "MAIN_PORTFOLIO_MARGIN",
            "PORTFOLIO_MARGIN_MAIN"
        ]
    );
}

#[test]
fn localentity_travel_status_keeps_its_distinct_native_codes() {
    integers!(wallet::enums::TravelRuleStatus, [0, 1, 2]);
}

#[test]
fn trade_lite_side_and_margin_call_position_side_have_native_types() {
    let wire = json!({"e":"TRADE_LITE","E":1_721_895_408_092_i64,"T":1_721_895_408_214_i64,"s":"BTCUSDT","q":"0.001","p":"0","m":false,"c":"native-order","S":"BUY","L":"64089.20","l":"0.040","t":109_100_866,"i":8_886_774});
    let event: usdm::stream_models::TradeLiteEvent = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(event.upper_s, usdm::enums::OrderSide::Buy);
    assert_eq!(serde_json::to_value(event).unwrap()["S"], "BUY");
    let mut future = wire;
    future["S"] = json!("FUTURE_SIDE");
    let event: usdm::stream_models::TradeLiteEvent = serde_json::from_value(future).unwrap();
    assert!(
        matches!(event.upper_s,usdm::enums::OrderSide::Unknown(ref spelling) if spelling=="FUTURE_SIDE")
    );
    assert_eq!(serde_json::to_value(event).unwrap()["S"], "FUTURE_SIDE");
    let event: usdm::stream_models::MarginCallEvent = serde_json::from_value(
        json!({"e":"MARGIN_CALL","E":1,"p":[{"s":"BTCUSDT","ps":"LONG","pa":"1","mt":"CROSSED","mp":"100","up":"0","mm":"1"}]}),
    )
    .unwrap();
    assert_eq!(event.p[0].ps, usdm::enums::PositionSide::Long);
    let event: coinm::stream_models::MarginCallEvent = serde_json::from_value(
        json!({"e":"MARGIN_CALL","E":1,"p":[{"s":"BTCUSD_PERP","ps":"LONG","pa":"1","mt":"CROSSED","mp":"100","up":"0","mm":"1"}]}),
    )
    .unwrap();
    assert_eq!(
        event.p.as_ref().unwrap()[0].ps,
        Some(coinm::enums::PositionSide::Long)
    );
}

#[test]
fn account_update_reasons_use_each_markets_documented_values() {
    strings!(
        usdm::enums::AccountUpdateReason,
        [
            "DEPOSIT",
            "WITHDRAW",
            "ORDER",
            "FUNDING_FEE",
            "WITHDRAW_REJECT",
            "ADJUSTMENT",
            "INSURANCE_CLEAR",
            "ADMIN_DEPOSIT",
            "ADMIN_WITHDRAW",
            "MARGIN_TRANSFER",
            "MARGIN_TYPE_CHANGE",
            "ASSET_TRANSFER",
            "OPTIONS_PREMIUM_FEE",
            "OPTIONS_SETTLE_PROFIT",
            "AUTO_EXCHANGE",
            "COIN_SWAP_DEPOSIT",
            "COIN_SWAP_WITHDRAW"
        ]
    );
    strings!(
        coinm::enums::AccountUpdateReason,
        [
            "DEPOSIT",
            "WITHDRAW",
            "ORDER",
            "FUNDING_FEE",
            "ADJUSTMENT",
            "INSURANCE_CLEAR",
            "ADMIN_DEPOSIT",
            "ADMIN_WITHDRAW",
            "MARGIN_TRANSFER",
            "MARGIN_TYPE_CHANGE",
            "ASSET_TRANSFER",
            "COIN_SWAP_DEPOSIT",
            "COIN_SWAP_WITHDRAW"
        ]
    );
    let wire = json!({"e":"ACCOUNT_UPDATE","E":1,"T":1,"a":{"m":"ORDER","B":[],"P":[]}});
    let um: usdm::stream_models::AccountUpdateEvent = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(um.a.m, usdm::enums::AccountUpdateReason::Order);
    let cm: coinm::stream_models::AccountUpdateEvent =
        serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(cm.a.m, Some(coinm::enums::AccountUpdateReason::Order));
    let mut future = wire;
    future["a"]["m"] = json!("OPTIONS_PREMIUM_FEE");
    let cm: coinm::stream_models::AccountUpdateEvent =
        serde_json::from_value(future.clone()).unwrap();
    assert!(
        matches!(&cm.a.m,Some(coinm::enums::AccountUpdateReason::Unknown(v)) if v=="OPTIONS_PREMIUM_FEE")
    );
    assert_eq!(
        serde_json::to_value(cm).unwrap()["a"]["m"],
        future["a"]["m"]
    );
    future["a"]["m"] = json!("FUTURE_REASON");
    let um: usdm::stream_models::AccountUpdateEvent = serde_json::from_value(future).unwrap();
    assert!(matches!(&um.a.m,usdm::enums::AccountUpdateReason::Unknown(v) if v=="FUTURE_REASON"));
    assert_eq!(serde_json::to_value(um).unwrap()["a"]["m"], "FUTURE_REASON");
}

#[test]
fn response_rate_enums_keep_product_specific_known_and_unknown_values() {
    strings!(usdm::enums::RateLimitType, ["REQUEST_WEIGHT", "ORDERS"]);
    strings!(coinm::enums::RateLimitType, ["REQUEST_WEIGHT", "ORDERS"]);
    strings!(usdm::enums::RateLimitInterval, ["SECOND", "MINUTE"]);
    strings!(coinm::enums::RateLimitInterval, ["MINUTE"]);
    strings!(
        spot::enums::RateLimitType,
        ["REQUEST_WEIGHT", "ORDERS", "RAW_REQUESTS", "CONNECTIONS"]
    );
    strings!(
        spot::enums::RateLimitInterval,
        ["SECOND", "MINUTE", "HOUR", "DAY"]
    );
    let wire =
        json!({"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":2400});
    let um: usdm::rest_models::ExchangeInformationResponseRateLimitsItem =
        serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(
        um.rate_limit_type,
        Some(usdm::enums::RateLimitType::RequestWeight)
    );
    assert_eq!(um.interval, Some(usdm::enums::RateLimitInterval::Minute));
    let cm: coinm::rest_models::ExchangeInformationResponseRateLimitsItem =
        serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(cm.interval, Some(coinm::enums::RateLimitInterval::Minute));
    let spot_rest: spot::rest_models::ExchangeInfoResponseRateLimitsItem =
        serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(
        spot_rest.interval,
        Some(spot::enums::RateLimitInterval::Minute)
    );
    let spot_ws: spot::ws_models::ExchangeInfoResponseRateLimitsItem =
        serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(
        spot_ws.rate_limit_type,
        Some(spot::enums::RateLimitType::RequestWeight)
    );
    let unknown = json!({"rateLimitType":"FUTURE_BUDGET","interval":"FUTURE_INTERVAL","intervalNum":1,"limit":2400});
    let um: usdm::rest_models::ExchangeInformationResponseRateLimitsItem =
        serde_json::from_value(unknown.clone()).unwrap();
    assert!(
        matches!(&um.interval,Some(usdm::enums::RateLimitInterval::Unknown(v)) if v=="FUTURE_INTERVAL")
    );
    assert_eq!(serde_json::to_value(um).unwrap(), unknown);
}
