//! Construction of the CAL-CEF-3 worlds (boundary section 1).
//!
//! Everything is built from the local rules: candidate bridges in
//! lexicographic order, configurations in bit order, plans by depth-first
//! search, `N_tau(G)` and `N_R(G)` by integer matrix powers of
//! `L_G = A_G + I`, system states `(G, p, t)` ordered by configuration, then
//! plan, then cursor, and the STEP, REPLAN, WEATHER, and FUEL channels of
//! section 1.3 with exact integer rates. Every channel carries its source,
//! destination, class, store, rate, and packet direction; a WEATHER channel
//! and a FUEL channel between the same two states are separate channels.
//! Rates are computed by `channel_rate` from `RateInputs` only, a type that
//! holds the state, the bridge, the class, and the store's ratio.

use std::collections::BTreeMap;

/// Home island `h`.
pub(crate) const HOME: usize = 0;
/// Energy `lambda` stored per built bridge.
pub(crate) const LAMBDA: u64 = 1;
/// The unit rate; it fixes the clock.
pub(crate) const UNIT_RATE: u64 = 1;
/// Reference horizon `R` of the open future `N_R(G)` (section 3).
pub(crate) const REFERENCE_HORIZON: usize = 2;

/// Declared experimental inputs `(N, tau, R, b_c, b_h)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WorldSpec {
    pub(crate) n: usize,
    pub(crate) tau: usize,
    pub(crate) r: usize,
    pub(crate) b_c: u64,
    pub(crate) b_h: u64,
}

impl WorldSpec {
    pub(crate) const fn new(n: usize, tau: usize, b_c: u64, b_h: u64) -> Self {
        Self {
            n,
            tau,
            r: REFERENCE_HORIZON,
            b_c,
            b_h,
        }
    }

    pub(crate) fn label(&self) -> String {
        format!(
            "({},{},{},{},{})",
            self.n, self.tau, self.r, self.b_c, self.b_h
        )
    }
}

/// The twelve worlds in the row order of boundary section 4.2.
pub(crate) const WORLDS: [WorldSpec; 12] = [
    WorldSpec::new(3, 1, 3, 2),
    WorldSpec::new(3, 2, 3, 2),
    WorldSpec::new(3, 3, 3, 2),
    WorldSpec::new(3, 1, 8, 2),
    WorldSpec::new(3, 2, 8, 2),
    WorldSpec::new(3, 3, 8, 2),
    WorldSpec::new(3, 1, 6, 3),
    WorldSpec::new(3, 2, 6, 3),
    WorldSpec::new(3, 3, 6, 3),
    WorldSpec::new(4, 1, 3, 2),
    WorldSpec::new(4, 1, 8, 2),
    WorldSpec::new(4, 1, 6, 3),
];

/// Primary witness `(N, tau, R, b_c, b_h) = (3, 2, 2, 8, 2)` (section 3).
pub(crate) const PRIMARY: WorldSpec = WorldSpec::new(3, 2, 8, 2);

/// The two declared agents, LOCAL before GLOBAL (section 5).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Agent {
    Local,
    Global,
}

impl Agent {
    pub(crate) const ALL: [Self; 2] = [Self::Local, Self::Global];

    pub(crate) const fn code(self) -> &'static str {
        match self {
            Self::Local => "LOCAL",
            Self::Global => "GLOBAL",
        }
    }
}

/// One declared input or reference value of the primary witness mutated by
/// a control (boundary section 8). `None` is the preregistered construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Mutation {
    None,
    /// C01: every FUEL return channel is deleted; FUEL builds stay.
    OneWayFuel,
    /// C02: WEATHER may also remove bridges the held plan uses.
    PlanBreakingWeather,
    /// C03: `b_h = b_c`.
    EqualStores,
    /// C04: the first WEATHER build in canonical state order has rate 2.
    BiasedWeather,
    /// C05: FUEL uses the cold ratio `1 : b_c`.
    WrongStore,
    /// C06: G07 checks the GLOBAL law against `q = 1/b_c`.
    WrongCalibration,
}

