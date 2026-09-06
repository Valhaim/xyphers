# Valhaim thermodynamic-kernel preregistration

Status: **freeze boundary: the first commit containing this protocol**

Date: 2026-08-03

Parent task: `td-5d4a6f`

Freeze task: `td-33e84c`

This protocol asks whether Valhaim can deliberately construct a small,
payment-native thermodynamic kernel. It does not ask whether the current
adaptive economy spontaneously produces thermodynamics. That separate
question belongs to CAL-KINETICS.

The first commit containing this file and its freeze manifest is the protocol
freeze. No VAL-KERNEL runner exists at that boundary. The completed runner,
including every named mutation, receives a second committed identity before
its first execution. No output from either baseline or mutation code may be
used to alter the fixture, predictions, acceptance rules, or controls below.
Any later change must be listed as a deviation in `RESULTS.md` before the
changed runner is executed.

## 1. Frozen claim ladder

The experiment earns conclusions in order. A later conclusion may not be
used to rescue an earlier failed gate.

| Step | Frozen question | Strongest allowed positive conclusion |
|---:|---|---|
| 1 | Are the claim, boundary, inputs, predictions, and falsifiers frozen before output? | The experiment is interpretable rather than target-fitted. |
| 2 | Is there one finite closed payment body with a complete state and fixed opportunity alphabet? | A finite closed candidate body is specified. |
| 3 | Are multiplicity, entropy, energy, and reservoir parameter $\alpha$ constructed independently of rates? | The candidate has independent thermodynamic coordinates. |
| 4 | Does a microscopic slot law generate every move, reverse, and `WAIT` without consulting $\Xi$, equilibrium, or measured traffic? | The candidate has an independent dynamics generator. |
| 5 | Do exact T1--T5 checks and frozen mutations pass? | One engineered payment-allocation kernel is an equilibrium thermodynamic graph model. |
| 6 | Does the production source-law gate supply one voter-verifiable, actor-independent slot with exact four-way weight under a named cryptographic assumption, and does Valhaim derive the unique outcome? | The tested stochastic clock and transition law are protocol-owned under that declared assumption. |
| 7 | Do non-`WAIT` moves traverse the real `PaymentLedger`, actor intent where applicable, proposer signature, quorum votes, finality, persistence, and replay path? | Valhaim executes the candidate rather than merely analyzing it beside the ledger. |
| 8 | Does the ledger projection reproduce the independently frozen generator exactly, with omitted history proved irrelevant? | With step 6, Valhaim contains the same stochastic kernel verified at step 5; without step 6, it contains only the exact counterfactual transition-function realization. |
| 9 | Does equal-$\alpha$ contact preserve the declared clocks, reproduce the independently prepared product law, give zero equilibrium current, and repeat with a fresh third body? | $\alpha$ is operational temperature for the declared exact body/contact model; protocol ownership additionally requires the contact source-law gate. |

A step-5 pass alone does **not** establish PaymentLedger ownership,
operational temperature, production $S_\tau$, monetary calibration of energy,
or thermodynamic closure of Valhaim's advanced features.

## 2. Primary isolated fixture

### 2.1 Payment body

The body has two dedicated payment accounts joined by one declared kernel
edge:

```text
L  <->  H
```

Their frozen integer heights are

$$
h_L=0,
\qquad
h_H=1.
$$

The body contains two elementary payment units, labelled $a$ and $b$ in the
complete microscopic state. A placement is

$$
\sigma:\{a,b\}\longrightarrow\{L,H\}.
$$

Write $x$ for the number of units at $H$. The visible balance mesostates are

$$
Z=\{x=0,1,2\}.
$$

The labels are not decorative. In the exact preflight they are literal
microscopic coordinates. Step 7 must either persist them in a protocol-owned
thermal state or expose an executable auxiliary Crystal readout and prove the
balance projection strongly lumpable. Imagined labels cannot support a
literal native-Xypher claim.

The integration boundary is closed: exactly these two accounts and two units,
a fixed edge and fixed heights, no outside deposits or withdrawals, no
minting, burning, fees, decay, topology changes, Ruby, Emerald, forecasts,
annotations, profiles, routing, or other actions. Only the declared thermal
lane may mutate the body. An existing payment path passes this boundary only
if its receipts and downstream stores are proved inert under this restricted
alphabet or are included in the complete causal state.

