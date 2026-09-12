---
title: "A Purely Digital Thermodynamic System"
type: "research-article"
status: "published"
published: "2026-09-06"
revised: "2026-09-06"
website_path: "/research/proving-true-thermodynamic-graph-systems/"
web_status: "live"
source_revision: "c46f4a808"
---

# A Purely Digital Thermodynamic System


A purely digital graph can support an exact thermodynamic system. We have constructed one with sixteen complete arrangements, a conserved energy account, and a temperature recovered independently from its reservoir and its movement.

The same object has the three parts introduced in the [Core Thesis](core-thesis.md): a **Graph Substrate** describing what can happen, a **Thermodynamic Harness** accounting for change, and a **Praxion** that makes the moves happen.

Its small size is useful. We can inspect every arrangement and every allowed transition. The result follows from counting the entire object.

## A body and its surroundings

Imagine a body that can occupy three positions in a line: **0 ↔ 1 ↔ 2**. Moving one position to the right transfers one energy packet from a reservoir into the body. Moving left returns one packet. Body and reservoir together always hold two packets.

An energy packet is the digital model's own unit. The graph represents where that energy is held and which transfers can occur. The temperature we will measure belongs to this computational world.

Each position contains several internal arrangements. Think of a building whose floors mark energy and whose rooms are the different ways to hold that energy. We declare the rooms before calculating how the system moves.

| Position | Body × reservoir | Together |
|---|---:|---:|
| 0 | 1 × 4 | 4 |
| 1 | 4 × 2 | 8 |
| 2 | 4 × 1 | 4 |

A complete arrangement specifies both the body and its reservoir. Multiply their room counts at each position: **4 + 8 + 4 = 16**.

Each allowed microscopic move between neighbouring positions has a reverse with the same rate. Every arrangement at position 0 connects to every arrangement at position 1; likewise between positions 1 and 2. The unit rate sets the clock used in this construction.

## The first thermometer counts rooms

The reservoir has one arrangement when empty, two when it holds one packet, and four when it holds two. Each additional energy packet doubles its number of arrangements.

Entropy measures the spread of possibilities. For **M** equally likely arrangements, it is **ln M**, where **ln** is the natural logarithm. This measures entropy in **nats**. Doubling the number of arrangements adds **ln 2** nats.

Temperature compares the energy added with the entropy gained. Let **λ** be the size of one energy packet. Write **α** for temperature in energy per nat.

$$
\alpha=\frac{\lambda}{\ln 2}
$$

Read it as: *One packet of energy buys one doubling of reservoir arrangements.*

Taking the packet as one energy unit gives **α = 1 / ln 2**. This is an independently grounded temperature: its value follows from the reservoir we declared before examining any transition rate.

In ordinary temperature units, **α = k<sub>B</sub>T**. Boltzmann's constant **k<sub>B</sub>** converts kelvin into the energy scale used with entropy measured in nats.

## The second thermometer counts movement

Start in any complete arrangement at position 0. There are eight arrangements available at position 1, each connected by a unit-rate move. The total rate from 0 to 1 is therefore eight. From position 1, four arrangements lead back to position 0.

The same counting gives all four rates:

| Move | Forward rate | Reverse rate | Forward / reverse |
|---|---:|---:|---:|
| 0 → 1 | 8 | 4 | 2 |
| 1 → 2 | 4 | 8 | 1/2 |

These are rates per unit time. They describe how quickly transitions occur, rather than probabilities that must sum to one.

Knowing only the body position is sufficient to predict these rates. Every internal arrangement at the same position has the same total rate into each neighbouring position. The exact calculation checks this property, called **strong lumpability**, before treating the three positions as a complete description of body movement.

Now use the physical relation between forward and reverse traffic. Write **ΔS** for the change in the body's entropy and **ΔU** for its stored-energy change. Let **k<sub>forward</sub>** and **k<sub>reverse</sub>** name the two rates.

$$
\ln\frac{k_{\rm forward}}{k_{\rm reverse}}=\Delta S-\frac{\Delta U}{\alpha}
$$

Read it as: *The imbalance between a move and its reverse reflects the total entropy change of body and reservoir.*

This relation is called **local detailed balance**. Moving energy into the body removes it from the reservoir, which supplies the second term.

For **0 → 1**, the body gains **ln 4** nats and one energy unit. The measured rate ratio is two. Substituting those three independently known values gives **ln 2 = ln 4 − 1/α**, hence **α = 1 / ln 2**.

For **1 → 2**, the body still has four arrangements, so its entropy change is zero. It takes in another energy unit. The measured ratio is one-half, giving the same temperature.

**The two thermometers agree exactly.** The reservoir count predicts α. Independently enumerated microscopic movement recovers it.

## Where future possibility enters

The body's arrangements are also executable outcomes of a declared graph action. At each body position, a refresh action selects uniformly among its internal arrangements while preserving energy. Its one-step endpoint entropy is therefore the logarithm of the body room count: **0, ln 4, ln 4**.

This supplies the connection to **S<sub>τ</sub>**, the future-entropy observable in the Core Thesis. Here **τ = 1** and the declared perspective is that refresh action. The future count and the state count refer to the same possibilities.

