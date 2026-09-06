//! M3 verification: EXHAUSTIVE N=8 filter test at τ=3 for δ_min ≥ 2.
//!
//! The sampling binary found violations at N=8 that were absent at N=7.
//! This binary does exhaustive enumeration of all 2^28 edge subsets at N=8
//! to confirm or refute those results.

use std::collections::VecDeque;

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

fn mat_pow(m: &[f64], n: usize, tau: usize) -> Vec<f64> {
    if tau == 1 {
        return m.to_vec();
    }
    let half = mat_pow(m, n, tau / 2);
    let sq = mat_mul(&half, &half, n);
    if tau % 2 == 0 {
        sq
    } else {
        mat_mul(&sq, m, n)
    }
}

fn shannon(row: &[f64]) -> f64 {
    let mut h = 0.0;
    for &p in row {
        if p > 1e-15 {
            h -= p * p.log2();
        }
    }
    h
}

/// Full filter check returning per-node ΔS values
fn check_filter_detailed(
    adj: &[Vec<usize>],
    n: usize,
    u: usize,
    v: usize,
    tau: usize,
) -> (Vec<f64>, f64, f64, f64, f64) {
    let mut p_before = vec![0.0; n * n];
    for i in 0..n {
        let d = adj[i].len() as f64;
        for &j in &adj[i] {
            p_before[i * n + j] = 1.0 / d;
        }
    }

    let mut adj2 = adj.to_vec();
    adj2[u].push(v);
    adj2[v].push(u);
    let mut p_after = vec![0.0; n * n];
    for i in 0..n {
        let d = adj2[i].len() as f64;
        for &j in &adj2[i] {
            p_after[i * n + j] = 1.0 / d;
        }
    }

    let pt_b = mat_pow(&p_before, n, tau);
    let pt_a = mat_pow(&p_after, n, tau);

    let mut ds = vec![0.0; n];
    for w in 0..n {
        ds[w] = shannon(&pt_a[w * n..(w + 1) * n]) - shannon(&pt_b[w * n..(w + 1) * n]);
    }

    let mut in_a = vec![false; n];
    in_a[u] = true;
    in_a[v] = true;
    for &nb in &adj[u] {
        in_a[nb] = true;
    }
    for &nb in &adj[v] {
        in_a[nb] = true;
    }

    let phi01: f64 = (0..n).filter(|&w| in_a[w]).map(|w| ds[w]).sum();
    let phi2_neg: f64 = (0..n)
        .filter(|&w| !in_a[w] && ds[w] < 0.0)
        .map(|w| ds[w])
        .sum();
    let phi2_pos: f64 = (0..n)
        .filter(|&w| !in_a[w] && ds[w] > 0.0)
        .map(|w| ds[w])
        .sum();
    let global: f64 = ds.iter().sum();

    (ds, phi01, phi2_neg, phi2_pos, global)
}

