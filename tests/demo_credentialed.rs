// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicitly invoked, credentialed demo probes for unpublished quota facts.
//!
//! These probes never run in normal CI (`--ignored` only) and require the
//! operator's explicit authorization (BN-VALIDATE-01). Credentials are read
//! from the environment (`TEST_KEY`, `TEST_SECRET`); they are never logged,
//! and neither are signed URLs or authentication payloads. Only response
//! status and rate-counter evidence is reported.

#[cfg(test)]
mod tests {
    use binance_client::usdm::{
        Config, Environment, RestClient,
        rest_requests::{CheckServerTime, TestOrder},
    };
    use binance_client::{ClientOrderId, Credentials, Decimal, Symbol};
    use std::time::Duration;
    fn deadline() -> tokio::time::Instant {
        tokio::time::Instant::now() + Duration::from_secs(15)
    }
    fn credentials() -> Credentials {
        let key = std::env::var("TEST_KEY").expect("TEST_KEY set for the authorized probe");
        let secret = std::env::var("TEST_SECRET").expect("TEST_SECRET set");
        Credentials::hmac(&key, &secret).expect("valid demo credentials")
    }

    /// Measures the actual demo quota consumption of one validation-only test
    /// order: the order-count headers on its own response and the IP weight
    /// delta between surrounding public reads. Issue #4 evidence; the
    /// validation-only order never reaches the matching engine.
    #[tokio::test]
    #[ignore = "authorized credentialed demo probe; never runs in normal CI"]
    async fn demo_test_order_reports_actual_quota_counters() {
        let client = RestClient::new(
            Config::new(Environment::Demo)
                .unwrap()
                .credentials(credentials()),
        )
        .unwrap();
        let probe = TestOrder::new()
            .symbol(Symbol::new("BTCUSDT").unwrap())
            .side("BUY")
            .type_value("MARKET")
            .quantity(Decimal::new(2, 3))
            .new_client_order_id(ClientOrderId::new("quota-probe/test").unwrap());
        let before = client
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap();
        println!("before: {:?}", before.meta.rates);
        let result = client.test_order(&probe, deadline()).await;
        let evidence = match &result {
            Ok(response) => {
                println!("test order accepted; counters: {:?}", response.meta.rates);
                Some(response.meta.rates.clone())
            }
            Err(error) => {
                let rates = match error {
                    binance_client::Error::Venue(venue) => Some(venue.rates.clone()),
                    binance_client::Error::Transport {
                        meta: Some(meta), ..
                    } => Some(meta.rates.clone()),
                    _ => None,
                };
                println!("test order body refused locally; counters: {rates:?}");
                rates
            }
        };
        let after = client
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap();
        println!("after: {:?}", after.meta.rates);
        // The venue reports the order-limit counters even when the
        // acknowledgment body cannot decode as order evidence.
        assert!(evidence.is_some_and(|rates| {
            rates.counters.contains_key("x-mbx-order-count-10s")
                && rates.counters.contains_key("x-mbx-order-count-1m")
        }));
    }
}
