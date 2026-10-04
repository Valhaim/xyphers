use std::collections::{BTreeMap, BTreeSet};

use crate::exact::Ratio;
use crate::model;
use crate::thermal::ExactLog;

pub const ALPHABET_BASES: [u8; 3] = [2, 3, 4];
pub const ENERGY_QUANTUM: i128 = 1;
pub const PROMOTED_DEPTH: u8 = 3;
pub const REPAIR_RESERVE_CELLS: u64 = 4;
pub const REPAIR_OPERATION_RECORDS: u64 = 4;
pub const REPAIR_SEMANTIC_RECORDS: u64 = 2;
pub const PERTURB_EXTERNAL_CELLS: u64 = 1;
pub const CANDIDATE_KIND_COUNT: u64 = 2;
pub const CAUSAL_LIVE_SYMBOL: u8 = 1;
pub const CAUSAL_INACTIVE_SYMBOL: u8 = 2;
pub const CAUSAL_OBSERVATION_RECORD_ID: u64 = 0;
pub const CAUSAL_CANDIDATE_WORK_CELLS: u64 = 2;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CandidateKind {
    Fresh,
    Alias,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BucketKey {
    pub symbol: u8,
    pub kind: CandidateKind,
}

impl BucketKey {
    pub const fn new(symbol: u8, kind: CandidateKind) -> Self {
        Self { symbol, kind }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordState {
    Blank,
    Pending,
    Success,
    Failure,
}

impl RecordState {
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Success | Self::Failure)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticRecord {
    pub id: u64,
    pub bucket: BucketKey,
    pub state: RecordState,
}

impl SemanticRecord {
    pub const fn blank(id: u64, bucket: BucketKey) -> Self {
        Self {
            id,
            bucket,
            state: RecordState::Blank,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BucketStatistics {
    pub successes: u64,
    pub terminal_trials: u64,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MemoryLog {
    records: BTreeMap<u64, SemanticRecord>,
}

impl MemoryLog {
    pub fn insert_blank(&mut self, id: u64, bucket: BucketKey) {
        let previous = self.records.insert(id, SemanticRecord::blank(id, bucket));
        assert!(previous.is_none(), "physical record IDs cannot be reused");
    }

    pub fn mark_pending(&mut self, id: u64) {
        let record = self.record_mut(id);
        assert_eq!(record.state, RecordState::Blank);
        record.state = RecordState::Pending;
    }

    pub fn mark_failure(&mut self, id: u64) {
        let record = self.record_mut(id);
        assert_eq!(record.state, RecordState::Blank);
        record.state = RecordState::Failure;
    }

    pub fn credit_success(&mut self, id: u64) {
        let record = self.record_mut(id);
        assert_eq!(record.state, RecordState::Pending);
        record.state = RecordState::Success;
    }

    pub fn absorb_success(&mut self, id: u64) {
        let record = self.record_mut(id);
        assert_eq!(record.state, RecordState::Blank);
        record.state = RecordState::Success;
    }

    pub fn absorb_failure(&mut self, id: u64) {
        let record = self.record_mut(id);
        assert_eq!(record.state, RecordState::Blank);
        record.state = RecordState::Failure;
    }

    pub fn reassign_bucket_intervention(&mut self, id: u64, bucket: BucketKey) {
        let record = self.record_mut(id);
        assert!(
            record.state.is_terminal(),
            "the frozen do(M) intervention moves a terminal record"
        );
        record.bucket = bucket;
    }

    pub fn statistics(&self, bucket: BucketKey) -> BucketStatistics {
        let mut successes = 0_u64;
        let mut terminal_trials = 0_u64;
        for record in self
            .records
            .values()
            .filter(|record| record.bucket == bucket)
        {
            match record.state {
                RecordState::Success => {
                    successes = successes.checked_add(1).expect("success-count overflow");
                    terminal_trials = terminal_trials
                        .checked_add(1)
                        .expect("trial-count overflow");
                }
                RecordState::Failure => {
                    terminal_trials = terminal_trials
                        .checked_add(1)
                        .expect("trial-count overflow");
                }
                RecordState::Blank | RecordState::Pending => {}
            }
        }
        BucketStatistics {
            successes,
            terminal_trials,
        }
    }

    pub fn records(&self) -> &BTreeMap<u64, SemanticRecord> {
        &self.records
    }

    fn record_mut(&mut self, id: u64) -> &mut SemanticRecord {
        self.records
            .get_mut(&id)
            .expect("physical record ID must exist")
    }
}

pub fn posterior_predictive(statistics: BucketStatistics) -> Ratio {
    assert!(statistics.successes <= statistics.terminal_trials);
    Ratio::new(
        statistics
            .successes
            .checked_add(1)
            .expect("posterior numerator overflow") as i128,
        statistics
            .terminal_trials
            .checked_add(2)
            .expect("posterior denominator overflow") as i128,
    )
}

pub fn opal_phi(ruby: Ratio) -> Ratio {
    assert!(ruby > Ratio::ZERO && ruby < Ratio::ONE);
    let one_plus_ruby = Ratio::ONE + ruby;
    one_plus_ruby * one_plus_ruby * (Ratio::integer(3) - ruby) / Ratio::integer(8)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PhiBoundsReport {
    pub ruby: Ratio,
    pub phi: Ratio,
    pub lower_gap: Ratio,
    pub lower_factorization: Ratio,
    pub upper_gap: Ratio,
    pub upper_factorization: Ratio,
    pub exact_identities_hold: bool,
    pub strictly_between_three_eighths_and_one: bool,
}

pub fn phi_bounds_report(ruby: Ratio) -> PhiBoundsReport {
    assert!(ruby > Ratio::ZERO && ruby < Ratio::ONE);
    let phi = opal_phi(ruby);
    let lower = Ratio::new(3, 8);
    let lower_gap = phi - lower;
    let lower_factorization = ruby * (Ratio::integer(5) + ruby - ruby * ruby) / Ratio::integer(8);
    let upper_gap = Ratio::ONE - phi;
    let upper_factorization =
        (Ratio::ONE - ruby) * (Ratio::integer(5) - ruby * ruby) / Ratio::integer(8);
    PhiBoundsReport {
        ruby,
        phi,
        lower_gap,
        lower_factorization,
        upper_gap,
        upper_factorization,
        exact_identities_hold: lower_gap == lower_factorization && upper_gap == upper_factorization,
        strictly_between_three_eighths_and_one: lower_gap.is_positive() && upper_gap.is_positive(),
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PrefixWord(Vec<u8>);

impl PrefixWord {
    pub const fn root() -> Self {
        Self(Vec::new())
    }

    pub fn from_symbols(symbols: Vec<u8>, base: u8) -> Self {
        assert!(base >= 2);
        assert!(symbols.iter().all(|&symbol| (1..=base).contains(&symbol)));
        Self(symbols)
    }

    pub fn child(&self, symbol: u8, base: u8) -> Self {
        assert!((1..=base).contains(&symbol));
        let mut symbols = self.0.clone();
        symbols.push(symbol);
        Self(symbols)
    }

    pub fn parent(&self) -> Option<Self> {
        let mut symbols = self.0.clone();
        symbols.pop()?;
        Some(Self(symbols))
    }

    pub fn depth(&self) -> usize {
        self.0.len()
    }

    pub fn symbols(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DescriptorBinding {
    pub word: PrefixWord,
    pub descriptor_id: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrefixArchitecture {
    pub base: u8,
    promoted: BTreeMap<PrefixWord, u64>,
    workspace: BTreeMap<PrefixWord, u64>,
    free_descriptors: BTreeSet<u64>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OpenPort {
    pub parent: PrefixWord,
    pub symbol: u8,
}

impl PrefixArchitecture {
    pub fn root_with_inventory(base: u8, descriptor_count: u64) -> Self {
        assert!(base >= 2);
        let mut promoted = BTreeMap::new();
        promoted.insert(PrefixWord::root(), 0);
        Self {
            base,
            promoted,
            workspace: BTreeMap::new(),
            free_descriptors: (1..=descriptor_count).collect(),
        }
    }

    pub fn enumerated_reference(base: u8, depth: u8) -> Self {
        assert!(base >= 2);
        let mut promoted = BTreeMap::new();
        promoted.insert(PrefixWord::root(), 0);
        let mut next_descriptor = 1_u64;
        for shell in 1..=depth {
            let parents: Vec<_> = promoted
                .keys()
                .filter(|word| word.depth() == usize::from(shell - 1))
                .cloned()
                .collect();
            for parent in parents {
                for symbol in 1..=base {
                    let child = parent.child(symbol, base);
                    let previous = promoted.insert(child, next_descriptor);
                    assert!(previous.is_none(), "canonical words must be injective");
                    next_descriptor = next_descriptor
                        .checked_add(1)
                        .expect("descriptor-ID overflow");
                }
            }
        }
        Self {
            base,
            promoted,
            workspace: BTreeMap::new(),
            free_descriptors: BTreeSet::new(),
        }
    }

    pub fn off_shell_control(base: u8) -> Self {
        let fixture = prefix_fixture(base);
        let mut architecture = Self::enumerated_reference(base, PROMOTED_DEPTH);
        let parent = architecture
            .promoted
            .keys()
            .find(|word| word.depth() == usize::from(PROMOTED_DEPTH))
            .cloned()
            .expect("depth-three parent must exist");
        let word = parent.child(1, base);
        let previous = architecture
            .workspace
            .insert(word, fixture.primary_descriptors + 1);
        assert!(previous.is_none());
        architecture
    }

    pub fn promoted(&self) -> &BTreeMap<PrefixWord, u64> {
        &self.promoted
    }

    pub fn workspace(&self) -> &BTreeMap<PrefixWord, u64> {
        &self.workspace
    }

    pub fn free_descriptors(&self) -> &BTreeSet<u64> {
        &self.free_descriptors
    }

    pub fn deepest_promoted_depth(&self) -> usize {
        self.promoted
            .keys()
            .map(PrefixWord::depth)
            .max()
            .expect("the root is always promoted")
    }

    pub fn open_ports(&self) -> Vec<OpenPort> {
        let deepest = self.deepest_promoted_depth();
        let mut ports = Vec::new();
        for parent in self.promoted.keys().filter(|word| word.depth() == deepest) {
            for symbol in 1..=self.base {
                let child = parent.child(symbol, self.base);
                if !self.promoted.contains_key(&child) && !self.workspace.contains_key(&child) {
                    ports.push(OpenPort {
                        parent: parent.clone(),
                        symbol,
                    });
                }
            }
        }
        ports
    }

    /// Observer-side graph rewrite only; the authoritative action also updates Z's ledgers.
    pub fn bind_fresh_structural_reference(&mut self, port: &OpenPort) -> DescriptorBinding {
        assert_eq!(port.parent.depth(), self.deepest_promoted_depth());
        assert!(self.promoted.contains_key(&port.parent));
        let word = port.parent.child(port.symbol, self.base);
        assert!(!self.promoted.contains_key(&word));
        assert!(!self.workspace.contains_key(&word));
        let descriptor_id = self
            .free_descriptors
            .pop_first()
            .expect("FRESH requires one free descriptor");
        let previous = self.workspace.insert(word.clone(), descriptor_id);
        assert!(previous.is_none());
        DescriptorBinding {
            word,
            descriptor_id,
        }
    }

    pub fn frontier_is_locally_complete(&self) -> bool {
        self.open_ports().is_empty() && !self.workspace.is_empty()
    }

    /// Observer-side closure helper; authoritative PROMOTE additionally requires CREDIT.
    pub fn promote_structural_reference_if_complete(&mut self) -> bool {
        if !self.frontier_is_locally_complete() {
            return false;
        }
        let next_depth = self
            .deepest_promoted_depth()
            .checked_add(1)
            .expect("prefix depth overflow");
        assert!(
            self.workspace.keys().all(|word| word.depth() == next_depth),
            "only the current detached frontier may be promoted"
        );
        let frontier = std::mem::take(&mut self.workspace);
        for (word, descriptor_id) in frontier {
            let previous = self.promoted.insert(word, descriptor_id);
            assert!(previous.is_none());
        }
        true
    }

    pub fn shell_counts(&self, through_depth: u8) -> Vec<u64> {
        (0..=through_depth)
            .map(|depth| {
                u64::try_from(
                    self.promoted
                        .keys()
                        .filter(|word| word.depth() == usize::from(depth))
                        .count(),
                )
                .expect("shell count does not fit u64")
            })
            .collect()
    }

    pub fn workspace_shell_count(&self, depth: u8) -> u64 {
        u64::try_from(
            self.workspace
                .keys()
                .filter(|word| word.depth() == usize::from(depth))
                .count(),
        )
        .expect("workspace count does not fit u64")
    }

    pub fn injective_descriptor_encoding(&self) -> bool {
        let descriptor_count = self
            .promoted
            .values()
            .chain(self.workspace.values())
            .copied()
            .collect::<BTreeSet<_>>()
            .len();
        descriptor_count == self.promoted.len() + self.workspace.len()
    }

    pub fn unique_parentage(&self) -> bool {
        self.promoted.keys().all(|word| {
            word.depth() == 0
                || word
                    .parent()
                    .is_some_and(|parent| self.promoted.contains_key(&parent))
        })
    }

    pub fn locally_complete_through(&self, depth: u8) -> bool {
        self.promoted
            .keys()
            .filter(|word| word.depth() < usize::from(depth))
            .all(|word| {
                (1..=self.base)
                    .all(|symbol| self.promoted.contains_key(&word.child(symbol, self.base)))
            })
    }

    pub fn detach_depth_three_leaf(&mut self, word: &PrefixWord) -> DescriptorBinding {
        assert_eq!(word.depth(), usize::from(PROMOTED_DEPTH));
        assert!(
            !self.promoted.keys().any(|candidate| {
                candidate.depth() > word.depth() && candidate.symbols().starts_with(word.symbols())
            }),
            "the frozen repair family deletes leaves without descendants"
        );
        let descriptor_id = self
            .promoted
            .remove(word)
            .expect("selected leaf must be promoted");
        let inserted = self.free_descriptors.insert(descriptor_id);
        assert!(inserted);
        DescriptorBinding {
            word: word.clone(),
            descriptor_id,
        }
    }

    pub fn restore_depth_three_leaf(&mut self, detached: DescriptorBinding) {
        assert_eq!(detached.word.depth(), usize::from(PROMOTED_DEPTH));
        assert!(self.free_descriptors.remove(&detached.descriptor_id));
        let previous = self.promoted.insert(detached.word, detached.descriptor_id);
        assert!(previous.is_none());
    }

    pub fn reverse_descriptor_ids(&self) -> Self {
        let maximum = self
            .promoted
            .values()
            .chain(self.workspace.values())
            .chain(self.free_descriptors.iter())
            .copied()
            .max()
            .unwrap_or(0);
        let reverse = |descriptor: u64| {
            if descriptor == 0 {
                0
            } else {
                maximum
                    .checked_add(1)
                    .and_then(|upper| upper.checked_sub(descriptor))
                    .expect("descriptor permutation overflow")
            }
        };
        Self {
            base: self.base,
            promoted: self
                .promoted
                .iter()
                .map(|(word, &descriptor)| (word.clone(), reverse(descriptor)))
                .collect(),
            workspace: self
                .workspace
                .iter()
                .map(|(word, &descriptor)| (word.clone(), reverse(descriptor)))
                .collect(),
            free_descriptors: self
                .free_descriptors
                .iter()
                .map(|&descriptor| reverse(descriptor))
                .collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrefixFixture {
    pub base: u8,
    pub energy_quantum: i128,
    pub promoted_depth: u8,
    pub expected_shell_counts: Vec<u64>,
    pub primary_descriptors: u64,
    pub primary_semantic_records: u64,
    pub primary_attempt_bits: u64,
    pub primary_build_work_cells: u64,
    pub primary_operation_records: u64,
    pub repair_work_cells: u64,
    pub repair_operation_records: u64,
    pub repair_semantic_records: u64,
    pub repair_attempt_bits: u64,
    pub total_internal_work_cells: u64,
    pub off_shell_descriptors: u64,
    pub off_shell_semantic_records: u64,
    pub off_shell_attempt_bits: u64,
    pub off_shell_build_work_cells: u64,
    pub off_shell_operation_records: u64,
}

pub fn prefix_fixture(base: u8) -> PrefixFixture {
    assert!(ALPHABET_BASES.contains(&base));
    let b = u64::from(base);
    let b2 = b.checked_mul(b).expect("prefix count overflow");
    let b3 = b2.checked_mul(b).expect("prefix count overflow");
    let primary_descriptors = b
        .checked_add(b2)
        .and_then(|value| value.checked_add(b3))
        .expect("prefix count overflow");
    let primary_build_work_cells = primary_descriptors
        .checked_mul(3)
        .and_then(|value| value.checked_add(u64::from(PROMOTED_DEPTH)))
        .expect("work-cell capacity overflow");
    PrefixFixture {
        base,
        energy_quantum: ENERGY_QUANTUM,
        promoted_depth: PROMOTED_DEPTH,
        expected_shell_counts: vec![1, b, b2, b3],
        primary_descriptors,
        primary_semantic_records: primary_descriptors
            .checked_mul(CANDIDATE_KIND_COUNT)
            .expect("semantic-record capacity overflow"),
        primary_attempt_bits: primary_descriptors
            .checked_mul(CANDIDATE_KIND_COUNT)
            .expect("attempt-bit capacity overflow"),
        primary_build_work_cells,
        primary_operation_records: primary_build_work_cells,
        repair_work_cells: REPAIR_RESERVE_CELLS,
        repair_operation_records: REPAIR_OPERATION_RECORDS,
        repair_semantic_records: REPAIR_SEMANTIC_RECORDS,
        repair_attempt_bits: REPAIR_SEMANTIC_RECORDS,
        total_internal_work_cells: primary_build_work_cells
            .checked_add(REPAIR_RESERVE_CELLS)
            .expect("total work-cell capacity overflow"),
        off_shell_descriptors: primary_descriptors
            .checked_add(1)
            .expect("off-shell descriptor capacity overflow"),
        off_shell_semantic_records: primary_descriptors
            .checked_mul(CANDIDATE_KIND_COUNT)
            .and_then(|value| value.checked_add(CANDIDATE_KIND_COUNT))
            .expect("off-shell semantic-record capacity overflow"),
        off_shell_attempt_bits: primary_descriptors
            .checked_mul(CANDIDATE_KIND_COUNT)
            .and_then(|value| value.checked_add(CANDIDATE_KIND_COUNT))
            .expect("off-shell attempt-bit capacity overflow"),
        off_shell_build_work_cells: primary_build_work_cells
            .checked_add(CANDIDATE_KIND_COUNT)
            .expect("off-shell work-cell capacity overflow"),
        off_shell_operation_records: primary_build_work_cells
            .checked_add(CANDIDATE_KIND_COUNT)
            .expect("off-shell operation-record capacity overflow"),
    }
}

pub fn primary_initial_architecture(base: u8) -> PrefixArchitecture {
    let fixture = prefix_fixture(base);
    PrefixArchitecture::root_with_inventory(base, fixture.primary_descriptors)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InitialPrefixReport {
    pub base: u8,
    pub promoted_counts: Vec<u64>,
    pub workspace_is_empty: bool,
    pub free_descriptor_count: u64,
    pub only_root_is_executable: bool,
    pub claimed_interval_has_too_few_nonempty_shells: bool,
}

pub fn initial_prefix_report(base: u8) -> InitialPrefixReport {
    let architecture = primary_initial_architecture(base);
    let promoted_counts = architecture.shell_counts(PROMOTED_DEPTH);
    let nonempty_shells = promoted_counts.iter().filter(|&&count| count > 0).count();
    InitialPrefixReport {
        base,
        workspace_is_empty: architecture.workspace().is_empty(),
        free_descriptor_count: u64::try_from(architecture.free_descriptors().len())
            .expect("free-descriptor count does not fit u64"),
        only_root_is_executable: promoted_counts.first() == Some(&1)
            && promoted_counts.iter().skip(1).all(|&count| count == 0),
        claimed_interval_has_too_few_nonempty_shells: nonempty_shells < 2,
        promoted_counts,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrefixCountReport {
    pub base: u8,
    pub actual_shell_counts: Vec<u64>,
    pub expected_shell_counts: Vec<u64>,
    pub actual_enumeration_matches_prediction: bool,
    pub shell_recursion_holds: bool,
    pub canonical_words_are_injective: bool,
    pub descriptor_encoding_is_injective: bool,
    pub unique_parentage_holds: bool,
    pub local_port_completeness_holds: bool,
}

pub fn prefix_count_report(base: u8) -> PrefixCountReport {
    let architecture = PrefixArchitecture::enumerated_reference(base, PROMOTED_DEPTH);
    classify_prefix_architecture(&architecture)
}

pub fn classify_prefix_architecture(architecture: &PrefixArchitecture) -> PrefixCountReport {
    let base = architecture.base;
    let expected_shell_counts = prefix_fixture(base).expected_shell_counts;
    let actual_shell_counts = architecture.shell_counts(PROMOTED_DEPTH);
    let shell_recursion_holds = actual_shell_counts
        .windows(2)
        .all(|window| window[1] == window[0] * u64::from(base));
    let canonical_word_count = architecture
        .promoted
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        .len();
    PrefixCountReport {
        base,
        actual_enumeration_matches_prediction: actual_shell_counts == expected_shell_counts,
        shell_recursion_holds,
        canonical_words_are_injective: canonical_word_count == architecture.promoted.len(),
        descriptor_encoding_is_injective: architecture.injective_descriptor_encoding(),
        unique_parentage_holds: architecture.unique_parentage(),
        local_port_completeness_holds: architecture.locally_complete_through(PROMOTED_DEPTH),
        actual_shell_counts,
        expected_shell_counts,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResourceReport {
    pub fixture: PrefixFixture,
    pub worst_candidate_attempt_operations: u64,
    pub success_credit_operations: u64,
    pub promote_operations: u64,
    pub derived_build_operation_bound: u64,
    pub build_capacity_covers_worst_ordering: bool,
    pub repair_reserve_locked_during_build: bool,
    pub total_internal_capacity_is_build_plus_repair: bool,
    pub off_shell_extra_operations: u64,
    pub off_shell_capacity_is_primary_plus_two: bool,
}

pub fn resource_report(base: u8) -> ResourceReport {
    let fixture = prefix_fixture(base);
    let worst_candidate_attempt_operations = fixture
        .primary_descriptors
        .checked_mul(CANDIDATE_KIND_COUNT)
        .expect("candidate-operation bound overflow");
    let success_credit_operations = fixture.primary_descriptors;
    let promote_operations = u64::from(PROMOTED_DEPTH);
    let derived_build_operation_bound = worst_candidate_attempt_operations
        .checked_add(success_credit_operations)
        .and_then(|value| value.checked_add(promote_operations))
        .expect("build-operation bound overflow");
    ResourceReport {
        build_capacity_covers_worst_ordering: derived_build_operation_bound
            == fixture.primary_build_work_cells
            && derived_build_operation_bound == fixture.primary_operation_records,
        repair_reserve_locked_during_build: fixture.repair_work_cells == REPAIR_RESERVE_CELLS,
        total_internal_capacity_is_build_plus_repair: fixture.total_internal_work_cells
            == fixture.primary_build_work_cells + fixture.repair_work_cells,
        off_shell_extra_operations: CANDIDATE_KIND_COUNT,
        off_shell_capacity_is_primary_plus_two: fixture.off_shell_build_work_cells
            == fixture.primary_build_work_cells + CANDIDATE_KIND_COUNT
            && fixture.off_shell_operation_records
                == fixture.primary_operation_records + CANDIDATE_KIND_COUNT,
        fixture,
        worst_candidate_attempt_operations,
        success_credit_operations,
        promote_operations,
        derived_build_operation_bound,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OffShellReport {
    pub base: u8,
    pub promoted_counts: Vec<u64>,
    pub expected_core_counts: Vec<u64>,
    pub workspace_depth_four_count: u64,
    pub promoted_depth_four_count: u64,
    pub workspace_is_excluded_from_multiplicity: bool,
    pub promoted_core_unchanged: bool,
    pub descriptor_permutation_preserves_counts: bool,
    pub descriptor_permutation_preserves_classification: bool,
}

pub fn off_shell_report(base: u8) -> OffShellReport {
    let fixture = prefix_fixture(base);
    let architecture = PrefixArchitecture::off_shell_control(base);
    let promoted_counts = architecture.shell_counts(PROMOTED_DEPTH);
    let promoted_depth_four_count = architecture
        .shell_counts(PROMOTED_DEPTH + 1)
        .last()
        .copied()
        .expect("depth-four shell count must exist");
    let workspace_depth_four_count = architecture.workspace_shell_count(PROMOTED_DEPTH + 1);
    let permuted = architecture.reverse_descriptor_ids();
    let descriptor_permutation_preserves_counts = permuted.shell_counts(PROMOTED_DEPTH)
        == promoted_counts
        && permuted.workspace_shell_count(PROMOTED_DEPTH + 1) == workspace_depth_four_count;
    let descriptor_permutation_preserves_classification =
        classify_prefix_architecture(&permuted) == classify_prefix_architecture(&architecture);
    OffShellReport {
        base,
        promoted_core_unchanged: promoted_counts == fixture.expected_shell_counts,
        workspace_is_excluded_from_multiplicity: promoted_depth_four_count == 0
            && workspace_depth_four_count == 1,
        promoted_counts,
        expected_core_counts: fixture.expected_shell_counts,
        workspace_depth_four_count,
        promoted_depth_four_count,
        descriptor_permutation_preserves_counts,
        descriptor_permutation_preserves_classification,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalCursorState {
    pub open_ports: u64,
    pub alias_untried: bool,
}

impl LocalCursorState {
    pub fn at_open_port(open_ports: u64) -> Self {
        assert!(open_ports > 0);
        Self {
            open_ports,
            alias_untried: true,
        }
    }

    pub const fn terminal() -> Self {
        Self {
            open_ports: 0,
            alias_untried: false,
        }
    }

    pub fn rank_value(self) -> u64 {
        self.open_ports
            .checked_mul(2)
            .and_then(|value| value.checked_add(if self.alias_untried { 1 } else { 0 }))
            .expect("cursor-rank overflow")
    }

    pub fn after_alias(self) -> Self {
        assert!(self.open_ports > 0 && self.alias_untried);
        Self {
            open_ports: self.open_ports,
            alias_untried: false,
        }
    }

    pub fn after_fresh(self) -> Self {
        assert!(self.open_ports > 0);
        if self.open_ports == 1 {
            Self::terminal()
        } else {
            Self {
                open_ports: self.open_ports - 1,
                alias_untried: true,
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ConstructionPhase {
    Promote,
    Credit,
    Build,
}

impl ConstructionPhase {
    pub const fn rank_coordinate(self) -> u8 {
        match self {
            Self::Promote => 0,
            Self::Credit => 1,
            Self::Build => 2,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ConstructionRank {
    pub remaining_promotions: u8,
    pub phase_coordinate: u8,
    pub inner: u64,
}

impl ConstructionRank {
    pub fn build(remaining_promotions: u8, cursor: LocalCursorState) -> Self {
        Self {
            remaining_promotions,
            phase_coordinate: ConstructionPhase::Build.rank_coordinate(),
            inner: cursor.rank_value(),
        }
    }

    pub fn credit(remaining_promotions: u8, pending_records: u64) -> Self {
        Self {
            remaining_promotions,
            phase_coordinate: ConstructionPhase::Credit.rank_coordinate(),
            inner: pending_records,
        }
    }

    pub fn promote(remaining_promotions: u8) -> Self {
        Self {
            remaining_promotions,
            phase_coordinate: ConstructionPhase::Promote.rank_coordinate(),
            inner: 1,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreationRankReport {
    pub base: u8,
    pub total_ports: u64,
    pub maximum_frontier_ports: u64,
    pub candidate_kind_histories: u128,
    pub alias_minimum_rank_drop: u64,
    pub fresh_after_alias_minimum_rank_drop: u64,
    pub fresh_first_minimum_rank_drop: u64,
    pub every_candidate_transition_decreases_local_rank: bool,
    pub build_to_credit_decreases_lexicographic_rank: bool,
    pub every_credit_decreases_lexicographic_rank: bool,
    pub credit_to_promote_decreases_lexicographic_rank: bool,
    pub promote_to_next_build_decreases_lexicographic_rank: bool,
    pub every_feasible_hazard_exceeds_three_eighths: bool,
    pub resources_cover_worst_ordering: bool,
    pub finite_rank_certificate_closes_creation: bool,
}

pub fn creation_rank_report(base: u8) -> CreationRankReport {
    let fixture = prefix_fixture(base);
    let maximum_frontier_ports = integer_power_u64(u64::from(base), PROMOTED_DEPTH.into());
    let mut alias_minimum_rank_drop = u64::MAX;
    let mut fresh_after_alias_minimum_rank_drop = u64::MAX;
    let mut fresh_first_minimum_rank_drop = u64::MAX;
    let mut every_candidate_transition_decreases_local_rank = true;
    for open_ports in 1..=maximum_frontier_ports {
        let before = LocalCursorState::at_open_port(open_ports);
        let after_alias = before.after_alias();
        let alias_drop = before.rank_value() - after_alias.rank_value();
        alias_minimum_rank_drop = alias_minimum_rank_drop.min(alias_drop);
        every_candidate_transition_decreases_local_rank &= alias_drop > 0;

        let after_alias_then_fresh = after_alias.after_fresh();
        let fresh_after_alias_drop = after_alias.rank_value() - after_alias_then_fresh.rank_value();
        fresh_after_alias_minimum_rank_drop =
            fresh_after_alias_minimum_rank_drop.min(fresh_after_alias_drop);
        every_candidate_transition_decreases_local_rank &= fresh_after_alias_drop > 0;

        let after_fresh_first = before.after_fresh();
        let fresh_first_drop = before.rank_value() - after_fresh_first.rank_value();
        fresh_first_minimum_rank_drop = fresh_first_minimum_rank_drop.min(fresh_first_drop);
        every_candidate_transition_decreases_local_rank &= fresh_first_drop >= 2;
    }

    let pending = maximum_frontier_ports;
    let build_to_credit_decreases_lexicographic_rank =
        ConstructionRank::credit(PROMOTED_DEPTH, pending)
            < ConstructionRank::build(PROMOTED_DEPTH, LocalCursorState::terminal());
    let every_credit_decreases_lexicographic_rank = (1..=pending).all(|count| {
        ConstructionRank::credit(PROMOTED_DEPTH, count - 1)
            < ConstructionRank::credit(PROMOTED_DEPTH, count)
    });
    let credit_to_promote_decreases_lexicographic_rank =
        ConstructionRank::promote(PROMOTED_DEPTH) < ConstructionRank::credit(PROMOTED_DEPTH, 0);
    let promote_to_next_build_decreases_lexicographic_rank =
        ConstructionRank::build(
            PROMOTED_DEPTH - 1,
            LocalCursorState::at_open_port(u64::from(base)),
        ) < ConstructionRank::promote(PROMOTED_DEPTH);

    let maximum_trials_per_bucket = 1_u64
        .checked_add(u64::from(base))
        .and_then(|value| value.checked_add(u64::from(base).pow(2)))
        .expect("bucket trial bound overflow");
    let every_feasible_hazard_exceeds_three_eighths =
        all_feasible_phi_bounds(maximum_trials_per_bucket)
            .iter()
            .all(|report| report.strictly_between_three_eighths_and_one);
    let resources_cover_worst_ordering = resource_report(base).build_capacity_covers_worst_ordering;
    let candidate_kind_histories = 1_u128
        .checked_shl(
            u32::try_from(fixture.primary_descriptors).expect("history exponent does not fit u32"),
        )
        .expect("candidate-kind history count does not fit u128");
    let finite_rank_certificate_closes_creation = every_candidate_transition_decreases_local_rank
        && build_to_credit_decreases_lexicographic_rank
        && every_credit_decreases_lexicographic_rank
        && credit_to_promote_decreases_lexicographic_rank
        && promote_to_next_build_decreases_lexicographic_rank
        && every_feasible_hazard_exceeds_three_eighths
        && resources_cover_worst_ordering;

    CreationRankReport {
        base,
        total_ports: fixture.primary_descriptors,
        maximum_frontier_ports,
        candidate_kind_histories,
        alias_minimum_rank_drop,
        fresh_after_alias_minimum_rank_drop,
        fresh_first_minimum_rank_drop,
        every_candidate_transition_decreases_local_rank,
        build_to_credit_decreases_lexicographic_rank,
        every_credit_decreases_lexicographic_rank,
        credit_to_promote_decreases_lexicographic_rank,
        promote_to_next_build_decreases_lexicographic_rank,
        every_feasible_hazard_exceeds_three_eighths,
        resources_cover_worst_ordering,
        finite_rank_certificate_closes_creation,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OnePortContext {
    pub live_symbol: u8,
    pub port_open: bool,
    pub free_descriptor_id: u64,
    pub charged_candidate_work_cells: u64,
    pub observation_payload_consumed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnePortLaw {
    pub fresh_ruby: Ratio,
    pub alias_ruby: Ratio,
    pub fresh_hazard: Ratio,
    pub alias_hazard: Ratio,
    pub fresh_before_alias_probability: Ratio,
    pub expected_candidate_attempts: Ratio,
    pub expected_work_cells: Ratio,
}

pub fn one_port_law(memory: &MemoryLog, live_symbol: u8) -> OnePortLaw {
    let fresh_ruby =
        posterior_predictive(memory.statistics(BucketKey::new(live_symbol, CandidateKind::Fresh)));
    let alias_ruby =
        posterior_predictive(memory.statistics(BucketKey::new(live_symbol, CandidateKind::Alias)));
    let fresh_hazard = opal_phi(fresh_ruby);
    let alias_hazard = opal_phi(alias_ruby);
    let fresh_before_alias_probability = fresh_hazard / (fresh_hazard + alias_hazard);
    let expected_candidate_attempts =
        Ratio::integer(i128::from(CANDIDATE_KIND_COUNT)) - fresh_before_alias_probability;
    OnePortLaw {
        fresh_ruby,
        alias_ruby,
        fresh_hazard,
        alias_hazard,
        fresh_before_alias_probability,
        expected_candidate_attempts,
        expected_work_cells: expected_candidate_attempts,
    }
}

pub fn enabled_candidate_hazards(
    memory: &MemoryLog,
    live_symbol: u8,
    cursor: LocalCursorState,
    free_descriptor_available: bool,
    charged_work_cells: u64,
) -> BTreeMap<CandidateKind, Ratio> {
    if cursor.open_ports == 0 || !free_descriptor_available || charged_work_cells == 0 {
        return BTreeMap::new();
    }
    let law = one_port_law(memory, live_symbol);
    let mut hazards = BTreeMap::new();
    hazards.insert(CandidateKind::Fresh, law.fresh_hazard);
    if cursor.alias_untried {
        hazards.insert(CandidateKind::Alias, law.alias_hazard);
    }
    hazards
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateTransition {
    pub kind: CandidateKind,
    pub cursor_before: LocalCursorState,
    pub cursor_after: LocalCursorState,
    pub record_before: RecordState,
    pub record_after: RecordState,
    pub port_closes: bool,
    pub descriptors_debited: u64,
    pub work_cells_debited: u64,
    pub rank_strictly_decreases: bool,
}

pub fn candidate_transition(cursor: LocalCursorState, kind: CandidateKind) -> CandidateTransition {
    assert!(cursor.open_ports > 0);
    let cursor_after = match kind {
        CandidateKind::Fresh => cursor.after_fresh(),
        CandidateKind::Alias => cursor.after_alias(),
    };
    CandidateTransition {
        kind,
        cursor_before: cursor,
        cursor_after,
        record_before: RecordState::Blank,
        record_after: match kind {
            CandidateKind::Fresh => RecordState::Pending,
            CandidateKind::Alias => RecordState::Failure,
        },
        port_closes: kind == CandidateKind::Fresh,
        descriptors_debited: if kind == CandidateKind::Fresh { 1 } else { 0 },
        work_cells_debited: 1,
        rank_strictly_decreases: cursor_after.rank_value() < cursor.rank_value(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdaptiveLoopReport {
    pub diamond_opportunity: Ratio,
    pub blank_ruby: Ratio,
    pub blank_phi: Ratio,
    pub charged_hazards: BTreeMap<CandidateKind, Ratio>,
    pub discharged_hazards: BTreeMap<CandidateKind, Ratio>,
    pub fresh_transition: CandidateTransition,
    pub alias_transition: CandidateTransition,
    pub graph_state_changes_candidate_set: bool,
    pub thermo_work_cell_causally_enables_action: bool,
    pub fresh_changes_binding_and_records_pending: bool,
    pub alias_quarantines_as_terminal_failure: bool,
}

pub fn adaptive_loop_report() -> AdaptiveLoopReport {
    let memory = MemoryLog::default();
    let cursor = LocalCursorState::at_open_port(1);
    let charged_hazards = enabled_candidate_hazards(&memory, CAUSAL_LIVE_SYMBOL, cursor, true, 1);
    let discharged_hazards =
        enabled_candidate_hazards(&memory, CAUSAL_LIVE_SYMBOL, cursor, true, 0);
    let closed_hazards = enabled_candidate_hazards(
        &memory,
        CAUSAL_LIVE_SYMBOL,
        LocalCursorState::terminal(),
        true,
        1,
    );
    let fresh_transition = candidate_transition(cursor, CandidateKind::Fresh);
    let alias_transition = candidate_transition(cursor, CandidateKind::Alias);
    AdaptiveLoopReport {
        diamond_opportunity: Ratio::ONE,
        blank_ruby: posterior_predictive(BucketStatistics {
            successes: 0,
            terminal_trials: 0,
        }),
        blank_phi: opal_phi(Ratio::new(1, 2)),
        graph_state_changes_candidate_set: !charged_hazards.is_empty() && closed_hazards.is_empty(),
        thermo_work_cell_causally_enables_action: !charged_hazards.is_empty()
            && discharged_hazards.is_empty(),
        fresh_changes_binding_and_records_pending: fresh_transition.port_closes
            && fresh_transition.descriptors_debited == 1
            && fresh_transition.record_after == RecordState::Pending,
        alias_quarantines_as_terminal_failure: !alias_transition.port_closes
            && alias_transition.descriptors_debited == 0
            && alias_transition.record_after == RecordState::Failure,
        charged_hazards,
        discharged_hazards,
        fresh_transition,
        alias_transition,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryCausalityReport {
    pub context_before_intervention: OnePortContext,
    pub context_after_intervention: OnePortContext,
    pub observed_memory: MemoryLog,
    pub intervened_memory: MemoryLog,
    pub observed_law: OnePortLaw,
    pub intervened_law: OnePortLaw,
    pub absorption_is_reachable_blank_to_success: bool,
    pub listed_non_memory_coordinates_are_identical: bool,
    pub next_action_law_changes: bool,
    pub expected_work_changes: bool,
    pub frozen_observed_values_hold: bool,
    pub frozen_intervened_values_hold: bool,
}

pub fn memory_causality_report() -> MemoryCausalityReport {
    let live_symbol = CAUSAL_LIVE_SYMBOL;
    let inactive_symbol = CAUSAL_INACTIVE_SYMBOL;
    let observation_record_id = CAUSAL_OBSERVATION_RECORD_ID;
    let context = OnePortContext {
        live_symbol,
        port_open: true,
        free_descriptor_id: 1,
        charged_candidate_work_cells: CAUSAL_CANDIDATE_WORK_CELLS,
        observation_payload_consumed: true,
    };
    let intervened_context = context;
    let mut observed_memory = MemoryLog::default();
    observed_memory.insert_blank(
        observation_record_id,
        BucketKey::new(live_symbol, CandidateKind::Fresh),
    );
    let was_blank = observed_memory
        .records()
        .get(&observation_record_id)
        .is_some_and(|record| record.state == RecordState::Blank);
    observed_memory.absorb_success(observation_record_id);
    let became_success = observed_memory
        .records()
        .get(&observation_record_id)
        .is_some_and(|record| record.state == RecordState::Success);
    let mut intervened_memory = observed_memory.clone();
    intervened_memory.reassign_bucket_intervention(
        observation_record_id,
        BucketKey::new(inactive_symbol, CandidateKind::Fresh),
    );
    let observed_law = one_port_law(&observed_memory, live_symbol);
    let intervened_law = one_port_law(&intervened_memory, live_symbol);
    let frozen_observed_values_hold = observed_law.fresh_ruby == Ratio::new(2, 3)
        && observed_law.alias_ruby == Ratio::new(1, 2)
        && observed_law.fresh_hazard == Ratio::new(175, 216)
        && observed_law.alias_hazard == Ratio::new(45, 64)
        && observed_law.fresh_before_alias_probability == Ratio::new(280, 523)
        && observed_law.expected_candidate_attempts == Ratio::new(766, 523);
    let frozen_intervened_values_hold = intervened_law.fresh_ruby == Ratio::new(1, 2)
        && intervened_law.alias_ruby == Ratio::new(1, 2)
        && intervened_law.fresh_hazard == Ratio::new(45, 64)
        && intervened_law.alias_hazard == Ratio::new(45, 64)
        && intervened_law.fresh_before_alias_probability == Ratio::new(1, 2)
        && intervened_law.expected_candidate_attempts == Ratio::new(3, 2);
    MemoryCausalityReport {
        context_before_intervention: context,
        context_after_intervention: intervened_context,
        absorption_is_reachable_blank_to_success: was_blank && became_success,
        listed_non_memory_coordinates_are_identical: context == intervened_context,
        next_action_law_changes: observed_law.fresh_before_alias_probability
            != intervened_law.fresh_before_alias_probability,
        expected_work_changes: observed_law.expected_work_cells
            != intervened_law.expected_work_cells,
        observed_memory,
        intervened_memory,
        observed_law,
        intervened_law,
        frozen_observed_values_hold,
        frozen_intervened_values_hold,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstantRNonadaptiveReport {
    pub installed_ruby: Ratio,
    pub installed_hazard: Ratio,
    pub observed_memory_law: OnePortLaw,
    pub intervened_memory_law: OnePortLaw,
    pub constant_policy_observed_law: OnePortLaw,
    pub constant_policy_intervened_law: OnePortLaw,
    pub memory_intervention_changes_physical_memory: bool,
    pub constant_policy_ignores_memory: bool,
    pub construction_support_is_preserved: bool,
    pub rejected_as_adaptive: bool,
}

impl ConstantRNonadaptiveReport {
    pub fn matches_frozen_classification(&self) -> bool {
        self.installed_ruby == Ratio::new(1, 2)
            && self.installed_hazard == Ratio::new(45, 64)
            && self.memory_intervention_changes_physical_memory
            && self.constant_policy_ignores_memory
            && self.constant_policy_observed_law == self.constant_policy_intervened_law
            && self
                .constant_policy_observed_law
                .fresh_before_alias_probability
                == Ratio::new(1, 2)
            && self
                .constant_policy_observed_law
                .expected_candidate_attempts
                == Ratio::new(3, 2)
            && self.construction_support_is_preserved
            && self.rejected_as_adaptive
    }
}

pub fn constant_r_nonadaptive_report() -> ConstantRNonadaptiveReport {
    let memory = memory_causality_report();
    let installed_ruby = Ratio::new(1, 2);
    let installed_hazard = opal_phi(installed_ruby);
    let constant_law = OnePortLaw {
        fresh_ruby: installed_ruby,
        alias_ruby: installed_ruby,
        fresh_hazard: installed_hazard,
        alias_hazard: installed_hazard,
        fresh_before_alias_probability: Ratio::new(1, 2),
        expected_candidate_attempts: Ratio::new(3, 2),
        expected_work_cells: Ratio::new(3, 2),
    };
    let constant_policy_observed_law = constant_law.clone();
    let constant_policy_intervened_law = constant_law;
    let memory_intervention_changes_physical_memory =
        memory.observed_memory != memory.intervened_memory;
    let constant_policy_ignores_memory = constant_policy_observed_law
        == constant_policy_intervened_law
        && memory.observed_law != memory.intervened_law;
    let construction_support_is_preserved = constant_policy_observed_law.fresh_hazard.is_positive()
        && constant_policy_observed_law.alias_hazard.is_positive();
    ConstantRNonadaptiveReport {
        installed_ruby,
        installed_hazard,
        observed_memory_law: memory.observed_law,
        intervened_memory_law: memory.intervened_law,
        constant_policy_observed_law,
        constant_policy_intervened_law,
        memory_intervention_changes_physical_memory,
        constant_policy_ignores_memory,
        construction_support_is_preserved,
        rejected_as_adaptive: memory_intervention_changes_physical_memory
            && constant_policy_ignores_memory
            && construction_support_is_preserved,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlankLiveBucketClampReport {
    pub underlying_memory: MemoryLog,
    pub clamped_readout: MemoryLog,
    pub underlying_law: OnePortLaw,
    pub clamped_law: OnePortLaw,
    pub underlying_success_remains_in_live_bucket: bool,
    pub record_bucket_was_not_permuted: bool,
    pub both_clamped_live_buckets_are_blank: bool,
    pub frozen_probability_is_one_half: bool,
    pub frozen_expected_work_is_three_halves: bool,
    pub distinct_from_bucket_permutation: bool,
}

impl BlankLiveBucketClampReport {
    pub fn matches_frozen_classification(&self) -> bool {
        self.underlying_law.fresh_before_alias_probability == Ratio::new(280, 523)
            && self.clamped_law.fresh_before_alias_probability == Ratio::new(1, 2)
            && self.clamped_law.expected_candidate_attempts == Ratio::new(3, 2)
            && self.underlying_success_remains_in_live_bucket
            && self.record_bucket_was_not_permuted
            && self.both_clamped_live_buckets_are_blank
            && self.frozen_probability_is_one_half
            && self.frozen_expected_work_is_three_halves
            && self.distinct_from_bucket_permutation
    }
}

pub fn blank_live_bucket_clamp_report() -> BlankLiveBucketClampReport {
    let causal = memory_causality_report();
    let underlying_memory = causal.observed_memory;
    let underlying_law = one_port_law(&underlying_memory, CAUSAL_LIVE_SYMBOL);
    let clamped_readout = MemoryLog::default();
    let clamped_law = one_port_law(&clamped_readout, CAUSAL_LIVE_SYMBOL);
    let live_bucket = BucketKey::new(CAUSAL_LIVE_SYMBOL, CandidateKind::Fresh);
    let underlying_success_remains_in_live_bucket = underlying_memory
        .records()
        .values()
        .any(|record| record.bucket == live_bucket && record.state == RecordState::Success);
    let record_bucket_was_not_permuted = underlying_memory.records().values().all(|record| {
        record.bucket != BucketKey::new(CAUSAL_INACTIVE_SYMBOL, CandidateKind::Fresh)
    });
    let both_clamped_live_buckets_are_blank = [CandidateKind::Fresh, CandidateKind::Alias]
        .into_iter()
        .all(|kind| {
            clamped_readout.statistics(BucketKey::new(CAUSAL_LIVE_SYMBOL, kind))
                == BucketStatistics {
                    successes: 0,
                    terminal_trials: 0,
                }
        });
    BlankLiveBucketClampReport {
        underlying_memory,
        clamped_readout,
        underlying_law,
        frozen_probability_is_one_half: clamped_law.fresh_before_alias_probability
            == Ratio::new(1, 2),
        frozen_expected_work_is_three_halves: clamped_law.expected_work_cells == Ratio::new(3, 2),
        distinct_from_bucket_permutation: underlying_success_remains_in_live_bucket
            && record_bucket_was_not_permuted,
        clamped_law,
        underlying_success_remains_in_live_bucket,
        record_bucket_was_not_permuted,
        both_clamped_live_buckets_are_blank,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdealUnboundedCounterState {
    pub successes: u64,
    pub terminal_trials: u64,
    pub finite_capacity: Option<u64>,
}

impl IdealUnboundedCounterState {
    pub const fn empty() -> Self {
        Self {
            successes: 0,
            terminal_trials: 0,
            finite_capacity: None,
        }
    }

    pub fn absorb_success(&mut self) {
        self.successes = self
            .successes
            .checked_add(1)
            .expect("ideal success-counter overflow");
        self.terminal_trials = self
            .terminal_trials
            .checked_add(1)
            .expect("ideal trial-counter overflow");
    }

    pub fn predictive_mean(self) -> Ratio {
        posterior_predictive(BucketStatistics {
            successes: self.successes,
            terminal_trials: self.terminal_trials,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdealCounterDowngradeReport {
    pub fresh_counter: IdealUnboundedCounterState,
    pub alias_counter: IdealUnboundedCounterState,
    pub fresh_ruby: Ratio,
    pub alias_ruby: Ratio,
    pub fresh_hazard: Ratio,
    pub alias_hazard: Ratio,
    pub logical_law_matches_physical_fixture: bool,
    pub has_finite_record_bank: bool,
    pub has_finite_work_store: bool,
    pub has_operation_records: bool,
    pub has_named_inverse_protocol: bool,
    pub full_physical_label_rejected: bool,
}

impl IdealCounterDowngradeReport {
    pub fn matches_frozen_classification(&self) -> bool {
        self.fresh_counter.successes == 1
            && self.fresh_counter.terminal_trials == 1
            && self.fresh_counter.finite_capacity.is_none()
            && self.alias_counter.successes == 0
            && self.alias_counter.terminal_trials == 0
            && self.alias_counter.finite_capacity.is_none()
            && self.fresh_ruby == Ratio::new(2, 3)
            && self.alias_ruby == Ratio::new(1, 2)
            && self.fresh_hazard == Ratio::new(175, 216)
            && self.alias_hazard == Ratio::new(45, 64)
            && self.logical_law_matches_physical_fixture
            && !self.has_finite_record_bank
            && !self.has_finite_work_store
            && !self.has_operation_records
            && !self.has_named_inverse_protocol
            && self.full_physical_label_rejected
    }
}

pub fn ideal_counter_downgrade_report() -> IdealCounterDowngradeReport {
    let mut fresh_counter = IdealUnboundedCounterState::empty();
    fresh_counter.absorb_success();
    let alias_counter = IdealUnboundedCounterState::empty();
    let fresh_ruby = fresh_counter.predictive_mean();
    let alias_ruby = alias_counter.predictive_mean();
    let fresh_hazard = opal_phi(fresh_ruby);
    let alias_hazard = opal_phi(alias_ruby);
    let physical = memory_causality_report().observed_law;
    let logical_law_matches_physical_fixture = fresh_ruby == physical.fresh_ruby
        && alias_ruby == physical.alias_ruby
        && fresh_hazard == physical.fresh_hazard
        && alias_hazard == physical.alias_hazard;
    let has_finite_record_bank =
        fresh_counter.finite_capacity.is_some() && alias_counter.finite_capacity.is_some();
    let has_finite_work_store = false;
    let has_operation_records = false;
    let has_named_inverse_protocol = false;
    IdealCounterDowngradeReport {
        fresh_counter,
        alias_counter,
        fresh_ruby,
        alias_ruby,
        fresh_hazard,
        alias_hazard,
        logical_law_matches_physical_fixture,
        has_finite_record_bank,
        has_finite_work_store,
        has_operation_records,
        has_named_inverse_protocol,
        full_physical_label_rejected: logical_law_matches_physical_fixture
            && !has_finite_record_bank
            && !has_finite_work_store
            && !has_operation_records
            && !has_named_inverse_protocol,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutcomeCopyInformation {
    pub outcome_probabilities: [Ratio; 2],
    pub record_probabilities: [Ratio; 2],
    pub joint_distribution: [[Ratio; 2]; 2],
    pub outcome_entropy: ExactLog,
    pub record_outcome_mutual_information: ExactLog,
    pub terminal_record_is_exact_copy: bool,
    pub mutual_information_equals_entropy: bool,
}

impl OutcomeCopyInformation {
    pub fn exact_copy_identity_passes(&self) -> bool {
        let outcome_normalized =
            self.outcome_probabilities.iter().copied().sum::<Ratio>() == Ratio::ONE;
        let record_normalized =
            self.record_probabilities.iter().copied().sum::<Ratio>() == Ratio::ONE;
        let joint_normalized = self
            .joint_distribution
            .iter()
            .flat_map(|row| row.iter())
            .copied()
            .sum::<Ratio>()
            == Ratio::ONE;
        let record_marginals_match = (0..2).all(|record| {
            self.joint_distribution[record]
                .iter()
                .copied()
                .sum::<Ratio>()
                == self.record_probabilities[record]
        });
        let outcome_marginals_match = (0..2).all(|outcome| {
            (0..2)
                .map(|record| self.joint_distribution[record][outcome])
                .sum::<Ratio>()
                == self.outcome_probabilities[outcome]
        });
        outcome_normalized
            && record_normalized
            && joint_normalized
            && record_marginals_match
            && outcome_marginals_match
            && self.terminal_record_is_exact_copy
            && self.mutual_information_equals_entropy
            && self.record_outcome_mutual_information == self.outcome_entropy
    }
}

pub fn exact_outcome_copy_information(success_probability: Ratio) -> OutcomeCopyInformation {
    assert!(success_probability > Ratio::ZERO && success_probability < Ratio::ONE);
    let failure_probability = Ratio::ONE - success_probability;
    let joint_distribution = [
        [success_probability, Ratio::ZERO],
        [Ratio::ZERO, failure_probability],
    ];
    outcome_copy_information_from_joint(joint_distribution)
}

fn outcome_copy_information_from_joint(
    joint_distribution: [[Ratio; 2]; 2],
) -> OutcomeCopyInformation {
    let record_probabilities = joint_distribution.map(|row| row.into_iter().sum::<Ratio>());
    let outcome_probabilities = [
        joint_distribution[0][0] + joint_distribution[1][0],
        joint_distribution[0][1] + joint_distribution[1][1],
    ];
    let outcome_entropy =
        outcome_probabilities
            .into_iter()
            .fold(ExactLog::zero(), |entropy, probability| {
                if probability.is_zero() {
                    entropy
                } else {
                    entropy - ExactLog::ln_ratio(probability).scaled(probability)
                }
            });
    let mut record_outcome_mutual_information = ExactLog::zero();
    for (record, row) in joint_distribution.iter().enumerate() {
        for (outcome, &joint_probability) in row.iter().enumerate() {
            if joint_probability.is_zero() {
                continue;
            }
            let information_density = ExactLog::ln_ratio(
                joint_probability / (record_probabilities[record] * outcome_probabilities[outcome]),
            );
            record_outcome_mutual_information =
                record_outcome_mutual_information + information_density.scaled(joint_probability);
        }
    }
    let terminal_record_is_exact_copy = joint_distribution[0][1].is_zero()
        && joint_distribution[1][0].is_zero()
        && joint_distribution[0][0] == outcome_probabilities[0]
        && joint_distribution[1][1] == outcome_probabilities[1];
    let mutual_information_equals_entropy = record_outcome_mutual_information == outcome_entropy;
    OutcomeCopyInformation {
        outcome_probabilities,
        record_probabilities,
        joint_distribution,
        outcome_entropy,
        record_outcome_mutual_information,
        terminal_record_is_exact_copy,
        mutual_information_equals_entropy,
    }
}

fn construction_terminal_record_label(
    semantic_record: &model::SemanticRecord,
    origin_operation_record: &model::OperationRecord,
) -> Option<usize> {
    match (
        semantic_record.candidate.as_ref(),
        semantic_record.state,
        origin_operation_record.entry.as_ref(),
    ) {
        (
            Some(candidate),
            model::SemanticState::Success,
            Some(model::OperationEntry::Fresh { .. }),
        ) if candidate.kind == model::CandidateKind::Fresh => Some(0),
        (
            Some(candidate),
            model::SemanticState::Failure,
            Some(model::OperationEntry::AliasQuarantine { .. }),
        ) if candidate.kind == model::CandidateKind::Alias => Some(1),
        _ => None,
    }
}

fn construction_outcome_label(outcome: model::FirstCandidateOutcome) -> usize {
    match outcome {
        model::FirstCandidateOutcome::FreshFirst => 0,
        model::FirstCandidateOutcome::AliasFirst => 1,
    }
}

fn physical_construction_outcome_information(
    audit: &model::ConstructionQuotientPhysicalAudit,
) -> OutcomeCopyInformation {
    let mut joint_distribution = [[Ratio::ZERO; 2]; 2];
    for (outcome, probability, terminal_record, origin_operation_record) in [
        (
            model::FirstCandidateOutcome::FreshFirst,
            audit.first_event_records.fresh.probability,
            &audit.fresh_first.terminal_fresh_record,
            &audit.first_event_records.fresh.operation_record,
        ),
        (
            model::FirstCandidateOutcome::AliasFirst,
            audit.first_event_records.alias.probability,
            &audit.alias_first.terminal_alias_record,
            &audit.first_event_records.alias.operation_record,
        ),
    ] {
        let record = construction_terminal_record_label(terminal_record, origin_operation_record)
            .expect("construction branch must expose a typed physical outcome record");
        let outcome = construction_outcome_label(outcome);
        joint_distribution[record][outcome] = joint_distribution[record][outcome] + probability;
    }
    outcome_copy_information_from_joint(joint_distribution)
}

fn adaptive_record_state(state: model::SemanticState) -> RecordState {
    match state {
        model::SemanticState::Blank => RecordState::Blank,
        model::SemanticState::Pending => RecordState::Pending,
        model::SemanticState::Success => RecordState::Success,
        model::SemanticState::Failure => RecordState::Failure,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructionQuotientInformation {
    pub base: u8,
    pub frontier_depth: u8,
    pub prior_fresh_successes: u64,
    pub prior_alias_failures: u64,
    pub fresh_ruby: Ratio,
    pub alias_ruby: Ratio,
    pub fresh_hazard: Ratio,
    pub alias_hazard: Ratio,
    pub fresh_first_probability: Ratio,
    pub expected_candidate_attempts: Ratio,
    pub information: OutcomeCopyInformation,
    pub fresh_first_branch_attempts: u8,
    pub alias_first_branch_attempts: u8,
    pub fresh_first_immediate_record: RecordState,
    pub fresh_first_terminal_record: RecordState,
    pub fresh_first_credit_required: bool,
    pub alias_first_immediate_record: RecordState,
    pub alias_first_terminal_record: RecordState,
    pub alias_then_fresh_immediate_record: RecordState,
    pub alias_then_fresh_terminal_record: RecordState,
    pub alias_then_fresh_is_forced: bool,
    pub branch_weighted_attempt_expectation: Ratio,
    pub branch_expectation_matches_formula: bool,
    pub physical_branch_records_copy_first_outcome: bool,
    pub physical_hazards_match_posterior: bool,
    pub physical_probabilities_match_hazards: bool,
    pub physical_audit: model::ConstructionQuotientPhysicalAudit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstructionFrontierInformationReport {
    pub base: u8,
    pub frontier_depth: u8,
    pub total_frontier_ports: u64,
    pub current_frontier_ports_per_symbol: u64,
    pub prior_fresh_successes_per_symbol: u64,
    pub maximum_prior_alias_failures_per_symbol: u64,
    pub current_frontier_fresh_records_are_pending_until_credit: bool,
    pub quotients: Vec<ConstructionQuotientInformation>,
    pub every_feasible_alias_count_is_reported: bool,
    pub every_joint_distribution_is_normalized: bool,
    pub every_terminal_record_is_an_exact_copy: bool,
    pub every_mutual_information_equals_entropy: bool,
    pub every_branch_matches_physical_record_semantics: bool,
}

impl ConstructionFrontierInformationReport {
    pub fn exact_information_account_passes(&self) -> bool {
        if !ALPHABET_BASES.contains(&self.base)
            || !(1..=PROMOTED_DEPTH).contains(&self.frontier_depth)
        {
            return false;
        }
        let b = u64::from(self.base);
        let expected_total = integer_power_u64(b, u32::from(self.frontier_depth));
        let expected_per_symbol = integer_power_u64(b, u32::from(self.frontier_depth - 1));
        let mut expected_prior_fresh = 0_u64;
        for completed_depth in 1..self.frontier_depth {
            expected_prior_fresh = expected_prior_fresh
                .checked_add(integer_power_u64(b, u32::from(completed_depth - 1)))
                .expect("frozen prior-success count fits u64");
        }
        let expected_maximum_alias = expected_prior_fresh
            .checked_add(expected_per_symbol)
            .and_then(|value| value.checked_sub(1))
            .expect("a frozen frontier has at least one port");
        self.total_frontier_ports == expected_total
            && self.current_frontier_ports_per_symbol == expected_per_symbol
            && self.prior_fresh_successes_per_symbol == expected_prior_fresh
            && self.maximum_prior_alias_failures_per_symbol == expected_maximum_alias
            && self.current_frontier_fresh_records_are_pending_until_credit
            && self.every_feasible_alias_count_is_reported
            && self.every_joint_distribution_is_normalized
            && self.every_terminal_record_is_an_exact_copy
            && self.every_mutual_information_equals_entropy
            && self.every_branch_matches_physical_record_semantics
            && self.quotients.iter().all(|quotient| {
                quotient.base == self.base
                    && quotient.frontier_depth == self.frontier_depth
                    && quotient.prior_fresh_successes == expected_prior_fresh
                    && quotient.information.exact_copy_identity_passes()
                    && quotient.physical_hazards_match_posterior
                    && quotient.physical_probabilities_match_hazards
                    && quotient.physical_audit.representative_is_reachable_quotient
                    && quotient
                        .physical_audit
                        .both_branches_complete_required_record_protocol
            })
    }
}

pub fn construction_frontier_information_reports(
    base: u8,
) -> Vec<ConstructionFrontierInformationReport> {
    assert!(ALPHABET_BASES.contains(&base));
    (1..=PROMOTED_DEPTH)
        .map(|frontier_depth| construction_frontier_information_report(base, frontier_depth))
        .collect()
}

pub fn construction_frontier_information_report(
    base: u8,
    frontier_depth: u8,
) -> ConstructionFrontierInformationReport {
    assert!(ALPHABET_BASES.contains(&base));
    assert!((1..=PROMOTED_DEPTH).contains(&frontier_depth));
    let b = u64::from(base);
    let mut prior_fresh_successes_per_symbol = 0_u64;
    for completed_depth in 1..frontier_depth {
        prior_fresh_successes_per_symbol = prior_fresh_successes_per_symbol
            .checked_add(integer_power_u64(b, u32::from(completed_depth - 1)))
            .expect("prior FRESH-success count overflow");
    }
    let current_symbol_ports = integer_power_u64(b, u32::from(frontier_depth - 1));
    let total_frontier_ports = integer_power_u64(b, u32::from(frontier_depth));
    let maximum_prior_alias_failures_per_symbol = prior_fresh_successes_per_symbol
        .checked_add(current_symbol_ports)
        .and_then(|value| value.checked_sub(1))
        .expect("frontier has at least one port per symbol");
    let quotients = (0..=maximum_prior_alias_failures_per_symbol)
        .map(|prior_alias_failures| {
            construction_quotient_information(
                base,
                frontier_depth,
                prior_fresh_successes_per_symbol,
                prior_alias_failures,
            )
        })
        .collect::<Vec<_>>();
    let every_feasible_alias_count_is_reported = quotients.len()
        == usize::try_from(maximum_prior_alias_failures_per_symbol + 1)
            .expect("quotient count fits usize")
        && quotients
            .iter()
            .enumerate()
            .all(|(index, quotient)| quotient.prior_alias_failures == index as u64);
    let every_joint_distribution_is_normalized = quotients
        .iter()
        .all(|quotient| quotient.information.exact_copy_identity_passes());
    let current_frontier_fresh_records_are_pending_until_credit =
        quotients.iter().all(|quotient| {
            quotient.fresh_first_immediate_record == RecordState::Pending
                && quotient.fresh_first_terminal_record == RecordState::Success
                && quotient.fresh_first_credit_required
                && quotient.alias_then_fresh_immediate_record == RecordState::Pending
                && quotient.alias_then_fresh_terminal_record == RecordState::Success
        });
    ConstructionFrontierInformationReport {
        base,
        frontier_depth,
        total_frontier_ports,
        current_frontier_ports_per_symbol: current_symbol_ports,
        prior_fresh_successes_per_symbol,
        maximum_prior_alias_failures_per_symbol,
        current_frontier_fresh_records_are_pending_until_credit,
        every_feasible_alias_count_is_reported,
        every_joint_distribution_is_normalized,
        every_terminal_record_is_an_exact_copy: quotients
            .iter()
            .all(|quotient| quotient.information.terminal_record_is_exact_copy),
        every_mutual_information_equals_entropy: quotients.iter().all(|quotient| {
            quotient.information.mutual_information_equals_entropy
                && quotient.information.outcome_entropy
                    == quotient.information.record_outcome_mutual_information
        }),
        every_branch_matches_physical_record_semantics: quotients.iter().all(|quotient| {
            quotient.physical_branch_records_copy_first_outcome
                && quotient.branch_expectation_matches_formula
        }),
        quotients,
    }
}

fn construction_quotient_information(
    base: u8,
    frontier_depth: u8,
    prior_fresh_successes: u64,
    prior_alias_failures: u64,
) -> ConstructionQuotientInformation {
    let physical_audit = model::construction_quotient_physical_audit(
        usize::from(base),
        usize::from(frontier_depth),
        usize::try_from(prior_alias_failures).expect("frozen alias-failure count fits usize"),
    )
    .expect("every frozen construction quotient has a reachable model state");
    let (actual_fresh_successes, actual_fresh_trials) =
        physical_audit.representative.fresh_bucket_counts;
    let (actual_alias_successes, actual_alias_trials) =
        physical_audit.representative.alias_bucket_counts;
    let actual_prior_fresh_successes = u64::from(actual_fresh_successes);
    let actual_prior_alias_failures = u64::from(
        actual_alias_trials
            .checked_sub(actual_alias_successes)
            .expect("alias successes cannot exceed trials"),
    );
    let fresh_ruby = posterior_predictive(BucketStatistics {
        successes: actual_prior_fresh_successes,
        terminal_trials: u64::from(actual_fresh_trials),
    });
    let alias_ruby = posterior_predictive(BucketStatistics {
        successes: u64::from(actual_alias_successes),
        terminal_trials: u64::from(actual_alias_trials),
    });
    let fresh_hazard = physical_audit.first_event_records.fresh_hazard;
    let alias_hazard = physical_audit.first_event_records.alias_hazard;
    let posterior_fresh_hazard = opal_phi(fresh_ruby);
    let posterior_alias_hazard = opal_phi(alias_ruby);
    let physical_hazards_match_posterior = fresh_hazard == posterior_fresh_hazard
        && alias_hazard == posterior_alias_hazard
        && actual_prior_fresh_successes == prior_fresh_successes
        && actual_prior_alias_failures == prior_alias_failures;
    let fresh_first_probability = physical_audit.first_event_records.fresh.probability;
    let alias_first_probability = physical_audit.first_event_records.alias.probability;
    let physical_probabilities_match_hazards = physical_audit.branch_probabilities_are_normalized
        && fresh_first_probability == fresh_hazard / (fresh_hazard + alias_hazard)
        && alias_first_probability == alias_hazard / (fresh_hazard + alias_hazard);
    let expected_candidate_attempts = Ratio::integer(2) - fresh_first_probability;
    let information = physical_construction_outcome_information(&physical_audit);
    let fresh_first_branch_attempts = physical_audit.fresh_first.candidate_attempts;
    let alias_first_branch_attempts = physical_audit.alias_first.candidate_attempts;
    let fresh_first_immediate_record =
        adaptive_record_state(physical_audit.fresh_first.immediate_fresh_state);
    let fresh_first_terminal_record =
        adaptive_record_state(physical_audit.fresh_first.terminal_fresh_state);
    let fresh_first_credit_required = physical_audit
        .fresh_first
        .eventual_credit_updates_same_fresh_record;
    let alias_first_immediate_record =
        adaptive_record_state(physical_audit.alias_first.immediate_alias_state);
    let alias_first_terminal_record =
        adaptive_record_state(physical_audit.alias_first.terminal_alias_state);
    let alias_then_fresh_immediate_record =
        adaptive_record_state(physical_audit.alias_first.immediate_fresh_state);
    let alias_then_fresh_terminal_record =
        adaptive_record_state(physical_audit.alias_first.terminal_fresh_state);
    let alias_then_fresh_is_forced = physical_audit.alias_first.alias_then_fresh_is_forced;
    let branch_weighted_attempt_expectation = fresh_first_probability
        * Ratio::integer(i128::from(fresh_first_branch_attempts))
        + alias_first_probability * Ratio::integer(i128::from(alias_first_branch_attempts));
    let branch_expectation_matches_formula =
        branch_weighted_attempt_expectation == expected_candidate_attempts;
    let physical_branch_records_copy_first_outcome = physical_audit
        .records_injectively_copy_first_outcome
        && physical_audit.both_branches_complete_required_record_protocol
        && information.terminal_record_is_exact_copy
        && fresh_first_immediate_record == RecordState::Pending
        && fresh_first_terminal_record == RecordState::Success
        && fresh_first_credit_required
        && alias_first_immediate_record == RecordState::Failure
        && alias_first_terminal_record == RecordState::Failure
        && alias_then_fresh_immediate_record == RecordState::Pending
        && alias_then_fresh_terminal_record == RecordState::Success
        && alias_then_fresh_is_forced;
    ConstructionQuotientInformation {
        base,
        frontier_depth,
        prior_fresh_successes: actual_prior_fresh_successes,
        prior_alias_failures: actual_prior_alias_failures,
        fresh_ruby,
        alias_ruby,
        fresh_hazard,
        alias_hazard,
        fresh_first_probability,
        expected_candidate_attempts,
        information,
        fresh_first_branch_attempts,
        alias_first_branch_attempts,
        fresh_first_immediate_record,
        fresh_first_terminal_record,
        fresh_first_credit_required,
        alias_first_immediate_record,
        alias_first_terminal_record,
        alias_then_fresh_immediate_record,
        alias_then_fresh_terminal_record,
        alias_then_fresh_is_forced,
        branch_weighted_attempt_expectation,
        branch_expectation_matches_formula,
        physical_branch_records_copy_first_outcome,
        physical_hazards_match_posterior,
        physical_probabilities_match_hazards,
        physical_audit,
    }
}

pub fn application_gate(phi_history: &[Ratio]) -> Option<Ratio> {
    if phi_history.is_empty() {
        return None;
    }
    let sum: Ratio = phi_history.iter().copied().sum();
    assert!(sum.is_positive());
    Some(Ratio::integer(phi_history.len() as i128) / sum)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyAPNoGoReport {
    pub checked_ruby_values: u64,
    pub minimum_phi: Ratio,
    pub maximum_phi: Ratio,
    pub every_factorization_identity_holds: bool,
    pub every_phi_is_strictly_between_three_eighths_and_one: bool,
    pub zero_history_is_undefined: bool,
    pub nonempty_history_mean_is_below_one: bool,
    pub nonempty_application_gate_is_above_one: bool,
    pub every_candidate_phi_is_below_application_gate: bool,
    pub strict_legacy_gate_deadlocks: bool,
}

pub fn legacy_ap_no_go_report() -> LegacyAPNoGoReport {
    let maximum_trials_per_bucket = ALPHABET_BASES
        .into_iter()
        .map(|base| {
            1_u64
                .checked_add(u64::from(base))
                .and_then(|value| value.checked_add(u64::from(base).pow(2)))
                .and_then(|value| value.checked_add(1))
                .expect("bucket trial bound overflow")
        })
        .max()
        .expect("at least one frozen base must exist");
    let reports = all_feasible_phi_bounds(maximum_trials_per_bucket);
    let checked_ruby_values =
        u64::try_from(reports.len()).expect("checked Ruby-state count does not fit u64");
    let minimum_phi = reports
        .iter()
        .map(|report| report.phi)
        .min()
        .expect("at least one feasible Ruby state must exist");
    let maximum_phi = reports
        .iter()
        .map(|report| report.phi)
        .max()
        .expect("at least one feasible Ruby state must exist");
    let every_factorization_identity_holds =
        reports.iter().all(|report| report.exact_identities_hold);
    let every_phi_is_strictly_between_three_eighths_and_one = reports
        .iter()
        .all(|report| report.strictly_between_three_eighths_and_one);

    let nonempty_history_mean_is_below_one = minimum_phi.is_positive() && maximum_phi < Ratio::ONE;
    let nonempty_application_gate_is_above_one = nonempty_history_mean_is_below_one;
    let every_candidate_phi_is_below_application_gate =
        maximum_phi < Ratio::ONE && nonempty_application_gate_is_above_one;
    LegacyAPNoGoReport {
        checked_ruby_values,
        minimum_phi,
        maximum_phi,
        every_factorization_identity_holds,
        every_phi_is_strictly_between_three_eighths_and_one,
        zero_history_is_undefined: application_gate(&[]).is_none(),
        nonempty_history_mean_is_below_one,
        nonempty_application_gate_is_above_one,
        every_candidate_phi_is_below_application_gate,
        strict_legacy_gate_deadlocks: every_factorization_identity_holds
            && every_phi_is_strictly_between_three_eighths_and_one
            && application_gate(&[]).is_none()
            && every_candidate_phi_is_below_application_gate,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaintenanceReport {
    pub base: u8,
    pub leaf_edges_checked: u64,
    pub expected_complete_counts: Vec<u64>,
    pub expected_damaged_counts: Vec<u64>,
    pub every_leaf_deletion_has_frozen_damaged_counts: bool,
    pub every_leaf_repair_restores_complete_counts: bool,
    pub every_deleted_descriptor_is_returned: bool,
    pub every_damage_breaks_geometric_shell_recursion: bool,
    pub worst_candidate_attempts: u64,
    pub credit_operations: u64,
    pub promote_operations: u64,
    pub total_internal_repair_operations: u64,
    pub locked_repair_reserve_cells: u64,
    pub reserve_covers_worst_repair_ordering: bool,
    pub same_local_rank_certificate_applies: bool,
    pub every_repair_hazard_is_positive: bool,
    pub probability_one_finite_mean_repair_certified: bool,
    pub perturb_uses_separate_external_cell: bool,
    pub descriptor_withheld_is_resource_obstruction: bool,
    pub repair_disabled_leaves_defect_outside_target: bool,
}

pub fn maintenance_report(base: u8) -> MaintenanceReport {
    let fixture = prefix_fixture(base);
    let complete = PrefixArchitecture::enumerated_reference(base, PROMOTED_DEPTH);
    let leaves: Vec<_> = complete
        .promoted()
        .keys()
        .filter(|word| word.depth() == usize::from(PROMOTED_DEPTH))
        .cloned()
        .collect();
    let mut every_leaf_deletion_has_frozen_damaged_counts = true;
    let mut every_leaf_repair_restores_complete_counts = true;
    let mut every_deleted_descriptor_is_returned = true;
    let mut every_damage_breaks_geometric_shell_recursion = true;
    let mut descriptor_withheld_is_resource_obstruction = true;
    let mut repair_disabled_leaves_defect_outside_target = true;
    let mut expected_damaged_counts = fixture.expected_shell_counts.clone();
    let last = expected_damaged_counts
        .last_mut()
        .expect("depth-three count must exist");
    *last = last.checked_sub(1).expect("leaf count underflow");

    for leaf in &leaves {
        let mut damaged = complete.clone();
        let detached = damaged.detach_depth_three_leaf(leaf);
        every_leaf_deletion_has_frozen_damaged_counts &=
            damaged.shell_counts(PROMOTED_DEPTH) == expected_damaged_counts;
        let damaged_counts = damaged.shell_counts(PROMOTED_DEPTH);
        every_damage_breaks_geometric_shell_recursion &= damaged_counts
            .windows(2)
            .any(|window| window[1] != window[0] * u64::from(base));
        every_deleted_descriptor_is_returned &=
            damaged.free_descriptors().contains(&detached.descriptor_id);
        let mut descriptor_withheld = damaged.clone();
        descriptor_withheld
            .free_descriptors
            .remove(&detached.descriptor_id);
        descriptor_withheld_is_resource_obstruction &=
            descriptor_withheld.free_descriptors().is_empty();
        repair_disabled_leaves_defect_outside_target &=
            damaged_counts != fixture.expected_shell_counts;
        damaged.restore_depth_three_leaf(detached);
        every_leaf_repair_restores_complete_counts &=
            damaged.shell_counts(PROMOTED_DEPTH) == fixture.expected_shell_counts;
    }

    let worst_candidate_attempts = CANDIDATE_KIND_COUNT;
    let credit_operations = 1;
    let promote_operations = 1;
    let total_internal_repair_operations =
        worst_candidate_attempts + credit_operations + promote_operations;
    let cursor = LocalCursorState::at_open_port(1);
    let same_local_rank_certificate_applies = candidate_transition(cursor, CandidateKind::Fresh)
        .rank_strictly_decreases
        && candidate_transition(cursor, CandidateKind::Alias).rank_strictly_decreases
        && candidate_transition(cursor.after_alias(), CandidateKind::Fresh).rank_strictly_decreases;
    let maximum_repair_trials_per_bucket = 1_u64
        .checked_add(u64::from(base))
        .and_then(|value| value.checked_add(u64::from(base).pow(2)))
        .and_then(|value| value.checked_add(1))
        .expect("repair trial bound overflow");
    let every_repair_hazard_is_positive = all_feasible_phi_bounds(maximum_repair_trials_per_bucket)
        .iter()
        .all(|report| report.phi.is_positive());
    let reserve_covers_worst_repair_ordering =
        total_internal_repair_operations == fixture.repair_work_cells;
    let probability_one_finite_mean_repair_certified = every_leaf_deletion_has_frozen_damaged_counts
        && every_leaf_repair_restores_complete_counts
        && every_deleted_descriptor_is_returned
        && same_local_rank_certificate_applies
        && every_repair_hazard_is_positive
        && reserve_covers_worst_repair_ordering;
    MaintenanceReport {
        base,
        leaf_edges_checked: u64::try_from(leaves.len()).expect("leaf-edge count does not fit u64"),
        expected_complete_counts: fixture.expected_shell_counts,
        expected_damaged_counts,
        every_leaf_deletion_has_frozen_damaged_counts,
        every_leaf_repair_restores_complete_counts,
        every_deleted_descriptor_is_returned,
        every_damage_breaks_geometric_shell_recursion,
        worst_candidate_attempts,
        credit_operations,
        promote_operations,
        total_internal_repair_operations,
        locked_repair_reserve_cells: fixture.repair_work_cells,
        reserve_covers_worst_repair_ordering,
        same_local_rank_certificate_applies,
        every_repair_hazard_is_positive,
        probability_one_finite_mean_repair_certified,
        perturb_uses_separate_external_cell: PERTURB_EXTERNAL_CELLS == 1,
        descriptor_withheld_is_resource_obstruction,
        repair_disabled_leaves_defect_outside_target,
    }
}

fn all_feasible_phi_bounds(maximum_trials: u64) -> Vec<PhiBoundsReport> {
    let mut reports = Vec::new();
    for terminal_trials in 0..=maximum_trials {
        for successes in 0..=terminal_trials {
            let ruby = posterior_predictive(BucketStatistics {
                successes,
                terminal_trials,
            });
            reports.push(phi_bounds_report(ruby));
        }
    }
    reports
}

fn integer_power_u64(mut base: u64, mut exponent: u32) -> u64 {
    let mut result = 1_u64;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result.checked_mul(base).expect("integer-power overflow");
        }
        exponent >>= 1;
        if exponent > 0 {
            base = base.checked_mul(base).expect("integer-power overflow");
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beta_bernoulli_and_opal_are_exact() {
        let ruby = posterior_predictive(BucketStatistics {
            successes: 1,
            terminal_trials: 1,
        });
        assert_eq!(ruby, Ratio::new(2, 3));
        assert_eq!(opal_phi(ruby), Ratio::new(175, 216));
        let blank = posterior_predictive(BucketStatistics {
            successes: 0,
            terminal_trials: 0,
        });
        assert_eq!(blank, Ratio::new(1, 2));
        assert_eq!(opal_phi(blank), Ratio::new(45, 64));
    }

    #[test]
    fn one_port_memory_intervention_changes_law_and_work() {
        let report = memory_causality_report();
        assert!(report.absorption_is_reachable_blank_to_success);
        assert!(report.listed_non_memory_coordinates_are_identical);
        assert!(report.next_action_law_changes);
        assert!(report.expected_work_changes);
        assert!(report.frozen_observed_values_hold);
        assert!(report.frozen_intervened_values_hold);
    }

    #[test]
    fn local_prefix_actions_need_no_depth_target() {
        let mut architecture = PrefixArchitecture::root_with_inventory(2, 2);
        let first = architecture
            .open_ports()
            .into_iter()
            .next()
            .expect("first port exists");
        architecture.bind_fresh_structural_reference(&first);
        assert!(!architecture.frontier_is_locally_complete());
        let second = architecture
            .open_ports()
            .into_iter()
            .next()
            .expect("second port exists");
        architecture.bind_fresh_structural_reference(&second);
        assert!(architecture.frontier_is_locally_complete());
        assert!(architecture.promote_structural_reference_if_complete());
        assert_eq!(architecture.shell_counts(1), vec![1, 2]);
    }

    #[test]
    fn frozen_resource_formulas_are_derived_from_ports() {
        let binary = resource_report(2);
        assert_eq!(binary.fixture.primary_descriptors, 14);
        assert_eq!(binary.derived_build_operation_bound, 45);
        assert!(binary.build_capacity_covers_worst_ordering);
        let quaternary = creation_rank_report(4);
        assert_eq!(quaternary.total_ports, 84);
        assert_eq!(quaternary.candidate_kind_histories, 1_u128 << 84);
        assert!(quaternary.finite_rank_certificate_closes_creation);
    }

    #[test]
    fn historical_application_gate_has_the_frozen_no_go() {
        let report = legacy_ap_no_go_report();
        assert!(report.every_factorization_identity_holds);
        assert!(report.every_phi_is_strictly_between_three_eighths_and_one);
        assert!(report.zero_history_is_undefined);
        assert!(report.nonempty_application_gate_is_above_one);
        assert!(report.strict_legacy_gate_deadlocks);
    }

    #[test]
    fn negative_memory_and_counter_controls_receive_frozen_labels() {
        assert!(constant_r_nonadaptive_report().matches_frozen_classification());
        assert!(blank_live_bucket_clamp_report().matches_frozen_classification());
        assert!(ideal_counter_downgrade_report().matches_frozen_classification());
    }

    #[test]
    fn every_construction_quotient_has_an_exact_physical_information_account() {
        for base in ALPHABET_BASES {
            let reports = construction_frontier_information_reports(base);
            assert_eq!(reports.len(), usize::from(PROMOTED_DEPTH));
            assert!(reports
                .iter()
                .all(ConstructionFrontierInformationReport::exact_information_account_passes));
        }
    }
}
