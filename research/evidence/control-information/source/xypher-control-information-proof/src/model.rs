//! Construction of the CAL-CEF-2 instrumented archipelago (boundary sections
//! 3.1--3.3).
//!
//! Everything here is built from the local rules only: plans by depth-first
//! search, the plan count and the destination counts by integer matrix
//! powers, system states `(G, p, t, y, w)` and complete states `(z, r)` in
//! canonical order, and the STEP / REPLAN / BUILD / DISMANTLE / MEASURE /
//! UNMEASURE / HARVEST / COMMIT / ERASE / SCRIBBLE channels. No constructor
//! reads a stationary weight or an averaged rate, and no hazard reads
//! `N_tau`, `n_y`, `I`, `I*`, `K`, THAIM, or Xi.

use std::collections::{BTreeMap, BTreeSet};

use crate::ratio::{integer_power, Ratio};

/// Home island `h`.
pub(crate) const HOME: usize = 0;
/// Packet energy `lambda` stored per bridge and per stockpile packet.
pub(crate) const LAMBDA: i64 = 1;
/// Micro channel hazard `kappa`; it fixes only the clock unit.
pub(crate) const KAPPA: Ratio = Ratio::ONE;
/// `kappa` as the integer hazard of an explicit micro channel.
const KAPPA_MICRO: u32 = 1;

/// Declared experimental inputs `(N, tau, b, W)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Case {
    pub(crate) n: usize,
    pub(crate) tau: usize,
    pub(crate) b: i128,
    /// Stockpile capacity `W`.
    pub(crate) w: usize,
}

impl Case {
    pub(crate) const fn new(n: usize, tau: usize, b: i128, w: usize) -> Self {
        Self { n, tau, b, w }
    }

    pub(crate) fn label(self) -> String {
        format!("({},{},{},{})", self.n, self.tau, self.b, self.w)
    }
}

/// Primary witness of boundary section 5.1.
pub(crate) const PRIMARY: Case = Case::new(3, 2, 2, 2);

/// Confirmatory family of boundary section 5.2, in table order.
pub(crate) const FAMILY: [Case; 7] = [
    Case::new(3, 1, 2, 2),
    Case::new(3, 2, 2, 2),
    Case::new(3, 3, 2, 2),
    Case::new(3, 2, 3, 2),
    Case::new(3, 1, 3, 1),
    Case::new(4, 1, 2, 1),
    Case::new(4, 2, 2, 1),
];

/// Boundary section 8, C09: the primary is constructed with `W = 3`.
pub(crate) const SLIPPED_STOCKPILE: usize = 3;

/// One declared input of the primary witness mutated by a control
/// (boundary section 8). `None` is the preregistered construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mutation {
    None,
    /// C01: every COMMIT channel deleted, HARVEST kept.
    OneWayHarvest,
    /// C02: ERASE/SCRIBBLE reset or write the notebook at fixed stockpile,
    /// moving no packet.
    FreeReset,
    /// C03: MEASURE writes island 0 regardless of `d(p)` (UNMEASURE is its
    /// declared reverse).
    BlindMeasurement,
    /// C04: HARVEST from any written record (COMMIT is its declared reverse).
    CheatingHarvest,
    /// C05: HARVEST also from a blank notebook when the held plan ends at
    /// home (COMMIT is its declared reverse).
    TelepathicDemon,
    /// C06: forward hazard of the first HARVEST micro channel doubled.
    DirectedKinetic,
    /// C07: HARVEST keeps the held plan (`p' = p` only; COMMIT is its
    /// declared reverse).
    HoardingDemon,
    /// C08: the shuffled law uses a uniform record marginal on `V`.
    UnequalShuffle,
    /// C09: the primary built with `W = 3`; predictions stay those of `W = 2`.
    StockpileSlip,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Kind {
    Step,
    Replan,
    Build,
    Dismantle,
    Measure,
    Unmeasure,
    Harvest,
    Commit,
    Erase,
    Scribble,
}

