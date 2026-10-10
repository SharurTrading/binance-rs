// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Invariant sweep regressions: a branch that cannot happen never invents a business
//! value, and an accepted answer or instruction is never silently abandoned.
//!
//! Each test asserts the typed observable outcome and the preserved business state.
//! Fixtures are synthetic; no credential, network or venue access is involved.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    reason = "synthetic contract assertions"
)]
#[allow(
    dead_code,
    reason = "shared HTTP fixture helpers are used by the HTTP integration suite"
)]
mod support;
use binance_client::{
    Decimal, Outcome, RequestId, coinm::rest_models::PositionInformationResponse,
    spot::rest_models::MyFiltersResponse,
    spot::ws_models::MyFiltersResponse as SocketMyFiltersResponse,
};
use serde_json::json;

fn order() -> binance_client::usdm::ws_requests::NewOrder {
    binance_client::usdm::ws_requests::NewOrder::new()
        .symbol(binance_client::Symbol::new("BTCUSDT").unwrap())
        .side("BUY")
        .type_value("MARKET")
        .quantity(Decimal::ONE)
        .new_client_order_id(binance_client::ClientOrderId::new("invariant/order:1").unwrap())
}

/// A filter kind is chosen by the venue's own `filterType`, never by declaration order,
/// and each member reports the venue's evidence instead of a fabricated absence.
fn filter_payload() -> serde_json::Value {
    json!({
        "exchangeFilters": [
            {"filterType": "EXCHANGE_MAX_NUM_ORDERS", "maxNumOrders": 200},
            {"filterType": "EXCHANGE_MAX_NUM_ALGO_ORDERS", "maxNumAlgoOrders": 5},
        ],
        "symbolFilters": [
            {"filterType": "PRICE_FILTER", "minPrice": "0.00000100", "maxPrice": "1000.00000000", "tickSize": "0.00001000"},
            {"filterType": "LOT_SIZE", "minQty": "0.00100000", "maxQty": "9000.00000000", "stepSize": "0.00100000"},
            {"filterType": "NOTIONAL", "minNotional": "5.00000000", "applyMinToMarket": true, "maxNotional": "90000.00000000", "applyMaxToMarket": true, "avgPriceMins": 5},
            {"filterType": "MAX_POSITION", "maxPosition": "10.00000000"},
            {"filterType": "FUTURE_FILTER", "futureSetting": "1"},
        ],
        "assetFilters": [
            {"filterType":"MAX_ASSET", "asset":"USDC", "limit":"42.00000001"},
            {"filterType":"FUTURE_ASSET_FILTER", "futureSetting":"1"},
        ],
    })
}

#[test]
fn spot_rest_filter_lists_dispatch_on_the_venue_filter_type() {
    use binance_client::spot::rest_models::{
        MyFiltersResponseExchangeFiltersItem as Exchange,
        MyFiltersResponseSymbolFiltersItem as Symbol_,
    };
    let response: MyFiltersResponse = serde_json::from_value(filter_payload()).unwrap();
    let exchange = response.exchange_filters.as_ref().unwrap();
    assert!(matches!(&exchange[0], Exchange::ExchangeMaxNumOrders(f) if f.max_num_orders == 200));
    assert!(
        matches!(&exchange[1], Exchange::ExchangeMaxNumAlgoOrders(f) if f.max_num_algo_orders == 5)
    );
    let symbol = response.symbol_filters.as_ref().unwrap();
    assert!(matches!(&symbol[0], Symbol_::PriceFilter(f)
        if f.min_price == "0.00000100".parse::<Decimal>().unwrap()
            && f.max_price == "1000.00000000".parse::<Decimal>().unwrap()
            && f.tick_size == "0.00001000".parse::<Decimal>().unwrap()));
    assert!(matches!(&symbol[1], Symbol_::LotSize(f)
        if f.min_qty == "0.00100000".parse::<Decimal>().unwrap()
            && f.max_qty == "9000.00000000".parse::<Decimal>().unwrap()
            && f.step_size == "0.00100000".parse::<Decimal>().unwrap()));
    assert!(matches!(&symbol[2], Symbol_::Notional(f) if f.apply_min_to_market));
    assert!(matches!(&symbol[3], Symbol_::MaxPosition(f) if f.max_position == Decimal::TEN));
    // A filter this build does not model is retained, never forced into a documented
    // kind with invented evidence.
    assert!(matches!(&symbol[4], Symbol_::Unknown(_)));
    let assets = response.asset_filters.unwrap();
    assert!(
        matches!(&assets[0], binance_client::spot::rest_models::MyFiltersResponseAssetFiltersItem::MaxAsset(f)
        if f.asset.as_str() == "USDC" && f.limit == "42.00000001".parse::<Decimal>().unwrap())
    );
    assert!(matches!(
        &assets[1],
        binance_client::spot::rest_models::MyFiltersResponseAssetFiltersItem::Unknown(_)
    ));
}