### 2.2 Energy and reservoir

Let $\varepsilon>0$ be one abstract energy quantum. It is a unit convention,
not yet one micro-THAIM. The body energy is

$$
U_X(\sigma)=\varepsilon x.
$$

The finite binary reservoir has alphabet $\{0,1\}$ and depth $D=2$. Its
microscopic state is a word $w$ whose length is

$$
q=2-x.
$$

The isolated body-plus-reservoir energy is therefore fixed:

$$
U_X+E_R=\varepsilon x+\varepsilon|w|=2\varepsilon.
$$

The reservoir multiplicity and entropy are

$$
g_R(E_R)=2^{E_R/\varepsilon},
\qquad
S_R(E_R)=\frac{E_R}{\varepsilon}\ln2.
$$

The reservoir relation independently defines

$$
\alpha=\frac{\varepsilon}{\ln2}.
$$

This relation is frozen before any rate is generated. Production
`current_alpha()` is prohibited as a substitute even if it happens to return
the same number.

### 2.3 Complete states and multiplicity

The complete isolated state space is

$$
\Omega=\{(\sigma,w): |w|=2-x(\sigma)\}.
$$

The nine states, grouped by visible balance, are:

```text
x=0:  LL|00  LL|01  LL|10  LL|11
x=1:  LH|0   LH|1   HL|0   HL|1
x=2:  HH|empty
```

Here `LH` means $a$ is at $L$ and $b$ is at $H$; `HL` means the reverse.
The body multiplicity is

$$
g_X(x)=\binom2x=(1,2,1),
\qquad
S_X(x)=\ln g_X(x).
$$

The complete fiber multiplicities are

$$
G_x=g_X(x)2^{2-x}=(4,4,1),
\qquad
|\Omega|=9.
$$

The verifier must obtain these counts by enumerating placements and reservoir
words. It may not insert $(4,4,1)$ as the target distribution.

### 2.4 Crystal readout

At fixed visible balance $x$ and fixed reservoir word $w$, a marked `REFRESH`
readout resolves one of the $g_X(x)$ compatible labelled unit placements with
probability $1/g_X(x)$. It reports that placement and leaves every persistent
coordinate, including $\sigma$, unchanged. Its endpoint entropy is therefore

$$
S_X(x)=\ln g_X(x).
$$

This auxiliary readout is not a transition in $P$ or $Q$, is not read by the
move law, and does not replace or renormalize the four move slots below. In
the exact model it is a representational Crystal channel. A later
protocol-native claim requires a replayable mechanism that implements the
same non-mutating uniform readout without actor selection. Reservoir words do
**not** receive
same-energy `Replace` channels in this fixture. They change only through the
frozen pop/append moves. This exclusion distinguishes the nine-state payment
kernel from earlier causal-stack fixtures that include within-level reservoir
replacement.

## 3. Frozen microscopic opportunity law

### 3.1 One clock, four permanent slots

The executable clock has the four permanent slots

$$
(\ell,b),
\qquad
\ell\in\{a,b\},
\quad
b\in\{0,1\}.
$$

Each slot has probability $1/4$ per logical opportunity. Equivalently,
opportunities arrive at total rate $4\kappa$ and every active microscopic
edge has continuous-time hazard $\kappa$. The activity $\kappa$ fixes the
time unit; it is not temperature.

For selected slot $(\ell,b)$:

1. If unit $\ell$ is at $L$, the reservoir is nonempty, and $b$ equals the
   last symbol of $w$, move $\ell:L\to H$ and remove that last symbol.
2. If unit $\ell$ is at $H$, move $\ell:H\to L$ and append $b$ to $w$.
3. Otherwise record `WAIT` and leave the complete state unchanged.

The same slot is the literal reverse at the destination. An uphill move pops
the bit that its reverse downhill move appends. Illegal or dormant slots are
never removed or renormalized.

Independent per-unit clock superposition is an explicit construction axiom.
The experiment does not claim that existing payment demand spontaneously
supplies it.

