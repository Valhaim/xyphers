//! Signed arbitrary-precision integers (boundary section 6).
//!
//! A magnitude is a little-endian vector of 64-bit limbs with no high zero
//! limb; zero is the empty magnitude and is never negative. Multiplication is
//! schoolbook over `u128` limb products. Exact division strips the common
//! factor of two and then runs the 2-adic (Jebelean) quotient, which needs
//! only the low limbs of the dividend. Floor division is binary long division
//! (one `u128` division per limb for a single-limb divisor). The gcd is binary
//! (Stein). No floating-point value is used anywhere.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Mul, Neg, Sub};

/// Signed arbitrary-precision integer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Big {
    negative: bool,
    limbs: Vec<u64>,
}

/// Reusable limb buffers for the in-place Bareiss update, so that the inner
/// elimination loop does not allocate once the buffers have grown.
#[derive(Debug, Default)]
pub struct Scratch {
    product: Vec<u64>,
    cross: Vec<u64>,
    difference: Vec<u64>,
    quotient: Vec<u64>,
    numerator: Vec<u64>,
    denominator: Vec<u64>,
}

// ------------------------------------------------------------------ magnitudes

fn trim(limbs: &mut Vec<u64>) {
    while limbs.last() == Some(&0) {
        limbs.pop();
    }
}

/// Compares two trimmed magnitudes.
fn cmp_mag(a: &[u64], b: &[u64]) -> Ordering {
    if a.len() != b.len() {
        return a.len().cmp(&b.len());
    }
    for (x, y) in a.iter().rev().zip(b.iter().rev()) {
        if x != y {
            return x.cmp(y);
        }
    }
    Ordering::Equal
}

fn bit_length(a: &[u64]) -> u64 {
    match a.last() {
        None => 0,
        Some(&top) => (a.len() as u64 - 1) * 64 + u64::from(64 - top.leading_zeros()),
    }
}

fn trailing_zero_bits(a: &[u64]) -> u64 {
    for (index, &limb) in a.iter().enumerate() {
        if limb != 0 {
            return index as u64 * 64 + u64::from(limb.trailing_zeros());
        }
    }
    0
}

/// Whether any of the lowest `bits` bits of `a` is set.
fn low_bits_nonzero(a: &[u64], bits: u64) -> bool {
    let full = (bits / 64) as usize;
    let rest = bits % 64;
    if a.iter().take(full).any(|&limb| limb != 0) {
        return true;
    }
    match a.get(full) {
        Some(&limb) if rest > 0 => limb & ((1_u64 << rest) - 1) != 0,
        _ => false,
    }
}

/// `out = a + b`.
fn add_mag_into(out: &mut Vec<u64>, a: &[u64], b: &[u64]) {
    let (long, short) = if a.len() >= b.len() { (a, b) } else { (b, a) };
    out.clear();
    out.reserve(long.len() + 1);
    let mut carry = false;
    for (index, &x) in long.iter().enumerate() {
        let y = if index < short.len() { short[index] } else { 0 };
        let (s1, c1) = x.overflowing_add(y);
        let (s2, c2) = s1.overflowing_add(u64::from(carry));
        out.push(s2);
        carry = c1 || c2;
    }
    if carry {
        out.push(1);
    }
}

/// `out = a - b` for `a >= b`.
fn sub_mag_into(out: &mut Vec<u64>, a: &[u64], b: &[u64]) {
    debug_assert!(cmp_mag(a, b) != Ordering::Less);
    out.clear();
    out.reserve(a.len());
    let mut borrow = false;
    for (index, &x) in a.iter().enumerate() {
        let y = if index < b.len() { b[index] } else { 0 };
        let (d1, b1) = x.overflowing_sub(y);
        let (d2, b2) = d1.overflowing_sub(u64::from(borrow));
        out.push(d2);
        borrow = b1 || b2;
    }
    debug_assert!(!borrow);
    trim(out);
}

