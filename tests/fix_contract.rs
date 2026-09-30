// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Provider-native FIX framing, groups, decimals, identity and session contracts.
#![allow(
    clippy::unwrap_used,
    clippy::panic,
    reason = "synthetic protocol assertions"
)]
use binance_client::{
    Decimal,
    spot::fix::{Role, Value, decode},
};

fn frame(body: &str) -> Vec<u8> {
    let body = body.replace('|', "\x01");
    let mut message = format!("8=FIX.4.4\x019={}\x01{body}", body.len()).into_bytes();
    let sum = message.iter().fold(0_u8, |sum, b| sum.wrapping_add(*b));
    message.extend_from_slice(format!("10={sum:03}\x01").as_bytes());
    message
}
#[test]
fn execution_report_keeps_decimal_quantities_and_fee_asset() {
    let bytes = frame(
        "35=8|49=SPOT|56=CLIENT|34=2|52=20260927-01:02:03.123456|11=owned_1|37=123|38=0.125|40=2|54=1|44=123.00000001|55=BTCUSDT|150=F|14=0.125|32=0.125|39=2|136=1|137=0.0000123|138=BNB|139=4|",
    );
    let m = decode(Role::OrderEntry, &bytes).unwrap();
    assert_eq!(m.header.sequence, 2);
    assert!(
        matches!(m.field("CumQty"),Some(Value::Quantity(v)) if *v == "0.125".parse::<Decimal>().unwrap())
    );
    let Some(Value::Group(fees)) = m.field("NoMiscFees") else {
        panic!("fees")
    };
    assert!(matches!(fees[0].field("MiscFeeCurr"),Some(Value::Asset(a)) if a.as_str()=="BNB"));
    assert!(
        matches!(fees[0].field("MiscFeeAmt"),Some(Value::Amount(v)) if *v=="0.0000123".parse::<Decimal>().unwrap())
    );
    let mut corrupt = bytes.clone();
    corrupt[30] ^= 1;
    assert!(decode(Role::OrderEntry, &corrupt).is_err());
    for n in 0..bytes.len() {
        assert!(decode(Role::OrderEntry, &bytes[..n]).is_err());
    }
}
/// A venue error code is never overruled by an accept-shaped status pair, and an
/// undocumented code is never read as either acceptance or a documented rejection.
#[test]
fn a_venue_error_code_is_never_overruled_by_an_accept_shaped_status() {
    use binance_client::Outcome;
    // ExecutionReport tag 150 carries ExecType, tag 39 carries OrdStatus.
    let report = |tail: &str| {
        let bytes = frame(&format!(
            "35=8|49=SPOT|56=CLIENT|34=2|52=20260927-01:02:03.123456|11=owned_1|55=BTCUSDT|40=2|54=1|44=0.125|60=20260927-01:02:03.123456|{tail}"
        ));
        decode(Role::OrderEntry, &bytes).unwrap()
    };
    let accepted_pair = "150=0|14=0.125|32=0.125|39=0|";
    // A definitive documented code stays a rejection whatever the status pair says.
    for code in ["-2011", "-2015", "-1100"] {
        let message = report(&format!("{accepted_pair}25016={code}|"));
        assert_eq!(
            message.outcome(),
            Outcome::Rejected,
            "{code} must not read as acceptance"
        );
        assert_eq!(
            message.field("ErrorCode"),
            Some(&Value::Integer(code.parse().unwrap()))
        );
    }
    // An undocumented or execution-unknown code is not promoted to a rejection.
    for code in ["-1007", "-999999", "7000"] {
        assert_eq!(
            report(&format!("{accepted_pair}25016={code}|")).outcome(),
            Outcome::Unknown,
            "{code}"
        );
    }
    // Without an error code the status pair is the only evidence.
    assert_eq!(report(accepted_pair).outcome(), Outcome::Accepted);
    assert_eq!(
        report("150=F|14=0.125|32=0.125|39=2|").outcome(),
        Outcome::Accepted
    );
    assert_eq!(report("150=8|14=0|32=0|39=8|").outcome(), Outcome::Rejected);
    // A non-ExecutionReport kind carries no execution evidence at all.
    let heartbeat = frame("35=0|49=SPOT|56=CLIENT|34=2|52=20260927-01:02:03.123456|112=probe|");
    assert_eq!(
        decode(Role::OrderEntry, &heartbeat).unwrap().outcome(),
        Outcome::Unknown
    );
}

