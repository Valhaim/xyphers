//! Construction of the CAL-CEF-4 worlds (boundary sections 1 and 3).
//!
//! Everything is built from the local rules: candidate bridges in
//! lexicographic order, configurations in bit order, routes `V^tau` in
//! lexicographic order read as the walk `0, v_1, ..., v_tau`, `N_2(G, x)` by
//! integer matrix products of `I + A_G`, system states `(G, r, t)` ordered by
//! configuration, then route, then cursor (every combination is a state), and
//! the STEP, REPLAN, WEATHER, and FUEL channels of section 1.3 with exact
//! integer rates. Every channel carries its source, destination, class,
//! store, rate, and packet direction; a WEATHER channel and a FUEL channel
//! between the same two states are separate channels. Rates are computed by
//! `channel_rate` from `RateInputs` only: the state, the bridge, the class,
//! the store ratio, and the agent's permission set `A(r, t)`.

use crate::symmetry::{orbits, relabellings, Orbits, Relabelling};

/// Home island `0`.
pub(crate) const HOME: usize = 0;
/// Energy `lambda` stored per built bridge.
pub(crate) const LAMBDA: u64 = 1;
/// The unit rate; it fixes the clock.
pub(crate) const UNIT_RATE: u64 = 1;

/// Declared experimental inputs `(N, tau, b_c, b_h)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct WorldSpec {
    pub(crate) n: usize,
    pub(crate) tau: usize,
    pub(crate) b_c: u64,
    pub(crate) b_h: u64,
}

impl WorldSpec {
    pub(crate) const fn new(n: usize, tau: usize, b_c: u64, b_h: u64) -> Self {
        Self { n, tau, b_c, b_h }
    }

    pub(crate) fn label(&self) -> String {
        format!("({},{},{},{})", self.n, self.tau, self.b_c, self.b_h)
    }

    /// `K = N(N-1)/2`.
    pub(crate) const fn bridge_count(&self) -> usize {
        self.n * (self.n - 1) / 2
    }

    /// The agents of section 3 in the order listed: LOCAL, AIM-1 to
    /// AIM-tau, GLOBAL.
    pub(crate) fn agents(&self) -> Vec<Agent> {
        let mut agents = vec![Agent::Local];
        agents.extend((1..=self.tau).map(Agent::Aim));
        agents.push(Agent::Global);
        agents
    }
}

/// The temperature pairs `(b_c, b_h)` in the run order of section 3.
pub(crate) const PAIRS: [(u64, u64); 3] = [(3, 2), (8, 2), (6, 3)];

/// The eighteen worlds of section 3: pair by pair, rows in table order.
pub(crate) const WORLDS: [WorldSpec; 18] = [
    WorldSpec::new(3, 1, 3, 2),
    WorldSpec::new(3, 2, 3, 2),
    WorldSpec::new(3, 3, 3, 2),
    WorldSpec::new(4, 1, 3, 2),
    WorldSpec::new(4, 2, 3, 2),
    WorldSpec::new(5, 1, 3, 2),
    WorldSpec::new(3, 1, 8, 2),
    WorldSpec::new(3, 2, 8, 2),
    WorldSpec::new(3, 3, 8, 2),
    WorldSpec::new(4, 1, 8, 2),
    WorldSpec::new(4, 2, 8, 2),
    WorldSpec::new(5, 1, 8, 2),
    WorldSpec::new(3, 1, 6, 3),
    WorldSpec::new(3, 2, 6, 3),
    WorldSpec::new(3, 3, 6, 3),
    WorldSpec::new(4, 1, 6, 3),
    WorldSpec::new(4, 2, 6, 3),
    WorldSpec::new(5, 1, 6, 3),
];

/// Primary witness `(N, tau, b_c, b_h) = (3, 2, 8, 2)` (section 3).
pub(crate) const PRIMARY: WorldSpec = WorldSpec::new(3, 2, 8, 2);

/// The declared agents of section 1.4; they differ only in `A(r, t)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Agent {
    /// The bridges incident to the traveller's island.
    Local,
    /// AIM-k: the bridges of the moving steps `max(0, t-k)` to
    /// `min(tau, t+k) - 1` of the held route.
    Aim(usize),
    /// Every candidate bridge.
    Global,
}