The authoritative raw outcome table is below. `-` means `WAIT`; the rightmost
reservoir symbol is popped first. The runner must generate this table from the
three rules above; it may use the table only as an acceptance oracle.

| Pre-state | `a:0` | `a:1` | `b:0` | `b:1` |
|---|---|---|---|---|
| `LL|00` | `HL|0` | `-` | `LH|0` | `-` |
| `LL|01` | `-` | `HL|0` | `-` | `LH|0` |
| `LL|10` | `HL|1` | `-` | `LH|1` | `-` |
| `LL|11` | `-` | `HL|1` | `-` | `LH|1` |
| `LH|0` | `HH|empty` | `-` | `LL|00` | `LL|01` |
| `LH|1` | `-` | `HH|empty` | `LL|10` | `LL|11` |
| `HL|0` | `LL|00` | `LL|01` | `HH|empty` | `-` |
| `HL|1` | `LL|10` | `LL|11` | `-` | `HH|empty` |
| `HH|empty` | `LH|0` | `LH|1` | `HL|0` | `HL|1` |

### 3.2 Frozen primary predictions

After lumping by $x$, the exact discrete kernel must be

$$
P=
\begin{pmatrix}
1/2&1/2&0\\
1/2&1/4&1/4\\
0&1&0
\end{pmatrix}.
$$

The continuous generator in units of $\kappa$ must be

$$
\frac Q\kappa=
\begin{pmatrix}
-2&2&0\\
2&-3&1\\
0&4&-4
\end{pmatrix}.
$$

The independently predicted equilibrium is

$$
\pi=\frac{(4,4,1)}9.
$$

The exact eigenvalues are

$$
\operatorname{eig}(Q/\kappa)=\{0,-3,-6\},
\qquad
\operatorname{eig}(P)=\{1,1/4,-1/2\}.
$$

These are frozen predictions, not runner inputs.

### 3.3 Independent local-detailed-balance prediction

For an uphill move $x\to x+1$,

$$
\frac{k_{x\to x+1}}{k_{x+1\to x}}
=\frac{g_X(x+1)}{g_X(x)}2^{-1}
=\exp\left(\Delta S_X-\frac{\Delta U_X}{\alpha}\right).
$$

The first uphill ratio is $1$ and its affinity is $\Xi=0$. The second uphill
ratio is $1/4$ and its affinity is $\Xi=-2\varepsilon$. Negative-affinity
moves remain possible.

The verifier must check local detailed balance by exact integer
cross-multiplication,

$$
G_x k_{x\to y}=G_y k_{y\to x},
$$

and independently factor $G_y/G_x$ into body multiplicity and reservoir
energy terms. Approximate logarithms cannot decide this gate.

### 3.4 Receipt boundary

The kernel-local one-sided diagnostic is

$$
R_+(a)=\alpha\max(0,\Delta S_X(a)).
$$

It is nonpersistent and is not minted. It may be called `THAIM_+` only after
$\varepsilon$ is calibrated to a THAIM-denominated energy boundary and the
observable contract admits this $S_X$. Neither condition is assumed here.

## 4. Exact preflight gates: steps 1--5

The microscopic enumerator must use only labels, words, the permanent slot
alphabet, and the three raw rules in section 3. It may not call the
thermodynamic predictor, $\Xi$, $\pi$, or the expected macro matrix.

| Gate | Exact requirement |
|---|---|
| K01 freeze identity | The protocol/manifest commit predates runner creation, and a later runner/mutation commit predates every execution output. The executed identities match both seals. |
| K02 independent coordinates | Exactly nine complete states and fibers $(4,4,1)$ are enumerated; $g_X$, $g_R$, $U_X$, and the relation $\alpha\ln2=\varepsilon$ are derived without importing rate code or production `current_alpha()`. |
| K03 row closure | Every complete state exposes exactly four slots; probabilities sum to one; generator rows sum to zero. |
| K04 reverse involution | Every active move has the same-slot reverse, with source and destination exchanged. |
| K05 path accounting | Every move has $\Delta U_X+\Delta E_R=0$ and external work zero; every `WAIT` has zero changes. |
| K06 strong lumpability | Every complete state in one $x$ fiber has identical total probability into every destination fiber. |
| K07 thermodynamic prediction | Exact state-count ratios reproduce every forward/reverse rate ratio. |
| K08 equilibrium | $\pi$ is positive, normalized, stationary, and satisfies channel detailed balance exactly. |
| K09 relaxation | The complete micrograph and macrograph are connected; the marked discrete kernel is aperiodic; the stationary law is unique. |
| K10 independent generator | The enumerated lumped $P$ and $Q$ equal the frozen tables without using them to construct channels. This final conformance check runs after the diagnostic physical gates. |
| X01 Xypher representation | The separate mapping below is complete and every auxiliary component is labelled as such. This is not a T1--T5 gate. |