impl Kind {
    pub(crate) const fn code(self) -> &'static str {
        match self {
            Self::Step => "STEP",
            Self::Replan => "REPLAN",
            Self::Build => "BUILD",
            Self::Dismantle => "DISMANTLE",
            Self::Measure => "MEASURE",
            Self::Unmeasure => "UNMEASURE",
            Self::Harvest => "HARVEST",
            Self::Commit => "COMMIT",
            Self::Erase => "ERASE",
            Self::Scribble => "SCRIBBLE",
        }
    }

    /// The declared reverse of each channel (Thaw).
    pub(crate) const fn reverse(self) -> Self {
        match self {
            Self::Step => Self::Step,
            Self::Replan => Self::Replan,
            Self::Build => Self::Dismantle,
            Self::Dismantle => Self::Build,
            Self::Measure => Self::Unmeasure,
            Self::Unmeasure => Self::Measure,
            Self::Harvest => Self::Commit,
            Self::Commit => Self::Harvest,
            Self::Erase => Self::Scribble,
            Self::Scribble => Self::Erase,
        }
    }
}

/// Notebook register: `None` is blank, `Some(y)` a written island.
pub(crate) type Note = Option<usize>;

pub(crate) fn note_label(note: Note) -> String {
    match note {
        None => "blank".to_string(),
        Some(island) => island.to_string(),
    }
}

/// REPLAN resolution at one configuration (the Crystal): outcome alphabet
/// and outcome probabilities.
#[derive(Clone, Debug)]
pub(crate) struct Resolution {
    pub(crate) outcomes: Vec<Vec<usize>>,
    pub(crate) probabilities: Vec<Ratio>,
}

/// A channel as produced by the local rules of section 3.3, before any
/// hazard is attached. `to` is `None` when the destination description is
/// not a state of the declared state space.
#[derive(Clone, Debug)]
pub(crate) struct LocalMove {
    pub(crate) kind: Kind,
    pub(crate) bridge: Option<usize>,
    pub(crate) to_config: usize,
    /// Plan register written into the destination.
    pub(crate) register: Vec<usize>,
    pub(crate) to_cursor: usize,
    pub(crate) to_note: Note,
    pub(crate) to_stock: usize,
    pub(crate) to: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SystemState {
    pub(crate) config: usize,
    pub(crate) plan: usize,
    pub(crate) cursor: usize,
    pub(crate) note: Note,
    pub(crate) stock: usize,
}

/// One channel of a generator row.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Edge {
    pub(crate) to: usize,
    pub(crate) kind: Kind,
    pub(crate) hazard: Ratio,
}

/// Sparse system-state generator: off-diagonal rows sorted by destination
/// and channel kind; the diagonal is the closed row (`-escape`).
#[derive(Clone, Debug)]
pub(crate) struct Generator {
    pub(crate) rows: Vec<Vec<Edge>>,
}

impl Generator {
    fn from_rows(mut rows: Vec<Vec<Edge>>) -> Self {
        for row in &mut rows {
            row.sort_by_key(|edge| (edge.to, edge.kind));
            let mut merged: Vec<Edge> = Vec::with_capacity(row.len());
            for edge in row.drain(..) {
                match merged.last_mut() {
                    Some(last) if last.to == edge.to && last.kind == edge.kind => {
                        last.hazard = last.hazard + edge.hazard;
                    }
                    _ => merged.push(edge),
                }
            }
            *row = merged;
        }
        Self { rows }
    }

    /// Total rate `k(from -> to)` over every channel kind.
    pub(crate) fn hazard(&self, from: usize, to: usize) -> Ratio {
        self.rows[from]
            .iter()
            .filter(|edge| edge.to == to)
            .fold(Ratio::ZERO, |sum, edge| sum + edge.hazard)
    }

    /// Rate of the channel of one kind from `from` to `to`.
    pub(crate) fn channel_hazard(&self, from: usize, to: usize, kind: Kind) -> Ratio {
        let row = &self.rows[from];
        match row.binary_search_by_key(&(to, kind), |edge| (edge.to, edge.kind)) {
            Ok(position) => row[position].hazard,
            Err(_) => Ratio::ZERO,
        }
    }

    pub(crate) fn escape(&self, from: usize) -> Ratio {
        self.rows[from]
            .iter()
            .fold(Ratio::ZERO, |sum, edge| sum + edge.hazard)
    }

    pub(crate) fn diagonal(&self, from: usize) -> Ratio {
        Ratio::ZERO - self.escape(from)
    }
}

