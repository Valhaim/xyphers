use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::env;
use std::fs;

use thaim_core::molecular::{
    bond_energy, parse_smiles, pearson_r, shannon_entropy, spearman_rho, MolGraph,
};

const TAU: u32 = 5;
const WL_ROUNDS: usize = 8;
const BOOTSTRAP_REPS: usize = 5_000;
const PERMUTATION_REPS: usize = 20_000;
const MIN_FAMILY_N: usize = 30;
const SEED: u64 = 0x5859_5048_4552_5638;
const MIN_ASSOCIATION_RHO: f64 = 0.50; // THUMB td-4e6875: frozen before target reveal.
const MIN_FAMILY_RHO: f64 = 0.70; // THUMB td-4e6875: matches the prior exploratory effect.
const MAX_FAMILY_P: f64 = 0.025; // THUMB td-4e6875: frozen before target reveal.
const MATERIAL_DELTA_RHO: f64 = 0.10; // THUMB td-4e6875: frozen before target reveal.
const WEIGHTING_DELTA_RHO: f64 = 0.05; // THUMB td-4e6875: frozen before target reveal.

#[derive(Clone)]
struct Prediction {
    id: String,
    smiles: String,
    family: String,
    graph_signature: String,
    s_weighted: f64,
    s_unweighted: f64,
    atom_count: f64,
    heavy_atom_count: f64,
    shell_counts: [f64; 5],
    ball_counts: [f64; 5],
    endpoint_support_5: f64,
    branch_excess_r5: f64,
    mean_distance: f64,
    p_degree: f64,
    p_weighted_degree: f64,
    molecular_weight: f64,
    wiener_index: f64,
    randic_index: f64,
    zagreb_m1: f64,
    balaban_j: f64,
}

#[derive(Clone)]
struct EvaluationRow {
    prediction: Prediction,
    vbur: f64,
    buried_b5: Option<f64>,
    molecular_volume: Option<f64>,
}

#[derive(Default)]
struct Rejections {
    disconnected: usize,
    charged: usize,
    unsupported_syntax: usize,
    parse: usize,
    phosphorus_count: usize,
    p_c3_contract: usize,
    default_bde: usize,
    exposed: usize,
    duplicate_signature: usize,
}

#[derive(Clone, Copy)]
struct Interval {
    low: f64,
    high: f64,
}

struct CandidateResult {
    rho: f64,
    rho_ci: Interval,
    permutation_p: f64,
    delta_simple: f64,
    delta_simple_ci: Interval,
    incremental_p: f64,
    alkyl_n: usize,
    alkyl_rho: f64,
    alkyl_ci: Interval,
    alkyl_p: f64,
    alkyl_delta_simple: f64,
    alkyl_delta_ci: Interval,
    aryl_n: usize,
    aryl_rho: f64,
    aryl_ci: Interval,
    aryl_p: f64,
    aryl_delta_simple: f64,
    aryl_delta_ci: Interval,
    association_pass: bool,
    incremental_pass: bool,
    replication_pass: bool,
}

struct GraphMetrics {
    shell_counts: [f64; 5],
    ball_counts: [f64; 5],
    branch_excess_r5: f64,
    mean_distance: f64,
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("predict") if args.len() == 5 => predict(&args[2], &args[3], &args[4]),
        Some("evaluate") if args.len() == 5 => evaluate(&args[2], &args[3], &args[4]),
        _ => Err(format!(
            "usage:\n  {} predict STRUCTURES.tsv EXPOSED_SOURCE_DIR PREDICTIONS.tsv\n  {} evaluate PREDICTIONS.tsv TARGETS.tsv REPORT.md",
            args.first().map(String::as_str).unwrap_or("chem-v8-blind"),
            args.first().map(String::as_str).unwrap_or("chem-v8-blind")
        )),
    }
}

fn predict(structures_path: &str, source_dir: &str, output_path: &str) -> Result<(), String> {
    let exposed = exposed_signatures(source_dir)?;
    let (_, rows) = read_tsv(structures_path)?;
    let headers = read_headers(structures_path)?;
    let id_col = column(&headers, "id")?;
    let smiles_col = column(&headers, "smiles")?;
    let mut predictions = Vec::new();
    let mut rejected = Rejections::default();
    let mut ids = BTreeSet::new();

    for row in rows {
        let id = cell(&row, id_col, "id")?.to_string();
        if !ids.insert(id.clone()) {
            return Err(format!("duplicate structure id {id}"));
        }
        let smiles = cell(&row, smiles_col, "smiles")?.to_string();
        if smiles.contains('.') {
            rejected.disconnected += 1;
            continue;
        }
        if has_formal_charge(&smiles) {
            rejected.charged += 1;
            continue;
        }
        if !parser_syntax_supported(&smiles) {
            rejected.unsupported_syntax += 1;
            continue;
        }
        let ligand = match parse_smiles(&smiles) {
            Ok(value) => value,
            Err(_) => {
                rejected.parse += 1;
                continue;
            }
        };
        let phosphorus: Vec<usize> = ligand
            .atoms
            .iter()
            .enumerate()
            .filter_map(|(index, atom)| (atom.symbol == "P").then_some(index))
            .collect();
        if phosphorus.len() != 1 {
            rejected.phosphorus_count += 1;
            continue;
        }
        let p_ligand = phosphorus[0];
        if !is_p_c3_single(&ligand, p_ligand) {
            rejected.p_c3_contract += 1;
            continue;
        }
        let signature = connectivity_signature(&ligand);
        if exposed.contains(&signature) {
            rejected.exposed += 1;
            continue;
        }
        let family = ligand_family(&ligand, p_ligand);
        let metrics = graph_metrics(&ligand, p_ligand);
        let (complex, p_complex) = build_complex(&ligand, p_ligand);
        if uses_default_bde(&ligand) {
            rejected.default_bde += 1;
            continue;
        }
        let weighted_dist = endpoint_distribution(&complex, p_complex, TAU, true);
        let unweighted_dist = endpoint_distribution(&complex, p_complex, TAU, false);
        predictions.push(Prediction {
            id,
            smiles,
            family,
            graph_signature: signature,
            s_weighted: shannon_entropy(&weighted_dist),
            s_unweighted: shannon_entropy(&unweighted_dist),
            atom_count: ligand.n() as f64,
            heavy_atom_count: ligand
                .atoms
                .iter()
                .filter(|atom| atom.symbol != "H")
                .count() as f64,
            shell_counts: metrics.shell_counts,
            ball_counts: metrics.ball_counts,
            endpoint_support_5: endpoint_distribution(&ligand, p_ligand, TAU, false)
                .iter()
                .filter(|&&p| p > 1e-12)
                .count() as f64,
            branch_excess_r5: metrics.branch_excess_r5,
            mean_distance: metrics.mean_distance,
            p_degree: ligand.degree(p_ligand) as f64,
            p_weighted_degree: weighted_degree(&ligand, p_ligand),
            molecular_weight: ligand.molecular_weight(),
            wiener_index: ligand.wiener_index(),
            randic_index: ligand.randic_index(),
            zagreb_m1: ligand.zagreb_m1(),
            balaban_j: ligand.balaban_j(),
        });
    }

    predictions.sort_by(|a, b| a.id.cmp(&b.id));
    let mut signatures = BTreeSet::new();
    predictions.retain(|prediction| {
        let keep = signatures.insert(prediction.graph_signature.clone());
        if !keep {
            rejected.duplicate_signature += 1;
        }
        keep
    });
    write_predictions(output_path, &predictions)?;
    let mut family_counts = BTreeMap::<String, usize>::new();
    for prediction in &predictions {
        *family_counts.entry(prediction.family.clone()).or_default() += 1;
    }
    println!(
        "source_rows\t{}",
        predictions.len()
            + rejected.disconnected
            + rejected.charged
            + rejected.unsupported_syntax
            + rejected.parse
            + rejected.phosphorus_count
            + rejected.p_c3_contract
            + rejected.default_bde
            + rejected.exposed
            + rejected.duplicate_signature
    );
    println!("predicted_rows\t{}", predictions.len());
    println!("rejected_disconnected\t{}", rejected.disconnected);
    println!("rejected_charged\t{}", rejected.charged);
    println!(
        "rejected_unsupported_syntax\t{}",
        rejected.unsupported_syntax
    );
    println!("rejected_parse\t{}", rejected.parse);
    println!("rejected_phosphorus_count\t{}", rejected.phosphorus_count);
    println!("rejected_p_c3_contract\t{}", rejected.p_c3_contract);
    println!("rejected_default_bde\t{}", rejected.default_bde);
    println!("rejected_exposed\t{}", rejected.exposed);
    println!(
        "rejected_duplicate_signature\t{}",
        rejected.duplicate_signature
    );
    for (family, count) in family_counts {
        println!("family_{}\t{}", family, count);
    }
    Ok(())
}