The minimal mapping is:

| Xypher component | Frozen realization |
|---|---|
| Substrate | The complete graph on $\Omega$ induced by active same-slot moves; $Z$ is its strongly lumped balance projection. |
| Crystal | The non-mutating $g_X(x)$ placement readout; auxiliary until a protocol owns its uniform source and replay. |
| Thermodynamic Harness | The independently constructed $(g_X,g_R,U_X,E_R,\alpha)$ relation, exact path accounting, and nonpersistent $R_+$. |
| Base Praxion | Receive a slot, inspect the complete pre-state, and apply exactly one of the three raw rules. It has one memory state and does not learn. |
| Action | One reciprocal unit transfer or an explicit `WAIT`; the same slot supplies every reverse. |
| Ruby | Empty. No forecasting is used by the kernel. |

This representation test prevents architectural vocabulary from silently
substituting for the thermodynamic gates. A literal protocol-native Crystal
must expose a replayable uniform readout over the $g_X(x)$ labelled
placements. An auxiliary `REFRESH` is allowed only when named as such.

## 5. Preregistered structural variants and metamorphic fixtures

The primary result must survive variants whose acceptance values are fixed
here before execution. They are confirmatory algebraic generalizations, not a
statistically hidden dataset.

### 5.1 Ternary reservoir

Keep two accounts, two labelled units, and depth two. Replace the reservoir
alphabet with $r=3$. Use six permanent slots $(\ell,b)$.

Frozen consequences:

$$
|\Omega|=9+6+1=16,
\qquad
G=(9,6,1),
\qquad
\alpha=\frac{\varepsilon}{\ln3}.
$$

Its independently frozen macro predictions are

$$
P_3=
\begin{pmatrix}
2/3&1/3&0\\
1/2&1/3&1/6\\
0&1&0
\end{pmatrix},
\qquad
\frac{Q_3}{\kappa}=
\begin{pmatrix}
-2&2&0\\
3&-4&1\\
0&6&-6
\end{pmatrix},
\qquad
\pi_3=\frac{(9,6,1)}{16}.
$$

### 5.2 Three-account path

Use the fixed path $0\leftrightarrow1\leftrightarrow2$, heights $(0,1,2)$,
two labelled units, $r=2$, and $D=4$. The permanent slots are

$$
(\ell,e,b),
$$

where $\ell$ is a unit label, $e$ is one of the two undirected path edges,
and $b$ is a reservoir symbol. A slot acts only when $\ell$ is at an endpoint
of $e$; it applies the same pop/append rule in the edge's upward/downward
orientation. All other cases are `WAIT`.

The frozen complete-state count is

$$
|\Omega|
=2^4\left(1+2^{-1}+2^{-2}\right)^2
=49.
$$

This fixture has eight permanent slots per complete state. Its exact matrix
must be derived by enumeration rather than added to this protocol after the
primary result is seen. Its macro projection is the occupation vector
$(n_0,n_1,n_2)$. Its complete-state invariant is

$$
q=4-h(\sigma(a))-h(\sigma(b)),
\qquad
U_X+E_R=4\varepsilon.
$$

It passes only if enumeration independently produces 49 states, every row is
closed, the full micrograph is connected, the macro projection is strongly
lumpable, the stationary fibers equal the independently enumerated fiber
counts, and every rate ratio factorizes into body multiplicity and reservoir
energy terms.

### 5.3 Metamorphic invariances

The following transformations must leave the covariantly relabelled law
unchanged:

- exchange unit labels $a$ and $b$;
- exchange reservoir symbols;
- exchange node identifiers while exchanging their frozen heights;
- scale $\varepsilon$ and $\alpha$ by the same positive factor; and
- scale $\kappa$ and, separately, $\kappa_C$ by positive common factors within
  their respective clocks, changing only time units.

The unit-label and reservoir-symbol permutations are checked at the raw
state-and-slot outcome level, not only after macro aggregation. All
logarithmic identities are accepted through exact integer or rational
cross-products. The symbolic pair $(\varepsilon,r)$ represents $\alpha$;
floating-point equality never decides a gate.

## 6. Frozen broken controls

Each mutation must fail first at the named gate. If it passes, the verifier is
not discriminating the intended claim.

| ID | Frozen operation | Expected first failure |
|---|---|---|
| M01 `MISSING_REVERSE` | At `LL\|00`, change slot `a:0` from `HL\|0` to `WAIT`, transferring its mass to the self outcome; leave the reverse `HL\|0 --a:0--> LL\|00` intact. | K04 reverse involution / T3 support |
| M02 `DIRECTED_HAZARD` | Double only the continuous hazard of `LL\|00 --a:0--> HL\|0` to $2\kappa$ and close that generator diagonal; leave its reverse at $\kappa$. | K07 local detailed balance / T3 |
| M03 `FALSE_BODY_MULTIPLICITY` | Declare $g_X(1)=3$ while leaving the labelled lift unchanged. | K02 multiplicity provenance / T2 |
| M04 `ENDOGENOUS_ALPHA` | Substitute production `current_alpha()` for the reservoir construction, even if it returns the same symbolic value. | K02 independence provenance / T2 |
| M05 `HIDDEN_ROW_SPLIT` | Evaluate `LH\|0` under two omitted policy histories mapped to the same declared state; change only history 1's `a:0` outcome to `WAIT`. | K06 strong lumpability / T1 |
| M06 `CORRUPT_RESERVOIR_DELTA` | Keep `LL\|00 --a:0--> HL\|0` but record $\Delta E_R=0$ instead of $-\varepsilon$. | K05 path accounting / T4 |
| M07 `DISCONNECTED_TOP_STATE` | Convert every microscopic move in both directions between $x=1$ and $x=2$ to `WAIT`, retaining four slots and closed diagonals. | K09 frozen irreducibility and relaxation target / T5 |
| M08 `POSITIVE_AFFINITY_ONLY` | Convert every move with $\Xi\leq0$ to `WAIT` while leaving its positive-affinity reverse active. | K04 reciprocal finite-temperature dynamics |
| M09 `ACTIVE_ONLY_CONTACT` | At contact, remove dormant slots and renormalize over only active label pairs. | C02 clock retention / T6 |
| M10 `WRONG_WORD_LENGTH` | At `LL\|00`, let slot `a:0` raise unit $a$ without popping the reservoir, producing invalid `HL\|00`. | K03 state/row closure / T1 |

## 7. PaymentLedger integration decision

### 7.1 Existing-protocol arms must run first

The experiment distinguishes two current payment paths and never submits both
for one thermal move:

1. The legacy compatibility probe submits `NodeAction::Transfer` for one
   `MicroTau` unit between the dedicated accounts. `SubmittedAction` itself
   has no consensus action signature, so API authentication may not be
   reported as canonical transaction authorization.
2. Where the CAL-OPPORTUNITY implementation is under test, its native arm is
   `OpportunityDecision::Transfer` $\to$ `OpportunitySettled` $\to$
   `PaymentLedger::apply_transfer` plus the ordinary transfer receipt. The
   actor signs the opportunity intent; the proposer signs the block; quorum
   voters sign votes. An additional `NodeAction::Transfer` would duplicate the
   payment and invalidates the trial.

Both arms must test multi-height block production, the signatures actually
present on their path, votes, finality, snapshots, persistence, restart,
replay, receipts, and accounting. They must compare two different full ledger
histories with the same proposed thermal state. If the next thermal rows
differ, that projection is not a complete causal state. Matching rows are a
regression result, not a proof over all omitted history; the pass also requires
a structural audit of every admitted background state and transition
dependency.

