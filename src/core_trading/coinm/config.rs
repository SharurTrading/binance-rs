// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

/// Fixed COIN-M native protocol marker; callers cannot implement product facts.
#[derive(Clone, Debug)]
pub enum Protocol {}
impl crate::core_trading::futures::sealed::Sealed for Protocol {}
impl crate::core_trading::futures::Product for Protocol {
    const DEMO: [&'static str; 3] = [
        "https://demo-dapi.binance.com",
        "wss://testnet.binancefuture.com/ws-dapi/v1",
        "wss://demo-dstream.binance.com",
    ];
    const PRODUCTION: [&'static str; 3] = [
        "https://dapi.binance.com",
        "wss://ws-dapi.binance.com/ws-dapi/v1",
        "wss://dstream.binance.com",
    ];
    fn configure_budgets(budgets: crate::Budgets) -> crate::Budgets {
        budgets.websocket_weight_on_rest()
    }
}
/// Explicit COIN-M endpoints and shared services.
pub type Config = crate::core_trading::futures::Config<Protocol>;
pub use crate::core_trading::futures::Environment;
