// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::*;
use serde::{Serialize, Serializer};
use std::collections::BTreeMap;
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};

struct Attempt {
    serializations: AtomicUsize,
    authority_checks: AtomicUsize,
    validated_id: Mutex<Option<String>>,
}

impl Serialize for Attempt {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let snapshot = self.serializations.fetch_add(1, Ordering::SeqCst) + 1;
        serde_json::json!({"symbol":"BTCUSDT", "newClientOrderId":format!("retained:{snapshot}")})
            .serialize(serializer)
    }
}

impl Request for Attempt {
    type Response = serde_json::Value;
    const OP: Operation = <crate::usdm::rest_requests::NewOrder as Request>::OP;

    fn validate_with_parameters(
        &self,
        params: &BTreeMap<String, serde_json::Value>,
    ) -> Result<(), Error> {
        let id = params
            .get("newClientOrderId")
            .and_then(serde_json::Value::as_str)
            .ok_or(Error::Validation("snapshot identity absent"))?;
        *self
            .validated_id
            .lock()
            .map_err(|_| Error::Configuration("snapshot state poisoned"))? = Some(id.to_owned());
        Ok(())
    }

    fn validate_authority(&self, _now: u64) -> Result<(), Error> {
        if self.authority_checks.fetch_add(1, Ordering::SeqCst) == 0 {
            Ok(())
        } else {
            Err(Error::Validation("authority revoked before sending"))
        }
    }

    fn cost_with_parameters(
        &self,
        params: &BTreeMap<String, serde_json::Value>,
    ) -> Result<Cost, Error> {
        self.check_snapshot(params, "cost used a different parameter snapshot")?;
        Ok(Cost::default())
    }

    fn validate_authority_with_parameters(
        &self,
        params: &BTreeMap<String, serde_json::Value>,
        now: u64,
    ) -> Result<(), Error> {
        self.check_snapshot(params, "authority used a different parameter snapshot")?;
        self.validate_authority(now)
    }
}

impl Attempt {
    fn check_snapshot(
        &self,
        params: &BTreeMap<String, serde_json::Value>,
        reason: &'static str,
    ) -> Result<(), Error> {
        let validated = self
            .validated_id
            .lock()
            .map_err(|_| Error::Configuration("snapshot state poisoned"))?;
        if validated.as_deref()
            == params
                .get("newClientOrderId")
                .and_then(serde_json::Value::as_str)
        {
            Ok(())
        } else {
            Err(Error::Validation(reason))
        }
    }
}

struct RecordingSigner(Arc<Mutex<Option<String>>>);
impl super::super::Signer for RecordingSigner {
    fn sign(&self, payload: &[u8]) -> Result<String, Error> {
        let payload = std::str::from_utf8(payload).map_err(|_| Error::Signing)?;
        *self.0.lock().map_err(|_| Error::Signing)? = Some(payload.to_owned());
        Ok("fixture+/=".into())
    }
}

#[tokio::test]
async fn one_parameter_snapshot_survives_signing_and_the_authority_recheck() -> Result<(), Error> {
    let attempt = Attempt {
        serializations: AtomicUsize::new(0),
        authority_checks: AtomicUsize::new(0),
        validated_id: Mutex::new(None),
    };
    let signed = Arc::new(Mutex::new(None));
    let client = HttpClient::new(
        super::super::validate_url("http://127.0.0.1:9", false)?,
        Some(Credentials::external(
            "fixture-key",
            Arc::new(RecordingSigner(Arc::clone(&signed))),
        )?),
        Arc::new(super::super::SystemClock),
        Budgets::new(super::super::BudgetLimits::usdm())?,
        Duration::from_secs(1),
        None,
    )?;
    let result = client
        .execute(&attempt, Instant::now() + Duration::from_secs(1))
        .await;
    assert!(
        matches!(
            &result,
            Err(Error::Validation("authority revoked before sending"))
        ),
        "{result:?}"
    );
    assert_eq!(attempt.authority_checks.load(Ordering::SeqCst), 2);
    assert_eq!(attempt.serializations.load(Ordering::SeqCst), 1);
    let signed = signed.lock().map_err(|_| Error::Signing)?;
    assert!(signed.as_deref().is_some_and(|payload| {
        url::form_urlencoded::parse(payload.as_bytes())
            .any(|(name, value)| name == "newClientOrderId" && value == "retained:1")
    }));
    Ok(())
}

impl RecoveryRead for Attempt {}

#[tokio::test]
async fn recovery_dispatch_refuses_a_mutation_before_encoding_or_sending() -> Result<(), Error> {
    let attempt = Attempt {
        serializations: AtomicUsize::new(0),
        authority_checks: AtomicUsize::new(0),
        validated_id: Mutex::new(None),
    };
    let client = HttpClient::new(
        super::super::validate_url("http://127.0.0.1:9", false)?,
        None,
        Arc::new(super::super::SystemClock),
        Budgets::new(super::super::BudgetLimits::usdm())?,
        Duration::from_secs(1),
        None,
    )?;
    let result = client
        .execute_recovery_read(&attempt, Instant::now() + Duration::from_secs(1))
        .await;
    assert!(
        matches!(
            result,
            Err(Error::Configuration("recovery request must be read-only"))
        ),
        "{result:?}"
    );
    assert_eq!(attempt.serializations.load(Ordering::SeqCst), 0);
    assert_eq!(attempt.authority_checks.load(Ordering::SeqCst), 0);
    Ok(())
}
