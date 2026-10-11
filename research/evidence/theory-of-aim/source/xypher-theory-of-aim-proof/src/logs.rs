//! Certified logarithm enclosures with rational endpoints (boundary section
//! 6).
//!
//! `ln x` for a rational `x = a/b` in `[1, 2]` is `2 atanh(u)` with
//! `u = (a - b)/(a + b)` in `[0, 1/3]`. The series
//! `sum_j u^(2j+1)/(2j+1)` is evaluated in fixed point with `WORK` fractional
//! bits, every lower-bound operation rounded down and every upper-bound
//! operation rounded up, and the truncated tail is bounded by the geometric
//! remainder `u^(2J+1) / ((2J+1)(1 - u^2))`. `ln 2` is the case `x = 2`.
//!
//! `ln n` for an integer `n >= 1` writes `n = m 2^k` with the mantissa
//! truncated to `MANTISSA` bits, `m_lo = floor(n / 2^k)` and
//! `m_hi = m_lo + [n mod 2^k != 0]`; since `ln` is monotone,
//! `ln n` lies in `[k ln 2 + ln m_lo, k ln 2 + ln m_hi]`, a width of at most
//! `1/m_lo <= 2^-(MANTISSA - 1)`. Endpoints are then rounded outward to the
//! dyadic denominator `2^PRECISION`, so that weighted sums of logarithms stay
//! integer numerators over one common denominator.

use std::sync::LazyLock;

use crate::big::Big;
use crate::rat::Rat;

/// Output precision `P`: dyadic endpoints have denominator `2^P`.
pub const PRECISION: u64 = 128;
/// Guard bits of the fixed-point series beyond `P`.
const GUARD: u64 = 64;
/// Fixed-point working precision `W = P + guard`.
const WORK: u64 = PRECISION + GUARD;
/// Bits kept from a large integer before its logarithm is taken.
const MANTISSA: u64 = 128;

/// Certified enclosure `[lo, hi]` of a real number.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enclosure {
    pub lo: Rat,
    pub hi: Rat,
}

/// Certified enclosure `[lo / 2^P, hi / 2^P]` with integer numerators.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dyadic {
    pub lo: Big,
    pub hi: Big,
}

/// `ln(a/b)` for `1 <= a/b <= 2` as integer bounds in units of `2^-WORK`.
fn ln_one_two_work(a: &Big, b: &Big) -> (Big, Big) {
    assert!(
        b.is_positive() && a >= b && *a <= b.shifted_left(1),
        "logarithm argument outside [1, 2]"
    );
    let numerator = a - b;
    if numerator.is_zero() {
        return (Big::zero(), Big::zero());
    }
    let denominator = a + b;
    let scaled = numerator.shifted_left(WORK);
    let u_lo = scaled.div_floor(&denominator);
    let u_hi = scaled.div_ceil(&denominator);
    let square_lo = (&u_lo * &u_lo).shr_floor(WORK);
    let square_hi = (&u_hi * &u_hi).shr_ceil(WORK);
    let mut power_lo = u_lo;
    let mut power_hi = u_hi;
    let mut sum_lo = Big::zero();
    let mut sum_hi = Big::zero();
    // `odd = 2j + 1` is the index of the power currently held.
    let mut odd: u64 = 1;
    let cap = 4 * WORK;
    loop {
        let divisor = Big::from_u64(odd);
        sum_lo = &sum_lo + &power_lo.div_floor(&divisor);
        sum_hi = &sum_hi + &power_hi.div_ceil(&divisor);
        power_lo = (&power_lo * &square_lo).shr_floor(WORK);
        power_hi = (&power_hi * &square_hi).shr_ceil(WORK);
        odd += 2;
        if power_hi <= Big::one() || odd > cap {
            break;
        }
    }
    // Remainder from index `odd` on, valid wherever the loop stops:
    // u^odd / (odd (1 - u^2)) <= power_hi 2^W / (odd (2^W - square_hi)).
    let one = Big::power_of_two(WORK);
    let tail_denominator = (&one - &square_hi).mul_small(odd);
    let tail = power_hi.shifted_left(WORK).div_ceil(&tail_denominator);
    sum_hi = &sum_hi + &tail;
    (sum_lo.shifted_left(1), sum_hi.shifted_left(1))
}

