use crate::exact::{Ratio, Temperature};

pub const BODY_ENERGY_INDICES: [i32; 4] = [0, 0, 1, 3];
pub const TOTAL_ENERGY_INDEX: i32 = 3;
pub const BODY_DEGENERACIES: [[i128; 4]; 2] = [[1, 2, 4, 8], [1, 3, 5, 11]];
pub const RESERVOIR_BASES: [u64; 2] = [3, 5];
pub const ENERGY_QUANTA: [i128; 2] = [1, 3];
pub const CYCLE_EDGES: [(usize, usize); 4] = [(0, 1), (1, 2), (2, 3), (3, 0)];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfirmatoryCase {
    pub body_variant: usize,
    pub reservoir_base: u64,
    pub lambda: Ratio,
    pub degeneracies: [i128; 4],
    pub energy_indices: [i32; 4],
}

impl ConfirmatoryCase {
    pub fn id(&self) -> String {
        format!(
            "b{}-lambda{}-g{}",
            self.reservoir_base,
            self.lambda,
            self.body_variant + 1
        )
    }

    pub fn reservoir_temperature(&self) -> Temperature {
        Temperature::from_base(self.lambda, self.reservoir_base)
    }

    pub fn energy(&self, state: usize) -> Ratio {
        self.lambda * Ratio::integer(self.energy_indices[state] as i128)
    }

    pub fn fiber_size(&self, state: usize) -> i128 {
        let reservoir_index = TOTAL_ENERGY_INDEX - self.energy_indices[state];
        self.degeneracies[state]
            .checked_mul(integer_power(
                self.reservoir_base as i128,
                reservoir_index as u32,
            ))
            .expect("fiber-size overflow")
    }

    pub fn macro_hazard(&self, destination: usize) -> Ratio {
        Ratio::integer(self.fiber_size(destination))
    }
}

pub fn confirmatory_cases() -> Vec<ConfirmatoryCase> {
    let mut cases = Vec::with_capacity(8);
    for reservoir_base in RESERVOIR_BASES {
        for lambda in ENERGY_QUANTA {
            for (body_variant, degeneracies) in BODY_DEGENERACIES.into_iter().enumerate() {
                cases.push(ConfirmatoryCase {
                    body_variant,
                    reservoir_base,
                    lambda: Ratio::integer(lambda),
                    degeneracies,
                    energy_indices: BODY_ENERGY_INDICES,
                });
            }
        }
    }
    cases
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalibrationRow {
    pub reservoir_base: u64,
    pub lambda: Ratio,
    pub degeneracies: [i128; 3],
    pub energy_indices: [i32; 3],
    pub fibers: [i128; 3],
    pub rates: [i128; 4],
    pub evaluation_data: bool,
}

pub fn inherited_calibration() -> CalibrationRow {
    CalibrationRow {
        reservoir_base: 2,
        lambda: Ratio::ONE,
        degeneracies: [1, 4, 4],
        energy_indices: [0, 1, 2],
        fibers: [4, 8, 4],
        rates: [8, 4, 4, 8],
        evaluation_data: false,
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Lane {
    TypedSources,
    Reservoir,
    Channel,
    CrossCase,
    Gauge,
    Component,
    GrossTau,
    SignedTau,
    VariablePrice,
    HeatWork,
    ProtocolPrice,
    EquationOfState,
}

impl Lane {
    pub const fn label(self) -> &'static str {
        match self {
            Self::TypedSources => "typed-sources",
            Self::Reservoir => "reservoir",
            Self::Channel => "channel",
            Self::CrossCase => "cross-case",
            Self::Gauge => "gauge",
            Self::Component => "component",
            Self::GrossTau => "gross-tau",
            Self::SignedTau => "signed-tau",
            Self::VariablePrice => "variable-price",
            Self::HeatWork => "heat-work",
            Self::ProtocolPrice => "protocol-price",
            Self::EquationOfState => "equation-of-state",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlId {
    C01,
    C02,
    C03,
    C04,
    C05,
    C06,
    C07,
    C08,
    C09,
    C10,
    C11,
    C12,
    C13,
    C14,
    C15,
    C16,
    C17,
    C18,
    C19,
}

impl ControlId {
    pub const ALL: [Self; 19] = [
        Self::C01,
        Self::C02,
        Self::C03,
        Self::C04,
        Self::C05,
        Self::C06,
        Self::C07,
        Self::C08,
        Self::C09,
        Self::C10,
        Self::C11,
        Self::C12,
        Self::C13,
        Self::C14,
        Self::C15,
        Self::C16,
        Self::C17,
        Self::C18,
        Self::C19,
    ];

    pub const fn code(self) -> &'static str {
        match self {
            Self::C01 => "C01",
            Self::C02 => "C02",
            Self::C03 => "C03",
            Self::C04 => "C04",
            Self::C05 => "C05",
            Self::C06 => "C06",
            Self::C07 => "C07",
            Self::C08 => "C08",
            Self::C09 => "C09",
            Self::C10 => "C10",
            Self::C11 => "C11",
            Self::C12 => "C12",
            Self::C13 => "C13",
            Self::C14 => "C14",
            Self::C15 => "C15",
            Self::C16 => "C16",
            Self::C17 => "C17",
            Self::C18 => "C18",
            Self::C19 => "C19",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::C01 => "half-reservoir-temperature",
            Self::C02 => "inverse-label",
            Self::C03 => "body-dependent-map",
            Self::C04 => "scale-energy-only",
            Self::C05 => "flat-energy-all",
            Self::C06 => "informative-plus-flat",
            Self::C07 => "mixed-component-baths",
            Self::C08 => "nonlinear-reservoir",
            Self::C09 => "zero-energy-ratio",
            Self::C10 => "inconsistent-slopes",
            Self::C11 => "aggregation-trap",
            Self::C12 => "component-offset-gauge",
            Self::C13 => "per-component-scale",
            Self::C14 => "tau-as-state",
            Self::C15 => "unpaired-tau-xi",
            Self::C16 => "source-price",
            Self::C17 => "heat-work-conflation",
            Self::C18 => "relabeled-gate",
            Self::C19 => "tau-is-equation-of-state",
        }
    }

    pub const fn lane(self) -> Lane {
        match self {
            Self::C01 | Self::C08 => Lane::Reservoir,
            Self::C02 => Lane::TypedSources,
            Self::C03 => Lane::CrossCase,
            Self::C04 | Self::C09 | Self::C10 | Self::C11 => Lane::Channel,
            Self::C05 | Self::C06 | Self::C07 | Self::C13 => Lane::Component,
            Self::C12 => Lane::Gauge,
            Self::C14 => Lane::GrossTau,
            Self::C15 => Lane::SignedTau,
            Self::C16 => Lane::VariablePrice,
            Self::C17 => Lane::HeatWork,
            Self::C18 => Lane::ProtocolPrice,
            Self::C19 => Lane::EquationOfState,
        }
    }
}

pub fn integer_power(mut base: i128, mut exponent: u32) -> i128 {
    let mut result = 1_i128;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = result.checked_mul(base).expect("integer power overflow");
        }
        exponent >>= 1;
        if exponent > 0 {
            base = base.checked_mul(base).expect("integer power overflow");
        }
    }
    result
}
