// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Operator-invoked read-only probes. Normal CI never needs credentials or network.
//! Credentials come from the managed task bindings and never enter repository files.
//! SAPI Advanced Trading has no documented demo environment; demo keys are never
//! sent to production SAPI hosts. Only public Alpha data is queried in production.
use binance_client::{Credentials, Error, advanced_trading::alpha, spot, usdm};
use std::time::Duration;
fn credentials() -> Result<Credentials, Error> {
    let key = zeroize::Zeroizing::new(
        std::env::var("BINANCE_TEST_KEY").map_err(|_| Error::CredentialsRequired)?,
    );
    let secret = zeroize::Zeroizing::new(
        std::env::var("BINANCE_TEST_SECRET").map_err(|_| Error::CredentialsRequired)?,
    );
    Credentials::hmac(key.as_str(), secret.as_str())
}
fn proxy() -> Result<String, Error> {
    std::env::var("HTTPS_PROXY")
        .map_err(|_| Error::Configuration("managed probe requires an explicit HTTPS proxy"))
}
fn deadline() -> tokio::time::Instant {
    tokio::time::Instant::now() + Duration::from_secs(20)
}
#[tokio::test]
#[ignore = "explicitly authorized live read-only probe; never normal CI"]
async fn spot_demo_account_read_only() -> Result<(), Error> {
    let proxy =
        reqwest::Proxy::all(proxy()?).map_err(|_| Error::Configuration("explicit probe proxy"))?;
    let config = spot::Config::new(spot::Environment::Demo)?
        .credentials(credentials()?)
        .http_proxy(proxy);
    let result = spot::RestClient::new(config)?
        .get_account(&spot::rest_requests::GetAccount::new(), deadline())
        .await?;
    eprintln!("Spot demo signed account read: HTTP {}", result.meta.status);
    Ok(())
}
#[tokio::test]
#[ignore = "explicitly authorized live read-only probe; never normal CI"]
async fn usdm_demo_account_read_only() -> Result<(), Error> {
    let proxy =
        reqwest::Proxy::all(proxy()?).map_err(|_| Error::Configuration("explicit probe proxy"))?;
    let config = usdm::Config::new(usdm::Environment::Demo)?
        .credentials(credentials()?)
        .http_proxy(proxy);
    let result = usdm::RestClient::new(config)?
        .account_information_v3(
            &usdm::rest_requests::AccountInformationV3::new(),
            deadline(),
        )
        .await?;
    eprintln!(
        "USD-M demo signed account read: HTTP {}",
        result.meta.status
    );
    Ok(())
}
#[tokio::test]
#[ignore = "explicitly authorized public Alpha read; never normal CI"]
async fn alpha_public_exchange_information_read_only() -> Result<(), Error> {
    let config = alpha::Config::new()?.proxy(&proxy()?)?;
    let result = alpha::RestClient::new(config)?
        .get_exchange_info(&alpha::rest_requests::GetExchangeInfo::new(), deadline())
        .await?;
    eprintln!("Alpha public metadata read: HTTP {}", result.meta.status);
    Ok(())
}
