use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::types::{EntropyResult, NodeId, WeightedAdj};

/// Threshold: use sparse CSR matrix when N exceeds this.
const SPARSE_THRESHOLD: usize = 200;

struct TransitionMatrix {
    node_ids: Vec<NodeId>, // Sorted — use binary search for O(log N) lookups
    p: Vec<f64>,           // Row-major n×n
    degrees: Vec<f64>,     // Total weight per node (from adjacency)
    n: usize,
}

impl TransitionMatrix {
    /// O(log N) index lookup via binary search on sorted node_ids.
    /// Much faster than BTreeMap due to cache locality (contiguous Vec).
    #[inline]
    fn idx_of(&self, id: NodeId) -> Option<usize> {
        self.node_ids.binary_search(&id).ok()
    }
}

/// Build row-stochastic transition matrix from weighted adjacency.
fn build_transition_matrix(adj: &WeightedAdj) -> TransitionMatrix {
    let mut node_ids: Vec<NodeId> = adj.keys().copied().collect();
    node_ids.sort();
    let n = node_ids.len();

    let mut p = vec![0.0f64; n * n];
    let mut degrees = vec![0.0f64; n];

    for (i, &node_id) in node_ids.iter().enumerate() {
        let neighbors = match adj.get(&node_id) {
            Some(n) if !n.is_empty() => n,
            _ => {
                // Absorbing state: self-loop
                p[i * n + i] = 1.0;
                // degrees[i] stays 0.0 (isolated)
                continue;
            }
        };

        let total_weight: f64 = neighbors.iter().map(|(_, w)| w).sum();
        if total_weight == 0.0 {
            p[i * n + i] = 1.0;
            continue;
        }
        degrees[i] = total_weight;

        for &(neighbor_id, weight) in neighbors {
            if let Ok(j) = node_ids.binary_search(&neighbor_id) {
                p[i * n + j] = weight / total_weight;
            }
        }
    }

    TransitionMatrix {
        node_ids,
        p,
        degrees,
        n,
    }
}

/// Compute endpoint distribution: e_startIdx · P^τ via repeated vector-matrix multiply.
fn endpoint_distribution(p: &[f64], n: usize, start_idx: usize, horizon: u32) -> Vec<f64> {
    debug_assert!(start_idx < n);
    debug_assert_eq!(p.len(), n * n);

    let mut current = vec![0.0f64; n];
    let mut next = vec![0.0f64; n];
    current[start_idx] = 1.0;

    for _step in 0..horizon {
        // Zero out next
        for x in next.iter_mut() {
            *x = 0.0;
        }
        // Vector-matrix multiply: next[j] = Σ_i current[i] · P[i*n + j]
        for i in 0..n {
            let c = current[i];
            if c == 0.0 {
                continue;
            }
            let row_offset = i * n;
            for j in 0..n {
                // Safety: dimensions validated above
                unsafe {
                    *next.get_unchecked_mut(j) += c * *p.get_unchecked(row_offset + j);
                }
            }
        }
        std::mem::swap(&mut current, &mut next);
    }

    current
}

/// Endpoint distribution with pre-allocated buffers (avoids allocation in hot loops).
/// After return, `current` contains the distribution.
#[cfg(not(feature = "parallel"))]
fn endpoint_distribution_reuse(
    p: &[f64],
    n: usize,
    start_idx: usize,
    horizon: u32,
    current: &mut Vec<f64>,
    next: &mut Vec<f64>,
) {
    debug_assert!(start_idx < n);
    debug_assert_eq!(p.len(), n * n);

    for x in current.iter_mut() {
        *x = 0.0;
    }
    current[start_idx] = 1.0;

    for _step in 0..horizon {
        for x in next.iter_mut() {
            *x = 0.0;
        }
        for i in 0..n {
            let c = current[i];
            if c == 0.0 {
                continue;
            }
            let row_offset = i * n;
            for j in 0..n {
                unsafe {
                    *next.get_unchecked_mut(j) += c * *p.get_unchecked(row_offset + j);
                }
            }
        }
        std::mem::swap(current, next);
    }
}

/// Shannon entropy (bits) of a probability distribution.
fn shannon_entropy(dist: &[f64]) -> f64 {
    let mut h = 0.0;
    for &p in dist {
        if p > 1e-15 {
            h -= p * crate::canonical_math::log2(p);
        }
    }
    h
}

/// Exact S_τ for a single node.
pub fn node_entropy_exact(adj: &WeightedAdj, node_id: NodeId, horizon: u32) -> f64 {
    let tm = build_transition_matrix(adj);
    match tm.idx_of(node_id) {
        Some(idx) => {
            let dist = endpoint_distribution(&tm.p, tm.n, idx, horizon);
            shannon_entropy(&dist)
        }
        None => 0.0,
    }
}

