---
title: "Can THAIM Train an Agent?"
description: "Yes, in one form. Trained only on the net THAIM its own moves create, a small model learned exactly when a bridge is worth its timber, acted only where agency pays, and came 1.6 to 1.9 times closer to the thermodynamic floor than hand-written aim. Paid in THAIM receipts instead, it bought churn. One prediction failed."
type: "research-article"
status: "published"
maturity: "bounded-exact-result"
published: "2026-10-13"
revised: "2026-10-13"
website_path: "/research/can-thaim-train-an-agent/"
web_status: "live"
---

# Can THAIM Train an Agent?

Yes, in one form, and the form matters. A small model was trained on nothing but the net THAIM its own moves create: the futures it keeps open, valued at the world's temperature, minus the fuel it spends. It learned exactly when a bridge is worth its timber. It acted only in worlds where acting pays, and it came 1.6 to 1.9 times closer to the thermodynamic floor than an agent we designed by hand. Paid in THAIM receipts instead, the same model bought churn. Booked for the whole world, THAIM taught it nothing. One prediction failed, and the failure shows where THAIM's price parts ways with an older measure of aim.

## Why ask

The [Core Thesis](core-thesis.md) introduced THAIM as the receipt for future opened: an action that opens new possibilities is paid in proportion to them, at the world's temperature. In a Xypher economy THAIM is money. The obvious next question is whether THAIM can also be the reward an artificial agent learns from. If it can, the agent's goal is not a number someone chose. It is the same quantity the world's thermodynamics already counts.

There is a reason to expect it might work. In the mathematics of reinforcement learning that rewards keeping options open, the value of a position, with nothing else at stake, is the logarithm of how many futures it leaves open. That is exactly what THAIM measures. What THAIM adds is a temperature that is not a tuning knob: it belongs to the world.

## Three ways to pay in THAIM

The agent lives in a small archipelago. Weather keeps building and wearing bridges, drawing on a cold store of timber. A traveller wanders across whatever bridges stand. The agent can spend fuel from a hotter store to build bridges, and every build can be undone, returning its packet. What the agent wants is open future for the traveller.

THAIM can be paid to it in three ways:

1. **As receipts.** Every build that opens futures earns a receipt. Undoing a build costs nothing.
2. **As its own net flow.** Every move the agent makes is booked, gains and losses alike: futures opened, minus futures closed, minus the fuel it spends.
3. **As the world's net flow.** Every change in futures is booked, whoever caused it.

Two of these fail on paper. The world's net flow is always zero once things settle: in a steady world, futures are opened exactly as fast as they are forgotten. An agent paid that way learns only to stop spending. Receipts reward every build and never charge for the undoing, so building and dismantling the same bridge earns again and again. The Core Thesis warned about exactly this for THAIM as money: the books must charge every loss as surely as they reward every gain.

That leaves the agent's own net flow. Booked that way, a build pays exactly when the bridge multiplies the traveller's futures by more than the cold store multiplies its own arrangements per packet. That is the world's own exchange rate between futures and timber, the same ratio that governs which bridges a [committed traveller keeps](committed-futures-as-microstates.md). We predicted, before any of the test worlds was computed, that an agent trained on its own net flow would learn exactly that rule.

## The rule it learned

It did, in every world. Trained only on its own net THAIM, the agent fuelled every build that multiplies the traveller's futures by more than the cold store's factor, and none that does not. It found that rule without being told it, in six worlds with two horizons and four temperatures. It found it again from a different starting point. Where the hot store and the traveller's speed both changed but the world's store did not, it learned the same rule: the price of futures is set by the world, not by the fuel.

![Two strip charts of the share of possible builds each agent fuels, against how much the build multiplies the traveller's futures, on a logarithmic axis. Upper panel: cold store ×5, hot store ×2, three steps ahead. Lower panel: cold store ×9, hot store ×2. The agent trained on its own net THAIM flow fuels every build above the THAIM price (the cold store's factor) and none below it: cold store ×5: 96 of 768 builds, cold store ×9: 72 of 768 builds. The agent trained on THAIM receipts fuels builds down to its much lower receipt price: cold store ×5: 348 of 768, cold store ×9: 96 of 768.](figures/can-thaim-train-an-agent/the-rule.svg)

*Each dot is a group of possible builds with the same effect on the traveller's futures. The agent paid its own net THAIM switches on exactly at the THAIM price; the agent paid receipts switches on far below it.*

