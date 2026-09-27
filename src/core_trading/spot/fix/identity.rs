// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use crate::Error;
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

/// Checked native FIX timestamp, represented as UTC microseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(i64);
impl Timestamp {
    /// Preserve a timestamp representable by FIX's four-digit UTC year.
    ///
    /// # Errors
    /// Refuses out-of-range calendar values.
    pub fn from_micros(micros: i64) -> Result<Self, Error> {
        let dt = time::OffsetDateTime::from_unix_timestamp_nanos(i128::from(micros) * 1000)
            .map_err(|_| Error::Validation("FIX timestamp range"))?;
        if !(0..=9999).contains(&dt.year()) {
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
        fn n(s: &str) -> Result<u32, Error> {
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
            i32::try_from(n(&text[..4])?).map_err(|_| Error::Validation("FIX timestamp year"))?;
        let month = time::Month::try_from(
            u8::try_from(n(&text[4..6])?).map_err(|_| Error::Validation("FIX timestamp month"))?,
        )
        .map_err(|_| Error::Validation("FIX timestamp month"))?;
        let date = time::Date::from_calendar_date(
            year,
            month,
            u8::try_from(n(&text[6..8])?).map_err(|_| Error::Validation("FIX timestamp day"))?,
        )
        .map_err(|_| Error::Validation("FIX timestamp date"))?;
        let micros = if text.len() == 17 {
            0
        } else {
            let v = n(&text[18..])?;
            if text.len() == 21 { v * 1000 } else { v }
        };
        let datetime = date
            .with_hms_micro(
                u8::try_from(n(&text[9..11])?)
                    .map_err(|_| Error::Validation("FIX timestamp hour"))?,
                u8::try_from(n(&text[12..14])?)
                    .map_err(|_| Error::Validation("FIX timestamp minute"))?,
                u8::try_from(n(&text[15..17])?)
                    .map_err(|_| Error::Validation("FIX timestamp second"))?,
                micros,
            )
            .map_err(|_| Error::Validation("FIX timestamp time"))?
            .assume_utc();
        Self::from_micros(
            i64::try_from(datetime.unix_timestamp_nanos() / 1000)
                .map_err(|_| Error::Validation("FIX timestamp range"))?,
        )
    }
    pub(super) fn wire(self) -> Result<String, Error> {
        let dt = time::OffsetDateTime::from_unix_timestamp_nanos(i128::from(self.0) * 1000)
            .map_err(|_| Error::Validation("FIX timestamp range"))?;
        Ok(format!(
            "{:04}{:02}{:02}-{:02}:{:02}:{:02}.{:06}",
            dt.year(),
            u8::from(dt.month()),
            dt.day(),
            dt.hour(),
            dt.minute(),
            dt.second(),
            dt.microsecond()
        ))
    }
}
