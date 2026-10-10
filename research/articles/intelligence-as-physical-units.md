---
title: "Intelligence as Physical Units"
description: "How control over the future can be measured in bits, and why a real temperature gives every bit a weight in energy."
type: "research-article"
status: "published"
maturity: "open-frontier"
published: "2026-08-02"
revised: "2026-10-10"
website_path: "/research/intelligence-as-physical-units/"
web_status: "live"
---

# Intelligence as Physical Units

A furnace can spend more energy than a brain. A library can hold more information than either.

Neither fact tells us how intelligent the furnace, the library, or the brain is.

Something is missing: **consequence**.

Imagine a switch connected to a lamp. The switch carries one simple distinction, up or down, and that distinction changes what happens next. Now disconnect the switch. It can still be moved. Its two positions still exist. But the distinction has lost its consequence.

That missing ingredient can be measured. One component of intelligence can be given a physical unit: **control over future outcomes, measured in bits**. One bit means an action can reliably choose between two futures. Two bits, among four. Three bits, among eight.

Bits alone are bookkeeping. What makes them physical is temperature. In a digital world with a real temperature, every bit of control is worth a definite amount of energy. Place control inside the digital world we built, where the rooms double with every energy packet, and **one bit of control is worth exactly one packet**.

The derivation rests on physics almost a century old. The experiment that places a controlling Praxion inside that world is the next one we build. When it holds, intelligence stops being only a metaphor. It becomes something a system has an amount of, at a cost we can calculate. I believe this is an extraordinary leap in how we understand intelligence and life.

## Eight doors: possibility is not control

Imagine a Praxion, the part of a Xypher that perceives and acts, facing eight doors. Behind them lie eight different futures.

In the first version of the experiment, the Praxion has eight commands. Each command reliably opens a different door. The same command always produces the same result. By choosing a command, the Praxion can select one future from eight.

That is three bits of control, because three yes-or-no choices can distinguish eight outcomes.

Now keep all eight doors, but disconnect the commands. A gust of wind opens one at random, with every door equally likely. The future still has eight possibilities. An observer still needs three bits to report which door opened. But the Praxion has no control over the result. Changing its command changes nothing.

The number of possible futures is the same. The causal power of action is completely different.

Between perfect command and pure wind lies noise. One command may usually open the red door, sometimes the blue, and rarely the green. We can repeat each command and measure how much knowing the command tells us about the door that eventually opens.

Information theory calls that shared knowledge **mutual information**. Try the commands in the mixture that reveals the most, and the result is the capacity of the channel from action to future. In agent research, this quantity is called **empowerment**.

Let **A** stand for the command, or sequence of commands, given to the Praxion. Let **Y<sub>τ</sub>** stand for the future outcome after a chosen number of steps **τ**, and let **z** stand for the same starting state in every trial. The symbol **q** describes how often each command is tried.

One rule keeps the measure clean: if the world keeps a copy of the command, **Y<sub>τ</sub>** leaves that copy out. A system that merely remembers which command it received has not controlled anything.

**CAUSAL-CONTROL CAPACITY**

$$
I_\tau^{\mathrm{ctrl}}(z) = \max_q I_q(A; Y_\tau \mid z) \quad \text{bits}
$$

**I(A; Y<sub>τ</sub>)** asks how much knowing the command tells us about the future that occurs. Maximizing over **q** finds the strongest control the commands make available.

Read it as: *From the same starting point, how many future choices can action actually command?*

For the eight reliable doors, the answer is three bits. For eight wind-blown doors, it is zero. For noisy doors, it lies between them.

This is a measurable coordinate of intelligence: not how many futures exist, but how much influence action has over which future becomes real.

Measured this way, control is still information: a count of distinctions. To weigh it, we need a temperature.

## Temperature gives the bit a weight

