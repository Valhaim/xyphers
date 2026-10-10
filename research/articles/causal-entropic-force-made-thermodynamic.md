---
title: "The Causal Entropic Force, Made Thermodynamic"
description: "A traveller that commits to its futures and acts only where it stands. No rule counts futures, yet the log-count of futures it could hold becomes the entropy of each arrangement of its world."
type: "research-article"
status: "published"
maturity: "bounded-exact-result"
published: "2026-10-10"
revised: "2026-10-10"
website_path: "/research/causal-entropic-force-made-thermodynamic/"
web_status: "live"
---

# The Causal Entropic Force, Made Thermodynamic

The causal entropic force is a push toward whatever keeps the most futures open. Its equation needs a temperature. In the simulations where the idea was born, that temperature was a dial the researchers set.

The [Core Thesis](core-thesis.md) asks for more: a world in which the drive toward an open future comes from the world's own thermodynamics, at a temperature nobody chose.

This article presents such a world, and the proof.

A traveller lives on a small archipelago. It commits to one future at a time, a short walk from home, and acts only on the island where it stands. No rule in its world counts how many futures exist. Every move is as likely as its reverse.

Yet when the world settles, each arrangement of bridges is weighted by exactly the number of futures the traveller could commit to, against the timber it costs. The logarithm of that count is the entropy of each arrangement of bridges. And on average, the traveller builds bridges with exactly the odds the causal entropic force predicts, at the temperature of its own timber store.

**In this world, the drive toward an open future is not a law laid on top of the physics. It is the physics.**

The result is exact. There is no random seed, no sample, and no fitted curve. The experiment was preregistered, independently reviewed before it ran, executed once, and passed every test. You can rerun it in about a second.

## Counting futures, not guessing them

The Core Thesis measured an open future by following a traveller who picks bridges at random and asking how evenly its possible destinations are spread. That spread is useful, but it cannot be thermodynamic as it stands.

Temperature, as the [first digital thermodynamic system](proving-true-thermodynamic-graph-systems.md) showed, comes from counting rooms: the equally weighted ways a system can be arranged. The spread of a random traveller's destinations is uneven, and the entropy of an uneven spread is generally not the logarithm of any whole number. It is not a count of rooms.

So this world counts futures instead of weighing them.

A **plan** is a walk from home, a fixed number of steps long, along existing bridges. At each step the traveller may cross a bridge or stay where it is. Write **N<sub>τ</sub>(G)** for the number of different plans the bridges **G** allow, where **τ** is the number of steps. Staying put is always allowed, so every configuration has at least one plan.

**THE COUNT OF COMMITTABLE FUTURES**

$$
N_\tau(G) = \#\{\tau\text{-step walks from home, staying allowed}\}
$$

Read it as: *How many different futures could the traveller commit to from home?*

Take three islands, home being island 0, and look two steps ahead. This is the main example of the experiment. With no bridges, the traveller can only stay: one plan. With one bridge from home, it can stay twice, stay and then cross, cross and come back, or cross and stay: four plans. With both home bridges, seven. With all three bridges, nine. A bridge between the two other islands, on its own, adds nothing, because the traveller cannot reach it.

## The traveller

The traveller is the simplest kind of Praxion, the part of a Xypher that acts. It does not learn. It follows four rules:

1. **It holds one plan** as part of its state, and walks it, a step forward or a step back.
2. **It replans only at home**, choosing uniformly among the plans the current bridges allow.
3. **It builds or dismantles only a bridge at the island where it stands.**
4. **It never dismantles a bridge its plan still uses.**

Every bridge stores one packet of timber, drawn from a reservoir. Write **λ** for the energy of one packet. The reservoir's possible arrangements multiply by a fixed factor **b** with every packet it holds, which fixes its temperature, **α = λ / ln b**, before anything moves: one packet of energy buys one multiplication by **b**.

At the finest level, every move is exactly as likely as its reverse. Nothing in the rules reads **N<sub>τ</sub>**. Nothing favours building over dismantling.