#[test]
fn spot_socket_filter_lists_dispatch_on_the_venue_filter_type() {
    use binance_client::spot::ws_models::{
        MyFiltersResponseExchangeFiltersItem as Exchange,
        MyFiltersResponseSymbolFiltersItem as SocketSymbolFilters,
    };
    let response: SocketMyFiltersResponse = serde_json::from_value(filter_payload()).unwrap();
    let exchange = response.exchange_filters.as_ref().unwrap();
    assert!(matches!(&exchange[0], Exchange::ExchangeMaxNumOrders(f) if f.max_num_orders == 200));
    assert!(
        matches!(&exchange[1], Exchange::ExchangeMaxNumAlgoOrders(f) if f.max_num_algo_orders == 5)
    );
    let symbol = response.symbol_filters.as_ref().unwrap();
    assert!(matches!(&symbol[1], SocketSymbolFilters::LotSize(f)
        if f.min_qty == "0.00100000".parse::<Decimal>().unwrap()
            && f.max_qty == "9000.00000000".parse::<Decimal>().unwrap()
            && f.step_size == "0.00100000".parse::<Decimal>().unwrap()));
    assert!(matches!(&symbol[4], SocketSymbolFilters::Unknown(_)));
    let assets = response.asset_filters.unwrap();
    assert!(
        matches!(&assets[0], binance_client::spot::ws_models::MyFiltersResponseAssetFiltersItem::MaxAsset(f)
        if f.asset.as_str() == "USDC" && f.limit == "42.00000001".parse::<Decimal>().unwrap())
    );
    assert!(matches!(
        &assets[1],
        binance_client::spot::ws_models::MyFiltersResponseAssetFiltersItem::Unknown(_)
    ));
}

/// A filter without its discriminator, or missing the evidence its kind requires, is
/// refused outright: the client never reports "no limit supplied" for a limit the
/// venue did supply, and never supplies one it did not.
#[test]
fn incomplete_or_undiscriminated_filters_are_refused() {
    for payload in [
        json!({"symbolFilters": [{"minQty": "0.00100000", "maxQty": "9000.00000000", "stepSize": "0.00100000"}]}),
        json!({"symbolFilters": [{"filterType": "LOT_SIZE", "maxQty": "9000.00000000", "stepSize": "0.00100000"}]}),
        json!({"symbolFilters": [{"filterType": "LOT_SIZE", "minQty": "0.00100000", "stepSize": ""}]}),
        json!({"exchangeFilters": [{"filterType": "EXCHANGE_MAX_NUM_ORDERS"}]}),
        json!({"symbolFilters": [{"filterType":"MAX_POSITION", "maxPosition":"NaN"}]}),
        json!({"assetFilters": [{"filterType":"MAX_ASSET", "asset":"BTC", "limit":"NaN"}]}),
        json!({"assetFilters": [{"filterType":"MAX_ASSET", "limit":"1"}]}),
    ] {
        let rest: Result<MyFiltersResponse, _> = serde_json::from_value(payload.clone());
        assert!(rest.is_err(), "rest accepted {payload}");
        let text = payload.to_string();
        let socket: Result<SocketMyFiltersResponse, _> = serde_json::from_value(payload);
        assert!(socket.is_err(), "socket accepted {text}");
    }
}