/// One explicit micro channel between complete states (integer hazard in
/// units of `kappa`).
#[derive(Clone, Copy, Debug)]
pub(crate) struct MicroEdge {
    pub(crate) to: u32,
    pub(crate) kind: Kind,
    pub(crate) hazard: u32,
}

impl MicroEdge {
    pub(crate) fn edge(self) -> Edge {
        Edge {
            to: self.to as usize,
            kind: self.kind,
            hazard: KAPPA * Ratio::integer(i128::from(self.hazard)),
        }
    }
}

/// Explicit complete-state level (N = 3 only).
#[derive(Clone, Debug)]
pub(crate) struct MicroLevel {
    /// Complete-state offset of each system state (length `states + 1`).
    pub(crate) offsets: Vec<usize>,
    pub(crate) system_of: Vec<usize>,
    pub(crate) label_of: Vec<usize>,
    pub(crate) rows: Vec<Vec<MicroEdge>>,
}

impl MicroLevel {
    pub(crate) fn len(&self) -> usize {
        self.rows.len()
    }

    pub(crate) fn channel_hazard(&self, from: usize, to: usize, kind: Kind) -> Ratio {
        let row = &self.rows[from];
        match row.binary_search_by_key(&(to, kind), |edge| (edge.to as usize, edge.kind)) {
            Ok(position) => row[position].edge().hazard,
            Err(_) => Ratio::ZERO,
        }
    }

    pub(crate) fn escape(&self, from: usize) -> Ratio {
        self.rows[from]
            .iter()
            .fold(Ratio::ZERO, |sum, edge| sum + edge.edge().hazard)
    }
}

/// The only quantities a system-state hazard may read (checked by G14).
#[derive(Clone, Copy, Debug)]
pub(crate) struct HazardInputs {
    pub(crate) source_reservoir_energy: usize,
    pub(crate) destination_reservoir_energy: usize,
    pub(crate) base: i128,
}

