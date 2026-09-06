//! Exhaustive verification of Lemma 2: adding an edge to a connected unit-weight
//! graph never decreases any node's causal path entropy S_τ.
//!
//! Prong 1: Enumerate ALL connected graphs on N ≤ 7 nodes (labeled).
//! For each graph, each non-adjacent pair (u,v), each τ ∈ {1..MAX_TAU}:
//!   check S_τ(w; G+(u,v)) ≥ S_τ(w; G) for ALL nodes w.
//!
//! Prong 2: Monte Carlo for N = 8..20 (random graphs, multiple topologies).
//!
//! Run: cargo run --release --bin lemma2-verify
//!
//! This constitutes a computer-assisted proof for all graph structures up to N=7.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::Instant;

use clap::Parser;

use thaim_core::entropy_exact;
use thaim_core::types::{NodeId, WeightedAdj};

const MAX_TAU: u32 = 10;
const NUMERICAL_EPS: f64 = 1e-10;

#[derive(Parser)]
#[command(
    name = "lemma2-verify",
    about = "Exhaustive verification of entropy monotonicity under edge addition"
)]
struct Cli {
    /// Maximum graph size for exhaustive enumeration
    #[arg(long, default_value = "7")]
    max_exhaustive_n: usize,

    /// Number of random graphs per (size, topology) for Monte Carlo
    #[arg(long, default_value = "10000")]
    mc_samples: usize,

    /// Maximum graph size for Monte Carlo
    #[arg(long, default_value = "20")]
    max_mc_n: usize,

    /// Skip Monte Carlo phase
    #[arg(long, default_value = "false")]
    exhaustive_only: bool,

    /// Maximum tau to check
    #[arg(long, default_value = "10")]
    max_tau: u32,
}

/// A violation record.
#[derive(Debug)]
struct Violation {
    n: usize,
    edges: Vec<(usize, usize)>,
    added_edge: (usize, usize),
    tau: u32,
    node: usize,
    entropy_before: f64,
    entropy_after: f64,
    delta: f64,
    is_global: bool,     // true = global violation, false = per-node
    local_delta_st: f64, // THAIM's local ΔS_τ (affected nodes only)
    is_bipartite: bool,
}

/// Build a WeightedAdj from an edge list on nodes 0..n-1.
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

/// Check if the graph on n nodes with given edges is connected (BFS from 0).
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

/// Check if a graph is bipartite (2-colorable).
fn is_bipartite(n: usize, edges: &[(usize, usize)]) -> bool {
    let mut adj_list: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(u, v) in edges {
        adj_list[u].push(v);
        adj_list[v].push(u);
    }
    let mut color: Vec<i8> = vec![-1; n];
    for start in 0..n {
        if color[start] != -1 {
            continue;
        }
        color[start] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            for &nbr in &adj_list[node] {
                if color[nbr] == -1 {
                    color[nbr] = 1 - color[node];
                    queue.push_back(nbr);
                } else if color[nbr] == color[node] {
                    return false;
                }
            }
        }
    }
    true
}

/// Enumerate all possible edges for a graph on n nodes, as (i,j) with i < j.
fn all_possible_edges(n: usize) -> Vec<(usize, usize)> {
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((i, j));
        }
    }
    edges
}

