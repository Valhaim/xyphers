---
title: "When Does Information Become State in a Xypher?"
description: "The same mark appears beside two controllers: one copies and uses it; the other never reads or changes it. When must that distinction enter a Xypher’s state for its future behavior to remain predictable?"
type: "research-article"
status: "published"
maturity: "formal-framework-draft"
published: "2026-08-18"
revised: "2026-08-18"
website_path: "/research/when-does-information-become-state-in-a-xypher/"
web_status: "live"
---

# When Does Information Become State in a Xypher?

Put a blue mark beside two controllers. The first never looks at it. The second copies it into a register, then turns left or right according to what the register says.

For the first controller, the mark is an annotation. For the second, the distinction belongs in any complete model of the controller's next-action rule. In a Xypher, information becomes part of complete state when the declared dynamics read it, change it, or require it to determine what happens next.

That answer opens three further questions. May a simpler view safely hide the distinction? May private learning become shared state? Does any of this make the state thermodynamic?

The tests are different. Treating them as one ladder is where the trouble begins.

## A mark beside a machine

Suppose the mark changes from blue to gold while both controllers are idle.

Nothing inside the first controller changes. Its next action has the same probabilities as before. The mark may still be a real physical patch of paint, but it sits outside the controller's autonomous rule.

The second controller senses the change and stores it. Two machines that look identical from the outside can now act differently because their registers differ. A state description that records only the casing and the motor position has left out something its own transition rule uses.

Here **dynamics** means the rule assigning the next elementary event and its probability or rate. **Complete state** means an honest inventory from which that rule can be evaluated at the boundary being claimed. The inventory may include an internal register, a clock phase, a source of randomness, or a reservoir coordinate. If the rule consults it, the model cannot quietly leave it in the wings.

This is boundary-relative. An external laboratory may drive a controller with a sequence of marks. The model can call that sequence an external drive, in which case the controller alone is an open system. If the claim is that the controller is autonomous, the relevant input and clock state must move inside the boundary.

Completeness answers one question only: *what must the full model carry?* It does not tell us whether a smaller description can still predict a chosen visible behavior.

## What a simpler view may forget

Imagine a station display that shows only three symbols: **A**, **L**, and **R**. Behind the display, the mechanism has two different phases that both appear as A. From the first phase it always moves to L. From the second it always moves to R.

If all we see is A, there is no single honest answer to “what comes next?” The answer depends on which hidden phase produced the same visible symbol.

A process whose next-step law depends only on its present state, rather than on an unrecorded history, is called a **Markov process**. Grouping several complete states under one visible label is a **projection**. For that projection to remain an exact Markov state, every complete state hidden under one source label must send the same total probability into every possible destination label.

Let **P(x,y)** be the probability of moving from complete state **x** to complete state **y** in one step. Let **C** be one visible source class and **D** one visible destination class. The symbols **x** and **x′** name any two complete states hidden inside **C**. The capital sigma, **Σ**, means “add”; **y ∈ D** means every complete destination **y** that belongs to visible class **D**. The exact discrete-time test is:

