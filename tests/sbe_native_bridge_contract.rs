// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native binary delivery preserves JSON maintenance and lifecycle order.
use binance_client::{
    Credentials, Decimal, Error, Signer, Symbol,
    spot::sbe::{MarketConfig, MarketEvent, MarketStream, MarketStreams, StreamEvent},
};
use futures_util::{SinkExt, StreamExt};
use std::{sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tokio_websockets::{Limits, Message, ServerBuilder};

struct NoSigning;
impl Signer for NoSigning {
    fn sign(&self, _: &[u8]) -> Result<String, Error> {
        Err(Error::Signing)
    }
}

fn quote() -> Vec<u8> {
    let mut bytes = [50_u16, 10001, 1, 0]
        .into_iter()
        .flat_map(u16::to_le_bytes)
        .collect::<Vec<_>>();
    bytes.extend_from_slice(&123_i64.to_le_bytes());
    bytes.extend_from_slice(&42_i64.to_le_bytes());
    bytes.extend_from_slice(&[(-8_i8).cast_unsigned(), (-3_i8).cast_unsigned()]);
    for mantissa in [100_000_001_i64, 2500, 100_000_002, 3250] {
        bytes.extend_from_slice(&mantissa.to_le_bytes());
    }
    bytes.push(7);
    bytes.extend_from_slice(b"BTCUSDT");
    bytes
}

#[tokio::test]
async fn native_binary_and_json_shutdown_keep_one_generation_and_source_order() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("ws://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let (_, mut socket) = ServerBuilder::new()
            .limits(Limits::unlimited())
            .accept(stream)
            .await
            .unwrap();
        socket.send(Message::binary(quote())).await.unwrap();
        socket
            .send(Message::text(r#"{"data":{"e":"serverShutdown","E":456}}"#))
            .await
            .unwrap();
        socket.send(Message::binary(quote())).await.unwrap();
        socket.send(Message::binary(vec![0])).await.unwrap();
        while let Some(frame) = socket.next().await {
            if frame.unwrap().is_close() {
                break;
            }
        }
    });
    let credentials =
        Credentials::external_ed25519("synthetic-sbe-key", Arc::new(NoSigning)).unwrap();
    let config = MarketConfig::production(credentials)
        .unwrap()
        .endpoint(&endpoint)
        .unwrap();
    let (mut events, driver) = MarketStreams::connect(
        config,
        &[MarketStream::best_bid_ask(&Symbol::new("BTCUSDT").unwrap()).unwrap()],
    )
    .await
    .unwrap();
    let owner = tokio::spawn(driver.run());
    let generation = match events.recv().await.unwrap() {
        StreamEvent::Established(generation) => generation,
        event => panic!("unexpected first event: {event:?}"),
    };
    for index in 0..3 {
        let event = events.recv().await.unwrap();
        if index == 1 {
            assert!(
                matches!(event, StreamEvent::ServerShutdown { generation: g, event_time: 456 } if g == generation)
            );
        } else {
            let StreamEvent::Data {
                generation: g,
                payload: MarketEvent::BestBidAsk(quote),
            } = event
            else {
                panic!("expected native quote")
            };
            assert_eq!(g, generation);
            assert_eq!(quote.bid_price, "1.00000001".parse::<Decimal>().unwrap());
        }
    }
    assert!(
        matches!(events.recv().await, Some(StreamEvent::Gap { generation:g, error:Error::BinaryDecode { .. } }) if g == generation)
    );
    assert!(matches!(events.recv().await, Some(StreamEvent::Retired(g)) if g == generation));
    assert_eq!(events.queue_stats().unwrap().depth, 0);
    tokio::time::timeout(Duration::from_secs(3), owner)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    tokio::time::timeout(Duration::from_secs(3), server)
        .await
        .unwrap()
        .unwrap();
}
