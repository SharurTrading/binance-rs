// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

// COIN-M operation costs are product policy; the shared core only admits/observes budgets.
use crate::{Error, core::Cost};
use std::collections::BTreeMap;

pub(crate) fn cost(
    op: crate::core::Operation,
    p: &BTreeMap<String, serde_json::Value>,
) -> Result<Cost, Error> {
    let mut c = Cost {
        weight: weight(op, p)?,
        ..Cost::default()
    };
    // A conditional algo placement consumes the same order slots as a plain
    // order: the demo probe of 2026-10-03 observed one slot on each of
    // X-MBX-ORDER-COUNT-10S/-1M per accepted algoOrder (issue #11).
    if matches!(op.name, "newOrder" | "newAlgoOrder" | "modifyOrder") {
        c.orders10 = 1;
        c.orders60 = 1;
    }
    if matches!(op.name, "placeMultipleOrders" | "modifyMultipleOrders") {
        c.orders10 = 5;
        c.orders60 = 1;
    }

    c.download = match op.name {
        "getDownloadIdForFuturesOrderHistory" => 4,
        "getDownloadIdForFuturesTradeHistory" => 5,
        "getDownloadIdForFuturesTransactionHistory" => 6,
        _ => 0,
    };
    if !op.path.starts_with("/dapi/") && !op.path.starts_with("/futures/") {
        c.ws_weight = c.weight;
        c.weight = 0;
    }
    Ok(c)
}

fn weight(
    op: crate::core::Operation,
    p: &BTreeMap<String, serde_json::Value>,
) -> Result<u64, Error> {
    let symbol = p.contains_key("symbol");
    let limit = p
        .get("limit")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(500);
    Ok(match op.name {
        "currentAllOpenOrders" | "ticker24hrPriceChangeStatistics" | "openAlgoOrders" => {
            // openAlgoOrders: 1 symbol-scoped, 40 unscoped (demo probe
            // 2026-10-03, issue #11); the others are catalog weights.
            if symbol { 1 } else { 40 }
        }
        "symbolOrderBookTicker" => {
            if symbol {
                2
            } else {
                5
            }
        }
        "symbolPriceTicker" | "notionalBracketForSymbol" => {
            if symbol {
                1
            } else {
                2
            }
        }
        "usersForceOrders" => {
            if symbol {
                20
            } else {
                50
            }
        }
        "orderBook" => match limit {
            5 | 10 | 20 | 50 => 2,
            100 => 5,
            500 => 10,
            1000 => 20,
            _ => return Err(Error::Validation("depth limit")),
        },
        "continuousContractKlineCandlestickData"
        | "indexPriceKlineCandlestickData"
        | "klineCandlestickData"
        | "markPriceKlineCandlestickData"
        | "premiumIndexKlineData" => match limit {
            1..=99 => 1,
            100..=499 => 2,
            500..=1000 => 5,
            1001..=1500 => 10,
            _ => return Err(Error::Validation("kline limit")),
        },
        "modifyOrder" => 1,
        "newOrder" => 0,
        "accountTradeList" | "allOrders" | "placeMultipleOrders" | "modifyMultipleOrders" => 5,
        _ => op.weight,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BudgetLimits, Budgets, core::Request};
    #[test]
    fn probe_verified_algo_and_funding_costs_charge_exact_budgets() {
        // Weights and order slots observed on demo-dapi 2026-10-03 (issue #11):
        // an algo placement charges no IP weight but one slot on each order
        // limit; its reads and cancel charge 1 symbol-scoped / 40 unscoped and
        // 1 respectively; fundingInfo is free.
        use super::super::rest_requests::{
            CancelAlgoOrder, FundingInfo, NewAlgoOrder, OpenAlgoOrders,
        };
        use crate::core::parameters;
        use crate::{ClientOrderId, Decimal, Symbol};
        let algo = NewAlgoOrder::new()
            .algo_type("CONDITIONAL")
            .symbol(Symbol::new("BTCUSD_PERP").unwrap())
            .side("SELL")
            .type_value("STOP_MARKET")
            .trigger_price(Decimal::from(10_000))
            .client_algo_id(ClientOrderId::new("probe/1").unwrap());
        let charge = cost(NewAlgoOrder::OP, &parameters(&algo).unwrap()).unwrap();
        assert_eq!(charge.weight, 0);
        assert_eq!(charge.orders10, 1);
        assert_eq!(charge.orders60, 1);
        let scoped = OpenAlgoOrders::new().symbol(Symbol::new("BTCUSD_PERP").unwrap());
        assert_eq!(
            cost(OpenAlgoOrders::OP, &parameters(&scoped).unwrap())
                .unwrap()
                .weight,
            1
        );
        assert_eq!(
            cost(OpenAlgoOrders::OP, &BTreeMap::new()).unwrap().weight,
            40
        );
        assert_eq!(
            cost(CancelAlgoOrder::OP, &BTreeMap::new()).unwrap().weight,
            1
        );
        assert_eq!(cost(FundingInfo::OP, &BTreeMap::new()).unwrap().weight, 0);
    }
    #[test]
    fn each_coinm_download_kind_has_eight_calendar_month_slots() {
        use super::super::rest_requests::{
            GetDownloadIdForFuturesOrderHistory, GetDownloadIdForFuturesTradeHistory,
            GetDownloadIdForFuturesTransactionHistory,
        };
        for op in [
            GetDownloadIdForFuturesOrderHistory::OP,
            GetDownloadIdForFuturesTradeHistory::OP,
            GetDownloadIdForFuturesTransactionHistory::OP,
        ] {
            let b = Budgets::new(BudgetLimits::coinm()).unwrap();
            let c = cost(op, &BTreeMap::new()).unwrap();
            for minute in 0..8 {
                b.admit(c, minute * 60_000).unwrap();
            }
            assert!(matches!(
                b.admit(c, 8 * 60_000),
                Err(Error::Admission { .. })
            ));
            b.admit(c, 2_678_400_000).unwrap();
        }
    }
}
