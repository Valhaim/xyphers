//! Detailed breakdown of δ_min ≥ 2 violations on general graphs (N=7)
//!
//! The exhaustive scan found 34,020 violations on graphs with δ_min ≥ 2.
//! This binary digs into:
//! (a) Per-τ breakdown (which τ values produce violations?)
//! (b) Example violation graphs with δ_min ≥ 2
//! (c) Structural analysis (what do these graphs look like?)
//!
//! Run: cargo run --release --bin dmin2-detail

use std::collections::{BTreeSet, VecDeque};

fn mat_mul(a: &[f64], b: &[f64], n: usize) -> Vec<f64> {
    let mut c = vec![0.0; n * n];
    for i in 0..n {
        for k in 0..n {
            let a_ik = a[i * n + k];
            if a_ik == 0.0 {
                continue;
            }
            for j in 0..n {
                c[i * n + j] += a_ik * b[k * n + j];
            }
        }
    }
    c
}

fn mat_pow(p: &[f64], n: usize, tau: u32) -> Vec<f64> {
    if tau == 0 {
        let mut id = vec![0.0; n * n];
        for i in 0..n {
            id[i * n + i] = 1.0;
        }
        return id;
    }
    if tau == 1 {
        return p.to_vec();
    }
    let half = mat_pow(p, n, tau / 2);
    let result = mat_mul(&half, &half, n);
    if tau % 2 == 0 {
        result
    } else {
        mat_mul(&result, p, n)
    }
}

fn shannon_entropy(dist: &[f64]) -> f64 {
    let mut h = 0.0;
    for &p in dist {
        if p > 1e-15 {
            h -= p * p.log2();
        }
    }
    h
}

fn build_transition_matrix(adj: &[Vec<usize>], n: usize) -> Vec<f64> {
    let mut p = vec![0.0; n * n];
    for i in 0..n {
        let deg = adj[i].len();
        if deg > 0 {
            let w = 1.0 / deg as f64;
            for &j in &adj[i] {
                p[i * n + j] = w;
            }
        }
    }
    p
}

fn is_connected(adj: &[Vec<usize>], n: usize) -> bool {
    let mut visited = vec![false; n];
    let mut queue = VecDeque::new();
    visited[0] = true;
    queue.push_back(0);
    let mut count = 1usize;
    while let Some(node) = queue.pop_front() {
        for &nb in &adj[node] {
            if !visited[nb] {
                visited[nb] = true;
                count += 1;
                queue.push_back(nb);
            }
        }
    }
    count == n
}

fn edges_to_adj(edges: &[(usize, usize)], n: usize) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    adj
}

fn check_filter(adj: &[Vec<usize>], n: usize, u: usize, v: usize, tau: u32) -> (bool, f64, f64) {
    let p_before = build_transition_matrix(adj, n);
    let mut adj_after = adj.to_vec();
    adj_after[u].push(v);
    adj_after[v].push(u);
    let p_after = build_transition_matrix(&adj_after, n);

    let pt_before = mat_pow(&p_before, n, tau);
    let pt_after = mat_pow(&p_after, n, tau);

    let mut affected = BTreeSet::new();
    affected.insert(u);
    affected.insert(v);
    for &nb in &adj[u] {
        affected.insert(nb);
    }
    for &nb in &adj[v] {
        affected.insert(nb);
    }

    let local_sum: f64 = affected
        .iter()
        .map(|&w| {
            shannon_entropy(&pt_after[w * n..(w + 1) * n])
                - shannon_entropy(&pt_before[w * n..(w + 1) * n])
        })
        .sum();
    let local_delta = local_sum / affected.len() as f64;

    let global_sum: f64 = (0..n)
        .map(|w| {
            shannon_entropy(&pt_after[w * n..(w + 1) * n])
                - shannon_entropy(&pt_before[w * n..(w + 1) * n])
        })
        .sum();
    let global_delta = global_sum / n as f64;

    (
        local_delta > 1e-12 && global_delta < -1e-12,
        local_delta,
        global_delta,
    )
}

fn degree_sequence(adj: &[Vec<usize>]) -> Vec<usize> {
    let mut degs: Vec<usize> = adj.iter().map(|nb| nb.len()).collect();
    degs.sort_unstable_by(|a, b| b.cmp(a));
    degs
}

