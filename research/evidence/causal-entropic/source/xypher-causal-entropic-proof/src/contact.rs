//! Same-temperature contact (boundary section 4.5, gate G11).
//!
//! Two archipelagos are prepared in their own equilibria (`b^(-|G|)` against
//! their own reservoir base), detached, and coupled by TRANSFER channels that
//! dismantle a permitted bridge at one traveller's island while building a
//! bridge at the other traveller's island. Each archipelago's STEP and
//! REPLAN continue. All contact channels have unit hazard. Contact states are
//! pairs `(z_A, z_B)` ordered by `z_A`, then `z_B`.

use crate::model::{Case, Kind, Model, Mutation, KAPPA, PRIMARY};
use crate::ratio::Ratio;

pub(crate) const HETEROGENEOUS_PARTNER: Case = Case::new(3, 1, 2);
/// C08 prepares archipelago B of the identical pair against this base.
pub(crate) const UNEQUAL_PREPARATION_BASE: i128 = 3;

#[derive(Clone, Debug)]
pub(crate) struct ShellAnalysis {
    pub(crate) shell: usize,
    pub(crate) states: usize,
    /// First violation of check 1 (stationarity or detailed balance of the
    /// conditioned prepared law, or a channel leaving the shell).
    pub(crate) balance_failure: Option<String>,
    /// Connected components of the shell (sizes, canonical first-state order).
    pub(crate) components: Vec<usize>,
    /// First violation of check 2 (prepared law not uniform on a component).
    pub(crate) component_failure: Option<String>,
    /// Expected signed packet current into A under the conditioned prepared law.
    pub(crate) current: Ratio,
}

#[derive(Clone, Debug)]
pub(crate) struct PairAnalysis {
    pub(crate) name: &'static str,
    pub(crate) a: Case,
    pub(crate) b: Case,
    pub(crate) prepared_base_b: i128,
    pub(crate) shells: Vec<ShellAnalysis>,
}

