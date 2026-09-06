//! Weighted Filter Theorem Verification
//!
//! Tests whether the filter theorem (ΔS_τ^local > 0 ⟹ ΔS̄_τ^global increases)
//! holds under non-unit edge weights. The existing `lemma2_verify.rs` only tests
//! unit-weight graphs. This binary samples random weight vectors from Exp(1) and
//! verifies the filter across all connected graphs on N ≤ 5 nodes.
//!
//! Run: cargo run --release --bin weighted-filter-verify

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::Instant;

use clap::Parser;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rand_distr::{Distribution, Exp};

use thaim_core::entropy_exact;
use thaim_core::types::{NodeId, WeightedAdj};

const NUMERICAL_EPS: f64 = 1e-10;

#[derive(Parser)]
#[command(
    name = "weighted-filter-verify",
    about = "Verify the filter theorem under non-unit edge weights"
)]
struct Cli {
    /// Maximum graph size for exhaustive enumeration
    #[arg(long, default_value = "5")]
    max_n: usize,

    /// Number of random weight samples per (graph, non-edge) pair
    #[arg(long, default_value = "50")]
    weight_samples: usize,

    /// Maximum tau horizon
    #[arg(long, default_value = "5")]
    max_tau: u32,

    /// RNG seed
    #[arg(long, default_value = "42")]
    seed: u64,

    /// Fix new edge weight to 1.0 (THAIM protocol regime)
    #[arg(long, default_value = "false")]
    unit_new_edge: bool,
}

/// Build a WeightedAdj from edge list with specified weights.
fn build_weighted_adj(n: usize, edges: &[(usize, usize)], weights: &[f64]) -> WeightedAdj {
    let mut adj: WeightedAdj = BTreeMap::new();
    for i in 0..n {
        adj.insert(i as NodeId, Vec::new());
    }
    for (idx, &(u, v)) in edges.iter().enumerate() {
        let w = weights[idx];
        adj.get_mut(&(u as NodeId)).unwrap().push((v as NodeId, w));
        adj.get_mut(&(v as NodeId)).unwrap().push((u as NodeId, w));
    }
    adj
}

/// Check if the graph on n nodes with given edges is connected.
fn is_connected(n: usize, edges: &[(usize, usize)]) -> bool {
    if n == 0 {
        return true;
    }
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(u, v) in edges {
        adj_list[u].push(v);
        adj_list[v].push(u);
    }
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[0] = true;
    queue.push_back(0);
    let mut count = 1usize;
    while let Some(node) = queue.pop_front() {
        for &nbr in &adj_list[node] {
            if !visited[nbr] {
                visited[nbr] = true;
                count += 1;
                queue.push_back(nbr);
            }
        }
    }
    count == n
}

/// All possible edges for n nodes.
fn all_possible_edges(n: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((i, j));
        }
    }
    edges
}

/// A single violation record.
#[derive(Debug)]
struct Violation {
    n: usize,
    edges: Vec<(usize, usize)>,
    edge_weights: Vec<f64>,
    added_edge: (usize, usize),
    new_edge_weight: f64,
    tau: u32,
    local_delta: f64,
    global_delta: f64,
}

