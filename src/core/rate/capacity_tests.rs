// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;

fn assert_refused(budgets: &Budgets, cost: Cost, now: u64, delay: u64) {
    assert!(
        matches!(budgets.admit(cost, now), Err(Error::Admission { retry_after })
        if retry_after == Duration::from_millis(delay))
    );
}

#[test]
fn every_interval_budget_admits_its_full_allowance_and_resets_only_at_its_boundary() {
    // Production baseline and additional endpoint quotas; these exercise admission,
    // not live venue throughput. Official sources are recorded in docs/coverage.md.
    let cases = [
        (
            Cost {
                weight: 1,
                ..Cost::default()
            },
            2400,
            60_000,
        ),
        (
            Cost {
                ws_weight: 1,
                ..Cost::default()
            },
            2400,
            60_000,
        ),
        (
            Cost {
                orders10: 1,
                ..Cost::default()
            },
            300,
            10_000,
        ),
        (
            Cost {
                orders60: 1,
                ..Cost::default()
            },
            1200,
            60_000,
        ),
        (
            Cost {
                funding: true,
                ..Cost::default()
            },
            500,
            300_000,
        ),
        (
            Cost {
                history: true,
                ..Cost::default()
            },
            1000,
            300_000,
        ),
        (
            Cost {
                quote: true,
                ..Cost::default()
            },
            360,
            3_600_000,
        ),
    ];
    for (cost, capacity, window) in cases {
        let budgets = Budgets::new(BudgetLimits::usdm()).unwrap();
        for _ in 0..capacity {
            budgets.admit(cost, 1000).unwrap();
        }
        assert_refused(&budgets, cost, 1000, window - 1000);
        assert_refused(&budgets, cost, window - 1, 1);
        budgets.admit(cost, window).unwrap();
        // Conversion also has a daily quota, so its remaining allowance is tested
        // separately below rather than assuming the hourly reset clears both.
        if !cost.quote {
            for _ in 1..capacity {
                budgets.admit(cost, window).unwrap();
            }
            assert_refused(&budgets, cost, window, window);
        }
    }
}

#[test]
fn conversion_hourly_reset_preserves_daily_usage_and_allows_every_remaining_quote() {
    let budgets = Budgets::new(BudgetLimits::usdm()).unwrap();
    let cost = Cost {
        quote: true,
        ..Cost::default()
    };
    for _ in 0..360 {
        budgets.admit(cost, 1000).unwrap();
    }
    assert_refused(&budgets, cost, 1000, 3_599_000);
    for _ in 0..140 {
        budgets.admit(cost, 3_600_000).unwrap();
    }
    assert_refused(&budgets, cost, 3_600_000, 82_800_000);
    assert_refused(&budgets, cost, 86_399_999, 1);
    for _ in 0..360 {
        budgets.admit(cost, 86_400_000).unwrap();
    }
    assert_refused(&budgets, cost, 86_400_000, 3_600_000);
}

#[test]
fn refusal_in_any_scope_consumes_no_capacity_in_other_scopes() {
    let scopes = [
        Cost {
            weight: 1,
            ..Cost::default()
        },
        Cost {
            ws_weight: 1,
            ..Cost::default()
        },
        Cost {
            orders10: 1,
            ..Cost::default()
        },
        Cost {
            orders60: 1,
            ..Cost::default()
        },
    ];
    for (full_scope, cost) in scopes.iter().enumerate() {
        let budgets = Budgets::new(
            BudgetLimits::usdm()
                .weight_per_minute(4)
                .ws_weight_per_minute(4)
                .orders(4, 4),
        )
        .unwrap();
        for _ in 0..4 {
            budgets.admit(*cost, 1000).unwrap();
        }
        assert!(matches!(
            budgets.admit(
                Cost {
                    weight: 1,
                    ws_weight: 1,
                    orders10: 1,
                    orders60: 1,
                    ..Cost::default()
                },
                1000
            ),
            Err(Error::Admission { .. })
        ));
        for (scope, cost) in scopes.iter().enumerate() {
            if scope != full_scope {
                for _ in 0..4 {
                    budgets.admit(*cost, 1000).unwrap();
                }
                assert!(matches!(
                    budgets.admit(*cost, 1000),
                    Err(Error::Admission { .. })
                ));
            }
        }
    }
}