fn evaluate(predictions_path: &str, targets_path: &str, report_path: &str) -> Result<(), String> {
    let predictions = read_predictions(predictions_path)?;
    let prediction_count = predictions.len();
    let headers = read_headers(targets_path)?;
    let (_, target_rows) = read_tsv(targets_path)?;
    let id_col = column(&headers, "id")?;
    let vbur_col = column(&headers, "vbur_vbur_boltz")?;
    let b5_col = column(&headers, "sterimol_burB5_boltz")?;
    let volume_col = column(&headers, "volume_boltz")?;
    let mut targets = BTreeMap::<String, (f64, Option<f64>, Option<f64>)>::new();
    for row in target_rows {
        let id = cell(&row, id_col, "id")?.to_string();
        let Ok(vbur) = parse_number(cell(&row, vbur_col, "vbur_vbur_boltz")?) else {
            continue;
        };
        if vbur.is_finite() {
            let b5 = optional_number(cell(&row, b5_col, "sterimol_burB5_boltz")?);
            let volume = optional_number(cell(&row, volume_col, "volume_boltz")?);
            if targets.insert(id.clone(), (vbur, b5, volume)).is_some() {
                return Err(format!("duplicate target id {id}"));
            }
        }
    }
    let mut rows = Vec::new();
    for prediction in predictions {
        if let Some(&(vbur, buried_b5, molecular_volume)) = targets.get(&prediction.id) {
            rows.push(EvaluationRow {
                prediction,
                vbur,
                buried_b5,
                molecular_volume,
            });
        }
    }
    if rows.len() != prediction_count {
        return Err(format!(
            "primary target coverage mismatch: {} predictions, {} finite Vbur matches",
            prediction_count,
            rows.len()
        ));
    }

    let vbur: Vec<f64> = rows.iter().map(|row| row.vbur).collect();
    let weighted: Vec<f64> = rows.iter().map(|row| row.prediction.s_weighted).collect();
    let unweighted: Vec<f64> = rows.iter().map(|row| row.prediction.s_unweighted).collect();
    let baselines = simple_baselines(&rows);
    let weighted_result = candidate_result(&rows, &weighted, &vbur, &baselines, SEED)?;
    let unweighted_result = candidate_result(&rows, &unweighted, &vbur, &baselines, SEED ^ 0x1111)?;
    let weighting_delta = spearman_rho(&weighted, &vbur) - spearman_rho(&unweighted, &vbur);
    let weighting_delta_ci =
        bootstrap_delta(&weighted, &unweighted, &vbur, BOOTSTRAP_REPS, SEED ^ 0x2222);
    let unweighted_graph_pass = unweighted_result.association_pass
        && unweighted_result.incremental_pass
        && unweighted_result.replication_pass;
    let weighted_graph_pass = weighted_result.association_pass
        && weighted_result.incremental_pass
        && weighted_result.replication_pass;
    let weighting_pass = unweighted_graph_pass
        && weighting_delta >= WEIGHTING_DELTA_RHO
        && weighting_delta_ci.low > 0.0;

    let mut report = String::new();
    report.push_str("# CHEM-V8 blinded Kraken result\n\n");
    report.push_str(&format!(
        "Evaluable ligands: **{}**. Horizon: **τ={}**.\n\n",
        rows.len(),
        TAU
    ));
    report.push_str("## Primary target: Boltzmann-average buried volume\n\n");
    report.push_str("| Predictor | Pearson r | tie-aware Spearman ρ |\n|---|---:|---:|\n");
    for (name, values) in predictor_table(&rows) {
        report.push_str(&format!(
            "| {} | {:+.4} | {:+.4} |\n",
            name,
            pearson_r(&values, &vbur),
            spearman_rho(&values, &vbur)
        ));
    }
    report.push_str(&format!(
        "| DFT molecular volume (positive control) | {:+.4} | {:+.4} |\n",
        optional_correlation(&rows, |row| row.molecular_volume, false),
        optional_correlation(&rows, |row| row.molecular_volume, true)
    ));
    report.push_str(&format!(
        "| Buried Sterimol B5 (positive control) | {:+.4} | {:+.4} |\n",
        optional_correlation(&rows, |row| row.buried_b5, false),
        optional_correlation(&rows, |row| row.buried_b5, true)
    ));
    report.push_str("\n");
    append_candidate(&mut report, "Unweighted Sτ(P)", &unweighted_result);
    append_candidate(&mut report, "BDE-weighted Sτ(P)", &weighted_result);
    report.push_str("## Physical-weighting test\n\n");
    report.push_str(&format!("Weighted minus unweighted ρ: **{:+.4}**, 95% paired-bootstrap CI **[{:+.4}, {:+.4}]**. Preregistered weighting advantage: **{}**.\n\n", weighting_delta, weighting_delta_ci.low, weighting_delta_ci.high, pass_word(weighting_pass)));
    report.push_str("## Secondary target: Boltzmann-average buried Sterimol B5\n\n");
    let (weighted_b5, unweighted_b5, buried_b5) = secondary_vectors(&rows);
    report.push_str(&format!(
        "Complete cases: n={}. Weighted Sτ(P): r={:+.4}, ρ={:+.4}. Unweighted Sτ(P): r={:+.4}, ρ={:+.4}.\n\n",
        buried_b5.len(),
        pearson_r(&weighted_b5, &buried_b5),
        spearman_rho(&weighted_b5, &buried_b5),
        pearson_r(&unweighted_b5, &buried_b5),
        spearman_rho(&unweighted_b5, &buried_b5)
    ));
    report.push_str("## Frozen decision\n\n");
    report.push_str(&format!(
        "- Primary unweighted topology-descriptor claim: **{}**.\n",
        pass_word(unweighted_graph_pass)
    ));
    report.push_str(&format!(
        "- Secondary weighted graph-descriptor claim: **{}**.\n",
        pass_word(weighted_graph_pass)
    ));
    report.push_str(&format!(
        "- BDE weighting adds physical information: **{}**.\n",
        pass_word(weighting_pass)
    ));
    report.push_str("\nPassing the graph claim requires ρ≥0.50 with permutation p≤0.001, an advantage of at least 0.10 over the strongest preregistered simple topology baseline with bootstrap lower bound above zero, and ρ≥0.70 with p≤0.025 plus the same baseline margin in both P–direct-saturated3 and P–direct-aromatic3 families (n≥30 each).\n");
    fs::write(report_path, &report).map_err(|error| format!("write {report_path}: {error}"))?;
    print!("{report}");
    Ok(())
}