impl Agent {
    pub(crate) fn code(self) -> String {
        match self {
            Self::Local => "LOCAL".to_string(),
            Self::Aim(k) => format!("AIM-{k}"),
            Self::Global => "GLOBAL".to_string(),
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
    /// C02: WEATHER never wears a bridge the held route uses.
    FreeProtection,
    /// C03: `b_h = b_c`.
    EqualStores,
    /// C04: every forward STEP has rate 2.
    BiasedStep,
    /// C05: AIM-k aims at the route steps within `k + 1` moves.
    AimOffByOne,
    /// C06: G07 checks the GLOBAL law against `q = 1/b_c`.
    WrongCalibration,
    /// C07: the rival's inverse uses `b_h` in place of `b_h + 1`.
    WrongRival,
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

    /// STEP, REPLAN, and WEATHER make up the undriven world (section 1.5).
    pub(crate) const fn is_undriven(self) -> bool {
        matches!(self, Self::Step | Self::Replan | Self::Weather)
    }
}

/// The two declared stores (section 1.2).
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

/// System state `z = (G, r, t)`: configuration bit mask, index of the held
/// route in lexicographic order, and cursor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SystemState {
    pub(crate) config: usize,
    pub(crate) route: usize,
    pub(crate) cursor: usize,
}

/// One channel. `to` is `None` when the destination description is not a
/// system state of the world.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Channel {
    pub(crate) from: usize,
    pub(crate) to: Option<usize>,
    pub(crate) to_config: usize,
    pub(crate) to_route: usize,
    pub(crate) to_cursor: usize,
    pub(crate) class: Class,
    pub(crate) store: Option<Store>,
    pub(crate) bridge: Option<usize>,
    pub(crate) rate: u64,
    pub(crate) packet: Packet,
}

/// The only inputs of a rate function (G10 check 3): the state, the bridge,
/// the channel class, the ratio of the channel's store, and the agent's
/// permission set `A(r, t)` as a bridge mask. There is no field for `N_2`,
/// `H`, `J`, `Y`, `E`, or `eta`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct RateInputs {
    pub(crate) state: SystemState,
    pub(crate) bridge: Option<usize>,
    pub(crate) class: Class,
    pub(crate) store_ratio: Option<u64>,
    pub(crate) permission: usize,
}

/// Quantities a rate function could in principle read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Quantity {
    State,
    Bridge,
    Class,
    StoreRatio,
    PermissionSet,
    OpenFuture,
    OpenFutureHeld,
    FuelFlow,
    Yield,
    Edge,
    Efficiency,
}

/// What `channel_rate` reads: exactly the fields of `RateInputs`.
pub(crate) const RATE_READS: [Quantity; 5] = [
    Quantity::State,
    Quantity::Bridge,
    Quantity::Class,
    Quantity::StoreRatio,
    Quantity::PermissionSet,
];