/// `ln 2` in units of `2^-WORK`, computed once.
static LN2_WORK: LazyLock<(Big, Big)> =
    LazyLock::new(|| ln_one_two_work(&Big::from_u64(2), &Big::one()));

/// `ln m` for an integer `m >= 1` in units of `2^-WORK`:
/// `m = 2^(t-1) x` with `x` in `[1, 2)`.
fn ln_mantissa_work(m: &Big) -> (Big, Big) {
    let top = m.bits() - 1;
    let (x_lo, x_hi) = ln_one_two_work(m, &Big::power_of_two(top));
    let (two_lo, two_hi) = &*LN2_WORK;
    (
        &x_lo + &two_lo.mul_small(top),
        &x_hi + &two_hi.mul_small(top),
    )
}

/// Certified enclosure of `ln n` for an integer `n >= 1`.
pub fn ln_integer(n: &Big) -> Dyadic {
    assert!(n.is_positive(), "logarithm of a nonpositive integer");
    let bits = n.bits();
    let (lo, hi) = if bits <= MANTISSA {
        ln_mantissa_work(n)
    } else {
        let shift = bits - MANTISSA;
        let m_lo = n.shr_floor(shift);
        let m_hi = if n.trailing_zeros() >= shift {
            m_lo.clone()
        } else {
            &m_lo + &Big::one()
        };
        let (two_lo, two_hi) = &*LN2_WORK;
        let (lo, _) = ln_mantissa_work(&m_lo);
        let (_, hi) = ln_mantissa_work(&m_hi);
        (
            &lo + &two_lo.mul_small(shift),
            &hi + &two_hi.mul_small(shift),
        )
    };
    Dyadic {
        lo: lo.shr_floor(GUARD),
        hi: hi.shr_ceil(GUARD),
    }
}

impl Dyadic {
    pub fn zero() -> Self {
        Self {
            lo: Big::zero(),
            hi: Big::zero(),
        }
    }

    /// `self += weight * term` for an integer weight of either sign.
    pub fn add_scaled(&mut self, weight: &Big, term: &Dyadic) {
        if weight.is_negative() {
            self.lo = &self.lo + &(weight * &term.hi);
            self.hi = &self.hi + &(weight * &term.lo);
        } else {
            self.lo = &self.lo + &(weight * &term.lo);
            self.hi = &self.hi + &(weight * &term.hi);
        }
    }

    /// `[lo / 2^P, hi / 2^P]` with rational endpoints.
    pub fn enclosure(&self) -> Enclosure {
        Enclosure {
            lo: Rat::dyadic(self.lo.clone(), PRECISION),
            hi: Rat::dyadic(self.hi.clone(), PRECISION),
        }
    }

    /// `-self / divisor` for a positive integer divisor, rounded outward to
    /// `2^-P`.
    pub fn negated_over(&self, divisor: &Big) -> Enclosure {
        assert!(divisor.is_positive(), "divisor must be positive");
        Enclosure {
            lo: Rat::dyadic((-&self.hi).div_floor(divisor), PRECISION),
            hi: Rat::dyadic((-&self.lo).div_ceil(divisor), PRECISION),
        }
    }
}

impl Enclosure {
    pub fn point(value: Rat) -> Self {
        Self {
            lo: value.clone(),
            hi: value,
        }
    }

    pub fn width(&self) -> Rat {
        &self.hi - &self.lo
    }

    pub fn minus(&self, other: &Self) -> Self {
        Self {
            lo: &self.lo - &other.hi,
            hi: &self.hi - &other.lo,
        }
    }

