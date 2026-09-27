// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

// USD-M operation costs are product policy; the shared core only admits/observes budgets.
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
    if matches!(
        op.name,
        "newOrder" | "testOrder" | "newAlgoOrder" | "modifyOrder"
    ) {
        c.orders10 = 1;
        c.orders60 = 1;
    }
    if matches!(op.name, "placeMultipleOrders" | "modifyMultipleOrders") {
        c.orders10 = 5;
        c.orders60 = 1;
    }
    c.funding = matches!(op.name, "getFundingRateHistory" | "getFundingRateInfo");
    c.history = op.path.starts_with("/futures/data/");
    c.quote = op.name == "sendQuoteRequest";
    c.download = match op.name {
        "getDownloadIdForFuturesOrderHistory" => 1,
        "getDownloadIdForFuturesTradeHistory" => 2,
        "getDownloadIdForFuturesTransactionHistory" => 3,
        _ => 0,
    };
    if op.path.starts_with("/fapi/") {
        if matches!(
            op.name,
            "newOrder"
                | "newAlgoOrder"
                | "modifyOrder"
                | "cancelOrder"
                | "cancelAlgoOrder"
                | "placeMultipleOrders"
                | "modifyMultipleOrders"
                | "cancelMultipleOrders"
        ) {
            c.ws_weight = c.weight;
        }
    } else if !op.path.starts_with("/futures/") {
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
        "futuresTradingQuantitativeRulesIndicators" | "markPrice" | "assetIndex" => {
            if symbol {
                1
            } else {
                10
            }
        }
        "currentAllOpenOrders" | "currentAllAlgoOpenOrders" | "ticker24hrPriceChangeStatistics" => {
            if symbol {
                1
            } else {
                40
            }
        }
        "symbolOrderBookTicker" => {
            if symbol {
                2
            } else {
                5
            }
        }
        "symbolPriceTicker" | "symbolPriceTickerV2" => {
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
        "rpiOrderBook" => {
            if p.get("limit").is_none_or(|v| v.as_i64() == Some(1000)) {
                20
            } else {
                return Err(Error::Validation("RPI depth limit"));
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
        "sendQuoteRequest" => 50,
        // The catalog omits testOrder weight. Reserve one IP unit plus an order
        // slot conservatively; retain/observe actual venue header evidence.
        // Verification: https://github.com/SharurTrading/binance-rs/issues/4.
        "testOrder" => 1,
        "newOrder"
        | "newAlgoOrder"
        | "modifyOrder"
        | "getFundingRateHistory"
        | "getFundingRateInfo" => 0,
        "placeMultipleOrders" | "modifyMultipleOrders" => 5,
        _ => op.weight,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Request, parameters};
    use crate::core_trading::usdm::rest_requests::RpiOrderBook;
    #[test]
    fn rpi_depth_has_only_the_documented_1000_level_request() {
        let request = RpiOrderBook::new().symbol(crate::Symbol::new("BTCUSDT").unwrap());
        assert_eq!(
            cost(RpiOrderBook::OP, &parameters(&request).unwrap())
                .unwrap()
                .weight,
            20
        );
        assert!(cost(RpiOrderBook::OP, &parameters(&request.limit(500)).unwrap()).is_err());
    }
}