fn append_candidate(report: &mut String, name: &str, result: &CandidateResult) {
    report.push_str(&format!("### {}\n\n", name));
    report.push_str(&format!("ρ={:+.4}, 95% CI [{:+.4}, {:+.4}], permutation p={:.6}. Association threshold: **{}**.\n\n", result.rho, result.rho_ci.low, result.rho_ci.high, result.permutation_p, pass_word(result.association_pass)));
    report.push_str(&format!("Advantage over the bootstrap-strongest simple baseline: Δρ={:+.4}, 95% CI [{:+.4}, {:+.4}], family-stratified incremental permutation p={:.6}. Incremental threshold: **{}**.\n\n", result.delta_simple, result.delta_simple_ci.low, result.delta_simple_ci.high, result.incremental_p, pass_word(result.incremental_pass)));
    report.push_str(&format!("P–direct-saturated3: n={}, ρ={:+.4}, CI [{:+.4}, {:+.4}], p={:.6}. P–direct-aromatic3: n={}, ρ={:+.4}, CI [{:+.4}, {:+.4}], p={:.6}. Family replication: **{}**.\n\n", result.alkyl_n, result.alkyl_rho, result.alkyl_ci.low, result.alkyl_ci.high, result.alkyl_p, result.aryl_n, result.aryl_rho, result.aryl_ci.low, result.aryl_ci.high, result.aryl_p, pass_word(result.replication_pass)));
    report.push_str(&format!("Within-family margin over the strongest simple baseline: alkyl Δρ={:+.4}, CI [{:+.4}, {:+.4}]; aryl Δρ={:+.4}, CI [{:+.4}, {:+.4}].\n\n", result.alkyl_delta_simple, result.alkyl_delta_ci.low, result.alkyl_delta_ci.high, result.aryl_delta_simple, result.aryl_delta_ci.low, result.aryl_delta_ci.high));
}

fn candidate_result(
    rows: &[EvaluationRow],
    candidate: &[f64],
    target: &[f64],
    baselines: &[(String, Vec<f64>)],
    seed: u64,
) -> Result<CandidateResult, String> {
    let rho = spearman_rho(candidate, target);
    let rho_ci = bootstrap_rho(candidate, target, BOOTSTRAP_REPS, seed);
    let permutation_p =
        stratified_permutation_p(rows, candidate, target, PERMUTATION_REPS, seed ^ 0x3333);
    let strongest = baselines
        .iter()
        .map(|(_, values)| spearman_rho(values, target).abs())
        .fold(f64::NEG_INFINITY, f64::max);
    let delta_simple = rho - strongest;
    let baseline_refs: Vec<&[f64]> = baselines
        .iter()
        .map(|(_, values)| values.as_slice())
        .collect();
    let delta_simple_ci = bootstrap_delta_vs_max(
        candidate,
        &baseline_refs,
        target,
        BOOTSTRAP_REPS,
        seed ^ 0x4444,
    );
    let incremental_p = stratified_incremental_permutation_p(
        rows,
        candidate,
        target,
        &baseline_refs,
        PERMUTATION_REPS,
        seed ^ 0xaaaa,
    );
    let alkyl_indices = family_indices(rows, "p_direct_saturated3");
    let aryl_indices = family_indices(rows, "p_direct_aromatic3");
    let alkyl_candidate = select_values(candidate, &alkyl_indices);
    let alkyl_target = select_values(target, &alkyl_indices);
    let aryl_candidate = select_values(candidate, &aryl_indices);
    let aryl_target = select_values(target, &aryl_indices);
    let (alkyl_rho, alkyl_ci, alkyl_p) =
        family_result(&alkyl_candidate, &alkyl_target, seed ^ 0x5555);
    let (aryl_rho, aryl_ci, aryl_p) = family_result(&aryl_candidate, &aryl_target, seed ^ 0x6666);
    let alkyl_baselines: Vec<Vec<f64>> = baselines
        .iter()
        .map(|(_, values)| select_values(values, &alkyl_indices))
        .collect();
    let aryl_baselines: Vec<Vec<f64>> = baselines
        .iter()
        .map(|(_, values)| select_values(values, &aryl_indices))
        .collect();
    let alkyl_delta_simple = delta_vs_strongest(&alkyl_candidate, &alkyl_target, &alkyl_baselines);
    let aryl_delta_simple = delta_vs_strongest(&aryl_candidate, &aryl_target, &aryl_baselines);
    let alkyl_refs: Vec<&[f64]> = alkyl_baselines.iter().map(Vec::as_slice).collect();
    let aryl_refs: Vec<&[f64]> = aryl_baselines.iter().map(Vec::as_slice).collect();
    let alkyl_delta_ci = bootstrap_delta_vs_max(
        &alkyl_candidate,
        &alkyl_refs,
        &alkyl_target,
        BOOTSTRAP_REPS,
        seed ^ 0x8888,
    );
    let aryl_delta_ci = bootstrap_delta_vs_max(
        &aryl_candidate,
        &aryl_refs,
        &aryl_target,
        BOOTSTRAP_REPS,
        seed ^ 0x9999,
    );
    let association_pass = rho >= MIN_ASSOCIATION_RHO && permutation_p <= 0.001;
    let incremental_pass =
        delta_simple >= MATERIAL_DELTA_RHO && delta_simple_ci.low > 0.0 && incremental_p <= 0.001;
    let replication_pass = alkyl_candidate.len() >= MIN_FAMILY_N
        && aryl_candidate.len() >= MIN_FAMILY_N
        && alkyl_rho >= MIN_FAMILY_RHO
        && aryl_rho >= MIN_FAMILY_RHO
        && alkyl_p <= MAX_FAMILY_P
        && aryl_p <= MAX_FAMILY_P
        && alkyl_ci.low > 0.0
        && aryl_ci.low > 0.0
        && alkyl_delta_simple >= MATERIAL_DELTA_RHO
        && aryl_delta_simple >= MATERIAL_DELTA_RHO
        && alkyl_delta_ci.low > 0.0
        && aryl_delta_ci.low > 0.0;
    Ok(CandidateResult {
        rho,
        rho_ci,
        permutation_p,
        delta_simple,
        delta_simple_ci,
        incremental_p,
        alkyl_n: alkyl_candidate.len(),
        alkyl_rho,
        alkyl_ci,
        alkyl_p,
        alkyl_delta_simple,
        alkyl_delta_ci,
        aryl_n: aryl_candidate.len(),
        aryl_rho,
        aryl_ci,
        aryl_p,
        aryl_delta_simple,
        aryl_delta_ci,
        association_pass,
        incremental_pass,
        replication_pass,
    })
}