/// Exact S_τ for all nodes + global average.
pub fn compute_all_entropies_exact(adj: &WeightedAdj, horizon: u32) -> EntropyResult {
    if adj.is_empty() {
        return EntropyResult {
            per_node: BTreeMap::new(),
            global: 0.0,
        };
    }

    let tm = build_transition_matrix(adj);

    #[cfg(feature = "parallel")]
    let results: Vec<(NodeId, f64)> = (0..tm.n)
        .into_par_iter()
        .map(|i| {
            let dist = endpoint_distribution(&tm.p, tm.n, i, horizon);
            (tm.node_ids[i], shannon_entropy(&dist))
        })
        .collect();

    #[cfg(not(feature = "parallel"))]
    let results: Vec<(NodeId, f64)> = (0..tm.n)
        .map(|i| {
            let dist = endpoint_distribution(&tm.p, tm.n, i, horizon);
            (tm.node_ids[i], shannon_entropy(&dist))
        })
        .collect();

    let mut per_node = BTreeMap::new();
    let mut total = 0.0;
    for (id, e) in &results {
        per_node.insert(*id, *e);
        total += e;
    }

    EntropyResult {
        per_node,
        global: total / tm.n as f64,
    }
}

/// Exact ΔS_τ for a single candidate edge.
pub fn delta_st_exact(adj: &WeightedAdj, from_id: NodeId, to_id: NodeId, horizon: u32) -> f64 {
    // Get affected nodes
    let affected = get_affected_nodes(adj, from_id, to_id);

    // Before
    let tm_before = build_transition_matrix(adj);
    let mut before_sum = 0.0;
    for &node_id in &affected {
        if let Some(idx) = tm_before.idx_of(node_id) {
            let dist = endpoint_distribution(&tm_before.p, tm_before.n, idx, horizon);
            before_sum += shannon_entropy(&dist);
        }
    }

    // Clone and add edge
    let adj_after = add_candidate_edge(adj, from_id, to_id);
    let tm_after = build_transition_matrix(&adj_after);
    let mut after_sum = 0.0;
    for &node_id in &affected {
        if let Some(idx) = tm_after.idx_of(node_id) {
            let dist = endpoint_distribution(&tm_after.p, tm_after.n, idx, horizon);
            after_sum += shannon_entropy(&dist);
        }
    }

    (after_sum - before_sum) / affected.len() as f64
}

/// Batch evaluation: build transition matrix once, pre-compute entropy cache.
pub fn batch_delta_st_exact(
    adj: &WeightedAdj,
    candidates: &[(NodeId, NodeId)],
    horizon: u32,
) -> BTreeMap<(NodeId, NodeId), f64> {
    if candidates.is_empty() {
        return BTreeMap::new();
    }

    // Pre-build transition matrix and entropy cache
    let tm_before = build_transition_matrix(adj);
    let entropy_cache: Vec<f64> = (0..tm_before.n)
        .map(|i| {
            let dist = endpoint_distribution(&tm_before.p, tm_before.n, i, horizon);
            shannon_entropy(&dist)
        })
        .collect();

    #[cfg(feature = "parallel")]
    let results: Vec<((NodeId, NodeId), f64)> = candidates
        .par_iter()
        .map(|&(from_id, to_id)| {
            let affected = get_affected_nodes(adj, from_id, to_id);

            // Before sum from cache
            let mut before_sum = 0.0;
            for &node_id in &affected {
                if let Some(idx) = tm_before.idx_of(node_id) {
                    before_sum += entropy_cache[idx];
                }
            }

            // Clone and add edge
            let adj_after = add_candidate_edge(adj, from_id, to_id);
            let tm_after = build_transition_matrix(&adj_after);
            let mut after_sum = 0.0;
            for &node_id in &affected {
                if let Some(idx) = tm_after.idx_of(node_id) {
                    let dist = endpoint_distribution(&tm_after.p, tm_after.n, idx, horizon);
                    after_sum += shannon_entropy(&dist);
                }
            }

            (
                (from_id, to_id),
                (after_sum - before_sum) / affected.len() as f64,
            )
        })
        .collect();

    #[cfg(not(feature = "parallel"))]
    let results: Vec<((NodeId, NodeId), f64)> = candidates
        .iter()
        .map(|&(from_id, to_id)| {
            let affected = get_affected_nodes(adj, from_id, to_id);

            let mut before_sum = 0.0;
            for &node_id in &affected {
                if let Some(idx) = tm_before.idx_of(node_id) {
                    before_sum += entropy_cache[idx];
                }
            }

            let adj_after = add_candidate_edge(adj, from_id, to_id);
            let tm_after = build_transition_matrix(&adj_after);
            let mut after_sum = 0.0;
            for &node_id in &affected {
                if let Some(idx) = tm_after.idx_of(node_id) {
                    let dist = endpoint_distribution(&tm_after.p, tm_after.n, idx, horizon);
                    after_sum += shannon_entropy(&dist);
                }
            }

            (
                (from_id, to_id),
                (after_sum - before_sum) / affected.len() as f64,
            )
        })
        .collect();

    results.into_iter().collect()
}

// ── Incremental batch evaluation (dense) ──
//
// Instead of cloning the adjacency and rebuilding the transition matrix for
// each candidate edge, patch only the 2 affected rows and share the base matrix.
// Eliminates O(N·d̄) rebuild per candidate → O(N) row-patch per candidate.

