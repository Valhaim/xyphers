---
title: "Does Seeing Further Ahead Buy More Open Future?"
description: "An exact driven world tests foresight against aim at equal power. Fuel placed where the traveller is holds up to 2.2 times the open future of random placement, and the edge grows with every island. Aiming further along the route buys more open future in total, but less per unit of energy."
type: "research-article"
status: "published"
maturity: "bounded-exact-result"
published: "2026-10-10"
revised: "2026-10-10"
website_path: "/research/does-seeing-further-ahead-buy-more-open-future/"
web_status: "live"
---

# Does Seeing Further Ahead Buy More Open Future?

Not in these worlds. A traveller that spends fuel further along its planned route keeps more of its future open in total, but every extra step of aim costs more energy than it returns. The best agent in the whole experiment is the one that prepares only its next crossing. Aim itself pays handsomely: at equal power, fuel placed around the traveller holds between 1.35 and 2.21 times the open future that the same fuel buys when it is spread at random. And that edge grows by nearly equal steps with every island added to the world. Three of five predictions written down before the run held. The two about foresight failed, for the second experiment in a row.

## The question left open

[How Much Open Future Can a Unit of Energy Buy?](how-much-open-future-can-a-unit-of-energy-buy.md) measured what a steady flow of energy buys a traveller in a small driven archipelago. Fuel spent where the traveller stood beat fuel spread over every bridge, in every world. But a traveller that planned further ahead did not widen that edge.

Two features of that world may have kept foresight from mattering. The held plan protected its bridges from the weather for free, which helped every agent equally. And the fuel never looked at the plan. This experiment removes both, and asks the question again: does seeing further ahead let an agent buy more open future per unit of energy?

## A world with no free protection

The traveller still lives on three, four, or five islands and holds a route of one, two, or three steps from home. But the route is now only an intention. The weather, drawing timber from a cold store, builds and wears every bridge, including the ones the route needs. When a bridge on the route is missing, the traveller simply cannot take that step until something rebuilds it. Nothing is protected for free.

Fuel from a hot store keeps bridges standing, and each agent spends it in its own way:

![Four small diagrams of four islands with the traveller at home holding the route 0 to 1 to 2. Warming fuels all six bridges. LOCAL fuels the three bridges at home. AIM-1 fuels only the bridge from home to island 1, the next crossing. AIM-2 fuels the whole route, home to 1 and 1 to 2. Compared at the same fuel flow, which fixes the mean number of bridges, only where the bridges stand differs.](figures/does-seeing-further-ahead-buy-more-open-future/aim-agents.svg)

*Where each agent spends its fuel, with the traveller at home and the route 0→1→2. AIM-k fuels the route's bridges within k moves of the traveller, ahead or behind, because the traveller walks back along its route to plan again.*

**Warming** fuels every bridge. **LOCAL** fuels every bridge at the island where the traveller stands. **AIM-k** reads the route: it fuels the bridges the traveller can cross in its next k moves along it. AIM-1 prepares only the next crossing; AIM-3 prepares the whole three-step route. The depth k is the agent's foresight, and it is the only thing that changes along the ladder.

The open future is counted from where the traveller stands: the number of two-step walks it has from there, each step a stay or a crossing. That is its own freedom of action, right now.

## A fair fight at equal power

Comparing raw yields would be unfair. An agent that burns more fuel holds more bridges, and in this world even randomly placed bridges buy more per packet the more of them there are, so heavy spenders look better simply for spending.

The world itself offers a cleaner comparison. Because the weather touches every bridge, the fuel an agent burns fixes exactly how many bridges stand, on average, and the number of bridges fixes the fuel. So every agent can be matched with a rival that holds the same mean number of bridges, placed at random. That rival is warming at exactly the agent's fuel flow.

**THE EDGE AT EQUAL POWER**

$$
E = \frac{H}{H_{\mathrm{warm}}}
$$

**H** is the open future the agent holds for the traveller above the world left to the weather. **H<sub>warm</sub>** is the open future that the same mean number of bridges, placed at random, would hold. Both cost the same fuel.

Read it as: *For the same energy, how much more future does the agent's choice of where to put bridges keep open than chance would?*

## Five predictions, and a blind designer

Five predictions were fixed before anything ran: fuel buys open future; local fuel beats warming at equal power; aiming further along the route raises the edge; a fully prepared route beats looking around; and the edge of local fuel grows with the size of the world.

This time no one saw any aiming agent's result before the run. The designer computed no steady state of a local or aiming agent at any temperature. An independent referee rebuilt every world and was allowed to solve aiming agents only at temperatures outside the experiment. It used those solves only to flag degenerate designs, and reported no values. A literature check found related ideas, but no earlier measure of open future per unit of energy flow set against an equal-power rival and the least price physics allows. The closest is Takahashi and Hayashi's empowerment per joule.

