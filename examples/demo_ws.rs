// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Credential-free read-only demo WebSocket request with caller-owned teardown.

use binance_client::usdm::{Config, Environment, WsClient, ws_requests::OrderBook};
use binance_client::{Error, RequestId, Symbol};
use std::time::Duration;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Error> {
    let request = OrderBook::new()
        .symbol(Symbol::new("BTCUSDT")?)
        .limit(5)
        .build()?;
    let id = RequestId::new("public-demo-depth")?;
    let (client, mut events, driver) = WsClient::connect(Config::new(Environment::Demo)?).await?;
    let task = tokio::spawn(driver.run());
    let response = client
        .order_book(
            &request,
            id,
            tokio::time::Instant::now() + Duration::from_secs(10),
        )
        .await;
    let close = client.close().await;
    while events.recv().await.is_some() {}
    task.await.map_err(|_| Error::Task)??;
    close?;
    let response = response?;
    println!("Depth response status: {}", response.meta.status);
    Ok(())
}