#[test]
fn missing_required_and_malformed_money_never_become_defaults() {
    for suffix in ["150=0|14=bad|32=0|39=0|", "150=0|32=0|39=0|"] {
        let bytes = frame(&format!(
            "35=8|49=SPOT|56=CLIENT|34=2|52=20260927-01:02:03.123456|40=2|54=1|55=BTCUSDT|{suffix}"
        ));
        assert!(decode(Role::OrderEntry, &bytes).is_err());
    }
}

#[tokio::test]
async fn session_signs_once_services_control_and_preserves_reused_id_candidates() {
    use binance_client::{
        BudgetLimits, Budgets, Credentials, Error, RequestId, Signer,
        spot::fix::{AccountBudgets, CompId, Config, Event, Session},
    };
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };
    use tokio::net::TcpListener;
    struct Synthetic(Arc<Mutex<Vec<Vec<u8>>>>);
    impl Signer for Synthetic {
        fn sign(&self, payload: &[u8]) -> Result<String, Error> {
            self.0.lock().unwrap().push(payload.to_vec());
            Ok("synthetic-signature".into())
        }
    }
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("tcp://{}", listener.local_addr().unwrap());
    let (control_tx, control_rx) = tokio::sync::oneshot::channel();
    let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
    let server = tokio::spawn(session_peer(listener, control_tx, reply_rx));
    let signed = Arc::new(Mutex::new(vec![]));
    let credentials =
        Credentials::external_ed25519("synthetic-key", Arc::new(Synthetic(signed.clone())))
            .unwrap();
    let config = Config::new(
        Role::OrderEntry,
        CompId::new("CLIENT").unwrap(),
        credentials,
        AccountBudgets::new(Budgets::new(BudgetLimits::spot()).unwrap()),
    )
    .unwrap()
    .endpoint(&address)
    .unwrap()
    .heartbeat(5)
    .unwrap();
    let (session, mut events, driver) = Session::connect(config).await.unwrap();
    let owner = tokio::spawn(driver.run());
    let first = events.recv().await;
    assert!(
        matches!(first, Some(Event::Established { .. })),
        "{first:?}"
    );
    for local in ["attempt-1", "attempt-2"] {
        let order = limit_order("reused", "1.00000001");
        assert_eq!(
            session
                .send(RequestId::new(local).unwrap(), order)
                .await
                .unwrap()
                .outcome,
            binance_client::Outcome::Unknown
        );
    }
    tokio::time::timeout(Duration::from_secs(3), control_rx)
        .await
        .unwrap()
        .unwrap();
    assert!(events.queue_stats().unwrap().depth >= 1);
    reply_tx.send(()).unwrap();
    let mut receipt = false;
    let mut gap = false;
    let mut retired = false;
    while let Some(event) = events.recv().await {
        match event {
            Event::Message {
                message,
                candidates,
                ..
            } if message.kind.as_str() == "8" => {
                assert_eq!(candidates.len(), 2);
                assert_eq!(message.outcome(), binance_client::Outcome::Accepted);
                receipt = true;
            }
            Event::Gap { .. } => gap = true,
            Event::Retired(_) => retired = true,
            _ => (),
        }
    }
    assert!(receipt && gap && retired);
    owner.await.unwrap().unwrap();
    server.await.unwrap();
    let signatures = signed.lock().unwrap();
    assert_eq!(signatures.len(), 1);
    let text = std::str::from_utf8(&signatures[0]).unwrap();
    assert!(text.starts_with("A\x01CLIENT\x01SPOT\x011\x01"));
}