/// An inverse position's margin mode and auto-add flag are venue vocabulary, never
/// decimal quantities, and an absent mode stays absent rather than defaulting.
#[test]
fn inverse_position_retains_its_margin_mode_verbatim() {
    let payload = json!([{
        "symbol": "BTCUSD_PERP", "positionAmt": "-1", "entryPrice": "60000.0",
        "markPrice": "61000.0", "unRealizedProfit": "-1000.0", "liquidationPrice": "50000.0",
        "leverage": "20", "maxQty": "50", "marginType": "isolated", "isolatedMargin": "3000.0",
        "isAutoAddMargin": "false", "positionSide": "SHORT", "notionalValue": "61000",
        "isolatedWallet": "3000", "updateTime": 1_758_931_200_000_i64
    }]);
    let parsed: PositionInformationResponse = serde_json::from_value(payload.clone()).unwrap();
    let position = &parsed[0];
    assert_eq!(position.margin_type.as_deref(), Some("isolated"));
    assert_eq!(position.is_auto_add_margin.as_deref(), Some("false"));
    assert_eq!(
        position
            .position_side
            .as_ref()
            .map(binance_client::coinm::enums::PositionSide::as_str),
        Some("SHORT")
    );
    assert_eq!(
        position.position_amt,
        Some("-1".parse::<Decimal>().unwrap()),
        "the position quantity keeps its own exact scale"
    );
    let absent: PositionInformationResponse =
        serde_json::from_value(json!([{"symbol": "BTCUSD_PERP", "positionAmt": "0"}])).unwrap();
    assert_eq!(absent[0].margin_type, None);
    assert_eq!(absent[0].is_auto_add_margin, None);
    // A margin mode is venue vocabulary: a JSON number is never read as one.
    let refused: Result<PositionInformationResponse, _> =
        serde_json::from_value(json!([{"symbol": "BTCUSD_PERP", "marginType": 1.5}]));
    assert!(refused.is_err(), "a numeric margin mode was accepted");
}

#[tokio::test]
async fn unrepresentable_retry_timing_keeps_the_acknowledgment() {
    use binance_client::usdm::rest_requests::CheckServerTime;
    use support::{HttpFixture, config, deadline};

    // A venue may answer with a Retry-After this client cannot represent. The answer
    // still arrives; the timing is honoured by refusing, never by discarding it.
    let fixture = HttpFixture::new(
        200,
        "Retry-After: 18446744073709551615\r\n",
        &json!({"serverTime": 1_700_000_000_000_i64}).to_string(),
        None,
        false,
    )
    .await;
    let client =
        binance_client::usdm::RestClient::new(config().rest_url(&fixture.url).unwrap()).unwrap();
    let acknowledged = client
        .check_server_time(&CheckServerTime::new(), deadline())
        .await
        .unwrap();
    assert_eq!(acknowledged.data.server_time, Some(1_700_000_000_000_i64));
    let refused = client
        .check_server_time(&CheckServerTime::new(), deadline())
        .await;
    assert!(
        matches!(refused, Err(binance_client::Error::Admission { .. })),
        "a wider cooldown must refuse the next send, got {refused:?}"
    );
    assert_eq!(
        fixture.connections_accepted(),
        1,
        "the refusal never reached the wire"
    );
    fixture.finish().await;
}

#[tokio::test]
async fn malformed_http_retry_timing_keeps_the_answer_and_refuses_future_sends() {
    use binance_client::{Error, usdm::rest_requests::CheckServerTime};
    use support::{HttpFixture, config, deadline};
    for value in ["invalid", "-1", "18446744073709551616"] {
        let fixture = HttpFixture::new(
            200,
            &format!("Retry-After: {value}\r\n"),
            r#"{"serverTime":1700000000000}"#,
            None,
            false,
        )
        .await;
        let client =
            binance_client::usdm::RestClient::new(config().rest_url(&fixture.url).unwrap())
                .unwrap();
        let response = client
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap();
        assert_eq!(response.data.server_time, Some(1_700_000_000_000));
        assert!(response.meta.rates.retry_after_unusable);
        assert!(matches!(
            client
                .check_server_time(&CheckServerTime::new(), deadline())
                .await,
            Err(Error::CooldownTimingUnknown)
        ));
        assert_eq!(fixture.connections_accepted(), 1);
        fixture.finish().await;
    }
}