fn main() {
    let n: usize = 8;
    let tau: usize = 3;
    let min_deg: usize = 2;

    println!();
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!(
        "║  EXHAUSTIVE N={} VERIFICATION: τ={}, δ_min ≥ {}              ║",
        n, tau, min_deg
    );
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    let max_edges = n * (n - 1) / 2;
    let total_masks = 1u64 << max_edges;
    let mut edges = Vec::new();
    for i in 0..n {
        for j in (i + 1)..n {
            edges.push((i, j));
        }
    }

    println!("  Total edge subsets: {} (2^{})", total_masks, max_edges);
    println!("  Testing τ={}, δ_min ≥ {}", tau, min_deg);
    println!();

    let mut graph_count = 0u64;
    let mut checks = 0u64;
    let mut violations = 0u64;
    let mut worst_ratio = 0.0f64;
    let mut worst_margin = f64::INFINITY;
    let mut phi2neg_count = 0u64; // how many checks have any Φ₂⁻
    let mut first_violation_printed = false;

    // Store worst violation for detailed output
    let mut worst_adj: Vec<Vec<usize>> = vec![];
    let mut worst_uv = (0, 0);

    let report_interval = 10_000_000u64;
    let mut next_report = report_interval;

    for mask in 0..total_masks {
        if mask >= next_report {
            eprint!(
                "\r  Progress: {:.1}% ({} masks, {} graphs, {} checks, {} violations)",
                100.0 * mask as f64 / total_masks as f64,
                mask,
                graph_count,
                checks,
                violations
            );
            next_report += report_interval;
        }

        let mut adj = vec![vec![]; n];
        for (idx, &(i, j)) in edges.iter().enumerate() {
            if mask & (1u64 << idx) != 0 {
                adj[i].push(j);
                adj[j].push(i);
            }
        }
        if adj.iter().any(|nb| nb.len() < min_deg) {
            continue;
        }

        // Connectivity
        let mut visited = vec![false; n];
        let mut queue = VecDeque::new();
        visited[0] = true;
        queue.push_back(0);
        while let Some(node) = queue.pop_front() {
            for &nb in &adj[node] {
                if !visited[nb] {
                    visited[nb] = true;
                    queue.push_back(nb);
                }
            }
        }
        if visited.iter().any(|&v| !v) {
            continue;
        }
        graph_count += 1;

        for u in 0..n {
            for v in (u + 1)..n {
                if adj[u].contains(&v) {
                    continue;
                }

                let (ds, phi01, phi2n, _phi2p, global) = check_filter_detailed(&adj, n, u, v, tau);

                if phi01 > 1e-12 {
                    checks += 1;

                    if phi2n < -1e-15 {
                        phi2neg_count += 1;
                    }

                    let ratio = if phi01 > 1e-15 && phi2n < 0.0 {
                        (-phi2n) / phi01
                    } else {
                        0.0
                    };

                    if global < -1e-12 {
                        violations += 1;

                        // Print first 5 violations in detail
                        if violations <= 5 {
                            let degs: Vec<usize> = adj.iter().map(|nb| nb.len()).collect();
                            println!("\n  VIOLATION #{}: mask={:#010x}", violations, mask);
                            println!("    Degree sequence: {:?}", degs);
                            println!(
                                "    Edge: ({}, {}), degs=({}, {})",
                                u,
                                v,
                                adj[u].len(),
                                adj[v].len()
                            );
                            println!("    Adjacency:");
                            for i in 0..n {
                                println!("      {}: {:?}", i, adj[i]);
                            }
                            println!("    Per-node ΔS:");
                            let mut in_a = vec![false; n];
                            in_a[u] = true;
                            in_a[v] = true;
                            for &nb in &adj[u] {
                                in_a[nb] = true;
                            }
                            for &nb in &adj[v] {
                                in_a[nb] = true;
                            }
                            for w in 0..n {
                                let shell = if w == u || w == v {
                                    "S0"
                                } else if in_a[w] {
                                    "S1"
                                } else {
                                    "S2"
                                };
                                println!(
                                    "      node {}: ΔS = {:+.8e}  [{}]  deg={}",
                                    w,
                                    ds[w],
                                    shell,
                                    adj[w].len()
                                );
                            }
                            println!("    Φ₀₁ = {:+.8e}", phi01);
                            println!("    Φ₂⁻ = {:+.8e}", phi2n);
                            println!("    ratio = {:.6}", ratio);
                            println!("    global = {:+.8e}", global);
                        }
                    }

                    if ratio > worst_ratio {
                        worst_ratio = ratio;
                        worst_adj = adj.clone();
                        worst_uv = (u, v);
                    }
                    if global < worst_margin {
                        worst_margin = global;
                    }
                }
            }
        }
    }

    eprintln!("\r  Done.                                                                      ");

    println!();
    println!("════════════════════════════════════════════════════════════════");
    println!(
        "  EXHAUSTIVE RESULTS: N={}, τ={}, δ_min ≥ {}",
        n, tau, min_deg
    );
    println!("════════════════════════════════════════════════════════════════");
    println!("  Connected graphs: {}", graph_count);
    println!("  Filter checks (local > 0): {}", checks);
    println!(
        "  Checks with Φ₂⁻ < 0: {} ({:.4}%)",
        phi2neg_count,
        100.0 * phi2neg_count as f64 / checks as f64
    );
    println!(
        "  VIOLATIONS: {} ({:.6}%)",
        violations,
        if checks > 0 {
            100.0 * violations as f64 / checks as f64
        } else {
            0.0
        }
    );
    println!("  Worst ratio |Φ₂⁻|/(Φ₀+Φ₁): {:.6}", worst_ratio);
    println!(
        "  Safety factor: {:.4}×",
        if worst_ratio > 0.0 {
            1.0 / worst_ratio
        } else {
            f64::INFINITY
        }
    );
    println!("  Worst margin (min global ΔS): {:+.8e}", worst_margin);

    if !worst_adj.is_empty() {
        let degs: Vec<usize> = worst_adj.iter().map(|nb| nb.len()).collect();
        println!(
            "  Worst graph: degs={:?}, edge ({},{})",
            degs, worst_uv.0, worst_uv.1
        );
    }

    println!();
    if violations == 0 {
        println!(
            "  RESULT: Filter HOLDS exhaustively at N={}, τ={}, δ_min≥{}",
            n, tau, min_deg
        );
    } else {
        println!(
            "  RESULT: Filter FAILS at N={}, τ={}, δ_min≥{} ({} violations)",
            n, tau, min_deg, violations
        );
    }
    println!();
}