fn family_indices(rows: &[EvaluationRow], family: &str) -> Vec<usize> {
    rows.iter()
        .enumerate()
        .filter_map(|(index, row)| (row.prediction.family == family).then_some(index))
        .collect()
}

fn select_values(values: &[f64], indices: &[usize]) -> Vec<f64> {
    indices.iter().map(|&index| values[index]).collect()
}

fn delta_vs_strongest(candidate: &[f64], target: &[f64], baselines: &[Vec<f64>]) -> f64 {
    let strongest = baselines
        .iter()
        .map(|values| spearman_rho(values, target).abs())
        .fold(f64::NEG_INFINITY, f64::max);
    spearman_rho(candidate, target) - strongest
}

fn family_result(xs: &[f64], ys: &[f64], seed: u64) -> (f64, Interval, f64) {
    if xs.len() < 3 {
        return (
            0.0,
            Interval {
                low: 0.0,
                high: 0.0,
            },
            1.0,
        );
    }
    (
        spearman_rho(xs, ys),
        bootstrap_rho(xs, ys, BOOTSTRAP_REPS, seed),
        permutation_p(xs, ys, PERMUTATION_REPS, seed ^ 0x7777),
    )
}

fn predictor_table(rows: &[EvaluationRow]) -> Vec<(&'static str, Vec<f64>)> {
    let mut predictors = vec![
        (
            "BDE-weighted Sτ(P), τ=5",
            rows.iter().map(|row| row.prediction.s_weighted).collect(),
        ),
        (
            "Unweighted Sτ(P), τ=5",
            rows.iter().map(|row| row.prediction.s_unweighted).collect(),
        ),
        (
            "All-atom count",
            rows.iter().map(|row| row.prediction.atom_count).collect(),
        ),
        (
            "Heavy-atom count",
            rows.iter()
                .map(|row| row.prediction.heavy_atom_count)
                .collect(),
        ),
        (
            "Molecular weight",
            rows.iter()
                .map(|row| row.prediction.molecular_weight)
                .collect(),
        ),
    ];
    const SHELL_NAMES: [&str; 5] = [
        "Shell-1 count",
        "Shell-2 count",
        "Shell-3 count",
        "Shell-4 count",
        "Shell-5 count",
    ];
    const BALL_NAMES: [&str; 5] = [
        "Ball-1 count",
        "Ball-2 count",
        "Ball-3 count",
        "Ball-4 count",
        "Ball-5 count",
    ];
    for radius in 0..5 {
        predictors.push((
            SHELL_NAMES[radius],
            rows.iter()
                .map(|row| row.prediction.shell_counts[radius])
                .collect(),
        ));
    }
    for radius in 0..5 {
        predictors.push((
            BALL_NAMES[radius],
            rows.iter()
                .map(|row| row.prediction.ball_counts[radius])
                .collect(),
        ));
    }
    predictors.extend([
        (
            "Exact-step-5 endpoint support",
            rows.iter()
                .map(|row| row.prediction.endpoint_support_5)
                .collect(),
        ),
        (
            "Radius-5 branch excess",
            rows.iter()
                .map(|row| row.prediction.branch_excess_r5)
                .collect(),
        ),
        (
            "Mean P-to-atom distance",
            rows.iter()
                .map(|row| row.prediction.mean_distance)
                .collect(),
        ),
        (
            "P degree",
            rows.iter().map(|row| row.prediction.p_degree).collect(),
        ),
        (
            "P BDE-weighted degree",
            rows.iter()
                .map(|row| row.prediction.p_weighted_degree)
                .collect(),
        ),
        (
            "Wiener index",
            rows.iter().map(|row| row.prediction.wiener_index).collect(),
        ),
        (
            "Randić index",
            rows.iter().map(|row| row.prediction.randic_index).collect(),
        ),
        (
            "Zagreb M1",
            rows.iter().map(|row| row.prediction.zagreb_m1).collect(),
        ),
        (
            "Balaban J",
            rows.iter().map(|row| row.prediction.balaban_j).collect(),
        ),
    ]);
    predictors
}

fn simple_baselines(rows: &[EvaluationRow]) -> Vec<(String, Vec<f64>)> {
    predictor_table(rows)
        .into_iter()
        .skip(2)
        .map(|(name, values)| (name.to_string(), values))
        .collect()
}

fn graph_metrics(mol: &MolGraph, start: usize) -> GraphMetrics {
    let distances = graph_distances(mol, start);
    let mut shell_counts = [0.0; 5];
    let mut ball_counts = [0.0; 5];
    for &distance in &distances {
        if (1..=5).contains(&distance) {
            shell_counts[distance - 1] += 1.0;
        }
        for radius in 1..=5 {
            if distance >= 1 && distance <= radius {
                ball_counts[radius - 1] += 1.0;
            }
        }
    }
    let branch_excess_r5 = distances
        .iter()
        .enumerate()
        .filter(|&(_, &distance)| distance <= 5)
        .map(|(index, _)| mol.degree(index).saturating_sub(2) as f64)
        .sum();
    let reachable: Vec<usize> = distances
        .iter()
        .copied()
        .filter(|&distance| distance > 0 && distance < usize::MAX)
        .collect();
    let mean_distance = if reachable.is_empty() {
        0.0
    } else {
        reachable.iter().sum::<usize>() as f64 / reachable.len() as f64
    };
    GraphMetrics {
        shell_counts,
        ball_counts,
        branch_excess_r5,
        mean_distance,
    }
}