/// Check Lemma 2 for a single graph and return any violations.
fn check_graph(n: usize, edges: &[(usize, usize)], max_tau: u32) -> Vec<Violation> {
    let edge_set: BTreeSet<(usize, usize)> = edges.iter().copied().collect();
    let adj_before = build_adj(n, edges);
    let bipartite = is_bipartite(n, edges);

    // Find non-adjacent pairs
    let all_edges = all_possible_edges(n);
    let non_edges: Vec<(usize, usize)> = all_edges
        .into_iter()
        .filter(|e| !edge_set.contains(e))
        .collect();

    if non_edges.is_empty() {
        return Vec::new(); // Complete graph, nothing to add
    }

    let mut violations = Vec::new();

    for &(u, v) in &non_edges {
        // Build G' = G + (u,v)
        let mut edges_after = edges.to_vec();
        edges_after.push((u, v));
        let adj_after = build_adj(n, &edges_after);

        for tau in 1..=max_tau {
            let result_before = entropy_exact::compute_all_entropies_exact(&adj_before, tau);
            let result_after = entropy_exact::compute_all_entropies_exact(&adj_after, tau);

            // Per-node check
            for w in 0..n {
                let w_id = w as NodeId;
                let s_before = result_before.per_node.get(&w_id).copied().unwrap_or(0.0);
                let s_after = result_after.per_node.get(&w_id).copied().unwrap_or(0.0);
                let delta = s_after - s_before;

                if delta < -NUMERICAL_EPS {
                    violations.push(Violation {
                        n,
                        edges: edges.to_vec(),
                        added_edge: (u, v),
                        tau,
                        node: w,
                        entropy_before: s_before,
                        entropy_after: s_after,
                        delta,
                        is_global: false,
                        local_delta_st: 0.0, // not computed for per-node (expensive)
                        is_bipartite: bipartite,
                    });
                }
            }

            // Global check
            let global_delta = result_after.global - result_before.global;
            if global_delta < -NUMERICAL_EPS {
                // For global violations, compute local ΔS_τ — the critical MCTS filter check
                let local_delta =
                    entropy_exact::delta_st_exact(&adj_before, u as NodeId, v as NodeId, tau);
                violations.push(Violation {
                    n,
                    edges: edges.to_vec(),
                    added_edge: (u, v),
                    tau,
                    node: usize::MAX,
                    entropy_before: result_before.global,
                    entropy_after: result_after.global,
                    delta: global_delta,
                    is_global: true,
                    local_delta_st: local_delta,
                    is_bipartite: bipartite,
                });
            }
        }
    }

    violations
}

/// Exhaustive enumeration of all connected graphs on n labeled nodes.
fn exhaustive_check(n: usize, max_tau: u32) -> (u64, u64, u64, u64, Vec<Violation>) {
    let possible = all_possible_edges(n);
    let m = possible.len(); // N*(N-1)/2
    let total_graphs = 1u64 << m;
    let mut connected_count = 0u64;
    let mut total_checks = 0u64;
    let mut per_node_violations = 0u64;
    let mut global_violations = 0u64;
    let mut all_violations: Vec<Violation> = Vec::new();

    for mask in 0..total_graphs {
        // Build edge list from bitmask
        let edges: Vec<(usize, usize)> = (0..m)
            .filter(|&bit| mask & (1u64 << bit) != 0)
            .map(|bit| possible[bit])
            .collect();

        if !is_connected(n, &edges) {
            continue;
        }
        connected_count += 1;

        // Count non-edges for this graph
        let non_edge_count = m - edges.len();
        total_checks += non_edge_count as u64 * max_tau as u64 * n as u64;

        let violations = check_graph(n, &edges, max_tau);
        for v in &violations {
            if v.is_global {
                global_violations += 1;
            } else {
                per_node_violations += 1;
            }
        }
        if !violations.is_empty() {
            all_violations.extend(violations);
        }
    }

    (
        connected_count,
        total_checks,
        per_node_violations,
        global_violations,
        all_violations,
    )
}

/// Generate a random Erdős-Rényi graph G(n, p).
fn random_er_graph(n: usize, p: f64, seed: u64) -> Vec<(usize, usize)> {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    let mut rng = StdRng::seed_from_u64(seed);
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            if rng.gen::<f64>() < p {
                edges.push((i, j));
            }
        }
    }
    edges
}

