// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native advanced streams preserve generations, protocol routes and exact values.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic fixture assertions"
)]
#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;
use binance_client::{
    Credentials, Decimal, SensitiveString, WeightPools,
    advanced_trading::{
        self,
        config::{Config, Product},
        streams::StreamEvent,
    },
};
use futures_util::{SinkExt, StreamExt};
use support::deadline;
use tokio::net::TcpListener;
use tokio_websockets::{Message, ServerBuilder};

fn config<P: Product>() -> Config<P> {
    Config::with_pools(&WeightPools::new())
        .unwrap()
        .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
}

macro_rules! user_stream {
    ($test:ident,$product:ident,$base:literal,$path:literal,$frame:literal,$variant:ident) => {
        #[tokio::test]
        async fn $test() {
            use advanced_trading::$product;
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let addr = listener.local_addr().unwrap();
            let (send_later, receive_later) = tokio::sync::oneshot::channel();
            let server = tokio::spawn(async move {
                let (tcp, _) = listener.accept().await.unwrap();
                let mut peek = [0; 1024];
                let len = tcp.peek(&mut peek).await.unwrap();
                assert!(
                    std::str::from_utf8(&peek[..len])
                        .unwrap()
                        .starts_with(concat!("GET ", $path, " "))
                );
                let (_, mut ws) = ServerBuilder::new().accept(tcp).await.unwrap();
                ws.send(Message::text($frame)).await.unwrap();
                ws.send(Message::text(
                    r#"{"e":"futureNativeEvent","E":1700000001001,"future":"native"}"#,
                ))
                .await
                .unwrap();
                receive_later.await.unwrap();
                ws.send(Message::text($frame)).await.unwrap();
                while let Some(m) = ws.next().await {
                    if m.unwrap().is_close() {
                        break;
                    }
                }
            });
            let (mut stream, driver) = $product::Streams::user_data(
                config()
                    .streams_url(&format!("ws://{addr}{}", $base))
                    .unwrap(),
                SensitiveString::new("synthetic-listen-key"),
            )
            .await
            .unwrap();
            let driver = tokio::spawn(driver.run());
            assert!(matches!(
                stream.recv().await,
                Some(StreamEvent::Established(_))
            ));
            assert!(matches!(
                stream.recv().await,
                Some(StreamEvent::Data {
                    payload: $product::event_payloads::Payload::$variant(_),
                    ..
                })
            ));
            assert!(matches!(
                stream.recv().await,
                Some(StreamEvent::Data {
                    payload: $product::event_payloads::Payload::Unknown(_),
                    ..
                })
            ));
            if <$product::Protocol as advanced_trading::streams::StreamingProduct>::MAX_GENERATION_AGE.is_none() {
                tokio::time::pause();
                tokio::time::advance(std::time::Duration::from_hours(25)).await;
            }
            send_later.send(()).unwrap();
            assert!(matches!(stream.recv().await, Some(StreamEvent::Data { payload: $product::event_payloads::Payload::$variant(_), .. })));
            stream.close().await.unwrap();
            loop {
                match stream.recv().await.expect("retirement evidence") {
                    StreamEvent::Retired(_) => break,
                    StreamEvent::Gap { error, .. } => panic!("unexpected gap: {error:?}"),
                    _ => {},
                }
            }
            driver.await.unwrap().unwrap();
            server.await.unwrap();
        }
    };
}
user_stream!(
    portfolio_margin_native_execution_route,
    portfolio_margin,
    "/pm",
    "/pm/ws/synthetic-listen-key",
    r#"{"e":"ACCOUNT_UPDATE","fs":"CM","E":1700000001000,"T":1700000000999,"a":{"m":"ORDER","B":[{"a":"BTC","wb":"0.123456789123456789","cw":"0.1","bc":"0"}],"P":[]}}"#,
    AccountUpdate
);
user_stream!(
    portfolio_margin_pro_native_execution_route,
    portfolio_margin_pro,
    "/pm-classic",
    "/pm-classic/ws/synthetic-listen-key",
    r#"{"e":"PM_PRO_ACCOUNT_UPDATE","E":1700000001000,"T":1700000000999,"u":"1.99999999","eq":"30.23416728","ae":"30.23416728","im":"20.6783833","mm":"15.11708371","avb":"3.98972738","vmw":"3.98972738"}"#,
    PmProAccountUpdate
);
user_stream!(
    stocks_native_order_report_route,
    stocks,
    "/equity",
    "/equity/ws/synthetic-listen-key@orderReport",
    r#"{"e":"orderReport","E":1700000001000,"T":1700000000999,"s":"AAPL","i":"synthetic-order","c":"0123456789abcdef0123456789abcdef","S":"BUY","o":"LIMIT","X":"PARTIALLY_FILLED","q":"USD","b":"EQ_AAPL","p":185.5,"Q":10,"fq":5,"FN":927.5,"tc":1856.2,"Z":50}"#,
    OrderReportStream
);