fn graph_distances(mol: &MolGraph, start: usize) -> Vec<usize> {
    let mut adjacency = vec![Vec::<usize>::new(); mol.n()];
    for bond in &mol.bonds {
        adjacency[bond.a].push(bond.b);
        adjacency[bond.b].push(bond.a);
    }
    let mut distances = vec![usize::MAX; mol.n()];
    distances[start] = 0;
    let mut queue = VecDeque::from([start]);
    while let Some(node) = queue.pop_front() {
        for &neighbor in &adjacency[node] {
            if distances[neighbor] == usize::MAX {
                distances[neighbor] = distances[node] + 1;
                queue.push_back(neighbor);
            }
        }
    }
    distances
}

fn endpoint_distribution(mol: &MolGraph, start: usize, tau: u32, weighted: bool) -> Vec<f64> {
    let mut adjacency = vec![Vec::<(usize, f64)>::new(); mol.n()];
    let mut totals = vec![0.0; mol.n()];
    for bond in &mol.bonds {
        let weight = if weighted {
            bond_energy(
                &mol.atoms[bond.a].symbol,
                &mol.atoms[bond.b].symbol,
                bond.order,
            )
        } else {
            1.0
        };
        adjacency[bond.a].push((bond.b, weight));
        adjacency[bond.b].push((bond.a, weight));
        totals[bond.a] += weight;
        totals[bond.b] += weight;
    }
    let mut distribution = vec![0.0; mol.n()];
    distribution[start] = 1.0;
    for _ in 0..tau {
        let mut next = vec![0.0; mol.n()];
        for node in 0..mol.n() {
            if distribution[node] == 0.0 || totals[node] == 0.0 {
                continue;
            }
            for &(neighbor, weight) in &adjacency[node] {
                next[neighbor] += distribution[node] * weight / totals[node];
            }
        }
        distribution = next;
    }
    distribution
}

fn weighted_degree(mol: &MolGraph, node: usize) -> f64 {
    mol.bonds
        .iter()
        .filter(|bond| bond.a == node || bond.b == node)
        .map(|bond| {
            bond_energy(
                &mol.atoms[bond.a].symbol,
                &mol.atoms[bond.b].symbol,
                bond.order,
            )
        })
        .sum()
}

fn build_complex(ligand: &MolGraph, p_ligand: usize) -> (MolGraph, usize) {
    let mut mol = MolGraph::new();
    let rh = mol.add_heavy("Rh");
    let c_co = mol.add_heavy("C");
    let o_co = mol.add_heavy("O");
    mol.add_bond(rh, c_co, 1);
    mol.add_bond(c_co, o_co, 3);
    let h_lig = mol.add_heavy("H");
    mol.add_bond(rh, h_lig, 1);
    let offset = mol.n();
    for atom in &ligand.atoms {
        mol.add_atom(&atom.symbol, atom.aromatic);
    }
    for bond in &ligand.bonds {
        mol.add_bond(bond.a + offset, bond.b + offset, bond.order);
    }
    let p_complex = p_ligand + offset;
    mol.add_bond(rh, p_complex, 1);
    (mol, p_complex)
}

fn ligand_family(ligand: &MolGraph, p: usize) -> String {
    let neighbors: Vec<usize> = ligand
        .bonds
        .iter()
        .filter_map(|bond| {
            let neighbor = if bond.a == p {
                Some(bond.b)
            } else if bond.b == p {
                Some(bond.a)
            } else {
                None
            }?;
            (ligand.atoms[neighbor].symbol != "H").then_some(neighbor)
        })
        .collect();
    if neighbors.iter().all(|&index| ligand.atoms[index].aromatic) {
        "p_direct_aromatic3".to_string()
    } else if neighbors.iter().all(|&index| {
        !ligand.atoms[index].aromatic
            && ligand
                .bonds
                .iter()
                .filter(|bond| bond.a == index || bond.b == index)
                .all(|bond| bond.order == 1)
    }) {
        "p_direct_saturated3".to_string()
    } else {
        "p_mixed_or_unsaturated3".to_string()
    }
}

fn is_p_c3_single(ligand: &MolGraph, p: usize) -> bool {
    let incident: Vec<_> = ligand
        .bonds
        .iter()
        .filter(|bond| bond.a == p || bond.b == p)
        .collect();
    incident.len() == 3
        && incident.iter().all(|bond| {
            let neighbor = if bond.a == p { bond.b } else { bond.a };
            ligand.atoms[neighbor].symbol == "C" && bond.order == 1
        })
}

fn has_formal_charge(smiles: &str) -> bool {
    let mut bracket_depth = 0usize;
    for character in smiles.chars() {
        match character {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '+' | '-' if bracket_depth > 0 => return true,
            _ => {}
        }
    }
    false
}

fn parser_syntax_supported(smiles: &str) -> bool {
    let characters: Vec<char> = smiles.chars().collect();
    let mut index = 0usize;
    while index < characters.len() {
        if characters[index] != '[' {
            index += 1;
            continue;
        }
        let Some(relative_end) = characters[index + 1..].iter().position(|&c| c == ']') else {
            return false;
        };
        let content: String = characters[index + 1..index + 1 + relative_end]
            .iter()
            .collect();
        let token = content.trim_start_matches(|character: char| character.is_ascii_digit());
        let token_characters: Vec<char> = token.chars().collect();
        if token_characters.is_empty() {
            return false;
        }
        let (symbol, consumed) = if token_characters[0].is_lowercase() {
            if token_characters
                .get(1)
                .is_some_and(|character| character.is_lowercase())
            {
                return false;
            }
            (token_characters[0].to_uppercase().to_string(), 1)
        } else {
            let mut symbol = token_characters[0].to_string();
            let mut consumed = 1;
            if token_characters
                .get(1)
                .is_some_and(|character| character.is_lowercase())
            {
                symbol.push(token_characters[1]);
                consumed = 2;
            }
            (symbol, consumed)
        };
        let explicit_hydrogen = token_characters[consumed..].contains(&'H');
        let implicit_hydrogen_supported = matches!(
            symbol.as_str(),
            "C" | "N" | "O" | "S" | "P" | "F" | "I" | "H" | "Br" | "Cl"
        );
        if explicit_hydrogen && !implicit_hydrogen_supported {
            return false;
        }
        index += relative_end + 2;
    }
    true
}

