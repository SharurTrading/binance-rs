// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::{Budgets, Error};

/// Caller-selected capacity kept for explicit venue cancellations and reducing orders.
///
/// All values are native quota units per applicable window, not a count of exits.
/// Zero disables a dimension. This policy never increases a venue limit: ordinary
/// charged attempts use limit minus reserve (floored at zero), eligible attempts use
/// the full limit, and both share counters and cooldowns. A reserve above a newly
/// stated limit refuses ordinary charged attempts until the caller changes policy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct ExitReserve {
    /// REST IP request-weight units, including WebSocket weight sharing that counter.
    pub request_weight: u64,
    /// WebSocket API weight units when its IP counter is separate from REST.
    pub websocket_weight: u64,
    /// Raw REST requests per counted raw-request window.
    pub raw_requests: u64,
    /// Account order placements per every counted order window, including Margin/FIX.
    pub orders: u64,
    /// SAPI endpoint weight units, in its IP or UID scope.
    pub sapi_weight: u64,
    /// SAPI endpoint requests per documented second/minute request window.
    pub sapi_requests: u64,
}

impl ExitReserve {
    /// Set REST/shared-IP request-weight units per counted minute.
    #[must_use]
    pub fn request_weight(mut self, value: u64) -> Self {
        self.request_weight = value;
        self
    }

    /// Set separate WebSocket API request-weight units per counted minute.
    #[must_use]
    pub fn websocket_weight(mut self, value: u64) -> Self {
        self.websocket_weight = value;
        self
    }

    /// Set raw-request units per counted window.
    #[must_use]
    pub fn raw_requests(mut self, value: u64) -> Self {
        self.raw_requests = value;
        self
    }

    /// Set account order-placement units in every native order window.
    #[must_use]
    pub fn orders(mut self, value: u64) -> Self {
        self.orders = value;
        self
    }

    /// Set endpoint IP/UID SAPI weight units per counted minute.
    #[must_use]
    pub fn sapi_weight(mut self, value: u64) -> Self {
        self.sapi_weight = value;
        self
    }

    /// Set endpoint SAPI request units per counted second/minute window.
    #[must_use]
    pub fn sapi_requests(mut self, value: u64) -> Self {
        self.sapi_requests = value;
        self
    }
}

impl Budgets {
    /// Replace the exit reserve for this owner's shared IP, WebSocket and account scopes.
    ///
    /// This is an intentional policy change for every client sharing those scopes.
    /// Retain this owner or a product configuration to adjust policy at runtime.
    /// Zero releases the reserve. One consumer should coordinate each shared scope:
    /// replacements are last-write-wins, not additive leases. Default construction
    /// never resets it. Other accounts sharing this IP inherit its IP reserve, while
    /// their account reserve remains their own. Accepted usage, venue limit evidence
    /// and cooldowns stay intact; raising a reserve cannot recover spent capacity.
    /// FIX session message reserves use their configuration and session setter.
    ///
    /// # Errors
    /// Returns a configuration error if a budget mutex is poisoned; no policy changes.
    pub fn set_exit_reserve(&self, reserve: ExitReserve) -> Result<(), Error> {
        let mut ip = self
            .ip
            .lock()
            .map_err(|_| Error::Configuration("IP budget poisoned"))?;
        let mut ws = self
            .ws_ip
            .lock()
            .map_err(|_| Error::Configuration("WS IP budget poisoned"))?;
        let mut account = self
            .account
            .lock()
            .map_err(|_| Error::Configuration("account budget poisoned"))?;
        ip.exit_reserve = reserve;
        ws.exit_reserve = reserve;
        account.exit_reserve = reserve;
        Ok(())
    }
}

#[derive(Clone, Copy, Default)]
pub(crate) enum ExitEligibility {
    #[default]
    Ordinary,
    Eligible,
}

impl ExitEligibility {
    pub(crate) fn from_flag(eligible: bool) -> Self {
        if eligible {
            Self::Eligible
        } else {
            Self::Ordinary
        }
    }