#[tokio::test]
async fn alpha_subscription_retains_full_depth_continuity_without_building_a_book() {
    use advanced_trading::alpha;
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let (_, mut ws) = ServerBuilder::new().accept(tcp).await.unwrap();
        let request: serde_json::Value =
            serde_json::from_str(ws.next().await.unwrap().unwrap().as_text().unwrap()).unwrap();
        assert_eq!(request["method"], "SUBSCRIBE");
        assert_eq!(
            request["params"],
            serde_json::json!(["alpha_173usdt@fulldepth@500ms"])
        );
        ws.send(Message::text(
            serde_json::json!({"result":null,"id":request["id"]}).to_string(),
        ))
        .await
        .unwrap();
        ws.send(Message::text(r#"{"stream":"alpha_173usdt@fulldepth@500ms","data":{"e":"depthUpdate","E":1700000001000,"T":1700000000999,"s":"ALPHA_173USDT","U":7,"u":9,"pu":6,"b":[["0.123456789123456789","0"]],"a":[["0.2","3"]]}}"#)).await.unwrap();
        let list: serde_json::Value =
            serde_json::from_str(ws.next().await.unwrap().unwrap().as_text().unwrap()).unwrap();
        assert_eq!(list["method"], "LIST_SUBSCRIPTION");
        ws.send(Message::text(
            serde_json::json!({"result":["alpha_173usdt@fulldepth@500ms"],"id":list["id"]})
                .to_string(),
        ))
        .await
        .unwrap();
        while let Some(m) = ws.next().await {
            if m.unwrap().is_close() {
                break;
            }
        }
    });
    let (mut streams, driver) = alpha::Streams::connect(
        config()
            .streams_url(&format!("ws://{addr}/w3w/wsa/stream"))
            .unwrap(),
    )
    .await
    .unwrap();
    let driver = tokio::spawn(driver.run());
    assert!(matches!(
        streams.recv().await,
        Some(StreamEvent::Established(_))
    ));
    let name = alpha::Stream::full_depth_stream(
        &binance_client::Symbol::new("ALPHA_173USDT").unwrap(),
        "500ms",
    )
    .unwrap();
    streams.subscribe(&[name], deadline()).await.unwrap();
    let Some(StreamEvent::Data {
        payload: alpha::event_payloads::Payload::FullDepthStream(data),
        ..
    }) = streams.recv().await
    else {
        panic!("typed native depth")
    };
    assert_eq!(data.upper_u, 7);
    assert_eq!(data.u, 9);
    assert_eq!(data.pu, 6);
    let levels = data.b;
    assert_eq!(
        levels[0].price,
        Decimal::from_str_exact("0.123456789123456789").unwrap()
    );
    assert_eq!(levels[0].quantity, Decimal::ZERO);
    assert_eq!(
        serde_json::to_value(levels[0]).unwrap(),
        serde_json::json!(["0.123456789123456789", "0"])
    );
    assert_eq!(
        streams.list_subscriptions(deadline()).await.unwrap(),
        vec!["alpha_173usdt@fulldepth@500ms"]
    );
    streams.close().await.unwrap();
    while !matches!(streams.recv().await, Some(StreamEvent::Retired(_))) {}
    driver.await.unwrap().unwrap();
    server.await.unwrap();
}