/// Endpoint distribution with 2 rows overridden, using pre-allocated buffers.
/// After return, `current` contains the distribution.
fn endpoint_distribution_with_overrides_reuse(
    base_p: &[f64],
    n: usize,
    row_u: usize,
    patched_u: &[f64],
    row_v: usize,
    patched_v: &[f64],
    start_idx: usize,
    horizon: u32,
    current: &mut Vec<f64>,
    next: &mut Vec<f64>,
) {
    debug_assert!(start_idx < n);
    debug_assert_eq!(base_p.len(), n * n);
    debug_assert_eq!(patched_u.len(), n);
    debug_assert_eq!(patched_v.len(), n);

    for x in current.iter_mut() {
        *x = 0.0;
    }
    current[start_idx] = 1.0;

    for _step in 0..horizon {
        for x in next.iter_mut() {
            *x = 0.0;
        }
        for i in 0..n {
            let c = current[i];
            if c == 0.0 {
                continue;
            }
            let row = if i == row_u {
                patched_u
            } else if i == row_v {
                patched_v
            } else {
                &base_p[i * n..(i + 1) * n]
            };
            for j in 0..n {
                unsafe {
                    *next.get_unchecked_mut(j) += c * *row.get_unchecked(j);
                }
            }
        }
        std::mem::swap(current, next);
    }
}

/// Build the patched row for adding edge (row_idx → new_col) with weight 1.0.
///
/// `old_total_weight` is the sum of edge weights for this node (from adjacency).
/// New P[row_idx][j] = old_w(j) / (old_total_weight + 1.0) for existing neighbors,
/// and 1.0 / (old_total_weight + 1.0) for the new edge.
fn build_patched_row(
    base_p: &[f64],
    n: usize,
    row_idx: usize,
    new_col: usize,
    old_total_weight: f64,
) -> Vec<f64> {
    let base_row = &base_p[row_idx * n..(row_idx + 1) * n];
    let new_total = old_total_weight + 1.0;

    // scale = old_total / new_total: rescales P_old[i][j] * old_total (= w(i,j)) / new_total
    let scale = if old_total_weight > 1e-15 {
        old_total_weight / new_total
    } else {
        0.0
    };

    let mut patched = vec![0.0f64; n];
    for j in 0..n {
        if j == new_col {
            // New edge: 1.0 / new_total, plus any existing weight (should be 0 for non-edges)
            patched[j] = base_row[j] * scale + 1.0 / new_total;
        } else if j == row_idx && old_total_weight < 1e-15 {
            // Was an isolated node with self-loop; self-loop disappears
            patched[j] = 0.0;
        } else {
            patched[j] = base_row[j] * scale;
        }
    }

    patched
}

/// Incremental batch ΔS_τ: build the transition matrix once, patch 2 rows per candidate.
///
/// Functionally identical to `batch_delta_st_exact` but avoids rebuilding the
/// entire transition matrix for each candidate edge. For K candidates on a graph
/// with N nodes: old cost = O(K·N²), new cost = O(N² + K·N).
pub fn batch_delta_st_incremental(
    adj: &WeightedAdj,
    candidates: &[(NodeId, NodeId)],
    horizon: u32,
) -> Vec<f64> {
    if candidates.is_empty() {
        return Vec::new();
    }

    // Build base transition matrix and entropy cache (shared across candidates)
    let tm = build_transition_matrix(adj);

    // Entropy cache: parallel path uses per-thread alloc, sequential reuses buffers
    #[cfg(feature = "parallel")]
    let entropy_cache: Vec<f64> = (0..tm.n)
        .into_par_iter()
        .map(|i| {
            let dist = endpoint_distribution(&tm.p, tm.n, i, horizon);
            shannon_entropy(&dist)
        })
        .collect();

    #[cfg(not(feature = "parallel"))]
    let entropy_cache: Vec<f64> = {
        let mut buf_a = vec![0.0f64; tm.n];
        let mut buf_b = vec![0.0f64; tm.n];
        (0..tm.n)
            .map(|i| {
                endpoint_distribution_reuse(&tm.p, tm.n, i, horizon, &mut buf_a, &mut buf_b);
                shannon_entropy(&buf_a)
            })
            .collect()
    };

    // Per-candidate evaluation: parallel reuses buffers within each task,
    // sequential reuses buffers across all candidates.
    #[cfg(feature = "parallel")]
    let results: Vec<f64> = candidates
        .par_iter()
        .map(|&(from_id, to_id)| {
            let affected = get_affected_nodes(adj, from_id, to_id);

            let mut before_sum = 0.0;
            for &node_id in &affected {
                if let Some(idx) = tm.idx_of(node_id) {
                    before_sum += entropy_cache[idx];
                }
            }

            let u = tm.idx_of(from_id).unwrap();
            let v = tm.idx_of(to_id).unwrap();
            let patched_u = build_patched_row(&tm.p, tm.n, u, v, tm.degrees[u]);
            let patched_v = build_patched_row(&tm.p, tm.n, v, u, tm.degrees[v]);

            // Reuse buffers across all affected nodes for this candidate
            let mut buf_a = vec![0.0f64; tm.n];
            let mut buf_b = vec![0.0f64; tm.n];
            let mut after_sum = 0.0;
            for &node_id in &affected {
                if let Some(idx) = tm.idx_of(node_id) {
                    endpoint_distribution_with_overrides_reuse(
                        &tm.p, tm.n, u, &patched_u, v, &patched_v, idx, horizon, &mut buf_a,
                        &mut buf_b,
                    );
                    after_sum += shannon_entropy(&buf_a);
                }
            }

            (after_sum - before_sum) / affected.len() as f64
        })
        .collect();

    #[cfg(not(feature = "parallel"))]
    let results: Vec<f64> = {
        let mut buf_a = vec![0.0f64; tm.n];
        let mut buf_b = vec![0.0f64; tm.n];
        candidates
            .iter()
            .map(|&(from_id, to_id)| {
                let affected = get_affected_nodes(adj, from_id, to_id);

                let mut before_sum = 0.0;
                for &node_id in &affected {
                    if let Some(idx) = tm.idx_of(node_id) {
                        before_sum += entropy_cache[idx];
                    }
                }

                let u = tm.idx_of(from_id).unwrap();
                let v = tm.idx_of(to_id).unwrap();
                let patched_u = build_patched_row(&tm.p, tm.n, u, v, tm.degrees[u]);
                let patched_v = build_patched_row(&tm.p, tm.n, v, u, tm.degrees[v]);

                let mut after_sum = 0.0;
                for &node_id in &affected {
                    if let Some(idx) = tm.idx_of(node_id) {
                        endpoint_distribution_with_overrides_reuse(
                            &tm.p, tm.n, u, &patched_u, v, &patched_v, idx, horizon, &mut buf_a,
                            &mut buf_b,
                        );
                        after_sum += shannon_entropy(&buf_a);
                    }
                }

                (after_sum - before_sum) / affected.len() as f64
            })
            .collect()
    };

    results
}

