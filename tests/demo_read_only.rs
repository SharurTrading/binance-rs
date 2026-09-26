// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Explicitly invoked, credential-free, read-only Futures demo probes.

#[cfg(test)]
mod tests {
    use binance_client::Symbol;
    use binance_client::usdm::{
        Config, Environment, RestClient, Stream, StreamEvent, Streams, WsClient,
        rest_requests::{CheckServerTime, ExchangeInformation, TestConnectivity},
        ws_requests::OrderBook,
    };
    use std::time::Duration;
    fn deadline() -> tokio::time::Instant {
        tokio::time::Instant::now() + Duration::from_secs(15)
    }

    #[tokio::test]
    #[ignore = "explicit read-only network probe; never runs in normal CI"]
    async fn demo_rest_public_metadata() {
        let client = RestClient::new(Config::new(Environment::Demo).unwrap()).unwrap();
        client
            .test_connectivity(&TestConnectivity::new(), deadline())
            .await
            .unwrap();
        let time = client
            .check_server_time(&CheckServerTime::new(), deadline())
            .await
            .unwrap();
        assert!(time.data.server_time.is_some());
        let info = client
            .exchange_information(&ExchangeInformation::new(), deadline())
            .await
            .unwrap();
        assert!(
            info.data
                .symbols
                .unwrap()
                .iter()
                .any(|s| s.symbol.as_ref().is_some_and(|s| s.as_str() == "BTCUSDT"))
        );
    }

    #[tokio::test]
    #[ignore = "explicit read-only network probe; never runs in normal CI"]
    async fn demo_websocket_public_depth() {
        let (client, mut events, driver) =
            WsClient::connect(Config::new(Environment::Demo).unwrap())
                .await
                .unwrap();
        let task = tokio::spawn(driver.run());
        events.recv().await.unwrap();
        let result = client
            .order_book(
                &OrderBook::new()
                    .symbol(Symbol::new("BTCUSDT").unwrap())
                    .limit(5),
                binance_client::RequestId::new("read-only-depth").unwrap(),
                deadline(),
            )
            .await;
        client.close().await.unwrap();
        while events.recv().await.is_some() {}
        task.await.unwrap().unwrap();
        assert!(result.unwrap().data.last_update_id.is_some());
    }

    #[tokio::test]
    #[ignore = "explicit read-only network probe; never runs in normal CI"]
    async fn demo_routed_market_depth() {
        let stream =
            Stream::diff_book_depth_streams(&Symbol::new("BTCUSDT").unwrap(), "100ms").unwrap();
        let (mut streams, driver) =
            Streams::connect(Config::new(Environment::Demo).unwrap(), &[stream])
                .await
                .unwrap();
        let task = tokio::spawn(driver.run());
        streams.recv().await.unwrap();
        let result = tokio::time::timeout(Duration::from_secs(15), streams.recv()).await;
        let _ = streams.close().await;
        while streams.recv().await.is_some() {}
        task.await.unwrap().unwrap();
        assert!(matches!(result.unwrap(), Some(StreamEvent::Data { .. })));
    }
}
