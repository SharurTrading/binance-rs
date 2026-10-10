// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicitly invoked, credentialed demo capture of cross-margin position
//! `ACCOUNT_UPDATE` evidence for issue #69.
//!
//! These probes never run in normal CI (`--ignored` only) and require the
//! operator's explicit authorization (BN-VALIDATE-01): each probe places and
//! fully closes one minimal demo market position on its market, then verifies
//! the flat state before recording evidence. Credentials are read from the
//! environment (`TEST_KEY`, `TEST_SECRET`); they, the listen key, signed URLs
//! and authentication payloads are never logged. Recorded frames keep only
//! what the venue put on the wire: `ACCOUNT_UPDATE` carries no account
//! identifier, and the fixture never stores the listen key or credentials.

#[cfg(test)]
mod tests {
    use binance_client::{ClientOrderId, Credentials, Decimal, Symbol};
    use futures_util::{SinkExt, StreamExt};
    use serde_json::Value;
    use std::sync::Arc;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use tokio::net::TcpStream;
    use tokio_websockets::{
        ClientBuilder, Connector, Limits, MaybeTlsStream, Message, WebSocketStream,
    };

    fn deadline() -> tokio::time::Instant {
        tokio::time::Instant::now() + Duration::from_secs(15)
    }

    fn credentials() -> Credentials {
        let key = std::env::var("TEST_KEY").expect("TEST_KEY set for the authorized probe");
        let secret = std::env::var("TEST_SECRET").expect("TEST_SECRET set");
        Credentials::hmac(&key, &secret).expect("valid demo credentials")
    }

    fn tls_config() -> rustls::ClientConfig {
        let roots = webpki_roots::TLS_SERVER_ROOTS
            .iter()
            .cloned()
            .collect::<rustls::RootCertStore>();
        rustls::ClientConfig::builder_with_provider(Arc::new(
            rustls::crypto::aws_lc_rs::default_provider(),
        ))
        .with_safe_default_protocol_versions()
        .expect("TLS versions")
        .with_root_certificates(roots)
        .with_no_client_auth()
    }

    type DemoStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

    async fn connect_private_stream(url: &str) -> DemoStream {
        let connector = Connector::Rustls(tokio_rustls::TlsConnector::from(Arc::new(tls_config())));
        let (stream, _) = ClientBuilder::new()
            .uri(url)
            .expect("demo private stream URI")
            .connector(&connector)
            .limits(Limits::unlimited())
            .connect()
            .await
            .expect("demo private stream connect");
        stream
    }

    /// One recorded wire frame with the probe phase that observed it.
    struct CapturedFrame {
        phase: &'static str,
        raw: String,
    }

    /// Read frames until `stop` accepts the accumulated evidence, a frame
    /// timeout passes with no traffic, or a safety cap is reached. Every
    /// `ACCOUNT_UPDATE` and `MARGIN_CALL` frame is recorded verbatim.
    async fn capture_frames(
        stream: &mut DemoStream,
        phase: &'static str,
        mut stop: impl FnMut(&[CapturedFrame]) -> bool,
    ) -> Vec<CapturedFrame> {
        let mut frames = Vec::new();
        for _ in 0..100 {
            if stop(&frames) {
                return frames;
            }
            let Ok(Some(Ok(message))) =
                tokio::time::timeout(Duration::from_secs(30), stream.next()).await
            else {
                break;
            };
            let Some(text) = message.as_text() else {
                continue;
            };
            let value: Value = serde_json::from_str(text).expect("demo frame is JSON");
            match value.get("e").and_then(Value::as_str) {
                Some("ACCOUNT_UPDATE" | "MARGIN_CALL") => {
                    println!(
                        "{phase}: recorded {}",
                        value["e"].as_str().unwrap_or("event")
                    );
                    frames.push(CapturedFrame {
                        phase,
                        raw: text.to_owned(),
                    });
                }
                _ => println!("{phase}: unrecorded frame kind"),
            }
        }
        frames
    }

    fn position_entries(frame: &CapturedFrame, symbol: &str) -> Vec<Value> {
        let value: Value = serde_json::from_str(&frame.raw).expect("captured frame is JSON");
        value["a"]["P"]
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter(|item| item["s"].as_str() == Some(symbol))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default()
    }

    fn write_fixture(market: &str, body: &Value) {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(format!(
                "{market}-cross-margin-account-update-2026-10-11.json"
            ));
        std::fs::write(&path, serde_json::to_string_pretty(&body).unwrap()).expect("fixture write");
        println!("fixture recorded: {}", path.display());
    }