#[test]
fn counter_overflow_is_refused_without_spending_other_budgets() {
    let budgets = Budgets::new(BudgetLimits::usdm().weight_per_minute(u64::MAX)).unwrap();
    budgets
        .admit(
            Cost {
                weight: u64::MAX,
                ..Cost::default()
            },
            1000,
        )
        .unwrap();
    assert_refused(
        &budgets,
        Cost {
            weight: 1,
            orders10: 1,
            ..Cost::default()
        },
        1000,
        59_000,
    );
    budgets
        .admit(
            Cost {
                orders10: 300,
                ..Cost::default()
            },
            1000,
        )
        .unwrap();
}

#[test]
fn all_download_kinds_admit_the_full_calendar_quota_and_reset_together() {
    let january = u64::try_from(
        time::Date::from_calendar_date(2026, time::Month::January, 31)
            .unwrap()
            .midnight()
            .assume_utc()
            .unix_timestamp(),
    )
    .unwrap()
        * 1000;
    let february = january + 86_400_000;
    let budgets = Budgets::new(BudgetLimits::usdm()).unwrap();
    for (kind, capacity) in [(1, 10), (2, 5), (3, 5)] {
        let cost = Cost {
            download: kind,
            ..Cost::default()
        };
        for _ in 0..capacity {
            budgets.admit(cost, january).unwrap();
        }
        assert_refused(&budgets, cost, january, 86_400_000);
        for _ in 0..capacity {
            budgets.admit(cost, february).unwrap();
        }
        assert!(matches!(
            budgets.admit(cost, february),
            Err(Error::Admission { .. })
        ));
    }
}
#[test]
fn spot_full_weight_raw_and_connection_budgets_share_an_ip_owner() {
    let budgets = Budgets::new(BudgetLimits::spot()).unwrap();
    for _ in 0..300 {
        budgets
            .admit(
                Cost {
                    connections: 1,
                    ..Cost::default()
                },
                0,
            )
            .unwrap();
    }
    assert_refused(
        &budgets,
        Cost {
            connections: 1,
            ..Cost::default()
        },
        0,
        300_000,
    );
    for _ in 0..300_000 {
        budgets
            .admit(
                Cost {
                    raw_requests: 1,
                    ..Cost::default()
                },
                0,
            )
            .unwrap();
    }
    assert_refused(
        &budgets,
        Cost {
            raw_requests: 1,
            ..Cost::default()
        },
        0,
        300_000,
    );
    for i in 0..6000 {
        budgets
            .admit(
                if i % 2 == 0 {
                    Cost {
                        weight: 1,
                        ..Cost::default()
                    }
                } else {
                    Cost {
                        ws_weight: 1,
                        ..Cost::default()
                    }
                },
                0,
            )
            .unwrap();
    }
    assert_refused(
        &budgets,
        Cost {
            weight: 1,
            ..Cost::default()
        },
        0,
        60_000,
    );
    assert_refused(
        &budgets.for_account(),
        Cost {
            ws_weight: 1,
            ..Cost::default()
        },
        0,
        60_000,
    );
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
fn spot_daily_order_evidence_refuses_until_the_aligned_day_boundary() {
    let budgets = Budgets::new(BudgetLimits::spot()).unwrap();
    let mut rates = RateEvidence::default();
    rates
        .counters
        .insert("x-mbx-order-count-1d".into(), 159_999);
    budgets.observe(&rates, 0, true).unwrap();
    let cost = Cost {
        orders10: 1,
        orders_day: 1,
        ..Cost::default()
    };
    budgets.admit(cost, 0).unwrap();
    assert_refused(&budgets, cost, 10_000, 86_390_000);
    budgets.admit(cost, 86_400_000).unwrap();
}

#[test]
fn weight_refunds_never_remove_venue_evidence_or_a_new_interval_reservation() {
    let b = Budgets::new(BudgetLimits::spot().weight_per_minute(2)).unwrap();
    let c = Cost {
        weight: 1,
        ..Cost::default()
    };
    b.admit(c, 0).unwrap();
    b.admit(c, 0).unwrap();
    let mut evidence = RateEvidence::default();
    evidence.counters.insert("x-mbx-used-weight-1m".into(), 2);
    b.observe(&evidence, 0, false).unwrap();
    b.refund_weight(1, 0, false).unwrap();
    assert_refused(&b, c, 0, 60_000);
    b.admit(c, 60_000).unwrap();
    b.admit(c, 60_000).unwrap();
    // A late answer belongs to its admission interval, including on the WS API.
    b.refund_weight(1, 0, true).unwrap();
    assert_refused(&b, c, 60_000, 60_000);
}

fn stated(
    kind: &'static str,
    interval: &'static str,
    interval_num: i64,
    limit: Option<i64>,
) -> StatedLimit<'static> {
    StatedLimit {
        kind: Some(kind),
        interval: Some(interval),
        interval_num: Some(interval_num),
        limit,
    }
}

