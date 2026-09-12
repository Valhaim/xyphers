---
title: "Can a Local Future Test Protect a Xypher Network?"
description: "A proposed connection spreads nearby outcomes more evenly. Does that local gain guarantee that the network-wide average also rises?"
type: "research-article"
status: "published"
maturity: "bounded-graph-result"
published: "2026-08-18"
revised: "2026-08-18"
website_path: "/research/can-a-local-future-test-protect-a-xypher-network/"
web_status: "live"
---

# Can a Local Future Test Protect a Xypher Network?

Two nodes in a network are not connected. The network can add the missing edge, but first it asks a local question: do the possible futures become more diverse near the two endpoints?

At one or two steps, that test gives an exact guarantee. If the average future spread rises across the endpoints and their original neighbours, the same average rises across the whole graph. The proof works for graphs of any size in the declared model, where every connection has equal strength.

At three steps, the guarantee can fail. A node just beyond the measured neighbourhood can lose more than the nearby nodes gain. In the exhaustive scans through seven nodes and five steps, the first such unit-weight failures appear on seven-node graphs at a three-step horizon.

So the answer to the title is bounded: **yes for the exact two-step test; no for the same threshold used as a universal network-safety rule.** The result concerns two averages of endpoint entropy. It does not, by itself, prove that an actor benefits, that every node benefits, or that an AI system is aligned.

