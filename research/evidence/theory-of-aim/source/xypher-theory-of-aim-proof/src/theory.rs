//! The frozen theory of boundary section 4.3: the embedded copy of
//! `predictions_cef5.txt` (56 lines, each the world `(N,tau,b_c,b_h,v=V,R=R)`,
//! the agent, and the theory's `E_th` with exactly six decimals), its frozen
//! SHA-256, and its parser. The table is read only by G12, which checks the
//! table itself, and by the hypothesis and world lines of the report; never
//! by a rate or by any gate's decision about a world.

use std::collections::BTreeMap;

use crate::big::Big;
use crate::model::{Agent, WorldSpec};
use crate::rat::Rat;

/// Boundary section 4.3, verbatim: lines joined by `\n` with a trailing
/// `\n`, the bytes of `research/physics/xypher-agency-theory/predictions_cef5.txt`
/// committed at `c06ffaebbf3fad3a07742549f93cb8a369db29c6`.
pub(crate) const THEORY_TABLE: &str = "\
(3,2,5,2,v=1,R=2) LOCAL 1.390775
(3,2,5,2,v=1,R=2) AIM-1 1.408448
(3,2,5,2,v=1,R=2) AIM-2 1.240044
(3,2,5,2,v=4,R=2) LOCAL 1.347621
(3,2,5,2,v=4,R=2) AIM-1 1.380819
(3,2,5,2,v=4,R=2) AIM-2 1.225313
(3,2,5,2,v=16,R=2) LOCAL 1.256795
(3,2,5,2,v=16,R=2) AIM-1 1.315088
(3,2,5,2,v=16,R=2) AIM-2 1.195344
(4,2,5,2,v=1,R=2) LOCAL 1.755343
(4,2,5,2,v=1,R=2) AIM-1 1.789761
(4,2,5,2,v=1,R=2) AIM-2 1.474870
(4,2,5,2,v=4,R=2) LOCAL 1.672729
(4,2,5,2,v=4,R=2) AIM-1 1.737555
(4,2,5,2,v=4,R=2) AIM-2 1.447227
(4,2,5,2,v=16,R=2) LOCAL 1.500032
(4,2,5,2,v=16,R=2) AIM-1 1.614504
(4,2,5,2,v=16,R=2) AIM-2 1.391200
(3,1,5,2,v=1,R=2) LOCAL 1.395191
(3,1,5,2,v=1,R=2) AIM-1 1.417151
(4,1,5,2,v=1,R=2) LOCAL 1.763832
(4,1,5,2,v=1,R=2) AIM-1 1.805372
(5,1,5,2,v=1,R=2) LOCAL 2.112110
(5,1,5,2,v=1,R=2) AIM-1 2.170470
(3,3,5,2,v=1,R=3) LOCAL 1.342026
(3,3,5,2,v=1,R=3) AIM-1 1.358722
(3,3,5,2,v=1,R=3) AIM-2 1.171789
(3,3,5,2,v=1,R=3) AIM-3 1.139421
(3,2,12,3,v=1,R=2) LOCAL 1.445631
(3,2,12,3,v=1,R=2) AIM-1 1.455677
(3,2,12,3,v=1,R=2) AIM-2 1.269867
(3,2,12,3,v=4,R=2) LOCAL 1.430886
(3,2,12,3,v=4,R=2) AIM-1 1.447266
(3,2,12,3,v=4,R=2) AIM-2 1.262955
(3,2,12,3,v=16,R=2) LOCAL 1.384205
(3,2,12,3,v=16,R=2) AIM-1 1.418012
(3,2,12,3,v=16,R=2) AIM-2 1.248591
(4,2,12,3,v=1,R=2) LOCAL 1.873447
(4,2,12,3,v=1,R=2) AIM-1 1.894422
(4,2,12,3,v=1,R=2) AIM-2 1.542982
(4,2,12,3,v=4,R=2) LOCAL 1.844688
(4,2,12,3,v=4,R=2) AIM-1 1.878159
(4,2,12,3,v=4,R=2) AIM-2 1.528385
(4,2,12,3,v=16,R=2) LOCAL 1.754138
(4,2,12,3,v=16,R=2) AIM-1 1.822746
(4,2,12,3,v=16,R=2) AIM-2 1.501549
(3,1,12,3,v=1,R=2) LOCAL 1.446938
(3,1,12,3,v=1,R=2) AIM-1 1.458810
(4,1,12,3,v=1,R=2) LOCAL 1.875998
(4,1,12,3,v=1,R=2) AIM-1 1.900107
(5,1,12,3,v=1,R=2) LOCAL 2.289413
(5,1,12,3,v=1,R=2) AIM-1 2.325015
(3,3,12,3,v=1,R=3) LOCAL 1.416430
(3,3,12,3,v=1,R=3) AIM-1 1.427271
(3,3,12,3,v=1,R=3) AIM-2 1.204813
(3,3,12,3,v=1,R=3) AIM-3 1.166972
";