// ── Sparse CSR Matrix ──

/// CSR (Compressed Sparse Row) transition matrix — O(nnz) per multiply.
/// For a graph with average degree d̄, nnz ≈ N·d̄, so multiply is O(N·d̄)
/// instead of O(N²) for the dense version.
pub struct SparseTransitionMatrix {
    pub node_ids: Vec<NodeId>,
    pub idx_map: BTreeMap<NodeId, usize>,
    /// row_ptr[i]..row_ptr[i+1] gives the range of non-zero entries for row i
    row_ptr: Vec<usize>,
    /// Column indices of non-zero entries
    col_idx: Vec<usize>,
    /// Transition probabilities
    values: Vec<f64>,
    pub n: usize,
}

/// Build sparse row-stochastic transition matrix from weighted adjacency.
pub fn build_sparse_transition_matrix(adj: &WeightedAdj) -> SparseTransitionMatrix {
    let mut node_ids: Vec<NodeId> = adj.keys().copied().collect();
    node_ids.sort();
    let n = node_ids.len();
    let mut idx_map = BTreeMap::new();
    for (i, &id) in node_ids.iter().enumerate() {
        idx_map.insert(id, i);
    }

    // Pre-estimate capacity: avg degree × N
    let est_nnz: usize = adj.values().map(|v| v.len().max(1)).sum();
    let mut row_ptr = Vec::with_capacity(n + 1);
    let mut col_idx = Vec::with_capacity(est_nnz);
    let mut values = Vec::with_capacity(est_nnz);

    for (i, &node_id) in node_ids.iter().enumerate() {
        row_ptr.push(col_idx.len());

        let neighbors = match adj.get(&node_id) {
            Some(n) if !n.is_empty() => n,
            _ => {
                // Absorbing state: self-loop
                col_idx.push(i);
                values.push(1.0);
                continue;
            }
        };

        let total_weight: f64 = neighbors.iter().map(|(_, w)| w).sum();
        if total_weight == 0.0 {
            col_idx.push(i);
            values.push(1.0);
            continue;
        }

        for &(neighbor_id, weight) in neighbors {
            if let Some(&j) = idx_map.get(&neighbor_id) {
                col_idx.push(j);
                values.push(weight / total_weight);
            }
        }
    }
    row_ptr.push(col_idx.len());

    SparseTransitionMatrix {
        node_ids,
        idx_map,
        row_ptr,
        col_idx,
        values,
        n,
    }
}

/// Sparse endpoint distribution using pre-allocated buffers (avoids allocation in hot loop).
/// After return, `current` contains the endpoint distribution.
fn endpoint_distribution_sparse_reuse(
    tm: &SparseTransitionMatrix,
    start_idx: usize,
    horizon: u32,
    current: &mut Vec<f64>,
    next: &mut Vec<f64>,
) {
    let n = tm.n;
    for x in current.iter_mut() {
        *x = 0.0;
    }
    current[start_idx] = 1.0;

    for _step in 0..horizon {
        for x in next.iter_mut() {
            *x = 0.0;
        }
        for i in 0..n {
            let c = current[i];
            if c == 0.0 {
                continue;
            }
            let row_start = tm.row_ptr[i];
            let row_end = tm.row_ptr[i + 1];
            for k in row_start..row_end {
                // SAFETY: col_idx values are produced from idx_map (< n),
                // row_ptr bounds are validated by CSR construction.
                unsafe {
                    *next.get_unchecked_mut(*tm.col_idx.get_unchecked(k)) +=
                        c * *tm.values.get_unchecked(k);
                }
            }
        }
        std::mem::swap(current, next);
    }
}

/// Sparse endpoint distribution: e_startIdx · P^τ (allocating version).
fn endpoint_distribution_sparse(
    tm: &SparseTransitionMatrix,
    start_idx: usize,
    horizon: u32,
) -> Vec<f64> {
    let mut current = vec![0.0f64; tm.n];
    let mut next = vec![0.0f64; tm.n];
    endpoint_distribution_sparse_reuse(tm, start_idx, horizon, &mut current, &mut next);
    current
}