In two colder worlds, no single bridge is worth its timber at the world's price. There the THAIM-trained agent did nothing at all, spending a few hundred-thousandths of the fuel that hand-written aim spends. This is the theory's claim made concrete: agency should act only where acting pays, and the world's temperature decides where that is.

## What receipts buy

The agent paid in receipts learned a different price, far too cheap. It fuelled builds that open futures but are worth less than their timber, and it kept undoing and redoing them. In every world where some build pays, it spent more fuel and lost net THAIM, while the net-flow agent gained. In one cold world where the net-flow agent rightly did nothing, the receipt agent kept spending.

![Grouped bar chart of the net THAIM flow, in energy per unit time, for three agents in four worlds where some builds pay. Values: ×5 store: own net THAIM 0.435, hand-written aim 0.003, THAIM receipts -0.222; ×5, hotter fuel: own net THAIM 0.257, hand-written aim 0.009, THAIM receipts -0.494; ×9 store: own net THAIM 0.095, hand-written aim -0.329, THAIM receipts -0.041; ×20, four steps: own net THAIM 0.070, hand-written aim -0.313, THAIM receipts -0.237. The agent trained on its own net THAIM flow earns the most in every world; the agent trained on THAIM receipts loses in every world.](figures/can-thaim-train-an-agent/what-it-buys.svg)

*Net THAIM flow in four worlds. Paid its own net THAIM, the agent profits everywhere; paid receipts, it loses everywhere. Hand-written aim breaks even at best.*

Receipts are a good record of what an action opened. They are a bad reward. That settles a design question for THAIM as money as well as for THAIM as a training signal: the receipt is the record, and the net flow is what must be paid.

## Closer to the floor

Every agent that holds a world away from equilibrium pays at least the [floor](how-much-open-future-can-a-unit-of-energy-buy.md): the rate at which the world, left alone, would forget the state it is held in. The THAIM-trained agent came much closer to that floor than either hand-written policy. In the warmest worlds it paid 1.3 to 1.4 times the floor, where hand-written aim paid 2.5 to 2.7 times. Fuel spent only where it pays is fuel not wasted.

![Grouped bar chart of thermodynamic efficiency, the Horowitz-Zhou-England floor divided by the entropy the agent produces, for three agents in four worlds. Values: ×5 store: own net THAIM 0.71, hand-written aim 0.37, fuel everywhere 0.39; ×5, hotter fuel: own net THAIM 0.75, hand-written aim 0.40, fuel everywhere 0.44; ×9 store: own net THAIM 0.52, hand-written aim 0.32, fuel everywhere 0.33; ×20, four steps: own net THAIM 0.46, hand-written aim 0.29, fuel everywhere 0.29. The agent trained on its own net THAIM flow comes 1.6 to 1.9 times closer to the floor than hand-written aim.](figures/can-thaim-train-an-agent/closer-to-the-floor.svg)

*Efficiency against the thermodynamic floor. One means the agent produces only the entropy the physics demands.*

## The prediction that failed

We also predicted that the THAIM-trained agent would beat hand-written aim on an older yardstick: the open future it holds per unit of fuel, measured against spreading the same fuel evenly. It did in three of four worlds. In the first, it did not: hand-written aim scored 1.62, the THAIM agent 1.59.

The two yardsticks value futures differently. THAIM prices the logarithm of the number of futures, at the world's temperature. The older yardstick counts futures one by one and rewards whatever buys the most of them per unit of fuel. Hand-written aim fuels every bridge at the traveller gently, including some worth less than their timber, and in that world it held slightly more futures per packet on the older count. An agent told to value futures more cheaply than the world does scored 1.97 on the same yardstick. THAIM's price is an exchange rate set by thermodynamics, not a tool for maximising the older measure, and in one world the two disagree.

## A map is worth what the rule needs to see

Some agents saw only the bridges at the traveller's island; others saw the whole archipelago. In the worlds where the profitable rule can be read off the traveller's own island, the two did equally well. In the worlds where it cannot, because whether a build pays depends on bridges the traveller cannot see, the agent with only a local view did nothing, and the agent with the map earned the full value. How much a map is worth followed exactly from what the rule needs to see.

## What did not learn

