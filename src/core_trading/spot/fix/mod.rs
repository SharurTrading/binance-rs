// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native Spot FIX 4.4 and FIX SBE. Sessions do not replay mutations or assert
//! continuity across connections. Official contracts: `docs/core-protocols.md`.

mod binary;
mod budgets;
mod config;
mod session;
pub use budgets::AccountBudgets;
pub use config::{Config, Encoding};
pub use session::{Attempt, Event, Events, SendFailure, Session, SessionDriver};
mod codec;
mod identity;
mod request;
pub use binary::Precision;
pub use codec::{Field, Fields, Header, Message, Role, Value, decode, decode_sbe};
pub use identity::{ClientId, CompId, Timestamp, WireId};
pub use request::{Request, RequestBuilder, RequestKind};