    /// USD-M: one minimal `BTCUSDT` cross-margin round trip on the demo
    /// environment, capturing the native `ACCOUNT_UPDATE` pushes. Issue #69's
    /// remaining acceptance criterion; the recorded evidence settles whether a
    /// cross-margin position carries `iw`.
    #[tokio::test]
    #[ignore = "authorized credentialed demo probe placing and closing one demo market position; never runs in normal CI"]
    #[allow(
        clippy::too_many_lines,
        reason = "one sequential authorized probe; splitting would obscure the capture-then-cleanup sequence"
    )]
    async fn demo_usdm_cross_margin_account_update_capture() {
        use binance_client::usdm::{
            Config, Environment, RestClient,
            rest_requests::{
                ChangeMarginType, CloseUserDataStream, CurrentAllOpenOrders,
                GetCurrentPositionMode, NewOrder, PositionInformationV2, StartUserDataStream,
            },
        };
        let symbol = Symbol::new("BTCUSDT").unwrap();
        let client = RestClient::new(
            Config::new(Environment::Demo)
                .unwrap()
                .credentials(credentials()),
        )
        .unwrap();
        let dual = client
            .get_current_position_mode(&GetCurrentPositionMode::new().build().unwrap(), deadline())
            .await
            .unwrap()
            .data
            .dual_side_position
            .unwrap_or(false);
        println!("position mode: {}", if dual { "hedge" } else { "one-way" });

        // The probe owns its cleanup: it never touches an existing position.
        let before = client
            .position_information_v2(
                &PositionInformationV2::new()
                    .symbol(symbol.clone())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
            .unwrap()
            .data;
        assert!(
            before
                .iter()
                .all(|item| item.position_amt.unwrap_or(Decimal::ZERO) == Decimal::ZERO),
            "refusing to run beside an existing BTCUSDT position"
        );

        // -4046 is the venue's "no need to change margin type" reply: already
        // cross. Any other refusal fails the probe before any order is sent.
        match client
            .change_margin_type(
                &ChangeMarginType::new()
                    .symbol(symbol.clone())
                    .margin_type("CROSSED")
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
        {
            Ok(_) => println!("margin type set to CROSSED"),
            Err(binance_client::Error::Venue(failure)) if failure.code == Some(-4046) => {
                println!("margin type already CROSSED");
            }
            Err(error) => panic!("changeMarginType refused: {error}"),
        }

        let listen_key = client
            .start_user_data_stream(&StartUserDataStream::new().build().unwrap(), deadline())
            .await
            .unwrap()
            .data
            .listen_key
            .expect("listen key");
        let mut stream = connect_private_stream(&format!(
            "wss://demo-fstream.binance.com/private/ws/{}",
            listen_key.as_str()
        ))
        .await;

        let mut order = NewOrder::new()
            .symbol(symbol.clone())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::new(1, 3))
            .new_client_order_id(ClientOrderId::new("issue69/usdm-open").unwrap());
        if dual {
            order = order.position_side("LONG");
        }
        client
            .new_order(&order.build().unwrap(), deadline())
            .await
            .unwrap();
        let mut frames = capture_frames(&mut stream, "open", |frames| {
            frames.iter().any(|frame| {
                position_entries(frame, "BTCUSDT")
                    .iter()
                    .any(|entry| entry["pa"].as_str().is_some_and(|pa| pa != "0"))
            })
        })
        .await;
        assert!(
            frames.iter().any(|frame| {
                position_entries(frame, "BTCUSDT")
                    .iter()
                    .any(|entry| entry["pa"].as_str().is_some_and(|pa| pa != "0"))
            }),
            "no open-leg ACCOUNT_UPDATE observed"
        );

        let held = client
            .position_information_v2(
                &PositionInformationV2::new()
                    .symbol(symbol.clone())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
            .unwrap()
            .data
            .into_iter()
            .map(|item| (item.position_amt.unwrap_or(Decimal::ZERO), item.margin_type))
            .collect::<Vec<_>>();
        let (held_amt, margin_type) = held
            .iter()
            .find(|(amt, _)| *amt != Decimal::ZERO)
            .expect("REST confirms the demo position");
        println!("held {held_amt} with marginType {margin_type:?}");

        let mut close = NewOrder::new()
            .symbol(symbol.clone())
            .side("SELL")
            .type_value("MARKET")
            .quantity(held_amt.abs())
            .reduce_only("true")
            .new_client_order_id(ClientOrderId::new("issue69/usdm-close").unwrap());
        if dual {
            close = close.position_side("LONG");
        }
        client
            .new_order(&close.build().unwrap(), deadline())
            .await
            .unwrap();
        let close_frames = capture_frames(&mut stream, "close", |frames| {
            frames.iter().any(|frame| {
                position_entries(frame, "BTCUSDT")
                    .iter()
                    .any(|entry| entry["pa"].as_str() == Some("0"))
            })
        })
        .await;
        assert!(
            close_frames.iter().any(|frame| {
                position_entries(frame, "BTCUSDT")
                    .iter()
                    .any(|entry| entry["pa"].as_str() == Some("0"))
            }),
            "no close-leg ACCOUNT_UPDATE observed"
        );
        frames.extend(close_frames);

        // Owned cleanup is verified, not assumed.
        let after = client
            .position_information_v2(
                &PositionInformationV2::new()
                    .symbol(symbol.clone())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
            .unwrap()
            .data;
        assert!(
            after
                .iter()
                .all(|item| item.position_amt.unwrap_or(Decimal::ZERO) == Decimal::ZERO),
            "demo position not flat after cleanup"
        );
        let open = client
            .current_all_open_orders(
                &CurrentAllOpenOrders::new()
                    .symbol(symbol.clone())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
            .unwrap()
            .data;
        assert!(open.is_empty(), "demo orders remain open");
        client
            .close_user_data_stream(&CloseUserDataStream::new().build().unwrap(), deadline())
            .await
            .unwrap();
        stream.send(Message::close(None, "")).await.unwrap();

        write_fixture(
            "usdm",
            &serde_json::json!({
                "run": "issue-69-usdm-cross-margin-account-update",
                "observedAtUnixSeconds": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                "credentialed": true,
                "scrub": "No API key, secret, signed URL or listen key is recorded; ACCOUNT_UPDATE carries no account identifier.",
                "environment": {
                    "rest": "https://demo-fapi.binance.com",
                    "streams": "wss://demo-fstream.binance.com",
                    "streamPath": "/private/ws/<listenKey>"
                },
                "market": "usdm",
                "symbol": "BTCUSDT",
                "positionMode": if dual { "hedge" } else { "one-way" },
                "restMarginType": margin_type,
                "tradedQuantity": "0.001",
                "events": frames.iter().map(|frame| serde_json::json!({
                    "phase": frame.phase,
                    "raw": frame.raw
                })).collect::<Vec<_>>()
            }),
        );
    }

    /// COIN-M: one minimal `BTCUSD_PERP` cross-margin round trip on the demo
    /// environment, capturing the native `ACCOUNT_UPDATE` pushes for the
    /// inverse market's own schema.
    #[tokio::test]
    #[ignore = "authorized credentialed demo probe placing and closing one demo market position; never runs in normal CI"]
    #[allow(
        clippy::too_many_lines,
        reason = "one sequential authorized probe; splitting would obscure the capture-then-cleanup sequence"
    )]
    async fn demo_coinm_cross_margin_account_update_capture() {
        use binance_client::coinm::{
            Config, Environment, RestClient,
            rest_requests::{
                ChangeMarginType, CloseUserDataStream, CurrentAllOpenOrders,
                GetCurrentPositionMode, NewOrder, PositionInformation, StartUserDataStream,
            },
        };
        let symbol = Symbol::new("BTCUSD_PERP").unwrap();
        let client = RestClient::new(
            Config::new(Environment::Demo)
                .unwrap()
                .credentials(credentials()),
        )
        .unwrap();
        let dual = client
            .get_current_position_mode(&GetCurrentPositionMode::new().build().unwrap(), deadline())
            .await
            .unwrap()
            .data
            .dual_side_position
            .unwrap_or(false);
        println!("position mode: {}", if dual { "hedge" } else { "one-way" });

        let before = client
            .position_information(
                &PositionInformation::new()
                    .pair(Symbol::new("BTCUSD").unwrap())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
            .unwrap()
            .data;
        assert!(
            before
                .iter()
                .all(|item| item.position_amt.unwrap_or(Decimal::ZERO) == Decimal::ZERO),
            "refusing to run beside an existing BTCUSD_PERP position"
        );

        match client
            .change_margin_type(
                &ChangeMarginType::new()
                    .symbol(symbol.clone())
                    .margin_type("CROSSED")
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
        {
            Ok(_) => println!("margin type set to CROSSED"),
            Err(binance_client::Error::Venue(failure)) if failure.code == Some(-4046) => {
                println!("margin type already CROSSED");
            }
            Err(error) => panic!("changeMarginType refused: {error}"),
        }

        let listen_key = client
            .start_user_data_stream(&StartUserDataStream::new().build().unwrap(), deadline())
            .await
            .unwrap()
            .data
            .listen_key
            .expect("listen key");
        let mut stream = connect_private_stream(&format!(
            "wss://demo-dstream.binance.com/ws/{}",
            listen_key.as_str()
        ))
        .await;

        let mut order = NewOrder::new()
            .symbol(symbol.clone())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::ONE)
            .new_client_order_id(ClientOrderId::new("issue69/coinm-open").unwrap());
        if dual {
            order = order.position_side("LONG");
        }
        client
            .new_order(&order.build().unwrap(), deadline())
            .await
            .unwrap();
        let mut frames = capture_frames(&mut stream, "open", |frames| {
            frames.iter().any(|frame| {
                position_entries(frame, "BTCUSD_PERP")
                    .iter()
                    .any(|entry| entry["pa"].as_str().is_some_and(|pa| pa != "0"))
            })
        })
        .await;
        assert!(
            frames.iter().any(|frame| {
                position_entries(frame, "BTCUSD_PERP")
                    .iter()
                    .any(|entry| entry["pa"].as_str().is_some_and(|pa| pa != "0"))
            }),
            "no open-leg ACCOUNT_UPDATE observed"
        );

        let held = client
            .position_information(
                &PositionInformation::new()
                    .pair(Symbol::new("BTCUSD").unwrap())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
            .unwrap()
            .data;
        let (held_amt, margin_type) = held
            .into_iter()
            .filter(|item| {
                item.symbol
                    .as_ref()
                    .is_some_and(|s| s.as_str() == "BTCUSD_PERP")
            })
            .map(|item| (item.position_amt.unwrap_or(Decimal::ZERO), item.margin_type))
            .find(|(amt, _)| *amt != Decimal::ZERO)
            .expect("REST confirms the demo position");
        println!("held {held_amt} with marginType {margin_type:?}");

        let mut close = NewOrder::new()
            .symbol(symbol.clone())
            .side("SELL")
            .type_value("MARKET")
            .quantity(held_amt.abs())
            .reduce_only("true")
            .new_client_order_id(ClientOrderId::new("issue69/coinm-close").unwrap());
        if dual {
            close = close.position_side("LONG");
        }
        client
            .new_order(&close.build().unwrap(), deadline())
            .await
            .unwrap();
        let close_frames = capture_frames(&mut stream, "close", |frames| {
            frames.iter().any(|frame| {
                position_entries(frame, "BTCUSD_PERP")
                    .iter()
                    .any(|entry| entry["pa"].as_str() == Some("0"))
            })
        })
        .await;
        assert!(
            close_frames.iter().any(|frame| {
                position_entries(frame, "BTCUSD_PERP")
                    .iter()
                    .any(|entry| entry["pa"].as_str() == Some("0"))
            }),
            "no close-leg ACCOUNT_UPDATE observed"
        );
        frames.extend(close_frames);

        let after = client
            .position_information(
                &PositionInformation::new()
                    .pair(Symbol::new("BTCUSD").unwrap())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
            .unwrap()
            .data;
        assert!(
            after
                .iter()
                .filter(|item| item
                    .symbol
                    .as_ref()
                    .is_some_and(|s| s.as_str() == "BTCUSD_PERP"))
                .all(|item| item.position_amt.unwrap_or(Decimal::ZERO) == Decimal::ZERO),
            "demo position not flat after cleanup"
        );
        let open = client
            .current_all_open_orders(
                &CurrentAllOpenOrders::new()
                    .symbol(symbol.clone())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
            .unwrap()
            .data;
        assert!(open.is_empty(), "demo orders remain open");
        client
            .close_user_data_stream(&CloseUserDataStream::new().build().unwrap(), deadline())
            .await
            .unwrap();
        stream.send(Message::close(None, "")).await.unwrap();

        write_fixture(
            "coinm",
            &serde_json::json!({
                "run": "issue-69-coinm-cross-margin-account-update",
                "observedAtUnixSeconds": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                "credentialed": true,
                "scrub": "No API key, secret, signed URL or listen key is recorded; ACCOUNT_UPDATE carries no account identifier.",
                "environment": {
                    "rest": "https://demo-dapi.binance.com",
                    "streams": "wss://demo-dstream.binance.com",
                    "streamPath": "/ws/<listenKey>"
                },
                "market": "coinm",
                "symbol": "BTCUSD_PERP",
                "positionMode": if dual { "hedge" } else { "one-way" },
                "restMarginType": margin_type,
                "tradedQuantity": "1",
                "events": frames.iter().map(|frame| serde_json::json!({
                    "phase": frame.phase,
                    "raw": frame.raw
                })).collect::<Vec<_>>()
            }),
        );
    }
}
