// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

pub(crate) mod control;
mod credentials;
pub(crate) mod error;
pub(crate) mod http;
mod identity;
pub(crate) mod rate;
pub(crate) mod request;
pub(crate) mod sapi;
pub(crate) mod socket;
mod time;

pub use control::StreamControl;
pub use credentials::{Credentials, Signer};
pub use error::{
    Error, OperationLeg, Outcome, PartialOperation, RateEvidence, Response, ResponseMeta,
    VenueFailure,
};
pub(crate) use http::HttpClient;
pub use identity::{Asset, ClientOrderId, RequestId, SensitiveString, Symbol};
pub use rate::{
    BudgetLimits, Budgets, LimitSource, PoolEnvironment, PoolKey, PoolUsage, VenuePool,
    WeightPools, WindowUsage,
};
pub(crate) use rate::{Cost, StatedLimit};
pub(crate) use request::{Operation, Request, Security};
pub(crate) use request::{parameters, validate_parameters};
pub(crate) use socket::{Socket, SocketEvent};
pub(crate) use time::validate_url;
pub use time::{Clock, SystemClock, TimeUnit};
