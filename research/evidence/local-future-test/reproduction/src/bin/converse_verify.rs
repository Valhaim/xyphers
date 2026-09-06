//! Exhaustive verification of the CONVERSE of the filter theorem.
//!
//! The filter theorem (proved for N ≤ 6): ΔS_τ^local > 0 ⟹ S̄_τ(G') ≥ S̄_τ(G)
//! The converse question:                 S̄_τ(G') > S̄_τ(G) ⟹ ΔS_τ^local > 0 ?
//!
//! If the converse holds: the game is a full ordinal potential game (Monderer-Shapley).
//! If it fails: edges exist that improve the network but aren't locally rewarding.
//!              The game is still a generalized ordinal potential game (convergence
//!              follows from the forward direction alone), but the current proof overclaims.
//!
//! This binary classifies EVERY edge addition into 4 categories:
//!   A: ΔS_τ^local > 0, S̄_τ increases   (filter theorem confirmed: good edges are good)
//!   B: ΔS_τ^local > 0, S̄_τ decreases   (filter theorem violation — zero expected)
//!   C: ΔS_τ^local ≤ 0, S̄_τ decreases   (correctly filtered out)
//!   D: ΔS_τ^local ≤ 0, S̄_τ increases   (CONVERSE violation — globally helpful but locally unrewarding)
//!
//! Also checks strictness: does ΔS_τ^local > 0 imply S̄_τ STRICTLY increases?
//!
//! Run: cargo run --release --bin converse-verify

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::Instant;

use clap::Parser;

use thaim_core::entropy_exact;
use thaim_core::types::{NodeId, WeightedAdj};

const MAX_TAU: u32 = 10;
const NUMERICAL_EPS: f64 = 1e-10;

#[derive(Parser)]
#[command(
    name = "converse-verify",
    about = "Check the converse of the filter theorem"
)]
struct Cli {
    /// Maximum graph size for exhaustive enumeration
    #[arg(long, default_value = "6")]
    max_n: usize,

    /// Maximum tau to check
    #[arg(long, default_value = "5")]
    max_tau: u32,
}

/// Classification of an edge addition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    A, // local > 0, global > 0  (filter confirmed)
    B, // local > 0, global < 0  (filter VIOLATION)
    C, // local ≤ 0, global < 0  (correctly filtered)
    D, // local ≤ 0, global > 0  (CONVERSE violation)
    // Two "neutral" categories for when global = 0:
    NeutralPos, // local > 0, global = 0  (local positive but global flat — strictness failure)
    NeutralNeg, // local ≤ 0, global = 0  (both flat — uninteresting)
}

struct EdgeClassification {
    n: usize,
    edges: Vec<(usize, usize)>,
    added_edge: (usize, usize),
    tau: u32,
    category: Category,
    local_delta: f64,
    global_delta: f64,
}

fn build_adj(n: usize, edges: &[(usize, usize)]) -> WeightedAdj {
    let mut adj: WeightedAdj = BTreeMap::new();
    for i in 0..n {
        adj.insert(i as NodeId, Vec::new());
    }
    for &(u, v) in edges {
        adj.get_mut(&(u as NodeId))
            .unwrap()
            .push((v as NodeId, 1.0));
        adj.get_mut(&(v as NodeId))
            .unwrap()
            .push((u as NodeId, 1.0));
    }
    adj
}

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

fn all_possible_edges(n: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((i, j));
        }
    }
    edges
}

/// Classify every edge addition for a single graph.
fn classify_graph(n: usize, edges: &[(usize, usize)], max_tau: u32) -> Vec<EdgeClassification> {
    let edge_set: BTreeSet<(usize, usize)> = edges.iter().copied().collect();
    let adj_before = build_adj(n, edges);

    let all_edges = all_possible_edges(n);
    let non_edges: Vec<(usize, usize)> = all_edges
        .into_iter()
        .filter(|e| !edge_set.contains(e))
        .collect();

    if non_edges.is_empty() {
        return Vec::new();
    }

    let mut results = Vec::new();

    for &(u, v) in &non_edges {
        let mut edges_after = edges.to_vec();
        edges_after.push((u, v));
        let adj_after = build_adj(n, &edges_after);

        for tau in 1..=max_tau {
            // Compute global entropy before and after
            let result_before = entropy_exact::compute_all_entropies_exact(&adj_before, tau);
            let result_after = entropy_exact::compute_all_entropies_exact(&adj_after, tau);
            let global_delta = result_after.global - result_before.global;

            // Compute local ΔS_τ (THAIM's minting signal)
            let local_delta =
                entropy_exact::delta_st_exact(&adj_before, u as NodeId, v as NodeId, tau);

            // Classify
            let category = if local_delta > NUMERICAL_EPS {
                if global_delta > NUMERICAL_EPS {
                    Category::A
                } else if global_delta < -NUMERICAL_EPS {
                    Category::B
                } else {
                    Category::NeutralPos
                }
            } else {
                if global_delta < -NUMERICAL_EPS {
                    Category::C
                } else if global_delta > NUMERICAL_EPS {
                    Category::D
                } else {
                    Category::NeutralNeg
                }
            };

            results.push(EdgeClassification {
                n,
                edges: edges.to_vec(),
                added_edge: (u, v),
                tau,
                category,
                local_delta,
                global_delta,
            });
        }
    }

    results
}

