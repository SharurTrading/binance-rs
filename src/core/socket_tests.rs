// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::super::Request;
use super::*;
use serde_json::json;
use tokio::net::TcpListener;
use tokio_websockets::ServerBuilder;

type ResponseReceiver = oneshot::Receiver<Result<(Value, ResponseMeta), Error>>;

async fn fixture()
-> Result<(SocketDriver, SocketEvents, WebSocketStream<TcpStream>), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let url = url::Url::parse(&format!("ws://{}", listener.local_addr()?))?;
    let connect = Socket::connect_with_policy(
        url,
        None,
        Arc::new(super::super::SystemClock),
        Budgets::new(super::super::BudgetLimits::usdm())?,
        Duration::from_secs(1),
        SocketPolicy {
            handshake: Cost::default(),
            incoming_limit: 5,
            time_unit: super::super::TimeUnit::Milliseconds,
            binary_decoder: None,
            api_key_header: false,
            max_generation_age: None,
        },
    );
    let accept = async {
        let (stream, _) = listener.accept().await?;
        let (_, peer) = ServerBuilder::new().accept(stream).await?;
        Ok::<_, Box<dyn std::error::Error>>(peer)
    };
    let (connected, peer) = tokio::join!(connect, accept);
    let (_, events, driver) = connected?;
    Ok((driver, events, peer?))
}

fn pending() -> Result<(Pending, ResponseReceiver), Error> {
    let (reply, rx) = oneshot::channel();
    Ok((
        Pending {
            admitted_at: 0,
            weight: 0,
            client_order_ids: BTreeMap::from([("newClientOrderId".into(), "retained:1".into())]),
            op: <crate::usdm::ws_requests::NewOrder as Request>::OP,
            id: RequestId::new("correlated")?,
            deadline: Instant::now() + Duration::from_secs(1),
            reply: Some(reply),
        },
        rx,
    ))
}

#[tokio::test]
async fn cancelled_waiter_gets_one_late_answer_with_original_attribution()
-> Result<(), Box<dyn std::error::Error>> {
    let (mut driver, mut events, _peer) = fixture().await?;
    let (pending, rx) = pending()?;
    drop(rx);
    driver.pending.insert("correlated".into(), pending);
    let payload = json!({"orderId":7,"clientOrderId":"retained:1","future":{"nested":[1,2,3]}});
    driver.response(json!({"id":"correlated","status":200,"result":payload}))?;
    let event = events.recv().await;
    assert!(
        matches!(event, Some(SocketEvent::Late { id, value, meta, .. })
        if id.as_str() == "correlated" && value == payload && meta.status == 200
        && meta.client_order_ids["newClientOrderId"] == "retained:1")
    );
    assert!(driver.pending.is_empty());
    assert!(events.rx.try_recv().is_err());
    Ok(())
}

#[tokio::test]
async fn malformed_limits_deliver_correlated_answer_and_full_frame_before_gap()
-> Result<(), Box<dyn std::error::Error>> {
    let (mut driver, mut events, _peer) = fixture().await?;
    let (pending, rx) = pending()?;
    driver.pending.insert("correlated".into(), pending);
    let payload = json!({"orderId":7,"clientOrderId":"retained:1"});
    let frame = json!({"id":"correlated","status":200,"result":payload,
        "rateLimits":[{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":0,"count":3}],
        "futureEnvelope":{"retained":true}});
    let result = driver.response(frame.clone());
    assert!(matches!(
        result,
        Err(Error::Gap("stated rate limit without a positive limit"))
    ));
    let (received, meta) = rx.await??;
    assert_eq!(received, payload);
    assert!(meta.rates.stated_limits_unusable);
    assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 3);
    assert!(matches!(events.recv().await, Some(SocketEvent::Data { value, .. }) if value == frame));
    assert!(driver.pending.is_empty());
    assert!(events.rx.try_recv().is_err());
    Ok(())
}

#[tokio::test]
async fn missing_payload_keeps_original_frame_and_correlated_response_evidence()
-> Result<(), Box<dyn std::error::Error>> {
    let (mut driver, mut events, _peer) = fixture().await?;
    let (pending, rx) = pending()?;
    driver.pending.insert("correlated".into(), pending);
    let frame = json!({"id":"correlated","status":200,"futureEnvelope":[1,2,3]});
    assert!(matches!(
        driver.response(frame.clone()),
        Err(Error::Gap("WebSocket response payload"))
    ));
    let result = rx.await?;
    assert!(
        matches!(result, Err(Error::Transport { meta:Some(meta), outcome:Outcome::Unknown, .. })
        if meta.status == 200 && meta.client_order_ids["newClientOrderId"] == "retained:1")
    );
    assert!(matches!(events.recv().await, Some(SocketEvent::Data { value, .. }) if value == frame));
    Ok(())
}

#[tokio::test]
async fn malformed_limits_after_cancellation_deliver_late_answer_then_original_frame()
-> Result<(), Box<dyn std::error::Error>> {
    let (mut driver, mut events, _peer) = fixture().await?;
    let (pending, rx) = pending()?;
    drop(rx);
    driver.pending.insert("correlated".into(), pending);
    let payload = json!({"orderId":7,"clientOrderId":"retained:1"});
    let frame = json!({"id":"correlated","status":200,"result":payload,
        "rateLimits":[{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":0,"count":3}]});
    assert!(matches!(
        driver.response(frame.clone()),
        Err(Error::Gap("stated rate limit without a positive limit"))
    ));
    assert!(
        matches!(events.recv().await, Some(SocketEvent::Late { id, value, meta, .. })
        if id.as_str() == "correlated" && value == payload && meta.rates.stated_limits_unusable)
    );
    assert!(matches!(events.recv().await, Some(SocketEvent::Data { value, .. }) if value == frame));
    assert!(events.rx.try_recv().is_err());
    Ok(())
}

#[tokio::test]
async fn exhausted_incoming_budget_retires_without_flushing_queued_pong()
-> Result<(), Box<dyn std::error::Error>> {
    let (mut driver, mut events, mut peer) = fixture().await?;
    driver.incoming_limit = 1;
    assert!(
        driver
            .charge_incoming(Instant::now(), IncomingCharge::Pong)
            .is_ok()
    );
    peer.send(Message::ping(b"queued-pong".as_slice())).await?;
    driver.run().await?;
    assert!(matches!(
        events.recv().await,
        Some(SocketEvent::Established(_))
    ));
    assert!(matches!(
        events.recv().await,
        Some(SocketEvent::Gap {
            error: Error::Gap("documented ping/pong rate exceeded"),
            ..
        })
    ));
    assert!(matches!(events.recv().await, Some(SocketEvent::Retired(_))));
    assert!(events.recv().await.is_none());
    let received = tokio::time::timeout(Duration::from_secs(1), peer.next()).await?;
    assert!(
        !matches!(received, Some(Ok(_))),
        "retirement flushed an unadmitted frame"
    );
    Ok(())
}
