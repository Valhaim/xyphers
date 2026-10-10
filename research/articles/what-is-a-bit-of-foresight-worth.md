---
title: "What Is a Bit of Foresight Worth?"
description: "A traveller writes down where its plan will take it. Inside a world with its own temperature, that note has an exact price: one bit of foresight trades evenly for one packet of timber."
type: "research-article"
status: "published"
maturity: "bounded-exact-result"
published: "2026-10-10"
revised: "2026-10-10"
website_path: "/research/what-is-a-bit-of-foresight-worth/"
web_status: "live"
---

# What Is a Bit of Foresight Worth?

A traveller writes down where its walk will end. That note is worth timber: exactly one packet for every bit of foresight it carries, when the timber store doubles with every packet it holds.

[Intelligence as Physical Units](intelligence-as-physical-units.md) proposed that control has a price in energy, **α ln 2** per bit, and named the experiment that would measure it: put the command and its outcome inside one fully accounted world, and check that the temperature still reads the same. That experiment has run, in the archipelago where [committed futures are microstates](committed-futures-as-microstates.md). It was preregistered, built under embargo, reviewed before it ran, and executed once. Every test passed, exactly.

## The price on the table

The [Core Thesis](core-thesis.md#intelligence-as-physical-units) gave intelligence its first physical unit: how much difference an action can reliably make to what happens next, measured in bits. One reliable choice between two futures is one bit of control.

A real temperature puts a price on that unit. Temperature is energy per nat, and one bit is ln 2 nats, so each bit of control should be worth **α ln 2** of energy, whenever the controller and what it controls share one energy account. The claim is simple to state. It needs a world where it can be measured: a world with its own temperature, a controller inside it, and an outcome the controller decides.

The archipelago of [When Futures Become Entropy](causal-entropic-force-made-thermodynamic.md) is that world. A traveller holds one plan, a short walk from home, and builds or dismantles bridges only where it stands. Each bridge holds one packet of timber, drawn from a store whose arrangements multiply by **b** with every packet, which fixes the temperature **α = λ / ln b**. With **b = 2**, one packet of energy is exactly **α ln 2**. The prediction becomes a sentence a child can check: one bit, one packet.

## A notebook and a stockpile

The held plan is a command. Where it ends is the outcome. To make the link between them a physical thing, the traveller gets two new possessions.

A **notebook** sits in the world's state, like the plan. At home, the traveller can copy into a blank notebook the island where its held plan ends. It can also uncopy a note that still matches the plan, at no cost: a matching copy can be undone exactly.

A **stockpile** is a stack of timber beside the shore. Each packet on it carries one packet of energy and no arrangements of its own, like a weight raised on a rope.

Two new moves connect them. With a correct note in hand, the traveller may **harvest**: it lets go of its plan, free to hold any plan at all, and keeps the note, while one packet moves from the store onto the stockpile. The reverse move **commits**: it takes a packet from the stockpile back to the store, and the plan becomes one that ends where the note says. And the traveller may **erase** a written note back to blank, which drops a stockpile packet into the store as heat, or **scribble** a symbol onto a blank page, which lifts one.

Every move is exactly as likely as its reverse. No move reads how much information the note carries. Then the world is left alone.

## Trading a correct note for timber

Take three islands, two steps ahead, and the store that doubles per packet. Build one bridge, from home to island 1. The traveller can now hold four plans. Two end at home and two end at island 1, so a note saying where the plan ends carries exactly one bit.

The figure lines up the two sides of the trade.

![The feedback exchange at the arrangement with one bridge from home to island 1, two steps ahead, store doubling per packet. Left: the note says the walk ends at island 1; two of the four plans match, two states at weight 1, total 2. Right: the plan is released and one packet is stored; all four plans with the note kept, weight one half each, total 2. Even odds: one bit of foresight trades for one packet.](figures/what-is-a-bit-of-foresight-worth/the-trade.svg)

*The trade at one bridge from home to island 1. Left: the note reads 1, and two of the four plans agree with it. Right: the plan is released, the note is kept, and one packet sits on the stockpile, which halves the weight of every state.*

With the note correct, two states agree with it. After the harvest, all four plans are possible again, but each state carries one more packet of stored energy and so half the weight. Two against two: once the world has settled, it spends exactly as long on one side of the trade as on the other. One bit of foresight is worth exactly one packet of timber.

The same counting holds for every note on every arrangement of bridges:

**THE PRICE OF A CORRECT NOTE**

$$
K(G, y) = \frac{N_\tau(G)}{n_y(G)\, b}
$$

**K** is the long-run odds of trading a correct note for one stored packet. **N<sub>τ</sub>(G)** is the number of plans the bridges **G** allow. **n<sub>y</sub>(G)** is how many of them end at island **y**, the island the note names. **b** is the store's multiplier per packet.

Read it as: *A note is worth what it rules out: the more plans it excludes, the more timber it can buy.*

When **K = 1** the trade is even, and the note is worth exactly one packet. When the note names one of three equally likely destinations, as with all three bridges built, **K = 3/2**: the note carries log₂ 3 ≈ 1.585 bits, more than one packet's worth, and the trade toward timber is favoured. A note about a certain outcome, with only one place the walk can end, has **K = 1/2**. It is worth nothing, and storing a packet for it is a loss.

Averaged over the traveller's plans, a correct note is worth its information: in packets, its information in bits, when the store doubles per packet. The chart shows that worth for all eight arrangements of three islands.

![Bar chart of the average worth of a correct note, in timber packets, for the eight arrangements of three islands, two steps ahead, store doubling per packet: 0 with no bridge or only the far bridge, exactly 1 with one bridge from home, about 1.52 with a home bridge plus the far bridge, about 1.56 with both home bridges, and about 1.58 with all three. Red marks show the control capacity, which the note reaches whenever every destination is equally likely.](figures/what-is-a-bit-of-foresight-worth/foresight-worth.svg)

*Average worth of a correct note, in packets, for the eight arrangements of three islands, two steps ahead, with a store that doubles per packet. Red marks: the control capacity, the most information a choice of plan can carry about where the walk ends.*

Where every reachable island ends the same number of plans, the note's worth reaches the control capacity, the value Intelligence as Physical Units put on control. Where some islands are reached by more plans than others, as with both bridges from home built, the plans themselves carry slightly less than the capacity: 1.557 bits against 1.585.

## The shuffled deck

The same price appears without any trade at all.

Keep the notebook, keep the plans, and keep how often each note and each plan appears, but pair them at random. Nothing about the energy changes: the note costs nothing to hold, right or wrong. Only the link between note and plan is gone. The correctly paired world has exactly **α ln 2** more free energy per bit of that link than the shuffled one, under the traveller's own uniform choice of plans and under the capacity's choice alike. The possibilities stay; the control is gone; its price is exact.

## Wiping the page

Information has a cost on the way out, too. Erasing a written note returns the notebook to one blank state from many possible symbols, and each erasure drops a stockpile packet into the store as heat.

Here the notebook has three symbols, one per island. A page that could read any of the three holds log₂ 3 bits, more than one packet can pay for when the store doubles per packet: the long-run odds of erasing are **2/3**. With a store that triples per packet, one packet pays exactly for a three-symbol page, and the odds are even. Read the other way, a blank page is fuel: scribbling on it stores timber at odds **3/2** when the store doubles per packet.

A note that still matches the plan never needs erasing. It can be uncopied for free, the way a careful computer undoes a calculation step by step.

## No free lunch

Copy a note for free, harvest a packet with it, wipe it for a packet. At equilibrium the stockpile gains nothing on average: every route that stores a packet is exactly **b** times less likely than its reverse, and the net flow of timber onto the stockpile is exactly zero. Foresight buys timber only at the price the store sets. The second law holds, inside a world with a notebook in it.

## The world keeps its temperature

The instrument does not bend what it measures. The same store sets the odds of every move that carries timber: building bridges, harvesting, erasing. The world still weights each arrangement of bridges by the futures the traveller could hold in it, exactly as before the notebook arrived. And left to itself, the notebook knows nothing: a written note is correct a third of the time, as chance alone would have it. Foresight is a prepared condition, and the experiment measures exactly what it is worth.

## How we know

The experiment, CAL-CEF-2, followed the same protocol as its predecessor.

1. **Preregistration.** Every number above, fourteen acceptance tests, and nine deliberately broken worlds with the exact place each must fail were fixed in a boundary document before anything ran. An independent referee rebuilt the whole model separately and reproduced every prediction before it was frozen.
2. **Embargoed build.** A dependency-free Rust verifier, with a Futuruna declaration of the construction, was written and only compiled until approval.
3. **Three reviews of the exact code.** Mathematical correspondence, adversarial checking, and Futuruna/Xypher semantics each approved the same commit without running it.
4. **One run.** The approved commit was executed once. All fourteen tests passed on the main example; all seven family cases, with three or four islands, one to three steps ahead, and stores that double or triple per packet, passed every test that applies to them.

The broken worlds failed exactly where they were predicted to. Three of them are demons:

| Broken world | Where it failed first |
|---|---|
| A **cheating harvester** that trades a wrong note for timber | The harvest rule, on the empty archipelago |
| A **telepathic demon** that harvests with a blank notebook, reading the future directly | The harvest rule, on the empty archipelago |
| A **hoarding demon** that stores a packet but keeps its plan | The harvest rule, at the first bridge from home |

Each demon left the world reversible and still at a temperature. They failed because the tests check what a correct note, a release, and a reset actually are, not only that a temperature exists. The other six broken worlds, from a one-way harvest to a free reset, a blind notebook, a biased move, an unequal shuffle, and a larger stockpile, failed at their predicted places too.

## What it means

Control now has a measured price. A note about where your future goes is a physical correlation, and in a world with its own temperature it is worth **α ln 2** per bit: as free energy, as an exchange rate for timber, and against the cost of forgetting. I believe this is the first exact place where a Xypher's control over its own future and its thermodynamics meet as one quantity, and it is small enough to hold in your mind: four plans, one note, two against two.

The world here sits at equilibrium, so the trade runs both ways equally often and no engine turns. A first driven world has since run: [How Much Open Future Can a Unit of Energy Buy?](how-much-open-future-can-a-unit-of-energy-buy.md) feeds the archipelago from a hotter store and measures how many open futures a unit of fuel flow holds, against the least price physics allows. Driving this notebook itself, with a load on the stockpile and a stream of blank pages, would turn foresight into an engine, with its efficiency bounded by the second law.

## Read, inspect, reproduce

**Read:** [the evidence guide](../evidence/control-information/README.txt) explains every file.

**Inspect:** [the preregistered boundary](../evidence/control-information/BOUNDARY.md) states the construction, the predictions, the fourteen tests, the nine broken worlds, and the claims. [The result record](../evidence/control-information/RESULT.md) gives the verdict, the review chronology, every file digest, and the [complete report](../evidence/control-information/expected-report.txt). The research archive calls this experiment CAL-CEF-2.

**Run:** the [verifier source](../evidence/control-information/source/xypher-control-information-proof/README.md) is a dependency-free Rust crate with a prospective [Futuruna construction](../evidence/control-information/source/xypher-control-information-proof/xypher.runa). Following the [run guide](../evidence/control-information/RUN.txt), from the crate's folder:

```sh
cargo test --locked
cargo run --locked --quiet
```

The 41-line report ends with `OVERALL PASS`. The [whole packet](../evidence/control-information/) sits beside this article.

## Research trail

- L. Szilard, [“Über die Entropieverminderung in einem thermodynamischen System bei Eingriffen intelligenter Wesen”](https://doi.org/10.1007/BF01341281), *Zeitschrift für Physik* 53, 840 (1929): one bit of information, one unit of work at the bath's temperature.
- R. Landauer, [“Irreversibility and Heat Generation in the Computing Process”](https://doi.org/10.1147/rd.53.0183), *IBM Journal of Research and Development* 5, 183 (1961): the cost of erasure.
- C. H. Bennett, [“The thermodynamics of computation—a review”](https://doi.org/10.1007/BF02084158), *International Journal of Theoretical Physics* 21, 905 (1982): measurement can be reversible; erasure carries the cost.
- T. Sagawa and M. Ueda, [“Minimal Energy Cost for Thermodynamic Information Processing: Measurement and Information Erasure”](https://doi.org/10.1103/PhysRevLett.102.250602), *Physical Review Letters* 102, 250602 (2009).
- D. Mandal and C. Jarzynski, [“Work and information processing in a solvable model of Maxwell's demon”](https://doi.org/10.1073/pnas.1204263109), *PNAS* 109, 11641 (2012): a demon whose moves read the demon and the bit together.
- J. M. R. Parrondo, J. M. Horowitz, and T. Sagawa, [“Thermodynamics of information”](https://doi.org/10.1038/nphys3230), *Nature Physics* 11, 131 (2015).
- Xyphers research, [“Intelligence as Physical Units”](intelligence-as-physical-units.md): the price of control, and the experiment that measures it.
- Xyphers research, [“When Futures Become Entropy”](causal-entropic-force-made-thermodynamic.md) and [“Committed Futures as Microstates”](committed-futures-as-microstates.md): the archipelago, its temperature, and the futures that are its microstates.

[Return to the research index](../README.md).