/// `a -= b` in place for `a >= b`.
fn sub_mag_assign(a: &mut Vec<u64>, b: &[u64]) {
    debug_assert!(cmp_mag(a, b) != Ordering::Less);
    let mut borrow = false;
    for index in 0..a.len() {
        if index >= b.len() && !borrow {
            break;
        }
        let y = if index < b.len() { b[index] } else { 0 };
        let (d1, b1) = a[index].overflowing_sub(y);
        let (d2, b2) = d1.overflowing_sub(u64::from(borrow));
        a[index] = d2;
        borrow = b1 || b2;
    }
    debug_assert!(!borrow);
    trim(a);
}

/// `out = a * b` (schoolbook).
fn mul_mag_into(out: &mut Vec<u64>, a: &[u64], b: &[u64]) {
    out.clear();
    if a.is_empty() || b.is_empty() {
        return;
    }
    out.resize(a.len() + b.len(), 0);
    for (index, &x) in a.iter().enumerate() {
        if x == 0 {
            continue;
        }
        let mut carry = 0_u64;
        for (slot, &y) in out[index..index + b.len()].iter_mut().zip(b) {
            let t = u128::from(x) * u128::from(y) + u128::from(*slot) + u128::from(carry);
            *slot = t as u64;
            carry = (t >> 64) as u64;
        }
        // Row `index` is the first to reach this limb.
        out[index + b.len()] = carry;
    }
    trim(out);
}

fn mul_small_assign(a: &mut Vec<u64>, factor: u64) {
    if factor == 0 {
        a.clear();
        return;
    }
    let mut carry = 0_u64;
    for slot in a.iter_mut() {
        let t = u128::from(*slot) * u128::from(factor) + u128::from(carry);
        *slot = t as u64;
        carry = (t >> 64) as u64;
    }
    if carry != 0 {
        a.push(carry);
    }
}

fn add_small_assign(a: &mut Vec<u64>, addend: u64) {
    let mut carry = addend;
    for slot in a.iter_mut() {
        if carry == 0 {
            return;
        }
        let (value, overflow) = slot.overflowing_add(carry);
        *slot = value;
        carry = u64::from(overflow);
    }
    if carry != 0 {
        a.push(carry);
    }
}

/// Divides `a` in place by a nonzero single limb and returns the remainder.
fn divrem_small_assign(a: &mut Vec<u64>, divisor: u64) -> u64 {
    assert!(divisor != 0, "division by zero");
    let divisor = u128::from(divisor);
    let mut remainder = 0_u128;
    for slot in a.iter_mut().rev() {
        let current = (remainder << 64) | u128::from(*slot);
        *slot = (current / divisor) as u64;
        remainder = current % divisor;
    }
    trim(a);
    remainder as u64
}

fn shl_mag(a: &[u64], bits: u64) -> Vec<u64> {
    if a.is_empty() {
        return Vec::new();
    }
    let whole = (bits / 64) as usize;
    let rest = (bits % 64) as u32;
    let mut out = vec![0_u64; whole];
    out.reserve(a.len() + 1);
    if rest == 0 {
        out.extend_from_slice(a);
    } else {
        let mut carry = 0_u64;
        for &x in a {
            out.push((x << rest) | carry);
            carry = x >> (64 - rest);
        }
        if carry != 0 {
            out.push(carry);
        }
    }
    out
}

/// `out = floor(a / 2^bits)`.
fn shr_mag_into(out: &mut Vec<u64>, a: &[u64], bits: u64) {
    out.clear();
    let whole = (bits / 64) as usize;
    if whole >= a.len() {
        return;
    }
    let rest = (bits % 64) as u32;
    let source = &a[whole..];
    if rest == 0 {
        out.extend_from_slice(source);
    } else {
        for index in 0..source.len() {
            let high = if index + 1 < source.len() {
                source[index + 1] << (64 - rest)
            } else {
                0
            };
            out.push((source[index] >> rest) | high);
        }
    }
    trim(out);
}