$$
\text{for every } x,x'\in C \text{ and every } D:
\qquad
\sum_{y\in D}P(x,y)=\sum_{y\in D}P(x',y).
$$

*Hidden versions of one visible state must agree on the probability of every visible next state.*

Mathematicians often call this **strong lumpability**. Buchholz calls the same outgoing block-sum condition **ordinary lumpability**, reserving “exact lumpability” for a different incoming condition. When the outgoing test holds, the common block sums form one projected transition rule for every possible initial mixture of hidden states. That all-initial-mixtures result is a consequence of the equality above. It is weaker to find one specially prepared mixture that happens to work.

[Kemeny and Snell](https://link.springer.com/book/9780387901923) give the classical finite-chain criterion. We will return to the continuous-time version after using the simpler probability test on a complete example.

## Eight complete states, three views

The entire obstruction fits into eight states that can be checked by hand.

The first coordinate is a phase: **A₀**, **L**, **A₁**, or **R**. The second is a colour: blue or gold. This time colour is carried forward but never consulted by the phase mechanism. Four phases times two colours give eight complete states.

Every state has exactly one next state, reached with probability one. Colour is carried along unchanged:

| Current phase | Next phase | Probability |
|---|---|---:|
| A₀ | L | 1 |
| L | A₁ | 1 |
| A₁ | R | 1 |
| R | A₀ | 1 |

The rows form a deterministic four-cycle. Now hide different amounts of its state and inspect what happens.

| View | Visible classes | Exact repeated prediction? | Decisive test |
|---|---:|---:|---|
| Coarse | A, L, R | No | From A₀, probability to L is 1; from A₁, it is 0 |
| Phase | A₀, L, A₁, R | Yes | Blue and gold in each phase have the same next phase |
| Full | Eight phase-and-colour singletons | Yes | Every complete state has its own row |

The coarse A class fails because it hides two phases with different visible futures. Splitting A into A₀ and A₁ repairs the law. Keeping colour as well is exact, but it adds no predictive power for this phase process.

This exposes the two sides of minimality. Too little state makes the future law ambiguous. More state than the law needs remains possible, but it carries a distinction that does no work for the declared prediction. To find the smallest exact view, split any visible label whose hidden states disagree about their destinations. Repeat until no such disagreement remains. For a fixed finite model and starting partition, the result is the **coarsest lumpable refinement**. [Derisavi, Hermanns, and Sanders](https://doi.org/10.1016/S0020-0190(03)00343-0) give an optimal construction.

When events can occur at any instant, compare total transition **rates** rather than one-step probabilities. Hidden states in one source class must send the same total rate into every *other* destination class. The table also carries one bookkeeping number in its own-state column: the negative of all rates leaving that row, chosen so the row adds to zero. If no transition leaves, that number is zero. [Buchholz](https://doi.org/10.1017/S0021900200107338) develops this finite continuous-time criterion.

One prepared success also does not repair the coarse view. If A begins as an equal mixture of A₀ and A₁, its next visible state is half L and half R. That one next-step distribution looks lawful. After repeated observation, the previous symbol reveals the phase: A reached from R is A₀ and goes next to L; A reached from L is A₁ and goes next to R. The visible history still changes the prediction.

Nor can one scalar entropy certify state equivalence. Suppose there are three named outcomes. Distribution **p** assigns probabilities one-half, one-half, and zero to them. Distribution **q** moves the second half-probability to the third outcome. Let **H** be Shannon entropy in bits:

$$
p=(1/2,1/2,0),
\qquad
q=(1/2,0,1/2),
\qquad
H(p)=H(q)=1\ \text{bit}.
$$

Both distributions have the same spread, yet they put probability on different outcomes. Later actions or transition rates can distinguish them immediately. Equal endpoint future entropy at one horizon therefore does not establish equal endpoint distributions, equal repeated dynamics, or equal states.

## State is not a staircase

The word “state” already names three relationships. They cross rather than line up. The table separates the tests established so far.

| Kind | Test | What the test does not establish |
|---|---|---|
| External annotation or drive | It lies outside the declared autonomous transition law | That the information lacks physical existence |
| Complete-state coordinate | The dynamics read it, change it, or require it to determine the next event | An exact projection, shared authority, or thermodynamic status |
| Exact projected Markov state | In discrete time, every hidden member sends equal probability into every visible block; in continuous time, it sends equal total rate into every other block | That participants can rebuild the label from one shared record |

Instead of asking whether information “is state,” ask the specific question: whose dynamics use it, and which projection is trying to hide it? Readouts, shared memory, and thermodynamic status add further tests of their own.

## Put the tests inside a Xypher

In the [Core Thesis](core-thesis.md#the-xypher), the Graph Substrate carries possible states and changes, the Praxion perceives and acts, and the Thermodynamic Harness supplies the declared physical account. A **Crystal** is the declared readout through which the Praxion sees the substrate; it is an instrument inside that three-part architecture, not a fourth layer.

The complete-state test follows the executable rule. If a Praxion reads a memory bit before choosing an action, the bit belongs in the complete state of that action process. If a clock changes which candidates are available, its relevant phase belongs there too. If a random seed determines a future transition and the model claims autonomous replay, the randomness state or its external source must be named.

The projection test asks a different question. Perhaps the full process contains thousands of coordinates while a three-state summary is enough for one exact future law. Strong lumpability can earn that summary. For a claim about where the process ends after one fixed number of steps, preserving that full endpoint distribution may be enough. It is enough only for that bounded prediction. For repeated Markov dynamics, every one-step destination block must pass the stronger test.

Architecture still does not confer thermodynamics. A Xypher earns that status only when its complete boundary closes the independent requirements for state, energy and entropy, reciprocal transitions, pathwise heat and work accounting, equilibrium, and lawful contact. Complete state is the first obligation, not the last.

## One exact Xypher boundary

A small Xypher with only sixteen complete graph-and-reservoir arrangements makes the distinction exact. The research archive calls it **CAL-XTHERM**.

Its visible substrate has three positions, **0 ↔ 1 ↔ 2**, with model energies **0, 1, and 2**. Each position admits a declared number of internal graph arrangements. A finite reservoir supplies the remaining energy-compatible arrangements.

| Visible position | Internal graph arrangements | Reservoir arrangements | Complete states |
|---:|---:|---:|---:|
| 0 | 1 | 4 | 4 |
| 1 | 4 | 2 | 8 |
| 2 | 4 | 1 | 4 |

Multiplying each row gives **4, 8, and 4** complete states: sixteen in all. The verifier lists every one of them. Nothing is estimated from a sample.

The reservoir count doubles with each available unit of reservoir energy. The expression **ln 2** is the natural-log measure of that doubling. A **nat** is one unit of information measured with the natural logarithm. The state-count relation therefore fixes **α = 1/ln 2** model-energy units per nat: α is the energy price of the multiplicity change in this declared reservoir. It is fixed before the movement rates are evaluated and is not a claim about the host computer's temperature in kelvins.

Movement supplies a second reading. The exact rates for **0→1, 1→0, 1→2, and 2→1** are **8, 4, 4, and 8**. Their ratios match the same declared changes in state-count entropy, the same energy changes, and the same α. An independent enumeration of reciprocal microscopic moves produces those rates again.

At equilibrium, the three visible positions carry probabilities **1/4, 1/2, and 1/4**. The traffic balances in both directions: **(1/4)×8 = (1/2)×4** on the first edge, and **(1/2)×4 = (1/4)×8** on the second. When two copies prepared at the same α are allowed to exchange energy, the expected equilibrium current is exactly zero.

Every complete state hidden under one visible position also sends the same total rate into each neighboring position. The three-position view is therefore an exact Markov projection, even though it hides sixteen complete arrangements.

The construction's readout label participates in its executable dynamics: an internal refresh operation can change that label while preserving energy. The same proof also covers a different system that exposes a temporary report while changing no persistent coordinate and no later transition rate. Such a report can represent a counted internal distinction without becoming persistent state merely because it was observed. The CAL-XTHERM boundary record names this reverse construction the **representation theorem**, or Theorem 2.

The sixteen-state construction has an empty **Ruby**—no directed-flow scoring layer—and its non-adaptive memory has only one condition, so that memory never changes. It proves one exact minimal thermodynamic Xypher and an exact three-position projection. It does not prove an adaptive learning law.

The [exact result](../evidence/information-state/CAL-XTHERM-Result.md) records every state, rate, equilibrium weight, and control. The [standalone verifier](../downloads/xypher-thermodynamics-proof-v1.zip) can be inspected and run without external Rust dependencies; its [two-minute guide](../downloads/README-FIRST-xypher-thermodynamics-proof-v1.txt) gives the commands.

## When memory changes the road

Replace the inert colour in the toy with a notebook. Two actors have the same public balance and position, but one notebook says “take the left road” while the other says “take the right.” If the actor reads that page, the notebook is part of complete state. Hiding it behind the public balance fails the block-sum test.

A later exact-control study, **CAL-ADAPT**, tested that boundary. Behavior-changing hidden memory produced two complete states with the same visible balance and different next-balance laws; the visible projection failed. An inert hidden label passed.

The study then paired every memory-writing move with a memory-erasing reverse. Its primary two-gap information reservoir contained seven complete states across three energy levels. A separately generated thirteen-state version passed the same exact reversibility, equilibrium, and contact checks. The second construction matters because the result is not confined to one hand-written seven-state table.

An append-only history is different. When a counter or receipt can only increase, the transition that advances it has no microscopic reverse inside the declared state space. Reversing a visible balance does not erase the retained receipt.

These controls establish that hidden behavior-changing memory breaks the chosen visible Markov state, while a finite reversible information reservoir can be engineered. They do not derive the live Xypher's own learning mechanism. The symbols its memory may hold, the reversible operations that erase them, and the timing of those operations have not yet been obtained from the Xypher's own Crystal and Praxion. Adaptive-Xypher thermodynamics remains open.

Private and shared memory add an authority question that lumpability does not answer. Before two participants act on the same learned memory, both must be able to reconstruct it from the same shared record, know who may change it, and know what happens when later evidence contradicts it. The Xypher architecture treats those as candidate design requirements. They are not consequences of the Markov theorem or laws of nature.

The [CAL-ADAPT result](../evidence/information-state/CAL-ADAPT-Result.md) and [reproduction guide](../evidence/information-state/RUN.txt) expose the exact controls and their open boundary. The [checksummed evidence packet](../downloads/xypher-information-state-evidence-v1.zip) includes the source and tests ([SHA-256](../downloads/xypher-information-state-evidence-v1.zip.sha256)).

## The physical bill arrives at reset

A chalk mark can be read many times without being wiped away. To reuse the board as a guaranteed blank surface, however, every possible old message must be driven into the same “blank” condition. Several distinguishable logical states have merged into one.

This is **logical irreversibility**. [Landauer's analysis](https://doi.org/10.1147/rd.53.0183) tied that many-to-one operation to entropy generation in a physical implementation. For the familiar symmetric case, the bit begins at thermal equilibrium with its two alternatives equally represented and is reset to one standard value. Let **Q<sub>reset</sub>** be the heat delivered to the environment in joules, **k<sub>B</sub>** Boltzmann's constant in joules per kelvin, and **T** the environment's absolute temperature in kelvins. The lower bound is:

$$
Q_{\mathrm{reset}} \ge k_B T\ln 2.
$$

The right-hand side is the ideal thermal lower bound for that equally represented bit. A known or biased input can have a smaller entropy to remove and need not dissipate that full amount. The equation is not a universal price attached to the existence of every abstract bit.

[Bennett](https://doi.org/10.1147/rd.176.0525) showed that general computation can be arranged as logically reversible steps by retaining enough history and later **uncomputing** it—running the retained reversible steps backward. A reversible logical design does not make a real device free of friction, errors, or control costs. It removes the claim that every useful computational step must pay an erasure cost merely because it computes.

For a thermodynamic Xypher claim, the practical rule is narrower and firmer. If memory creation, change, or reset affects future behavior, it belongs inside the complete process being accounted. Its energy change, work source, heat destination, and the reservoir that receives the displaced heat and entropy cannot remain implicit.

## An inspection card for state claims

The word “state” is useful once the claim names the test it needs. This short card keeps the obligations separate.

| Claim being made | Test it must pass |
|---|---|
| Exact repeated Markov dynamics | Equal destination-block probabilities for every hidden source state, or equal total transition rates into every other visible block in continuous time |
| One fixed-horizon endpoint claim | Preserve the full endpoint distribution at that horizon |
| Equal scalar future entropy | No state conclusion follows without the distribution and dynamics required by the claim |
| Shared learned state in the current Xypher architecture | Everyone relying on it can rebuild it from the same record, knows who may change it, and knows how later contradiction can reverse or demote it |
| Thermodynamic state | Include the complete boundary, energy and entropy coordinates, reciprocal channels, heat and work account, equilibrium, and contact test |

Applied to a new distinction, the order of questions is straightforward.

1. Do the declared dynamics read it, change it, or require it? If yes, put it in complete state.
2. If a discrete-time projection hides it, do all hidden versions send the same probability into every visible block? In continuous time, do they send the same total rate into every other block?
3. If not, what is the coarsest refinement that restores the exact future law?
4. Is the claim about repeated dynamics, one endpoint distribution, or merely one scalar summary?
5. If the information is to become shared learned state, who can reconstruct it and what contradiction can reverse or demote it?
6. If the claim is thermodynamic, where are the energy, work, heat, records, reservoir, and contact tests?

Information enters a Xypher's complete state when its declared dynamics use or alter the distinction. A projection may forget it only when every hidden version preserves the same visible future law. Shared authority and thermodynamic status remain separate things to earn.

## Research trail

- John G. Kemeny and J. Laurie Snell, [*Finite Markov Chains*](https://link.springer.com/book/9780387901923), supplies the classical finite-state lumpability criterion.
- Peter Buchholz, [“Exact and Ordinary Lumpability in Finite Markov Chains”](https://doi.org/10.1017/S0021900200107338), calls the outgoing block-sum condition used here ordinary lumpability and distinguishes it from a separate incoming, transpose-style exact-lumpability condition.
- Salem Derisavi, Holger Hermanns, and William H. Sanders, [“Optimal State-Space Lumping in Markov Chains”](https://doi.org/10.1016/S0020-0190(03)00343-0), gives a finite optimal partition-refinement construction.
- Cosma Shalizi and James Crutchfield, [“Computational Mechanics: Pattern and Prediction, Structure and Simplicity”](https://doi.org/10.1023/A:1010388907793), develops minimal predictive causal states. Its objective is adjacent to, but not identical with, authoritative Xypher replay.
- Rolf Landauer, [“Irreversibility and Heat Generation in the Computing Process”](https://doi.org/10.1147/rd.53.0183), and Charles Bennett, [“Logical Reversibility of Computation”](https://doi.org/10.1147/rd.176.0525), establish the logical-irreversibility and reversible-computation boundary.
- Xyphers research, [the CAL-XTHERM boundary](../evidence/information-state/CAL-XTHERM-Boundary.md) and [exact result](../evidence/information-state/CAL-XTHERM-Result.md), contains the complete 16-state construction and its operational thermodynamic boundary.
- Xyphers research, [the CAL-ADAPT boundary](../evidence/information-state/CAL-ADAPT-Boundary.md) and [exact bounded result](../evidence/information-state/CAL-ADAPT-Result.md), contains the hidden-memory controls and engineered reversible information reservoir.

[Return to the research index](../README.md).