/// Exact S_τ for all nodes using sparse matrix.
pub fn compute_all_entropies_sparse(adj: &WeightedAdj, horizon: u32) -> EntropyResult {
    if adj.is_empty() {
        return EntropyResult {
            per_node: BTreeMap::new(),
            global: 0.0,
        };
    }

    let tm = build_sparse_transition_matrix(adj);

    #[cfg(feature = "parallel")]
    let results: Vec<(NodeId, f64)> = (0..tm.n)
        .into_par_iter()
        .map(|i| {
            let dist = endpoint_distribution_sparse(&tm, i, horizon);
            (tm.node_ids[i], shannon_entropy(&dist))
        })
        .collect();

    #[cfg(not(feature = "parallel"))]
    let results: Vec<(NodeId, f64)> = {
        // Reuse buffers across all N endpoint distributions (2 allocations, not 2N)
        let mut buf_a = vec![0.0f64; tm.n];
        let mut buf_b = vec![0.0f64; tm.n];
        (0..tm.n)
            .map(|i| {
                endpoint_distribution_sparse_reuse(&tm, i, horizon, &mut buf_a, &mut buf_b);
                (tm.node_ids[i], shannon_entropy(&buf_a))
            })
            .collect()
    };

    let mut per_node = BTreeMap::new();
    let mut total = 0.0;
    for (id, e) in &results {
        per_node.insert(*id, *e);
        total += e;
    }

    EntropyResult {
        per_node,
        global: total / tm.n as f64,
    }
}

/// Sparse ΔS_τ for a single candidate edge.
pub fn delta_st_sparse(adj: &WeightedAdj, from_id: NodeId, to_id: NodeId, horizon: u32) -> f64 {
    let affected = get_affected_nodes(adj, from_id, to_id);

    let tm_before = build_sparse_transition_matrix(adj);
    let mut before_sum = 0.0;
    for &node_id in &affected {
        if let Some(&idx) = tm_before.idx_map.get(&node_id) {
            let dist = endpoint_distribution_sparse(&tm_before, idx, horizon);
            before_sum += shannon_entropy(&dist);
        }
    }

    let adj_after = add_candidate_edge(adj, from_id, to_id);
    let tm_after = build_sparse_transition_matrix(&adj_after);
    let mut after_sum = 0.0;
    for &node_id in &affected {
        if let Some(&idx) = tm_after.idx_map.get(&node_id) {
            let dist = endpoint_distribution_sparse(&tm_after, idx, horizon);
            after_sum += shannon_entropy(&dist);
        }
    }

    (after_sum - before_sum) / affected.len() as f64
}

/// Sparse batch ΔS_τ evaluation.
pub fn batch_delta_st_sparse(
    adj: &WeightedAdj,
    candidates: &[(NodeId, NodeId)],
    horizon: u32,
) -> BTreeMap<(NodeId, NodeId), f64> {
    if candidates.is_empty() {
        return BTreeMap::new();
    }

    let tm_before = build_sparse_transition_matrix(adj);
    let entropy_cache: Vec<f64> = (0..tm_before.n)
        .map(|i| {
            let dist = endpoint_distribution_sparse(&tm_before, i, horizon);
            shannon_entropy(&dist)
        })
        .collect();

    #[cfg(feature = "parallel")]
    let results: Vec<((NodeId, NodeId), f64)> = candidates
        .par_iter()
        .map(|&(from_id, to_id)| {
            let affected = get_affected_nodes(adj, from_id, to_id);

            let mut before_sum = 0.0;
            for &node_id in &affected {
                if let Some(&idx) = tm_before.idx_map.get(&node_id) {
                    before_sum += entropy_cache[idx];
                }
            }

            let adj_after = add_candidate_edge(adj, from_id, to_id);
            let tm_after = build_sparse_transition_matrix(&adj_after);
            let mut after_sum = 0.0;
            for &node_id in &affected {
                if let Some(&idx) = tm_after.idx_map.get(&node_id) {
                    let dist = endpoint_distribution_sparse(&tm_after, idx, horizon);
                    after_sum += shannon_entropy(&dist);
                }
            }

            (
                (from_id, to_id),
                (after_sum - before_sum) / affected.len() as f64,
            )
        })
        .collect();

    #[cfg(not(feature = "parallel"))]
    let results: Vec<((NodeId, NodeId), f64)> = candidates
        .iter()
        .map(|&(from_id, to_id)| {
            let affected = get_affected_nodes(adj, from_id, to_id);

            let mut before_sum = 0.0;
            for &node_id in &affected {
                if let Some(&idx) = tm_before.idx_map.get(&node_id) {
                    before_sum += entropy_cache[idx];
                }
            }

            let adj_after = add_candidate_edge(adj, from_id, to_id);
            let tm_after = build_sparse_transition_matrix(&adj_after);
            let mut after_sum = 0.0;
            for &node_id in &affected {
                if let Some(&idx) = tm_after.idx_map.get(&node_id) {
                    let dist = endpoint_distribution_sparse(&tm_after, idx, horizon);
                    after_sum += shannon_entropy(&dist);
                }
            }

            (
                (from_id, to_id),
                (after_sum - before_sum) / affected.len() as f64,
            )
        })
        .collect();

    results.into_iter().collect()
}

// ── Locality: k-hop subgraph extraction for O(d̄^k) MCTS ──

