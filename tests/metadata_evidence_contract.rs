// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Invalid exchange metadata retains safe response evidence without partial adoption.

#[cfg(test)]
#[allow(dead_code, reason = "shared synthetic fixture helpers")]
mod support;

use binance_client::{
    Error, ExitReserve, LimitSource, Outcome, TimeUnit, TransportFailure, WeightPools, coinm,
    options, spot, usdm,
};
use std::sync::Arc;
use support::{FixedClock, HttpFixture, deadline};

const BODY: &str = r#"{"serverTime":1700000001000,"symbols":[],"optionContracts":[],"optionAssets":[],"optionSymbols":[],"private":"synthetic-private-payload","rateLimits":[{"rateLimitType":"REQUEST_WEIGHT","interval":"MINUTE","intervalNum":1,"limit":100},{"rateLimitType":"ORDERS","interval":"SECOND","intervalNum":10,"limit":0}]}"#;

#[cfg(test)]
fn assert_evidence(error: &Error, operation: &str) {
    assert_eq!(error.outcome(), Some(Outcome::ReadFailed));
    let Error::Transport {
        operation: observed,
        failure: Some(TransportFailure::RateEvidenceFailed),
        meta: Some(meta),
        ..
    } = error
    else {
        panic!("missing typed received evidence: {error:?}");
    };
    assert_eq!(*observed, operation);
    assert_eq!(meta.operation, operation);
    assert_eq!(meta.status, 200);
    assert_eq!(meta.time_unit, TimeUnit::Milliseconds);
    assert!(meta.client_order_ids.is_empty());
    assert!(meta.rates.stated_limits_unusable);
    assert_eq!(meta.rates.counters["x-mbx-used-weight-1m"], 77);
    assert_eq!(meta.rates.counters["x-mbx-used-weight-9m"], 9);
    assert!(!format!("{error:?}").contains("synthetic-private-payload"));
}

macro_rules! metadata_evidence {
    ($name:ident, $product:ident, $request:ident, $method:ident, $operation:literal, $protected:literal, $native:literal) => {
        #[tokio::test]
        async fn $name() {
            let fixture = HttpFixture::new(200, "X-MBX-USED-WEIGHT-1M: 77\r\nX-MBX-USED-WEIGHT-9M: 9\r\nSet-Cookie: synthetic-private-header\r\n", BODY, None, false).await;
            let config = $product::Config::with_pools($product::Environment::Demo, &WeightPools::new()).unwrap()
                .rest_url(&fixture.url).unwrap().clock(Arc::new(FixedClock(1_700_000_001_000)));
            if $protected {
                config.set_exit_reserve(ExitReserve::default().request_weight(10_000)).unwrap();
            }
            let observer = config.clone();
            let before = observer.account_usage().unwrap();
            let client = $product::RestClient::new(config).unwrap();
            let error = client.$method(&$product::rest_requests::$request::new(), deadline()).await.unwrap_err();
            assert_evidence(&error, $operation);
            let usage = observer.pool_usage().unwrap().request_weight;
            assert_eq!(usage.limit, $native);
            assert_eq!(usage.source, LimitSource::Documented);
            assert_eq!(usage.used, 77);
            assert_eq!(observer.account_usage().unwrap(), before);
            assert!(!format!("{error:?}").contains("synthetic-private-header"));
            assert_eq!(fixture.connections_accepted(), 1);
            fixture.finish().await;
        }
    };
}

metadata_evidence!(
    spot_ordinary_invalid_metadata_retains_received_evidence,
    spot,
    ExchangeInfo,
    exchange_info,
    "exchangeInfo",
    false,
    6000
);
metadata_evidence!(
    spot_recovery_invalid_metadata_retains_received_evidence,
    spot,
    ExchangeInfo,
    exchange_info_for_recovery,
    "exchangeInfo",
    true,
    6000
);
metadata_evidence!(
    usdm_ordinary_invalid_metadata_retains_received_evidence,
    usdm,
    ExchangeInformation,
    exchange_information,
    "exchangeInformation",
    false,
    2400
);
metadata_evidence!(
    usdm_recovery_invalid_metadata_retains_received_evidence,
    usdm,
    ExchangeInformation,
    exchange_information_for_recovery,
    "exchangeInformation",
    true,
    2400
);
metadata_evidence!(
    coinm_ordinary_invalid_metadata_retains_received_evidence,
    coinm,
    ExchangeInformation,
    exchange_information,
    "exchangeInformation",
    false,
    2400
);
metadata_evidence!(
    coinm_recovery_invalid_metadata_retains_received_evidence,
    coinm,
    ExchangeInformation,
    exchange_information_for_recovery,
    "exchangeInformation",
    true,
    2400
);
metadata_evidence!(
    options_ordinary_invalid_metadata_retains_received_evidence,
    options,
    ExchangeInformation,
    exchange_information,
    "exchangeInformation",
    false,
    2400
);
