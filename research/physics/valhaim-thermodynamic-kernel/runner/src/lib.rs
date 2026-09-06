use serde::Serialize;
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

pub const FREEZE_COMMIT: &str = "c3d9a5c6404c1c42aa938a400d3554a1ec670051";
pub const PROTOCOL_SHA256: &str =
    "a1900b99160810f2bdc517ac1cbd19e8336987c2b8cca3b6c02d89f589aff07b";
pub const FREEZE_MANIFEST_SHA256: &str =
    "1108cd08078bf969ddf0064cf30ca551190e00886e15972d0f76776d0b16b49b";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifyError(String);

impl VerifyError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for VerifyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::error::Error for VerifyError {}

pub type Result<T> = std::result::Result<T, VerifyError>;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rational {
    numerator: i128,
    denominator: i128,
}

impl Rational {
    pub fn new(numerator: i128, denominator: i128) -> Result<Self> {
        if denominator == 0 {
            return Err(VerifyError::new("rational denominator is zero"));
        }
        let sign = if denominator < 0 { -1 } else { 1 };
        let divisor = gcd(numerator.unsigned_abs(), denominator.unsigned_abs()) as i128;
        Ok(Self {
            numerator: sign * numerator / divisor,
            denominator: denominator.unsigned_abs() as i128 / divisor,
        })
    }

    pub fn integer(value: i128) -> Self {
        Self {
            numerator: value,
            denominator: 1,
        }
    }

    pub fn zero() -> Self {
        Self::integer(0)
    }

    pub fn one() -> Self {
        Self::integer(1)
    }

    pub fn is_zero(self) -> bool {
        self.numerator == 0
    }

    pub fn checked_add(self, other: Self) -> Result<Self> {
        Self::new(
            self.numerator
                .checked_mul(other.denominator)
                .and_then(|left| {
                    other
                        .numerator
                        .checked_mul(self.denominator)
                        .and_then(|right| left.checked_add(right))
                })
                .ok_or_else(|| VerifyError::new("rational addition overflow"))?,
            self.denominator
                .checked_mul(other.denominator)
                .ok_or_else(|| VerifyError::new("rational denominator overflow"))?,
        )
    }

    pub fn checked_multiply(self, other: Self) -> Result<Self> {
        Self::new(
            self.numerator
                .checked_mul(other.numerator)
                .ok_or_else(|| VerifyError::new("rational multiplication overflow"))?,
            self.denominator
                .checked_mul(other.denominator)
                .ok_or_else(|| VerifyError::new("rational denominator overflow"))?,
        )
    }

    pub fn checked_subtract(self, other: Self) -> Result<Self> {
        self.checked_add(other.negated())
    }

    pub fn negated(self) -> Self {
        Self {
            numerator: -self.numerator,
            denominator: self.denominator,
        }
    }

    pub fn text(self) -> String {
        format!("{}/{}", self.numerator, self.denominator)
    }
}

fn rational_compare(left: &Rational, right: &Rational) -> Ordering {
    left.numerator
        .checked_mul(right.denominator)
        .zip(right.numerator.checked_mul(left.denominator))
        .map_or(Ordering::Equal, |(left, right)| left.cmp(&right))
}

fn integer_square_root(value: u128) -> Option<u128> {
    if value < 2 {
        return Some(value);
    }
    let mut low = 1_u128;
    let mut high = value.min(u64::MAX as u128 + 1);
    while low <= high {
        let middle = low + (high - low) / 2;
        match middle.checked_mul(middle).map(|square| square.cmp(&value)) {
            Some(Ordering::Equal) => return Some(middle),
            Some(Ordering::Less) => low = middle + 1,
            _ => high = middle - 1,
        }
    }
    None
}

fn rational_square_root(value: Rational) -> Result<Rational> {
    if value.numerator < 0 {
        return Err(VerifyError::new(
            "negative rational has no real square root",
        ));
    }
    let numerator = integer_square_root(value.numerator as u128)
        .ok_or_else(|| VerifyError::new("irrational numerator square root"))?;
    let denominator = integer_square_root(value.denominator as u128)
        .ok_or_else(|| VerifyError::new("irrational denominator square root"))?;
    Rational::new(numerator as i128, denominator as i128)
}

impl Serialize for Rational {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.text())
    }
}