    pub(crate) fn is_eligible(self) -> bool {
        matches!(self, Self::Eligible)
    }
}

/// Capacity remains bounded by the venue; reservation only reduces ordinary admission.
pub(crate) fn admission_limit(limit: u64, reserve: u64, exit: bool) -> u64 {
    if exit {
        limit
    } else {
        limit.saturating_sub(reserve)
    }
}

fn enabled(value: Option<&serde_json::Value>) -> bool {
    value.is_some_and(|value| value.as_bool() == Some(true) || value.as_str() == Some("true"))
}

/// Recognize explicit creation flags, including every member of a native batch.
pub(crate) fn reducing_placement(
    operation: &str,
    parameters: &std::collections::BTreeMap<String, serde_json::Value>,
    close_position: bool,
    batch_parameter: &str,
) -> bool {
    let reducing = |reduce, close| enabled(reduce) || (close_position && enabled(close));
    match operation {
        "newOrder" | "newAlgoOrder" => reducing(
            parameters.get("reduceOnly"),
            parameters.get("closePosition"),
        ),
        "placeMultipleOrders" => parameters
            .get(batch_parameter)
            .and_then(serde_json::Value::as_array)
            .is_some_and(|orders| {
                !orders.is_empty()
                    && orders.iter().all(|order| {
                        order.is_object()
                            && reducing(order.get("reduceOnly"), order.get("closePosition"))
                    })
            }),
        _ => false,
    }
}