Opportunity trials compare `opportunity_consensus_state_hash`, not a legacy
ledger-only hash. Protocol-v3 finality must traverse
`apply_finalized_certificate`; direct `apply_finalized_block` rejection is an
expected safety check, not an experiment failure.

The existing opportunity path binds a transfer to its scheduled actor and
account owner. The trial must therefore report whether reciprocal moves work
across rotating proposers. A one-validator fixture is a scoped integration
result, not evidence that the production authorization model supports the
kernel. `Rejected`, unauthorized, or otherwise unexecuted opportunities are
failures; they never count as physical `WAIT` outcomes.

Omitting unit labels or reservoir-word contents does not automatically fail
macro T1. The balance state may still be an exact thermodynamic projection if
an explicit microscopic lift exists, strong lumpability is proved, and every
omitted history is irrelevant to the next row. Such a pass proves the
three-state macro realization. It does not prove that Valhaim natively owns
the literal nine-state realization or its Crystal readout.

An empty block cannot be relabelled as `WAIT` unless the opportunity identity,
slot source, unique expected outcome, and empty result are canonical and
replayable.

### 7.2 Explicit extension arm

Only after both applicable existing-protocol arms reach their frozen verdicts
may the experiment add an explicitly named Valhaim thermal-kernel extension.
The smallest admissible extension must commit, in consensus-hashed replayable
state or events:

- kernel identity and frozen specification identity;
- either the literal labelled unit locations and reservoir word, or the
  strongly lumped macrostate plus its frozen lift and generator identity;
- the complete permanent opportunity identity, including `WAIT`;
- slot-source state, identity, proof, and exact slot derivation;
- pre-state, outcome, reverse slot, and post-state;
- $\Delta U_X$, $\Delta E_R$, heat, and external work;
- the ordinary payment transfer caused by every isolated non-`WAIT`; and
- rejection identity and deterministic retry semantics.

The proposal/finality/persistence layer, rather than the block it finalizes,
must bind proposer signature, signed quorum votes or certificate, durable
commit, restart, and replay evidence. A quorum certificate is not required to
exist inside the block that it finalizes.

The extension may wrap ordinary transfers, but it may not silently replace
them with a test-only balance mutation. It must be called a protocol
extension, not evidence that production Valhaim was already thermal.

The ordinary-arm result may identify which current fields are insufficient,
but it may not be used to tune the extension's physics. Before the first
extension execution, a second commit must freeze the exact extension schema,
state transitions, slot-source contract, expected generator identity, and
extension-specific mutations. If that second freeze does not exist, the
extension arm is invalidated rather than interpreted.

### 7.3 Evidence strata for the slot law

The experiment separates three claims that may not substitute for one
another:

1. **Abstract exact generator.** The preflight assigns mass $1/4$ directly to
   each of the four slots and evaluates all four from every complete state. It
   uses no runtime sampler or observed frequencies.
2. **Canonical execution feasibility.** The first `ThermalStep` extension
   execution arm may use one validator controlling both dedicated accounts,
   exhaustive counterfactual calls, and the deterministic conformance schedule
   below. Current existing-protocol arms do not satisfy this source contract.
   The extension arm can establish transition execution, signatures,
   one-of-one finality, persistence, retry identity, and replay. It cannot
   establish an unbiased deployed stochastic clock, decentralized
   authorization, or bias resistance.
3. **Production slot-source ownership.** Every voter must verify a unique slot
   certificate derived from a protocol-defined pre-height source that the
   scheduled actor cannot select, replace, or grind. A declared distribution,
   opaque `before/draw/after` hashes, a test override, or an actor-selected draw
   is insufficient.

Only stratum 3 can pass step 6. The deterministic schedule in stratum 2 earns
canonical transition and replay feasibility, but it cannot be promoted into a
deployed $1/4$ stochastic kernel by observed frequencies or terminology.

For the scoped one-validator `ThermalStep` extension arm only, freeze

```text
d_h = SHA256(domain || chain_id || kernel_id || genesis_seed || opportunity_ordinal)
slot_h = low_two_bits(d_h)
```

