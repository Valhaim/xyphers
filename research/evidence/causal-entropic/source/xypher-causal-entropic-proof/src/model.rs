//! Construction of the CAL-CEF-1 archipelago (boundary sections 3.1--3.4).
//!
//! Everything here is built from the local rules only: plans by depth-first
//! search, the plan count by integer matrix powers, system and complete
//! states in canonical order, and the STEP / REPLAN / BUILD / DISMANTLE
//! channels. No constructor reads a stationary weight, an averaged rate,
//! THAIM, or Xi, and no hazard reads `N_tau`.

use std::collections::{BTreeSet, HashMap};

use crate::ratio::{integer_power, Ratio};

/// Home island `h`.
pub(crate) const HOME: usize = 0;
/// Packet energy `lambda` stored per bridge.
pub(crate) const LAMBDA: i64 = 1;
/// Micro channel hazard `kappa`; it fixes only the clock unit.
pub(crate) const KAPPA: Ratio = Ratio::ONE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Case {
    pub(crate) n: usize,
    pub(crate) tau: usize,
    pub(crate) b: i128,
}

impl Case {
    pub(crate) const fn new(n: usize, tau: usize, b: i128) -> Self {
        Self { n, tau, b }
    }

    pub(crate) fn label(self) -> String {
        format!("({},{},{})", self.n, self.tau, self.b)
    }
}

/// Primary witness of boundary section 5.1.
pub(crate) const PRIMARY: Case = Case::new(3, 2, 2);

/// Confirmatory family of boundary section 5.2, in table order.
pub(crate) const FAMILY: [Case; 8] = [
    Case::new(3, 1, 2),
    Case::new(3, 2, 2),
    Case::new(3, 3, 2),
    Case::new(3, 2, 3),
    Case::new(4, 1, 2),
    Case::new(4, 2, 2),
    Case::new(4, 2, 3),
    Case::new(4, 3, 2),
];

/// One declared input of the primary witness mutated by a control
/// (boundary section 8). `None` is the preregistered construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mutation {
    None,
    /// C01: every DISMANTLE channel deleted, BUILD kept.
    OneWayBuilder,
    /// C02: REPLAN draws from the lazy random-walk endpoint law from home.
    DestinationReadout,
    /// C03: forward hazard of the first STEP channel in canonical
    /// complete-state order doubled.
    DirectedKinetic,
    /// C04: Xi drops the reverse THAIM receipt.
    UnpairedReceipt,
    /// C05: plans may not stay put (`A_G` instead of `L_G`).
    NoStaying,
    /// C06: REPLAN draws uniformly over plans of horizon `tau - 1`.
    HorizonSlip,
    /// C07: DISMANTLE ignores whether the held plan uses the bridge.
    PlanBreaker,
    /// C08: archipelago B of the identical contact pair prepared at `b = 3`.
    UnequalReservoirs,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Kind {
    Step,
    Replan,
    Build,
    Dismantle,
}

impl Kind {
    pub(crate) const fn code(self) -> &'static str {
        match self {
            Self::Step => "STEP",
            Self::Replan => "REPLAN",
            Self::Build => "BUILD",
            Self::Dismantle => "DISMANTLE",
        }
    }

    pub(crate) const fn is_toggle(self) -> bool {
        matches!(self, Self::Build | Self::Dismantle)
    }

    pub(crate) const fn reverse(self) -> Self {
        match self {
            Self::Step => Self::Step,
            Self::Replan => Self::Replan,
            Self::Build => Self::Dismantle,
            Self::Dismantle => Self::Build,
        }
    }
}

/// One outcome of the REPLAN resolution (the Crystal). The preregistered
/// resolution writes a plan; the C02 destination readout reports an island.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Outcome {
    Plan(Vec<usize>),
    Endpoint(usize),
}

impl Outcome {
    /// The register contents this outcome writes into the held-plan slot.
    pub(crate) fn register(&self) -> &[usize] {
        match self {
            Self::Plan(sequence) => sequence,
            Self::Endpoint(island) => std::slice::from_ref(island),
        }
    }
}

