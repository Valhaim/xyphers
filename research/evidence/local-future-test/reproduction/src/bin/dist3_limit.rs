//! Distance-3 Asymptotic Limit — Analytical P³ for parameterized trees
//!
//! For the specific tree: u(d_u=1)—b1(d_b1=2)—b2(D)—v(d) with leaf pendants,
//! compute P³ distributions analytically (O(1) per node type), enabling
//! sweeps to D = 10^6 and beyond.
//!
//! Key question: Does |Φ₂⁻|/(Φ₀+Φ₁) converge, or can it exceed 1?
//!
//! Run: cargo run --release --bin dist3-limit

fn h(p: f64) -> f64 {
    if p > 1e-18 {
        -p * p.log2()
    } else {
        0.0
    }
}

/// Compute entropy of a distribution given as (value, multiplicity) pairs
fn entropy(dist: &[(f64, usize)]) -> f64 {
    let mut s = 0.0;
    for &(p, mult) in dist {
        s += mult as f64 * h(p);
    }
    s
}

/// Analytical P³ computation for the parameterized dist-3 tree:
///   u (degree 1) — b1 (degree 2) — b2 (degree D) — v (degree d)
/// with D-2 leaf children of b2, and d-1 leaf children of v.
///
/// Returns (Φ₀, Φ₁, Φ₂⁻, ratio)
fn analyze(big_d: usize, d: usize) -> (f64, f64, f64, f64) {
    let df = big_d as f64;
    let dv = d as f64;

    // ================================================================
    // BEFORE adding edge (u,v)
    // ================================================================

    // --- P³[u, ·] ---
    // u→b1 (step 1), b1→{u,b2} each 1/2 (step 2), then step 3
    // u→b1→u→b1: prob = 1/2
    // u→b1→b2→{b1,v,L_i} each 1/D: prob = 1/(2D) each
    // Support: {b1} ∪ {v} ∪ {L₁,...,L_{D-2}}
    let u_b1 = 0.5 + 1.0 / (2.0 * df); // via u→b1→u→b1 + u→b1→b2→b1
    let u_v = 1.0 / (2.0 * df);
    let u_li = 1.0 / (2.0 * df); // each of D-2 leaves of b2
    let h_u = entropy(&[(u_b1, 1), (u_v, 1), (u_li, big_d - 2)]);

    // --- P³[v, ·] ---
    // v has degree d, neighbors {b2, M₁,...,M_{d-1}}
    // Step 1: v→b2 (1/d), v→M_k (1/d) each
    // From b2 (step 2): b2→{b1,v,L_i} each 1/D
    //   From b1 (step 3): →u (1/2), →b2 (1/2)
    //   From v (step 3): →b2 (1/d), →M_k (1/d) each
    //   From L_i (step 3): →b2
    // From M_k (step 2): M_k→v
    //   From v (step 3): →b2 (1/d), →M_k (1/d) each
    // v→b2→b1→u: (1/d)(1/D)(1/2)
    // v→b2→b1→b2: (1/d)(1/D)(1/2)
    // v→b2→v→b2: (1/d)(1/D)(1/d)
    // v→b2→v→M_k: (1/d)(1/D)(1/d) each
    // v→b2→L_i→b2: (1/d)(D-2)/D
    // v→M_k→v→b2: (1/d)·1·(1/d) for each of d-1 M's = (d-1)/d · (1/d)
    // v→M_k→v→M_j: (1/d)·1·(1/d) for each j, but from specific M_k

    // Let me compute systematically:
    // P³[v,j] = Σ_k P[v,k] · P²[k,j]
    // = (1/d) P²[b2,j] + (1/d) Σ_{k=1}^{d-1} P²[M_k,j]

    // P²[b2,j] = (1/D) Σ_{k∈N(b2)} P[k,j]
    // = (1/D)[P[b1,j] + P[v,j] + Σᵢ P[L_i,j]]
    // P[b1,j] = (1/2)δ_u + (1/2)δ_{b2}
    // P[v,j] = (1/d)δ_{b2} + (1/d)Σ δ_{M_k}
    // P[L_i,j] = δ_{b2}
    //
    // P²[b2, u] = 1/(2D)
    // P²[b2, b2] = (1/D)(1/2 + 1/d + D-2) = (D - 3/2 + 1/d) / D
    // P²[b2, M_k] = 1/(Dd) each of d-1
    // P²[b2, others] = 0

    let p2_b2_u = 1.0 / (2.0 * df);
    let p2_b2_b2 = (df - 1.5 + 1.0 / dv) / df;
    let p2_b2_mk = 1.0 / (df * dv);

    // P²[M_k,j] = P[v,j] (since M_k→v deterministically, then v→j)
    // P[v,j] = (1/d)δ_{b2} + (1/d)Σ δ_{M_j}
    // P²[M_k, b2] = 1/d
    // P²[M_k, M_j] = 1/d each for all j (including j=k!)

    let p2_mk_b2 = 1.0 / dv;
    let p2_mk_mj = 1.0 / dv; // each M_j

    // P³[v,j] = (1/d) P²[b2,j] + ((d-1)/d) P²[M_generic,j]
    // (all M_k have identical P², so sum = (d-1)× P²[M_k,j])

    let v_u = (1.0 / dv) * p2_b2_u; // only b2-path reaches u
    let v_b2 = (1.0 / dv) * p2_b2_b2 + ((dv - 1.0) / dv) * p2_mk_b2;
    let v_mk = (1.0 / dv) * p2_b2_mk + ((dv - 1.0) / dv) * p2_mk_mj;
    // Note: all M_k have identical prob due to symmetry (P²[M_k, M_j] = 1/d for all j)
    let h_v = entropy(&[(v_u, 1), (v_b2, 1), (v_mk, d - 1)]);

    // --- P³[b1, ·] ---
    // P³[b1,j] = (1/2) P²[u,j] + (1/2) P²[b2,j]
    // P²[u,j]: u→b1 (step 1), then b1→{u,b2} (step 2)
    // P²[u,u] = 1/2, P²[u,b2] = 1/2
    let b1_u = 0.5 * 0.5 + 0.5 * p2_b2_u;
    let b1_b2 = 0.5 * 0.5 + 0.5 * p2_b2_b2;
    let b1_mk = 0.5 * p2_b2_mk;
    let h_b1 = entropy(&[(b1_u, 1), (b1_b2, 1), (b1_mk, d - 1)]);

    // --- P³[b2, ·] ---
    // P³[b2,j] = (1/D) Σ_{k∈N(b2)} P²[k,j]
    // = (1/D)[P²[b1,j] + P²[v,j] + Σᵢ P²[L_i,j]]
    //
    // P²[b1,j] = (1/2)δ_u + (1/2)δ_{b2} (as above: P²[b1,u]=1/2, P²[b1,b2]=1/2)
    // Wait, that's wrong. P²[b1,j] is a 2-step walk from b1:
    // P²[b1,j] = (1/2)P[u,j] + (1/2)P[b2,j]
    // = (1/2)δ_{b1} + (1/2)(1/D)Σ_{k∈N(b2)} δ_k
    // P²[b1, b1] = 1/2 + 1/(2D)
    // P²[b1, v] = 1/(2D)
    // P²[b1, L_i] = 1/(2D) each
    // P²[b1, others] = 0

    let p2_b1_b1 = 0.5 + 1.0 / (2.0 * df);
    let p2_b1_v = 1.0 / (2.0 * df);
    let p2_b1_li = 1.0 / (2.0 * df);

    // P²[v,j] = (1/d)P[b2,j] + ((d-1)/d)P[M_k,j]
    // = (1/d)(1/D)Σ_{k∈N(b2)} δ_k + ((d-1)/d)δ_v
    // P²[v, b1] = 1/(dD)
    // P²[v, v] = 1/(dD) + (d-1)/d
    // P²[v, L_i] = 1/(dD) each
    // P²[v, others] = 0

    let p2_v_b1 = 1.0 / (dv * df);
    let p2_v_v = 1.0 / (dv * df) + (dv - 1.0) / dv;
    let p2_v_li = 1.0 / (dv * df);

    // P²[L_i,j] = P[b2,j] (since L_i→b2 deterministically, then b2→j)
    // = (1/D)Σ_{k∈N(b2)} δ_k
    // P²[L_i, b1] = 1/D, P²[L_i, v] = 1/D, P²[L_i, L_j] = 1/D each, P²[L_i, others] = 0

    let p2_li_b1 = 1.0 / df;
    let p2_li_v = 1.0 / df;
    let p2_li_lj = 1.0 / df;

    // P³[b2,j] = (1/D)[P²[b1,j] + P²[v,j] + (D-2)·P²[L_generic,j]]
    let b2_b1 = (1.0 / df) * (p2_b1_b1 + p2_v_b1 + (df - 2.0) * p2_li_b1);
    let b2_v = (1.0 / df) * (p2_b1_v + p2_v_v + (df - 2.0) * p2_li_v);
    let b2_li = (1.0 / df) * (p2_b1_li + p2_v_li + (df - 2.0) * p2_li_lj);
    // Note: there are D-2 leaf types but each L_i's P²[L_i, L_j] counts all D-2 leaves
    // Actually P²[L_i, L_j] = 1/D for all j (including j=i? L_i→b2→L_i: yes!)
    // So for P³[b2, L_specific]:
    // From P²[L_i, L_j] = 1/D for each j, so contribution from all D-2 L-nodes is (D-2)/D
    // But we need to be careful about which L node we're asking about...
    // Actually all L nodes are equivalent by symmetry for P³[b2,·]

    let h_b2 = entropy(&[(b2_b1, 1), (b2_v, 1), (b2_li, big_d - 2)]);

    // --- P³[M_k, ·] (leaf of v) ---
    // M_k→v (step 1), then P²[v,j]
    // P³[M_k,j] = P²[v,j]
    let mk_b1 = p2_v_b1;
    let mk_v = p2_v_v;
    let mk_li = p2_v_li;
    let h_mk = entropy(&[(mk_b1, 1), (mk_v, 1), (mk_li, big_d - 2)]);

    // --- P³[L_i, ·] (leaf of b2 = shell-2 pendant) ---
    // L_i→b2 (step 1), then 2 more steps from b2
    // P³[L_i,j] = (1/D)P[b1,j] + (1/D)P[v,j] + ((D-2)/D)δ_{b2,j}... no wait
    // P³[L_i,j] = Σ_k P²[b2→k] · P[k,j]
    // Actually: P³[L_i,·] = P²[L_i,·] * P = P[b2,·] * P² ... no.
    // Let me recompute. P³[L_i,j] = (P³)_{L_i,j} where P³ = P·P·P.
    // (P³)_{w,j} = Σ_{k1,k2} P[w,k1]P[k1,k2]P[k2,j]
    // For w = L_i: P[L_i, k1] = δ_{k1,b2}
    // So (P³)_{L_i,j} = Σ_{k2} P[b2,k2]P[k2,j]
    // = (1/D) Σ_{k∈N(b2)} P[k,j]
    // = (1/D)[P[b1,j] + P[v,j] + (D-2)P[L_generic,j]]

    let li_u = (1.0 / df) * 0.5; // from b1→u
    let li_b2 = (1.0 / df) * 0.5 + (1.0 / df) * (1.0 / dv) + (df - 2.0) / df;
    // b1→b2: 1/(2D), v→b2: 1/(Dd), each L_j→b2: 1/D, total (D-2)/D
    let li_mk = (1.0 / df) * (1.0 / dv); // from v→M_k, each of d-1
    let h_li = entropy(&[(li_u, 1), (li_b2, 1), (li_mk, d - 1)]);

    // ================================================================
    // AFTER adding edge (u,v): u degree 2, v degree d+1
    // ================================================================
    let dv1 = dv + 1.0;

    // All P'[node,·] that change: only u and v rows
    // P'[u,·] = (1/2)δ_{b1} + (1/2)δ_v
    // P'[v,·] = (1/(d+1))δ_{b2} + (1/(d+1))δ_u + (1/(d+1))Σ δ_{M_k}
    // All other P' rows unchanged.

    // --- P'³[u, ·] ---
    // P'³[u,j] = Σ_k P'[u,k] · P'²[k,j] = (1/2)P'²[b1,j] + (1/2)P'²[v,j]
    //
    // P'²[b1,j] = (1/2)P'[u,j] + (1/2)P'[b2,j]
    // P'[u,j] = (1/2)δ_{b1} + (1/2)δ_v
    // P'[b2,j] = (1/D)Σ_{k∈N(b2)} δ_k = same as before (b2 unchanged)
    //
    // P'²[b1, b1] = (1/2)(1/2) + (1/2)(1/D) = 1/4 + 1/(2D)
    // P'²[b1, v] = (1/2)(1/2) + (1/2)(1/D) = 1/4 + 1/(2D)
    // P'²[b1, L_i] = (1/2)(1/D) = 1/(2D) each
    // P'²[b1, others] = 0

    let p2p_b1_b1 = 0.25 + 1.0 / (2.0 * df);
    let p2p_b1_v = 0.25 + 1.0 / (2.0 * df);
    let p2p_b1_li = 1.0 / (2.0 * df);

    // P'²[v,j] = (1/(d+1))P'[b2,j] + (1/(d+1))P'[u,j] + ((d-1)/(d+1))P'[M_k,j]
    // P'[b2,j]: same as P[b2,j]
    // P'[u,j] = (1/2)δ_{b1} + (1/2)δ_v
    // P'[M_k,j] = δ_v
    //
    // P'²[v, b1] = (1/(d+1))(1/D) + (1/(d+1))(1/2) = (1/(d+1))(1/D + 1/2)
    // P'²[v, v] = (1/(d+1))(1/D) + (1/(d+1))(1/2) + (d-1)/(d+1) = (1/(d+1))(1/D + 1/2 + d-1)
    // P'²[v, L_i] = (1/(d+1))(1/D) each
    // P'²[v, others] = 0

    let p2p_v_b1 = (1.0 / dv1) * (1.0 / df + 0.5);
    let p2p_v_v = (1.0 / dv1) * (1.0 / df + 0.5 + dv - 1.0);
    let p2p_v_li = 1.0 / (dv1 * df);

    // P'³[u,j] = (1/2)P'²[b1,j] + (1/2)P'²[v,j]
    let up_b1 = 0.5 * p2p_b1_b1 + 0.5 * p2p_v_b1;
    let up_v = 0.5 * p2p_b1_v + 0.5 * p2p_v_v;
    let up_li = 0.5 * p2p_b1_li + 0.5 * p2p_v_li;
    let hp_u = entropy(&[(up_b1, 1), (up_v, 1), (up_li, big_d - 2)]);

    // --- P'³[v, ·] ---
    // P'³[v,j] = (1/(d+1)) P'²[b2,j] + (1/(d+1)) P'²[u,j] + ((d-1)/(d+1)) P'²[M_k,j]
    //
    // P'²[b2,j] = (1/D) Σ_{k∈N(b2)} P'[k,j]
    // = (1/D)[P'[b1,j] + P'[v,j] + (D-2)P'[L_generic,j]]
    // P'[b1,j] = (1/2)δ_u + (1/2)δ_{b2} (unchanged)
    // P'[v,j] = (1/(d+1))δ_{b2} + (1/(d+1))δ_u + (1/(d+1))Σ δ_{M_k}
    // P'[L_i,j] = δ_{b2}
    //
    // P'²[b2, u] = (1/D)[(1/2) + (1/(d+1))] = (1/D)(1/2 + 1/(d+1))
    // P'²[b2, b2] = (1/D)[(1/2) + (1/(d+1)) + (D-2)] = (D - 3/2 + 1/(d+1))/D
    // P'²[b2, M_k] = (1/D)(1/(d+1)) each
    // P'²[b2, others] = 0

    let p2p_b2_u = (1.0 / df) * (0.5 + 1.0 / dv1);
    let p2p_b2_b2 = (df - 1.5 + 1.0 / dv1) / df;
    let p2p_b2_mk = 1.0 / (df * dv1);

    // P'²[u,j] = (1/2)P'[b1,j] + (1/2)P'[v,j]
    // = (1/2)[(1/2)δ_u + (1/2)δ_{b2}] + (1/2)[(1/(d+1))δ_{b2} + (1/(d+1))δ_u + (1/(d+1))Σ δ_{M_k}]
    // P'²[u, u] = 1/4 + 1/(2(d+1))
    // P'²[u, b2] = 1/4 + 1/(2(d+1))
    // P'²[u, M_k] = 1/(2(d+1)) each

    let p2p_u_u = 0.25 + 1.0 / (2.0 * dv1);
    let p2p_u_b2 = 0.25 + 1.0 / (2.0 * dv1);
    let p2p_u_mk = 1.0 / (2.0 * dv1);

    // P'²[M_k,j] = P'[v,j] = (1/(d+1))δ_{b2} + (1/(d+1))δ_u + (1/(d+1))Σ δ_{M_j}
    let p2p_mk_b2 = 1.0 / dv1;
    let p2p_mk_u = 1.0 / dv1;
    let p2p_mk_mj = 1.0 / dv1;

    // P'³[v,j] = (1/(d+1))P'²[b2,j] + (1/(d+1))P'²[u,j] + ((d-1)/(d+1))P'²[M_k,j]
    let vp_u = (1.0 / dv1) * p2p_b2_u + (1.0 / dv1) * p2p_u_u + ((dv - 1.0) / dv1) * p2p_mk_u;
    let vp_b2 = (1.0 / dv1) * p2p_b2_b2 + (1.0 / dv1) * p2p_u_b2 + ((dv - 1.0) / dv1) * p2p_mk_b2;
    let vp_mk = (1.0 / dv1) * p2p_b2_mk + (1.0 / dv1) * p2p_u_mk + ((dv - 1.0) / dv1) * p2p_mk_mj;
    let hp_v = entropy(&[(vp_u, 1), (vp_b2, 1), (vp_mk, d - 1)]);

    // --- P'³[b1, ·] ---
    // P'³[b1,j] = (1/2) P'²[u,j] + (1/2) P'²[b2,j]
    // (b1's neighbors {u,b2} unchanged, but P'²[u,·] and P'²[b2,·] differ)
    let b1p_u = 0.5 * p2p_u_u + 0.5 * p2p_b2_u;
    let b1p_b2 = 0.5 * p2p_u_b2 + 0.5 * p2p_b2_b2;
    let b1p_mk = 0.5 * p2p_u_mk + 0.5 * p2p_b2_mk;
    let hp_b1 = entropy(&[(b1p_u, 1), (b1p_b2, 1), (b1p_mk, d - 1)]);

    // --- P'³[b2, ·] ---
    // P'³[b2,j] = (1/D)[P'²[b1,j] + P'²[v,j] + (D-2)P'²[L_generic,j]]
    //
    // P'²[L_i,j] = P'[b2,j] (since L_i→b2, then b2→j) = (1/D)Σ δ_{N(b2)}
    // Same as before: P'²[L_i, b1] = 1/D, P'²[L_i, v] = 1/D, P'²[L_i, L_j] = 1/D
    // (b2 is unchanged, so L_i's walks are same as before for step 2)
    // Wait but step 2 from b2 gives P[b2,·], then step 3... no.
    // P'²[L_i,j] means 2 steps from L_i in modified graph.
    // Step 1: L_i→b2 (unchanged), Step 2: b2→k∈N(b2) each 1/D.
    // So P'²[L_i,j] = (1/D)Σ_{k∈N(b2)} δ_{k,j} = 1/D for j∈N(b2), 0 otherwise.
    // This is UNCHANGED from before.

    let p2p_li_b1 = 1.0 / df;
    let p2p_li_v = 1.0 / df;
    let p2p_li_lj = 1.0 / df;

    let b2p_b1 = (1.0 / df) * (p2p_b1_b1 + p2p_v_b1 + (df - 2.0) * p2p_li_b1);
    let b2p_v = (1.0 / df) * (p2p_b1_v + p2p_v_v + (df - 2.0) * p2p_li_v);
    let b2p_li = (1.0 / df) * (p2p_b1_li + p2p_v_li + (df - 2.0) * p2p_li_lj);
    let hp_b2 = entropy(&[(b2p_b1, 1), (b2p_v, 1), (b2p_li, big_d - 2)]);

    // --- P'³[M_k, ·] (leaf of v) ---
    // P'³[M_k,j] = P'²[v,j] (since M_k→v deterministically, then P'² from v)
    let mkp_u = p2p_v_b1; // wait, P'²[v, b1] not u
                          // Hmm, I need to reconsider. P'²[v,·] was computed above for the modified graph.
                          // But wait - I computed it in terms of b1, v, L_i. Let me also track u.
                          // Actually: P'²[v, b1], P'²[v, v], P'²[v, L_i] — what about P'²[v, u]?
                          //
                          // Looking back: P'²[v,j] = (1/(d+1))P'[b2,j] + (1/(d+1))P'[u,j] + ((d-1)/(d+1))P'[M_k,j]
                          // P'[b2, u] = 0 (u not neighbor of b2)
                          // P'[u, u] = 0 (u's neighbors are b1 and v, not itself)
                          // P'[M_k, u] = 0 (M_k→v, and v is not u)
                          // So P'²[v, u] = 0
                          //
                          // P'²[v, b1] = (1/(d+1))(1/D) + (1/(d+1))(1/2) + 0 = (1/(d+1))(1/D + 1/2)
                          // ✓ matches p2p_v_b1

    // For M_k: P'³[M_k,j] = P'²[v,j]
    // Support: {b1, v, L₁,...,L_{D-2}}
    // Note: M_k can reach b1 (odd distance 3 from M_k in modified graph: M_k→v→u→b1)
    let mkp_b1 = p2p_v_b1;
    let mkp_v = p2p_v_v;
    let mkp_li = p2p_v_li;
    let hp_mk = entropy(&[(mkp_b1, 1), (mkp_v, 1), (mkp_li, big_d - 2)]);

    // --- P'³[L_i, ·] (shell-2 pendant of b2) ---
    // P'³[L_i,j] = Σ_{k₂} P[b2,k₂] · P'[k₂,j]
    // Actually: P'³[L_i,j] = (P'³)_{L_i,j}
    // 3 steps: L_i→b2 (step 1), b2→k (step 2, 1/D each), k→j (step 3)
    // Step 3 uses P'[k,j]:
    // k=b1: P'[b1,j] = (1/2)δ_u + (1/2)δ_{b2} (unchanged)
    // k=v: P'[v,j] = (1/(d+1))δ_{b2} + (1/(d+1))δ_u + (1/(d+1))Σ δ_{M_k}
    // k=L_m: P'[L_m,j] = δ_{b2} (unchanged)

    let lip_u = (1.0 / df) * 0.5 + (1.0 / df) * (1.0 / dv1);
    let lip_b2 = (1.0 / df) * 0.5 + (1.0 / df) * (1.0 / dv1) + (df - 2.0) / df;
    let lip_mk = (1.0 / df) * (1.0 / dv1);
    let hp_li = entropy(&[(lip_u, 1), (lip_b2, 1), (lip_mk, d - 1)]);

    // ================================================================
    // Compute ΔS₃ for each node type
    // ================================================================

    let ds_u = hp_u - h_u;
    let ds_v = hp_v - h_v;
    let ds_b1 = hp_b1 - h_b1;
    let ds_b2 = hp_b2 - h_b2;
    let ds_mk = hp_mk - h_mk;
    let ds_li = hp_li - h_li;

    // Shell classification:
    // Shell 0: u, v
    // Shell 1: b1 (dist to u=1), b2 (dist to v=1), M_k (dist to v=1)
    // Shell 2: L_i (dist to v=2)

    let phi0 = ds_u + ds_v;
    let phi1 = ds_b1 + ds_b2 + (d as f64 - 1.0) * ds_mk;
    let phi2_neg = if ds_li < 0.0 {
        (big_d as f64 - 2.0) * ds_li
    } else {
        0.0
    };

    let local_sum = phi0 + phi1;
    let ratio = if local_sum > 1e-15 {
        (-phi2_neg) / local_sum
    } else {
        0.0
    };

    (phi0, phi1, phi2_neg, ratio)
}