/// Extract the weighted adjacency for nodes within k hops of the given center nodes.
pub fn extract_local_adj(adj: &WeightedAdj, center_nodes: &[NodeId], k_hops: u32) -> WeightedAdj {
    // BFS to find k-hop neighborhood
    let mut visited: BTreeSet<NodeId> = BTreeSet::new();
    let mut queue: VecDeque<(NodeId, u32)> = VecDeque::new();

    for &center in center_nodes {
        if visited.insert(center) {
            queue.push_back((center, 0));
        }
    }

    while let Some((current, depth)) = queue.pop_front() {
        if depth >= k_hops {
            continue;
        }
        if let Some(neighbors) = adj.get(&current) {
            for &(neighbor_id, _) in neighbors {
                if visited.insert(neighbor_id) {
                    queue.push_back((neighbor_id, depth + 1));
                }
            }
        }
    }

    // Extract subgraph adjacency (only edges where both endpoints are in the subgraph)
    let mut local_adj: WeightedAdj = BTreeMap::new();
    for &node_id in &visited {
        if let Some(neighbors) = adj.get(&node_id) {
            let local_neighbors: Vec<(NodeId, f64)> = neighbors
                .iter()
                .filter(|(nid, _)| visited.contains(nid))
                .copied()
                .collect();
            if !local_neighbors.is_empty() {
                local_adj.insert(node_id, local_neighbors);
            } else {
                // Isolated in subgraph — still include for entropy computation
                local_adj.insert(node_id, Vec::new());
            }
        }
    }

    local_adj
}

/// Compute ΔS_τ using only a k-hop local subgraph around the candidate edge.
/// Much faster than full-graph ΔS_τ for large networks.
pub fn local_delta_st(
    adj: &WeightedAdj,
    from_id: NodeId,
    to_id: NodeId,
    k_hops: u32,
    horizon: u32,
) -> f64 {
    let local_adj = extract_local_adj(adj, &[from_id, to_id], k_hops);
    let affected = get_affected_nodes(&local_adj, from_id, to_id);

    // Before
    let tm_before = build_sparse_transition_matrix(&local_adj);
    let mut before_sum = 0.0;
    for &node_id in &affected {
        if let Some(&idx) = tm_before.idx_map.get(&node_id) {
            let dist = endpoint_distribution_sparse(&tm_before, idx, horizon);
            before_sum += shannon_entropy(&dist);
        }
    }

    // Add candidate edge to local subgraph
    let local_adj_after = add_candidate_edge(&local_adj, from_id, to_id);
    let tm_after = build_sparse_transition_matrix(&local_adj_after);
    let mut after_sum = 0.0;
    for &node_id in &affected {
        if let Some(&idx) = tm_after.idx_map.get(&node_id) {
            let dist = endpoint_distribution_sparse(&tm_after, idx, horizon);
            after_sum += shannon_entropy(&dist);
        }
    }

    (after_sum - before_sum) / affected.len() as f64
}

/// Batch local ΔS_τ for multiple candidate edges.
pub fn batch_local_delta_st(
    adj: &WeightedAdj,
    candidates: &[(NodeId, NodeId)],
    k_hops: u32,
    horizon: u32,
) -> Vec<f64> {
    if candidates.is_empty() {
        return Vec::new();
    }

    #[cfg(feature = "parallel")]
    let results: Vec<f64> = candidates
        .par_iter()
        .map(|&(from_id, to_id)| local_delta_st(adj, from_id, to_id, k_hops, horizon))
        .collect();

    #[cfg(not(feature = "parallel"))]
    let results: Vec<f64> = candidates
        .iter()
        .map(|&(from_id, to_id)| local_delta_st(adj, from_id, to_id, k_hops, horizon))
        .collect();

    results
}

// ── Auto-selecting wrapper ──

/// Compute all entropies, auto-selecting dense or sparse based on network size.
pub fn compute_all_entropies_auto(adj: &WeightedAdj, horizon: u32) -> EntropyResult {
    if adj.len() > SPARSE_THRESHOLD {
        compute_all_entropies_sparse(adj, horizon)
    } else {
        compute_all_entropies_exact(adj, horizon)
    }
}

/// Compute per-node endpoint distributions (dense only, for analysis tools).
///
/// Returns a map from each source node to its τ-step endpoint distribution,
/// represented as `Vec<(NodeId, f64)>` of (target, probability) pairs.
/// Only non-zero probabilities are included.
pub fn compute_endpoint_distributions(
    adj: &WeightedAdj,
    horizon: u32,
) -> BTreeMap<NodeId, Vec<(NodeId, f64)>> {
    if adj.is_empty() {
        return BTreeMap::new();
    }

    let tm = build_transition_matrix(adj);
    let mut result = BTreeMap::new();

    for i in 0..tm.n {
        let dist = endpoint_distribution(&tm.p, tm.n, i, horizon);
        let entries: Vec<(NodeId, f64)> = dist
            .iter()
            .enumerate()
            .filter(|&(_, &p)| p > 1e-15)
            .map(|(j, &p)| (tm.node_ids[j], p))
            .collect();
        result.insert(tm.node_ids[i], entries);
    }

    result
}

// ── Helpers ──

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

