//! Exact logarithms without evaluating a logarithm (boundary section 6).
//!
//! A positive rational `q` is represented by its prime-exponent vector
//! `e(q)`: `ln q = sum_p e_p(q) ln p`. An entropy `S = -sum q ln q` of a law
//! with rational atoms is the rational-coefficient vector `-sum_q q e(q)` over
//! `ln(prime)`. Two such vectors denote the same real number if and only if
//! their coefficients agree, because the logarithms of distinct primes are
//! linearly independent over the rationals. Products such as
//! `N^N / prod_y n_y^(n_y)` are compared as exponent vectors in the same way.

use std::collections::BTreeMap;
use std::fmt;

use crate::ratio::Ratio;

/// Rational-coefficient vector over `ln p`, keyed by prime. Zero
/// coefficients are never stored, so equality is coefficient equality.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct LogVector {
    coefficients: BTreeMap<u128, Ratio>,
}

impl LogVector {
    /// `ln 1 = 0`.
    pub(crate) fn zero() -> Self {
        Self::default()
    }

    /// Prime-exponent vector of a positive integer, by trial division.
    pub(crate) fn of_integer(value: u128) -> Self {
        assert!(value >= 1, "logarithm of a non-positive integer");
        let mut vector = Self::zero();
        let mut rest = value;
        let mut prime = 2_u128;
        while prime * prime <= rest {
            let mut exponent = 0_i128;
            while rest.is_multiple_of(prime) {
                rest /= prime;
                exponent += 1;
            }
            vector.accumulate(prime, Ratio::integer(exponent));
            prime += 1;
        }
        if rest > 1 {
            vector.accumulate(rest, Ratio::ONE);
        }
        vector
    }

    /// Prime-exponent vector `e(q)` of a positive rational.
    pub(crate) fn of_ratio(value: Ratio) -> Self {
        assert!(value.is_positive(), "logarithm of a non-positive rational");
        let numerator = Self::of_integer(value.numerator() as u128);
        let denominator = Self::of_integer(value.denominator() as u128);
        numerator.minus(&denominator)
    }

    /// Shannon entropy `-sum q ln q` of a law given by its atoms; zero atoms
    /// contribute nothing.
    pub(crate) fn entropy(atoms: impl IntoIterator<Item = Ratio>) -> Self {
        let mut vector = Self::zero();
        for atom in atoms {
            if atom.is_zero() {
                continue;
            }
            vector = vector.plus(&Self::of_ratio(atom).scaled(Ratio::ZERO - atom));
        }
        vector
    }

    fn accumulate(&mut self, prime: u128, coefficient: Ratio) {
        if coefficient.is_zero() {
            return;
        }
        let entry = self.coefficients.entry(prime).or_insert(Ratio::ZERO);
        *entry = *entry + coefficient;
        if entry.is_zero() {
            self.coefficients.remove(&prime);
        }
    }

    pub(crate) fn plus(&self, other: &Self) -> Self {
        let mut sum = self.clone();
        for (&prime, &coefficient) in &other.coefficients {
            sum.accumulate(prime, coefficient);
        }
        sum
    }

    pub(crate) fn minus(&self, other: &Self) -> Self {
        self.plus(&other.scaled(Ratio::ZERO - Ratio::ONE))
    }

    pub(crate) fn scaled(&self, factor: Ratio) -> Self {
        let mut scaled = Self::zero();
        for (&prime, &coefficient) in &self.coefficients {
            scaled.accumulate(prime, coefficient * factor);
        }
        scaled
    }

    /// `exp` of the vector as an exact rational when every coefficient is an
    /// integer (display only; acceptance compares vectors).
    pub(crate) fn exp_value(&self) -> Option<Ratio> {
        let mut value = Ratio::ONE;
        for (&prime, &coefficient) in &self.coefficients {
            if !coefficient.is_integer() {
                return None;
            }
            value = value * Ratio::power(prime as i128, coefficient.numerator() as i64);
        }
        Some(value)
    }
}

impl fmt::Display for LogVector {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(value) = self.exp_value() {
            return write!(formatter, "ln({value})");
        }
        let terms: Vec<String> = self
            .coefficients
            .iter()
            .map(|(prime, coefficient)| format!("({coefficient})ln{prime}"))
            .collect();
        write!(formatter, "{}", terms.join("+"))
    }
}

/// `ln[N^N / prod_y n_y^(n_y)] = N ln N - sum_y n_y ln n_y` over the
/// destinations with `n_y > 0`.
pub(crate) fn plan_information_target(total: i128, counts: &[i128]) -> LogVector {
    let mut vector = LogVector::of_integer(total as u128).scaled(Ratio::integer(total));
    for &count in counts.iter().filter(|&&count| count > 0) {
        vector = vector.minus(&LogVector::of_integer(count as u128).scaled(Ratio::integer(count)));
    }
    vector
}

/// `ln[|D|^|D|] = |D| ln |D|`.
pub(crate) fn capacity_target(reachable: usize) -> LogVector {
    LogVector::of_integer(reachable as u128).scaled(Ratio::integer(reachable as i128))
}