    pub fn times(&self, other: &Self) -> Self {
        let products = [
            &self.lo * &other.lo,
            &self.lo * &other.hi,
            &self.hi * &other.lo,
            &self.hi * &other.hi,
        ];
        let lo = products.iter().min().cloned().unwrap_or_else(Rat::zero);
        let hi = products.iter().max().cloned().unwrap_or_else(Rat::zero);
        Self { lo, hi }
    }

    /// `self / other`; `None` unless `other` excludes zero.
    pub fn divided_by(&self, other: &Self) -> Option<Self> {
        if !(other.lo.is_positive() || other.hi.is_negative()) {
            return None;
        }
        let one = Rat::one();
        let reciprocal = Self {
            lo: &one / &other.hi,
            hi: &one / &other.lo,
        };
        Some(self.times(&reciprocal))
    }

    /// Endpoints rounded outward to the dyadic denominator `2^P`.
    pub fn round_outward(&self) -> Self {
        Self {
            lo: Rat::dyadic(self.lo.floor_scaled(PRECISION), PRECISION),
            hi: Rat::dyadic(self.hi.ceil_scaled(PRECISION), PRECISION),
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.lo <= other.hi && other.lo <= self.hi
    }

    /// Whether `lo > a` and `hi < b`.
    pub fn strictly_inside(&self, a: &Rat, b: &Rat) -> bool {
        self.lo > *a && self.hi < *b
    }

    /// `[floor(lo), ceil(hi)]` to `places` decimals (still certified).
    pub fn render(&self, places: u32) -> String {
        format!(
            "[{}, {}]",
            self.lo.floor_decimal(places),
            self.hi.ceil_decimal(places)
        )
    }

    /// The common rounding of both endpoints to `places` decimals, when it is
    /// certified (both endpoints round to the same decimal).
    pub fn certified_rounding(&self, places: u32) -> Option<String> {
        let lo = self.lo.round_decimal(places);
        (lo == self.hi.round_decimal(places)).then_some(lo)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Whether the enclosure meets `[t, t + 10^-40]`, which holds the true
    /// value when `t` is its 40-digit truncation.
    fn meets_truncation(dyadic: &Dyadic, truncation: &str) -> bool {
        let t = Rat::new(
            Big::parse(truncation).expect("decimal digits"),
            Big::from_u64(10).pow(40),
        );
        let ulp = Rat::new(Big::one(), Big::from_u64(10).pow(40));
        dyadic.enclosure().intersects(&Enclosure {
            lo: t.clone(),
            hi: &t + &ulp,
        })
    }

    #[test]
    fn small_logarithms_are_enclosed_tightly() {
        let ln2 = ln_integer(&Big::from_u64(2));
        let ln3 = ln_integer(&Big::from_u64(3));
        assert!(meets_truncation(
            &ln2,
            "6931471805599453094172321214581765680755"
        ));
        assert!(meets_truncation(
            &ln3,
            "10986122886681096913952452369225257046474"
        ));
        let bound = Rat::dyadic(Big::from_u64(8), PRECISION);
        assert!(ln2.enclosure().width() <= bound);
        assert!(ln3.enclosure().width() <= bound);
        assert_eq!(ln_integer(&Big::one()), Dyadic::zero());
    }

    #[test]
    fn large_logarithms_respect_monotonicity_and_width() {
        // ln(2^4000) = 4000 ln 2 and ln(3^2000) = 2000 ln 3.
        let power_two = ln_integer(&Big::power_of_two(4000)).enclosure();
        let ln2 = ln_integer(&Big::from_u64(2)).enclosure();
        assert!(power_two.lo <= &ln2.hi * &Rat::from_i64(4000));
        assert!(&ln2.lo * &Rat::from_i64(4000) <= power_two.hi);
        let power_three = ln_integer(&Big::from_u64(3).pow(2000)).enclosure();
        let ln3 = ln_integer(&Big::from_u64(3)).enclosure();
        assert!(power_three.lo <= &ln3.hi * &Rat::from_i64(2000));
        assert!(&ln3.lo * &Rat::from_i64(2000) <= power_three.hi);
        let bound = Rat::dyadic(Big::one(), 120);
        assert!(power_three.width() <= bound);
    }
}
