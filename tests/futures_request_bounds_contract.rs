// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Futures request ranges follow what each market's own pages document.
#![allow(clippy::unwrap_used, reason = "synthetic request assertions")]

use binance_client::{ClientOrderId, Decimal, Symbol, coinm, usdm};

fn symbol() -> Symbol {
    Symbol::new("BTCUSDT").unwrap()
}
fn id() -> ClientOrderId {
    ClientOrderId::new("bounds-caller").unwrap()
}
fn rate(text: &str) -> Decimal {
    Decimal::from_str_exact(text).unwrap()
}

#[test]
fn usdm_algo_trailing_stop_trails_by_a_tenth_to_twenty_percent() {
    let rest = |callback: &str| {
        usdm::rest_requests::NewAlgoOrder::new()
            .symbol(symbol())
            .side("SELL")
            .algo_type("CONDITIONAL")
            .type_value("TRAILING_STOP_MARKET")
            .quantity(Decimal::ONE)
            .client_algo_id(id())
            .callback_rate(rate(callback))
            .build()
    };
    let socket = |callback: &str| {
        usdm::ws_requests::NewAlgoOrder::new()
            .symbol(symbol())
            .side("SELL")
            .algo_type("CONDITIONAL")
            .type_value("TRAILING_STOP_MARKET")
            .quantity(Decimal::ONE)
            .client_algo_id(id())
            .callback_rate(rate(callback))
            .build()
    };
    for accepted in ["0.1", "10.1", "20"] {
        assert!(rest(accepted).is_ok(), "{accepted}");
        assert!(socket(accepted).is_ok(), "{accepted}");
    }
    for refused in ["0.09", "20.1"] {
        assert!(rest(refused).is_err(), "{refused}");
        assert!(socket(refused).is_err(), "{refused}");
    }
}

#[test]
fn other_trailing_orders_keep_their_own_documented_ceilings() {
    let validation_only = |callback: &str| {
        usdm::rest_requests::TestOrder::new()
            .symbol(symbol())
            .side("SELL")
            .type_value("TRAILING_STOP_MARKET")
            .quantity(Decimal::ONE)
            .new_client_order_id(id())
            .callback_rate(rate(callback))
            .build()
    };
    assert!(validation_only("5").is_ok());
    assert!(validation_only("5.1").is_err());
    let inverse = |callback: &str| {
        coinm::rest_requests::NewAlgoOrder::new()
            .symbol(Symbol::new("BTCUSD_PERP").unwrap())
            .side("SELL")
            .algo_type("CONDITIONAL")
            .type_value("TRAILING_STOP_MARKET")
            .quantity(Decimal::ONE)
            .client_algo_id(id())
            .callback_rate(rate(callback))
            .build()
    };
    assert!(inverse("10").is_ok());
    assert!(inverse("10.1").is_err());
}

#[test]
fn usdm_leverage_has_a_floor_and_no_fixed_ceiling() {
    let leverage = |value: i64| {
        usdm::rest_requests::ChangeInitialLeverage::new()
            .symbol(symbol())
            .leverage(value)
            .build()
    };
    assert!(leverage(1).is_ok());
    assert!(leverage(150).is_ok());
    assert!(leverage(0).is_err());
}

#[test]
fn usdm_continuous_klines_accept_the_one_second_interval() {
    let klines = |interval: &str| {
        usdm::rest_requests::ContinuousContractKlineCandlestickData::new()
            .pair(symbol())
            .contract_type("PERPETUAL")
            .interval(interval)
            .build()
    };
    assert!(klines("1s").is_ok());
    assert!(klines("1m").is_ok());
    assert!(klines("2s").is_err());
}