The domain, seed, encoding, and ordinal rule are committed in genesis. Every
voter recomputes them; source state advances only through verified finality;
retry at the same coordinates reproduces identical intent bytes. This is a
deterministic replay schedule. It is not used to derive the abstract $1/4$
measure and does not pass the production stochastic-clock gate.

A live production-clock pass additionally requires all of the following to be
frozen before its first use:

- the source probability space and cryptographic assumption;
- source state fixed before actor and action eligibility;
- domain separation by protocol version, kernel ID, opportunity index, and
  canonical pre-state;
- a direct two-bit mapping to the four primary slots, with no modulo bias;
- no actor selection, post-selection, resampling, or retry into a preferred
  slot; withholding may halt progress but cannot change the fixed message or
  slot, advance the opportunity ordinal, or trigger a redraw;
- a proof checked by every voter and exact replay after restart; and
- an explicit declaration of whether source state is inside the kernel or an
  external work/information boundary.

Validators must derive the slot from the verified source and then recompute
the unique transfer or `WAIT`, energy row, and post-state from canonical
pre-state. A signed actor intent naming a different eligible action is
rejected.

A prior-block or quorum-certificate hash is reported separately as a
consensus-recomputable hash clock. It supports an equal-weight interpretation
only under named random-oracle and no-grinding/no-withholding assumptions. A
full production pass requires a threshold VRF, unique threshold-signature
beacon, or equivalent one-output source over a fixed pre-height message,
including epoch-key, resharing, retry, withholding, and liveness contracts.
Failure to reach the threshold is reported as a liveness failure, never
converted into another slot.

The first one-validator `ThermalStep` extension fixture maps both kernel
accounts to its sole key. A multi-validator extension instead needs protocol
authorization over explicitly opted-in kernel accounts. The proposer supplies
a slot certificate, not a discretionary transfer. An active step calls the
ordinary ledger balance transfer exactly once and binds its receipt; a dormant
step records `WAIT` and performs no transfer; a rejected settlement fails the
gate.

## 8. Counterfactual generator and live-clock comparison

The counterfactual executor bypasses live sampling. For each complete primary
thermal state and each of its four abstract slots, it calls the same pure voter
transition function and compares:

1. the thermal complete-state outcome;
2. the projected body balance;
3. the ordinary transfer or explicit `WAIT` record;
4. the energy-accounting row;
5. the replayed post-state hash; and
6. the probability mass induced by the frozen abstract slot measure for every
   projected destination. `PaymentLedger` is not expected to emit this mass.

The resulting ledger-projected kernel must equal the independently frozen
microscopic kernel by exact rational equality. This proves the transition
function, not a live source distribution. A unique live source produces only
one valid slot from one complete canonical pre-state; the test slot parameter
is not misreported as its certificate.

The live-clock gate separately verifies the source contract in section 7.3,
voter recomputation, the unique derived outcome, multi-height authorization,
finality, and replay. Consensus-path coverage of all four slots may use four
fixtures whose thermal projection is identical but whose genesis source state
and valid proof differ. They are not the same complete canonical state. Sample
frequencies and tolerances are prohibited as substitutes for either exact
transition equality or source-law verification.

## 9. Frozen contact experiment

### 9.1 Equal-$\alpha$ contact

Prepare two isolated primary bodies independently against binary reservoirs
with the same $\alpha=\varepsilon/\ln2$. Detach both reservoirs. Condition on

$$
x_A+x_B=2.
$$

Preparation and conditioning are exact enumeration, not rejection sampling
or optional stopping. The six remaining labelled body microstates are
uniformly weighted. Their
three macro fibers are

$$
(1,4,1),
\qquad
\pi_C=(1,4,1)/6.
$$

Bath detachment ends the isolated body--bath pop/append clock before contact
is opened. Those heat-exchange channels are not claimed to remain active
without their reservoirs. The non-mutating Crystal readout remains unchanged.
Contact uses a distinct four-slot clock whose alphabet and activity are frozen
here. Before contact opens, all four contact slots are dormant `WAIT` outcomes;
opening contact changes only their outcomes and does not renormalize the
Crystal readout or any retained energy-preserving body channel.

Contact has four permanent slots $(\ell_A,\ell_B)$. If the selected label in
$A$ is low and the selected label in $B$ is high, raise $A$ and lower $B$.
If their positions are reversed, perform the reverse paired move. Otherwise
record `WAIT`. Each active microscopic pair has hazard $\kappa_C$.