The [Core Thesis](core-thesis.md#hyperintelligence) asks whether a Xypher can make individually rewarded action and collective improvement coincide. Here the measurable object underneath that proposal is a single edge addition on a graph.

## One more edge can narrow a future

Begin with four nodes numbered 0 through 3. Their connections are:

```text
0—1   0—2   0—3   1—2
```

Now consider adding **1—3**.

To measure the future from node 1, release a walker there. At each junction it chooses uniformly among the available connections. After exactly two moves, record where it has ended. Repeating this thought experiment gives a probability distribution over the four possible endpoints.

Before the new edge, node 1 has two first moves, to 0 or 2. Its two-step endpoint probabilities are:

| Endpoint | 0 | 1 | 2 | 3 |
|---|---:|---:|---:|---:|
| Before adding 1—3 | 1/4 | 5/12 | 1/6 | 1/6 |
| After adding 1—3 | 1/3 | 4/9 | 1/9 | 1/9 |

No endpoint disappears. The probabilities become less even: after the edge, seven ninths of the mass lies on nodes 0 and 1.

Shannon entropy turns that spread into a number. For a starting node **i** and a horizon of **τ** moves, let **p<sup>(τ)</sup><sub>ij</sub>** be the chance of ending at node **j**. Then:

$$
S_\tau(i) = -\sum_j p_{ij}^{(\tau)} \log_2 p_{ij}^{(\tau)}
$$

The capital sigma means “add one term for every endpoint.” The base-two logarithm sets the unit to bits. A distribution spread evenly across many endpoints has high entropy. A distribution piled onto a few endpoints has less.

For node 1, the value falls from **1.888 bits** to **1.753 bits**. The added connection did not remove a destination. It concentrated the odds of where the walker would finish.

This quantity is **endpoint entropy**. It measures the distribution after exactly τ moves, not the entropy of every complete route taken along the way. That distinction stays in force throughout this article.

## The network average can fall

One starting node can lose while the rest gain. A stronger question is whether the average across all starting nodes can fall.

It can. Take six nodes with these seven edges:

```text
0—1   0—2   0—3   0—5   1—2   1—3   1—4
```

Add **0—4**, and follow each walker for five moves. The table gives every endpoint distribution in node order **[0, 1, 2, 3, 4, 5]**. Probabilities are rounded to four decimal places; the entropies and averages were computed from the exact fractions.

| Start | Before: five-step endpoint probabilities | Entropy before | After: five-step endpoint probabilities | Entropy after | Change |
|---:|---|---:|---|---:|---:|
| 0 | [0.2031, 0.2666, 0.1768, 0.1768, 0.0742, 0.1025] | 2.474808 | [0.2213, 0.2011, 0.1636, 0.1636, 0.1636, 0.0868] | 2.534769 | +0.059961 |
| 1 | [0.2666, 0.2031, 0.1768, 0.1768, 0.1025, 0.0742] | 2.474808 | [0.2514, 0.2025, 0.1564, 0.1564, 0.1564, 0.0769] | 2.507805 | +0.032997 |
| 2 | [0.3535, 0.3535, 0.0977, 0.0977, 0.0488, 0.0488] | 2.141557 | [0.4091, 0.3128, 0.0800, 0.0800, 0.0800, 0.0381] | 2.106214 | −0.035343 |
| 3 | [0.3535, 0.3535, 0.0977, 0.0977, 0.0488, 0.0488] | 2.141557 | [0.4091, 0.3128, 0.0800, 0.0800, 0.0800, 0.0381] | 2.106214 | −0.035343 |
| 4 | [0.2969, 0.4102, 0.0977, 0.0977, 0.0313, 0.0664] | 2.119071 | [0.4091, 0.3128, 0.0800, 0.0800, 0.0800, 0.0381] | 2.106214 | −0.012857 |
| 5 | [0.4102, 0.2969, 0.0977, 0.0977, 0.0664, 0.0313] | 2.119071 | [0.4338, 0.3075, 0.0763, 0.0763, 0.0763, 0.0300] | 2.047002 | −0.072069 |

The gains at nodes 0 and 1 do not cover the four losses. The graph-wide average moves from **2.245145 bits** to **2.234703 bits**, a decrease of **0.010442 bits per node**.

In the exhaustive small-graph record, **26,910** graph–candidate-edge–horizon cases have a lower global average after the edge. Adding connections without a screen is therefore unsafe even for this one declared measure.

The word “global” needs care. It means an average over all starting nodes. It does not mean welfare, and it does not mean that each node improved.

## Test the doorstep

The screen does not inspect one endpoint alone. For a proposed edge between non-adjacent nodes **u** and **v**, it takes the two endpoints and every node that was an immediate neighbour of either endpoint before the edge was added:

$$
A_G(u,v) = \{u,v\} \cup N_G(u) \cup N_G(v)
$$

Call this the **affected set**. In the equation, **N<sub>G</sub>(u)** means the neighbours of **u** in the original graph **G**. “Affected” here names the measurement boundary; at longer horizons, nodes outside it can also change.

Let **V** be the set of all **N** nodes, and let **|A|** be the number of nodes in the affected set. Let **Δ<sub>i</sub>** be node **i**'s endpoint-entropy change after adding the edge. The tested local score **L** is the average over the affected set. The whole-network score **W** is the average over **V**:

$$
L_\tau(u,v) = \frac{1}{|A|}\sum_{i\in A}\Delta_i,
\qquad
W_\tau(u,v) = \frac{1}{N}\sum_{i\in V}\Delta_i.
$$

The threshold rule accepts when **L<sub>τ</sub> > 0**. The protection question is now exact: does **L<sub>τ</sub> > 0** force **W<sub>τ</sub> > 0**?

That is a question about two averages. An actor has not yet entered the definition.

## Four possible verdicts

The finite classifier enumerated every connected labelled graph with **N = 3, 4, 5, or 6** nodes. “Labelled” means that the node identities count, so two drawings with the same shape but different assignments of node numbers remain separate cases. There are **27,474** such connected graphs.

For every graph, the program considered every missing edge and every horizon from one through five. One case is therefore a tuple: **graph, candidate edge, horizon**. There were **947,935** cases.

| Affected-set average | Graph-wide average rises | Graph-wide average falls |
|---|---:|---:|
| **L<sub>τ</sub> > 0** | **919,585** | **0** |
| **L<sub>τ</sub> ≤ 0** | **1,440** | **26,910** |

The zero is in the safety-critical box: no edge that passed the local threshold lowered the global average in this finite domain. The **1,440** in the opposite corner matter too. Some edges failed the local threshold even though the global average rose. The screen had false negatives, not a perfect match between local and global signs.

The classifier represents entropy with standard computer floating-point numbers and treats changes within **10<sup>−10</sup>** of zero as neutral. It found no locally positive, globally flat cases either, so the positive implication was strict at that tolerance.

These numbers are sometimes compressed into “5.67 million checks.” That larger number counts something else: **5,671,025 individual node–graph–candidate-edge–horizon comparisons**. Among them, **772,758** individual nodes lost endpoint entropy. They are evidence against every-node benefit, not millions of independently safe edge additions.

## Why two steps are exact

The finite table covers graphs only through six nodes. The two-step result needs no size limit.

Write **P** for the table of one-step probabilities before the edge and **P′** for the table after it. Adding **u—v** changes only two rows: the choices available when the walker is standing at **u** or at **v**.

At one step, only walkers starting at those endpoints see a changed distribution. Each endpoint gains one equally weighted option, so its one-step entropy rises; every other row remains fixed.

At two steps, consider a starting node **w** outside the affected set. It is neither endpoint, and it is not adjacent to either endpoint. Its first move therefore cannot land on **u** or **v**. The row used for that first move is unchanged, and every row used for the second move is unchanged as well. In the equation below, **e<sub>w</sub>** means a walker placed with certainty at **w**; multiplying it by the one-step table twice gives the two-step endpoint distribution. That distribution is identical before and after:

$$
w \notin A \quad\Longrightarrow\quad e_w {P'}^2 = e_w P^2
\quad\Longrightarrow\quad \Delta_w = 0.
$$

Every nonzero change is confined to **A**. The two sums are consequently the same sum with different positive divisors:

$$
N W_2 = \sum_{i\in V}\Delta_i = \sum_{i\in A}\Delta_i = |A|L_2.
$$

If **L<sub>2</sub>** is positive, **W<sub>2</sub>** must be positive. The one-step case is even more direct. This proves the implication at **τ ≤ 2** for connected simple unit-weight graphs of any size, using the endpoint observable and affected-set definition above. Here **simple** means undirected, with no self-connections or repeated edges; **unit-weight** means that every connection has equal strength.

The proof does not say that every proposed edge passes. It says that every edge which passes this particular threshold has the same sign globally at one or two steps.

## The third step crosses the doorstep

At three steps, a walker starting two connections away can reach **u** or **v** and then use a changed row. Its endpoint distribution can move even though its starting node lies outside **A**. The identity **N W = |A|L** is gone.

Within the exhaustive scans through seven nodes and five steps, the first unit-weight threshold failure appears at seven nodes and τ = 3. The search covered all **1,866,256 connected labelled graphs** on seven nodes, every missing edge, and horizons **τ = 3, 4, and 5**. Across **56,779,632 graph–candidate-edge–horizon cases**, **117,180** had a positive affected-set average and a negative graph-wide average. Of those, **5,040** occurred at τ = 3. The stored research record gives conflicting splits for the remaining τ = 4 and τ = 5 cases, so only their combined contribution is used here.

The result establishes that failures occur already at **τ = 3**. It does not establish that every graph, every candidate, or every horizon at or above three fails.

A tempting repair is to ban leaves by requiring every node to have at least two neighbours. On seven-node graphs this removes the τ = 3 failures, but **34,020** failures remain at τ = 4. The repair is already non-universal.

The short-horizon problem then returns. At **N = 8**, an exhaustive τ = 3 search considered all **169,488,200** connected labelled graphs with minimum degree at least two. Among **1,952,212,864 locally positive graph–candidate-edge checks** at that fixed horizon, **384,720** lowered the global average. The vulnerable node in the recorded examples has degree two and sits outside the affected set while connecting into a high-degree core. It is peripheral, but it is not a leaf. The N = 7 and N = 8 verifiers use **10<sup>−12</sup>** as their numerical sign tolerance.

The evidence forms a scope staircase:

| Domain | Result |
|---|---|
| Any graph size, τ ≤ 2 | Positive local average implies positive global average by algebraic locality |
| N = 3…6, τ = 1…5 | 919,585 approvals; zero globally negative approvals |
| N = 7, τ = 3…5 | 117,180 threshold failures; first ones at τ = 3 |
| N = 7, minimum degree ≥ 2 | 34,020 failures at τ = 4 |
| N = 8, τ = 3, minimum degree ≥ 2 | 384,720 failures among 1,952,212,864 approvals |

Other restrictions need their own accounting. A deterministic finite search in which connections could have unequal strengths found counterexamples at horizons of three or more. That establishes existence inside its sampled weighting design, not a sharp critical weight ratio.

Trees are not covered by a universal rescue either. Exhaustive enumeration produced **419,040,783 clean graph–candidate-edge–horizon checks** across all labelled trees on seven through nine nodes at τ = 3 through 5. A deterministic floating-point evaluation of a separate distance-three tree family reports a τ = 3 counterexample at **N = 1,770**: one endpoint is a leaf, the next node has degree two, a hub has degree 1,762, and the other endpoint has degree seven. The affected-set sum is **+0.01407462**, while the nodes outside it contribute **−0.01419718**, leaving a graph-wide sum of **−0.00012256**. This is one restricted-family counterexample; it does not identify the smallest failing tree.

A different finite program chose the missing edge with the largest **unnormalised sum** over the affected set and reported no failures in its declared N = 7 and N = 8 scans at τ = 3 and 5 with minimum-degree restrictions. That is evidence about a centralized, all-graph choice rule. It is not yet a theorem about the canonical score: the canonical score is an **average**, so two candidates with different affected-set sizes can reverse order when a sum replaces that average. An embodied actor may also own only some candidate edges, rather than every missing edge in the graph. The maximum-policy claim remains withheld until the score, ownership boundary, available actions, and proof all match.

## What the two averages do not mean

Return to the **772,758 individual losses** in the small-domain comparison. A positive graph-wide average plainly allows some nodes to lose. It is not a Pareto improvement, which would require that no individual becomes worse off.

Nor is **L<sub>τ</sub>** automatically the proposing actor's self-interest. It averages the changes at both endpoints and all their original neighbours. Calling that score an actor's payoff requires a model in which the actor owns that whole boundary. The model must also specify which candidate edges the actor can choose and whether it accepts any positive edge or maximizes among its own alternatives.

The graph-wide average is equally specific. It is the mean of endpoint entropies. It does not measure material welfare, moral value, generosity, power, consent, or benefit to every node. Those interpretations need separate bridges and can fail even while **W<sub>τ</sub> > 0**.

The proved object is therefore smaller than a complete Xypher:

- a connected, add-only, simple unit-weight **Graph Substrate**;
- the Shannon entropy of exactly τ-step endpoint distributions;
- an affected-set threshold screen for one proposed edge.

No independently grounded **Thermodynamic Harness** appears in the proof. There is no reservoir-derived **α**, contact test, or THAIM accounting. No **Praxion** is placed inside the graph with an owned action set, perception boundary, memory, and outcome loop. The graph calculation does not become thermodynamics or alignment merely because a Xypher could use it as one readout.

Existing language models are not shown to instantiate this graph, this payoff, or this action rule. Translating the result to AI alignment would require, at minimum, a declared actor boundary, the actions it can actually take, a reason its payoff equals the affected-set average, a bridge from endpoint distributions to the future quantity it controls, and a defensible collective measure.

## The answer inside the boundary

A local future test can protect one declared network average when the horizon is no more than two steps. In that setting, the edge's influence cannot reach beyond the measured doorstep, so the local and global signs are algebraically locked together. The N = 3…6 classification extends the observed zero-failure domain through five steps, but only for those finite graph sizes.

The same threshold is not a universal protection rule. Seven-node counterexamples end that claim, and the eight-node result shows that removing leaves does not restore it at τ = 3. Weighted graphs, trees, maximum-choice policies, actor payoffs, path observables, thermodynamics, and AI alignment each require their own result.

Inside the exact graph class, horizon, observable, and affected-set rule, the test does what it says. Outside them, the word “protect” has not yet been earned.

## Evidence

**Read:** The [evidence guide](../evidence/local-future-test/README.txt) separates the exact result from the historical conclusions still present in the copied programs. The [expected-results dictionary](../evidence/local-future-test/results/EXPECTED-RESULTS.txt) defines every counted unit and tolerance.

**Inspect:** The [canonical endpoint-entropy implementation](../evidence/local-future-test/reproduction/src/entropy_exact.rs) defines the affected-set **average**. The [small-domain classifier](../evidence/local-future-test/reproduction/src/bin/converse_verify.rs) produces the 947,935-case table. The [node-level verifier](../evidence/local-future-test/source/lemma2_verify.rs) supplies the 5,671,025 comparison count; its historical header and conclusion do not control the four-way result. The [two-step proof record](../evidence/local-future-test/proof/two-step-and-tree-record.md) gives the locality argument.

The later boundary is inspectable in the [N = 7 verifier](../evidence/local-future-test/reproduction/src/bin/tau_bounded_verify.rs), the [N = 7 minimum-degree breakdown](../evidence/local-future-test/reproduction/src/bin/dmin2_detail.rs), and the [exhaustive N = 8 verifier](../evidence/local-future-test/reproduction/src/bin/m3_verify_n8.rs). The [weighted verifier](../evidence/local-future-test/reproduction/src/bin/weighted_filter_verify.rs) records its finite sampling design. The [distance-three tree evaluator](../evidence/local-future-test/reproduction/src/bin/dist3_limit.rs) supplies the N = 1,770 case. The [maximum-edge probe](../evidence/local-future-test/source/m5_max_edge.rs) exposes the sum-versus-average and centralized action-set differences.

**Download:** [the checksummed standalone evidence packet](../downloads/xypher-local-future-test-evidence-v1.zip) ([SHA-256](../downloads/xypher-local-future-test-evidence-v1.zip.sha256)).

**Run:** After extracting the packet, the small four-way classification can be reproduced with:

```bash
cd reproduction
cargo run --release --bin converse-verify -- --max-n 6 --max-tau 5
```

The [run guide](../evidence/local-future-test/RUN.txt) also gives the tree command and explains why the N = 7 and N = 8 exhaustive programs are supplied primarily for inspection. The N = 8 program traverses all 2<sup>28</sup> possible edge sets before filtering and is computationally expensive.
