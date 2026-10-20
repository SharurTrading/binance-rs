// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Convert keeps every identity in its documented signed 64-bit wire domain.

use binance_client::convert::{AcceptanceOrderId, OrderId};

#[test]
fn signed_integer_order_id_boundaries_round_trip_without_losing_evidence() {
    for value in [i64::MIN, -1, 0, 1, i64::MAX] {
        assert_eq!(OrderId::new(value).value(), value);
        let encoded = value.to_string();
        let decoded = serde_json::from_str::<OrderId>(&encoded);
        assert!(
            decoded.is_ok(),
            "provider order identity {value} was refused"
        );
        if let Ok(id) = decoded {
            assert_eq!(id.value(), value);
            assert_eq!(
                serde_json::to_string(&id).ok().as_deref(),
                Some(encoded.as_str())
            );
            let query_id = AcceptanceOrderId::from(id);
            assert_eq!(query_id.as_str(), encoded);
        }
    }
}

#[test]
fn integer_order_ids_still_refuse_out_of_range_or_wrong_wire_types() {
    for value in [
        "9223372036854775808",
        "-9223372036854775809",
        "1.5",
        "null",
        "\"123\"",
    ] {
        assert!(serde_json::from_str::<OrderId>(value).is_err());
    }
}
