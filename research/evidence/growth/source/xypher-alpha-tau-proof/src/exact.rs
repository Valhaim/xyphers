use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Ratio {
    numerator: i128,
    denominator: i128,
}

impl Ratio {
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

    pub const ZERO: Self = Self {
        numerator: 0,
        denominator: 1,
    };

    pub const ONE: Self = Self {
        numerator: 1,
        denominator: 1,
    };

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

    pub const fn is_integer(self) -> bool {
        self.denominator == 1
    }

    pub const fn signum(self) -> i8 {
        if self.numerator < 0 {
            -1
        } else if self.numerator > 0 {
            1
        } else {
            0
        }
    }

    pub fn abs(self) -> Self {
        if self.numerator < 0 {
            -self
        } else {
            self
        }
    }

    pub fn powi(self, exponent: i32) -> Self {
        if exponent == 0 {
            return Self::ONE;
        }
        let magnitude = exponent.unsigned_abs();
        let numerator = checked_pow_i128(self.numerator, magnitude);
        let denominator = checked_pow_i128(self.denominator, magnitude);
        if exponent > 0 {
            Self::new(numerator, denominator)
        } else {
            Self::new(denominator, numerator)
        }
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactLog {
    exponents: BTreeMap<u128, i32>,
}

impl ExactLog {
    pub fn zero() -> Self {
        Self {
            exponents: BTreeMap::new(),
        }
    }

    pub fn of_positive_integer(value: i128) -> Self {
        Self::of_positive_ratio(Ratio::integer(value))
    }

    pub fn of_positive_ratio(value: Ratio) -> Self {
        assert!(
            value.is_positive(),
            "a logarithm requires a positive argument"
        );
        let mut exponents = BTreeMap::new();
        factor_into(value.numerator() as u128, 1, &mut exponents);
        factor_into(value.denominator() as u128, -1, &mut exponents);
        exponents.retain(|_, exponent| *exponent != 0);
        Self { exponents }
    }

    pub fn base_power(base: u64, exponent: i32) -> Self {
        Self::of_positive_integer(base as i128).scaled(exponent)
    }

    pub fn is_zero(&self) -> bool {
        self.exponents.is_empty()
    }

    pub fn scaled(&self, factor: i32) -> Self {
        if factor == 0 {
            return Self::zero();
        }
        let exponents = self
            .exponents
            .iter()
            .map(|(&prime, &exponent)| {
                (
                    prime,
                    exponent.checked_mul(factor).expect("log exponent overflow"),
                )
            })
            .collect();
        Self { exponents }
    }

    pub fn plus(&self, other: &Self) -> Self {
        let mut exponents = self.exponents.clone();
        for (&prime, &exponent) in &other.exponents {
            let next = exponents
                .get(&prime)
                .copied()
                .unwrap_or(0)
                .checked_add(exponent)
                .expect("log exponent overflow");
            if next == 0 {
                exponents.remove(&prime);
            } else {
                exponents.insert(prime, next);
            }
        }
        Self { exponents }
    }

    pub fn minus(&self, other: &Self) -> Self {
        self.plus(&other.scaled(-1))
    }

    pub fn negated(&self) -> Self {
        self.scaled(-1)
    }

    pub fn signum(&self) -> i8 {
        match self.to_ratio().cmp(&Ratio::ONE) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    pub fn to_ratio(&self) -> Ratio {
        let mut numerator = 1_i128;
        let mut denominator = 1_i128;
        for (&prime, &exponent) in &self.exponents {
            let prime = i128::try_from(prime).expect("prime does not fit i128");
            let power = checked_pow_i128(prime, exponent.unsigned_abs());
            if exponent > 0 {
                numerator = numerator
                    .checked_mul(power)
                    .expect("exact log numerator overflow");
            } else {
                denominator = denominator
                    .checked_mul(power)
                    .expect("exact log denominator overflow");
            }
        }
        Ratio::new(numerator, denominator)
    }

    pub fn proportional_factor(&self, basis: &Self) -> Option<Ratio> {
        if basis.is_zero() {
            return self.is_zero().then_some(Ratio::ZERO);
        }
        if self.is_zero() {
            return Some(Ratio::ZERO);
        }
        let (&pivot, &basis_exponent) = basis.exponents.iter().next()?;
        let factor = Ratio::new(
            self.exponents.get(&pivot).copied().unwrap_or(0) as i128,
            basis_exponent as i128,
        );
        let mut primes = BTreeMap::new();
        for &prime in self.exponents.keys().chain(basis.exponents.keys()) {
            primes.insert(prime, ());
        }
        for prime in primes.keys() {
            let left = Ratio::integer(self.exponents.get(prime).copied().unwrap_or(0) as i128);
            let right =
                Ratio::integer(basis.exponents.get(prime).copied().unwrap_or(0) as i128) * factor;
            if left != right {
                return None;
            }
        }
        Some(factor)
    }

    fn primitive_positive(&self) -> (i128, Self) {
        assert!(!self.is_zero(), "zero cannot be a temperature denominator");
        let sign = self.signum() as i128;
        let oriented = if sign < 0 {
            self.negated()
        } else {
            self.clone()
        };
        let divisor = oriented
            .exponents
            .values()
            .fold(0_u128, |accumulator, exponent| {
                gcd(accumulator, exponent.unsigned_abs() as u128)
            })
            .max(1) as i32;
        let exponents = oriented
            .exponents
            .iter()
            .map(|(&prime, &exponent)| (prime, exponent / divisor))
            .collect();
        (sign * divisor as i128, Self { exponents })
    }
}

impl fmt::Display for ExactLog {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            write!(formatter, "0")
        } else {
            write!(formatter, "ln({})", self.to_ratio())
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Temperature {
    numerator: Ratio,
    denominator: ExactLog,
}

impl Temperature {
    pub fn new(numerator: Ratio, denominator: ExactLog) -> Self {
        assert!(
            !denominator.is_zero(),
            "temperature denominator cannot be zero"
        );
        let (factor, denominator) = denominator.primitive_positive();
        Self {
            numerator: numerator / Ratio::integer(factor),
            denominator,
        }
    }

    pub fn from_base(lambda: Ratio, base: u64) -> Self {
        assert!(base >= 2, "reservoir base must be at least two");
        Self::new(lambda, ExactLog::of_positive_integer(base as i128))
    }

    pub fn numerator(&self) -> Ratio {
        self.numerator
    }

    pub fn denominator(&self) -> &ExactLog {
        &self.denominator
    }

    pub fn is_positive(&self) -> bool {
        self.numerator.is_positive()
    }

    pub fn scaled_energy(&self, factor: Ratio) -> Self {
        Self::new(self.numerator * factor, self.denominator.clone())
    }

    pub fn energy_for_log(&self, logarithm: &ExactLog) -> Option<Ratio> {
        Some(self.numerator * logarithm.proportional_factor(&self.denominator)?)
    }

    pub fn dimensionless_energy(&self, energy: Ratio) -> Option<ExactLog> {
        let coefficient = energy / self.numerator;
        if !coefficient.is_integer() {
            return None;
        }
        let exponent = i32::try_from(coefficient.numerator()).ok()?;
        Some(self.denominator.scaled(exponent))
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.numerator == Ratio::ONE {
            write!(formatter, "1/{}", self.denominator)
        } else if self.numerator.denominator() == 1 {
            write!(formatter, "{}/{}", self.numerator, self.denominator)
        } else {
            write!(formatter, "({})/{}", self.numerator, self.denominator)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScaledLog {
    pub temperature: Temperature,
    pub logarithm: ExactLog,
}

impl ScaledLog {
    pub fn new(temperature: Temperature, logarithm: ExactLog) -> Self {
        Self {
            temperature,
            logarithm,
        }
    }

    pub fn zero(temperature: Temperature) -> Self {
        Self::new(temperature, ExactLog::zero())
    }

    pub fn scaled_energy(&self, factor: Ratio) -> Self {
        Self::new(
            self.temperature.scaled_energy(factor),
            self.logarithm.clone(),
        )
    }

    pub fn negated(&self) -> Self {
        Self::new(self.temperature.clone(), self.logarithm.negated())
    }

    pub fn exact_energy(&self) -> Option<Ratio> {
        self.temperature.energy_for_log(&self.logarithm)
    }
}

impl fmt::Display for ScaledLog {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({})*{}", self.temperature, self.logarithm)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TemperatureSet {
    Empty,
    AllPositive,
    Singleton(Temperature),
}

impl TemperatureSet {
    pub fn intersect(&self, other: &Self) -> Self {
        match (self, other) {
            (Self::Empty, _) | (_, Self::Empty) => Self::Empty,
            (Self::AllPositive, right) => right.clone(),
            (left, Self::AllPositive) => left.clone(),
            (Self::Singleton(left), Self::Singleton(right)) if left == right => {
                Self::Singleton(left.clone())
            }
            (Self::Singleton(_), Self::Singleton(_)) => Self::Empty,
        }
    }
}

impl fmt::Display for TemperatureSet {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(formatter, "EMPTY"),
            Self::AllPositive => write!(formatter, "ALL_POSITIVE"),
            Self::Singleton(temperature) => write!(formatter, "{{{temperature}}}"),
        }
    }
}

fn factor_into(mut value: u128, sign: i32, exponents: &mut BTreeMap<u128, i32>) {
    assert!(value > 0, "factorization requires a positive integer");
    let mut prime = 2_u128;
    while prime <= value / prime {
        while value % prime == 0 {
            let next = exponents
                .get(&prime)
                .copied()
                .unwrap_or(0)
                .checked_add(sign)
                .expect("log exponent overflow");
            exponents.insert(prime, next);
            value /= prime;
        }
        prime = if prime == 2 { 3 } else { prime + 2 };
    }
    if value > 1 {
        let next = exponents
            .get(&value)
            .copied()
            .unwrap_or(0)
            .checked_add(sign)
            .expect("log exponent overflow");
        exponents.insert(value, next);
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

fn checked_pow_i128(mut base: i128, mut exponent: u32) -> i128 {
    let mut result = 1_i128;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result.checked_mul(base).expect("integer power overflow");
        }
        exponent >>= 1;
        if exponent > 0 {
            base = base.checked_mul(base).expect("integer power overflow");
        }
    }
    result
}
