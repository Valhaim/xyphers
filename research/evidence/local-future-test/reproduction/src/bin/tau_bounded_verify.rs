//! τ-Bounded Filter Theorem Verification
//!
//! Tests the filter theorem at small τ (3, 4, 5) for N beyond the exhaustive
//! regime (N ≤ 6). Provides computational evidence for the conjecture:
//!
//!   There exists C ≥ 5 such that the filter theorem holds for
//!   all connected unit-weight graphs at τ ≤ C, all N.
//!
//! Phase 1: All labeled trees via Prüfer sequences (N = 7, 8, 9)
//! Phase 2: All connected graphs on N = 7
//! Phase 3: Random connected graphs at N = 10, 15, 20, 30, 50
//!
//! Run: cargo run --release --bin tau-bounded-verify

use std::collections::{BTreeSet, VecDeque};

// ─── Dense matrix operations ──────────────────────────────────────────

/// Multiply two row-major N×N matrices
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

/// Compute P^τ by repeated squaring
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

/// Shannon entropy of a distribution (in bits)
fn shannon_entropy(dist: &[f64]) -> f64 {
    let mut h = 0.0;
    for &p in dist {
        if p > 1e-15 {
            h -= p * p.log2();
        }
    }
    h
}

/// Build transition matrix from adjacency list (unit weights)
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

/// Compute per-node τ-step entropy: S_τ(i) = H(e_i · P^τ)
fn per_node_entropy(p_tau: &[f64], n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| {
            let row = &p_tau[i * n..(i + 1) * n];
            shannon_entropy(row)
        })
        .collect()
}

// ─── Graph utilities ──────────────────────────────────────────────────

/// Check if adjacency list represents a connected graph
fn is_connected(adj: &[Vec<usize>], n: usize) -> bool {
    if n == 0 {
        return true;
    }
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

/// Build adjacency list from edge set
fn edges_to_adj(edges: &[(usize, usize)], n: usize) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    adj
}

/// Prüfer sequence → tree edges (Cayley's formula: n^(n-2) labeled trees)
fn prufer_to_tree(seq: &[usize], n: usize) -> Vec<(usize, usize)> {
    let mut degree = vec![1usize; n];
    for &s in seq {
        degree[s] += 1;
    }

    let mut edges = Vec::with_capacity(n - 1);
    let mut ptr = 0;
    // Find the smallest leaf
    while degree[ptr] != 1 {
        ptr += 1;
    }
    let mut leaf = ptr;

    for &s in seq {
        edges.push((leaf.min(s), leaf.max(s)));
        degree[s] -= 1;
        degree[leaf] -= 1;
        if degree[s] == 1 && s < ptr {
            leaf = s;
        } else {
            ptr += 1;
            while ptr < n && degree[ptr] != 1 {
                ptr += 1;
            }
            leaf = ptr;
        }
    }
    // Last edge: connect the two remaining degree-1 nodes
    let remaining: Vec<usize> = (0..n).filter(|&i| degree[i] == 1).collect();
    if remaining.len() == 2 {
        edges.push((
            remaining[0].min(remaining[1]),
            remaining[0].max(remaining[1]),
        ));
    }
    edges
}

/// Enumerate all Prüfer sequences of length n-2 with values in 0..n
fn enumerate_prufer(n: usize) -> Vec<Vec<usize>> {
    let len = n - 2;
    if len == 0 {
        return vec![vec![]];
    }
    let total = n.pow(len as u32);
    let mut result = Vec::with_capacity(total);
    for i in 0..total {
        let mut seq = vec![0usize; len];
        let mut val = i;
        for k in (0..len).rev() {
            seq[k] = val % n;
            val /= n;
        }
        result.push(seq);
    }
    result
}

// ─── Filter theorem check ─────────────────────────────────────────────

struct FilterCheck {
    n: usize,
    u: usize,
    v: usize,
    tau: u32,
    local_delta: f64,
    global_delta: f64,
    affected_size: usize,
    is_violation: bool,
}