/// What no rate may read (G10 check 3): `N_2`, `H`, `J`, `Y`, `E`, `eta`.
pub(crate) const FORBIDDEN_RATE_READS: [Quantity; 6] = [
    Quantity::OpenFuture,
    Quantity::OpenFutureHeld,
    Quantity::FuelFlow,
    Quantity::Yield,
    Quantity::Edge,
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

// Code-level assertion supporting G10 check 3: the declared rate reads
// exclude every count of futures and every measured quantity.
const _: () = assert!(
    rate_reads_admissible(),
    "a rate may read only the state, the bridge, the class, the store ratio, and A(r, t)"
);

/// Rate of a channel (section 1.3), `None` when the rule grants no channel:
/// STEP and REPLAN at the unit rate; a WEATHER toggle on any bridge and a
/// FUEL toggle on a bridge of `A(r, t)` at the unit rate when it builds and
/// at its store's ratio when it removes. The exhaustive destructuring fails
/// to compile if `RateInputs` gains a field.
pub(crate) fn channel_rate(inputs: RateInputs) -> Option<u64> {
    let RateInputs {
        state,
        bridge,
        class,
        store_ratio,
        permission,
    } = inputs;
    match class {
        Class::Step | Class::Replan => Some(UNIT_RATE),
        Class::Weather | Class::Fuel => {
            let bit = 1_usize << bridge?;
            if class == Class::Fuel && permission & bit == 0 {
                return None;
            }
            if state.config & bit == 0 {
                Some(UNIT_RATE)
            } else {
                store_ratio
            }
        }
    }
}

/// Candidate bridges `{u, w}`, `u < w`, in lexicographic order.
pub(crate) fn candidate_bridges(n: usize) -> Vec<(usize, usize)> {
    (0..n)
        .flat_map(|u| (u + 1..n).map(move |w| (u, w)))
        .collect()
}

/// `V^tau` in lexicographic order, each route read as the walk
/// `(0, v_1, ..., v_tau)`.
pub(crate) fn lexicographic_routes(n: usize, tau: usize) -> Vec<Vec<usize>> {
    let count = n.pow(tau as u32);
    (0..count)
        .map(|index| {
            let mut walk = vec![HOME; tau + 1];
            let mut rest = index;
            for slot in walk[1..].iter_mut().rev() {
                *slot = rest % n;
                rest /= n;
            }
            walk
        })
        .collect()
}

/// Lexicographic index of the route whose walk is `walk`.
pub(crate) fn route_index(n: usize, walk: &[usize]) -> usize {
    walk[1..].iter().fold(0, |index, &v| index * n + v)
}

/// `N_2(G, x) = e_x^T (I + A_G)^2 1` for every island `x`, by integer
/// matrix products.
pub(crate) fn open_future_by_matrix(
    n: usize,
    bridges: &[(usize, usize)],
    config: usize,
) -> Vec<u64> {
    let mut walk = vec![vec![0_u64; n]; n];
    for (v, row) in walk.iter_mut().enumerate() {
        row[v] = 1;
    }
    for (bridge, &(u, w)) in bridges.iter().enumerate() {
        if config & (1 << bridge) != 0 {
            walk[u][w] = 1;
            walk[w][u] = 1;
        }
    }
    let square: Vec<Vec<u64>> = walk
        .iter()
        .map(|row| {
            (0..n)
                .map(|j| row.iter().zip(&walk).map(|(a, column)| a * column[j]).sum())
                .collect()
        })
        .collect();
    square.iter().map(|row| row.iter().sum()).collect()
}

#[derive(Clone, Debug)]
pub(crate) struct Model {
    /// The section 3 world whose frozen values apply.
    pub(crate) declared: WorldSpec,
    /// The world constructed (differs from `declared` only under C03).
    pub(crate) spec: WorldSpec,
    pub(crate) agent: Agent,
    pub(crate) mutation: Mutation,
    pub(crate) bridges: Vec<(usize, usize)>,
    /// `bridge_of[u][w]`: the candidate bridge `{u, w}`, if `u != w`.
    bridge_of: Vec<Vec<Option<usize>>>,
    pub(crate) config_count: usize,
    /// Every route as its walk `(0, v_1, ..., v_tau)`, lexicographic.
    pub(crate) routes: Vec<Vec<usize>>,
    /// The bridge of each step of each route; `None` when the step stays.
    pub(crate) step_bridges: Vec<Vec<Option<usize>>>,
    /// The bridges each route uses, as a mask.
    route_uses: Vec<usize>,
    /// `A(r, t)` as constructed, `[route][cursor]`, as a bridge mask.
    pub(crate) permissions: Vec<Vec<usize>>,
    /// `N_2(G, x)` by integer matrix products, `[config][island]`.
    pub(crate) open_future: Vec<Vec<u64>>,
    pub(crate) states: Vec<SystemState>,
    /// Channels out of each state, in construction order.
    pub(crate) channels: Vec<Vec<Channel>>,
    /// `(source state, channel index)` of every resolved channel into each
    /// state.
    pub(crate) incoming: Vec<Vec<(usize, usize)>>,
    /// Relabellings of islands `1, ..., N-1` with home fixed; identity first.
    pub(crate) relabellings: Vec<Relabelling>,
    pub(crate) orbits: Orbits,
}

impl Model {
    pub(crate) fn build(declared: WorldSpec, agent: Agent, mutation: Mutation) -> Self {
        let spec = mutation.constructed(declared);
        let n = spec.n;
        let bridges = candidate_bridges(n);
        let mut bridge_of = vec![vec![None; n]; n];
        for (bridge, &(u, w)) in bridges.iter().enumerate() {
            bridge_of[u][w] = Some(bridge);
            bridge_of[w][u] = Some(bridge);
        }
        let config_count = 1_usize << bridges.len();
        let routes = lexicographic_routes(n, spec.tau);
        let step_bridges: Vec<Vec<Option<usize>>> = routes
            .iter()
            .map(|walk| {
                walk.windows(2)
                    .map(|step| bridge_of[step[0]][step[1]])
                    .collect()
            })
            .collect();
        let route_uses = step_bridges
            .iter()
            .map(|steps| steps.iter().flatten().fold(0, |mask, &b| mask | (1 << b)))
            .collect();
        let open_future = (0..config_count)
            .map(|config| open_future_by_matrix(n, &bridges, config))
            .collect();
        let relabellings = relabellings(n, &bridges, &routes);
        let mut model = Self {
            declared,
            spec,
            agent,
            mutation,
            bridges,
            bridge_of,
            config_count,
            routes,
            step_bridges,
            route_uses,
            permissions: Vec::new(),
            open_future,
            states: Vec::new(),
            channels: Vec::new(),
            incoming: Vec::new(),
            relabellings,
            orbits: Orbits::default(),
        };
        model.permissions = (0..model.routes.len())
            .map(|route| {
                (0..=spec.tau)
                    .map(|cursor| model.permission_set(route, cursor))
                    .collect()
            })
            .collect();
        for config in 0..config_count {
            for route in 0..model.routes.len() {
                for cursor in 0..=spec.tau {
                    model.states.push(SystemState {
                        config,
                        route,
                        cursor,
                    });
                }
            }
        }
        let channels: Vec<Vec<Channel>> = (0..model.states.len())
            .map(|state| model.channels_from(state))
            .collect();
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
        model.orbits = orbits(&model);
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

    /// The walk `(0, v_1, ..., v_tau)` of the route held at a state.
    pub(crate) fn walk_of(&self, state: usize) -> &[usize] {
        &self.routes[self.states[state].route]
    }

    /// Island `x(z) = v_t` on which the traveller stands.
    pub(crate) fn position(&self, state: usize) -> usize {
        self.walk_of(state)[self.states[state].cursor]
    }

    /// `N_2(G, x(z))` at a state.
    pub(crate) fn open_future_at(&self, state: usize) -> u64 {
        self.open_future[self.states[state].config][self.position(state)]
    }

    /// Index of `(G, r, t)` when it is a system state of the world.
    pub(crate) fn state_index(&self, config: usize, route: usize, cursor: usize) -> Option<usize> {
        if config >= self.config_count || route >= self.routes.len() || cursor > self.spec.tau {
            return None;
        }
        Some((config * self.routes.len() + route) * (self.spec.tau + 1) + cursor)
    }

    /// The image of a state under a relabelling of the islands.
    pub(crate) fn image_state(&self, relabelling: &Relabelling, state: usize) -> usize {
        let SystemState {
            config,
            route,
            cursor,
        } = self.states[state];
        self.state_index(
            relabelling.configs[config],
            relabelling.routes[route],
            cursor,
        )
        .expect("a relabelling maps states to states")
    }

    /// The declared stores with their ratios (section 1.2).
    pub(crate) fn stores(&self) -> [(Store, u64); 2] {
        [(Store::Cold, self.spec.b_c), (Store::Hot, self.spec.b_h)]
    }

    fn store_ratio(&self, store: Store) -> u64 {
        match store {
            Store::Cold => self.spec.b_c,
            Store::Hot => self.spec.b_h,
        }
    }

    pub(crate) fn config_label(&self, config: usize) -> String {
        config_label(&self.bridges, config)
    }

    pub(crate) fn describe_state(&self, state: usize) -> String {
        let SystemState { config, cursor, .. } = self.states[state];
        format!(
            "z{state}[G={} r={} t={cursor}]",
            self.config_label(config),
            route_label(self.walk_of(state))
        )
    }

    pub(crate) fn describe_channel(&self, channel: &Channel) -> String {
        let destination = match channel.to {
            Some(to) => self.describe_state(to),
            None => format!(
                "[G={} r#{} t={}] (not a system state)",
                self.config_label(channel.to_config),
                channel.to_route,
                channel.to_cursor
            ),
        };
        let bridge = channel
            .bridge
            .and_then(|bridge| self.bridges.get(bridge))
            .map(|(u, w)| format!(" bridge {{{u},{w}}}"))
            .unwrap_or_default();
        format!(
            "{}{bridge} rate {} {} -> {destination}",
            channel.class.code(),
            channel.rate,
            self.describe_state(channel.from)
        )
    }

    // ------------------------------------------------------- control hooks

    /// WEATHER wear permission: every present bridge; C02 protects the
    /// bridges the held route uses (the CAL-CEF-3 rule).
    fn weather_may_wear(&self, route: usize, bridge: usize) -> bool {
        self.mutation != Mutation::FreeProtection || self.route_uses[route] & (1 << bridge) == 0
    }

    /// FUEL returns exist; C01 deletes them.
    fn fuel_returns_present(&self) -> bool {
        self.mutation != Mutation::OneWayFuel
    }

    /// Factor on the forward STEP rate: 1; C04 makes it 2.
    fn forward_step_factor(&self) -> u64 {
        match self.mutation {
            Mutation::BiasedStep => 2,
            _ => 1,
        }
    }

    /// The depth AIM-k aims at: `k`; C05 aims at `k + 1`.
    fn aim_depth(&self, k: usize) -> usize {
        match self.mutation {
            Mutation::AimOffByOne => k + 1,
            _ => k,
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

    /// The weight of `J` in the denominator of the rival's inverse
    /// `s*(J) = J (b_c + 1)/[K (b_c - b_h) - J w]`: `w = b_h + 1`; C07 uses
    /// `b_h`.
    pub(crate) fn rival_hot_weight(&self) -> u64 {
        match self.mutation {
            Mutation::WrongRival => self.spec.b_h,
            _ => self.spec.b_h + 1,
        }
    }

    // -------------------------------------------------------- construction

    /// The agent's permission set `A(r, t)` (section 1.4) as a bridge mask.
    fn permission_set(&self, route: usize, cursor: usize) -> usize {
        match self.agent {
            Agent::Local => {
                let island = self.routes[route][cursor];
                self.bridges
                    .iter()
                    .enumerate()
                    .filter(|(_, &(u, w))| u == island || w == island)
                    .fold(0, |mask, (bridge, _)| mask | (1 << bridge))
            }
            Agent::Aim(k) => {
                let depth = self.aim_depth(k);
                let first = cursor.saturating_sub(depth);
                let last = (cursor + depth).min(self.spec.tau);
                self.step_bridges[route][first..last]
                    .iter()
                    .flatten()
                    .fold(0, |mask, &bridge| mask | (1 << bridge))
            }
            Agent::Global => (1 << self.bridge_count()) - 1,
        }
    }

    /// Step `step` of a route is open in a configuration: it stays, or its
    /// bridge is in `G`.
    fn step_open(&self, config: usize, route: usize, step: usize) -> bool {
        self.step_bridges[route][step].is_none_or(|bridge| self.has_bridge(config, bridge))
    }

    fn channel(
        &self,
        from: usize,
        class: Class,
        bridge: Option<usize>,
        destination: SystemState,
    ) -> Option<Channel> {
        let state = self.states[from];
        let store = match class {
            Class::Step | Class::Replan => None,
            Class::Weather => Some(Store::Cold),
            Class::Fuel => Some(Store::Hot),
        };
        let rate = channel_rate(RateInputs {
            state,
            bridge,
            class,
            store_ratio: store.map(|store| self.store_ratio(store)),
            permission: self.permissions[state.route][state.cursor],
        })?;
        let packet = match bridge {
            None => Packet::Neutral,
            Some(bridge) if self.has_bridge(state.config, bridge) => Packet::IntoStore,
            Some(_) => Packet::FromStore,
        };
        Some(Channel {
            from,
            to: self.state_index(destination.config, destination.route, destination.cursor),
            to_config: destination.config,
            to_route: destination.route,
            to_cursor: destination.cursor,
            class,
            store,
            bridge,
            rate,
            packet,
        })
    }

    /// The channels of section 1.3 out of one state, in the order STEP
    /// (forward, back), REPLAN (route order), WEATHER (bridge order), FUEL
    /// (bridge order).
    fn channels_from(&self, state: usize) -> Vec<Channel> {
        let SystemState {
            config,
            route,
            cursor,
        } = self.states[state];
        let tau = self.spec.tau;
        let at = |config: usize, route: usize, cursor: usize| SystemState {
            config,
            route,
            cursor,
        };
        let mut out = Vec::new();

        // STEP: walk the route forward and back, through open steps only.
        if cursor < tau && self.step_open(config, route, cursor) {
            if let Some(mut channel) =
                self.channel(state, Class::Step, None, at(config, route, cursor + 1))
            {
                channel.rate *= self.forward_step_factor();
                out.push(channel);
            }
        }
        if cursor > 0 && self.step_open(config, route, cursor - 1) {
            out.extend(self.channel(state, Class::Step, None, at(config, route, cursor - 1)));
        }

        // REPLAN (Crystal): at cursor 0, every other route.
        if cursor == 0 {
            for other in (0..self.routes.len()).filter(|&other| other != route) {
                out.extend(self.channel(state, Class::Replan, None, at(config, other, 0)));
            }
        }

        // WEATHER (environment, cold store): every candidate bridge.
        for bridge in 0..self.bridge_count() {
            let bit = 1_usize << bridge;
            let target = if config & bit == 0 {
                Some(config | bit)
            } else if self.weather_may_wear(route, bridge) {
                Some(config & !bit)
            } else {
                None
            };
            if let Some(target) = target {
                out.extend(self.channel(
                    state,
                    Class::Weather,
                    Some(bridge),
                    at(target, route, cursor),
                ));
            }
        }

        // FUEL (Praxion, hot store): the bridges of A(r, t); the rate rule
        // grants no channel outside it.
        for bridge in 0..self.bridge_count() {
            let bit = 1_usize << bridge;
            let target = if config & bit == 0 {
                config | bit
            } else if self.fuel_returns_present() {
                config & !bit
            } else {
                continue;
            };
            out.extend(self.channel(state, Class::Fuel, Some(bridge), at(target, route, cursor)));
        }
        out
    }

    /// The candidate bridge `{u, w}`, if `u != w` are islands.
    pub(crate) fn bridge_between(&self, u: usize, w: usize) -> Option<usize> {
        self.bridge_of.get(u)?.get(w).copied().flatten()
    }
}

/// The walk of a route as its islands, e.g. `012`.
pub(crate) fn route_label(walk: &[usize]) -> String {
    walk.iter().map(usize::to_string).collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Route-and-cursor classes `(r, t)` at which two agents' permission
    /// sets differ, built for one world.
    fn differing_classes(spec: WorldSpec, left: Agent, right: Agent) -> usize {
        let a = Model::build(spec, left, Mutation::None);
        let b = Model::build(spec, right, Mutation::None);
        a.permissions
            .iter()
            .flatten()
            .zip(b.permissions.iter().flatten())
            .filter(|(x, y)| x != y)
            .count()
    }

    // Structural counts of boundary section 7; permission sets only, no
    // steady state.
    #[test]
    fn permission_sets_match_the_section_7_counts() {
        let world = |n, tau| WorldSpec::new(n, tau, 8, 2);
        assert_eq!(
            27 - differing_classes(world(3, 2), Agent::Aim(1), Agent::Local),
            2
        );
        assert_eq!(
            108 - differing_classes(world(3, 3), Agent::Aim(1), Agent::Local),
            12
        );
        for (n, tau) in [(4_usize, 1_usize), (4, 2), (5, 1)] {
            let classes = n.pow(tau as u32) * (tau + 1);
            assert_eq!(
                differing_classes(world(n, tau), Agent::Aim(1), Agent::Local),
                classes
            );
        }
        assert_eq!(
            differing_classes(world(3, 2), Agent::Aim(1), Agent::Aim(2)),
            8
        );
        assert_eq!(
            differing_classes(world(4, 2), Agent::Aim(1), Agent::Aim(2)),
            18
        );
        assert_eq!(
            differing_classes(world(3, 3), Agent::Aim(1), Agent::Aim(2)),
            44
        );
        assert_eq!(
            differing_classes(world(3, 3), Agent::Aim(2), Agent::Aim(3)),
            20
        );
        assert_eq!(
            27 - differing_classes(world(3, 2), Agent::Aim(2), Agent::Local),
            2
        );
        assert_eq!(
            108 - differing_classes(world(3, 3), Agent::Aim(3), Agent::Local),
            16
        );
    }

    #[test]
    fn aim_one_is_inside_local_and_aim_tau_is_the_whole_route() {
        let spec = WorldSpec::new(3, 3, 8, 2);
        let local = Model::build(spec, Agent::Local, Mutation::None);
        let aim_one = Model::build(spec, Agent::Aim(1), Mutation::None);
        let aim_tau = Model::build(spec, Agent::Aim(3), Mutation::None);
        for route in 0..local.routes.len() {
            for cursor in 0..=spec.tau {
                let one = aim_one.permissions[route][cursor];
                assert_eq!(one & !local.permissions[route][cursor], 0);
                assert_eq!(
                    aim_tau.permissions[route][cursor],
                    aim_tau.route_uses[route]
                );
            }
        }
    }
}
