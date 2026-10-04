use std::collections::{BTreeMap, BTreeSet};

use crate::exact::Ratio;

pub type DescriptorId = u32;
pub type SemanticRecordId = usize;
pub type OperationRecordId = usize;
pub type WorkCellId = usize;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Word(pub Vec<u8>);

impl Word {
    pub fn root() -> Self {
        Self(Vec::new())
    }

    pub fn child(&self, port: u8) -> Self {
        let mut symbols = self.0.clone();
        symbols.push(port);
        Self(symbols)
    }

    pub fn depth(&self) -> usize {
        self.0.len()
    }

    pub fn parent(&self) -> Option<Self> {
        let mut symbols = self.0.clone();
        symbols.pop()?;
        Some(Self(symbols))
    }

    pub fn final_port(&self) -> Option<u8> {
        self.0.last().copied()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReservoirNode {
    pub word: Word,
    pub descriptor: DescriptorId,
    pub energy_index: usize,
    pub executable: bool,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct IncidenceEdge {
    pub parent: Word,
    pub port: u8,
    pub child: Word,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigurationSpace {
    pub reservoir: BTreeMap<Word, ReservoirNode>,
    pub workspace: BTreeMap<Word, ReservoirNode>,
    pub structural_incidence: BTreeSet<IncidenceEdge>,
}

impl ConfigurationSpace {
    pub fn shell_words(&self, depth: usize) -> Vec<Word> {
        self.reservoir
            .keys()
            .filter(|word| word.depth() == depth)
            .cloned()
            .collect()
    }

    pub fn shell_count(&self, depth: usize) -> usize {
        self.shell_words(depth).len()
    }

    pub fn deepest_promoted_depth(&self) -> usize {
        self.reservoir.keys().map(Word::depth).max().unwrap_or(0)
    }

    pub fn contains_executable(&self, word: &Word) -> bool {
        self.reservoir.get(word).is_some_and(|node| node.executable)
    }

    pub fn locally_injective(&self) -> bool {
        let descriptor_count = self
            .reservoir
            .values()
            .chain(self.workspace.values())
            .map(|node| node.descriptor)
            .collect::<BTreeSet<_>>()
            .len();
        descriptor_count == self.reservoir.len() + self.workspace.len()
            && self
                .reservoir
                .iter()
                .chain(self.workspace.iter())
                .all(|(word, node)| {
                    word == &node.word
                        && node.energy_index == word.depth()
                        && (word.depth() == 0
                            || word.parent().is_some_and(|parent| {
                                self.reservoir.contains_key(&parent)
                                    || self.workspace.contains_key(&parent)
                            }))
                })
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CandidateKind {
    Fresh,
    Alias,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PortRef {
    pub parent: Word,
    pub port: u8,
}

impl PortRef {
    pub fn child(&self) -> Word {
        self.parent.child(self.port)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CandidateKey {
    pub episode: u32,
    pub port: PortRef,
    pub kind: CandidateKind,
}

impl CandidateKey {
    pub fn bucket(&self) -> BucketKey {
        BucketKey {
            port: self.port.port,
            kind: self.kind,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AttemptLane {
    Build,
    Repair,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttemptBit {
    pub id: usize,
    pub lane: AttemptLane,
    pub identity: Option<CandidateKey>,
    pub attempted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttemptBitStore {
    pub bits: Vec<AttemptBit>,
}

impl AttemptBitStore {
    fn with_construction_identities(
        identities: impl IntoIterator<Item = CandidateKey>,
        repair_slots: usize,
    ) -> Self {
        let mut bits = identities
            .into_iter()
            .enumerate()
            .map(|(id, identity)| AttemptBit {
                id,
                lane: AttemptLane::Build,
                identity: Some(identity),
                attempted: false,
            })
            .collect::<Vec<_>>();
        let first_repair_id = bits.len();
        bits.extend((0..repair_slots).map(|offset| AttemptBit {
            id: first_repair_id + offset,
            lane: AttemptLane::Repair,
            identity: None,
            attempted: false,
        }));
        Self { bits }
    }

    pub fn declared_count(&self) -> usize {
        self.bits.len()
    }

    pub fn attempted_count(&self) -> usize {
        self.bits.iter().filter(|bit| bit.attempted).count()
    }

    pub fn attempted_identities(&self) -> BTreeSet<CandidateKey> {
        self.bits
            .iter()
            .filter(|bit| bit.attempted)
            .filter_map(|bit| bit.identity.clone())
            .collect()
    }

    pub fn contains_attempted(&self, identity: &CandidateKey) -> bool {
        self.bits
            .iter()
            .any(|bit| bit.identity.as_ref() == Some(identity) && bit.attempted)
    }

    pub fn has_unattempted(&self, identity: &CandidateKey) -> bool {
        self.unattempted_index(identity).is_some()
    }

    fn unattempted_index(&self, identity: &CandidateKey) -> Option<usize> {
        self.bits
            .iter()
            .position(|bit| bit.identity.as_ref() == Some(identity) && !bit.attempted)
    }

    fn blank_repair_slots(&self) -> usize {
        self.bits
            .iter()
            .filter(|bit| {
                bit.lane == AttemptLane::Repair && bit.identity.is_none() && !bit.attempted
            })
            .count()
    }

    fn bind_repair_episode(&mut self, episode: u32, port: &PortRef) -> bool {
        if self.blank_repair_slots() != 2 {
            return false;
        }
        let identities = [CandidateKind::Fresh, CandidateKind::Alias].map(|kind| CandidateKey {
            episode,
            port: port.clone(),
            kind,
        });
        for (bit, identity) in self
            .bits
            .iter_mut()
            .filter(|bit| bit.lane == AttemptLane::Repair)
            .zip(identities)
        {
            bit.identity = Some(identity);
        }
        true
    }

    fn has_repair_pair(&self, episode: u32, port: &PortRef) -> bool {
        [CandidateKind::Fresh, CandidateKind::Alias]
            .into_iter()
            .all(|kind| {
                self.bits.iter().any(|bit| {
                    bit.lane == AttemptLane::Repair
                        && bit.identity.as_ref()
                            == Some(&CandidateKey {
                                episode,
                                port: port.clone(),
                                kind,
                            })
                })
            })
    }

    pub fn is_well_formed(&self) -> bool {
        let ids = self.bits.iter().map(|bit| bit.id).collect::<BTreeSet<_>>();
        let identities = self
            .bits
            .iter()
            .filter_map(|bit| bit.identity.clone())
            .collect::<Vec<_>>();
        let unique_identities = identities.iter().cloned().collect::<BTreeSet<_>>();
        let repair = self
            .bits
            .iter()
            .filter(|bit| bit.lane == AttemptLane::Repair)
            .collect::<Vec<_>>();
        let repair_binding_is_coherent = repair.is_empty()
            || repair
                .iter()
                .all(|bit| bit.identity.is_none() && !bit.attempted)
            || (repair.len() == 2
                && repair.iter().all(|bit| bit.identity.is_some())
                && repair
                    .iter()
                    .filter_map(|bit| bit.identity.as_ref())
                    .map(|identity| (identity.episode, &identity.port))
                    .all(|pair| {
                        let first = repair[0].identity.as_ref().expect("bound repair identity");
                        pair == (first.episode, &first.port)
                    })
                && repair
                    .iter()
                    .filter_map(|bit| bit.identity.as_ref())
                    .map(|identity| identity.kind)
                    .collect::<BTreeSet<_>>()
                    == BTreeSet::from([CandidateKind::Fresh, CandidateKind::Alias]));
        ids.len() == self.bits.len()
            && ids.iter().copied().eq(0..self.bits.len())
            && identities.len() == unique_identities.len()
            && self
                .bits
                .iter()
                .all(|bit| !bit.attempted || bit.identity.is_some())
            && self.bits.iter().all(|bit| {
                bit.lane != AttemptLane::Build
                    || bit
                        .identity
                        .as_ref()
                        .is_some_and(|identity| identity.episode == 0)
            })
            && repair_binding_is_coherent
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BucketKey {
    pub port: u8,
    pub kind: CandidateKind,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SemanticState {
    Blank,
    Pending,
    Success,
    Failure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticRecord {
    pub id: SemanticRecordId,
    pub candidate: Option<CandidateKey>,
    pub bucket: Option<BucketKey>,
    pub descriptor: Option<DescriptorId>,
    pub state: SemanticState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhysicalMemory {
    pub records: Vec<SemanticRecord>,
}

impl PhysicalMemory {
    pub fn blank(capacity: usize) -> Self {
        Self {
            records: (0..capacity)
                .map(|id| SemanticRecord {
                    id,
                    candidate: None,
                    bucket: None,
                    descriptor: None,
                    state: SemanticState::Blank,
                })
                .collect(),
        }
    }

    pub fn terminal_counts(&self, bucket: BucketKey) -> (u32, u32) {
        self.records
            .iter()
            .filter(|record| record.bucket == Some(bucket))
            .fold((0_u32, 0_u32), |(successes, trials), record| {
                match record.state {
                    SemanticState::Success => (successes + 1, trials + 1),
                    SemanticState::Failure => (successes, trials + 1),
                    SemanticState::Blank | SemanticState::Pending => (successes, trials),
                }
            })
    }

    fn next_blank(&self) -> Option<SemanticRecordId> {
        self.records
            .iter()
            .find(|record| record.state == SemanticState::Blank && record.bucket.is_none())
            .map(|record| record.id)
    }

    fn reset(&mut self, id: SemanticRecordId) {
        self.records[id] = SemanticRecord {
            id,
            candidate: None,
            bucket: None,
            descriptor: None,
            state: SemanticState::Blank,
        };
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorkLane {
    Build,
    Repair,
    Observation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorkCell {
    pub id: WorkCellId,
    pub charged: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkPool {
    pub lane: WorkLane,
    pub locked: bool,
    pub cells: Vec<WorkCell>,
}

impl WorkPool {
    fn new(lane: WorkLane, capacity: usize, locked: bool) -> Self {
        Self {
            lane,
            locked,
            cells: (0..capacity)
                .map(|id| WorkCell { id, charged: true })
                .collect(),
        }
    }

    pub fn charged_count(&self) -> usize {
        self.cells.iter().filter(|cell| cell.charged).count()
    }

    fn next_charged(&self) -> Option<WorkCellId> {
        if self.locked {
            None
        } else {
            self.cells
                .iter()
                .find(|cell| cell.charged)
                .map(|cell| cell.id)
        }
    }
}

#[allow(non_snake_case)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkStore {
    pub B_build: WorkPool,
    pub B_repair: WorkPool,
    pub B_obs: WorkPool,
    pub epsilon_W: u64,
}

impl WorkStore {
    pub fn pool(&self, lane: WorkLane) -> &WorkPool {
        match lane {
            WorkLane::Build => &self.B_build,
            WorkLane::Repair => &self.B_repair,
            WorkLane::Observation => &self.B_obs,
        }
    }

    pub fn pool_mut(&mut self, lane: WorkLane) -> &mut WorkPool {
        match lane {
            WorkLane::Build => &mut self.B_build,
            WorkLane::Repair => &mut self.B_repair,
            WorkLane::Observation => &mut self.B_obs,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RecordLane {
    Build,
    Repair,
    Observation,
    ExternalPerturbation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Outcome {
    Success,
    Failure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationPayload {
    pub bucket: BucketKey,
    pub outcome: Outcome,
    pub consumed: bool,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProbeState {
    pub mesostate: u8,
    pub microstate: u8,
}

/// Forbidden observer values are deliberately external to `CompleteState`.
/// They can be varied by a causal-non-use harness, but neither the builder
/// view nor an inverse record can capture them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PoisonFields {
    pub beta: i128,
    pub target_count: i128,
    pub held_out_rate: i128,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum DescriptorOrder {
    Forward,
    Reversed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Phase {
    Build { depth: usize },
    Credit { depth: usize, maintenance: bool },
    Promote { depth: usize, maintenance: bool },
    Repair { missing: PortRef },
    Thermal,
    Contact,
}

// BEGIN AUTHORITATIVE_BUILDER_SURFACE
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchedulerState {
    pub episode: u32,
    pub cursor: Option<PortRef>,
    pub repair_target: Option<PortRef>,
    pub attempt_bits: AttemptBitStore,
    pub observation_payload: Option<ObservationPayload>,
    pub absorb_before_candidates: bool,
    pub scheduled_perturbation: Option<Word>,
    pub descriptor_order: DescriptorOrder,
    pub external_perturbation_cell: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteState {
    pub Gamma: ConfigurationSpace,
    pub r: Word,
    pub A: Vec<u8>,
    pub lambda: u64,
    pub X: ProbeState,
    pub F: BTreeSet<DescriptorId>,
    pub B_W: WorkStore,
    pub M: PhysicalMemory,
    pub O: OperationBank,
    pub P: Phase,
    pub Q: SchedulerState,
}

/// The only authoritative data surface visible to BUILD and REPAIR policy.
/// In particular it contains no probe coordinate, occupied thermal state,
/// temperature, state-count target, held-out traffic, or poison envelope.
#[allow(non_snake_case)]
#[derive(Clone, Copy, Debug)]
pub struct BuilderView<'a> {
    pub Gamma: &'a ConfigurationSpace,
    pub A: &'a [u8],
    pub F: &'a BTreeSet<DescriptorId>,
    pub B_W: &'a WorkStore,
    pub M: &'a PhysicalMemory,
    pub O: &'a OperationBank,
    pub P: &'a Phase,
    pub Q: &'a SchedulerState,
}

// BEGIN AUTHORITATIVE_BUILDER_ADMISSION

/// A concrete request to steer construction toward observer-selected shell
/// counts. It is deliberately not part of [`BuilderView`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetCountRequest {
    pub shell_counts: Vec<u128>,
    pub target_multiplicity_ratios: Vec<Ratio>,
    pub global_deficit: Vec<i128>,
}

/// A concrete request to train construction against sealed held-out traffic.
/// It is deliberately not part of [`BuilderView`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HeldOutTrafficRequest {
    pub labelled_rate_pairs: Vec<(Ratio, Ratio)>,
    pub agreement_reward: Ratio,
}

#[derive(Clone, Copy, Debug)]
pub enum BuilderRequest<'a> {
    Local(&'a CompleteState),
    TargetCount(&'a CompleteState, &'a TargetCountRequest),
    HeldOutTraffic(&'a CompleteState, &'a HeldOutTrafficRequest),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuilderAdmissionError {
    TargetCountInput,
    HeldOutTrafficInput,
}

/// The sole admission boundary for data presented to BUILD or REPAIR policy.
/// Observer-selected targets and held-out traffic are rejected before a
/// restricted projection can be constructed.
pub fn admit_builder_request<'a>(
    request: BuilderRequest<'a>,
) -> Result<BuilderView<'a>, BuilderAdmissionError> {
    match request {
        BuilderRequest::Local(state) => Ok(state.restricted_builder_view()),
        BuilderRequest::TargetCount(_, _) => Err(BuilderAdmissionError::TargetCountInput),
        BuilderRequest::HeldOutTraffic(_, _) => Err(BuilderAdmissionError::HeldOutTrafficInput),
    }
}

// END AUTHORITATIVE_BUILDER_ADMISSION

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DrivenChannel {
    Fresh,
    AliasQuarantine,
    Credit,
    Promote,
    Perturb,
    Absorb,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ModelEvent {
    Candidate(CandidateKey),
    Credit,
    Promote,
    Perturb { leaf: Word },
    Absorb,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnabledEvent {
    Candidate {
        candidate: CandidateKey,
        hazard: Ratio,
    },
    Credit {
        hazard: Ratio,
    },
    Promote {
        hazard: Ratio,
    },
    Perturb {
        leaf: Word,
        hazard: Ratio,
    },
    Absorb {
        hazard: Ratio,
    },
}

impl EnabledEvent {
    pub fn event(&self) -> ModelEvent {
        match self {
            Self::Candidate { candidate, .. } => ModelEvent::Candidate(candidate.clone()),
            Self::Credit { .. } => ModelEvent::Credit,
            Self::Promote { .. } => ModelEvent::Promote,
            Self::Perturb { leaf, .. } => ModelEvent::Perturb { leaf: leaf.clone() },
            Self::Absorb { .. } => ModelEvent::Absorb,
        }
    }

    pub fn hazard(&self) -> Ratio {
        match self {
            Self::Candidate { hazard, .. }
            | Self::Credit { hazard }
            | Self::Promote { hazard }
            | Self::Perturb { hazard, .. }
            | Self::Absorb { hazard } => *hazard,
        }
    }

    pub fn channel(&self) -> DrivenChannel {
        match self {
            Self::Candidate { candidate, .. } => match candidate.kind {
                CandidateKind::Fresh => DrivenChannel::Fresh,
                CandidateKind::Alias => DrivenChannel::AliasQuarantine,
            },
            Self::Credit { .. } => DrivenChannel::Credit,
            Self::Promote { .. } => DrivenChannel::Promote,
            Self::Perturb { .. } => DrivenChannel::Perturb,
            Self::Absorb { .. } => DrivenChannel::Absorb,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct KernelSupport {
    pub build: bool,
    pub repair: bool,
    pub protocol: bool,
    pub absorb: bool,
    pub perturb: bool,
}

impl KernelSupport {
    pub const PRIMARY: Self = Self {
        build: true,
        repair: true,
        protocol: true,
        absorb: true,
        perturb: true,
    };

    pub const REPAIR_DISABLED: Self = Self {
        repair: false,
        ..Self::PRIMARY
    };
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ResourceDelta {
    pub free_descriptors: i128,
    pub blank_semantic_records: i128,
    pub attempted_candidates: i128,
    pub charged_build_cells: i128,
    pub charged_repair_cells: i128,
    pub charged_observation_cells: i128,
    pub charged_external_perturbation_cells: i128,
    pub written_operation_records: i128,
}

impl ResourceDelta {
    fn reversed(&self) -> Self {
        Self {
            free_descriptors: -self.free_descriptors,
            blank_semantic_records: -self.blank_semantic_records,
            attempted_candidates: -self.attempted_candidates,
            charged_build_cells: -self.charged_build_cells,
            charged_repair_cells: -self.charged_repair_cells,
            charged_observation_cells: -self.charged_observation_cells,
            charged_external_perturbation_cells: -self.charged_external_perturbation_cells,
            written_operation_records: -self.written_operation_records,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceSnapshot {
    pub free_descriptors: usize,
    pub blank_semantic_records: usize,
    pub attempted_candidates: usize,
    pub charged_build_cells: usize,
    pub charged_repair_cells: usize,
    pub charged_observation_cells: usize,
    pub charged_external_perturbation_cells: usize,
    pub written_operation_records: usize,
}

impl ResourceSnapshot {
    pub fn difference_from(&self, before: &Self) -> ResourceDelta {
        ResourceDelta {
            free_descriptors: self.free_descriptors as i128 - before.free_descriptors as i128,
            blank_semantic_records: self.blank_semantic_records as i128
                - before.blank_semantic_records as i128,
            attempted_candidates: self.attempted_candidates as i128
                - before.attempted_candidates as i128,
            charged_build_cells: self.charged_build_cells as i128
                - before.charged_build_cells as i128,
            charged_repair_cells: self.charged_repair_cells as i128
                - before.charged_repair_cells as i128,
            charged_observation_cells: self.charged_observation_cells as i128
                - before.charged_observation_cells as i128,
            charged_external_perturbation_cells: self.charged_external_perturbation_cells as i128
                - before.charged_external_perturbation_cells as i128,
            written_operation_records: self.written_operation_records as i128
                - before.written_operation_records as i128,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerRow {
    pub channel: DrivenChannel,
    pub delta_working: i128,
    pub delta_work_store: i128,
    pub work_on_working: i128,
    pub heat_to_working: i128,
    pub delta_augmented: i128,
    pub external_work: i128,
    pub external_heat: i128,
    pub delta_external_work_store: i128,
    pub delta_complete: i128,
    pub complete_external_work: i128,
    pub complete_external_heat: i128,
    pub resources: ResourceDelta,
}

impl LedgerRow {
    fn internal(channel: DrivenChannel, epsilon: u64, resources: ResourceDelta) -> Self {
        let epsilon = epsilon as i128;
        Self {
            channel,
            delta_working: epsilon,
            delta_work_store: -epsilon,
            work_on_working: epsilon,
            heat_to_working: 0,
            delta_augmented: 0,
            external_work: 0,
            external_heat: 0,
            delta_external_work_store: 0,
            delta_complete: 0,
            complete_external_work: 0,
            complete_external_heat: 0,
            resources,
        }
    }

    fn perturb(epsilon: u64) -> Self {
        let epsilon = epsilon as i128;
        Self {
            channel: DrivenChannel::Perturb,
            delta_working: epsilon,
            delta_work_store: 0,
            work_on_working: epsilon,
            heat_to_working: 0,
            delta_augmented: epsilon,
            external_work: epsilon,
            external_heat: 0,
            delta_external_work_store: -epsilon,
            delta_complete: 0,
            complete_external_work: 0,
            complete_external_heat: 0,
            resources: ResourceDelta {
                free_descriptors: 1,
                charged_external_perturbation_cells: -1,
                written_operation_records: 1,
                ..ResourceDelta::default()
            },
        }
    }

    pub fn working_first_law_closes(&self) -> bool {
        self.delta_working == self.heat_to_working + self.work_on_working
    }

    pub fn internal_augmented_first_law_closes(&self) -> bool {
        self.delta_working + self.delta_work_store == self.delta_augmented
            && self.delta_augmented == self.external_heat + self.external_work
    }

    pub fn complete_first_law_closes(&self) -> bool {
        self.delta_augmented + self.delta_external_work_store == self.delta_complete
            && self.delta_complete == self.complete_external_heat + self.complete_external_work
    }

    fn reversed(&self) -> Self {
        Self {
            channel: self.channel,
            delta_working: -self.delta_working,
            delta_work_store: -self.delta_work_store,
            work_on_working: -self.work_on_working,
            heat_to_working: -self.heat_to_working,
            delta_augmented: -self.delta_augmented,
            external_work: -self.external_work,
            external_heat: -self.external_heat,
            delta_external_work_store: -self.delta_external_work_store,
            delta_complete: -self.delta_complete,
            complete_external_work: -self.complete_external_work,
            complete_external_heat: -self.complete_external_heat,
            resources: self.resources.reversed(),
        }
    }
}

fn internal_resource_delta(channel: DrivenChannel, lane: WorkLane) -> ResourceDelta {
    let mut delta = ResourceDelta {
        written_operation_records: 1,
        ..ResourceDelta::default()
    };
    match lane {
        WorkLane::Build => delta.charged_build_cells = -1,
        WorkLane::Repair => delta.charged_repair_cells = -1,
        WorkLane::Observation => delta.charged_observation_cells = -1,
    }
    match channel {
        DrivenChannel::Fresh => {
            delta.free_descriptors = -1;
            delta.blank_semantic_records = -1;
            delta.attempted_candidates = 1;
        }
        DrivenChannel::AliasQuarantine => {
            delta.blank_semantic_records = -1;
            delta.attempted_candidates = 1;
        }
        DrivenChannel::Absorb => delta.blank_semantic_records = -1,
        DrivenChannel::Credit | DrivenChannel::Promote => {}
        DrivenChannel::Perturb => unreachable!("PERTURB has an external resource row"),
    }
    delta
}

fn record_lane_for_work_lane(lane: WorkLane) -> RecordLane {
    match lane {
        WorkLane::Build => RecordLane::Build,
        WorkLane::Repair => RecordLane::Repair,
        WorkLane::Observation => RecordLane::Observation,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OperationEntry {
    Fresh {
        candidate: CandidateKey,
        hazard: Ratio,
        node: ReservoirNode,
        semantic_record: SemanticRecordId,
        work_lane: WorkLane,
        work_cell: WorkCellId,
        old_phase: Phase,
        old_queue: SchedulerState,
    },
    AliasQuarantine {
        candidate: CandidateKey,
        hazard: Ratio,
        semantic_record: SemanticRecordId,
        work_lane: WorkLane,
        work_cell: WorkCellId,
        old_phase: Phase,
        old_queue: SchedulerState,
    },
    Credit {
        semantic_record: SemanticRecordId,
        work_lane: WorkLane,
        work_cell: WorkCellId,
        old_phase: Phase,
        old_queue: SchedulerState,
    },
    Promote {
        moved_words: Vec<Word>,
        work_lane: WorkLane,
        work_cell: WorkCellId,
        old_phase: Phase,
        old_queue: SchedulerState,
    },
    Perturb {
        node: ReservoirNode,
        edge: IncidenceEdge,
        old_phase: Phase,
        old_queue: SchedulerState,
        repair_was_locked: bool,
    },
    Absorb {
        semantic_record: SemanticRecordId,
        work_cell: WorkCellId,
        old_phase: Phase,
        old_queue: SchedulerState,
    },
}

impl OperationEntry {
    pub fn channel(&self) -> DrivenChannel {
        match self {
            Self::Fresh { .. } => DrivenChannel::Fresh,
            Self::AliasQuarantine { .. } => DrivenChannel::AliasQuarantine,
            Self::Credit { .. } => DrivenChannel::Credit,
            Self::Promote { .. } => DrivenChannel::Promote,
            Self::Perturb { .. } => DrivenChannel::Perturb,
            Self::Absorb { .. } => DrivenChannel::Absorb,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationRecord {
    pub id: OperationRecordId,
    pub lane: RecordLane,
    pub entry: Option<OperationEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationBank {
    pub records: Vec<OperationRecord>,
    pub inverse_order: Vec<OperationRecordId>,
}

impl OperationBank {
    pub fn new(capacities: &[(RecordLane, usize)]) -> Self {
        let mut records = Vec::new();
        for &(lane, capacity) in capacities {
            for _ in 0..capacity {
                records.push(OperationRecord {
                    id: records.len(),
                    lane,
                    entry: None,
                });
            }
        }
        Self {
            records,
            inverse_order: Vec::new(),
        }
    }

    pub fn written_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.entry.is_some())
            .count()
    }

    fn next_blank(&self, lane: RecordLane) -> Option<OperationRecordId> {
        self.records
            .iter()
            .find(|record| record.lane == lane && record.entry.is_none())
            .map(|record| record.id)
    }

    fn write(&mut self, id: OperationRecordId, entry: OperationEntry) {
        assert!(self.records[id].entry.is_none());
        self.records[id].entry = Some(entry);
        self.inverse_order.push(id);
    }

    fn clear_last(&mut self, id: OperationRecordId) {
        assert_eq!(self.inverse_order.pop(), Some(id));
        self.records[id].entry = None;
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionReceipt {
    pub event: ModelEvent,
    pub hazard: Ratio,
    pub operation_record: OperationRecordId,
    pub ledger: LedgerRow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EventTransition {
    pub enabled: EnabledEvent,
    pub hazard: Ratio,
    pub channel: DrivenChannel,
    pub successor: CompleteState,
    pub forward_receipt: ActionReceipt,
    pub ledger: LedgerRow,
    pub inverse: ActionReceipt,
    pub inverse_restores_source: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActionError {
    WrongPhase,
    WrongCandidate,
    CursorMismatch,
    CandidateAlreadyAttempted,
    AttemptBitUnavailable,
    PortClosed,
    DescriptorUnavailable,
    SemanticRecordUnavailable,
    WorkUnavailable,
    OperationRecordUnavailable,
    FrontierIncomplete,
    PendingCreditUnavailable,
    OccupiedStateNotRoot,
    LeafMissing,
    LeafHasDescendants,
    ObservationUnavailable,
    ObservationMustPrecedeAction,
    ExternalPerturbationWorkUnavailable,
    PerturbationNotScheduled,
    EventNotEnabled,
    PromotionCreditIncomplete,
    InverseOrderEmpty,
    InverseStateMismatch,
}

pub const ENERGY_QUANTUM: u64 = 1;
pub const REPAIR_RESERVE_CELLS: usize = 4;

pub fn descriptor_requirement(base: usize) -> usize {
    checked_pow(base, 1) + checked_pow(base, 2) + checked_pow(base, 3)
}

pub fn build_capacity(base: usize) -> usize {
    3 * descriptor_requirement(base) + 3
}

/// Exact finite-resource witness for the episode-keyed attempt bits used by
/// the frozen construction family. The two repair identities are the FRESH
/// and ALIAS choices of the single declared repair episode.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AttemptBitCapacityCertificate {
    pub base: usize,
    pub off_shell_inventory: bool,
    pub descriptor_capacity: usize,
    pub construction_candidate_identities: usize,
    pub repair_attempt_slots: usize,
    pub declared_attempt_bits: usize,
    pub semantic_record_capacity: usize,
    pub candidate_identities: Vec<CandidateKey>,
    pub assigned_attempt_bits: BTreeMap<CandidateKey, usize>,
    pub every_candidate_identity_has_distinct_bit: bool,
    pub repair_slots_are_predeclared_blank_resources: bool,
    pub every_admissible_leaf_can_bind_the_same_repair_slots: bool,
    pub capacity_cannot_truncate_construction: bool,
}

impl AttemptBitCapacityCertificate {
    pub fn passes(&self) -> bool {
        self.every_candidate_identity_has_distinct_bit
            && self.repair_slots_are_predeclared_blank_resources
            && self.every_admissible_leaf_can_bind_the_same_repair_slots
            && self.capacity_cannot_truncate_construction
            && self.declared_attempt_bits == self.semantic_record_capacity
    }
}

fn construction_candidate_identities(base: usize, off_shell_inventory: bool) -> Vec<CandidateKey> {
    let alphabet = (1..=base as u8).collect::<Vec<_>>();
    let mut parents = vec![Word::root()];
    let mut construction_ports = Vec::new();
    for _ in 1..=3 {
        let mut children = Vec::new();
        for parent in &parents {
            for &port in &alphabet {
                construction_ports.push(PortRef {
                    parent: parent.clone(),
                    port,
                });
                children.push(parent.child(port));
            }
        }
        parents = children;
    }
    if off_shell_inventory {
        construction_ports.push(PortRef {
            parent: Word(vec![alphabet[0]; 3]),
            port: alphabet[0],
        });
    }
    construction_ports
        .into_iter()
        .flat_map(|port| {
            [CandidateKind::Fresh, CandidateKind::Alias]
                .into_iter()
                .map(move |kind| CandidateKey {
                    episode: 0,
                    port: port.clone(),
                    kind,
                })
        })
        .collect()
}

pub fn attempt_bit_capacity_certificate(
    base: usize,
    off_shell_inventory: bool,
) -> AttemptBitCapacityCertificate {
    assert!((2..=4).contains(&base));
    let off_shell_count = usize::from(off_shell_inventory);
    let descriptor_capacity = descriptor_requirement(base) + off_shell_count;
    let construction_candidate_identity_count = 2 * descriptor_capacity;
    let repair_attempt_slots = 2;
    let declared_attempt_bits = construction_candidate_identity_count + repair_attempt_slots;
    let semantic_record_capacity = 2 * descriptor_capacity + 2;
    let candidate_identities = construction_candidate_identities(base, off_shell_inventory);
    let unique_identities = candidate_identities
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let assigned_attempt_bits = unique_identities
        .iter()
        .cloned()
        .enumerate()
        .map(|(bit, identity)| (identity, bit))
        .collect::<BTreeMap<_, _>>();
    let assigned_bit_indices = assigned_attempt_bits
        .values()
        .copied()
        .collect::<BTreeSet<_>>();
    let every_candidate_identity_has_distinct_bit = candidate_identities.len()
        == unique_identities.len()
        && assigned_attempt_bits.len() == construction_candidate_identity_count
        && assigned_bit_indices.len() == assigned_attempt_bits.len();
    let initial_store = AttemptBitStore::with_construction_identities(
        candidate_identities.iter().cloned(),
        repair_attempt_slots,
    );
    let repair_slots_are_predeclared_blank_resources = initial_store.is_well_formed()
        && initial_store.declared_count() == declared_attempt_bits
        && initial_store.blank_repair_slots() == repair_attempt_slots;
    let alphabet = (1..=base as u8).collect::<Vec<_>>();
    let every_admissible_leaf_can_bind_the_same_repair_slots =
        (0..checked_pow(base, 3)).all(|index| {
            let mut digits = vec![0_u8; 3];
            let mut remaining = index;
            for digit in digits.iter_mut().rev() {
                *digit = alphabet[remaining % base];
                remaining /= base;
            }
            let leaf = Word(digits);
            let port = PortRef {
                parent: leaf.parent().expect("depth-three leaf has a parent"),
                port: leaf.final_port().expect("depth-three leaf has a port"),
            };
            let mut store = initial_store.clone();
            store.bind_repair_episode(1, &port)
                && [CandidateKind::Fresh, CandidateKind::Alias]
                    .into_iter()
                    .all(|kind| {
                        store.has_unattempted(&CandidateKey {
                            episode: 1,
                            port: port.clone(),
                            kind,
                        })
                    })
                && store.is_well_formed()
        });
    AttemptBitCapacityCertificate {
        base,
        off_shell_inventory,
        descriptor_capacity,
        construction_candidate_identities: construction_candidate_identity_count,
        repair_attempt_slots,
        declared_attempt_bits,
        semantic_record_capacity,
        candidate_identities,
        assigned_attempt_bits,
        every_candidate_identity_has_distinct_bit,
        repair_slots_are_predeclared_blank_resources,
        every_admissible_leaf_can_bind_the_same_repair_slots,
        capacity_cannot_truncate_construction: every_candidate_identity_has_distinct_bit
            && construction_candidate_identity_count == unique_identities.len(),
    }
}

pub fn primary_fixture(base: usize, off_shell_inventory: bool) -> CompleteState {
    primary_fixture_with_quantum(base, ENERGY_QUANTUM, off_shell_inventory)
}

pub fn primary_fixture_with_quantum(
    base: usize,
    energy_quantum: u64,
    off_shell_inventory: bool,
) -> CompleteState {
    primary_fixture_with_order(
        base,
        energy_quantum,
        off_shell_inventory,
        DescriptorOrder::Forward,
    )
}

pub fn primary_fixture_with_order(
    base: usize,
    energy_quantum: u64,
    off_shell_inventory: bool,
    descriptor_order: DescriptorOrder,
) -> CompleteState {
    assert!((2..=4).contains(&base));
    assert!(energy_quantum > 0);
    let off_shell_count = if off_shell_inventory { 1 } else { 0 };
    let descriptor_capacity = descriptor_requirement(base) + off_shell_count;
    let build_cells = build_capacity(base) + 2 * off_shell_count;
    let semantic_capacity = 2 * descriptor_capacity + 2;
    let alphabet = (1..=base as u8).collect::<Vec<_>>();
    let root = ReservoirNode {
        word: Word::root(),
        descriptor: 0,
        energy_index: 0,
        executable: true,
    };
    let mut reservoir = BTreeMap::new();
    reservoir.insert(root.word.clone(), root);
    let cursor = Some(PortRef {
        parent: Word::root(),
        port: alphabet[0],
    });
    CompleteState {
        Gamma: ConfigurationSpace {
            reservoir,
            workspace: BTreeMap::new(),
            structural_incidence: BTreeSet::new(),
        },
        r: Word::root(),
        A: alphabet,
        lambda: energy_quantum,
        X: ProbeState {
            mesostate: 0,
            microstate: 0,
        },
        F: (1..=descriptor_capacity as u32).collect(),
        B_W: WorkStore {
            B_build: WorkPool::new(WorkLane::Build, build_cells, false),
            B_repair: WorkPool::new(WorkLane::Repair, REPAIR_RESERVE_CELLS, true),
            B_obs: WorkPool::new(WorkLane::Observation, 0, false),
            epsilon_W: 1,
        },
        M: PhysicalMemory::blank(semantic_capacity),
        O: OperationBank::new(&[
            (RecordLane::Build, build_cells),
            (RecordLane::Repair, REPAIR_RESERVE_CELLS),
            (RecordLane::ExternalPerturbation, 1),
        ]),
        P: Phase::Build { depth: 1 },
        Q: SchedulerState {
            episode: 0,
            cursor,
            repair_target: None,
            attempt_bits: AttemptBitStore::with_construction_identities(
                construction_candidate_identities(base, off_shell_inventory),
                2,
            ),
            observation_payload: None,
            absorb_before_candidates: false,
            scheduled_perturbation: None,
            descriptor_order,
            external_perturbation_cell: true,
        },
    }
}

pub fn causal_memory_fixture() -> CompleteState {
    let mut state = primary_fixture(2, false);
    state.F = (1..=1).collect();
    state.B_W.B_build = WorkPool::new(WorkLane::Build, 2, false);
    state.B_W.B_repair = WorkPool::new(WorkLane::Repair, 0, true);
    state.B_W.B_obs = WorkPool::new(WorkLane::Observation, 1, false);
    state.M = PhysicalMemory::blank(3);
    state.O = OperationBank::new(&[(RecordLane::Build, 2), (RecordLane::Observation, 1)]);
    state.Q.observation_payload = Some(ObservationPayload {
        bucket: BucketKey {
            port: 1,
            kind: CandidateKind::Fresh,
        },
        outcome: Outcome::Success,
        consumed: false,
    });
    state.Q.absorb_before_candidates = true;
    state.Q.external_perturbation_cell = false;
    state.Q.attempt_bits = AttemptBitStore::with_construction_identities(
        [CandidateKind::Fresh, CandidateKind::Alias]
            .into_iter()
            .map(|kind| CandidateKey {
                episode: 0,
                port: PortRef {
                    parent: Word::root(),
                    port: 1,
                },
                kind,
            }),
        0,
    );
    state
}

fn checked_pow(mut base: usize, mut exponent: u32) -> usize {
    let mut result = 1_usize;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result.checked_mul(base).expect("power overflow");
        }
        exponent >>= 1;
        if exponent > 0 {
            base = base.checked_mul(base).expect("power overflow");
        }
    }
    result
}

impl BuilderView<'_> {
    fn observation_precedes_candidates(&self) -> bool {
        self.Q.absorb_before_candidates
            && self
                .Q
                .observation_payload
                .as_ref()
                .is_some_and(|payload| !payload.consumed)
    }

    fn next_free_descriptor(&self) -> Option<DescriptorId> {
        match self.Q.descriptor_order {
            DescriptorOrder::Forward => self.F.iter().next().copied(),
            DescriptorOrder::Reversed => self.F.iter().next_back().copied(),
        }
    }

    pub fn open_build_ports(&self, depth: usize) -> Vec<PortRef> {
        let mut ports = Vec::new();
        for parent in self.Gamma.shell_words(depth.saturating_sub(1)) {
            for &port in self.A {
                let port_ref = PortRef {
                    parent: parent.clone(),
                    port,
                };
                if self.is_port_open(&port_ref) {
                    ports.push(port_ref);
                }
            }
        }
        ports
    }

    pub fn is_port_open(&self, port: &PortRef) -> bool {
        let child = port.child();
        self.Gamma.reservoir.contains_key(&port.parent)
            && !self.Gamma.reservoir.contains_key(&child)
            && !self.Gamma.workspace.contains_key(&child)
    }

    fn candidate_keys_before_attempt_bit_lookup(&self) -> Vec<CandidateKey> {
        if self.observation_precedes_candidates() {
            return Vec::new();
        }
        let Some(port) = self.Q.cursor.clone() else {
            return Vec::new();
        };
        if !matches!(self.P, Phase::Build { .. } | Phase::Repair { .. })
            || !self.is_port_open(&port)
        {
            return Vec::new();
        }
        let lane = match self.P {
            Phase::Build { .. } => WorkLane::Build,
            Phase::Repair { .. } => WorkLane::Repair,
            _ => unreachable!(),
        };
        let record_lane = match lane {
            WorkLane::Build => RecordLane::Build,
            WorkLane::Repair => RecordLane::Repair,
            WorkLane::Observation => unreachable!(),
        };
        if self.F.is_empty()
            || self.M.next_blank().is_none()
            || self.B_W.pool(lane).next_charged().is_none()
            || self.O.next_blank(record_lane).is_none()
        {
            return Vec::new();
        }
        [CandidateKind::Fresh, CandidateKind::Alias]
            .into_iter()
            .filter_map(|kind| {
                let key = CandidateKey {
                    episode: self.Q.episode,
                    port: port.clone(),
                    kind,
                };
                if self.Q.attempt_bits.contains_attempted(&key) {
                    None
                } else {
                    Some(key)
                }
            })
            .collect()
    }

    /// Current-state evidence that the declared episode-keyed attempt bits
    /// cannot suppress one of the otherwise enabled local candidates.
    pub fn attempt_bit_capacity_nontruncating_now(&self) -> bool {
        self.candidate_keys_before_attempt_bit_lookup()
            .iter()
            .all(|candidate| self.Q.attempt_bits.has_unattempted(candidate))
    }

    fn candidate_keys(&self) -> Vec<CandidateKey> {
        self.candidate_keys_before_attempt_bit_lookup()
            .into_iter()
            .filter(|candidate| self.Q.attempt_bits.has_unattempted(candidate))
            .collect()
    }

    pub fn enabled_candidates(&self) -> Vec<CandidateKey> {
        self.candidate_keys()
    }

    pub fn candidate_hazard(&self, candidate: &CandidateKey) -> Option<Ratio> {
        if !self.candidate_keys().contains(candidate) {
            return None;
        }
        let (successes, trials) = self.M.terminal_counts(candidate.bucket());
        let ruby = Ratio::new(i128::from(successes) + 1, i128::from(trials) + 2);
        Some((Ratio::ONE + ruby).powu(2) * (Ratio::integer(3) - ruby) / Ratio::integer(8))
    }

    pub fn enabled_candidate_events(&self, support: KernelSupport) -> Vec<EnabledEvent> {
        let candidate_support = match self.P {
            Phase::Build { .. } => support.build,
            Phase::Repair { .. } => support.repair,
            _ => false,
        };
        if !candidate_support {
            return Vec::new();
        }
        self.candidate_keys()
            .into_iter()
            .map(|candidate| {
                let hazard = self
                    .candidate_hazard(&candidate)
                    .expect("enabled candidate has an exact hazard");
                EnabledEvent::Candidate { candidate, hazard }
            })
            .collect()
    }
}

impl CompleteState {
    fn restricted_builder_view(&self) -> BuilderView<'_> {
        BuilderView {
            Gamma: &self.Gamma,
            A: &self.A,
            F: &self.F,
            B_W: &self.B_W,
            M: &self.M,
            O: &self.O,
            P: &self.P,
            Q: &self.Q,
        }
    }

    pub fn builder_view(&self) -> BuilderView<'_> {
        admit_builder_request(BuilderRequest::Local(self))
            .expect("a local authoritative state is admitted")
    }

    pub fn resource_snapshot(&self) -> ResourceSnapshot {
        ResourceSnapshot {
            free_descriptors: self.F.len(),
            blank_semantic_records: self
                .M
                .records
                .iter()
                .filter(|record| record.state == SemanticState::Blank)
                .count(),
            attempted_candidates: self.Q.attempt_bits.attempted_count(),
            charged_build_cells: self.B_W.B_build.charged_count(),
            charged_repair_cells: self.B_W.B_repair.charged_count(),
            charged_observation_cells: self.B_W.B_obs.charged_count(),
            charged_external_perturbation_cells: if self.Q.external_perturbation_cell {
                1
            } else {
                0
            },
            written_operation_records: self.O.written_count(),
        }
    }

    fn observation_precedes_candidates(&self) -> bool {
        self.builder_view().observation_precedes_candidates()
    }

    fn next_free_descriptor(&self) -> Option<DescriptorId> {
        self.builder_view().next_free_descriptor()
    }

    pub fn open_build_ports(&self, depth: usize) -> Vec<PortRef> {
        self.builder_view().open_build_ports(depth)
    }

    pub fn is_port_open(&self, port: &PortRef) -> bool {
        self.builder_view().is_port_open(port)
    }

    pub fn attempt_bit_capacity_nontruncating_now(&self) -> bool {
        self.builder_view().attempt_bit_capacity_nontruncating_now()
    }

    pub fn enabled_candidates(&self) -> Vec<CandidateKey> {
        self.builder_view().enabled_candidates()
    }

    pub fn candidate_hazard(&self, candidate: &CandidateKey) -> Option<Ratio> {
        self.builder_view().candidate_hazard(candidate)
    }

    pub fn enabled_events(&self) -> Vec<EnabledEvent> {
        self.enabled_events_with_support(KernelSupport::PRIMARY)
    }

    pub fn enabled_events_with_support(&self, support: KernelSupport) -> Vec<EnabledEvent> {
        let mut events = Vec::new();
        if support.absorb
            && self
                .Q
                .observation_payload
                .as_ref()
                .is_some_and(|payload| !payload.consumed)
            && self.M.next_blank().is_some()
            && self.B_W.B_obs.next_charged().is_some()
            && self.O.next_blank(RecordLane::Observation).is_some()
        {
            events.push(EnabledEvent::Absorb { hazard: Ratio::ONE });
        }
        events.extend(self.builder_view().enabled_candidate_events(support));
        match self.P {
            Phase::Credit { maintenance, .. }
                if support.protocol && (!maintenance || support.repair) =>
            {
                if self.next_pending_credit().is_some()
                    && self.credit_resources_available(maintenance)
                {
                    events.push(EnabledEvent::Credit { hazard: Ratio::ONE });
                }
            }
            Phase::Promote { depth, maintenance }
                if support.protocol && (!maintenance || support.repair) =>
            {
                if self.frontier_is_complete(depth, maintenance)
                    && self.promotion_records_ready(depth, maintenance)
                    && self.promotion_resources_available(maintenance)
                {
                    events.push(EnabledEvent::Promote { hazard: Ratio::ONE });
                }
            }
            Phase::Thermal if support.perturb => {
                if let Some(leaf) = self.Q.scheduled_perturbation.clone() {
                    if self.perturbation_is_enabled(&leaf) {
                        events.push(EnabledEvent::Perturb {
                            leaf,
                            hazard: Ratio::ONE,
                        });
                    }
                }
            }
            _ => {}
        }
        events.sort_by_key(EnabledEvent::event);
        events
    }

    pub fn forbidden_input_projection(&self) -> Vec<CandidateKey> {
        self.enabled_candidates()
    }

    fn pending_frontier_records(&self, depth: usize) -> Vec<SemanticRecordId> {
        let workspace_words = self
            .Gamma
            .workspace
            .keys()
            .filter(|word| word.depth() == depth)
            .cloned()
            .collect::<BTreeSet<_>>();
        let mut records = self
            .M
            .records
            .iter()
            .filter(|record| {
                record.state == SemanticState::Pending
                    && record.candidate.as_ref().is_some_and(|candidate| {
                        candidate.episode == self.Q.episode
                            && candidate.kind == CandidateKind::Fresh
                            && workspace_words.contains(&candidate.port.child())
                    })
            })
            .map(|record| record.id)
            .collect::<Vec<_>>();
        records.sort_by_key(|&id| self.M.records[id].descriptor);
        records
    }

    fn next_pending_credit(&self) -> Option<SemanticRecordId> {
        let depth = match self.P {
            Phase::Credit { depth, .. } => depth,
            _ => return None,
        };
        self.pending_frontier_records(depth).into_iter().next()
    }

    fn credit_resources_available(&self, maintenance: bool) -> bool {
        let (work_lane, record_lane) = if maintenance {
            (WorkLane::Repair, RecordLane::Repair)
        } else {
            (WorkLane::Build, RecordLane::Build)
        };
        self.B_W.pool(work_lane).next_charged().is_some()
            && self.O.next_blank(record_lane).is_some()
    }

    fn promotion_resources_available(&self, maintenance: bool) -> bool {
        self.credit_resources_available(maintenance)
    }

    fn promotion_records_ready(&self, depth: usize, maintenance: bool) -> bool {
        let frontier = if maintenance {
            self.Q
                .repair_target
                .as_ref()
                .map(|target| [target.child()].into_iter().collect::<BTreeSet<_>>())
                .unwrap_or_default()
        } else {
            self.Gamma
                .workspace
                .keys()
                .filter(|word| word.depth() == depth)
                .cloned()
                .collect::<BTreeSet<_>>()
        };
        if frontier.is_empty() || self.pending_frontier_records(depth).len() != 0 {
            return false;
        }
        frontier.iter().all(|word| {
            let descriptor = self.Gamma.workspace.get(word).map(|node| node.descriptor);
            descriptor.is_some()
                && self
                    .M
                    .records
                    .iter()
                    .filter(|record| {
                        record.state == SemanticState::Success
                            && record.descriptor == descriptor
                            && record.candidate.as_ref().is_some_and(|candidate| {
                                candidate.episode == self.Q.episode
                                    && candidate.kind == CandidateKind::Fresh
                                    && candidate.port.child() == *word
                            })
                    })
                    .count()
                    == 1
        })
    }

    fn perturbation_is_enabled(&self, leaf: &Word) -> bool {
        if self.P != Phase::Thermal
            || self.r != Word::root()
            || leaf.depth() != 3
            || !self.Q.external_perturbation_cell
            || self.Q.attempt_bits.blank_repair_slots() != 2
            || self
                .O
                .next_blank(RecordLane::ExternalPerturbation)
                .is_none()
        {
            return false;
        }
        let Some(node) = self.Gamma.reservoir.get(leaf) else {
            return false;
        };
        if self.F.contains(&node.descriptor)
            || self
                .Gamma
                .reservoir
                .keys()
                .any(|word| word.depth() > leaf.depth() && word.0.starts_with(&leaf.0))
        {
            return false;
        }
        let Some(parent) = leaf.parent() else {
            return false;
        };
        let Some(port) = leaf.final_port() else {
            return false;
        };
        self.Gamma.structural_incidence.contains(&IncidenceEdge {
            parent,
            port,
            child: leaf.clone(),
        })
    }

    pub fn schedule_leaf_perturbation(&mut self, leaf: Word) -> Result<(), ActionError> {
        if self.Q.scheduled_perturbation.is_some() || !self.perturbation_is_enabled(&leaf) {
            return Err(ActionError::PerturbationNotScheduled);
        }
        self.Q.scheduled_perturbation = Some(leaf);
        Ok(())
    }

    pub fn apply_event(&mut self, event: &ModelEvent) -> Result<ActionReceipt, ActionError> {
        self.apply_event_with_support(event, KernelSupport::PRIMARY)
    }

    pub fn apply_event_with_support(
        &mut self,
        event: &ModelEvent,
        support: KernelSupport,
    ) -> Result<ActionReceipt, ActionError> {
        if !self
            .enabled_events_with_support(support)
            .iter()
            .any(|enabled| enabled.event().eq(event))
        {
            return Err(ActionError::EventNotEnabled);
        }
        match event {
            ModelEvent::Candidate(candidate) => match candidate.kind {
                CandidateKind::Fresh => self.fresh(candidate.clone()),
                CandidateKind::Alias => self.alias_quarantine(candidate.clone()),
            },
            ModelEvent::Credit => self.credit(),
            ModelEvent::Promote => self.promote(),
            ModelEvent::Perturb { leaf } => self.perturb_leaf(leaf),
            ModelEvent::Absorb => self.absorb(),
        }
    }

    pub fn event_transition(
        &self,
        enabled: &EnabledEvent,
        support: KernelSupport,
    ) -> Result<EventTransition, ActionError> {
        if !self.enabled_events_with_support(support).contains(enabled) {
            return Err(ActionError::EventNotEnabled);
        }
        let mut successor = self.clone();
        let event = enabled.event();
        let forward_receipt = successor.apply_event_with_support(&event, support)?;
        let mut reversed = successor.clone();
        let inverse = reversed.reverse_last()?;
        let inverse_restores_source = reversed.eq(self);
        Ok(EventTransition {
            enabled: enabled.clone(),
            hazard: enabled.hazard(),
            channel: enabled.channel(),
            successor,
            ledger: forward_receipt.ledger.clone(),
            forward_receipt,
            inverse,
            inverse_restores_source,
        })
    }

    pub fn repair_disabled_kernel_is_closed(&self) -> bool {
        matches!(self.P, Phase::Repair { .. })
            && self
                .enabled_events_with_support(KernelSupport::REPAIR_DISABLED)
                .is_empty()
    }

    pub fn fresh(&mut self, candidate: CandidateKey) -> Result<ActionReceipt, ActionError> {
        if candidate.kind != CandidateKind::Fresh {
            return Err(ActionError::WrongCandidate);
        }
        if self.observation_precedes_candidates() {
            return Err(ActionError::ObservationMustPrecedeAction);
        }
        let hazard = self
            .candidate_hazard(&candidate)
            .ok_or(ActionError::EventNotEnabled)?;
        let (depth, maintenance, work_lane, record_lane) = match self.P {
            Phase::Build { depth } => (depth, false, WorkLane::Build, RecordLane::Build),
            Phase::Repair { ref missing } if *missing == candidate.port => (
                missing.child().depth(),
                true,
                WorkLane::Repair,
                RecordLane::Repair,
            ),
            Phase::Repair { .. } => return Err(ActionError::CursorMismatch),
            _ => return Err(ActionError::WrongPhase),
        };
        if self.Q.cursor.as_ref() != Some(&candidate.port) {
            return Err(ActionError::CursorMismatch);
        }
        if candidate.episode != self.Q.episode {
            return Err(ActionError::WrongCandidate);
        }
        if self.Q.attempt_bits.contains_attempted(&candidate) {
            return Err(ActionError::CandidateAlreadyAttempted);
        }
        let attempt_bit = self
            .Q
            .attempt_bits
            .unattempted_index(&candidate)
            .ok_or(ActionError::AttemptBitUnavailable)?;
        if !self.is_port_open(&candidate.port) {
            return Err(ActionError::PortClosed);
        }
        let descriptor = self
            .next_free_descriptor()
            .ok_or(ActionError::DescriptorUnavailable)?;
        let semantic_record = self
            .M
            .next_blank()
            .ok_or(ActionError::SemanticRecordUnavailable)?;
        let work_cell = self
            .B_W
            .pool(work_lane)
            .next_charged()
            .ok_or(ActionError::WorkUnavailable)?;
        let operation_record = self
            .O
            .next_blank(record_lane)
            .ok_or(ActionError::OperationRecordUnavailable)?;
        let old_phase = self.P.clone();
        let old_queue = self.Q.clone();
        let child = candidate.port.child();
        let node = ReservoirNode {
            word: child.clone(),
            descriptor,
            energy_index: depth,
            executable: false,
        };
        self.F.remove(&descriptor);
        self.Gamma.workspace.insert(child.clone(), node.clone());
        self.Gamma.structural_incidence.insert(IncidenceEdge {
            parent: candidate.port.parent.clone(),
            port: candidate.port.port,
            child,
        });
        self.Q.attempt_bits.bits[attempt_bit].attempted = true;
        self.M.records[semantic_record] = SemanticRecord {
            id: semantic_record,
            candidate: Some(candidate.clone()),
            bucket: Some(candidate.bucket()),
            descriptor: Some(descriptor),
            state: SemanticState::Pending,
        };
        self.B_W.pool_mut(work_lane).cells[work_cell].charged = false;
        if maintenance {
            self.P = Phase::Credit {
                depth,
                maintenance: true,
            };
            self.Q.cursor = None;
        } else if let Some(next) = self.open_build_ports(depth).into_iter().next() {
            self.Q.cursor = Some(next);
        } else {
            self.P = Phase::Credit {
                depth,
                maintenance: false,
            };
            self.Q.cursor = None;
        }
        self.O.write(
            operation_record,
            OperationEntry::Fresh {
                candidate: candidate.clone(),
                hazard,
                node,
                semantic_record,
                work_lane,
                work_cell,
                old_phase,
                old_queue,
            },
        );
        Ok(ActionReceipt {
            event: ModelEvent::Candidate(candidate.clone()),
            hazard,
            operation_record,
            ledger: LedgerRow::internal(
                DrivenChannel::Fresh,
                self.B_W.epsilon_W,
                internal_resource_delta(DrivenChannel::Fresh, work_lane),
            ),
        })
    }

    pub fn alias_quarantine(
        &mut self,
        candidate: CandidateKey,
    ) -> Result<ActionReceipt, ActionError> {
        if candidate.kind != CandidateKind::Alias {
            return Err(ActionError::WrongCandidate);
        }
        if self.observation_precedes_candidates() {
            return Err(ActionError::ObservationMustPrecedeAction);
        }
        let hazard = self
            .candidate_hazard(&candidate)
            .ok_or(ActionError::EventNotEnabled)?;
        let (work_lane, record_lane) = match self.P {
            Phase::Build { .. } => (WorkLane::Build, RecordLane::Build),
            Phase::Repair { ref missing } if *missing == candidate.port => {
                (WorkLane::Repair, RecordLane::Repair)
            }
            Phase::Repair { .. } => return Err(ActionError::CursorMismatch),
            _ => return Err(ActionError::WrongPhase),
        };
        if self.Q.cursor.as_ref() != Some(&candidate.port) {
            return Err(ActionError::CursorMismatch);
        }
        if candidate.episode != self.Q.episode {
            return Err(ActionError::WrongCandidate);
        }
        if self.Q.attempt_bits.contains_attempted(&candidate) {
            return Err(ActionError::CandidateAlreadyAttempted);
        }
        let attempt_bit = self
            .Q
            .attempt_bits
            .unattempted_index(&candidate)
            .ok_or(ActionError::AttemptBitUnavailable)?;
        if !self.is_port_open(&candidate.port) {
            return Err(ActionError::PortClosed);
        }
        let semantic_record = self
            .M
            .next_blank()
            .ok_or(ActionError::SemanticRecordUnavailable)?;
        let work_cell = self
            .B_W
            .pool(work_lane)
            .next_charged()
            .ok_or(ActionError::WorkUnavailable)?;
        let operation_record = self
            .O
            .next_blank(record_lane)
            .ok_or(ActionError::OperationRecordUnavailable)?;
        let old_phase = self.P.clone();
        let old_queue = self.Q.clone();
        self.Q.attempt_bits.bits[attempt_bit].attempted = true;
        self.M.records[semantic_record] = SemanticRecord {
            id: semantic_record,
            candidate: Some(candidate.clone()),
            bucket: Some(candidate.bucket()),
            descriptor: None,
            state: SemanticState::Failure,
        };
        self.B_W.pool_mut(work_lane).cells[work_cell].charged = false;
        self.O.write(
            operation_record,
            OperationEntry::AliasQuarantine {
                candidate: candidate.clone(),
                hazard,
                semantic_record,
                work_lane,
                work_cell,
                old_phase,
                old_queue,
            },
        );
        Ok(ActionReceipt {
            event: ModelEvent::Candidate(candidate),
            hazard,
            operation_record,
            ledger: LedgerRow::internal(
                DrivenChannel::AliasQuarantine,
                self.B_W.epsilon_W,
                internal_resource_delta(DrivenChannel::AliasQuarantine, work_lane),
            ),
        })
    }

    pub fn credit(&mut self) -> Result<ActionReceipt, ActionError> {
        let (depth, maintenance, work_lane, record_lane) = match self.P {
            Phase::Credit { depth, maintenance } => {
                let work_lane = if maintenance {
                    WorkLane::Repair
                } else {
                    WorkLane::Build
                };
                let record_lane = if maintenance {
                    RecordLane::Repair
                } else {
                    RecordLane::Build
                };
                (depth, maintenance, work_lane, record_lane)
            }
            _ => return Err(ActionError::WrongPhase),
        };
        let semantic_record = self
            .next_pending_credit()
            .ok_or(ActionError::PendingCreditUnavailable)?;
        let work_cell = self
            .B_W
            .pool(work_lane)
            .next_charged()
            .ok_or(ActionError::WorkUnavailable)?;
        let operation_record = self
            .O
            .next_blank(record_lane)
            .ok_or(ActionError::OperationRecordUnavailable)?;
        let old_phase = self.P.clone();
        let old_queue = self.Q.clone();
        self.M.records[semantic_record].state = SemanticState::Success;
        self.B_W.pool_mut(work_lane).cells[work_cell].charged = false;
        let pending_remain = !self.pending_frontier_records(depth).is_empty();
        if !pending_remain {
            self.P = Phase::Promote { depth, maintenance };
        }
        self.O.write(
            operation_record,
            OperationEntry::Credit {
                semantic_record,
                work_lane,
                work_cell,
                old_phase,
                old_queue,
            },
        );
        Ok(ActionReceipt {
            event: ModelEvent::Credit,
            hazard: Ratio::ONE,
            operation_record,
            ledger: LedgerRow::internal(
                DrivenChannel::Credit,
                self.B_W.epsilon_W,
                internal_resource_delta(DrivenChannel::Credit, work_lane),
            ),
        })
    }

    pub fn frontier_is_complete(&self, depth: usize, maintenance: bool) -> bool {
        if !self.Gamma.locally_injective() {
            return false;
        }
        if maintenance {
            return self.Q.repair_target.as_ref().is_some_and(|target| {
                self.Gamma.workspace.len() == 1
                    && self
                        .Gamma
                        .workspace
                        .keys()
                        .all(|word| word == &target.child())
                    && target.child().depth() == depth
                    && self.Gamma.workspace.contains_key(&target.child())
                    && self.Gamma.structural_incidence.contains(&IncidenceEdge {
                        parent: target.parent.clone(),
                        port: target.port,
                        child: target.child(),
                    })
            });
        }
        let parents = self.Gamma.shell_words(depth.saturating_sub(1));
        let expected = parents
            .iter()
            .flat_map(|parent| self.A.iter().map(|&port| parent.child(port)))
            .collect::<BTreeSet<_>>();
        let actual = self
            .Gamma
            .workspace
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>();
        !parents.is_empty()
            && actual == expected
            && parents.iter().all(|parent| {
                self.A.iter().all(|&port| {
                    let child = parent.child(port);
                    self.Gamma.structural_incidence.contains(&IncidenceEdge {
                        parent: parent.clone(),
                        port,
                        child,
                    })
                })
            })
    }

    pub fn promote(&mut self) -> Result<ActionReceipt, ActionError> {
        let (depth, maintenance, work_lane, record_lane) = match self.P {
            Phase::Promote { depth, maintenance } => {
                let work_lane = if maintenance {
                    WorkLane::Repair
                } else {
                    WorkLane::Build
                };
                let record_lane = if maintenance {
                    RecordLane::Repair
                } else {
                    RecordLane::Build
                };
                (depth, maintenance, work_lane, record_lane)
            }
            _ => return Err(ActionError::WrongPhase),
        };
        if !self.frontier_is_complete(depth, maintenance) {
            return Err(ActionError::FrontierIncomplete);
        }
        if !self.promotion_records_ready(depth, maintenance) {
            return Err(ActionError::PromotionCreditIncomplete);
        }
        let moved_words = if maintenance {
            vec![self
                .Q
                .repair_target
                .as_ref()
                .ok_or(ActionError::FrontierIncomplete)?
                .child()]
        } else {
            self.Gamma
                .workspace
                .keys()
                .filter(|word| word.depth() == depth)
                .cloned()
                .collect::<Vec<_>>()
        };
        let work_cell = self
            .B_W
            .pool(work_lane)
            .next_charged()
            .ok_or(ActionError::WorkUnavailable)?;
        let operation_record = self
            .O
            .next_blank(record_lane)
            .ok_or(ActionError::OperationRecordUnavailable)?;
        let old_phase = self.P.clone();
        let old_queue = self.Q.clone();
        for word in &moved_words {
            let mut node = self
                .Gamma
                .workspace
                .remove(word)
                .ok_or(ActionError::FrontierIncomplete)?;
            node.executable = true;
            self.Gamma.reservoir.insert(word.clone(), node);
        }
        self.B_W.pool_mut(work_lane).cells[work_cell].charged = false;
        if maintenance {
            self.B_W.B_repair.locked = true;
            self.P = Phase::Thermal;
            self.Q.cursor = None;
            self.Q.repair_target = None;
        } else if self.F.is_empty() {
            self.P = Phase::Thermal;
            self.Q.cursor = None;
        } else {
            let next_depth = depth + 1;
            self.P = Phase::Build { depth: next_depth };
            self.Q.cursor = self.open_build_ports(next_depth).into_iter().next();
        }
        self.O.write(
            operation_record,
            OperationEntry::Promote {
                moved_words,
                work_lane,
                work_cell,
                old_phase,
                old_queue,
            },
        );
        Ok(ActionReceipt {
            event: ModelEvent::Promote,
            hazard: Ratio::ONE,
            operation_record,
            ledger: LedgerRow::internal(
                DrivenChannel::Promote,
                self.B_W.epsilon_W,
                internal_resource_delta(DrivenChannel::Promote, work_lane),
            ),
        })
    }

    pub fn perturb_leaf(&mut self, leaf: &Word) -> Result<ActionReceipt, ActionError> {
        if self.Q.scheduled_perturbation.as_ref() != Some(leaf) {
            return Err(ActionError::PerturbationNotScheduled);
        }
        if self.P != Phase::Thermal {
            return Err(ActionError::WrongPhase);
        }
        if self.r != Word::root() {
            return Err(ActionError::OccupiedStateNotRoot);
        }
        if leaf.depth() != 3 {
            return Err(ActionError::LeafMissing);
        }
        if self
            .Gamma
            .reservoir
            .keys()
            .any(|word| word.depth() > leaf.depth() && word.0.starts_with(&leaf.0))
        {
            return Err(ActionError::LeafHasDescendants);
        }
        if !self.Q.external_perturbation_cell {
            return Err(ActionError::ExternalPerturbationWorkUnavailable);
        }
        let node = self
            .Gamma
            .reservoir
            .get(leaf)
            .cloned()
            .ok_or(ActionError::LeafMissing)?;
        let parent = leaf.parent().ok_or(ActionError::LeafMissing)?;
        let port = leaf.final_port().ok_or(ActionError::LeafMissing)?;
        let edge = IncidenceEdge {
            parent: parent.clone(),
            port,
            child: leaf.clone(),
        };
        if !self.Gamma.structural_incidence.contains(&edge) {
            return Err(ActionError::LeafMissing);
        }
        let operation_record = self
            .O
            .next_blank(RecordLane::ExternalPerturbation)
            .ok_or(ActionError::OperationRecordUnavailable)?;
        let next_episode = self
            .Q
            .episode
            .checked_add(1)
            .ok_or(ActionError::InverseStateMismatch)?;
        let missing = PortRef { parent, port };
        let mut next_attempt_bits = self.Q.attempt_bits.clone();
        if !next_attempt_bits.bind_repair_episode(next_episode, &missing) {
            return Err(ActionError::AttemptBitUnavailable);
        }
        let old_phase = self.P.clone();
        let old_queue = self.Q.clone();
        let repair_was_locked = self.B_W.B_repair.locked;
        self.Gamma.reservoir.remove(leaf);
        self.Gamma.structural_incidence.remove(&edge);
        self.F.insert(node.descriptor);
        self.B_W.B_repair.locked = false;
        self.Q.episode = next_episode;
        self.Q.attempt_bits = next_attempt_bits;
        self.Q.cursor = Some(missing.clone());
        self.Q.repair_target = Some(missing.clone());
        self.Q.scheduled_perturbation = None;
        self.Q.external_perturbation_cell = false;
        self.P = Phase::Repair { missing };
        self.O.write(
            operation_record,
            OperationEntry::Perturb {
                node,
                edge,
                old_phase,
                old_queue,
                repair_was_locked,
            },
        );
        Ok(ActionReceipt {
            event: ModelEvent::Perturb { leaf: leaf.clone() },
            hazard: Ratio::ONE,
            operation_record,
            ledger: LedgerRow::perturb(self.B_W.epsilon_W),
        })
    }

    pub fn absorb(&mut self) -> Result<ActionReceipt, ActionError> {
        let payload = self
            .Q
            .observation_payload
            .as_ref()
            .filter(|payload| !payload.consumed)
            .cloned()
            .ok_or(ActionError::ObservationUnavailable)?;
        let semantic_record = self
            .M
            .next_blank()
            .ok_or(ActionError::SemanticRecordUnavailable)?;
        let work_cell = self
            .B_W
            .B_obs
            .next_charged()
            .ok_or(ActionError::WorkUnavailable)?;
        let operation_record = self
            .O
            .next_blank(RecordLane::Observation)
            .ok_or(ActionError::OperationRecordUnavailable)?;
        let old_phase = self.P.clone();
        let old_queue = self.Q.clone();
        self.M.records[semantic_record] = SemanticRecord {
            id: semantic_record,
            candidate: None,
            bucket: Some(payload.bucket),
            descriptor: None,
            state: match payload.outcome {
                Outcome::Success => SemanticState::Success,
                Outcome::Failure => SemanticState::Failure,
            },
        };
        self.Q
            .observation_payload
            .as_mut()
            .expect("payload was checked")
            .consumed = true;
        self.B_W.B_obs.cells[work_cell].charged = false;
        self.O.write(
            operation_record,
            OperationEntry::Absorb {
                semantic_record,
                work_cell,
                old_phase,
                old_queue,
            },
        );
        Ok(ActionReceipt {
            event: ModelEvent::Absorb,
            hazard: Ratio::ONE,
            operation_record,
            ledger: LedgerRow::internal(
                DrivenChannel::Absorb,
                self.B_W.epsilon_W,
                internal_resource_delta(DrivenChannel::Absorb, WorkLane::Observation),
            ),
        })
    }

    pub fn reverse_last(&mut self) -> Result<ActionReceipt, ActionError> {
        self.preflight_reverse_last()?;
        let mut next = self.clone();
        let receipt = next.reverse_last_in_place()?;
        *self = next;
        Ok(receipt)
    }

    fn preflight_reverse_last(&self) -> Result<(), ActionError> {
        let operation_record = self
            .O
            .inverse_order
            .last()
            .copied()
            .ok_or(ActionError::InverseOrderEmpty)?;
        let operation = self
            .O
            .records
            .get(operation_record)
            .filter(|record| record.id == operation_record)
            .ok_or(ActionError::InverseStateMismatch)?;
        let entry = operation
            .entry
            .as_ref()
            .ok_or(ActionError::InverseStateMismatch)?;
        let record_lane = operation.lane;
        let valid = match entry {
            OperationEntry::Fresh {
                candidate,
                node,
                semantic_record,
                work_lane,
                work_cell,
                ..
            } => {
                let expected_semantic = SemanticRecord {
                    id: *semantic_record,
                    candidate: Some(candidate.clone()),
                    bucket: Some(candidate.bucket()),
                    descriptor: Some(node.descriptor),
                    state: SemanticState::Pending,
                };
                record_lane == record_lane_for_work_lane(*work_lane)
                    && self.Gamma.workspace.get(&node.word) == Some(node)
                    && !self.F.contains(&node.descriptor)
                    && self.Q.attempt_bits.contains_attempted(candidate)
                    && self.Gamma.structural_incidence.contains(&IncidenceEdge {
                        parent: candidate.port.parent.clone(),
                        port: candidate.port.port,
                        child: node.word.clone(),
                    })
                    && self.M.records.get(*semantic_record) == Some(&expected_semantic)
                    && self
                        .B_W
                        .pool(*work_lane)
                        .cells
                        .get(*work_cell)
                        .is_some_and(|cell| !cell.charged)
            }
            OperationEntry::AliasQuarantine {
                candidate,
                semantic_record,
                work_lane,
                work_cell,
                ..
            } => {
                let expected_semantic = SemanticRecord {
                    id: *semantic_record,
                    candidate: Some(candidate.clone()),
                    bucket: Some(candidate.bucket()),
                    descriptor: None,
                    state: SemanticState::Failure,
                };
                record_lane == record_lane_for_work_lane(*work_lane)
                    && self.Q.attempt_bits.contains_attempted(candidate)
                    && self.M.records.get(*semantic_record) == Some(&expected_semantic)
                    && self
                        .B_W
                        .pool(*work_lane)
                        .cells
                        .get(*work_cell)
                        .is_some_and(|cell| !cell.charged)
            }
            OperationEntry::Credit {
                semantic_record,
                work_lane,
                work_cell,
                ..
            } => {
                record_lane == record_lane_for_work_lane(*work_lane)
                    && self
                        .M
                        .records
                        .get(*semantic_record)
                        .is_some_and(|record| record.state == SemanticState::Success)
                    && self
                        .B_W
                        .pool(*work_lane)
                        .cells
                        .get(*work_cell)
                        .is_some_and(|cell| !cell.charged)
            }
            OperationEntry::Promote {
                moved_words,
                work_lane,
                work_cell,
                ..
            } => {
                record_lane == record_lane_for_work_lane(*work_lane)
                    && !moved_words.is_empty()
                    && moved_words.iter().all(|word| {
                        self.Gamma
                            .reservoir
                            .get(word)
                            .is_some_and(|node| node.executable)
                            && !self.Gamma.workspace.contains_key(word)
                    })
                    && self
                        .B_W
                        .pool(*work_lane)
                        .cells
                        .get(*work_cell)
                        .is_some_and(|cell| !cell.charged)
            }
            OperationEntry::Perturb {
                node,
                edge,
                old_phase,
                old_queue,
                repair_was_locked,
            } => {
                record_lane == RecordLane::ExternalPerturbation
                    && old_phase == &Phase::Thermal
                    && *repair_was_locked
                    && old_queue.external_perturbation_cell
                    && old_queue.scheduled_perturbation.as_ref() == Some(&node.word)
                    && old_queue.episode.checked_add(1) == Some(self.Q.episode)
                    && node.executable
                    && node.energy_index == node.word.depth()
                    && edge.child == node.word
                    && edge.child == edge.parent.child(edge.port)
                    && !self.Gamma.reservoir.contains_key(&node.word)
                    && self.F.contains(&node.descriptor)
                    && !self.Gamma.structural_incidence.contains(edge)
                    && !self.Q.external_perturbation_cell
                    && self.Q.scheduled_perturbation.is_none()
                    && self.Q.repair_target.as_ref().is_some_and(|missing| {
                        missing.parent == edge.parent && missing.port == edge.port
                    })
                    && matches!(
                        &self.P,
                        Phase::Repair { missing }
                            if missing.parent == edge.parent && missing.port == edge.port
                    )
            }
            OperationEntry::Absorb {
                semantic_record,
                work_cell,
                old_queue,
                ..
            } => {
                let expected = old_queue
                    .observation_payload
                    .as_ref()
                    .filter(|payload| !payload.consumed)
                    .map(|payload| SemanticRecord {
                        id: *semantic_record,
                        candidate: None,
                        bucket: Some(payload.bucket),
                        descriptor: None,
                        state: match payload.outcome {
                            Outcome::Success => SemanticState::Success,
                            Outcome::Failure => SemanticState::Failure,
                        },
                    });
                record_lane == RecordLane::Observation
                    && expected.as_ref().is_some_and(|expected| {
                        self.M.records.get(*semantic_record) == Some(expected)
                    })
                    && self
                        .Q
                        .observation_payload
                        .as_ref()
                        .is_some_and(|payload| payload.consumed)
                    && self
                        .B_W
                        .B_obs
                        .cells
                        .get(*work_cell)
                        .is_some_and(|cell| !cell.charged)
            }
        };
        if valid {
            Ok(())
        } else {
            Err(ActionError::InverseStateMismatch)
        }
    }

    fn reverse_last_in_place(&mut self) -> Result<ActionReceipt, ActionError> {
        let operation_record = self
            .O
            .inverse_order
            .last()
            .copied()
            .ok_or(ActionError::InverseOrderEmpty)?;
        let entry = self.O.records[operation_record]
            .entry
            .clone()
            .ok_or(ActionError::InverseStateMismatch)?;
        let (event, hazard) = match &entry {
            OperationEntry::Fresh {
                candidate, hazard, ..
            }
            | OperationEntry::AliasQuarantine {
                candidate, hazard, ..
            } => (ModelEvent::Candidate(candidate.clone()), *hazard),
            OperationEntry::Credit { .. } => (ModelEvent::Credit, Ratio::ONE),
            OperationEntry::Promote { .. } => (ModelEvent::Promote, Ratio::ONE),
            OperationEntry::Perturb { node, .. } => (
                ModelEvent::Perturb {
                    leaf: node.word.clone(),
                },
                Ratio::ONE,
            ),
            OperationEntry::Absorb { .. } => (ModelEvent::Absorb, Ratio::ONE),
        };
        let forward_ledger = match &entry {
            OperationEntry::Fresh { work_lane, .. } => LedgerRow::internal(
                DrivenChannel::Fresh,
                self.B_W.epsilon_W,
                internal_resource_delta(DrivenChannel::Fresh, *work_lane),
            ),
            OperationEntry::AliasQuarantine { work_lane, .. } => LedgerRow::internal(
                DrivenChannel::AliasQuarantine,
                self.B_W.epsilon_W,
                internal_resource_delta(DrivenChannel::AliasQuarantine, *work_lane),
            ),
            OperationEntry::Credit { work_lane, .. } => LedgerRow::internal(
                DrivenChannel::Credit,
                self.B_W.epsilon_W,
                internal_resource_delta(DrivenChannel::Credit, *work_lane),
            ),
            OperationEntry::Promote { work_lane, .. } => LedgerRow::internal(
                DrivenChannel::Promote,
                self.B_W.epsilon_W,
                internal_resource_delta(DrivenChannel::Promote, *work_lane),
            ),
            OperationEntry::Perturb { .. } => LedgerRow::perturb(self.B_W.epsilon_W),
            OperationEntry::Absorb { .. } => LedgerRow::internal(
                DrivenChannel::Absorb,
                self.B_W.epsilon_W,
                internal_resource_delta(DrivenChannel::Absorb, WorkLane::Observation),
            ),
        };
        match entry {
            OperationEntry::Fresh {
                candidate,
                node,
                semantic_record,
                work_lane,
                work_cell,
                old_phase,
                old_queue,
                ..
            } => {
                let current = self
                    .Gamma
                    .workspace
                    .remove(&node.word)
                    .ok_or(ActionError::InverseStateMismatch)?;
                if current.descriptor != node.descriptor
                    || self.M.records[semantic_record].state != SemanticState::Pending
                {
                    return Err(ActionError::InverseStateMismatch);
                }
                self.Gamma.structural_incidence.remove(&IncidenceEdge {
                    parent: candidate.port.parent.clone(),
                    port: candidate.port.port,
                    child: node.word,
                });
                self.F.insert(node.descriptor);
                self.M.reset(semantic_record);
                self.B_W.pool_mut(work_lane).cells[work_cell].charged = true;
                self.P = old_phase;
                self.Q = old_queue;
            }
            OperationEntry::AliasQuarantine {
                semantic_record,
                work_lane,
                work_cell,
                old_phase,
                old_queue,
                ..
            } => {
                if self.M.records[semantic_record].state != SemanticState::Failure {
                    return Err(ActionError::InverseStateMismatch);
                }
                self.M.reset(semantic_record);
                self.B_W.pool_mut(work_lane).cells[work_cell].charged = true;
                self.P = old_phase;
                self.Q = old_queue;
            }
            OperationEntry::Credit {
                semantic_record,
                work_lane,
                work_cell,
                old_phase,
                old_queue,
            } => {
                if self.M.records[semantic_record].state != SemanticState::Success {
                    return Err(ActionError::InverseStateMismatch);
                }
                self.M.records[semantic_record].state = SemanticState::Pending;
                self.B_W.pool_mut(work_lane).cells[work_cell].charged = true;
                self.P = old_phase;
                self.Q = old_queue;
            }
            OperationEntry::Promote {
                moved_words,
                work_lane,
                work_cell,
                old_phase,
                old_queue,
            } => {
                for word in moved_words.iter().rev() {
                    let mut node = self
                        .Gamma
                        .reservoir
                        .remove(word)
                        .ok_or(ActionError::InverseStateMismatch)?;
                    node.executable = false;
                    self.Gamma.workspace.insert(word.clone(), node);
                }
                self.B_W.pool_mut(work_lane).cells[work_cell].charged = true;
                if work_lane == WorkLane::Repair {
                    self.B_W.B_repair.locked = false;
                }
                self.P = old_phase;
                self.Q = old_queue;
            }
            OperationEntry::Perturb {
                node,
                edge,
                old_phase,
                old_queue,
                repair_was_locked,
            } => {
                if self.Gamma.reservoir.contains_key(&node.word) || !self.F.remove(&node.descriptor)
                {
                    return Err(ActionError::InverseStateMismatch);
                }
                self.Gamma.reservoir.insert(node.word.clone(), node);
                self.Gamma.structural_incidence.insert(edge);
                self.B_W.B_repair.locked = repair_was_locked;
                self.Q = old_queue;
                self.P = old_phase;
            }
            OperationEntry::Absorb {
                semantic_record,
                work_cell,
                old_phase,
                old_queue,
            } => {
                if !matches!(
                    self.M.records[semantic_record].state,
                    SemanticState::Success | SemanticState::Failure
                ) {
                    return Err(ActionError::InverseStateMismatch);
                }
                self.M.reset(semantic_record);
                self.B_W.B_obs.cells[work_cell].charged = true;
                self.P = old_phase;
                self.Q = old_queue;
            }
        }
        self.O.clear_last(operation_record);
        Ok(ActionReceipt {
            event,
            hazard,
            operation_record,
            ledger: forward_ledger.reversed(),
        })
    }

    pub fn working_energy(&self) -> u128 {
        let occupied = self.lambda as u128 * self.r.depth() as u128;
        let probe = self.lambda as u128 * self.X.mesostate as u128;
        let records = self.B_W.epsilon_W as u128 * self.O.written_count() as u128;
        occupied + probe + records
    }

    pub fn internal_work_store_energy(&self) -> u128 {
        let charged = self.B_W.B_build.charged_count()
            + self.B_W.B_repair.charged_count()
            + self.B_W.B_obs.charged_count();
        self.B_W.epsilon_W as u128 * charged as u128
    }

    pub fn augmented_internal_energy(&self) -> u128 {
        self.working_energy() + self.internal_work_store_energy()
    }

    pub fn augmented_energy_with_external_perturbation_cell(&self) -> u128 {
        self.augmented_internal_energy()
            + if self.Q.external_perturbation_cell {
                self.B_W.epsilon_W as u128
            } else {
                0
            }
    }

    pub fn same_non_memory_coordinates(&self, other: &Self) -> bool {
        self.Gamma == other.Gamma
            && self.r == other.r
            && self.A == other.A
            && self.lambda == other.lambda
            && self.X == other.X
            && self.F == other.F
            && self.B_W == other.B_W
            && self.O == other.O
            && self.P == other.P
            && self.Q == other.Q
    }

    fn structural_incidence_is_well_formed(&self) -> bool {
        let all_nodes = self
            .Gamma
            .reservoir
            .keys()
            .chain(self.Gamma.workspace.keys())
            .cloned()
            .collect::<BTreeSet<_>>();
        let expected = all_nodes
            .iter()
            .filter_map(|word| {
                Some(IncidenceEdge {
                    parent: word.parent()?,
                    port: word.final_port()?,
                    child: word.clone(),
                })
            })
            .collect::<BTreeSet<_>>();
        self.Gamma.structural_incidence == expected
            && expected.iter().all(|edge| {
                self.A.contains(&edge.port)
                    && edge.child == edge.parent.child(edge.port)
                    && all_nodes.contains(&edge.parent)
                    && all_nodes.contains(&edge.child)
            })
    }

    fn memory_is_well_formed(&self) -> bool {
        self.M.records.iter().enumerate().all(|(id, record)| {
            if record.id != id {
                return false;
            }
            match record.state {
                SemanticState::Blank => {
                    record.candidate.is_none()
                        && record.bucket.is_none()
                        && record.descriptor.is_none()
                }
                SemanticState::Pending => record.candidate.as_ref().is_some_and(|candidate| {
                    candidate.kind == CandidateKind::Fresh
                        && record.bucket == Some(candidate.bucket())
                        && record.descriptor.is_some()
                }),
                SemanticState::Success => match &record.candidate {
                    Some(candidate) => {
                        candidate.kind == CandidateKind::Fresh
                            && record.bucket == Some(candidate.bucket())
                            && record.descriptor.is_some()
                    }
                    None => record.bucket.is_some() && record.descriptor.is_none(),
                },
                SemanticState::Failure => match &record.candidate {
                    Some(candidate) => {
                        candidate.kind == CandidateKind::Alias
                            && record.bucket == Some(candidate.bucket())
                            && record.descriptor.is_none()
                    }
                    None => record.bucket.is_some() && record.descriptor.is_none(),
                },
            }
        })
    }

    fn work_store_is_well_formed(&self) -> bool {
        self.B_W.epsilon_W > 0
            && [
                (&self.B_W.B_build, WorkLane::Build),
                (&self.B_W.B_repair, WorkLane::Repair),
                (&self.B_W.B_obs, WorkLane::Observation),
            ]
            .into_iter()
            .all(|(pool, lane)| {
                pool.lane == lane
                    && pool
                        .cells
                        .iter()
                        .enumerate()
                        .all(|(id, cell)| cell.id == id)
            })
    }

    fn operation_bank_is_well_formed(&self) -> bool {
        let written = self
            .O
            .records
            .iter()
            .filter(|record| record.entry.is_some())
            .map(|record| record.id)
            .collect::<BTreeSet<_>>();
        let order = self
            .O
            .inverse_order
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if written != order
            || order.len() != self.O.inverse_order.len()
            || self
                .O
                .records
                .iter()
                .enumerate()
                .any(|(id, record)| record.id != id)
        {
            return false;
        }
        self.O.records.iter().all(|record| {
            let Some(entry) = &record.entry else {
                return true;
            };
            let (expected_lane, semantic_record, work_lane, work_cell) = match entry {
                OperationEntry::Fresh {
                    semantic_record,
                    work_lane,
                    work_cell,
                    ..
                }
                | OperationEntry::AliasQuarantine {
                    semantic_record,
                    work_lane,
                    work_cell,
                    ..
                }
                | OperationEntry::Credit {
                    semantic_record,
                    work_lane,
                    work_cell,
                    ..
                } => (
                    match work_lane {
                        WorkLane::Build => RecordLane::Build,
                        WorkLane::Repair => RecordLane::Repair,
                        WorkLane::Observation => RecordLane::Observation,
                    },
                    Some(*semantic_record),
                    Some(*work_lane),
                    Some(*work_cell),
                ),
                OperationEntry::Promote {
                    work_lane,
                    work_cell,
                    ..
                } => (
                    match work_lane {
                        WorkLane::Build => RecordLane::Build,
                        WorkLane::Repair => RecordLane::Repair,
                        WorkLane::Observation => RecordLane::Observation,
                    },
                    None,
                    Some(*work_lane),
                    Some(*work_cell),
                ),
                OperationEntry::Perturb { .. } => {
                    (RecordLane::ExternalPerturbation, None, None, None)
                }
                OperationEntry::Absorb {
                    semantic_record,
                    work_cell,
                    ..
                } => (
                    RecordLane::Observation,
                    Some(*semantic_record),
                    Some(WorkLane::Observation),
                    Some(*work_cell),
                ),
            };
            record.lane == expected_lane
                && semantic_record
                    .map(|id| id < self.M.records.len())
                    .unwrap_or(true)
                && work_lane
                    .zip(work_cell)
                    .map(|(lane, id)| id < self.B_W.pool(lane).cells.len())
                    .unwrap_or(true)
        })
    }

    fn semantic_operation_correspondence_is_well_formed(&self) -> bool {
        let mut semantic_candidates = BTreeSet::new();
        for record in &self.M.records {
            let origin_count = self
                .O
                .records
                .iter()
                .filter(|operation| match operation.entry.as_ref() {
                    Some(OperationEntry::Fresh {
                        semantic_record, ..
                    })
                    | Some(OperationEntry::AliasQuarantine {
                        semantic_record, ..
                    })
                    | Some(OperationEntry::Absorb {
                        semantic_record, ..
                    }) => *semantic_record == record.id,
                    _ => false,
                })
                .count();
            let credit_count = self
                .O
                .records
                .iter()
                .filter(|operation| {
                    matches!(
                        operation.entry.as_ref(),
                        Some(OperationEntry::Credit {
                            semantic_record, ..
                        }) if *semantic_record == record.id
                    )
                })
                .count();
            match (&record.candidate, record.state) {
                (None, SemanticState::Blank) => {
                    if origin_count != 0 || credit_count != 0 {
                        return false;
                    }
                }
                (None, SemanticState::Success | SemanticState::Failure) => {
                    if origin_count != 1 || credit_count != 0 {
                        return false;
                    }
                }
                (Some(candidate), SemanticState::Pending | SemanticState::Success)
                    if candidate.kind == CandidateKind::Fresh =>
                {
                    let expected_credit_count = if record.state == SemanticState::Success {
                        1
                    } else {
                        0
                    };
                    if !semantic_candidates.insert(candidate.clone())
                        || origin_count != 1
                        || credit_count != expected_credit_count
                    {
                        return false;
                    }
                    if candidate.episode == self.Q.episode {
                        let word = candidate.port.child();
                        let node = self
                            .Gamma
                            .workspace
                            .get(&word)
                            .or_else(|| self.Gamma.reservoir.get(&word));
                        if node.map(|node| node.descriptor) != record.descriptor {
                            return false;
                        }
                    }
                }
                (Some(candidate), SemanticState::Failure)
                    if candidate.kind == CandidateKind::Alias =>
                {
                    if !semantic_candidates.insert(candidate.clone())
                        || origin_count != 1
                        || credit_count != 0
                    {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        if semantic_candidates != self.Q.attempt_bits.attempted_identities() {
            return false;
        }
        self.O
            .records
            .iter()
            .all(|operation| match operation.entry.as_ref() {
                Some(OperationEntry::Fresh {
                    candidate,
                    hazard,
                    node,
                    semantic_record,
                    ..
                }) => {
                    hazard.is_positive()
                        && node.word == candidate.port.child()
                        && node.energy_index == node.word.depth()
                        && !node.executable
                        && self.M.records.get(*semantic_record).is_some_and(|record| {
                            record.candidate.as_ref() == Some(candidate)
                                && record.descriptor == Some(node.descriptor)
                                && matches!(
                                    record.state,
                                    SemanticState::Pending | SemanticState::Success
                                )
                        })
                }
                Some(OperationEntry::AliasQuarantine {
                    candidate,
                    hazard,
                    semantic_record,
                    ..
                }) => {
                    hazard.is_positive()
                        && self.M.records.get(*semantic_record).is_some_and(|record| {
                            record.candidate.as_ref() == Some(candidate)
                                && record.state == SemanticState::Failure
                                && record.descriptor.is_none()
                        })
                }
                Some(OperationEntry::Credit {
                    semantic_record, ..
                }) => self.M.records.get(*semantic_record).is_some_and(|record| {
                    record.state == SemanticState::Success
                        && record
                            .candidate
                            .as_ref()
                            .is_some_and(|candidate| candidate.kind == CandidateKind::Fresh)
                }),
                Some(OperationEntry::Absorb {
                    semantic_record,
                    old_queue,
                    ..
                }) => old_queue
                    .observation_payload
                    .as_ref()
                    .is_some_and(|payload| {
                        !payload.consumed
                            && self.M.records.get(*semantic_record).is_some_and(|record| {
                                record.candidate.is_none()
                                    && record.bucket == Some(payload.bucket)
                                    && record.descriptor.is_none()
                                    && record.state
                                        == match payload.outcome {
                                            Outcome::Success => SemanticState::Success,
                                            Outcome::Failure => SemanticState::Failure,
                                        }
                            })
                    }),
                Some(OperationEntry::Promote { .. })
                | Some(OperationEntry::Perturb { .. })
                | None => true,
            })
    }

    fn resource_operation_correspondence_is_well_formed(&self) -> bool {
        let operation_cells = self
            .O
            .records
            .iter()
            .filter_map(|record| match record.entry.as_ref() {
                Some(OperationEntry::Fresh {
                    work_lane,
                    work_cell,
                    ..
                })
                | Some(OperationEntry::AliasQuarantine {
                    work_lane,
                    work_cell,
                    ..
                })
                | Some(OperationEntry::Credit {
                    work_lane,
                    work_cell,
                    ..
                })
                | Some(OperationEntry::Promote {
                    work_lane,
                    work_cell,
                    ..
                }) => Some((*work_lane, *work_cell)),
                Some(OperationEntry::Absorb { work_cell, .. }) => {
                    Some((WorkLane::Observation, *work_cell))
                }
                Some(OperationEntry::Perturb { .. }) | None => None,
            })
            .collect::<Vec<_>>();
        let operation_cell_set = operation_cells.iter().copied().collect::<BTreeSet<_>>();
        let uncharged_cell_set = [&self.B_W.B_build, &self.B_W.B_repair, &self.B_W.B_obs]
            .into_iter()
            .flat_map(|pool| {
                pool.cells
                    .iter()
                    .filter(|cell| !cell.charged)
                    .map(|cell| (pool.lane, cell.id))
            })
            .collect::<BTreeSet<_>>();
        let perturbation_records = self
            .O
            .records
            .iter()
            .filter(|record| matches!(record.entry.as_ref(), Some(OperationEntry::Perturb { .. })))
            .count();
        let external_capacity = self
            .O
            .records
            .iter()
            .filter(|record| record.lane == RecordLane::ExternalPerturbation)
            .count();
        let external_cells_remaining = if self.Q.external_perturbation_cell {
            1
        } else {
            0
        };
        operation_cells.len() == operation_cell_set.len()
            && operation_cell_set == uncharged_cell_set
            && external_capacity <= 1
            && external_capacity == perturbation_records + external_cells_remaining
    }

    fn scheduler_is_well_formed(&self) -> bool {
        let attempts_valid = self.Q.attempt_bits.is_well_formed()
            && self.Q.attempt_bits.declared_count() <= self.M.records.len()
            && self.Q.attempt_bits.bits.iter().all(|bit| {
                bit.identity.as_ref().map_or(true, |candidate| {
                    candidate.episode <= self.Q.episode
                        && self.A.contains(&candidate.port.port)
                        && candidate.port.child().depth() > 0
                })
            })
            && self.Q.repair_target.as_ref().map_or(true, |target| {
                self.Q.attempt_bits.has_repair_pair(self.Q.episode, target)
            });
        let phase_valid = match &self.P {
            Phase::Build { depth } => {
                self.Q.cursor.as_ref().is_some_and(|cursor| {
                    cursor.child().depth() == *depth
                        && self.A.contains(&cursor.port)
                        && self.is_port_open(cursor)
                }) && self.Q.repair_target.is_none()
                    && self.B_W.B_repair.locked
            }
            Phase::Repair { missing } => {
                self.Q.cursor.as_ref() == Some(missing)
                    && self.Q.repair_target.as_ref() == Some(missing)
                    && self.A.contains(&missing.port)
                    && self.is_port_open(missing)
                    && !self.B_W.B_repair.locked
            }
            Phase::Credit { depth, maintenance } => {
                self.Q.cursor.is_none()
                    && !self.pending_frontier_records(*depth).is_empty()
                    && if *maintenance {
                        self.Q.repair_target.is_some() && !self.B_W.B_repair.locked
                    } else {
                        self.Q.repair_target.is_none() && self.B_W.B_repair.locked
                    }
            }
            Phase::Promote { depth, maintenance } => {
                self.Q.cursor.is_none()
                    && self.promotion_records_ready(*depth, *maintenance)
                    && if *maintenance {
                        self.Q.repair_target.is_some() && !self.B_W.B_repair.locked
                    } else {
                        self.Q.repair_target.is_none() && self.B_W.B_repair.locked
                    }
            }
            Phase::Thermal | Phase::Contact => {
                self.Q.cursor.is_none()
                    && self.Q.repair_target.is_none()
                    && self.B_W.B_repair.locked
                    && self.Gamma.workspace.is_empty()
            }
        };
        let perturbation_valid = self.Q.scheduled_perturbation.as_ref().map_or(true, |leaf| {
            matches!(self.P, Phase::Thermal) && self.perturbation_is_enabled(leaf)
        });
        let observation_valid =
            !self.Q.absorb_before_candidates || self.Q.observation_payload.is_some();
        attempts_valid && phase_valid && perturbation_valid && observation_valid
    }

    pub fn complete_state_is_well_formed(&self) -> bool {
        self.A.len() >= 2
            && self.lambda > 0
            && self.A.windows(2).all(|pair| pair[0] < pair[1])
            && self.A.iter().copied().collect::<BTreeSet<_>>().len() == self.A.len()
            && self.Gamma.locally_injective()
            && self
                .Gamma
                .reservoir
                .get(&Word::root())
                .is_some_and(|root| root.descriptor == 0 && root.energy_index == 0)
            && self.F.iter().all(|descriptor| *descriptor != 0)
            && self
                .Gamma
                .reservoir
                .values()
                .chain(self.Gamma.workspace.values())
                .all(|node| node.word == Word::root() || node.descriptor != 0)
            && self.structural_incidence_is_well_formed()
            && self.Gamma.contains_executable(&self.r)
            && self.Gamma.reservoir.values().all(|node| node.executable)
            && self.Gamma.workspace.values().all(|node| !node.executable)
            && self.F.is_disjoint(
                &self
                    .Gamma
                    .reservoir
                    .values()
                    .chain(self.Gamma.workspace.values())
                    .map(|node| node.descriptor)
                    .collect(),
            )
            && self.memory_is_well_formed()
            && self.work_store_is_well_formed()
            && self.operation_bank_is_well_formed()
            && self.semantic_operation_correspondence_is_well_formed()
            && self.resource_operation_correspondence_is_well_formed()
            && self.scheduler_is_well_formed()
            && match self.X.mesostate {
                0 | 2 => self.X.microstate == 0,
                1 => self.X.microstate < 2,
                _ => false,
            }
    }
}

// END AUTHORITATIVE_BUILDER_SURFACE

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolStep {
    pub phase_before: Phase,
    pub phase_after: Phase,
    pub enabled_before: Vec<EnabledEvent>,
    pub selected: EnabledEvent,
    pub receipt: ActionReceipt,
    pub reservoir_words_after: Vec<Word>,
    pub workspace_words_after: Vec<Word>,
    pub free_descriptor_count_after: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathLawStep {
    pub phase_before: Phase,
    pub phase_after: Phase,
    pub enabled_before: Vec<EnabledEvent>,
    pub selected: EnabledEvent,
    pub channel: DrivenChannel,
    pub ledger: LedgerRow,
    pub reservoir_words_after: Vec<Word>,
    pub workspace_words_after: Vec<Word>,
    pub free_descriptor_count_after: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SupportTracePolicy {
    FreshFirst,
    AliasBeforeFresh,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolTrace {
    pub initial_state: CompleteState,
    pub terminal_state: CompleteState,
    pub receipts: Vec<ActionReceipt>,
    pub forward_steps: Vec<ProtocolStep>,
    pub reverse_lifo_receipts: Vec<ActionReceipt>,
    pub every_forward_map_has_exact_local_inverse: bool,
    pub full_reverse_lifo_restores_initial: bool,
    pub every_working_ledger_closes: bool,
    pub every_augmented_ledger_closes: bool,
    pub every_complete_ledger_closes: bool,
    pub every_forward_energy_difference_matches_ledger: bool,
    pub every_forward_resource_difference_matches_ledger: bool,
    pub every_state_has_nontruncating_attempt_bit_capacity: bool,
}

impl ProtocolTrace {
    pub fn channel_count(&self, channel: DrivenChannel) -> usize {
        self.receipts
            .iter()
            .filter(|receipt| receipt.ledger.channel == channel)
            .count()
    }

    pub fn path_law_projection(&self) -> Vec<PathLawStep> {
        self.forward_steps
            .iter()
            .map(|step| PathLawStep {
                phase_before: step.phase_before.clone(),
                phase_after: step.phase_after.clone(),
                enabled_before: step.enabled_before.clone(),
                selected: step.selected.clone(),
                channel: step.receipt.ledger.channel,
                ledger: step.receipt.ledger.clone(),
                reservoir_words_after: step.reservoir_words_after.clone(),
                workspace_words_after: step.workspace_words_after.clone(),
                free_descriptor_count_after: step.free_descriptor_count_after,
            })
            .collect()
    }
}

pub fn instantiate_one_support_trace(state: CompleteState) -> Result<ProtocolTrace, ActionError> {
    instantiate_support_trace(state, SupportTracePolicy::FreshFirst)
}

pub fn instantiate_alias_before_fresh_support_trace(
    state: CompleteState,
) -> Result<ProtocolTrace, ActionError> {
    instantiate_support_trace(state, SupportTracePolicy::AliasBeforeFresh)
}

pub fn instantiate_support_trace(
    mut state: CompleteState,
    policy: SupportTracePolicy,
) -> Result<ProtocolTrace, ActionError> {
    let initial_state = state.clone();
    let mut receipts = Vec::new();
    let mut forward_steps = Vec::new();
    let mut every_forward_map_has_exact_local_inverse = true;
    let mut every_forward_energy_difference_matches_ledger = true;
    let mut every_forward_resource_difference_matches_ledger = true;
    let mut every_state_has_nontruncating_attempt_bit_capacity =
        state.attempt_bit_capacity_nontruncating_now();
    loop {
        let before = state.clone();
        let enabled_before = state.enabled_events();
        if enabled_before.is_empty() {
            break;
        }
        let selected = if let Some(absorb) = enabled_before
            .iter()
            .find(|event| matches!(event, EnabledEvent::Absorb { .. }))
        {
            absorb.clone()
        } else if enabled_before
            .iter()
            .any(|event| matches!(event, EnabledEvent::Candidate { .. }))
        {
            let preferred_kind = match policy {
                SupportTracePolicy::FreshFirst => CandidateKind::Fresh,
                SupportTracePolicy::AliasBeforeFresh => CandidateKind::Alias,
            };
            enabled_before
                .iter()
                .find(|event| {
                    matches!(
                        event,
                        EnabledEvent::Candidate { candidate, .. }
                            if candidate.kind == preferred_kind
                    )
                })
                .or_else(|| {
                    enabled_before
                        .iter()
                        .find(|event| matches!(event, EnabledEvent::Candidate { .. }))
                })
                .cloned()
                .ok_or(ActionError::EventNotEnabled)?
        } else {
            enabled_before
                .first()
                .cloned()
                .ok_or(ActionError::EventNotEnabled)?
        };
        let before_resources = before.resource_snapshot();
        let receipt = state.apply_event(&selected.event())?;
        every_state_has_nontruncating_attempt_bit_capacity &= before
            .attempt_bit_capacity_nontruncating_now()
            && state.attempt_bit_capacity_nontruncating_now();
        let mut reversed = state.clone();
        every_forward_map_has_exact_local_inverse &=
            reversed.reverse_last().is_ok() && reversed == before;
        let delta_working = state.working_energy() as i128 - before.working_energy() as i128;
        let delta_augmented =
            state.augmented_internal_energy() as i128 - before.augmented_internal_energy() as i128;
        let delta_complete = state.augmented_energy_with_external_perturbation_cell() as i128
            - before.augmented_energy_with_external_perturbation_cell() as i128;
        every_forward_energy_difference_matches_ledger &= delta_working
            == receipt.ledger.delta_working
            && delta_augmented == receipt.ledger.delta_augmented
            && delta_complete == receipt.ledger.delta_complete;
        every_forward_resource_difference_matches_ledger &=
            state.resource_snapshot().difference_from(&before_resources)
                == receipt.ledger.resources;
        forward_steps.push(ProtocolStep {
            phase_before: before.P,
            phase_after: state.P.clone(),
            enabled_before,
            selected,
            receipt: receipt.clone(),
            reservoir_words_after: state.Gamma.reservoir.keys().cloned().collect(),
            workspace_words_after: state.Gamma.workspace.keys().cloned().collect(),
            free_descriptor_count_after: state.F.len(),
        });
        receipts.push(receipt);
    }
    let terminal_state = state.clone();
    let initial_inverse_depth = initial_state.O.inverse_order.len();
    let mut reversed = terminal_state.clone();
    let mut reverse_lifo_receipts = Vec::new();
    while reversed.O.inverse_order.len() > initial_inverse_depth {
        reverse_lifo_receipts.push(reversed.reverse_last()?);
    }
    let full_reverse_lifo_restores_initial = reversed == initial_state;
    let every_working_ledger_closes = receipts
        .iter()
        .chain(reverse_lifo_receipts.iter())
        .all(|receipt| receipt.ledger.working_first_law_closes());
    let every_augmented_ledger_closes = receipts
        .iter()
        .chain(reverse_lifo_receipts.iter())
        .all(|receipt| receipt.ledger.internal_augmented_first_law_closes());
    let every_complete_ledger_closes = receipts
        .iter()
        .chain(reverse_lifo_receipts.iter())
        .all(|receipt| receipt.ledger.complete_first_law_closes());
    Ok(ProtocolTrace {
        initial_state,
        terminal_state,
        receipts,
        forward_steps,
        reverse_lifo_receipts,
        every_forward_map_has_exact_local_inverse,
        full_reverse_lifo_restores_initial,
        every_working_ledger_closes,
        every_augmented_ledger_closes,
        every_complete_ledger_closes,
        every_forward_energy_difference_matches_ledger,
        every_forward_resource_difference_matches_ledger,
        every_state_has_nontruncating_attempt_bit_capacity,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescriptorOrderAudit {
    pub forward: ProtocolTrace,
    pub reversed: ProtocolTrace,
    pub path_law_quotient_identical: bool,
    pub promoted_words_identical: bool,
    pub shell_counts_identical: bool,
}

pub fn descriptor_order_audit(
    base: usize,
    off_shell_inventory: bool,
) -> Result<DescriptorOrderAudit, ActionError> {
    let forward = instantiate_one_support_trace(primary_fixture_with_order(
        base,
        ENERGY_QUANTUM,
        off_shell_inventory,
        DescriptorOrder::Forward,
    ))?;
    let reversed = instantiate_one_support_trace(primary_fixture_with_order(
        base,
        ENERGY_QUANTUM,
        off_shell_inventory,
        DescriptorOrder::Reversed,
    ))?;
    let path_law_quotient_identical =
        forward.path_law_projection() == reversed.path_law_projection();
    let promoted_words_identical = forward.terminal_state.Gamma.reservoir.keys().eq(reversed
        .terminal_state
        .Gamma
        .reservoir
        .keys());
    let maximum_depth = forward
        .terminal_state
        .Gamma
        .deepest_promoted_depth()
        .max(reversed.terminal_state.Gamma.deepest_promoted_depth());
    let shell_counts_identical = (0..=maximum_depth).all(|depth| {
        forward.terminal_state.Gamma.shell_count(depth)
            == reversed.terminal_state.Gamma.shell_count(depth)
    });
    Ok(DescriptorOrderAudit {
        forward,
        reversed,
        path_law_quotient_identical,
        promoted_words_identical,
        shell_counts_identical,
    })
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum FirstCandidateOutcome {
    FreshFirst,
    AliasFirst,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhysicalOutcomeRecord {
    pub outcome: FirstCandidateOutcome,
    pub probability: Ratio,
    pub semantic_record: SemanticRecord,
    pub operation_record: OperationRecord,
    pub record_identifies_outcome: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhysicalBranchRecordAudit {
    pub fresh_hazard: Ratio,
    pub alias_hazard: Ratio,
    pub fresh: PhysicalOutcomeRecord,
    pub alias: PhysicalOutcomeRecord,
    pub records_are_distinct: bool,
    pub records_injectively_identify_first_event: bool,
}

fn physical_outcome_record(
    state: &CompleteState,
    enabled: &EnabledEvent,
    outcome: FirstCandidateOutcome,
    probability: Ratio,
) -> Result<PhysicalOutcomeRecord, ActionError> {
    let transition = state.event_transition(enabled, KernelSupport::PRIMARY)?;
    let operation = transition
        .successor
        .O
        .records
        .get(transition.forward_receipt.operation_record)
        .cloned()
        .ok_or(ActionError::OperationRecordUnavailable)?;
    let semantic_record_id = match operation.entry.as_ref() {
        Some(OperationEntry::Fresh {
            semantic_record, ..
        })
        | Some(OperationEntry::AliasQuarantine {
            semantic_record, ..
        }) => *semantic_record,
        _ => return Err(ActionError::InverseStateMismatch),
    };
    let semantic_record = transition
        .successor
        .M
        .records
        .get(semantic_record_id)
        .cloned()
        .ok_or(ActionError::SemanticRecordUnavailable)?;
    let record_identifies_outcome = match outcome {
        FirstCandidateOutcome::FreshFirst => {
            semantic_record
                .candidate
                .as_ref()
                .is_some_and(|candidate| candidate.kind == CandidateKind::Fresh)
                && semantic_record.state == SemanticState::Pending
                && matches!(operation.entry.as_ref(), Some(OperationEntry::Fresh { .. }))
        }
        FirstCandidateOutcome::AliasFirst => {
            semantic_record
                .candidate
                .as_ref()
                .is_some_and(|candidate| candidate.kind == CandidateKind::Alias)
                && semantic_record.state == SemanticState::Failure
                && matches!(
                    operation.entry.as_ref(),
                    Some(OperationEntry::AliasQuarantine { .. })
                )
        }
    };
    Ok(PhysicalOutcomeRecord {
        outcome,
        probability,
        semantic_record,
        operation_record: operation,
        record_identifies_outcome,
    })
}

pub fn physical_branch_record_audit(
    pre_candidate_state: &CompleteState,
) -> Result<PhysicalBranchRecordAudit, ActionError> {
    let candidates = pre_candidate_state
        .enabled_events()
        .into_iter()
        .filter(|event| matches!(event, EnabledEvent::Candidate { .. }))
        .collect::<Vec<_>>();
    let fresh_enabled = candidates
        .iter()
        .find(|event| {
            matches!(
                event,
                EnabledEvent::Candidate { candidate, .. }
                    if candidate.kind == CandidateKind::Fresh
            )
        })
        .cloned()
        .ok_or(ActionError::WrongCandidate)?;
    let alias_enabled = candidates
        .iter()
        .find(|event| {
            matches!(
                event,
                EnabledEvent::Candidate { candidate, .. }
                    if candidate.kind == CandidateKind::Alias
            )
        })
        .cloned()
        .ok_or(ActionError::WrongCandidate)?;
    let fresh_hazard = fresh_enabled.hazard();
    let alias_hazard = alias_enabled.hazard();
    let total = fresh_hazard + alias_hazard;
    let fresh_probability = fresh_hazard / total;
    let alias_probability = alias_hazard / total;
    let fresh = physical_outcome_record(
        pre_candidate_state,
        &fresh_enabled,
        FirstCandidateOutcome::FreshFirst,
        fresh_probability,
    )?;
    let alias = physical_outcome_record(
        pre_candidate_state,
        &alias_enabled,
        FirstCandidateOutcome::AliasFirst,
        alias_probability,
    )?;
    let records_are_distinct = fresh.semantic_record != alias.semantic_record
        && fresh.operation_record.entry != alias.operation_record.entry;
    let records_injectively_identify_first_event =
        fresh.record_identifies_outcome && alias.record_identifies_outcome && records_are_distinct;
    Ok(PhysicalBranchRecordAudit {
        fresh_hazard,
        alias_hazard,
        fresh,
        alias,
        records_are_distinct,
        records_injectively_identify_first_event,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructionQuotientRepresentative {
    pub base: usize,
    pub frontier_depth: usize,
    pub requested_prior_alias_failures: usize,
    pub initial_state: CompleteState,
    pub pre_candidate_state: CompleteState,
    pub reachability_steps: Vec<ProtocolStep>,
    pub target_port: PortRef,
    pub fresh_bucket_counts: (u32, u32),
    pub alias_bucket_counts: (u32, u32),
    pub every_step_used_enabled_event_seam: bool,
    pub every_step_has_exact_inverse: bool,
    pub every_state_has_nontruncating_attempt_bit_capacity: bool,
    pub quotient_counts_match_request: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreshFirstConstructionBranch {
    pub first_event_record: PhysicalOutcomeRecord,
    pub fresh_semantic_record: SemanticRecordId,
    pub immediate_fresh_state: SemanticState,
    pub terminal_fresh_state: SemanticState,
    pub terminal_fresh_record: SemanticRecord,
    pub candidate_attempts: u8,
    pub continuation_steps: Vec<ProtocolStep>,
    pub credit_receipt: ActionReceipt,
    pub eventual_credit_updates_same_fresh_record: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AliasFirstConstructionBranch {
    pub first_event_record: PhysicalOutcomeRecord,
    pub alias_semantic_record: SemanticRecordId,
    pub immediate_alias_state: SemanticState,
    pub terminal_alias_state: SemanticState,
    pub terminal_alias_record: SemanticRecord,
    pub fresh_semantic_record: SemanticRecordId,
    pub immediate_fresh_state: SemanticState,
    pub terminal_fresh_state: SemanticState,
    pub terminal_fresh_record: SemanticRecord,
    pub candidate_attempts: u8,
    pub alias_then_fresh_is_forced: bool,
    pub continuation_steps: Vec<ProtocolStep>,
    pub credit_receipt: ActionReceipt,
    pub eventual_credit_updates_same_fresh_record: bool,
    pub alias_record_stays_failure_through_credit: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructionQuotientPhysicalAudit {
    pub representative: ConstructionQuotientRepresentative,
    pub first_event_records: PhysicalBranchRecordAudit,
    pub fresh_first: FreshFirstConstructionBranch,
    pub alias_first: AliasFirstConstructionBranch,
    pub branch_probabilities_are_normalized: bool,
    pub representative_is_reachable_quotient: bool,
    pub branch_hazards_match_representative: bool,
    pub records_injectively_copy_first_outcome: bool,
    pub both_branches_complete_required_record_protocol: bool,
}

pub fn construction_quotient_prior_fresh_successes(base: usize, frontier_depth: usize) -> usize {
    assert!((2..=4).contains(&base));
    assert!((1..=3).contains(&frontier_depth));
    (1..frontier_depth)
        .map(|completed_depth| checked_pow(base, (completed_depth - 1) as u32))
        .sum()
}

pub fn construction_quotient_maximum_prior_alias_failures(
    base: usize,
    frontier_depth: usize,
) -> usize {
    construction_quotient_prior_fresh_successes(base, frontier_depth)
        + checked_pow(base, (frontier_depth - 1) as u32)
        - 1
}

fn protocol_step_from_transition(
    source: &CompleteState,
    enabled_before: Vec<EnabledEvent>,
    selected: EnabledEvent,
    transition: &EventTransition,
) -> ProtocolStep {
    ProtocolStep {
        phase_before: source.P.clone(),
        phase_after: transition.successor.P.clone(),
        enabled_before,
        selected,
        receipt: transition.forward_receipt.clone(),
        reservoir_words_after: transition
            .successor
            .Gamma
            .reservoir
            .keys()
            .cloned()
            .collect(),
        workspace_words_after: transition
            .successor
            .Gamma
            .workspace
            .keys()
            .cloned()
            .collect(),
        free_descriptor_count_after: transition.successor.F.len(),
    }
}

fn apply_enabled_transition(
    state: &mut CompleteState,
    selected: EnabledEvent,
) -> Result<(ProtocolStep, EventTransition), ActionError> {
    let source = state.clone();
    let enabled_before = source.enabled_events();
    if !enabled_before.contains(&selected) {
        return Err(ActionError::EventNotEnabled);
    }
    let transition = source.event_transition(&selected, KernelSupport::PRIMARY)?;
    let step = protocol_step_from_transition(&source, enabled_before, selected, &transition);
    *state = transition.successor.clone();
    Ok((step, transition))
}

fn candidate_enabled_event(state: &CompleteState, kind: CandidateKind) -> Option<EnabledEvent> {
    state.enabled_events().into_iter().find(|event| {
        matches!(
            event,
            EnabledEvent::Candidate { candidate, .. } if candidate.kind == kind
        )
    })
}

fn candidate_semantic_record(
    state: &CompleteState,
    receipt: &ActionReceipt,
) -> Result<SemanticRecordId, ActionError> {
    match state
        .O
        .records
        .get(receipt.operation_record)
        .and_then(|record| record.entry.as_ref())
    {
        Some(OperationEntry::Fresh {
            semantic_record, ..
        })
        | Some(OperationEntry::AliasQuarantine {
            semantic_record, ..
        }) => Ok(*semantic_record),
        _ => Err(ActionError::InverseStateMismatch),
    }
}

fn credit_updates_semantic_record(
    state: &CompleteState,
    receipt: &ActionReceipt,
    expected: SemanticRecordId,
) -> bool {
    matches!(
        state
            .O
            .records
            .get(receipt.operation_record)
            .and_then(|record| record.entry.as_ref()),
        Some(OperationEntry::Credit {
            semantic_record, ..
        }) if *semantic_record == expected
    )
}

fn drive_until_record_is_credited(
    state: &mut CompleteState,
    fresh_semantic_record: SemanticRecordId,
) -> Result<(Vec<ProtocolStep>, ActionReceipt), ActionError> {
    let mut steps = Vec::new();
    loop {
        let enabled = state.enabled_events();
        let selected = if let Some(fresh) = enabled.iter().find(|event| {
            matches!(
                event,
                EnabledEvent::Candidate { candidate, .. }
                    if candidate.kind == CandidateKind::Fresh
            )
        }) {
            fresh.clone()
        } else {
            enabled
                .first()
                .cloned()
                .ok_or(ActionError::PendingCreditUnavailable)?
        };
        let (step, transition) = apply_enabled_transition(state, selected)?;
        let receipt = transition.forward_receipt;
        let credited_target =
            credit_updates_semantic_record(state, &receipt, fresh_semantic_record);
        steps.push(step);
        if credited_target {
            return Ok((steps, receipt));
        }
        if state
            .M
            .records
            .get(fresh_semantic_record)
            .is_some_and(|record| record.state != SemanticState::Pending)
        {
            return Err(ActionError::InverseStateMismatch);
        }
    }
}

/// The first symbol at the lexicographically last parent has every earlier
/// same-symbol opportunity behind it. Selecting ALIAS on the first `r` of
/// those opportunities realizes quotient `r` with one path, rather than
/// enumerating every FRESH/ALIAS construction history.
pub fn construction_quotient_representative(
    base: usize,
    frontier_depth: usize,
    prior_alias_failures: usize,
) -> Result<ConstructionQuotientRepresentative, ActionError> {
    if !(2..=4).contains(&base)
        || !(1..=3).contains(&frontier_depth)
        || prior_alias_failures
            > construction_quotient_maximum_prior_alias_failures(base, frontier_depth)
    {
        return Err(ActionError::WrongCandidate);
    }
    let initial_state = primary_fixture(base, false);
    let mut state = initial_state.clone();
    let target_symbol = state.A[0];
    let target_parent = Word(vec![
        *state.A.last().ok_or(ActionError::WrongCandidate)?;
        frontier_depth - 1
    ]);
    let target_port = PortRef {
        parent: target_parent,
        port: target_symbol,
    };
    let mut aliases_remaining = prior_alias_failures;
    let mut reachability_steps = Vec::new();
    let mut every_step_has_exact_inverse = true;
    let mut every_state_has_nontruncating_attempt_bit_capacity =
        state.attempt_bit_capacity_nontruncating_now();
    loop {
        if state.P
            == (Phase::Build {
                depth: frontier_depth,
            })
            && state.Q.cursor.as_ref() == Some(&target_port)
        {
            break;
        }
        let enabled = state.enabled_events();
        let selected = if aliases_remaining > 0 {
            enabled
                .iter()
                .find(|event| {
                    matches!(
                        event,
                        EnabledEvent::Candidate { candidate, .. }
                            if candidate.kind == CandidateKind::Alias
                                && candidate.port.port == target_symbol
                    )
                })
                .or_else(|| {
                    enabled.iter().find(|event| {
                        matches!(
                            event,
                            EnabledEvent::Candidate { candidate, .. }
                                if candidate.kind == CandidateKind::Fresh
                        )
                    })
                })
                .or_else(|| enabled.first())
                .cloned()
                .ok_or(ActionError::EventNotEnabled)?
        } else {
            enabled
                .iter()
                .find(|event| {
                    matches!(
                        event,
                        EnabledEvent::Candidate { candidate, .. }
                            if candidate.kind == CandidateKind::Fresh
                    )
                })
                .or_else(|| enabled.first())
                .cloned()
                .ok_or(ActionError::EventNotEnabled)?
        };
        if matches!(
            &selected,
            EnabledEvent::Candidate { candidate, .. }
                if candidate.kind == CandidateKind::Alias
        ) {
            aliases_remaining = aliases_remaining
                .checked_sub(1)
                .ok_or(ActionError::InverseStateMismatch)?;
        }
        let (step, transition) = apply_enabled_transition(&mut state, selected)?;
        every_step_has_exact_inverse &= transition.inverse_restores_source;
        every_state_has_nontruncating_attempt_bit_capacity &=
            state.attempt_bit_capacity_nontruncating_now();
        reachability_steps.push(step);
    }
    let fresh_bucket = BucketKey {
        port: target_symbol,
        kind: CandidateKind::Fresh,
    };
    let alias_bucket = BucketKey {
        port: target_symbol,
        kind: CandidateKind::Alias,
    };
    let fresh_bucket_counts = state.M.terminal_counts(fresh_bucket);
    let alias_bucket_counts = state.M.terminal_counts(alias_bucket);
    let expected_fresh_successes =
        construction_quotient_prior_fresh_successes(base, frontier_depth);
    let quotient_counts_match_request = aliases_remaining == 0
        && fresh_bucket_counts
            == (
                u32::try_from(expected_fresh_successes)
                    .map_err(|_| ActionError::InverseStateMismatch)?,
                u32::try_from(expected_fresh_successes)
                    .map_err(|_| ActionError::InverseStateMismatch)?,
            )
        && alias_bucket_counts
            == (
                0,
                u32::try_from(prior_alias_failures)
                    .map_err(|_| ActionError::InverseStateMismatch)?,
            );
    let every_step_used_enabled_event_seam = reachability_steps.iter().all(|step| {
        step.enabled_before.contains(&step.selected)
            && step.receipt.event == step.selected.event()
            && step.receipt.hazard == step.selected.hazard()
    });
    every_step_has_exact_inverse &= reachability_steps.iter().all(|step| {
        step.receipt.ledger.working_first_law_closes()
            && step.receipt.ledger.internal_augmented_first_law_closes()
            && step.receipt.ledger.complete_first_law_closes()
    });
    Ok(ConstructionQuotientRepresentative {
        base,
        frontier_depth,
        requested_prior_alias_failures: prior_alias_failures,
        initial_state,
        pre_candidate_state: state,
        reachability_steps,
        target_port,
        fresh_bucket_counts,
        alias_bucket_counts,
        every_step_used_enabled_event_seam,
        every_step_has_exact_inverse,
        every_state_has_nontruncating_attempt_bit_capacity,
        quotient_counts_match_request,
    })
}

pub fn construction_quotient_physical_audit(
    base: usize,
    frontier_depth: usize,
    prior_alias_failures: usize,
) -> Result<ConstructionQuotientPhysicalAudit, ActionError> {
    let representative =
        construction_quotient_representative(base, frontier_depth, prior_alias_failures)?;
    let first_event_records = physical_branch_record_audit(&representative.pre_candidate_state)?;

    let mut fresh_state = representative.pre_candidate_state.clone();
    let fresh_enabled = candidate_enabled_event(&fresh_state, CandidateKind::Fresh)
        .ok_or(ActionError::WrongCandidate)?;
    let (_, fresh_transition) = apply_enabled_transition(&mut fresh_state, fresh_enabled)?;
    let fresh_semantic_record =
        candidate_semantic_record(&fresh_state, &fresh_transition.forward_receipt)?;
    let fresh_first_candidate_attempts = u8::try_from(
        [&fresh_transition.forward_receipt]
            .into_iter()
            .filter(|receipt| matches!(&receipt.event, ModelEvent::Candidate(_)))
            .count(),
    )
    .map_err(|_| ActionError::InverseStateMismatch)?;
    let immediate_fresh_state = fresh_state.M.records[fresh_semantic_record].state;
    let (fresh_continuation_steps, fresh_credit_receipt) =
        drive_until_record_is_credited(&mut fresh_state, fresh_semantic_record)?;
    let terminal_fresh_state = fresh_state.M.records[fresh_semantic_record].state;
    let terminal_fresh_record = fresh_state.M.records[fresh_semantic_record].clone();
    let fresh_credit_updates_same_record =
        credit_updates_semantic_record(&fresh_state, &fresh_credit_receipt, fresh_semantic_record);
    let fresh_first = FreshFirstConstructionBranch {
        first_event_record: first_event_records.fresh.clone(),
        fresh_semantic_record,
        immediate_fresh_state,
        terminal_fresh_state,
        terminal_fresh_record,
        candidate_attempts: fresh_first_candidate_attempts,
        continuation_steps: fresh_continuation_steps,
        credit_receipt: fresh_credit_receipt,
        eventual_credit_updates_same_fresh_record: fresh_credit_updates_same_record,
    };

    let mut alias_state = representative.pre_candidate_state.clone();
    let alias_enabled = candidate_enabled_event(&alias_state, CandidateKind::Alias)
        .ok_or(ActionError::WrongCandidate)?;
    let (_, alias_transition) = apply_enabled_transition(&mut alias_state, alias_enabled)?;
    let alias_semantic_record =
        candidate_semantic_record(&alias_state, &alias_transition.forward_receipt)?;
    let immediate_alias_state = alias_state.M.records[alias_semantic_record].state;
    let candidates_after_alias = alias_state
        .enabled_events()
        .into_iter()
        .filter(|event| matches!(event, EnabledEvent::Candidate { .. }))
        .collect::<Vec<_>>();
    let alias_then_fresh_is_forced = candidates_after_alias.len() == 1
        && matches!(
            &candidates_after_alias[0],
            EnabledEvent::Candidate { candidate, .. }
                if candidate.kind == CandidateKind::Fresh
                    && candidate.port == representative.target_port
        );
    let forced_fresh = candidates_after_alias
        .first()
        .cloned()
        .ok_or(ActionError::WrongCandidate)?;
    if !alias_then_fresh_is_forced {
        return Err(ActionError::InverseStateMismatch);
    }
    let (_, alias_then_fresh_transition) =
        apply_enabled_transition(&mut alias_state, forced_fresh)?;
    let alias_branch_fresh_semantic_record =
        candidate_semantic_record(&alias_state, &alias_then_fresh_transition.forward_receipt)?;
    let alias_first_candidate_attempts = u8::try_from(
        [
            &alias_transition.forward_receipt,
            &alias_then_fresh_transition.forward_receipt,
        ]
        .into_iter()
        .filter(|receipt| matches!(&receipt.event, ModelEvent::Candidate(_)))
        .count(),
    )
    .map_err(|_| ActionError::InverseStateMismatch)?;
    let alias_branch_immediate_fresh_state =
        alias_state.M.records[alias_branch_fresh_semantic_record].state;
    let (alias_continuation_steps, alias_credit_receipt) =
        drive_until_record_is_credited(&mut alias_state, alias_branch_fresh_semantic_record)?;
    let alias_branch_terminal_fresh_state =
        alias_state.M.records[alias_branch_fresh_semantic_record].state;
    let terminal_alias_state = alias_state.M.records[alias_semantic_record].state;
    let terminal_alias_record = alias_state.M.records[alias_semantic_record].clone();
    let alias_branch_terminal_fresh_record =
        alias_state.M.records[alias_branch_fresh_semantic_record].clone();
    let alias_credit_updates_same_record = credit_updates_semantic_record(
        &alias_state,
        &alias_credit_receipt,
        alias_branch_fresh_semantic_record,
    );
    let alias_first = AliasFirstConstructionBranch {
        first_event_record: first_event_records.alias.clone(),
        alias_semantic_record,
        immediate_alias_state,
        terminal_alias_state,
        terminal_alias_record,
        fresh_semantic_record: alias_branch_fresh_semantic_record,
        immediate_fresh_state: alias_branch_immediate_fresh_state,
        terminal_fresh_state: alias_branch_terminal_fresh_state,
        terminal_fresh_record: alias_branch_terminal_fresh_record,
        candidate_attempts: alias_first_candidate_attempts,
        alias_then_fresh_is_forced,
        continuation_steps: alias_continuation_steps,
        credit_receipt: alias_credit_receipt,
        eventual_credit_updates_same_fresh_record: alias_credit_updates_same_record,
        alias_record_stays_failure_through_credit: immediate_alias_state == SemanticState::Failure
            && terminal_alias_state == SemanticState::Failure,
    };

    let branch_probabilities_are_normalized =
        first_event_records.fresh.probability + first_event_records.alias.probability == Ratio::ONE;
    let representative_is_reachable_quotient = representative.every_step_used_enabled_event_seam
        && representative.every_step_has_exact_inverse
        && representative.every_state_has_nontruncating_attempt_bit_capacity
        && representative.quotient_counts_match_request
        && representative
            .pre_candidate_state
            .complete_state_is_well_formed();
    let branch_hazards_match_representative = first_event_records.fresh_hazard
        == representative
            .pre_candidate_state
            .candidate_hazard(
                first_event_records
                    .fresh
                    .semantic_record
                    .candidate
                    .as_ref()
                    .ok_or(ActionError::WrongCandidate)?,
            )
            .ok_or(ActionError::WrongCandidate)?
        && first_event_records.alias_hazard
            == representative
                .pre_candidate_state
                .candidate_hazard(
                    first_event_records
                        .alias
                        .semantic_record
                        .candidate
                        .as_ref()
                        .ok_or(ActionError::WrongCandidate)?,
                )
                .ok_or(ActionError::WrongCandidate)?;
    let terminal_fresh_identifies_outcome = fresh_first
        .terminal_fresh_record
        .candidate
        .as_ref()
        .is_some_and(|candidate| candidate.kind == CandidateKind::Fresh)
        && fresh_first.terminal_fresh_record.state == SemanticState::Success
        && matches!(
            fresh_first
                .first_event_record
                .operation_record
                .entry
                .as_ref(),
            Some(OperationEntry::Fresh { .. })
        );
    let terminal_alias_identifies_outcome = alias_first
        .terminal_alias_record
        .candidate
        .as_ref()
        .is_some_and(|candidate| candidate.kind == CandidateKind::Alias)
        && alias_first.terminal_alias_record.state == SemanticState::Failure
        && matches!(
            alias_first
                .first_event_record
                .operation_record
                .entry
                .as_ref(),
            Some(OperationEntry::AliasQuarantine { .. })
        );
    let records_injectively_copy_first_outcome = first_event_records
        .records_injectively_identify_first_event
        && terminal_fresh_identifies_outcome
        && terminal_alias_identifies_outcome
        && fresh_first.terminal_fresh_record != alias_first.terminal_alias_record;
    let both_branches_complete_required_record_protocol = fresh_first.immediate_fresh_state
        == SemanticState::Pending
        && fresh_first.fresh_semantic_record == first_event_records.fresh.semantic_record.id
        && fresh_first.terminal_fresh_state == SemanticState::Success
        && fresh_first.terminal_fresh_record.id == fresh_first.fresh_semantic_record
        && fresh_first.terminal_fresh_record.state == SemanticState::Success
        && fresh_first.eventual_credit_updates_same_fresh_record
        && alias_first.immediate_alias_state == SemanticState::Failure
        && alias_first.alias_semantic_record == first_event_records.alias.semantic_record.id
        && alias_first.terminal_alias_state == SemanticState::Failure
        && alias_first.terminal_alias_record.id == alias_first.alias_semantic_record
        && alias_first.terminal_alias_record.state == SemanticState::Failure
        && alias_first.fresh_semantic_record != alias_first.alias_semantic_record
        && alias_first.immediate_fresh_state == SemanticState::Pending
        && alias_first.terminal_fresh_state == SemanticState::Success
        && alias_first.terminal_fresh_record.id == alias_first.fresh_semantic_record
        && alias_first.terminal_fresh_record.state == SemanticState::Success
        && alias_first.alias_then_fresh_is_forced
        && alias_first.eventual_credit_updates_same_fresh_record
        && alias_first.alias_record_stays_failure_through_credit;
    Ok(ConstructionQuotientPhysicalAudit {
        representative,
        first_event_records,
        fresh_first,
        alias_first,
        branch_probabilities_are_normalized,
        representative_is_reachable_quotient,
        branch_hazards_match_representative,
        records_injectively_copy_first_outcome,
        both_branches_complete_required_record_protocol,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScmMemoryIntervention {
    pub observed_state: CompleteState,
    pub intervened_state: CompleteState,
    pub semantic_record: SemanticRecordId,
    pub observed_bucket: BucketKey,
    pub intervened_bucket: BucketKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PostInterventionKernelAudit {
    pub support: KernelSupport,
    pub enabled_events: Vec<EnabledEvent>,
    pub transitions: Vec<EventTransition>,
    pub kernel_is_defined: bool,
    pub every_transition_replays_from_intervened_state: bool,
}

impl ScmMemoryIntervention {
    /// Evaluate the ordinary event kernel on the surgically intervened state.
    /// The source need not satisfy the autonomous-history correspondence: SCM
    /// surgery replaces M while deliberately leaving its historical ABSORB
    /// record untouched.
    pub fn post_intervention_kernel(
        &self,
        support: KernelSupport,
    ) -> Result<PostInterventionKernelAudit, ActionError> {
        let enabled_events = self.intervened_state.enabled_events_with_support(support);
        let transitions = enabled_events
            .iter()
            .map(|enabled| self.intervened_state.event_transition(enabled, support))
            .collect::<Result<Vec<_>, _>>()?;
        let every_transition_replays_from_intervened_state = transitions.iter().all(|transition| {
            transition.inverse_restores_source
                && transition.enabled.hazard() == transition.hazard
                && transition.forward_receipt.event == transition.enabled.event()
                && transition.forward_receipt.hazard == transition.hazard
                && transition.ledger == transition.forward_receipt.ledger
                && transition.ledger.working_first_law_closes()
                && transition.ledger.internal_augmented_first_law_closes()
                && transition.ledger.complete_first_law_closes()
        });
        Ok(PostInterventionKernelAudit {
            support,
            kernel_is_defined: !enabled_events.is_empty()
                && transitions.len() == enabled_events.len(),
            enabled_events,
            transitions,
            every_transition_replays_from_intervened_state,
        })
    }
}

pub fn scm_memory_intervention() -> Result<ScmMemoryIntervention, ActionError> {
    let mut observed_state = causal_memory_fixture();
    let receipt = observed_state.absorb()?;
    let semantic_record = match &observed_state.O.records[receipt.operation_record].entry {
        Some(OperationEntry::Absorb {
            semantic_record, ..
        }) => *semantic_record,
        _ => return Err(ActionError::InverseStateMismatch),
    };
    let observed_bucket = observed_state.M.records[semantic_record]
        .bucket
        .ok_or(ActionError::InverseStateMismatch)?;
    let intervened_bucket = BucketKey {
        port: 2,
        kind: CandidateKind::Fresh,
    };
    let mut intervened_state = observed_state.clone();
    intervened_state.M.records[semantic_record].bucket = Some(intervened_bucket);
    Ok(ScmMemoryIntervention {
        observed_state,
        intervened_state,
        semantic_record,
        observed_bucket,
        intervened_bucket,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryInterventionAudit {
    pub absorb_changes_no_graph_coordinate: bool,
    pub absorb_consumes_one_observation_cell: bool,
    pub absorb_writes_one_terminal_record: bool,
    pub absorb_has_exact_inverse: bool,
    pub observed_live_bucket_counts: (u32, u32),
    pub intervened_live_bucket_counts: (u32, u32),
    pub full_non_memory_coordinates_identical: bool,
    pub candidate_support_identical: bool,
    pub observed_state_is_well_formed: bool,
    pub intervention_is_off_manifold_scm: bool,
    pub exactly_one_memory_coordinate_replaced: bool,
    pub historical_absorb_record_is_unchanged: bool,
    pub post_intervention_kernel_defined: bool,
    pub every_post_intervention_transition_replays: bool,
    pub intervention: ScmMemoryIntervention,
    pub post_intervention_kernel: PostInterventionKernelAudit,
}

pub fn memory_intervention_audit() -> Result<MemoryInterventionAudit, ActionError> {
    let initial = causal_memory_fixture();
    let intervention = scm_memory_intervention()?;
    let observed = &intervention.observed_state;
    let intervened = &intervention.intervened_state;
    let before_graph = initial.Gamma.clone();
    let before_obs_cells = initial.B_W.B_obs.charged_count();
    let semantic_record = intervention.semantic_record;
    let observed_live_bucket = BucketKey {
        port: 1,
        kind: CandidateKind::Fresh,
    };
    let observed_live_bucket_counts = observed.M.terminal_counts(observed_live_bucket);
    let intervened_live_bucket_counts = intervened.M.terminal_counts(observed_live_bucket);
    let mut reversed = intervention.observed_state.clone();
    let absorb_has_exact_inverse = reversed.reverse_last().is_ok() && reversed == initial;
    let differing_memory_records = observed
        .M
        .records
        .iter()
        .zip(&intervened.M.records)
        .filter(|(left, right)| left != right)
        .count();
    let observed_record = &observed.M.records[semantic_record];
    let intervened_record = &intervened.M.records[semantic_record];
    let exactly_one_memory_coordinate_replaced = observed.M.records.len()
        == intervened.M.records.len()
        && differing_memory_records == 1
        && observed_record.id == intervened_record.id
        && observed_record.candidate == intervened_record.candidate
        && observed_record.descriptor == intervened_record.descriptor
        && observed_record.state == intervened_record.state
        && observed_record.bucket == Some(intervention.observed_bucket)
        && intervened_record.bucket == Some(intervention.intervened_bucket);
    let observed_state_is_well_formed = observed.complete_state_is_well_formed();
    let intervention_is_off_manifold_scm =
        observed_state_is_well_formed && !intervened.complete_state_is_well_formed();
    let historical_absorb_record_is_unchanged = observed.O == intervened.O;
    let post_intervention_kernel = intervention.post_intervention_kernel(KernelSupport::PRIMARY)?;
    Ok(MemoryInterventionAudit {
        absorb_changes_no_graph_coordinate: observed.Gamma == before_graph,
        absorb_consumes_one_observation_cell: before_obs_cells
            == observed.B_W.B_obs.charged_count() + 1,
        absorb_writes_one_terminal_record: observed.M.records[semantic_record].state
            == SemanticState::Success,
        absorb_has_exact_inverse,
        observed_live_bucket_counts,
        intervened_live_bucket_counts,
        full_non_memory_coordinates_identical: observed.same_non_memory_coordinates(intervened),
        candidate_support_identical: observed.enabled_candidates()
            == intervened.enabled_candidates(),
        observed_state_is_well_formed,
        intervention_is_off_manifold_scm,
        exactly_one_memory_coordinate_replaced,
        historical_absorb_record_is_unchanged,
        post_intervention_kernel_defined: post_intervention_kernel.kernel_is_defined,
        every_post_intervention_transition_replays: post_intervention_kernel
            .every_transition_replays_from_intervened_state,
        intervention,
        post_intervention_kernel,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DrivenProtocolAudit {
    pub primary_fixture_is_markov_complete: bool,
    pub initial_build_cells: usize,
    pub initial_repair_cells: usize,
    pub repair_reserve_locked_during_build: bool,
    pub fresh_inverse_exact: bool,
    pub alias_inverse_exact: bool,
    pub absorb_inverse_exact: bool,
    pub perturb_inverse_exact: bool,
    pub construction_trace_inverse_exact: bool,
    pub alias_before_fresh_trace_inverse_exact: bool,
    pub repair_trace_inverse_exact: bool,
    pub construction_ledgers_close: bool,
    pub repair_ledgers_close: bool,
    pub perturb_complete_energy_closes: bool,
    pub perturb_unlocks_no_injected_repair_cells: bool,
    pub repair_restores_deleted_leaf: bool,
    pub repair_relocks_reserve: bool,
    pub repair_disabled_kernel_is_closed: bool,
    pub descriptor_order_path_law_invariant: bool,
    pub physical_branch_records_are_injective: bool,
    pub ledger_leak_is_rejected: bool,
    pub all_driven_channels_exercised: bool,
}

pub fn driven_protocol_audit() -> Result<DrivenProtocolAudit, ActionError> {
    let fixture = primary_fixture(2, false);
    let initial_build_cells = fixture.B_W.B_build.charged_count();
    let initial_repair_cells = fixture.B_W.B_repair.charged_count();

    let fresh_candidate = fixture
        .enabled_candidates()
        .into_iter()
        .find(|candidate| candidate.kind == CandidateKind::Fresh)
        .ok_or(ActionError::WrongCandidate)?;
    let mut fresh_state = fixture.clone();
    let fresh_receipt = fresh_state.fresh(fresh_candidate)?;
    let fresh_ledger_closes = fresh_receipt.ledger.working_first_law_closes()
        && fresh_receipt.ledger.internal_augmented_first_law_closes()
        && fresh_receipt.ledger.complete_first_law_closes();
    let fresh_inverse_exact = fresh_state.reverse_last().is_ok() && fresh_state == fixture;

    let alias_candidate = fixture
        .enabled_candidates()
        .into_iter()
        .find(|candidate| candidate.kind == CandidateKind::Alias)
        .ok_or(ActionError::WrongCandidate)?;
    let mut alias_state = fixture.clone();
    let alias_receipt = alias_state.alias_quarantine(alias_candidate)?;
    let alias_ledger_closes = alias_receipt.ledger.working_first_law_closes()
        && alias_receipt.ledger.internal_augmented_first_law_closes()
        && alias_receipt.ledger.complete_first_law_closes();
    let alias_inverse_exact = alias_state.reverse_last().is_ok() && alias_state == fixture;

    let causal = causal_memory_fixture();
    let mut absorbed = causal.clone();
    let absorb_receipt = absorbed.absorb()?;
    let absorb_ledger_closes = absorb_receipt.ledger.working_first_law_closes()
        && absorb_receipt.ledger.internal_augmented_first_law_closes()
        && absorb_receipt.ledger.complete_first_law_closes();
    let absorb_inverse_exact = absorbed.reverse_last().is_ok() && absorbed == causal;
    let mut observed_for_branches = causal.clone();
    observed_for_branches.absorb()?;
    let physical_branch_records_are_injective =
        physical_branch_record_audit(&observed_for_branches)?
            .records_injectively_identify_first_event;

    let construction = instantiate_one_support_trace(fixture.clone())?;
    let alias_before_fresh = instantiate_alias_before_fresh_support_trace(fixture.clone())?;
    let descriptor_order = descriptor_order_audit(2, false)?;
    let completed = construction.terminal_state.clone();
    let leaf = completed
        .Gamma
        .shell_words(3)
        .into_iter()
        .next()
        .ok_or(ActionError::LeafMissing)?;
    let mut perturbed = completed.clone();
    perturbed.schedule_leaf_perturbation(leaf.clone())?;
    let scheduled = perturbed.clone();
    let before_complete_energy = scheduled.augmented_energy_with_external_perturbation_cell();
    let before_internal_energy = scheduled.augmented_internal_energy();
    let repair_cells_before = perturbed.B_W.B_repair.charged_count();
    let perturb_receipt = perturbed.perturb_leaf(&leaf)?;
    let after_complete_energy = perturbed.augmented_energy_with_external_perturbation_cell();
    let after_internal_energy = perturbed.augmented_internal_energy();
    let perturb_complete_energy_closes = before_complete_energy == after_complete_energy
        && after_internal_energy == before_internal_energy + u128::from(completed.B_W.epsilon_W)
        && perturb_receipt.ledger.working_first_law_closes()
        && perturb_receipt.ledger.internal_augmented_first_law_closes()
        && perturb_receipt.ledger.complete_first_law_closes();
    let repair_cells_after = perturbed.B_W.B_repair.charged_count();
    let perturb_unlocks_no_injected_repair_cells =
        repair_cells_before == repair_cells_after && !perturbed.B_W.B_repair.locked;
    let mut perturb_reversed = perturbed.clone();
    let perturb_inverse_exact =
        perturb_reversed.reverse_last().is_ok() && perturb_reversed == scheduled;
    let repair_disabled_kernel_is_closed = perturbed.repair_disabled_kernel_is_closed();

    let repair = instantiate_one_support_trace(perturbed)?;
    let repair_restores_deleted_leaf = repair.terminal_state.Gamma.contains_executable(&leaf)
        && repair.terminal_state.Gamma.shell_count(3) == 8;
    let repair_relocks_reserve = repair.terminal_state.B_W.B_repair.locked;

    let mut leaking = LedgerRow::internal(
        DrivenChannel::Fresh,
        1,
        internal_resource_delta(DrivenChannel::Fresh, WorkLane::Build),
    );
    leaking.delta_work_store = 0;
    let ledger_leak_is_rejected = !leaking.internal_augmented_first_law_closes();
    let all_driven_channels_exercised = fresh_receipt.ledger.channel == DrivenChannel::Fresh
        && alias_receipt.ledger.channel == DrivenChannel::AliasQuarantine
        && absorb_receipt.ledger.channel == DrivenChannel::Absorb
        && perturb_receipt.ledger.channel == DrivenChannel::Perturb
        && construction.channel_count(DrivenChannel::Credit) > 0
        && construction.channel_count(DrivenChannel::Promote) > 0;
    Ok(DrivenProtocolAudit {
        primary_fixture_is_markov_complete: fixture.complete_state_is_well_formed(),
        initial_build_cells,
        initial_repair_cells,
        repair_reserve_locked_during_build: fixture.B_W.B_repair.locked,
        fresh_inverse_exact: fresh_inverse_exact && fresh_ledger_closes,
        alias_inverse_exact: alias_inverse_exact && alias_ledger_closes,
        absorb_inverse_exact: absorb_inverse_exact && absorb_ledger_closes,
        perturb_inverse_exact,
        construction_trace_inverse_exact: construction.every_forward_map_has_exact_local_inverse
            && construction.full_reverse_lifo_restores_initial,
        alias_before_fresh_trace_inverse_exact: alias_before_fresh
            .every_forward_map_has_exact_local_inverse
            && alias_before_fresh.full_reverse_lifo_restores_initial,
        repair_trace_inverse_exact: repair.every_forward_map_has_exact_local_inverse
            && repair.full_reverse_lifo_restores_initial,
        construction_ledgers_close: construction.every_working_ledger_closes
            && construction.every_augmented_ledger_closes
            && construction.every_complete_ledger_closes
            && construction.every_forward_energy_difference_matches_ledger
            && construction.every_forward_resource_difference_matches_ledger,
        repair_ledgers_close: repair.every_working_ledger_closes
            && repair.every_augmented_ledger_closes
            && repair.every_complete_ledger_closes
            && repair.every_forward_energy_difference_matches_ledger
            && repair.every_forward_resource_difference_matches_ledger,
        perturb_complete_energy_closes,
        perturb_unlocks_no_injected_repair_cells,
        repair_restores_deleted_leaf,
        repair_relocks_reserve,
        repair_disabled_kernel_is_closed,
        descriptor_order_path_law_invariant: descriptor_order.path_law_quotient_identical
            && descriptor_order.promoted_words_identical
            && descriptor_order.shell_counts_identical,
        physical_branch_records_are_injective,
        ledger_leak_is_rejected,
        all_driven_channels_exercised,
    })
}