/// Check the filter theorem for adding edge (u,v) at horizon tau
fn check_filter(adj: &[Vec<usize>], n: usize, u: usize, v: usize, tau: u32) -> FilterCheck {
    // Build transition matrices before and after
    let p_before = build_transition_matrix(adj, n);

    let mut adj_after = adj.to_vec();
    adj_after[u].push(v);
    adj_after[v].push(u);
    let p_after = build_transition_matrix(&adj_after, n);

    // Compute P^τ
    let pt_before = mat_pow(&p_before, n, tau);
    let pt_after = mat_pow(&p_after, n, tau);

    // Per-node entropies
    let ent_before = per_node_entropy(&pt_before, n);
    let ent_after = per_node_entropy(&pt_after, n);

    // Affected set: {u,v} ∪ N(u) ∪ N(v)
    let mut affected = BTreeSet::new();
    affected.insert(u);
    affected.insert(v);
    for &nb in &adj[u] {
        affected.insert(nb);
    }
    for &nb in &adj[v] {
        affected.insert(nb);
    }

    // Local delta (average over affected set)
    let local_sum: f64 = affected.iter().map(|&w| ent_after[w] - ent_before[w]).sum();
    let local_delta = local_sum / affected.len() as f64;

    // Global delta (average over all nodes)
    let global_sum: f64 = (0..n).map(|w| ent_after[w] - ent_before[w]).sum();
    let global_delta = global_sum / n as f64;

    let is_violation = local_delta > 1e-12 && global_delta < -1e-12;

    FilterCheck {
        n,
        u,
        v,
        tau,
        local_delta,
        global_delta,
        affected_size: affected.len(),
        is_violation,
    }
}

// ─── Random graph generation ──────────────────────────────────────────

/// Simple LCG for reproducible randomness (no dep on rand crate features)
struct Rng {
    state: u64,
}
impl Rng {
    fn new(seed: u64) -> Self {
        Rng { state: seed }
    }
    fn next(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state >> 33
    }
    fn next_usize(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }
    fn next_f64(&mut self) -> f64 {
        (self.next() as f64) / (u64::MAX >> 33) as f64
    }
}

/// Generate a random connected graph on n nodes with edge probability p
fn random_connected_graph(rng: &mut Rng, n: usize, p: f64) -> Vec<Vec<usize>> {
    loop {
        let mut adj = vec![vec![]; n];
        for i in 0..n {
            for j in i + 1..n {
                if rng.next_f64() < p {
                    adj[i].push(j);
                    adj[j].push(i);
                }
            }
        }
        if is_connected(&adj, n) {
            return adj;
        }
    }
}

/// Generate a random tree on n nodes (random Prüfer sequence)
fn random_tree(rng: &mut Rng, n: usize) -> Vec<Vec<usize>> {
    let seq: Vec<usize> = (0..n - 2).map(|_| rng.next_usize(n)).collect();
    let edges = prufer_to_tree(&seq, n);
    edges_to_adj(&edges, n)
}

// ─── Main ─────────────────────────────────────────────────────────────