fn main() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  DETAILED δ_min ≥ 2 VIOLATION ANALYSIS (N=7, exhaustive)  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let n = 7usize;
    let max_edges = n * (n - 1) / 2;
    let total_subsets = 1u64 << max_edges;
    let taus = [3u32, 4, 5];

    let all_edges: Vec<(usize, usize)> = {
        let mut e = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                e.push((i, j));
            }
        }
        e
    };

    // Per-τ counters for δ_min ≥ 2 graphs
    let mut viol_dmin2_tau3 = 0u64;
    let mut viol_dmin2_tau4 = 0u64;
    let mut viol_dmin2_tau5 = 0u64;
    let mut checks_dmin2_tau3 = 0u64;
    let mut checks_dmin2_tau4 = 0u64;
    let mut checks_dmin2_tau5 = 0u64;

    // Per-τ counters for δ_min ≥ 3 graphs
    let mut viol_dmin3_tau3 = 0u64;
    let mut viol_dmin3_tau4 = 0u64;
    let mut viol_dmin3_tau5 = 0u64;

    // Example violations on δ_min ≥ 2 graphs
    let mut dmin2_examples: Vec<String> = Vec::new();

    // Track unique degree sequences for δ_min ≥ 2 violations
    let mut dmin2_deg_seqs: BTreeSet<Vec<usize>> = BTreeSet::new();

    let mut connected_count = 0u64;
    let mut total_violations = 0u64;

    for mask in 0..total_subsets {
        let edges: Vec<(usize, usize)> = (0..max_edges)
            .filter(|&bit| mask & (1u64 << bit) != 0)
            .map(|bit| all_edges[bit])
            .collect();

        if edges.len() < n - 1 {
            continue;
        }

        let adj = edges_to_adj(&edges, n);
        if !is_connected(&adj, n) {
            continue;
        }
        connected_count += 1;

        let min_degree = adj.iter().map(|nb| nb.len()).min().unwrap();
        let is_dmin2 = min_degree >= 2;
        let is_dmin3 = min_degree >= 3;

        let edge_set: BTreeSet<(usize, usize)> = edges.iter().copied().collect();

        for &(u, v) in &all_edges {
            if edge_set.contains(&(u, v)) {
                continue;
            }

            let du = adj[u].len();
            let dv = adj[v].len();

            for &tau in &taus {
                if is_dmin2 {
                    match tau {
                        3 => checks_dmin2_tau3 += 1,
                        4 => checks_dmin2_tau4 += 1,
                        5 => checks_dmin2_tau5 += 1,
                        _ => {}
                    }
                }

                let (is_viol, local_d, global_d) = check_filter(&adj, n, u, v, tau);

                if is_viol {
                    total_violations += 1;

                    if is_dmin2 {
                        match tau {
                            3 => viol_dmin2_tau3 += 1,
                            4 => viol_dmin2_tau4 += 1,
                            5 => viol_dmin2_tau5 += 1,
                            _ => {}
                        }

                        let ds = degree_sequence(&adj);
                        dmin2_deg_seqs.insert(ds);

                        if dmin2_examples.len() < 20 {
                            let degs: Vec<usize> = adj.iter().map(|nb| nb.len()).collect();
                            dmin2_examples.push(format!(
                                "  τ={}, ({},{}), d_u={}, d_v={}, δ_min={}, degs={:?}, ΔS_l={:.6}, ΔS_g={:.6}, edges={:?}",
                                tau, u, v, du, dv, min_degree, degs, local_d, global_d, edges));
                        }
                    }

                    if is_dmin3 {
                        match tau {
                            3 => viol_dmin3_tau3 += 1,
                            4 => viol_dmin3_tau4 += 1,
                            5 => viol_dmin3_tau5 += 1,
                            _ => {}
                        }
                    }
                }
            }
        }

        if connected_count % 100_000 == 0 {
            let pct = 100.0 * mask as f64 / total_subsets as f64;
            eprint!(
                "  [{:.0}%] {} graphs, {} violations\r",
                pct, connected_count, total_violations
            );
        }
    }

    println!("═══ Per-τ breakdown for δ_min ≥ 2 graphs ═══");
    println!();
    println!(
        "  {:>4}  {:>12}  {:>12}  {:>8}",
        "τ", "checks", "violations", "rate"
    );
    println!("  {}", "-".repeat(42));
    for &(tau, checks, viols) in &[
        (3, checks_dmin2_tau3, viol_dmin2_tau3),
        (4, checks_dmin2_tau4, viol_dmin2_tau4),
        (5, checks_dmin2_tau5, viol_dmin2_tau5),
    ] {
        let rate = if checks > 0 {
            viols as f64 / checks as f64
        } else {
            0.0
        };
        println!("  {:>4}  {:>12}  {:>12}  {:>8.6}", tau, checks, viols, rate);
    }
    println!();

    println!("═══ Per-τ breakdown for δ_min ≥ 3 graphs ═══");
    println!();
    println!("  τ=3: {} violations", viol_dmin3_tau3);
    println!("  τ=4: {} violations", viol_dmin3_tau4);
    println!("  τ=5: {} violations", viol_dmin3_tau5);
    println!();

    println!("═══ Unique degree sequences in δ_min ≥ 2 violations ═══");
    println!();
    for ds in &dmin2_deg_seqs {
        println!("  {:?}", ds);
    }
    println!("  Total unique degree sequences: {}", dmin2_deg_seqs.len());
    println!();

    println!("═══ Example violations on δ_min ≥ 2 graphs ═══");
    println!();
    for ex in &dmin2_examples {
        println!("{}", ex);
    }
    println!();

    println!("═══ SUMMARY ═══");
    println!();
    println!("  Total violations: {}", total_violations);
    println!(
        "  δ_min ≥ 2 violations: {} (τ3={}, τ4={}, τ5={})",
        viol_dmin2_tau3 + viol_dmin2_tau4 + viol_dmin2_tau5,
        viol_dmin2_tau3,
        viol_dmin2_tau4,
        viol_dmin2_tau5
    );
    println!(
        "  δ_min ≥ 3 violations: {} (τ3={}, τ4={}, τ5={})",
        viol_dmin3_tau3 + viol_dmin3_tau4 + viol_dmin3_tau5,
        viol_dmin3_tau3,
        viol_dmin3_tau4,
        viol_dmin3_tau5
    );
    println!();

    if viol_dmin2_tau3 == 0 {
        println!(
            "  ★ τ=3 CLEAN on δ_min ≥ 2 graphs ({} checks)",
            checks_dmin2_tau3
        );
    } else {
        println!(
            "  ✗ τ=3 has {} violations on δ_min ≥ 2 graphs",
            viol_dmin2_tau3
        );
    }

    if viol_dmin3_tau3 + viol_dmin3_tau4 + viol_dmin3_tau5 == 0 {
        println!("  ★ δ_min ≥ 3 CLEAN at all τ ∈ {{3,4,5}}");
    }
    println!();
}
