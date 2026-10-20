// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;

#[test]
#[ignore = "manual admission throughput measurement; no timing threshold"]
fn admission_throughput() -> Result<(), Error> {
    const ATTEMPTS: u64 = 250_000;
    for sample in 0..5 {
        let budgets = Budgets::new(BudgetLimits::usdm())?;
        budgets.admit(
            Cost {
                weight: 1,
                ..Cost::default()
            },
            0,
        )?;
        let started = std::time::Instant::now();
        for attempt in 0..ATTEMPTS {
            budgets.admit(
                Cost {
                    weight: 1,
                    ..Cost::default()
                },
                attempt * 1000,
            )?;
        }
        eprintln!(
            "REST sample={sample} attempts={ATTEMPTS} elapsed={:?}",
            started.elapsed()
        );
        let budgets = Budgets::sapi()?;
        let cost = super::super::sapi::SapiCost {
            endpoint: "synthetic-admission-benchmark",
            uid: true,
            weight: 1,
            requests_per_second: Some(100),
            requests_per_minute: None,
        };
        budgets.admit_sapi(cost, Cost::default(), 0)?;
        let started = std::time::Instant::now();
        for attempt in 0..ATTEMPTS {
            budgets.admit_sapi(cost, Cost::default(), attempt * 1000)?;
        }
        eprintln!(
            "SAPI sample={sample} attempts={ATTEMPTS} elapsed={:?}",
            started.elapsed()
        );
    }
    Ok(())
}
