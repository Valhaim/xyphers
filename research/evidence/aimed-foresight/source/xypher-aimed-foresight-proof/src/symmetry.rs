//! Island relabellings, orbits, the lumped generator, and the lifting
//! (boundary sections 1.5 and 6).
//!
//! The group is every permutation of the islands `1, ..., N-1` with home `0`
//! fixed; it acts on a state `(G, r, t)` by relabelling the endpoints of every
//! bridge of `G` and every island of the route, and keeps the cursor. The
//! canonical representative of a state is the least state index in its
//! orbit. The lumped rate from orbit `O` to orbit `O'` is the sum over
//! `z'` in `O'` of `k(z -> z')` for the representative `z` of `O`; its
//! integer stationary vector `n_O` (orbit totals) is lifted to every state as
//! `w(z) = n_O (N-1)!/|O|`. Whether the channels are symmetric is tested by
//! G06 check 2, and whether `w` balances the full chain by G06 check 4.

use std::collections::BTreeMap;

use crate::big::Big;
use crate::model::{route_index, Model, HOME};

/// One relabelling `sigma` of the islands, with its action on bridges,
/// configurations, and routes.
#[derive(Clone, Debug)]
pub(crate) struct Relabelling {
    /// `sigma(v)` for every island; `sigma(0) = 0`.
    pub(crate) islands: Vec<usize>,
    /// Image of each candidate bridge.
    pub(crate) bridges: Vec<usize>,
    /// Image of each configuration.
    pub(crate) configs: Vec<usize>,
    /// Image of each route.
    pub(crate) routes: Vec<usize>,
}

/// Orbits of the states, numbered in the canonical order of their
/// representatives.
#[derive(Clone, Debug, Default)]
pub(crate) struct Orbits {
    /// Orbit of every state.
    pub(crate) of_state: Vec<usize>,
    /// Canonical representative (least state index) of every orbit.
    pub(crate) representatives: Vec<usize>,
    /// `|O|` of every orbit.
    pub(crate) sizes: Vec<usize>,
}

impl Orbits {
    pub(crate) fn count(&self) -> usize {
        self.representatives.len()
    }
}

/// Every permutation of `items`, lexicographic in the order given.
fn permutations(items: &[usize]) -> Vec<Vec<usize>> {
    if items.is_empty() {
        return vec![Vec::new()];
    }
    let mut out = Vec::new();
    for (index, &first) in items.iter().enumerate() {
        let mut rest = items.to_vec();
        rest.remove(index);
        for mut tail in permutations(&rest) {
            tail.insert(0, first);
            out.push(tail);
        }
    }
    out
}

/// `(N-1)!`, the order of the relabelling group.
pub(crate) fn group_order(n: usize) -> u64 {
    (1..n as u64).product()
}

/// The relabellings of islands `1, ..., N-1` with home fixed, identity first.
pub(crate) fn relabellings(
    n: usize,
    bridges: &[(usize, usize)],
    routes: &[Vec<usize>],
) -> Vec<Relabelling> {
    let movable: Vec<usize> = (0..n).filter(|&v| v != HOME).collect();
    permutations(&movable)
        .into_iter()
        .map(|images| {
            let mut islands = vec![HOME; n];
            for (&v, &image) in movable.iter().zip(&images) {
                islands[v] = image;
            }
            let bridge_images: Vec<usize> = bridges
                .iter()
                .map(|&(u, w)| {
                    let (a, b) = (islands[u], islands[w]);
                    let pair = (a.min(b), a.max(b));
                    bridges
                        .iter()
                        .position(|&candidate| candidate == pair)
                        .expect("a relabelling maps candidate bridges to candidate bridges")
                })
                .collect();
            let configs = (0..1_usize << bridges.len())
                .map(|config| {
                    bridge_images
                        .iter()
                        .enumerate()
                        .filter(|&(bridge, _)| config & (1 << bridge) != 0)
                        .fold(0, |image, (_, &target)| image | (1 << target))
                })
                .collect();
            let route_images = routes
                .iter()
                .map(|walk| {
                    let image: Vec<usize> = walk.iter().map(|&v| islands[v]).collect();
                    route_index(n, &image)
                })
                .collect();
            Relabelling {
                islands,
                bridges: bridge_images,
                configs,
                routes: route_images,
            }
        })
        .collect()
}

