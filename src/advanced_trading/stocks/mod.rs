// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native Stocks Trading bindings. Mutations are sent once; resolve uncertainty with venue reads.
//! The caller owns credentials, runtime, deadlines and recovery.

mod client;
pub mod event_payloads;
mod identity;
mod rate;
pub mod rest_models;
pub mod rest_requests;
pub mod stream_models;
mod stream_names;
pub(super) mod validation;
mod wire;

pub use client::{Config, Protocol, RestClient, Stream, Streams};
pub use identity::ClientOrderId;
