// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Spot user-data subscription filters and the shutdown time a notice states.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "local fixture assertions"
)]

use binance_client::WeightPools;
use binance_client::spot::ws_models::UserDataFilterInput;
use binance_client::spot::ws_requests::{
    UserDataStreamSubscribe, UserDataStreamSubscribeSignature,
};
use binance_client::{Credentials, Error, RequestId, Symbol, spot};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::net::{TcpListener, TcpStream};
#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;
use support::deadline;
use tokio_websockets::{Limits, Message, ServerBuilder, WebSocketStream};

async fn listener() -> (TcpListener, String) {
    let l = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("ws://{}", l.local_addr().unwrap());
    (l, url)
}
async fn accept(l: TcpListener) -> WebSocketStream<TcpStream> {
    let (stream, _) = l.accept().await.unwrap();
    ServerBuilder::new()
        .limits(Limits::unlimited())
        .accept(stream)
        .await
        .unwrap()
        .1
}
async fn finish(mut peer: WebSocketStream<TcpStream>) {
    while let Some(m) = peer.next().await {
        if m.unwrap().is_close() {
            peer.flush().await.unwrap();
            break;
        }
    }
}
fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[test]
fn a_filter_the_venue_would_refuse_is_refused_before_sending() {
    let fills = UserDataFilterInput::new()
        .event_types(strings(&["executionReport"]))
        .execution_types(strings(&["TRADE"]));
    assert!(fills.clone().build().is_ok());
    assert!(UserDataStreamSubscribe::new().filter(fills).build().is_ok());
    // The venue lists execution types but documents no refusal for another value.
    let future = UserDataFilterInput::new().execution_types(strings(&["FILLED"]));
    assert!(future.clone().build().is_ok());
    assert!(
        UserDataStreamSubscribeSignature::new()
            .filter(future)
            .build()
            .is_ok()
    );
    for (refused, message) in [
        (UserDataFilterInput::new(), "empty user data filter"),
        (
            UserDataFilterInput::new().event_types(Vec::new()),
            "empty user data filter values",
        ),
        (
            UserDataFilterInput::new()
                .event_types(strings(&["executionReport", "executionReport"])),
            "duplicate user data filter value",
        ),
        (
            UserDataFilterInput::new().execution_types(strings(&["TRADE", "TRADE"])),
            "duplicate user data filter value",
        ),
        (
            UserDataFilterInput::new().event_types(strings(&["badType"])),
            "user data filter value",
        ),
        (
            UserDataFilterInput::new().symbols(strings(&[""])),
            "user data filter value",
        ),
        (
            UserDataFilterInput::new().assets(strings(&[""])),
            "user data filter value",
        ),
    ] {
        assert!(
            matches!(refused.clone().build(), Err(Error::Validation(found)) if found == message),
            "{message}"
        );
        assert!(
            matches!(
                UserDataStreamSubscribeSignature::new().filter(refused).build(),
                Err(Error::Validation(found)) if found == message
            ),
            "{message}"
        );
    }
}

