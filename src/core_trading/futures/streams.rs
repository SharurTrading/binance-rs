// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Shared futures transport ownership and live subscription control.

use super::{Config, Product};
use crate::Error;
use crate::core::Socket;
use crate::core::control::{self, Membership};
use crate::core::socket::{SocketDriver, SocketEvents, SocketPolicy};
use tokio::time::Instant;

pub(crate) struct StreamSocket {
    pub(crate) socket: Socket,
    pub(crate) events: SocketEvents,
    pub(crate) membership: Membership,
    pub(crate) private: bool,
}

impl StreamSocket {
    pub(crate) async fn connect<P: Product>(
        config: Config<P>,
        url: url::Url,
        membership: Option<Membership>,
    ) -> Result<(Self, SocketDriver), Error> {
        let private = membership.is_none();
        let (socket, events, driver) = Socket::connect_with_policy(
            url,
            None,
            config.clock,
            config.budgets,
            config.timeout,
            // Both pinned futures stream contracts count pongs and JSON control
            // against ten incoming messages per second, and expire at 24 hours
            // (verified 2026-10-10). Product-native routes and models stay local.
            // https://developers.binance.com/docs/derivatives/usds-margined-futures/websocket-market-streams
            // https://developers.binance.com/docs/derivatives/coin-margined-futures/websocket-market-streams
            SocketPolicy {
                handshake: crate::core::Cost::default(),
                incoming_limit: 10,
                time_unit: crate::TimeUnit::Milliseconds,
                binary_decoder: None,
                api_key_header: false,
                max_generation_age: Some(std::time::Duration::from_hours(24)),
            },
        )
        .await?;
        Ok((
            Self {
                socket,
                events,
                membership: membership.unwrap_or_default(),
                private,
            },
            driver,
        ))
    }

    pub(crate) fn market_socket(&self) -> Result<(), Error> {
        if self.private {
            return Err(Error::Validation(
                "user-data socket has no stream membership",
            ));
        }
        Ok(())
    }

    pub(crate) async fn subscribe(
        &mut self,
        streams: &[(&str, &'static str)],
        deadline: Instant,
    ) -> Result<(), Error> {
        self.market_socket()?;
        control::subscribe(&self.socket, &mut self.membership, streams, deadline).await
    }

    pub(crate) async fn unsubscribe(
        &mut self,
        streams: &[&str],
        deadline: Instant,
    ) -> Result<(), Error> {
        self.market_socket()?;
        control::unsubscribe(&self.socket, &mut self.membership, streams, deadline).await
    }

    pub(crate) async fn list(&self, deadline: Instant) -> Result<Vec<String>, Error> {
        self.market_socket()?;
        control::list(&self.socket, deadline).await
    }
}