fn main() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  τ-BOUNDED FILTER THEOREM VERIFICATION                     ║");
    println!("║  Testing τ = 3, 4, 5 for N > 6                            ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    let taus = [3u32, 4, 5];
    let mut total_checks = 0u64;
    let mut total_violations = 0u64;

    // ══════════════════════════════════════════════════════════════════
    // Phase 1: All labeled trees via Prüfer sequences
    // ══════════════════════════════════════════════════════════════════

    for n in [7usize, 8, 9] {
        println!("── Phase 1: All labeled trees on N = {} ──", n);
        let num_trees = n.pow((n - 2) as u32);
        println!("   Trees to test: {} (Cayley: {}^{})", num_trees, n, n - 2);

        let mut phase_checks = 0u64;
        let mut phase_violations = 0u64;
        let mut edge_set_cache = BTreeSet::new();

        let prufer_seqs = enumerate_prufer(n);
        for (ti, seq) in prufer_seqs.iter().enumerate() {
            let edges = prufer_to_tree(seq, n);
            let adj = edges_to_adj(&edges, n);

            // Canonical edge set for dedup (labeled trees can share structure)
            edge_set_cache.clear();
            for &(a, b) in &edges {
                edge_set_cache.insert((a.min(b), a.max(b)));
            }

            // All non-edges
            for u in 0..n {
                for v in u + 1..n {
                    if edge_set_cache.contains(&(u, v)) {
                        continue;
                    }

                    for &tau in &taus {
                        let result = check_filter(&adj, n, u, v, tau);
                        phase_checks += 1;

                        if result.is_violation {
                            phase_violations += 1;
                            println!("   *** VIOLATION at N={}, τ={}, edge ({},{})", n, tau, u, v);
                            println!(
                                "       ΔS_local = {:.9}, ΔS_global = {:.9}",
                                result.local_delta, result.global_delta
                            );
                            println!("       Prüfer: {:?}", seq);
                            println!("       Edges: {:?}", edges);
                        }
                    }
                }
            }

            if (ti + 1) % (num_trees / 10).max(1) == 0 {
                let pct = 100.0 * (ti + 1) as f64 / num_trees as f64;
                print!(
                    "   [{:.0}%] tree {}/{}, checks so far: {}, violations: {}\r",
                    pct,
                    ti + 1,
                    num_trees,
                    phase_checks,
                    phase_violations
                );
            }
        }

        println!();
        println!(
            "   N={}: {} checks, {} violations",
            n, phase_checks, phase_violations
        );
        total_checks += phase_checks;
        total_violations += phase_violations;
        println!();
    }

    // ══════════════════════════════════════════════════════════════════
    // Phase 2: All connected graphs on N = 7
    // ══════════════════════════════════════════════════════════════════

    println!("── Phase 2: All connected graphs on N = 7 ──");
    let n = 7usize;
    let max_edges = n * (n - 1) / 2; // 21
    let total_subsets = 1u64 << max_edges;
    println!(
        "   Edge subsets to scan: {} (2^{})",
        total_subsets, max_edges
    );

    // All possible edges in order
    let all_edges: Vec<(usize, usize)> = {
        let mut e = Vec::new();
        for i in 0..n {
            for j in i + 1..n {
                e.push((i, j));
            }
        }
        e
    };

    let mut phase_checks = 0u64;
    let mut phase_violations = 0u64;
    let mut connected_count = 0u64;

    for mask in 0..total_subsets {
        // Build edge list from bitmask
        let edges: Vec<(usize, usize)> = (0..max_edges)
            .filter(|&bit| mask & (1u64 << bit) != 0)
            .map(|bit| all_edges[bit])
            .collect();

        // Need at least n-1 = 6 edges for connectivity
        if edges.len() < n - 1 {
            continue;
        }

        let adj = edges_to_adj(&edges, n);
        if !is_connected(&adj, n) {
            continue;
        }
        connected_count += 1;

        let edge_set: BTreeSet<(usize, usize)> = edges.iter().copied().collect();

        // Test all non-edges
        for &(u, v) in &all_edges {
            if edge_set.contains(&(u, v)) {
                continue;
            }

            for &tau in &taus {
                let result = check_filter(&adj, n, u, v, tau);
                phase_checks += 1;

                if result.is_violation {
                    phase_violations += 1;
                    println!("   *** VIOLATION at N={}, τ={}, edge ({},{})", n, tau, u, v);
                    println!(
                        "       ΔS_local = {:.9}, ΔS_global = {:.9}",
                        result.local_delta, result.global_delta
                    );
                    println!("       Edges: {:?}", edges);
                }
            }
        }

        if connected_count % 100_000 == 0 {
            let pct = 100.0 * mask as f64 / total_subsets as f64;
            print!(
                "   [{:.0}%] {} connected graphs, {} checks, {} violations\r",
                pct, connected_count, phase_checks, phase_violations
            );
        }
    }

    println!();
    println!(
        "   N=7: {} connected graphs, {} checks, {} violations",
        connected_count, phase_checks, phase_violations
    );
    total_checks += phase_checks;
    total_violations += phase_violations;
    println!();

    // ══════════════════════════════════════════════════════════════════
    // Phase 3: Random connected graphs at larger N
    // ══════════════════════════════════════════════════════════════════

    println!("── Phase 3: Random connected graphs ──");

    let configs: Vec<(usize, f64, usize)> = vec![
        (10, 0.3, 5_000),
        (10, 0.5, 5_000),
        (15, 0.3, 3_000),
        (15, 0.5, 3_000),
        (20, 0.3, 2_000),
        (20, 0.5, 2_000),
        (30, 0.3, 1_000),
        (50, 0.3, 500),
    ];

    // Also test random trees (worst case for filter theorem)
    let tree_configs: Vec<(usize, usize)> = vec![
        (10, 10_000),
        (15, 5_000),
        (20, 3_000),
        (30, 2_000),
        (50, 1_000),
        (100, 500),
    ];

    let mut phase_checks = 0u64;
    let mut phase_violations = 0u64;
    let mut rng = Rng::new(42);

    // Random graphs (Erdős-Rényi)
    for &(n, p, samples) in &configs {
        let mut sub_checks = 0u64;
        let mut sub_violations = 0u64;

        for _ in 0..samples {
            let adj = random_connected_graph(&mut rng, n, p);
            let edge_set: BTreeSet<(usize, usize)> = {
                let mut s = BTreeSet::new();
                for i in 0..n {
                    for &j in &adj[i] {
                        if i < j {
                            s.insert((i, j));
                        }
                    }
                }
                s
            };

            // Sample up to 10 random non-edges
            let non_edges: Vec<(usize, usize)> = {
                let mut ne = Vec::new();
                for i in 0..n {
                    for j in i + 1..n {
                        if !edge_set.contains(&(i, j)) {
                            ne.push((i, j));
                        }
                    }
                }
                // Shuffle and take up to 10
                for k in 0..ne.len().min(20) {
                    let swap = k + rng.next_usize(ne.len() - k);
                    ne.swap(k, swap);
                }
                ne.truncate(10);
                ne
            };

            for &(u, v) in &non_edges {
                for &tau in &taus {
                    let result = check_filter(&adj, n, u, v, tau);
                    sub_checks += 1;

                    if result.is_violation {
                        sub_violations += 1;
                        println!(
                            "   *** VIOLATION at N={}, p={}, τ={}, edge ({},{})",
                            n, p, tau, u, v
                        );
                        println!(
                            "       ΔS_local = {:.9}, ΔS_global = {:.9}",
                            result.local_delta, result.global_delta
                        );
                    }
                }
            }
        }

        println!(
            "   G(N={}, p={:.1}): {} samples, {} checks, {} violations",
            n, p, samples, sub_checks, sub_violations
        );
        phase_checks += sub_checks;
        phase_violations += sub_violations;
    }

    // Random trees (worst case)
    println!();
    println!("   Random trees (worst case for filter theorem):");
    for &(n, samples) in &tree_configs {
        let mut sub_checks = 0u64;
        let mut sub_violations = 0u64;

        for _ in 0..samples {
            let adj = random_tree(&mut rng, n);
            let edge_set: BTreeSet<(usize, usize)> = {
                let mut s = BTreeSet::new();
                for i in 0..n {
                    for &j in &adj[i] {
                        if i < j {
                            s.insert((i, j));
                        }
                    }
                }
                s
            };

            // Sample non-edges
            let non_edges: Vec<(usize, usize)> = {
                let mut ne = Vec::new();
                for i in 0..n {
                    for j in i + 1..n {
                        if !edge_set.contains(&(i, j)) {
                            ne.push((i, j));
                        }
                    }
                }
                for k in 0..ne.len().min(20) {
                    let swap = k + rng.next_usize(ne.len() - k);
                    ne.swap(k, swap);
                }
                ne.truncate(10);
                ne
            };

            for &(u, v) in &non_edges {
                for &tau in &taus {
                    let result = check_filter(&adj, n, u, v, tau);
                    sub_checks += 1;

                    if result.is_violation {
                        sub_violations += 1;
                        println!(
                            "   *** VIOLATION at N={}, tree, τ={}, edge ({},{})",
                            n, tau, u, v
                        );
                        println!(
                            "       ΔS_local = {:.9}, ΔS_global = {:.9}",
                            result.local_delta, result.global_delta
                        );
                    }
                }
            }
        }

        println!(
            "   Tree(N={}): {} samples, {} checks, {} violations",
            n, samples, sub_checks, sub_violations
        );
        phase_checks += sub_checks;
        phase_violations += sub_violations;
    }

    total_checks += phase_checks;
    total_violations += phase_violations;

    // ══════════════════════════════════════════════════════════════════
    // Summary
    // ══════════════════════════════════════════════════════════════════

    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  SUMMARY                                                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    println!("  Total checks: {}", total_checks);
    println!("  Total violations at τ ∈ {{3,4,5}}: {}", total_violations);
    println!();

    if total_violations == 0 {
        println!("  RESULT: Zero violations found at τ ≤ 5 for any N tested.");
        println!();
        println!("  This supports the Bounded Horizon Conjecture:");
        println!("  The filter theorem holds for all connected unit-weight");
        println!("  graphs at τ ≤ 5, for all N.");
        println!();
        println!("  Combined with the existing exhaustive result (N ≤ 6, τ ≤ 5),");
        println!("  and the τ = 2 algebraic proof (all N), this provides strong");
        println!("  evidence that the operational regime (τ = 5) is safe.");
    } else {
        println!("  WARNING: {} violations found!", total_violations);
        println!("  The bounded horizon conjecture at τ ≤ 5 is FALSIFIED.");
    }
    println!();
}
