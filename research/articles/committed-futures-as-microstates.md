---
title: "Committed Futures as Microstates"
description: "The causal entropic force pushes a system toward open futures with a strength set by hand. A traveller that holds one future and acts only where it stands shows what sets that strength once futures are real states: the temperature of its world, on every archipelago and over every horizon."
type: "research-article"
status: "published"
maturity: "bounded-exact-result"
published: "2026-10-10"
revised: "2026-10-10"
website_path: "/research/committed-futures-as-microstates/"
web_status: "live"
---

# Committed Futures as Microstates

The push toward an open future has a dial. In the equation behind the causal entropic force, a temperature sets how hard a system is pushed, and in the simulations where the idea was born, someone chose it.

Give the futures a body, and the dial is gone. A traveller that holds one future as part of its state, and acts only where it stands, makes its world favour open futures at exactly the temperature of the world's own timber store. How often the traveller changes its mind makes no difference.

This holds on every archipelago and over every horizon. It is a proof, and the [manuscript](../evidence/committed-futures/manuscript.pdf) behind this article sets it out for physicists.

[When Futures Become Entropy](causal-entropic-force-made-thermodynamic.md) built this traveller and checked it exactly in eight small cases: three or four islands, one to three steps ahead, two store temperatures.

## A dial in the equation

