// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Options native identity, financial data, and single-attempt execution contracts.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic contract assertions"
)]
use binance_client::{Decimal, options};
use serde_json::json;

#[test]
fn option_order_requires_caller_identity_and_limit_price() {
    let order = options::rest_requests::NewOrder::new()
        .symbol(options::Symbol::new("BTC-261030-90000-C").unwrap())
        .side("BUY")
        .type_value("LIMIT")
        .quantity(Decimal::ONE);
    assert!(order.clone().price(Decimal::ONE).build().is_err());
    assert!(
        order
            .clone()
            .client_order_id(options::ClientOrderId::new("caller-1").unwrap())
            .build()
            .is_err()
    );
    assert!(
        order
            .price(Decimal::ONE)
            .client_order_id(options::ClientOrderId::new("caller-1").unwrap())
            .build()
            .is_ok()
    );
}

#[test]
fn option_financial_strings_are_exact_and_required() {
    let data = json!({"time": 1, "indexPrice": "0.0000000000000000000000000001"});
    let response: options::rest_models::IndexPriceResponse = serde_json::from_value(data).unwrap();
    assert_eq!(
        response.index_price,
        Decimal::from_str_exact("0.0000000000000000000000000001").unwrap()
    );
    assert!(
        serde_json::from_value::<options::rest_models::IndexPriceResponse>(
            json!({"time": 1, "indexPrice":"1e-29"})
        )
        .is_err()
    );
    assert!(
        serde_json::from_value::<options::rest_models::IndexPriceResponse>(json!({"time": 1}))
            .is_err()
    );
}

#[test]
fn option_streams_retain_contract_and_route() {
    let symbol = options::Symbol::new("BTC-261030-90000-C").unwrap();
    let stream = options::Stream::trade_streams(&symbol).unwrap();
    assert_eq!(stream.name(), "btc-261030-90000-c@optionTrade");
    assert_eq!(stream.route(), options::Route::Public);
    assert!(options::Stream::diff_book_depth_streams(&symbol, "200ms").is_err());
}

#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;
use binance_client::{BudgetLimits, Budgets, Credentials, Error, Outcome};
fn config() -> options::Config {
    options::Config::new(options::Environment::Demo)
        .unwrap()
        .budgets(Budgets::new(BudgetLimits::options()).unwrap())
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap())
}
fn order() -> options::rest_requests::NewOrder {
    options::rest_requests::NewOrder::new()
        .symbol(options::Symbol::new("BTC-261030-90000-C").unwrap())
        .side("BUY")
        .type_value("LIMIT")
        .quantity(Decimal::ONE)
        .price(Decimal::ONE)
        .client_order_id(options::ClientOrderId::new("caller-1").unwrap())
        .build()
        .unwrap()
}