/// A receipt that names no caller identity is still attributable by the venue's
/// reference to the attempt's sequence number; that evidence path must not be dead.
#[tokio::test]
async fn a_receipt_without_a_caller_id_is_attributed_by_its_reference_sequence() {
    use binance_client::{
        BudgetLimits, Budgets, Credentials, RequestId,
        spot::fix::{AccountBudgets, CompId, Config, Event, Session},
    };
    use std::time::Duration;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("tcp://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        use tokio::io::AsyncWriteExt;
        let (mut stream, _) = listener.accept().await.unwrap();
        assert_eq!(
            decode(Role::OrderEntry, &read_fix(&mut stream).await)
                .unwrap()
                .kind
                .as_str(),
            "A"
        );
        stream
            .write_all(&frame("35=A|34=1|49=SPOT|56=CLIENT|52=20260927-01:02:03.000001|98=0|108=5|25037=synthetic-server|"))
            .await
            .unwrap();
        let request = decode(Role::OrderEntry, &read_fix(&mut stream).await).unwrap();
        assert_eq!(request.kind.as_str(), "D");
        // Tag 45 references the outbound sequence, and no client ID is returned.
        stream
            .write_all(&frame(&format!(
                "35=8|34=2|49=SPOT|56=CLIENT|52=20260927-01:02:03.000002|45={}|37=123|38=1|40=2|54=1|44=1.00000001|55=BTCUSDT|150=0|14=0|32=0|39=0|",
                request.header.sequence
            )))
            .await
            .unwrap();
        stream.shutdown().await.unwrap();
    });
    let credentials =
        Credentials::external_ed25519("synthetic-key", std::sync::Arc::new(FixedSigner)).unwrap();
    let config = Config::new(
        Role::OrderEntry,
        CompId::new("CLIENT").unwrap(),
        credentials,
        AccountBudgets::new(Budgets::new(BudgetLimits::spot()).unwrap()),
    )
    .unwrap()
    .endpoint(&address)
    .unwrap()
    .heartbeat(5)
    .unwrap();
    let (session, mut events, driver) = Session::connect(config).await.unwrap();
    let owner = tokio::spawn(driver.run());
    assert!(matches!(
        tokio::time::timeout(Duration::from_secs(5), events.recv())
            .await
            .expect("established"),
        Some(Event::Established { .. })
    ));
    assert_eq!(
        session
            .send(
                RequestId::new("attributed-1").unwrap(),
                limit_order("owned", "1.00000001")
            )
            .await
            .unwrap()
            .outcome,
        binance_client::Outcome::Unknown
    );
    let mut attributed = 0;
    while let Some(event) = tokio::time::timeout(Duration::from_secs(5), events.recv())
        .await
        .expect("session event")
    {
        if let Event::Message {
            message,
            candidates,
            ..
        } = event
            && message.kind.as_str() == "8"
        {
            assert_eq!(message.field("RefSeqNum"), Some(&Value::Integer(2)));
            attributed = candidates.len();
        }
    }
    assert_eq!(attributed, 1, "the receipt is attributed to its attempt");
    owner.await.unwrap().unwrap();
    server.await.unwrap();
}