/// The frozen SHA-256 of `predictions_cef5.txt` (boundary sections 4.3 and
/// 5, G12 check 1).
pub(crate) const FROZEN_THEORY_DIGEST: &str =
    "27afb36675ece665b0b77dffc18d27a6bdfa2a42193d41bd9c26c3b1ac7a9fc5";

/// Decimals of every `E_th` (G12 check 2).
pub(crate) const THEORY_DECIMALS: usize = 6;

/// The first `E_th` of the table and its C09 corruption (section 8).
const FIRST_VALUE: &str = "1.390775";
const CORRUPTED_FIRST_VALUE: &str = "1.390776";

/// The embedded table as the apparatus reads it: verbatim, or under C09
/// with the first `E_th`, 1.390775, written as 1.390776.
pub(crate) fn embedded_table(corrupted: bool) -> String {
    if corrupted {
        THEORY_TABLE.replacen(FIRST_VALUE, CORRUPTED_FIRST_VALUE, 1)
    } else {
        THEORY_TABLE.to_string()
    }
}

/// One parsed line: the world, the agent, and `E_th` as an exact rational.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TheoryEntry {
    pub(crate) world: WorldSpec,
    pub(crate) agent: Agent,
    pub(crate) value: Rat,
}

impl TheoryEntry {
    /// The key `(world, agent)`, rendered canonically.
    pub(crate) fn key(&self) -> String {
        theory_key(&self.world, self.agent)
    }
}

/// The key of a steady state in the table's own notation.
pub(crate) fn theory_key(world: &WorldSpec, agent: Agent) -> String {
    format!("{} {}", world.label(), agent.code())
}

