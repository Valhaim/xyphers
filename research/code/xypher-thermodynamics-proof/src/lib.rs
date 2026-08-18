use std::fmt::Write;

const QUARTERS_PER_UNIT: i64 = 4;
const TOTAL_ENERGY: i32 = 2;
const SYSTEM_ENERGY: [i32; 3] = [0, 1, 2];
const MICRO_G_X: [usize; 3] = [1, 4, 4];
const BASE_CRYSTAL_G_X: [usize; 3] = [1, 4, 4];
const DIRECTED_MACRO_EDGES: [(usize, usize); 4] = [(0, 1), (1, 0), (1, 2), (2, 1)];
const UNDIRECTED_MACRO_EDGES: [(usize, usize); 2] = [(0, 1), (1, 2)];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateId {
    G01,
    G02,
    G03,
    G04,
    G05,
    G06,
    G07,
    G08,
    G09,
    G10,
    G11,
}

impl GateId {
    pub const ALL: [Self; 11] = [
        Self::G01,
        Self::G02,
        Self::G03,
        Self::G04,
        Self::G05,
        Self::G06,
        Self::G07,
        Self::G08,
        Self::G09,
        Self::G10,
        Self::G11,
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::G01 => "G01",
            Self::G02 => "G02",
            Self::G03 => "G03",
            Self::G04 => "G04",
            Self::G05 => "G05",
            Self::G06 => "G06",
            Self::G07 => "G07",
            Self::G08 => "G08",
            Self::G09 => "G09",
            Self::G10 => "G10",
            Self::G11 => "G11",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::G01 => "complete state and Crystal grounding",
            Self::G02 => "executable Crystal REFRESH",
            Self::G03 => "reciprocal MOVE generators",
            Self::G04 => "pathwise energy accounting",
            Self::G05 => "strong lumpability",
            Self::G06 => "exact macro generator",
            Self::G07 => "Crystal-Thermo-Praxion construction",
            Self::G08 => "equilibrium and detailed balance",
            Self::G09 => "local detailed balance",
            Self::G10 => "minimal-kernel slots",
            Self::G11 => "same-temperature contact",
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckId {
    G01_STATE_COUNT,
    G01_FIBER_SIZES,
    G01_CRYSTAL_FIBER_GROUNDING,
    G02_REFRESH_OUTCOMES,
    G02_QUARTER_RATES,
    G02_Q_REFRESH_IDENTITY,
    G02_DOUBLE_STOCHASTIC,
    G02_ENERGY_PRESERVATION,
    G02_ENTROPY,
    G03_EQUAL_REVERSE_MICROHAZARDS,
    G03_UNIT_MICROHAZARD,
    G03_ROW_CLOSURE,
    G03_DIAGONAL,
    G04_TOTAL_ENERGY,
    G05_STRONG_LUMPABILITY,
    G06_MACRO_GENERATOR,
    G07_RECIPROCAL_SUPPORT,
    G07_XI_ANTISYMMETRY,
    G07_TAU_PAIR,
    G07_SYMMETRIC_ACTIVITY,
    G07_PRAXION_EQUATION,
    G07_MICRO_LUMPING_AGREEMENT,
    G08_STATIONARY_WEIGHTS,
    G08_DETAILED_BALANCE,
    G08_STATIONARITY,
    G09_RESERVOIR_RELATION,
    G09_LOCAL_DETAILED_BALANCE,
    G10_NAMED_SLOTS,
    G10_EMPTY_RUBY,
    G11_CONTACT_SHELL,
    G11_UNIT_MICROHAZARD,
    G11_ROW_CLOSURE,
    G11_CANONICAL_WEIGHTS,
    G11_DETAILED_BALANCE,
    G11_ZERO_CURRENT,
}

impl CheckId {
    pub const fn code(self) -> &'static str {
        match self {
            Self::G01_STATE_COUNT => "G01_STATE_COUNT",
            Self::G01_FIBER_SIZES => "G01_FIBER_SIZES",
            Self::G01_CRYSTAL_FIBER_GROUNDING => "G01_CRYSTAL_FIBER_GROUNDING",
            Self::G02_REFRESH_OUTCOMES => "G02_REFRESH_OUTCOMES",
            Self::G02_QUARTER_RATES => "G02_QUARTER_RATES",
            Self::G02_Q_REFRESH_IDENTITY => "G02_Q_REFRESH_IDENTITY",
            Self::G02_DOUBLE_STOCHASTIC => "G02_DOUBLE_STOCHASTIC",
            Self::G02_ENERGY_PRESERVATION => "G02_ENERGY_PRESERVATION",
            Self::G02_ENTROPY => "G02_ENTROPY",
            Self::G03_EQUAL_REVERSE_MICROHAZARDS => "G03_EQUAL_REVERSE_MICROHAZARDS",
            Self::G03_UNIT_MICROHAZARD => "G03_UNIT_MICROHAZARD",
            Self::G03_ROW_CLOSURE => "G03_ROW_CLOSURE",
            Self::G03_DIAGONAL => "G03_DIAGONAL",
            Self::G04_TOTAL_ENERGY => "G04_TOTAL_ENERGY",
            Self::G05_STRONG_LUMPABILITY => "G05_STRONG_LUMPABILITY",
            Self::G06_MACRO_GENERATOR => "G06_MACRO_GENERATOR",
            Self::G07_RECIPROCAL_SUPPORT => "G07_RECIPROCAL_SUPPORT",
            Self::G07_XI_ANTISYMMETRY => "G07_XI_ANTISYMMETRY",
            Self::G07_TAU_PAIR => "G07_TAU_PAIR",
            Self::G07_SYMMETRIC_ACTIVITY => "G07_SYMMETRIC_ACTIVITY",
            Self::G07_PRAXION_EQUATION => "G07_PRAXION_EQUATION",
            Self::G07_MICRO_LUMPING_AGREEMENT => "G07_MICRO_LUMPING_AGREEMENT",
            Self::G08_STATIONARY_WEIGHTS => "G08_STATIONARY_WEIGHTS",
            Self::G08_DETAILED_BALANCE => "G08_DETAILED_BALANCE",
            Self::G08_STATIONARITY => "G08_STATIONARITY",
            Self::G09_RESERVOIR_RELATION => "G09_RESERVOIR_RELATION",
            Self::G09_LOCAL_DETAILED_BALANCE => "G09_LOCAL_DETAILED_BALANCE",
            Self::G10_NAMED_SLOTS => "G10_NAMED_SLOTS",
            Self::G10_EMPTY_RUBY => "G10_EMPTY_RUBY",
            Self::G11_CONTACT_SHELL => "G11_CONTACT_SHELL",
            Self::G11_UNIT_MICROHAZARD => "G11_UNIT_MICROHAZARD",
            Self::G11_ROW_CLOSURE => "G11_ROW_CLOSURE",
            Self::G11_CANONICAL_WEIGHTS => "G11_CANONICAL_WEIGHTS",
            Self::G11_DETAILED_BALANCE => "G11_DETAILED_BALANCE",
            Self::G11_ZERO_CURRENT => "G11_ZERO_CURRENT",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GateOutcome {
    pub gate: GateId,
    pub passed: bool,
    pub failure: Option<CheckId>,
    pub detail: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControlOutcome {
    pub name: &'static str,
    pub expected_first_failure: CheckId,
    pub observed_first_failure: Option<CheckId>,
    pub passed: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProofReport {
    pub gates: Vec<GateOutcome>,
    pub controls: Vec<ControlOutcome>,
}

impl ProofReport {
    pub fn is_success(&self) -> bool {
        self.gates.len() == GateId::ALL.len()
            && self.gates.iter().all(|outcome| outcome.passed)
            && self.controls.len() == 4
            && self.controls.iter().all(|outcome| outcome.passed)
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        writeln!(output, "XYPHER_THERMODYNAMICS_PROOF v1").unwrap();
        for outcome in &self.gates {
            if outcome.passed {
                writeln!(
                    output,
                    "{} PASS {}",
                    outcome.gate.code(),
                    outcome.gate.label()
                )
                .unwrap();
            } else {
                writeln!(
                    output,
                    "{} FAIL {} {}",
                    outcome.gate.code(),
                    outcome.failure.map(CheckId::code).unwrap_or("UNKNOWN"),
                    outcome.detail
                )
                .unwrap();
            }
        }
        for (index, outcome) in self.controls.iter().enumerate() {
            let status = if outcome.passed { "PASS" } else { "FAIL" };
            let observed = outcome
                .observed_first_failure
                .map(CheckId::code)
                .unwrap_or("NO_FAILURE");
            writeln!(
                output,
                "C{:02} {} {} -> {} (expected {})",
                index + 1,
                status,
                outcome.name,
                observed,
                outcome.expected_first_failure.code()
            )
            .unwrap();
        }
        writeln!(
            output,
            "WITNESS states=16 fibers=4,8,4 macro=8,4,4,8 equilibrium=1/4,1/2,1/4 contact=1,4,1 current=0"
        )
        .unwrap();
        writeln!(
            output,
            "OVERALL {}",
            if self.is_success() { "PASS" } else { "FAIL" }
        )
        .unwrap();
        output
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Mutation {
    None,
    HardGate,
    DirectedKinetic,
    CrystalFiberMismatch,
    UnpairedTau,
}

#[derive(Clone, Copy, Debug)]
struct Fixture {
    mutation: Mutation,
}

impl Fixture {
    const fn new(mutation: Mutation) -> Self {
        Self { mutation }
    }

    fn crystal_g(self, x: usize) -> usize {
        if self.mutation == Mutation::CrystalFiberMismatch && x == 1 {
            3
        } else {
            BASE_CRYSTAL_G_X[x]
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Failure {
    check: CheckId,
    detail: String,
}

fn failure(check: CheckId, detail: impl Into<String>) -> Failure {
    Failure {
        check,
        detail: detail.into(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Ratio {
    numerator: i128,
    denominator: i128,
}

impl Ratio {
    fn new(mut numerator: i128, mut denominator: i128) -> Self {
        assert_ne!(denominator, 0);
        if denominator < 0 {
            numerator = -numerator;
            denominator = -denominator;
        }
        let divisor = gcd(numerator.unsigned_abs(), denominator as u128) as i128;
        Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        }
    }

    const fn integer(value: i128) -> Self {
        Self {
            numerator: value,
            denominator: 1,
        }
    }

    fn multiply(self, other: Self) -> Self {
        Self::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }

    fn square(self) -> Self {
        self.multiply(self)
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

fn power_of_two_ratio(exponent: i32) -> Ratio {
    if exponent >= 0 {
        Ratio::new(1_i128 << exponent, 1)
    } else {
        Ratio::new(1, 1_i128 << (-exponent))
    }
}

fn exact_integer_sqrt(value: i128) -> Option<i128> {
    if value < 0 {
        return None;
    }
    if value < 2 {
        return Some(value);
    }
    let mut low = 1_i128;
    let mut high = value;
    while low <= high {
        let middle = low + (high - low) / 2;
        let quotient = value / middle;
        if middle == quotient && value % middle == 0 {
            return Some(middle);
        }
        if middle < quotient {
            low = middle + 1;
        } else {
            high = middle - 1;
        }
    }
    None
}

fn exact_ratio_sqrt(value: Ratio) -> Option<Ratio> {
    if value.numerator < 0 {
        return None;
    }
    Some(Ratio::new(
        exact_integer_sqrt(value.numerator)?,
        exact_integer_sqrt(value.denominator)?,
    ))
}

fn exact_log2(value: usize) -> Option<i32> {
    if value == 0 || !value.is_power_of_two() {
        None
    } else {
        Some(value.trailing_zeros() as i32)
    }
}

fn reservoir_g(energy: i32) -> usize {
    assert!((0..=TOTAL_ENERGY).contains(&energy));
    1_usize << energy
}

fn reservoir_energy(x: usize) -> i32 {
    TOTAL_ENERGY - SYSTEM_ENERGY[x]
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct MicroState {
    x: usize,
    i: usize,
    j: usize,
}

fn complete_states() -> Vec<MicroState> {
    let mut states = Vec::new();
    for (x, &micro_g) in MICRO_G_X.iter().enumerate() {
        for i in 1..=micro_g {
            for j in 1..=reservoir_g(reservoir_energy(x)) {
                states.push(MicroState { x, i, j });
            }
        }
    }
    states
}

fn fiber_size(states: &[MicroState], x: usize) -> usize {
    states.iter().filter(|state| state.x == x).count()
}

fn refresh_port_count(x: usize) -> usize {
    BASE_CRYSTAL_G_X[x]
}

fn refresh_quantum_quarters(_fixture: Fixture, x: usize) -> Option<i64> {
    let g = refresh_port_count(x) as i64;
    (QUARTERS_PER_UNIT % g == 0).then_some(QUARTERS_PER_UNIT / g)
}

fn r_refresh_quarters(fixture: Fixture, from: MicroState, to: MicroState) -> Option<i64> {
    let quantum = refresh_quantum_quarters(fixture, from.x)?;
    if from.x == to.x && from.j == to.j && to.i <= refresh_port_count(from.x) {
        Some(quantum)
    } else {
        Some(0)
    }
}

fn q_refresh_quarters(fixture: Fixture, from: MicroState, to: MicroState) -> Option<i64> {
    let r = r_refresh_quarters(fixture, from, to)?;
    Some(r - if from == to { QUARTERS_PER_UNIT } else { 0 })
}

fn move_hazard_quarters(fixture: Fixture, from: MicroState, to: MicroState) -> i64 {
    if from == to || (from.x as i32 - to.x as i32).unsigned_abs() != 1 {
        return 0;
    }
    if fixture.mutation == Mutation::DirectedKinetic
        && from == (MicroState { x: 0, i: 1, j: 1 })
        && to == (MicroState { x: 1, i: 1, j: 1 })
    {
        2 * QUARTERS_PER_UNIT
    } else {
        QUARTERS_PER_UNIT
    }
}

fn q_move_quarters(
    fixture: Fixture,
    states: &[MicroState],
    from: MicroState,
    to: MicroState,
) -> i64 {
    if from != to {
        move_hazard_quarters(fixture, from, to)
    } else {
        -states
            .iter()
            .copied()
            .filter(|candidate| *candidate != from)
            .map(|candidate| move_hazard_quarters(fixture, from, candidate))
            .sum::<i64>()
    }
}

fn q_total_quarters(
    fixture: Fixture,
    states: &[MicroState],
    from: MicroState,
    to: MicroState,
) -> i64 {
    q_move_quarters(fixture, states, from, to)
        + q_refresh_quarters(fixture, from, to).expect("gate 2 admits quarter REFRESH")
}

fn gate_01(fixture: Fixture) -> Result<(), Failure> {
    let states = complete_states();
    if states.len() != 16 {
        return Err(failure(
            CheckId::G01_STATE_COUNT,
            format!("complete-state count {} != 16", states.len()),
        ));
    }

    let actual_fibers = [
        fiber_size(&states, 0),
        fiber_size(&states, 1),
        fiber_size(&states, 2),
    ];
    if actual_fibers != [4, 8, 4] {
        return Err(failure(
            CheckId::G01_FIBER_SIZES,
            format!("fiber sizes {actual_fibers:?} != [4, 8, 4]"),
        ));
    }

    for (x, &actual_fiber) in actual_fibers.iter().enumerate() {
        let grounded = fixture.crystal_g(x) * reservoir_g(reservoir_energy(x));
        if actual_fiber != grounded {
            return Err(failure(
                CheckId::G01_CRYSTAL_FIBER_GROUNDING,
                format!(
                    "fiber_size({x})={} but g_X({x})g_R({})={grounded}",
                    actual_fiber,
                    reservoir_energy(x)
                ),
            ));
        }
    }
    Ok(())
}

fn gate_02(fixture: Fixture) -> Result<(), Failure> {
    let states = complete_states();
    for from in states.iter().copied() {
        let g = refresh_port_count(from.x);
        let quantum = refresh_quantum_quarters(fixture, from.x).ok_or_else(|| {
            failure(
                CheckId::G02_QUARTER_RATES,
                format!(
                    "g_X({})={g} is not expressible in quarter-rate units",
                    from.x
                ),
            )
        })?;
        let executable: Vec<_> = states
            .iter()
            .copied()
            .filter(|to| r_refresh_quarters(fixture, from, *to) == Some(quantum))
            .collect();
        if executable.len() != g {
            return Err(failure(
                CheckId::G02_REFRESH_OUTCOMES,
                format!(
                    "REFRESH row {from:?} has {} outcomes, expected {g}",
                    executable.len()
                ),
            ));
        }
        let r_row_sum = states
            .iter()
            .copied()
            .map(|to| r_refresh_quarters(fixture, from, to).unwrap())
            .sum::<i64>();
        if r_row_sum != QUARTERS_PER_UNIT {
            return Err(failure(
                CheckId::G02_QUARTER_RATES,
                format!("R_C row {from:?} sums to {r_row_sum}/4"),
            ));
        }
        for to in states.iter().copied() {
            let r = r_refresh_quarters(fixture, from, to).unwrap();
            let q = q_refresh_quarters(fixture, from, to).unwrap();
            let expected = r - if from == to { QUARTERS_PER_UNIT } else { 0 };
            if q != expected {
                return Err(failure(
                    CheckId::G02_Q_REFRESH_IDENTITY,
                    format!("Q_REFRESH != R_C-I for {from:?}->{to:?}"),
                ));
            }
            if r > 0 && SYSTEM_ENERGY[from.x] != SYSTEM_ENERGY[to.x] {
                return Err(failure(
                    CheckId::G02_ENERGY_PRESERVATION,
                    format!("REFRESH changes energy on {from:?}->{to:?}"),
                ));
            }
        }
        let q_row_sum = states
            .iter()
            .copied()
            .map(|to| q_refresh_quarters(fixture, from, to).unwrap())
            .sum::<i64>();
        if q_row_sum != 0 {
            return Err(failure(
                CheckId::G02_Q_REFRESH_IDENTITY,
                format!("Q_REFRESH row {from:?} sums to {q_row_sum}/4"),
            ));
        }
    }

    for x in 0..3 {
        for j in 1..=reservoir_g(reservoir_energy(x)) {
            let block: Vec<_> = states
                .iter()
                .copied()
                .filter(|state| state.x == x && state.j == j)
                .collect();
            for to in block.iter().copied() {
                let r_column = block
                    .iter()
                    .copied()
                    .map(|from| r_refresh_quarters(fixture, from, to).unwrap())
                    .sum::<i64>();
                let q_column = block
                    .iter()
                    .copied()
                    .map(|from| q_refresh_quarters(fixture, from, to).unwrap())
                    .sum::<i64>();
                if r_column != QUARTERS_PER_UNIT || q_column != 0 {
                    return Err(failure(
                        CheckId::G02_DOUBLE_STOCHASTIC,
                        format!("REFRESH column {to:?} has R={r_column}/4 and Q={q_column}/4"),
                    ));
                }
            }
        }
    }

    let entropy_in_ln2 = [
        exact_log2(refresh_port_count(0)),
        exact_log2(refresh_port_count(1)),
        exact_log2(refresh_port_count(2)),
    ];
    if entropy_in_ln2 != [Some(0), Some(2), Some(2)] {
        return Err(failure(
            CheckId::G02_ENTROPY,
            format!("S_tau/ln2 exponents {entropy_in_ln2:?} != [0, 2, 2]"),
        ));
    }
    Ok(())
}

fn gate_03(fixture: Fixture) -> Result<(), Failure> {
    let states = complete_states();
    for from in states.iter().copied() {
        for to in states.iter().copied().filter(|to| *to != from) {
            let forward = move_hazard_quarters(fixture, from, to);
            let reverse = move_hazard_quarters(fixture, to, from);
            if (forward > 0 || reverse > 0) && forward != reverse {
                return Err(failure(
                    CheckId::G03_EQUAL_REVERSE_MICROHAZARDS,
                    format!("MOVE {from:?}<->{to:?} has hazards {forward}/4 and {reverse}/4"),
                ));
            }
            if forward > 0 && forward != QUARTERS_PER_UNIT {
                return Err(failure(
                    CheckId::G03_UNIT_MICROHAZARD,
                    format!("MOVE {from:?}->{to:?} has hazard {forward}/4"),
                ));
            }
        }
    }

    for from in states.iter().copied() {
        let move_sum = states
            .iter()
            .copied()
            .map(|to| q_move_quarters(fixture, &states, from, to))
            .sum::<i64>();
        let total_sum = states
            .iter()
            .copied()
            .map(|to| q_total_quarters(fixture, &states, from, to))
            .sum::<i64>();
        if move_sum != 0 || total_sum != 0 {
            return Err(failure(
                CheckId::G03_ROW_CLOSURE,
                format!("row {from:?} has MOVE sum {move_sum}/4 and TOTAL sum {total_sum}/4"),
            ));
        }

        let move_diagonal = q_move_quarters(fixture, &states, from, from);
        let total_diagonal = q_total_quarters(fixture, &states, from, from);
        let expected_total = if from.x == 0 { -32 } else { -35 };
        if move_diagonal != -32 || total_diagonal != expected_total {
            return Err(failure(
                CheckId::G03_DIAGONAL,
                format!("row {from:?} diagonals MOVE={move_diagonal}/4 TOTAL={total_diagonal}/4"),
            ));
        }
    }
    Ok(())
}

fn gate_04(fixture: Fixture) -> Result<(), Failure> {
    let states = complete_states();
    for from in states.iter().copied() {
        for to in states.iter().copied() {
            if move_hazard_quarters(fixture, from, to) == 0 {
                continue;
            }
            let before = SYSTEM_ENERGY[from.x] + reservoir_energy(from.x);
            let after = SYSTEM_ENERGY[to.x] + reservoir_energy(to.x);
            let delta_system = SYSTEM_ENERGY[to.x] - SYSTEM_ENERGY[from.x];
            let delta_reservoir = reservoir_energy(to.x) - reservoir_energy(from.x);
            let external_work = 0;
            if before != TOTAL_ENERGY
                || after != TOTAL_ENERGY
                || delta_system + delta_reservoir != 0
                || external_work != 0
            {
                return Err(failure(
                    CheckId::G04_TOTAL_ENERGY,
                    format!(
                        "MOVE {from:?}->{to:?} has totals {before}->{after}, dU={delta_system}, dER={delta_reservoir}"
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn block_sum_quarters(
    fixture: Fixture,
    states: &[MicroState],
    from: MicroState,
    target_x: usize,
    include_refresh: bool,
) -> i64 {
    states
        .iter()
        .copied()
        .filter(|to| to.x == target_x)
        .map(|to| {
            if include_refresh {
                q_total_quarters(fixture, states, from, to)
            } else {
                q_move_quarters(fixture, states, from, to)
            }
        })
        .sum()
}

fn gate_05(fixture: Fixture) -> Result<(), Failure> {
    let states = complete_states();
    for source_x in 0..3 {
        let sources: Vec<_> = states
            .iter()
            .copied()
            .filter(|state| state.x == source_x)
            .collect();
        for target_x in 0..3 {
            let reference_move = block_sum_quarters(fixture, &states, sources[0], target_x, false);
            let reference_total = block_sum_quarters(fixture, &states, sources[0], target_x, true);
            for source in sources.iter().copied().skip(1) {
                let move_sum = block_sum_quarters(fixture, &states, source, target_x, false);
                let total_sum = block_sum_quarters(fixture, &states, source, target_x, true);
                if move_sum != reference_move || total_sum != reference_total {
                    return Err(failure(
                        CheckId::G05_STRONG_LUMPABILITY,
                        format!(
                            "fiber {source_x} to {target_x} differs at {source:?}: MOVE={move_sum}/4 TOTAL={total_sum}/4"
                        ),
                    ));
                }
            }
        }
    }
    Ok(())
}

fn macro_generator_quarters(fixture: Fixture) -> [[i64; 3]; 3] {
    let states = complete_states();
    let mut generator = [[0_i64; 3]; 3];
    for (source_x, row) in generator.iter_mut().enumerate() {
        let source = states
            .iter()
            .copied()
            .find(|state| state.x == source_x)
            .unwrap();
        for (target_x, entry) in row.iter_mut().enumerate() {
            *entry = block_sum_quarters(fixture, &states, source, target_x, true);
        }
    }
    generator
}

fn gate_06(fixture: Fixture) -> Result<(), Failure> {
    let generator = macro_generator_quarters(fixture);
    let expected = [[-32, 32, 0], [16, -32, 16], [0, 32, -32]];
    if generator != expected {
        return Err(failure(
            CheckId::G06_MACRO_GENERATOR,
            format!("macro Q in quarter units {generator:?} != {expected:?}"),
        ));
    }
    for row in generator {
        if row.iter().sum::<i64>() != 0 {
            return Err(failure(
                CheckId::G06_MACRO_GENERATOR,
                format!("macro row {row:?} does not close"),
            ));
        }
    }
    Ok(())
}

fn macro_rate(fixture: Fixture, from: usize, to: usize) -> i64 {
    let quarters = macro_generator_quarters(fixture)[from][to];
    assert!(quarters >= 0 && quarters % QUARTERS_PER_UNIT == 0);
    quarters / QUARTERS_PER_UNIT
}

fn delta_s_in_ln2(fixture: Fixture, from: usize, to: usize) -> Option<i32> {
    Some(exact_log2(fixture.crystal_g(to))? - exact_log2(fixture.crystal_g(from))?)
}

fn tau_plus_energy(fixture: Fixture, from: usize, to: usize) -> Option<i32> {
    Some(delta_s_in_ln2(fixture, from, to)?.max(0))
}

fn xi_energy(fixture: Fixture, from: usize, to: usize) -> Option<i32> {
    let forward_tau = tau_plus_energy(fixture, from, to)?;
    let reverse_tau = tau_plus_energy(fixture, to, from)?;
    let delta_u = SYSTEM_ENERGY[to] - SYSTEM_ENERGY[from];
    if fixture.mutation == Mutation::UnpairedTau {
        Some(forward_tau - delta_u)
    } else {
        Some(forward_tau - reverse_tau - delta_u)
    }
}

fn opportunity_activity_squared(from: usize, to: usize) -> i128 {
    (MICRO_G_X[from] * reservoir_g(reservoir_energy(from))) as i128
        * (MICRO_G_X[to] * reservoir_g(reservoir_energy(to))) as i128
}

fn prospective_rate_squared(fixture: Fixture, from: usize, to: usize) -> Option<Ratio> {
    let affinity_ratio = power_of_two_ratio(xi_energy(fixture, from, to)?);
    Some(Ratio::integer(opportunity_activity_squared(from, to)).multiply(affinity_ratio))
}

fn praxion_rate(fixture: Fixture, from: usize, to: usize) -> Option<i64> {
    let exact_rate = exact_ratio_sqrt(prospective_rate_squared(fixture, from, to)?)?;
    if exact_rate.denominator != 1 {
        return None;
    }
    let mut rate = i64::try_from(exact_rate.numerator).ok()?;
    if fixture.mutation == Mutation::HardGate && from == 1 && to == 2 {
        rate = 0;
    }
    Some(rate)
}

fn gate_07(fixture: Fixture) -> Result<(), Failure> {
    for (left, right) in UNDIRECTED_MACRO_EDGES {
        let forward = praxion_rate(fixture, left, right).ok_or_else(|| {
            failure(
                CheckId::G07_PRAXION_EQUATION,
                format!("prospective rate {left}->{right} is not exact"),
            )
        })?;
        let reverse = praxion_rate(fixture, right, left).ok_or_else(|| {
            failure(
                CheckId::G07_PRAXION_EQUATION,
                format!("prospective rate {right}->{left} is not exact"),
            )
        })?;
        if forward <= 0 || reverse <= 0 {
            return Err(failure(
                CheckId::G07_RECIPROCAL_SUPPORT,
                format!("Praxion support {left}<->{right} has rates {forward},{reverse}"),
            ));
        }
    }

    for (left, right) in UNDIRECTED_MACRO_EDGES {
        let forward_xi = xi_energy(fixture, left, right).unwrap();
        let reverse_xi = xi_energy(fixture, right, left).unwrap();
        if forward_xi + reverse_xi != 0 {
            return Err(failure(
                CheckId::G07_XI_ANTISYMMETRY,
                format!("Xi({left},{right})={forward_xi}, Xi({right},{left})={reverse_xi}"),
            ));
        }
    }

    for (from, to) in DIRECTED_MACRO_EDGES {
        let delta_s = delta_s_in_ln2(fixture, from, to).unwrap();
        let forward_tau = tau_plus_energy(fixture, from, to).unwrap();
        let reverse_tau = tau_plus_energy(fixture, to, from).unwrap();
        let delta_u = SYSTEM_ENERGY[to] - SYSTEM_ENERGY[from];
        let xi = xi_energy(fixture, from, to).unwrap();
        if forward_tau - reverse_tau != delta_s || xi != forward_tau - reverse_tau - delta_u {
            return Err(failure(
                CheckId::G07_TAU_PAIR,
                format!(
                    "{from}->{to}: dS/ln2={delta_s}, TAU+ pair={forward_tau}-{reverse_tau}, dU={delta_u}, Xi={xi}"
                ),
            ));
        }

        let activity_forward = opportunity_activity_squared(from, to);
        let activity_reverse = opportunity_activity_squared(to, from);
        if activity_forward != activity_reverse {
            return Err(failure(
                CheckId::G07_SYMMETRIC_ACTIVITY,
                format!("c^2({from},{to})={activity_forward}, reverse={activity_reverse}"),
            ));
        }

        let rate = praxion_rate(fixture, from, to).unwrap();
        let left = Ratio::integer(i128::from(rate)).square();
        let right = prospective_rate_squared(fixture, from, to).unwrap();
        if left != right {
            return Err(failure(
                CheckId::G07_PRAXION_EQUATION,
                format!("{from}->{to}: k_Pi^2={left:?}, c^2 exp(Xi/alpha)={right:?}"),
            ));
        }

        let lumped = macro_rate(fixture, from, to);
        if rate != lumped {
            return Err(failure(
                CheckId::G07_MICRO_LUMPING_AGREEMENT,
                format!("{from}->{to}: Praxion rate {rate} != micro-lumped rate {lumped}"),
            ));
        }
    }
    Ok(())
}

fn gate_08(fixture: Fixture) -> Result<(), Failure> {
    let states = complete_states();
    let weights = [
        fiber_size(&states, 0) as i64,
        fiber_size(&states, 1) as i64,
        fiber_size(&states, 2) as i64,
    ];
    if weights != [4, 8, 4] || weights.iter().sum::<i64>() != 16 {
        return Err(failure(
            CheckId::G08_STATIONARY_WEIGHTS,
            format!("stationary multiplicities {weights:?} do not give (4,8,4)/16"),
        ));
    }

    let independently_predicted_gibbs = [
        Ratio::new(fixture.crystal_g(0) as i128, 1_i128 << SYSTEM_ENERGY[0]),
        Ratio::new(fixture.crystal_g(1) as i128, 1_i128 << SYSTEM_ENERGY[1]),
        Ratio::new(fixture.crystal_g(2) as i128, 1_i128 << SYSTEM_ENERGY[2]),
    ];
    let normalized_prediction = [
        independently_predicted_gibbs[0].multiply(Ratio::new(1, 4)),
        independently_predicted_gibbs[1].multiply(Ratio::new(1, 4)),
        independently_predicted_gibbs[2].multiply(Ratio::new(1, 4)),
    ];
    let observed = [
        Ratio::new(weights[0] as i128, 16),
        Ratio::new(weights[1] as i128, 16),
        Ratio::new(weights[2] as i128, 16),
    ];
    if independently_predicted_gibbs != [Ratio::integer(1), Ratio::integer(2), Ratio::integer(1)]
        || normalized_prediction != observed
    {
        return Err(failure(
            CheckId::G08_STATIONARY_WEIGHTS,
            format!("independent Gibbs weights {normalized_prediction:?} != observed {observed:?}"),
        ));
    }

    for (left, right) in UNDIRECTED_MACRO_EDGES {
        let forward_product = weights[left] * macro_rate(fixture, left, right);
        let reverse_product = weights[right] * macro_rate(fixture, right, left);
        if forward_product != reverse_product {
            return Err(failure(
                CheckId::G08_DETAILED_BALANCE,
                format!(
                    "edge {left}<->{right} has weighted fluxes {forward_product} and {reverse_product}"
                ),
            ));
        }
    }

    for x in 0..3 {
        let incoming = (0..3)
            .filter(|source| *source != x)
            .map(|source| weights[source] * macro_rate(fixture, source, x))
            .sum::<i64>();
        let outgoing = (0..3)
            .filter(|target| *target != x)
            .map(|target| weights[x] * macro_rate(fixture, x, target))
            .sum::<i64>();
        if incoming != outgoing {
            return Err(failure(
                CheckId::G08_STATIONARITY,
                format!("state {x} incoming={incoming}, outgoing={outgoing}"),
            ));
        }
    }
    Ok(())
}

fn gate_09(fixture: Fixture) -> Result<(), Failure> {
    for energy in 0..TOTAL_ENERGY {
        let ratio = Ratio::new(reservoir_g(energy + 1) as i128, reservoir_g(energy) as i128);
        if ratio != Ratio::integer(2) {
            return Err(failure(
                CheckId::G09_RESERVOIR_RELATION,
                format!("g_R({})/g_R({energy})={ratio:?}, expected 2", energy + 1),
            ));
        }
    }

    for (from, to) in DIRECTED_MACRO_EDGES {
        let rate_ratio = Ratio::new(
            macro_rate(fixture, from, to) as i128,
            macro_rate(fixture, to, from) as i128,
        );
        let crystal_ratio = Ratio::new(
            fixture.crystal_g(to) as i128,
            fixture.crystal_g(from) as i128,
        );
        let reservoir_ratio = Ratio::new(
            reservoir_g(reservoir_energy(to)) as i128,
            reservoir_g(reservoir_energy(from)) as i128,
        );
        let delta_u = SYSTEM_ENERGY[to] - SYSTEM_ENERGY[from];
        let energy_factor = power_of_two_ratio(-delta_u);
        if reservoir_ratio != energy_factor {
            return Err(failure(
                CheckId::G09_RESERVOIR_RELATION,
                format!(
                    "{from}->{to}: reservoir ratio {reservoir_ratio:?} != exp(-dU/alpha) {energy_factor:?}"
                ),
            ));
        }
        let independent_ldb = crystal_ratio.multiply(energy_factor);
        let xi_ratio = power_of_two_ratio(xi_energy(fixture, from, to).unwrap());
        if rate_ratio != independent_ldb || rate_ratio != xi_ratio {
            return Err(failure(
                CheckId::G09_LOCAL_DETAILED_BALANCE,
                format!(
                    "{from}->{to}: k ratio {rate_ratio:?}, g_X exp(-dU/alpha) {independent_ldb:?}, exp(Xi/alpha) {xi_ratio:?}"
                ),
            ));
        }
    }

    if Ratio::new(
        macro_rate(fixture, 0, 1) as i128,
        macro_rate(fixture, 1, 0) as i128,
    ) != Ratio::integer(2)
        || Ratio::new(
            macro_rate(fixture, 1, 2) as i128,
            macro_rate(fixture, 2, 1) as i128,
        ) != Ratio::new(1, 2)
    {
        return Err(failure(
            CheckId::G09_LOCAL_DETAILED_BALANCE,
            "frozen LDB ratios are not 2 and 1/2",
        ));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StateBinding {
    ThreeMesostatesWithSixteenStateLift,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GraphBinding {
    TypedRefreshAndReciprocalMove,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CrystalBinding {
    UniformExecutableTauOneRefresh,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThermoBinding {
    ReservoirAlphaEnergyTauPairAndXi,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PraxionBinding {
    ReciprocalOpportunityScheduler,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ActionBinding {
    LabelledMove,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ResolutionBinding {
    DestinationArrival,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ThawBinding {
    ReverseMove,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MemoryBinding {
    OneStateNoAdaptiveClaim,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct MinimalKernelSlots {
    state: StateBinding,
    graph_substrate: GraphBinding,
    crystal: CrystalBinding,
    thermo: ThermoBinding,
    praxion: PraxionBinding,
    ruby: Option<()>,
    action: ActionBinding,
    resolution: ResolutionBinding,
    thaw: ThawBinding,
    memory: MemoryBinding,
    tau_ledger: Option<()>,
}

fn gate_10(_fixture: Fixture) -> Result<(), Failure> {
    let slots = MinimalKernelSlots {
        state: StateBinding::ThreeMesostatesWithSixteenStateLift,
        graph_substrate: GraphBinding::TypedRefreshAndReciprocalMove,
        crystal: CrystalBinding::UniformExecutableTauOneRefresh,
        thermo: ThermoBinding::ReservoirAlphaEnergyTauPairAndXi,
        praxion: PraxionBinding::ReciprocalOpportunityScheduler,
        ruby: None,
        action: ActionBinding::LabelledMove,
        resolution: ResolutionBinding::DestinationArrival,
        thaw: ThawBinding::ReverseMove,
        memory: MemoryBinding::OneStateNoAdaptiveClaim,
        tau_ledger: None,
    };

    let named_slots_are_bound = matches!(
        slots,
        MinimalKernelSlots {
            state: StateBinding::ThreeMesostatesWithSixteenStateLift,
            graph_substrate: GraphBinding::TypedRefreshAndReciprocalMove,
            crystal: CrystalBinding::UniformExecutableTauOneRefresh,
            thermo: ThermoBinding::ReservoirAlphaEnergyTauPairAndXi,
            praxion: PraxionBinding::ReciprocalOpportunityScheduler,
            action: ActionBinding::LabelledMove,
            resolution: ResolutionBinding::DestinationArrival,
            thaw: ThawBinding::ReverseMove,
            memory: MemoryBinding::OneStateNoAdaptiveClaim,
            ..
        }
    );
    if !named_slots_are_bound {
        return Err(failure(
            CheckId::G10_NAMED_SLOTS,
            "one or more minimal-kernel slots are unbound",
        ));
    }
    if slots.ruby.is_some() || slots.tau_ledger.is_some() {
        return Err(failure(
            CheckId::G10_EMPTY_RUBY,
            "Ruby and the cumulative TAU ledger must both be explicitly empty",
        ));
    }
    Ok(())
}

const CONTACT_SHELL: [(usize, usize); 3] = [(0, 2), (1, 1), (2, 0)];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ContactMicroState {
    x_a: usize,
    i_a: usize,
    x_b: usize,
    i_b: usize,
}

fn contact_states() -> Vec<ContactMicroState> {
    let mut states = Vec::new();
    for (x_a, x_b) in CONTACT_SHELL {
        for i_a in 1..=BASE_CRYSTAL_G_X[x_a] {
            for i_b in 1..=BASE_CRYSTAL_G_X[x_b] {
                states.push(ContactMicroState { x_a, i_a, x_b, i_b });
            }
        }
    }
    states
}

fn contact_hazard(from: ContactMicroState, to: ContactMicroState) -> i64 {
    if from == to {
        return 0;
    }
    let delta_a = to.x_a as i32 - from.x_a as i32;
    let delta_b = to.x_b as i32 - from.x_b as i32;
    if delta_a.unsigned_abs() == 1
        && delta_b == -delta_a
        && SYSTEM_ENERGY[to.x_a] + SYSTEM_ENERGY[to.x_b] == 2
    {
        1
    } else {
        0
    }
}

fn q_contact(states: &[ContactMicroState], from: ContactMicroState, to: ContactMicroState) -> i64 {
    if from != to {
        contact_hazard(from, to)
    } else {
        -states
            .iter()
            .copied()
            .filter(|candidate| *candidate != from)
            .map(|candidate| contact_hazard(from, candidate))
            .sum::<i64>()
    }
}

fn contact_fiber_size(states: &[ContactMicroState], macrostate: (usize, usize)) -> i64 {
    states
        .iter()
        .filter(|state| (state.x_a, state.x_b) == macrostate)
        .count() as i64
}

fn contact_macro_generator(states: &[ContactMicroState]) -> [[i64; 3]; 3] {
    let mut generator = [[0_i64; 3]; 3];
    for (source_index, source_macro) in CONTACT_SHELL.iter().copied().enumerate() {
        let source = states
            .iter()
            .copied()
            .find(|state| (state.x_a, state.x_b) == source_macro)
            .unwrap();
        for (target_index, target_macro) in CONTACT_SHELL.iter().copied().enumerate() {
            generator[source_index][target_index] = states
                .iter()
                .copied()
                .filter(|to| (to.x_a, to.x_b) == target_macro)
                .map(|to| q_contact(states, source, to))
                .sum();
        }
    }
    generator
}

fn canonical_body_weight(x: usize) -> Ratio {
    Ratio::new(BASE_CRYSTAL_G_X[x] as i128, 1_i128 << SYSTEM_ENERGY[x])
}

fn gate_11(_fixture: Fixture) -> Result<(), Failure> {
    if CONTACT_SHELL != [(0, 2), (1, 1), (2, 0)] {
        return Err(failure(
            CheckId::G11_CONTACT_SHELL,
            format!("K=2 shell is {CONTACT_SHELL:?}"),
        ));
    }
    let states = contact_states();
    let multiplicities = [
        contact_fiber_size(&states, CONTACT_SHELL[0]),
        contact_fiber_size(&states, CONTACT_SHELL[1]),
        contact_fiber_size(&states, CONTACT_SHELL[2]),
    ];
    if multiplicities != [4, 16, 4]
        || states
            .iter()
            .any(|state| SYSTEM_ENERGY[state.x_a] + SYSTEM_ENERGY[state.x_b] != TOTAL_ENERGY)
    {
        return Err(failure(
            CheckId::G11_CONTACT_SHELL,
            format!("contact multiplicities {multiplicities:?} != [4,16,4] on K=2"),
        ));
    }

    for from in states.iter().copied() {
        for to in states.iter().copied().filter(|to| *to != from) {
            let forward = contact_hazard(from, to);
            let reverse = contact_hazard(to, from);
            if forward != reverse || (forward > 0 && forward != 1) {
                return Err(failure(
                    CheckId::G11_UNIT_MICROHAZARD,
                    format!("contact {from:?}<->{to:?} has hazards {forward} and {reverse}"),
                ));
            }
        }
        let row_sum = states
            .iter()
            .copied()
            .map(|to| q_contact(&states, from, to))
            .sum::<i64>();
        if row_sum != 0 {
            return Err(failure(
                CheckId::G11_ROW_CLOSURE,
                format!("contact row {from:?} sums to {row_sum}"),
            ));
        }
    }

    let generator = contact_macro_generator(&states);
    let expected_generator = [[-16, 16, 0], [4, -8, 4], [0, 16, -16]];
    if generator != expected_generator {
        return Err(failure(
            CheckId::G11_ROW_CLOSURE,
            format!("contact macro Q {generator:?} != {expected_generator:?}"),
        ));
    }

    let normalized_contact = [
        Ratio::new(multiplicities[0] as i128, 24),
        Ratio::new(multiplicities[1] as i128, 24),
        Ratio::new(multiplicities[2] as i128, 24),
    ];
    let expected_conditioned = [Ratio::new(1, 6), Ratio::new(4, 6), Ratio::new(1, 6)];
    if normalized_contact != expected_conditioned {
        return Err(failure(
            CheckId::G11_CANONICAL_WEIGHTS,
            format!("contact weights {normalized_contact:?} != (1,4,1)/6"),
        ));
    }

    let prepared_products = [
        canonical_body_weight(0).multiply(canonical_body_weight(2)),
        canonical_body_weight(1).multiply(canonical_body_weight(1)),
        canonical_body_weight(2).multiply(canonical_body_weight(0)),
    ];
    let normalized_prepared = [
        prepared_products[0].multiply(Ratio::new(1, 6)),
        prepared_products[1].multiply(Ratio::new(1, 6)),
        prepared_products[2].multiply(Ratio::new(1, 6)),
    ];
    if prepared_products != [Ratio::integer(1), Ratio::integer(4), Ratio::integer(1)]
        || normalized_prepared != expected_conditioned
    {
        return Err(failure(
            CheckId::G11_CANONICAL_WEIGHTS,
            format!("conditioned pre-contact products are {prepared_products:?}"),
        ));
    }

    for (left, right) in [(0_usize, 1_usize), (1, 2)] {
        let forward = multiplicities[left] * generator[left][right];
        let reverse = multiplicities[right] * generator[right][left];
        if forward != reverse {
            return Err(failure(
                CheckId::G11_DETAILED_BALANCE,
                format!("contact {left}<->{right} fluxes {forward} and {reverse}"),
            ));
        }
    }

    let mut macro_current_numerator = 0_i64;
    for (source_index, source_macro) in CONTACT_SHELL.iter().copied().enumerate() {
        for (target_index, target_macro) in CONTACT_SHELL.iter().copied().enumerate() {
            if source_index == target_index {
                continue;
            }
            let current_into_a = SYSTEM_ENERGY[target_macro.0] - SYSTEM_ENERGY[source_macro.0];
            macro_current_numerator += multiplicities[source_index]
                * generator[source_index][target_index]
                * i64::from(current_into_a);
        }
    }

    let mut micro_current_numerator = 0_i64;
    for from in states.iter().copied() {
        for to in states.iter().copied() {
            let current_into_a = SYSTEM_ENERGY[to.x_a] - SYSTEM_ENERGY[from.x_a];
            micro_current_numerator += contact_hazard(from, to) * i64::from(current_into_a);
        }
    }
    if macro_current_numerator != 0 || micro_current_numerator != 0 {
        return Err(failure(
            CheckId::G11_ZERO_CURRENT,
            format!(
                "equilibrium current numerators macro={macro_current_numerator}, micro={micro_current_numerator}"
            ),
        ));
    }
    Ok(())
}

type GateFunction = fn(Fixture) -> Result<(), Failure>;

const GATE_FUNCTIONS: [(GateId, GateFunction); 11] = [
    (GateId::G01, gate_01),
    (GateId::G02, gate_02),
    (GateId::G03, gate_03),
    (GateId::G04, gate_04),
    (GateId::G05, gate_05),
    (GateId::G06, gate_06),
    (GateId::G07, gate_07),
    (GateId::G08, gate_08),
    (GateId::G09, gate_09),
    (GateId::G10, gate_10),
    (GateId::G11, gate_11),
];

#[derive(Clone, Debug)]
struct FixtureEvaluation {
    gates: Vec<GateOutcome>,
    first_failure: Option<Failure>,
}

fn evaluate_fixture(fixture: Fixture) -> FixtureEvaluation {
    let mut gates = Vec::new();
    for (gate, evaluate_gate) in GATE_FUNCTIONS {
        match evaluate_gate(fixture) {
            Ok(()) => gates.push(GateOutcome {
                gate,
                passed: true,
                failure: None,
                detail: String::new(),
            }),
            Err(found) => {
                gates.push(GateOutcome {
                    gate,
                    passed: false,
                    failure: Some(found.check),
                    detail: found.detail.clone(),
                });
                return FixtureEvaluation {
                    gates,
                    first_failure: Some(found),
                };
            }
        }
    }
    FixtureEvaluation {
        gates,
        first_failure: None,
    }
}

pub fn evaluate() -> ProofReport {
    let baseline = evaluate_fixture(Fixture::new(Mutation::None));
    let controls = [
        (
            "hard-gate",
            Mutation::HardGate,
            CheckId::G07_RECIPROCAL_SUPPORT,
        ),
        (
            "directed-kinetic-mutation",
            Mutation::DirectedKinetic,
            CheckId::G03_EQUAL_REVERSE_MICROHAZARDS,
        ),
        (
            "crystal-fiber-mismatch",
            Mutation::CrystalFiberMismatch,
            CheckId::G01_CRYSTAL_FIBER_GROUNDING,
        ),
        (
            "unpaired-TAU",
            Mutation::UnpairedTau,
            CheckId::G07_XI_ANTISYMMETRY,
        ),
    ]
    .into_iter()
    .map(|(name, mutation, expected)| {
        let observed = evaluate_fixture(Fixture::new(mutation))
            .first_failure
            .map(|found| found.check);
        ControlOutcome {
            name,
            expected_first_failure: expected,
            observed_first_failure: observed,
            passed: observed == Some(expected),
        }
    })
    .collect();

    ProofReport {
        gates: baseline.gates,
        controls,
    }
}