fn main() {
    let cli = Cli::parse();
    let max_tau = cli.max_tau.min(MAX_TAU);

    println!("\n{:=<90}", "");
    println!("  CONVERSE VERIFICATION: S̄_τ increases ⟹ ΔS_τ^local > 0 ?");
    println!("  Exhaustive for N ≤ {}, τ ∈ {{1..{}}}", cli.max_n, max_tau);
    println!("  ε = {:.0e}", NUMERICAL_EPS);
    println!("{:=<90}", "");

    let mut total_a = 0u64; // local > 0, global > 0 (filter confirmed)
    let mut total_b = 0u64; // local > 0, global < 0 (filter VIOLATION)
    let mut total_c = 0u64; // local ≤ 0, global < 0 (correctly filtered)
    let mut total_d = 0u64; // local ≤ 0, global > 0 (CONVERSE violation)
    let mut total_np = 0u64; // local > 0, global = 0 (strictness failure)
    let mut total_nn = 0u64; // local ≤ 0, global = 0

    let mut converse_violations: Vec<EdgeClassification> = Vec::new();
    let mut filter_violations: Vec<EdgeClassification> = Vec::new();
    let mut strictness_failures: Vec<EdgeClassification> = Vec::new();

    println!(
        "\n  {:>3} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>8}",
        "N", "graphs", "A(✓)", "B(FT!)", "C(ok)", "D(conv!)", "NP(≈0)", "NN", "time"
    );

    for n in 3..=cli.max_n {
        let possible = all_possible_edges(n);
        let m = possible.len();
        let total_graphs = 1u64 << m;

        let mut n_connected = 0u64;
        let mut n_a = 0u64;
        let mut n_b = 0u64;
        let mut n_c = 0u64;
        let mut n_d = 0u64;
        let mut n_np = 0u64;
        let mut n_nn = 0u64;

        let start = Instant::now();

        for mask in 0..total_graphs {
            let edges: Vec<(usize, usize)> = (0..m)
                .filter(|&bit| mask & (1u64 << bit) != 0)
                .map(|bit| possible[bit])
                .collect();

            if !is_connected(n, &edges) {
                continue;
            }
            n_connected += 1;

            let classifications = classify_graph(n, &edges, max_tau);
            for c in classifications {
                match c.category {
                    Category::A => n_a += 1,
                    Category::B => {
                        n_b += 1;
                        filter_violations.push(c);
                    }
                    Category::C => n_c += 1,
                    Category::D => {
                        n_d += 1;
                        converse_violations.push(c);
                    }
                    Category::NeutralPos => {
                        n_np += 1;
                        if strictness_failures.len() < 20 {
                            strictness_failures.push(c);
                        }
                    }
                    Category::NeutralNeg => n_nn += 1,
                }
            }
        }

        let elapsed = start.elapsed().as_secs_f64();

        total_a += n_a;
        total_b += n_b;
        total_c += n_c;
        total_d += n_d;
        total_np += n_np;
        total_nn += n_nn;

        println!(
            "  {:>3} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>7.1}s",
            n, n_connected, n_a, n_b, n_c, n_d, n_np, n_nn, elapsed
        );
    }

    // ── Report ──
    let total = total_a + total_b + total_c + total_d + total_np + total_nn;

    println!("\n{:=<90}", "");
    println!("  RESULTS");
    println!("{:=<90}", "");

    println!("\n  Four-way classification of {} edge additions:", total);
    println!("  ┌─────────────────────────────┬──────────────────────────────────┐");
    println!("  │                             │  Global entropy change           │");
    println!("  │                             │  increases    decreases   flat   │");
    println!("  ├─────────────────────────────┼──────────────────────────────────┤");
    println!(
        "  │ ΔS_τ^local > 0 (rewarded)  │  A: {:>8}  B: {:>8}  {:>8} │",
        total_a, total_b, total_np
    );
    println!(
        "  │ ΔS_τ^local ≤ 0 (filtered)  │  D: {:>8}  C: {:>8}  {:>8} │",
        total_d, total_c, total_nn
    );
    println!("  └─────────────────────────────┴──────────────────────────────────┘");

    println!("\n  Key questions:");

    // Filter theorem check
    if total_b == 0 {
        println!(
            "  ✓ FILTER THEOREM HOLDS: 0 cases where local > 0 but global decreases (cell B = 0)"
        );
    } else {
        println!("  ✗ FILTER THEOREM VIOLATED: {} cases in cell B!", total_b);
        for (i, v) in filter_violations.iter().enumerate().take(5) {
            println!(
                "    #{}: N={}, edges={:?}, added=({},{}), τ={}, local={:.6}, global={:.6}",
                i + 1,
                v.n,
                v.edges,
                v.added_edge.0,
                v.added_edge.1,
                v.tau,
                v.local_delta,
                v.global_delta
            );
        }
    }

    // Converse check
    if total_d == 0 {
        println!("  ✓ CONVERSE HOLDS: 0 cases where global increases but local ≤ 0 (cell D = 0)");
        println!("    ⟹ Full ordinal potential game (Monderer-Shapley biconditional satisfied)");
    } else {
        println!(
            "  ✗ CONVERSE FAILS: {} cases where S̄_τ increases but ΔS_τ^local ≤ 0 (cell D)",
            total_d
        );
        println!("    ⟹ NOT a full ordinal potential game");
        println!("    ⟹ IS a generalized ordinal potential game (forward direction sufficient for convergence)");
        println!();
        println!("  Converse violations (first 30):");
        for (i, v) in converse_violations.iter().enumerate().take(30) {
            println!(
                "    #{}: N={}, edges={:?}, added=({},{}), τ={}, local_ΔS_τ={:+.6}, global_ΔS̄_τ={:+.6}",
                i + 1, v.n, v.edges, v.added_edge.0, v.added_edge.1,
                v.tau, v.local_delta, v.global_delta
            );
        }

        // Analyze: at what N and τ do converse violations first appear?
        let mut first_by_n: BTreeMap<usize, u32> = BTreeMap::new();
        for v in &converse_violations {
            let entry = first_by_n.entry(v.n).or_insert(v.tau);
            if v.tau < *entry {
                *entry = v.tau;
            }
        }
        println!("\n  First converse violation by N:");
        for (n, tau) in &first_by_n {
            println!("    N={}: first at τ={}", n, tau);
        }
    }

    // Strictness check
    if total_np == 0 {
        println!("  ✓ STRICTNESS HOLDS: ΔS_τ^local > 0 always implies S̄_τ STRICTLY increases");
    } else {
        println!(
            "  ⚠ STRICTNESS FAILS: {} cases where local > 0 but global exactly 0",
            total_np
        );
        println!("    (forward direction gives ≥ not >; weak generalized ordinal potential game)");
        if !strictness_failures.is_empty() {
            println!("    Examples:");
            for (i, v) in strictness_failures.iter().enumerate().take(5) {
                println!(
                    "      #{}: N={}, added=({},{}), τ={}, local={:.2e}, global={:.2e}",
                    i + 1,
                    v.n,
                    v.added_edge.0,
                    v.added_edge.1,
                    v.tau,
                    v.local_delta,
                    v.global_delta
                );
            }
        }
    }

    // Final verdict
    println!("\n{:=<90}", "");
    println!("  VERDICT");
    println!("{:=<90}", "");

    if total_b == 0 && total_d == 0 && total_np == 0 {
        println!("  The ΔS_τ-filtered network formation game is a STRICT ordinal potential game.");
        println!("  Both directions of Monderer-Shapley hold with strict inequality.");
        println!("  Convergence to Nash equilibrium is guaranteed in O(N²) steps.");
    } else if total_b == 0 && total_d == 0 {
        println!("  The game is a WEAK ordinal potential game (≥ not > in some cases).");
        println!("  Both directions of Monderer-Shapley hold with weak inequality.");
    } else if total_b == 0 {
        println!("  The game is a GENERALIZED ordinal potential game.");
        println!("  Forward direction holds: intelligent moves always improve the potential.");
        println!("  Reverse fails: some potential-improving moves are not locally rewarding.");
        println!("  Convergence still follows (Monderer-Shapley Thm 2.6).");
        println!("  But the biconditional (full ordinal) claim must be weakened.");
    } else {
        println!("  ✗ FILTER THEOREM ITSELF FAILED. This should not happen for N ≤ 6, τ ≤ 5.");
    }

    println!();
}