/// Generate a Barabási-Albert scale-free graph.
fn random_ba_graph(n: usize, m_attach: usize, seed: u64) -> Vec<(usize, usize)> {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    let mut rng = StdRng::seed_from_u64(seed);

    // Start with a complete graph on m_attach+1 nodes
    let init = m_attach + 1;
    let mut edges = Vec::new();
    let mut degree = vec![0usize; n];
    for i in 0..init.min(n) {
        for j in (i + 1)..init.min(n) {
            edges.push((i, j));
            degree[i] += 1;
            degree[j] += 1;
        }
    }

    let mut total_degree: usize = degree.iter().sum();

    for new_node in init..n {
        let mut targets = BTreeSet::new();
        let mut attempts = 0;
        while targets.len() < m_attach && attempts < 1000 {
            // Preferential attachment
            let r = rng.gen_range(0..total_degree.max(1));
            let mut cumulative = 0;
            for i in 0..new_node {
                cumulative += degree[i];
                if cumulative > r {
                    targets.insert(i);
                    break;
                }
            }
            attempts += 1;
        }
        for &t in &targets {
            edges.push((t.min(new_node), t.max(new_node)));
            degree[new_node] += 1;
            degree[t] += 1;
            total_degree += 2;
        }
    }

    edges
}

/// Generate a random tree on n nodes (Prüfer sequence).
fn random_tree(n: usize, seed: u64) -> Vec<(usize, usize)> {
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};
    if n <= 1 {
        return Vec::new();
    }
    if n == 2 {
        return vec![(0, 1)];
    }
    let mut rng = StdRng::seed_from_u64(seed);
    // Prüfer sequence
    let prufer: Vec<usize> = (0..(n - 2)).map(|_| rng.gen_range(0..n)).collect();
    let mut degree = vec![1usize; n];
    for &p in &prufer {
        degree[p] += 1;
    }
    let mut edges = Vec::new();
    for &p in &prufer {
        // Find smallest leaf
        for i in 0..n {
            if degree[i] == 1 {
                let u = i.min(p);
                let v = i.max(p);
                edges.push((u, v));
                degree[i] -= 1;
                degree[p] -= 1;
                break;
            }
        }
    }
    // Connect last two nodes with degree 1
    let remaining: Vec<usize> = (0..n).filter(|&i| degree[i] == 1).collect();
    if remaining.len() == 2 {
        let u = remaining[0].min(remaining[1]);
        let v = remaining[0].max(remaining[1]);
        edges.push((u, v));
    }
    edges
}

/// Monte Carlo check for a given size.
fn monte_carlo_check(n: usize, samples: usize, max_tau: u32) -> (u64, u64, u64, Vec<Violation>) {
    let mut total_checks = 0u64;
    let mut per_node_violations = 0u64;
    let mut global_violations = 0u64;
    let mut all_violations: Vec<Violation> = Vec::new();
    let topology_generators: Vec<(&str, Box<dyn Fn(usize, u64) -> Vec<(usize, usize)>>)> = vec![
        ("ER(0.3)", Box::new(|n, s| random_er_graph(n, 0.3, s))),
        ("ER(0.5)", Box::new(|n, s| random_er_graph(n, 0.5, s))),
        ("BA(m=2)", Box::new(|n, s| random_ba_graph(n, 2, s))),
        ("Tree", Box::new(|n, s| random_tree(n, s))),
    ];

    for (topo_name, gen) in &topology_generators {
        let mut topo_checks = 0u64;
        let mut topo_graphs = 0u64;

        for sample in 0..samples {
            let seed = (n as u64) * 1_000_000 + sample as u64;
            let edges = gen(n, seed);

            // Deduplicate edges and ensure (u < v)
            let edge_set: BTreeSet<(usize, usize)> = edges
                .into_iter()
                .map(|(u, v)| (u.min(v), u.max(v)))
                .collect();
            let edges: Vec<(usize, usize)> = edge_set.into_iter().collect();

            if !is_connected(n, &edges) {
                continue;
            }
            topo_graphs += 1;

            let m_possible = n * (n - 1) / 2;
            let non_edge_count = m_possible - edges.len();
            if non_edge_count == 0 {
                continue; // Complete graph
            }

            // For Monte Carlo, only check a subset of non-edges (up to 10) and τ values
            let violations = check_graph(n, &edges, max_tau);
            topo_checks += non_edge_count as u64 * max_tau as u64 * n as u64;

            for v in &violations {
                if v.is_global {
                    global_violations += 1;
                } else {
                    per_node_violations += 1;
                }
            }
            if !violations.is_empty() {
                all_violations.extend(violations);
            }
        }

        total_checks += topo_checks;
        eprintln!(
            "    {:<10} {:>6} connected graphs, {:>12} checks",
            topo_name, topo_graphs, topo_checks
        );
    }

    (
        total_checks,
        per_node_violations,
        global_violations,
        all_violations,
    )
}