The [Core Thesis](core-thesis.md) built temperature from a picture of floors and rooms. An energy store beside the system, its **reservoir**, is a building. Each floor holds one more packet of energy than the floor below. Each room on a floor is one exact way the store can be arranged at that energy. Temperature is the exchange rate between them: how much energy it takes to multiply the rooms, counted in nats.

Rooms are counted with the natural logarithm, **ln**, in units called **nats**. One bit is **ln 2** nats. Let **Ω<sub>R</sub>(E<sub>R</sub>)** be the number of rooms when the reservoir holds energy **E<sub>R</sub>**, let **λ** be one packet of energy, and let **α** be the temperature, in energy per nat.

**THE STATE-COUNT THERMOMETER**

$$
\frac{1}{\alpha} = \frac{\ln \Omega_R(E_R + \lambda) - \ln \Omega_R(E_R)}{\lambda}
$$

*Temperature is the energy price of multiplying the ways the reservoir can exist.*

Read it as: *When one packet of energy enters, how much do the reservoir's possible arrangements multiply?*

This is the standard statistical-physics definition of temperature, written for discrete packets. In ordinary units, **α** is Boltzmann's constant times the temperature in kelvin. A digital world counts in its own energy packets instead of joules, just as a model of an orbit can count kilometres instead of metres.

Energy chooses the floor. Entropy counts the rooms. Temperature is the exchange rate between them.

## A digital world with a real temperature

Digital systems are not normally thermodynamic: nothing inside a program has a temperature of its own. We built one that does.

Picture a body that can sit in three positions joined in a line:

> **0 ↔ 1 ↔ 2**

Each step to the right moves one energy packet from the reservoir into the body; each step to the left returns it. Body and reservoir always hold two packets between them. The body has one internal arrangement at position 0, four at position 1, and four at position 2. The reservoir has four arrangements when it holds both packets, two when it holds one, and one when it is empty.

Multiply body and reservoir at each position, and the whole world has **4, 8, and 4** arrangements: sixteen in total. A short program lists every arrangement and every allowed move. Nothing is sampled, and nothing is estimated.

Its temperature can be read twice. The first reading counts: the reservoir's rooms double with each packet, so **α = λ / ln 2**. The second watches the world move, with every allowed move equally likely. From any arrangement at position 1 there are four ways to step forward; from any arrangement at position 2 there are eight ways to step back. The body has four arrangements on both sides, so that factor of two is the price of the packet alone. A factor of two is ln 2 nats, and it gives the same **α = λ / ln 2**.

The two thermometers agree exactly. Every packet is accounted for along every move, the world settles into the predicted balance, and two bodies prepared at the same temperature trade packets with zero net flow on average. The temperature is a property of this world, not a label put on it.

**Read:** [A Purely Digital Thermodynamic System](proving-true-thermodynamic-graph-systems.md) builds the construction step by step.

**Inspect:** [the exact recorded result and its complete output](../code/xypher-thermodynamics-proof/evidence/RESULT.md).

**Run:** [the standalone verifier source](../code/xypher-thermodynamics-proof/). It has no external Rust dependencies. [Read its quick-start guide](../code/xypher-thermodynamics-proof/README.md).

With Rust installed, run these commands inside that folder:

```text
cargo test
cargo run --quiet
```

We now have both instruments: a measure of control in bits, and a world with a real temperature. What joins them?

## When the doors open inside the thermometer's world

Physics already knows what one bit is worth.

In 1929, Leo Szilard imagined a box holding a single molecule, in contact with its surroundings at temperature **T**. A being who knows which half of the box the molecule is in can let it push a piston and draw **k<sub>B</sub>T ln 2** of work from the surrounding heat. A being who does not know cannot. That knowledge is itself physical: a correlation between the being's memory and the molecule. One bit of it is worth that much energy at the system's temperature. Szilard titled his paper *On the Decrease of Entropy in a Thermodynamic System by the Intervention of Intelligent Beings*.

