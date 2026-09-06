//! M5: Does the max-ΔS_τ edge ever cause a global violation?
//!
//! The filter theorem fails on general graphs at τ=3 for N≥8, but violations
//! involve edges with near-zero local gain. The protocol selects the BEST edge
//! (maximum ΔS_τ at the proposer). This binary tests whether the max-ΔS edge
//! ever causes a global decrease.
//!
//! For each graph: find the non-edge with maximum local sum (Φ₀₁), check if
//! adding it decreases global average entropy.

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

/// Returns (phi01, global_ds) for adding edge (u,v)
fn eval_edge(adj: &[Vec<usize>], n: usize, u: usize, v: usize, tau: usize) -> (f64, f64) {
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
    let global: f64 = ds.iter().sum();

    (phi01, global)
}

fn main() {
    println!();
    println!("╔════════════════════════════════════════════════════════════════╗");
    println!("║  M5: Does the MAX-ΔS edge ever cause a global violation?     ║");
    println!("╚════════════════════════════════════════════════════════════════╝");
    println!();

    for &n in &[7, 8] {
        let max_edges = n * (n - 1) / 2;
        let total_masks = 1u64 << max_edges;
        let mut edges = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                edges.push((i, j));
            }
        }

        for &tau in &[3, 5] {
            for &min_deg in &[2, 3] {
                let mut graph_count = 0u64;
                let mut max_edge_violations = 0u64;
                let mut top_edge_violations = 0u64; // top-3 edges
                let mut any_positive_edges = 0u64; // graphs that have at least one locally-positive edge
                let mut worst_global_on_max = f64::INFINITY;
                let mut best_local_that_violates = 0.0f64;

                let report_interval = if n <= 7 { 500_000u64 } else { 10_000_000 };
                let mut next_report = report_interval;

                for mask in 0..total_masks {
                    if mask >= next_report {
                        eprint!(
                            "\r  N={} τ={} δ≥{}: {:.1}%  graphs={} max_violations={}",
                            n,
                            tau,
                            min_deg,
                            100.0 * mask as f64 / total_masks as f64,
                            graph_count,
                            max_edge_violations
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

                    // Evaluate all non-edges
                    let mut edge_results: Vec<(f64, f64, usize, usize)> = Vec::new(); // (phi01, global, u, v)
                    for u in 0..n {
                        for v in (u + 1)..n {
                            if adj[u].contains(&v) {
                                continue;
                            }
                            let (phi01, global) = eval_edge(&adj, n, u, v, tau);
                            if phi01 > 1e-12 {
                                edge_results.push((phi01, global, u, v));
                            }
                        }
                    }

                    if edge_results.is_empty() {
                        continue;
                    }
                    any_positive_edges += 1;

                    // Sort by local gain (phi01), descending
                    edge_results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

                    // Check the MAX-ΔS edge
                    let (best_phi01, best_global, _bu, _bv) = edge_results[0];
                    if best_global < -1e-12 {
                        max_edge_violations += 1;
                        if best_phi01 > best_local_that_violates {
                            best_local_that_violates = best_phi01;
                        }
                    }
                    if best_global < worst_global_on_max {
                        worst_global_on_max = best_global;
                    }

                    // Check top-3 edges
                    let top_count = edge_results.len().min(3);
                    let any_top3_violates = edge_results[..top_count]
                        .iter()
                        .any(|&(_, g, _, _)| g < -1e-12);
                    if any_top3_violates {
                        top_edge_violations += 1;
                    }
                }

                eprintln!("\r                                                                    ");
                println!("  N={}, τ={}, δ_min ≥ {}:", n, tau, min_deg);
                println!("    Graphs: {}", graph_count);
                println!(
                    "    Graphs with locally-positive edges: {}",
                    any_positive_edges
                );
                println!(
                    "    MAX-ΔS edge violations: {} ({:.6}%)",
                    max_edge_violations,
                    if any_positive_edges > 0 {
                        100.0 * max_edge_violations as f64 / any_positive_edges as f64
                    } else {
                        0.0
                    }
                );
                println!(
                    "    Top-3 edge violations: {} ({:.6}%)",
                    top_edge_violations,
                    if any_positive_edges > 0 {
                        100.0 * top_edge_violations as f64 / any_positive_edges as f64
                    } else {
                        0.0
                    }
                );
                println!(
                    "    Worst global ΔS on max edge: {:+.8e}",
                    worst_global_on_max
                );
                println!(
                    "    Largest local gain that still violated: {:+.8e}",
                    best_local_that_violates
                );
                println!();
            }
        }
    }

    println!("════════════════════════════════════════════════════════════════");
    println!("  If max-edge violations = 0: protocol is safe by construction");
    println!("  (no need for the filter theorem on general graphs)");
    println!();
}
