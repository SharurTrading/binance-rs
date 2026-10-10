// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Generated venue enumerations; regenerate with scripts/codegen/generate.py.

/// Symbol status (`status` in exchange information).
///
/// One variant per value documented at:
///
/// - <https://github.com/binance/binance-spot-api-docs/blob/master/enums.md#symbol-status-status>
///
/// A value the venue sends that is not documented there decodes to `Unknown`
/// exactly as sent, and encodes back unchanged.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SymbolStatus {
    /// Venue `TRADING`.
    Trading,
    /// Venue `END_OF_DAY`.
    EndOfDay,
    /// Venue `HALT`.
    Halt,
    /// Venue `BREAK`.
    Break,
    /// Venue `CANCEL_ONLY`.
    CancelOnly,
    /// A value the source documentation does not list, kept exactly as the venue sent it.
    Unknown(String),
}
impl SymbolStatus {
    /// The exact venue spelling.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Trading => "TRADING",
            Self::EndOfDay => "END_OF_DAY",
            Self::Halt => "HALT",
            Self::Break => "BREAK",
            Self::CancelOnly => "CANCEL_ONLY",
            Self::Unknown(value) => value,
        }
    }
}
impl serde::Serialize for SymbolStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SymbolStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match value.as_str() {
            "TRADING" => Self::Trading,
            "END_OF_DAY" => Self::EndOfDay,
            "HALT" => Self::Halt,
            "BREAK" => Self::Break,
            "CANCEL_ONLY" => Self::CancelOnly,
            _ => Self::Unknown(value),
        })
    }
}