fn add_candidate_edge(adj: &WeightedAdj, from_id: NodeId, to_id: NodeId) -> WeightedAdj {
    let mut adj_after = adj.clone();
    adj_after.entry(from_id).or_default().push((to_id, 1.0));
    adj_after.entry(to_id).or_default().push((from_id, 1.0));
    adj_after
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_complete_graph(n: u32) -> WeightedAdj {
        let mut adj = BTreeMap::new();
        for i in 0..n {
            let neighbors: Vec<(NodeId, f64)> =
                (0..n).filter(|&j| j != i).map(|j| (j, 1.0)).collect();
            adj.insert(i, neighbors);
        }
        adj
    }

    #[test]
    fn test_transition_matrix_row_stochastic() {
        let adj = make_complete_graph(5);
        let tm = build_transition_matrix(&adj);
        for i in 0..tm.n {
            let row_sum: f64 = (0..tm.n).map(|j| tm.p[i * tm.n + j]).sum();
            assert!(
                (row_sum - 1.0).abs() < 1e-10,
                "Row {} sums to {}",
                i,
                row_sum
            );
        }
    }

    #[test]
    fn test_complete_graph_entropy() {
        let adj = make_complete_graph(4);
        let result = compute_all_entropies_exact(&adj, 10);
        // At high horizon, entropy -> log2(4) = 2.0
        assert!(
            (result.global - 2.0).abs() < 0.01,
            "Expected ~2.0, got {}",
            result.global
        );
    }

    #[test]
    fn test_delta_st_positive_for_new_edge() {
        // Line graph: 0-1-2-3, adding 0-3 should increase entropy
        let mut adj: WeightedAdj = BTreeMap::new();
        adj.insert(0, vec![(1, 1.0)]);
        adj.insert(1, vec![(0, 1.0), (2, 1.0)]);
        adj.insert(2, vec![(1, 1.0), (3, 1.0)]);
        adj.insert(3, vec![(2, 1.0)]);

        let delta = delta_st_exact(&adj, 0, 3, 5);
        assert!(
            delta > 0.0,
            "Adding shortcut should increase entropy, got {}",
            delta
        );
    }

    #[test]
    fn test_batch_matches_individual() {
        let mut adj: WeightedAdj = BTreeMap::new();
        adj.insert(0, vec![(1, 1.0)]);
        adj.insert(1, vec![(0, 1.0), (2, 1.0)]);
        adj.insert(2, vec![(1, 1.0), (3, 1.0)]);
        adj.insert(3, vec![(2, 1.0)]);

        let candidates = vec![(0, 3), (0, 2)];
        let batch = batch_delta_st_exact(&adj, &candidates, 5);
        let individual_03 = delta_st_exact(&adj, 0, 3, 5);
        let individual_02 = delta_st_exact(&adj, 0, 2, 5);

        assert!(
            (batch[&(0, 3)] - individual_03).abs() < 1e-10,
            "Batch and individual should match for (0,3)"
        );
        assert!(
            (batch[&(0, 2)] - individual_02).abs() < 1e-10,
            "Batch and individual should match for (0,2)"
        );
    }

    // ── Sparse CSR tests ──

    #[test]
    fn test_sparse_matches_dense_complete() {
        let adj = make_complete_graph(5);
        let dense = compute_all_entropies_exact(&adj, 10);
        let sparse = compute_all_entropies_sparse(&adj, 10);

        assert!(
            (dense.global - sparse.global).abs() < 1e-10,
            "Sparse global {:.6} != dense global {:.6}",
            sparse.global,
            dense.global
        );
        for (&id, &dense_e) in &dense.per_node {
            let sparse_e = sparse.per_node[&id];
            assert!(
                (dense_e - sparse_e).abs() < 1e-10,
                "Node {} sparse {:.6} != dense {:.6}",
                id,
                sparse_e,
                dense_e
            );
        }
    }

    #[test]
    fn test_sparse_matches_dense_line() {
        let mut adj: WeightedAdj = BTreeMap::new();
        adj.insert(0, vec![(1, 1.0)]);
        adj.insert(1, vec![(0, 1.0), (2, 1.0)]);
        adj.insert(2, vec![(1, 1.0), (3, 1.0)]);
        adj.insert(3, vec![(2, 1.0), (4, 1.0)]);
        adj.insert(4, vec![(3, 1.0)]);

        let dense = compute_all_entropies_exact(&adj, 5);
        let sparse = compute_all_entropies_sparse(&adj, 5);

        assert!(
            (dense.global - sparse.global).abs() < 1e-10,
            "Line graph: sparse {:.6} != dense {:.6}",
            sparse.global,
            dense.global
        );
    }

    #[test]
    fn test_sparse_delta_st_matches_dense() {
        let mut adj: WeightedAdj = BTreeMap::new();
        adj.insert(0, vec![(1, 1.0)]);
        adj.insert(1, vec![(0, 1.0), (2, 1.0)]);
        adj.insert(2, vec![(1, 1.0), (3, 1.0)]);
        adj.insert(3, vec![(2, 1.0)]);

        let dense_delta = delta_st_exact(&adj, 0, 3, 5);
        let sparse_delta = delta_st_sparse(&adj, 0, 3, 5);

        assert!(
            (dense_delta - sparse_delta).abs() < 1e-10,
            "ΔS_τ sparse {:.6} != dense {:.6}",
            sparse_delta,
            dense_delta
        );
    }

    #[test]
    fn test_sparse_batch_matches_dense() {
        let mut adj: WeightedAdj = BTreeMap::new();
        adj.insert(0, vec![(1, 1.0)]);
        adj.insert(1, vec![(0, 1.0), (2, 1.0)]);
        adj.insert(2, vec![(1, 1.0), (3, 1.0)]);
        adj.insert(3, vec![(2, 1.0)]);

        let candidates = vec![(0, 3), (0, 2)];
        let dense = batch_delta_st_exact(&adj, &candidates, 5);
        let sparse = batch_delta_st_sparse(&adj, &candidates, 5);

        for &(from, to) in &candidates {
            assert!(
                (dense[&(from, to)] - sparse[&(from, to)]).abs() < 1e-10,
                "({},{}) sparse {:.6} != dense {:.6}",
                from,
                to,
                sparse[&(from, to)],
                dense[&(from, to)]
            );
        }
    }

    // ── Incremental batch tests ──

    #[test]
    fn test_incremental_matches_exact_line() {
        // Line: 0-1-2-3
        let mut adj: WeightedAdj = BTreeMap::new();
        adj.insert(0, vec![(1, 1.0)]);
        adj.insert(1, vec![(0, 1.0), (2, 1.0)]);
        adj.insert(2, vec![(1, 1.0), (3, 1.0)]);
        adj.insert(3, vec![(2, 1.0)]);

        let candidates = vec![(0, 3), (0, 2)];
        let exact = batch_delta_st_exact(&adj, &candidates, 5);
        let incr = batch_delta_st_incremental(&adj, &candidates, 5);

        for (ci, &(from, to)) in candidates.iter().enumerate() {
            let e = exact[&(from, to)];
            let i = incr[ci];
            assert!(
                (e - i).abs() < 1e-10,
                "({},{}) incremental {:.10} != exact {:.10}",
                from,
                to,
                i,
                e
            );
        }
    }

    #[test]
    fn test_incremental_matches_exact_star() {
        // Star: hub 0, spokes 1..5
        let mut adj: WeightedAdj = BTreeMap::new();
        adj.insert(0, vec![(1, 1.0), (2, 1.0), (3, 1.0), (4, 1.0), (5, 1.0)]);
        for i in 1..=5u32 {
            adj.insert(i, vec![(0, 1.0)]);
        }

        let candidates = vec![(1, 2), (1, 3), (2, 4), (3, 5)];
        let exact = batch_delta_st_exact(&adj, &candidates, 5);
        let incr = batch_delta_st_incremental(&adj, &candidates, 5);

        for (ci, &(from, to)) in candidates.iter().enumerate() {
            let e = exact[&(from, to)];
            let i = incr[ci];
            assert!(
                (e - i).abs() < 1e-10,
                "({},{}) incremental {:.10} != exact {:.10}",
                from,
                to,
                i,
                e
            );
        }
    }

    #[test]
    fn test_incremental_matches_exact_complete_minus_edge() {
        // K5 with edge (0,4) removed — test with weighted neighbor counts > 1
        let mut adj: WeightedAdj = BTreeMap::new();
        for i in 0..5u32 {
            let neighbors: Vec<(NodeId, f64)> = (0..5)
                .filter(|&j| j != i && !(i == 0 && j == 4) && !(i == 4 && j == 0))
                .map(|j| (j, 1.0))
                .collect();
            adj.insert(i, neighbors);
        }

        let candidates = vec![(0, 4)];
        let exact = batch_delta_st_exact(&adj, &candidates, 5);
        let incr = batch_delta_st_incremental(&adj, &candidates, 5);

        let e = exact[&(0, 4)];
        let i = incr[0];
        assert!(
            (e - i).abs() < 1e-10,
            "(0,4) incremental {:.10} != exact {:.10}",
            i,
            e
        );
    }

    // ── Locality tests ──

    #[test]
    fn test_local_delta_st_approximates_global() {
        // Star graph: hub 0 connected to 1,2,3,4,5
        let mut adj: WeightedAdj = BTreeMap::new();
        adj.insert(0, vec![(1, 1.0), (2, 1.0), (3, 1.0), (4, 1.0), (5, 1.0)]);
        for i in 1..=5u32 {
            adj.insert(i, vec![(0, 1.0)]);
        }

        // Adding edge 1-2 (connecting two spokes)
        let global_dst = delta_st_exact(&adj, 1, 2, 5);
        let local_dst = local_delta_st(&adj, 1, 2, 2, 5);

        // With k=2 hops from both endpoints, we capture the full star.
        // Local should be very close to global for small graphs.
        assert!(
            (global_dst - local_dst).abs() < 0.01,
            "Local {:.6} should approximate global {:.6}",
            local_dst,
            global_dst
        );
    }

    #[test]
    fn test_extract_local_adj() {
        // Line: 0-1-2-3-4-5
        let mut adj: WeightedAdj = BTreeMap::new();
        adj.insert(0, vec![(1, 1.0)]);
        adj.insert(1, vec![(0, 1.0), (2, 1.0)]);
        adj.insert(2, vec![(1, 1.0), (3, 1.0)]);
        adj.insert(3, vec![(2, 1.0), (4, 1.0)]);
        adj.insert(4, vec![(3, 1.0), (5, 1.0)]);
        adj.insert(5, vec![(4, 1.0)]);

        // 1-hop from node 2: should include 1, 2, 3
        let local = extract_local_adj(&adj, &[2], 1);
        assert!(local.contains_key(&1));
        assert!(local.contains_key(&2));
        assert!(local.contains_key(&3));
        assert!(!local.contains_key(&0), "Node 0 is 2 hops away");
        assert!(!local.contains_key(&4), "Node 4 is 2 hops away");
    }
}