The plan matters. It decides where the traveller stands, and the plan's route decides which bridges are protected. Two travellers standing at home on the same archipelago, one planning to stay and one planning to cross to island 1, can do different things: only the first may dismantle the bridge to island 1. The plan is not a label beside the dynamics; it is part of what moves.

## Where the futures go

Here is the whole mechanism in one observation.

A plan the traveller holds must be a valid walk on the current bridges, and the traveller can be at any of its **τ + 1** positions along it. So a configuration of bridges contains exactly **(τ + 1) × N<sub>τ</sub>(G)** states of the traveller. Every way of committing to a future is a separate state of the world.

Because every move is as likely as its reverse, every complete arrangement of traveller and reservoir is equally likely. Each bridge takes a packet from the reservoir and leaves it **b** times fewer arrangements, so each state of the traveller is weighted by its timber **U**, the energy stored in bridges: the Boltzmann factor **e<sup>−U/α</sup>**, which divides by **b** for every bridge. Add up the states of each configuration, and the futures appear:

**THE CONFIGURATION LAW**

$$
\pi(G) \propto N_\tau(G)\, e^{-U(G)/\alpha}
$$

Read it as: *Each arrangement of bridges is weighted by the futures it lets the traveller hold, against the timber it costs.*

**U(G)** is the timber stored in bridges and **α** is the reservoir's temperature. Taking logarithms, the configuration's free energy is **U(G) − α ln N<sub>τ</sub>(G)**. The entropy term is the log-count of committable futures.

The effect is large. With three islands and a reservoir that doubles per packet, a world in which futures did not matter would hold one bridge on average. This world holds more, and more as the traveller looks further ahead:

| Steps ahead | Mean bridges at equilibrium | Without futures |
|---:|---:|---:|
| 1 | 19/15 ≈ 1.27 | 1 |
| 2 | 131/87 ≈ 1.51 | 1 |
| 3 | 313/185 ≈ 1.69 | 1 |

A colder reservoir makes timber dearer. With **b = 3** and two steps ahead, the mean falls to **5/4**, still above the **3/4** a futureless world would hold at that temperature.

## The force appears

Now ask the Core Thesis's question about a single bridge: how strongly does the world favour having it?

At equilibrium, a bridge is built and dismantled equally often overall; that is what balance means. The favour shows in the rates. Take one arrangement of bridges without this bridge, and the same arrangement with it, holding every other bridge fixed. Average over everything the traveller might be doing, and compare the rate at which the bridge is built from the first arrangement with the rate at which it is dismantled from the second. Equivalently, compare how long the archipelago spends in those two arrangements. The ratio is exactly:

**THE EMERGENT CAUSAL ENTROPIC ODDS**

$$
\frac{\text{build rate}}{\text{dismantle rate}} = e^{\Xi/\alpha},\qquad \Xi = \alpha\,\Delta\ln N_\tau - \Delta U
$$

Read it as: *A bridge is favoured by the futures it opens, valued at the world's temperature, against the timber it binds.*

This **Ξ** is the Core Thesis's thermodynamic drive, with the future measured as committable plans. Its first term is the causal entropic force made into a discrete step on a graph. Nothing in the rules computed it.

Two honest notes belong here. First, once the configuration law holds, these odds follow from it automatically; they are not a separate discovery. The discovery is that local rules which never look at the future count produce them. Second, the force exists only as an average. At any moment, what the traveller can build depends on where it stands, so there is no rule at the level of configurations alone. The force emerges from the traveller's committed futures, the way pressure emerges from molecules.

In the main example, one pair of neighbouring arrangements never exchanges at all. While home has no bridge, the traveller is confined to home and can never reach the bridge between the two far islands, so it can neither build nor remove that bridge there. The verifier reports that pair and leaves it out of the odds. With four islands there are more such pairs, for the same reason.

## The traveller prefers well-connected islands