impl PairAnalysis {
    pub(crate) fn label(&self) -> String {
        format!(
            "{}x{}{}",
            self.a.label(),
            self.b.label(),
            if self.prepared_base_b == self.b.b {
                String::new()
            } else {
                format!("[B prepared at b={}]", self.prepared_base_b)
            }
        )
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ContactAnalysis {
    pub(crate) pairs: Vec<PairAnalysis>,
}

impl ContactAnalysis {
    pub(crate) fn compute(mutation: Mutation) -> Self {
        let identical_base_b = if mutation == Mutation::UnequalReservoirs {
            UNEQUAL_PREPARATION_BASE
        } else {
            PRIMARY.b
        };
        Self {
            pairs: vec![
                analyse_pair("identical", PRIMARY, PRIMARY, identical_base_b),
                analyse_pair(
                    "heterogeneous",
                    PRIMARY,
                    HETEROGENEOUS_PARTNER,
                    HETEROGENEOUS_PARTNER.b,
                ),
            ],
        }
    }
}

/// Per-archipelago channel lists read from the local rules (no reservoir).
struct LocalChannels {
    internal: Vec<Vec<usize>>,
    builds: Vec<Vec<usize>>,
    dismantles: Vec<Vec<usize>>,
}

fn local_channels(model: &Model) -> LocalChannels {
    let count = model.states.len();
    let mut channels = LocalChannels {
        internal: vec![Vec::new(); count],
        builds: vec![Vec::new(); count],
        dismantles: vec![Vec::new(); count],
    };
    for (state, moves) in model.moves.iter().enumerate() {
        for local in moves {
            let Some(to) = local.to else { continue };
            match local.kind {
                Kind::Step | Kind::Replan => channels.internal[state].push(to),
                Kind::Build => channels.builds[state].push(to),
                Kind::Dismantle => channels.dismantles[state].push(to),
            }
        }
    }
    channels
}

fn analyse_pair(name: &'static str, a_case: Case, b_case: Case, base_b: i128) -> PairAnalysis {
    let a = Model::build(a_case, Mutation::None);
    let b = Model::build(b_case, Mutation::None);
    let local_a = local_channels(&a);
    let local_b = local_channels(&b);
    let count_b = b.states.len();
    let total = a.states.len() * count_b;
    let size_a = |state: usize| a.size(a.states[state / count_b].config);
    let size_b = |state: usize| b.size(b.states[state % count_b].config);
    let shell_of = |state: usize| size_a(state) + size_b(state);
    let describe = |state: usize| {
        format!(
            "(A {}, B {})",
            a.describe_state(state / count_b),
            b.describe_state(state % count_b)
        )
    };

    // Contact channels: (destination, signed packet change of A).
    let mut forward: Vec<Vec<(usize, i64)>> = vec![Vec::new(); total];
    for (state, channels) in forward.iter_mut().enumerate() {
        let (za, zb) = (state / count_b, state % count_b);
        for &to in &local_a.internal[za] {
            channels.push((to * count_b + zb, 0));
        }
        for &to in &local_b.internal[zb] {
            channels.push((za * count_b + to, 0));
        }
        for &dismantled in &local_a.dismantles[za] {
            for &built in &local_b.builds[zb] {
                channels.push((dismantled * count_b + built, -1));
            }
        }
        for &dismantled in &local_b.dismantles[zb] {
            for &built in &local_a.builds[za] {
                channels.push((built * count_b + dismantled, 1));
            }
        }
        channels.sort_unstable();
    }
    let hazard = |from: usize, to: usize| -> Ratio {
        let count = forward[from]
            .iter()
            .filter(|(target, _)| *target == to)
            .count();
        KAPPA * Ratio::integer(count as i128)
    };

    // Conditioned prepared law, unnormalised: b_A^(-|G_A|) b_B^(-|G_B|).
    let weight = |state: usize| {
        Ratio::power(a_case.b, -(size_a(state) as i64))
            * Ratio::power(base_b, -(size_b(state) as i64))
    };
    let weights: Vec<Ratio> = (0..total).map(weight).collect();

    let mut inflow = vec![Ratio::ZERO; total];
    for (state, channels) in forward.iter().enumerate() {
        for &(to, _) in channels {
            inflow[to] = inflow[to] + weights[state] * KAPPA;
        }
    }

    let mut neighbours: Vec<Vec<usize>> = vec![Vec::new(); total];
    for (state, channels) in forward.iter().enumerate() {
        for &(to, _) in channels {
            neighbours[state].push(to);
            neighbours[to].push(state);
        }
    }

    let max_shell = a.bridge_count() + b.bridge_count();
    let mut shells = Vec::with_capacity(max_shell + 1);
    for shell in 0..=max_shell {
        let members: Vec<usize> = (0..total).filter(|&s| shell_of(s) == shell).collect();

        // Check 1: stationarity and detailed balance of the conditioned law.
        let mut balance_failure = None;
        'balance: for &state in &members {
            let outflow = weights[state] * KAPPA * Ratio::integer(forward[state].len() as i128);
            if inflow[state] != outflow {
                balance_failure = Some(format!(
                    "shell M={shell} state {}: inflow {} != outflow {}",
                    describe(state),
                    inflow[state],
                    outflow
                ));
                break;
            }
            for &(to, _) in &forward[state] {
                if shell_of(to) != shell {
                    balance_failure = Some(format!(
                        "shell M={shell} channel {} -> {} leaves the shell",
                        describe(state),
                        describe(to)
                    ));
                    break 'balance;
                }
                let forward_flux = weights[state] * hazard(state, to);
                let reverse_flux = weights[to] * hazard(to, state);
                if forward_flux != reverse_flux {
                    balance_failure = Some(format!(
                        "shell M={shell} channel {} <-> {}: fluxes {} and {}",
                        describe(state),
                        describe(to),
                        forward_flux,
                        reverse_flux
                    ));
                    break 'balance;
                }
            }
        }

        // Check 2: components and uniformity of the prepared law on each.
        let mut component_of = vec![usize::MAX; total];
        let mut components = Vec::new();
        let mut component_failure = None;
        for &root in &members {
            if component_of[root] != usize::MAX {
                continue;
            }
            let id = components.len();
            component_of[root] = id;
            let mut queue = vec![root];
            let mut size = 0;
            while let Some(state) = queue.pop() {
                size += 1;
                if component_failure.is_none() && weights[state] != weights[root] {
                    component_failure = Some(format!(
                        "shell M={shell} component {id}: prepared weight {} at {} != {} at {}",
                        weights[state],
                        describe(state),
                        weights[root],
                        describe(root)
                    ));
                }
                for &next in &neighbours[state] {
                    if component_of[next] == usize::MAX {
                        component_of[next] = id;
                        queue.push(next);
                    }
                }
            }
            components.push(size);
        }

        // Check 3: expected signed packet current into A, conditioned on M.
        let mass = members
            .iter()
            .fold(Ratio::ZERO, |sum, &state| sum + weights[state]);
        let mut current = Ratio::ZERO;
        for &state in &members {
            for &(_, delta_a) in &forward[state] {
                if delta_a != 0 {
                    current = current + weights[state] * KAPPA * Ratio::integer(delta_a.into());
                }
            }
        }
        let current = current.checked_div(mass).unwrap_or(Ratio::ZERO);

        shells.push(ShellAnalysis {
            shell,
            states: members.len(),
            balance_failure,
            components,
            component_failure,
            current,
        });
    }

    PairAnalysis {
        name,
        a: a_case,
        b: b_case,
        prepared_base_b: base_b,
        shells,
    }
}
