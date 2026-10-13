// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! V2 deposit history sends one exact, signed `depositId` query value.
#![allow(
    clippy::unwrap_used,
    reason = "synthetic fixture assertions use fixed inputs"
)]
#[allow(dead_code, reason = "shared fixture helpers")]
mod support;

use binance_client::{Credentials, Error, WeightPools, wallet};
use std::sync::Arc;
use support::{FixedClock, HttpFixture, deadline};

fn client(url: &str) -> wallet::RestClient {
    let config = wallet::Config::with_pools(&WeightPools::new())
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)))
        .credentials(Credentials::hmac("synthetic-key", "synthetic-secret").unwrap())
        .rest_url(url)
        .unwrap();
    wallet::RestClient::new(config).unwrap()
}

fn deposit_id_values(wire: &str) -> Vec<String> {
    let target = wire.lines().next().unwrap().split(' ').nth(1).unwrap();
    let url = url::Url::parse(&format!("http://fixture{target}")).unwrap();
    url.query_pairs()
        .filter(|(key, _)| key == "depositId")
        .map(|(_, value)| value.into_owned())
        .collect()
}

#[tokio::test]
async fn single_id_setter_keeps_its_exact_signed_query_value() {
    let mut fixture = HttpFixture::new(200, "", "[]", None, false).await;
    let request = wallet::rest_requests::DepositHistoryV2::new()
        .deposit_id(4_615_328_107_052_018_945)
        .build()
        .unwrap();
    let response = client(&fixture.url)
        .deposit_history_v2(&request, deadline())
        .await
        .unwrap();
    assert!(response.data.is_empty());
    let wire = fixture.requests.recv().await.unwrap();
    assert!(wire.starts_with("GET /sapi/v2/localentity/deposit/history?"));
    assert!(
        wire.to_ascii_lowercase()
            .contains("x-mbx-apikey: synthetic-key")
    );
    assert!(wire.contains("signature="));
    assert_eq!(deposit_id_values(&wire), ["4615328107052018945".to_owned()]);
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn multiple_exact_ids_use_one_query_parameter_without_a_record_cap() {
    let mut fixture = HttpFixture::new(200, "", "[]", None, false).await;
    let ids = wallet::DepositIds::new(vec![
        "0001".to_owned(),
        "18446744073709551616".to_owned(),
        "0001".to_owned(),
    ])
    .unwrap();
    let request = wallet::rest_requests::DepositHistoryV2::new()
        .deposit_ids(ids)
        .build()
        .unwrap();
    let response = client(&fixture.url)
        .deposit_history_v2(&request, deadline())
        .await
        .unwrap();
    assert!(response.data.is_empty());
    let wire = fixture.requests.recv().await.unwrap();
    assert_eq!(
        deposit_id_values(&wire),
        ["0001,18446744073709551616,0001".to_owned()]
    );
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;

    let mut fixture = HttpFixture::new(200, "", "[]", None, false).await;
    let ids = wallet::DepositIds::new((0..46).map(|number| number.to_string()).collect()).unwrap();
    let request = wallet::rest_requests::DepositHistoryV2::new()
        .deposit_ids(ids)
        .build()
        .unwrap();
    client(&fixture.url)
        .deposit_history_v2(&request, deadline())
        .await
        .unwrap();
    let wire = fixture.requests.recv().await.unwrap();
    let values = deposit_id_values(&wire);
    assert_eq!(values.len(), 1);
    let parts = values[0].split(',').collect::<Vec<_>>();
    assert_eq!(parts.len(), 46);
    assert_eq!(parts.first(), Some(&"0"));
    assert_eq!(parts.last(), Some(&"45"));
    assert_eq!(fixture.connections_accepted(), 1);
    fixture.finish().await;
}

#[tokio::test]
async fn empty_or_delimiter_contaminated_ids_cannot_be_dispatched() {
    let fixture = HttpFixture::new(200, "", "[]", None, false).await;
    for ids in [
        vec![],
        vec![String::new()],
        vec!["first".to_owned(), String::new()],
        vec!["first,second".to_owned()],
        vec!["first\nsecond".to_owned()],
    ] {
        assert!(matches!(
            wallet::DepositIds::new(ids),
            Err(Error::Validation("deposit ID list"))
        ));
    }
    assert_eq!(fixture.connections_accepted(), 0);
    fixture.finish().await;
}
