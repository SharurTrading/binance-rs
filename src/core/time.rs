// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::Error;
use std::time::{SystemTime, UNIX_EPOCH};

/// Time source for request signing and aligned venue budget windows.
pub trait Clock: Send + Sync {
    /// UTC milliseconds since the Unix epoch.
    ///
    /// # Errors
    /// Return an error when a valid venue timestamp cannot be produced.
    fn now_millis(&self) -> Result<u64, Error>;
}

/// Wall-clock implementation; deterministic tests inject their own clock.
#[derive(Debug, Default)]
pub struct SystemClock;
impl Clock for SystemClock {
    fn now_millis(&self) -> Result<u64, Error> {
        let elapsed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Configuration("clock before Unix epoch"))?;
        u64::try_from(elapsed.as_millis()).map_err(|_| Error::Configuration("clock overflow"))
    }
}

pub(crate) fn validate_url(value: &str, websocket: bool) -> Result<url::Url, Error> {
    let url = url::Url::parse(value).map_err(|_| Error::Configuration("invalid endpoint URL"))?;
    let secure = if websocket { "wss" } else { "https" };
    let fixture = if websocket { "ws" } else { "http" };
    let loopback = matches!(
        url.host_str(),
        Some("localhost" | "127.0.0.1" | "[::1]" | "::1")
    );
    if (url.scheme() != secure && !(loopback && url.scheme() == fixture))
        || !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
        || url.host_str().is_none()
    {
        return Err(Error::Configuration(
            "endpoint requires TLS without URL credentials, query, or fragment",
        ));
    }
    Ok(url)
}
