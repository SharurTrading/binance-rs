// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

/// Fixed USDⓈ-M native protocol marker; callers cannot implement product facts.
#[derive(Clone, Debug)]
pub enum Protocol {}
impl crate::core_trading::futures::sealed::Sealed for Protocol {}
impl crate::core_trading::futures::Product for Protocol {
    const DEMO: [&'static str; 3] = [
        "https://demo-fapi.binance.com",
        "wss://testnet.binancefuture.com/ws-fapi/v1",
        "wss://demo-fstream.binance.com",
    ];
    const PRODUCTION: [&'static str; 3] = [
        "https://fapi.binance.com",
        "wss://ws-fapi.binance.com/ws-fapi/v1",
        "wss://fstream.binance.com",
    ];
    fn configure_budgets(budgets: crate::Budgets) -> crate::Budgets {
        budgets
    }
}
/// Explicit USDⓈ-M endpoints and shared services.
pub type Config = crate::core_trading::futures::Config<Protocol>;
pub use crate::core_trading::futures::Environment;
