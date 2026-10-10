// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Caller prices retain their signs; venue filters own admission of price values.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic protocol assertions"
)]
use binance_client::{ClientOrderId, Decimal, Symbol, coinm, margin, options, spot, usdm};
use serde::Serialize;

fn symbol() -> Symbol {
    Symbol::new("BTCUSDT").unwrap()
}
fn id() -> ClientOrderId {
    ClientOrderId::new("price-caller").unwrap()
}
fn exact<T: Serialize>(request: &T, name: &str, value: Decimal) {
    assert_eq!(
        serde_json::to_value(request).unwrap()[name],
        value.to_string()
    );
}
fn prices() -> [Decimal; 2] {
    [Decimal::ZERO, Decimal::new(-125, 2)]
}

#[test]
fn native_limit_orders_preserve_zero_and_negative_prices() {
    for price in prices() {
        let request = spot::rest_requests::NewOrder::new()
            .symbol(symbol())
            .side("BUY")
            .type_value("LIMIT")
            .time_in_force("GTC")
            .quantity(Decimal::ONE)
            .new_client_order_id(spot::ClientOrderId::new("price-caller").unwrap())
            .price(price);
        let request = request.build().unwrap();
        exact(&request, "price", price);
        assert!(request.clone().quantity(Decimal::ZERO).build().is_err());
        let request = usdm::rest_requests::NewOrder::new()
            .symbol(symbol())
            .side("BUY")
            .type_value("LIMIT")
            .time_in_force("GTC")
            .quantity(Decimal::ONE)
            .new_client_order_id(id())
            .price(price)
            .build()
            .unwrap();
        exact(&request, "price", price);
        assert!(request.quantity(-Decimal::ONE).build().is_err());
        let request = coinm::rest_requests::NewOrder::new()
            .symbol(Symbol::new("BTCUSD_PERP").unwrap())
            .side("BUY")
            .type_value("LIMIT")
            .time_in_force("GTC")
            .quantity(Decimal::ONE)
            .new_client_order_id(id())
            .price(price)
            .build()
            .unwrap();
        exact(&request, "price", price);
        assert!(request.quantity(Decimal::ZERO).build().is_err());
        let request = margin::rest_requests::MarginAccountNewOrder::new()
            .symbol(symbol())
            .side("BUY")
            .type_value("LIMIT")
            .time_in_force("GTC")
            .quantity(Decimal::ONE)
            .new_client_order_id(margin::ClientOrderId::new("price-caller").unwrap())
            .price(price)
            .build()
            .unwrap();
        exact(&request, "price", price);
        assert!(request.quantity(Decimal::ZERO).build().is_err());
        let request = options::rest_requests::NewOrder::new()
            .symbol(options::Symbol::new("BTC-261030-90000-C").unwrap())
            .side("BUY")
            .type_value("LIMIT")
            .quantity(Decimal::ONE)
            .client_order_id(options::ClientOrderId::new("price-caller").unwrap())
            .price(price)
            .build()
            .unwrap();
        exact(&request, "price", price);
        assert!(request.quantity(-Decimal::ONE).build().is_err());
    }
}

#[test]
fn futures_trigger_stop_and_activation_prices_have_no_unstated_floor() {
    for price in prices() {
        let base = usdm::rest_requests::NewAlgoOrder::new()
            .symbol(symbol())
            .side("BUY")
            .algo_type("CONDITIONAL")
            .quantity(Decimal::ONE)
            .client_algo_id(id());
        let stop = base
            .clone()
            .type_value("STOP")
            .trigger_price(price)
            .price(price)
            .time_in_force("GTC")
            .build()
            .unwrap();
        exact(&stop, "triggerPrice", price);
        exact(&stop, "price", price);
        let trailing = base
            .type_value("TRAILING_STOP_MARKET")
            .callback_rate(Decimal::ONE)
            .activate_price(price)
            .build()
            .unwrap();
        exact(&trailing, "activatePrice", price);
        let legacy = usdm::rest_requests::TestOrder::new()
            .symbol(symbol())
            .side("BUY")
            .type_value("TRAILING_STOP_MARKET")
            .quantity(Decimal::ONE)
            .new_client_order_id(id())
            .callback_rate(Decimal::ONE)
            .activation_price(price)
            .build()
            .unwrap();
        exact(&legacy, "activationPrice", price);
        let stop = usdm::rest_requests::TestOrder::new()
            .symbol(symbol())
            .side("BUY")
            .type_value("STOP_MARKET")
            .quantity(Decimal::ONE)
            .new_client_order_id(id())
            .stop_price(price)
            .build()
            .unwrap();
        exact(&stop, "stopPrice", price);
    }
}