fn uses_default_bde(mol: &MolGraph) -> bool {
    mol.bonds.iter().any(|bond| {
        let fallback = match bond.order {
            1 => 250.0,
            2 => 500.0,
            3 => 750.0,
            10 => 400.0,
            _ => 250.0,
        };
        bond_energy(
            &mol.atoms[bond.a].symbol,
            &mol.atoms[bond.b].symbol,
            bond.order,
        ) == fallback
    })
}

fn exposed_signatures(source_dir: &str) -> Result<BTreeSet<String>, String> {
    let mut signatures = BTreeSet::new();
    for name in ["tolman50.rs", "tolman_heldout.rs"] {
        let path = format!("{source_dir}/{name}");
        let source = fs::read_to_string(&path).map_err(|error| format!("read {path}: {error}"))?;
        for line in source.lines().filter(|line| line.contains("smiles:")) {
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    let smiles = &line[start + 1..start + 1 + end];
                    if let Ok(mol) = parse_smiles(smiles) {
                        signatures.insert(connectivity_signature(&mol));
                    }
                }
            }
        }
    }
    Ok(signatures)
}

fn connectivity_signature(mol: &MolGraph) -> String {
    let mut labels: Vec<u64> = (0..mol.n())
        .map(|node| {
            stable_hash(format!("{}:{}", mol.atoms[node].symbol, mol.degree(node)).as_bytes())
        })
        .collect();
    let mut adjacency = vec![Vec::<usize>::new(); mol.n()];
    for bond in &mol.bonds {
        adjacency[bond.a].push(bond.b);
        adjacency[bond.b].push(bond.a);
    }
    for _ in 0..WL_ROUNDS {
        let mut next = Vec::with_capacity(mol.n());
        for node in 0..mol.n() {
            let mut neighborhood: Vec<u64> = adjacency[node]
                .iter()
                .map(|&neighbor| labels[neighbor])
                .collect();
            neighborhood.sort_unstable();
            let mut bytes = labels[node].to_le_bytes().to_vec();
            for value in neighborhood {
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            next.push(stable_hash(&bytes));
        }
        labels = next;
    }
    labels.sort_unstable();
    let mut bytes = (mol.n() as u64).to_le_bytes().to_vec();
    bytes.extend_from_slice(&(mol.bonds.len() as u64).to_le_bytes());
    for label in labels {
        bytes.extend_from_slice(&label.to_le_bytes());
    }
    format!("{:016x}", stable_hash(&bytes))
}

fn stable_hash(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for &byte in bytes {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn write_predictions(path: &str, predictions: &[Prediction]) -> Result<(), String> {
    let mut output = String::from("id\tsmiles\tfamily\tgraph_signature\ts_tau_p_weighted\ts_tau_p_unweighted\tatom_count\theavy_atom_count\tshell_1\tshell_2\tshell_3\tshell_4\tshell_5\tball_1\tball_2\tball_3\tball_4\tball_5\tendpoint_support_5\tbranch_excess_r5\tmean_distance\tp_degree\tp_weighted_degree\tmolecular_weight\twiener_index\trandic_index\tzagreb_m1\tbalaban_j\n");
    for p in predictions {
        output.push_str(&format!(
            "{}\t{}\t{}\t{}\t{:.12}\t{:.12}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.12}\t{:.0}\t{:.12}\t{:.12}\t{:.12}\t{:.12}\t{:.12}\t{:.12}\n",
            p.id,
            p.smiles,
            p.family,
            p.graph_signature,
            p.s_weighted,
            p.s_unweighted,
            p.atom_count,
            p.heavy_atom_count,
            p.shell_counts[0], p.shell_counts[1], p.shell_counts[2], p.shell_counts[3], p.shell_counts[4],
            p.ball_counts[0], p.ball_counts[1], p.ball_counts[2], p.ball_counts[3], p.ball_counts[4],
            p.endpoint_support_5,
            p.branch_excess_r5,
            p.mean_distance,
            p.p_degree,
            p.p_weighted_degree,
            p.molecular_weight,
            p.wiener_index,
            p.randic_index,
            p.zagreb_m1,
            p.balaban_j,
        ));
    }
    fs::write(path, output).map_err(|error| format!("write {path}: {error}"))
}

fn read_predictions(path: &str) -> Result<Vec<Prediction>, String> {
    let headers = read_headers(path)?;
    let (_, rows) = read_tsv(path)?;
    let col = |name| column(&headers, name);
    let id = col("id")?;
    let smiles = col("smiles")?;
    let family = col("family")?;
    let graph_signature = col("graph_signature")?;
    let s_weighted = col("s_tau_p_weighted")?;
    let s_unweighted = col("s_tau_p_unweighted")?;
    let atom_count = col("atom_count")?;
    let heavy_atom_count = col("heavy_atom_count")?;
    let shell = [
        col("shell_1")?,
        col("shell_2")?,
        col("shell_3")?,
        col("shell_4")?,
        col("shell_5")?,
    ];
    let ball = [
        col("ball_1")?,
        col("ball_2")?,
        col("ball_3")?,
        col("ball_4")?,
        col("ball_5")?,
    ];
    let endpoint_support_5 = col("endpoint_support_5")?;
    let branch_excess_r5 = col("branch_excess_r5")?;
    let mean_distance = col("mean_distance")?;
    let p_degree = col("p_degree")?;
    let p_weighted_degree = col("p_weighted_degree")?;
    let molecular_weight = col("molecular_weight")?;
    let wiener_index = col("wiener_index")?;
    let randic_index = col("randic_index")?;
    let zagreb_m1 = col("zagreb_m1")?;
    let balaban_j = col("balaban_j")?;
    let predictions: Vec<Prediction> = rows
        .into_iter()
        .map(|row| {
            Ok(Prediction {
                id: cell(&row, id, "id")?.to_string(),
                smiles: cell(&row, smiles, "smiles")?.to_string(),
                family: cell(&row, family, "family")?.to_string(),
                graph_signature: cell(&row, graph_signature, "graph_signature")?.to_string(),
                s_weighted: parse_number(cell(&row, s_weighted, "s_tau_p_weighted")?)?,
                s_unweighted: parse_number(cell(&row, s_unweighted, "s_tau_p_unweighted")?)?,
                atom_count: parse_number(cell(&row, atom_count, "atom_count")?)?,
                heavy_atom_count: parse_number(cell(&row, heavy_atom_count, "heavy_atom_count")?)?,
                shell_counts: parse_array(&row, shell, "shell")?,
                ball_counts: parse_array(&row, ball, "ball")?,
                endpoint_support_5: parse_number(cell(
                    &row,
                    endpoint_support_5,
                    "endpoint_support_5",
                )?)?,
                branch_excess_r5: parse_number(cell(&row, branch_excess_r5, "branch_excess_r5")?)?,
                mean_distance: parse_number(cell(&row, mean_distance, "mean_distance")?)?,
                p_degree: parse_number(cell(&row, p_degree, "p_degree")?)?,
                p_weighted_degree: parse_number(cell(
                    &row,
                    p_weighted_degree,
                    "p_weighted_degree",
                )?)?,
                molecular_weight: parse_number(cell(&row, molecular_weight, "molecular_weight")?)?,
                wiener_index: parse_number(cell(&row, wiener_index, "wiener_index")?)?,
                randic_index: parse_number(cell(&row, randic_index, "randic_index")?)?,
                zagreb_m1: parse_number(cell(&row, zagreb_m1, "zagreb_m1")?)?,
                balaban_j: parse_number(cell(&row, balaban_j, "balaban_j")?)?,
            })
        })
        .collect::<Result<_, String>>()?;
    let mut ids = BTreeSet::new();
    let mut signatures = BTreeSet::new();
    for prediction in &predictions {
        if !ids.insert(prediction.id.clone()) {
            return Err(format!("duplicate prediction id {}", prediction.id));
        }
        if !signatures.insert(prediction.graph_signature.clone()) {
            return Err(format!(
                "duplicate prediction graph signature {}",
                prediction.graph_signature
            ));
        }
    }
    Ok(predictions)
}

fn parse_array(row: &[String], indices: [usize; 5], name: &str) -> Result<[f64; 5], String> {
    let mut values = [0.0; 5];
    for index in 0..5 {
        values[index] = parse_number(cell(row, indices[index], name)?)?;
    }
    Ok(values)
}

fn read_headers(path: &str) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(path).map_err(|error| format!("read {path}: {error}"))?;
    let line = content
        .lines()
        .next()
        .ok_or_else(|| format!("empty TSV: {path}"))?;
    Ok(line.split('\t').map(str::to_string).collect())
}

fn read_tsv(path: &str) -> Result<(Vec<String>, Vec<Vec<String>>), String> {
    let content = fs::read_to_string(path).map_err(|error| format!("read {path}: {error}"))?;
    let mut lines = content.lines();
    let headers: Vec<String> = lines
        .next()
        .ok_or_else(|| format!("empty TSV: {path}"))?
        .split('\t')
        .map(str::to_string)
        .collect();
    let rows = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split('\t').map(str::to_string).collect())
        .collect();
    Ok((headers, rows))
}

fn column(headers: &[String], name: &str) -> Result<usize, String> {
    headers
        .iter()
        .position(|header| header == name)
        .ok_or_else(|| format!("missing column {name}"))
}

fn cell<'a>(row: &'a [String], index: usize, name: &str) -> Result<&'a str, String> {
    row.get(index)
        .map(String::as_str)
        .ok_or_else(|| format!("missing cell in column {name}"))
}

