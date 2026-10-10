---
title: "How Much Open Future Can a Unit of Energy Buy?"
description: "Feed a world energy and let a traveller spend it. In an exact driven archipelago, fuel spent where the traveller stands keeps up to 79% more futures open per packet than fuel spread everywhere, and planning further ahead did not add to that edge."
type: "research-article"
status: "published"
maturity: "bounded-exact-result"
published: "2026-10-10"
revised: "2026-10-10"
website_path: "/research/how-much-open-future-can-a-unit-of-energy-buy/"
web_status: "live"
---

# How Much Open Future Can a Unit of Energy Buy?

Energy keeps futures open. A cell, a firm, or a machine spends power to keep options it would otherwise lose. This experiment measures the exchange rate exactly, in a small world fed by a hot store and worn down by a cold one. Fuel buys open future. Fuel spent where the traveller stands buys between 21% and 79% more of it per packet than the same fuel spread over the whole world. And a traveller that plans further ahead did not widen that edge: two of the four predictions written down before the run failed.

## From a price to a budget

[What Is a Bit of Foresight Worth?](what-is-a-bit-of-foresight-worth.md) put an exact price on foresight: **α ln 2** of energy per bit, in a world at equilibrium. At equilibrium every trade runs both ways equally often. Nothing has to be paid continuously, because nothing is being held against the world's drift.

Agency is not like that. A living thing that stops eating does not keep its options; it loses them, at a rate set by its surroundings. The question for an agent is not what a future costs once. It is a rate: how many open futures a steady flow of energy can hold open.

Physics already sets a floor under that rate. Horowitz, Zhou, and England showed in 2017 that holding a system in any state away from equilibrium, by adding driven transitions while the energies stay fixed, costs at least the rate at which the undriven world would relax that state back. You pay at least as fast as the world forgets. That floor gives every agent a yardstick: how much open future it holds per unit of energy, and how close it comes to the least that physics allows.

## A world with weather and fuel

The archipelago of [When Futures Become Entropy](causal-entropic-force-made-thermodynamic.md) returns, with three or four islands and a traveller who holds one plan: a walk of one, two, or three steps from home. Each bridge holds one packet of timber.

Now the world has **weather**. A cold store builds bridges anywhere and wears them down anywhere, except the bridges the traveller's held plan crosses. Left alone, the weather settles the world at the cold store's temperature, with a known number of near futures open from home.

The traveller has **fuel**: a second, hotter store, which it may spend only on bridges touching the island where it stands. It can build a bridge there with a packet from the hot store, or return an unused one to it.

What makes one store hotter than the other is how readily it parts with a packet. A store whose arrangements multiply by 8 with every packet it holds gives up a packet only reluctantly, because losing one costs it a factor of 8 in ways to be. That reluctance is what cold means. A store that multiplies by 2 parts with packets easily, so it is hot. Every move in this world is as likely as its reverse up to its store's factor, and no move reads how many futures it opens.

