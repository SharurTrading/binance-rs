// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native REST transport and shared client configuration.

use super::Protocol;
use crate::core_trading::futures;

/// Cloneable REST client sharing native transport pools and venue budgets.
pub type RestClient = futures::RestClient<Protocol>;