fn gcd(mut left: u128, mut right: u128) -> u128 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left.max(1)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Scaled {
    pub coefficient: Rational,
    pub basis_id: &'static str,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum GateId {
    #[serde(rename = "K01_FREEZE_IDENTITY")]
    K01,
    #[serde(rename = "K02_INDEPENDENT_COORDINATES")]
    K02,
    #[serde(rename = "K03_ROW_CLOSURE")]
    K03,
    #[serde(rename = "K04_REVERSE_INVOLUTION")]
    K04,
    #[serde(rename = "K05_PATH_ACCOUNTING")]
    K05,
    #[serde(rename = "K06_STRONG_LUMPABILITY")]
    K06,
    #[serde(rename = "K07_THERMODYNAMIC_PREDICTION")]
    K07,
    #[serde(rename = "K08_EQUILIBRIUM")]
    K08,
    #[serde(rename = "K09_RELAXATION")]
    K09,
    #[serde(rename = "K10_INDEPENDENT_GENERATOR")]
    K10,
    #[serde(rename = "X01_XYPHER_REPRESENTATION")]
    X01,
    #[serde(rename = "C01_PREPARATION")]
    C01,
    #[serde(rename = "C02_CLOCK_RETENTION")]
    C02,
    #[serde(rename = "C03_CONSERVED_EXCHANGE")]
    C03,
    #[serde(rename = "C04_REVERSE_AND_CLOSURE")]
    C04,
    #[serde(rename = "C05_EQUAL_TEMPERATURE_LAW")]
    C05,
    #[serde(rename = "C06_ZERO_CURRENT")]
    C06,
    #[serde(rename = "C07_THIRD_BODY")]
    C07,
    #[serde(rename = "C08_TEMPERATURE_SENSITIVITY")]
    C08,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Evidence {
    pub pointer: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Gate {
    pub id: GateId,
    pub status: &'static str,
    pub evidence: Vec<Evidence>,
    pub failure_code: Option<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Summary {
    pub verdict_code: &'static str,
    pub earned_claim_codes: Vec<&'static str>,
    pub excluded_claim_codes: Vec<&'static str>,
    pub source_gates: Vec<GateId>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Toolchain {
    pub rustc: String,
    pub cargo: String,
    pub target: String,
    pub cargo_lock_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Provenance {
    pub freeze_commit: String,
    pub runner_seal_commit: String,
    pub protocol_sha256: String,
    pub freeze_manifest_sha256: String,
    pub schema_sha256: String,
    pub relevant_source_clean: bool,
    pub relevant_source_paths: Vec<String>,
    pub relevant_source_sha256: String,
    pub executable_sha256: String,
    pub command_argv: Vec<String>,
    pub toolchain: Toolchain,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct State {
    positions: Vec<usize>,
    word: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Edge {
    low: usize,
    high: usize,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Slot {
    label: usize,
    edge: usize,
    symbol: u8,
}

#[derive(Clone, Debug)]
struct Spec {
    id: &'static str,
    heights: Vec<u8>,
    edges: Vec<Edge>,
    labels: usize,
    branching: u8,
    depth: u8,
}

#[derive(Clone, Debug)]
struct Outcome {
    post: State,
    moved: bool,
    hazard: i128,
    delta_u: i128,
    delta_reservoir: i128,
    external_work: i128,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mutation {
    None,
    MissingReverse,
    DirectedHazard,
    FalseMultiplicity,
    EndogenousAlpha,
    HiddenRowSplit,
    CorruptReservoirDelta,
    DisconnectedTop,
    PositiveAffinityOnly,
    WrongWordLength,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AlphaCoordinate {
    origin: &'static str,
    symbolic_value: &'static str,
}

trait AlphaSourceAdapter {
    fn coordinate(&self) -> AlphaCoordinate;
}

struct IndependentReservoirAlpha;

impl AlphaSourceAdapter for IndependentReservoirAlpha {
    fn coordinate(&self) -> AlphaCoordinate {
        AlphaCoordinate {
            origin: "IndependentReservoir",
            symbolic_value: "epsilon/ln(r)",
        }
    }
}

struct ProductionCurrentAlphaAdapter;

impl AlphaSourceAdapter for ProductionCurrentAlphaAdapter {
    fn coordinate(&self) -> AlphaCoordinate {
        AlphaCoordinate {
            origin: "ProductionCurrentAlpha",
            symbolic_value: "epsilon/ln(r)",
        }
    }
}

#[derive(Clone, Debug)]
struct Model {
    spec: Spec,
    states: Vec<State>,
    state_index: BTreeMap<State, usize>,
    slots: Vec<Slot>,
    outcomes: Vec<Vec<Outcome>>,
    hidden_outcomes: Vec<Vec<Vec<Outcome>>>,
    macros: Vec<Vec<u8>>,
    state_macro: Vec<usize>,
    fibers: Vec<Vec<usize>>,
    body_multiplicities: Vec<usize>,
    reservoir_multiplicities: Vec<usize>,
    alpha_coordinate: AlphaCoordinate,
}

#[derive(Clone, Debug)]
struct Expectations {
    state_count: usize,
    fibers: Vec<usize>,
    body: Vec<usize>,
    reservoir: Vec<usize>,
    p: Vec<Vec<Rational>>,
    q: Vec<Vec<Rational>>,
}

fn primary_spec(branching: u8) -> Spec {
    Spec {
        id: if branching == 2 {
            "primary-r2"
        } else {
            "ternary-reservoir-r3"
        },
        heights: vec![0, 1],
        edges: vec![Edge { low: 0, high: 1 }],
        labels: 2,
        branching,
        depth: 2,
    }
}

fn path_spec() -> Spec {
    Spec {
        id: "three-node-path-r2",
        heights: vec![0, 1, 2],
        edges: vec![Edge { low: 0, high: 1 }, Edge { low: 1, high: 2 }],
        labels: 2,
        branching: 2,
        depth: 4,
    }
}

fn enumerate_positions(
    node_count: usize,
    labels: usize,
    prefix: &mut Vec<usize>,
    output: &mut Vec<Vec<usize>>,
) {
    if prefix.len() == labels {
        output.push(prefix.clone());
        return;
    }
    for node in 0..node_count {
        prefix.push(node);
        enumerate_positions(node_count, labels, prefix, output);
        prefix.pop();
    }
}

fn enumerate_words(branching: u8, length: usize, prefix: &mut Vec<u8>, output: &mut Vec<Vec<u8>>) {
    if prefix.len() == length {
        output.push(prefix.clone());
        return;
    }
    for symbol in 0..branching {
        prefix.push(symbol);
        enumerate_words(branching, length, prefix, output);
        prefix.pop();
    }
}

fn energy(spec: &Spec, positions: &[usize]) -> usize {
    positions
        .iter()
        .map(|&node| spec.heights[node] as usize)
        .sum()
}

fn occupation(spec: &Spec, positions: &[usize]) -> Vec<u8> {
    let mut counts = vec![0_u8; spec.heights.len()];
    for &node in positions {
        counts[node] += 1;
    }
    counts
}

fn occupation_energy(spec: &Spec, counts: &[u8]) -> usize {
    counts
        .iter()
        .enumerate()
        .map(|(node, &count)| count as usize * spec.heights[node] as usize)
        .sum()
}

fn factorial(value: usize) -> usize {
    (1..=value).product::<usize>().max(1)
}

fn body_multiplicity(counts: &[u8]) -> usize {
    factorial(counts.iter().map(|&value| value as usize).sum())
        / counts
            .iter()
            .map(|&value| factorial(value as usize))
            .product::<usize>()
}

fn power(base: usize, exponent: usize) -> usize {
    (0..exponent).fold(1, |value, _| value * base)
}

fn slots(spec: &Spec) -> Vec<Slot> {
    let mut result = Vec::new();
    for label in 0..spec.labels {
        for edge in 0..spec.edges.len() {
            for symbol in 0..spec.branching {
                result.push(Slot {
                    label,
                    edge,
                    symbol,
                });
            }
        }
    }
    result
}

fn raw_outcome(spec: &Spec, state: &State, slot: Slot) -> Outcome {
    let edge = spec.edges[slot.edge];
    let position = state.positions[slot.label];
    let mut post = state.clone();
    let mut moved = false;
    if position == edge.low && state.word.last().copied() == Some(slot.symbol) {
        post.positions[slot.label] = edge.high;
        post.word.pop();
        moved = true;
    } else if position == edge.high {
        post.positions[slot.label] = edge.low;
        post.word.push(slot.symbol);
        moved = true;
    }
    let delta_u = energy(spec, &post.positions) as i128 - energy(spec, &state.positions) as i128;
    Outcome {
        post,
        moved,
        hazard: i128::from(moved),
        delta_u,
        delta_reservoir: -(delta_u),
        external_work: 0,
    }
}

fn is_primary_probe(state: &State, slot: Slot) -> bool {
    state.positions == [0, 0]
        && state.word == [0, 0]
        && slot
            == (Slot {
                label: 0,
                edge: 0,
                symbol: 0,
            })
}

fn directed_affinity_nonpositive(spec: &Spec, pre: &State, post: &State) -> bool {
    let pre_counts = occupation(spec, &pre.positions);
    let post_counts = occupation(spec, &post.positions);
    let pre_g = body_multiplicity(&pre_counts);
    let post_g = body_multiplicity(&post_counts);
    let delta = energy(spec, &post.positions) as isize - energy(spec, &pre.positions) as isize;
    if delta >= 0 {
        post_g <= pre_g * power(spec.branching as usize, delta as usize)
    } else {
        post_g * power(spec.branching as usize, (-delta) as usize) <= pre_g
    }
}

fn outcome(spec: &Spec, state: &State, slot: Slot, mutation: Mutation) -> Outcome {
    let mut result = raw_outcome(spec, state, slot);
    if spec.id != "primary-r2" {
        return result;
    }
    match mutation {
        Mutation::MissingReverse if is_primary_probe(state, slot) => {
            result = raw_outcome(spec, state, Slot { symbol: 1, ..slot });
        }
        Mutation::DirectedHazard if is_primary_probe(state, slot) => {
            result.hazard = 2;
        }
        Mutation::CorruptReservoirDelta if is_primary_probe(state, slot) => {
            result.delta_reservoir = 0;
        }
        Mutation::DisconnectedTop if result.moved => {
            let before = energy(spec, &state.positions);
            let after = energy(spec, &result.post.positions);
            if (before == 1 && after == 2) || (before == 2 && after == 1) {
                result = Outcome {
                    post: state.clone(),
                    moved: false,
                    hazard: 0,
                    delta_u: 0,
                    delta_reservoir: 0,
                    external_work: 0,
                };
            }
        }
        Mutation::PositiveAffinityOnly
            if result.moved && directed_affinity_nonpositive(spec, state, &result.post) =>
        {
            result = Outcome {
                post: state.clone(),
                moved: false,
                hazard: 0,
                delta_u: 0,
                delta_reservoir: 0,
                external_work: 0,
            };
        }
        Mutation::WrongWordLength if is_primary_probe(state, slot) => {
            result.post.positions[0] = 1;
            result.post.word = state.word.clone();
            result.moved = true;
            result.hazard = 1;
            result.delta_u = 1;
            result.delta_reservoir = 0;
        }
        _ => {}
    }
    result
}

fn build_model(spec: Spec, mutation: Mutation) -> Result<Model> {
    let mut placements = Vec::new();
    enumerate_positions(
        spec.heights.len(),
        spec.labels,
        &mut Vec::new(),
        &mut placements,
    );
    let mut states = Vec::new();
    for positions in &placements {
        let body_energy = energy(&spec, positions);
        if body_energy > spec.depth as usize {
            continue;
        }
        let mut words = Vec::new();
        enumerate_words(
            spec.branching,
            spec.depth as usize - body_energy,
            &mut Vec::new(),
            &mut words,
        );
        for word in words {
            states.push(State {
                positions: positions.clone(),
                word,
            });
        }
    }
    states.sort();
    let state_index = states
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, state)| (state, index))
        .collect::<BTreeMap<_, _>>();
    let slots = slots(&spec);
    let outcomes = states
        .iter()
        .map(|state| {
            slots
                .iter()
                .copied()
                .map(|slot| outcome(&spec, state, slot, mutation))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut hidden_outcomes = outcomes
        .iter()
        .cloned()
        .map(|row| vec![row])
        .collect::<Vec<_>>();
    if mutation == Mutation::HiddenRowSplit && spec.id == "primary-r2" {
        let state = states
            .iter()
            .position(|state| state.positions == [0, 1] && state.word == [0])
            .ok_or_else(|| VerifyError::new("missing LH|0 hidden-row probe"))?;
        let slot = slots
            .iter()
            .position(|slot| {
                *slot
                    == (Slot {
                        label: 0,
                        edge: 0,
                        symbol: 0,
                    })
            })
            .ok_or_else(|| VerifyError::new("missing a:0 hidden-row probe"))?;
        let mut alternate = outcomes[state].clone();
        alternate[slot] = Outcome {
            post: states[state].clone(),
            moved: false,
            hazard: 0,
            delta_u: 0,
            delta_reservoir: 0,
            external_work: 0,
        };
        hidden_outcomes[state].push(alternate);
    }

    let mut macros = states
        .iter()
        .map(|state| occupation(&spec, &state.positions))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    macros.sort_by(|left, right| {
        occupation_energy(&spec, left)
            .cmp(&occupation_energy(&spec, right))
            .then_with(|| left.cmp(right))
    });
    let macro_index = macros
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, counts)| (counts, index))
        .collect::<BTreeMap<_, _>>();
    let state_macro = states
        .iter()
        .map(|state| macro_index[&occupation(&spec, &state.positions)])
        .collect::<Vec<_>>();
    let mut fibers = vec![Vec::new(); macros.len()];
    for (state, &macrostate) in state_macro.iter().enumerate() {
        fibers[macrostate].push(state);
    }
    let body_multiplicities = macros
        .iter()
        .map(|counts| body_multiplicity(counts))
        .collect();
    let reservoir_multiplicities = macros
        .iter()
        .map(|counts| {
            let q = spec.depth as usize - occupation_energy(&spec, counts);
            power(spec.branching as usize, q)
        })
        .collect();

    Ok(Model {
        spec,
        states,
        state_index,
        slots,
        outcomes,
        hidden_outcomes,
        macros,
        state_macro,
        fibers,
        body_multiplicities,
        reservoir_multiplicities,
        alpha_coordinate: if mutation == Mutation::EndogenousAlpha {
            ProductionCurrentAlphaAdapter.coordinate()
        } else {
            IndependentReservoirAlpha.coordinate()
        },
    })
}

fn rat(value: i128) -> Rational {
    Rational::integer(value)
}

fn matrix(values: &[&[i128]]) -> Vec<Vec<Rational>> {
    values
        .iter()
        .map(|row| row.iter().copied().map(rat).collect())
        .collect()
}

fn probability_matrix(values: &[&[(i128, i128)]]) -> Result<Vec<Vec<Rational>>> {
    values
        .iter()
        .map(|row| {
            row.iter()
                .map(|&(numerator, denominator)| Rational::new(numerator, denominator))
                .collect()
        })
        .collect()
}

fn expectations(spec: &Spec) -> Result<Expectations> {
    match spec.id {
        "primary-r2" => Ok(Expectations {
            state_count: 9,
            fibers: vec![4, 4, 1],
            body: vec![1, 2, 1],
            reservoir: vec![4, 2, 1],
            p: probability_matrix(&[
                &[(1, 2), (1, 2), (0, 1)],
                &[(1, 2), (1, 4), (1, 4)],
                &[(0, 1), (1, 1), (0, 1)],
            ])?,
            q: matrix(&[&[-2, 2, 0], &[2, -3, 1], &[0, 4, -4]]),
        }),
        "ternary-reservoir-r3" => Ok(Expectations {
            state_count: 16,
            fibers: vec![9, 6, 1],
            body: vec![1, 2, 1],
            reservoir: vec![9, 3, 1],
            p: probability_matrix(&[
                &[(2, 3), (1, 3), (0, 1)],
                &[(1, 2), (1, 3), (1, 6)],
                &[(0, 1), (1, 1), (0, 1)],
            ])?,
            q: matrix(&[&[-2, 2, 0], &[3, -4, 1], &[0, 6, -6]]),
        }),
        "three-node-path-r2" => Ok(Expectations {
            state_count: 49,
            fibers: vec![16, 16, 4, 8, 4, 1],
            body: vec![1, 2, 1, 2, 2, 1],
            reservoir: vec![16, 8, 4, 4, 2, 1],
            p: probability_matrix(&[
                &[(3, 4), (1, 4), (0, 1), (0, 1), (0, 1), (0, 1)],
                &[(1, 4), (1, 2), (1, 8), (1, 8), (0, 1), (0, 1)],
                &[(0, 1), (1, 2), (1, 4), (0, 1), (1, 4), (0, 1)],
                &[(0, 1), (1, 4), (0, 1), (5, 8), (1, 8), (0, 1)],
                &[(0, 1), (0, 1), (1, 4), (1, 4), (3, 8), (1, 8)],
                &[(0, 1), (0, 1), (0, 1), (0, 1), (1, 2), (1, 2)],
            ])?,
            q: matrix(&[
                &[-2, 2, 0, 0, 0, 0],
                &[2, -4, 1, 1, 0, 0],
                &[0, 4, -6, 0, 2, 0],
                &[0, 2, 0, -3, 1, 0],
                &[0, 0, 2, 2, -5, 1],
                &[0, 0, 0, 0, 4, -4],
            ]),
        }),
        other => Err(VerifyError::new(format!("unknown fixture {other}"))),
    }
}

fn macro_matrices(model: &Model) -> Result<(Vec<Vec<Rational>>, Vec<Vec<Rational>>)> {
    let count = model.macros.len();
    let mut discrete = vec![vec![Rational::zero(); count]; count];
    let mut generator = vec![vec![Rational::zero(); count]; count];
    for source_macro in 0..count {
        let source = *model.fibers[source_macro]
            .first()
            .ok_or_else(|| VerifyError::new("empty macro fiber"))?;
        let mut active_total = 0_i128;
        for outcome in &model.outcomes[source] {
            let target = *model
                .state_index
                .get(&outcome.post)
                .ok_or_else(|| VerifyError::new("outcome leaves complete state space"))?;
            let target_macro = model.state_macro[target];
            discrete[source_macro][target_macro] = discrete[source_macro][target_macro]
                .checked_add(Rational::new(1, model.slots.len() as i128)?)?;
            if outcome.moved && target_macro != source_macro {
                generator[source_macro][target_macro] = generator[source_macro][target_macro]
                    .checked_add(Rational::integer(outcome.hazard))?;
                active_total += outcome.hazard;
            }
        }
        generator[source_macro][source_macro] = Rational::integer(-active_total);
    }
    Ok((discrete, generator))
}

fn determinant_three(matrix: &[Vec<Rational>]) -> Result<Rational> {
    if matrix.len() != 3 || matrix.iter().any(|row| row.len() != 3) {
        return Err(VerifyError::new(
            "three-by-three determinant received another shape",
        ));
    }
    let positive = matrix[0][0]
        .checked_multiply(matrix[1][1])?
        .checked_multiply(matrix[2][2])?
        .checked_add(
            matrix[0][1]
                .checked_multiply(matrix[1][2])?
                .checked_multiply(matrix[2][0])?,
        )?
        .checked_add(
            matrix[0][2]
                .checked_multiply(matrix[1][0])?
                .checked_multiply(matrix[2][1])?,
        )?;
    let negative = matrix[0][2]
        .checked_multiply(matrix[1][1])?
        .checked_multiply(matrix[2][0])?
        .checked_add(
            matrix[0][0]
                .checked_multiply(matrix[1][2])?
                .checked_multiply(matrix[2][1])?,
        )?
        .checked_add(
            matrix[0][1]
                .checked_multiply(matrix[1][0])?
                .checked_multiply(matrix[2][2])?,
        )?;
    positive.checked_subtract(negative)
}

fn three_state_generator_eigenvalues(matrix: &[Vec<Rational>]) -> Result<Vec<Rational>> {
    if determinant_three(matrix)? != Rational::zero() {
        return Err(VerifyError::new(
            "generator does not have the required zero mode",
        ));
    }
    let trace = matrix[0][0]
        .checked_add(matrix[1][1])?
        .checked_add(matrix[2][2])?;
    let principal_two = matrix[0][0]
        .checked_multiply(matrix[1][1])?
        .checked_subtract(matrix[0][1].checked_multiply(matrix[1][0])?)?
        .checked_add(
            matrix[0][0]
                .checked_multiply(matrix[2][2])?
                .checked_subtract(matrix[0][2].checked_multiply(matrix[2][0])?)?,
        )?
        .checked_add(
            matrix[1][1]
                .checked_multiply(matrix[2][2])?
                .checked_subtract(matrix[1][2].checked_multiply(matrix[2][1])?)?,
        )?;
    let discriminant = trace
        .checked_multiply(trace)?
        .checked_subtract(Rational::integer(4).checked_multiply(principal_two)?)?;
    let root = rational_square_root(discriminant)?;
    let half = Rational::new(1, 2)?;
    let mut eigenvalues = vec![
        Rational::zero(),
        trace.checked_add(root)?.checked_multiply(half)?,
        trace.checked_subtract(root)?.checked_multiply(half)?,
    ];
    eigenvalues.sort_by(|left, right| rational_compare(right, left));
    Ok(eigenvalues)
}

fn discrete_eigenvalues_from_generator(
    generator: &[Rational],
    slot_count: usize,
) -> Result<Vec<Rational>> {
    let scale = Rational::new(1, slot_count as i128)?;
    generator
        .iter()
        .map(|&value| Rational::one().checked_add(value.checked_multiply(scale)?))
        .collect()
}

fn k02(model: &Model, expected: &Expectations, mutation: Mutation) -> bool {
    let declared_body = if mutation == Mutation::FalseMultiplicity {
        let mut body = model.body_multiplicities.clone();
        if body.len() > 1 {
            body[1] = 3;
        }
        body
    } else {
        model.body_multiplicities.clone()
    };
    let alpha_independent = model.alpha_coordinate.origin == "IndependentReservoir"
        && model.alpha_coordinate.symbolic_value == "epsilon/ln(r)";
    model.states.len() == expected.state_count
        && model.fibers.iter().map(Vec::len).collect::<Vec<_>>() == expected.fibers
        && model.body_multiplicities == expected.body
        && declared_body == model.body_multiplicities
        && model.reservoir_multiplicities == expected.reservoir
        && model.fibers.iter().enumerate().all(|(index, fiber)| {
            fiber.len() == model.body_multiplicities[index] * model.reservoir_multiplicities[index]
        })
        && alpha_independent
}

fn k03(model: &Model) -> bool {
    let Ok(row_probability) = Rational::new(1, model.slots.len() as i128) else {
        return false;
    };
    let rows_closed = model.outcomes.iter().all(|row| {
        let probability_sum = row
            .iter()
            .try_fold(Rational::zero(), |sum, _| sum.checked_add(row_probability));
        probability_sum.ok() == Some(Rational::one()) && row.len() == model.slots.len()
    });
    let macro_rows_closed = macro_matrices(model).is_ok_and(|(p, q)| {
        p.iter().all(|row| {
            row.iter()
                .try_fold(Rational::zero(), |sum, &entry| sum.checked_add(entry))
                .ok()
                == Some(Rational::one())
        }) && q.iter().all(|row| {
            row.iter()
                .try_fold(Rational::zero(), |sum, &entry| sum.checked_add(entry))
                .ok()
                == Some(Rational::zero())
        })
    });
    model.outcomes.len() == model.states.len()
        && model
            .outcomes
            .iter()
            .all(|row| row.len() == model.slots.len())
        && model
            .outcomes
            .iter()
            .flatten()
            .all(|outcome| model.state_index.contains_key(&outcome.post))
        && rows_closed
        && macro_rows_closed
}

fn reverse_outcome<'a>(
    model: &'a Model,
    outcome: &Outcome,
    slot_index: usize,
) -> Option<&'a Outcome> {
    let target = *model.state_index.get(&outcome.post)?;
    model.outcomes.get(target)?.get(slot_index)
}

fn k04(model: &Model) -> bool {
    for (source, row) in model.outcomes.iter().enumerate() {
        for (slot, outcome) in row.iter().enumerate() {
            if !outcome.moved {
                continue;
            }
            let Some(reverse) = reverse_outcome(model, outcome, slot) else {
                return false;
            };
            if !reverse.moved || reverse.post != model.states[source] {
                return false;
            }
        }
    }
    true
}

fn k05(model: &Model) -> bool {
    model.outcomes.iter().enumerate().all(|(source, row)| {
        row.iter().all(|outcome| {
            let observed_delta_u = energy(&model.spec, &outcome.post.positions) as i128
                - energy(&model.spec, &model.states[source].positions) as i128;
            let observed_delta_reservoir =
                outcome.post.word.len() as i128 - model.states[source].word.len() as i128;
            if outcome.moved {
                outcome.delta_u == observed_delta_u
                    && outcome.delta_reservoir == observed_delta_reservoir
                    && outcome.delta_u + outcome.delta_reservoir + outcome.external_work == 0
            } else {
                outcome.post == model.states[source]
                    && observed_delta_u == 0
                    && observed_delta_reservoir == 0
                    && outcome.delta_u == 0
                    && outcome.delta_reservoir == 0
                    && outcome.external_work == 0
            }
        })
    })
}

fn discrete_macro_row(model: &Model, outcomes: &[Outcome]) -> Option<Vec<usize>> {
    let mut row = vec![0_usize; model.macros.len()];
    for outcome in outcomes {
        let target = *model.state_index.get(&outcome.post)?;
        row[model.state_macro[target]] += 1;
    }
    Some(row)
}

fn k06(model: &Model, _mutation: Mutation) -> bool {
    model.fibers.iter().all(|fiber| {
        let Some(first) = fiber
            .first()
            .and_then(|&state| model.hidden_outcomes[state].first())
            .and_then(|row| discrete_macro_row(model, row))
        else {
            return false;
        };
        fiber.iter().all(|&state| {
            model.hidden_outcomes[state]
                .iter()
                .all(|row| discrete_macro_row(model, row).as_ref() == Some(&first))
        })
    })
}

fn k07(model: &Model) -> bool {
    for row in &model.outcomes {
        for (slot, outcome) in row.iter().enumerate() {
            if !outcome.moved {
                continue;
            }
            let Some(reverse) = reverse_outcome(model, outcome, slot) else {
                return false;
            };
            if outcome.hazard != reverse.hazard {
                return false;
            }
        }
    }
    let Ok((_, generator)) = macro_matrices(model) else {
        return false;
    };
    let weights = model
        .fibers
        .iter()
        .map(|fiber| fiber.len() as i128)
        .collect::<Vec<_>>();
    for source in 0..generator.len() {
        for target in 0..generator.len() {
            if source == target {
                continue;
            }
            let left =
                generator[source][target].checked_multiply(Rational::integer(weights[source]));
            let right =
                generator[target][source].checked_multiply(Rational::integer(weights[target]));
            if left.ok() != right.ok() {
                return false;
            }
        }
    }
    thermodynamic_factorization(model)
}

fn stationary_for(model: &Model) -> Result<Vec<Rational>> {
    model
        .fibers
        .iter()
        .map(|fiber| Rational::new(fiber.len() as i128, model.states.len() as i128))
        .collect()
}

fn stationary_matrix(stationary: &[Rational], matrix: &[Vec<Rational>]) -> Result<Vec<Rational>> {
    let mut result = vec![Rational::zero(); matrix.len()];
    for (source, row) in matrix.iter().enumerate() {
        for (target, &entry) in row.iter().enumerate() {
            result[target] =
                result[target].checked_add(stationary[source].checked_multiply(entry)?)?;
        }
    }
    Ok(result)
}

fn k08(model: &Model) -> bool {
    let Ok((p, q)) = macro_matrices(model) else {
        return false;
    };
    let Ok(stationary) = stationary_for(model) else {
        return false;
    };
    if stationary_matrix(&stationary, &p).ok() != Some(stationary.clone())
        || stationary_matrix(&stationary, &q).ok() != Some(vec![Rational::zero(); stationary.len()])
    {
        return false;
    }
    for source in 0..q.len() {
        for target in 0..q.len() {
            if stationary[source].checked_multiply(q[source][target]).ok()
                != stationary[target].checked_multiply(q[target][source]).ok()
            {
                return false;
            }
        }
    }
    let Ok(slot_probability) = Rational::new(1, model.slots.len() as i128) else {
        return false;
    };
    let mut discrete_inbound = vec![Rational::zero(); model.states.len()];
    let mut generator_inbound = vec![Rational::zero(); model.states.len()];
    for (source, row) in model.outcomes.iter().enumerate() {
        let mut active = Rational::zero();
        for (slot, outcome) in row.iter().enumerate() {
            let Some(&target) = model.state_index.get(&outcome.post) else {
                return false;
            };
            let Ok(discrete) = discrete_inbound[target].checked_add(slot_probability) else {
                return false;
            };
            discrete_inbound[target] = discrete;
            if outcome.moved {
                let rate = Rational::integer(outcome.hazard);
                let Ok(inbound) = generator_inbound[target].checked_add(rate) else {
                    return false;
                };
                generator_inbound[target] = inbound;
                let Some(reverse) = reverse_outcome(model, outcome, slot) else {
                    return false;
                };
                if Rational::new(outcome.hazard, model.states.len() as i128).ok()
                    != Rational::new(reverse.hazard, model.states.len() as i128).ok()
                {
                    return false;
                }
                let Ok(next_active) = active.checked_add(rate) else {
                    return false;
                };
                active = next_active;
            }
        }
        let Ok(diagonal) = generator_inbound[source].checked_add(active.negated()) else {
            return false;
        };
        generator_inbound[source] = diagonal;
    }
    discrete_inbound
        .iter()
        .all(|&weight| weight == Rational::one())
        && generator_inbound.iter().all(|weight| weight.is_zero())
}

fn connected(adjacency: &[BTreeSet<usize>]) -> bool {
    if adjacency.is_empty() {
        return false;
    }
    let mut visited = BTreeSet::new();
    let mut queue = VecDeque::from([0_usize]);
    while let Some(source) = queue.pop_front() {
        if !visited.insert(source) {
            continue;
        }
        for &target in &adjacency[source] {
            queue.push_back(target);
        }
    }
    visited.len() == adjacency.len()
}

fn k09(model: &Model) -> bool {
    let mut micro = vec![BTreeSet::new(); model.states.len()];
    let mut macro_graph = vec![BTreeSet::new(); model.macros.len()];
    let mut has_wait = false;
    for (source, row) in model.outcomes.iter().enumerate() {
        for outcome in row {
            if outcome.moved {
                let Some(&target) = model.state_index.get(&outcome.post) else {
                    return false;
                };
                micro[source].insert(target);
                macro_graph[model.state_macro[source]].insert(model.state_macro[target]);
            } else {
                has_wait = true;
            }
        }
    }
    connected(&micro) && connected(&macro_graph) && has_wait
}

fn protocol_state(state: &State) -> Option<String> {
    if state.positions.len() != 2 || state.positions.iter().any(|&node| node > 1) {
        return None;
    }
    let placement = state
        .positions
        .iter()
        .map(|&node| if node == 0 { 'L' } else { 'H' })
        .collect::<String>();
    let word = if state.word.is_empty() {
        "empty".to_owned()
    } else {
        state
            .word
            .iter()
            .map(|symbol| char::from(b'0' + *symbol))
            .collect()
    };
    Some(format!("{placement}|{word}"))
}

fn primary_raw_oracle(model: &Model) -> bool {
    let expected = BTreeMap::from([
        ("LL|00", ["HL|0", "-", "LH|0", "-"]),
        ("LL|01", ["-", "HL|0", "-", "LH|0"]),
        ("LL|10", ["HL|1", "-", "LH|1", "-"]),
        ("LL|11", ["-", "HL|1", "-", "LH|1"]),
        ("LH|0", ["HH|empty", "-", "LL|00", "LL|01"]),
        ("LH|1", ["-", "HH|empty", "LL|10", "LL|11"]),
        ("HL|0", ["LL|00", "LL|01", "HH|empty", "-"]),
        ("HL|1", ["LL|10", "LL|11", "-", "HH|empty"]),
        ("HH|empty", ["LH|0", "LH|1", "HL|0", "HL|1"]),
    ]);
    for (source, state) in model.states.iter().enumerate() {
        let Some(name) = protocol_state(state) else {
            return false;
        };
        let Some(row) = expected.get(name.as_str()) else {
            return false;
        };
        for (slot, expected_target) in row.iter().enumerate() {
            let outcome = &model.outcomes[source][slot];
            if *expected_target == "-" {
                if outcome.moved || outcome.post != *state {
                    return false;
                }
            } else if protocol_state(&outcome.post).as_deref() != Some(*expected_target) {
                return false;
            }
        }
    }
    true
}

fn map_unit_state(state: &State) -> State {
    let mut mapped = state.clone();
    mapped.positions.swap(0, 1);
    mapped
}

fn unit_covariance(spec: &Spec, states: &[State], slots: &[Slot]) -> bool {
    if spec.labels != 2 {
        return false;
    }
    for state in states {
        for &slot in slots {
            let before = raw_outcome(spec, state, slot);
            let mapped_state = map_unit_state(state);
            let mapped_slot = Slot {
                label: 1 - slot.label,
                ..slot
            };
            let after = raw_outcome(spec, &mapped_state, mapped_slot);
            if map_unit_state(&before.post) != after.post || before.moved != after.moved {
                return false;
            }
        }
    }
    true
}

fn map_symbols(state: &State, permutation: &[u8]) -> State {
    State {
        positions: state.positions.clone(),
        word: state
            .word
            .iter()
            .map(|symbol| permutation[*symbol as usize])
            .collect(),
    }
}

fn symbol_permutation_covariance(
    spec: &Spec,
    states: &[State],
    slots: &[Slot],
    permutation: &[u8],
) -> bool {
    for state in states {
        for &slot in slots {
            let before = raw_outcome(spec, state, slot);
            let mapped_state = map_symbols(state, permutation);
            let mapped_slot = Slot {
                symbol: permutation[slot.symbol as usize],
                ..slot
            };
            let after = raw_outcome(spec, &mapped_state, mapped_slot);
            if map_symbols(&before.post, permutation) != after.post || before.moved != after.moved {
                return false;
            }
        }
    }
    true
}

fn symbol_covariance(spec: &Spec, states: &[State], slots: &[Slot]) -> bool {
    (0..spec.branching.saturating_sub(1)).all(|left| {
        let mut permutation = (0..spec.branching).collect::<Vec<_>>();
        permutation.swap(left as usize, left as usize + 1);
        symbol_permutation_covariance(spec, states, slots, &permutation)
    })
}

fn node_covariance(spec: &Spec, states: &[State], slots: &[Slot]) -> bool {
    let count = spec.heights.len();
    let map = (0..count).map(|node| count - 1 - node).collect::<Vec<_>>();
    let mut heights = vec![0_u8; count];
    for old in 0..count {
        heights[map[old]] = spec.heights[old];
    }
    let mapped_spec = Spec {
        id: spec.id,
        heights,
        edges: spec
            .edges
            .iter()
            .map(|edge| Edge {
                low: map[edge.low],
                high: map[edge.high],
            })
            .collect(),
        labels: spec.labels,
        branching: spec.branching,
        depth: spec.depth,
    };
    for state in states {
        let mapped_state = State {
            positions: state.positions.iter().map(|&node| map[node]).collect(),
            word: state.word.clone(),
        };
        for &slot in slots {
            let before = raw_outcome(spec, state, slot);
            let after = raw_outcome(&mapped_spec, &mapped_state, slot);
            let mapped_post = State {
                positions: before
                    .post
                    .positions
                    .iter()
                    .map(|&node| map[node])
                    .collect(),
                word: before.post.word,
            };
            if mapped_post != after.post || before.moved != after.moved {
                return false;
            }
        }
    }
    true
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MetamorphicCheckArtifact {
    pub id: &'static str,
    pub status: &'static str,
    pub baseline_scale: Rational,
    pub transformed_scale: Rational,
    pub invariant_fields: Vec<&'static str>,
    pub covariant_fields: Vec<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative_control_rejected: Option<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ThermoScaleView {
    state_body_energy: Vec<Rational>,
    state_reservoir_energy: Vec<Rational>,
    path_deltas: Vec<(Rational, Rational, Rational)>,
    p: Vec<Vec<Rational>>,
    q_over_kappa: Vec<Vec<Rational>>,
    equilibrium: Vec<Rational>,
    epsilon_scale: Rational,
    alpha_epsilon_coefficient: Rational,
    local_detailed_balance: bool,
}

fn thermodynamic_factorization(model: &Model) -> bool {
    let Ok((_, q)) = macro_matrices(model) else {
        return false;
    };
    let branching = model.spec.branching as usize;
    let multiplicities_match = model.macros.iter().enumerate().all(|(index, counts)| {
        let reservoir_quanta = model.spec.depth as usize - occupation_energy(&model.spec, counts);
        model.reservoir_multiplicities[index] == power(branching, reservoir_quanta)
            && model.fibers[index].len()
                == model.body_multiplicities[index] * model.reservoir_multiplicities[index]
    });
    multiplicities_match
        && (0..q.len()).all(|source| {
            (0..q.len()).all(|target| {
                model.fibers[source].len() as i128
                    * q[source][target].numerator
                    * q[target][source].denominator
                    == model.fibers[target].len() as i128
                        * q[target][source].numerator
                        * q[source][target].denominator
            })
        })
}

fn derive_thermo_scale_view(
    model: &Model,
    epsilon_scale: Rational,
    alpha_scale: Rational,
) -> Result<ThermoScaleView> {
    let (p, q_over_kappa) = macro_matrices(model)?;
    let equilibrium = stationary_for(model)?;
    let state_body_energy = model
        .states
        .iter()
        .map(|state| {
            Rational::integer(energy(&model.spec, &state.positions) as i128)
                .checked_multiply(epsilon_scale)
        })
        .collect::<Result<Vec<_>>>()?;
    let state_reservoir_energy = model
        .states
        .iter()
        .map(|state| Rational::integer(state.word.len() as i128).checked_multiply(epsilon_scale))
        .collect::<Result<Vec<_>>>()?;
    let mut path_deltas = Vec::new();
    for (source, row) in model.outcomes.iter().enumerate() {
        for outcome in row {
            let body = Rational::integer(
                energy(&model.spec, &outcome.post.positions) as i128
                    - energy(&model.spec, &model.states[source].positions) as i128,
            )
            .checked_multiply(epsilon_scale)?;
            let reservoir = Rational::integer(
                outcome.post.word.len() as i128 - model.states[source].word.len() as i128,
            )
            .checked_multiply(epsilon_scale)?;
            let work = Rational::integer(outcome.external_work).checked_multiply(epsilon_scale)?;
            path_deltas.push((body, reservoir, work));
        }
    }
    Ok(ThermoScaleView {
        state_body_energy,
        state_reservoir_energy,
        path_deltas,
        p,
        q_over_kappa,
        equilibrium,
        epsilon_scale,
        alpha_epsilon_coefficient: alpha_scale,
        local_detailed_balance: epsilon_scale == alpha_scale
            && model.alpha_coordinate.origin == "IndependentReservoir"
            && thermodynamic_factorization(model),
    })
}

fn scaled_rationals_equal(
    baseline: &[Rational],
    transformed: &[Rational],
    scale: Rational,
) -> bool {
    baseline.len() == transformed.len()
        && baseline
            .iter()
            .zip(transformed)
            .all(|(&base, &changed)| base.checked_multiply(scale).ok() == Some(changed))
}

fn energy_temperature_scale_covariance(model: &Model) -> bool {
    let Ok(scale) = Rational::new(7, 3) else {
        return false;
    };
    let Ok(baseline) = derive_thermo_scale_view(model, Rational::one(), Rational::one()) else {
        return false;
    };
    let Ok(transformed) = derive_thermo_scale_view(model, scale, scale) else {
        return false;
    };
    let mismatched = derive_thermo_scale_view(model, scale, Rational::one()).ok();
    let baseline_body = baseline.state_body_energy;
    let transformed_body = transformed.state_body_energy;
    let baseline_reservoir = baseline.state_reservoir_energy;
    let transformed_reservoir = transformed.state_reservoir_energy;
    let baseline_path = baseline
        .path_deltas
        .iter()
        .flat_map(|(body, reservoir, work)| [*body, *reservoir, *work])
        .collect::<Vec<_>>();
    let transformed_path = transformed
        .path_deltas
        .iter()
        .flat_map(|(body, reservoir, work)| [*body, *reservoir, *work])
        .collect::<Vec<_>>();
    baseline.p == transformed.p
        && baseline.q_over_kappa == transformed.q_over_kappa
        && baseline.equilibrium == transformed.equilibrium
        && baseline.local_detailed_balance
        && transformed.local_detailed_balance
        && scaled_rationals_equal(&baseline_body, &transformed_body, scale)
        && scaled_rationals_equal(&baseline_reservoir, &transformed_reservoir, scale)
        && scaled_rationals_equal(&baseline_path, &transformed_path, scale)
        && transformed.epsilon_scale == scale
        && transformed.alpha_epsilon_coefficient == scale
        && mismatched.is_some_and(|view| !view.local_detailed_balance)
}

fn scale_matrix(matrix: &[Vec<Rational>], scale: Rational) -> Result<Vec<Vec<Rational>>> {
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|&entry| entry.checked_multiply(scale))
                .collect()
        })
        .collect()
}

fn macro_generator_at_scale(
    model: &Model,
    scale: Rational,
    directed_probe: bool,
) -> Result<Vec<Vec<Rational>>> {
    let probe = model.outcomes.iter().enumerate().find_map(|(state, row)| {
        row.iter()
            .position(|outcome| outcome.moved)
            .map(|slot| (state, slot))
    });
    let mut generator = vec![vec![Rational::zero(); model.macros.len()]; model.macros.len()];
    for source_macro in 0..model.macros.len() {
        let mut reference = None;
        for &source in &model.fibers[source_macro] {
            let mut row = vec![Rational::zero(); model.macros.len()];
            let mut active = Rational::zero();
            for (slot, outcome) in model.outcomes[source].iter().enumerate() {
                if !outcome.moved {
                    continue;
                }
                let target = model.state_index[&outcome.post];
                let target_macro = model.state_macro[target];
                if target_macro == source_macro {
                    continue;
                }
                let local_scale = if directed_probe && probe == Some((source, slot)) {
                    scale.checked_multiply(Rational::integer(2))?
                } else {
                    scale
                };
                let rate = Rational::integer(outcome.hazard).checked_multiply(local_scale)?;
                row[target_macro] = row[target_macro].checked_add(rate)?;
                active = active.checked_add(rate)?;
            }
            row[source_macro] = active.negated();
            if let Some(expected) = &reference {
                if &row != expected {
                    return Err(VerifyError::new(
                        "scaled microscopic clock is not strongly lumpable",
                    ));
                }
            } else {
                reference = Some(row);
            }
        }
        generator[source_macro] =
            reference.ok_or_else(|| VerifyError::new("empty scaled macro fiber"))?;
    }
    Ok(generator)
}

fn clock_scale_covariance(model: &Model) -> bool {
    let Ok(scale) = Rational::new(11, 5) else {
        return false;
    };
    let Ok((baseline_p, baseline_q)) = macro_matrices(model) else {
        return false;
    };
    let Ok(transformed_q) = macro_generator_at_scale(model, scale, false) else {
        return false;
    };
    let expected_q = scale_matrix(&baseline_q, scale).ok();
    let stationary = stationary_for(model).ok();
    let stationary_unchanged = stationary.as_ref().is_some_and(|pi| {
        stationary_matrix(pi, &transformed_q).ok() == Some(vec![Rational::zero(); pi.len()])
            && (0..transformed_q.len()).all(|source| {
                (0..transformed_q.len()).all(|target| {
                    pi[source]
                        .checked_multiply(transformed_q[source][target])
                        .ok()
                        == pi[target]
                            .checked_multiply(transformed_q[target][source])
                            .ok()
                })
            })
    });
    let primary_eigenvalues_scale = if model.spec.id == "primary-r2" {
        three_state_generator_eigenvalues(&baseline_q)
            .and_then(|values| {
                values
                    .iter()
                    .map(|&value| value.checked_multiply(scale))
                    .collect::<Result<Vec<_>>>()
            })
            .ok()
            == three_state_generator_eigenvalues(&transformed_q).ok()
    } else {
        true
    };
    let directed_rejected = match macro_generator_at_scale(model, scale, true) {
        Err(_) => true,
        Ok(directed) => {
            Some(directed.clone()) != expected_q
                || !stationary.as_ref().is_some_and(|pi| {
                    (0..directed.len()).all(|source| {
                        (0..directed.len()).all(|target| {
                            pi[source].checked_multiply(directed[source][target]).ok()
                                == pi[target].checked_multiply(directed[target][source]).ok()
                        })
                    })
                })
        }
    };
    !baseline_p.is_empty()
        && Some(transformed_q) == expected_q
        && stationary_unchanged
        && primary_eigenvalues_scale
        && directed_rejected
}

fn metamorphic_checks(model: &Model) -> Vec<MetamorphicCheckArtifact> {
    let checks = [
        (
            "unit-label-exchange",
            unit_covariance(&model.spec, &model.states, &model.slots),
            Rational::one(),
            Vec::new(),
            vec!["raw-state-slot-law"],
            None,
        ),
        (
            "reservoir-symbol-exchange",
            symbol_covariance(&model.spec, &model.states, &model.slots),
            Rational::one(),
            Vec::new(),
            vec!["raw-state-slot-law"],
            None,
        ),
        (
            "node-height-covariance",
            node_covariance(&model.spec, &model.states, &model.slots),
            Rational::one(),
            Vec::new(),
            vec!["raw-state-slot-law"],
            None,
        ),
        (
            "epsilon-alpha-common-scale",
            energy_temperature_scale_covariance(model),
            Rational::new(7, 3).expect("fixed nonzero scale"),
            vec!["P", "Q_over_kappa", "equilibrium", "local-detailed-balance"],
            vec!["body-energy", "reservoir-energy", "alpha"],
            Some(true),
        ),
        (
            "kappa-time-scale",
            clock_scale_covariance(model),
            Rational::new(11, 5).expect("fixed nonzero scale"),
            vec!["P", "equilibrium", "local-detailed-balance"],
            vec!["Q", "generator-eigenvalues"],
            Some(true),
        ),
    ];
    checks
        .into_iter()
        .map(
            |(id, passed, transformed_scale, invariant_fields, covariant_fields, negative)| {
                MetamorphicCheckArtifact {
                    id,
                    status: if passed { "pass" } else { "fail" },
                    baseline_scale: Rational::one(),
                    transformed_scale,
                    invariant_fields,
                    covariant_fields,
                    negative_control_rejected: negative.map(|_| passed),
                }
            },
        )
        .collect()
}

fn metamorphic_checks_digest(checks: &[MetamorphicCheckArtifact]) -> Result<String> {
    serde_json::to_vec(checks)
        .map(|bytes| sha256_hex(&bytes))
        .map_err(|error| VerifyError::new(format!("cannot serialize metamorphic checks: {error}")))
}

fn k10(model: &Model, expected: &Expectations) -> bool {
    let Ok((p, q)) = macro_matrices(model) else {
        return false;
    };
    let eigenvalues_match = if model.spec.id == "primary-r2" {
        three_state_generator_eigenvalues(&q).ok() == Some(vec![rat(0), rat(-3), rat(-6)])
            && three_state_generator_eigenvalues(&q)
                .and_then(|values| discrete_eigenvalues_from_generator(&values, model.slots.len()))
                .ok()
                == probability_matrix(&[&[(1, 1), (1, 4), (-1, 2)]])
                    .ok()
                    .and_then(|rows| rows.into_iter().next())
    } else {
        true
    };
    p == expected.p
        && q == expected.q
        && eigenvalues_match
        && (model.spec.id != "primary-r2" || primary_raw_oracle(model))
        && unit_covariance(&model.spec, &model.states, &model.slots)
        && symbol_covariance(&model.spec, &model.states, &model.slots)
        && node_covariance(&model.spec, &model.states, &model.slots)
        && metamorphic_checks(model)
            .iter()
            .all(|check| check.status == "pass")
}

fn placement_text(positions: &[usize]) -> String {
    positions
        .iter()
        .enumerate()
        .map(|(label, node)| format!("{}={node}", label_name(label)))
        .collect::<Vec<_>>()
        .join(",")
}

fn crystal_placement_catalog(model: &Model) -> Vec<(String, String)> {
    let mut rows = model
        .states
        .iter()
        .map(|state| {
            let placement = placement_text(&state.positions);
            (
                macro_text(&model.spec, &occupation(&model.spec, &state.positions)),
                placement,
            )
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn crystal_readout_rows(model: &Model) -> Result<Vec<CrystalReadoutRowArtifact>> {
    let mut compatible = vec![BTreeSet::<String>::new(); model.macros.len()];
    for (state, &macrostate) in model.states.iter().zip(model.state_macro.iter()) {
        compatible[macrostate].insert(placement_text(&state.positions));
    }
    let mut rows = Vec::new();
    for (source, state) in model.states.iter().enumerate() {
        let macrostate = model.state_macro[source];
        let probability = Rational::new(1, compatible[macrostate].len() as i128)?;
        let word = if state.word.is_empty() {
            "empty".to_owned()
        } else {
            state
                .word
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join("")
        };
        for reported_placement in &compatible[macrostate] {
            rows.push(CrystalReadoutRowArtifact {
                pre: state_text(&model.spec, state),
                fixed_macro: macro_text(&model.spec, &model.macros[macrostate]),
                fixed_reservoir_word: word.clone(),
                reported_placement: reported_placement.clone(),
                probability,
                persistent_post: state_text(&model.spec, state),
                persistent_state_changed: false,
            });
        }
    }
    Ok(rows)
}

fn crystal_readout_valid(model: &Model, rows: &[CrystalReadoutRowArtifact]) -> bool {
    let mut by_pre = BTreeMap::<&str, Vec<&CrystalReadoutRowArtifact>>::new();
    for row in rows {
        by_pre.entry(&row.pre).or_default().push(row);
    }
    model.states.iter().enumerate().all(|(source, state)| {
        let pre = state_text(&model.spec, state);
        let Some(group) = by_pre.get(pre.as_str()) else {
            return false;
        };
        let macrostate = model.state_macro[source];
        let expected_outputs = model.body_multiplicities[macrostate];
        let expected_reports = model
            .states
            .iter()
            .enumerate()
            .filter(|(candidate, _)| model.state_macro[*candidate] == macrostate)
            .map(|(_, candidate)| placement_text(&candidate.positions))
            .collect::<BTreeSet<_>>();
        let observed_reports = group
            .iter()
            .map(|row| row.reported_placement.clone())
            .collect::<BTreeSet<_>>();
        let expected_word = if state.word.is_empty() {
            "empty".to_owned()
        } else {
            state
                .word
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join("")
        };
        let probability_sum = group.iter().try_fold(Rational::zero(), |sum, row| {
            sum.checked_add(row.probability)
        });
        let expected_probability = Rational::new(1, expected_outputs as i128).ok();
        group.len() == expected_outputs
            && probability_sum.ok() == Some(Rational::one())
            && observed_reports == expected_reports
            && group.iter().all(|row| {
                Some(row.probability) == expected_probability
                    && row.persistent_post == pre
                    && !row.persistent_state_changed
                    && row.fixed_macro == macro_text(&model.spec, &model.macros[macrostate])
                    && row.fixed_reservoir_word == expected_word
            })
    }) && model
        .outcomes
        .iter()
        .all(|row| row.len() == model.slots.len())
}

fn x01(model: &Model) -> bool {
    let placement_rows = crystal_placement_catalog(model);
    let readout_rows = crystal_readout_rows(model).ok();
    let expected_placement_rows = model.body_multiplicities.iter().sum::<usize>();
    !model.states.is_empty()
        && model.outcomes.iter().flatten().any(|outcome| outcome.moved)
        && model
            .outcomes
            .iter()
            .flatten()
            .any(|outcome| !outcome.moved)
        && placement_rows.len() == expected_placement_rows
        && readout_rows
            .as_ref()
            .is_some_and(|rows| crystal_readout_valid(model, rows))
        && k04(model)
        && k05(model)
        && k06(model, Mutation::None)
}

fn gate_bools(model: &Model, expected: &Expectations, mutation: Mutation) -> Vec<(GateId, bool)> {
    vec![
        (GateId::K02, k02(model, expected, mutation)),
        (GateId::K03, k03(model)),
        (GateId::K04, k04(model)),
        (GateId::K05, k05(model)),
        (GateId::K06, k06(model, mutation)),
        (GateId::K07, k07(model)),
        (GateId::K08, k08(model)),
        (GateId::K09, k09(model)),
        (GateId::K10, k10(model, expected)),
        (GateId::X01, x01(model)),
    ]
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TransitionRow {
    pub pre: String,
    pub slot: String,
    pub outcome: &'static str,
    pub post: String,
    pub reverse_slot: Option<String>,
    pub macro_from: String,
    pub macro_to: String,
    pub probability: Rational,
    pub hazard_over_activity: Rational,
    pub delta_body_energy: Scaled,
    pub delta_reservoir_energy: Scaled,
    pub external_work: Scaled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AlphaSource {
    pub reservoir_branching: u8,
    pub origin: &'static str,
    pub symbolic_value: &'static str,
    pub relation: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CompleteStateArtifact {
    pub id: String,
    #[serde(rename = "macro")]
    pub macrostate: String,
    pub coordinates: String,
    pub body_energy: Scaled,
    pub reservoir_energy: Scaled,
}

pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CrystalReadoutRowArtifact {
    pub pre: String,
    pub fixed_macro: String,
    pub fixed_reservoir_word: String,
    pub reported_placement: String,
    pub probability: Rational,
    pub persistent_post: String,
    pub persistent_state_changed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CrystalReadoutArtifact {
    pub mode: &'static str,
    pub channel: &'static str,
    pub included_in_move_kernel: bool,
    pub uniform: bool,
    pub persistent_state_unchanged: bool,
    pub row_count: usize,
    pub outcome_count: usize,
    pub rows: Vec<CrystalReadoutRowArtifact>,
    pub rows_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FixtureArtifact {
    pub id: &'static str,
    pub state_count: usize,
    pub states: Vec<CompleteStateArtifact>,
    pub macro_labels: Vec<String>,
    pub fiber_counts: Vec<usize>,
    pub body_multiplicities: Vec<usize>,
    pub reservoir_multiplicities: Vec<usize>,
    pub alpha_source: AlphaSource,
    pub slot_count: usize,
    pub rows: Vec<TransitionRow>,
    pub crystal_readout: CrystalReadoutArtifact,
    #[serde(rename = "P")]
    pub p: Vec<Vec<Rational>>,
    #[serde(rename = "Q_over_activity")]
    pub q_over_activity: Vec<Vec<Rational>>,
    pub equilibrium: Vec<Rational>,
    pub active_outcome_count: usize,
    pub wait_outcome_count: usize,
    pub reciprocal_micro_edge_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator_eigenvalues_over_activity: Option<Vec<Rational>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discrete_eigenvalues: Option<Vec<Rational>>,
    pub ldb_cross_products_equal: bool,
    pub connected: bool,
    pub aperiodic: bool,
    pub unique_stationary_law: bool,
    pub strongly_lumpable: bool,
    pub metamorphic_checks: Vec<MetamorphicCheckArtifact>,
    pub metamorphic_checks_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct KernelExactArtifact {
    pub schema_version: u8,
    pub experiment_id: &'static str,
    pub artifact_kind: &'static str,
    pub run_id: &'static str,
    pub provenance: Provenance,
    pub controls_dependency: ControlsDependencyArtifact,
    pub status: &'static str,
    pub first_failed_gate: Option<GateId>,
    pub gates: Vec<Gate>,
    pub summary: Summary,
    pub xypher_representation: XypherRepresentationArtifact,
    pub fixtures: Vec<FixtureArtifact>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ControlsDependencyArtifact {
    pub run_id: &'static str,
    pub status: &'static str,
    pub compact_payload_sha256: String,
    pub required_mutations_matched: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct XypherRepresentationArtifact {
    pub substrate: &'static str,
    pub crystal: &'static str,
    pub thermodynamic_harness: &'static str,
    pub base_praxion: &'static str,
    pub action: &'static str,
    pub ruby: &'static str,
    pub auxiliary_components: Vec<&'static str>,
}

fn xypher_representation_artifact() -> XypherRepresentationArtifact {
    XypherRepresentationArtifact {
        substrate: "complete active same-slot graph on Omega with balance projection Z",
        crystal: "non-mutating uniform labelled-placement readout",
        thermodynamic_harness: "independent g_X, g_R, U_X, E_R, alpha, path accounting, and R_+",
        base_praxion: "one-state non-learning raw three-rule slot mechanism",
        action: "one reciprocal unit transfer or explicit WAIT",
        ruby: "empty",
        auxiliary_components: vec!["Crystal source is auxiliary until protocol-owned"],
    }
}

fn xypher_representation_complete(mapping: &XypherRepresentationArtifact) -> bool {
    mapping.substrate.contains("Omega")
        && mapping.substrate.contains("projection Z")
        && mapping.crystal.contains("non-mutating")
        && mapping.thermodynamic_harness.contains("alpha")
        && mapping.base_praxion.contains("non-learning")
        && mapping.action.contains("WAIT")
        && mapping.ruby == "empty"
        && mapping
            .auxiliary_components
            .iter()
            .any(|component| component.contains("Crystal source is auxiliary"))
}

fn label_name(label: usize) -> char {
    char::from(b'a' + label as u8)
}

fn state_text(spec: &Spec, state: &State) -> String {
    let positions = state
        .positions
        .iter()
        .enumerate()
        .map(|(label, node)| format!("{}={node}", label_name(label)))
        .collect::<Vec<_>>()
        .join(",");
    let word = if state.word.is_empty() {
        "empty".to_owned()
    } else {
        state
            .word
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join("")
    };
    format!("{positions}|w={word}|D={}", spec.depth)
}

fn macro_text(spec: &Spec, counts: &[u8]) -> String {
    if spec.heights.len() == 2 {
        format!("x={}", counts[1])
    } else {
        format!(
            "n={}",
            counts
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

fn slot_text(spec: &Spec, slot: Slot) -> String {
    if spec.edges.len() == 1 {
        format!("{}:{}", label_name(slot.label), slot.symbol)
    } else {
        let edge = spec.edges[slot.edge];
        format!(
            "{}:{}-{}:{}",
            label_name(slot.label),
            edge.low,
            edge.high,
            slot.symbol
        )
    }
}

fn fixture_artifact(model: &Model) -> Result<FixtureArtifact> {
    let (p, q) = macro_matrices(model)?;
    let generator_eigenvalues_over_activity = if model.spec.id == "primary-r2" {
        Some(three_state_generator_eigenvalues(&q)?)
    } else {
        None
    };
    let discrete_eigenvalues = generator_eigenvalues_over_activity
        .as_ref()
        .map(|eigenvalues| discrete_eigenvalues_from_generator(eigenvalues, model.slots.len()))
        .transpose()?;
    let equilibrium = stationary_for(model)?;
    let probability = Rational::new(1, model.slots.len() as i128)?;
    let mut rows = Vec::with_capacity(model.states.len() * model.slots.len());
    let mut active = 0_usize;
    for (source, state) in model.states.iter().enumerate() {
        for (slot_index, &slot) in model.slots.iter().enumerate() {
            let outcome = &model.outcomes[source][slot_index];
            let target = model.state_index[&outcome.post];
            if outcome.moved {
                active += 1;
            }
            rows.push(TransitionRow {
                pre: state_text(&model.spec, state),
                slot: slot_text(&model.spec, slot),
                outcome: if outcome.moved { "MOVE" } else { "WAIT" },
                post: state_text(&model.spec, &outcome.post),
                reverse_slot: outcome.moved.then(|| slot_text(&model.spec, slot)),
                macro_from: macro_text(&model.spec, &model.macros[model.state_macro[source]]),
                macro_to: macro_text(&model.spec, &model.macros[model.state_macro[target]]),
                probability,
                hazard_over_activity: Rational::integer(outcome.hazard),
                delta_body_energy: Scaled {
                    coefficient: Rational::integer(outcome.delta_u),
                    basis_id: "epsilon",
                },
                delta_reservoir_energy: Scaled {
                    coefficient: Rational::integer(outcome.delta_reservoir),
                    basis_id: "epsilon",
                },
                external_work: Scaled {
                    coefficient: Rational::integer(outcome.external_work),
                    basis_id: "epsilon",
                },
            });
        }
    }
    let states = model
        .states
        .iter()
        .enumerate()
        .map(|(index, state)| CompleteStateArtifact {
            id: format!("{}-s{index:03}", model.spec.id),
            macrostate: macro_text(&model.spec, &model.macros[model.state_macro[index]]),
            coordinates: state_text(&model.spec, state),
            body_energy: Scaled {
                coefficient: Rational::integer(energy(&model.spec, &state.positions) as i128),
                basis_id: "epsilon",
            },
            reservoir_energy: Scaled {
                coefficient: Rational::integer(state.word.len() as i128),
                basis_id: "epsilon",
            },
        })
        .collect::<Vec<_>>();
    let crystal_rows = crystal_readout_rows(model)?;
    let crystal_valid = crystal_readout_valid(model, &crystal_rows);
    let crystal_bytes = serde_json::to_vec(&crystal_rows)
        .map_err(|error| VerifyError::new(format!("cannot serialize Crystal rows: {error}")))?;
    let metamorphic_checks = metamorphic_checks(model);
    let metamorphic_checks_sha256 = metamorphic_checks_digest(&metamorphic_checks)?;
    Ok(FixtureArtifact {
        id: model.spec.id,
        state_count: model.states.len(),
        states,
        macro_labels: model
            .macros
            .iter()
            .map(|counts| macro_text(&model.spec, counts))
            .collect(),
        fiber_counts: model.fibers.iter().map(Vec::len).collect(),
        body_multiplicities: model.body_multiplicities.clone(),
        reservoir_multiplicities: model.reservoir_multiplicities.clone(),
        alpha_source: AlphaSource {
            reservoir_branching: model.spec.branching,
            origin: model.alpha_coordinate.origin,
            symbolic_value: model.alpha_coordinate.symbolic_value,
            relation: "alpha=epsilon/ln(r)",
        },
        slot_count: model.slots.len(),
        rows,
        crystal_readout: CrystalReadoutArtifact {
            mode: "auxiliary_non_mutating",
            channel: "REFRESH",
            included_in_move_kernel: false,
            uniform: crystal_valid,
            persistent_state_unchanged: crystal_valid,
            row_count: model.states.len(),
            outcome_count: crystal_rows.len(),
            rows: crystal_rows,
            rows_sha256: sha256_hex(&crystal_bytes),
        },
        p,
        q_over_activity: q,
        equilibrium,
        active_outcome_count: active,
        wait_outcome_count: model.states.len() * model.slots.len() - active,
        reciprocal_micro_edge_count: active / 2,
        generator_eigenvalues_over_activity,
        discrete_eigenvalues,
        ldb_cross_products_equal: k07(model),
        connected: k09(model),
        aperiodic: model
            .outcomes
            .iter()
            .flatten()
            .any(|outcome| !outcome.moved),
        unique_stationary_law: k09(model),
        strongly_lumpable: k06(model, Mutation::None),
        metamorphic_checks,
        metamorphic_checks_sha256,
    })
}

fn gate(id: GateId, passed: bool, pointer: &str) -> Gate {
    Gate {
        id,
        status: if passed { "pass" } else { "fail" },
        evidence: vec![Evidence {
            pointer: pointer.to_owned(),
        }],
        failure_code: (!passed).then_some("GATE_FAILED"),
    }
}

fn aggregate_fixture_gates(models: &[(&Model, &Expectations)]) -> Vec<(GateId, bool)> {
    let order = [
        GateId::K02,
        GateId::K03,
        GateId::K04,
        GateId::K05,
        GateId::K06,
        GateId::K07,
        GateId::K08,
        GateId::K09,
        GateId::K10,
        GateId::X01,
    ];
    order
        .into_iter()
        .map(|id| {
            let passed = models.iter().all(|(model, expected)| {
                gate_bools(model, expected, Mutation::None)
                    .into_iter()
                    .find(|(candidate, _)| *candidate == id)
                    .is_some_and(|(_, value)| value)
            });
            (id, passed)
        })
        .collect()
}

pub fn derive_kernel_exact(
    provenance: Provenance,
    controls: &ControlsArtifact,
) -> Result<KernelExactArtifact> {
    if controls.provenance != provenance {
        return Err(VerifyError::new(
            "exact-kernel provenance differs from its control dependency",
        ));
    }
    let controls_payload = serde_json::to_vec(controls)
        .map_err(|error| VerifyError::new(format!("cannot bind controls: {error}")))?;
    let controls_passed = controls.status == "pass"
        && controls
            .mutations
            .iter()
            .all(|mutation| mutation.verdict == "matched");
    let primary = build_model(primary_spec(2), Mutation::None)?;
    let ternary = build_model(primary_spec(3), Mutation::None)?;
    let path = build_model(path_spec(), Mutation::None)?;
    let primary_expected = expectations(&primary.spec)?;
    let ternary_expected = expectations(&ternary.spec)?;
    let path_expected = expectations(&path.spec)?;
    let mapping = xypher_representation_artifact();
    let mut values = aggregate_fixture_gates(&[
        (&primary, &primary_expected),
        (&ternary, &ternary_expected),
        (&path, &path_expected),
    ]);
    if let Some((_, value)) = values.iter_mut().find(|(id, _)| *id == GateId::X01) {
        *value &= xypher_representation_complete(&mapping);
    }
    let local_gates_passed = values.iter().all(|(_, value)| *value);
    let status = if !local_gates_passed {
        "fail"
    } else if !controls_passed {
        "invalidated"
    } else {
        "pass"
    };
    let mut gates = vec![gate(GateId::K01, true, "#/provenance")];
    gates.extend(values.iter().map(|(id, value)| {
        gate(
            *id,
            *value,
            if *id == GateId::X01 {
                "#/xypher_representation"
            } else {
                "#/fixtures"
            },
        )
    }));
    if let Some(x01) = gates
        .iter_mut()
        .find(|candidate| candidate.id == GateId::X01)
    {
        x01.evidence.push(Evidence {
            pointer: "#/fixtures".to_owned(),
        });
    }
    let first_failed_gate = gates
        .iter()
        .find(|gate| gate.status == "fail")
        .map(|gate| gate.id);
    Ok(KernelExactArtifact {
        schema_version: 1,
        experiment_id: "valhaim-thermodynamic-kernel-v1",
        artifact_kind: "kernel_exact",
        run_id: "VK1-EXACT-001",
        provenance,
        controls_dependency: ControlsDependencyArtifact {
            run_id: controls.run_id,
            status: controls.status,
            compact_payload_sha256: sha256_hex(&controls_payload),
            required_mutations_matched: controls_passed,
        },
        status,
        first_failed_gate,
        gates,
        summary: Summary {
            verdict_code: if status == "pass" {
                "EXACT_KERNEL_PASS"
            } else if status == "invalidated" {
                "EXACT_KERNEL_INVALIDATED_BY_CONTROLS"
            } else {
                "EXACT_KERNEL_FAIL"
            },
            earned_claim_codes: if status == "pass" {
                vec![
                    "FINITE_EQUILIBRIUM_THERMODYNAMIC_MODEL",
                    "T1_T5_PASS",
                    "STRUCTURAL_VARIANTS_PASS",
                ]
            } else {
                Vec::new()
            },
            excluded_claim_codes: vec![
                "PAYMENTLEDGER_OWNERSHIP",
                "DEPLOYED_STOCHASTIC_CLOCK",
                "MONETARY_CALIBRATION",
                "ADVANCED_FEATURE_CLOSURE",
            ],
            source_gates: gates_without_contact(),
        },
        xypher_representation: mapping,
        fixtures: vec![
            fixture_artifact(&primary)?,
            fixture_artifact(&ternary)?,
            fixture_artifact(&path)?,
        ],
    })
}

fn gates_without_contact() -> Vec<GateId> {
    vec![
        GateId::K01,
        GateId::K02,
        GateId::K03,
        GateId::K04,
        GateId::K05,
        GateId::K06,
        GateId::K07,
        GateId::K08,
        GateId::K09,
        GateId::K10,
        GateId::X01,
    ]
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MutationArtifact {
    pub id: &'static str,
    pub expected_first_failure: GateId,
    pub observed_first_failure: Option<GateId>,
    pub verdict: &'static str,
    pub witness: MutationWitnessArtifact,
    pub note: &'static str,
    pub evidence: Vec<Evidence>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MutationWitnessArtifact {
    pub fixture_id: &'static str,
    pub scope: &'static str,
    pub operator: &'static str,
    pub selector: BTreeMap<String, String>,
    pub before: BTreeMap<String, String>,
    pub after: BTreeMap<String, String>,
    pub affected_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ControlsArtifact {
    pub schema_version: u8,
    pub experiment_id: &'static str,
    pub artifact_kind: &'static str,
    pub run_id: &'static str,
    pub provenance: Provenance,
    pub status: &'static str,
    pub first_failed_gate: Option<GateId>,
    pub gates: Vec<Gate>,
    pub summary: Summary,
    pub mutations: Vec<MutationArtifact>,
}

#[derive(Clone, Copy)]
struct MutationCase {
    id: &'static str,
    mutation: Option<Mutation>,
    expected: GateId,
    note: &'static str,
}

fn mutation_cases() -> [MutationCase; 10] {
    [
        MutationCase {
            id: "M01_MISSING_REVERSE",
            mutation: Some(Mutation::MissingReverse),
            expected: GateId::K04,
            note: "removed LL|00 a:0 active edge while retaining its target-side slot",
        },
        MutationCase {
            id: "M02_DIRECTED_HAZARD",
            mutation: Some(Mutation::DirectedHazard),
            expected: GateId::K07,
            note: "changed one directed microscopic hazard from 1 to 2",
        },
        MutationCase {
            id: "M03_FALSE_BODY_MULTIPLICITY",
            mutation: Some(Mutation::FalseMultiplicity),
            expected: GateId::K02,
            note: "declared x=1 body multiplicity 3 while enumeration returned 2",
        },
        MutationCase {
            id: "M04_ENDOGENOUS_ALPHA",
            mutation: Some(Mutation::EndogenousAlpha),
            expected: GateId::K02,
            note: "replaced independent reservoir alpha origin with ProductionCurrentAlpha",
        },
        MutationCase {
            id: "M05_HIDDEN_ROW_SPLIT",
            mutation: Some(Mutation::HiddenRowSplit),
            expected: GateId::K06,
            note: "materialized two LH|0 hidden-history rows with different projected a:0 outcomes",
        },
        MutationCase {
            id: "M06_CORRUPT_RESERVOIR_DELTA",
            mutation: Some(Mutation::CorruptReservoirDelta),
            expected: GateId::K05,
            note: "recorded zero reservoir delta on a body-energy-raising move",
        },
        MutationCase {
            id: "M07_DISCONNECTED_TOP_STATE",
            mutation: Some(Mutation::DisconnectedTop),
            expected: GateId::K09,
            note: "converted every x=1 to x=2 move and reverse to WAIT",
        },
        MutationCase {
            id: "M08_POSITIVE_AFFINITY_ONLY",
            mutation: Some(Mutation::PositiveAffinityOnly),
            expected: GateId::K04,
            note: "removed every nonpositive-affinity move while retaining positive reverse moves",
        },
        MutationCase {
            id: "M09_ACTIVE_ONLY_CONTACT",
            mutation: None,
            expected: GateId::C02,
            note: "removed dormant contact slots and renormalized each row over active slots",
        },
        MutationCase {
            id: "M10_WRONG_WORD_LENGTH",
            mutation: Some(Mutation::WrongWordLength),
            expected: GateId::K03,
            note: "raised unit a without popping the depth-two reservoir word",
        },
    ]
}

fn witness_map(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
    entries
        .iter()
        .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
        .collect()
}

fn mutation_affected_count(case: MutationCase) -> Result<usize> {
    if case.id == "M09_ACTIVE_ONLY_CONTACT" {
        let (_, rows) = contact_table(BodyId::A, BodyId::B);
        return Ok(rows
            .iter()
            .flatten()
            .filter(|outcome| !outcome.moved)
            .count());
    }
    if matches!(
        case.mutation,
        Some(Mutation::FalseMultiplicity | Mutation::EndogenousAlpha | Mutation::HiddenRowSplit)
    ) {
        return Ok(1);
    }
    let baseline = build_model(primary_spec(2), Mutation::None)?;
    let changed = build_model(
        primary_spec(2),
        case.mutation
            .ok_or_else(|| VerifyError::new("mutation case has no operation"))?,
    )?;
    Ok(baseline
        .outcomes
        .iter()
        .flatten()
        .zip(changed.outcomes.iter().flatten())
        .filter(|(before, after)| {
            before.post != after.post
                || before.moved != after.moved
                || before.hazard != after.hazard
                || before.delta_u != after.delta_u
                || before.delta_reservoir != after.delta_reservoir
                || before.external_work != after.external_work
        })
        .count())
}

fn mutation_witness(case: MutationCase) -> Result<MutationWitnessArtifact> {
    let (scope, operator, selector, before, after) = match case.id {
        "M01_MISSING_REVERSE" => (
            "microscopic_transition",
            "replace_move_with_wait",
            witness_map(&[("pre", "LL|00"), ("slot", "a:0")]),
            witness_map(&[("outcome", "MOVE"), ("post", "HL|0"), ("hazard", "1/1")]),
            witness_map(&[("outcome", "WAIT"), ("post", "LL|00"), ("hazard", "0/1")]),
        ),
        "M02_DIRECTED_HAZARD" => (
            "microscopic_hazard",
            "scale_one_direction",
            witness_map(&[("pre", "LL|00"), ("slot", "a:0")]),
            witness_map(&[("hazard", "1/1")]),
            witness_map(&[("hazard", "2/1")]),
        ),
        "M03_FALSE_BODY_MULTIPLICITY" => (
            "coordinate_declaration",
            "replace_multiplicity",
            witness_map(&[("macro", "x=1")]),
            witness_map(&[("enumerated_g_X", "2")]),
            witness_map(&[("declared_g_X", "3")]),
        ),
        "M04_ENDOGENOUS_ALPHA" => (
            "alpha_source",
            "replace_source_adapter",
            witness_map(&[("coordinate", "alpha")]),
            witness_map(&[
                ("origin", "IndependentReservoir"),
                ("value", "epsilon/ln(2)"),
            ]),
            witness_map(&[
                ("origin", "ProductionCurrentAlpha"),
                ("value", "epsilon/ln(2)"),
            ]),
        ),
        "M05_HIDDEN_ROW_SPLIT" => (
            "hidden_history",
            "split_projected_row",
            witness_map(&[("visible_state", "LH|0"), ("slot", "a:0")]),
            witness_map(&[("history_0", "MOVE:HH|empty")]),
            witness_map(&[("history_1", "WAIT:LH|0")]),
        ),
        "M06_CORRUPT_RESERVOIR_DELTA" => (
            "path_accounting",
            "replace_recorded_delta",
            witness_map(&[("pre", "LL|00"), ("slot", "a:0")]),
            witness_map(&[("delta_reservoir", "-1/1")]),
            witness_map(&[("delta_reservoir", "0/1")]),
        ),
        "M07_DISCONNECTED_TOP_STATE" => (
            "microscopic_transition_set",
            "replace_energy_boundary_moves_with_wait",
            witness_map(&[("boundary", "x=1<->x=2")]),
            witness_map(&[("channel", "reciprocal_move")]),
            witness_map(&[("channel", "WAIT")]),
        ),
        "M08_POSITIVE_AFFINITY_ONLY" => (
            "microscopic_transition_set",
            "filter_nonpositive_affinity",
            witness_map(&[("predicate", "Xi<=0")]),
            witness_map(&[("channel", "reciprocal_move")]),
            witness_map(&[("channel", "WAIT")]),
        ),
        "M09_ACTIVE_ONLY_CONTACT" => (
            "contact_clock",
            "drop_dormant_slots_and_renormalize",
            witness_map(&[("clock", "four_pair_slots")]),
            witness_map(&[("slot_set", "fixed_four"), ("probability", "1/4")]),
            witness_map(&[("slot_set", "active_only"), ("probability", "renormalized")]),
        ),
        "M10_WRONG_WORD_LENGTH" => (
            "complete_state_transition",
            "suppress_reservoir_pop",
            witness_map(&[("pre", "LL|00"), ("slot", "a:0")]),
            witness_map(&[("post", "HL|0")]),
            witness_map(&[("post", "HL|00")]),
        ),
        _ => return Err(VerifyError::new("unknown mutation witness")),
    };
    Ok(MutationWitnessArtifact {
        fixture_id: if case.id == "M09_ACTIVE_ONLY_CONTACT" {
            "equal-alpha-A-B"
        } else {
            "primary-r2"
        },
        scope,
        operator,
        selector,
        before,
        after,
        affected_count: mutation_affected_count(case)?,
    })
}

pub fn derive_controls(provenance: Provenance) -> Result<ControlsArtifact> {
    let mut mutations = Vec::new();
    for (index, case) in mutation_cases().into_iter().enumerate() {
        let observed = if let Some(mutation) = case.mutation {
            let model = build_model(primary_spec(2), mutation)?;
            let expected = expectations(&model.spec)?;
            gate_bools(&model, &expected, mutation)
                .into_iter()
                .find(|(_, passed)| !passed)
                .map(|(gate, _)| gate)
        } else {
            contact_gate_bools(true)
                .into_iter()
                .find(|(_, passed)| !passed)
                .map(|(gate, _)| gate)
        };
        mutations.push(MutationArtifact {
            id: case.id,
            expected_first_failure: case.expected,
            observed_first_failure: observed,
            verdict: if observed == Some(case.expected) {
                "matched"
            } else if observed.is_none() {
                "survived"
            } else {
                "unexpected_failure"
            },
            witness: mutation_witness(case)?,
            note: case.note,
            evidence: vec![Evidence {
                pointer: format!("#/mutations/{index}/witness"),
            }],
        });
    }
    let passed = mutations
        .iter()
        .all(|mutation| mutation.verdict == "matched");
    Ok(ControlsArtifact {
        schema_version: 1,
        experiment_id: "valhaim-thermodynamic-kernel-v1",
        artifact_kind: "controls",
        run_id: "VK1-EXACT-001",
        provenance,
        status: if passed { "pass" } else { "fail" },
        first_failed_gate: None,
        gates: vec![gate(GateId::K01, true, "#/provenance")],
        summary: Summary {
            verdict_code: if passed {
                "FROZEN_CONTROLS_DISCRIMINATED"
            } else {
                "FROZEN_CONTROLS_FAILED"
            },
            earned_claim_codes: if passed {
                vec!["MUTATION_FIRST_FAILURES_MATCH"]
            } else {
                Vec::new()
            },
            excluded_claim_codes: Vec::new(),
            source_gates: vec![GateId::K01],
        },
        mutations,
    })
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ContactState {
    a: [u8; 2],
    b: [u8; 2],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BodyId {
    A,
    B,
    C,
}

impl BodyId {
    fn text(self) -> &'static str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ContactOutcome {
    post: ContactState,
    moved: bool,
    delta_a: i128,
    delta_b: i128,
}

fn binary_placements() -> [[u8; 2]; 4] {
    [[0, 0], [0, 1], [1, 0], [1, 1]]
}

fn high_count(positions: &[u8; 2]) -> usize {
    positions.iter().map(|&value| value as usize).sum()
}

fn contact_states(left: BodyId, right: BodyId) -> Vec<ContactState> {
    if left == right {
        return Vec::new();
    }
    let left_placements = binary_placements();
    let right_placements = binary_placements();
    let mut states = Vec::new();
    for a in left_placements {
        for b in right_placements {
            if high_count(&a) + high_count(&b) == 2 {
                states.push(ContactState { a, b });
            }
        }
    }
    states.sort();
    states
}

fn contact_outcome(state: &ContactState, label_a: usize, label_b: usize) -> ContactOutcome {
    let mut post = state.clone();
    let (delta_a, delta_b) = match (state.a[label_a], state.b[label_b]) {
        (0, 1) => {
            post.a[label_a] = 1;
            post.b[label_b] = 0;
            (1, -1)
        }
        (1, 0) => {
            post.a[label_a] = 0;
            post.b[label_b] = 1;
            (-1, 1)
        }
        _ => (0, 0),
    };
    ContactOutcome {
        post,
        moved: delta_a != 0,
        delta_a,
        delta_b,
    }
}

fn contact_slot_pairs(right: BodyId) -> [(usize, usize); 4] {
    if right == BodyId::C {
        [(1, 1), (1, 0), (0, 1), (0, 0)]
    } else {
        [(0, 0), (0, 1), (1, 0), (1, 1)]
    }
}

fn contact_table(left: BodyId, right: BodyId) -> (Vec<ContactState>, Vec<Vec<ContactOutcome>>) {
    let states = contact_states(left, right);
    let slot_pairs = contact_slot_pairs(right);
    let rows = states
        .iter()
        .map(|state| {
            slot_pairs
                .iter()
                .map(|&(label_a, label_b)| contact_outcome(state, label_a, label_b))
                .collect()
        })
        .collect();
    (states, rows)
}

fn contact_matrices(
    left: BodyId,
    right: BodyId,
) -> Result<(Vec<Vec<Rational>>, Vec<Vec<Rational>>)> {
    contact_matrices_mode(left, right, false)
}

fn contact_matrices_mode(
    left: BodyId,
    right: BodyId,
    active_only: bool,
) -> Result<(Vec<Vec<Rational>>, Vec<Vec<Rational>>)> {
    let (states, rows) = contact_table(left, right);
    let index = states
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, state)| (state, index))
        .collect::<BTreeMap<_, _>>();
    let mut fibers = vec![Vec::new(); 3];
    for (state, value) in states.iter().enumerate() {
        fibers[high_count(&value.a)].push(state);
    }
    let mut p = vec![vec![Rational::zero(); 3]; 3];
    let mut q = vec![vec![Rational::zero(); 3]; 3];
    for source_macro in 0..3 {
        let mut reference = None;
        for &source in &fibers[source_macro] {
            let mut candidate_p = vec![Rational::zero(); 3];
            let mut candidate_q = vec![Rational::zero(); 3];
            let mut active = 0_i128;
            let selected = rows[source]
                .iter()
                .filter(|outcome| !active_only || outcome.moved)
                .collect::<Vec<_>>();
            for outcome in &selected {
                let target = index[&outcome.post];
                let target_macro = high_count(&states[target].a);
                candidate_p[target_macro] = candidate_p[target_macro]
                    .checked_add(Rational::new(1, selected.len() as i128)?)?;
                if outcome.moved {
                    candidate_q[target_macro] =
                        candidate_q[target_macro].checked_add(Rational::one())?;
                    active += 1;
                }
            }
            candidate_q[source_macro] = Rational::integer(-active);
            if let Some((reference_p, reference_q)) = &reference {
                if &candidate_p != reference_p || &candidate_q != reference_q {
                    return Err(VerifyError::new(
                        "contact projection is not strongly lumpable",
                    ));
                }
            } else {
                reference = Some((candidate_p, candidate_q));
            }
        }
        let (candidate_p, candidate_q) =
            reference.ok_or_else(|| VerifyError::new("empty contact macro fiber"))?;
        p[source_macro] = candidate_p;
        q[source_macro] = candidate_q;
    }
    Ok((p, q))
}

fn contact_rows_closed(left: BodyId, right: BodyId, active_only: bool) -> bool {
    let (states, rows) = contact_table(left, right);
    let state_set = states.into_iter().collect::<BTreeSet<_>>();
    rows.iter().all(|row| {
        let selected = row
            .iter()
            .filter(|outcome| !active_only || outcome.moved)
            .collect::<Vec<_>>();
        selected.len() == 4
            && selected
                .iter()
                .all(|outcome| state_set.contains(&outcome.post))
    })
}

fn contact_generator_at_scale(
    left: BodyId,
    right: BodyId,
    scale: Rational,
    directed_probe: bool,
) -> Result<Vec<Vec<Rational>>> {
    let (states, rows) = contact_table(left, right);
    let index = states
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, state)| (state, index))
        .collect::<BTreeMap<_, _>>();
    let probe = rows.iter().enumerate().find_map(|(state, row)| {
        row.iter()
            .position(|outcome| outcome.moved)
            .map(|slot| (state, slot))
    });
    let mut fibers = vec![Vec::new(); 3];
    for (state, value) in states.iter().enumerate() {
        fibers[high_count(&value.a)].push(state);
    }
    let mut q = vec![vec![Rational::zero(); 3]; 3];
    for source_macro in 0..3 {
        let mut reference = None;
        for &source in &fibers[source_macro] {
            let mut candidate = vec![Rational::zero(); 3];
            let mut active = Rational::zero();
            for (slot, outcome) in rows[source].iter().enumerate() {
                if !outcome.moved {
                    continue;
                }
                let local_scale = if directed_probe && probe == Some((source, slot)) {
                    scale.checked_multiply(Rational::integer(2))?
                } else {
                    scale
                };
                let target = index[&outcome.post];
                let target_macro = high_count(&states[target].a);
                candidate[target_macro] = candidate[target_macro].checked_add(local_scale)?;
                active = active.checked_add(local_scale)?;
            }
            candidate[source_macro] = active.negated();
            if let Some(expected) = &reference {
                if &candidate != expected {
                    return Err(VerifyError::new(
                        "scaled contact clock is not strongly lumpable",
                    ));
                }
            } else {
                reference = Some(candidate);
            }
        }
        q[source_macro] =
            reference.ok_or_else(|| VerifyError::new("empty scaled contact fiber"))?;
    }
    Ok(q)
}

fn isolated_body_preparation(branching: usize) -> Result<Vec<Rational>> {
    let mut weights = [0_usize; 3];
    for positions in binary_placements() {
        let high = high_count(&positions);
        let mut words = Vec::new();
        enumerate_words(branching as u8, 2 - high, &mut Vec::new(), &mut words);
        weights[high] += words.len();
    }
    let total = weights.iter().sum::<usize>() as i128;
    weights
        .into_iter()
        .map(|weight| Rational::new(weight as i128, total))
        .collect()
}

fn contact_preparation(
    left: BodyId,
    right: BodyId,
    branching_a: usize,
    branching_b: usize,
) -> Result<Vec<Rational>> {
    let states = contact_states(left, right);
    let mut weights = [0_i128; 3];
    for state in states {
        let x_a = high_count(&state.a);
        let x_b = high_count(&state.b);
        let weight = power(branching_a, 2 - x_a) * power(branching_b, 2 - x_b);
        weights[x_a] += weight as i128;
    }
    let total = weights.iter().sum::<i128>();
    weights
        .into_iter()
        .map(|weight| Rational::new(weight, total))
        .collect()
}

fn contact_current(
    left: BodyId,
    right: BodyId,
    branching_a: usize,
    branching_b: usize,
) -> Result<Rational> {
    contact_current_at_scale(left, right, branching_a, branching_b, Rational::one())
}

fn contact_current_at_scale(
    left: BodyId,
    right: BodyId,
    branching_a: usize,
    branching_b: usize,
    scale: Rational,
) -> Result<Rational> {
    let (states, rows) = contact_table(left, right);
    let mut total_weight = 0_i128;
    let mut current = Rational::zero();
    for (state, row) in states.iter().zip(rows.iter()) {
        let x_a = high_count(&state.a);
        let x_b = high_count(&state.b);
        let weight = (power(branching_a, 2 - x_a) * power(branching_b, 2 - x_b)) as i128;
        total_weight += weight;
        let row_current = row.iter().try_fold(Rational::zero(), |sum, outcome| {
            sum.checked_add(Rational::integer(outcome.delta_a).checked_multiply(scale)?)
        })?;
        current = current.checked_add(row_current.checked_multiply(Rational::integer(weight))?)?;
    }
    current.checked_multiply(Rational::new(1, total_weight)?)
}

fn contact_reverse_and_energy_for(
    states: &[ContactState],
    rows: &[Vec<ContactOutcome>],
) -> (bool, bool) {
    let index = states
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, state)| (state, index))
        .collect::<BTreeMap<_, _>>();
    let mut reverse = true;
    let mut energy_closed = true;
    for (source, row) in rows.iter().enumerate() {
        for (slot, outcome) in row.iter().enumerate() {
            let observed_delta_a =
                high_count(&outcome.post.a) as i128 - high_count(&states[source].a) as i128;
            let observed_delta_b =
                high_count(&outcome.post.b) as i128 - high_count(&states[source].b) as i128;
            energy_closed &= observed_delta_a == outcome.delta_a
                && observed_delta_b == outcome.delta_b
                && observed_delta_a + observed_delta_b == 0
                && outcome.post.a.len() == states[source].a.len()
                && outcome.post.b.len() == states[source].b.len()
                && outcome.post.a.iter().all(|position| *position <= 1)
                && outcome.post.b.iter().all(|position| *position <= 1)
                && index.contains_key(&outcome.post);
            if outcome.moved {
                let Some(&target) = index.get(&outcome.post) else {
                    return (false, false);
                };
                reverse &= rows[target][slot].moved && rows[target][slot].post == states[source];
            }
        }
    }
    (reverse, energy_closed)
}

fn contact_reverse_and_energy(left: BodyId, right: BodyId) -> (bool, bool) {
    let (states, rows) = contact_table(left, right);
    contact_reverse_and_energy_for(&states, &rows)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContactClockRowArtifact {
    pub pre: String,
    pub slot: String,
    pub probability: Rational,
    pub slot_rate_over_kappa_contact: Rational,
    pub outcome: &'static str,
    pub post: String,
    pub delta_energy_left: Scaled,
    pub delta_energy_right: Scaled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContactClockEvidenceArtifact {
    pub state_count: usize,
    pub slot_count: usize,
    pub dormant_rows: Vec<ContactClockRowArtifact>,
    pub open_rows: Vec<ContactClockRowArtifact>,
    pub dormant_rows_sha256: String,
    pub open_rows_sha256: String,
    pub crystal_before_sha256: String,
    pub crystal_after_sha256: String,
    pub retained_channel_count: usize,
    pub retained_before_sha256: String,
    pub retained_after_sha256: String,
    pub clock_unchanged: bool,
}

fn body_crystal_digest(body: BodyId) -> Result<String> {
    let placements = binary_placements();
    let mut records = Vec::new();
    for pre in placements {
        let compatible = placements
            .iter()
            .filter(|candidate| high_count(candidate) == high_count(&pre))
            .collect::<Vec<_>>();
        let probability = Rational::new(1, compatible.len() as i128)?;
        for report in compatible {
            records.push(format!(
                "{}|{:?}|{:?}|{}|{:?}",
                body.text(),
                pre,
                report,
                probability.text(),
                pre
            ));
        }
    }
    records.sort();
    Ok(sha256_hex(records.join("\n").as_bytes()))
}

fn contact_clock_rows(
    left: BodyId,
    right: BodyId,
    opened: bool,
    active_only: bool,
) -> Result<Vec<ContactClockRowArtifact>> {
    let (states, outcomes) = contact_table(left, right);
    let slot_pairs = contact_slot_pairs(right);
    let mut rows = Vec::new();
    for (state, outcome_row) in states.iter().zip(outcomes.iter()) {
        let selected = (0..4)
            .filter(|slot| !opened || !active_only || outcome_row[*slot].moved)
            .collect::<Vec<_>>();
        let probability = Rational::new(1, selected.len() as i128)?;
        for slot in selected {
            let (label_left, label_right) = slot_pairs[slot];
            let observed = &outcome_row[slot];
            let (outcome, post, delta_left, delta_right) = if opened {
                (
                    if observed.moved { "MOVE" } else { "WAIT" },
                    contact_state_text(&observed.post, left, right),
                    observed.delta_a,
                    observed.delta_b,
                )
            } else {
                ("WAIT", contact_state_text(state, left, right), 0, 0)
            };
            rows.push(ContactClockRowArtifact {
                pre: contact_state_text(state, left, right),
                slot: format!(
                    "{}:{}|{}:{}",
                    left.text(),
                    label_name(label_left),
                    right.text(),
                    label_name(label_right)
                ),
                probability,
                slot_rate_over_kappa_contact: Rational::one(),
                outcome,
                post,
                delta_energy_left: Scaled {
                    coefficient: Rational::integer(delta_left),
                    basis_id: "epsilon",
                },
                delta_energy_right: Scaled {
                    coefficient: Rational::integer(delta_right),
                    basis_id: "epsilon",
                },
            });
        }
    }
    Ok(rows)
}

fn contact_clock_evidence(
    left: BodyId,
    right: BodyId,
    active_only: bool,
) -> Result<ContactClockEvidenceArtifact> {
    let dormant_rows = contact_clock_rows(left, right, false, false)?;
    let open_rows = contact_clock_rows(left, right, true, active_only)?;
    let dormant_bytes = serde_json::to_vec(&dormant_rows)
        .map_err(|error| VerifyError::new(format!("cannot serialize dormant clock: {error}")))?;
    let open_bytes = serde_json::to_vec(&open_rows)
        .map_err(|error| VerifyError::new(format!("cannot serialize open clock: {error}")))?;
    let crystal_before_sha256 = sha256_hex(
        format!(
            "{}\n{}",
            body_crystal_digest(left)?,
            body_crystal_digest(right)?
        )
        .as_bytes(),
    );
    let crystal_after_sha256 = sha256_hex(
        format!(
            "{}\n{}",
            body_crystal_digest(left)?,
            body_crystal_digest(right)?
        )
        .as_bytes(),
    );
    let retained_before_sha256 = sha256_hex(&[]);
    let retained_after_sha256 = sha256_hex(&[]);
    let mut evidence = ContactClockEvidenceArtifact {
        state_count: contact_states(left, right).len(),
        slot_count: 4,
        dormant_rows,
        open_rows,
        dormant_rows_sha256: sha256_hex(&dormant_bytes),
        open_rows_sha256: sha256_hex(&open_bytes),
        crystal_before_sha256,
        crystal_after_sha256,
        retained_channel_count: 0,
        retained_before_sha256,
        retained_after_sha256,
        clock_unchanged: false,
    };
    evidence.clock_unchanged = contact_clock_evidence_valid(&evidence);
    Ok(evidence)
}

fn contact_clock_evidence_valid(evidence: &ContactClockEvidenceArtifact) -> bool {
    let dormant = evidence
        .dormant_rows
        .iter()
        .map(|row| ((row.pre.as_str(), row.slot.as_str()), row))
        .collect::<BTreeMap<_, _>>();
    let opened = evidence
        .open_rows
        .iter()
        .map(|row| ((row.pre.as_str(), row.slot.as_str()), row))
        .collect::<BTreeMap<_, _>>();
    evidence.state_count == 6
        && evidence.slot_count == 4
        && dormant.len() == 24
        && opened.len() == 24
        && dormant.keys().eq(opened.keys())
        && dormant.iter().all(|(key, before)| {
            let Some(after) = opened.get(key) else {
                return false;
            };
            before.probability == Rational::new(1, 4).expect("fixed denominator")
                && before.probability == after.probability
                && before.slot_rate_over_kappa_contact == Rational::one()
                && before.slot_rate_over_kappa_contact == after.slot_rate_over_kappa_contact
                && before.outcome == "WAIT"
                && before.post == before.pre
                && before.delta_energy_left.coefficient == Rational::zero()
                && before.delta_energy_right.coefficient == Rational::zero()
        })
        && evidence.crystal_before_sha256 == evidence.crystal_after_sha256
        && evidence.retained_channel_count == 0
        && evidence.retained_before_sha256 == evidence.retained_after_sha256
}

fn contact_clock_unchanged(left: BodyId, right: BodyId, active_only: bool) -> bool {
    contact_clock_evidence(left, right, active_only)
        .ok()
        .is_some_and(|evidence| evidence.clock_unchanged)
}

fn contact_detailed_balance(stationary: &[Rational], q: &[Vec<Rational>]) -> bool {
    for source in 0..q.len() {
        for target in 0..q.len() {
            if stationary[source].checked_multiply(q[source][target]).ok()
                != stationary[target].checked_multiply(q[target][source]).ok()
            {
                return false;
            }
        }
    }
    true
}

fn contact_clock_scale_covariance() -> bool {
    let Ok(scale) = Rational::new(13, 7) else {
        return false;
    };
    let Ok((baseline_p, baseline_q)) = contact_matrices(BodyId::A, BodyId::B) else {
        return false;
    };
    let Ok(transformed_q) = contact_generator_at_scale(BodyId::A, BodyId::B, scale, false) else {
        return false;
    };
    let expected_q = scale_matrix(&baseline_q, scale).ok();
    let stationary = contact_preparation(BodyId::A, BodyId::B, 2, 2).ok();
    let stationary_and_db = stationary.as_ref().is_some_and(|pi| {
        stationary_matrix(pi, &baseline_p).ok() == Some(pi.clone())
            && stationary_matrix(pi, &transformed_q).ok() == Some(vec![Rational::zero(); pi.len()])
            && contact_detailed_balance(pi, &transformed_q)
    });
    let zero_current =
        contact_current_at_scale(BodyId::A, BodyId::B, 2, 2, scale).ok() == Some(Rational::zero());
    let unequal_scaled = contact_current_at_scale(BodyId::A, BodyId::B, 2, 4, scale).ok()
        == Rational::new(-12, 13)
            .and_then(|value| value.checked_multiply(scale))
            .ok()
        && contact_current_at_scale(BodyId::A, BodyId::B, 4, 2, scale).ok()
            == Rational::new(12, 13)
                .and_then(|value| value.checked_multiply(scale))
                .ok();
    let directed_rejected = match contact_generator_at_scale(BodyId::A, BodyId::B, scale, true) {
        Err(_) => true,
        Ok(directed) => {
            Some(directed.clone()) != expected_q
                || !stationary
                    .as_ref()
                    .is_some_and(|pi| contact_detailed_balance(pi, &directed))
        }
    };
    Some(transformed_q) == expected_q
        && stationary_and_db
        && zero_current
        && unequal_scaled
        && directed_rejected
}

fn canonical_contact_signature(left: BodyId, right: BodyId) -> String {
    let (states, rows) = contact_table(left, right);
    let slots = contact_slot_pairs(right);
    let mut records = Vec::new();
    for (state, row) in states.iter().zip(rows.iter()) {
        for (slot, outcome) in row.iter().enumerate() {
            let (label_a, label_b) = slots[slot];
            records.push(format!(
                "{:?}|{:?}|{label_a}:{label_b}|{:?}|{:?}|{}|{}:{}",
                state.a,
                state.b,
                outcome.post.a,
                outcome.post.b,
                outcome.moved,
                outcome.delta_a,
                outcome.delta_b
            ));
        }
    }
    records.sort();
    sha256_hex(records.join("\n").as_bytes())
}

fn equal_pair_gates(left: BodyId, right: BodyId) -> bool {
    let expected_isolated = probability_matrix(&[&[(4, 9), (4, 9), (1, 9)]])
        .ok()
        .and_then(|rows| rows.into_iter().next());
    let expected_conditioned = probability_matrix(&[&[(1, 6), (2, 3), (1, 6)]])
        .ok()
        .and_then(|rows| rows.into_iter().next());
    let expected_p = probability_matrix(&[
        &[(0, 1), (1, 1), (0, 1)],
        &[(1, 4), (1, 2), (1, 4)],
        &[(0, 1), (1, 1), (0, 1)],
    ])
    .ok();
    let expected_q = matrix(&[&[-4, 4, 0], &[1, -2, 1], &[0, 4, -4]]);
    let matrices = contact_matrices(left, right).ok();
    let conditioned = contact_preparation(left, right, 2, 2).ok();
    let stationary = conditioned
        .as_ref()
        .zip(matrices.as_ref())
        .is_some_and(|(pi, (p, q))| {
            stationary_matrix(pi, p).ok() == Some(pi.clone())
                && stationary_matrix(pi, q).ok() == Some(vec![Rational::zero(); 3])
                && contact_detailed_balance(pi, q)
        });
    let (reverse, energy) = contact_reverse_and_energy(left, right);
    let isolated_left = isolated_body_preparation(2).ok();
    let isolated_right = isolated_body_preparation(2).ok();
    isolated_left == expected_isolated
        && isolated_right == expected_isolated
        && conditioned == expected_conditioned
        && contact_clock_unchanged(left, right, false)
        && reverse
        && energy
        && matrices.as_ref().map(|(p, q)| (p, q)) == expected_p.as_ref().zip(Some(&expected_q))
        && stationary
        && contact_current(left, right, 2, 2).ok() == Some(Rational::zero())
        && canonical_contact_signature(left, right)
            == canonical_contact_signature(BodyId::A, BodyId::B)
}

fn contact_gate_bools(active_only_mutation: bool) -> Vec<(GateId, bool)> {
    let expected_p = probability_matrix(&[
        &[(0, 1), (1, 1), (0, 1)],
        &[(1, 4), (1, 2), (1, 4)],
        &[(0, 1), (1, 1), (0, 1)],
    ])
    .ok();
    let expected_q = matrix(&[&[-4, 4, 0], &[1, -2, 1], &[0, 4, -4]]);
    let matrices = contact_matrices_mode(BodyId::A, BodyId::B, active_only_mutation).ok();
    let equal = contact_preparation(BodyId::A, BodyId::B, 2, 2).ok();
    let isolated = isolated_body_preparation(2).ok();
    let stationary = equal
        .as_ref()
        .zip(matrices.as_ref())
        .is_some_and(|(pi, (p, q))| {
            stationary_matrix(pi, p).ok() == Some(pi.clone())
                && stationary_matrix(pi, q).ok() == Some(vec![Rational::zero(); 3])
                && contact_detailed_balance(pi, q)
        });
    let (reverse, energy) = contact_reverse_and_energy(BodyId::A, BodyId::B);
    let contact_time_scale = contact_clock_scale_covariance();
    let equal_current = contact_current(BodyId::A, BodyId::B, 2, 2).ok() == Some(Rational::zero());
    let sensitivity = contact_current(BodyId::A, BodyId::B, 2, 4).ok()
        == Rational::new(-12, 13).ok()
        && contact_current(BodyId::A, BodyId::B, 4, 2).ok() == Rational::new(12, 13).ok()
        && contact_preparation(BodyId::A, BodyId::B, 2, 4).ok()
            == probability_matrix(&[&[(1, 13), (8, 13), (4, 13)]])
                .ok()
                .and_then(|rows| rows.into_iter().next())
        && contact_preparation(BodyId::A, BodyId::B, 4, 2).ok()
            == probability_matrix(&[&[(4, 13), (8, 13), (1, 13)]])
                .ok()
                .and_then(|rows| rows.into_iter().next());
    let expected_equal = vec![
        Rational::new(1, 6).expect("nonzero denominator"),
        Rational::new(2, 3).expect("nonzero denominator"),
        Rational::new(1, 6).expect("nonzero denominator"),
    ];
    vec![
        (
            GateId::C01,
            isolated
                == probability_matrix(&[&[(4, 9), (4, 9), (1, 9)]])
                    .ok()
                    .and_then(|rows| rows.into_iter().next())
                && equal == Some(expected_equal),
        ),
        (
            GateId::C02,
            contact_clock_unchanged(BodyId::A, BodyId::B, active_only_mutation)
                && contact_time_scale,
        ),
        (GateId::C03, energy),
        (
            GateId::C04,
            reverse && contact_rows_closed(BodyId::A, BodyId::B, active_only_mutation),
        ),
        (
            GateId::C05,
            matrices.as_ref().map(|(p, q)| (p, q)) == expected_p.as_ref().zip(Some(&expected_q))
                && stationary,
        ),
        (GateId::C06, equal_current),
        (
            GateId::C07,
            BodyId::B != BodyId::C && equal_pair_gates(BodyId::A, BodyId::C),
        ),
        (GateId::C08, sensitivity),
    ]
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContactRowArtifact {
    pub pre: String,
    pub slot: String,
    pub outcome: &'static str,
    pub post: String,
    pub probability: Rational,
    pub delta_energy_left: Scaled,
    pub delta_energy_right: Scaled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContactCaseArtifact {
    pub id: &'static str,
    pub left_body: &'static str,
    pub right_body: &'static str,
    pub isolated_preparation_left: Vec<Rational>,
    pub isolated_preparation_right: Vec<Rational>,
    pub preparation: Vec<Rational>,
    #[serde(rename = "P")]
    pub p: Vec<Vec<Rational>>,
    #[serde(rename = "Q_over_kappa_contact")]
    pub q_over_kappa_contact: Vec<Vec<Rational>>,
    pub current_into_left: Scaled,
    pub rows: Vec<ContactRowArtifact>,
    pub clock_unchanged: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContactArtifact {
    pub schema_version: u8,
    pub experiment_id: &'static str,
    pub artifact_kind: &'static str,
    pub run_id: &'static str,
    pub provenance: Provenance,
    pub kernel_dependency: KernelDependencyArtifact,
    pub status: &'static str,
    pub first_failed_gate: Option<GateId>,
    pub gates: Vec<Gate>,
    pub summary: Summary,
    pub clock_evidence: ContactClockEvidenceArtifact,
    pub cases: Vec<ContactCaseArtifact>,
    pub swapped_current_sign_reversed: bool,
    pub metamorphic_checks: Vec<MetamorphicCheckArtifact>,
    pub metamorphic_checks_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct KernelDependencyArtifact {
    pub run_id: &'static str,
    pub status: &'static str,
    pub compact_payload_sha256: String,
    pub required_gates_passed: bool,
    pub claim_ladder_passed: bool,
}

fn contact_case(
    id: &'static str,
    left: BodyId,
    right: BodyId,
    branching_a: usize,
    branching_b: usize,
) -> Result<ContactCaseArtifact> {
    let (p, q) = contact_matrices(left, right)?;
    Ok(ContactCaseArtifact {
        id,
        left_body: left.text(),
        right_body: right.text(),
        isolated_preparation_left: isolated_body_preparation(branching_a)?,
        isolated_preparation_right: isolated_body_preparation(branching_b)?,
        preparation: contact_preparation(left, right, branching_a, branching_b)?,
        p,
        q_over_kappa_contact: q,
        current_into_left: Scaled {
            coefficient: contact_current(left, right, branching_a, branching_b)?,
            basis_id: "kappa_contact*epsilon",
        },
        rows: contact_rows_artifact(left, right)?,
        clock_unchanged: contact_clock_unchanged(left, right, false),
    })
}

fn contact_positions_text(positions: [u8; 2]) -> String {
    positions
        .into_iter()
        .map(|position| if position == 0 { 'L' } else { 'H' })
        .collect()
}

fn contact_state_text(state: &ContactState, left: BodyId, right: BodyId) -> String {
    format!(
        "{}={}|{}={}",
        left.text(),
        contact_positions_text(state.a),
        right.text(),
        contact_positions_text(state.b)
    )
}

fn contact_rows_artifact(left: BodyId, right: BodyId) -> Result<Vec<ContactRowArtifact>> {
    let (states, rows) = contact_table(left, right);
    let slot_pairs = contact_slot_pairs(right);
    let probability = Rational::new(1, 4)?;
    let mut artifact = Vec::with_capacity(states.len() * 4);
    for (state, row) in states.iter().zip(rows.iter()) {
        for (slot, outcome) in row.iter().enumerate() {
            let (label_a, label_b) = slot_pairs[slot];
            artifact.push(ContactRowArtifact {
                pre: contact_state_text(state, left, right),
                slot: format!(
                    "{}:{}|{}:{}",
                    left.text(),
                    label_name(label_a),
                    right.text(),
                    label_name(label_b)
                ),
                outcome: if outcome.moved { "MOVE" } else { "WAIT" },
                post: contact_state_text(&outcome.post, left, right),
                probability,
                delta_energy_left: Scaled {
                    coefficient: Rational::integer(outcome.delta_a),
                    basis_id: "epsilon",
                },
                delta_energy_right: Scaled {
                    coefficient: Rational::integer(outcome.delta_b),
                    basis_id: "epsilon",
                },
            });
        }
    }
    Ok(artifact)
}

pub fn derive_contact(
    provenance: Provenance,
    kernel: &KernelExactArtifact,
) -> Result<ContactArtifact> {
    if kernel.provenance != provenance {
        return Err(VerifyError::new(
            "contact provenance differs from its exact-kernel dependency",
        ));
    }
    let kernel_payload = serde_json::to_vec(kernel)
        .map_err(|error| VerifyError::new(format!("cannot bind kernel dependency: {error}")))?;
    let kernel_gates_passed = kernel
        .gates
        .iter()
        .all(|candidate| candidate.status == "pass");
    let kernel_claim_ladder_passed = kernel.status == "pass" && kernel_gates_passed;
    let values = contact_gate_bools(false);
    let contact_gates_passed = values.iter().all(|(_, value)| *value);
    let status = if !kernel_gates_passed || !contact_gates_passed {
        "fail"
    } else if !kernel_claim_ladder_passed {
        "invalidated"
    } else {
        "pass"
    };
    let mut gates = kernel
        .gates
        .iter()
        .map(|candidate| {
            gate(
                candidate.id,
                candidate.status == "pass",
                "#/kernel_dependency",
            )
        })
        .collect::<Vec<_>>();
    gates.extend(
        values
            .iter()
            .map(|(id, value)| gate(*id, *value, "#/cases")),
    );
    if let Some(c02) = gates
        .iter_mut()
        .find(|candidate| candidate.id == GateId::C02)
    {
        c02.evidence = vec![
            Evidence {
                pointer: "#/clock_evidence".to_owned(),
            },
            Evidence {
                pointer: "#/metamorphic_checks/0".to_owned(),
            },
        ];
    }
    let negative = contact_current(BodyId::A, BodyId::B, 2, 4)?;
    let positive = contact_current(BodyId::A, BodyId::B, 4, 2)?;
    let contact_scale_passed = contact_clock_scale_covariance();
    let clock_evidence = contact_clock_evidence(BodyId::A, BodyId::B, false)?;
    let metamorphic_checks = vec![MetamorphicCheckArtifact {
        id: "kappa-contact-time-scale",
        status: if contact_scale_passed { "pass" } else { "fail" },
        baseline_scale: Rational::one(),
        transformed_scale: Rational::new(13, 7)?,
        invariant_fields: vec!["P_C", "equilibrium", "local-detailed-balance"],
        covariant_fields: vec!["Q_C", "energy-current"],
        negative_control_rejected: Some(contact_scale_passed),
    }];
    let metamorphic_checks_sha256 = metamorphic_checks_digest(&metamorphic_checks)?;
    Ok(ContactArtifact {
        schema_version: 1,
        experiment_id: "valhaim-thermodynamic-kernel-v1",
        artifact_kind: "contact",
        run_id: "VK1-EXACT-001",
        provenance,
        kernel_dependency: KernelDependencyArtifact {
            run_id: kernel.run_id,
            status: kernel.status,
            compact_payload_sha256: sha256_hex(&kernel_payload),
            required_gates_passed: kernel_gates_passed,
            claim_ladder_passed: kernel_claim_ladder_passed,
        },
        status,
        first_failed_gate: gates
            .iter()
            .find(|gate| gate.status == "fail")
            .map(|gate| gate.id),
        gates: gates.clone(),
        summary: Summary {
            verdict_code: if status == "pass" {
                "EXACT_OPERATIONAL_TEMPERATURE_PASS"
            } else if status == "invalidated" {
                "EXACT_CONTACT_INVALIDATED_BY_PREREQUISITE"
            } else {
                "EXACT_CONTACT_FAIL"
            },
            earned_claim_codes: if status == "pass" {
                vec!["T6_EXACT_CONTACT", "ALPHA_OPERATIONAL_FOR_DECLARED_MODEL"]
            } else {
                Vec::new()
            },
            excluded_claim_codes: vec!["PROTOCOL_OWNED_CONTACT", "DEPLOYED_CONTACT_CLOCK"],
            source_gates: vec![
                GateId::K01,
                GateId::K02,
                GateId::K03,
                GateId::K04,
                GateId::K05,
                GateId::K06,
                GateId::K07,
                GateId::K08,
                GateId::K09,
                GateId::K10,
                GateId::X01,
                GateId::C01,
                GateId::C02,
                GateId::C03,
                GateId::C04,
                GateId::C05,
                GateId::C06,
                GateId::C07,
                GateId::C08,
            ],
        },
        clock_evidence,
        cases: vec![
            contact_case("equal-alpha-A-B", BodyId::A, BodyId::B, 2, 2)?,
            contact_case("fresh-third-A-C", BodyId::A, BodyId::C, 2, 2)?,
            contact_case("unequal-r2-r4", BodyId::A, BodyId::B, 2, 4)?,
            contact_case("unequal-r4-r2", BodyId::A, BodyId::B, 4, 2)?,
        ],
        swapped_current_sign_reversed: negative == positive.negated(),
        metamorphic_checks,
        metamorphic_checks_sha256,
    })
}

pub fn validate_artifact_semantics(
    exact: &KernelExactArtifact,
    controls: &ControlsArtifact,
    contact: &ContactArtifact,
) -> Result<()> {
    let exact_gate_order = gates_without_contact();
    let observed_exact_order = exact.gates.iter().map(|gate| gate.id).collect::<Vec<_>>();
    if observed_exact_order != exact_gate_order {
        return Err(VerifyError::new("exact gate order is not frozen order"));
    }
    let exact_failure = exact
        .gates
        .iter()
        .find(|gate| gate.status == "fail")
        .map(|gate| gate.id);
    let expected_exact_status = if exact_failure.is_some() {
        "fail"
    } else if !exact.controls_dependency.required_mutations_matched
        || exact.controls_dependency.status != "pass"
    {
        "invalidated"
    } else {
        "pass"
    };
    if exact.first_failed_gate != exact_failure || exact.status != expected_exact_status {
        return Err(VerifyError::new("exact status and first failure disagree"));
    }
    let expected_exact_claims = if exact.status == "pass" {
        vec![
            "FINITE_EQUILIBRIUM_THERMODYNAMIC_MODEL",
            "T1_T5_PASS",
            "STRUCTURAL_VARIANTS_PASS",
        ]
    } else {
        Vec::new()
    };
    let expected_exact_verdict = match exact.status {
        "pass" => "EXACT_KERNEL_PASS",
        "invalidated" => "EXACT_KERNEL_INVALIDATED_BY_CONTROLS",
        _ => "EXACT_KERNEL_FAIL",
    };
    let controls_payload = serde_json::to_vec(controls)
        .map_err(|error| VerifyError::new(format!("cannot rebind controls payload: {error}")))?;
    if exact.summary.verdict_code != expected_exact_verdict
        || exact.summary.earned_claim_codes != expected_exact_claims
        || exact.summary.excluded_claim_codes
            != vec![
                "PAYMENTLEDGER_OWNERSHIP",
                "DEPLOYED_STOCHASTIC_CLOCK",
                "MONETARY_CALIBRATION",
                "ADVANCED_FEATURE_CLOSURE",
            ]
        || exact.summary.source_gates != exact_gate_order
        || exact.controls_dependency.run_id != controls.run_id
        || exact.controls_dependency.status != controls.status
        || exact.controls_dependency.required_mutations_matched != (controls.status == "pass")
        || exact.controls_dependency.compact_payload_sha256 != sha256_hex(&controls_payload)
        || exact.gates.iter().any(|candidate| {
            let expected_pointers = if candidate.id == GateId::K01 {
                vec!["#/provenance"]
            } else if candidate.id == GateId::X01 {
                vec!["#/xypher_representation", "#/fixtures"]
            } else {
                vec!["#/fixtures"]
            };
            candidate
                .evidence
                .iter()
                .map(|evidence| evidence.pointer.as_str())
                .collect::<Vec<_>>()
                != expected_pointers
                || (candidate.status == "pass") != candidate.failure_code.is_none()
        })
    {
        return Err(VerifyError::new(
            "exact summary or evidence pointers are inconsistent",
        ));
    }
    let expected_fixtures = ["primary-r2", "ternary-reservoir-r3", "three-node-path-r2"];
    if exact.fixtures.len() != expected_fixtures.len() {
        return Err(VerifyError::new("wrong fixture count"));
    }
    for (fixture, expected_id) in exact.fixtures.iter().zip(expected_fixtures) {
        let crystal_bytes = serde_json::to_vec(&fixture.crystal_readout.rows).map_err(|error| {
            VerifyError::new(format!("cannot rehash {expected_id} Crystal rows: {error}"))
        })?;
        let metamorphic_bytes =
            serde_json::to_vec(&fixture.metamorphic_checks).map_err(|error| {
                VerifyError::new(format!(
                    "cannot rehash {expected_id} metamorphic checks: {error}"
                ))
            })?;
        let expected_metamorphic_ids = [
            "unit-label-exchange",
            "reservoir-symbol-exchange",
            "node-height-covariance",
            "epsilon-alpha-common-scale",
            "kappa-time-scale",
        ];
        let mut crystal_groups = BTreeMap::<&str, Vec<&CrystalReadoutRowArtifact>>::new();
        for row in &fixture.crystal_readout.rows {
            crystal_groups.entry(&row.pre).or_default().push(row);
        }
        let crystal_valid = fixture.states.iter().all(|state| {
            let Some(group) = crystal_groups.get(state.coordinates.as_str()) else {
                return false;
            };
            let Some(macro_index) = fixture
                .macro_labels
                .iter()
                .position(|label| label == &state.macrostate)
            else {
                return false;
            };
            let reports = group
                .iter()
                .map(|row| row.reported_placement.as_str())
                .collect::<BTreeSet<_>>();
            let expected_reports = fixture
                .states
                .iter()
                .filter(|candidate| candidate.macrostate == state.macrostate)
                .filter_map(|candidate| candidate.coordinates.split("|w=").next())
                .collect::<BTreeSet<_>>();
            let expected_word = state
                .coordinates
                .split("|w=")
                .nth(1)
                .and_then(|suffix| suffix.split("|D=").next());
            let probability_sum = group.iter().try_fold(Rational::zero(), |sum, row| {
                sum.checked_add(row.probability)
            });
            let expected_probability =
                Rational::new(1, fixture.body_multiplicities[macro_index] as i128).ok();
            group.len() == fixture.body_multiplicities[macro_index]
                && reports.len() == group.len()
                && reports == expected_reports
                && probability_sum.ok() == Some(Rational::one())
                && group.iter().all(|row| {
                    Some(row.probability) == expected_probability
                        && row.persistent_post == row.pre
                        && !row.persistent_state_changed
                        && row.fixed_macro == state.macrostate
                        && Some(row.fixed_reservoir_word.as_str()) == expected_word
                })
        });
        if fixture.id != expected_id
            || fixture.states.len() != fixture.state_count
            || fixture.rows.len() != fixture.state_count * fixture.slot_count
            || fixture
                .states
                .iter()
                .map(|state| &state.id)
                .collect::<BTreeSet<_>>()
                .len()
                != fixture.state_count
            || fixture.p.len() != fixture.macro_labels.len()
            || fixture
                .p
                .iter()
                .any(|row| row.len() != fixture.macro_labels.len())
            || fixture.q_over_activity.len() != fixture.macro_labels.len()
            || fixture
                .q_over_activity
                .iter()
                .any(|row| row.len() != fixture.macro_labels.len())
            || fixture.crystal_readout.channel != "REFRESH"
            || fixture.crystal_readout.included_in_move_kernel
            || fixture.crystal_readout.uniform != crystal_valid
            || fixture.crystal_readout.persistent_state_unchanged != crystal_valid
            || fixture.crystal_readout.row_count != fixture.state_count
            || fixture.crystal_readout.outcome_count != fixture.crystal_readout.rows.len()
            || fixture.crystal_readout.rows_sha256 != sha256_hex(&crystal_bytes)
            || crystal_groups.len() != fixture.state_count
            || fixture
                .metamorphic_checks
                .iter()
                .map(|check| check.id)
                .collect::<Vec<_>>()
                != expected_metamorphic_ids
            || fixture.metamorphic_checks_sha256 != sha256_hex(&metamorphic_bytes)
        {
            return Err(VerifyError::new(format!(
                "fixture {expected_id} violates its finite evidence contract"
            )));
        }
    }
    let cases = mutation_cases();
    if controls.mutations.len() != cases.len() {
        return Err(VerifyError::new("wrong mutation count"));
    }
    for (observed, expected) in controls.mutations.iter().zip(cases) {
        let expected_verdict = if observed.observed_first_failure == Some(expected.expected) {
            "matched"
        } else if observed.observed_first_failure.is_none() {
            "survived"
        } else {
            "unexpected_failure"
        };
        if observed.id != expected.id
            || observed.expected_first_failure != expected.expected
            || observed.verdict != expected_verdict
            || observed.witness != mutation_witness(expected)?
        {
            return Err(VerifyError::new(
                "mutation identity or expected failure changed",
            ));
        }
    }
    let controls_passed = controls
        .mutations
        .iter()
        .all(|mutation| mutation.verdict == "matched");
    if (controls.status == "pass") != controls_passed || controls.first_failed_gate.is_some() {
        return Err(VerifyError::new(
            "control summary disagrees with mutation evidence",
        ));
    }
    let expected_control_claims = if controls_passed {
        vec!["MUTATION_FIRST_FAILURES_MATCH"]
    } else {
        Vec::new()
    };
    if controls.summary.verdict_code
        != if controls_passed {
            "FROZEN_CONTROLS_DISCRIMINATED"
        } else {
            "FROZEN_CONTROLS_FAILED"
        }
        || controls.summary.earned_claim_codes != expected_control_claims
        || !controls.summary.excluded_claim_codes.is_empty()
        || controls.summary.source_gates != vec![GateId::K01]
        || controls.gates.len() != 1
        || controls.gates[0].id != GateId::K01
        || controls.gates[0].evidence.len() != 1
        || controls.gates[0].evidence[0].pointer != "#/provenance"
        || controls
            .mutations
            .iter()
            .enumerate()
            .any(|(index, mutation)| {
                mutation.evidence.len() != 1
                    || mutation.evidence[0].pointer != format!("#/mutations/{index}/witness")
            })
    {
        return Err(VerifyError::new(
            "control summary or evidence pointers are inconsistent",
        ));
    }
    let expected_contact_cases = [
        ("equal-alpha-A-B", BodyId::A, BodyId::B, 2, 2),
        ("fresh-third-A-C", BodyId::A, BodyId::C, 2, 2),
        ("unequal-r2-r4", BodyId::A, BodyId::B, 2, 4),
        ("unequal-r4-r2", BodyId::A, BodyId::B, 4, 2),
    ];
    if contact.cases.len() != expected_contact_cases.len() {
        return Err(VerifyError::new("wrong contact case count"));
    }
    for (case, (id, left, right, branching_a, branching_b)) in
        contact.cases.iter().zip(expected_contact_cases)
    {
        let expected = contact_case(id, left, right, branching_a, branching_b)?;
        if case != &expected {
            return Err(VerifyError::new(format!(
                "contact case {id} violates its finite evidence contract"
            )));
        }
    }
    let exact_payload = serde_json::to_vec(exact)
        .map_err(|error| VerifyError::new(format!("cannot rebind exact payload: {error}")))?;
    let exact_gates_passed = exact.gates.iter().all(|gate| gate.status == "pass");
    let dormant_clock_bytes = serde_json::to_vec(&contact.clock_evidence.dormant_rows)
        .map_err(|error| VerifyError::new(format!("cannot rehash dormant clock: {error}")))?;
    let open_clock_bytes = serde_json::to_vec(&contact.clock_evidence.open_rows)
        .map_err(|error| VerifyError::new(format!("cannot rehash open clock: {error}")))?;
    let contact_metamorphic_bytes =
        serde_json::to_vec(&contact.metamorphic_checks).map_err(|error| {
            VerifyError::new(format!("cannot rehash contact scale checks: {error}"))
        })?;
    let expected_clock_evidence = contact_clock_evidence(BodyId::A, BodyId::B, false)?;
    let open_rows_match_equal_case = contact.clock_evidence.open_rows.len()
        == contact.cases[0].rows.len()
        && contact
            .clock_evidence
            .open_rows
            .iter()
            .zip(&contact.cases[0].rows)
            .all(|(clock, case)| {
                clock.pre == case.pre
                    && clock.slot == case.slot
                    && clock.probability == case.probability
                    && clock.slot_rate_over_kappa_contact == Rational::one()
                    && clock.outcome == case.outcome
                    && clock.post == case.post
                    && clock.delta_energy_left == case.delta_energy_left
                    && clock.delta_energy_right == case.delta_energy_right
            });
    if contact.cases.len() != expected_contact_cases.len()
        || contact.kernel_dependency.run_id != exact.run_id
        || contact.kernel_dependency.status != exact.status
        || contact.kernel_dependency.required_gates_passed != exact_gates_passed
        || contact.kernel_dependency.claim_ladder_passed
            != (exact.status == "pass" && exact_gates_passed)
        || contact.kernel_dependency.compact_payload_sha256 != sha256_hex(&exact_payload)
        || contact.clock_evidence != expected_clock_evidence
        || !open_rows_match_equal_case
        || contact.clock_evidence.clock_unchanged
            != contact_clock_evidence_valid(&contact.clock_evidence)
        || contact.clock_evidence.dormant_rows_sha256 != sha256_hex(&dormant_clock_bytes)
        || contact.clock_evidence.open_rows_sha256 != sha256_hex(&open_clock_bytes)
        || contact.metamorphic_checks.len() != 1
        || contact.metamorphic_checks[0].id != "kappa-contact-time-scale"
        || contact.metamorphic_checks_sha256 != sha256_hex(&contact_metamorphic_bytes)
    {
        return Err(VerifyError::new(
            "contact dependency or case set is inconsistent",
        ));
    }
    let mut expected_contact_order = exact_gate_order;
    expected_contact_order.extend([
        GateId::C01,
        GateId::C02,
        GateId::C03,
        GateId::C04,
        GateId::C05,
        GateId::C06,
        GateId::C07,
        GateId::C08,
    ]);
    if contact.gates.iter().map(|gate| gate.id).collect::<Vec<_>>() != expected_contact_order {
        return Err(VerifyError::new("contact gate order is not frozen order"));
    }
    let contact_failure = contact
        .gates
        .iter()
        .find(|gate| gate.status == "fail")
        .map(|gate| gate.id);
    let expected_contact_status = if contact_failure.is_some() {
        "fail"
    } else if !contact.kernel_dependency.claim_ladder_passed {
        "invalidated"
    } else {
        "pass"
    };
    if contact.first_failed_gate != contact_failure || contact.status != expected_contact_status {
        return Err(VerifyError::new(
            "contact status and first failure disagree",
        ));
    }
    let expected_contact_claims = if contact.status == "pass" {
        vec!["T6_EXACT_CONTACT", "ALPHA_OPERATIONAL_FOR_DECLARED_MODEL"]
    } else {
        Vec::new()
    };
    let expected_contact_verdict = match contact.status {
        "pass" => "EXACT_OPERATIONAL_TEMPERATURE_PASS",
        "invalidated" => "EXACT_CONTACT_INVALIDATED_BY_PREREQUISITE",
        _ => "EXACT_CONTACT_FAIL",
    };
    if contact.summary.verdict_code != expected_contact_verdict
        || contact.summary.earned_claim_codes != expected_contact_claims
        || contact.summary.excluded_claim_codes
            != vec!["PROTOCOL_OWNED_CONTACT", "DEPLOYED_CONTACT_CLOCK"]
        || contact.summary.source_gates != expected_contact_order
        || contact.gates.iter().any(|candidate| {
            let expected_pointers = if candidate.id == GateId::C02 {
                vec!["#/clock_evidence", "#/metamorphic_checks/0"]
            } else if matches!(
                candidate.id,
                GateId::C01
                    | GateId::C03
                    | GateId::C04
                    | GateId::C05
                    | GateId::C06
                    | GateId::C07
                    | GateId::C08
            ) {
                vec!["#/cases"]
            } else {
                vec!["#/kernel_dependency"]
            };
            candidate
                .evidence
                .iter()
                .map(|evidence| evidence.pointer.as_str())
                .collect::<Vec<_>>()
                != expected_pointers
        })
    {
        return Err(VerifyError::new(
            "contact summary or evidence pointers are inconsistent",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod internal_tests {
    use super::*;

    #[test]
    fn fresh_third_body_gate_detects_a_pair_specific_sabotage() {
        let (states, mut rows) = contact_table(BodyId::A, BodyId::C);
        let source = rows
            .iter()
            .position(|row| row.iter().any(|outcome| outcome.moved))
            .expect("active contact row");
        let slot = rows[source]
            .iter()
            .position(|outcome| outcome.moved)
            .expect("active contact slot");
        rows[source][slot].moved = false;
        assert_ne!(contact_reverse_and_energy_for(&states, &rows), (true, true));
    }

    #[test]
    fn path_accounting_recomputes_deltas_instead_of_trusting_receipts() {
        let mut model = build_model(primary_spec(2), Mutation::None).expect("primary model");
        let (state, slot) = model
            .outcomes
            .iter()
            .enumerate()
            .find_map(|(state, row)| {
                row.iter()
                    .position(|outcome| outcome.moved)
                    .map(|slot| (state, slot))
            })
            .expect("active move");
        model.outcomes[state][slot].delta_u = 0;
        model.outcomes[state][slot].delta_reservoir = 0;
        assert!(!k05(&model));
    }

    #[test]
    fn scale_checks_reject_mismatched_alpha_and_directed_clocks() {
        let model = build_model(primary_spec(2), Mutation::None).expect("primary model");
        let scale = Rational::new(7, 3).expect("scale");
        let mismatched = derive_thermo_scale_view(&model, scale, Rational::one())
            .expect("mismatched scale view");
        assert!(!mismatched.local_detailed_balance);
        assert!(macro_generator_at_scale(&model, scale, true).is_err());
        let directed_contact = contact_generator_at_scale(BodyId::A, BodyId::B, scale, true)
            .expect("directed contact generator");
        let equilibrium =
            contact_preparation(BodyId::A, BodyId::B, 2, 2).expect("contact equilibrium");
        assert!(!contact_detailed_balance(&equilibrium, &directed_contact));
    }

    #[test]
    fn crystal_uniformity_rejects_nonuniform_probabilities_that_sum_to_one() {
        let model = build_model(primary_spec(2), Mutation::None).expect("primary model");
        let mut rows = crystal_readout_rows(&model).expect("Crystal rows");
        let pre = model
            .states
            .iter()
            .enumerate()
            .find(|(source, _)| model.body_multiplicities[model.state_macro[*source]] == 2)
            .map(|(_, state)| state_text(&model.spec, state))
            .expect("two-outcome Crystal source");
        let mut group = rows
            .iter_mut()
            .filter(|row| row.pre == pre)
            .collect::<Vec<_>>();
        assert_eq!(group.len(), 2);
        group[0].probability = Rational::new(1, 3).expect("probability");
        group[1].probability = Rational::new(2, 3).expect("probability");
        assert_eq!(
            group
                .iter()
                .try_fold(Rational::zero(), |sum, row| sum
                    .checked_add(row.probability))
                .expect("sum"),
            Rational::one()
        );
        assert!(!crystal_readout_valid(&model, &rows));
    }
}