/// `a = floor(a / 2^bits)` in place.
fn shr_mag_assign(a: &mut Vec<u64>, bits: u64) {
    let whole = (bits / 64) as usize;
    if whole >= a.len() {
        a.clear();
        return;
    }
    if whole > 0 {
        a.drain(..whole);
    }
    let rest = (bits % 64) as u32;
    if rest > 0 {
        let length = a.len();
        for index in 0..length {
            let high = if index + 1 < length {
                a[index + 1] << (64 - rest)
            } else {
                0
            };
            a[index] = (a[index] >> rest) | high;
        }
    }
    trim(a);
}

/// Inverse of an odd limb modulo `2^64` by Newton iteration (3, 6, 12, 24,
/// 48, 96 correct bits).
fn inverse_mod_word(odd: u64) -> u64 {
    debug_assert!(odd & 1 == 1);
    let mut inverse = odd;
    for _ in 0..5 {
        inverse = inverse.wrapping_mul(2_u64.wrapping_sub(odd.wrapping_mul(inverse)));
    }
    inverse
}

/// `out = a / b` for a division known to be exact (`b != 0`).
///
/// After removing the common power of two the divisor is odd, and the
/// quotient `q < 2^(64 L)`, `L = len(a) - len(b) + 1`, equals
/// `a * b^(-1) mod 2^(64 L)`. It is produced limb by limb from the bottom,
/// subtracting each `q_i b` from the low `L` limbs only.
fn divexact_mag_into(
    out: &mut Vec<u64>,
    a: &[u64],
    b: &[u64],
    shifted_a: &mut Vec<u64>,
    shifted_b: &mut Vec<u64>,
) {
    assert!(!b.is_empty(), "division by zero");
    out.clear();
    if a.is_empty() {
        return;
    }
    let shift = trailing_zero_bits(b);
    let (a, b): (&[u64], &[u64]) = if shift == 0 {
        (a, b)
    } else {
        shr_mag_into(shifted_a, a, shift);
        shr_mag_into(shifted_b, b, shift);
        (shifted_a.as_slice(), shifted_b.as_slice())
    };
    if a.len() < b.len() {
        // Only a zero dividend divides exactly by a longer divisor.
        return;
    }
    let length = a.len() - b.len() + 1;
    let inverse = inverse_mod_word(b[0]);
    out.extend_from_slice(&a[..length]);
    for index in 0..length {
        let digit = out[index].wrapping_mul(inverse);
        let span = (length - index).min(b.len());
        let mut carry = 0_u64;
        let mut borrow = false;
        for (offset, &limb) in b.iter().take(span).enumerate() {
            let product = u128::from(digit) * u128::from(limb) + u128::from(carry);
            carry = (product >> 64) as u64;
            let (d1, b1) = out[index + offset].overflowing_sub(product as u64);
            let (d2, b2) = d1.overflowing_sub(u64::from(borrow));
            out[index + offset] = d2;
            borrow = b1 || b2;
        }
        let mut position = index + span;
        while position < length && (carry != 0 || borrow) {
            let (d1, b1) = out[position].overflowing_sub(carry);
            let (d2, b2) = d1.overflowing_sub(u64::from(borrow));
            out[position] = d2;
            carry = 0;
            borrow = b1 || b2;
            position += 1;
        }
        out[index] = digit;
    }
    trim(out);
}

/// Truncated division of magnitudes: `a = q b + r`, `0 <= r < b`.
fn divrem_mag(a: &[u64], b: &[u64]) -> (Vec<u64>, Vec<u64>) {
    assert!(!b.is_empty(), "division by zero");
    if cmp_mag(a, b) == Ordering::Less {
        return (Vec::new(), a.to_vec());
    }
    if b.len() == 1 {
        let mut quotient = a.to_vec();
        let remainder = divrem_small_assign(&mut quotient, b[0]);
        let remainder = if remainder == 0 {
            Vec::new()
        } else {
            vec![remainder]
        };
        return (quotient, remainder);
    }
    let shift = bit_length(a) - bit_length(b);
    let mut divisor = shl_mag(b, shift);
    let mut remainder = a.to_vec();
    let mut quotient = vec![0_u64; (shift / 64) as usize + 1];
    let mut bit = shift;
    loop {
        if cmp_mag(&remainder, &divisor) != Ordering::Less {
            sub_mag_assign(&mut remainder, &divisor);
            quotient[(bit / 64) as usize] |= 1_u64 << (bit % 64);
        }
        if bit == 0 {
            break;
        }
        shr_mag_assign(&mut divisor, 1);
        bit -= 1;
    }
    trim(&mut quotient);
    (quotient, remainder)
}