/// A venue recovery request changes the expected inbound sequence. It is reported
/// as an explicit boundary instead of being silently ignored until a later
/// sequence mismatch is mistaken for the failure.
#[tokio::test]
async fn a_resend_request_is_reported_instead_of_ignored() {
    use binance_client::{
        BudgetLimits, Budgets, Credentials, Error,
        spot::fix::{AccountBudgets, CompId, Config, Event, Session},
    };
    use std::time::Duration;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("tcp://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        use tokio::io::AsyncWriteExt;
        let (mut stream, _) = listener.accept().await.unwrap();
        assert_eq!(
            decode(Role::OrderEntry, &read_fix(&mut stream).await)
                .unwrap()
                .kind
                .as_str(),
            "A"
        );
        stream
            .write_all(&frame("35=A|34=1|49=SPOT|56=CLIENT|52=20260927-01:02:03.000001|98=0|108=5|25037=synthetic-server|"))
            .await
            .unwrap();
        // The venue asks for a resend of what it believes it missed. This session
        // does not implement recovery, so the request must be reported.
        stream
            .write_all(&frame(
                "35=2|34=2|49=SPOT|56=CLIENT|52=20260927-01:02:03.000002|7=1|16=0|",
            ))
            .await
            .unwrap();
        stream.shutdown().await.unwrap();
    });
    let credentials =
        Credentials::external_ed25519("synthetic-key", std::sync::Arc::new(FixedSigner)).unwrap();
    let config = Config::new(
        Role::OrderEntry,
        CompId::new("CLIENT").unwrap(),
        credentials,
        AccountBudgets::new(Budgets::new(BudgetLimits::spot()).unwrap()),
    )
    .unwrap()
    .endpoint(&address)
    .unwrap()
    .heartbeat(5)
    .unwrap();
    let (_session, mut events, driver) = Session::connect(config).await.unwrap();
    let owner = tokio::spawn(driver.run());
    let mut delivered = 0;
    let mut reported = None;
    let mut retired = false;
    while let Some(event) = tokio::time::timeout(Duration::from_secs(5), events.recv())
        .await
        .expect("session event")
    {
        match event {
            // The request itself is still delivered as accepted ingress.
            Event::Message { message, .. } if message.kind.as_str() == "2" => delivered += 1,
            Event::Gap { error, .. } => reported = Some(error),
            Event::Retired(_) => retired = true,
            _ => (),
        }
    }
    assert_eq!(
        delivered, 1,
        "the recovery request is retained before it is reported"
    );
    assert!(
        matches!(
            reported,
            Some(Error::Gap("FIX ResendRequest is unsupported"))
        ),
        "{reported:?}"
    );
    assert!(retired, "the boundary is exposed by an explicit retirement");
    owner.await.unwrap().unwrap();
    server.await.unwrap();
}

