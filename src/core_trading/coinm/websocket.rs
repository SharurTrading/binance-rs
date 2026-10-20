// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

pub use crate::core_trading::futures::ConnectionDriver;
/// Native WebSocket API handle; clones share generation and budgets.
pub type WsClient = crate::core_trading::futures::WsClient<super::Protocol>;
/// Native lifecycle and late-response evidence.
pub type ApiEvent = crate::core_trading::futures::ApiEvent<super::Protocol>;
/// Source-ordered native lifecycle and late-response consumer.
pub type ApiEvents = crate::core_trading::futures::ApiEvents<super::Protocol>;

impl crate::core_trading::futures::ApiProduct for super::Protocol {
    type ApiPayload = super::event_payloads::ApiPayload;
    type SessionStatus = super::event_payloads::SessionStatus;
    fn api_payload(name: &str, value: serde_json::Value) -> Result<Self::ApiPayload, crate::Error> {
        super::event_payloads::api_payload(name, value)
    }
    fn validate_time(
        parameters: &std::collections::BTreeMap<String, serde_json::Value>,
        now: u64,
    ) -> Result<(), crate::Error> {
        super::validation::validate_time(parameters, now)
    }
    fn definitive(status: u16, value: &serde_json::Value) -> bool {
        super::validation::definitive(status, value)
    }
}
