---
title: "A Purely Digital Thermodynamic System"
type: "research-article"
status: "published"
published: "2026-09-06"
revised: "2026-10-10"
website_path: "/research/proving-true-thermodynamic-graph-systems/"
web_status: "live"
source_revision: "f859b38e0"
---

# A Purely Digital Thermodynamic System


Digital systems are not normally thermodynamic. A program can model heat, report a temperature, or warm the chips it runs on, but nothing inside its own world has a temperature that the world itself fixes. When software shows a temperature, it is describing something outside itself, or reading back a number someone set.

This article presents a digital system that is thermodynamic in its own right, and the proof.

It is a world of sixteen arrangements, small enough to hold in your mind and to check completely. Its temperature is counted from its structure before anything moves. Its motion, counted separately, returns the same number. It keeps an exact energy account, settles where physics says it must, and passes the test of contact with another body at the same temperature.

There is no random seed, no sample, and no fitted curve. Every arrangement and every move is listed, and every check is an exact identity between whole numbers and fractions. You can run it yourself in seconds.

It is also a Xypher, with the three parts introduced in the [Core Thesis](core-thesis.md): a **Graph Substrate** describing what can happen, a **Thermodynamic Harness** accounting for change, and a **Praxion** that makes the moves.

## What it takes to earn a temperature

Calling a number “temperature” is easy. Earning the name is not. Here, a digital system counts as thermodynamic only if it passes six tests, each stated so that a computer can check it exactly:

1. **Nothing hidden.** The state includes everything the dynamics reads or changes.
2. **Declared in advance.** Energy, entropy, and temperature come from the system's own structure, fixed before anyone looks at how it moves.
3. **Every move reversible, and fairly so.** Every move has a reverse, and the imbalance between them matches the entropy the move creates.
4. **Every packet accounted for.** Each move says where its energy came from, where it went, and what work, if any, was supplied from outside.
5. **Settles where predicted.** Left alone, the system relaxes into the balance that its energy and entropy predict.
6. **Contact.** Two bodies at the same temperature, allowed to trade energy, have no net energy flow between them.

These are the defining features of a system in equilibrium with a heat bath, written for a finite graph. The [boundary](../code/xypher-thermodynamics-proof/evidence/BOUNDARY.md) states them precisely as **T1–T6**. They mention nothing about Xyphers; any finite graph system that passes all six is a thermodynamic graph system.

The tests, the expected results, and four deliberately broken constructions were all fixed and independently reviewed before the verifier ran. The [result record](../code/xypher-thermodynamics-proof/evidence/RESULT.md) documents that chronology.

## The world

Imagine a body that can occupy three positions in a line: **0 ↔ 1 ↔ 2**. Moving one position to the right transfers one energy packet from a reservoir into the body. Moving left returns it. Body and reservoir together always hold two packets.

The energy packet is the digital model's own unit. The graph represents where that energy is held and which transfers can occur, and the temperature we will measure belongs to this computational world, not to the hardware running it.

Each position contains several internal arrangements. Picture a building whose floors mark energy and whose rooms are the different ways to hold it. The body has one room at position 0 and four at positions 1 and 2. The reservoir has four rooms when it holds both packets, two when it holds one, and one when it is empty.

| Position | Body × reservoir | Together |
|---|---:|---:|
| 0 | 1 × 4 | 4 |
| 1 | 4 × 2 | 8 |
| 2 | 4 × 1 | 4 |

A complete arrangement specifies both the body and its reservoir: **4 + 8 + 4 = 16**. Draw each complete arrangement as a point and each allowed move as a line, and the world is a graph.

The moves are as plain as possible. Every arrangement at position 0 connects to every arrangement at position 1, and every arrangement at position 1 to every arrangement at position 2. Each connection works in both directions at the same rate, one per unit of time. Nothing favours any move over another.

There is one more kind of move. Within a position, the body can reshuffle among its own arrangements, choosing uniformly. A reshuffle moves no energy and leaves the position, and so the traffic between positions, unchanged. It becomes important when we look for the Xypher inside this world.

## The first thermometer counts rooms

This reading uses test 2: temperature declared from the structure alone.

The reservoir has one arrangement when empty, two with one packet, and four with two. Each packet doubles its number of arrangements.

Entropy measures the spread of possibilities. For **M** equally likely arrangements it is **ln M**, where **ln** is the natural logarithm, and it is measured in **nats**. Doubling the arrangements adds **ln 2** nats.

Temperature compares the energy added with the entropy gained. Let **λ** be the size of one energy packet, and write **α** for temperature in energy per nat.

$$
\alpha=\frac{\lambda}{\ln 2}
$$

Read it as: *One packet of energy buys one doubling of reservoir arrangements.*

Taking the packet as one energy unit gives **α = 1 / ln 2**. Its value follows from the reservoir alone, before any transition rate is examined. In laboratory physics, the same quantity is written **k<sub>B</sub>T**, with Boltzmann's constant turning kelvin into joules. Here energy is counted in the model's own packets, so α needs no kelvin value.