The [Core Thesis](core-thesis.md#the-causal-entropic-force) introduced the push toward an open future in its compact form:

**THE PUSH TOWARD AN OPEN FUTURE**

$$
F = T\,\nabla S_\tau
$$

**F** is the push. **∇S<sub>τ</sub>** is the slope toward more future, measured as the entropy of the paths a system could follow over the next stretch of time **τ**. **T** sets how hard the push is.

Read it as: *Push toward a more open future, with a strength set by a temperature.*

Alex Wissner-Gross and Cameron Freer, who proposed this force in 2013, worked with two temperatures. One belongs to the surroundings and sets how hard the system is jostled by noise. The other is the **T** in this equation, which they called the causal path temperature. It was a separate setting, chosen for each simulation.

They offered a guess about what it might mean. A rubber band's molecules can take many shapes, and they keep turning from one shape into another. Picture every possible path of a system as such a shape, turning into the others. The causal path temperature, they suggested, might measure how quickly that happens.

The Core Thesis asked for something stricter: a push whose strength comes from the world itself. If futures were real states of a world, what would set the dial?

## A traveller with one future in hand

A traveller lives on an archipelago. One island is home. A bridge can be built between any two islands, and each bridge holds one packet of timber, energy **λ**, taken from a store. The store's possible arrangements multiply by a fixed factor **b** with every packet it holds, which fixes its temperature, **α = λ / ln b**, before the traveller moves.

A **plan** is a walk from home, **τ** steps long, where each step crosses a bridge or stays put. Write **N<sub>τ</sub>(G)** for the number of plans the bridges **G** allow. With three islands, two steps, and both bridges from home built, there are seven.

The traveller is the simplest Praxion, the acting part of a [Xypher](core-thesis.md#the-xypher). It holds one plan as part of its state and walks it, a step forward or a step back. At home it may swap its plan for any other. It builds or dismantles only a bridge at the island where it stands, and it never dismantles one its plan crosses. Every move is exactly as likely as its reverse.

Holding a plan is a commitment: while the traveller holds it, the world must keep that walk possible. No move's likelihood depends on **N<sub>τ</sub>**.

## Every archipelago, every horizon

Let the world run until it settles. How often does each arrangement of bridges appear?

Every way of holding a future is a separate state of the world: a plan, and a position along it. The [Core Thesis](core-thesis.md#one-floor-many-rooms) called the equally weighted ways a world can be arranged its rooms; physicists call them microstates. Because every move is as likely as its reverse, every complete state of traveller and timber store is equally likely in the long run. Each bridge leaves the store **b** times fewer arrangements. Add up the traveller's states in each arrangement of bridges, and the count of futures appears:

**THE CONFIGURATION LAW**

$$
\pi(G) \propto N_\tau(G)\, e^{-U(G)/\alpha}
$$

**π(G)** is how often the arrangement **G** appears once the world has settled. **U(G)** is the timber stored in its bridges. **e<sup>−U/α</sup>** is the Boltzmann factor: it divides by **b** for every bridge.

Read it as: *Each arrangement of bridges is weighted by the futures the traveller could hold in it, against the timber it costs.*

The earlier article checked this exactly in those eight cases. It is now proven for any number of islands from two upward and any horizon of at least one step. With a horizon of zero steps the traveller never leaves home, bridges between other islands can never change, and the proof needs at least one step.

The proof is short, and its shortness is the point. It needs three facts: the world's states pair an arrangement of bridges with a plan valid in it; every move has an equally likely reverse; and every state can reach every other. Walking the plan, acting locally, and swapping plans only at home decide how fast the world settles and which arrangements trade with which. They do not decide where it settles. One ingredient makes the futures count: the world may never break a walk the traveller holds.

So the log-count of futures the traveller could hold is the entropy of each arrangement of its world. Committed futures are microstates.

## The dial is the world's temperature

Take a particle that rolls downhill in energy **U**, is pushed up the slope of future entropy with strength **T<sub>c</sub>**, and is jostled by surroundings at temperature **T<sub>r</sub>**. As long as the future entropy depends only on where the particle is, the push and the energy combine into one landscape, and over the long run the particle settles into the Boltzmann law for that landscape:

**THE LONG-RUN ODDS OF A PUSH TOWARD THE FUTURE**

$$
\pi \propto e^{-(U - T_c \ln N)/T_r}
$$

**T<sub>c</sub>** is the strength of the push, the dial. **T<sub>r</sub>** is the temperature of the surroundings. **ln N** is the entropy of the futures, here the log-count of plans.

Read it as: *The push toward futures and the jostling of the surroundings each carry their own temperature.*

The configuration law has exactly this form, with **T<sub>c</sub> = T<sub>r</sub> = α**. The strength of the push is the temperature of the timber store. There is no second dial to set.

And the guess about speed? Make the traveller swap plans a thousand times faster, or a thousand times slower. The world settles faster or slower, but where it settles does not move: every move still has an equally likely reverse, and the configuration law contains no rate. In this world, how quickly futures turn into one another does not set the strength of the push.

## Why a bridge stays

If no rule counts futures, why does the world keep the bridges that open them?

Watch one bridge. Take three islands, two steps ahead, a store that doubles per packet, and an empty archipelago. The traveller has one plan, staying home, and three positions along it. From each of those three states it can build the bridge to island 1.

Build it, and the traveller has four plans and twelve states. Only three of them can dismantle the bridge again: those holding the plan that stays home. The other nine hold a plan that crosses the bridge, and the traveller never removes a bridge its plan crosses.

Lay the states side by side. Gold states can change the bridge; red states hold a plan that crosses it.

![Traveller states with and without the bridge from home to island 1, three islands, two steps ahead, a store that doubles per packet. Without the bridge: one plan, three states, all three can build it, total weight 3. With it: four plans, twelve states; only the three holding the stay-home plan can dismantle it, nine are protected; weight one half each, total 6. Odds two to one.](figures/committed-futures-as-microstates/protection.svg)

*Every state of the traveller, before and after the bridge to island 1 is built. Each row is a plan, written as the islands it visits; each square is a position along it, showing where the traveller stands. Three islands, two steps ahead, a store that doubles per packet.*

The states that can dismantle the bridge are exactly the states that could build it: the same plan, at the same positions. So the bridge is built and removed equally often. Yet the world with the bridge holds four times as many states, at half the weight each for its timber. It spends twice as long with the bridge as without: odds of two to one, exactly the causal entropic odds.

This holds for every bridge on every archipelago. The world keeps a bridge in proportion to how many futures exist with it, compared with without it. No rule refers to that count; the only reference to the future is the plan the traveller holds. The pull toward open futures is protection.

Protection also explains the traveller's taste in islands. Looking two steps ahead, a bridge from home to an island with **d** bridges adds exactly **d + 3** plans, so more plans cross a bridge to a well-connected island, and more of the traveller's states protect it. The preference is gentle. Compared with a bridge to an isolated island, the odds of a bridge to an island with **d** bridges are higher by the factor **(N<sub>2</sub> + d + 3) / (N<sub>2</sub> + 3)**, where **N<sub>2</sub>** is the number of two-step plans before either is built; the factor shrinks toward 1 as home's neighbourhood grows. In Barabási and Albert's growing networks, a new link picks an island in proportion to the links it already has, a preference written into the rule; walkers wandering a growing network can produce the same preference without one. Here the preference is weaker, and it belongs to a world at equilibrium.

## Nothing has to count

A skeptic could still point at one rule. When the traveller swaps plans at home, it chooses among all valid plans, so in a sense it lists them.

Take that away. Let the traveller, at home, change its plan one island at a time: swap the island at a single step of the walk for another, as long as the walk stays valid. Every plan can still reach every other, every move is still reversible, and the configuration law is unchanged. Now nothing in the world lists, samples, or scores the futures. The count arrives anyway.

## Odds without a rule

Can the push be written as a rule on bridges alone, a chance of building or dismantling that depends only on which bridges stand?

It cannot. Put the traveller at home beside the bridge to island 1. If it holds the plan that stays home, it may dismantle the bridge. If it holds a plan that crosses, it may not. Same bridges, different possibilities. On every archipelago and for every horizon, the bridges alone do not say what happens next.

The clock confirms it. If the bridges alone set the future, the world would leave an arrangement on a memoryless clock, the kind that times radioactive decay, with every state of the traveller leaving at the same rate. In the bridge example above (three islands, two steps ahead, a store that doubles per packet), with only the bridge to island 1 built, the traveller's states leave at rates averaging 4, and the mean square of those rates is 28 instead of the 16 a single rate would give.

So the causal entropic odds are exact, and they are averages over what the traveller holds. The push lives in the futures, the way pressure lives in molecules.

## Two travellers, one world

Let a second traveller share home. Each holds its own plan, and a bridge may be dismantled only if neither plan crosses it.

The futures multiply. Each arrangement is now weighted by **N<sub>τ</sub>(G)<sup>2</sup>**, the number of ways the two travellers could hold plans together, against the same timber. With three islands, two steps ahead, and a store that doubles per packet, the full archipelago appears about 19% of the time instead of 10%, and the empty one about 2% of the time instead of 9%. The chart sets all eight arrangements side by side.

![Paired bar chart of the long-run share of time each of the eight bridge arrangements appears, three islands, two steps ahead, store doubling per packet, for one traveller and for two travellers sharing home. With two travellers the full archipelago rises from 10% to 19% and the empty one falls from 9% to 2%.](figures/committed-futures-as-microstates/two-travellers.svg)

*Long-run share of time for each of the eight arrangements of three islands, two steps ahead, with a store that doubles per packet: one traveller against two travellers sharing home.*

Arrangements rich in futures gain and poor ones lose: every arrangement with five or more plans is kept more often, and every arrangement with four or fewer, less.

Measured against one traveller's count, two travellers with the same home and horizon lean on the world twice as hard; measured against their joint count, the temperature is unchanged. Both readings describe the same law. Travellers with different homes multiply their own counts the same way: each arrangement is weighted by the product of their counts. The product is exact because these travellers interact only through protection. Travellers that crowd each other off islands, or share plans, are open territory.

## Looking further ahead

What happens as the horizon grows?

Over a long horizon, each extra step multiplies the number of plans by nearly the same factor, **Λ**. That factor depends only on the part of the archipelago home can reach. Mathematicians call it the largest eigenvalue of that part's connection table, with a stay counted as a connection. Its logarithm, **ln Λ**, is the entropy per step of the maximal-entropy random walk, the walk that treats every long path as equally likely. A traveller looking far ahead weights its world, to leading order, by this one property of the part of the network home can reach.

Exact enumeration on five islands, with a store that doubles per packet, shows how the settled world responds. The table gives the average number of bridges standing, and how many of the four bridges from home stand. Watch whether either column ever falls as the horizon grows.

| Steps ahead | Mean bridges, of 10 | Mean home bridges, of 4 |
|---:|---:|---:|
| 1 | 3.71 | 1.71 |
| 2 | 4.04 | 1.89 |
| 4 | 4.54 | 2.13 |
| 8 | 5.39 | 2.44 |
| 12 | 6.12 | 2.69 |

Both rise at every step from one to twelve. On this archipelago, the further the traveller looks, the more of its world it keeps built.

The chart follows every horizon from one to twelve, with dashed lines where the averages would sit if only the timber counted.

![Line chart of the long-run mean number of bridges on five islands, store doubling per packet, for horizons one to twelve steps. All bridges rise from 3.71 to 6.12 of 10; bridges from home rise from 1.71 to 2.69 of 4. Both rise at every step and stay above the values 3.33 and 1.33 that hold when futures are not counted.](figures/committed-futures-as-microstates/long-horizon.svg)

*Long-run averages on five islands with a store that doubles per packet, computed exactly for every horizon from one to twelve steps. Dashed lines: the same averages when only the timber is counted.*

The gap between each line and its dashed floor is the count of futures at work, and it widens with every step the traveller looks ahead.

## Where this sits in physics

The counting at the heart of this result is old physics, and that is good news: anyone trained in statistical mechanics can check it.

A rubber band pulls back because a stretched chain of molecules has fewer shapes than a relaxed one, and each shape is a walk. The traveller's plans are walks too. The Fortuin–Kasteleyn and Edwards–Sokal representations of magnets use the same trick: they weight each arrangement of bonds by the number of spin arrangements compatible with it. Here a walk takes the place of the spins. Network scientists call a model that weights each network by an exponential of some measured property an exponential random graph model. The configuration law is one, and its property is the log-count of walks from home.

The original force met an objection this world has to answer. Hilbert Kappen showed that when the noise is the same everywhere, the original path entropy is the same in every state, so the push vanishes. Here the count of plans changes with the bridges, so the objection does not apply.

On the side of agents, the empowerment of Alexander Klyubin, Daniel Polani, and Chrystopher Nehaniv measures how many distinct places an agent can reach, destinations rather than routes, and Christoph Salge, Cornelius Glackin, and Daniel Polani built empowerment-driven agents that place and remove blocks to keep their options open. In the path-integral control of Kappen and the linearly solvable control of Emanuel Todorov, when allowed paths cost nothing and forbidden ones are excluded, an optimal controller weights each move by the allowed paths that follow it, at a temperature set by the noise or by the price of control. In all of these, the drive toward futures is something the agent wants or computes.

Here it is neither. The traveller's moves are blind to the count, and the drive shows up in the long-run law of the world it lives in. A literature search found no earlier construction of a local, reversible actor whose world settles into this walk-weighted law, nor the observation that these odds carry the push toward futures at exactly the temperature of the world, whatever the rate at which futures turn into one another.

## Toward a thermodynamics of committed futures

The proof covers a world at equilibrium: a fixed archipelago, a steady store, and a traveller that holds its futures without learning from them. Each of those edges is a frontier. I believe this is where a thermodynamics of agency starts: not with a principle that tells actors what to want, but with the futures they hold. The manuscript closes its discussion with six questions, each precise enough to test.

1. **Control as information.** The plan fixes where the walk ends. A record of that destination, used to steer, is worth at most one packet of timber per bit when the store doubles per packet. With both bridges from home built, the destinations of the seven plans fall 3/7, 2/7, and 2/7: 1.557 bits, just under the 1.585 bits of three equal outcomes, so a perfect record of where the walk ends is worth at most 1.557 packets. Can a full cycle of measuring, steering, and erasing close exactly in this world? [It does](what-is-a-bit-of-foresight-worth.md): at equilibrium, one bit of foresight trades evenly for one packet. Driving that cycle to lift real loads is next.
2. **Futures not committed.** Can the entropy of paths a traveller merely might take, rather than holds, become thermodynamic entropy? The walk that reaches the long-horizon rate needs global knowledge of the network, exactly what one-step replanning removed. Is there a walk that uses only local knowledge and still reaches it?
3. **Growth.** Feed timber from a store at another temperature and let new islands appear, and the world is held away from equilibrium, settling at best into a steady state that keeps a flow running. Does the count of committed futures still shape what grows, and at what cost in entropy production? The first half [has run](how-much-open-future-can-a-unit-of-energy-buy.md): fed from a hotter store, a traveller that spends fuel where it stands holds more open future per unit of fuel flow than one that warms the whole world, against a price floor physics sets. New islands are next.
4. **Learning.** A traveller that remembers must carry its memory as state, with every learning step reversible, or the law changes. Which ways of learning keep the thermodynamics exact, and what does learning cost?
5. **Crowds.** Travellers that only protect each other's bridges multiply their futures. Which ways of interacting, such as crowding each other off islands or sharing plans, break the product, and do they favour structures no product of counts predicts?
6. **Scale.** On large archipelagos with a fixed horizon, bridges far from home barely feel the futures. Large-scale structure needs the horizon or the price of timber to grow with the world. When they grow together, what large-scale structure appears?

## Read, inspect, reproduce

**Read:** [the manuscript](../evidence/committed-futures/manuscript.pdf), "Committed futures as microstates: causal entropic bias as configurational entropy at the bath temperature", with every proof. [The evidence guide](../evidence/committed-futures/README.txt) explains every file.

**Inspect:** [the LaTeX source](../evidence/committed-futures/manuscript.tex) and [its references](../evidence/committed-futures/references.bib). The preregistered experiment the manuscript builds on, CAL-CEF-1, has [its own packet](../evidence/causal-entropic/README.txt).

**Run:** [checks.py](../evidence/committed-futures/checks.py) is a dependency-free Python script that recomputes every check the preregistration did not cover, using whole numbers and exact fractions only. From the packet's folder:

```sh
python3 checks.py
```

Its 52 lines end with `ALL CHECKS PASS`, in a few seconds; the [expected output](../evidence/committed-futures/expected-checks.txt) is in the packet. The [whole packet](../evidence/committed-futures/) sits beside this article.

## Research trail

- A. D. Wissner-Gross and C. E. Freer, [“Causal Entropic Forces”](https://doi.org/10.1103/PhysRevLett.110.168702), *Physical Review Letters* 110, 168702 (2013): the push toward open futures, with the causal path temperature as a separate parameter.
- H. J. Kappen, [“Comment: Causal entropic forces”](https://arxiv.org/abs/1312.4185), arXiv:1312.4185 (2013): with noise the same everywhere, the original path entropy is the same in every state, so its push vanishes.
- C. M. Fortuin and P. W. Kasteleyn, [“On the random-cluster model”](https://doi.org/10.1016/0031-8914(72)90045-6), *Physica* 57, 536 (1972), and R. G. Edwards and A. D. Sokal, [“Generalization of the Fortuin-Kasteleyn-Swendsen-Wang representation and Monte Carlo algorithm”](https://doi.org/10.1103/PhysRevD.38.2009), *Physical Review D* 38, 2009 (1988): weighting bonds by the configurations compatible with them.
- J. Park and M. E. J. Newman, [“Statistical mechanics of networks”](https://doi.org/10.1103/PhysRevE.70.066117), *Physical Review E* 70, 066117 (2004): exponential random graph models.
- A.-L. Barabási and R. Albert, [“Emergence of Scaling in Random Networks”](https://doi.org/10.1126/science.286.5439.509), *Science* 286, 509 (1999): growing networks with preferential attachment written into the rule.
- Z. Burda, J. Duda, J. M. Luck, and B. Waclaw, [“Localization of the Maximal Entropy Random Walk”](https://doi.org/10.1103/PhysRevLett.102.160602), *Physical Review Letters* 102, 160602 (2009): the walk that treats every path as equally likely.
- A. S. Klyubin, D. Polani, and C. L. Nehaniv, [“Empowerment: A Universal Agent-Centric Measure of Control”](https://doi.org/10.1109/CEC.2005.1554676), IEEE Congress on Evolutionary Computation (2005): counting the futures an agent can reach.
- C. Salge, C. Glackin, and D. Polani, [“Changing the Environment Based on Empowerment as Intrinsic Motivation”](https://doi.org/10.3390/e16052789), *Entropy* 16, 2789 (2014): agents that build and remove blocks to keep their options open.
- H. J. Kappen, [“Linear Theory for Control of Nonlinear Stochastic Systems”](https://doi.org/10.1103/PhysRevLett.95.200201), *Physical Review Letters* 95, 200201 (2005), and E. Todorov, [“Efficient computation of optimal actions”](https://doi.org/10.1073/pnas.0710743106), *PNAS* 106, 11478 (2009): controllers that weight moves by the admissible paths ahead.
- T. Sagawa and M. Ueda, [“Minimal Energy Cost for Thermodynamic Information Processing: Measurement and Information Erasure”](https://doi.org/10.1103/PhysRevLett.102.250602), *Physical Review Letters* 102, 250602 (2009): the energy price of a bit of measurement and erasure.
- Xyphers research, [“When Futures Become Entropy”](causal-entropic-force-made-thermodynamic.md): the traveller, and the preregistered exact experiment on eight small cases.
- Xyphers research, [“A Purely Digital Thermodynamic System”](proving-true-thermodynamic-graph-systems.md): the six tests a digital temperature must pass.

[Return to the research index](../README.md).
