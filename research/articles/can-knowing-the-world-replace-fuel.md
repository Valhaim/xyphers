---
title: "Can Knowing the World Replace Fuel?"
description: "Yes, one for one, and never better. An agent that knows its world can hold it with less fuel than any blind agent needs, by at most the information its actions use. Its memory pays for that information at the same rate, plus what the world's noise erases. Proved for a whole class of worlds and checked by exact computation."
type: "research-article"
status: "published"
maturity: "bounded-exact-result"
published: "2026-10-11"
revised: "2026-10-11"
website_path: "/research/can-knowing-the-world-replace-fuel/"
web_status: "live"
---

# Can Knowing the World Replace Fuel?

Yes, one for one, and never better. An agent that knows its world can hold that world away from equilibrium with less fuel than any blind agent needs. The saving is at most the information its actions use, and it approaches that limit as the actions become gentle and reversible. The knowledge is not free: the agent's memory pays for it at the same rate, plus whatever the world's noise erases. Knowledge moves the bill from the fuel to the memory, and counted in entropy it never brings the total below the floor. This is now a theorem, proved for a whole class of worlds and checked by exact computation.

## The question

Holding a world in a state it would not choose costs something. [How Much Open Future Can a Unit of Energy Buy?](how-much-open-future-can-a-unit-of-energy-buy.md) priced this with a floor found by Horowitz, Zhou and England. No agent that acts on the world can pay less than the rate at which the world, left alone, would forget the state it is held in.

That floor describes an agent that acts without knowing anything. Physics has long suspected that knowledge changes the price. Maxwell's demon sorts molecules by watching them, and Landauer's principle makes the demon's memory pay for what it learns. [Our formula sheet](https://xyphers.com/formulas/) listed the question as its one conjecture. Information should stand in for fuel at Landauer's exchange rate, and no better. The conjecture is now a theorem.

## A demon holding a world

The smallest example shows everything. The world has two states, 0 and 1, and noise flips it back and forth. Left alone it spends half its time in each. The agent wants it in state 0 three times in four.

The agent has a one-bit memory that copies the world, correctly nine times in ten. It has one action. When the memory reads 1, it pushes the world back to 0.

![Diagram of the demon world. Four states: the world in state 0 or 1, each with the memory reading 0 or 1. Noise flips the world in both memory states. The memory copies the world, correctly 9 times in 10. One action, taken only when the memory reads 1, pushes the world from 1 back to 0, holding it in state 0 three times in four.](figures/can-knowing-the-world-replace-fuel/demon-world.svg)

*The demon world. Noise flips the world; the memory copies it; one action, taken only when the memory reads 1, pushes the world back.*

A blind agent holding the same world needs fuel that produces at least 0.55 units of entropy per unit of time; that is the floor. The demon, with its actions made gentle enough, needs less than nothing. Its action runs the fuel line backwards, returning close to 0.55 units to the store while it holds the world exactly where it wants it.

The memory pays the difference. It pays 2.86 units to keep copying the world. Of those, 1.10 are the information the actions use, and the rest is what the world's noise erases from the memory between copies. Add everything up and the total is 2.31. That is the floor plus exactly what the noise erased.