fn parse_number(value: &str) -> Result<f64, String> {
    value
        .parse::<f64>()
        .map_err(|error| format!("invalid number {value:?}: {error}"))
}

fn optional_number(value: &str) -> Option<f64> {
    value
        .parse::<f64>()
        .ok()
        .filter(|number| number.is_finite())
}

fn optional_correlation<F>(rows: &[EvaluationRow], value: F, rank: bool) -> f64
where
    F: Fn(&EvaluationRow) -> Option<f64>,
{
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for row in rows {
        if let Some(number) = value(row) {
            xs.push(number);
            ys.push(row.vbur);
        }
    }
    if rank {
        spearman_rho(&xs, &ys)
    } else {
        pearson_r(&xs, &ys)
    }
}

fn secondary_vectors(rows: &[EvaluationRow]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let mut weighted = Vec::new();
    let mut unweighted = Vec::new();
    let mut target = Vec::new();
    for row in rows {
        if let Some(value) = row.buried_b5 {
            weighted.push(row.prediction.s_weighted);
            unweighted.push(row.prediction.s_unweighted);
            target.push(value);
        }
    }
    (weighted, unweighted, target)
}

fn midranks(values: &[f64]) -> Vec<f64> {
    let mut indexed: Vec<(usize, f64)> = values.iter().copied().enumerate().collect();
    indexed.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let mut ranks = vec![0.0; values.len()];
    let mut start = 0;
    while start < indexed.len() {
        let mut end = start + 1;
        while end < indexed.len() && indexed[end].1 == indexed[start].1 {
            end += 1;
        }
        let rank = (start + 1 + end) as f64 / 2.0;
        for &(index, _) in &indexed[start..end] {
            ranks[index] = rank;
        }
        start = end;
    }
    ranks
}

fn bootstrap_rho(xs: &[f64], ys: &[f64], reps: usize, seed: u64) -> Interval {
    bootstrap_values(reps, xs.len(), seed, |indices| {
        spearman_rho(&sample_values(xs, indices), &sample_values(ys, indices))
    })
}

fn bootstrap_delta(xs_a: &[f64], xs_b: &[f64], ys: &[f64], reps: usize, seed: u64) -> Interval {
    bootstrap_values(reps, ys.len(), seed, |indices| {
        let sampled_y = sample_values(ys, indices);
        spearman_rho(&sample_values(xs_a, indices), &sampled_y)
            - spearman_rho(&sample_values(xs_b, indices), &sampled_y)
    })
}

fn bootstrap_delta_vs_max(
    candidate: &[f64],
    baselines: &[&[f64]],
    target: &[f64],
    reps: usize,
    seed: u64,
) -> Interval {
    bootstrap_values(reps, target.len(), seed, |indices| {
        let sampled_target = sample_values(target, indices);
        let candidate_rho = spearman_rho(&sample_values(candidate, indices), &sampled_target);
        let strongest = baselines
            .iter()
            .map(|values| spearman_rho(&sample_values(values, indices), &sampled_target).abs())
            .fold(f64::NEG_INFINITY, f64::max);
        candidate_rho - strongest
    })
}

fn sample_values(values: &[f64], indices: &[usize]) -> Vec<f64> {
    indices.iter().map(|&index| values[index]).collect()
}

fn bootstrap_values<F: FnMut(&[usize]) -> f64>(
    reps: usize,
    n: usize,
    seed: u64,
    mut statistic: F,
) -> Interval {
    let mut rng = XorShift64::new(seed);
    let mut values = Vec::with_capacity(reps);
    let mut indices = vec![0usize; n];
    for _ in 0..reps {
        for index in &mut indices {
            *index = rng.index(n);
        }
        values.push(statistic(&indices));
    }
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Interval {
        low: values[((reps as f64 * 0.025).floor() as usize).min(reps - 1)],
        high: values[((reps as f64 * 0.975).floor() as usize).min(reps - 1)],
    }
}