/// Binary gcd of magnitudes.
fn gcd_mag(a: &[u64], b: &[u64]) -> Vec<u64> {
    if a.is_empty() {
        return b.to_vec();
    }
    if b.is_empty() {
        return a.to_vec();
    }
    let mut u = a.to_vec();
    let mut v = b.to_vec();
    let zeros_u = trailing_zero_bits(&u);
    let zeros_v = trailing_zero_bits(&v);
    shr_mag_assign(&mut u, zeros_u);
    shr_mag_assign(&mut v, zeros_v);
    loop {
        match cmp_mag(&u, &v) {
            Ordering::Equal => break,
            Ordering::Greater => {
                sub_mag_assign(&mut u, &v);
                let zeros = trailing_zero_bits(&u);
                shr_mag_assign(&mut u, zeros);
            }
            Ordering::Less => {
                sub_mag_assign(&mut v, &u);
                let zeros = trailing_zero_bits(&v);
                shr_mag_assign(&mut v, zeros);
            }
        }
    }
    shl_mag(&u, zeros_u.min(zeros_v))
}

/// `out = (-1)^a_negative a - (-1)^b_negative b`; returns whether the result
/// is negative (never for zero).
fn signed_sub_into(
    out: &mut Vec<u64>,
    a_negative: bool,
    a: &[u64],
    b_negative: bool,
    b: &[u64],
) -> bool {
    if a_negative != b_negative {
        add_mag_into(out, a, b);
        return a_negative && !out.is_empty();
    }
    if cmp_mag(a, b) == Ordering::Less {
        sub_mag_into(out, b, a);
        !a_negative
    } else {
        sub_mag_into(out, a, b);
        a_negative && !out.is_empty()
    }
}

// ------------------------------------------------------------------ integers