#[test]
fn spot_list_leg_prices_and_stops_preserve_exact_signs() {
    for price in prices() {
        let request = spot::rest_requests::OrderListOco::new()
            .symbol(symbol())
            .side("SELL")
            .quantity(Decimal::ONE)
            .above_type("LIMIT_MAKER")
            .above_price(price)
            .below_type("STOP_LOSS_LIMIT")
            .below_price(price)
            .below_stop_price(price)
            .below_time_in_force("GTC")
            .list_client_order_id(spot::ClientOrderId::new("list").unwrap())
            .above_client_order_id(spot::ClientOrderId::new("above").unwrap())
            .below_client_order_id(spot::ClientOrderId::new("below").unwrap())
            .build()
            .unwrap();
        for name in ["abovePrice", "belowPrice", "belowStopPrice"] {
            exact(&request, name, price);
        }
        assert!(request.quantity(Decimal::ZERO).build().is_err());
    }
}

#[test]
fn margin_working_pending_and_stop_limit_prices_preserve_exact_signs() {
    for price in prices() {
        let request = margin::rest_requests::MarginAccountNewOtoco::new()
            .symbol(symbol())
            .working_type("LIMIT")
            .working_side("BUY")
            .working_price(price)
            .working_quantity(Decimal::ONE)
            .working_time_in_force("GTC")
            .pending_side("SELL")
            .pending_quantity(Decimal::ONE)
            .pending_above_type("LIMIT_MAKER")
            .pending_above_price(price)
            .pending_below_type("STOP_LOSS_LIMIT")
            .pending_below_price(price)
            .pending_below_stop_price(price)
            .pending_below_time_in_force("GTC")
            .list_client_order_id(margin::ClientOrderId::new("list").unwrap())
            .working_client_order_id(margin::ClientOrderId::new("working").unwrap())
            .pending_above_client_order_id(margin::ClientOrderId::new("above").unwrap())
            .pending_below_client_order_id(margin::ClientOrderId::new("below").unwrap())
            .build()
            .unwrap();
        for name in [
            "workingPrice",
            "pendingAbovePrice",
            "pendingBelowPrice",
            "pendingBelowStopPrice",
        ] {
            exact(&request, name, price);
        }
        assert!(request.pending_quantity(Decimal::ZERO).build().is_err());
        let request = margin::rest_requests::MarginAccountNewOco::new()
            .symbol(symbol())
            .side("SELL")
            .quantity(Decimal::ONE)
            .price(price)
            .stop_price(price)
            .stop_limit_price(price)
            .stop_limit_time_in_force("GTC")
            .list_client_order_id(margin::ClientOrderId::new("list").unwrap())
            .limit_client_order_id(margin::ClientOrderId::new("limit").unwrap())
            .stop_client_order_id(margin::ClientOrderId::new("stop").unwrap())
            .build()
            .unwrap();
        for name in ["price", "stopPrice", "stopLimitPrice"] {
            exact(&request, name, price);
        }
    }
}