An agent that sees nothing cannot tell a paying build from a wasteful one, and it rightly did nothing. More instructively, a small neural network trained on the same reward mostly collapsed to doing nothing too. Only a few bridges pay, so the gradient tells the network, early on, to switch everything off, and it never recovers the few that matter. The lookup-table model learned the rule everywhere. Training a network that does not fall into this trap is the next engineering step.

## How we know

1. **Theory before computation.** The three ways of paying, the rule, and what receipts should do were derived before any test world was computed. The test worlds were chosen so that no build sits exactly at the price.
2. **Preregistration with blind review.** The worlds, five claims, thirteen acceptance tests and six deliberately broken versions were frozen after four rounds of independent review. The reviewer found a real weakness: the training budget, not THAIM, could have decided one claim. The training protocol was then calibrated on separate practice worlds, and every number looked at during that calibration is published.
3. **Three reviews of the exact code**, each approving the same version without running it on the test worlds.
4. **One run**, twenty-four minutes. Every trained policy was frozen as an exact table of fractions and judged in exact arithmetic. Every broken version failed exactly where it was predicted to.
5. **An independent check**, written from the preregistration alone with its own exact solver, recomputed every result from the frozen tables and agreed on all 907 comparisons.

## What it means

THAIM can train an agent, if it is paid as the net flow of the agent's own moves. In that form it needs no tuned constant. It teaches the world's exchange rate between futures and energy, it keeps agents idle where agency does not pay, and it buys open future close to the cost physics demands. Paid as receipts, THAIM is a trap.

I believe this is the beginning of rewards that cannot be argued with. A reward that is the system's own thermodynamics leaves nothing arbitrary to exploit, only the physics. The next steps are a learning network that does not collapse, a memory whose knowledge is priced by [the information law](can-knowing-the-world-replace-fuel.md), and several agents trained on their own net THAIM in one world, to test where greed and generosity coincide.

## Read, inspect, reproduce

**Read:** [the evidence guide](../evidence/thaim-reward/README.txt) explains every file. [The preregistration](../evidence/thaim-reward/BOUNDARY.md) states the worlds, the three ways of paying, the claims, the tests, the broken versions, and every calibration number. [The result record](../evidence/thaim-reward/RESULT.md) gives the verdicts, the review history, and what was learned.

**Inspect:** [the complete report](../evidence/thaim-reward/report.txt), and the 78 frozen agents, one table of exact fractions each, such as [the agent trained on its own net THAIM in the first world](../evidence/thaim-reward/tables/W1/NET-full.txt). Each table's fingerprint is printed in the report.

**Run:** the [apparatus](../evidence/thaim-reward/source/README.md) needs Python 3 with numpy, scipy and python-flint. Following the [run guide](../evidence/thaim-reward/RUN.txt):

```sh
python tests.py
python run.py > my-report.txt
```

The run takes about twenty-five minutes and ends with `OVERALL PASS`. The [independent check](../evidence/thaim-reward/independent/check.py) recomputes every result from the frozen tables. The [whole packet](../evidence/thaim-reward/) sits beside this article.

## Research trail

- J. M. Horowitz, K. Zhou, and J. L. England, [“Minimum energetic cost to maintain a target nonequilibrium state”](https://doi.org/10.1103/PhysRevE.95.042102), *Physical Review E* 95, 042102 (2017): the floor.
- T. Haarnoja, A. Zhou, P. Abbeel, and S. Levine, [“Soft Actor-Critic”](https://proceedings.mlr.press/v80/haarnoja18b.html), *ICML* (2018): reinforcement learning that rewards keeping options open.
- A. Y. Ng, D. Harada, and S. Russell, [“Policy invariance under reward transformations”](https://ai.stanford.edu/~ang/papers/shaping-icml99.pdf), *ICML* (1999): why rewards built from differences of a potential cancel out.
- A. M. Turner, L. Smith, R. Shah, A. Critch, and P. Tadepalli, [“Optimal policies tend to seek power”](https://proceedings.neurips.cc/paper/2021/hash/c26820b8a4c1b3c2aa868d6d57e14a79-Abstract.html), *NeurIPS* (2021): why rewarding open futures needs care.
- K. Takahashi and Y. Hayashi, [“Thermodynamic limits of physical intelligence”](https://doi.org/10.1007/978-3-032-33195-3_24), *AGI 2026*: empowerment per joule, the closest earlier measure.

[Return to the research index](../README.md).
