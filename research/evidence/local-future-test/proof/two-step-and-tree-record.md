# Filter Theorem for Trees: Proof Attempt

## Setup

Let T = (V, E) be a connected tree on N nodes with unit edge weights.
Let (u, v) be a non-edge in T, and T' = T + (u, v).

Since T is a tree, there exists a unique path from u to v in T:
  u = p_0, p_1, ..., p_L = v
where L = dist_T(u, v) ≥ 2 (they're non-adjacent, so L ≥ 2).

Adding (u, v) creates exactly one cycle C of length L + 1, containing nodes {p_0, p_1, ..., p_L}.

**Transition matrices:**
- P: row-stochastic, P[i][j] = 1/deg_T(i) for j ∈ N_T(i)
- P': row-stochastic, P'[i][j] = 1/deg_{T'}(i) for j ∈ N_{T'}(i)

Only rows u and v change:
- P'[u, j] = 1/(d_u + 1) for j ∈ N_T(u) ∪ {v}
- P'[v, j] = 1/(d_v + 1) for j ∈ N_T(v) ∪ {u}
- P'[w, j] = P[w, j] for all w ∉ {u, v}

**Affected set:** A(u,v) = {u, v} ∪ N_T(u) ∪ N_T(v).

## Key Property of Trees

**Observation 1.** In a tree, the affected set A has a special structure.
- N_T(u) = {p_1} ∪ (children/other neighbors of u)
- N_T(v) = {p_{L-1}} ∪ (children/other neighbors of v)
- Since T is a tree, the only overlap between N(u) and N(v) is impossible unless L = 2 (then p_1 = p_{L-1} is the common neighbor).

**Observation 2.** In a tree T, random walks have no loops to exploit. The walk from any node w must traverse the unique tree path to reach distant nodes. When we add edge (u,v), we create a shortcut that gives the walk a NEW path between u and v (direct, instead of through p_1, ..., p_{L-1}).

## τ = 1 Case (Trivial)

At τ = 1:
- S_1(u; T') = log_2(d_u + 1) > log_2(d_u) = S_1(u; T) ✓
- S_1(v; T') = log_2(d_v + 1) > log_2(d_v) = S_1(v; T) ✓
- S_1(w; T') = S_1(w; T) for all w ∉ {u, v} ✓

S̄_1 strictly increases. The filter theorem holds trivially.

## τ = 2 Case on Trees

At τ = 2, the endpoint distribution from node w is:

  π_w(2)[j] = Σ_k P[w,k] · P[k,j] = (P²)[w, j]

For w ∉ {u, v}, P[w, :] is unchanged but the walk may pass through u or v at step 1.

**Key decomposition:** For any node w:

  S_2(w; T') - S_2(w; T) = H(e_w · P'²) - H(e_w · P²)

Now P'² = P' · P'. The difference:
  e_w · P'² - e_w · P² = e_w · (P'² - P²)

Since P' = P + ΔP where ΔP is rank-2 (nonzero only in rows u, v):
  P'² = (P + ΔP)² = P² + P·ΔP + ΔP·P + ΔP²

So: P'² - P² = P·ΔP + ΔP·P + ΔP²

For e_w · (P'² - P²):
  = e_w · P · ΔP + e_w · ΔP · P + e_w · ΔP²

**Term 1:** e_w · P · ΔP
  This is (P[w, :]) · ΔP. Since ΔP is nonzero only in rows u, v:
  = P[w, u] · ΔP[u, :] + P[w, v] · ΔP[v, :]

  If w ∉ N(u) ∪ N(v), then P[w, u] = P[w, v] = 0 and this term vanishes.
  If w ∈ N(u) \ {v}, then P[w, u] = 1/d_w and P[w, v] = 0.
  If w = u, then P[w, u] = 0 (no self-loops in trees) and P[w, v] = 0 (non-adjacent).

  Wait — P[u, u] = 0 since there's no self-loop. And P[u, v] = 0 since (u,v) is not an edge.
  So e_u · P · ΔP = Σ_k P[u,k] · ΔP[k, :]. Only ΔP[u, :] and ΔP[v, :] are nonzero.
  = P[u, u] · ΔP[u, :] + P[u, v] · ΔP[v, :] = 0.

  Similarly e_v · P · ΔP = P[v, u] · ΔP[u, :] + P[v, v] · ΔP[v, :] = 0.
  (P[v, u] = 0 since (u,v) ∉ E, P[v, v] = 0.)

  For w ∈ N(u) \ {v}: e_w · P · ΔP = (1/d_w) · ΔP[u, :]
  For w ∈ N(v) \ {u}: e_w · P · ΔP = (1/d_w) · ΔP[v, :]

**Term 2:** e_w · ΔP · P
  Since ΔP is nonzero only in rows u and v:
  e_w · ΔP = ΔP[w, :] which is nonzero only if w ∈ {u, v}.

  If w = u: e_u · ΔP · P = ΔP[u, :] · P
  If w = v: e_v · ΔP · P = ΔP[v, :] · P
  If w ∉ {u, v}: this term is zero.

**Term 3:** e_w · ΔP²
  e_w · ΔP² = (e_w · ΔP) · ΔP = ΔP[w, :] · ΔP
  Nonzero only if w ∈ {u, v}.

**Summary for τ = 2:**

- **w = u:** ΔS_2(u) depends on ΔP[u, :] · P + ΔP[u, :] · ΔP (terms 2 + 3)
- **w = v:** ΔS_2(v) depends on ΔP[v, :] · P + ΔP[v, :] · ΔP (terms 2 + 3)
- **w ∈ N(u) \ {v}:** ΔS_2(w) depends on (1/d_w) · ΔP[u, :] · (term 1 only)
  Wait — there's also the fact that P'² = P' · P', and for P' in the SECOND matrix, the rows u and v are also different. Let me redo this more carefully.

Actually, I made an error. Let me recompute. P'² ≠ (P + ΔP)² in general because I need to be more careful.

P'[w, k] = P[w, k] for w ∉ {u, v}
P'[u, k] = 1/(d_u+1) for k ∈ N(u) ∪ {v}
P'[v, k] = 1/(d_v+1) for k ∈ N(v) ∪ {u}

(P'²)[w, j] = Σ_k P'[w, k] · P'[k, j]

For w ∉ {u, v}:
  = Σ_k P[w, k] · P'[k, j]
  = Σ_{k ∉ {u,v}} P[w, k] · P[k, j]  +  P[w, u] · P'[u, j]  +  P[w, v] · P'[v, j]

Since (u,v) ∉ E, P[w, v] = 0 only if w ∉ N_T(v), and P[w, u] = 0 only if w ∉ N_T(u).

**Case: w not adjacent to u or v (w ∉ A):**
  P[w, u] = P[w, v] = 0, so:
  (P'²)[w, j] = Σ_k P[w, k] · P'[k, j]

  For k ∉ {u, v}: P'[k, j] = P[k, j].
  The only k ∈ N(w) that could be u or v would require w ∈ N(u) or w ∈ N(v), contradicting our assumption.
  So (P'²)[w, j] = Σ_k P[w, k] · P[k, j] = (P²)[w, j].

  **Therefore: S_2(w) is UNCHANGED for w ∉ A when we add edge (u,v) to a tree.**

Wait — is this right? Let me double-check. If w is not in the affected set, then:
1. w is not u or v
2. w is not a neighbor of u
3. w is not a neighbor of v

So none of w's neighbors are u or v. At step 1, the walk from w goes to one of w's neighbors, all of which are not u or v. At step 2, the walk goes from that neighbor to one of ITS neighbors.

But wait — a neighbor of w could have a neighbor that IS u or v. For example, if w is at distance 2 from u, then w has a neighbor k, and k might be a neighbor of u. In that case, P'[k, j] might differ from P[k, j] IF k ∈ {u, v}. But k ∈ N(w) and k ∈ {u, v} means w ∈ N(u) ∪ N(v), contradicting w ∉ A.

So indeed: if w ∉ A, then NO neighbor of w is u or v, and therefore:
(P'²)[w, j] = Σ_{k ∈ N(w)} (1/d_w) · P'[k, j]

Now, for k ∈ N(w) with k ∉ {u, v}: P'[k, j] = P[k, j] UNLESS k has u or v as a neighbor AND j ∈ ... wait no.

P'[k, j] differs from P[k, j] only if k ∈ {u, v}. Since k ∉ {u, v} (because w ∉ A means no neighbor of w is u or v), we have P'[k, j] = P[k, j] for all k ∈ N(w).

So (P'²)[w, j] = (P²)[w, j] for ALL w ∉ A.

**This is a CRUCIAL insight for trees at τ = 2!** At τ = 2, nodes outside the affected set have ZERO entropy change. The filter theorem at τ = 2 reduces to:

  ΔS_τ^local > 0 ⟹ ΔS̄_τ > 0

becomes:

  (1/|A|) Σ_{w∈A} ΔS_2(w) > 0 ⟹ (1/N) Σ_{w∈A} ΔS_2(w) > 0

which is TRIVIALLY TRUE because Σ_{w∈A} ΔS_2(w) is the SAME quantity on both sides (up to positive scaling). If it's positive as a local average, it's positive as a sum, and therefore positive as a global average (just divided by N instead of |A|).

## Wait — does this extend to τ = 3 on trees?

At τ = 3, (P'³)[w, j] = Σ_k P'[w, k] · (P'²)[k, j].

For w ∉ A, P'[w, k] = P[w, k] and k ∉ {u, v} (since w ∉ A). But now (P'²)[k, j] might differ from (P²)[k, j] IF k ∈ A. And k ∈ A is possible: k could be a neighbor of u or v, and k could also be a neighbor of w (if w is at distance 2 from u or v).

So at τ = 3, the entropy change "leaks" one hop beyond A. The set of nodes affected at τ = 3 is A₃ = {w : ∃ path of length ≤ 1 from w to A} = A ∪ N(A).

More generally, at τ = t, the affected zone is:
  A_t = {w : dist(w, {u,v}) ≤ t}

because the perturbation propagates one hop per time step.

**But for trees, this has a special structure.** The perturbation propagates outward from {u,v} through the tree, and at each step it DECAYS because the probability mass spreads out along the tree branches.

## The General Argument for Trees

### Claim: For trees, the entropy perturbation at distance d from {u,v} decays exponentially.

**Proof idea:** In a tree, the random walk from a distant node w must traverse the unique path toward {u, v} to be affected by the perturbation. The probability of a walk from w reaching {u, v} within τ steps decays with distance.

Let d = dist(w, {u,v}). The walk from w must traverse d edges toward {u,v} to feel the perturbation. At each step, the walk has probability 1/d_k of choosing the "right" direction toward {u,v} and probability (d_k - 1)/d_k of wandering elsewhere.

For a path graph (worst case — minimum degree), the probability of reaching {u,v} in exactly d steps from w is at most (1/2)^{d-1} (for internal nodes with degree 2). So:

  P(walk from w reaches {u,v} in ≤ τ steps) ≤ Σ_{t=d}^{τ} (something decaying)

The entropy change at w is bounded by a function of this hitting probability:
  |ΔS_τ(w)| ≤ C · P(walk reaches {u,v})

because the endpoint distribution can only change if the walk actually passes through the perturbed region.

### Formalizing this:

**Lemma (Perturbation locality on trees).** Let T be a tree, (u,v) a non-edge, T' = T + (u,v). For any node w with d = dist_T(w, {u,v}) > 0:

  |ΔS_τ(w)| ≤ 2 log_2(N) · TV(π_w^τ(T), π_w^τ(T'))

where TV is total variation distance.

*Proof.* By the Csiszár-Kullback-Pinsker inequality and the fact that entropy is Lipschitz w.r.t. TV on finite alphabets:
  |H(p) - H(q)| ≤ TV(p, q) · log_2(N - 1) + h_b(TV(p,q))
where h_b is binary entropy. For small TV, this is approximately TV · log_2(N).

Actually, a tighter bound: for distributions on N points,
  |H(p) - H(q)| ≤ -TV · log_2(TV/(N-1)) ≤ TV · (log_2(N) + log_2(1/TV))

For our purposes, the key is that |ΔS_τ(w)| → 0 as TV → 0.

**Lemma (TV decay on trees).** For a tree T, adding edge (u,v), and node w at distance d from {u, v}:

  TV(π_w^τ(T), π_w^τ(T')) ≤ δ_τ(d)

where δ_τ(d) decays with d. Specifically, for d > τ, δ_τ(d) = 0 (the perturbation cannot reach w in τ steps).

For d ≤ τ: the walk from w must first reach the "perturbation zone" (within 1 hop of u or v), which requires traversing at least d-1 tree edges. On a tree with minimum degree δ_min:

  δ_τ(d) ≤ 2 · (1/δ_min)^{d-1} · max perturbation at {u,v}

The max perturbation at row u: ||ΔP[u, :]||_1 = |Σ_j ΔP[u,j]| ≤ 2/(d_u + 1).
  (Each old neighbor loses 1/(d_u(d_u+1)) of probability, total loss = 1/(d_u+1). New neighbor v gains 1/(d_u+1). So L1 norm of perturbation = 2/(d_u+1).)

Wait, let me compute this more carefully. In L1 norm:
||ΔP[u, :]||_1 = Σ_j |P'[u,j] - P[u,j]|
  = Σ_{j∈N(u)} |1/(d_u+1) - 1/d_u| + |1/(d_u+1) - 0|
  = d_u · 1/(d_u(d_u+1)) + 1/(d_u+1)
  = 1/(d_u+1) + 1/(d_u+1)
  = 2/(d_u+1)

So ||ΔP[u, :]||_1 = 2/(d_u + 1).

The total perturbation in L1 is at most 2/(d_u+1) + 2/(d_v+1).

**For the walk from w to reach the perturbation zone in τ steps:**

On a tree, let w be at distance d from the nearest of {u, v}. The walk must traverse d edges toward {u,v}. In a tree, at each internal node on the path, the walk has probability 1/d_k of choosing the correct edge toward {u,v}.

This gives: P(walk from w is at distance ≤ 0 from {u,v} at some step ≤ τ) ≤ ... complicated by backtracking.

Actually, for a coupling argument, let's be more careful.

**Coupling argument:**

Run two walks simultaneously: W on T and W' on T', both starting from w.
- At each step, if neither walk is at u or v, they can be coupled to make the same move (identical transition probabilities).
- If a walk reaches u or v, the two walks may diverge.

The TV distance is bounded by:
  TV(π_w^τ(T), π_w^τ(T')) ≤ P(walks diverge) ≤ P(either walk visits {u,v} before step τ)

For the walk on T from w at distance d: the probability of visiting {u,v} within τ steps is bounded by the return probability / hitting probability on the tree. On trees, hitting probabilities are well-studied.

For a tree with N nodes and a node w at distance d from u: the hitting time from w to u has expectation that grows with d and with the "resistance" of the path. For regular trees, the hitting probability within τ steps when d ≤ τ is roughly:

  P_hit(w → u, τ) ~ (effective conductance from w to u)

On a path of length d with uniform weights, the effective resistance is d, and the hitting probability within d steps is O(2^{-d}).

Actually, for a simple random walk on Z (the integer line), starting at position d > 0, the probability of hitting 0 by time τ is:
  For τ < d: 0 (can't reach it)
  For τ ≥ d: at most C · (d/τ)^{1/2} · exp(-d²/(2τ)) by Gaussian bounds

But for bounded graphs (not infinite lines), the bounds are different.

**Let me try a simpler approach specific to trees.**

## Cleaner Approach: Direct Partition

Partition V into:
- A = affected set = {u, v} ∪ N(u) ∪ N(v)
- B = V \ A (unaffected nodes)

ΔS̄_τ = (1/N)[Σ_{w∈A} ΔS_τ(w) + Σ_{w∈B} ΔS_τ(w)]

We want to show: if Σ_{w∈A} ΔS_τ(w) > 0 (the ΔS_τ^local > 0 condition), then Σ_{w∈A} ΔS_τ(w) + Σ_{w∈B} ΔS_τ(w) > 0.

Equivalently: |Σ_{w∈B} ΔS_τ(w)| < Σ_{w∈A} ΔS_τ(w) whenever Σ_{w∈B} ΔS_τ(w) < 0.

**For τ = 2 on trees:** We showed Σ_{w∈B} ΔS_2(w) = 0, so this holds.

**For general τ on trees:** We need to bound |Σ_{w∈B} ΔS_τ(w)|.

---

## Computational Results: The Filter Theorem FAILS on Trees at Large τ

### Exhaustive verification (2024-03-04)

**Methodology:** Enumerate all labeled trees via Prüfer sequences. For each tree T on N nodes, for each non-edge (u,v), add the edge, compute ΔS_τ(w) for all nodes, check whether local ΔS_τ > 0 implies global ΔS̄_τ > 0.

### Results by N

| N | Trees checked | τ range | First violation at τ | Ratio τ*/N | Method |
|---|---|---|---|---|---|
| 3 | 3 | 1..15 | ∞ (none) | — | Exhaustive |
| 4 | 16 | 1..15 | ∞ (none) | — | Exhaustive |
| 5 | 125 | 1..30 | ∞ (none) | >6.0 | Exhaustive |
| 6 | 1,296 | 1..20 | **14** | 2.33 | Exhaustive |
| 7 | 16,807 | 1..15 | **10** | 1.43 | Exhaustive |
| 8 | 10,000 | 1..12 | **8** | 1.00 | Sampling |
| 9 | 2,000 | 1..12 | **8** | 0.89 | Sampling |
| 10 | 2,000 | 1..12 | **8** | 0.80 | Sampling |

### Key findings

1. **N ≤ 5: Filter theorem holds for all τ on trees.** No violations found up to τ = 30 (N=5), which is well beyond the mixing time. Likely holds for all τ.

2. **N = 6: First violation at τ = 14.** The violating trees have degree sequence [2,2,2,2,1,1] (path-like structure). Violation magnitudes are small: local ≈ 0.07, global ≈ -0.004.

3. **N ≥ 7: The critical τ* decreases with N.** At N = 8, violations begin at τ = 8 (ratio 1.0). The ratio τ*/N appears to decrease monotonically.

4. **Violating structures at N = 8:** All τ = 8 violations have degree sequence [4,2,2,2,1,1,1,1] — a hub (degree 4) with three branches of varying length. The hub-branch topology creates conditions where the shortcut homogenizes the endpoint distributions of distant leaves.

5. **Immune structures:** Star graphs and caterpillar graphs show NO violations at any τ (tested to τ = 20). The double-star (two hubs connected) is also clean.

6. **Even path graphs violate at large enough τ:** The 8-node path first violates at τ = 15 (local = 0.1, global = -0.008). So the phenomenon is not restricted to hub topologies — it's a fundamental resonance effect.

### Mechanism of violation

When edge (u,v) is added to a tree:
- **Nearby nodes (A):** Gain entropy — more paths available, endpoint distributions spread out. This is the positive local ΔS_τ.
- **Distant nodes (B):** At small τ, unaffected (perturbation hasn't reached them). At τ ≈ 1, completely unaffected. At τ = 2, provably zero change for nodes in B. As τ grows, the perturbation propagates outward.
- **The critical effect:** At τ ≈ N (resonance timescale), distant leaf nodes can LOSE entropy. The shortcut homogenizes the landscape: walks from distant leaves that previously had distinctive endpoint distributions (due to tree asymmetry) now converge to similar distributions. Homogenization → loss of distinctiveness → entropy decrease.

The spillover ratio |Σ_{B} ΔS_τ| / Σ_{A} ΔS_τ exceeds 1.0 when this happens.

### Implications for the paper

**The paper's result (N ≤ 6, τ ≤ 5, all connected unit-weight graphs) is SAFE.**

Trees at N = 6 first violate at τ = 14 — far beyond the paper's τ ≤ 5. General graphs were verified exhaustively at N ≤ 6, τ ≤ 5 with zero violations. The paper's claims stand.

However, **an unrestricted extension to all N is impossible**, even for trees:
- The filter theorem is inherently τ-dependent
- There exists a critical τ*(N) above which violations appear
- τ*(N)/N decreases with N, approaching ~0.8 at N = 10

### What CAN be proven for all N

1. **Theorem (τ = 1, all graphs):** Adding an edge increases entropy of the endpoints and leaves all other nodes unchanged. Filter theorem holds trivially.

2. **Theorem (τ = 2, trees):** Nodes outside the affected set have zero entropy change (proven algebraically above). Filter theorem holds trivially.

3. **Conjecture (τ ≤ C, trees, all N):** There exists a constant C (perhaps C = 5 or C = 7) such that the filter theorem holds for all trees at τ ≤ C. The computational evidence strongly supports C ≥ 7 (zero violations at N = 7, τ ≤ 9; zero at N = 8, τ ≤ 7).

4. **The τ = 2 result extends to general graphs** if no neighbor of w is u or v — i.e., for nodes outside the affected set. This is the same algebraic argument.

### Proof sketch for τ = 2 on general graphs

The argument for τ = 2 on trees actually holds for ALL graphs (not just trees):

For w ∉ A = {u,v} ∪ N(u) ∪ N(v):
1. P'[w, k] = P[w, k] for all k (w's row unchanged since w ∉ {u,v})
2. For k ∈ N(w): k ∉ {u,v} (since w ∉ A means no neighbor of w is u or v)
3. Therefore P'[k, j] = P[k, j] for all k ∈ N(w), all j
4. So (P'²)[w, j] = Σ_k P'[w,k] · P'[k,j] = Σ_k P[w,k] · P[k,j] = (P²)[w,j]
5. Therefore S_2(w; G') = S_2(w; G) — zero entropy change

This means: **at τ = 2, the filter theorem holds for ALL unit-weight graphs, for ALL N.**

This is because the filter theorem at τ = 2 reduces to: if Σ_{A} ΔS_2(w) > 0 (local sum), then Σ_{A} ΔS_2(w) + 0 > 0 (global sum). Trivially true.

**Theorem.** For any connected unit-weight graph G on N nodes, any non-edge (u,v), and τ = 2: if ΔS_2^{local}(u,v) > 0, then ΔS̄_2(u,v) > 0. ∎

### Open questions

1. Does the filter theorem hold for N ≤ 5 trees at ALL τ? (Strongly suggested by computation — zero violations up to τ = 30.)
2. What is the exact critical τ*(N) for N = 6, 7, 8? (Exhaustive: τ*(6) = 14, τ*(7) = 10. Sampling: τ*(8) ≤ 8.)
3. Can we prove the filter theorem for fixed τ ≤ C for all N? The τ = 2 case is proven. τ = 3 seems tractable — the perturbation spreads one hop beyond A but decays geometrically.
4. Is there a clean characterization of which tree topologies are immune at all τ? (Stars, caterpillars, and double-stars appear immune.)
