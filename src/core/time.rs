// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::Error;
use std::time::{SystemTime, UNIX_EPOCH};

/// Explicit units for provider timestamps; receive windows remain milliseconds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum TimeUnit {
    /// Binance's default JSON timestamp units.
    #[default]
    Milliseconds,
    /// Explicit Spot microsecond timestamp mode.
    Microseconds,
}
impl TimeUnit {
    pub(crate) fn timestamp(self, clock: &dyn Clock) -> Result<u64, Error> {
        match self {
            Self::Milliseconds => clock.now_millis(),
            Self::Microseconds => clock.now_micros(),
        }
    }
}

/// Time source for request signing and aligned venue budget windows.
pub trait Clock: Send + Sync {
    /// UTC milliseconds since the Unix epoch.
    ///
    /// # Errors
    /// Return an error when a valid venue timestamp cannot be produced.
    fn now_millis(&self) -> Result<u64, Error>;
    /// UTC microseconds. The default preserves a millisecond clock's resolution.
    ///
    /// # Errors
    /// Returns the clock error or checked conversion overflow.
    fn now_micros(&self) -> Result<u64, Error> {
        self.now_millis()?
            .checked_mul(1000)
            .ok_or(Error::Configuration("clock overflow"))
    }
}

/// Wall-clock implementation; deterministic tests inject their own clock.
#[derive(Debug, Default)]
pub struct SystemClock;
impl Clock for SystemClock {
    fn now_micros(&self) -> Result<u64, Error> {
        let elapsed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::Configuration("clock before Unix epoch"))?;
        u64::try_from(elapsed.as_micros()).map_err(|_| Error::Configuration("clock overflow"))
    }
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