The frozen macro kernel is

$$
P_C=
\begin{pmatrix}
0&1&0\\
1/4&1/2&1/4\\
0&1&0
\end{pmatrix},
$$

where $P_C$ is the kernel per opportunity of the distinct contact clock. Its
continuous generator is

$$
Q_C/\kappa_C=
\begin{pmatrix}
-4&4&0\\
1&-2&1\\
0&4&-4
\end{pmatrix}.
$$

The expected signed energy current into $A$ at $\pi_C$ is exactly zero. A
fresh third identically prepared body must satisfy the same pairwise law.
Contact is a simultaneous two-body move. Current opportunity and `NodeAction`
paths do not supply one atomic paired-transfer receipt. Native contact
therefore requires an explicitly named `ThermalContactStep` extension with
its own pre-execution schema freeze. Its source follows the same verified
two-bit contract as the isolated four-slot clock under a contact-specific
domain.

### 9.2 Temperature-sensitivity control

Prepare depth-two $A$ with $r_A=2$ and depth-two $B$ with $r_B=4$, using the
same $\varepsilon$. Their isolated body weights before normalization are
$(4,4,1)$ and $(16,8,1)$. Detach and condition exactly on the same combined
body energy. Before contact,

$$
p_{AB}=(1,8,4)/13
$$

over $(x_A,x_B)=(0,2),(1,1),(2,0)$. The frozen initial current is

$$
\frac{J_A}{\kappa_C\varepsilon}=-\frac{12}{13}.
$$

Exchanging the preparations must reverse the sign. This control prevents a
zero-current result produced only by a symmetric contact matrix from being
mistaken for temperature sensitivity.

### 9.3 Contact gates

| Gate | Exact requirement |
|---|---|
| C01 preparation | Each isolated body has the independently predicted canonical law before bath detachment. |
| C02 clock retention | The four contact slots existed as dormant `WAIT`s before opening; their clock rate is unchanged, and Crystal or any retained energy-preserving body channel is unaltered. |
| C03 conserved exchange | Every active contact slot keeps both inventories fixed and conserves $U_A+U_B$. |
| C04 reverse and closure | Every paired move has the same-slot reverse and every row has four slots. |
| C05 equal-temperature law | $(1,4,1)/6$ is stationary and satisfies detailed balance exactly. |
| C06 zero current | The expected equilibrium current into either body is exactly zero. |
| C07 third body | A fresh third identical preparation passes the same pairwise gates. |
| C08 sensitivity | The unequal preparations give the frozen nonzero current and sign reversal. |

## 10. Advanced-feature boundary

Steps 1--9 exclude topology growth, minting and burning, route search, Ruby,
Emerald, profiles, zero-knowledge conditions, adaptive learning, and competing
Praxions. After the kernel verdict, each feature receives a separate
before/after classification:

```text
equilibrium-preserving
driven but thermodynamically accounted
observer or verifier only
new causal state required
boundary-breaking under the current model
```

No feature may inherit the kernel's thermodynamic verdict merely because it
is useful or already implemented.

## 11. Frozen interpretation rules

1. A step-5 pass proves an exact engineered digital payment-allocation
   thermodynamic model, not present production ownership.
2. Steps 6--8 pass only when canonical Valhaim execution preserves the same
   complete kernel and generator.
3. Exact step 9 is required before calling $\alpha$ operational temperature
   for the declared body/contact model. A protocol-owned contact claim also
   requires the step-6 source-law standard under the contact domain.
4. A positive explicit-extension arm proves **constructibility in Valhaim**,
   not spontaneous emergence from the existing economy.
5. Failure of the ordinary-transfer arm followed by success of the extension
   is a substantive architecture result, not a rescued null result.
6. If the production source-law gate remains open, the result must distinguish
   an exact generator and counterfactual transition realization from a
   deployed stochastic clock.
7. Advanced features are outside the minimal positive claim until their own
   return experiments are complete.
8. No result here establishes a new fundamental law of physics or external
   novelty. The tested question is the exact digital construction and its
   realization inside Valhaim.