#[tokio::test]
async fn signature_subscription_signs_its_filter_with_sorted_keys_and_no_whitespace() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(l).await;
        let request: Value =
            serde_json::from_slice(peer.next().await.unwrap().unwrap().as_payload()).unwrap();
        assert_eq!(request["method"], "userDataStream.subscribe.signature");
        let params = request["params"].as_object().unwrap();
        assert_eq!(
            params["filter"],
            json!({"a": ["BTC"], "e": ["executionReport", "balanceUpdate"], "s": ["BAZUSD"], "x": ["TRADE"]})
        );
        let timestamp = params["timestamp"].as_u64().unwrap();
        let payload = format!(
            r#"apiKey=synthetic-api-key&filter={{"a":["BTC"],"e":["executionReport","balanceUpdate"],"s":["BAZUSD"],"x":["TRADE"]}}&timestamp={timestamp}"#
        );
        let key = aws_lc_rs::hmac::Key::new(aws_lc_rs::hmac::HMAC_SHA256, b"synthetic-secret");
        let signature = aws_lc_rs::hmac::sign(&key, payload.as_bytes())
            .as_ref()
            .iter()
            .fold(String::new(), |mut text, b| {
                use std::fmt::Write as _;
                write!(&mut text, "{b:02x}").unwrap();
                text
            });
        assert_eq!(params["signature"], signature);
        peer.send(Message::text(
            json!({"id":request["id"],"status":200,"result":{"subscriptionId":3}}).to_string(),
        ))
        .await
        .unwrap();
        finish(peer).await;
    });
    let config = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .websocket_url(&url)
        .unwrap()
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap());
    let (client, mut events, driver) = spot::WsClient::connect(config).await.unwrap();
    let driver = tokio::spawn(driver.run());
    let generation = client.generation();
    assert!(matches!(events.recv().await, Some(spot::ApiEvent::Established(g)) if g == generation));
    let filter = UserDataFilterInput::new()
        .execution_types(strings(&["TRADE"]))
        .symbols(strings(&["BAZUSD"]))
        .event_types(strings(&["executionReport", "balanceUpdate"]))
        .assets(strings(&["BTC"]));
    let response = client
        .user_data_stream_subscribe_signature(
            &UserDataStreamSubscribeSignature::new().filter(filter),
            RequestId::new("subscribe").unwrap(),
            deadline(),
        )
        .await
        .unwrap();
    assert_eq!(response.data.subscription_id, 3);
    client.close().await.unwrap();
    assert!(matches!(events.recv().await, Some(spot::ApiEvent::Retired(g)) if g == generation));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn market_stream_delivers_every_shutdown_notice_with_what_it_states() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(l).await;
        for notice in [
            json!({"stream":"!serverShutdown","data":{"e":"serverShutdown","E":1_770_123_456_789_i64,"T":1_770_124_000_000_i64}}),
            json!({"e":"serverShutdown","E":5}),
            json!({"e":"serverShutdown","E":6,"T":"soon"}),
            json!({"e":"serverShutdown","T":7}),
            json!({"stream":"!serverShutdown","data":{"e":"serverShutdown","T":8}}),
        ] {
            peer.send(Message::text(notice.to_string())).await.unwrap();
        }
        finish(peer).await;
    });
    let config = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .streams_url(&url)
        .unwrap();
    let stream = spot::Stream::diff_book_depth(&Symbol::new("BTCUSDT").unwrap(), "100ms").unwrap();
    let (mut events, driver) = spot::Streams::connect(config, &[stream]).await.unwrap();
    let driver = tokio::spawn(driver.run());
    let generation = events.generation();
    assert!(
        matches!(events.recv().await, Some(spot::StreamEvent::Established(g)) if g == generation)
    );
    assert!(matches!(
        events.recv().await,
        Some(spot::StreamEvent::ServerShutdown {
            generation: g,
            event_time: 1_770_123_456_789,
            shutdown_time: Ok(Some(1_770_124_000_000)),
        }) if g == generation
    ));
    assert!(matches!(
        events.recv().await,
        Some(spot::StreamEvent::ServerShutdown {
            generation: g,
            event_time: 5,
            shutdown_time: Ok(None),
        }) if g == generation
    ));
    // A shutdown time that cannot be read does not hide the shutdown itself.
    assert!(matches!(
        events.recv().await,
        Some(spot::StreamEvent::ServerShutdown {
            generation: g,
            event_time: 6,
            shutdown_time: Err(Error::Gap(_)),
        }) if g == generation
    ));
    // An unenveloped notice names no stream; an enveloped one names its own.
    assert!(matches!(
        events.recv().await,
        Some(spot::StreamEvent::RecordLoss { generation: g, stream: None, error: Error::Gap("malformed shutdown notice") }) if g == generation
    ));
    assert!(matches!(
        events.recv().await,
        Some(spot::StreamEvent::RecordLoss { generation: g, stream: Some(s), error: Error::Gap("malformed shutdown notice") }) if g == generation && s == "!serverShutdown"
    ));
    events.close().await.unwrap();
    assert!(matches!(events.recv().await, Some(spot::StreamEvent::Retired(g)) if g == generation));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[tokio::test]
async fn api_socket_delivers_every_shutdown_notice_with_what_it_states() {
    let (l, url) = listener().await;
    let server = tokio::spawn(async move {
        let mut peer = accept(l).await;
        for notice in [
            json!({"event":{"e":"serverShutdown","E":3,"T":4}}),
            json!({"event":{"e":"serverShutdown","E":5}}),
            json!({"event":{"e":"serverShutdown","E":6,"T":"soon"}}),
            json!({"event":{"e":"serverShutdown","T":7}}),
        ] {
            peer.send(Message::text(notice.to_string())).await.unwrap();
        }
        finish(peer).await;
    });
    let config = spot::Config::with_pools(spot::Environment::Demo, &WeightPools::new())
        .unwrap()
        .websocket_url(&url)
        .unwrap();
    let (client, mut events, driver) = spot::WsClient::connect(config).await.unwrap();
    let driver = tokio::spawn(driver.run());
    let generation = client.generation();
    assert!(matches!(events.recv().await, Some(spot::ApiEvent::Established(g)) if g == generation));
    assert!(matches!(
        events.recv().await,
        Some(spot::ApiEvent::ServerShutdown { generation: g, shutdown_time: Ok(Some(4)), .. }) if g == generation
    ));
    assert!(matches!(
        events.recv().await,
        Some(spot::ApiEvent::ServerShutdown { generation: g, shutdown_time: Ok(None), .. }) if g == generation
    ));
    match events.recv().await {
        Some(spot::ApiEvent::ServerShutdown {
            generation: g,
            shutdown_time: Err(Error::Gap(_)),
            payload,
        }) => {
            assert_eq!(g, generation);
            assert_eq!(payload.as_value()["event"]["E"], 6);
        }
        other => panic!("unexpected event {other:?}"),
    }
    assert!(matches!(
        events.recv().await,
        Some(spot::ApiEvent::RecordLoss { generation: g, error: Error::Gap(_) }) if g == generation
    ));
    client.close().await.unwrap();
    assert!(matches!(events.recv().await, Some(spot::ApiEvent::Retired(g)) if g == generation));
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}