Looking two steps ahead, the futures can be counted by hand. Building a bridge from home to an island that already has **d** bridges adds exactly

**THE HOME-BRIDGE LAW**

$$
\Delta N_2 = d + 3
$$

Read it as: *A bridge to a well-connected island opens more futures than a bridge to an isolated one.*

The new futures all use the new bridge: stay and then cross, cross and come back, cross and stay, or cross and continue along any of the island's **d** existing bridges.

So the same bridge is favoured more when the island across is already connected. With a reservoir that doubles per packet, each bridge halves the weight. From an empty archipelago, the bridge from home to island 1 turns one plan into four, so it is favoured 4/2, two to one. If island 1 is already bridged to island 2, the same home bridge turns one plan into five, favoured five to two.

This is the intuition of the Core Thesis's opening island example, now exact. There, a bridge to an island with three bridges reached four destinations; here it opens six new two-step plans, twice the three an isolated island offers. A preference for joining the network where it is already rich appears without any attachment rule. It is the entropy of committed futures.

## Same temperature, no flow

A temperature must survive contact. Two archipelagos, each prepared at equilibrium against reservoirs with the same **b**, are detached from their reservoirs and allowed to trade timber: one dismantles a bridge while the other builds one. On every possible total of bridges, the energy flowing between them is exactly zero on average, both for two identical archipelagos and for two that look different distances ahead.

Prepare the second archipelago against a colder reservoir instead, and the prepared state no longer holds still: the first joint arrangement checked gains probability at rate 8/3 but loses it at 7/3.

## We tried to break it

A verifier that says "pass" to everything proves nothing. Before it ran, the experiment fixed eight deliberately broken versions and where each must fail:

| Broken version | What failed first |
|---|---|
| A builder that can build but never dismantle | Reversibility, at the first bridge. A take-only-gains builder cannot hold a temperature. |
| Replanning by the random traveller's destinations instead of by plans | The futures are no longer equally weighted: the first uneven law is 4/9, 5/18, 5/18. |
| One microscopic move made twice as likely as its reverse | Equal reverse rates. |
| Ξ without the reverse receipt | Reversing an action no longer reverses its value. |
| Plans that may not stay put | The empty archipelago has no futures at all. |
| Replanning with plans one step too short | The futures held are not the futures counted. |
| Dismantling a bridge the plan still uses | The traveller is left holding an impossible plan. |
| One archipelago prepared against a colder reservoir before contact | Same-temperature balance fails. |

Every broken version failed first exactly where it was predicted to. The intended construction passed all twelve tests on the main example, and every applicable test on all eight cases of a family with three or four islands, one to three steps ahead, and two reservoir temperatures. Every count, partition function, and mean predicted in advance came out exactly.

## What this establishes, and what it does not

Within this finite model, the experiment establishes:

1. A world in which a simple acting mechanism holds one future of one to three steps as state, walks it, replans uniformly at home, and builds or dismantles only where it stands passes the first five of the six operational tests of a thermodynamic system in all eight cases (the reservoir itself enumerated for three islands) and the contact test for two pairs of archipelagos, at a temperature fixed by an independent reservoir.
2. Its equilibrium law over bridge configurations is the count of committable futures times the Boltzmann factor of their timber. The log-count of futures is the entropy of each arrangement.
3. Although no rule reads that count, between any two arrangements the traveller can actually toggle, the average odds of building and dismantling are the causal entropic odds at the world's own temperature.
4. Looking two steps ahead, the preference for bridging to well-connected islands follows exactly.

It does not establish:

- that the entropy of a random traveller's destinations, or of the traveller's actual path, equals the log-count of plans;
- anything about agents that do not hold their futures as state;
- growth with new islands, or any world kept out of equilibrium by a steady supply of timber;
- learning, control, or intelligence;
- new fundamental physics. This is equilibrium statistical mechanics of a declared model. Its counting is the same mechanism that makes rubber elastic, where a polymer's shapes are walks. Here the walks are committed futures.