## The second thermometer counts moves

Now set the reservoir's room count aside and watch the world move. This is test 3. The second reading uses only the body's rooms, its energies, and the counted rates.

Start in any complete arrangement at position 0. There are eight arrangements at position 1, each one move away, so the total rate from 0 to 1 is eight. From any arrangement at position 1, four arrangements lead back to position 0. The same counting gives every rate:

| Move | Forward rate | Reverse rate | Forward / reverse |
|---|---:|---:|---:|
| 0 → 1 | 8 | 4 | 2 |
| 1 → 2 | 4 | 8 | 1/2 |

These are rates per unit of time: how quickly transitions occur, not probabilities that must sum to one.

Physics relates a move and its reverse. When a move changes the body's entropy by **ΔS** and its stored energy by **ΔU**, energy drawn from a reservoir at temperature **α** changes the reservoir's entropy by **−ΔU / α**. The ratio between a move's rate and its reverse rate must match the total.

$$
\ln\frac{k_{\rm forward}}{k_{\rm reverse}}=\Delta S-\frac{\Delta U}{\alpha}
$$

Read it as: *The imbalance between a move and its reverse reflects the total entropy change of body and reservoir.*

This relation is called **local detailed balance**. It lets us read α from the motion.

For **0 → 1**, the body gains **ln 4** nats and one energy unit, and the measured ratio is two: **ln 2 = ln 4 − 1/α**, so **α = 1 / ln 2**.

For **1 → 2**, the body keeps its four arrangements, so its entropy does not change. It takes in another energy unit, and the measured ratio is one-half: **ln ½ = 0 − 1/α**, so again **α = 1 / ln 2**.

**The two thermometers agree exactly.** The rooms predicted the temperature. The moves, counted on their own, recover it. A temperature that was merely declared would have no reason to show up in the motion. This one does, on every edge.

## Nothing hidden, nothing lost

Tests 1 and 4 guard against two quiet ways of cheating.

The first is hiding something in the state. Here the state is the full list of sixteen arrangements, and the moves read nothing else. This article, like the verifier, often speaks only of the body's three positions. That shortcut is allowed only if no detail it hides can change what happens next, and the verifier checks it: every arrangement at the same position has the same total rate into each neighbouring position. Mathematicians call this **strong lumpability**.

The second is losing track of energy. Every step between positions transfers exactly one packet between body and reservoir; a reshuffle transfers none. Their total stays two, and this undriven world receives no work from outside. Every packet is somewhere on every move.

## Settling and contact

Test 5 asks where the world settles. Its energy and entropy predict the answer before any motion is simulated: each position is weighted by its number of complete arrangements, so positions 0, 1, and 2 should be occupied **1/4, 1/2, 1/4** of the time.

Under the counted rates, traffic balances on both edges at exactly those weights. Between positions 0 and 1, for example, **(1/4) × 8 = (1/2) × 4**. Moves continue at equilibrium, but each one is matched by its reverse.

Test 6 is the one that makes α a temperature in the everyday sense. Prepare two bodies, each against its own reservoir at the same α. Detach the reservoirs and let the two bodies trade packets with each other while holding two in total. They can share them as **(0, 2), (1, 1), or (2, 0)**.

Because both reservoirs shared the same α, each case is weighted by its body arrangements times an energy factor **e<sup>−U/α</sup>** for each body. The pair always holds two packets in total, so those energy factors are identical for all three cases and cancel. What remains is the arrangement count: **4, 16, 4**, or **1/6, 4/6, 1/6**. That is already the balance of the joined pair, so on average no energy flows into either body. Had the two reservoirs been at different temperatures, the factors would not cancel, the prepared weights would not be the joint balance, and energy would flow. Nothing flows between things at the same temperature.

The reservoir thermometer, the movement thermometer, and the contact test describe one temperature.

## The same world is a Xypher

The thermodynamic tests never mention Xyphers. The same world nevertheless has all three Xypher parts, and they reproduce its physics exactly.

**Future possibility.** The reshuffle is the Xypher's view of the future. One step ahead, the spread of its possible outcomes is the logarithm of the body's room count: **0, ln 4, ln 4**. This is **S<sub>τ</sub>**, the future-entropy measure of the Core Thesis, with **τ = 1**. Because the reshuffle is one of this world's own moves, the future count and the room count are the same count by construction. Showing the same for a many-step walk on an ordinary graph is the open frontier.

**Consequence.** The Thermodynamic Harness computes the full signed change **α · ΔS<sub>τ</sub>** first. The expansion receipt, **THAIM<sub>+</sub>**, records only its positive part, so a contraction earns no receipt while its loss still counts in the comparison.

$$
\Xi=\mathrm{THAIM}_{+}(\text{forward})-\mathrm{THAIM}_{+}(\text{reverse})-\Delta U=\alpha\Delta S_\tau-\Delta U
$$

Read it as: *Keep the future gained, the future lost, and the energy stored in one comparison.*