fn limit_order(id: &str, price: &str) -> binance_client::spot::fix::Request {
    use binance_client::{
        Symbol,
        spot::fix::{ClientId, Request, RequestKind},
    };
    Request::builder(Role::OrderEntry, RequestKind::NewOrderSingle)
        .field("ClOrdID", Value::ClientId(ClientId::new(id).unwrap()))
        .unwrap()
        .field("Symbol", Value::Symbol(Symbol::new("BTCUSDT").unwrap()))
        .unwrap()
        .field("Side", Value::Code("1".into()))
        .unwrap()
        .field("OrdType", Value::Code("2".into()))
        .unwrap()
        .field("TimeInForce", Value::Code("1".into()))
        .unwrap()
        .field("OrderQty", Value::Quantity("0.125".parse().unwrap()))
        .unwrap()
        .field("Price", Value::Price(price.parse().unwrap()))
        .unwrap()
        .build()
        .unwrap()
}
#[test]
fn sbe_requests_roundtrip_exact_native_values_and_refuse_rounding() {
    use binance_client::{
        Symbol,
        spot::fix::{CompId, Fields, Header, Precision, Timestamp, decode_sbe},
    };
    let instrument = Fields::new(Role::MarketData)
        .with("Symbol", Value::Symbol(Symbol::new("BTCUSDT").unwrap()))
        .unwrap()
        .with(
            "MinPriceIncrement",
            Value::Price("0.00000001".parse().unwrap()),
        )
        .unwrap()
        .with("MinQtyIncrement", Value::Quantity("0.001".parse().unwrap()))
        .unwrap();
    let precision = Precision::from_instrument(&instrument).unwrap();
    let header = Header::request(
        CompId::new("CLIENT").unwrap(),
        2,
        Timestamp::from_micros(1_790_471_000_123_456).unwrap(),
    )
    .unwrap();
    let request = limit_order("exact", "123.00000001");
    let bytes = request
        .encode_sbe(&header, std::slice::from_ref(&precision))
        .unwrap();
    let message = decode_sbe(Role::OrderEntry, &bytes).unwrap();
    assert_eq!(message.header.sequence, 2);
    assert_eq!(message.header.sending_time, header.sending_time);
    assert!(message.header.sender.is_none() && message.header.target.is_none());
    assert_eq!(message.field("Price"), request.fields().get("Price"));
    assert_eq!(message.field("OrderQty"), request.fields().get("OrderQty"));
    assert!(
        limit_order("rounded", "123.000000001")
            .encode_sbe(&header, std::slice::from_ref(&precision))
            .is_err()
    );
    assert!(request.encode_sbe(&header, &[]).is_err());
    for n in 0..bytes.len() {
        assert!(decode_sbe(Role::OrderEntry, &bytes[..n]).is_err());
    }
}
#[test]
fn future_status_and_codes_never_prove_execution() {
    let root = "35=8|49=SPOT|56=CLIENT|34=2|52=20260927-01:02:03.123456|11=owned_1|40=2|54=1|55=BTCUSDT|150=0|14=0|32=0|";
    for suffix in ["39=Z|", "39=0|25016=-1007|", "39=0|25016=-999999|"] {
        let message = decode(Role::OrderEntry, &frame(&format!("{root}{suffix}"))).unwrap();
        assert_eq!(message.outcome(), binance_client::Outcome::Unknown);
    }
    for seq in ["+2", "-1", ""] {
        assert!(
            decode(
                Role::OrderEntry,
                &frame(&format!(
                    "35=0|34={seq}|49=SPOT|56=CLIENT|52=20260927-01:02:03.000000|"
                ))
            )
            .is_err()
        );
    }
}
#[test]
fn quote_spend_is_distinct_from_base_quantity_and_role_authority() {
    use binance_client::{
        Symbol,
        spot::fix::{ClientId, Request, RequestKind},
    };
    let market = || {
        Request::builder(Role::OrderEntry, RequestKind::NewOrderSingle)
            .field("ClOrdID", Value::ClientId(ClientId::new("quote").unwrap()))
            .unwrap()
            .field("Symbol", Value::Symbol(Symbol::new("BTCUSDT").unwrap()))
            .unwrap()
            .field("Side", Value::Code("1".into()))
            .unwrap()
            .field("OrdType", Value::Code("1".into()))
            .unwrap()
    };
    assert!(
        market()
            .field("CashOrderQty", Value::Quantity(Decimal::ONE))
            .is_err()
    );
    let quote = market()
        .field("CashOrderQty", Value::Amount("12.3456789".parse().unwrap()))
        .unwrap()
        .build()
        .unwrap();
    assert!(quote.fields().get("OrderQty").is_none());
    assert!(
        market()
            .field("CashOrderQty", Value::Amount(Decimal::ONE))
            .unwrap()
            .field("OrderQty", Value::Quantity(Decimal::ONE))
            .unwrap()
            .build()
            .is_err()
    );
    assert!(
        Request::builder(Role::DropCopy, RequestKind::NewOrderSingle)
            .build()
            .is_err()
    );
}

async fn read_fix(stream: &mut tokio::net::TcpStream) -> Vec<u8> {
    use tokio::io::AsyncReadExt;
    let mut bytes = vec![];
    loop {
        let mut b = [0_u8; 1];
        stream.read_exact(&mut b).await.unwrap();
        bytes.push(b[0]);
        if bytes.len() >= 7 && bytes[bytes.len() - 7..].starts_with(b"10=") && b[0] == 1 {
            return bytes;
        }
    }
}

