// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::{AccountBudgets, CompId, Precision, Role};
use crate::{Clock, Credentials, Decimal, Error, SystemClock};
use std::{sync::Arc, time::Duration};

/// Native request/response encoding negotiated by endpoint and initial Logon.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Encoding {
    /// FIX 4.4 ASCII requests and responses, port 9000.
    #[default]
    Ascii,
    /// ASCII requests, schema 1:1 SBE responses, port 9001.
    AsciiSbe,
    /// Schema 1:1 SBE requests and responses, port 9002.
    Sbe,
}
/// Caller-owned FIX configuration. No runtime or task is started by this builder.
#[derive(Clone)]
pub struct Config {
    pub(super) role: Role,
    pub(super) component: CompId,
    pub(super) credentials: Credentials,
    pub(super) budgets: AccountBudgets,
    pub(super) endpoint: url::Url,
    pub(super) encoding: Encoding,
    pub(super) heartbeat: Duration,
    pub(super) timeout: Duration,
    pub(super) clock: Arc<dyn Clock>,
    pub(super) window: Option<Decimal>,
    pub(super) precision: Vec<Precision>,
}
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Config")
            .field("role", &self.role)
            .field("component", &self.component)
            .field("encoding", &self.encoding)
            .finish_non_exhaustive()
    }
}
impl Config {
    /// Select production TLS with caller credentials and an explicit shared account owner.
    /// Sequential message handling preserves caller send order at the Matching Engine.
    ///
    /// # Errors
    /// Refuses non-Ed25519 credentials or invalid endpoint configuration.
    pub fn new(
        role: Role,
        component: CompId,
        credentials: Credentials,
        budgets: AccountBudgets,
    ) -> Result<Self, Error> {
        if !credentials.is_ed25519() {
            return Err(Error::Validation("FIX requires Ed25519 credentials"));
        }
        let host = match role {
            Role::OrderEntry => "fix-oe.binance.com",
            Role::DropCopy => "fix-dc.binance.com",
            Role::MarketData => "fix-md.binance.com",
        };
        Ok(Self {
            role,
            component,
            credentials,
            budgets,
            endpoint: url::Url::parse(&format!("tls://{host}:9000"))
                .map_err(|_| Error::Configuration("FIX endpoint"))?,
            encoding: Encoding::Ascii,
            heartbeat: Duration::from_secs(30),
            timeout: Duration::from_secs(10),
            clock: Arc::new(SystemClock),
            window: None,
            precision: vec![],
        })
    }
    /// Select documented encoding/production port. Call before a fixture endpoint override.
    #[must_use]
    pub fn encoding(mut self, encoding: Encoding) -> Self {
        self.encoding = encoding;
        let port = match encoding {
            Encoding::Ascii => 9000,
            Encoding::AsciiSbe => 9001,
            Encoding::Sbe => 9002,
        };
        let _ = self.endpoint.set_port(Some(port));
        self
    }
    /// Override TLS endpoint or an exact loopback TCP fixture. No ambient proxy is used.
    ///
    /// # Errors
    /// Refuses remote plaintext, credentials, paths, queries, fragments or absent port.
    pub fn endpoint(mut self, endpoint: &str) -> Result<Self, Error> {
        let url = url::Url::parse(endpoint).map_err(|_| Error::Configuration("FIX endpoint"))?;
        let fixture = matches!(
            url.host_str(),
            Some("localhost" | "127.0.0.1" | "[::1]" | "::1")
        );
        if (url.scheme() != "tls" && !(fixture && url.scheme() == "tcp"))
            || !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || !matches!(url.path(), "" | "/")
            || url.port().is_none()
            || url.host_str().is_none()
        {
            return Err(Error::Configuration(
                "FIX endpoint requires TLS or exact loopback TCP",
            ));
        }
        self.endpoint = url;
        Ok(self)
    }
    /// Negotiate the documented heartbeat interval in seconds.
    ///
    /// # Errors
    /// Refuses values outside 5 through 60 seconds.
    pub fn heartbeat(mut self, seconds: u8) -> Result<Self, Error> {
        if !(5..=60).contains(&seconds) {
            return Err(Error::Validation("FIX heartbeat interval"));
        }
        self.heartbeat = Duration::from_secs(u64::from(seconds));
        Ok(self)
    }
    /// Bound connection, queued send and each socket write. Cancellation prevents a queued send.
    ///
    /// # Errors
    /// Refuses a zero or unrepresentable timeout.
    pub fn timeout(mut self, timeout: Duration) -> Result<Self, Error> {
        if timeout.is_zero() || tokio::time::Instant::now().checked_add(timeout).is_none() {
            return Err(Error::Validation("FIX timeout"));
        }
        self.timeout = timeout;
        Ok(self)
    }
    /// Inject the request-signing and shared Spot rate clock.
    ///
    /// Clients sharing one [`WeightPools`](crate::WeightPools) registry must share one
    /// clock (the same `Arc<dyn Clock>`, or wall time for all): each charges the pool's
    /// fixed, aligned windows at the instant its own clock reports.
    #[must_use]
    pub fn clock(mut self, clock: Arc<dyn Clock>) -> Self {
        self.clock = clock;
        self
    }
    /// Apply native millisecond receive windows, with at most three decimal places.
    ///
    /// # Errors
    /// Refuses nonpositive, over-60000 or excessively precise windows without rounding.
    pub fn recv_window(mut self, window: Decimal) -> Result<Self, Error> {
        if window <= Decimal::ZERO
            || window > Decimal::from(60_000)
            || window.normalize().scale() > 3
        {
            return Err(Error::Validation("FIX receive window"));
        }
        self.window = Some(window);
        Ok(self)
    }
    /// Supply instrument-derived precision for binary financial requests.
    ///
    /// # Errors
    /// Refuses duplicate symbol metadata; no symbol spelling is used to infer precision.
    pub fn precision(mut self, precision: Vec<Precision>) -> Result<Self, Error> {
        let mut symbols = std::collections::BTreeSet::new();
        for p in &precision {
            if !symbols.insert(p.symbol().clone()) {
                return Err(Error::Validation("duplicate FIX precision"));
            }
        }
        self.precision = precision;
        Ok(self)
    }
}