![Schematic of the driven archipelago. Three islands; the traveller stands at home and holds the plan 0 to 1 to 1. The bridge from home to island 1 is built and crossed by the plan, so the weather leaves it alone. The bridge from island 1 to island 2 is built and the weather may wear it, sending a packet to the cold store. The bridge from home to island 2 is absent; the weather may build it, and so may fuel, because it touches the traveller's island. Below, packets flow from the hot store, which multiplies by 2 per packet, through the bridges, into the cold store, which multiplies by 8: the fuel flow J.](figures/how-much-open-future-can-a-unit-of-energy-buy/driven-world.svg)

*The driven archipelago. Weather acts everywhere but leaves the held plan's bridges alone; fuel acts only beside the traveller. At steady state, packets flow from the hot store through the bridges into the cold one.*

Fed this way, the world never settles to equilibrium. It settles to a steady state through which energy keeps flowing: packets leave the hot store, pass through the bridges, and end up in the cold one. That flow is what the traveller spends.

## What is measured

The **open future** of an arrangement of bridges is the number of two-step walks from home it allows: how many near futures are open. The same two-step yardstick is used whatever horizon the traveller plans with, so travellers with different horizons are compared on one scale.

**OPEN FUTURE PER UNIT OF FUEL**

$$
Y = \frac{H}{J}
$$

**H** is the average number of open two-step walks in the driven world, minus the same average in the world left to the weather alone. **J** is the fuel flow: the net number of packets per unit time that leave the hot store. **Y** is the open future held per unit of fuel flow.

Read it as: *Feed the world one packet per unit time, and Y more near futures stay open, on average, at the traveller's way of spending.*

Two travellers spend the same kind of fuel in different ways. The **targeted** traveller spends it only where it stands. The **spreading** traveller spends it on every bridge in the world. The spreading traveller turns out to be exactly the same as warming the whole world to one temperature between the two stores: fuel spent everywhere is heat. That gives the targeted traveller a fair rival with the same fuel and no aim.

## A test that could fail

The two earlier experiments in this archipelago were exact calculations: once the world was built correctly, the answer followed. This one was built so that the world could say no. Four predictions were written down, with the exact worlds that decide each, before anything ran:

1. **Fuel buys open future**: the targeted traveller holds more open walks than the weather alone, in every world.
2. **Aim beats warmth**: per unit of fuel flow, the targeted traveller holds more open future than the spreading one, in every world.
3. **The yield rises with the horizon**: a traveller that plans further ahead holds more per unit of fuel.
4. **Foresight sharpens the aim**: the targeted traveller's edge over the spreading one grows with the horizon.

The twelve worlds have three islands with horizons of one, two, and three steps, or four islands with one, each at three pairs of stores: cold ×3 and hot ×2, cold ×8 and hot ×2, cold ×6 and hot ×3. An early pilot, at other temperatures and with a different measure, had shown the targeted traveller's edge shrinking as the horizon grew, so the fourth prediction was a real risk. The pilot was published with the predictions.

## Fuel buys open future

It does, in all twelve worlds. With three islands, a two-step plan, and stores ×8 and ×2, the weather alone leaves 2.84 two-step walks open from home on average. The targeted traveller holds 3.35 open, and draws 0.75 packets per unit time from the hot store to do it.

## Aim beats warmth

The spreading traveller holds even more open, 3.44 in the same world. It also burns 1.23 packets per unit time to do it. Per packet, the targeted traveller wins: **0.69** open walks per unit of fuel flow against **0.49**, an edge of 1.41.

That pattern holds everywhere. In every world the spreading traveller holds more extra futures in total, and in every world the targeted traveller holds more per unit of fuel, by a factor between 1.21 and 1.79. The edge was largest with four islands, between 1.62 and 1.79. In a wider world, a larger share of the warming lands on bridges away from the traveller: half of them with four islands, a third with three.

![Line chart of open two-step futures held above the cold world per packet of fuel flow, three islands, planning one, two, or three steps ahead, for three pairs of stores. Fuel spent where the traveller stands beats fuel spread over every bridge at every point, but the edge shrinks as the horizon grows. Values, for horizons one to three: cold 3 hot 2: targeted 1.06, 1.11, 1.09, spread 0.78, 0.86, 0.90, edge 1.36, 1.29, 1.21; cold 8 hot 2: targeted 0.54, 0.69, 0.73, spread 0.38, 0.49, 0.53, edge 1.44, 1.41, 1.38; cold 6 hot 3: targeted 0.68, 0.81, 0.84, spread 0.48, 0.59, 0.63, edge 1.42, 1.39, 1.34.](figures/how-much-open-future-can-a-unit-of-energy-buy/open-future-per-fuel.svg)

*Open futures held per packet of fuel flow, three islands. Gold: fuel spent where the traveller stands. Grey: the same fuel spread over every bridge. The edge, in red beneath each horizon, is the ratio of the two.*

Warmth is not useless. It keeps bridges standing everywhere, and some of them carry futures. But most of what the spreading traveller pays for sits where it will not walk. Targeted fuel keeps open the part of the world the traveller is about to use.

## Foresight did not sharpen the aim

The two remaining predictions failed.

The targeted traveller's yield did rise with its horizon at the two colder pairs of stores. At the warmest pair, cold ×3 and hot ×2, it rose from 1.06 to 1.11 open walks per unit of fuel between one and two steps ahead, then fell to 1.09 at three. The third prediction holds at two pairs and fails at one, so it is refuted.

The fourth failed everywhere. At every pair the edge of aim over warmth shrank as the horizon grew: from 1.36 to 1.29 to 1.21 at the warmest pair, from 1.44 to 1.41 to 1.38 at the coldest. Planning further ahead helped the spreading traveller more than it helped the targeted one.

The design points to why. In this world, the plan does one thing besides choosing the walk: it decides which bridges the weather leaves alone, and that protection is free, whoever paid for the bridge. A longer plan covers more of the world with free protection, for the spreading traveller as much as for the targeted one. And the targeted traveller spends fuel on every bridge around its island, whether or not its plan crosses that bridge next. Its aim comes from where it stands, not from where it is going. Foresight that aims the fuel, and protection that has to be paid for, are [the next experiment](does-seeing-further-ahead-buy-more-open-future.md).

## The price floor

Every steady state has a floor under its cost: the rate at which the weather alone would undo it. The experiment computed that floor for every state the travellers held, with certified bounds, and compared it with what they actually paid.

![Horizontal bar chart for three islands, two steps ahead, cold store multiplying by 8 and hot store by 2. Open futures held per packet of fuel flow: 0.49 when fuel is spread over every bridge, 0.69 when it is spent where the traveller stands, and 2.07 for the same held state at the Horowitz, Zhou and England price floor.](figures/how-much-open-future-can-a-unit-of-energy-buy/price-floor.svg)

*Open futures per packet of fuel flow, three islands, two steps ahead, stores ×8 and ×2. The dashed bar is what the targeted traveller's state would yield if it were held at the price floor.*

Both travellers pay between 2.2 and 3.0 times the floor for the states they hold. The spreading traveller comes a little closer to its floor than the targeted one, in every world. Its efficiency is fixed by the two stores alone, 0.34 at stores ×8 and ×2; the targeted traveller's is 0.33.

So the traveller that comes closer to its own floor is the one that buys less open future. Efficiency asks how cheaply an agent holds the state it holds. Yield asks how much open future that state contains. Choosing what to hold mattered more than holding it cheaply.

The floor also shows the room left. Held at the floor, the targeted traveller's state would yield 2.07 open walks per unit of fuel, three times what it gets. No agent with a fixed hot store reaches that floor exactly, but the gap measures how much better aiming could still do.

## How we know

The experiment, CAL-CEF-3, followed the protocol of the two before it.

1. **Preregistration.** The worlds, the four predictions, ten acceptance tests, and six deliberately broken worlds with the exact place each must fail were fixed in a boundary document before anything ran. An independent referee rebuilt every world separately, confirmed every value fixed in advance, and was forbidden from computing the targeted traveller's results before the freeze.
2. **Embargoed build.** A dependency-free Rust verifier with its own exact arithmetic, and a Futuruna declaration of the world, was written and only compiled until approval. Its generic arithmetic was also checked against Python on 119,224 cases before approval; the result record lists exactly what ran.
3. **Three reviews of the exact code.** Mathematical correspondence, adversarial checking, and Futuruna/Xypher semantics each approved the same commit without running it.
4. **One run.** The approved commit was executed once. All ten tests passed in all twelve worlds for both travellers, 24 exact steady states with up to 344 states each.
5. **An independent check.** After the run, a separate Python program written from the boundary alone rebuilt every world and solved it exactly. It agrees with the recorded run on every value, as exact fractions, and on every verdict.

The broken worlds failed exactly where they were predicted to:

| Broken world | Where it failed first |
|---|---|
| **One-way fuel**: the traveller can build with fuel but never return a packet | Reciprocity, on the empty archipelago |
| **Plan-breaking weather**: weather may wear a bridge the held plan crosses | The rules, at the first bridge from home |
| **Equal stores**: both stores multiply by 8 | The second law: no fuel flows at all |
| **Biased weather**: one weather move made twice as likely | Reciprocity, on the empty archipelago |
| **Wrong store**: fuel moves at the cold store's odds | Reciprocity, on the empty archipelago |
| **Wrong calibration**: the spreading traveller checked against the cold temperature | The calibration of the spreading traveller |

## What it means

Energy buys open future, and where it is spent matters more than how much is spent. That is now a measured statement, in a world small enough to hold in mind, with the least price physics allows drawn beneath it.

I believe this ratio, open future held per unit of energy flow, is the right yardstick for agency. It applies to anything with a declared energy account and a declared set of futures: a cell, a firm, an economy, a learning machine. It says how much of the future an agent keeps open with what it spends, and the floor says how much more it could keep.

The failed predictions are the most useful part. They show that a longer horizon is not foresight unless something reads it: here the plan protected bridges for free, and the fuel never looked at it. The next experiment gives the traveller fuel that it aims along its own plan, and makes protection something it has to pay for. Then the question becomes sharp: does seeing further ahead let an agent buy more open future per unit of energy? [That experiment has run](does-seeing-further-ahead-buy-more-open-future.md): at equal power, aim pays more the larger the world, and preparing the road beyond the next crossing costs more than it opens.

## Read, inspect, reproduce

**Read:** [the evidence guide](../evidence/open-future-price/README.txt) explains every file.

**Inspect:** [the preregistered boundary](../evidence/open-future-price/BOUNDARY.md) states the worlds, the predictions, the ten tests, the six broken worlds, and the pilot that came before. [The result record](../evidence/open-future-price/RESULT.md) gives the verdict, the review chronology, every file digest, and the [complete report](../evidence/open-future-price/expected-report.txt). The research archive calls this experiment CAL-CEF-3.

**Run:** the [verifier source](../evidence/open-future-price/source/xypher-open-future-price-proof/README.md) is a dependency-free Rust crate with a prospective [Futuruna declaration](../evidence/open-future-price/source/xypher-open-future-price-proof/xypher.runa). Following the [run guide](../evidence/open-future-price/RUN.txt), from the crate's folder:

```sh
cargo test --release --locked
cargo run --release --locked --quiet
```

The 46-line report ends with `OVERALL PASS` and the four verdicts. The [independent check](../evidence/open-future-price/independent_check.py) needs only Python. The [whole packet](../evidence/open-future-price/) sits beside this article.

## Research trail

- J. M. Horowitz, K. Zhou, and J. L. England, [“Minimum energetic cost to maintain a target nonequilibrium state”](https://doi.org/10.1103/PhysRevE.95.042102), *Physical Review E* 95, 042102 (2017): the price floor.
- J. Schnakenberg, [“Network theory of microscopic and macroscopic behavior of master equation systems”](https://doi.org/10.1103/RevModPhys.48.571), *Reviews of Modern Physics* 48, 571 (1976): entropy production of a driven network.
- U. Seifert, [“Stochastic thermodynamics, fluctuation theorems and molecular machines”](https://doi.org/10.1088/0034-4885/75/12/126001), *Reports on Progress in Physics* 75, 126001 (2012).
- A. D. Wissner-Gross and C. E. Freer, [“Causal Entropic Forces”](https://doi.org/10.1103/PhysRevLett.110.168702), *Physical Review Letters* 110, 168702 (2013): the push toward open futures.
- Xyphers research, [“When Futures Become Entropy”](causal-entropic-force-made-thermodynamic.md), [“Committed Futures as Microstates”](committed-futures-as-microstates.md), and [“What Is a Bit of Foresight Worth?”](what-is-a-bit-of-foresight-worth.md): the archipelago, its temperature, and the price of foresight.

[Return to the research index](../README.md).