## Aim beats random placement

Every agent that spends fuel on purpose keeps more of the traveller's future open than the weather alone. And at equal power, local fuel beats random placement in all eighteen worlds, holding between 1.35 and 2.21 times its open future.

## Seeing further ahead buys more, at a worse price

The foresight ladder told a clear story. Climbing it, from preparing the next crossing to preparing the whole route, holds more open future in total, at every rung, in every world. But the fuel bill grows faster. At equal power the edge falls at every rung of all nine ladders.

![Two line charts for three islands, three steps ahead, at three pairs of stores, as fuel is aimed one, two, or three steps along the route. Left: the open future held rises with the depth of aim. Right: the edge at equal power over warming falls at every step, and the nearest aim beats LOCAL. Values: cold 3 hot 2: H 0.110, 0.137, 0.150, E 1.37, 1.18, 1.14, LOCAL E 1.35; cold 8 hot 2: H 0.156, 0.193, 0.213, E 1.44, 1.21, 1.17, LOCAL E 1.42; cold 6 hot 3: H 0.114, 0.140, 0.154, E 1.42, 1.20, 1.16, LOCAL E 1.41.](figures/does-seeing-further-ahead-buy-more-open-future/foresight-ladder.svg)

*Three islands, three steps ahead. Left: the open future held rises as the aim reaches further. Right: the edge at equal power falls at every step. The dashes mark LOCAL, which the nearest aim beats.*

With three islands, a three-step route, and stores ×8 and ×2, preparing the next crossing gives an edge of 1.44; preparing two steps, 1.21; the whole route, 1.17. Fuel on every bridge at the traveller's island gives 1.42. In no world with a route of two or three steps did the whole route beat that: the fourth prediction failed too.

The winner was a surprise, though not a hypothesis: AIM-1, which fuels only the route's bridges at the traveller's island, the step just taken and the step about to be taken. It is a strict subset of what LOCAL fuels, and it beats LOCAL in all eighteen worlds. Knowing which way you are about to step pays. Knowing the rest of the road, in these worlds, does not.

## The value of aim grows with the world

The fifth prediction held cleanly. With one step of planning, the edge of local fuel rises with every island at every pair of stores, from about 1.4 with three islands to between 2.0 and 2.2 with five, by nearly equal steps of 0.31 to 0.40 per island.

![Line chart of the edge at equal power over warming, one step ahead, for three, four, and five islands at three pairs of stores. The edge rises by nearly equal steps with every island. Values: cold 3 hot 2: LOCAL 1.36, 1.70, 2.01, AIM-1 1.39, 1.74, 2.06; cold 8 hot 2: LOCAL 1.43, 1.83, 2.21, AIM-1 1.44, 1.86, 2.26; cold 6 hot 3: LOCAL 1.41, 1.80, 2.17, AIM-1 1.43, 1.83, 2.21.](figures/does-seeing-further-ahead-buy-more-open-future/world-size.svg)

*The edge at equal power, one step ahead, with three, four, and five islands. Solid: LOCAL. Dashed: AIM-1, the next crossing.*

Random placement spreads bridges over the whole world, and only some of them touch the traveller's island: two in every three bridges with three islands, two in every five with five. The larger the world, the more of a random budget lands where the traveller is not, and the more aim is worth.

## Why foresight lost, twice

Both experiments count the open future only two steps out, while the prepared route reaches up to three. A bridge further along the route pays only once the traveller arrives, and in these worlds the fuel that kept it standing would have kept more open nearby. The design said so before the run, so the refutations are precise: fuel aimed further along a route does not pay under a near-sighted measure of the future.

That points straight at the next experiment. Count the open future as far ahead as the plan looks, and slow the weather so that a prepared bridge is still standing when the traveller arrives. Then ask again whether seeing further ahead buys more.

## How we know

The experiment, CAL-CEF-4, followed the protocol of the three before it.

1. **Preregistration.** The worlds, the five predictions, eleven acceptance tests, and seven deliberately broken worlds with the exact place each must fail were frozen before anything ran, after two rounds of independent review.
2. **Embargoed build.** A dependency-free Rust verifier was written and only compiled until approval. It reuses, unchanged, the exact arithmetic of the previous experiment and solves each world on the orbits of its symmetry, then checks the answer on every state.
3. **Three reviews of the exact code.** Mathematical correspondence, adversarial checking, and Futuruna/Xypher semantics each approved the same commit without running it.
4. **One run.** The approved commit ran once: all eleven tests passed in all eighteen worlds, for 66 exact steady states with up to 10,240 states each.
5. **An independent check.** A separate Python program, written from the boundary alone, rebuilt every world, solved it exactly, and agrees with the recorded run on every value and every verdict.

