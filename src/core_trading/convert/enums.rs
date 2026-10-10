// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated venue enumerations; regenerate with scripts/codegen/generate.py.

/// Native `OrderStatus` values documented for this product.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/catalog/core-trading-convert/api/rest-api/trade#accept-quote>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum OrderStatus {
    /// Venue `PROCESS`.
    Process,
    /// Venue `ACCEPT_SUCCESS`.
    AcceptSuccess,
    /// Venue `SUCCESS`.
    Success,
    /// Venue `FAIL`.
    Fail,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl OrderStatus {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Process => "PROCESS",
            Self::AcceptSuccess => "ACCEPT_SUCCESS",
            Self::Success => "SUCCESS",
            Self::Fail => "FAIL",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for OrderStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for OrderStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "PROCESS" => Self::Process,
            "ACCEPT_SUCCESS" => Self::AcceptSuccess,
            "SUCCESS" => Self::Success,
            "FAIL" => Self::Fail,
            _ => Self::Unknown(value),
        })
    }
}