/// Section 4.1: lumping reservoir labels gives `b^(E_R(z'))` for a channel
/// that changes `E_R` and the unit hazard otherwise.
pub(crate) fn lumped_hazard(inputs: HazardInputs) -> Ratio {
    if inputs.destination_reservoir_energy != inputs.source_reservoir_energy {
        KAPPA
            * Ratio::integer(integer_power(
                inputs.base,
                inputs.destination_reservoir_energy as u64,
            ))
    } else {
        KAPPA
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Model {
    pub(crate) case: Case,
    pub(crate) mutation: Mutation,
    pub(crate) bridges: Vec<(usize, usize)>,
    pub(crate) config_count: usize,
    /// `e_h^T L_G^tau 1` by integer matrix powers.
    pub(crate) matrix_count: Vec<i128>,
    /// `e_h^T L_G^tau e_y` by integer matrix powers, for every island `y`.
    pub(crate) matrix_endpoints: Vec<Vec<i128>>,
    /// `Pi_tau(G)` by depth-first search, lexicographic.
    pub(crate) plans: Vec<Vec<Vec<usize>>>,
    pub(crate) plan_index: Vec<BTreeMap<Vec<usize>, usize>>,
    /// `Pi_tau(G)` by brute-force filtering of all sequences in `V^(tau+1)`.
    pub(crate) reference_plans: Vec<BTreeSet<Vec<usize>>>,
    pub(crate) resolutions: Vec<Resolution>,
    pub(crate) config_offsets: Vec<usize>,
    pub(crate) states: Vec<SystemState>,
    pub(crate) moves: Vec<Vec<LocalMove>>,
    pub(crate) system: Generator,
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
            matrix_endpoints: Vec::with_capacity(config_count),
            plans: Vec::with_capacity(config_count),
            plan_index: Vec::with_capacity(config_count),
            reference_plans: Vec::with_capacity(config_count),
            resolutions: Vec::with_capacity(config_count),
            config_offsets: Vec::with_capacity(config_count + 1),
            states: Vec::new(),
            moves: Vec::new(),
            system: Generator { rows: Vec::new() },
            micro: None,
        };

        for config in 0..config_count {
            let endpoints = model.endpoints_by_matrix_power(config);
            let plans = model.plans_by_depth_first_search(config);
            let index = plans
                .iter()
                .enumerate()
                .map(|(position, plan)| (plan.clone(), position))
                .collect();
            let reference = model.plans_by_brute_force(config);
            model.matrix_count.push(endpoints.iter().sum());
            model.matrix_endpoints.push(endpoints);
            model.resolutions.push(uniform_resolution(&plans));
            model.plans.push(plans);
            model.plan_index.push(index);
            model.reference_plans.push(reference);
        }

        for config in 0..config_count {
            model.config_offsets.push(model.states.len());
            for plan in 0..model.plans[config].len() {
                for cursor in 0..=case.tau {
                    for note in model.notebook_values() {
                        for stock in 0..=case.w {
                            model.states.push(SystemState {
                                config,
                                plan,
                                cursor,
                                note,
                                stock,
                            });
                        }
                    }
                }
            }
        }
        model.config_offsets.push(model.states.len());

        model.moves = (0..model.states.len())
            .map(|state| model.local_moves(state))
            .collect();
        model.system = model.system_generator();
        if case.n == 3 {
            model.micro = Some(model.micro_level());
        }
        model
    }

    /// The case whose section 5 predictions apply. C09 changes the world but
    /// keeps the `W = 2` predictions of the primary.
    pub(crate) fn predicted_case(&self) -> Case {
        match self.mutation {
            Mutation::StockpileSlip => PRIMARY,
            _ => self.case,
        }
    }

    /// Whether the section 5.1 primary-witness predictions apply.
    pub(crate) fn is_primary(&self) -> bool {
        self.predicted_case() == PRIMARY
    }

    pub(crate) fn bridge_count(&self) -> usize {
        self.bridges.len()
    }

    /// Notebook alphabet size `A = N`.
    pub(crate) fn alphabet_size(&self) -> usize {
        self.case.n
    }

    /// Notebook values in canonical order: blank first, then islands.
    pub(crate) fn notebook_values(&self) -> Vec<Note> {
        std::iter::once(None)
            .chain((0..self.case.n).map(Some))
            .collect()
    }

    fn note_code(note: Note) -> usize {
        note.map_or(0, |island| island + 1)
    }

    pub(crate) fn size(&self, config: usize) -> usize {
        config.count_ones() as usize
    }

    /// Fixed total `E_tot = K + W` packets.
    pub(crate) fn total_packets(&self) -> usize {
        self.bridge_count() + self.case.w
    }

    /// `U(z) / lambda = |G| + w`.
    pub(crate) fn system_packets(&self, state: usize) -> usize {
        let SystemState { config, stock, .. } = self.states[state];
        self.size(config) + stock
    }

    /// `U(z) = lambda (|G| + w)`.
    pub(crate) fn system_energy(&self, state: usize) -> i64 {
        LAMBDA * self.system_packets(state) as i64
    }

    /// `E_R = K + W - |G| - w`.
    pub(crate) fn reservoir_energy(&self, state: usize) -> usize {
        self.total_packets() - self.system_packets(state)
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

    /// One plan step `v -> w` (section 3.1): stay, or cross a built bridge.
    pub(crate) fn step_allowed(&self, config: usize, v: usize, w: usize) -> bool {
        v == w || self.bridge_built(config, v, w)
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

    /// Destination `d(p) = v_tau` of a plan.
    pub(crate) fn destination(sequence: &[usize]) -> usize {
        sequence[sequence.len() - 1]
    }

    /// Island `x(z) = v_t` on which the traveller stands.
    pub(crate) fn position(&self, state: usize) -> usize {
        self.plan_of(state)[self.states[state].cursor]
    }

    pub(crate) fn states_of(&self, config: usize) -> std::ops::Range<usize> {
        self.config_offsets[config]..self.config_offsets[config + 1]
    }

    /// Index of `(G, p, t, y, w)` with `p` the plan of index `plan` in
    /// `Pi_tau(G)`.
    pub(crate) fn state_at(
        &self,
        config: usize,
        plan: usize,
        cursor: usize,
        note: Note,
        stock: usize,
    ) -> usize {
        let notes = self.case.n + 1;
        let stocks = self.case.w + 1;
        self.config_offsets[config]
            + ((plan * (self.case.tau + 1) + cursor) * notes + Self::note_code(note)) * stocks
            + stock
    }

    /// Index of the system state with these registers, if it is a state of
    /// the declared state space.
    pub(crate) fn state_index(
        &self,
        config: usize,
        sequence: &[usize],
        cursor: usize,
        note: Note,
        stock: usize,
    ) -> Option<usize> {
        if config >= self.config_count
            || cursor > self.case.tau
            || stock > self.case.w
            || note.is_some_and(|island| island >= self.case.n)
        {
            return None;
        }
        let plan = *self.plan_index[config].get(sequence)?;
        Some(self.state_at(config, plan, cursor, note, stock))
    }

    pub(crate) fn complete_state_count(&self) -> usize {
        match &self.micro {
            Some(micro) => micro.len(),
            None => (0..self.states.len())
                .map(|state| {
                    integer_power(self.case.b, self.reservoir_energy(state) as u64) as usize
                })
                .sum(),
        }
    }

    pub(crate) fn config_label(&self, config: usize) -> String {
        config_label(&self.bridges, config)
    }

    pub(crate) fn describe_state(&self, state: usize) -> String {
        let SystemState {
            config,
            cursor,
            note,
            stock,
            ..
        } = self.states[state];
        format!(
            "z{state}[G={} p={} t={cursor} y={} w={stock}]",
            self.config_label(config),
            plan_label(self.plan_of(state)),
            note_label(note)
        )
    }

    // ------------------------------------------------ mutation hooks

    /// MEASURE copies `d(p)`; C03 writes island 0 regardless.
    pub(crate) fn measured_symbol(&self, sequence: &[usize]) -> usize {
        match self.mutation {
            Mutation::BlindMeasurement => 0,
            _ => Self::destination(sequence),
        }
    }

    /// HARVEST permission at cursor 0 (section 3.3): a record equal to
    /// `d(p)` and `w < W`. C04 accepts any written record; C05 also a blank
    /// notebook whenever the held plan ends at home.
    pub(crate) fn harvest_allowed(&self, sequence: &[usize], note: Note, stock: usize) -> bool {
        if stock >= self.case.w {
            return false;
        }
        let destination = Self::destination(sequence);
        match note {
            Some(record) => record == destination || self.mutation == Mutation::CheatingHarvest,
            None => self.mutation == Mutation::TelepathicDemon && destination == HOME,
        }
    }

    /// Whether HARVEST from plan `from` releases into plan `to` (every plan;
    /// C07 keeps the held plan only).
    pub(crate) fn harvest_reaches(&self, from: usize, to: usize) -> bool {
        self.mutation != Mutation::HoardingDemon || from == to
    }

    /// Destination stockpile of ERASE from a written notebook at stockpile
    /// `stock`: one packet to the reservoir. C02 keeps the stockpile.
    pub(crate) fn erase_target_stock(&self, stock: usize) -> Option<usize> {
        match self.mutation {
            Mutation::FreeReset => Some(stock),
            _ => stock.checked_sub(1),
        }
    }

    /// Destination stockpile of SCRIBBLE from a blank notebook: one packet
    /// from the reservoir. C02 keeps the stockpile.
    pub(crate) fn scribble_target_stock(&self, stock: usize) -> Option<usize> {
        match self.mutation {
            Mutation::FreeReset => Some(stock),
            _ => (stock < self.case.w).then_some(stock + 1),
        }
    }

    /// Record marginal of the shuffled law `B_G` (section 4.4): the record
    /// marginal of `A_G`. C08 substitutes the uniform law on `V`.
    pub(crate) fn shuffled_record_marginal(&self, correlated: &[Ratio]) -> Vec<Ratio> {
        match self.mutation {
            Mutation::UnequalShuffle => {
                vec![Ratio::new(1, self.case.n as i128); self.case.n]
            }
            _ => correlated.to_vec(),
        }
    }

    // ------------------------------------------------ construction

    fn walk_matrix(&self, config: usize) -> Vec<Vec<i128>> {
        let n = self.case.n;
        let mut matrix = vec![vec![0_i128; n]; n];
        for (v, row) in matrix.iter_mut().enumerate() {
            row[v] = 1;
        }
        for (bridge, &(u, w)) in self.bridges.iter().enumerate() {
            if self.has_bridge(config, bridge) {
                matrix[u][w] = 1;
                matrix[w][u] = 1;
            }
        }
        matrix
    }

    /// Home row of `L_G^tau` by integer matrix powers: `e_h^T L_G^tau e_y`.
    fn endpoints_by_matrix_power(&self, config: usize) -> Vec<i128> {
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
        power[HOME].clone()
    }

    fn plans_by_depth_first_search(&self, config: usize) -> Vec<Vec<usize>> {
        let mut plans = Vec::new();
        let mut prefix = vec![HOME];
        self.extend_plan(config, &mut prefix, &mut plans);
        plans
    }

    fn extend_plan(&self, config: usize, prefix: &mut Vec<usize>, plans: &mut Vec<Vec<usize>>) {
        if prefix.len() == self.case.tau + 1 {
            plans.push(prefix.clone());
            return;
        }
        let last = prefix[prefix.len() - 1];
        for next in 0..self.case.n {
            if self.step_allowed(config, last, next) {
                prefix.push(next);
                self.extend_plan(config, prefix, plans);
                prefix.pop();
            }
        }
    }

    fn plans_by_brute_force(&self, config: usize) -> BTreeSet<Vec<usize>> {
        let n = self.case.n;
        let length = self.case.tau + 1;
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

    #[allow(clippy::too_many_arguments)]
    fn local_move(
        &self,
        kind: Kind,
        bridge: Option<usize>,
        to_config: usize,
        register: &[usize],
        to_cursor: usize,
        to_note: Note,
        to_stock: usize,
    ) -> LocalMove {
        LocalMove {
            kind,
            bridge,
            to_config,
            register: register.to_vec(),
            to_cursor,
            to_note,
            to_stock,
            to: self.state_index(to_config, register, to_cursor, to_note, to_stock),
        }
    }

    /// Local rules of section 3.3 at one system state.
    fn local_moves(&self, state: usize) -> Vec<LocalMove> {
        let SystemState {
            config,
            plan: plan_number,
            cursor,
            note,
            stock,
        } = self.states[state];
        let plan = self.plan_of(state);
        let plans = &self.plans[config];
        let tau = self.case.tau;
        let mut moves = Vec::new();

        // STEP: walk the plan forward and back (CAL-CEF-1 section 3.4).
        if cursor < tau {
            moves.push(self.local_move(Kind::Step, None, config, plan, cursor + 1, note, stock));
        }
        if cursor > 0 {
            moves.push(self.local_move(Kind::Step, None, config, plan, cursor - 1, note, stock));
        }

        if cursor == 0 {
            // REPLAN (Crystal): every other resolution outcome.
            for written in &self.resolutions[config].outcomes {
                if written.as_slice() != plan {
                    moves.push(self.local_move(
                        Kind::Replan,
                        None,
                        config,
                        written,
                        0,
                        note,
                        stock,
                    ));
                }
            }

            // MEASURE / UNMEASURE: copy the held plan's destination.
            match note {
                None => moves.push(self.local_move(
                    Kind::Measure,
                    None,
                    config,
                    plan,
                    0,
                    Some(self.measured_symbol(plan)),
                    stock,
                )),
                Some(record) if record == self.measured_symbol(plan) => {
                    moves.push(self.local_move(
                        Kind::Unmeasure,
                        None,
                        config,
                        plan,
                        0,
                        None,
                        stock,
                    ));
                }
                Some(_) => {}
            }

            // HARVEST: release the held plan, store one packet.
            if self.harvest_allowed(plan, note, stock) {
                for (target, released) in plans.iter().enumerate() {
                    if self.harvest_reaches(plan_number, target) {
                        moves.push(self.local_move(
                            Kind::Harvest,
                            None,
                            config,
                            released,
                            0,
                            note,
                            stock + 1,
                        ));
                    }
                }
            }

            // COMMIT: the declared reverse of HARVEST.
            if self.mutation != Mutation::OneWayHarvest && stock >= 1 {
                for (target, committed) in plans.iter().enumerate() {
                    if self.harvest_allowed(committed, note, stock - 1)
                        && self.harvest_reaches(target, plan_number)
                    {
                        moves.push(self.local_move(
                            Kind::Commit,
                            None,
                            config,
                            committed,
                            0,
                            note,
                            stock - 1,
                        ));
                    }
                }
            }

            // ERASE / SCRIBBLE: reset or write the notebook against a packet.
            match note {
                Some(_) => {
                    if let Some(to_stock) = self.erase_target_stock(stock) {
                        moves.push(self.local_move(
                            Kind::Erase,
                            None,
                            config,
                            plan,
                            0,
                            None,
                            to_stock,
                        ));
                    }
                }
                None => {
                    if let Some(to_stock) = self.scribble_target_stock(stock) {
                        for symbol in 0..self.case.n {
                            moves.push(self.local_move(
                                Kind::Scribble,
                                None,
                                config,
                                plan,
                                0,
                                Some(symbol),
                                to_stock,
                            ));
                        }
                    }
                }
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
                moves.push(self.local_move(
                    Kind::Build,
                    Some(bridge),
                    target,
                    plan,
                    cursor,
                    note,
                    stock,
                ));
            } else if !self.uses_bridge(plan, bridge) {
                let target = config & !(1 << bridge);
                moves.push(self.local_move(
                    Kind::Dismantle,
                    Some(bridge),
                    target,
                    plan,
                    cursor,
                    note,
                    stock,
                ));
            }
        }
        moves
    }

    fn system_generator(&self) -> Generator {
        let rows = self
            .moves
            .iter()
            .enumerate()
            .map(|(state, moves)| {
                moves
                    .iter()
                    .filter_map(|local| {
                        let to = local.to?;
                        let inputs = HazardInputs {
                            source_reservoir_energy: self.reservoir_energy(state),
                            destination_reservoir_energy: self.reservoir_energy(to),
                            base: self.case.b,
                        };
                        Some(Edge {
                            to,
                            kind: local.kind,
                            hazard: lumped_hazard(inputs),
                        })
                    })
                    .collect()
            })
            .collect();
        Generator::from_rows(rows)
    }

    /// Complete states `(z, r)` and unit-hazard micro channels (section
    /// 3.3): a channel that moves packets reaches every reservoir label at
    /// the destination energy; any other channel keeps the label.
    fn micro_level(&self) -> MicroLevel {
        let mut offsets = Vec::with_capacity(self.states.len() + 1);
        let mut system_of = Vec::new();
        let mut label_of = Vec::new();
        for state in 0..self.states.len() {
            offsets.push(system_of.len());
            let labels = integer_power(self.case.b, self.reservoir_energy(state) as u64) as usize;
            for label in 0..labels {
                system_of.push(state);
                label_of.push(label);
            }
        }
        offsets.push(system_of.len());

        let mut rows: Vec<Vec<MicroEdge>> = Vec::with_capacity(system_of.len());
        for complete in 0..system_of.len() {
            let state = system_of[complete];
            let label = label_of[complete];
            let mut row = Vec::new();
            for local in &self.moves[state] {
                let Some(to) = local.to else { continue };
                if self.reservoir_energy(to) != self.reservoir_energy(state) {
                    for target in offsets[to]..offsets[to + 1] {
                        row.push(MicroEdge {
                            to: target as u32,
                            kind: local.kind,
                            hazard: KAPPA_MICRO,
                        });
                    }
                } else {
                    row.push(MicroEdge {
                        to: (offsets[to] + label) as u32,
                        kind: local.kind,
                        hazard: KAPPA_MICRO,
                    });
                }
            }
            row.sort_by_key(|edge| (edge.to, edge.kind));
            rows.push(row);
        }

        if self.mutation == Mutation::DirectedKinetic {
            // First HARVEST micro channel ordered by source, then target.
            'search: for row in rows.iter_mut() {
                for edge in row.iter_mut() {
                    if edge.kind == Kind::Harvest {
                        edge.hazard *= 2;
                        break 'search;
                    }
                }
            }
        }

        MicroLevel {
            offsets,
            system_of,
            label_of,
            rows,
        }
    }
}

fn uniform_resolution(plans: &[Vec<usize>]) -> Resolution {
    let count = plans.len();
    Resolution {
        probabilities: vec![Ratio::new(1, count.max(1) as i128); count],
        outcomes: plans.to_vec(),
    }
}

pub(crate) fn plan_label(sequence: &[usize]) -> String {
    sequence.iter().map(usize::to_string).collect()
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
