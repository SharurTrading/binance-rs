// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native Algo Trading bindings. Mutations are sent once; resolve uncertainty with venue reads.
//! The caller owns credentials, runtime, deadlines and recovery.

pub mod event_payloads;
mod rate;
pub mod rest_models;
pub mod rest_requests;
pub(super) mod validation;
mod wire {
    pub(crate) use crate::advanced_trading::wire::*;
}

/// Fixed Algo Trading protocol marker; not caller-implementable.
#[derive(Clone)]
pub enum Protocol {}
impl super::config::sealed::Sealed for Protocol {}
impl super::config::Product for Protocol {
    const REST: &'static str = "https://api.binance.com";
    const STREAMS: Option<&'static str> = None;
    const POOL: crate::core::VenuePool = crate::core::VenuePool::Sapi;
}
/// Explicit production endpoints and shared services for Algo Trading.
pub type Config = super::config::Config<Protocol>;

/// REST transport whose clones share native configuration and quota ownership.
pub type RestClient = super::config::RestClient<Protocol>;

mod identity;
pub use identity::ClientAlgoId;