#[tokio::test]
async fn a_correlated_answer_survives_evidence_that_cannot_be_classified() {
    use binance_client::usdm::{ApiEvent, WsClient};
    use futures_util::{SinkExt, StreamExt};
    use support::{config, deadline};
    use tokio::net::TcpListener;
    use tokio_websockets::{Limits, Message, ServerBuilder};

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut ws = ServerBuilder::new()
            .limits(Limits::unlimited())
            .accept(stream)
            .await
            .unwrap()
            .1;
        let request = ws.next().await.unwrap().unwrap();
        let id = serde_json::from_slice::<serde_json::Value>(request.as_payload()).unwrap()["id"]
            .as_str()
            .unwrap()
            .to_owned();
        // A correlated answer with no result and no error: unusable as an
        // acknowledgment, but still an accepted event from the venue.
        ws.send(Message::text(json!({"id": id, "status": 200}).to_string()))
            .await
            .unwrap();
        while let Some(message) = ws.next().await {
            if message.unwrap().is_close() {
                ws.flush().await.unwrap();
                break;
            }
        }
    });
    let (client, mut events, driver) = WsClient::connect(config().websocket_url(&url).unwrap())
        .await
        .unwrap();
    let driver = tokio::spawn(driver.run());
    events.recv().await.unwrap();
    let error = client
        .new_order(&order(), RequestId::new("invariant-1").unwrap(), deadline())
        .await
        .unwrap_err();
    assert_eq!(error.outcome(), Some(Outcome::Unknown), "{error}");
    let mut accepted = Vec::new();
    while let Some(event) = events.recv().await {
        match event {
            ApiEvent::Unknown { payload, .. } => accepted.push(payload.as_value().clone()),
            ApiEvent::Gap { error, .. } => {
                assert!(
                    matches!(
                        error,
                        binance_client::Error::Gap("WebSocket response payload")
                    ),
                    "{error}"
                );
                accepted.push(serde_json::Value::String("reported gap".into()));
            }
            ApiEvent::Retired(_) => break,
            _ => (),
        }
    }
    // The venue's answer is delivered in source order ahead of the failure it caused.
    assert_eq!(accepted.len(), 2, "{accepted:?}");
    assert_eq!(accepted[0]["status"], 200);
    assert!(
        accepted[0]["id"].is_string(),
        "correlation evidence is preserved"
    );
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn unusable_venue_retry_deadlines_refuse_instead_of_reading_as_no_cooldown() {
    use binance_client::{Error, usdm::WsClient};
    use futures_util::{SinkExt, StreamExt};
    use support::{config, deadline};
    use tokio::net::TcpListener;
    use tokio_websockets::{Limits, Message, ServerBuilder};

    for retry_after in [
        json!(1_699_999_999_000_u64),
        json!(-1),
        json!("invalid"),
        json!(null),
    ] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("ws://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut ws = ServerBuilder::new()
                .limits(Limits::unlimited())
                .accept(stream)
                .await
                .unwrap()
                .1;
            let request = ws.next().await.unwrap().unwrap();
            let id =
                serde_json::from_slice::<serde_json::Value>(request.as_payload()).unwrap()["id"]
                    .as_str()
                    .unwrap()
                    .to_owned();
            // The venue's retry deadline is already behind the injected clock. Zero
            // remaining time is not evidence that no cooldown applies.
            ws.send(Message::text(
                json!({
                    "id": id, "status": 429,
                    "error": {"code": -1003, "msg": "Too many requests", "retryAfter": retry_after}
                })
                .to_string(),
            ))
            .await
            .unwrap();
            while let Some(message) = ws.next().await {
                if message.unwrap().is_close() {
                    ws.flush().await.unwrap();
                    break;
                }
            }
        });
        let (client, mut events, driver) = WsClient::connect(config().websocket_url(&url).unwrap())
            .await
            .unwrap();
        let driver = tokio::spawn(driver.run());
        events.recv().await.unwrap();
        let refused = client
            .new_order(&order(), RequestId::new("invariant-2").unwrap(), deadline())
            .await
            .unwrap_err();
        // The venue refusal is reported exactly as sent, with the unusable timing kept
        // as evidence instead of being read as "retry now".
        let Error::Venue(failure) = &refused else {
            panic!("expected the venue refusal, got {refused}");
        };
        assert_eq!(failure.status, 429);
        assert_eq!(failure.code, Some(-1003));
        assert_eq!(failure.outcome, Outcome::Unknown);
        assert!(failure.rates.retry_after.is_none());
        assert!(failure.rates.retry_after_unusable);
        let afterwards = client
            .new_order(&order(), RequestId::new("invariant-3").unwrap(), deadline())
            .await
            .unwrap_err();
        assert!(
            matches!(afterwards, Error::CooldownTimingUnknown),
            "{afterwards}"
        );
        client.close().await.unwrap();
        while let Some(event) = events.recv().await {
            if matches!(event, binance_client::usdm::ApiEvent::Retired(_)) {
                break;
            }
        }
        driver.await.unwrap().unwrap();
        server.await.unwrap();
    }
}

