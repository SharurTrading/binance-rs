// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

//! Exact IEEE-754 binary32 values declared by the pinned SBE ticker schemas.
use crate::Decimal;

pub(super) fn exact(bits: u32) -> Option<Decimal> {
    let exponent = (bits >> 23) & 0xff;
    if exponent == 0xff {
        return None;
    }
    let fraction = bits & 0x7f_ffff;
    let mut coefficient = i128::from(if exponent == 0 {
        fraction
    } else {
        fraction | 0x80_0000
    });
    if coefficient == 0 {
        return Some(Decimal::ZERO);
    }
    if bits >> 31 != 0 {
        coefficient = -coefficient;
    }
    let power = if exponent == 0 {
        -149
    } else {
        i32::try_from(exponent).ok()? - 150
    };
    if power >= 0 {
        let factor = 2_i128.checked_pow(u32::try_from(power).ok()?)?;
        return Decimal::try_from_i128_with_scale(coefficient.checked_mul(factor)?, 0).ok();
    }
    let mut scale = power.unsigned_abs();
    while scale > 0 && coefficient % 2 == 0 {
        coefficient /= 2;
        scale -= 1;
    }
    if scale > 28 {
        return None;
    }
    coefficient = coefficient.checked_mul(5_i128.checked_pow(scale)?)?;
    Decimal::try_from_i128_with_scale(coefficient, scale).ok()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, reason = "synthetic IEEE wire assertions")]
mod tests {
    use super::*;
    #[test]
    fn binary32_is_exact_or_explicitly_unrepresentable() {
        assert_eq!(exact(0), Some(Decimal::ZERO));
        assert_eq!(exact(0x8000_0000), Some(Decimal::ZERO));
        assert_eq!(exact(0x3f00_0000), Some(Decimal::new(5, 1)));
        assert_eq!(exact(0xbf00_0000), Some(Decimal::new(-5, 1)));
        assert_eq!(
            exact(0x3f8c_cccd),
            Some(Decimal::from_str_exact("1.10000002384185791015625").unwrap())
        );
        for bits in [
            0x7f80_0000,
            0xff80_0000,
            0x7fc0_0000,
            1,
            0x3a83_126f,
            0x7f7f_ffff,
        ] {
            assert_eq!(exact(bits), None);
        }
    }
}