impl Big {
    fn from_parts(negative: bool, mut limbs: Vec<u64>) -> Self {
        trim(&mut limbs);
        let negative = negative && !limbs.is_empty();
        Self { negative, limbs }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn one() -> Self {
        Self::from_u64(1)
    }

    pub fn from_u64(value: u64) -> Self {
        Self::from_parts(false, vec![value])
    }

    pub fn from_i64(value: i64) -> Self {
        Self::from_parts(value < 0, vec![value.unsigned_abs()])
    }

    pub fn from_i128(value: i128) -> Self {
        let magnitude = value.unsigned_abs();
        Self::from_parts(value < 0, vec![magnitude as u64, (magnitude >> 64) as u64])
    }

    /// `2^bits`.
    pub fn power_of_two(bits: u64) -> Self {
        Self::from_parts(false, shl_mag(&[1], bits))
    }

    pub fn is_zero(&self) -> bool {
        self.limbs.is_empty()
    }

    pub fn is_negative(&self) -> bool {
        self.negative
    }

    pub fn is_positive(&self) -> bool {
        !self.negative && !self.limbs.is_empty()
    }

    pub fn abs(&self) -> Self {
        Self {
            negative: false,
            limbs: self.limbs.clone(),
        }
    }

    /// Bit length of the magnitude (0 for zero).
    pub fn bits(&self) -> u64 {
        bit_length(&self.limbs)
    }

    /// Number of trailing zero bits of the magnitude (0 for zero).
    pub fn trailing_zeros(&self) -> u64 {
        trailing_zero_bits(&self.limbs)
    }

    /// The value, when it is a nonnegative integer below `2^64`.
    pub fn to_u64(&self) -> Option<u64> {
        if self.negative {
            return None;
        }
        match self.limbs.as_slice() {
            [] => Some(0),
            [limb] => Some(*limb),
            _ => None,
        }
    }

    /// `self * 2^bits`.
    pub fn shifted_left(&self, bits: u64) -> Self {
        Self::from_parts(self.negative, shl_mag(&self.limbs, bits))
    }

    /// `floor(self / 2^bits)`.
    pub fn shr_floor(&self, bits: u64) -> Self {
        let mut out = Vec::new();
        shr_mag_into(&mut out, &self.limbs, bits);
        if self.negative && low_bits_nonzero(&self.limbs, bits) {
            add_small_assign(&mut out, 1);
        }
        Self::from_parts(self.negative, out)
    }

    /// `ceil(self / 2^bits)`.
    pub fn shr_ceil(&self, bits: u64) -> Self {
        -(-self).shr_floor(bits)
    }

    pub fn pow(&self, exponent: u32) -> Self {
        let mut result = Self::one();
        let mut base = self.clone();
        let mut rest = exponent;
        while rest > 0 {
            if rest & 1 == 1 {
                result = &result * &base;
            }
            rest >>= 1;
            if rest > 0 {
                base = &base * &base;
            }
        }
        result
    }

    /// `self * factor` for a single-limb factor.
    pub fn mul_small(&self, factor: u64) -> Self {
        let mut limbs = self.limbs.clone();
        mul_small_assign(&mut limbs, factor);
        Self::from_parts(self.negative, limbs)
    }

    /// `self / divisor` for a division known to be exact.
    pub fn divexact(&self, divisor: &Self) -> Self {
        let mut out = Vec::new();
        let mut shifted_a = Vec::new();
        let mut shifted_b = Vec::new();
        divexact_mag_into(
            &mut out,
            &self.limbs,
            &divisor.limbs,
            &mut shifted_a,
            &mut shifted_b,
        );
        Self::from_parts(self.negative != divisor.negative, out)
    }

    /// Floor division: `self = q divisor + r` with `q = floor(self /
    /// divisor)`; `r` has the sign of the divisor (or is zero).
    pub fn div_rem_floor(&self, divisor: &Self) -> (Self, Self) {
        let (quotient, remainder) = divrem_mag(&self.limbs, &divisor.limbs);
        let quotient_negative = self.negative != divisor.negative;
        let quotient = Self::from_parts(quotient_negative, quotient);
        let remainder = Self::from_parts(self.negative, remainder);
        if quotient_negative && !remainder.is_zero() {
            (&quotient - &Self::one(), &remainder + divisor)
        } else {
            (quotient, remainder)
        }
    }

    /// `floor(self / divisor)`.
    pub fn div_floor(&self, divisor: &Self) -> Self {
        self.div_rem_floor(divisor).0
    }

    /// `ceil(self / divisor)`.
    pub fn div_ceil(&self, divisor: &Self) -> Self {
        -(-self).div_floor(divisor)
    }

    /// Nonnegative greatest common divisor (`gcd(0, 0) = 0`).
    pub fn gcd(&self, other: &Self) -> Self {
        Self::from_parts(false, gcd_mag(&self.limbs, &other.limbs))
    }

    /// Parses an optionally signed decimal integer.
    pub fn parse(text: &str) -> Option<Self> {
        let (negative, digits) = match text.strip_prefix('-') {
            Some(rest) => (true, rest),
            None => (false, text),
        };
        if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }
        let mut limbs = Vec::new();
        for chunk in digits.as_bytes().chunks(19) {
            let value: u64 = std::str::from_utf8(chunk).ok()?.parse().ok()?;
            mul_small_assign(&mut limbs, 10_u64.pow(chunk.len() as u32));
            add_small_assign(&mut limbs, value);
        }
        Some(Self::from_parts(negative, limbs))
    }

