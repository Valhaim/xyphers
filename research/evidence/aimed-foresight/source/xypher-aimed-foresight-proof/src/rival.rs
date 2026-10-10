//! Closed forms of boundary sections 2 and 4.2: `Phi_N`, the warming family,
//! `J_max`, the rival's inverse `s*(J)`, and the GLOBAL values. Exact
//! rationals only.

use crate::rat::Rat;

fn int(value: i64) -> Rat {
    Rat::from_i64(value)
}

/// `Phi_N(r) = 1 + (N-1) r + (N-1) r (2 + (N-2) r)`: `<N_2(G, x)>` under
/// independent bridges each present with probability `r`.
pub(crate) fn phi(n: usize, r: &Rat) -> Rat {
    let others = &int(n as i64 - 1) * r;
    let inner = &int(2) + &(&int(n as i64 - 2) * r);
    &(&Rat::one() + &others) + &(&others * &inner)
}

/// `r_0 = 1/(1 + b_c)`, the undriven bridge probability.
pub(crate) fn cold_density(b_c: u64) -> Rat {
    Rat::ratio(1, 1 + b_c as i64)
}

/// `2/(b_c + b_h + 2)`, the GLOBAL bridge probability (`r_s` at `s = 1`).
pub(crate) fn global_density(b_c: u64, b_h: u64) -> Rat {
    Rat::ratio(2, (b_c + b_h + 2) as i64)
}

/// `J_max = K (b_c - b_h)/(b_h + 1)`.
pub(crate) fn j_max(k: usize, b_c: u64, b_h: u64) -> Rat {
    Rat::ratio(k as i64 * (b_c as i64 - b_h as i64), b_h as i64 + 1)
}

/// `b_c + 1 + s (b_h + 1)`, the denominator of the warming family.
fn warming_denominator(b_c: u64, b_h: u64, s: &Rat) -> Option<Rat> {
    let value = &int(b_c as i64 + 1) + &(s * &int(b_h as i64 + 1));
    (!value.is_zero()).then_some(value)
}

/// `J_warm(s) = K s (b_c - b_h)/(b_c + 1 + s (b_h + 1))`.
pub(crate) fn warming_flow(k: usize, b_c: u64, b_h: u64, s: &Rat) -> Option<Rat> {
    let denominator = warming_denominator(b_c, b_h, s)?;
    Some(&(&int(k as i64 * (b_c as i64 - b_h as i64)) * s) / &denominator)
}

/// `r_s = (1 + s)/(b_c + 1 + s (b_h + 1))`.
pub(crate) fn warming_density(b_c: u64, b_h: u64, s: &Rat) -> Option<Rat> {
    let denominator = warming_denominator(b_c, b_h, s)?;
    Some(&(&Rat::one() + s) / &denominator)
}

/// The rival's inverse `s*(J) = J (b_c + 1)/[K (b_c - b_h) - J w]` with the
/// declared weight `w = b_h + 1` (C07 passes `b_h`); `None` when the
/// denominator vanishes.
pub(crate) fn rival_rate(k: usize, b_c: u64, b_h: u64, weight: u64, j: &Rat) -> Option<Rat> {
    let denominator = &int(k as i64 * (b_c as i64 - b_h as i64)) - &(j * &int(weight as i64));
    if denominator.is_zero() {
        return None;
    }
    Some(&(j * &int(b_c as i64 + 1)) / &denominator)
}

/// `H_global = Phi_N(2/(b_c + b_h + 2)) - Phi_N(1/(1 + b_c))` (section 4.2).
pub(crate) fn global_held(n: usize, b_c: u64, b_h: u64) -> Rat {
    &phi(n, &global_density(b_c, b_h)) - &phi(n, &cold_density(b_c))
}

/// `J_global = K (b_c - b_h)/(b_c + b_h + 2)` (section 4.2).
pub(crate) fn global_flow(k: usize, b_c: u64, b_h: u64) -> Rat {
    Rat::ratio(k as i64 * (b_c as i64 - b_h as i64), (b_c + b_h + 2) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::big::Big;
    use crate::model::{candidate_bridges, open_future_by_matrix, PAIRS};

    /// `<N_2(G, x)>` over independent bridges by direct enumeration of every
    /// configuration.
    fn enumerated_mean(n: usize, x: usize, r: &Rat) -> Rat {
        let bridges = candidate_bridges(n);
        let absent = &Rat::one() - r;
        let mut mean = Rat::zero();
        for config in 0..1_usize << bridges.len() {
            let mut weight = Rat::one();
            for bridge in 0..bridges.len() {
                let factor = if config & (1 << bridge) != 0 {
                    r
                } else {
                    &absent
                };
                weight = &weight * factor;
            }
            let count = open_future_by_matrix(n, &bridges, config)[x];
            mean = &mean + &(&weight * &Rat::integer(Big::from_u64(count)));
        }
        mean
    }

    #[test]
    fn phi_closed_form_equals_enumeration() {
        for n in 3..=5 {
            for r in [Rat::ratio(1, 3), Rat::ratio(2, 7), cold_density(8)] {
                for x in 0..n {
                    assert_eq!(enumerated_mean(n, x, &r), phi(n, &r), "N={n} x={x} r={r}");
                }
            }
        }
    }

    #[test]
    fn rival_inverse_recovers_flow_and_density() {
        for (b_c, b_h) in PAIRS {
            for k in [3_usize, 6, 10] {
                let limit = j_max(k, b_c, b_h);
                for fraction in [Rat::ratio(1, 1000), Rat::ratio(1, 3), Rat::ratio(7, 8)] {
                    let j = &limit * &fraction;
                    let s = rival_rate(k, b_c, b_h, b_h + 1, &j).expect("defined below J_max");
                    assert!(s.is_positive());
                    assert_eq!(warming_flow(k, b_c, b_h, &s), Some(j.clone()));
                    // J = (b_c + 1) K rho - K.
                    let rho = &(&j + &int(k as i64)) / &int((b_c as i64 + 1) * k as i64);
                    assert_eq!(warming_density(b_c, b_h, &s), Some(rho));
                    // The C07 inverse: J_warm(s_w) = J K (b_c - b_h)/[K (b_c - b_h) + J].
                    let spread = int(k as i64 * (b_c as i64 - b_h as i64));
                    let wrong = rival_rate(k, b_c, b_h, b_h, &j).expect("defined");
                    let expected = &(&j * &spread) / &(&spread + &j);
                    assert_eq!(warming_flow(k, b_c, b_h, &wrong), Some(expected.clone()));
                    assert!(expected < j);
                }
            }
        }
    }

    #[test]
    fn global_is_the_warming_family_at_unit_rate() {
        for (b_c, b_h) in PAIRS {
            for n in 3..=5 {
                let k = n * (n - 1) / 2;
                let flow = global_flow(k, b_c, b_h);
                assert!(flow < j_max(k, b_c, b_h));
                assert_eq!(rival_rate(k, b_c, b_h, b_h + 1, &flow), Some(Rat::one()));
                assert_eq!(
                    warming_density(b_c, b_h, &Rat::one()),
                    Some(global_density(b_c, b_h))
                );
            }
        }
    }
}
