//! Reduced rationals over `Big` (boundary section 6): the denominator is
//! positive and coprime to the numerator, so equality is structural and
//! comparison is exact cross-multiplication.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::big::Big;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rat {
    num: Big,
    den: Big,
}

impl Rat {
    /// `num / den`, reduced; panics on a zero denominator.
    pub fn new(num: Big, den: Big) -> Self {
        assert!(!den.is_zero(), "zero denominator");
        let (num, den) = if den.is_negative() {
            (-num, -den)
        } else {
            (num, den)
        };
        if num.is_zero() {
            return Self::zero();
        }
        let common = num.gcd(&den);
        if common == Big::one() {
            Self { num, den }
        } else {
            Self {
                num: num.divexact(&common),
                den: den.divexact(&common),
            }
        }
    }

    pub fn integer(value: Big) -> Self {
        Self {
            num: value,
            den: Big::one(),
        }
    }

    pub fn from_i64(value: i64) -> Self {
        Self::integer(Big::from_i64(value))
    }

    pub fn ratio(num: i64, den: i64) -> Self {
        Self::new(Big::from_i64(num), Big::from_i64(den))
    }

    pub fn zero() -> Self {
        Self::integer(Big::zero())
    }

    pub fn one() -> Self {
        Self::integer(Big::one())
    }

    /// `numerator / 2^bits`.
    pub fn dyadic(numerator: Big, bits: u64) -> Self {
        Self::new(numerator, Big::power_of_two(bits))
    }

    /// Parses `a/b` or `a` (decimal integers).
    pub fn parse(text: &str) -> Option<Self> {
        match text.split_once('/') {
            Some((num, den)) => {
                let den = Big::parse(den)?;
                if den.is_zero() {
                    return None;
                }
                Some(Self::new(Big::parse(num)?, den))
            }
            None => Some(Self::integer(Big::parse(text)?)),
        }
    }

    pub fn numer(&self) -> &Big {
        &self.num
    }

    pub fn denom(&self) -> &Big {
        &self.den
    }

    pub fn is_zero(&self) -> bool {
        self.num.is_zero()
    }

    pub fn is_positive(&self) -> bool {
        self.num.is_positive()
    }

    pub fn is_negative(&self) -> bool {
        self.num.is_negative()
    }

    /// `floor(self * 2^bits)`.
    pub fn floor_scaled(&self, bits: u64) -> Big {
        self.num.shifted_left(bits).div_floor(&self.den)
    }

    /// `ceil(self * 2^bits)`.
    pub fn ceil_scaled(&self, bits: u64) -> Big {
        self.num.shifted_left(bits).div_ceil(&self.den)
    }

    /// Decimal rounding to `places` digits, half away from zero.
    pub fn round_decimal(&self, places: u32) -> String {
        let scale = Big::from_u64(10).pow(places);
        let twice = &(&self.num.abs() * &scale).shifted_left(1) + &self.den;
        let magnitude = twice.div_floor(&self.den.shifted_left(1));
        let rounded = if self.num.is_negative() {
            -magnitude
        } else {
            magnitude
        };
        render_scaled(&rounded, places)
    }

    /// Largest decimal with `places` digits that is `<= self`.
    pub fn floor_decimal(&self, places: u32) -> String {
        let scale = Big::from_u64(10).pow(places);
        render_scaled(&(&self.num * &scale).div_floor(&self.den), places)
    }

    /// Smallest decimal with `places` digits that is `>= self`.
    pub fn ceil_decimal(&self, places: u32) -> String {
        let scale = Big::from_u64(10).pow(places);
        render_scaled(&(&self.num * &scale).div_ceil(&self.den), places)
    }
}

/// Renders `value / 10^places` with exactly `places` decimals.
fn render_scaled(value: &Big, places: u32) -> String {
    let digits = value.abs().to_string();
    let places = places as usize;
    let padded = if digits.len() <= places {
        format!("{}{digits}", "0".repeat(places + 1 - digits.len()))
    } else {
        digits
    };
    let split = padded.len() - places;
    let sign = if value.is_negative() { "-" } else { "" };
    if places == 0 {
        format!("{sign}{padded}")
    } else {
        format!("{sign}{}.{}", &padded[..split], &padded[split..])
    }
}

impl Add<&Rat> for &Rat {
    type Output = Rat;
    fn add(self, rhs: &Rat) -> Rat {
        Rat::new(
            &(&self.num * &rhs.den) + &(&rhs.num * &self.den),
            &self.den * &rhs.den,
        )
    }
}

impl Sub<&Rat> for &Rat {
    type Output = Rat;
    fn sub(self, rhs: &Rat) -> Rat {
        Rat::new(
            &(&self.num * &rhs.den) - &(&rhs.num * &self.den),
            &self.den * &rhs.den,
        )
    }
}

impl Mul<&Rat> for &Rat {
    type Output = Rat;
    fn mul(self, rhs: &Rat) -> Rat {
        Rat::new(&self.num * &rhs.num, &self.den * &rhs.den)
    }
}

impl Div<&Rat> for &Rat {
    type Output = Rat;
    /// Panics when `rhs` is zero; callers test the divisor first.
    fn div(self, rhs: &Rat) -> Rat {
        Rat::new(&self.num * &rhs.den, &self.den * &rhs.num)
    }
}

impl Neg for &Rat {
    type Output = Rat;
    fn neg(self) -> Rat {
        Rat {
            num: -&self.num,
            den: self.den.clone(),
        }
    }
}

impl Ord for Rat {
    fn cmp(&self, other: &Self) -> Ordering {
        (&self.num * &other.den).cmp(&(&other.num * &self.den))
    }
}

impl PartialOrd for Rat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Rat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.den == Big::one() {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reduction_sign_and_order() {
        let half = Rat::ratio(-3, -6);
        assert_eq!(half.to_string(), "1/2");
        assert_eq!(Rat::ratio(4, -6).to_string(), "-2/3");
        assert_eq!(&Rat::ratio(1, 3) + &Rat::ratio(1, 6), half);
        assert!(Rat::ratio(2, 3) > Rat::ratio(3, 5));
        assert_eq!(Rat::parse("2531/8624"), Some(Rat::ratio(2531, 8624)));
        assert_eq!(Rat::parse("2531/0"), None);
    }

    #[test]
    fn decimal_rounding_is_half_away_from_zero() {
        assert_eq!(Rat::ratio(194887, 250096).round_decimal(4), "0.7792");
        assert_eq!(Rat::ratio(1, 8).round_decimal(2), "0.13");
        assert_eq!(Rat::ratio(-1, 8).round_decimal(2), "-0.13");
        assert_eq!(Rat::ratio(-1, 1000).round_decimal(2), "0.00");
        assert_eq!(Rat::ratio(17, 6).round_decimal(4), "2.8333");
        assert_eq!(Rat::ratio(-17, 6).floor_decimal(3), "-2.834");
        assert_eq!(Rat::ratio(-17, 6).ceil_decimal(3), "-2.833");
    }
}