/// An unsent command reports that it was never sent. The caller's own deadline did
/// not expire, and no venue deadline or verdict is invented to explain the refusal.
#[test]
fn an_unsent_command_reports_not_sent_rather_than_an_expired_deadline() {
    let error = binance_client::Error::NotSent {
        client_order_ids: std::collections::BTreeMap::new(),
        operation: "newOrder",
        reason: "socket generation retired before send",
    };
    assert_eq!(error.outcome(), Some(Outcome::NotSent));
    assert!(
        !error.to_string().contains("deadline"),
        "an unsent command must not claim a deadline it never reached: {error}"
    );
}

#[tokio::test]
async fn generation_retirement_preserves_the_unsent_orders_identity() {
    use binance_client::{Error, usdm::WsClient};
    use futures_util::{SinkExt, StreamExt};
    use support::{config, deadline};
    use tokio::net::TcpListener;
    use tokio_websockets::ServerBuilder;

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut ws = ServerBuilder::new().accept(stream).await.unwrap().1;
        let message = ws.next().await.unwrap().unwrap();
        assert!(message.is_close(), "an unsent order reached the wire");
        ws.flush().await.unwrap();
    });
    let (client, mut events, driver) = WsClient::connect(config().websocket_url(&url).unwrap())
        .await
        .unwrap();
    let request = order();
    let call = client.new_order(
        &request,
        RequestId::new("queued-order").unwrap(),
        deadline(),
    );
    tokio::pin!(call);
    // Poll the call once to place it in the inbox before the driver starts.
    tokio::select! {
        biased;
        result = &mut call => panic!("queued call settled early: {result:?}"),
        () = std::future::ready(()) => (),
    }
    let (closed, retired) = tokio::join!(client.close(), driver.run());
    closed.unwrap();
    retired.unwrap();
    let error = call.await.unwrap_err();
    let Error::NotSent {
        client_order_ids,
        operation,
        ..
    } = error
    else {
        panic!("expected an unsent lifecycle refusal, got {error:?}");
    };
    assert_eq!(operation, "newOrder");
    assert_eq!(client_order_ids["newClientOrderId"], "invariant/order:1");
    assert!(matches!(
        events.recv().await,
        Some(binance_client::usdm::ApiEvent::Established(_))
    ));
    assert!(matches!(
        events.recv().await,
        Some(binance_client::usdm::ApiEvent::Retired(_))
    ));
    server.await.unwrap();
}