/// REPLAN resolution at one configuration: outcome alphabet, outcome
/// probabilities, and the multiplicity the Crystal reports.
#[derive(Clone, Debug)]
pub(crate) struct Resolution {
    pub(crate) outcomes: Vec<Outcome>,
    pub(crate) probabilities: Vec<Ratio>,
    pub(crate) reported_multiplicity: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DestinationPlan {
    /// The held plan is kept (STEP, BUILD, DISMANTLE).
    Held,
    /// REPLAN writes the resolution outcome with this index.
    Outcome(usize),
}

/// A channel as produced by the local rules of section 3.4, before any
/// hazard is attached. `to` is `None` when the destination description is
/// not a system state (for example, a plan invalid in the destination).
#[derive(Clone, Copy, Debug)]
pub(crate) struct LocalMove {
    pub(crate) kind: Kind,
    pub(crate) bridge: Option<usize>,
    pub(crate) to_config: usize,
    pub(crate) plan: DestinationPlan,
    pub(crate) to_cursor: usize,
    pub(crate) to: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SystemState {
    pub(crate) config: usize,
    pub(crate) plan: usize,
    pub(crate) cursor: usize,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Entry {
    pub(crate) to: usize,
    pub(crate) hazard: Ratio,
    pub(crate) kind: Kind,
}

/// Sparse generator: off-diagonal rows sorted by destination; the diagonal
/// is the closed row (`-escape`).
#[derive(Clone, Debug)]
pub(crate) struct Generator {
    pub(crate) rows: Vec<Vec<Entry>>,
}

impl Generator {
    fn from_rows(mut rows: Vec<Vec<Entry>>) -> Self {
        for row in &mut rows {
            row.sort_by_key(|entry| entry.to);
            let mut merged: Vec<Entry> = Vec::with_capacity(row.len());
            for entry in row.drain(..) {
                match merged.last_mut() {
                    Some(last) if last.to == entry.to => last.hazard = last.hazard + entry.hazard,
                    _ => merged.push(entry),
                }
            }
            *row = merged;
        }
        Self { rows }
    }

    pub(crate) fn hazard(&self, from: usize, to: usize) -> Ratio {
        let row = &self.rows[from];
        match row.binary_search_by_key(&to, |entry| entry.to) {
            Ok(position) => row[position].hazard,
            Err(_) => Ratio::ZERO,
        }
    }

    pub(crate) fn escape(&self, from: usize) -> Ratio {
        self.rows[from]
            .iter()
            .fold(Ratio::ZERO, |sum, entry| sum + entry.hazard)
    }

    pub(crate) fn diagonal(&self, from: usize) -> Ratio {
        Ratio::ZERO - self.escape(from)
    }

    pub(crate) fn len(&self) -> usize {
        self.rows.len()
    }
}

/// The only quantities a system-state hazard may read (checked by G10).
#[derive(Clone, Copy, Debug)]
pub(crate) struct HazardInputs {
    pub(crate) kind: Kind,
    pub(crate) destination_reservoir_energy: usize,
    pub(crate) base: i128,
}

/// Section 4.1: lumping reservoir labels gives `b^(E_R(z'))` for a toggle
/// into `z'` and the unit hazard otherwise.
pub(crate) fn lumped_hazard(inputs: HazardInputs) -> Ratio {
    if inputs.kind.is_toggle() {
        KAPPA
            * Ratio::integer(integer_power(
                inputs.base,
                inputs.destination_reservoir_energy as u64,
            ))
    } else {
        KAPPA
    }
}

/// Explicit complete-state level (N = 3 only).
#[derive(Clone, Debug)]
pub(crate) struct MicroLevel {
    /// Complete-state offset of each system state (length `states + 1`).
    pub(crate) offsets: Vec<usize>,
    pub(crate) system_of: Vec<usize>,
    pub(crate) label_of: Vec<usize>,
    pub(crate) generator: Generator,
}

#[derive(Clone, Debug)]
pub(crate) struct Model {
    pub(crate) case: Case,
    pub(crate) mutation: Mutation,
    pub(crate) bridges: Vec<(usize, usize)>,
    pub(crate) config_count: usize,
    /// `e_h^T L_G^tau 1` by integer matrix powers.
    pub(crate) matrix_count: Vec<i128>,
    /// `Pi_tau(G)` by depth-first search, lexicographic.
    pub(crate) plans: Vec<Vec<Vec<usize>>>,
    pub(crate) plan_index: Vec<HashMap<Vec<usize>, usize>>,
    /// `Pi_tau(G)` by brute-force filtering of all sequences in `V^(tau+1)`.
    pub(crate) reference_plans: Vec<BTreeSet<Vec<usize>>>,
    pub(crate) resolutions: Vec<Resolution>,
    pub(crate) config_offsets: Vec<usize>,
    pub(crate) states: Vec<SystemState>,
    pub(crate) moves: Vec<Vec<LocalMove>>,
    pub(crate) system: Generator,
    pub(crate) identity: Vec<usize>,
    pub(crate) micro: Option<MicroLevel>,
}

impl Model {
    pub(crate) fn build(case: Case, mutation: Mutation) -> Self {
        let bridges: Vec<(usize, usize)> = (0..case.n)
            .flat_map(|u| ((u + 1)..case.n).map(move |w| (u, w)))
            .collect();
        let config_count = 1_usize << bridges.len();
        let mut model = Self {
            case,
            mutation,
            bridges,
            config_count,
            matrix_count: Vec::with_capacity(config_count),
            plans: Vec::with_capacity(config_count),
            plan_index: Vec::with_capacity(config_count),
            reference_plans: Vec::with_capacity(config_count),
            resolutions: Vec::with_capacity(config_count),
            config_offsets: Vec::with_capacity(config_count + 1),
            states: Vec::new(),
            moves: Vec::new(),
            system: Generator { rows: Vec::new() },
            identity: Vec::new(),
            micro: None,
        };

        for config in 0..config_count {
            let matrix_count = model.count_by_matrix_power(config);
            let plans = model.plans_by_depth_first_search(config, case.tau);
            let index = plans
                .iter()
                .enumerate()
                .map(|(position, plan)| (plan.clone(), position))
                .collect();
            let reference = model.plans_by_brute_force(config, case.tau);
            model.matrix_count.push(matrix_count);
            model.plans.push(plans);
            model.plan_index.push(index);
            model.reference_plans.push(reference);
            let resolution = model.replan_resolution(config);
            model.resolutions.push(resolution);
        }

        for config in 0..config_count {
            model.config_offsets.push(model.states.len());
            for plan in 0..model.plans[config].len() {
                for cursor in 0..=case.tau {
                    model.states.push(SystemState {
                        config,
                        plan,
                        cursor,
                    });
                }
            }
        }
        model.config_offsets.push(model.states.len());

        model.moves = (0..model.states.len())
            .map(|state| model.local_moves(state))
            .collect();
        model.identity = (0..model.states.len()).collect();
        model.system = model.system_generator();
        if case.n == 3 {
            model.micro = Some(model.micro_level());
        }
        model
    }

    pub(crate) fn bridge_count(&self) -> usize {
        self.bridges.len()
    }

    pub(crate) fn size(&self, config: usize) -> usize {
        config.count_ones() as usize
    }

    pub(crate) fn reservoir_energy(&self, config: usize) -> usize {
        self.bridge_count() - self.size(config)
    }

    pub(crate) fn has_bridge(&self, config: usize, bridge: usize) -> bool {
        config & (1 << bridge) != 0
    }

    pub(crate) fn bridge_index(&self, u: usize, w: usize) -> Option<usize> {
        let (low, high) = if u < w { (u, w) } else { (w, u) };
        self.bridges
            .iter()
            .position(|&bridge| bridge == (low, high))
    }

    /// Whether `{v, w}` is a built bridge of `config` (`v != w`).
    pub(crate) fn bridge_built(&self, config: usize, v: usize, w: usize) -> bool {
        v != w
            && self
                .bridge_index(v, w)
                .is_some_and(|bridge| self.has_bridge(config, bridge))
    }

    pub(crate) fn degree(&self, config: usize, v: usize) -> usize {
        (0..self.case.n)
            .filter(|&w| self.bridge_built(config, v, w))
            .count()
    }

    /// One plan step `v -> w` (section 3.2): stay, or cross a built bridge.
    /// C05 forbids staying.
    pub(crate) fn step_allowed(&self, config: usize, v: usize, w: usize) -> bool {
        (v == w && self.mutation != Mutation::NoStaying) || self.bridge_built(config, v, w)
    }

    pub(crate) fn plan_valid(&self, config: usize, sequence: &[usize]) -> bool {
        sequence.len() == self.case.tau + 1
            && sequence[0] == HOME
            && sequence.iter().all(|&v| v < self.case.n)
            && sequence
                .windows(2)
                .all(|step| self.step_allowed(config, step[0], step[1]))
    }

    pub(crate) fn uses_bridge(&self, sequence: &[usize], bridge: usize) -> bool {
        let (u, w) = self.bridges[bridge];
        sequence
            .windows(2)
            .any(|step| (step[0], step[1]) == (u, w) || (step[0], step[1]) == (w, u))
    }

    pub(crate) fn plan_of(&self, state: usize) -> &[usize] {
        let SystemState { config, plan, .. } = self.states[state];
        &self.plans[config][plan]
    }

    /// Island `x(z) = v_t` on which the traveller stands.
    pub(crate) fn position(&self, state: usize) -> usize {
        self.plan_of(state)[self.states[state].cursor]
    }

    pub(crate) fn destination_register(&self, state: usize, local: &LocalMove) -> &[usize] {
        match local.plan {
            DestinationPlan::Held => self.plan_of(state),
            DestinationPlan::Outcome(outcome) => {
                self.resolutions[self.states[state].config].outcomes[outcome].register()
            }
        }
    }

    pub(crate) fn states_of(&self, config: usize) -> std::ops::Range<usize> {
        self.config_offsets[config]..self.config_offsets[config + 1]
    }

    /// Candidate stationary weight `b^(-U(z))` (section 4.1), `U = lambda |G|`.
    pub(crate) fn boltzmann_weight(&self, config: usize) -> Ratio {
        Ratio::power(self.case.b, -(LAMBDA * self.size(config) as i64))
    }

    pub(crate) fn complete_state_count(&self) -> usize {
        match &self.micro {
            Some(micro) => micro.system_of.len(),
            None => self
                .states
                .iter()
                .map(|state| {
                    integer_power(self.case.b, self.reservoir_energy(state.config) as u64) as usize
                })
                .sum(),
        }
    }

    pub(crate) fn config_label(&self, config: usize) -> String {
        config_label(&self.bridges, config)
    }

    pub(crate) fn describe_state(&self, state: usize) -> String {
        let SystemState { config, cursor, .. } = self.states[state];
        format!(
            "z{state}[G={} p={:?} t={cursor}]",
            self.config_label(config),
            self.plan_of(state)
        )
    }

    fn walk_matrix(&self, config: usize) -> Vec<Vec<i128>> {
        let n = self.case.n;
        let stay = i128::from(self.mutation != Mutation::NoStaying);
        let mut matrix = vec![vec![0_i128; n]; n];
        for (v, row) in matrix.iter_mut().enumerate() {
            row[v] = stay;
        }
        for (bridge, &(u, w)) in self.bridges.iter().enumerate() {
            if self.has_bridge(config, bridge) {
                matrix[u][w] = 1;
                matrix[w][u] = 1;
            }
        }
        matrix
    }

    /// `e_h^T L_G^tau 1` by integer matrix powers (C05: `A_G` instead).
    fn count_by_matrix_power(&self, config: usize) -> i128 {
        let n = self.case.n;
        let walk = self.walk_matrix(config);
        let mut power: Vec<Vec<i128>> = (0..n)
            .map(|i| (0..n).map(|j| i128::from(i == j)).collect())
            .collect();
        for _ in 0..self.case.tau {
            let mut next = vec![vec![0_i128; n]; n];
            for i in 0..n {
                for j in 0..n {
                    next[i][j] = (0..n).map(|m| power[i][m] * walk[m][j]).sum();
                }
            }
            power = next;
        }
        power[HOME].iter().sum()
    }

    fn plans_by_depth_first_search(&self, config: usize, horizon: usize) -> Vec<Vec<usize>> {
        let mut plans = Vec::new();
        let mut prefix = vec![HOME];
        self.extend_plan(config, horizon, &mut prefix, &mut plans);
        plans
    }

    fn extend_plan(
        &self,
        config: usize,
        horizon: usize,
        prefix: &mut Vec<usize>,
        plans: &mut Vec<Vec<usize>>,
    ) {
        if prefix.len() == horizon + 1 {
            plans.push(prefix.clone());
            return;
        }
        let last = prefix[prefix.len() - 1];
        for next in 0..self.case.n {
            if self.step_allowed(config, last, next) {
                prefix.push(next);
                self.extend_plan(config, horizon, prefix, plans);
                prefix.pop();
            }
        }
    }

    fn plans_by_brute_force(&self, config: usize, horizon: usize) -> BTreeSet<Vec<usize>> {
        let n = self.case.n;
        let length = horizon + 1;
        let total = n.pow(length as u32);
        (0..total)
            .map(|code| {
                let mut digits = vec![0; length];
                let mut rest = code;
                for slot in (0..length).rev() {
                    digits[slot] = rest % n;
                    rest /= n;
                }
                digits
            })
            .filter(|sequence| {
                sequence[0] == HOME
                    && sequence
                        .windows(2)
                        .all(|step| self.step_allowed(config, step[0], step[1]))
            })
            .collect()
    }

    fn replan_resolution(&self, config: usize) -> Resolution {
        match self.mutation {
            Mutation::DestinationReadout => self.lazy_walk_endpoint_resolution(config),
            Mutation::HorizonSlip => {
                let slipped =
                    self.plans_by_depth_first_search(config, self.case.tau.saturating_sub(1));
                uniform_resolution(slipped)
            }
            _ => uniform_resolution(self.plans[config].clone()),
        }
    }

    /// C02: home row of `(D^-1 L_G)^tau`; alphabet = islands in the support.
    fn lazy_walk_endpoint_resolution(&self, config: usize) -> Resolution {
        let n = self.case.n;
        let walk = self.walk_matrix(config);
        let mut law = vec![Ratio::ZERO; n];
        law[HOME] = Ratio::ONE;
        for _ in 0..self.case.tau {
            let mut next = vec![Ratio::ZERO; n];
            for (v, &mass) in law.iter().enumerate() {
                let out_degree: i128 = walk[v].iter().sum();
                for (w, &entry) in walk[v].iter().enumerate() {
                    if entry != 0 {
                        next[w] = next[w] + mass * Ratio::new(entry, out_degree);
                    }
                }
            }
            law = next;
        }
        let (outcomes, probabilities): (Vec<_>, Vec<_>) = law
            .into_iter()
            .enumerate()
            .filter(|(_, mass)| mass.is_positive())
            .map(|(island, mass)| (Outcome::Endpoint(island), mass))
            .unzip();
        Resolution {
            reported_multiplicity: outcomes.len(),
            outcomes,
            probabilities,
        }
    }

    fn state_index(&self, config: usize, sequence: &[usize], cursor: usize) -> Option<usize> {
        if cursor > self.case.tau {
            return None;
        }
        let plan = *self.plan_index[config].get(sequence)?;
        Some(self.config_offsets[config] + plan * (self.case.tau + 1) + cursor)
    }

    /// Local rules of section 3.4 at one system state.
    fn local_moves(&self, state: usize) -> Vec<LocalMove> {
        let SystemState { config, cursor, .. } = self.states[state];
        let plan = self.plan_of(state);
        let tau = self.case.tau;
        let mut moves = Vec::new();

        // STEP: walk the plan forward and back.
        if cursor < tau {
            moves.push(LocalMove {
                kind: Kind::Step,
                bridge: None,
                to_config: config,
                plan: DestinationPlan::Held,
                to_cursor: cursor + 1,
                to: self.state_index(config, plan, cursor + 1),
            });
        }
        if cursor > 0 {
            moves.push(LocalMove {
                kind: Kind::Step,
                bridge: None,
                to_config: config,
                plan: DestinationPlan::Held,
                to_cursor: cursor - 1,
                to: self.state_index(config, plan, cursor - 1),
            });
        }

        // REPLAN: only at home, cursor 0, to every other resolution outcome.
        if cursor == 0 {
            for (outcome, written) in self.resolutions[config].outcomes.iter().enumerate() {
                if written.register() == plan {
                    continue;
                }
                moves.push(LocalMove {
                    kind: Kind::Replan,
                    bridge: None,
                    to_config: config,
                    plan: DestinationPlan::Outcome(outcome),
                    to_cursor: 0,
                    to: self.state_index(config, written.register(), 0),
                });
            }
        }

        // BUILD / DISMANTLE: bridges incident to the traveller's island.
        let island = plan[cursor];
        for (bridge, &(u, w)) in self.bridges.iter().enumerate() {
            if island != u && island != w {
                continue;
            }
            if !self.has_bridge(config, bridge) {
                let target = config | (1 << bridge);
                moves.push(LocalMove {
                    kind: Kind::Build,
                    bridge: Some(bridge),
                    to_config: target,
                    plan: DestinationPlan::Held,
                    to_cursor: cursor,
                    to: self.state_index(target, plan, cursor),
                });
            } else {
                if self.mutation == Mutation::OneWayBuilder {
                    continue;
                }
                if self.mutation != Mutation::PlanBreaker && self.uses_bridge(plan, bridge) {
                    continue;
                }
                let target = config & !(1 << bridge);
                moves.push(LocalMove {
                    kind: Kind::Dismantle,
                    bridge: Some(bridge),
                    to_config: target,
                    plan: DestinationPlan::Held,
                    to_cursor: cursor,
                    to: self.state_index(target, plan, cursor),
                });
            }
        }
        moves
    }

    fn system_generator(&self) -> Generator {
        let rows = self
            .moves
            .iter()
            .map(|moves| {
                moves
                    .iter()
                    .filter_map(|local| {
                        let to = local.to?;
                        let inputs = HazardInputs {
                            kind: local.kind,
                            destination_reservoir_energy: self
                                .reservoir_energy(self.states[to].config),
                            base: self.case.b,
                        };
                        Some(Entry {
                            to,
                            hazard: lumped_hazard(inputs),
                            kind: local.kind,
                        })
                    })
                    .collect()
            })
            .collect();
        Generator::from_rows(rows)
    }

    /// Complete states `(z, r)` and unit-hazard micro channels (section 3.4).
    fn micro_level(&self) -> MicroLevel {
        let mut offsets = Vec::with_capacity(self.states.len() + 1);
        let mut system_of = Vec::new();
        let mut label_of = Vec::new();
        for (state, system_state) in self.states.iter().enumerate() {
            offsets.push(system_of.len());
            let labels = integer_power(
                self.case.b,
                self.reservoir_energy(system_state.config) as u64,
            ) as usize;
            for label in 0..labels {
                system_of.push(state);
                label_of.push(label);
            }
        }
        offsets.push(system_of.len());

        let mut rows: Vec<Vec<Entry>> = Vec::with_capacity(system_of.len());
        for complete in 0..system_of.len() {
            let state = system_of[complete];
            let label = label_of[complete];
            let mut row = Vec::new();
            for local in &self.moves[state] {
                let Some(to) = local.to else { continue };
                if local.kind.is_toggle() {
                    // Every reservoir label at the destination energy.
                    for target in offsets[to]..offsets[to + 1] {
                        row.push(Entry {
                            to: target,
                            hazard: KAPPA,
                            kind: local.kind,
                        });
                    }
                } else {
                    row.push(Entry {
                        to: offsets[to] + label,
                        hazard: KAPPA,
                        kind: local.kind,
                    });
                }
            }
            rows.push(row);
        }
        let mut generator = Generator::from_rows(rows);

        if self.mutation == Mutation::DirectedKinetic {
            'search: for (complete, row) in generator.rows.iter_mut().enumerate() {
                let from_cursor = self.states[system_of[complete]].cursor;
                for entry in row.iter_mut() {
                    if entry.kind == Kind::Step
                        && self.states[system_of[entry.to]].cursor == from_cursor + 1
                    {
                        entry.hazard = entry.hazard * Ratio::integer(2);
                        break 'search;
                    }
                }
            }
        }

        MicroLevel {
            offsets,
            system_of,
            label_of,
            generator,
        }
    }
}

fn uniform_resolution(plans: Vec<Vec<usize>>) -> Resolution {
    let count = plans.len();
    Resolution {
        probabilities: vec![Ratio::new(1, count.max(1) as i128); count],
        outcomes: plans.into_iter().map(Outcome::Plan).collect(),
        reported_multiplicity: count,
    }
}

pub(crate) fn config_label(bridges: &[(usize, usize)], config: usize) -> String {
    if config == 0 {
        return "none".to_string();
    }
    bridges
        .iter()
        .enumerate()
        .filter(|(bridge, _)| config & (1 << bridge) != 0)
        .map(|(_, (u, w))| format!("{{{u},{w}}}"))
        .collect::<Vec<_>>()
        .join(",")
}