async fn session_peer(
    listener: tokio::net::TcpListener,
    control_tx: tokio::sync::oneshot::Sender<()>,
    reply_rx: tokio::sync::oneshot::Receiver<()>,
) {
    use tokio::io::AsyncWriteExt;

    let (mut stream, _) = listener.accept().await.unwrap();
    let logon = decode(Role::OrderEntry, &read_fix(&mut stream).await).unwrap();
    assert_eq!(logon.kind.as_str(), "A");
    assert_eq!(logon.header.sequence, 1);
    assert_eq!(logon.field("HeartBtInt"), Some(&Value::Integer(5)));
    stream.write_all(&frame("35=A|34=1|49=SPOT|56=CLIENT|52=20260927-01:02:03.000001|98=0|108=5|25037=synthetic-server|")).await.unwrap();
    for seq in [2, 3] {
        let request = decode(Role::OrderEntry, &read_fix(&mut stream).await).unwrap();
        assert_eq!(request.kind.as_str(), "D");
        assert_eq!(request.header.sequence, seq);
    }
    stream
        .write_all(&frame(
            "35=1|34=2|49=SPOT|56=CLIENT|52=20260927-01:02:03.000002|112=probe-1|",
        ))
        .await
        .unwrap();
    let echo = decode(Role::OrderEntry, &read_fix(&mut stream).await).unwrap();
    assert_eq!(echo.kind.as_str(), "0");
    assert!(matches!(echo.field("TestReqID"),Some(Value::Id(v)) if v.as_str()=="probe-1"));
    control_tx.send(()).unwrap();
    reply_rx.await.unwrap();
    stream.write_all(&frame("35=8|34=3|49=SPOT|56=CLIENT|52=20260927-01:02:03.000003|11=reused|37=123|38=1|40=2|54=1|44=1.00000001|55=BTCUSDT|150=0|14=0|32=0|39=0|")).await.unwrap();
    stream.shutdown().await.unwrap();
}

struct FixedSigner;
impl binance_client::Signer for FixedSigner {
    fn sign(&self, _: &[u8]) -> Result<String, binance_client::Error> {
        Ok("synthetic-signature".into())
    }
}
fn native_config(role: Role, endpoint: &str) -> binance_client::spot::fix::Config {
    use binance_client::{
        BudgetLimits, Budgets, Credentials,
        spot::fix::{AccountBudgets, CompId, Config},
    };
    Config::new(
        role,
        CompId::new("CLIENT").unwrap(),
        Credentials::external_ed25519("synthetic-key", std::sync::Arc::new(FixedSigner)).unwrap(),
        AccountBudgets::new(Budgets::new(BudgetLimits::spot()).unwrap()),
    )
    .unwrap()
    .heartbeat(5)
    .unwrap()
    .endpoint(endpoint)
    .unwrap()
}
async fn read_sbe(stream: &mut tokio::net::TcpStream) -> Vec<u8> {
    use tokio::io::AsyncReadExt;
    let length = stream.read_u32_le().await.unwrap();
    let mut bytes = length.to_le_bytes().to_vec();
    bytes.resize(usize::try_from(length).unwrap(), 0);
    stream.read_exact(&mut bytes[4..]).await.unwrap();
    bytes
}
fn binary_logon_ack() -> Vec<u8> {
    let mut b = vec![0; 6];
    for v in [6_u16, 20_009, 1, 1] {
        b.extend_from_slice(&v.to_le_bytes());
    }
    b.extend_from_slice(&1_u32.to_le_bytes());
    b.extend_from_slice(&1_790_471_000_123_456_i64.to_le_bytes());
    b.push(0);
    b.extend_from_slice(&5_u32.to_le_bytes());
    b.push(0);
    b.push(16);
    b.extend_from_slice(b"synthetic-server");
    let len = u32::try_from(b.len()).unwrap();
    b[..4].copy_from_slice(&len.to_le_bytes());
    b[4..6].copy_from_slice(&0xeb50_u16.to_le_bytes());
    b
}