/// A nonempty run of ASCII digits as an integer.
fn digits(text: &str) -> Option<u64> {
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

/// `(N,tau,b_c,b_h,v=V,R=R)`.
fn parse_world(text: &str) -> Result<WorldSpec, String> {
    let inner = text
        .strip_prefix('(')
        .and_then(|rest| rest.strip_suffix(')'))
        .ok_or_else(|| format!("world {text:?} is not parenthesised"))?;
    let fields: Vec<&str> = inner.split(',').collect();
    let [n, tau, b_c, b_h, speed, radius] = fields.as_slice() else {
        return Err(format!("world {text:?} does not have six fields"));
    };
    let number = |field: &str, prefix: &str| {
        field
            .strip_prefix(prefix)
            .and_then(digits)
            .ok_or_else(|| format!("world {text:?}: field {field:?} is not {prefix}<integer>"))
    };
    let world = WorldSpec::new(
        number(n, "")? as usize,
        number(tau, "")? as usize,
        number(b_c, "")?,
        number(b_h, "")?,
        number(speed, "v=")?,
        number(radius, "R=")? as usize,
    );
    if world.label() != text {
        return Err(format!("world {text:?} is not in canonical form"));
    }
    Ok(world)
}

/// `LOCAL` or `AIM-k` with `k >= 1`; the table holds no GLOBAL value.
fn parse_agent(text: &str) -> Result<Agent, String> {
    if text == "LOCAL" {
        return Ok(Agent::Local);
    }
    match text.strip_prefix("AIM-").and_then(digits) {
        Some(k) if k >= 1 && Agent::Aim(k as usize).code() == text => Ok(Agent::Aim(k as usize)),
        _ => Err(format!("agent {text:?} is neither LOCAL nor AIM-k")),
    }
}

/// A positive decimal `a.bbbbbb` with exactly six decimals, as the exact
/// rational `abbbbbb / 10^6`.
pub(crate) fn parse_value(text: &str) -> Result<Rat, String> {
    let (whole, fraction) = text
        .split_once('.')
        .ok_or_else(|| format!("E_th {text:?} has no decimal point"))?;
    if digits(whole).is_none() || fraction.len() != THEORY_DECIMALS || digits(fraction).is_none() {
        return Err(format!(
            "E_th {text:?} is not a decimal with exactly {THEORY_DECIMALS} decimals"
        ));
    }
    let numerator = Big::parse(&format!("{whole}{fraction}"))
        .ok_or_else(|| format!("E_th {text:?} does not parse"))?;
    let value = Rat::new(numerator, Big::from_u64(10).pow(THEORY_DECIMALS as u32));
    if !value.is_positive() {
        return Err(format!("E_th {text:?} is not positive"));
    }
    Ok(value)
}

/// One line: world, agent, and `E_th`, separated by single spaces.
pub(crate) fn parse_line(line: &str) -> Result<TheoryEntry, String> {
    let fields: Vec<&str> = line.split(' ').collect();
    let [world, agent, value] = fields.as_slice() else {
        return Err(format!("line {line:?} does not have three fields"));
    };
    Ok(TheoryEntry {
        world: parse_world(world)?,
        agent: parse_agent(agent)?,
        value: parse_value(value)?,
    })
}

/// Every line of a table that ends in `\n`; `Err` names the first line
/// (1-based) that does not parse.
pub(crate) fn parse_table(text: &str) -> Result<Vec<TheoryEntry>, (usize, String)> {
    let body = text
        .strip_suffix('\n')
        .ok_or_else(|| (0, "the table does not end with a newline".to_string()))?;
    body.split('\n')
        .enumerate()
        .map(|(index, line)| parse_line(line).map_err(|reason| (index + 1, reason)))
        .collect()
}

/// `E_th` of every steady state in the verbatim table, by key; `None` when
/// the table does not parse or holds a key twice.
pub(crate) fn theory_values() -> Option<BTreeMap<String, Rat>> {
    let mut values = BTreeMap::new();
    for entry in parse_table(THEORY_TABLE).ok()? {
        if values.insert(entry.key(), entry.value).is_some() {
            return None;
        }
    }
    Some(values)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::WORLDS;

    #[test]
    fn the_table_parses_into_the_section_3_keys() {
        let entries = parse_table(THEORY_TABLE).expect("the frozen table parses");
        let expected: Vec<String> = WORLDS
            .iter()
            .flat_map(|world| {
                world
                    .agents()
                    .into_iter()
                    .filter(|&agent| agent != Agent::Global)
                    .map(move |agent| theory_key(world, agent))
            })
            .collect();
        let keys: Vec<String> = entries.iter().map(TheoryEntry::key).collect();
        assert_eq!(keys, expected);
        assert_eq!(entries.len(), 56);
        assert_eq!(entries[0].value, Rat::ratio(1_390_775, 1_000_000));
        assert_eq!(entries[55].value, Rat::ratio(1_166_972, 1_000_000));
    }

    #[test]
    fn malformed_lines_and_values_are_rejected() {
        assert!(parse_value("1.39077").is_err());
        assert!(parse_value("1.3907750").is_err());
        assert!(parse_value("0.000000").is_err());
        assert!(parse_value("-1.390775").is_err());
        assert!(parse_value("1,390775").is_err());
        assert!(parse_line("(3,2,5,2,v=1,R=2) GLOBAL 1.000000").is_err());
        assert!(parse_line("(3,2,5,2,v=1,R=2) AIM-0 1.000000").is_err());
        assert!(parse_line("(3,2,5,2,v=01,R=2) LOCAL 1.000000").is_err());
        assert!(parse_line("(3,2,5,2,R=2,v=1) LOCAL 1.000000").is_err());
        assert!(parse_line("(3,2,5,2,v=1,R=2)  LOCAL 1.000000").is_err());
        assert!(parse_table("(3,2,5,2,v=1,R=2) LOCAL 1.390775").is_err());
        let corrupted = embedded_table(true);
        assert_ne!(corrupted, THEORY_TABLE);
        assert_eq!(corrupted.len(), THEORY_TABLE.len());
        assert!(corrupted.starts_with("(3,2,5,2,v=1,R=2) LOCAL 1.390776\n"));
    }
}
