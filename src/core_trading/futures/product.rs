// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Native Futures transport facts and protocol validation boundaries.

/// Fixed transport facts implemented only by native Futures product markers.
pub trait Product: super::sealed::Sealed + Clone {
    /// REST, WebSocket API and stream endpoints in the demo environment.
    const DEMO: [&'static str; 3];
    /// REST, WebSocket API and stream endpoints in production.
    const PRODUCTION: [&'static str; 3];
    /// Apply native REST/WebSocket counter sharing to the supplied owner.
    fn configure_budgets(budgets: crate::Budgets) -> crate::Budgets;
}

/// Native WebSocket API payload and protocol validation boundary.
pub trait ApiProduct: Product {
    /// Product-native late-response body.
    type ApiPayload: std::fmt::Debug;
    /// Product-native authentication session receipt.
    type SessionStatus: serde::de::DeserializeOwned;
    /// Decode without converting linear and inverse account wire models.
    ///
    /// # Errors
    /// Returns malformed native response evidence.
    fn api_payload(name: &str, value: serde_json::Value) -> Result<Self::ApiPayload, crate::Error>;
    /// Check time-sensitive native request authority immediately before sending.
    ///
    /// # Errors
    /// Returns the product's validation refusal.
    fn validate_time(
        parameters: &std::collections::BTreeMap<String, serde_json::Value>,
        now: u64,
    ) -> Result<(), crate::Error>;
    /// Whether this product documents a refusal as definitive.
    fn definitive(status: u16, value: &serde_json::Value) -> bool;
}
