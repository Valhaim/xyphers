use std::cmp::Ordering;
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Ratio {
    numerator: i128,
    denominator: i128,
}

impl Ratio {
    pub const ZERO: Self = Self {
        numerator: 0,
        denominator: 1,
    };

    pub const ONE: Self = Self {
        numerator: 1,
        denominator: 1,
    };

    pub fn new(mut numerator: i128, mut denominator: i128) -> Self {
        assert_ne!(denominator, 0, "a ratio cannot have a zero denominator");
        if denominator < 0 {
            numerator = numerator.checked_neg().expect("ratio numerator overflow");
            denominator = denominator
                .checked_neg()
                .expect("ratio denominator overflow");
        }
        if numerator == 0 {
            return Self::ZERO;
        }
        let divisor = gcd(numerator.unsigned_abs(), denominator as u128) as i128;
        Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        }
    }

    pub const fn integer(value: i128) -> Self {
        Self {
            numerator: value,
            denominator: 1,
        }
    }

    pub const fn numerator(self) -> i128 {
        self.numerator
    }

    pub const fn denominator(self) -> i128 {
        self.denominator
    }

    pub const fn is_zero(self) -> bool {
        self.numerator == 0
    }

    pub const fn is_positive(self) -> bool {
        self.numerator > 0
    }

    pub const fn is_nonnegative(self) -> bool {
        self.numerator >= 0
    }

    pub const fn is_integer(self) -> bool {
        self.denominator == 1
    }

    pub fn reciprocal(self) -> Self {
        assert!(!self.is_zero(), "zero has no reciprocal");
        Self::new(self.denominator, self.numerator)
    }

    pub fn abs(self) -> Self {
        if self.numerator < 0 {
            -self
        } else {
            self
        }
    }

    pub fn powu(self, mut exponent: u32) -> Self {
        let mut base = self;
        let mut result = Self::ONE;
        while exponent > 0 {
            if exponent & 1 == 1 {
                result = result * base;
            }
            exponent >>= 1;
            if exponent > 0 {
                base = base * base;
            }
        }
        result
    }
}

impl From<i128> for Ratio {
    fn from(value: i128) -> Self {
        Self::integer(value)
    }
}

impl Add for Ratio {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let left = self
            .numerator
            .checked_mul(rhs.denominator)
            .expect("ratio addition overflow");
        let right = rhs
            .numerator
            .checked_mul(self.denominator)
            .expect("ratio addition overflow");
        Self::new(
            left.checked_add(right).expect("ratio addition overflow"),
            self.denominator
                .checked_mul(rhs.denominator)
                .expect("ratio addition overflow"),
        )
    }
}

impl Sub for Ratio {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for Ratio {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator
                .checked_mul(rhs.numerator)
                .expect("ratio multiplication overflow"),
            self.denominator
                .checked_mul(rhs.denominator)
                .expect("ratio multiplication overflow"),
        )
    }
}

impl Div for Ratio {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert!(!rhs.is_zero(), "division by zero ratio");
        Self::new(
            self.numerator
                .checked_mul(rhs.denominator)
                .expect("ratio division overflow"),
            self.denominator
                .checked_mul(rhs.numerator)
                .expect("ratio division overflow"),
        )
    }
}

impl Neg for Ratio {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(
            self.numerator
                .checked_neg()
                .expect("ratio negation overflow"),
            self.denominator,
        )
    }
}

impl Sum for Ratio {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::ZERO, |sum, value| sum + value)
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ratio {
    fn cmp(&self, other: &Self) -> Ordering {
        self.numerator
            .checked_mul(other.denominator)
            .expect("ratio comparison overflow")
            .cmp(
                &other
                    .numerator
                    .checked_mul(self.denominator)
                    .expect("ratio comparison overflow"),
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
    left
}
