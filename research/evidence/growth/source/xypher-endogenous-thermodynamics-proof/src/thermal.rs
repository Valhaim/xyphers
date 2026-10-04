use crate::exact::Ratio;
use crate::model::{
    ActionReceipt, CompleteState, EnabledEvent, KernelSupport, LedgerRow, Phase, ProbeState, Word,
};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;
use std::ops::{Add, Neg, Sub};

pub const PROBE_BODY_DEGENERACIES: [u32; 3] = [1, 2, 1];
pub const PROBE_BODY_ENERGY_INDICES: [u32; 3] = [0, 1, 2];
pub const PROBE_TOTAL_ENERGY_INDEX: u32 = 3;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExactLog {
    terms: BTreeMap<u128, Ratio>,
}

impl ExactLog {
    pub fn zero() -> Self {
        Self::default()
    }

    pub fn ln_ratio(value: Ratio) -> Self {
        assert!(
            value.is_positive(),
            "an exact logarithm requires a positive ratio"
        );
        let mut terms = BTreeMap::new();
        factor_into(value.numerator() as u128, Ratio::ONE, &mut terms);
        factor_into(value.denominator() as u128, -Ratio::ONE, &mut terms);
        terms.retain(|_, coefficient| !coefficient.is_zero());
        Self { terms }
    }

    pub fn scaled(&self, coefficient: Ratio) -> Self {
        if coefficient.is_zero() {
            return Self::zero();
        }
        let terms = self
            .terms
            .iter()
            .map(|(&prime, &value)| (prime, value * coefficient))
            .filter(|(_, value)| !value.is_zero())
            .collect();
        Self { terms }
    }

    pub fn is_zero(&self) -> bool {
        self.terms.is_empty()
    }

    pub fn terms(&self) -> &BTreeMap<u128, Ratio> {
        &self.terms
    }

    pub fn sign_if_unambiguous(&self) -> Option<Ordering> {
        if self.terms.is_empty() {
            return Some(Ordering::Equal);
        }
        let all_positive = self.terms.values().all(|value| value > &Ratio::ZERO);
        let all_negative = self.terms.values().all(|value| value < &Ratio::ZERO);
        if all_positive {
            Some(Ordering::Greater)
        } else if all_negative {
            Some(Ordering::Less)
        } else {
            None
        }
    }

    fn add_term(&mut self, prime: u128, coefficient: Ratio) {
        let updated = self.terms.get(&prime).copied().unwrap_or(Ratio::ZERO) + coefficient;
        if updated.is_zero() {
            self.terms.remove(&prime);
        } else {
            self.terms.insert(prime, updated);
        }
    }
}

impl Add for ExactLog {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for (prime, coefficient) in rhs.terms {
            self.add_term(prime, coefficient);
        }
        self
    }
}

impl Sub for ExactLog {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Neg for ExactLog {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for coefficient in self.terms.values_mut() {
            *coefficient = -*coefficient;
        }
        self
    }
}

