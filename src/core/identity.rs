// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use super::Error;
use serde::{Deserialize, Serialize};

macro_rules! identity {
    ($name:ident, $doc:literal, $validate:expr) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);
        impl $name {
            /// Validate a provider identity without modifying it.
            ///
            /// # Errors
            /// Returns `Error::Validation` for an invalid identity.
            pub fn new(value: impl Into<String>) -> Result<Self, Error> {
                let value = value.into();
                if !($validate)(&value) {
                    return Err(Error::Validation(stringify!($name)));
                }
                Ok(Self(value))
            }
            /// The exact provider identity.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                Self::new(String::deserialize(d)?).map_err(serde::de::Error::custom)
            }
        }
    };
}
identity!(
    Symbol,
    "A provider-supplied symbol; never infer assets from its spelling.",
    |s: &str| !s.is_empty() && !s.chars().any(char::is_control)
);
identity!(
    ClientOrderId,
    "Caller-owned Futures client order ID, with Binance's documented grammar.",
    |s: &str| !s.is_empty()
        && s.len() <= 36
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b".-_:/".contains(&b))
);
identity!(
    RequestId,
    "Caller-supplied WebSocket correlation ID, unique within a socket generation.",
    |s: &str| !s.is_empty() && !s.chars().any(char::is_control)
);

/// Redacted, zeroized provider secret (listen keys and signed download links).
#[derive(Clone)]
pub struct SensitiveString(std::sync::Arc<zeroize::Zeroizing<String>>);
impl SensitiveString {
    /// Keep a caller-supplied secret without logging it.
    #[must_use]
    pub fn new(value: impl Into<String>) -> Self {
        Self(std::sync::Arc::new(zeroize::Zeroizing::new(value.into())))
    }
    /// Explicit access for a provider request; never log the result.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl std::fmt::Debug for SensitiveString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[REDACTED]")
    }
}
impl PartialEq for SensitiveString {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for SensitiveString {}
impl serde::Serialize for SensitiveString {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SensitiveString {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Ok(Self::new(String::deserialize(d)?))
    }
}