/// Futures also treats the exact opposing side of a named hedge leg as reducing.
pub(crate) fn futures_reducing_placement(
    operation: &str,
    parameters: &std::collections::BTreeMap<String, serde_json::Value>,
) -> bool {
    fn hedge_close(
        side: Option<&serde_json::Value>,
        position_side: Option<&serde_json::Value>,
    ) -> bool {
        matches!(
            (
                side.and_then(serde_json::Value::as_str),
                position_side.and_then(serde_json::Value::as_str)
            ),
            (Some("SELL"), Some("LONG")) | (Some("BUY"), Some("SHORT"))
        )
    }

    match operation {
        "newOrder" => {
            reducing_placement(operation, parameters, true, "batchOrders")
                || hedge_close(parameters.get("side"), parameters.get("positionSide"))
        }
        "placeMultipleOrders" => parameters
            .get("batchOrders")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|orders| {
                !orders.is_empty()
                    && orders.iter().all(|order| {
                        order.is_object()
                            && (enabled(order.get("reduceOnly"))
                                || enabled(order.get("closePosition"))
                                || hedge_close(order.get("side"), order.get("positionSide")))
                    })
            }),
        _ => reducing_placement(operation, parameters, true, "batchOrders"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BudgetLimits, core::Cost};

    #[test]
    fn ordinary_weight_stops_before_shared_reserved_capacity() {
        let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(3)).unwrap();
        budgets
            .set_exit_reserve(ExitReserve {
                request_weight: 1,
                ..ExitReserve::default()
            })
            .unwrap();
        let cost = Cost {
            weight: 2,
            ..Cost::default()
        };
        budgets.admit(cost, 1000).unwrap();
        assert!(matches!(
            budgets.for_account().admit(
                Cost {
                    weight: 1,
                    ..Cost::default()
                },
                1000
            ),
            Err(Error::Admission { .. })
        ));
        budgets
            .clone()
            .admit(
                Cost {
                    weight: 1,
                    exit: ExitEligibility::Eligible,
                    ..Cost::default()
                },
                1000,
            )
            .unwrap();
        assert!(matches!(
            budgets.admit(
                Cost {
                    weight: 1,
                    exit: ExitEligibility::Eligible,
                    ..Cost::default()
                },
                1000
            ),
            Err(Error::Admission { .. })
        ));
    }

    #[test]
    fn runtime_reserve_replacement_and_release_preserve_every_charged_counter() {
        let mut limits = BudgetLimits::usdm()
            .weight_per_minute(4)
            .ws_weight_per_minute(4)
            .orders(4, 4);
        limits.raw_requests_per_five_minutes = Some(4);
        let budgets = Budgets::new(limits).unwrap();
        let sibling = budgets.clone();
        budgets.observe_order_window(2000, 0, 4, 1000).unwrap();
        let cost = Cost {
            weight: 1,
            ws_weight: 1,
            raw_requests: 1,
            orders10: 1,
            orders60: 1,
            ..Cost::default()
        };
        budgets.admit(cost, 1000).unwrap();
        let reserve = |n| {
            ExitReserve::default()
                .request_weight(n)
                .websocket_weight(n)
                .raw_requests(n)
                .orders(n)
        };
        budgets.set_exit_reserve(reserve(3)).unwrap();
        assert!(matches!(
            sibling.admit(cost, 1000),
            Err(Error::Admission { .. })
        ));
        sibling.set_exit_reserve(reserve(2)).unwrap();
        budgets.admit(cost, 1000).unwrap();
        assert!(matches!(
            sibling.admit(cost, 1000),
            Err(Error::Admission { .. })
        ));
        budgets.set_exit_reserve(ExitReserve::default()).unwrap();
        sibling.admit(cost, 1000).unwrap();
        sibling.admit(cost, 1000).unwrap();
        assert!(matches!(
            budgets.admit(
                Cost {
                    exit: ExitEligibility::Eligible,
                    ..cost
                },
                1000
            ),
            Err(Error::Admission { .. })
        ));
    }

    #[test]
    fn runtime_reserve_replacement_keeps_account_ownership_and_cooldowns() {
        let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(4).orders(4, 4)).unwrap();
        let other = budgets.for_account();
        let weight = Cost {
            weight: 2,
            ..Cost::default()
        };
        let order = Cost {
            orders10: 2,
            ..Cost::default()
        };
        budgets.admit(weight, 1000).unwrap();
        budgets.admit(order, 1000).unwrap();
        other.admit(order, 1000).unwrap();
        budgets
            .set_exit_reserve(ExitReserve::default().request_weight(2).orders(2))
            .unwrap();
        assert!(matches!(
            other.admit(weight, 1000),
            Err(Error::Admission { .. })
        ));
        assert!(matches!(
            budgets.admit(order, 1000),
            Err(Error::Admission { .. })
        ));
        other.admit(order, 1000).unwrap();
        other.set_exit_reserve(ExitReserve::default()).unwrap();
        assert!(matches!(
            budgets.admit(order, 1000),
            Err(Error::Admission { .. })
        ));
        budgets.admit(weight, 1000).unwrap();
        assert!(matches!(
            other.admit(weight, 1000),
            Err(Error::Admission { .. })
        ));
        budgets
            .observe(
                &crate::RateEvidence {
                    retry_after: Some(std::time::Duration::from_millis(5)),
                    ..crate::RateEvidence::default()
                },
                1000,
                false,
            )
            .unwrap();
        budgets.set_exit_reserve(ExitReserve::default()).unwrap();
        assert!(matches!(
            budgets.admit(order, 1000),
            Err(Error::Admission { .. })
        ));
        budgets.admit(order, 1005).unwrap();
        assert!(matches!(
            budgets.admit(order, 1005),
            Err(Error::Admission { .. })
        ));
    }

    #[test]
    fn runtime_reserve_release_preserves_tighter_venue_evidence_and_unknown_ban() {
        let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(4)).unwrap();
        let cost = Cost {
            weight: 1,
            ..Cost::default()
        };
        budgets.admit(cost, 1000).unwrap();
        budgets
            .adopt_stated([crate::core::StatedLimit {
                kind: Some("REQUEST_WEIGHT"),
                interval: Some("MINUTE"),
                interval_num: Some(1),
                limit: Some(2),
            }])
            .unwrap();
        budgets
            .set_exit_reserve(ExitReserve::default().request_weight(2))
            .unwrap();
        assert!(matches!(
            budgets.admit(cost, 1000),
            Err(Error::Admission { .. })
        ));
        budgets.set_exit_reserve(ExitReserve::default()).unwrap();
        budgets.admit(cost, 1000).unwrap();
        assert!(matches!(
            budgets.admit(
                Cost {
                    exit: ExitEligibility::Eligible,
                    ..cost
                },
                1000
            ),
            Err(Error::Admission { .. })
        ));
        budgets
            .observe_ban(418, &crate::RateEvidence::default())
            .unwrap();
        budgets
            .set_exit_reserve(ExitReserve::default().request_weight(1))
            .unwrap();
        budgets.set_exit_reserve(ExitReserve::default()).unwrap();
        assert!(matches!(
            budgets.admit(cost, 60_000),
            Err(Error::CooldownTimingUnknown)
        ));
    }

    fn multidimensional_budgets() -> Budgets {
        let mut limits = BudgetLimits::usdm()
            .weight_per_minute(5)
            .ws_weight_per_minute(4)
            .orders(3, 3);
        limits.raw_requests_per_five_minutes = Some(3);
        let budgets = Budgets::new(limits).unwrap();
        budgets
            .set_exit_reserve(
                ExitReserve::default()
                    .request_weight(2)
                    .websocket_weight(1)
                    .raw_requests(1)
                    .orders(1),
            )
            .unwrap();
        let account = Cost {
            orders10: 2,
            ..Cost::default()
        };
        budgets.admit(account, 1000).unwrap();
        budgets
    }

    #[test]
    fn reserve_applies_to_separate_ws_raw_and_each_account_order_window_atomically() {
        let budgets = multidimensional_budgets();
        // Account refusal must not partly spend the independently available IP budgets.
        assert!(matches!(
            budgets.admit(
                Cost {
                    weight: 3,
                    ws_weight: 3,
                    raw_requests: 2,
                    orders10: 1,
                    ..Cost::default()
                },
                1000
            ),
            Err(Error::Admission { .. })
        ));
        budgets
            .admit(
                Cost {
                    weight: 3,
                    ws_weight: 3,
                    raw_requests: 2,
                    ..Cost::default()
                },
                1000,
            )
            .unwrap();
        for cost in [
            Cost {
                weight: 1,
                ..Cost::default()
            },
            Cost {
                ws_weight: 1,
                ..Cost::default()
            },
            Cost {
                raw_requests: 1,
                ..Cost::default()
            },
            Cost {
                orders60: 1,
                ..Cost::default()
            },
        ] {
            assert!(matches!(
                budgets.admit(cost, 1000),
                Err(Error::Admission { .. })
            ));
        }
        budgets
            .admit(
                Cost {
                    weight: 2,
                    ws_weight: 1,
                    raw_requests: 1,
                    orders60: 1,
                    exit: ExitEligibility::Eligible,
                    ..Cost::default()
                },
                1000,
            )
            .unwrap();
        assert!(matches!(
            budgets.clone().admit(
                Cost {
                    weight: 1,
                    exit: ExitEligibility::Eligible,
                    ..Cost::default()
                },
                1000
            ),
            Err(Error::Admission { .. })
        ));
        budgets
            .for_account()
            .admit(
                Cost {
                    orders10: 2,
                    ..Cost::default()
                },
                1000,
            )
            .unwrap();
    }

    #[test]
    fn discovered_order_limits_below_reserve_stop_ordinary_but_zero_cost_never_blocks() {
        let budgets = Budgets::new(BudgetLimits::usdm()).unwrap();
        budgets
            .set_exit_reserve(ExitReserve {
                orders: 3,
                ..ExitReserve::default()
            })
            .unwrap();
        budgets.observe_order_window(2000, 0, 2, 1000).unwrap();
        assert!(matches!(
            budgets.admit(
                Cost {
                    orders10: 1,
                    weight: 1,
                    ..Cost::default()
                },
                1001
            ),
            Err(Error::Admission { .. })
        ));
        budgets
            .admit(
                Cost {
                    orders10: 2,
                    exit: ExitEligibility::Eligible,
                    ..Cost::default()
                },
                1001,
            )
            .unwrap();
        budgets
            .admit(
                Cost {
                    weight: 1,
                    ..Cost::default()
                },
                1001,
            )
            .unwrap();
        assert!(matches!(
            budgets.admit(
                Cost {
                    orders10: 1,
                    exit: ExitEligibility::Eligible,
                    ..Cost::default()
                },
                1001
            ),
            Err(Error::Admission { .. })
        ));
        budgets
            .admit(
                Cost {
                    orders10: 2,
                    exit: ExitEligibility::Eligible,
                    ..Cost::default()
                },
                3000,
            )
            .unwrap();
    }

    #[test]
    fn tighter_stated_weight_and_excess_usage_still_allow_zero_weight_cancel() {
        let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(5)).unwrap();
        budgets
            .set_exit_reserve(ExitReserve {
                request_weight: 3,
                ..ExitReserve::default()
            })
            .unwrap();
        budgets
            .admit(
                Cost {
                    weight: 2,
                    ..Cost::default()
                },
                1000,
            )
            .unwrap();
        budgets
            .adopt_stated([crate::core::StatedLimit {
                kind: Some("REQUEST_WEIGHT"),
                interval: Some("MINUTE"),
                interval_num: Some(1),
                limit: Some(1),
            }])
            .unwrap();
        for exit in [false, true] {
            assert!(matches!(
                budgets.admit(
                    Cost {
                        weight: 1,
                        exit: ExitEligibility::from_flag(exit),
                        ..Cost::default()
                    },
                    1000
                ),
                Err(Error::Admission { .. })
            ));
            budgets
                .admit(
                    Cost {
                        exit: ExitEligibility::from_flag(exit),
                        ..Cost::default()
                    },
                    1000,
                )
                .unwrap();
        }
        budgets
            .admit(
                Cost {
                    weight: 1,
                    exit: ExitEligibility::Eligible,
                    ..Cost::default()
                },
                60_000,
            )
            .unwrap();
    }

    #[test]
    fn eligible_attempts_honor_known_and_unknown_cooldowns_and_reset() {
        let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(2)).unwrap();
        budgets
            .set_exit_reserve(ExitReserve {
                request_weight: 1,
                ..ExitReserve::default()
            })
            .unwrap();
        budgets
            .observe(
                &crate::RateEvidence {
                    retry_after: Some(std::time::Duration::from_millis(1)),
                    ..crate::RateEvidence::default()
                },
                1000,
                false,
            )
            .unwrap();
        let exit = Cost {
            weight: 1,
            exit: ExitEligibility::Eligible,
            ..Cost::default()
        };
        assert!(matches!(
            budgets.admit(exit, 1000),
            Err(Error::Admission { .. })
        ));
        budgets.admit(exit, 1001).unwrap();
        budgets
            .observe_ban(418, &crate::RateEvidence::default())
            .unwrap();
        assert!(matches!(
            budgets.admit(exit, 1001),
            Err(Error::CooldownTimingUnknown)
        ));
        budgets.release_unknown_ban().unwrap();
        budgets.admit(exit, 1001).unwrap();
        assert!(matches!(
            budgets.admit(exit, 1001),
            Err(Error::Admission { .. })
        ));
        budgets
            .admit(
                Cost {
                    weight: 1,
                    ..Cost::default()
                },
                60_000,
            )
            .unwrap();
    }

    #[test]
    fn creation_eligibility_requires_exact_explicit_flags_and_every_batch_row() {
        use serde_json::json;
        use std::collections::BTreeMap;
        for flag in [json!(true), json!("true")] {
            let p = BTreeMap::from([("reduceOnly".into(), flag)]);
            assert!(reducing_placement("newOrder", &p, false, "batchOrders"));
            assert!(reducing_placement("newAlgoOrder", &p, true, "batchOrders"));
            for op in [
                "testOrder",
                "modifyOrder",
                "orderCancelReplace",
                "modifyMultipleOrders",
            ] {
                assert!(!reducing_placement(op, &p, true, "batchOrders"));
            }
        }
        for flag in [
            json!(false),
            json!("false"),
            json!("TRUE"),
            json!(1),
            json!(null),
        ] {
            assert!(!reducing_placement(
                "newOrder",
                &BTreeMap::from([("reduceOnly".into(), flag)]),
                true,
                "batchOrders"
            ));
        }
        let close = BTreeMap::from([("closePosition".into(), json!("true"))]);
        assert!(reducing_placement("newOrder", &close, true, "batchOrders"));
        assert!(!reducing_placement(
            "newOrder",
            &close,
            false,
            "batchOrders"
        ));
        let hedge = BTreeMap::from([
            ("side".into(), json!("SELL")),
            ("positionSide".into(), json!("LONG")),
        ]);
        assert!(!reducing_placement("newOrder", &hedge, true, "batchOrders"));
        assert!(futures_reducing_placement("newOrder", &hedge));
        for batch in [
            json!([]),
            json!([{"reduceOnly":true},{}]),
            json!([true]),
            json!("[{\"reduceOnly\":true}]"),
        ] {
            assert!(!reducing_placement(
                "placeMultipleOrders",
                &BTreeMap::from([("batchOrders".into(), batch)]),
                true,
                "batchOrders"
            ));
        }
        let batch = BTreeMap::from([(
            "batchOrders".into(),
            json!([{"reduceOnly":"true"},{"closePosition":true}]),
        )]);
        assert!(reducing_placement(
            "placeMultipleOrders",
            &batch,
            true,
            "batchOrders"
        ));
        assert!(!reducing_placement(
            "placeMultipleOrders",
            &batch,
            false,
            "batchOrders"
        ));
    }

    #[test]
    fn futures_regular_hedge_side_is_exact_and_every_batch_member_must_reduce() {
        use serde_json::json;
        use std::collections::BTreeMap;

        for (side, position_side) in [("SELL", "LONG"), ("BUY", "SHORT")] {
            let parameters = BTreeMap::from([
                ("side".into(), json!(side)),
                ("positionSide".into(), json!(position_side)),
            ]);
            assert!(futures_reducing_placement("newOrder", &parameters));
            assert!(!futures_reducing_placement("newAlgoOrder", &parameters));
            assert!(!reducing_placement(
                "newOrder",
                &parameters,
                false,
                "orders"
            ));
        }
        for (side, position_side) in [
            ("BUY", "LONG"),
            ("SELL", "SHORT"),
            ("SELL", "BOTH"),
            ("sell", "LONG"),
            ("SELL", "long"),
        ] {
            let parameters = BTreeMap::from([
                ("side".into(), json!(side)),
                ("positionSide".into(), json!(position_side)),
            ]);
            assert!(!futures_reducing_placement("newOrder", &parameters));
        }
        let all = BTreeMap::from([(
            "batchOrders".into(),
            json!([{"side":"SELL","positionSide":"LONG"},{"reduceOnly":"true"}]),
        )]);
        assert!(futures_reducing_placement("placeMultipleOrders", &all));
        assert!(!reducing_placement(
            "placeMultipleOrders",
            &all,
            false,
            "orders"
        ));
        for batch in [
            json!([{"side":"SELL","positionSide":"LONG"},{}]),
            json!([{"side":"SELL","positionSide":"LONG"},{"side":"BUY","positionSide":"LONG"}]),
            json!([{"side":"SELL","positionSide":"LONG"},true]),
            json!([]),
        ] {
            let parameters = BTreeMap::from([("batchOrders".into(), batch)]);
            assert!(!futures_reducing_placement(
                "placeMultipleOrders",
                &parameters
            ));
        }
    }
}