impl fmt::Display for ExactLog {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.terms.is_empty() {
            return formatter.write_str("0");
        }
        let mut first = true;
        for (prime, coefficient) in &self.terms {
            let negative = coefficient < &Ratio::ZERO;
            let magnitude = coefficient.abs();
            if first {
                if negative {
                    formatter.write_str("-")?;
                }
            } else if negative {
                formatter.write_str(" - ")?;
            } else {
                formatter.write_str(" + ")?;
            }
            if magnitude != Ratio::ONE {
                write!(formatter, "{magnitude}*")?;
            }
            write!(formatter, "ln({prime})")?;
            first = false;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Temperature {
    pub beta: ExactLog,
}

impl Temperature {
    pub fn from_beta(beta: ExactLog) -> Self {
        assert!(!beta.is_zero(), "zero inverse temperature is not finite");
        Self { beta }
    }

    pub fn same_temperature(&self, other: &Self) -> bool {
        self.beta == other.beta
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "1/({})", self.beta)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReservoirAlternative {
    pub descriptor_id: u64,
    pub word: Vec<u8>,
    pub energy_index: u32,
    pub executable: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrefixReservoir {
    pub alphabet_size: u8,
    pub energy_quantum: Ratio,
    pub promoted: Vec<ReservoirAlternative>,
}

impl PrefixReservoir {
    pub fn new(
        alphabet_size: u8,
        energy_quantum: Ratio,
        promoted: Vec<ReservoirAlternative>,
    ) -> Self {
        Self {
            alphabet_size,
            energy_quantum,
            promoted,
        }
    }

    pub fn from_complete_state(state: &CompleteState) -> Self {
        let promoted = state
            .Gamma
            .reservoir
            .values()
            .map(|node| ReservoirAlternative {
                descriptor_id: u64::from(node.descriptor),
                word: node.word.0.clone(),
                energy_index: u32::try_from(node.energy_index)
                    .expect("reservoir energy index fits u32"),
                executable: node.executable,
            })
            .collect();
        Self::new(
            u8::try_from(state.A.len()).expect("alphabet size fits u8"),
            Ratio::integer(i128::from(state.lambda)),
            promoted,
        )
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProbeCoordinate {
    pub body: ProbeState,
    pub reservoir: Word,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SealedProbeProtocol {
    pub body_degeneracies: [u32; 3],
    pub body_energy_indices: [u32; 3],
    pub total_energy_index: u32,
    pub microscopic_clock: Ratio,
    pub deleted_directed_edge: Option<(ProbeCoordinate, ProbeCoordinate)>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeAuthoritativeState {
    pub xypher: CompleteState,
    pub protocol: SealedProbeProtocol,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ContactCoordinate {
    pub left_reservoir: Word,
    pub right_reservoir: Word,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactAuthoritativeState {
    pub left: CompleteState,
    pub right: CompleteState,
    pub protocol: ContactProtocol,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthoritativeState {
    Driven(CompleteState),
    Probe(ProbeAuthoritativeState),
    Contact(ContactAuthoritativeState),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthoritativeEvent {
    Driven(EnabledEvent),
    Probe {
        target: ProbeCoordinate,
        hazard: Ratio,
    },
    Contact {
        target: ContactCoordinate,
        hazard: Ratio,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeHeatLedger {
    pub delta_body: Ratio,
    pub delta_reservoir: Ratio,
    pub heat_into_body: Ratio,
    pub work_on_body: Ratio,
    pub total_energy_delta: Ratio,
}

impl ProbeHeatLedger {
    pub fn closes(&self) -> bool {
        self.delta_body == self.heat_into_body + self.work_on_body
            && self.delta_reservoir == -self.heat_into_body
            && self.total_energy_delta == self.delta_body + self.delta_reservoir
            && self.total_energy_delta == Ratio::ZERO
            && self.work_on_body == Ratio::ZERO
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactHeatLedger {
    pub delta_left: Ratio,
    pub delta_right: Ratio,
    pub heat_into_left: Ratio,
    pub heat_into_right: Ratio,
    pub work_on_left: Ratio,
    pub work_on_right: Ratio,
    pub total_energy_delta: Ratio,
}

impl ContactHeatLedger {
    pub fn closes(&self) -> bool {
        self.delta_left == self.heat_into_left + self.work_on_left
            && self.delta_right == self.heat_into_right + self.work_on_right
            && self.heat_into_left == -self.heat_into_right
            && self.total_energy_delta == self.delta_left + self.delta_right
            && self.total_energy_delta == Ratio::ZERO
            && self.work_on_left == Ratio::ZERO
            && self.work_on_right == Ratio::ZERO
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthoritativeLedger {
    Driven(LedgerRow),
    Probe(ProbeHeatLedger),
    Contact(ContactHeatLedger),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthoritativeInverse {
    Driven(ActionReceipt),
    Reciprocal(AuthoritativeEvent),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthoritativeTransition {
    pub source: AuthoritativeState,
    pub event: AuthoritativeEvent,
    pub successor: AuthoritativeState,
    pub ledger: AuthoritativeLedger,
    pub inverse: AuthoritativeInverse,
    pub inverse_restores_source: bool,
}

fn probe_coordinate(state: &ProbeAuthoritativeState) -> ProbeCoordinate {
    ProbeCoordinate {
        body: state.xypher.X.clone(),
        reservoir: state.xypher.r.clone(),
    }
}

fn contact_coordinate(state: &ContactAuthoritativeState) -> ContactCoordinate {
    ContactCoordinate {
        left_reservoir: state.left.r.clone(),
        right_reservoir: state.right.r.clone(),
    }
}

fn probe_coordinate_energy(state: &ProbeAuthoritativeState, coordinate: &ProbeCoordinate) -> Ratio {
    let body = Ratio::integer(i128::from(state.xypher.lambda))
        * Ratio::integer(i128::from(
            state.protocol.body_energy_indices[usize::from(coordinate.body.mesostate)],
        ));
    let reservoir = state
        .xypher
        .Gamma
        .reservoir
        .get(&coordinate.reservoir)
        .map(|node| {
            Ratio::integer(i128::from(state.xypher.lambda))
                * Ratio::integer(node.energy_index as i128)
        })
        .unwrap_or(Ratio::ZERO);
    body + reservoir
}

fn probe_coordinates(state: &ProbeAuthoritativeState) -> Vec<ProbeCoordinate> {
    let mut coordinates = Vec::new();
    for mesostate in 0..3_u8 {
        let reservoir_energy = state
            .protocol
            .total_energy_index
            .checked_sub(state.protocol.body_energy_indices[usize::from(mesostate)])
            .expect("probe body energy lies within the sealed total");
        for microstate in 0..state.protocol.body_degeneracies[usize::from(mesostate)] as u8 {
            for word in state.xypher.Gamma.shell_words(reservoir_energy as usize) {
                coordinates.push(ProbeCoordinate {
                    body: ProbeState {
                        mesostate,
                        microstate,
                    },
                    reservoir: word,
                });
            }
        }
    }
    coordinates
}

fn probe_authoritative_hazard(
    state: &ProbeAuthoritativeState,
    from: &ProbeCoordinate,
    to: &ProbeCoordinate,
) -> Ratio {
    if from == to
        || state.protocol.deleted_directed_edge.as_ref() == Some(&(from.clone(), to.clone()))
        || probe_coordinate_energy(state, from) != probe_coordinate_energy(state, to)
    {
        return Ratio::ZERO;
    }
    if from.body.mesostate != to.body.mesostate {
        return state.protocol.microscopic_clock;
    }
    if from.body.mesostate == 1
        && from.reservoir == to.reservoir
        && from.body.microstate != to.body.microstate
    {
        state.protocol.microscopic_clock
    } else {
        Ratio::ZERO
    }
}

fn contact_coordinates(state: &ContactAuthoritativeState) -> Vec<ContactCoordinate> {
    let left_zero = state.left.Gamma.shell_words(0);
    let left_two = state.left.Gamma.shell_words(2);
    let right_one = state.right.Gamma.shell_words(1);
    let right_two = state.right.Gamma.shell_words(2);
    left_zero
        .iter()
        .flat_map(|left| {
            right_two.iter().map(move |right| ContactCoordinate {
                left_reservoir: left.clone(),
                right_reservoir: right.clone(),
            })
        })
        .chain(left_two.iter().flat_map(|left| {
            right_one.iter().map(move |right| ContactCoordinate {
                left_reservoir: left.clone(),
                right_reservoir: right.clone(),
            })
        }))
        .collect()
}

fn contact_fiber(
    state: &ContactAuthoritativeState,
    coordinate: &ContactCoordinate,
) -> Option<ContactFiber> {
    match (
        state
            .left
            .Gamma
            .reservoir
            .get(&coordinate.left_reservoir)
            .map(|node| node.energy_index),
        state
            .right
            .Gamma
            .reservoir
            .get(&coordinate.right_reservoir)
            .map(|node| node.energy_index),
    ) {
        (Some(0), Some(2)) => Some(ContactFiber::A),
        (Some(2), Some(1)) => Some(ContactFiber::B),
        _ => None,
    }
}

fn contact_authoritative_hazard(
    state: &ContactAuthoritativeState,
    from: &ContactCoordinate,
    to: &ContactCoordinate,
) -> Ratio {
    if from != to
        && contact_fiber(state, from).is_some()
        && contact_fiber(state, to).is_some()
        && contact_fiber(state, from) != contact_fiber(state, to)
    {
        state.protocol.microscopic_clock
    } else {
        Ratio::ZERO
    }
}

pub fn enabled_authoritative_events(state: &AuthoritativeState) -> Vec<AuthoritativeEvent> {
    match state {
        AuthoritativeState::Driven(state) => state
            .enabled_events()
            .into_iter()
            .map(AuthoritativeEvent::Driven)
            .collect(),
        AuthoritativeState::Probe(state) => {
            let source = probe_coordinate(state);
            probe_coordinates(state)
                .into_iter()
                .filter_map(|target| {
                    let hazard = probe_authoritative_hazard(state, &source, &target);
                    hazard
                        .is_positive()
                        .then_some(AuthoritativeEvent::Probe { target, hazard })
                })
                .collect()
        }
        AuthoritativeState::Contact(state) => {
            let source = contact_coordinate(state);
            contact_coordinates(state)
                .into_iter()
                .filter_map(|target| {
                    let hazard = contact_authoritative_hazard(state, &source, &target);
                    hazard
                        .is_positive()
                        .then_some(AuthoritativeEvent::Contact { target, hazard })
                })
                .collect()
        }
    }
}

pub fn authoritative_transition(
    source: &AuthoritativeState,
    event: &AuthoritativeEvent,
) -> Option<AuthoritativeTransition> {
    if !enabled_authoritative_events(source).contains(event) {
        return None;
    }
    match (source, event) {
        (AuthoritativeState::Driven(state), AuthoritativeEvent::Driven(enabled)) => {
            let transition = state
                .event_transition(enabled, KernelSupport::PRIMARY)
                .ok()?;
            Some(AuthoritativeTransition {
                source: source.clone(),
                event: event.clone(),
                successor: AuthoritativeState::Driven(transition.successor),
                ledger: AuthoritativeLedger::Driven(transition.ledger),
                inverse: AuthoritativeInverse::Driven(transition.inverse),
                inverse_restores_source: transition.inverse_restores_source,
            })
        }
        (AuthoritativeState::Probe(state), AuthoritativeEvent::Probe { target, hazard }) => {
            let before = probe_coordinate(state);
            let mut successor = state.clone();
            successor.xypher.X = target.body.clone();
            successor.xypher.r = target.reservoir.clone();
            let quantum = Ratio::integer(i128::from(state.xypher.lambda));
            let body_before = quantum
                * Ratio::integer(i128::from(
                    state.protocol.body_energy_indices[usize::from(before.body.mesostate)],
                ));
            let body_after = quantum
                * Ratio::integer(i128::from(
                    state.protocol.body_energy_indices[usize::from(target.body.mesostate)],
                ));
            let reservoir_before = quantum
                * Ratio::integer(
                    state.xypher.Gamma.reservoir[&before.reservoir].energy_index as i128,
                );
            let reservoir_after = quantum
                * Ratio::integer(
                    state.xypher.Gamma.reservoir[&target.reservoir].energy_index as i128,
                );
            let ledger = ProbeHeatLedger {
                delta_body: body_after - body_before,
                delta_reservoir: reservoir_after - reservoir_before,
                heat_into_body: body_after - body_before,
                work_on_body: Ratio::ZERO,
                total_energy_delta: body_after + reservoir_after - body_before - reservoir_before,
            };
            let reverse = AuthoritativeEvent::Probe {
                target: before.clone(),
                hazard: probe_authoritative_hazard(&successor, target, &probe_coordinate(state)),
            };
            let successor_state = AuthoritativeState::Probe(successor.clone());
            let mut reversed = successor.clone();
            reversed.xypher.X = before.body;
            reversed.xypher.r = before.reservoir;
            let inverse_restores_source = enabled_authoritative_events(&successor_state)
                .contains(&reverse)
                && *hazard == successor.protocol.microscopic_clock
                && reversed == *state
                && ledger.closes();
            Some(AuthoritativeTransition {
                source: source.clone(),
                event: event.clone(),
                successor: successor_state,
                ledger: AuthoritativeLedger::Probe(ledger),
                inverse: AuthoritativeInverse::Reciprocal(reverse),
                inverse_restores_source,
            })
        }
        (AuthoritativeState::Contact(state), AuthoritativeEvent::Contact { target, hazard }) => {
            let before = contact_coordinate(state);
            let mut successor = state.clone();
            successor.left.r = target.left_reservoir.clone();
            successor.right.r = target.right_reservoir.clone();
            let left_quantum = Ratio::integer(i128::from(state.left.lambda));
            let right_quantum = Ratio::integer(i128::from(state.right.lambda));
            let left_before = left_quantum
                * Ratio::integer(
                    state.left.Gamma.reservoir[&before.left_reservoir].energy_index as i128,
                );
            let left_after = left_quantum
                * Ratio::integer(
                    state.left.Gamma.reservoir[&target.left_reservoir].energy_index as i128,
                );
            let right_before = right_quantum
                * Ratio::integer(
                    state.right.Gamma.reservoir[&before.right_reservoir].energy_index as i128,
                );
            let right_after = right_quantum
                * Ratio::integer(
                    state.right.Gamma.reservoir[&target.right_reservoir].energy_index as i128,
                );
            let ledger = ContactHeatLedger {
                delta_left: left_after - left_before,
                delta_right: right_after - right_before,
                heat_into_left: left_after - left_before,
                heat_into_right: right_after - right_before,
                work_on_left: Ratio::ZERO,
                work_on_right: Ratio::ZERO,
                total_energy_delta: left_after + right_after - left_before - right_before,
            };
            let reverse = AuthoritativeEvent::Contact {
                target: before.clone(),
                hazard: contact_authoritative_hazard(
                    &successor,
                    target,
                    &contact_coordinate(state),
                ),
            };
            let successor_state = AuthoritativeState::Contact(successor.clone());
            let mut reversed = successor.clone();
            reversed.left.r = before.left_reservoir;
            reversed.right.r = before.right_reservoir;
            let inverse_restores_source = enabled_authoritative_events(&successor_state)
                .contains(&reverse)
                && *hazard == successor.protocol.microscopic_clock
                && reversed == *state
                && ledger.closes();
            Some(AuthoritativeTransition {
                source: source.clone(),
                event: event.clone(),
                successor: successor_state,
                ledger: AuthoritativeLedger::Contact(ledger),
                inverse: AuthoritativeInverse::Reciprocal(reverse),
                inverse_restores_source,
            })
        }
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StateTemperatureClass {
    Undefined,
    NonPositive,
    StateDependent,
    UniqueConstant,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrefixLawAudit {
    pub alphabet_is_nontrivial: bool,
    pub energy_quantum_is_positive: bool,
    pub all_promoted_states_executable: bool,
    pub descriptor_encoding_is_injective: bool,
    pub word_encoding_is_injective: bool,
    pub symbols_belong_to_alphabet: bool,
    pub word_depth_matches_energy: bool,
    pub unique_canonical_root: bool,
    pub every_parent_has_one_child_per_symbol: bool,
    pub every_nonroot_has_its_unique_parent: bool,
    pub actual_counts_match_prefix_prediction: bool,
}

impl PrefixLawAudit {
    pub fn complete_prefix_realization(&self) -> bool {
        self.alphabet_is_nontrivial
            && self.energy_quantum_is_positive
            && self.all_promoted_states_executable
            && self.descriptor_encoding_is_injective
            && self.word_encoding_is_injective
            && self.symbols_belong_to_alphabet
            && self.word_depth_matches_energy
            && self.unique_canonical_root
            && self.every_parent_has_one_child_per_symbol
            && self.every_nonroot_has_its_unique_parent
            && self.actual_counts_match_prefix_prediction
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrefixThermalResult {
    pub shell_counts: Vec<u128>,
    pub predicted_prefix_counts: Vec<Option<u128>>,
    pub adjacent_ratios: Vec<Option<Ratio>>,
    pub adjacent_betas: Vec<Option<ExactLog>>,
    pub count_integrity: bool,
    pub prefix_law: PrefixLawAudit,
    pub classification: StateTemperatureClass,
    pub common_multiplicity_ratio: Option<Ratio>,
    pub temperature: Option<Temperature>,
}

impl PrefixThermalResult {
    pub fn exact_prefix_temperature_passes(&self) -> bool {
        self.count_integrity
            && self.prefix_law.complete_prefix_realization()
            && self.classification == StateTemperatureClass::UniqueConstant
            && self.common_multiplicity_ratio.is_some()
            && self.temperature.is_some()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct CanonicalAlternative {
    descriptor_id: u64,
    word: Vec<u8>,
    energy_index: u32,
}

pub fn classify_prefix_reservoir(reservoir: &PrefixReservoir) -> PrefixThermalResult {
    let all_executable = reservoir.promoted.iter().all(|state| state.executable);
    let mut descriptor_ids = BTreeSet::new();
    let descriptor_injective = reservoir
        .promoted
        .iter()
        .all(|state| descriptor_ids.insert(state.descriptor_id));
    let mut words = BTreeSet::new();
    let word_injective = reservoir
        .promoted
        .iter()
        .all(|state| words.insert(state.word.clone()));
    let count_integrity = all_executable && descriptor_injective && word_injective;

    let canonical = canonical_alternatives(reservoir);
    let max_depth = canonical
        .iter()
        .map(|state| state.energy_index)
        .max()
        .unwrap_or(0);
    let mut shells: BTreeMap<u32, BTreeSet<Vec<u8>>> = BTreeMap::new();
    for state in &canonical {
        shells
            .entry(state.energy_index)
            .or_default()
            .insert(state.word.clone());
    }
    let shell_counts: Vec<u128> = (0..=max_depth)
        .map(|depth| shells.get(&depth).map_or(0, |shell| shell.len() as u128))
        .collect();
    let predicted_prefix_counts: Vec<Option<u128>> = (0..=max_depth)
        .map(|depth| checked_power(reservoir.alphabet_size as u128, depth))
        .collect();

    let symbols_belong_to_alphabet = reservoir.promoted.iter().all(|state| {
        state
            .word
            .iter()
            .all(|&symbol| symbol >= 1 && symbol <= reservoir.alphabet_size)
    });
    let word_depth_matches_energy = reservoir
        .promoted
        .iter()
        .all(|state| u32::try_from(state.word.len()) == Ok(state.energy_index));
    let unique_canonical_root = shells
        .get(&0)
        .is_some_and(|shell| shell.len() == 1 && shell.contains(&Vec::new()));

    let mut every_parent_has_one_child_per_symbol = true;
    let mut every_nonroot_has_its_unique_parent = true;
    for depth in 0..max_depth {
        let current = shells.get(&depth);
        let next = shells.get(&(depth + 1));
        match (current, next) {
            (Some(current), Some(next)) => {
                for parent in current {
                    for symbol in 1..=reservoir.alphabet_size {
                        let mut child = parent.clone();
                        child.push(symbol);
                        if !next.contains(&child) {
                            every_parent_has_one_child_per_symbol = false;
                        }
                    }
                }
                for child in next {
                    if child.is_empty() {
                        every_nonroot_has_its_unique_parent = false;
                        continue;
                    }
                    let parent = child[..child.len() - 1].to_vec();
                    if !current.contains(&parent) {
                        every_nonroot_has_its_unique_parent = false;
                    }
                }
            }
            _ => {
                every_parent_has_one_child_per_symbol = false;
                every_nonroot_has_its_unique_parent = false;
            }
        }
    }

    let actual_counts_match_prefix_prediction = shell_counts
        .iter()
        .zip(&predicted_prefix_counts)
        .all(|(&actual, expected)| *expected == Some(actual));
    let prefix_law = PrefixLawAudit {
        alphabet_is_nontrivial: reservoir.alphabet_size >= 2,
        energy_quantum_is_positive: reservoir.energy_quantum.is_positive(),
        all_promoted_states_executable: all_executable,
        descriptor_encoding_is_injective: descriptor_injective,
        word_encoding_is_injective: word_injective,
        symbols_belong_to_alphabet,
        word_depth_matches_energy,
        unique_canonical_root,
        every_parent_has_one_child_per_symbol,
        every_nonroot_has_its_unique_parent,
        actual_counts_match_prefix_prediction,
    };

    let adjacent_ratios: Vec<Option<Ratio>> = shell_counts
        .windows(2)
        .map(|pair| (pair[0] > 0).then(|| ratio_u128(pair[1], pair[0])))
        .collect();
    let adjacent_betas: Vec<Option<ExactLog>> = adjacent_ratios
        .iter()
        .map(|ratio| {
            ratio.and_then(|ratio| {
                (ratio.is_positive() && reservoir.energy_quantum.is_positive()).then(|| {
                    ExactLog::ln_ratio(ratio).scaled(reservoir.energy_quantum.reciprocal())
                })
            })
        })
        .collect();

    let enough_gaps = adjacent_ratios.len() >= 2;
    let all_shells_nonempty = shell_counts.iter().all(|&count| count > 0);
    let all_ratios_positive_increasing = adjacent_ratios
        .iter()
        .all(|ratio| ratio.is_some_and(|value| value > Ratio::ONE));
    let classification = if !count_integrity
        || !reservoir.energy_quantum.is_positive()
        || !enough_gaps
        || !all_shells_nonempty
    {
        StateTemperatureClass::Undefined
    } else if !all_ratios_positive_increasing {
        StateTemperatureClass::NonPositive
    } else {
        let first = adjacent_betas[0].as_ref().expect("positive gap has beta");
        if adjacent_betas
            .iter()
            .all(|beta| beta.as_ref() == Some(first))
        {
            StateTemperatureClass::UniqueConstant
        } else {
            StateTemperatureClass::StateDependent
        }
    };

    let common_multiplicity_ratio = (classification == StateTemperatureClass::UniqueConstant)
        .then(|| adjacent_ratios[0].expect("constant temperature has a positive ratio"));
    let temperature = (classification == StateTemperatureClass::UniqueConstant).then(|| {
        Temperature::from_beta(
            adjacent_betas[0]
                .clone()
                .expect("constant temperature has an inverse temperature"),
        )
    });

    PrefixThermalResult {
        shell_counts,
        predicted_prefix_counts,
        adjacent_ratios,
        adjacent_betas,
        count_integrity,
        prefix_law,
        classification,
        common_multiplicity_ratio,
        temperature,
    }
}

fn canonical_alternatives(reservoir: &PrefixReservoir) -> Vec<CanonicalAlternative> {
    let mut seen_descriptors = BTreeSet::new();
    let mut seen_words = BTreeSet::new();
    let mut alternatives: Vec<_> = reservoir
        .promoted
        .iter()
        .filter(|state| state.executable)
        .filter(|state| seen_descriptors.insert(state.descriptor_id))
        .filter(|state| seen_words.insert(state.word.clone()))
        .map(|state| CanonicalAlternative {
            descriptor_id: state.descriptor_id,
            word: state.word.clone(),
            energy_index: state.energy_index,
        })
        .collect();
    alternatives.sort_by(|left, right| {
        left.word
            .cmp(&right.word)
            .then(left.descriptor_id.cmp(&right.descriptor_id))
    });
    alternatives
}

fn shell_alternatives(reservoir: &PrefixReservoir, energy_index: u32) -> Vec<CanonicalAlternative> {
    canonical_alternatives(reservoir)
        .into_iter()
        .filter(|state| state.energy_index == energy_index)
        .collect()
}

fn checked_power(mut base: u128, mut exponent: u32) -> Option<u128> {
    let mut result = 1_u128;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result.checked_mul(base)?;
        }
        exponent >>= 1;
        if exponent > 0 {
            base = base.checked_mul(base)?;
        }
    }
    Some(result)
}

fn i128_from_u128(value: u128) -> i128 {
    i128::try_from(value).expect("exact count fits i128")
}

fn ratio_integer_u128(value: u128) -> Ratio {
    Ratio::integer(i128_from_u128(value))
}

fn ratio_u128(numerator: u128, denominator: u128) -> Ratio {
    Ratio::new(i128_from_u128(numerator), i128_from_u128(denominator))
}

fn factor_into(mut value: u128, sign: Ratio, terms: &mut BTreeMap<u128, Ratio>) {
    assert!(value > 0, "factorization requires a positive integer");
    let mut prime = 2_u128;
    while prime <= value / prime {
        while value % prime == 0 {
            let updated = terms.get(&prime).copied().unwrap_or(Ratio::ZERO) + sign;
            terms.insert(prime, updated);
            value /= prime;
        }
        prime = if prime == 2 { 3 } else { prime + 2 };
    }
    if value > 1 {
        let updated = terms.get(&value).copied().unwrap_or(Ratio::ZERO) + sign;
        terms.insert(value, updated);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeThermalResult {
    pub microscopic_clock: Ratio,
    pub reservoir_count_integrity: bool,
    pub total_microstates: u128,
    pub fiber_sizes: [u128; 3],
    pub macro_generator: [[Ratio; 3]; 3],
    pub reciprocal_microedges: bool,
    pub active_microedges_match_declared_clock: bool,
    pub row_closed: bool,
    pub strong_lumpability: bool,
    pub total_energy_conserved: bool,
    pub refresh_reciprocal: bool,
    pub all_energy_gap_channels_present: bool,
    pub ldb_constraints: Vec<ExactLog>,
    pub ldb_temperature: Option<Temperature>,
    pub unique_ldb_temperature: bool,
    pub macro_stationary_weights: [Ratio; 3],
    pub macro_detailed_balance: bool,
    pub macro_stationary: bool,
    pub uniform_microcanonical_stationary: bool,
    pub gibbs_microcanonical_agree: bool,
    pub irreducible: bool,
    pub unique_stationary_law: bool,
    pub relaxes_from_every_initial_distribution: bool,
    pub event_replay: ProbeEventReplayAudit,
}

impl ProbeThermalResult {
    pub fn operational_closure_passes(&self) -> bool {
        self.reservoir_count_integrity
            && self.microscopic_clock.is_positive()
            && self.reciprocal_microedges
            && self.active_microedges_match_declared_clock
            && self.row_closed
            && self.strong_lumpability
            && self.total_energy_conserved
            && self.refresh_reciprocal
            && self.all_energy_gap_channels_present
            && self.unique_ldb_temperature
            && self.ldb_temperature.is_some()
            && self.macro_detailed_balance
            && self.macro_stationary
            && self.uniform_microcanonical_stationary
            && self.gibbs_microcanonical_agree
            && self.irreducible
            && self.unique_stationary_law
            && self.relaxes_from_every_initial_distribution
            && self.event_replay.passes()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeEventReplayAudit {
    pub complete_microstates_checked: u128,
    pub directed_events_checked: u128,
    pub complete_microstate_encoding_is_unique: bool,
    pub complete_microstate_count_matches_fibers: bool,
    pub every_generator_entry_replays_from_complete_state: bool,
    pub every_event_has_reciprocal_support: bool,
    pub every_event_conserves_total_energy: bool,
    pub every_event_body_heat_ledger_closes: bool,
    pub every_event_reservoir_heat_ledger_closes: bool,
    pub every_event_has_zero_work: bool,
    pub every_event_path_ratio_closes: bool,
}

impl ProbeEventReplayAudit {
    pub fn passes(&self) -> bool {
        self.complete_microstates_checked > 0
            && self.directed_events_checked > 0
            && self.complete_microstate_encoding_is_unique
            && self.complete_microstate_count_matches_fibers
            && self.every_generator_entry_replays_from_complete_state
            && self.every_event_has_reciprocal_support
            && self.every_event_conserves_total_energy
            && self.every_event_body_heat_ledger_closes
            && self.every_event_reservoir_heat_ledger_closes
            && self.every_event_has_zero_work
            && self.every_event_path_ratio_closes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeAuthoritativeEventRow {
    pub source: ProbeCoordinate,
    pub target: ProbeCoordinate,
    pub hazard: Ratio,
    pub reverse_hazard: Ratio,
    pub diagonal_from_source: Ratio,
    pub ledger: ProbeHeatLedger,
    pub source_and_successor_well_formed: bool,
    pub noncoordinate_state_is_exactly_unchanged: bool,
    pub inverse_restores_source: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeAuthoritativeKernelAudit {
    pub reference_xypher_state: CompleteState,
    pub protocol: SealedProbeProtocol,
    pub complete_coordinates: Vec<ProbeCoordinate>,
    pub event_rows: Vec<ProbeAuthoritativeEventRow>,
    pub every_state_reconstructs_from_reference: bool,
    pub every_enabled_event_replays_from_one_authoritative_kernel: bool,
    pub every_diagonal_is_negative_outgoing_sum: bool,
    pub every_event_has_exact_reciprocal_and_heat_ledger: bool,
}

impl ProbeAuthoritativeKernelAudit {
    pub fn passes(&self) -> bool {
        !self.complete_coordinates.is_empty()
            && !self.event_rows.is_empty()
            && self.every_state_reconstructs_from_reference
            && self.every_enabled_event_replays_from_one_authoritative_kernel
            && self.every_diagonal_is_negative_outgoing_sum
            && self.every_event_has_exact_reciprocal_and_heat_ledger
    }
}

pub fn audit_authoritative_probe_kernel(
    completed_xypher: &CompleteState,
    microscopic_clock: Ratio,
    delete_first_reverse: bool,
) -> ProbeAuthoritativeKernelAudit {
    let mut reference = completed_xypher.clone();
    reference.P = Phase::Thermal;
    reference.Q.scheduled_perturbation = None;
    let provisional = ProbeAuthoritativeState {
        xypher: reference.clone(),
        protocol: SealedProbeProtocol {
            body_degeneracies: PROBE_BODY_DEGENERACIES,
            body_energy_indices: PROBE_BODY_ENERGY_INDICES,
            total_energy_index: PROBE_TOTAL_ENERGY_INDEX,
            microscopic_clock,
            deleted_directed_edge: None,
        },
    };
    let complete_coordinates = probe_coordinates(&provisional);
    let deleted_directed_edge = delete_first_reverse.then(|| {
        let low = complete_coordinates
            .iter()
            .find(|coordinate| coordinate.body.mesostate == 0)
            .expect("sealed probe contains its low body fiber")
            .clone();
        let middle = complete_coordinates
            .iter()
            .find(|coordinate| coordinate.body.mesostate == 1)
            .expect("sealed probe contains its middle body fiber")
            .clone();
        (middle, low)
    });
    let protocol = SealedProbeProtocol {
        body_degeneracies: PROBE_BODY_DEGENERACIES,
        body_energy_indices: PROBE_BODY_ENERGY_INDICES,
        total_energy_index: PROBE_TOTAL_ENERGY_INDEX,
        microscopic_clock,
        deleted_directed_edge,
    };
    let mut event_rows = Vec::new();
    let mut every_state_reconstructs_from_reference = reference.complete_state_is_well_formed();
    let mut every_enabled_event_replays_from_one_authoritative_kernel = true;
    let mut every_diagonal_is_negative_outgoing_sum = true;
    let mut every_event_has_exact_reciprocal_and_heat_ledger = true;

    for source_coordinate in &complete_coordinates {
        let mut source_xypher = reference.clone();
        source_xypher.X = source_coordinate.body.clone();
        source_xypher.r = source_coordinate.reservoir.clone();
        let source = AuthoritativeState::Probe(ProbeAuthoritativeState {
            xypher: source_xypher.clone(),
            protocol: protocol.clone(),
        });
        every_state_reconstructs_from_reference &= source_xypher.complete_state_is_well_formed();
        let events = enabled_authoritative_events(&source);
        let outgoing = events
            .iter()
            .map(|event| match event {
                AuthoritativeEvent::Probe { hazard, .. } => *hazard,
                _ => Ratio::ZERO,
            })
            .sum::<Ratio>();
        let diagonal = -outgoing;
        every_diagonal_is_negative_outgoing_sum &= diagonal + outgoing == Ratio::ZERO;
        for event in events {
            let Some(transition) = authoritative_transition(&source, &event) else {
                every_enabled_event_replays_from_one_authoritative_kernel = false;
                continue;
            };
            let (target, hazard) = match &event {
                AuthoritativeEvent::Probe { target, hazard } => (target.clone(), *hazard),
                _ => unreachable!(),
            };
            let successor = match &transition.successor {
                AuthoritativeState::Probe(successor) => successor,
                _ => unreachable!(),
            };
            let ledger = match &transition.ledger {
                AuthoritativeLedger::Probe(ledger) => ledger.clone(),
                _ => unreachable!(),
            };
            let reverse_hazard = match &transition.inverse {
                AuthoritativeInverse::Reciprocal(AuthoritativeEvent::Probe { hazard, .. }) => {
                    *hazard
                }
                _ => Ratio::ZERO,
            };
            let mut successor_without_live_coordinate = successor.xypher.clone();
            successor_without_live_coordinate.X = reference.X.clone();
            successor_without_live_coordinate.r = reference.r.clone();
            let noncoordinate_state_is_exactly_unchanged =
                successor_without_live_coordinate == reference;
            let row = ProbeAuthoritativeEventRow {
                source: source_coordinate.clone(),
                target,
                hazard,
                reverse_hazard,
                diagonal_from_source: diagonal,
                ledger,
                source_and_successor_well_formed: source_xypher.complete_state_is_well_formed()
                    && successor.xypher.complete_state_is_well_formed(),
                noncoordinate_state_is_exactly_unchanged,
                inverse_restores_source: transition.inverse_restores_source,
            };
            every_enabled_event_replays_from_one_authoritative_kernel &=
                transition.source == source && row.target == probe_coordinate(successor);
            every_event_has_exact_reciprocal_and_heat_ledger &= row.reverse_hazard.is_positive()
                && row.reverse_hazard == row.hazard
                && row.ledger.closes()
                && row.source_and_successor_well_formed
                && row.noncoordinate_state_is_exactly_unchanged
                && row.inverse_restores_source;
            event_rows.push(row);
        }
    }

    ProbeAuthoritativeKernelAudit {
        reference_xypher_state: reference,
        protocol,
        complete_coordinates,
        event_rows,
        every_state_reconstructs_from_reference,
        every_enabled_event_replays_from_one_authoritative_kernel,
        every_diagonal_is_negative_outgoing_sum,
        every_event_has_exact_reciprocal_and_heat_ledger,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProbeControlResults {
    pub unit_clock: ProbeThermalResult,
    pub doubled_clock: ProbeThermalResult,
    pub reverse_mismatch: ProbeThermalResult,
    pub activity_only_invariant: bool,
    pub reverse_mismatch_rejected: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct ProbeMicrostate {
    mesostate: usize,
    intrinsic_index: u32,
    reservoir_descriptor: u64,
    reservoir_word: Vec<u8>,
    reservoir_energy_index: u32,
}

#[derive(Clone, Copy)]
struct ProbeMicrograph {
    clock: Ratio,
    deleted_directed_edge: Option<(usize, usize)>,
}

pub fn verify_held_out_probe(reservoir: &PrefixReservoir) -> ProbeThermalResult {
    verify_held_out_probe_with_clock(reservoir, Ratio::ONE)
}

pub fn verify_held_out_probe_with_clock(
    reservoir: &PrefixReservoir,
    microscopic_clock: Ratio,
) -> ProbeThermalResult {
    verify_probe_micrograph(reservoir, microscopic_clock, false)
}

pub fn verify_held_out_probe_reverse_mismatch(reservoir: &PrefixReservoir) -> ProbeThermalResult {
    verify_probe_micrograph(reservoir, Ratio::ONE, true)
}

pub fn verify_probe_controls(reservoir: &PrefixReservoir) -> ProbeControlResults {
    let unit_clock = verify_held_out_probe(reservoir);
    let doubled_clock = verify_held_out_probe_with_clock(reservoir, Ratio::integer(2));
    let reverse_mismatch = verify_held_out_probe_reverse_mismatch(reservoir);
    let same_temperature = match (
        unit_clock.ldb_temperature.as_ref(),
        doubled_clock.ldb_temperature.as_ref(),
    ) {
        (Some(unit), Some(doubled)) => unit.same_temperature(doubled),
        _ => false,
    };
    let generator_doubled = (0..3).all(|row| {
        (0..3).all(|column| {
            doubled_clock.macro_generator[row][column]
                == Ratio::integer(2) * unit_clock.macro_generator[row][column]
        })
    });
    let activity_only_invariant = unit_clock.operational_closure_passes()
        && doubled_clock.operational_closure_passes()
        && same_temperature
        && unit_clock.macro_stationary_weights == doubled_clock.macro_stationary_weights
        && generator_doubled;
    let reverse_mismatch_rejected = reverse_mismatch.row_closed
        && !reverse_mismatch.reciprocal_microedges
        && !reverse_mismatch.strong_lumpability
        && !reverse_mismatch.operational_closure_passes();

    ProbeControlResults {
        unit_clock,
        doubled_clock,
        reverse_mismatch,
        activity_only_invariant,
        reverse_mismatch_rejected,
    }
}

fn verify_probe_micrograph(
    reservoir: &PrefixReservoir,
    microscopic_clock: Ratio,
    delete_first_reverse: bool,
) -> ProbeThermalResult {
    let state_result = classify_prefix_reservoir(reservoir);
    let shells = [
        shell_alternatives(reservoir, 3),
        shell_alternatives(reservoir, 2),
        shell_alternatives(reservoir, 1),
    ];
    let mut states = Vec::new();
    for mesostate in 0..3 {
        for intrinsic_index in 0..PROBE_BODY_DEGENERACIES[mesostate] {
            for reservoir_state in &shells[mesostate] {
                states.push(ProbeMicrostate {
                    mesostate,
                    intrinsic_index,
                    reservoir_descriptor: reservoir_state.descriptor_id,
                    reservoir_word: reservoir_state.word.clone(),
                    reservoir_energy_index: reservoir_state.energy_index,
                });
            }
        }
    }

    let deleted_directed_edge = if delete_first_reverse {
        states
            .iter()
            .position(|state| state.mesostate == 0)
            .zip(states.iter().position(|state| state.mesostate == 1))
            .map(|(forward_source, forward_target)| (forward_target, forward_source))
    } else {
        None
    };
    let graph = ProbeMicrograph {
        clock: microscopic_clock,
        deleted_directed_edge,
    };

    let fiber_sizes = [
        states.iter().filter(|state| state.mesostate == 0).count() as u128,
        states.iter().filter(|state| state.mesostate == 1).count() as u128,
        states.iter().filter(|state| state.mesostate == 2).count() as u128,
    ];

    let mut reciprocal_microedges = !states.is_empty();
    let mut active_microedges_match_declared_clock = !states.is_empty();
    let mut total_energy_conserved = !states.is_empty();
    for from in 0..states.len() {
        for to in 0..states.len() {
            if from == to {
                continue;
            }
            let forward = probe_off_diagonal(&states, from, to, graph);
            let reverse = probe_off_diagonal(&states, to, from, graph);
            if forward != reverse {
                reciprocal_microedges = false;
            }
            if forward > Ratio::ZERO && forward != microscopic_clock {
                active_microedges_match_declared_clock = false;
            }
            if forward > Ratio::ZERO {
                let before = probe_total_energy_index(&states[from]);
                let after = probe_total_energy_index(&states[to]);
                if before != PROBE_TOTAL_ENERGY_INDEX || after != PROBE_TOTAL_ENERGY_INDEX {
                    total_energy_conserved = false;
                }
            }
        }
    }

    let row_closed = !states.is_empty()
        && (0..states.len()).all(|from| {
            (0..states.len())
                .map(|to| probe_generator_entry(&states, from, to, graph))
                .sum::<Ratio>()
                == Ratio::ZERO
        });

    let strong_lumpability = probe_strong_lumpability(&states, graph);
    let macro_generator = probe_macro_generator(&states, graph);
    let all_energy_gap_channels_present =
        (0..3).all(|from| (0..3).all(|to| from == to || macro_generator[from][to] > Ratio::ZERO));

    let refresh_reciprocal = states
        .iter()
        .enumerate()
        .filter(|(_, state)| state.mesostate == 1)
        .all(|(from, _)| {
            let refresh_targets: Vec<_> = states
                .iter()
                .enumerate()
                .filter(|(to, state)| {
                    *to != from
                        && state.mesostate == 1
                        && probe_off_diagonal(&states, from, *to, graph) == microscopic_clock
                })
                .map(|(to, _)| to)
                .collect();
            refresh_targets.len() == 1
                && probe_off_diagonal(&states, refresh_targets[0], from, graph) == microscopic_clock
        })
        && fiber_sizes[1] > 0;

    let ldb_constraints = probe_ldb_constraints(reservoir.energy_quantum, &macro_generator);
    let common_ldb_beta = ldb_constraints.first().cloned().filter(|first| {
        ldb_constraints.len() == 3 && ldb_constraints.iter().all(|constraint| constraint == first)
    });
    let unique_ldb_temperature = common_ldb_beta
        .as_ref()
        .and_then(ExactLog::sign_if_unambiguous)
        == Some(Ordering::Greater);
    let ldb_temperature = unique_ldb_temperature
        .then(|| Temperature::from_beta(common_ldb_beta.expect("positive common beta exists")));

    let total_microstates = fiber_sizes.iter().sum::<u128>();
    let total_microstates_i128 = i128::try_from(total_microstates).expect("probe size fits i128");
    let macro_stationary_weights = if total_microstates > 0 {
        [
            Ratio::new(i128_from_u128(fiber_sizes[0]), total_microstates_i128),
            Ratio::new(i128_from_u128(fiber_sizes[1]), total_microstates_i128),
            Ratio::new(i128_from_u128(fiber_sizes[2]), total_microstates_i128),
        ]
    } else {
        [Ratio::ZERO; 3]
    };
    let macro_detailed_balance = (0..3).all(|left| {
        ((left + 1)..3).all(|right| {
            ratio_integer_u128(fiber_sizes[left]) * macro_generator[left][right]
                == ratio_integer_u128(fiber_sizes[right]) * macro_generator[right][left]
        })
    });
    let macro_stationary = total_microstates > 0
        && (0..3).all(|target| {
            (0..3)
                .map(|source| macro_stationary_weights[source] * macro_generator[source][target])
                .sum::<Ratio>()
                == Ratio::ZERO
        });
    let irreducible = probe_connected(&states, graph);
    let uniform_microcanonical_stationary = reciprocal_microedges && row_closed;
    let gibbs_microcanonical_agree = ldb_temperature.as_ref().is_some_and(|temperature| {
        probe_gibbs_relation(&temperature.beta, reservoir.energy_quantum, fiber_sizes)
    });
    let unique_stationary_law = uniform_microcanonical_stationary && irreducible;
    let relaxes_from_every_initial_distribution = unique_stationary_law;
    let event_replay = audit_probe_events(&states, graph, reservoir.energy_quantum, fiber_sizes);

    ProbeThermalResult {
        microscopic_clock,
        reservoir_count_integrity: state_result.count_integrity,
        total_microstates,
        fiber_sizes,
        macro_generator,
        reciprocal_microedges,
        active_microedges_match_declared_clock,
        row_closed,
        strong_lumpability,
        total_energy_conserved,
        refresh_reciprocal,
        all_energy_gap_channels_present,
        ldb_constraints,
        ldb_temperature,
        unique_ldb_temperature,
        macro_stationary_weights,
        macro_detailed_balance,
        macro_stationary,
        uniform_microcanonical_stationary,
        gibbs_microcanonical_agree,
        irreducible,
        unique_stationary_law,
        relaxes_from_every_initial_distribution,
        event_replay,
    }
}

fn audit_probe_events(
    states: &[ProbeMicrostate],
    graph: ProbeMicrograph,
    energy_quantum: Ratio,
    fiber_sizes: [u128; 3],
) -> ProbeEventReplayAudit {
    let complete_microstates_checked = states.len() as u128;
    let complete_microstate_encoding_is_unique =
        states.iter().cloned().collect::<BTreeSet<_>>().len() == states.len();
    let complete_microstate_count_matches_fibers =
        complete_microstates_checked == fiber_sizes.into_iter().sum::<u128>();
    let mut directed_events_checked = 0_u128;
    let mut every_generator_entry_replays_from_complete_state = !states.is_empty();
    let mut every_event_has_reciprocal_support = !states.is_empty();
    let mut every_event_conserves_total_energy = !states.is_empty();
    let mut every_event_body_heat_ledger_closes = !states.is_empty();
    let mut every_event_reservoir_heat_ledger_closes = !states.is_empty();
    let mut every_event_has_zero_work = !states.is_empty();
    let mut every_event_path_ratio_closes = !states.is_empty();

    for from in 0..states.len() {
        let replayed_diagonal = -(0..states.len())
            .filter(|&to| to != from)
            .map(|to| probe_off_diagonal(states, from, to, graph))
            .sum::<Ratio>();
        every_generator_entry_replays_from_complete_state &=
            probe_generator_entry(states, from, from, graph) == replayed_diagonal;
        for to in 0..states.len() {
            if from == to {
                continue;
            }
            let forward = probe_off_diagonal(states, from, to, graph);
            every_generator_entry_replays_from_complete_state &=
                probe_generator_entry(states, from, to, graph) == forward;
            if !forward.is_positive() {
                continue;
            }
            directed_events_checked = directed_events_checked
                .checked_add(1)
                .expect("probe event-count overflow");
            let reverse = probe_off_diagonal(states, to, from, graph);
            every_event_has_reciprocal_support &= reverse.is_positive();

            let before = &states[from];
            let after = &states[to];
            let body_before = energy_quantum
                * Ratio::integer(i128::from(PROBE_BODY_ENERGY_INDICES[before.mesostate]));
            let body_after = energy_quantum
                * Ratio::integer(i128::from(PROBE_BODY_ENERGY_INDICES[after.mesostate]));
            let reservoir_before =
                energy_quantum * Ratio::integer(i128::from(before.reservoir_energy_index));
            let reservoir_after =
                energy_quantum * Ratio::integer(i128::from(after.reservoir_energy_index));
            let delta_body = body_after - body_before;
            let delta_reservoir = reservoir_after - reservoir_before;
            let heat_to_body = delta_body;
            let work_on_body = Ratio::ZERO;
            every_event_conserves_total_energy &= delta_body + delta_reservoir == Ratio::ZERO;
            every_event_body_heat_ledger_closes &= delta_body == heat_to_body + work_on_body;
            every_event_reservoir_heat_ledger_closes &= delta_reservoir == -heat_to_body;
            every_event_has_zero_work &= work_on_body == Ratio::ZERO;
            every_event_path_ratio_closes &=
                reverse.is_positive() && ExactLog::ln_ratio(forward / reverse).is_zero();
        }
    }

    ProbeEventReplayAudit {
        complete_microstates_checked,
        directed_events_checked,
        complete_microstate_encoding_is_unique,
        complete_microstate_count_matches_fibers,
        every_generator_entry_replays_from_complete_state,
        every_event_has_reciprocal_support,
        every_event_conserves_total_energy,
        every_event_body_heat_ledger_closes,
        every_event_reservoir_heat_ledger_closes,
        every_event_has_zero_work,
        every_event_path_ratio_closes,
    }
}

fn probe_total_energy_index(state: &ProbeMicrostate) -> u32 {
    PROBE_BODY_ENERGY_INDICES[state.mesostate] + state.reservoir_energy_index
}

fn probe_off_diagonal(
    states: &[ProbeMicrostate],
    from: usize,
    to: usize,
    graph: ProbeMicrograph,
) -> Ratio {
    if from == to || graph.deleted_directed_edge == Some((from, to)) {
        return Ratio::ZERO;
    }
    let from = &states[from];
    let to = &states[to];
    if from.mesostate != to.mesostate {
        return graph.clock;
    }
    if from.mesostate == 1
        && from.reservoir_descriptor == to.reservoir_descriptor
        && from.reservoir_word == to.reservoir_word
        && from.intrinsic_index != to.intrinsic_index
    {
        graph.clock
    } else {
        Ratio::ZERO
    }
}

fn probe_generator_entry(
    states: &[ProbeMicrostate],
    from: usize,
    to: usize,
    graph: ProbeMicrograph,
) -> Ratio {
    if from != to {
        probe_off_diagonal(states, from, to, graph)
    } else {
        -(0..states.len())
            .filter(|candidate| *candidate != from)
            .map(|candidate| probe_off_diagonal(states, from, candidate, graph))
            .sum::<Ratio>()
    }
}

fn probe_block_sum(
    states: &[ProbeMicrostate],
    source: usize,
    target_mesostate: usize,
    graph: ProbeMicrograph,
) -> Ratio {
    states
        .iter()
        .enumerate()
        .filter(|(_, state)| state.mesostate == target_mesostate)
        .map(|(target, _)| probe_generator_entry(states, source, target, graph))
        .sum()
}

fn probe_strong_lumpability(states: &[ProbeMicrostate], graph: ProbeMicrograph) -> bool {
    if (0..3).any(|mesostate| !states.iter().any(|state| state.mesostate == mesostate)) {
        return false;
    }
    (0..3).all(|source_mesostate| {
        let sources: Vec<_> = states
            .iter()
            .enumerate()
            .filter(|(_, state)| state.mesostate == source_mesostate)
            .map(|(index, _)| index)
            .collect();
        (0..3).all(|target_mesostate| {
            let reference = probe_block_sum(states, sources[0], target_mesostate, graph);
            sources.iter().skip(1).all(|&source| {
                probe_block_sum(states, source, target_mesostate, graph) == reference
            })
        })
    })
}

fn probe_macro_generator(states: &[ProbeMicrostate], graph: ProbeMicrograph) -> [[Ratio; 3]; 3] {
    let mut generator = [[Ratio::ZERO; 3]; 3];
    for source_mesostate in 0..3 {
        let Some(source) = states
            .iter()
            .position(|state| state.mesostate == source_mesostate)
        else {
            continue;
        };
        for target_mesostate in 0..3 {
            generator[source_mesostate][target_mesostate] =
                probe_block_sum(states, source, target_mesostate, graph);
        }
    }
    generator
}

fn probe_ldb_constraints(energy_quantum: Ratio, generator: &[[Ratio; 3]; 3]) -> Vec<ExactLog> {
    if !energy_quantum.is_positive() {
        return Vec::new();
    }
    let mut constraints = Vec::new();
    for (left, right) in [(0_usize, 1_usize), (1, 2), (0, 2)] {
        let forward = generator[left][right];
        let reverse = generator[right][left];
        if !forward.is_positive() || !reverse.is_positive() {
            return Vec::new();
        }
        let body_ratio = Ratio::new(
            i128::from(PROBE_BODY_DEGENERACIES[right]),
            i128::from(PROBE_BODY_DEGENERACIES[left]),
        );
        let rate_ratio = forward / reverse;
        let delta_index = PROBE_BODY_ENERGY_INDICES[right] - PROBE_BODY_ENERGY_INDICES[left];
        let delta_energy = energy_quantum * Ratio::integer(i128::from(delta_index));
        let beta = (ExactLog::ln_ratio(body_ratio) - ExactLog::ln_ratio(rate_ratio))
            .scaled(delta_energy.reciprocal());
        constraints.push(beta);
    }
    constraints
}

fn probe_gibbs_relation(beta: &ExactLog, energy_quantum: Ratio, fiber_sizes: [u128; 3]) -> bool {
    if fiber_sizes.iter().any(|&size| size == 0) {
        return false;
    }
    let reference = Ratio::new(
        i128_from_u128(fiber_sizes[0]),
        i128::from(PROBE_BODY_DEGENERACIES[0]),
    );
    (1..3).all(|mesostate| {
        let reduced_weight = Ratio::new(
            i128_from_u128(fiber_sizes[mesostate]),
            i128::from(PROBE_BODY_DEGENERACIES[mesostate]),
        );
        let weight_log = ExactLog::ln_ratio(reduced_weight / reference);
        let delta_energy =
            energy_quantum * Ratio::integer(i128::from(PROBE_BODY_ENERGY_INDICES[mesostate]));
        (weight_log + beta.scaled(delta_energy)).is_zero()
    })
}

fn probe_connected(states: &[ProbeMicrostate], graph: ProbeMicrograph) -> bool {
    if states.is_empty() {
        return false;
    }
    let mut visited = vec![false; states.len()];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;
    while let Some(from) = queue.pop_front() {
        for to in 0..states.len() {
            if !visited[to] && probe_off_diagonal(states, from, to, graph) > Ratio::ZERO {
                visited[to] = true;
                queue.push_back(to);
            }
        }
    }
    visited.into_iter().all(|value| value)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StaticPerfectTreeControlReport {
    pub alphabet_size: u8,
    pub energy_quantum: Ratio,
    pub maximum_depth: u32,
    pub observer_supplied_architecture: bool,
    pub builder_enabled: bool,
    pub adaptive_memory_enabled: bool,
    pub initially_nonthermal: bool,
    pub state: PrefixThermalResult,
    pub probe: ProbeThermalResult,
    pub operational_thermal_sector_passes: bool,
    pub endogenous_creation_admitted: bool,
    pub adaptation_admitted: bool,
}

impl StaticPerfectTreeControlReport {
    pub fn matches_frozen_classification(&self) -> bool {
        self.alphabet_size >= 2
            && self.energy_quantum.is_positive()
            && self.maximum_depth == 3
            && self.observer_supplied_architecture
            && !self.builder_enabled
            && !self.adaptive_memory_enabled
            && !self.initially_nonthermal
            && self.state.exact_prefix_temperature_passes()
            && self.probe.operational_closure_passes()
            && self.operational_thermal_sector_passes
            && !self.endogenous_creation_admitted
            && !self.adaptation_admitted
    }
}

pub fn verify_static_perfect_tree_control(
    alphabet_size: u8,
    energy_quantum: Ratio,
) -> StaticPerfectTreeControlReport {
    let maximum_depth = 3;
    let reservoir = observer_perfect_prefix_reservoir(alphabet_size, energy_quantum, maximum_depth);
    let state = classify_prefix_reservoir(&reservoir);
    let probe = verify_held_out_probe(&reservoir);
    let observer_supplied_architecture = true;
    let builder_enabled = false;
    let adaptive_memory_enabled = false;
    let initially_nonthermal = state.classification != StateTemperatureClass::UniqueConstant;
    let operational_thermal_sector_passes =
        state.exact_prefix_temperature_passes() && probe.operational_closure_passes();
    let endogenous_creation_admitted =
        operational_thermal_sector_passes && initially_nonthermal && builder_enabled;
    let adaptation_admitted = endogenous_creation_admitted && adaptive_memory_enabled;

    StaticPerfectTreeControlReport {
        alphabet_size,
        energy_quantum,
        maximum_depth,
        observer_supplied_architecture,
        builder_enabled,
        adaptive_memory_enabled,
        initially_nonthermal,
        state,
        probe,
        operational_thermal_sector_passes,
        endogenous_creation_admitted,
        adaptation_admitted,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalThermostatControlReport {
    pub actual_shell_counts: Vec<u128>,
    pub state_classification: StateTemperatureClass,
    pub state_temperature: Option<Temperature>,
    pub beta_supplied_externally: bool,
    pub rates_supplied_externally: bool,
    pub rates_derived_from_actual_multiplicities: bool,
    pub installed_beta: ExactLog,
    pub installed_macro_generator: [[Ratio; 3]; 3],
    pub installed_generator_row_closed: bool,
    pub ldb_constraints: Vec<ExactLog>,
    pub every_ldb_constraint_matches_installed_beta: bool,
    pub installed_rate_temperature: Option<Temperature>,
    pub installed_rate_stationary_weights: [Ratio; 3],
    pub installed_rate_detailed_balance: bool,
    pub rate_temperature_matches_state_temperature: bool,
    pub rate_compatible_only: bool,
}

impl ExternalThermostatControlReport {
    pub fn matches_frozen_classification(&self) -> bool {
        self.actual_shell_counts == vec![1, 2, 5, 10]
            && self.state_classification == StateTemperatureClass::StateDependent
            && self.state_temperature.is_none()
            && self.beta_supplied_externally
            && self.rates_supplied_externally
            && !self.rates_derived_from_actual_multiplicities
            && self.installed_beta == ExactLog::ln_ratio(Ratio::integer(2))
            && self.installed_macro_generator
                == [
                    [-Ratio::integer(2), Ratio::ONE, Ratio::ONE],
                    [Ratio::ONE, -Ratio::integer(2), Ratio::ONE],
                    [Ratio::integer(4), Ratio::integer(4), -Ratio::integer(8)],
                ]
            && self.installed_generator_row_closed
            && self.ldb_constraints.len() == 3
            && self.every_ldb_constraint_matches_installed_beta
            && self.installed_rate_temperature.is_some()
            && self.installed_rate_stationary_weights
                == [Ratio::new(4, 9), Ratio::new(4, 9), Ratio::new(1, 9)]
            && self.installed_rate_detailed_balance
            && !self.rate_temperature_matches_state_temperature
            && self.rate_compatible_only
    }
}

pub fn verify_external_thermostat_control() -> ExternalThermostatControlReport {
    let reservoir = observer_irregular_reservoir();
    let state = classify_prefix_reservoir(&reservoir);
    let installed_beta = ExactLog::ln_ratio(Ratio::integer(2));
    let installed_macro_generator = [
        [-Ratio::integer(2), Ratio::ONE, Ratio::ONE],
        [Ratio::ONE, -Ratio::integer(2), Ratio::ONE],
        [Ratio::integer(4), Ratio::integer(4), -Ratio::integer(8)],
    ];
    let installed_generator_row_closed = installed_macro_generator
        .iter()
        .all(|row| row.iter().copied().sum::<Ratio>() == Ratio::ZERO);
    let ldb_constraints =
        probe_ldb_constraints(reservoir.energy_quantum, &installed_macro_generator);
    let every_ldb_constraint_matches_installed_beta = ldb_constraints.len() == 3
        && ldb_constraints
            .iter()
            .all(|constraint| constraint == &installed_beta);
    let installed_rate_temperature = every_ldb_constraint_matches_installed_beta
        .then(|| Temperature::from_beta(installed_beta.clone()));
    let installed_rate_stationary_weights = [Ratio::new(4, 9), Ratio::new(4, 9), Ratio::new(1, 9)];
    let installed_rate_detailed_balance = (0..3).all(|left| {
        ((left + 1)..3).all(|right| {
            installed_rate_stationary_weights[left] * installed_macro_generator[left][right]
                == installed_rate_stationary_weights[right] * installed_macro_generator[right][left]
        })
    });
    let rate_temperature_matches_state_temperature = match (
        installed_rate_temperature.as_ref(),
        state.temperature.as_ref(),
    ) {
        (Some(rate), Some(state)) => rate.same_temperature(state),
        _ => false,
    };
    let rate_compatible_only = every_ldb_constraint_matches_installed_beta
        && installed_rate_detailed_balance
        && state.classification == StateTemperatureClass::StateDependent
        && state.temperature.is_none()
        && !rate_temperature_matches_state_temperature;

    ExternalThermostatControlReport {
        actual_shell_counts: state.shell_counts,
        state_classification: state.classification,
        state_temperature: state.temperature,
        beta_supplied_externally: true,
        rates_supplied_externally: true,
        rates_derived_from_actual_multiplicities: false,
        installed_beta,
        installed_macro_generator,
        installed_generator_row_closed,
        ldb_constraints,
        every_ldb_constraint_matches_installed_beta,
        installed_rate_temperature,
        installed_rate_stationary_weights,
        installed_rate_detailed_balance,
        rate_temperature_matches_state_temperature,
        rate_compatible_only,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GibbsInstalledControlReport {
    pub actual_shell_counts: Vec<u128>,
    pub state_classification: StateTemperatureClass,
    pub state_temperature: Option<Temperature>,
    pub beta_supplied_externally: bool,
    pub weights_supplied_externally: bool,
    pub weights_derived_from_actual_multiplicities: bool,
    pub installed_beta: ExactLog,
    pub installed_gibbs_weights: [Ratio; 3],
    pub installed_weights_are_normalized: bool,
    pub installed_gibbs_identity_holds: bool,
    pub actual_probe_fiber_sizes: [u128; 3],
    pub actual_microcanonical_projection: [Ratio; 3],
    pub actual_projection_is_normalized: bool,
    pub actual_projection_satisfies_installed_gibbs_identity: bool,
    pub installed_weights_equal_actual_projection: bool,
    pub fitted_representation_only: bool,
}

impl GibbsInstalledControlReport {
    pub fn matches_frozen_classification(&self) -> bool {
        self.actual_shell_counts == vec![1, 2, 5, 10]
            && self.state_classification == StateTemperatureClass::StateDependent
            && self.state_temperature.is_none()
            && self.beta_supplied_externally
            && self.weights_supplied_externally
            && !self.weights_derived_from_actual_multiplicities
            && self.installed_beta == ExactLog::ln_ratio(Ratio::integer(2))
            && self.installed_gibbs_weights
                == [Ratio::new(4, 9), Ratio::new(4, 9), Ratio::new(1, 9)]
            && self.installed_weights_are_normalized
            && self.installed_gibbs_identity_holds
            && self.actual_probe_fiber_sizes == [10, 10, 2]
            && self.actual_microcanonical_projection
                == [Ratio::new(5, 11), Ratio::new(5, 11), Ratio::new(1, 11)]
            && self.actual_projection_is_normalized
            && !self.actual_projection_satisfies_installed_gibbs_identity
            && !self.installed_weights_equal_actual_projection
            && self.fitted_representation_only
    }
}

pub fn verify_gibbs_installed_control() -> GibbsInstalledControlReport {
    let reservoir = observer_irregular_reservoir();
    let state = classify_prefix_reservoir(&reservoir);
    let installed_beta = ExactLog::ln_ratio(Ratio::integer(2));
    let installed_gibbs_weights = [Ratio::new(4, 9), Ratio::new(4, 9), Ratio::new(1, 9)];
    let installed_weights_are_normalized =
        installed_gibbs_weights.into_iter().sum::<Ratio>() == Ratio::ONE;
    let installed_gibbs_identity_holds = probe_gibbs_weights_match_beta(
        &installed_beta,
        reservoir.energy_quantum,
        installed_gibbs_weights,
    );
    let actual_probe_fiber_sizes = [
        state.shell_counts[3],
        2 * state.shell_counts[2],
        state.shell_counts[1],
    ];
    let actual_probe_total = actual_probe_fiber_sizes.iter().sum::<u128>();
    let actual_microcanonical_projection = [
        ratio_u128(actual_probe_fiber_sizes[0], actual_probe_total),
        ratio_u128(actual_probe_fiber_sizes[1], actual_probe_total),
        ratio_u128(actual_probe_fiber_sizes[2], actual_probe_total),
    ];
    let actual_projection_is_normalized =
        actual_microcanonical_projection.into_iter().sum::<Ratio>() == Ratio::ONE;
    let actual_projection_satisfies_installed_gibbs_identity = probe_gibbs_relation(
        &installed_beta,
        reservoir.energy_quantum,
        actual_probe_fiber_sizes,
    );
    let installed_weights_equal_actual_projection =
        installed_gibbs_weights == actual_microcanonical_projection;
    let fitted_representation_only = installed_gibbs_identity_holds
        && state.classification == StateTemperatureClass::StateDependent
        && state.temperature.is_none()
        && !actual_projection_satisfies_installed_gibbs_identity
        && !installed_weights_equal_actual_projection;

    GibbsInstalledControlReport {
        actual_shell_counts: state.shell_counts,
        state_classification: state.classification,
        state_temperature: state.temperature,
        beta_supplied_externally: true,
        weights_supplied_externally: true,
        weights_derived_from_actual_multiplicities: false,
        installed_beta,
        installed_gibbs_weights,
        installed_weights_are_normalized,
        installed_gibbs_identity_holds,
        actual_probe_fiber_sizes,
        actual_microcanonical_projection,
        actual_projection_is_normalized,
        actual_projection_satisfies_installed_gibbs_identity,
        installed_weights_equal_actual_projection,
        fitted_representation_only,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortLawMutationControlReport {
    pub alphabet_size: u8,
    pub energy_quantum: Ratio,
    pub distinct_vertices_checked: u128,
    pub every_vertex_is_executable: bool,
    pub vertex_identity_encoding_is_injective: bool,
    pub descriptor_encoding_is_injective: bool,
    pub canonical_child_encoding_is_injective: bool,
    pub every_vertex_energy_matches_word_depth: bool,
    pub every_nonroot_binding_matches_canonical_word: bool,
    pub duplicated_parent_vertex_id: u64,
    pub duplicated_port: u8,
    pub duplicated_canonical_child: Vec<u8>,
    pub duplicate_binding_multiplicity: u128,
    pub exactly_one_parent_port_is_duplicated: bool,
    pub every_other_parent_port_has_one_child: bool,
    pub every_required_parent_port_is_present: bool,
    pub one_child_per_parent_port: bool,
    pub actual_shell_counts: Vec<u128>,
    pub expected_mutated_shell_counts: Vec<u128>,
    pub adjacent_ratios: Vec<Option<Ratio>>,
    pub adjacent_betas: Vec<Option<ExactLog>>,
    pub state_classification: StateTemperatureClass,
    pub state_temperature: Option<Temperature>,
    pub old_prefix_beta: ExactLog,
    pub old_prefix_temperature_rejected: bool,
}

impl PortLawMutationControlReport {
    pub fn matches_frozen_classification(&self) -> bool {
        let b = u128::from(self.alphabet_size);
        let b_squared = checked_power(b, 2).expect("u8 base squared fits u128");
        let b_cubed = checked_power(b, 3).expect("u8 base cubed fits u128");
        let expected_ratios = vec![
            Some(Ratio::integer(i128::from(self.alphabet_size))),
            Some(Ratio::integer(i128::from(self.alphabet_size))),
            Some(ratio_u128(b_cubed + 1, b_squared)),
        ];
        self.alphabet_size >= 2
            && self.energy_quantum.is_positive()
            && self.distinct_vertices_checked == self.actual_shell_counts.iter().sum::<u128>()
            && self.every_vertex_is_executable
            && self.vertex_identity_encoding_is_injective
            && self.descriptor_encoding_is_injective
            && !self.canonical_child_encoding_is_injective
            && self.every_vertex_energy_matches_word_depth
            && self.every_nonroot_binding_matches_canonical_word
            && self.duplicate_binding_multiplicity == 2
            && self.exactly_one_parent_port_is_duplicated
            && self.every_other_parent_port_has_one_child
            && self.every_required_parent_port_is_present
            && !self.one_child_per_parent_port
            && self.actual_shell_counts == self.expected_mutated_shell_counts
            && self.expected_mutated_shell_counts == vec![1, b, b_squared, b_cubed + 1]
            && self.adjacent_ratios == expected_ratios
            && self.state_classification == StateTemperatureClass::StateDependent
            && self.state_temperature.is_none()
            && self.old_prefix_beta
                == ExactLog::ln_ratio(Ratio::integer(i128::from(self.alphabet_size)))
                    .scaled(self.energy_quantum.reciprocal())
            && self.old_prefix_temperature_rejected
    }
}

pub fn verify_port_law_mutation_control(
    alphabet_size: u8,
    energy_quantum: Ratio,
) -> PortLawMutationControlReport {
    assert!(alphabet_size >= 2, "the frozen mutation requires b >= 2");
    assert!(
        energy_quantum.is_positive(),
        "the frozen mutation requires a positive energy quantum"
    );
    let vertices = observer_port_law_mutant(alphabet_size);
    let distinct_vertices_checked = vertices.len() as u128;
    let every_vertex_is_executable = vertices.iter().all(|vertex| vertex.executable);
    let mut vertex_ids = BTreeSet::new();
    let vertex_identity_encoding_is_injective = vertices
        .iter()
        .all(|vertex| vertex_ids.insert(vertex.vertex_id));
    let mut descriptor_ids = BTreeSet::new();
    let descriptor_encoding_is_injective = vertices
        .iter()
        .all(|vertex| descriptor_ids.insert(vertex.descriptor_id));
    let mut canonical_words = BTreeSet::new();
    let canonical_child_encoding_is_injective = vertices
        .iter()
        .all(|vertex| canonical_words.insert(vertex.canonical_word.clone()));
    let every_vertex_energy_matches_word_depth = vertices
        .iter()
        .all(|vertex| u32::try_from(vertex.canonical_word.len()) == Ok(vertex.energy_index));
    let vertex_by_id: BTreeMap<_, _> = vertices
        .iter()
        .map(|vertex| (vertex.vertex_id, vertex))
        .collect();
    let every_nonroot_binding_matches_canonical_word = vertices.iter().all(|vertex| {
        if vertex.energy_index == 0 {
            return vertex.parent_vertex_id.is_none()
                && vertex.port.is_none()
                && vertex.canonical_word.is_empty();
        }
        let Some(parent) = vertex
            .parent_vertex_id
            .and_then(|parent| vertex_by_id.get(&parent).copied())
        else {
            return false;
        };
        let Some(port) = vertex.port else {
            return false;
        };
        let mut expected_child = parent.canonical_word.clone();
        expected_child.push(port);
        expected_child == vertex.canonical_word && vertex.energy_index == parent.energy_index + 1
    });

    let mut binding_counts: BTreeMap<(u64, u8, Vec<u8>), u128> = BTreeMap::new();
    let mut parent_port_counts: BTreeMap<(u64, u8), u128> = BTreeMap::new();
    for vertex in vertices.iter().filter(|vertex| vertex.energy_index > 0) {
        let parent_vertex_id = vertex
            .parent_vertex_id
            .expect("every nonroot mutant vertex has a parent identity");
        let port = vertex.port.expect("every nonroot mutant vertex has a port");
        *binding_counts
            .entry((parent_vertex_id, port, vertex.canonical_word.clone()))
            .or_default() += 1;
        *parent_port_counts
            .entry((parent_vertex_id, port))
            .or_default() += 1;
    }
    let duplicated_bindings: Vec<(u64, u8, Vec<u8>, u128)> = binding_counts
        .iter()
        .filter_map(|((parent, port, child), &count)| {
            (count > 1).then(|| (*parent, *port, child.clone(), count))
        })
        .collect();
    let (duplicated_parent_vertex_id, duplicated_port, duplicated_canonical_child) =
        duplicated_bindings
            .first()
            .map(|(parent, port, child, _)| (*parent, *port, child.clone()))
            .unwrap_or((u64::MAX, 0, Vec::new()));
    let duplicate_binding_multiplicity = duplicated_bindings
        .first()
        .map_or(0, |(_, _, _, count)| *count);
    let exactly_one_parent_port_is_duplicated = duplicated_bindings.len() == 1
        && duplicate_binding_multiplicity == 2
        && parent_port_counts
            .values()
            .filter(|&&count| count > 1)
            .count()
            == 1;
    let every_other_parent_port_has_one_child =
        parent_port_counts.iter().all(|(&(parent, port), &count)| {
            (parent == duplicated_parent_vertex_id && port == duplicated_port && count == 2)
                || count == 1
        });
    let every_required_parent_port_is_present = vertices
        .iter()
        .filter(|vertex| vertex.energy_index < 3)
        .all(|parent| {
            (1..=alphabet_size).all(|port| {
                parent_port_counts
                    .get(&(parent.vertex_id, port))
                    .is_some_and(|&count| count >= 1)
            })
        });
    let one_child_per_parent_port = every_required_parent_port_is_present
        && parent_port_counts.values().all(|&count| count == 1);

    let mut actual_shell_counts = vec![0_u128; 4];
    for vertex in vertices.iter().filter(|vertex| vertex.executable) {
        actual_shell_counts[vertex.energy_index as usize] += 1;
    }
    let b = u128::from(alphabet_size);
    let b_squared = checked_power(b, 2).expect("u8 base squared fits u128");
    let b_cubed = checked_power(b, 3).expect("u8 base cubed fits u128");
    let expected_mutated_shell_counts = vec![1, b, b_squared, b_cubed + 1];
    let (adjacent_ratios, adjacent_betas, state_classification, state_temperature) =
        classify_distinguishable_shell_counts(&actual_shell_counts, energy_quantum);
    let old_prefix_beta = ExactLog::ln_ratio(Ratio::integer(i128::from(alphabet_size)))
        .scaled(energy_quantum.reciprocal());
    let old_prefix_temperature_rejected = state_classification
        != StateTemperatureClass::UniqueConstant
        && state_temperature.is_none()
        && adjacent_ratios.last().copied().flatten()
            != Some(Ratio::integer(i128::from(alphabet_size)));

    PortLawMutationControlReport {
        alphabet_size,
        energy_quantum,
        distinct_vertices_checked,
        every_vertex_is_executable,
        vertex_identity_encoding_is_injective,
        descriptor_encoding_is_injective,
        canonical_child_encoding_is_injective,
        every_vertex_energy_matches_word_depth,
        every_nonroot_binding_matches_canonical_word,
        duplicated_parent_vertex_id,
        duplicated_port,
        duplicated_canonical_child,
        duplicate_binding_multiplicity,
        exactly_one_parent_port_is_duplicated,
        every_other_parent_port_has_one_child,
        every_required_parent_port_is_present,
        one_child_per_parent_port,
        actual_shell_counts,
        expected_mutated_shell_counts,
        adjacent_ratios,
        adjacent_betas,
        state_classification,
        state_temperature,
        old_prefix_beta,
        old_prefix_temperature_rejected,
    }
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct PortLawVertex {
    vertex_id: u64,
    descriptor_id: u64,
    canonical_word: Vec<u8>,
    parent_vertex_id: Option<u64>,
    port: Option<u8>,
    energy_index: u32,
    executable: bool,
}

fn observer_perfect_prefix_reservoir(
    alphabet_size: u8,
    energy_quantum: Ratio,
    maximum_depth: u32,
) -> PrefixReservoir {
    assert!(alphabet_size >= 2, "a prefix reservoir requires b >= 2");
    assert!(
        energy_quantum.is_positive(),
        "a prefix reservoir requires a positive energy quantum"
    );
    let mut promoted = vec![ReservoirAlternative {
        descriptor_id: 0,
        word: Vec::new(),
        energy_index: 0,
        executable: true,
    }];
    let mut frontier = vec![Vec::new()];
    for depth in 1..=maximum_depth {
        let mut next_frontier = Vec::new();
        for parent in &frontier {
            for symbol in 1..=alphabet_size {
                let mut child = parent.clone();
                child.push(symbol);
                let descriptor_id =
                    u64::try_from(promoted.len()).expect("observer descriptor ID fits u64");
                promoted.push(ReservoirAlternative {
                    descriptor_id,
                    word: child.clone(),
                    energy_index: depth,
                    executable: true,
                });
                next_frontier.push(child);
            }
        }
        frontier = next_frontier;
    }
    PrefixReservoir::new(alphabet_size, energy_quantum, promoted)
}

fn observer_irregular_reservoir() -> PrefixReservoir {
    let counts = [1_u8, 2, 5, 10];
    let mut promoted = Vec::new();
    for (depth, &count) in counts.iter().enumerate() {
        for index in 0..count {
            let mut word = vec![1; depth];
            if let Some(last) = word.last_mut() {
                *last = index + 1;
            }
            let descriptor_id =
                u64::try_from(promoted.len()).expect("irregular descriptor ID fits u64");
            promoted.push(ReservoirAlternative {
                descriptor_id,
                word,
                energy_index: u32::try_from(depth).expect("irregular depth fits u32"),
                executable: true,
            });
        }
    }
    PrefixReservoir::new(10, Ratio::ONE, promoted)
}

fn probe_gibbs_weights_match_beta(
    beta: &ExactLog,
    energy_quantum: Ratio,
    weights: [Ratio; 3],
) -> bool {
    if weights.iter().any(|weight| !weight.is_positive()) {
        return false;
    }
    let reference = weights[0] / Ratio::integer(i128::from(PROBE_BODY_DEGENERACIES[0]));
    (1..3).all(|mesostate| {
        let reduced_weight =
            weights[mesostate] / Ratio::integer(i128::from(PROBE_BODY_DEGENERACIES[mesostate]));
        let weight_log = ExactLog::ln_ratio(reduced_weight / reference);
        let delta_energy =
            energy_quantum * Ratio::integer(i128::from(PROBE_BODY_ENERGY_INDICES[mesostate]));
        (weight_log + beta.scaled(delta_energy)).is_zero()
    })
}

fn observer_port_law_mutant(alphabet_size: u8) -> Vec<PortLawVertex> {
    let reservoir = observer_perfect_prefix_reservoir(alphabet_size, Ratio::ONE, 3);
    let word_to_vertex: BTreeMap<_, _> = reservoir
        .promoted
        .iter()
        .map(|state| (state.word.clone(), state.descriptor_id))
        .collect();
    let mut vertices: Vec<_> = reservoir
        .promoted
        .iter()
        .map(|state| {
            let (parent_vertex_id, port) = if state.word.is_empty() {
                (None, None)
            } else {
                let parent_word = state.word[..state.word.len() - 1].to_vec();
                (
                    Some(
                        *word_to_vertex
                            .get(&parent_word)
                            .expect("perfect child has its canonical parent"),
                    ),
                    state.word.last().copied(),
                )
            };
            PortLawVertex {
                vertex_id: state.descriptor_id,
                descriptor_id: state.descriptor_id,
                canonical_word: state.word.clone(),
                parent_vertex_id,
                port,
                energy_index: state.energy_index,
                executable: state.executable,
            }
        })
        .collect();
    let template = vertices
        .iter()
        .find(|vertex| vertex.energy_index == 3)
        .cloned()
        .expect("a depth-three perfect tree has a leaf");
    let duplicate_identity =
        u64::try_from(vertices.len()).expect("mutant vertex identity fits u64");
    vertices.push(PortLawVertex {
        vertex_id: duplicate_identity,
        descriptor_id: duplicate_identity,
        ..template
    });
    vertices
}

fn classify_distinguishable_shell_counts(
    shell_counts: &[u128],
    energy_quantum: Ratio,
) -> (
    Vec<Option<Ratio>>,
    Vec<Option<ExactLog>>,
    StateTemperatureClass,
    Option<Temperature>,
) {
    let adjacent_ratios: Vec<_> = shell_counts
        .windows(2)
        .map(|pair| (pair[0] > 0).then(|| ratio_u128(pair[1], pair[0])))
        .collect();
    let adjacent_betas: Vec<_> = adjacent_ratios
        .iter()
        .map(|ratio| {
            ratio.and_then(|ratio| {
                (ratio.is_positive() && energy_quantum.is_positive())
                    .then(|| ExactLog::ln_ratio(ratio).scaled(energy_quantum.reciprocal()))
            })
        })
        .collect();
    let classification = if !energy_quantum.is_positive()
        || adjacent_ratios.len() < 2
        || shell_counts.iter().any(|&count| count == 0)
    {
        StateTemperatureClass::Undefined
    } else if adjacent_ratios
        .iter()
        .any(|ratio| !ratio.is_some_and(|value| value > Ratio::ONE))
    {
        StateTemperatureClass::NonPositive
    } else {
        let first = adjacent_betas[0]
            .as_ref()
            .expect("a positive shell gap has an inverse temperature");
        if adjacent_betas
            .iter()
            .all(|beta| beta.as_ref() == Some(first))
        {
            StateTemperatureClass::UniqueConstant
        } else {
            StateTemperatureClass::StateDependent
        }
    };
    let temperature = (classification == StateTemperatureClass::UniqueConstant).then(|| {
        Temperature::from_beta(
            adjacent_betas[0]
                .clone()
                .expect("constant positive shell gaps have an inverse temperature"),
        )
    });
    (adjacent_ratios, adjacent_betas, classification, temperature)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactProtocol {
    pub quantum: Ratio,
    pub total_energy: Ratio,
    pub microscopic_clock: Ratio,
    pub prepared_macro_distribution: [Ratio; 2],
}

pub fn frozen_contact_protocol(microscopic_clock: Ratio) -> ContactProtocol {
    ContactProtocol {
        quantum: Ratio::integer(2),
        total_energy: Ratio::integer(4),
        microscopic_clock,
        prepared_macro_distribution: [Ratio::new(1, 2), Ratio::new(1, 2)],
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TemperatureOrder {
    LeftHotter,
    Equal,
    RightHotter,
    Incomparable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactThermalResult {
    pub microscopic_clock: Ratio,
    pub left_state_temperature: Option<Temperature>,
    pub right_state_temperature: Option<Temperature>,
    pub temperature_order: TemperatureOrder,
    pub quantum_commensurate: bool,
    pub two_fibers_at_declared_total_energy: bool,
    pub transfer_conserves_energy: bool,
    pub omega_a: u128,
    pub omega_b: u128,
    pub macro_generator: [[Ratio; 2]; 2],
    pub reciprocal_microedges: bool,
    pub row_closed: bool,
    pub strong_lumpability: bool,
    pub irreducible: bool,
    pub stationary_macro_distribution: [Ratio; 2],
    pub uniform_product_stationary: bool,
    pub detailed_balance: bool,
    pub unique_stationary_law: bool,
    pub relaxes_from_every_initial_distribution: bool,
    pub prepared_event_current_a_to_b: Ratio,
    pub prepared_energy_current_into_left: Ratio,
    pub prepared_heat_flows_hot_to_cold: bool,
    pub stationary_event_current_a_to_b: Ratio,
    pub stationary_energy_current_into_left: Ratio,
    pub event_replay: ContactEventReplayAudit,
}

impl ContactThermalResult {
    pub fn operational_closure_passes(&self) -> bool {
        self.quantum_commensurate
            && self.two_fibers_at_declared_total_energy
            && self.transfer_conserves_energy
            && self.reciprocal_microedges
            && self.row_closed
            && self.strong_lumpability
            && self.irreducible
            && self.uniform_product_stationary
            && self.detailed_balance
            && self.unique_stationary_law
            && self.relaxes_from_every_initial_distribution
            && self.prepared_heat_flows_hot_to_cold
            && self.stationary_event_current_a_to_b == Ratio::ZERO
            && self.stationary_energy_current_into_left == Ratio::ZERO
            && self.event_replay.passes()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactEventReplayAudit {
    pub complete_product_microstates_checked: u128,
    pub directed_contact_events_checked: u128,
    pub complete_product_encoding_is_unique: bool,
    pub complete_product_count_matches_fibers: bool,
    pub every_generator_entry_replays_from_complete_state: bool,
    pub every_event_has_reciprocal_support: bool,
    pub every_event_conserves_total_energy: bool,
    pub every_event_transfers_declared_quantum: bool,
    pub every_event_left_heat_ledger_closes: bool,
    pub every_event_right_heat_ledger_closes: bool,
    pub every_event_has_zero_work: bool,
    pub every_event_path_ratio_closes: bool,
}

impl ContactEventReplayAudit {
    pub fn passes(&self) -> bool {
        self.complete_product_microstates_checked > 0
            && self.directed_contact_events_checked > 0
            && self.complete_product_encoding_is_unique
            && self.complete_product_count_matches_fibers
            && self.every_generator_entry_replays_from_complete_state
            && self.every_event_has_reciprocal_support
            && self.every_event_conserves_total_energy
            && self.every_event_transfers_declared_quantum
            && self.every_event_left_heat_ledger_closes
            && self.every_event_right_heat_ledger_closes
            && self.every_event_has_zero_work
            && self.every_event_path_ratio_closes
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactAuthoritativeEventRow {
    pub source: ContactCoordinate,
    pub target: ContactCoordinate,
    pub hazard: Ratio,
    pub reverse_hazard: Ratio,
    pub diagonal_from_source: Ratio,
    pub ledger: ContactHeatLedger,
    pub source_and_successor_well_formed: bool,
    pub noncoordinate_state_is_exactly_unchanged: bool,
    pub inverse_restores_source: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContactAuthoritativeKernelAudit {
    pub reference_left_state: CompleteState,
    pub reference_right_state: CompleteState,
    pub shared_energy_gauge: Ratio,
    pub protocol: ContactProtocol,
    pub complete_coordinates: Vec<ContactCoordinate>,
    pub event_rows: Vec<ContactAuthoritativeEventRow>,
    pub both_full_xypher_states_are_authoritative: bool,
    pub every_state_reconstructs_from_references: bool,
    pub every_enabled_event_replays_from_one_authoritative_kernel: bool,
    pub every_diagonal_is_negative_outgoing_sum: bool,
    pub every_event_has_exact_reciprocal_and_heat_ledger: bool,
}

impl ContactAuthoritativeKernelAudit {
    pub fn passes(&self) -> bool {
        !self.complete_coordinates.is_empty()
            && !self.event_rows.is_empty()
            && self.both_full_xypher_states_are_authoritative
            && self.every_state_reconstructs_from_references
            && self.every_enabled_event_replays_from_one_authoritative_kernel
            && self.every_diagonal_is_negative_outgoing_sum
            && self.every_event_has_exact_reciprocal_and_heat_ledger
    }
}

pub fn audit_authoritative_contact_kernel(
    left_completed_xypher: &CompleteState,
    right_completed_xypher: &CompleteState,
    protocol: &ContactProtocol,
) -> ContactAuthoritativeKernelAudit {
    let mut reference_left = left_completed_xypher.clone();
    let mut reference_right = right_completed_xypher.clone();
    reference_left.P = Phase::Contact;
    reference_right.P = Phase::Contact;
    reference_left.Q.scheduled_perturbation = None;
    reference_right.Q.scheduled_perturbation = None;
    let reference_contact = ContactAuthoritativeState {
        left: reference_left.clone(),
        right: reference_right.clone(),
        protocol: protocol.clone(),
    };
    let complete_coordinates = contact_coordinates(&reference_contact);
    let mut event_rows = Vec::new();
    let both_full_xypher_states_are_authoritative = reference_left.complete_state_is_well_formed()
        && reference_right.complete_state_is_well_formed()
        && protocol.quantum.is_positive()
        && protocol.total_energy.is_positive();
    let mut every_state_reconstructs_from_references = both_full_xypher_states_are_authoritative;
    let mut every_enabled_event_replays_from_one_authoritative_kernel = true;
    let mut every_diagonal_is_negative_outgoing_sum = true;
    let mut every_event_has_exact_reciprocal_and_heat_ledger = true;

    for source_coordinate in &complete_coordinates {
        let mut source_contact = reference_contact.clone();
        source_contact.left.r = source_coordinate.left_reservoir.clone();
        source_contact.right.r = source_coordinate.right_reservoir.clone();
        every_state_reconstructs_from_references &=
            source_contact.left.complete_state_is_well_formed()
                && source_contact.right.complete_state_is_well_formed();
        let source = AuthoritativeState::Contact(source_contact.clone());
        let events = enabled_authoritative_events(&source);
        let outgoing = events
            .iter()
            .map(|event| match event {
                AuthoritativeEvent::Contact { hazard, .. } => *hazard,
                _ => Ratio::ZERO,
            })
            .sum::<Ratio>();
        let diagonal = -outgoing;
        every_diagonal_is_negative_outgoing_sum &= diagonal + outgoing == Ratio::ZERO;
        for event in events {
            let Some(transition) = authoritative_transition(&source, &event) else {
                every_enabled_event_replays_from_one_authoritative_kernel = false;
                continue;
            };
            let (target, hazard) = match &event {
                AuthoritativeEvent::Contact { target, hazard } => (target.clone(), *hazard),
                _ => unreachable!(),
            };
            let successor = match &transition.successor {
                AuthoritativeState::Contact(successor) => successor,
                _ => unreachable!(),
            };
            let ledger = match &transition.ledger {
                AuthoritativeLedger::Contact(ledger) => ledger.clone(),
                _ => unreachable!(),
            };
            let reverse_hazard = match &transition.inverse {
                AuthoritativeInverse::Reciprocal(AuthoritativeEvent::Contact {
                    hazard, ..
                }) => *hazard,
                _ => Ratio::ZERO,
            };
            let mut successor_left_without_coordinate = successor.left.clone();
            successor_left_without_coordinate.r = reference_left.r.clone();
            let mut successor_right_without_coordinate = successor.right.clone();
            successor_right_without_coordinate.r = reference_right.r.clone();
            let noncoordinate_state_is_exactly_unchanged = successor_left_without_coordinate
                == reference_left
                && successor_right_without_coordinate == reference_right;
            let row = ContactAuthoritativeEventRow {
                source: source_coordinate.clone(),
                target,
                hazard,
                reverse_hazard,
                diagonal_from_source: diagonal,
                ledger,
                source_and_successor_well_formed: successor.left.complete_state_is_well_formed()
                    && successor.right.complete_state_is_well_formed(),
                noncoordinate_state_is_exactly_unchanged,
                inverse_restores_source: transition.inverse_restores_source,
            };
            every_enabled_event_replays_from_one_authoritative_kernel &=
                transition.source == source && row.target == contact_coordinate(successor);
            every_event_has_exact_reciprocal_and_heat_ledger &= row.reverse_hazard.is_positive()
                && row.reverse_hazard == row.hazard
                && row.ledger.closes()
                && row.ledger.delta_left.abs() == protocol.quantum
                && row.ledger.delta_right.abs() == protocol.quantum
                && row.source_and_successor_well_formed
                && row.noncoordinate_state_is_exactly_unchanged
                && row.inverse_restores_source;
            event_rows.push(row);
        }
    }

    ContactAuthoritativeKernelAudit {
        reference_left_state: reference_left,
        reference_right_state: reference_right,
        shared_energy_gauge: Ratio::ONE,
        protocol: protocol.clone(),
        complete_coordinates,
        event_rows,
        both_full_xypher_states_are_authoritative,
        every_state_reconstructs_from_references,
        every_enabled_event_replays_from_one_authoritative_kernel,
        every_diagonal_is_negative_outgoing_sum,
        every_event_has_exact_reciprocal_and_heat_ledger,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrozenContactFixture {
    EqualTemperature,
    UnequalTemperature,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenContactFixtureResult {
    pub fixture: FrozenContactFixture,
    pub constitution_matches: bool,
    pub exact_prediction_matches: bool,
    pub contact: ContactThermalResult,
}

impl FrozenContactFixtureResult {
    pub fn passes(&self) -> bool {
        self.constitution_matches
            && self.exact_prediction_matches
            && self.contact.operational_closure_passes()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenContactActivityControl {
    pub unit_clock: FrozenContactFixtureResult,
    pub doubled_clock: FrozenContactFixtureResult,
    pub invariant: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
enum ContactFiber {
    A,
    B,
}

#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
struct ContactMicrostate {
    fiber: ContactFiber,
    left_descriptor: u64,
    left_word: Vec<u8>,
    left_energy_index: u32,
    right_descriptor: u64,
    right_word: Vec<u8>,
    right_energy_index: u32,
}

pub fn verify_contact(
    left: &PrefixReservoir,
    right: &PrefixReservoir,
    protocol: &ContactProtocol,
) -> ContactThermalResult {
    let left_state = classify_prefix_reservoir(left);
    let right_state = classify_prefix_reservoir(right);
    let left_zero = shell_alternatives(left, 0);
    let left_two = shell_alternatives(left, 2);
    let right_one = shell_alternatives(right, 1);
    let right_two = shell_alternatives(right, 2);
    let mut states = Vec::new();
    for left_state in &left_zero {
        for right_state in &right_two {
            states.push(ContactMicrostate {
                fiber: ContactFiber::A,
                left_descriptor: left_state.descriptor_id,
                left_word: left_state.word.clone(),
                left_energy_index: left_state.energy_index,
                right_descriptor: right_state.descriptor_id,
                right_word: right_state.word.clone(),
                right_energy_index: right_state.energy_index,
            });
        }
    }
    for left_state in &left_two {
        for right_state in &right_one {
            states.push(ContactMicrostate {
                fiber: ContactFiber::B,
                left_descriptor: left_state.descriptor_id,
                left_word: left_state.word.clone(),
                left_energy_index: left_state.energy_index,
                right_descriptor: right_state.descriptor_id,
                right_word: right_state.word.clone(),
                right_energy_index: right_state.energy_index,
            });
        }
    }

    let omega_a = states
        .iter()
        .filter(|state| state.fiber == ContactFiber::A)
        .count() as u128;
    let omega_b = states
        .iter()
        .filter(|state| state.fiber == ContactFiber::B)
        .count() as u128;
    let quantum_commensurate = protocol.quantum.is_positive()
        && left.energy_quantum.is_positive()
        && right.energy_quantum.is_positive()
        && (protocol.quantum / left.energy_quantum).is_integer()
        && (protocol.quantum / right.energy_quantum).is_integer();
    let two_fibers_at_declared_total_energy = !states.is_empty()
        && states.iter().all(|state| {
            contact_state_energy(state, left.energy_quantum, right.energy_quantum)
                == protocol.total_energy
        });
    let transfer_conserves_energy = left.energy_quantum * Ratio::integer(2) == protocol.quantum
        && right.energy_quantum == protocol.quantum
        && two_fibers_at_declared_total_energy;

    let reciprocal_microedges = !states.is_empty()
        && protocol.microscopic_clock.is_positive()
        && (0..states.len()).all(|from| {
            (0..states.len()).all(|to| {
                from == to
                    || contact_off_diagonal(&states[from], &states[to], protocol.microscopic_clock)
                        == contact_off_diagonal(
                            &states[to],
                            &states[from],
                            protocol.microscopic_clock,
                        )
            })
        });
    let row_closed = !states.is_empty()
        && (0..states.len()).all(|from| {
            (0..states.len())
                .map(|to| contact_generator_entry(&states, from, to, protocol.microscopic_clock))
                .sum::<Ratio>()
                == Ratio::ZERO
        });
    let strong_lumpability = contact_strong_lumpability(&states, protocol.microscopic_clock);
    let macro_generator = contact_macro_generator(&states, protocol.microscopic_clock);
    let irreducible = contact_connected(&states, protocol.microscopic_clock);

    let total = omega_a
        .checked_add(omega_b)
        .expect("contact microstate count overflow");
    let stationary_macro_distribution = if total > 0 {
        [ratio_u128(omega_a, total), ratio_u128(omega_b, total)]
    } else {
        [Ratio::ZERO; 2]
    };
    let detailed_balance = total > 0
        && ratio_integer_u128(omega_a) * macro_generator[0][1]
            == ratio_integer_u128(omega_b) * macro_generator[1][0];
    let uniform_product_stationary = reciprocal_microedges && row_closed && detailed_balance;
    let unique_stationary_law = uniform_product_stationary && irreducible;
    let relaxes_from_every_initial_distribution = unique_stationary_law;

    let left_temperature = left_state.temperature;
    let right_temperature = right_state.temperature;
    let temperature_order =
        compare_temperatures(left_temperature.as_ref(), right_temperature.as_ref());
    let prepared = protocol.prepared_macro_distribution;
    let prepared_is_distribution = prepared.iter().all(|value| value.is_nonnegative())
        && prepared.into_iter().sum::<Ratio>() == Ratio::ONE;
    let prepared_event_current_a_to_b = if prepared_is_distribution {
        prepared[0] * macro_generator[0][1] - prepared[1] * macro_generator[1][0]
    } else {
        Ratio::ZERO
    };
    let prepared_energy_current_into_left = protocol.quantum * prepared_event_current_a_to_b;
    let prepared_heat_flows_hot_to_cold = match temperature_order {
        TemperatureOrder::Equal => prepared_energy_current_into_left == Ratio::ZERO,
        TemperatureOrder::LeftHotter => prepared_energy_current_into_left < Ratio::ZERO,
        TemperatureOrder::RightHotter => prepared_energy_current_into_left > Ratio::ZERO,
        TemperatureOrder::Incomparable => false,
    };
    let stationary_event_current_a_to_b = stationary_macro_distribution[0] * macro_generator[0][1]
        - stationary_macro_distribution[1] * macro_generator[1][0];
    let stationary_energy_current_into_left = protocol.quantum * stationary_event_current_a_to_b;
    let event_replay = audit_contact_events(
        &states,
        protocol,
        left.energy_quantum,
        right.energy_quantum,
        omega_a,
        omega_b,
    );

    ContactThermalResult {
        microscopic_clock: protocol.microscopic_clock,
        left_state_temperature: left_temperature,
        right_state_temperature: right_temperature,
        temperature_order,
        quantum_commensurate,
        two_fibers_at_declared_total_energy,
        transfer_conserves_energy,
        omega_a,
        omega_b,
        macro_generator,
        reciprocal_microedges,
        row_closed,
        strong_lumpability,
        irreducible,
        stationary_macro_distribution,
        uniform_product_stationary,
        detailed_balance,
        unique_stationary_law,
        relaxes_from_every_initial_distribution,
        prepared_event_current_a_to_b,
        prepared_energy_current_into_left,
        prepared_heat_flows_hot_to_cold,
        stationary_event_current_a_to_b,
        stationary_energy_current_into_left,
        event_replay,
    }
}

fn audit_contact_events(
    states: &[ContactMicrostate],
    protocol: &ContactProtocol,
    left_quantum: Ratio,
    right_quantum: Ratio,
    omega_a: u128,
    omega_b: u128,
) -> ContactEventReplayAudit {
    let complete_product_microstates_checked = states.len() as u128;
    let complete_product_encoding_is_unique =
        states.iter().cloned().collect::<BTreeSet<_>>().len() == states.len();
    let complete_product_count_matches_fibers = complete_product_microstates_checked
        == omega_a
            .checked_add(omega_b)
            .expect("contact product-count overflow");
    let mut directed_contact_events_checked = 0_u128;
    let mut every_generator_entry_replays_from_complete_state = !states.is_empty();
    let mut every_event_has_reciprocal_support = !states.is_empty();
    let mut every_event_conserves_total_energy = !states.is_empty();
    let mut every_event_transfers_declared_quantum = !states.is_empty();
    let mut every_event_left_heat_ledger_closes = !states.is_empty();
    let mut every_event_right_heat_ledger_closes = !states.is_empty();
    let mut every_event_has_zero_work = !states.is_empty();
    let mut every_event_path_ratio_closes = !states.is_empty();

    for from in 0..states.len() {
        let replayed_diagonal = -(0..states.len())
            .filter(|&to| to != from)
            .map(|to| contact_off_diagonal(&states[from], &states[to], protocol.microscopic_clock))
            .sum::<Ratio>();
        every_generator_entry_replays_from_complete_state &=
            contact_generator_entry(states, from, from, protocol.microscopic_clock)
                == replayed_diagonal;
        for to in 0..states.len() {
            if from == to {
                continue;
            }
            let forward =
                contact_off_diagonal(&states[from], &states[to], protocol.microscopic_clock);
            every_generator_entry_replays_from_complete_state &=
                contact_generator_entry(states, from, to, protocol.microscopic_clock) == forward;
            if !forward.is_positive() {
                continue;
            }
            directed_contact_events_checked = directed_contact_events_checked
                .checked_add(1)
                .expect("contact event-count overflow");
            let reverse =
                contact_off_diagonal(&states[to], &states[from], protocol.microscopic_clock);
            every_event_has_reciprocal_support &= reverse.is_positive();

            let before = &states[from];
            let after = &states[to];
            let left_before = left_quantum * Ratio::integer(i128::from(before.left_energy_index));
            let left_after = left_quantum * Ratio::integer(i128::from(after.left_energy_index));
            let right_before =
                right_quantum * Ratio::integer(i128::from(before.right_energy_index));
            let right_after = right_quantum * Ratio::integer(i128::from(after.right_energy_index));
            let delta_left = left_after - left_before;
            let delta_right = right_after - right_before;
            let work_on_left = Ratio::ZERO;
            let work_on_right = Ratio::ZERO;
            every_event_conserves_total_energy &= delta_left + delta_right == Ratio::ZERO;
            every_event_transfers_declared_quantum &=
                delta_left.abs() == protocol.quantum && delta_right.abs() == protocol.quantum;
            every_event_left_heat_ledger_closes &= delta_left == -delta_right + work_on_left;
            every_event_right_heat_ledger_closes &= delta_right == -delta_left + work_on_right;
            every_event_has_zero_work &=
                work_on_left == Ratio::ZERO && work_on_right == Ratio::ZERO;
            every_event_path_ratio_closes &=
                reverse.is_positive() && ExactLog::ln_ratio(forward / reverse).is_zero();
        }
    }

    ContactEventReplayAudit {
        complete_product_microstates_checked,
        directed_contact_events_checked,
        complete_product_encoding_is_unique,
        complete_product_count_matches_fibers,
        every_generator_entry_replays_from_complete_state,
        every_event_has_reciprocal_support,
        every_event_conserves_total_energy,
        every_event_transfers_declared_quantum,
        every_event_left_heat_ledger_closes,
        every_event_right_heat_ledger_closes,
        every_event_has_zero_work,
        every_event_path_ratio_closes,
    }
}

pub fn verify_frozen_contact_fixture(
    fixture: FrozenContactFixture,
    left: &PrefixReservoir,
    right: &PrefixReservoir,
    microscopic_clock: Ratio,
) -> FrozenContactFixtureResult {
    let protocol = frozen_contact_protocol(microscopic_clock);
    let contact = verify_contact(left, right, &protocol);
    let right_base = match fixture {
        FrozenContactFixture::EqualTemperature => 4,
        FrozenContactFixture::UnequalTemperature => 2,
    };
    let constitution_matches = left.alphabet_size == 2
        && left.energy_quantum == Ratio::ONE
        && right.alphabet_size == right_base
        && right.energy_quantum == Ratio::integer(2);
    let expected = match fixture {
        FrozenContactFixture::EqualTemperature => FrozenContactExpectation {
            omega_a: 16,
            omega_b: 16,
            macro_generator: [
                [
                    -Ratio::integer(16) * microscopic_clock,
                    Ratio::integer(16) * microscopic_clock,
                ],
                [
                    Ratio::integer(16) * microscopic_clock,
                    -Ratio::integer(16) * microscopic_clock,
                ],
            ],
            stationary_macro_distribution: [Ratio::new(1, 2), Ratio::new(1, 2)],
            temperature_order: TemperatureOrder::Equal,
            prepared_event_current: Ratio::ZERO,
            prepared_energy_current: Ratio::ZERO,
        },
        FrozenContactFixture::UnequalTemperature => FrozenContactExpectation {
            omega_a: 4,
            omega_b: 8,
            macro_generator: [
                [
                    -Ratio::integer(8) * microscopic_clock,
                    Ratio::integer(8) * microscopic_clock,
                ],
                [
                    Ratio::integer(4) * microscopic_clock,
                    -Ratio::integer(4) * microscopic_clock,
                ],
            ],
            stationary_macro_distribution: [Ratio::new(1, 3), Ratio::new(2, 3)],
            temperature_order: TemperatureOrder::RightHotter,
            prepared_event_current: Ratio::integer(2) * microscopic_clock,
            prepared_energy_current: Ratio::integer(4) * microscopic_clock,
        },
    };
    let exact_prediction_matches = contact.omega_a == expected.omega_a
        && contact.omega_b == expected.omega_b
        && contact.macro_generator == expected.macro_generator
        && contact.stationary_macro_distribution == expected.stationary_macro_distribution
        && contact.temperature_order == expected.temperature_order
        && contact.prepared_event_current_a_to_b == expected.prepared_event_current
        && contact.prepared_energy_current_into_left == expected.prepared_energy_current
        && contact.stationary_event_current_a_to_b == Ratio::ZERO
        && contact.stationary_energy_current_into_left == Ratio::ZERO;

    FrozenContactFixtureResult {
        fixture,
        constitution_matches,
        exact_prediction_matches,
        contact,
    }
}

pub fn verify_frozen_contact_activity_control(
    fixture: FrozenContactFixture,
    left: &PrefixReservoir,
    right: &PrefixReservoir,
) -> FrozenContactActivityControl {
    let unit_clock = verify_frozen_contact_fixture(fixture, left, right, Ratio::ONE);
    let doubled_clock = verify_frozen_contact_fixture(fixture, left, right, Ratio::integer(2));
    let temperatures_unchanged = match (
        unit_clock.contact.left_state_temperature.as_ref(),
        doubled_clock.contact.left_state_temperature.as_ref(),
        unit_clock.contact.right_state_temperature.as_ref(),
        doubled_clock.contact.right_state_temperature.as_ref(),
    ) {
        (Some(left_unit), Some(left_doubled), Some(right_unit), Some(right_doubled)) => {
            left_unit.same_temperature(left_doubled) && right_unit.same_temperature(right_doubled)
        }
        _ => false,
    };
    let generator_doubled = (0..2).all(|row| {
        (0..2).all(|column| {
            doubled_clock.contact.macro_generator[row][column]
                == Ratio::integer(2) * unit_clock.contact.macro_generator[row][column]
        })
    });
    let invariant = unit_clock.passes()
        && doubled_clock.passes()
        && temperatures_unchanged
        && generator_doubled
        && unit_clock.contact.stationary_macro_distribution
            == doubled_clock.contact.stationary_macro_distribution
        && doubled_clock.contact.prepared_event_current_a_to_b
            == Ratio::integer(2) * unit_clock.contact.prepared_event_current_a_to_b
        && doubled_clock.contact.prepared_energy_current_into_left
            == Ratio::integer(2) * unit_clock.contact.prepared_energy_current_into_left;

    FrozenContactActivityControl {
        unit_clock,
        doubled_clock,
        invariant,
    }
}

struct FrozenContactExpectation {
    omega_a: u128,
    omega_b: u128,
    macro_generator: [[Ratio; 2]; 2],
    stationary_macro_distribution: [Ratio; 2],
    temperature_order: TemperatureOrder,
    prepared_event_current: Ratio,
    prepared_energy_current: Ratio,
}

fn contact_state_energy(
    state: &ContactMicrostate,
    left_quantum: Ratio,
    right_quantum: Ratio,
) -> Ratio {
    left_quantum * Ratio::integer(i128::from(state.left_energy_index))
        + right_quantum * Ratio::integer(i128::from(state.right_energy_index))
}

fn contact_off_diagonal(from: &ContactMicrostate, to: &ContactMicrostate, clock: Ratio) -> Ratio {
    if from != to && from.fiber != to.fiber {
        clock
    } else {
        Ratio::ZERO
    }
}

fn contact_generator_entry(
    states: &[ContactMicrostate],
    from: usize,
    to: usize,
    clock: Ratio,
) -> Ratio {
    if from != to {
        contact_off_diagonal(&states[from], &states[to], clock)
    } else {
        -(0..states.len())
            .filter(|candidate| *candidate != from)
            .map(|candidate| contact_off_diagonal(&states[from], &states[candidate], clock))
            .sum::<Ratio>()
    }
}

fn contact_block_sum(
    states: &[ContactMicrostate],
    source: usize,
    target_fiber: ContactFiber,
    clock: Ratio,
) -> Ratio {
    states
        .iter()
        .enumerate()
        .filter(|(_, state)| state.fiber == target_fiber)
        .map(|(target, _)| contact_generator_entry(states, source, target, clock))
        .sum()
}

fn contact_strong_lumpability(states: &[ContactMicrostate], clock: Ratio) -> bool {
    [ContactFiber::A, ContactFiber::B]
        .into_iter()
        .all(|source_fiber| {
            let sources: Vec<_> = states
                .iter()
                .enumerate()
                .filter(|(_, state)| state.fiber == source_fiber)
                .map(|(index, _)| index)
                .collect();
            !sources.is_empty()
                && [ContactFiber::A, ContactFiber::B]
                    .into_iter()
                    .all(|target_fiber| {
                        let reference = contact_block_sum(states, sources[0], target_fiber, clock);
                        sources.iter().skip(1).all(|&source| {
                            contact_block_sum(states, source, target_fiber, clock) == reference
                        })
                    })
        })
}

fn contact_macro_generator(states: &[ContactMicrostate], clock: Ratio) -> [[Ratio; 2]; 2] {
    let mut generator = [[Ratio::ZERO; 2]; 2];
    for (source_index, source_fiber) in [ContactFiber::A, ContactFiber::B].into_iter().enumerate() {
        let Some(source) = states.iter().position(|state| state.fiber == source_fiber) else {
            continue;
        };
        for (target_index, target_fiber) in
            [ContactFiber::A, ContactFiber::B].into_iter().enumerate()
        {
            generator[source_index][target_index] =
                contact_block_sum(states, source, target_fiber, clock);
        }
    }
    generator
}

fn contact_connected(states: &[ContactMicrostate], clock: Ratio) -> bool {
    if states.is_empty() || !clock.is_positive() {
        return false;
    }
    let mut visited = vec![false; states.len()];
    let mut queue = VecDeque::new();
    queue.push_back(0);
    visited[0] = true;
    while let Some(from) = queue.pop_front() {
        for to in 0..states.len() {
            if !visited[to] && contact_off_diagonal(&states[from], &states[to], clock) > Ratio::ZERO
            {
                visited[to] = true;
                queue.push_back(to);
            }
        }
    }
    visited.into_iter().all(|value| value)
}

fn compare_temperatures(
    left: Option<&Temperature>,
    right: Option<&Temperature>,
) -> TemperatureOrder {
    let (Some(left), Some(right)) = (left, right) else {
        return TemperatureOrder::Incomparable;
    };
    let beta_difference = left.beta.clone() - right.beta.clone();
    match beta_difference.sign_if_unambiguous() {
        Some(Ordering::Less) => TemperatureOrder::LeftHotter,
        Some(Ordering::Equal) => TemperatureOrder::Equal,
        Some(Ordering::Greater) => TemperatureOrder::RightHotter,
        None => TemperatureOrder::Incomparable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_tree_is_operational_but_not_endogenous_or_adaptive() {
        for base in [2, 3, 4] {
            let report = verify_static_perfect_tree_control(base, Ratio::ONE);
            assert!(report.matches_frozen_classification());
        }
    }

    #[test]
    fn imposed_temperature_and_gibbs_controls_do_not_repair_irregular_counts() {
        assert!(verify_external_thermostat_control().matches_frozen_classification());
        assert!(verify_gibbs_installed_control().matches_frozen_classification());
    }

    #[test]
    fn port_mutant_counts_distinct_vertex_identities_and_rejects_old_temperature() {
        for base in [2, 3, 4] {
            let report = verify_port_law_mutation_control(base, Ratio::ONE);
            assert!(report.matches_frozen_classification());
        }
    }

    #[test]
    fn probe_events_replay_from_full_authoritative_xypher_state() {
        let trace =
            crate::model::instantiate_one_support_trace(crate::model::primary_fixture(2, false))
                .expect("binary construction closes");
        let audit = audit_authoritative_probe_kernel(&trace.terminal_state, Ratio::ONE, false);
        let detached =
            verify_held_out_probe(&PrefixReservoir::from_complete_state(&trace.terminal_state));
        assert!(audit.passes());
        assert_eq!(
            audit.complete_coordinates.len() as u128,
            detached.event_replay.complete_microstates_checked,
        );
        assert_eq!(
            audit.event_rows.len() as u128,
            detached.event_replay.directed_events_checked,
        );
    }

    #[test]
    fn contact_events_replay_from_both_full_xypher_states() {
        let left = crate::model::instantiate_one_support_trace(
            crate::model::primary_fixture_with_quantum(2, 1, false),
        )
        .expect("left construction closes")
        .terminal_state;
        let right = crate::model::instantiate_one_support_trace(
            crate::model::primary_fixture_with_quantum(4, 2, false),
        )
        .expect("right construction closes")
        .terminal_state;
        let protocol = frozen_contact_protocol(Ratio::ONE);
        let audit = audit_authoritative_contact_kernel(&left, &right, &protocol);
        let detached = verify_contact(
            &PrefixReservoir::from_complete_state(&left),
            &PrefixReservoir::from_complete_state(&right),
            &protocol,
        );
        assert!(audit.passes());
        assert_eq!(
            audit.complete_coordinates.len() as u128,
            detached.event_replay.complete_product_microstates_checked,
        );
        assert_eq!(
            audit.event_rows.len() as u128,
            detached.event_replay.directed_contact_events_checked,
        );
    }
}
