//! Closed forms of boundary sections 2 and 4.2: `Phi_{N,R}` by exact walk
//! enumeration, the warming family, `J_max`, the rival's inverse `s*(J)`,
//! the GLOBAL values, and the simple law `L(N, rho)` of H5. Exact rationals
//! only.

use crate::rat::Rat;

fn int(value: i64) -> Rat {
    Rat::from_i64(value)
}

/// The walk polynomial of `Phi_{N,R}`: entry `b` is the number of `R`-step
/// walks from home on the complete graph of `N` islands, each step a stay
/// or a crossing, that cross exactly `b` distinct bridges. Exact depth-first
/// enumeration; bridges are tracked as a mask over the candidate pairs.
pub(crate) fn walk_polynomial(n: usize, radius: usize) -> Vec<u64> {
    fn extend(n: usize, steps: usize, at: usize, used: u64, counts: &mut Vec<u64>) {
        if steps == 0 {
            let distinct = used.count_ones() as usize;
            if counts.len() <= distinct {
                counts.resize(distinct + 1, 0);
            }
            counts[distinct] += 1;
            return;
        }
        for next in 0..n {
            let used = if next == at {
                used
            } else {
                let (u, w) = (at.min(next), at.max(next));
                used | (1 << (u * n + w))
            };
            extend(n, steps - 1, next, used, counts);
        }
    }
    let mut counts = vec![0];
    extend(n, radius, crate::model::HOME, 0, &mut counts);
    counts
}

/// `Phi_{N,R}(r)`: the sum over `R`-step walks `w` from home on the complete
/// graph of `r^b(w)`, `b(w)` the number of distinct bridges `w` crosses.
/// This is `<N_R(G, x)>` under independent bridges each present with
/// probability `r`.
pub(crate) fn phi(n: usize, radius: usize, r: &Rat) -> Rat {
    let mut sum = Rat::zero();
    let mut power = Rat::one();
    for &count in &walk_polynomial(n, radius) {
        sum = &sum + &(&int(count as i64) * &power);
        power = &power * r;
    }
    sum
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

/// `H_global = Phi_{N,R}(2/(b_c + b_h + 2)) - Phi_{N,R}(1/(1 + b_c))`
/// (section 4.2).
pub(crate) fn global_held(n: usize, radius: usize, b_c: u64, b_h: u64) -> Rat {
    &phi(n, radius, &global_density(b_c, b_h)) - &phi(n, radius, &cold_density(b_c))
}

/// `J_global = K (b_c - b_h)/(b_c + b_h + 2)` (section 4.2).
pub(crate) fn global_flow(k: usize, b_c: u64, b_h: u64) -> Rat {
    Rat::ratio(k as i64 * (b_c as i64 - b_h as i64), (b_c + b_h + 2) as i64)
}

/// The simple law of H5 (section 2),
/// `L(N, rho) = N (3 + (N - 2) r_0) / (2 (3 + (N - 2)(rho + r_0)))` with
/// `r_0 = 1/(1 + b_c)`; `None` when the denominator vanishes.
pub(crate) fn simple_law(n: usize, b_c: u64, rho: &Rat) -> Option<Rat> {
    let r0 = cold_density(b_c);
    let spread = int(n as i64 - 2);
    let numerator = &int(n as i64) * &(&int(3) + &(&spread * &r0));
    let denominator = &int(2) * &(&int(3) + &(&spread * &(rho + &r0)));
    (!denominator.is_zero()).then(|| &numerator / &denominator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::big::Big;
    use crate::model::{candidate_bridges, open_future_by_matrix, HOME, PAIRS};

    /// `<N_R(G, x)>` over independent bridges by direct enumeration of every
    /// configuration.
    fn enumerated_mean(n: usize, radius: usize, x: usize, r: &Rat) -> Rat {
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
            let count = open_future_by_matrix(n, &bridges, config, radius)[x];
            mean = &mean + &(&weight * &Rat::integer(Big::from_u64(count)));
        }
        mean
    }

    fn polynomial(coefficients: &[i64], r: &Rat) -> Rat {
        coefficients
            .iter()
            .rev()
            .fold(Rat::zero(), |sum, &c| &(&sum * r) + &int(c))
    }

    #[test]
    fn phi_matches_the_section_2_closed_forms() {
        for r in [Rat::ratio(1, 3), Rat::ratio(2, 7), cold_density(12)] {
            for n in 3..=5_i64 {
                // Phi_{N,2}(r) = 1 + 3(N-1) r + (N-1)(N-2) r^2.
                let closed = polynomial(&[1, 3 * (n - 1), (n - 1) * (n - 2)], &r);
                assert_eq!(phi(n as usize, 2, &r), closed, "N={n} r={r}");
            }
            // Phi_{3,3}(r) = 1 + 14 r + 10 r^2 + 2 r^3.
            assert_eq!(phi(3, 3, &r), polynomial(&[1, 14, 10, 2], &r), "r={r}");
        }
        assert_eq!(walk_polynomial(3, 3), vec![1, 14, 10, 2]);
        assert_eq!(walk_polynomial(5, 2), vec![1, 12, 12]);
    }

    #[test]
    fn phi_equals_enumeration_over_configurations() {
        for (n, radius) in [(3, 2), (4, 2), (5, 2), (3, 3)] {
            for r in [Rat::ratio(1, 3), Rat::ratio(2, 7), cold_density(5)] {
                for x in [HOME, n - 1] {
                    assert_eq!(
                        enumerated_mean(n, radius, x, &r),
                        phi(n, radius, &r),
                        "N={n} R={radius} x={x} r={r}"
                    );
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