A literature search found no prior construction in which an actor's count of committable futures is the entropy of an exact equilibrium ensemble at an independently grounded temperature, with the causal entropic odds emerging from local rules. That is a statement about the search, not a proof of novelty.

## Why it matters

The causal entropic force has always sounded like an extra principle: a push toward the future, added to physics. This world shows another way to see it. Give an actor a future to hold, let it act where it stands, and let every move be reversible. Here, the push toward more futures is simply what entropy does.

That changes what a Xypher is. Its drive toward an open future does not need to be programmed in or paid for by an outside rule. It can be the thermodynamics of the futures it carries.

The next experiment uses the same world. The held plan is a command, and where the walk ends is an outcome. Measure how many bits of control the plan gives over the destination, inside this same energy account, and test whether each bit is worth exactly one packet of energy, as [Intelligence as Physical Units](intelligence-as-physical-units.md) predicts for a reservoir that doubles per packet.

Beyond that lie the harder frontiers: futures that are not committed, worlds that grow new islands, and a steady flow of timber that keeps the traveller building.

## Inspect and reproduce

**Read:** [the evidence guide](../evidence/causal-entropic/README.txt) explains every file.

**Inspect:** [the preregistered boundary](../evidence/causal-entropic/BOUNDARY.md) states the construction, the predictions, the twelve tests (called gates), the eight broken versions, and the claims. It was frozen, before any evaluator ran, at the commit named in the result record; its header still reads "draft", the wording it was frozen with. [The result record](../evidence/causal-entropic/RESULT.md) gives the verdict, the review chronology, every file digest, and the [complete report](../evidence/causal-entropic/expected-report.txt). The research archive calls this experiment CAL-CEF-1.

**Run:** the [verifier source](../evidence/causal-entropic/source/xypher-causal-entropic-proof/README.md) is a dependency-free Rust crate with a prospective [Futuruna construction](../evidence/causal-entropic/source/xypher-causal-entropic-proof/xypher.runa). Following the [run guide](../evidence/causal-entropic/RUN.txt), from the crate's folder:

```sh
cargo test --locked
cargo run --locked --quiet
```

The 31-line report ends with `OVERALL PASS`. The [whole packet](../evidence/causal-entropic/) sits beside this article.

## Research trail

- A. D. Wissner-Gross and C. E. Freer, [“Causal Entropic Forces”](https://doi.org/10.1103/PhysRevLett.110.168702), *Physical Review Letters* 110, 168702 (2013): the entropy of a system's own future paths, with temperature as a model parameter.
- Z. Burda, J. Duda, J. M. Luck, and B. Waclaw, [“Localization of the Maximal Entropy Random Walk”](https://doi.org/10.1103/PhysRevLett.102.160602), *Physical Review Letters* 102, 160602 (2009): weighting every path on a fixed graph equally.
- S. Pressé, K. Ghosh, J. Lee, and K. A. Dill, [“Principles of maximum entropy and maximum caliber in statistical physics”](https://doi.org/10.1103/RevModPhys.85.1115), *Reviews of Modern Physics* 85, 1115 (2013): ensembles over paths.
- A. S. Klyubin, D. Polani, and C. L. Nehaniv, [“Empowerment: A Universal Agent-Centric Measure of Control”](https://doi.org/10.1109/CEC.2005.1554676), IEEE Congress on Evolutionary Computation (2005): counting the futures an agent can reach.
- J. Schnakenberg, [“Network theory of microscopic and macroscopic behavior of master equation systems”](https://doi.org/10.1103/RevModPhys.48.571), *Reviews of Modern Physics* 48, 571 (1976): reversible dynamics on graphs and local detailed balance.
- Xyphers research, [“A Purely Digital Thermodynamic System”](proving-true-thermodynamic-graph-systems.md): the six tests a digital temperature must pass, and the first world that passed them.

[Return to the research index](../README.md).
