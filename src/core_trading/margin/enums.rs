// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated venue enumerations; regenerate with scripts/codegen/generate.py.

/// Native liability-change kind announced for the Margin User Data Stream.
///
/// One variant per value documented at:
///
/// - <https://developers.binance.com/en/docs/products/margin-trading/change-log#2026-09-25>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum LiabilityChangeType {
    /// Venue `BORROW`.
    Borrow,
    /// Venue `REPAY`.
    Repay,
    /// Venue `INTEREST`.
    Interest,
    /// Venue `DEBT_CHANGE`.
    DebtChange,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl LiabilityChangeType {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Borrow => "BORROW",
            Self::Repay => "REPAY",
            Self::Interest => "INTEREST",
            Self::DebtChange => "DEBT_CHANGE",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for LiabilityChangeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LiabilityChangeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "BORROW" => Self::Borrow,
            "REPAY" => Self::Repay,
            "INTEREST" => Self::Interest,
            "DEBT_CHANGE" => Self::DebtChange,
            _ => Self::Unknown(value),
        })
    }
}