Control is the same kind of correlation, pointed forward: a correlation between what the Praxion commands and what then happens. When command and outcome are both physical parts of one world with a real temperature, that correlation is worth energy in exactly Szilard's way.

That “when” is the whole experiment. The thermometer prices the arrangements of one world. For it to price control, the doors must open inside the same world whose reservoir sets the temperature. The command cannot remain a label outside it. It must be kept in a physical command register until the future is read, and that register, the Praxion, the graph, the reservoir, and any outside source of work must share one energy account.

Two views of the same event keep the account exact. **Y<sub>τ</sub>** is the outcome whose control we measure; it leaves out the command register. **Z<sub>τ</sub>** is the complete physical state used for the energy account; it includes the register, the outcome, the graph, the reservoir, and the Praxion. The first stops stored labels from passing as control. The second makes the register, and whatever it cost to write, part of the account the thermometers check.

Now picture a deck of cards, one card for each trial: the command on the front, the future that followed on the back. In the real deck, fronts and backs belong together. Shuffle only the backs, and you have a second deck with the same commands and the same futures, each appearing just as often as before, but with no relationship between them.

There are far more ways to pair fronts and backs with no relationship than with the real one. Per card, the shuffled deck's entropy is higher by exactly the mutual information between command and future. Suppose linking a command to its future does not change the energy of either. Then both decks hold the same energy, and the real deck holds it with less entropy. At a fixed temperature, that difference is **free energy**: energy available to do work. Per card, its size is **α** multiplied by the mutual information in nats.

Try the commands in the mixture that reveals the full control capacity, and call the resulting free energy **V<sub>τ</sub><sup>ctrl</sup>(z)**.

**THE CORRELATION FREE ENERGY OF CONTROL**

$$
V_\tau^{\mathrm{ctrl}}(z) = \alpha \ln 2 \cdot I_\tau^{\mathrm{ctrl}}(z)
$$

*A physical correlation between action and future carries free energy in proportion to its control information.*

**I<sub>τ</sub><sup>ctrl</sup>(z)** is the Praxion's control capacity from starting state **z**, in bits. **ln 2** converts bits into nats. **α** converts nats into the energy units of the Thermodynamic Harness. **V<sub>τ</sub><sup>ctrl</sup>(z)** is the free-energy difference, per trial, between the linked register–outcome state and its shuffled counterpart.

Read it as: *How much control reaches the future, and how much energy is that correlation worth at this temperature?*

In ordinary units, this is Szilard's **k<sub>B</sub>T ln 2** for every bit. Put the command register inside our digital world, keep **α = λ / ln 2**, and the ln 2 cancels: **each bit of control is worth exactly one energy packet**.

The equation values the correlation present when the future is read. Preparing the command, and any work later drawn from the correlation, are separate entries in the same ledger.

Here is the punch: control does not require a new mystical substance to enter physics. Once command and future are physical coordinates, the information joining them already has a thermodynamic form.

The multiplication is simple. The scientific work lies in building the object that earns it. The temperature and the control must be measured independently, on the same complete system, using the same outcomes with the same probabilities.

## Where THAIM fits

The [Core Thesis](core-thesis.md) asks what happens when a Praxion changes the graph itself and opens more future possibility. The Thermodynamic Harness records positive expansion with a receipt called **THAIM**.

THAIM and empowerment answer different questions.

THAIM asks: *How much new future possibility did this action open?*

Empowerment asks: *How much control can action exert within the futures now available?*

A graph can gain possibilities that no Praxion can reliably select. A Praxion can also gain better control among states that already existed. Opening futures raises the ceiling on control: eight doors allow at most three bits. Empowerment measures how much of that ceiling action fills. At one temperature, both are priced in the same energy.

## The next experiment

The thermometer exists today. What we build next is the control instrument matched to it: the same complete world, the same outcomes, the same probabilities.