#[tokio::test]
async fn option_mutation_lost_body_is_unknown_sent_once_and_keeps_caller_id() {
    let fixture =
        support::HttpFixture::new(200, "X-MBX-USED-WEIGHT-1M: 13\r\n", "{", Some(90), false).await;
    let client = options::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let error = client
        .new_order(&order(), support::deadline())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown));
    match error {
        Error::Transport {
            client_order_ids,
            meta: Some(meta),
            ..
        } => {
            assert_eq!(client_order_ids["clientOrderId"], "caller-1");
            assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 13);
        }
        error => panic!("unexpected error {error:?}"),
    }
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn option_unknown_codes_stay_unknown_and_documented_refusals_are_rejected() {
    for (code, outcome) in [
        (-1121, Outcome::Rejected),
        (-4078, Outcome::Unknown),
        (-99_999, Outcome::Unknown),
    ] {
        let fixture = support::HttpFixture::new(
            400,
            "",
            &json!({"code":code,"msg":"synthetic"}).to_string(),
            None,
            false,
        )
        .await;
        let client = options::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
        let error = client
            .new_order(&order(), support::deadline())
            .await
            .unwrap_err();
        assert_eq!(error.outcome(), Some(outcome));
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}

#[test]
fn options_batch_preserves_success_refusal_and_malformed_member() {
    let data: options::rest_models::PlaceMultipleOrdersResponse = serde_json::from_value(json!([
        {"orderId":7,"symbol":"BTC-261030-90000-C","clientOrderId":"caller-1"},
        {"code":-2010,"msg":"synthetic"},
        {"orderId":9,"symbol":"BTC-261030-90000-C","clientOrderId":"caller-3","price":"1e-29"}
    ]))
    .unwrap();
    assert_eq!(data.len(), 3);
    assert!(matches!(&data[0],options::wire::BatchResult::Success(v) if v.order_id.value()==7));
    assert!(
        matches!(&data[1],options::wire::BatchResult::Failure(v) if v.outcome()==Outcome::Rejected)
    );
    assert!(matches!(&data[2], options::wire::BatchResult::Unknown(_)));
    let nested = options::rest_models::PlaceMultipleOrdersOrdersInputItem::new()
        .symbol(options::Symbol::new("BTC-261030-90000-C").unwrap())
        .side("BUY")
        .type_value("LIMIT")
        .quantity(Decimal::ONE)
        .price(Decimal::ONE)
        .client_order_id(options::ClientOrderId::new("same-id").unwrap())
        .build()
        .unwrap();
    assert!(
        options::rest_requests::PlaceMultipleOrders::new()
            .orders(vec![nested.clone(), nested])
            .build()
            .is_err()
    );
}

#[test]
fn options_partial_balance_event_retains_native_assets_without_replacing_account() {
    let payload = options::event_payloads::user_payload(json!({"stream":"synthetic","data":{
        "e":"BALANCE_POSITION_UPDATE","E":1,"T":1,"m":"ORDER",
        "B":[{"a":"USDT","b":"2","bc":"0.0000000000000000000000000001"}],
        "P":[{"s":"BTC-261030-90000-C","c":"-1","p":"-2","a":"2"}]
    }}))
    .unwrap();
    match payload {
        options::event_payloads::UserPayload::BalancePositionUpdate(v) => {
            assert_eq!(v.upper_b.len(), 1);
            assert_eq!(v.upper_b[0].a.as_str(), "USDT");
            assert_eq!(v.upper_p[0].c, Decimal::NEGATIVE_ONE);
            assert_eq!(v.upper_p[0].s.as_str(), "BTC-261030-90000-C");
        }
        _ => panic!("wrong event kind"),
    }
    assert!(options::event_payloads::user_payload(json!({"e":"BALANCE_POSITION_UPDATE","E":1,"B":[{"a":"USDT","b":"oops","bc":"0"}],"P":[]})).is_err());
}

#[test]
fn options_budget_authority_is_native_and_unsupported_intervals_are_refused() {
    let info = |interval| {
        serde_json::from_value::<options::rest_models::ExchangeInformationResponse>(json!({
        "serverTime":1,"optionContracts":[],"optionAssets":[],"optionSymbols":[],
        "rateLimits":[{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":73},
        {"rateLimitType":"ORDERS","interval":interval,"intervalNum":1,"limit":17}]
    })).unwrap()
    };
    let limits = options::budget_limits(&info("MINUTE")).unwrap();
    assert_eq!(limits.weight_per_minute, 73);
    assert_eq!(limits.orders_per_minute, 17);
    assert!(options::budget_limits(&info("HOUR")).is_err());
}

#[test]
fn options_depth_events_retain_native_ids_and_signed_exact_levels() {
    let value = json!({"e":"depthUpdate","E":1,"T":2,"s":"BTC-261030-90000-C",
        "U":10,"u":11,"pu":9,"b":[["-1.25000001","0"]],"a":[["0","-2.75"]]});
    let event: options::stream_models::DiffBookDepthStreamsEvent =
        serde_json::from_value(value.clone()).unwrap();
    assert_eq!((event.upper_u, event.u, event.pu), (10, 11, 9));
    assert_eq!(event.b[0].price, Decimal::new(-125_000_001, 8));
    assert_eq!(event.b[0].quantity, Decimal::ZERO);
    assert_eq!(event.a[0].price, Decimal::ZERO);
    assert_eq!(event.a[0].quantity, Decimal::new(-275, 2));
    let mut missing = value;
    missing.as_object_mut().unwrap().remove("pu");
    assert!(
        serde_json::from_value::<options::stream_models::DiffBookDepthStreamsEvent>(missing)
            .is_err()
    );
}

#[test]
fn options_depth_snapshots_retain_finite_venue_evidence_and_refuse_bad_decimals() {
    let value = json!({"lastUpdateId":12,"T":2,"bids":[["-1.25","2.75000001"]],"asks":[]});
    let snapshot: options::rest_models::OrderBookResponse =
        serde_json::from_value(value.clone()).unwrap();
    assert_eq!(snapshot.last_update_id, 12);
    assert_eq!(snapshot.bids[0].price, Decimal::new(-125, 2));
    assert_eq!(snapshot.bids[0].quantity, Decimal::new(275_000_001, 8));
    assert!(snapshot.asks.is_empty());
    let mut missing = value.clone();
    missing.as_object_mut().unwrap().remove("lastUpdateId");
    assert!(serde_json::from_value::<options::rest_models::OrderBookResponse>(missing).is_err());
    let mut malformed = value;
    malformed["bids"] = json!([["0.00000000000000000000000000001", "1"]]);
    assert!(serde_json::from_value::<options::rest_models::OrderBookResponse>(malformed).is_err());
    assert!(serde_json::from_value::<options::wire::PriceLevel>(json!(["1", "2", "3"])).is_err());
}

#[tokio::test]
async fn options_socket_retains_prefix_then_malformed_gap_and_joins_retirement() {
    use futures_util::{SinkExt, StreamExt};
    use tokio_websockets::{Limits, Message, ServerBuilder};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let (request, mut peer) = ServerBuilder::new()
            .limits(Limits::unlimited())
            .accept(tcp)
            .await
            .unwrap();
        assert!(request.uri().path().starts_with("/private/ws/"));
        peer.send(Message::ping(b"synthetic-ping".as_slice()))
            .await
            .unwrap();
        for event in [
            json!({"stream":"synthetic","data":{"e":"BALANCE_POSITION_UPDATE","E":1,"T":1,"m":"ORDER","B":[{"a":"USDT","b":"2","bc":"0"}],"P":[]}}),
            json!({"stream":"synthetic","data":{"e":"futureOptionsEvent","E":2,"acceptedEvidence":"retained"}}),
            json!({"stream":"synthetic","data":{"e":"BALANCE_POSITION_UPDATE","E":3,"B":[{"a":"USDT","b":"oops","bc":"0"}],"P":[]}}),
        ] {
            peer.send(Message::text(event.to_string())).await.unwrap();
        }
        while let Some(message) = peer.next().await {
            if message.unwrap().is_close() {
                peer.flush().await.unwrap();
                break;
            }
        }
    });
    let (mut streams, driver) = options::Streams::user_data(
        config().streams_url(&url).unwrap(),
        &binance_client::SensitiveString::new("synthetic-key"),
    )
    .await
    .unwrap();
    let driver = tokio::spawn(driver.run());
    let generation = streams.generation();
    assert!(
        matches!(streams.recv().await,Some(options::StreamEvent::Established(g)) if g==generation)
    );
    assert!(
        matches!(streams.recv().await,Some(options::StreamEvent::Data { generation:g,payload:options::streams::StreamPayload::User(options::event_payloads::UserPayload::BalancePositionUpdate(_)) }) if g==generation)
    );
    assert!(matches!(
        streams.recv().await,
        Some(options::StreamEvent::Data {
            payload: options::streams::StreamPayload::User(
                options::event_payloads::UserPayload::Unknown(_)
            ),
            ..
        })
    ));
    assert!(
        matches!(streams.recv().await,Some(options::StreamEvent::Gap{generation:g,..}) if g==generation)
    );
    assert!(matches!(streams.recv().await,Some(options::StreamEvent::Retired(g)) if g==generation));
    assert!(streams.recv().await.is_none());
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn options_clients_share_admission_and_expired_orders_never_send() {
    let fixture = support::HttpFixture::new(200, "", r#"{"serverTime":1}"#, None, false).await;
    let budgets = Budgets::new(BudgetLimits::options().weight_per_minute(1)).unwrap();
    let configured = config().budgets(budgets).rest_url(&fixture.url).unwrap();
    let first = options::RestClient::new(configured.clone()).unwrap();
    let second = options::RestClient::new(configured).unwrap();
    first
        .check_server_time(
            &options::rest_requests::CheckServerTime::new(),
            support::deadline(),
        )
        .await
        .unwrap();
    assert!(matches!(
        second
            .check_server_time(
                &options::rest_requests::CheckServerTime::new(),
                support::deadline()
            )
            .await,
        Err(Error::Admission { .. })
    ));
    assert_eq!(fixture.connections_accepted(), 1);
    let error = second
        .new_order(&order(), tokio::time::Instant::now())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[test]
fn options_order_update_preserves_commission_asset_and_caller_identity() {
    let payload =
        options::event_payloads::user_payload(json!({"e":"ORDER_TRADE_UPDATE","E":1,"T":1,"o":{
            "s":"BTC-261030-90000-C","c":"caller-1","S":"SELL","q":"1","p":"2","ap":"2",
            "x":"TRADE","X":"FILLED","i":7,"l":"1","z":"1","L":"2","N":"BNB","n":"-0.01",
            "T":1,"b":"0","a":"0","rp":"0"
        }}))
        .unwrap();
    match payload {
        options::event_payloads::UserPayload::OrderTradeUpdate(v) => {
            assert_eq!(v.o.c, "caller-1");
            assert_eq!(v.o.upper_n.as_ref().unwrap().as_str(), "BNB");
            assert_eq!(v.o.n, Decimal::new(-1, 2));
        }
        _ => panic!("wrong event kind"),
    }
}

#[test]
fn option_block_leg_builder_refuses_missing_and_invalid_native_fields() {
    use options::rest_models::NewBlockTradeOrderLegsInputItem;
    assert!(NewBlockTradeOrderLegsInputItem::new().build().is_err());
    let leg = NewBlockTradeOrderLegsInputItem::new()
        .symbol(options::Symbol::new("BTC-261030-90000-C").unwrap())
        .quantity(Decimal::ONE)
        .price(Decimal::ONE);
    assert!(
        leg.clone()
            .side("LONG")
            .type_value("LIMIT")
            .build()
            .is_err()
    );
    assert!(
        leg.clone()
            .side("BUY")
            .type_value("MARKET")
            .build()
            .is_err()
    );
    assert!(leg.side("BUY").type_value("LIMIT").build().is_ok());
}

#[tokio::test]
async fn options_history_authority_is_rechecked_before_dispatch() {
    use std::sync::Arc;
    let fixture = support::HttpFixture::new(200, "", "[]", None, false).await;
    let configured = config()
        .clock(Arc::new(support::FixedClock(1_791_590_400_000)))
        .rest_url(&fixture.url)
        .unwrap();
    let client = options::RestClient::new(configured).unwrap();
    let request = options::rest_requests::AccountTradeList::new()
        .symbol(options::Symbol::new("BTC-261030-90000-C").unwrap())
        .start_time(1);
    let result = client
        .account_trade_list(&request, support::deadline())
        .await;
    assert!(matches!(result, Err(Error::Validation(_))));
    assert_eq!(fixture.connections_accepted(), 0);
    fixture.finish().await;
}

#[test]
fn options_stream_underlying_identity_is_distinct_and_case_normalized() {
    let underlying = binance_client::Symbol::new("BTCUSDT").unwrap();
    assert_eq!(
        options::Stream::option_mark_price(&underlying)
            .unwrap()
            .name(),
        "btcusdt@optionMarkPrice"
    );
    assert!(options::Stream::open_interest(&underlying, "").is_err());
}

#[test]
fn options_new_contract_units_are_financial_and_metadata_identity_is_required() {
    let value = json!({"e":"optionSymbol","E":1,"s":"BTC-261225-50000-C",
        "ps":"BTCUSDT","qa":"USDT","sp":"50000","u":1});
    let record: options::stream_models::NewSymbolInfoEvent =
        serde_json::from_value(value.clone()).unwrap();
    assert_eq!(record.u, Decimal::ONE);
    assert_eq!(record.qa.as_str(), "USDT");
    assert_eq!(record.ps.as_str(), "BTCUSDT");
    let mut absent = value.clone();
    absent.as_object_mut().unwrap().remove("qa");
    assert!(serde_json::from_value::<options::stream_models::NewSymbolInfoEvent>(absent).is_err());
    let mut malformed = value;
    malformed["u"] = json!("0.00000000000000000000000000001");
    assert!(
        serde_json::from_value::<options::stream_models::NewSymbolInfoEvent>(malformed).is_err()
    );
}

#[test]
fn options_native_order_ids_keep_numeric_and_string_wire_forms_without_float() {
    let numeric: options::OrderId = serde_json::from_value(json!(7)).unwrap();
    let text: options::OrderId = serde_json::from_value(json!("4611875134427365000")).unwrap();
    assert_eq!(numeric.value(), 7);
    assert_eq!(text.value(), 4_611_875_134_427_365_000);
    assert_eq!(serde_json::to_value(numeric).unwrap(), json!(7));
    assert!(
        options::rest_requests::CancelMultipleOptionOrders::new()
            .symbol(options::Symbol::new("BTC-261225-50000-C").unwrap())
            .order_ids(vec![text.clone()])
            .build()
            .is_ok()
    );
    assert_eq!(
        serde_json::to_value(text).unwrap(),
        json!("4611875134427365000")
    );
    for value in [0, -1, i64::MIN] {
        let native = options::OrderId::new(value);
        assert_eq!(native.value(), value);
        assert!(
            options::rest_requests::QuerySingleOrder::new()
                .symbol(options::Symbol::new("BTC-261225-50000-C").unwrap())
                .order_id(native)
                .build()
                .is_ok()
        );
        let text = value.to_string();
        let native = options::OrderId::from_text(text.clone()).unwrap();
        assert_eq!(native.value(), value);
        assert_eq!(serde_json::to_value(native).unwrap(), json!(text));
    }
    assert!(serde_json::from_value::<options::OrderId>(json!("9223372036854775808")).is_err());
    assert!(serde_json::from_value::<options::OrderId>(json!("1e3")).is_err());
    assert!(options::BlockOrderMatchingKey::new("").is_err());
}

#[test]
fn options_batch_cancel_requires_one_native_identity_list() {
    let request = options::rest_requests::CancelMultipleOptionOrders::new()
        .symbol(options::Symbol::new("BTC-261225-50000-C").unwrap());
    let numeric = options::OrderId::new(7);
    let caller = options::ClientOrderId::new("owned-order-7").unwrap();
    assert!(request.clone().build().is_err());
    assert!(
        request
            .clone()
            .order_ids(vec![numeric.clone()])
            .build()
            .is_ok()
    );
    assert!(
        request
            .clone()
            .client_order_ids(vec![caller.clone()])
            .build()
            .is_ok()
    );
    assert!(request.clone().order_ids(Vec::new()).build().is_err());
    assert!(
        request
            .clone()
            .client_order_ids(Vec::new())
            .build()
            .is_err()
    );
    assert!(
        request
            .order_ids(vec![numeric])
            .client_order_ids(vec![caller])
            .build()
            .is_err()
    );
}

#[test]
fn options_filters_dispatch_known_types_and_keep_unknown_future_semantics() {
    use options::rest_models::ExchangeInformationResponseOptionSymbolsItemFiltersItem as Filter;
    let filter: Filter = serde_json::from_value(
        json!({"filterType":"FUTURE_OPTION_FILTER","unmodelled":"preserved"}),
    )
    .unwrap();
    assert!(matches!(filter, Filter::Unknown(_)));
    assert!(
        serde_json::from_value::<Filter>(json!({"filterType":"PRICE_FILTER","minPrice":"invalid"}))
            .is_err()
    );
    assert!(serde_json::from_value::<Filter>(json!({"minPrice":"1","maxPrice":"2","tickSize":"1","minQty":"1","maxQty":"2","stepSize":"1"})).is_err());
}
