//! Fraction-free (Bareiss) elimination over the integers (boundary section
//! 6). The matrix is dense; entries are updated in place with reusable limb
//! buffers. Every intermediate entry is a minor of the input, so each
//! division by the previous pivot is exact.

use crate::big::{Big, Scratch};

/// Solves `A x = b` exactly. Returns `(d, y)` where `d != 0` is the
/// determinant of `A` after the row exchanges made for pivoting, and
/// `y = d x` is an integer vector (Cramer's rule). Returns `None` when `A` is
/// singular.
pub fn bareiss_solve(mut matrix: Vec<Vec<Big>>, rhs: Vec<Big>) -> Option<(Big, Vec<Big>)> {
    let size = matrix.len();
    assert!(
        rhs.len() == size && matrix.iter().all(|row| row.len() == size),
        "square system expected"
    );
    if size == 0 {
        return None;
    }
    for (row, value) in matrix.iter_mut().zip(rhs) {
        row.push(value);
    }
    let mut previous = Big::one();
    let mut scratch = Scratch::default();
    for k in 0..size {
        if matrix[k][k].is_zero() {
            let swap = (k + 1..size).find(|&row| !matrix[row][k].is_zero())?;
            matrix.swap(k, swap);
        }
        let (head, tail) = matrix.split_at_mut(k + 1);
        let pivot_row = &head[k];
        let pivot = &pivot_row[k];
        for row in tail.iter_mut() {
            let (left, right) = row.split_at_mut(k + 1);
            let factor = &left[k];
            for (entry, upper) in right.iter_mut().zip(&pivot_row[k + 1..]) {
                if factor.is_zero() || upper.is_zero() {
                    if !entry.is_zero() {
                        entry.bareiss_scale(pivot, &previous, &mut scratch);
                    }
                } else {
                    entry.bareiss_update(pivot, factor, upper, &previous, &mut scratch);
                }
            }
            left[k] = Big::zero();
        }
        previous = matrix[k][k].clone();
    }
    let determinant = previous;
    // Back substitution: a_ii y_i = d b_i - sum_{j > i} a_ij y_j, exactly.
    let mut solution = vec![Big::zero(); size];
    for i in (0..size).rev() {
        let mut accumulator = &determinant * &matrix[i][size];
        for j in i + 1..size {
            if !matrix[i][j].is_zero() && !solution[j].is_zero() {
                accumulator = &accumulator - &(&matrix[i][j] * &solution[j]);
            }
        }
        solution[i] = accumulator.divexact(&matrix[i][i]);
    }
    Some((determinant, solution))
}

/// Integer stationary vector of a continuous-time Markov generator on
/// `states` states with off-diagonal rates `(from, to, rate)` (parallel
/// channels may repeat a pair). The balance equations `p Q = 0` are written
/// as `Q^T p^T = 0`; the last is replaced by the normalisation `sum p = 1`.
/// The result `n` satisfies `p(z) = n_z / sum n` with `sum n > 0`; `None`
/// when the system is singular.
pub fn stationary_vector(states: usize, rates: &[(usize, usize, u64)]) -> Option<Vec<Big>> {
    if states == 0 {
        return None;
    }
    let mut small = vec![vec![0_i128; states]; states];
    for &(from, to, rate) in rates {
        if from == to {
            continue;
        }
        let rate = i128::from(rate);
        small[to][from] += rate;
        small[from][from] -= rate;
    }
    let last = states - 1;
    small[last] = vec![1; states];
    let matrix = small
        .into_iter()
        .map(|row| row.into_iter().map(Big::from_i128).collect())
        .collect();
    let mut rhs = vec![Big::zero(); states];
    rhs[last] = Big::one();
    let (determinant, mut solution) = bareiss_solve(matrix, rhs)?;
    // sum y = d sum x = d; make the sum positive.
    if determinant.is_negative() {
        for value in &mut solution {
            *value = -std::mem::take(value);
        }
    }
    Some(solution)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn int(value: i64) -> Big {
        Big::from_i64(value)
    }

    #[test]
    fn three_state_chain_has_the_detailed_balance_law() {
        // 0 <-> 1 <-> 2, forward rate 1, backward rate 2: p = (4, 2, 1)/7.
        let rates = [(0, 1, 1), (1, 0, 2), (1, 2, 1), (2, 1, 2)];
        let n = stationary_vector(3, &rates).expect("connected chain");
        let total = &(&n[0] + &n[1]) + &n[2];
        assert!(total.is_positive());
        assert_eq!(&n[0], &n[2].mul_small(4));
        assert_eq!(&n[1], &n[2].mul_small(2));
    }

    #[test]
    fn driven_cycle_with_parallel_channels_is_uniform() {
        // A driven cycle plus a reciprocal parallel pair keeps every state
        // balanced under the uniform law.
        let rates = [(0, 1, 3), (1, 2, 3), (2, 0, 3), (0, 1, 1), (1, 0, 1)];
        let n = stationary_vector(3, &rates).expect("connected cycle");
        assert!(n[0].is_positive());
        assert_eq!(n[0], n[1]);
        assert_eq!(n[1], n[2]);
    }

    #[test]
    fn small_system_matches_cramer() {
        // [2 1; 1 3] x = [3; 5]: det 5, x = (4/5, 7/5).
        let matrix = vec![vec![int(2), int(1)], vec![int(1), int(3)]];
        let (d, y) = bareiss_solve(matrix, vec![int(3), int(5)]).expect("regular");
        assert_eq!(d, int(5));
        assert_eq!(y, vec![int(4), int(7)]);
        let singular = vec![vec![int(1), int(2)], vec![int(2), int(4)]];
        assert!(bareiss_solve(singular, vec![int(1), int(1)]).is_none());
    }
}