#[test]
fn a_stated_window_without_a_positive_limit_is_a_gap_and_keeps_the_previous_limit() {
    let budgets = Budgets::new(BudgetLimits::coinm()).unwrap();
    budgets
        .adopt_stated([stated("REQUEST_WEIGHT", "MINUTE", 1, Some(2))])
        .unwrap();
    for limit in [None, Some(0), Some(-1)] {
        assert!(matches!(
            budgets.adopt_stated([
                stated("REQUEST_WEIGHT", "MINUTE", 1, Some(5)),
                stated("REQUEST_WEIGHT", "MINUTE", 1, limit),
            ]),
            Err(Error::Gap(_))
        ));
    }
    let weight = Cost {
        weight: 1,
        ..Cost::default()
    };
    budgets.admit(weight, 0).unwrap();
    budgets.admit(weight, 0).unwrap();
    assert_refused(&budgets, weight, 0, 60_000);
}

#[test]
fn only_the_counted_windows_are_adopted_and_the_stricter_of_two_binds() {
    let budgets = Budgets::new(BudgetLimits::coinm()).unwrap();
    budgets
        .adopt_stated([
            stated("ORDERS", "SECOND", 1, Some(1)),
            stated("REQUEST_WEIGHT", "SECOND", 10, Some(1)),
            stated("REQUEST_WEIGHT", "MINUTE", 1, Some(4)),
            stated("REQUEST_WEIGHT", "MINUTE", 1, Some(3)),
        ])
        .unwrap();
    let order = Cost {
        weight: 1,
        orders10: 1,
        orders60: 1,
        ..Cost::default()
    };
    for _ in 0..3 {
        budgets.admit(order, 0).unwrap();
    }
    assert_refused(&budgets, order, 0, 60_000);
}

#[test]
fn a_stated_order_limit_replaces_each_order_window_and_the_stricter_of_two_binds() {
    // Options documents no ten-second or daily order limit; a statement adds them.
    let cases = [
        (
            "SECOND",
            10,
            Cost {
                orders10: 1,
                ..Cost::default()
            },
            10_000,
        ),
        (
            "MINUTE",
            1,
            Cost {
                orders60: 1,
                ..Cost::default()
            },
            60_000,
        ),
        (
            "DAY",
            1,
            Cost {
                orders_day: 1,
                ..Cost::default()
            },
            86_400_000,
        ),
    ];
    for (interval, interval_num, order, window) in cases {
        let budgets = Budgets::new(BudgetLimits::options()).unwrap();
        budgets
            .adopt_stated([
                stated("ORDERS", interval, interval_num, Some(3)),
                stated("ORDERS", interval, interval_num, Some(2)),
            ])
            .unwrap();
        let account = budgets.for_account();
        account.admit(order, 0).unwrap();
        account.admit(order, 0).unwrap();
        assert_refused(&account, order, 0, window);
    }
}