fn main() {
    let cli = Cli::parse();
    let mut rng = StdRng::seed_from_u64(cli.seed);
    let exp_dist = Exp::new(1.0).unwrap();

    println!("\n{:=<90}", "");
    println!("  WEIGHTED FILTER THEOREM VERIFICATION");
    println!("  ΔS_τ^local > 0 ⟹ ΔS̄_τ^global increases?");
    println!("  Weight distribution: Exp(1)");
    println!(
        "  Weight samples per (graph, non-edge): {}",
        cli.weight_samples
    );
    println!(
        "  New edge weight: {}",
        if cli.unit_new_edge {
            "1.0 (THAIM protocol)"
        } else {
            "Exp(1) random"
        }
    );
    println!("  τ range: 1..{}", cli.max_tau);
    println!("  Numerical tolerance: ε = {:.0e}", NUMERICAL_EPS);
    println!("{:=<90}", "");

    let mut grand_total_checks = 0u64;
    let mut grand_filter_violations = 0u64; // ΔS_local > 0 but ΔS_global < 0
    let mut grand_global_violations = 0u64; // ΔS_global < 0 (any)
    let mut all_violations: Vec<Violation> = Vec::new();
    let mut grand_connected = 0u64;

    println!(
        "\n  {:>3} {:>10} {:>14} {:>14} {:>14} {:>8}",
        "N", "connected", "checks", "global↓", "filter viol", "time"
    );

    for n in 3..=cli.max_n {
        let start = Instant::now();
        let possible = all_possible_edges(n);
        let m = possible.len();
        let total_graphs = 1u64 << m;

        let mut n_connected = 0u64;
        let mut n_checks = 0u64;
        let mut n_global_violations = 0u64;
        let mut n_filter_violations = 0u64;

        for mask in 0..total_graphs {
            let edges: Vec<(usize, usize)> = (0..m)
                .filter(|&bit| mask & (1u64 << bit) != 0)
                .map(|bit| possible[bit])
                .collect();

            if !is_connected(n, &edges) {
                continue;
            }
            n_connected += 1;

            // Find non-edges
            let edge_set: BTreeSet<(usize, usize)> = edges.iter().copied().collect();
            let non_edges: Vec<(usize, usize)> = possible
                .iter()
                .filter(|e| !edge_set.contains(e))
                .copied()
                .collect();

            if non_edges.is_empty() {
                continue;
            }

            // For each non-edge, sample random weights
            for &(u, v) in &non_edges {
                for _sample in 0..cli.weight_samples {
                    // Sample random weights for existing edges
                    let edge_weights: Vec<f64> = (0..edges.len())
                        .map(|_| exp_dist.sample(&mut rng))
                        .collect();
                    // Sample weight for new edge (or fix to 1.0 in THAIM regime)
                    let new_weight: f64 = if cli.unit_new_edge {
                        1.0
                    } else {
                        exp_dist.sample(&mut rng)
                    };

                    for tau in 1..=cli.max_tau {
                        n_checks += 1;

                        // Build before graph with random weights
                        let adj_before = build_weighted_adj(n, &edges, &edge_weights);

                        // Build after graph: add edge (u,v) with new_weight
                        let mut edges_after = edges.clone();
                        edges_after.push((u, v));
                        let mut weights_after = edge_weights.clone();
                        weights_after.push(new_weight);
                        let adj_after = build_weighted_adj(n, &edges_after, &weights_after);

                        // Compute global entropy before and after
                        let result_before =
                            entropy_exact::compute_all_entropies_exact(&adj_before, tau);
                        let result_after =
                            entropy_exact::compute_all_entropies_exact(&adj_after, tau);
                        let global_delta = result_after.global - result_before.global;

                        // Compute local delta (affected nodes = u, v, and their neighbors)
                        let affected = get_affected_nodes(&adj_before, u as NodeId, v as NodeId);
                        let mut before_sum = 0.0;
                        let mut after_sum = 0.0;
                        for &node_id in &affected {
                            before_sum +=
                                result_before.per_node.get(&node_id).copied().unwrap_or(0.0);
                            after_sum +=
                                result_after.per_node.get(&node_id).copied().unwrap_or(0.0);
                        }
                        let local_delta = (after_sum - before_sum) / affected.len() as f64;

                        // Check filter: local > 0 but global < 0?
                        if global_delta < -NUMERICAL_EPS {
                            n_global_violations += 1;

                            if local_delta > NUMERICAL_EPS {
                                n_filter_violations += 1;
                                all_violations.push(Violation {
                                    n,
                                    edges: edges.clone(),
                                    edge_weights: edge_weights.clone(),
                                    added_edge: (u, v),
                                    new_edge_weight: new_weight,
                                    tau,
                                    local_delta,
                                    global_delta,
                                });
                            }
                        }
                    }
                }
            }
        }

        let elapsed = start.elapsed().as_secs_f64();
        grand_total_checks += n_checks;
        grand_connected += n_connected;
        grand_global_violations += n_global_violations;
        grand_filter_violations += n_filter_violations;

        println!(
            "  {:>3} {:>10} {:>14} {:>14} {:>14} {:>7.1}s",
            n, n_connected, n_checks, n_global_violations, n_filter_violations, elapsed
        );
    }

    // ── Report violations ──
    if !all_violations.is_empty() {
        println!("\n{:-<90}", "");
        println!("  FILTER VIOLATIONS (ΔS_local > 0 but ΔS_global < 0)");
        println!("{:-<90}", "");
        for (i, v) in all_violations.iter().enumerate().take(20) {
            println!(
                "  #{}: N={}, added=({},{}), τ={}, w_new={:.3}, Δlocal={:.6}, Δglobal={:.2e}",
                i + 1,
                v.n,
                v.added_edge.0,
                v.added_edge.1,
                v.tau,
                v.new_edge_weight,
                v.local_delta,
                v.global_delta
            );
            println!(
                "       edges={:?}",
                v.edges
                    .iter()
                    .zip(&v.edge_weights)
                    .map(|(&(a, b), &w)| format!("({},{}):{:.2}", a, b, w))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
        if all_violations.len() > 20 {
            println!("  ... and {} more", all_violations.len() - 20);
        }
    }

    // ── Also check: how many global violations have local > 0 in unit-weight baseline? ──
    // (This was already verified by lemma2_verify, but good cross-check)

    // ── Summary ──
    println!("\n{:=<90}", "");
    println!("  SUMMARY");
    println!("{:=<90}", "");
    println!("\n  Connected graphs tested:    {:>14}", grand_connected);
    println!(
        "  Total (graph × edge × weight × τ) checks: {:>14}",
        grand_total_checks
    );
    println!(
        "  Global entropy decreased:   {:>14}",
        grand_global_violations
    );
    println!(
        "  Filter violations:          {:>14}  ← ΔS_local > 0 AND ΔS_global < 0",
        grand_filter_violations
    );

    if grand_filter_violations == 0 {
        println!("\n  FILTER THEOREM HOLDS UNDER WEIGHTED EDGES.");
        println!(
            "  Across {} weight samples per configuration,",
            cli.weight_samples
        );
        println!("  NO edge with ΔS_τ^local > 0 ever decreased global entropy.");
        println!("  The intelligence criterion remains a perfect alignment filter");
        println!("  for heterogeneous-weight graphs on N ≤ {}.", cli.max_n);
    } else {
        println!("\n  FILTER THEOREM VIOLATED UNDER WEIGHTED EDGES.");
        println!(
            "  {} violations found: edges with ΔS_τ^local > 0 that decreased",
            grand_filter_violations
        );
        println!("  global entropy. The filter theorem does NOT generalize to");
        println!("  arbitrary edge weights.");

        // ── Analyze violation patterns ──
        println!("\n{:-<90}", "");
        println!("  VIOLATION PATTERN ANALYSIS");
        println!("{:-<90}", "");

        // Weight ratio analysis: is the new edge much heavier than existing?
        let mut weight_ratios: Vec<f64> = Vec::new();
        let mut local_deltas: Vec<f64> = Vec::new();
        let mut global_deltas: Vec<f64> = Vec::new();
        let mut tau_counts = BTreeMap::new();
        let mut edge_count_dist = BTreeMap::new();

        for v in &all_violations {
            let max_existing = v.edge_weights.iter().cloned().fold(0.0_f64, f64::max);
            let min_existing = v.edge_weights.iter().cloned().fold(f64::INFINITY, f64::min);
            if max_existing > 0.0 {
                weight_ratios.push(v.new_edge_weight / max_existing);
            }
            local_deltas.push(v.local_delta);
            global_deltas.push(v.global_delta);
            *tau_counts.entry(v.tau).or_insert(0u64) += 1;
            *edge_count_dist.entry(v.edges.len()).or_insert(0u64) += 1;
        }

        weight_ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());
        local_deltas.sort_by(|a, b| a.partial_cmp(b).unwrap());
        global_deltas.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median = |v: &[f64]| -> f64 {
            if v.is_empty() {
                return 0.0;
            }
            let mid = v.len() / 2;
            if v.len() % 2 == 0 {
                (v[mid - 1] + v[mid]) / 2.0
            } else {
                v[mid]
            }
        };

        println!("\n  New/max-existing weight ratio:");
        println!(
            "    min: {:.3}, median: {:.3}, max: {:.3}",
            weight_ratios.first().unwrap_or(&0.0),
            median(&weight_ratios),
            weight_ratios.last().unwrap_or(&0.0)
        );

        println!("\n  Local ΔS_τ (all positive by definition):");
        println!(
            "    min: {:.6}, median: {:.6}, max: {:.6}",
            local_deltas.first().unwrap_or(&0.0),
            median(&local_deltas),
            local_deltas.last().unwrap_or(&0.0)
        );

        println!("\n  Global ΔS̄_τ (all negative by definition):");
        println!(
            "    min: {:.6}, median: {:.6}, max: {:.6}",
            global_deltas.first().unwrap_or(&0.0),
            median(&global_deltas),
            global_deltas.last().unwrap_or(&0.0)
        );

        println!("\n  τ distribution:");
        for (&tau, &count) in &tau_counts {
            println!("    τ={}: {} violations", tau, count);
        }

        println!("\n  Existing edge count distribution:");
        for (&edges, &count) in &edge_count_dist {
            println!("    {} edges: {} violations", edges, count);
        }

        // Check: do violations only occur with extreme weight heterogeneity?
        let high_ratio = weight_ratios.iter().filter(|&&r| r > 3.0).count();
        println!(
            "\n  Weight ratio > 3.0 (heavy new edge): {} / {} ({:.1}%)",
            high_ratio,
            weight_ratios.len(),
            100.0 * high_ratio as f64 / weight_ratios.len().max(1) as f64
        );
        let low_ratio = weight_ratios.iter().filter(|&&r| r < 1.0).count();
        println!(
            "  Weight ratio < 1.0 (light new edge): {} / {} ({:.1}%)",
            low_ratio,
            weight_ratios.len(),
            100.0 * low_ratio as f64 / weight_ratios.len().max(1) as f64
        );
    }

    println!();
}

fn get_affected_nodes(adj: &WeightedAdj, from_id: NodeId, to_id: NodeId) -> Vec<NodeId> {
    let mut affected = Vec::new();
    affected.push(from_id);
    affected.push(to_id);
    if let Some(neighbors) = adj.get(&from_id) {
        for &(n, _) in neighbors {
            affected.push(n);
        }
    }
    if let Some(neighbors) = adj.get(&to_id) {
        for &(n, _) in neighbors {
            affected.push(n);
        }
    }
    affected.sort();
    affected.dedup();
    affected
}
