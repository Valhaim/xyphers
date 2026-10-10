use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

/// Exact reduced rational over `i128`. The denominator is always positive.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Ratio {
    numerator: i128,
    denominator: i128,
}

impl Ratio {
    pub(crate) const ZERO: Self = Self {
        numerator: 0,
        denominator: 1,
    };
    pub(crate) const ONE: Self = Self {
        numerator: 1,
        denominator: 1,
    };

    pub(crate) fn new(mut numerator: i128, mut denominator: i128) -> Self {
        assert_ne!(denominator, 0, "zero denominator");
        if denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
        }
        let divisor = gcd(numerator.unsigned_abs(), denominator.unsigned_abs()) as i128;
        Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        }
    }

    pub(crate) const fn integer(value: i128) -> Self {
        Self {
            numerator: value,
            denominator: 1,
        }
    }

    /// `base^exponent` for a signed integer exponent.
    pub(crate) fn power(base: i128, exponent: i64) -> Self {
        let magnitude = integer_power(base, exponent.unsigned_abs());
        if exponent >= 0 {
            Self::integer(magnitude)
        } else {
            Self::new(1, magnitude)
        }
    }

    pub(crate) const fn is_zero(self) -> bool {
        self.numerator == 0
    }

    pub(crate) const fn is_positive(self) -> bool {
        self.numerator > 0
    }

    /// Reduced numerator (sign carried here). Read by the prime-exponent
    /// factorisation of `src/primes.rs`.
    pub(crate) const fn numerator(self) -> i128 {
        self.numerator
    }

    /// Reduced, always positive denominator.
    pub(crate) const fn denominator(self) -> i128 {
        self.denominator
    }

    pub(crate) const fn is_integer(self) -> bool {
        self.denominator == 1
    }

    pub(crate) fn checked_div(self, other: Self) -> Option<Self> {
        (!other.is_zero()).then(|| self / other)
    }
}

impl Add for Ratio {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}

impl Sub for Ratio {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.denominator - other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}

impl Mul for Ratio {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

impl Div for Ratio {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }
}

impl fmt::Display for Ratio {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.denominator == 1 {
            write!(formatter, "{}", self.numerator)
        } else {
            write!(formatter, "{}/{}", self.numerator, self.denominator)
        }
    }
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

pub(crate) fn integer_power(base: i128, exponent: u64) -> i128 {
    let mut result = 1_i128;
    for _ in 0..exponent {
        result *= base;
    }
    result
}