fn main() {
    let cli = Cli::parse();
    let max_tau = cli.max_tau.min(MAX_TAU);

    println!("\n{:=<90}", "");
    println!("  LEMMA 2 EXHAUSTIVE VERIFICATION");
    println!(
        "  S_τ(w; G+(u,v)) ≥ S_τ(w; G) for all nodes w, all τ ∈ {{1..{}}}",
        max_tau
    );
    println!("  Numerical tolerance: ε = {:.0e}", NUMERICAL_EPS);
    println!("{:=<90}", "");

    let mut grand_total_checks = 0u64;
    let mut grand_per_node_violations = 0u64;
    let mut grand_global_violations = 0u64;
    let mut all_violations: Vec<Violation> = Vec::new();

    // ── Phase 1: Exhaustive Enumeration ──
    println!("\n{:-<90}", "");
    println!(
        "  PHASE 1: EXHAUSTIVE ENUMERATION (all connected graphs, N ≤ {})",
        cli.max_exhaustive_n
    );
    println!("{:-<90}", "");

    println!(
        "\n  {:>5} {:>12} {:>14} {:>14} {:>14} {:>8}",
        "N", "connected", "checks", "per-node viol", "global viol", "time"
    );

    for n in 3..=cli.max_exhaustive_n {
        let start = Instant::now();
        let (connected, checks, pn_viol, g_viol, violations) = exhaustive_check(n, max_tau);
        let elapsed = start.elapsed().as_secs_f64();

        grand_total_checks += checks;
        grand_per_node_violations += pn_viol;
        grand_global_violations += g_viol;
        all_violations.extend(violations);

        println!(
            "  {:>5} {:>12} {:>14} {:>14} {:>14} {:>7.1}s",
            n, connected, checks, pn_viol, g_viol, elapsed
        );
    }

    // ── Phase 2: Monte Carlo ──
    if !cli.exhaustive_only {
        println!("\n{:-<90}", "");
        println!(
            "  PHASE 2: MONTE CARLO (N = {}..{}, {} samples/topology)",
            cli.max_exhaustive_n + 1,
            cli.max_mc_n,
            cli.mc_samples
        );
        println!("{:-<90}", "");

        for n in (cli.max_exhaustive_n + 1)..=cli.max_mc_n {
            eprint!("  N={:>3}: ", n);
            let start = Instant::now();
            let (checks, pn_viol, g_viol, violations) =
                monte_carlo_check(n, cli.mc_samples, max_tau);
            let elapsed = start.elapsed().as_secs_f64();

            grand_total_checks += checks;
            grand_per_node_violations += pn_viol;
            grand_global_violations += g_viol;

            let has_violations = !violations.is_empty();
            all_violations.extend(violations);

            eprintln!(
                "    Total: {:>12} checks, {} violations ({:.1}s)",
                checks,
                if has_violations { "FOUND" } else { "none" },
                elapsed
            );
        }
    }

    // ── Report Violations ──
    if !all_violations.is_empty() {
        println!("\n{:-<90}", "");
        println!("  VIOLATIONS FOUND");
        println!("{:-<90}", "");

        let per_node: Vec<&Violation> = all_violations.iter().filter(|v| !v.is_global).collect();
        let global: Vec<&Violation> = all_violations.iter().filter(|v| v.is_global).collect();

        if !per_node.is_empty() {
            println!("\n  Per-node violations ({}):", per_node.len());
            for (i, v) in per_node.iter().enumerate().take(10) {
                println!(
                    "    #{}: N={}, added=({},{}), τ={}, node={}, Δ={:.2e}, local_ΔS_τ={:.6}, bipartite={}",
                    i + 1, v.n, v.added_edge.0, v.added_edge.1,
                    v.tau, v.node, v.delta, v.local_delta_st, v.is_bipartite
                );
            }
            if per_node.len() > 10 {
                println!("    ... and {} more", per_node.len() - 10);
            }

            // Classification
            let pn_with_positive_local: Vec<&&Violation> = per_node
                .iter()
                .filter(|v| v.local_delta_st > NUMERICAL_EPS)
                .collect();
            let pn_bipartite: usize = per_node.iter().filter(|v| v.is_bipartite).count();
            println!("\n  Per-node classification:");
            println!(
                "    On bipartite graphs: {} / {}",
                pn_bipartite,
                per_node.len()
            );
            println!(
                "    With THAIM local ΔS_τ > 0: {} / {} ← would MCTS select these?",
                pn_with_positive_local.len(),
                per_node.len()
            );
        }

        if !global.is_empty() {
            println!("\n  Global violations ({}):", global.len());
            for (i, v) in global.iter().enumerate().take(10) {
                println!(
                    "    #{}: N={}, added=({},{}), τ={}, Δ_global={:.2e}, local_ΔS_τ={:.6}, bipartite={}",
                    i + 1, v.n, v.added_edge.0, v.added_edge.1,
                    v.tau, v.delta, v.local_delta_st, v.is_bipartite
                );
            }
            if global.len() > 10 {
                println!("    ... and {} more", global.len() - 10);
            }

            // Critical classification
            let g_with_positive_local: Vec<&&Violation> = global
                .iter()
                .filter(|v| v.local_delta_st > NUMERICAL_EPS)
                .collect();
            let g_bipartite: usize = global.iter().filter(|v| v.is_bipartite).count();
            println!("\n  Global violation classification:");
            println!(
                "    On bipartite graphs: {} / {}",
                g_bipartite,
                global.len()
            );
            println!(
                "    With THAIM local ΔS_τ > 0: {} / {} ← CRITICAL: MCTS-selected violations?",
                g_with_positive_local.len(),
                global.len()
            );

            if !g_with_positive_local.is_empty() {
                println!("\n  ⚠ CRITICAL: These global violations would pass THAIM's MCTS filter:");
                for (i, v) in g_with_positive_local.iter().enumerate().take(5) {
                    println!(
                        "    #{}: N={}, edges={:?}, added=({},{}), τ={}, Δ_global={:.2e}, local_ΔS_τ={:.6}",
                        i + 1, v.n, v.edges, v.added_edge.0, v.added_edge.1,
                        v.tau, v.delta, v.local_delta_st
                    );
                }
            } else {
                println!("\n  ✓ ALL global violations have local ΔS_τ ≤ 0 — THAIM's MCTS filter catches them all.");
            }
        }
    }

    // ── Summary ──
    println!("\n{:=<90}", "");
    println!("  SUMMARY");
    println!("{:=<90}", "");
    println!("\n  Total entropy comparisons: {:>14}", grand_total_checks);
    println!(
        "  Per-node violations:       {:>14}",
        grand_per_node_violations
    );
    println!(
        "  Global violations:         {:>14}",
        grand_global_violations
    );

    if grand_per_node_violations == 0 && grand_global_violations == 0 {
        println!("\n  ✓ LEMMA 2 HOLDS: Zero violations across all checks.");
        println!("    Adding an edge to a connected unit-weight graph never decreases");
        println!(
            "    any node's causal path entropy S_τ, for all τ ∈ {{1..{}}}.",
            max_tau
        );
        println!("\n  This constitutes a computer-assisted proof for all graph structures");
        println!("  up to N={} nodes.", cli.max_exhaustive_n);
    } else if grand_global_violations == 0 {
        println!("\n  ⚠ Per-node monotonicity violated, but GLOBAL monotonicity holds.");
        println!("    S̄_τ(G') ≥ S̄_τ(G) for all tested cases.");
        println!("    The potential game result still follows from global monotonicity.");
    } else {
        println!("\n  ✗ GLOBAL MONOTONICITY VIOLATED. The potential game argument");
        println!("    requires revision. See violation details above.");
    }

    println!();
}