That connection is part of the construction. Measuring an unrelated random walk on an arbitrary graph would require its own justification before its entropy could enter this thermodynamic account.

The harness first computes the full signed change **α · ΔS<sub>τ</sub>**. The positive expansion receipt, **THAIM<sub>+</sub>**, records only its positive part. A contraction earns zero receipt while retaining its negative contribution to the action comparison.

$$
\Xi=\mathrm{THAIM}_{+}(\text{forward})-\mathrm{THAIM}_{+}(\text{reverse})-\Delta U=\alpha\Delta S_\tau-\Delta U
$$

Read it as: *Keep the future gained, the future lost, and the energy stored in one comparison.*

The forward/reverse identity uses the same α for both directions. The receipt by itself loses information: preserving an arrangement and destroying future possibility can both earn zero. The signed change distinguishes them before any receipt is produced.

The Praxion combines Ξ with the declared opportunities for movement to construct transition rates. An independent verifier separately enumerates the microscopic graph. Both routes produce **8, 4, 4, 8** exactly. Neither route fits its answer to a sampled trajectory.

## Balance and contact

Every move transfers an energy packet between body and reservoir. Their total remains two, and this undriven construction receives no external work.

At equilibrium, the three body positions have probabilities **1/4, 1/2, 1/4**. The middle position contains twice as many complete arrangements as either end. Traffic balances on both edges: for example, **(1/4) × 8 = (1/2) × 4** between positions 0 and 1.

Individual moves continue at equilibrium. A positive Ξ makes a move more frequent relative to its reverse; finite temperature still permits the reverse. A mechanism that simply forbids every unfavoured move would change the thermodynamic system.

Temperature must also survive contact. Prepare two bodies independently against reservoirs at the same α. Detach those reservoirs and consider the combined bodies when they hold two energy packets in total. They can share those packets as **(0, 2), (1, 1), or (2, 0)**.

Their body arrangements give these three cases weights **4, 16, 4**, or probabilities **1/6, 4/6, 1/6**. Enable reciprocal energy exchange between the bodies. The prepared probabilities already form the contact equilibrium, and the expected energy current into either body is exactly zero.

The reservoir thermometer, the movement thermometer, and the contact calculation describe the same temperature.

## What makes the check demanding

The verifier checks the complete state, independent entropy and energy coordinates, reversible movement, every energy exchange, equilibrium, and contact. These are the six operational requirements named **T1–T6** in the mathematical boundary.

It also receives four deliberately broken constructions:

| Deliberate change | What detects it |
|---|---|
| Forbid one move while retaining its reverse | Reciprocal movement fails. |
| Double one microscopic forward rate only | The declared microscopic reverse-rate relation fails. |
| Change the stated body room count without changing the complete graph | The future/state counting correspondence fails. |
| Remove the reverse expansion receipt from Ξ | Reversing an action no longer reverses its signed comparison. |

All eleven acceptance gates passed on the intended construction. Each broken version failed first at its predicted gate. The arithmetic uses exact integer and rational identities; acceptance depends on no simulation duration, random seed, or floating-point tolerance.

## The achievement and the next question

The construction proves existence: at least one entirely digital graph has the declared operational thermodynamic properties and a minimal Xypher organization.

A separate analytic proof establishes a representation theorem for the stated **fixed, finite, one-temperature, reciprocal equilibrium class**, with independently grounded state multiplicities, energy, lawful projections, and contact. The sixteen-state enumeration supplies the example; it does not establish generality by testing one instance.

This minimal Praxion is non-adaptive. Its memory has one condition. It acts, but it does not learn, grow a graph, or modify its own decision process. Those richer capabilities become new constructions whose additional state and exchanges must also be accounted for.

The result uses established stochastic thermodynamics. Its contribution is the exact Xypher construction and correspondence: future outcomes, thermodynamic consequences, and executable action organized in one inspectable digital object. It supplies a concrete foundation for the experimental programme described in the [Core Thesis's agency section](core-thesis.md#a-computational-paradigm-for-agency).

The [growth article](equation-of-state-and-growth-of-a-xypher.md) asks how an adaptive system can construct and maintain the structure grounding its temperature. [Intelligence as Physical Units](intelligence-as-physical-units.md) asks how controllable futures can enter the same physical account.

## Inspect and reproduce

The [standalone verifier](../downloads/xypher-thermodynamics-proof-v1.zip) contains the prospective Futuruna construction, independent Rust enumeration, and source-contract test. With Rust installed, unpack it, enter the directory containing `Cargo.toml`, then run:

```sh
cargo test
cargo run --quiet
```

The report lists eleven passing gates, four correctly detected broken constructions, and ends with `OVERALL PASS`. The [run guide](../downloads/README-FIRST-xypher-thermodynamics-proof-v1.txt) explains the files and output.

The scientific records distinguish the [frozen boundary and representation proof](../code/xypher-thermodynamics-proof/evidence/BOUNDARY.md), the [frozen apparatus](../code/xypher-thermodynamics-proof/), and the [exact result and execution provenance](../code/xypher-thermodynamics-proof/evidence/RESULT.md). The earlier files retain their historical accounting identifiers; this article uses the current name THAIM.

[Return to the research index](../README.md).