fn permutation_p(xs: &[f64], ys: &[f64], reps: usize, seed: u64) -> f64 {
    let rx = midranks(xs);
    let mut ry = midranks(ys);
    let observed = pearson_r(&rx, &ry).abs();
    let mut rng = XorShift64::new(seed);
    let mut extreme = 0usize;
    for _ in 0..reps {
        for index in (1..ry.len()).rev() {
            let swap = rng.index(index + 1);
            ry.swap(index, swap);
        }
        if pearson_r(&rx, &ry).abs() >= observed {
            extreme += 1;
        }
    }
    (extreme + 1) as f64 / (reps + 1) as f64
}

fn stratified_permutation_p(
    rows: &[EvaluationRow],
    xs: &[f64],
    ys: &[f64],
    reps: usize,
    seed: u64,
) -> f64 {
    let rx = midranks(xs);
    let ry = midranks(ys);
    let observed = pearson_r(&rx, &ry).abs();
    let strata = family_strata(rows);
    let mut rng = XorShift64::new(seed);
    let mut extreme = 0usize;
    for _ in 0..reps {
        let permuted = permute_within_strata(&ry, &strata, &mut rng);
        if pearson_r(&rx, &permuted).abs() >= observed {
            extreme += 1;
        }
    }
    (extreme + 1) as f64 / (reps + 1) as f64
}

fn stratified_incremental_permutation_p(
    rows: &[EvaluationRow],
    candidate: &[f64],
    target: &[f64],
    baselines: &[&[f64]],
    reps: usize,
    seed: u64,
) -> f64 {
    let candidate_ranks = midranks(candidate);
    let target_ranks = midranks(target);
    let baseline_ranks: Vec<Vec<f64>> = baselines.iter().map(|values| midranks(values)).collect();
    let observed = pearson_r(&candidate_ranks, &target_ranks)
        - baseline_ranks
            .iter()
            .map(|ranks| pearson_r(ranks, &target_ranks).abs())
            .fold(f64::NEG_INFINITY, f64::max);
    let strata = family_strata(rows);
    let mut rng = XorShift64::new(seed);
    let mut extreme = 0usize;
    for _ in 0..reps {
        let permuted = permute_within_strata(&target_ranks, &strata, &mut rng);
        let delta = pearson_r(&candidate_ranks, &permuted)
            - baseline_ranks
                .iter()
                .map(|ranks| pearson_r(ranks, &permuted).abs())
                .fold(f64::NEG_INFINITY, f64::max);
        if delta >= observed {
            extreme += 1;
        }
    }
    (extreme + 1) as f64 / (reps + 1) as f64
}

fn family_strata(rows: &[EvaluationRow]) -> Vec<Vec<usize>> {
    let mut groups = BTreeMap::<String, Vec<usize>>::new();
    for (index, row) in rows.iter().enumerate() {
        groups
            .entry(row.prediction.family.clone())
            .or_default()
            .push(index);
    }
    groups.into_values().collect()
}

fn permute_within_strata(values: &[f64], strata: &[Vec<usize>], rng: &mut XorShift64) -> Vec<f64> {
    let mut permuted = values.to_vec();
    for indices in strata {
        let mut stratum_values: Vec<f64> = indices.iter().map(|&index| values[index]).collect();
        for index in (1..stratum_values.len()).rev() {
            let swap = rng.index(index + 1);
            stratum_values.swap(index, swap);
        }
        for (&row, value) in indices.iter().zip(stratum_values) {
            permuted[row] = value;
        }
    }
    permuted
}

struct XorShift64 {
    state: u64,
}

impl XorShift64 {
    fn new(seed: u64) -> Self {
        Self { state: seed.max(1) }
    }

    fn next(&mut self) -> u64 {
        let mut value = self.state;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.state = value;
        value
    }

    fn index(&mut self, upper: usize) -> usize {
        (self.next() % upper as u64) as usize
    }
}

fn pass_word(value: bool) -> &'static str {
    if value {
        "PASS"
    } else {
        "FAIL"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparse_entropy_matches_library_entropy() {
        let ligand = parse_smiles("CP(C)C").unwrap();
        let p = ligand
            .atoms
            .iter()
            .position(|atom| atom.symbol == "P")
            .unwrap();
        let (complex, p_complex) = build_complex(&ligand, p);
        for weighted in [false, true] {
            let sparse =
                shannon_entropy(&endpoint_distribution(&complex, p_complex, TAU, weighted));
            let dense = complex.s_tau_node(TAU, p_complex, weighted);
            assert!((sparse - dense).abs() < 1e-12, "{sparse} != {dense}");
        }
    }

    #[test]
    fn connectivity_signature_ignores_aromatic_notation() {
        let aromatic = parse_smiles("c1ccccc1P(c1ccccc1)c1ccccc1").unwrap();
        let kekule = parse_smiles("C1=CC=CC=C1P(C2=CC=CC=C2)C3=CC=CC=C3").unwrap();
        assert_eq!(
            connectivity_signature(&aromatic),
            connectivity_signature(&kekule)
        );
    }

    #[test]
    fn bootstrap_is_deterministic() {
        let xs = [1.0, 2.0, 3.0, 4.0, 5.0];
        let ys = [1.0, 2.0, 4.0, 3.0, 5.0];
        let first = bootstrap_rho(&xs, &ys, 100, SEED);
        let second = bootstrap_rho(&xs, &ys, 100, SEED);
        assert_eq!(first.low, second.low);
        assert_eq!(first.high, second.high);
    }

    #[test]
    fn rejects_bracket_forms_the_parser_cannot_preserve() {
        assert!(!parser_syntax_supported("c1c[se]cc1"));
        assert!(!parser_syntax_supported("[BH]1[BH]"));
        assert!(parser_syntax_supported("C[C@H](N)P(C)C"));
    }

    #[test]
    fn family_gate_does_not_call_kekule_aryl_saturated() {
        let aromatic = parse_smiles("c1ccccc1P(c1ccccc1)c1ccccc1").unwrap();
        let aromatic_p = aromatic
            .atoms
            .iter()
            .position(|atom| atom.symbol == "P")
            .unwrap();
        assert_eq!(ligand_family(&aromatic, aromatic_p), "p_direct_aromatic3");

        let kekule = parse_smiles("C1=CC=CC=C1P(C2=CC=CC=C2)C3=CC=CC=C3").unwrap();
        let kekule_p = kekule
            .atoms
            .iter()
            .position(|atom| atom.symbol == "P")
            .unwrap();
        assert_eq!(ligand_family(&kekule, kekule_p), "p_mixed_or_unsaturated3");

        let saturated = parse_smiles("CP(C)C").unwrap();
        let saturated_p = saturated
            .atoms
            .iter()
            .position(|atom| atom.symbol == "P")
            .unwrap();
        assert_eq!(
            ligand_family(&saturated, saturated_p),
            "p_direct_saturated3"
        );
    }
}