/// Canonical representative of a state: the least index in its orbit.
pub(crate) fn canonical(m: &Model, state: usize) -> usize {
    m.relabellings
        .iter()
        .map(|relabelling| m.image_state(relabelling, state))
        .min()
        .unwrap_or(state)
}

/// Orbits of the constructed states. A state is a representative exactly
/// when it is its own canonical representative, so orbits are numbered in
/// the canonical order of their least states.
pub(crate) fn orbits(m: &Model) -> Orbits {
    let mut orbits = Orbits {
        of_state: Vec::with_capacity(m.states.len()),
        representatives: Vec::new(),
        sizes: Vec::new(),
    };
    for state in 0..m.states.len() {
        let representative = canonical(m, state);
        let orbit = if representative == state {
            orbits.representatives.push(state);
            orbits.sizes.push(0);
            orbits.representatives.len() - 1
        } else {
            orbits.of_state[representative]
        };
        orbits.of_state.push(orbit);
        orbits.sizes[orbit] += 1;
    }
    orbits
}

/// Off-diagonal rates `(O, O', sum over z' in O' of k(z -> z'))` of the
/// lumped generator, read at each orbit's representative `z`; channels
/// within an orbit drop out with the diagonal.
pub(crate) fn lumped_rates(m: &Model) -> Vec<(usize, usize, u64)> {
    let orbits = &m.orbits;
    let mut rates = Vec::new();
    for (orbit, &representative) in orbits.representatives.iter().enumerate() {
        let mut row: BTreeMap<usize, u64> = BTreeMap::new();
        for channel in &m.channels[representative] {
            if let Some(to) = channel.to {
                *row.entry(orbits.of_state[to]).or_insert(0) += channel.rate;
            }
        }
        rates.extend(
            row.into_iter()
                .filter(|&(target, _)| target != orbit)
                .map(|(target, rate)| (orbit, target, rate)),
        );
    }
    rates
}

/// `w(z) = n_O (N-1)!/|O|` for every state; `Err` if some `|O|` does not
/// divide `(N-1)!`.
pub(crate) fn lift(m: &Model, orbit_weights: &[Big]) -> Result<Vec<Big>, String> {
    let order = group_order(m.spec.n);
    let orbits = &m.orbits;
    let factors = orbits
        .sizes
        .iter()
        .enumerate()
        .map(|(orbit, &size)| {
            let size = size as u64;
            if size == 0 || !order.is_multiple_of(size) {
                Err(format!(
                    "|O| = {size} of the orbit of {} does not divide (N-1)! = {order}",
                    m.describe_state(orbits.representatives[orbit])
                ))
            } else {
                Ok(order / size)
            }
        })
        .collect::<Result<Vec<u64>, String>>()?;
    Ok(orbits
        .of_state
        .iter()
        .map(|&orbit| orbit_weights[orbit].mul_small(factors[orbit]))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Agent, Mutation, WorldSpec};

    // Section 4.1 orbit counts of the two smallest worlds, and the
    // orbit-stabiliser divisibility the lifting needs; construction only.
    #[test]
    fn orbit_counts_of_small_worlds() {
        for (n, tau, count) in [(3, 1, 28), (4, 1, 120)] {
            let m = Model::build(WorldSpec::new(n, tau, 8, 2), Agent::Local, Mutation::None);
            assert_eq!(m.orbits.count(), count);
            assert_eq!(m.orbits.sizes.iter().sum::<usize>(), m.states.len());
            let order = group_order(n) as usize;
            assert!(m
                .orbits
                .sizes
                .iter()
                .all(|&size| order.is_multiple_of(size)));
            for (state, &orbit) in m.orbits.of_state.iter().enumerate() {
                assert!(m.orbits.representatives[orbit] <= state);
                assert_eq!(canonical(&m, state), m.orbits.representatives[orbit]);
            }
        }
    }

    #[test]
    fn relabellings_fix_home_and_act_on_routes() {
        let m = Model::build(WorldSpec::new(4, 2, 8, 2), Agent::Global, Mutation::None);
        assert_eq!(m.relabellings.len(), 6);
        assert!(m
            .relabellings
            .first()
            .is_some_and(|identity| identity.islands == [0, 1, 2, 3]));
        for relabelling in &m.relabellings {
            assert_eq!(relabelling.islands[HOME], HOME);
            for (route, walk) in m.routes.iter().enumerate() {
                let image = &m.routes[relabelling.routes[route]];
                for (&v, &w) in walk.iter().zip(image) {
                    assert_eq!(relabelling.islands[v], w);
                }
            }
        }
    }
}
