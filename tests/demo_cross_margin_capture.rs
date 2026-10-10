// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicitly invoked, credentialed demo capture of cross-margin position
//! `ACCOUNT_UPDATE` evidence for issue #69.
//!
//! These probes never run in normal CI (`--ignored` only) and require the
//! operator's explicit authorization (BN-VALIDATE-01): each probe places and
//! fully closes one minimal demo market position on its market. Credentials
//! are read from the environment (`TEST_KEY`, `TEST_SECRET`); they, the listen
//! key, signed URLs and authentication payloads are never logged. Frames are
//! recorded with the venue's account-alias field `i` redacted before anything
//! is retained; no API key, secret, signed URL or listen key is ever recorded.
//!
//! Cleanup is owned on every exit path, not only the happy one (BN-ORDER-01:
//! a timeout or failed read does not cancel the open order): after the open
//! order is accepted, the round trip returns a typed result and an
//! unconditional cleanup routine reads the venue, flattens any held position
//! reduce-only, cancels stray orders, closes the listen key and the socket,
//! and prints the final held state before the probe surfaces its own failure.

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
    /// `ACCOUNT_UPDATE` and `MARGIN_CALL` frame is recorded with the venue's
    /// account-alias field `i` removed; every other byte is as captured.
    ///
    /// A `phase` label is capture-session attribution: a delayed venue push is
    /// attributed to the leg that observed it, not to venue causality.
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
            let mut value: Value = serde_json::from_str(text).expect("demo frame is JSON");
            match value.get("e").and_then(Value::as_str) {
                Some("ACCOUNT_UPDATE" | "MARGIN_CALL") => {
                    if value
                        .as_object_mut()
                        .is_some_and(|object| object.remove("i").is_some())
                    {
                        println!("{phase}: redacted account-alias field");
                    }
                    println!(
                        "{phase}: recorded {}",
                        value["e"].as_str().unwrap_or("event")
                    );
                    frames.push(CapturedFrame {
                        phase,
                        raw: value.to_string(),
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
        for event in body["events"].as_array().expect("capture events") {
            let raw: Value = serde_json::from_str(event["raw"].as_str().unwrap()).unwrap();
            assert!(
                raw.get("i").is_none(),
                "the account-alias field must be redacted before recording"
            );
        }
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(format!(
                "{market}-cross-margin-account-update-2026-10-11.json"
            ));
        std::fs::write(&path, serde_json::to_string_pretty(body).unwrap()).expect("fixture write");
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
        reason = "one sequential authorized probe; splitting would obscure the capture-cleanup sequence"
    )]
    async fn demo_usdm_cross_margin_account_update_capture() {
        use binance_client::usdm::{
            Config, Environment, RestClient,
            rest_requests::{
                ChangeMarginType, GetCurrentPositionMode, NewOrder, PositionInformationV2,
                StartUserDataStream,
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

        // -4046 is the documented "No need to change margin type." reply
        // ([error code](https://developers.binance.com/en/docs/derivatives/usds-margined-futures/error-code)):
        // already cross. Any other refusal fails the probe before any order is
        // sent.
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

        // From the open order on, failures return typed errors so the
        // unconditional cleanup below still owns the venue state.
        let capture: Result<(Vec<CapturedFrame>, String), String> = async {
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
                .map_err(|error| format!("open order: {error}"))?;
            let mut frames = capture_frames(&mut stream, "open", |frames| {
                frames.iter().any(|frame| {
                    position_entries(frame, "BTCUSDT")
                        .iter()
                        .any(|entry| entry["pa"].as_str().is_some_and(|pa| pa != "0"))
                })
            })
            .await;
            if !frames.iter().any(|frame| {
                position_entries(frame, "BTCUSDT")
                    .iter()
                    .any(|entry| entry["pa"].as_str().is_some_and(|pa| pa != "0"))
            }) {
                return Err("no open-leg ACCOUNT_UPDATE observed".to_owned());
            }

            // A slow fill is resolved by venue reads, never by assumption.
            let (held_amt, margin_type) = loop {
                let positions = client
                    .position_information_v2(
                        &PositionInformationV2::new()
                            .symbol(symbol.clone())
                            .build()
                            .unwrap(),
                        deadline(),
                    )
                    .await
                    .map_err(|error| format!("position read: {error}"))?;
                if let Some(found) = positions
                    .data
                    .iter()
                    .find(|item| item.position_amt.unwrap_or(Decimal::ZERO) != Decimal::ZERO)
                {
                    break (
                        found.position_amt.unwrap_or(Decimal::ZERO),
                        found.margin_type.clone(),
                    );
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            };
            let margin_type = margin_type.unwrap_or_else(|| "absent".to_owned());
            println!("held {held_amt} with marginType {margin_type}");
            if margin_type != "cross" {
                return Err(format!(
                    "probe requires a cross position, read {margin_type}"
                ));
            }

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
                .map_err(|error| format!("close order: {error}"))?;
            let close_frames = capture_frames(&mut stream, "close", |frames| {
                frames.iter().any(|frame| {
                    position_entries(frame, "BTCUSDT")
                        .iter()
                        .any(|entry| entry["pa"].as_str() == Some("0"))
                })
            })
            .await;
            if !close_frames.iter().any(|frame| {
                position_entries(frame, "BTCUSDT")
                    .iter()
                    .any(|entry| entry["pa"].as_str() == Some("0"))
            }) {
                return Err("no close-leg ACCOUNT_UPDATE observed".to_owned());
            }
            frames.extend(close_frames);
            Ok((frames, margin_type))
        }
        .await;

        // Owned cleanup runs on every exit, success or failure.
        usdm_owned_cleanup(&client, &symbol, dual, &mut stream).await;
        let (frames, margin_type) = capture.expect("demo capture round trip");

        write_fixture(
            "usdm",
            &serde_json::json!({
                "run": "issue-69-usdm-cross-margin-account-update",
                "observedAtUnixSeconds": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                "credentialed": true,
                "scrub": "No API key, secret, signed URL or listen key is recorded; the recorded USD-M ACCOUNT_UPDATE frames carry no account-identifier field.",
                "phaseAttribution": "phase labels capture-session attribution; a delayed venue push is attributed to the leg that observed it.",
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

    /// Best-effort, failure-tolerant flatten-and-close: reads the venue,
    /// closes any held position reduce-only, cancels stray orders, closes the
    /// listen key and the socket, and always prints the final held state.
    #[allow(
        clippy::too_many_lines,
        reason = "failure-tolerant cleanup must stay one sequential routine"
    )]
    async fn usdm_owned_cleanup(
        client: &binance_client::usdm::RestClient,
        symbol: &Symbol,
        dual: bool,
        stream: &mut DemoStream,
    ) {
        use binance_client::usdm::rest_requests::{
            CancelAllOpenOrders, CloseUserDataStream, CurrentAllOpenOrders, PositionInformationV2,
        };
        for attempt in 0..3 {
            let Ok(positions) = client
                .position_information_v2(
                    &PositionInformationV2::new()
                        .symbol(symbol.clone())
                        .build()
                        .unwrap(),
                    deadline(),
                )
                .await
            else {
                println!("cleanup: position read failed on attempt {attempt}");
                break;
            };
            let held: Vec<_> = positions
                .data
                .iter()
                .filter(|item| item.position_amt.unwrap_or(Decimal::ZERO) != Decimal::ZERO)
                .collect();
            if held.is_empty() {
                println!("cleanup: flat after {attempt} attempt(s)");
                break;
            }
            for item in held {
                let amount = item.position_amt.unwrap_or(Decimal::ZERO);
                let mut close = binance_client::usdm::rest_requests::NewOrder::new()
                    .symbol(symbol.clone())
                    .side(if amount > Decimal::ZERO {
                        "SELL"
                    } else {
                        "BUY"
                    })
                    .type_value("MARKET")
                    .quantity(amount.abs())
                    .reduce_only("true")
                    .new_client_order_id(
                        ClientOrderId::new(format!("issue69/usdm-clean{attempt}").as_str())
                            .unwrap(),
                    );
                if dual {
                    let Some(side) = item
                        .position_side
                        .as_ref()
                        .map(binance_client::usdm::enums::PositionSide::as_str)
                    else {
                        println!(
                            "cleanup: hedge position without a reported side; not auto-closed"
                        );
                        continue;
                    };
                    close = close.position_side(side);
                }
                match client.new_order(&close.build().unwrap(), deadline()).await {
                    Ok(_) => println!("cleanup: reduce-only close sent for {amount}"),
                    Err(error) => println!("cleanup: close refused: {error}"),
                }
            }
        }
        match client
            .cancel_all_open_orders(
                &CancelAllOpenOrders::new()
                    .symbol(symbol.clone())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
        {
            Ok(_) => println!("cleanup: no open orders remain"),
            Err(error) => println!("cleanup: cancel-all failed: {error}"),
        }
        match client
            .current_all_open_orders(
                &CurrentAllOpenOrders::new()
                    .symbol(symbol.clone())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
        {
            Ok(orders) => println!("cleanup: {} open order(s) reported", orders.data.len()),
            Err(error) => println!("cleanup: open-order read failed: {error}"),
        }
        match client
            .close_user_data_stream(&CloseUserDataStream::new().build().unwrap(), deadline())
            .await
        {
            Ok(_) => println!("cleanup: listen key closed"),
            Err(error) => println!("cleanup: listen-key close failed: {error}"),
        }
        let _ = stream.send(Message::close(None, "")).await;
    }

    /// COIN-M: one minimal `BTCUSD_PERP` cross-margin round trip on the demo
    /// environment, capturing the native `ACCOUNT_UPDATE` pushes for the
    /// inverse market's own schema.
    #[tokio::test]
    #[ignore = "authorized credentialed demo probe placing and closing one demo market position; never runs in normal CI"]
    #[allow(
        clippy::too_many_lines,
        reason = "one sequential authorized probe; splitting would obscure the capture-cleanup sequence"
    )]
    async fn demo_coinm_cross_margin_account_update_capture() {
        use binance_client::coinm::{
            Config, Environment, RestClient,
            rest_requests::{
                ChangeMarginType, GetCurrentPositionMode, NewOrder, PositionInformation,
                StartUserDataStream,
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

        // -4046 is the documented "No need to change margin type." reply
        // ([error code](https://developers.binance.com/en/docs/derivatives/coin-margined-futures/error-code)):
        // already cross. Any other refusal fails the probe before any order is
        // sent.
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

        let capture: Result<(Vec<CapturedFrame>, String), String> = async {
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
                .map_err(|error| format!("open order: {error}"))?;
            let mut frames = capture_frames(&mut stream, "open", |frames| {
                frames.iter().any(|frame| {
                    position_entries(frame, "BTCUSD_PERP")
                        .iter()
                        .any(|entry| entry["pa"].as_str().is_some_and(|pa| pa != "0"))
                })
            })
            .await;
            if !frames.iter().any(|frame| {
                position_entries(frame, "BTCUSD_PERP")
                    .iter()
                    .any(|entry| entry["pa"].as_str().is_some_and(|pa| pa != "0"))
            }) {
                return Err("no open-leg ACCOUNT_UPDATE observed".to_owned());
            }

            let (held_amt, margin_type) = loop {
                let positions = client
                    .position_information(
                        &PositionInformation::new()
                            .pair(Symbol::new("BTCUSD").unwrap())
                            .build()
                            .unwrap(),
                        deadline(),
                    )
                    .await
                    .map_err(|error| format!("position read: {error}"))?;
                if let Some(found) = positions
                    .data
                    .into_iter()
                    .filter(|item| {
                        item.symbol
                            .as_ref()
                            .is_some_and(|s| s.as_str() == "BTCUSD_PERP")
                    })
                    .find(|item| item.position_amt.unwrap_or(Decimal::ZERO) != Decimal::ZERO)
                {
                    break (
                        found.position_amt.unwrap_or(Decimal::ZERO),
                        found.margin_type,
                    );
                }
                tokio::time::sleep(Duration::from_secs(1)).await;
            };
            let margin_type = margin_type.unwrap_or_else(|| "absent".to_owned());
            println!("held {held_amt} with marginType {margin_type}");
            if margin_type != "cross" {
                return Err(format!(
                    "probe requires a cross position, read {margin_type}"
                ));
            }

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
                .map_err(|error| format!("close order: {error}"))?;
            let close_frames = capture_frames(&mut stream, "close", |frames| {
                frames.iter().any(|frame| {
                    position_entries(frame, "BTCUSD_PERP")
                        .iter()
                        .any(|entry| entry["pa"].as_str() == Some("0"))
                })
            })
            .await;
            if !close_frames.iter().any(|frame| {
                position_entries(frame, "BTCUSD_PERP")
                    .iter()
                    .any(|entry| entry["pa"].as_str() == Some("0"))
            }) {
                return Err("no close-leg ACCOUNT_UPDATE observed".to_owned());
            }
            frames.extend(close_frames);
            Ok((frames, margin_type))
        }
        .await;

        coinm_owned_cleanup(&client, &symbol, dual, &mut stream).await;
        let (frames, margin_type) = capture.expect("demo capture round trip");

        write_fixture(
            "coinm",
            &serde_json::json!({
                "run": "issue-69-coinm-cross-margin-account-update",
                "observedAtUnixSeconds": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                "credentialed": true,
                "scrub": "No API key, secret, signed URL or listen key is recorded. The venue-supplied account-alias field i was redacted before recording; every other byte is as captured.",
                "phaseAttribution": "phase labels capture-session attribution; a delayed venue push is attributed to the leg that observed it.",
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

    /// COIN-M counterpart of the unconditional flatten-and-close routine.
    #[allow(
        clippy::too_many_lines,
        reason = "failure-tolerant cleanup must stay one sequential routine"
    )]
    async fn coinm_owned_cleanup(
        client: &binance_client::coinm::RestClient,
        symbol: &Symbol,
        dual: bool,
        stream: &mut DemoStream,
    ) {
        use binance_client::coinm::rest_requests::{
            CancelAllOpenOrders, CloseUserDataStream, CurrentAllOpenOrders, PositionInformation,
        };
        for attempt in 0..3 {
            let Ok(positions) = client
                .position_information(
                    &PositionInformation::new()
                        .pair(Symbol::new("BTCUSD").unwrap())
                        .build()
                        .unwrap(),
                    deadline(),
                )
                .await
            else {
                println!("cleanup: position read failed on attempt {attempt}");
                break;
            };
            let held: Vec<_> = positions
                .data
                .iter()
                .filter(|item| {
                    item.symbol
                        .as_ref()
                        .is_some_and(|s| s.as_str() == symbol.as_str())
                        && item.position_amt.unwrap_or(Decimal::ZERO) != Decimal::ZERO
                })
                .collect();
            if held.is_empty() {
                println!("cleanup: flat after {attempt} attempt(s)");
                break;
            }
            for item in held {
                let amount = item.position_amt.unwrap_or(Decimal::ZERO);
                let mut close = binance_client::coinm::rest_requests::NewOrder::new()
                    .symbol(symbol.clone())
                    .side(if amount > Decimal::ZERO {
                        "SELL"
                    } else {
                        "BUY"
                    })
                    .type_value("MARKET")
                    .quantity(amount.abs())
                    .reduce_only("true")
                    .new_client_order_id(
                        ClientOrderId::new(format!("issue69/coinm-clean{attempt}").as_str())
                            .unwrap(),
                    );
                if dual {
                    let Some(side) = item
                        .position_side
                        .as_ref()
                        .map(binance_client::coinm::enums::PositionSide::as_str)
                    else {
                        println!(
                            "cleanup: hedge position without a reported side; not auto-closed"
                        );
                        continue;
                    };
                    close = close.position_side(side);
                }
                match client.new_order(&close.build().unwrap(), deadline()).await {
                    Ok(_) => println!("cleanup: reduce-only close sent for {amount}"),
                    Err(error) => println!("cleanup: close refused: {error}"),
                }
            }
        }
        match client
            .cancel_all_open_orders(
                &CancelAllOpenOrders::new()
                    .symbol(symbol.clone())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
        {
            Ok(_) => println!("cleanup: no open orders remain"),
            Err(error) => println!("cleanup: cancel-all failed: {error}"),
        }
        match client
            .current_all_open_orders(
                &CurrentAllOpenOrders::new()
                    .symbol(symbol.clone())
                    .build()
                    .unwrap(),
                deadline(),
            )
            .await
        {
            Ok(orders) => println!("cleanup: {} open order(s) reported", orders.data.len()),
            Err(error) => println!("cleanup: open-order read failed: {error}"),
        }
        match client
            .close_user_data_stream(&CloseUserDataStream::new().build().unwrap(), deadline())
            .await
        {
            Ok(_) => println!("cleanup: listen key closed"),
            Err(error) => println!("cleanup: listen-key close failed: {error}"),
        }
        let _ = stream.send(Message::close(None, "")).await;
    }
}