    /// Bareiss update `self <- (pivot * self - factor * upper) / previous`,
    /// where the division is exact, reusing the scratch buffers.
    pub fn bareiss_update(
        &mut self,
        pivot: &Self,
        factor: &Self,
        upper: &Self,
        previous: &Self,
        scratch: &mut Scratch,
    ) {
        mul_mag_into(&mut scratch.product, &pivot.limbs, &self.limbs);
        let product_negative = pivot.negative != self.negative;
        mul_mag_into(&mut scratch.cross, &factor.limbs, &upper.limbs);
        let cross_negative = factor.negative != upper.negative;
        let negative = signed_sub_into(
            &mut scratch.difference,
            product_negative,
            &scratch.product,
            cross_negative,
            &scratch.cross,
        );
        divexact_mag_into(
            &mut scratch.quotient,
            &scratch.difference,
            &previous.limbs,
            &mut scratch.numerator,
            &mut scratch.denominator,
        );
        std::mem::swap(&mut self.limbs, &mut scratch.quotient);
        self.negative = (negative != previous.negative) && !self.limbs.is_empty();
    }

    /// Bareiss update with a vanishing cross term:
    /// `self <- pivot * self / previous` (exact).
    pub fn bareiss_scale(&mut self, pivot: &Self, previous: &Self, scratch: &mut Scratch) {
        mul_mag_into(&mut scratch.product, &pivot.limbs, &self.limbs);
        let negative = (pivot.negative != self.negative) != previous.negative;
        divexact_mag_into(
            &mut scratch.quotient,
            &scratch.product,
            &previous.limbs,
            &mut scratch.numerator,
            &mut scratch.denominator,
        );
        std::mem::swap(&mut self.limbs, &mut scratch.quotient);
        self.negative = negative && !self.limbs.is_empty();
    }
}

fn combine(a: &Big, b: &Big, subtract: bool) -> Big {
    // a + s b with s = -1 when subtracting: a - (-(s b)).
    let term_negative = b.negative != subtract;
    let mut out = Vec::new();
    let negative = signed_sub_into(&mut out, a.negative, &a.limbs, !term_negative, &b.limbs);
    Big {
        negative,
        limbs: out,
    }
}

impl Add<&Big> for &Big {
    type Output = Big;
    fn add(self, rhs: &Big) -> Big {
        combine(self, rhs, false)
    }
}

impl Sub<&Big> for &Big {
    type Output = Big;
    fn sub(self, rhs: &Big) -> Big {
        combine(self, rhs, true)
    }
}

impl Mul<&Big> for &Big {
    type Output = Big;
    fn mul(self, rhs: &Big) -> Big {
        let mut out = Vec::new();
        mul_mag_into(&mut out, &self.limbs, &rhs.limbs);
        Big::from_parts(self.negative != rhs.negative, out)
    }
}

impl Neg for &Big {
    type Output = Big;
    fn neg(self) -> Big {
        Big::from_parts(!self.negative, self.limbs.clone())
    }
}

impl Neg for Big {
    type Output = Big;
    fn neg(mut self) -> Big {
        if !self.limbs.is_empty() {
            self.negative = !self.negative;
        }
        self
    }
}

macro_rules! forward_owned {
    ($($trait:ident $method:ident),*) => {
        $(
            impl $trait<Big> for Big {
                type Output = Big;
                fn $method(self, rhs: Big) -> Big {
                    (&self).$method(&rhs)
                }
            }

            impl $trait<&Big> for Big {
                type Output = Big;
                fn $method(self, rhs: &Big) -> Big {
                    (&self).$method(rhs)
                }
            }

            impl $trait<Big> for &Big {
                type Output = Big;
                fn $method(self, rhs: Big) -> Big {
                    self.$method(&rhs)
                }
            }
        )*
    };
}

forward_owned!(Add add, Sub sub, Mul mul);

impl Ord for Big {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.negative, other.negative) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (false, false) => cmp_mag(&self.limbs, &other.limbs),
            (true, true) => cmp_mag(&other.limbs, &self.limbs),
        }
    }
}