impl Mutation {
    /// The world actually constructed from the declared inputs (C03 is the
    /// only control that changes a declared input value).
    pub(crate) fn constructed(self, declared: WorldSpec) -> WorldSpec {
        match self {
            Self::EqualStores => WorldSpec {
                b_h: declared.b_c,
                ..declared
            },
            _ => declared,
        }
    }
}

/// Channel classes of section 1.3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Class {
    Step,
    Replan,
    Weather,
    Fuel,
}

impl Class {
    pub(crate) const fn code(self) -> &'static str {
        match self {
            Self::Step => "STEP",
            Self::Replan => "REPLAN",
            Self::Weather => "WEATHER",
            Self::Fuel => "FUEL",
        }
    }

    /// STEP, REPLAN, and WEATHER make up the undriven world (section 1.4).
    pub(crate) const fn is_undriven(self) -> bool {
        matches!(self, Self::Step | Self::Replan | Self::Weather)
    }
}

/// The two declared timber stores (section 1.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Store {
    Cold,
    Hot,
}

impl Store {
    pub(crate) const fn code(self) -> &'static str {
        match self {
            Self::Cold => "cold",
            Self::Hot => "hot",
        }
    }
}

/// Packet direction of a channel: none, one packet from the channel's store
/// into a bridge (build), or one packet from a bridge into the store.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Packet {
    Neutral,
    FromStore,
    IntoStore,
}

/// System state `z = (G, p, t)`: configuration bit mask, index of the held
/// plan in `Pi_tau(G)`, and cursor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SystemState {
    pub(crate) config: usize,
    pub(crate) plan: usize,
    pub(crate) cursor: usize,
}

/// One channel. `to` is `None` when the destination description is not a
/// system state of the world.
#[derive(Clone, Debug)]
pub(crate) struct Channel {
    pub(crate) from: usize,
    pub(crate) to: Option<usize>,
    pub(crate) to_config: usize,
    pub(crate) to_plan: Vec<usize>,
    pub(crate) to_cursor: usize,
    pub(crate) class: Class,
    pub(crate) store: Option<Store>,
    pub(crate) bridge: Option<usize>,
    pub(crate) rate: u64,
    pub(crate) packet: Packet,
}

/// Destination description of a channel before it is resolved to a state.
struct Destination<'a> {
    config: usize,
    plan: &'a [usize],
    cursor: usize,
}

/// The only inputs of a rate function (G09): the state, the bridge, the
/// channel class, and the ratio of the channel's store. There is no field for
/// `N_tau`, `N_R`, `H`, `J`, `Y`, or `eta`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RateInputs {
    pub(crate) state: SystemState,
    pub(crate) bridge: Option<usize>,
    pub(crate) class: Class,
    pub(crate) store_ratio: Option<u64>,
}

/// Quantities a rate function could in principle read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Quantity {
    State,
    Bridge,
    Class,
    StoreRatio,
    PlanCount,
    OpenFuture,
    OpenFutureHeld,
    FuelFlow,
    Yield,
    Efficiency,
}

/// What `channel_rate` reads: exactly the fields of `RateInputs`.
pub(crate) const RATE_READS: [Quantity; 4] = [
    Quantity::State,
    Quantity::Bridge,
    Quantity::Class,
    Quantity::StoreRatio,
];

/// What no rate may read (G09): `N_tau`, `N_R`, `H`, `J`, `Y`, `eta`.
pub(crate) const FORBIDDEN_RATE_READS: [Quantity; 6] = [
    Quantity::PlanCount,
    Quantity::OpenFuture,
    Quantity::OpenFutureHeld,
    Quantity::FuelFlow,
    Quantity::Yield,
    Quantity::Efficiency,
];