#[test]
fn malformed_known_execution_money_fails_and_future_events_remain_explicit() {
    use advanced_trading::{portfolio_margin as pm, portfolio_margin_pro as pro, stocks};
    let good = serde_json::json!({"e":"ACCOUNT_UPDATE","fs":"CM","E":1_700_000_001_000_i64,"T":1_700_000_000_999_i64,"a":{"m":"ORDER","B":[{"a":"BTC","wb":"0.123456789123456789","cw":"0.1","bc":"0"}],"P":[]}});
    let pm::event_payloads::Payload::AccountUpdate(event) =
        pm::event_payloads::decode(None, good.clone()).unwrap()
    else {
        panic!("native update")
    };
    assert_eq!(event.fs, "CM");
    assert_eq!(
        event.a.upper_b.unwrap()[0].wb,
        Decimal::from_str_exact("0.123456789123456789").unwrap()
    );
    let mut bad = good;
    bad["a"]["B"][0]["wb"] = serde_json::json!("NaN");
    assert!(pm::event_payloads::decode(None, bad).is_err());
    assert!(
        pro::event_payloads::decode(
            None,
            serde_json::json!({"e":"PM_PRO_ACCOUNT_UPDATE","E":1,"eq":"bad"})
        )
        .is_err()
    );
    assert!(
        stocks::event_payloads::decode(
            Some("orderReportStream"),
            serde_json::json!({"e":"orderReport","E":1,"s":"AAPL","p":"bad"})
        )
        .is_err()
    );
}

#[test]
fn every_native_event_accepts_its_documented_categories_without_financial_coercion() {
    let fixtures: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/advanced-native-events.json")).unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let frame = case["frame"].clone();
        let kind = case["kind"].as_str();
        let result=match case["product"].as_str().unwrap(){
            "portfolio_margin"=>advanced_trading::portfolio_margin::event_payloads::decode(kind,frame).map(|v|!matches!(v,advanced_trading::portfolio_margin::event_payloads::Payload::Unknown(_))),
            "portfolio_margin_pro"=>advanced_trading::portfolio_margin_pro::event_payloads::decode(kind,frame).map(|v|!matches!(v,advanced_trading::portfolio_margin_pro::event_payloads::Payload::Unknown(_))),
            "alpha"=>advanced_trading::alpha::event_payloads::decode(kind,frame).map(|v|!matches!(v,advanced_trading::alpha::event_payloads::Payload::Unknown(_))),
            "stocks"=>advanced_trading::stocks::event_payloads::decode(kind,frame).map(|v|!matches!(v,advanced_trading::stocks::event_payloads::Payload::Unknown(_))),
            _=>panic!("fixture product"),
        };
        assert!(
            result.unwrap_or_else(|e| panic!("{} native typing: {e:?}", case["name"])),
            "{} must be typed",
            case["name"]
        );
    }
}

#[test]
fn missing_assets_position_mode_and_full_depth_continuity_are_gaps() {
    use advanced_trading::{alpha, portfolio_margin as pm};
    for frame in [
        serde_json::json!({"e":"ACCOUNT_UPDATE","fs":"CM","E":1,"T":1,"a":{"m":"ORDER","B":[{"wb":"1","cw":"1","bc":"0"}]}}),
        serde_json::json!({"e":"ACCOUNT_UPDATE","fs":"CM","E":1,"T":1,"a":{"m":"ORDER","P":[{"s":"BTCUSD_PERP","pa":"1","ep":"1","cr":"0","up":"0"}]}}),
    ] {
        assert!(pm::event_payloads::decode(None, frame).is_err());
    }
    let full = serde_json::json!({"e":"depthUpdate","E":1,"s":"ALPHA_1USDT","U":7,"u":9,"pu":6,"b":[["1","0"]],"a":[["2","1"]]});
    assert!(alpha::event_payloads::decode(Some("fullDepthStream"), full.clone()).is_ok());
    for field in ["U", "u", "pu", "b", "a"] {
        let mut missing = full.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert!(
            alpha::event_payloads::decode(Some("fullDepthStream"), missing).is_err(),
            "missing {field}"
        );
    }
}