impl PartialOrd for Big {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Big {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        const CHUNK: u64 = 10_000_000_000_000_000_000;
        if self.limbs.is_empty() {
            return f.write_str("0");
        }
        let mut magnitude = self.limbs.clone();
        let mut chunks = Vec::new();
        while !magnitude.is_empty() {
            chunks.push(divrem_small_assign(&mut magnitude, CHUNK));
        }
        let mut text = String::with_capacity(chunks.len() * 19 + 1);
        if self.negative {
            text.push('-');
        }
        let mut rest = chunks.iter().rev();
        if let Some(first) = rest.next() {
            text.push_str(&first.to_string());
        }
        for chunk in rest {
            text.push_str(&format!("{chunk:019}"));
        }
        f.write_str(&text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic xorshift generator for test operands.
    struct XorShift(u64);

    impl XorShift {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }

        fn big(&mut self, limbs: usize) -> Big {
            let negative = self.next() & 1 == 1;
            let values = (0..limbs).map(|_| self.next()).collect();
            Big::from_parts(negative, values)
        }
    }

    #[test]
    fn ring_identities_hold_for_wide_operands() {
        let mut rng = XorShift(0x9e37_79b9_7f4a_7c15);
        for round in 0..200 {
            let a = rng.big(1 + round % 80);
            let b = rng.big(1 + (round * 7) % 80);
            let c = rng.big(1 + (round * 3) % 20);
            assert_eq!(&(&a + &b) - &b, a);
            assert_eq!(&a * &(&b + &c), &(&a * &b) + &(&a * &c));
            assert_eq!(&a - &a, Big::zero());
            assert_eq!((&a * &b).cmp(&(&b * &a)), Ordering::Equal);
        }
    }

    #[test]
    fn exact_and_floor_division_agree() {
        let mut rng = XorShift(0x2545_f491_4f6c_dd1d);
        for round in 0..200 {
            let a = rng.big(1 + round % 70);
            let mut b = rng.big(1 + (round * 5) % 40);
            if b.is_zero() {
                b = Big::one();
            }
            let b = b.shifted_left((round % 130) as u64);
            assert_eq!((&a * &b).divexact(&b), a);
            let (q, r) = a.div_rem_floor(&b);
            assert_eq!(&(&q * &b) + &r, a);
            assert!(r.is_zero() || r.is_negative() == b.is_negative());
            assert_eq!(r.abs().cmp(&b.abs()), Ordering::Less);
        }
    }

    #[test]
    fn gcd_divides_and_is_maximal() {
        let mut rng = XorShift(0x1234_5678_9abc_def1);
        for round in 0..100 {
            let common = rng.big(1 + round % 5).abs();
            let a = &rng.big(1 + round % 30) * &common;
            let b = &rng.big(1 + (round * 3) % 30) * &common;
            let g = a.gcd(&b);
            if g.is_zero() {
                continue;
            }
            assert!(a.div_rem_floor(&g).1.is_zero());
            assert!(b.div_rem_floor(&g).1.is_zero());
            assert_eq!(a.divexact(&g).gcd(&b.divexact(&g)), Big::one());
        }
    }

    #[test]
    fn decimal_round_trip_and_shifts() {
        let mut rng = XorShift(0x0fed_cba9_8765_4321);
        for round in 0..100 {
            let a = rng.big(1 + round % 90);
            assert_eq!(Big::parse(&a.to_string()), Some(a.clone()));
            let bits = (round * 37 % 300) as u64;
            assert_eq!(a.shifted_left(bits).shr_floor(bits), a);
            assert_eq!(a.shr_floor(bits), a.div_floor(&Big::power_of_two(bits)));
            assert_eq!(a.shr_ceil(bits), a.div_ceil(&Big::power_of_two(bits)));
        }
        assert_eq!(Big::parse("-0"), Some(Big::zero()));
        assert_eq!(
            Big::from_i64(-7).div_floor(&Big::from_i64(2)),
            Big::from_i64(-4)
        );
        assert_eq!(
            Big::from_i64(-7).div_ceil(&Big::from_i64(2)),
            Big::from_i64(-3)
        );
    }
}