/// Whether no read of `RATE_READS` is forbidden.
pub(crate) const fn rate_reads_admissible() -> bool {
    let mut read = 0;
    while read < RATE_READS.len() {
        let mut forbidden = 0;
        while forbidden < FORBIDDEN_RATE_READS.len() {
            if RATE_READS[read] as u8 == FORBIDDEN_RATE_READS[forbidden] as u8 {
                return false;
            }
            forbidden += 1;
        }
        read += 1;
    }
    true
}

// Code-level assertion supporting G09: the declared rate reads exclude every
// count of futures and every measured quantity.
const _: () = assert!(
    rate_reads_admissible(),
    "a rate function may read only the state, the bridge, the class, and the store ratio"
);

/// Rate of a channel (section 1.3): STEP and REPLAN at the unit rate; a
/// toggle at the unit rate when it builds and at its store's ratio when it
/// removes. The exhaustive destructuring fails to compile if `RateInputs`
/// gains a field.
pub(crate) fn channel_rate(inputs: RateInputs) -> u64 {
    let RateInputs {
        state,
        bridge,
        class,
        store_ratio,
    } = inputs;
    match class {
        Class::Step | Class::Replan => UNIT_RATE,
        Class::Weather | Class::Fuel => {
            let bridge = bridge.expect("a toggle names its bridge");
            if state.config & (1 << bridge) == 0 {
                UNIT_RATE
            } else {
                store_ratio.expect("a toggle names its store ratio")
            }
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct Model {
    /// The section 4 row whose frozen values apply.
    pub(crate) declared: WorldSpec,
    /// The world constructed (differs from `declared` only under C03).
    pub(crate) spec: WorldSpec,
    pub(crate) agent: Agent,
    pub(crate) mutation: Mutation,
    pub(crate) bridges: Vec<(usize, usize)>,
    pub(crate) config_count: usize,
    /// `N_tau(G) = e_h^T L_G^tau 1` by integer matrix powers.
    pub(crate) matrix_count_tau: Vec<u64>,
    /// `N_R(G) = e_h^T L_G^R 1` by integer matrix powers.
    pub(crate) matrix_count_r: Vec<u64>,
    /// `Pi_tau(G)` by depth-first search, lexicographic.
    pub(crate) plans: Vec<Vec<Vec<usize>>>,
    /// `Pi_R(G)` by depth-first search, lexicographic.
    pub(crate) plans_r: Vec<Vec<Vec<usize>>>,
    pub(crate) plan_index: Vec<BTreeMap<Vec<usize>, usize>>,
    pub(crate) config_offsets: Vec<usize>,
    pub(crate) states: Vec<SystemState>,
    /// Channels out of each state, in construction order.
    pub(crate) channels: Vec<Vec<Channel>>,
    /// `(source state, channel index)` of every resolved channel into each
    /// state.
    pub(crate) incoming: Vec<Vec<(usize, usize)>>,
}

impl Model {
    pub(crate) fn build(declared: WorldSpec, agent: Agent, mutation: Mutation) -> Self {
        let spec = mutation.constructed(declared);
        let bridges: Vec<(usize, usize)> = (0..spec.n)
            .flat_map(|u| (u + 1..spec.n).map(move |w| (u, w)))
            .collect();
        let config_count = 1_usize << bridges.len();
        let mut model = Self {
            declared,
            spec,
            agent,
            mutation,
            bridges,
            config_count,
            matrix_count_tau: Vec::with_capacity(config_count),
            matrix_count_r: Vec::with_capacity(config_count),
            plans: Vec::with_capacity(config_count),
            plans_r: Vec::with_capacity(config_count),
            plan_index: Vec::with_capacity(config_count),
            config_offsets: Vec::with_capacity(config_count + 1),
            states: Vec::new(),
            channels: Vec::new(),
            incoming: Vec::new(),
        };

        for config in 0..config_count {
            let count_tau = model.walk_count(config, spec.tau);
            let count_r = model.walk_count(config, spec.r);
            let plans = model.plans_by_depth_first_search(config, spec.tau);
            let plans_r = model.plans_by_depth_first_search(config, spec.r);
            let index = plans
                .iter()
                .enumerate()
                .map(|(position, plan)| (plan.clone(), position))
                .collect();
            model.matrix_count_tau.push(count_tau);
            model.matrix_count_r.push(count_r);
            model.plans.push(plans);
            model.plans_r.push(plans_r);
            model.plan_index.push(index);
        }

        for config in 0..config_count {
            model.config_offsets.push(model.states.len());
            for plan in 0..model.plans[config].len() {
                for cursor in 0..=spec.tau {
                    model.states.push(SystemState {
                        config,
                        plan,
                        cursor,
                    });
                }
            }
        }
        model.config_offsets.push(model.states.len());

        let mut channels: Vec<Vec<Channel>> = (0..model.states.len())
            .map(|state| model.channels_from(state))
            .collect();
        if mutation == Mutation::BiasedWeather {
            bias_first_weather_build(&mut channels);
        }
        let mut incoming = vec![Vec::new(); model.states.len()];
        for (from, row) in channels.iter().enumerate() {
            for (index, channel) in row.iter().enumerate() {
                if let Some(to) = channel.to {
                    incoming[to].push((from, index));
                }
            }
        }
        model.channels = channels;
        model.incoming = incoming;
        model
    }

    // ------------------------------------------------------------ queries

    /// Number of candidate bridges `K = N(N-1)/2`.
    pub(crate) fn bridge_count(&self) -> usize {
        self.bridges.len()
    }

    /// `|G|`.
    pub(crate) fn size(&self, config: usize) -> usize {
        config.count_ones() as usize
    }

    /// `U(z) = lambda |G|`.
    pub(crate) fn energy(&self, state: usize) -> u64 {
        LAMBDA * self.size(self.states[state].config) as u64
    }

    pub(crate) fn has_bridge(&self, config: usize, bridge: usize) -> bool {
        config & (1 << bridge) != 0
    }

    /// Whether candidate bridge `bridge` has `island` as an endpoint.
    pub(crate) fn incident(&self, bridge: usize, island: usize) -> bool {
        let (u, w) = self.bridges[bridge];
        island == u || island == w
    }

    fn bridge_built(&self, config: usize, v: usize, w: usize) -> bool {
        v != w
            && self
                .bridges
                .iter()
                .position(|&bridge| bridge == (v.min(w), v.max(w)))
                .is_some_and(|bridge| self.has_bridge(config, bridge))
    }

    /// One plan step `v -> w`: stay, or cross a built bridge.
    pub(crate) fn step_allowed(&self, config: usize, v: usize, w: usize) -> bool {
        v == w || self.bridge_built(config, v, w)
    }

    /// Validity of a `tau`-step plan in a configuration, by the rule of
    /// section 1.1 (independent of the depth-first enumeration).
    pub(crate) fn plan_valid(&self, config: usize, sequence: &[usize]) -> bool {
        config < self.config_count
            && sequence.len() == self.spec.tau + 1
            && sequence[0] == HOME
            && sequence.iter().all(|&v| v < self.spec.n)
            && sequence
                .windows(2)
                .all(|step| self.step_allowed(config, step[0], step[1]))
    }

    /// A plan uses a bridge if some step crosses it.
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

    /// Index of `(G, p, t)` when it is a system state of the world.
    pub(crate) fn state_index(
        &self,
        config: usize,
        sequence: &[usize],
        cursor: usize,
    ) -> Option<usize> {
        if config >= self.config_count || cursor > self.spec.tau {
            return None;
        }
        let plan = *self.plan_index[config].get(sequence)?;
        Some(self.config_offsets[config] + plan * (self.spec.tau + 1) + cursor)
    }

    /// The open future `N_R(G)` of a state's configuration.
    pub(crate) fn open_future(&self, state: usize) -> u64 {
        self.matrix_count_r[self.states[state].config]
    }

    /// The declared stores with their ratios (section 1.2).
    pub(crate) fn stores(&self) -> [(Store, u64); 2] {
        [(Store::Cold, self.spec.b_c), (Store::Hot, self.spec.b_h)]
    }

    pub(crate) fn config_label(&self, config: usize) -> String {
        config_label(&self.bridges, config)
    }

    pub(crate) fn describe_state(&self, state: usize) -> String {
        let SystemState { config, cursor, .. } = self.states[state];
        format!(
            "z{state}[G={} p={} t={cursor}]",
            self.config_label(config),
            plan_label(self.plan_of(state))
        )
    }

    pub(crate) fn describe_channel(&self, channel: &Channel) -> String {
        let destination = match channel.to {
            Some(to) => self.describe_state(to),
            None => format!(
                "[G={} p={} t={}] (not a system state)",
                self.config_label(channel.to_config),
                plan_label(&channel.to_plan),
                channel.to_cursor
            ),
        };
        let bridge = channel
            .bridge
            .map(|bridge| {
                let (u, w) = self.bridges[bridge];
                format!(" bridge {{{u},{w}}}")
            })
            .unwrap_or_default();
        format!(
            "{}{bridge} rate {} {} -> {destination}",
            channel.class.code(),
            channel.rate,
            self.describe_state(channel.from)
        )
    }

    // ------------------------------------------------------- control hooks

    /// WEATHER wear permission: never a bridge the held plan uses; C02 lets
    /// weather remove those too.
    fn weather_may_wear(&self, sequence: &[usize], bridge: usize) -> bool {
        self.mutation == Mutation::PlanBreakingWeather || !self.uses_bridge(sequence, bridge)
    }

    /// FUEL permission of the agent: LOCAL toggles bridges incident to the
    /// traveller's island, GLOBAL every candidate bridge.
    pub(crate) fn fuel_may_toggle(&self, bridge: usize, island: usize) -> bool {
        match self.agent {
            Agent::Local => self.incident(bridge, island),
            Agent::Global => true,
        }
    }

    /// FUEL returns exist; C01 deletes them.
    fn fuel_returns_present(&self) -> bool {
        self.mutation != Mutation::OneWayFuel
    }

    /// Ratio the FUEL channels use: the hot store's `b_h`; C05 uses `b_c`.
    fn fuel_ratio(&self) -> u64 {
        match self.mutation {
            Mutation::WrongStore => self.spec.b_c,
            _ => self.spec.b_h,
        }
    }

    /// G07's reference law `q = numerator / denominator`:
    /// `2/(b_c + b_h)` from the world's own stores; C06 uses `1/b_c`.
    pub(crate) fn reference_q(&self) -> (u64, u64) {
        match self.mutation {
            Mutation::WrongCalibration => (1, self.spec.b_c),
            _ => (2, self.spec.b_c + self.spec.b_h),
        }
    }

    // -------------------------------------------------------- construction

    fn walk_matrix(&self, config: usize) -> Vec<Vec<u64>> {
        let n = self.spec.n;
        let mut matrix = vec![vec![0_u64; n]; n];
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

    /// `e_h^T L_G^steps 1` by integer matrix powers.
    fn walk_count(&self, config: usize, steps: usize) -> u64 {
        let n = self.spec.n;
        let walk = self.walk_matrix(config);
        let mut power: Vec<Vec<u64>> = (0..n)
            .map(|i| (0..n).map(|j| u64::from(i == j)).collect())
            .collect();
        for _ in 0..steps {
            power = (0..n)
                .map(|i| {
                    (0..n)
                        .map(|j| (0..n).map(|m| power[i][m] * walk[m][j]).sum())
                        .collect()
                })
                .collect();
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
        for next in 0..self.spec.n {
            if self.step_allowed(config, last, next) {
                prefix.push(next);
                self.extend_plan(config, horizon, prefix, plans);
                prefix.pop();
            }
        }
    }

    fn channel(
        &self,
        from: usize,
        class: Class,
        store: Option<Store>,
        bridge: Option<usize>,
        destination: Destination,
    ) -> Channel {
        let state = self.states[from];
        let packet = match bridge {
            None => Packet::Neutral,
            Some(bridge) if self.has_bridge(state.config, bridge) => Packet::IntoStore,
            Some(_) => Packet::FromStore,
        };
        let store_ratio = store.map(|store| match store {
            Store::Cold => self.spec.b_c,
            Store::Hot => self.fuel_ratio(),
        });
        let rate = channel_rate(RateInputs {
            state,
            bridge,
            class,
            store_ratio,
        });
        Channel {
            from,
            to: self.state_index(destination.config, destination.plan, destination.cursor),
            to_config: destination.config,
            to_plan: destination.plan.to_vec(),
            to_cursor: destination.cursor,
            class,
            store,
            bridge,
            rate,
            packet,
        }
    }

    /// The channels of section 1.3 out of one state, in the order STEP
    /// (forward, back), REPLAN (plan order), WEATHER (bridge order), FUEL
    /// (bridge order).
    fn channels_from(&self, state: usize) -> Vec<Channel> {
        let SystemState { config, cursor, .. } = self.states[state];
        let plan = self.plan_of(state);
        let tau = self.spec.tau;
        let mut out = Vec::new();
        let here = |config: usize, cursor: usize| Destination {
            config,
            plan,
            cursor,
        };

        // STEP: walk the plan forward and back.
        if cursor < tau {
            out.push(self.channel(state, Class::Step, None, None, here(config, cursor + 1)));
        }
        if cursor > 0 {
            out.push(self.channel(state, Class::Step, None, None, here(config, cursor - 1)));
        }

        // REPLAN (Crystal): at cursor 0, every other valid plan.
        if cursor == 0 {
            for other in &self.plans[config] {
                if other.as_slice() != plan {
                    let destination = Destination {
                        config,
                        plan: other,
                        cursor: 0,
                    };
                    out.push(self.channel(state, Class::Replan, None, None, destination));
                }
            }
        }

        // WEATHER (environment, cold store): every candidate bridge.
        for bridge in 0..self.bridge_count() {
            let bit = 1_usize << bridge;
            let target = if config & bit == 0 {
                Some(config | bit)
            } else if self.weather_may_wear(plan, bridge) {
                Some(config & !bit)
            } else {
                None
            };
            if let Some(target) = target {
                out.push(self.channel(
                    state,
                    Class::Weather,
                    Some(Store::Cold),
                    Some(bridge),
                    here(target, cursor),
                ));
            }
        }

        // FUEL (agent, hot store): the agent's permitted bridges.
        let island = plan[cursor];
        for bridge in 0..self.bridge_count() {
            if !self.fuel_may_toggle(bridge, island) {
                continue;
            }
            let bit = 1_usize << bridge;
            let target = if config & bit == 0 {
                Some(config | bit)
            } else if !self.uses_bridge(plan, bridge) && self.fuel_returns_present() {
                Some(config & !bit)
            } else {
                None
            };
            if let Some(target) = target {
                out.push(self.channel(
                    state,
                    Class::Fuel,
                    Some(Store::Hot),
                    Some(bridge),
                    here(target, cursor),
                ));
            }
        }
        out
    }
}

/// C04: double the rate of the first WEATHER build in canonical state order.
fn bias_first_weather_build(channels: &mut [Vec<Channel>]) {
    if let Some(channel) = channels
        .iter_mut()
        .flat_map(|row| row.iter_mut())
        .find(|channel| channel.class == Class::Weather && channel.packet == Packet::FromStore)
    {
        channel.rate *= 2;
    }
}

pub(crate) fn plan_label(sequence: &[usize]) -> String {
    sequence.iter().map(usize::to_string).collect()
}

/// `none` for the empty configuration, else its bridges `{u,w}` in candidate
/// order.
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
