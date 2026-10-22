// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native Copy Trading bindings. Mutations are sent once; resolve uncertainty with venue reads.
//! The caller owns credentials, runtime, deadlines and recovery.

mod client;
pub mod event_payloads;
mod rate;
pub mod rest_models;
pub mod rest_requests;
pub(super) mod validation;

pub use client::{Config, Protocol, RestClient};