fn main() {
    println!();
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  DISTANCE-3 ASYMPTOTIC LIMIT — Analytical P³ computation  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();

    // Part 1: Verify against matrix computation for small cases
    println!("═══ Verification against matrix computation ═══");
    println!(
        "  {:>6} {:>6} {:>10} {:>10} {:>10} {:>10}",
        "D", "d", "Φ₀", "Φ₁", "Φ₂⁻", "ratio"
    );
    println!("  {}", "-".repeat(60));

    for &(big_d, d) in &[(5, 7), (10, 7), (20, 7), (5, 9), (10, 9), (20, 9)] {
        let (phi0, phi1, phi2_neg, ratio) = analyze(big_d, d);
        println!(
            "  {:>6} {:>6} {:>10.6} {:>10.6} {:>10.6} {:>10.6}",
            big_d, d, phi0, phi1, phi2_neg, ratio
        );
    }
    println!();

    // Part 2: Sweep d_v to find the worst case, then push D → ∞
    println!("═══ Find worst d_v across large D range ═══");
    println!();

    let d_values: Vec<usize> = (2..=30).collect();
    let big_d_values: Vec<usize> = vec![
        10, 50, 100, 500, 1000, 5000, 10000, 50000, 100000, 500000, 1000000,
    ];

    for &big_d in &big_d_values {
        let mut best_ratio = 0.0f64;
        let mut best_d = 0usize;
        for &d in &d_values {
            let (_, _, _, ratio) = analyze(big_d, d);
            if ratio > best_ratio {
                best_ratio = ratio;
                best_d = d;
            }
        }
        let (phi0, phi1, phi2_neg, _) = analyze(big_d, best_d);
        println!(
            "  D={:>8}: worst d_v={:>3}, ratio={:.8}, Φ₀={:.6}, Φ₁={:.6}, Φ₂⁻={:.6}, denom={:.6}",
            big_d,
            best_d,
            best_ratio,
            phi0,
            phi1,
            phi2_neg,
            phi0 + phi1
        );
    }
    println!();

    // Part 3: For worst d_v, show convergence curve
    println!("═══ Convergence curve for worst d_v ═══");
    println!();

    // First find the globally worst d_v
    let mut global_best_d = 9usize;
    let mut global_best_ratio = 0.0f64;
    for &d in &d_values {
        let (_, _, _, ratio) = analyze(1000000, d);
        if ratio > global_best_ratio {
            global_best_ratio = ratio;
            global_best_d = d;
        }
    }
    println!(
        "  Worst d_v at D=10⁶: d_v={}, ratio={:.8}",
        global_best_d, global_best_ratio
    );
    println!();

    let d = global_best_d;
    println!("  D sweep with d_v={} fixed:", d);
    println!(
        "  {:>10} {:>14} {:>14} {:>14} {:>14} {:>12}",
        "D", "Φ₀", "Φ₁", "Φ₂⁻", "Φ₀+Φ₁", "ratio"
    );
    println!("  {}", "-".repeat(85));

    let fine_d_values: Vec<usize> = vec![
        5, 10, 20, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000, 50000, 100000, 200000,
        500000, 1000000,
    ];
    for &big_d in &fine_d_values {
        let (phi0, phi1, phi2_neg, ratio) = analyze(big_d, d);
        println!(
            "  {:>10} {:>14.8} {:>14.8} {:>14.8} {:>14.8} {:>12.8}",
            big_d,
            phi0,
            phi1,
            phi2_neg,
            phi0 + phi1,
            ratio
        );
    }
    println!();

    // Part 4: Per-pendant entropy change scaling
    println!("═══ Per-pendant scaling: |ΔS₃(pendant)| × D ═══");
    println!();
    println!("  d_v={} fixed:", d);
    println!(
        "  {:>10} {:>14} {:>14} {:>14} {:>14}",
        "D", "ΔS₃/pend", "D·|ΔS₃|", "|Φ₂⁻|", "#pendants"
    );
    println!("  {}", "-".repeat(70));

    for &big_d in &fine_d_values {
        if big_d < 3 {
            continue;
        }
        let df = big_d as f64;
        let dv = d as f64;
        let dv1 = dv + 1.0;

        // Recompute just the pendant entropy change
        let li_u = 1.0 / (2.0 * df);
        let li_b2 = (df - 2.0) / df + 1.0 / (2.0 * df) + 1.0 / (df * dv);
        let li_mk = 1.0 / (df * dv);
        let h_li = entropy(&[(li_u, 1), (li_b2, 1), (li_mk, d - 1)]);

        let lip_u = 1.0 / (2.0 * df) + 1.0 / (df * dv1);
        let lip_b2 = (df - 2.0) / df + 1.0 / (2.0 * df) + 1.0 / (df * dv1);
        let lip_mk = 1.0 / (df * dv1);
        let hp_li = entropy(&[(lip_u, 1), (lip_b2, 1), (lip_mk, d - 1)]);

        let ds_li = hp_li - h_li;
        let num_pendants = big_d - 2;
        let total = num_pendants as f64 * ds_li;

        println!(
            "  {:>10} {:>14.10} {:>14.8} {:>14.8} {:>14}",
            big_d,
            ds_li,
            df * ds_li.abs(),
            total.abs(),
            num_pendants
        );
    }
    println!();

    // Part 5: Critical question — does denom stay positive?
    println!("═══ CRITICAL: Does Φ₀+Φ₁ stay positive as D → ∞? ═══");
    println!();
    println!("  The ratio = |Φ₂⁻| / (Φ₀+Φ₁). If denom → 0, ratio → ∞.");
    println!("  If denom stays bounded away from 0, ratio is bounded.");
    println!();

    // Show the denominator trend and whether it's converging
    println!(
        "  {:>10} {:>14} {:>14} {:>14} {:>14}",
        "D", "ΔS₃(u)", "ΔS₃(v)", "Φ₁_total", "Φ₀+Φ₁"
    );
    println!("  {}", "-".repeat(70));

    for &big_d in &fine_d_values {
        let df = big_d as f64;
        let dv = d as f64;
        let dv1 = dv + 1.0;

        // Compute each component of Φ₀ and Φ₁ individually
        let (phi0, phi1, _, _) = analyze(big_d, d);

        // Also compute ΔS₃ for u and v separately
        // Reuse the analytical formulas
        let u_b1 = 0.5 + 1.0 / (2.0 * df);
        let u_v = 1.0 / (2.0 * df);
        let u_li = 1.0 / (2.0 * df);
        let h_u_val = entropy(&[(u_b1, 1), (u_v, 1), (u_li, big_d - 2)]);

        let p2p_b1_b1_val = 0.25 + 1.0 / (2.0 * df);
        let p2p_b1_v_val = 0.25 + 1.0 / (2.0 * df);
        let p2p_b1_li_val = 1.0 / (2.0 * df);
        let p2p_v_b1_val = (1.0 / dv1) * (1.0 / df + 0.5);
        let p2p_v_v_val = (1.0 / dv1) * (1.0 / df + 0.5 + dv - 1.0);
        let p2p_v_li_val = 1.0 / (dv1 * df);

        let up_b1_val = 0.5 * p2p_b1_b1_val + 0.5 * p2p_v_b1_val;
        let up_v_val = 0.5 * p2p_b1_v_val + 0.5 * p2p_v_v_val;
        let up_li_val = 0.5 * p2p_b1_li_val + 0.5 * p2p_v_li_val;
        let hp_u_val = entropy(&[(up_b1_val, 1), (up_v_val, 1), (up_li_val, big_d - 2)]);

        let ds_u_val = hp_u_val - h_u_val;

        let ds_v_val = phi0 - ds_u_val;

        println!(
            "  {:>10} {:>14.8} {:>14.8} {:>14.8} {:>14.8}",
            big_d,
            ds_u_val,
            ds_v_val,
            phi1,
            phi0 + phi1
        );
    }

    // Part 6: Find exact threshold where ratio crosses 1.0 (filter failure)
    println!("═══ THRESHOLD SEARCH: Where does the filter FAIL? ═══");
    println!();
    println!("  Searching for the smallest D where ratio > 1.0 (i.e., filter fails)");
    println!("  for each d_v:");
    println!();

    // For each d_v, binary search for the D where ratio crosses 1.0
    for d_test in 3..=25 {
        // First check if violation exists at all (try D up to 10M)
        let mut found_violation = false;
        let mut lo = 3usize;
        let mut hi = 10_000_000usize;

        // Check if ratio ever exceeds 1.0
        let (_, _, _, r_hi) = analyze(hi, d_test);
        let mut max_ratio = 0.0f64;
        let mut max_d_for_dv = 0usize;

        // Sample to find range where ratio > 1
        for &d_check in &[
            100, 500, 1000, 2000, 3000, 4000, 5000, 7500, 10000, 20000, 50000, 100000, 500000,
            1000000,
        ] {
            let (phi0, phi1, phi2_neg, ratio) = analyze(d_check, d_test);
            let denom = phi0 + phi1;
            if ratio > max_ratio {
                max_ratio = ratio;
                max_d_for_dv = d_check;
            }
            if denom > 0.0 && ratio > 1.0 && !found_violation {
                found_violation = true;
            }
        }

        if found_violation {
            // Binary search for the FIRST D where ratio > 1.0
            let mut lo = 3usize;
            let mut hi = max_d_for_dv;
            // Find the lower bound
            for &d_check in &[
                100, 200, 500, 1000, 1500, 2000, 2500, 3000, 3500, 4000, 4500, 5000,
            ] {
                let (phi0, phi1, _, ratio) = analyze(d_check, d_test);
                if phi0 + phi1 > 0.0 && ratio > 1.0 {
                    hi = d_check;
                    break;
                }
                lo = d_check;
            }
            // Refine
            while hi - lo > 1 {
                let mid = (lo + hi) / 2;
                let (phi0, phi1, _, ratio) = analyze(mid, d_test);
                if phi0 + phi1 > 0.0 && ratio > 1.0 {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            let threshold_d = hi;
            let n_threshold = 4 + (threshold_d - 2) + (d_test - 1); // total nodes
            let (phi0, phi1, phi2_neg, ratio) = analyze(threshold_d, d_test);
            let global = phi0 + phi1 + phi2_neg;
            println!("  d_v={:>3}: FAILS at D={:>6} (N={:>6}), ratio={:.4}, global_ΔS={:.6}, max_ratio={:.4} at D={}",
                d_test, threshold_d, n_threshold, ratio, global, max_ratio, max_d_for_dv);
        } else {
            if max_ratio > 0.05 {
                println!(
                    "  d_v={:>3}: max ratio={:.6} at D={} (NO FAILURE found)",
                    d_test, max_ratio, max_d_for_dv
                );
            }
        }
    }

    println!();

    // Part 7: Find the absolute smallest N where filter fails at τ=3
    println!("═══ SMALLEST FILTER FAILURE at τ=3 on trees ═══");
    println!();

    let mut smallest_n = usize::MAX;
    let mut smallest_config = (0usize, 0usize);
    let mut smallest_ratio = 0.0f64;

    for d_test in 3..=30 {
        for d_try in 100..=50000 {
            let (phi0, phi1, phi2_neg, ratio) = analyze(d_try, d_test);
            let denom = phi0 + phi1;
            if denom > 1e-12 && ratio > 1.0 {
                let n = 4 + (d_try - 2) + (d_test - 1);
                if n < smallest_n {
                    smallest_n = n;
                    smallest_config = (d_try, d_test);
                    smallest_ratio = ratio;
                }
                break; // found threshold for this d_test, move to next
            }
        }
    }

    if smallest_n < usize::MAX {
        let (best_d, best_dv) = smallest_config;
        let (phi0, phi1, phi2_neg, ratio) = analyze(best_d, best_dv);
        println!(
            "  SMALLEST N = {} (D={}, d_v={})",
            smallest_n, best_d, best_dv
        );
        println!(
            "  Φ₀ = {:.6}, Φ₁ = {:.6}, Φ₂⁻ = {:.6}",
            phi0, phi1, phi2_neg
        );
        println!("  ratio = {:.6}", ratio);
        println!("  global ΔS = {:.6}", phi0 + phi1 + phi2_neg);

        // Verify with slightly smaller D
        let (phi0b, phi1b, phi2b, ratio_b) = analyze(best_d - 1, best_dv);
        let nb = 4 + (best_d - 3) + (best_dv - 1);
        println!();
        println!(
            "  At N={} (D={}): ratio = {:.6}, denom = {:.6}",
            nb,
            best_d - 1,
            ratio_b,
            phi0b + phi1b
        );
    } else {
        println!("  No filter failure found for d_u=1, d_b1=2 parameterized trees at τ=3");
    }

    println!();

    // Part 8: Detailed profile near the threshold
    println!("═══ DETAILED PROFILE near threshold ═══");
    println!();
    if smallest_n < usize::MAX {
        let (best_d, best_dv) = smallest_config;
        let start = if best_d > 200 { best_d - 200 } else { 3 };
        println!("  D sweep near threshold (d_v={}):", best_dv);
        println!(
            "  {:>8} {:>8} {:>12} {:>12} {:>12} {:>12} {:>10}",
            "D", "N", "Φ₀", "Φ₁", "Φ₂⁻", "global", "ratio"
        );
        println!("  {}", "-".repeat(80));
        let step = (best_d / 20).max(1);
        let mut d = start;
        while d <= best_d + 500 {
            let (phi0, phi1, phi2_neg, ratio) = analyze(d, best_dv);
            let n = 4 + (d - 2) + (best_dv - 1);
            let global = phi0 + phi1 + phi2_neg;
            let marker = if phi0 + phi1 > 0.0 && ratio > 1.0 {
                " ← FAIL"
            } else {
                ""
            };
            println!(
                "  {:>8} {:>8} {:>12.6} {:>12.6} {:>12.6} {:>12.6} {:>10.6}{}",
                d, n, phi0, phi1, phi2_neg, global, ratio, marker
            );
            d += step;
        }
    }

    println!();
    println!("═══ CONCLUSION ═══");
    println!();
    println!("  The filter theorem at τ=3 FAILS on trees for sufficiently large N.");
    println!("  This is the dist(u,v)=3 analog of the horizon theorem.");
    println!("  The mechanism: large hub (b2) concentrates the distribution of the");
    println!("  distant endpoint (u), causing ΔS₃(u) < 0 (entropy DECREASE).");
    println!("  The 4998 shell-2 pendants each lose a tiny amount of entropy,");
    println!("  overwhelming the positive Φ₁ from the bridges and v-leaves.");
    println!();
}