The broken worlds failed exactly where they were predicted to:

| Broken world | Where it failed first |
|---|---|
| **One-way fuel**: fuel builds bridges but never returns them | Reciprocity, on the empty archipelago |
| **Free protection**: the weather spares the route's bridges, as in the previous experiment | The weather rule, at the first bridge from home |
| **Equal stores**: both stores multiply by 8 | The second law: no fuel flows |
| **Biased step**: stepping forward twice as likely as stepping back | Reciprocity, on the empty archipelago |
| **Aim off by one**: each aiming agent reaches one step too far | The fuel rule, for the nearest aim |
| **Wrong calibration**: warming checked against the cold temperature | The calibration of warming |
| **Wrong rival**: the equal-power rival computed with the wrong store | The rival check |

## What it means

Aim is real, measurable, and grows with the world. In a world where nothing is protected for free, putting energy where the agent is buys a third to more than double the future that the same energy buys at random, and every added island raises that edge by about the same amount. I believe this is the physical content of agency in its simplest form: not how much energy flows through a system, but where it goes.

Foresight is not free. Twice now, preparing the road further ahead has bought more future in total and less per unit of energy. In these worlds, foresight paid only one step deep: the agent that knew which way it was about to step beat every other. Whether longer foresight pays when the future is counted as far ahead as it looks is the sharpest open question this programme has, and it is the next experiment.

## Read, inspect, reproduce

**Read:** [the evidence guide](../evidence/aimed-foresight/README.txt) explains every file.

**Inspect:** [the preregistered boundary](../evidence/aimed-foresight/BOUNDARY.md) states the worlds, the predictions, the eleven tests, the seven broken worlds, the blinding, and the related work. [The result record](../evidence/aimed-foresight/RESULT.md) gives the verdict, the review chronology, every file digest, the [complete report](../evidence/aimed-foresight/expected-report.txt), and the independent check. The research archive calls this experiment CAL-CEF-4.

**Run:** the [verifier source](../evidence/aimed-foresight/source/xypher-aimed-foresight-proof/README.md) is a dependency-free Rust crate with a prospective [Futuruna declaration](../evidence/aimed-foresight/source/xypher-aimed-foresight-proof/xypher.runa). Following the [run guide](../evidence/aimed-foresight/RUN.txt), from the crate's folder:

```sh
cargo test --release --locked
cargo run --release --locked --quiet
```

The run takes about eight minutes. The 91-line report ends with the five verdicts and `OVERALL PASS`. The [independent check](../evidence/aimed-foresight/independent_check.py) needs only Python. The [whole packet](../evidence/aimed-foresight/) sits beside this article.

## Research trail

- J. M. Horowitz, K. Zhou, and J. L. England, [“Minimum energetic cost to maintain a target nonequilibrium state”](https://doi.org/10.1103/PhysRevE.95.042102), *Physical Review E* 95, 042102 (2017): the price floor.
- A. S. Klyubin, D. Polani, and C. L. Nehaniv, [“Empowerment: A universal agent-centric measure of control”](https://doi.org/10.1109/CEC.2005.1554676), *IEEE Congress on Evolutionary Computation* 1, 128 (2005): open future as control capacity.
- A. D. Wissner-Gross and C. E. Freer, [“Causal Entropic Forces”](https://doi.org/10.1103/PhysRevLett.110.168702), *Physical Review Letters* 110, 168702 (2013): the push toward open futures.
- J. Ramírez-Ruiz, D. Grytskyy, C. Mastrogiuseppe, Y. Habib, and R. Moreno-Bote, [“Complex behavior from intrinsic motivation to occupy future action-state path space”](https://doi.org/10.1038/s41467-024-49711-1), *Nature Communications* 15, 6368 (2024).
- J. Ehrich, S. Still, and D. A. Sivak, [“Energetic cost of feedback control”](https://doi.org/10.1103/PhysRevResearch.5.023080), *Physical Review Research* 5, 023080 (2023): the controller's own cost, not charged here.
- K. Takahashi and Y. Hayashi, [“Thermodynamic limits of physical intelligence”](https://doi.org/10.1007/978-3-032-33195-3_24), *Artificial General Intelligence (AGI 2026)*, Lecture Notes in Computer Science, 339 (2026): empowerment per joule, the closest earlier measure.
- Xyphers research, [“How Much Open Future Can a Unit of Energy Buy?”](how-much-open-future-can-a-unit-of-energy-buy.md): the first driven world and the yardstick.

[Return to the research index](../README.md).