#[test]
fn fix_prices_roundtrip_without_sign_policy_and_quantity_stays_positive() {
    use spot::fix::{
        ClientId, CompId, Fields, Header, Precision, Request, RequestKind, Role, Timestamp, Value,
        decode, decode_sbe,
    };
    let header = Header::request(
        CompId::new("CLIENT").unwrap(),
        2,
        Timestamp::from_micros(1_790_471_000_123_456).unwrap(),
    )
    .unwrap();
    for price in prices() {
        let base = Request::builder(Role::OrderEntry, RequestKind::NewOrderSingle)
            .field(
                "ClOrdID",
                Value::ClientId(ClientId::new("price-caller").unwrap()),
            )
            .unwrap()
            .field("Symbol", Value::Symbol(symbol()))
            .unwrap()
            .field("Side", Value::Code("1".into()))
            .unwrap()
            .field("OrdType", Value::Code("2".into()))
            .unwrap()
            .field("TimeInForce", Value::Code("1".into()))
            .unwrap()
            .field("Price", Value::Price(price))
            .unwrap();
        assert!(
            base.clone()
                .field("OrderQty", Value::Quantity(Decimal::ZERO))
                .unwrap()
                .build()
                .is_err()
        );
        let request = base
            .field("OrderQty", Value::Quantity(Decimal::ONE))
            .unwrap()
            .build()
            .unwrap();
        let wire = request.encode(&header).unwrap();
        assert_eq!(
            decode(Role::OrderEntry, &wire).unwrap().field("Price"),
            Some(&Value::Price(price))
        );
        let instrument = Fields::new(Role::MarketData)
            .with("Symbol", Value::Symbol(symbol()))
            .unwrap()
            .with("MinPriceIncrement", Value::Price(Decimal::new(1, 2)))
            .unwrap()
            .with("MinQtyIncrement", Value::Quantity(Decimal::new(1, 3)))
            .unwrap();
        let precision = Precision::from_instrument(&instrument).unwrap();
        let binary = request.encode_sbe(&header, &[precision]).unwrap();
        assert_eq!(
            decode_sbe(Role::OrderEntry, &binary)
                .unwrap()
                .field("Price"),
            Some(&Value::Price(price))
        );
    }
}

#[test]
fn fix_trigger_prices_preserve_sign_and_conditional_fields_remain_required() {
    use spot::fix::{ClientId, Request, RequestKind, Role, Value};
    for price in prices() {
        let base = Request::builder(Role::OrderEntry, RequestKind::NewOrderSingle)
            .field(
                "ClOrdID",
                Value::ClientId(ClientId::new("trigger-caller").unwrap()),
            )
            .unwrap()
            .field("Symbol", Value::Symbol(symbol()))
            .unwrap()
            .field("Side", Value::Code("1".into()))
            .unwrap()
            .field("OrdType", Value::Code("3".into()))
            .unwrap()
            .field("OrderQty", Value::Quantity(Decimal::ONE))
            .unwrap()
            .field("TriggerPrice", Value::Price(price))
            .unwrap()
            .field("TriggerType", Value::Code("4".into()))
            .unwrap()
            .field("TriggerAction", Value::Code("1".into()))
            .unwrap()
            .field("TriggerPriceType", Value::Code("2".into()))
            .unwrap();
        assert!(base.clone().build().is_err());
        let request = base
            .field("TriggerPriceDirection", Value::Code("U".into()))
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(
            request.fields().get("TriggerPrice"),
            Some(&Value::Price(price))
        );
    }
}

#[allow(dead_code, reason = "shared synthetic HTTP fixture helpers")]
mod support;

#[tokio::test]
async fn nonpositive_price_reaches_venue_once_and_keeps_exact_caller_evidence() {
    use binance_client::{Credentials, WeightPools};
    use std::sync::Arc;
    for price in prices() {
        let mut fixture = support::HttpFixture::new(
            200,
            "",
            r#"{"orderId":123,"clientOrderId":"price-caller"}"#,
            None,
            false,
        )
        .await;
        let config = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
            .unwrap()
            .clock(Arc::new(support::FixedClock(1000)))
            .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
            .rest_url(&fixture.url)
            .unwrap();
        let client = spot::RestClient::new(config).unwrap();
        let request = spot::rest_requests::NewOrder::new()
            .symbol(symbol())
            .side("BUY")
            .type_value("LIMIT")
            .time_in_force("GTC")
            .quantity(Decimal::ONE)
            .new_client_order_id(spot::ClientOrderId::new("price-caller").unwrap())
            .price(price)
            .build()
            .unwrap();
        let response = client
            .new_order(&request, support::deadline())
            .await
            .unwrap();
        assert_eq!(response.meta.status, 200);
        assert_eq!(response.data.order_id, 123);
        assert_eq!(
            response.meta.client_order_ids["newClientOrderId"],
            "price-caller"
        );
        let wire = fixture.requests.recv().await.unwrap();
        assert!(wire.contains(&format!("price={price}")));
        assert!(wire.contains("newClientOrderId=price-caller"));
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}