#[test]
fn binary_market_subscription_preserves_symbols_in_zero_fixed_width_entries() {
    use binance_client::spot::fix::decode_sbe;
    // Official FIX schema 1:1, template 202: RelatedSym contains variable
    // Symbol data only. MDEntryTypes has a one-byte fixed block.
    let mut b = vec![0; 6];
    for value in [4_u16, 202, 1, 1] {
        b.extend_from_slice(&value.to_le_bytes());
    }
    b.extend_from_slice(&2_u32.to_le_bytes());
    b.extend_from_slice(&1_790_471_000_123_456_i64.to_le_bytes());
    b.push(b'1'); // subscribe
    b.extend_from_slice(&1_u16.to_le_bytes());
    b.push(1); // aggregated book
    b.extend_from_slice(&0_u16.to_le_bytes());
    b.extend_from_slice(&1_u16.to_le_bytes());
    b.push(7);
    b.extend_from_slice(b"BTCUSDT");
    b.push(1); // MDEntryTypes fixed block width
    b.push(2); // two MDEntryTypes
    b.extend_from_slice(b"01");
    b.push(6);
    b.extend_from_slice(b"book-1");
    let len = u32::try_from(b.len()).unwrap();
    b[..4].copy_from_slice(&len.to_le_bytes());
    b[4..6].copy_from_slice(&0xeb50_u16.to_le_bytes());
    let message = decode_sbe(Role::MarketData, &b).unwrap();
    let Some(Value::Group(symbols)) = message.field("NoRelatedSym") else {
        panic!("missing symbol group");
    };
    assert!(
        matches!(symbols[0].get("Symbol"), Some(Value::Symbol(symbol)) if symbol.as_str() == "BTCUSDT")
    );
}
#[tokio::test]
async fn both_binary_session_modes_authenticate_and_logout_without_invented_header_ids() {
    use binance_client::spot::fix::{Encoding, Event, Session, decode_sbe};
    use tokio::{io::AsyncWriteExt, net::TcpListener};
    for mode in [Encoding::AsciiSbe, Encoding::Sbe] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("tcp://{}", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let logon = if mode == Encoding::Sbe {
                decode_sbe(Role::OrderEntry, &read_sbe(&mut stream).await).unwrap()
            } else {
                decode(Role::OrderEntry, &read_fix(&mut stream).await).unwrap()
            };
            assert_eq!(logon.kind.as_str(), "A");
            if mode == Encoding::AsciiSbe {
                assert_eq!(logon.field("SBESchemaID"), Some(&Value::Integer(1)));
            }
            let ack = binary_logon_ack();
            stream.write_all(&ack[..7]).await.unwrap();
            stream.write_all(&ack[7..]).await.unwrap();
            let logout = if mode == Encoding::Sbe {
                decode_sbe(Role::OrderEntry, &read_sbe(&mut stream).await).unwrap()
            } else {
                decode(Role::OrderEntry, &read_fix(&mut stream).await).unwrap()
            };
            assert_eq!(logout.kind.as_str(), "5");
            stream.shutdown().await.unwrap();
        });
        let config = native_config(Role::OrderEntry, &endpoint)
            .encoding(mode)
            .endpoint(&endpoint)
            .unwrap();
        let (session, mut events, driver) = Session::connect(config).await.unwrap();
        let owner = tokio::spawn(driver.run());
        let first = events.recv().await;
        let Some(Event::Established {
            component,
            acknowledgment,
            ..
        }) = first
        else {
            panic!("{first:?}")
        };
        assert_eq!(component.as_str(), "CLIENT");
        assert!(acknowledgment.header.sender.is_none() && acknowledgment.header.target.is_none());
        assert_eq!(
            acknowledgment.header.sending_time.micros(),
            1_790_471_000_123_456
        );
        session.shutdown();
        assert!(matches!(events.recv().await, Some(Event::Retired(_))));
        owner.await.unwrap().unwrap();
        server.await.unwrap();
    }
}