![Horizontal bar chart of the demon world's ledger, in nats per unit time, at the limit of reversible actions. The floor, the least fuel any agent without knowledge needs: 0.55. The world's fuel with knowledge: −0.55, negative, so the actions return fuel. Information the actions use: 1.10. What the memory pays: 2.86. The total, 2.31, is the floor plus the 1.76 that the world's noise erases from the memory.](figures/can-knowing-the-world-replace-fuel/demon-ledger.svg)

*The demon's books, in units of entropy per unit time. The fuel line runs backwards; the memory pays for it, and the world's noise takes its cut.*

## The law

For every finite world in which the world's own noise does not read the agent's memory, three exact statements hold:

1. **The world's fuel** equals the floor, minus the information the actions use, plus the waste in the actions.
2. **The memory's bill** equals the information it gathers, plus the waste in the memory.
3. **The information gathered** equals the information the actions use, plus the information the noise erases.

Waste is never negative, and neither is what the noise erases. Everything else follows:

- **Knowledge can stand in for fuel**, by at most one unit of entropy for each unit of information the actions use. As the actions become reversible, the saving approaches that limit exactly; the exchange rate is one.
- **The memory pays at the same rate.** It pays at least what it gathers, and it gathers more than the actions use, because the noise keeps erasing.
- **Counted in entropy, the total never falls below the floor.** Knowledge moves part of the bill from the fuel to the memory. It does not make holding the world cheaper than the floor.

In units of energy, at the world's temperature α, one unit of entropy is worth α. A bit is ln 2 units of information, so each bit the actions use saves at most α ln 2 of the fuel's free energy. That is Landauer's rate, appearing on the fuel side of the books. The memory pays its share at the temperature of its own store. A memory kept colder than the world pays less per bit than the fuel saves per bit. It also pays for the bits the noise erases, so it comes out cheaper in total only when it is cold enough: in the demon world, colder than 5/13 of the world's temperature. Landauer's principle allows exactly that.

## Why it holds

The proof takes a page. It rests on three facts:

1. **Every move splits three ways.** The entropy a move produces is the sum of three parts: what it sends to a store, how it changes what is likely, and how it changes what the memory knows about the world.
2. **At steady state the middle parts cancel.** Nothing changes on average, so what is likely stays put.
3. **The world's noise does not read the memory.** So averaging over the memory turns the noise's entropy into exactly the floor, plus the information the noise erases.

Put the three together and the world's fuel equals the floor, minus the information its actions consume, plus their waste. The same argument on the memory's side gives its bill.

## The exchange rate is exactly one

An inequality is only as good as its sharpest case. We built agents whose actions grow faster and gentler, step by step, while holding the same world and using the same information. The fuel saved below the floor, per unit of information used, climbs toward one. In the demon world it goes from 0.82 to 0.99999977. In a three-island archipelago with a sensing agent it goes from 0.92 to 0.99999991. No smaller number could replace the one.

![Line chart of the fuel saved below the floor per nat of information the actions use, against the speed κ of reversible actions, for the demon world and a three-island archipelago with a sensing agent. Both rise toward exactly one. Values: demon world: κ = 1: 0.81546488, κ = 10: 0.97779464, κ = 100: 0.99773007, κ = 1000: 0.99977250, κ = 10000: 0.99997724, κ = 1000000: 0.99999977; three islands: κ = 1: 0.91537849, κ = 10: 0.99072683, κ = 100: 0.99906331, κ = 1000: 0.99990624, κ = 10000: 0.99999062, κ = 1000000: 0.99999991.](figures/can-knowing-the-world-replace-fuel/exchange-rate.svg)

*Fuel saved per unit of information used, as the actions approach reversibility. Both worlds climb to exactly one, never past it.*

## Below the floor takes both halves of agency

The theorem also says what an agent needs in order to beat the floor at all. It needs a memory that reads the world, and actions that read the memory. Take away either and the fuel stays at or above the floor.

- A memory blind to the world can gather no information about it.
- Knowledge the actions ignore is, for the fuel, as good as none.

![Two-by-two grid. Rows: whether the agent's memory reads the world. Columns: whether its actions read the memory. Only when both are true can the world's fuel fall below the floor, and then by at most the information the actions use. In the other three cells the floor holds: knowledge left unused, a memory blind to the world, or no knowledge at all.](figures/can-knowing-the-world-replace-fuel/both-halves.svg)

*Knowing and using. Only the agent that does both can go below the floor.*

## Correcting our own record

The formula sheet previously said that three earlier experiments had confirmed the law's blind half. That was a misreading. In those worlds the agent's plan is part of the world itself: the weather never wears a bridge the plan uses. The floor was computed with the plan inside the world. Those experiments are the case with an empty memory, where the law reduces to formula III, and they did not test the information term. The sheet now says so.

## What it means

In the ideal limit, knowledge is a currency at par: it buys fuel one for one and costs at least as much to gather. So why would an agent ever want it? Because real agents are not ideal. The agents in our earlier experiments spend fuel at fixed strengths and pay two to three times the floor. Most of that is waste in their actions. The theorem says exactly when knowledge pays for such an agent: when it removes more waste from the actions than gathering it costs, including what the noise erases. Knowledge pays when it lets fuel go only where it is needed.

I believe this is the right shape for a thermodynamics of agency. An agent's purpose costs energy, its knowledge saves energy at a fixed exchange rate, and its memory pays the same rate to know. That fixes one more price. The next experiment puts real sensing agents, with fuel of fixed strength, against blind ones, with this exact condition written down in advance.

## How we know

1. **The theorem and its proof**, with the exact class of worlds it covers, were committed before any review.
2. **Exact verification.** A program checked every identity exactly in 500 random worlds, the demon world, and two archipelagos, with no rounding anywhere. It reduces every logarithm to a basis of numbers with no common factor, where an identity holds only if every coefficient is exactly zero. Every inequality is decided by exact signs or by guaranteed error bounds. The run takes nine seconds.
3. **A blind referee** read only the proof and rebuilt it by hand and with exact arithmetic. The referee found the mathematics correct, and three things to fix:
   - a printing error in one formula;
   - a missing condition in the sharpness proof;
   - an overstated claim of novelty. One bound we had called new was already in Horowitz and England's 2017 paper.

   All three are fixed, and the novelty claims are narrowed to what is new. On a second reading the referee accepted the revised proof.
4. **An independent program**, written from the theorem's statement alone, uses its own solver and factors every number into primes. Its first run caught the same printing error, and agreed on every other one of more than 13,000 checks.

## Read, inspect, reproduce

**Read:** [the evidence guide](../evidence/information-law/README.txt) explains every file, and [the theorem](../evidence/information-law/THEOREM.md) gives the full statement and proof, the review record, and what is borrowed and what is new.

**Run:** [the verification program](../evidence/information-law/verify.py) and its [library](../evidence/information-law/infolaw.py) need Python 3 with python-flint. Following the [run guide](../evidence/information-law/RUN.txt):

```sh
python3 verify.py > my-verification.txt
diff verification.txt my-verification.txt
```

The run takes about nine seconds and ends with `OVERALL PASS`. The [independent check](../evidence/information-law/independent/check.py) runs in about thirty-five seconds. The [whole packet](../evidence/information-law/) sits beside this article.

## Research trail

- J. M. Horowitz, K. Zhou, and J. L. England, [“Minimum energetic cost to maintain a target nonequilibrium state”](https://doi.org/10.1103/PhysRevE.95.042102), *Physical Review E* 95, 042102 (2017): the floor.
- J. M. Horowitz and J. L. England, [“Information-theoretic bound on the entropy production to maintain a classical nonequilibrium distribution using ancillary control”](https://doi.org/10.3390/e19070333), *Entropy* 19, 333 (2017): the total bound with an auxiliary controller, and fast reversible control.
- J. M. Horowitz and M. Esposito, [“Thermodynamics with continuous information flow”](https://doi.org/10.1103/PhysRevX.4.031015), *Physical Review X* 4, 031015 (2014): the second law with information flow.
- A. C. Barato, D. Hartich, and U. Seifert, [“Efficiency of cellular information processing”](https://doi.org/10.1088/1367-2630/16/10/103024), *New Journal of Physics* 16, 103024 (2014): what a sensor must pay to learn.
- R. Landauer, [“Irreversibility and heat generation in the computing process”](https://doi.org/10.1147/rd.53.0183), *IBM Journal of Research and Development* 5, 183 (1961): the price of a bit.

[Return to the research index](../README.md).