1. Add a physical command register, the Praxion, and any memory it uses to the exact list of states. If something outside supplies the command, record what that costs as work. [When Does Information Become State in a Xypher?](when-does-information-become-state-in-a-xypher.md) explains when a stored value belongs to the world's state.
2. From one fixed starting state, list exactly how likely each outcome **Y<sub>τ</sub>** is after every command. Separately, list every complete state **Z<sub>τ</sub>** and its probability.
3. Find the mixture of commands that carries the most information into the future. That is the control capacity, in bits.
4. Build the shuffled deck: the same commands and the same outcomes, with each command register independently paired with an outcome. The possibilities stay and the control is gone by construction. The check is that the average energy stays the same too, so that the free energy lost is exactly **α ln 2** per bit.
5. Read the temperature again on the enlarged world, over every complete state **Z<sub>τ</sub>**. Both thermometers must still agree, and every move and command must close its energy account.

If every command produces the same spread of futures, the graph has possibility without control. If two commands are used equally and reliably select two different futures, the channel carries one bit. If those outcomes live inside the same fully accounted world to which **α** applies, one Xypher has joined thermodynamics and causal control in a single executable object.

The test can fail. If the thermometers disagree once the Praxion is inside, or shuffling the pairs changes the average energy, the claim is wrong. We know what to build, what to measure, and what result would prove us wrong.

The experiment has now run. [What Is a Bit of Foresight Worth?](what-is-a-bit-of-foresight-worth.md) builds the matched instrument in the archipelago where committed futures are microstates. The temperature reads the same with the instrument inside, the shuffled deck loses exactly **α ln 2** per bit at equal energy, and one bit of foresight trades evenly for one packet of timber.

## A physical coordinate of intelligence

Said in ordinary language:

A system has one bit of control when its action can reliably select between two distinguishable futures. Information theory measures that control even when the world is noisy. Statistical physics prices distinguishable states in energy. A matched Xypher gives both measurements one physical home.

The digital thermometer is built and exactly reproducible, and the matched control instrument has passed. Driving that instrument to lift real loads is the frontier immediately in front of us.

A mind can express many capacities. This places one of them, the power to make a difference to its own future, on a physical scale. That is where a physics of intelligence begins, and with it a new way of asking what it means to be alive.

A furnace has energy. A library has information. A Praxion has consequence.

The unit is a bit of control. The temperature gives it weight.

## Research trail

- Claude E. Shannon, [“A Mathematical Theory of Communication”](https://doi.org/10.1002/j.1538-7305.1948.tb01338.x), establishes the bit, entropy, mutual information, and channel capacity used to measure the action-to-future link.
- Leo Szilard, [“Über die Entropieverminderung in einem thermodynamischen System bei Eingriffen intelligenter Wesen”](https://doi.org/10.1007/BF01341281), *Zeitschrift für Physik* 53, 840–856 (1929), shows that one bit of knowledge about a system at temperature T is worth k<sub>B</sub>T ln 2 of work.
- Alexander S. Klyubin, Daniel Polani, and Chrystopher L. Nehaniv, [“Empowerment: A Universal Agent-Centric Measure of Control”](https://doi.org/10.1109/CEC.2005.1554676), develops action-to-future channel capacity as an agent's potential control.
- Takahiro Sagawa and Masahito Ueda, [“Fluctuation Theorem with Information Exchange: Role of Correlations in Stochastic Thermodynamics”](https://doi.org/10.1103/PhysRevLett.109.180602), establishes how mutual information enters the thermodynamic account of physical correlations.
- Xyphers research, [“A Purely Digital Thermodynamic System”](proving-true-thermodynamic-graph-systems.md), builds the sixteen-arrangement world and its two thermometers.
- Xyphers research, [the exact operational-thermodynamics result](../code/xypher-thermodynamics-proof/evidence/RESULT.md), contains the construction, checks, output, and scientific boundary.
- Xyphers research, [the version-pinned standalone source](../code/xypher-thermodynamics-proof/), contains the dependency-free verifier used by the downloadable archive.

[Return to the research index](../README.md).