The forward and reverse receipts use the same α, so their difference recovers the signed change. The move 1 → 2 leaves the body's future unchanged (**α · ΔS<sub>τ</sub> = 0**); the move 1 → 0 shrinks it from ln 4 to nothing (**α · ΔS<sub>τ</sub> = −2**). Both earn zero receipt. Only the signed change records the difference.

**Action.** The Praxion sets each rate from two ingredients. The first is an opportunity that is the same in both directions: the square root of the complete arrangements on the two sides, **√(4 × 8) = 4√2** on both edges. The second is the factor **e<sup>Ξ/2α</sup>**. For 0 → 1, Ξ = 1, so the factor is √2 and the rate is **4√2 × √2 = 8**; its reverse has Ξ = −1 and rate **4**. Separately, an independent verifier enumerates the microscopic graph and counts its rates directly. Both routes produce **8, 4, 4, 8**, exactly. The Xypher was not tuned to the physics; it was built from its own parts first, and it lands on the same world.

## We tried to break it

A verifier that says “pass” to everything proves nothing. So it was also handed four deliberately broken versions, each with a predicted point of failure:

| Deliberate change | What detects it |
|---|---|
| Forbid the disfavoured move 1 → 2 (Ξ = −1), as a take-only-gains rule would, while keeping its reverse | Reciprocal movement fails. |
| Double one microscopic forward rate only | The microscopic reverse-rate relation fails. |
| Change the stated body room count without changing the graph | The future-to-room correspondence fails. |
| Remove the reverse expansion receipt from Ξ | Reversing an action no longer reverses its signed comparison. |

All eleven checks the verifier runs passed on the intended construction: the six tests split into finer pieces, plus checks that the three Xypher parts produce the same rates. Each broken version failed first exactly where it was predicted to. The first broken version matters most: a mechanism that only ever takes favoured moves, never the reverse, cannot hold a temperature. A real temperature requires the occasional step back.

## What this proves

The construction proves existence: at least one entirely digital graph is thermodynamic by these six tests, and the same graph is a minimal Xypher, built from exactly the three parts.

It does not stand alone. A separate mathematical proof, a **representation theorem**, works in both directions for finite systems at a single temperature, in equilibrium, in which every move can be undone.

- **From thermodynamics to Xyphers.** Every such system that passes the six tests can be represented as a minimal Xypher. Its view of the future can always be supplied as a read-out; it is part of the system's own machinery only when the system already has a move like the reshuffle here.
- **From Xyphers to thermodynamics.** A minimal Xypher passes tests 1 to 5 when its future count is an exact count of arrangements, its energy and temperature are declared independently, and every move is reversible with the same opportunity in both directions. Test 6 additionally needs an explicit two-body construction obeying the same rules.

The sixteen-arrangement world meets every condition. It is the example; the generality comes from the proof, not from testing one instance.

This first Praxion is the simplest possible: its memory holds a single fixed state, and it acts without learning, growing its graph, or changing how it decides. Learning, growth, and self-revision are the next constructions, each with its own state and exchanges to account for.

The physics used here is established stochastic thermodynamics. The contribution is the exact Xypher construction: future possibility, thermodynamic consequence, and executable action organized in one digital object that anyone can inspect and run.

That is the foundation. Digital systems are not normally thermodynamic. This one is, and the proof fits on a laptop. Everything the [Core Thesis](core-thesis.md) builds next stands on it: [growth](equation-of-state-and-growth-of-a-xypher.md) that constructs and repairs its own temperature, and [intelligence measured in physical units](intelligence-as-physical-units.md).

## Inspect and reproduce

Start with the [experiment](../code/xypher-thermodynamics-proof/). Its run guide connects the [prospective Futuruna construction](../code/xypher-thermodynamics-proof/xypher.runa), the [independent Rust verifier](../code/xypher-thermodynamics-proof/src/lib.rs), and the [expected report](../code/xypher-thermodynamics-proof/expected-report.txt). The [published article](https://xyphers.com/research/proving-true-thermodynamic-graph-systems/) links to a fixed source revision.

You can also download the [standalone verifier](../downloads/xypher-thermodynamics-proof-v1.zip), which includes the source-contract test. With Rust installed, unpack it, enter the directory containing `Cargo.toml`, then run:

```sh
cargo test
cargo run --quiet
```

The report lists eleven passing gates, four correctly detected broken constructions, and ends with `OVERALL PASS`. The [run guide](../downloads/README-FIRST-xypher-thermodynamics-proof-v1.txt) explains the files and output.

The scientific records distinguish the [frozen boundary and representation proof](../code/xypher-thermodynamics-proof/evidence/BOUNDARY.md), the [frozen apparatus](../code/xypher-thermodynamics-proof/), and the [exact result and execution provenance](../code/xypher-thermodynamics-proof/evidence/RESULT.md). The earlier files retain their historical accounting identifiers; this article uses the current name THAIM.

[Return to the research index](../README.md).
