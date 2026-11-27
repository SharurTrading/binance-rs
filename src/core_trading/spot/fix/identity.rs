// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::Error;
use jiff::{
    SignedDuration,
    civil::{Date, DateTime, Time},
};
macro_rules! identity {
    ($name:ident, $doc:literal, $valid:expr) => {
        #[doc=$doc]
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(String);
        impl $name {
            /// Validate exact native spelling without normalization.
            ///
            /// # Errors
            /// Refuses invalid provider grammar.
            pub fn new(value: impl Into<String>) -> Result<Self, Error> {
                let value = value.into();
                if !($valid)(&value) {
                    return Err(Error::Validation(stringify!($name)));
                }
                Ok(Self(value))
            }
            /// Exact provider identity.
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
    };
}
identity!(
    ClientId,
    "Caller-owned FIX client/list/subscription identity, using FIX's own grammar.",
    |s: &str| !s.is_empty()
        && s.len() <= 36
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_".contains(&b))
);
identity!(
    CompId,
    "FIX session component identity; unique across active account sessions.",
    |s: &str| !s.is_empty()
        && s.len() <= 8
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_".contains(&b))
);
identity!(
    WireId,
    "Exact opaque provider identity; no inferred asset or fabricated numeric representation.",
    |s: &str| !s.is_empty() && s.bytes().all(|b| b.is_ascii_graphic())
);

const UNIX_EPOCH_UTC: DateTime = DateTime::constant(1970, 1, 1, 0, 0, 0, 0);

// FIX uses UTC and permits every four-digit year, including the end of 9999.
fn civil_from_micros(micros: i64) -> Result<DateTime, Error> {
    UNIX_EPOCH_UTC
        .checked_add(SignedDuration::from_micros(micros))
        .map_err(|_| Error::Validation("FIX timestamp range"))
}

/// Checked native FIX timestamp, represented as UTC microseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(i64);
impl Timestamp {
    /// Preserve a timestamp representable by FIX's four-digit UTC year.
    ///
    /// # Errors
    /// Refuses out-of-range calendar values.
    pub fn from_micros(micros: i64) -> Result<Self, Error> {
        if civil_from_micros(micros)?.year() < 0 {
            return Err(Error::Validation("FIX timestamp year"));
        }
        Ok(Self(micros))
    }
    /// Native timestamp in microseconds; no millisecond conversion is implicit.
    #[must_use]
    pub fn micros(self) -> i64 {
        self.0
    }
    pub(super) fn parse(text: &str) -> Result<Self, Error> {
        fn n(s: &str) -> Result<i32, Error> {
            if !s.bytes().all(|b| b.is_ascii_digit()) {
                return Err(Error::Validation("FIX timestamp digits"));
            }
            s.parse()
                .map_err(|_| Error::Validation("FIX timestamp digits"))
        }
        if !text.is_ascii() || !matches!(text.len(), 17 | 21 | 24) {
            return Err(Error::Validation("FIX timestamp format"));
        }
        if text.get(8..9) != Some("-")
            || text.get(11..12) != Some(":")
            || text.get(14..15) != Some(":")
            || (text.len() > 17 && text.get(17..18) != Some("."))
        {
            return Err(Error::Validation("FIX timestamp separators"));
        }
        let year =
            i16::try_from(n(&text[..4])?).map_err(|_| Error::Validation("FIX timestamp year"))?;
        let month =
            i8::try_from(n(&text[4..6])?).map_err(|_| Error::Validation("FIX timestamp month"))?;
        let day =
            i8::try_from(n(&text[6..8])?).map_err(|_| Error::Validation("FIX timestamp day"))?;
        let date =
            Date::new(year, month, day).map_err(|_| Error::Validation("FIX timestamp date"))?;
        let nanos = match text.len() {
            17 => 0,
            21 => n(&text[18..])? * 1_000_000,
            _ => n(&text[18..])? * 1_000,
        };
        let time = Time::new(
            i8::try_from(n(&text[9..11])?).map_err(|_| Error::Validation("FIX timestamp hour"))?,
            i8::try_from(n(&text[12..14])?)
                .map_err(|_| Error::Validation("FIX timestamp minute"))?,
            i8::try_from(n(&text[15..17])?)
                .map_err(|_| Error::Validation("FIX timestamp second"))?,
            nanos,
        )
        .map_err(|_| Error::Validation("FIX timestamp time"))?;
        let elapsed = date.to_datetime(time).duration_since(UNIX_EPOCH_UTC);
        let micros = i64::try_from(elapsed.as_micros())
            .map_err(|_| Error::Validation("FIX timestamp range"))?;
        Ok(Self(micros))
    }
    pub(super) fn wire(self) -> Result<String, Error> {
        Ok(civil_from_micros(self.0)?
            .strftime("%Y%m%d-%H:%M:%S%.6f")
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::Timestamp;
    use crate::Error;

    #[test]
    fn fix_utc_fields_preserve_microseconds_through_the_last_four_digit_year() {
        for (text, micros, wire) in [
            ("19691231-23:59:59.999999", -1, "19691231-23:59:59.999999"),
            ("19700101-00:00:00.123", 123_000, "19700101-00:00:00.123000"),
            (
                "99991231-23:59:59.999999",
                253_402_300_799_999_999,
                "99991231-23:59:59.999999",
            ),
        ] {
            assert_eq!(Timestamp::parse(text).unwrap().micros(), micros);
            assert_eq!(
                Timestamp::from_micros(micros).unwrap().wire().unwrap(),
                wire
            );
        }
    }

    #[test]
    fn fix_seconds_reject_leap_second_spelling() {
        assert!(matches!(
            Timestamp::parse("20161231-23:59:60.000001"),
            Err(Error::Validation("FIX timestamp time"))
        ));
    }
}
