---
title: "Intelligence as Physical Units"
description: "How control over future outcomes can be measured in bits—and joined to a Xypher's thermodynamics."
type: "research-article"
status: "published"
maturity: "open-frontier"
published: "2026-08-02"
revised: "2026-08-02"
website_path: "/research/intelligence-as-physical-units/"
web_status: "deployment-pending"
---

# Intelligence as Physical Units

A furnace can spend more energy than a brain. A library can hold more information than either.

Neither fact tells us how intelligent the furnace, the library, or the brain is.

Something is missing: **consequence**.

Imagine a switch connected to a lamp. The switch carries one simple distinction—up or down—and that distinction changes what happens next. Now disconnect the switch. It can still be moved. Its two positions still exist. But the distinction has lost its consequence.

One component of intelligence can be given a physical unit: **control over future outcomes, measured in bits**.

In the cleanest possible case, one bit of control means that an action can reliably select between two different futures. Two bits mean four selectable futures. Three bits mean eight. When the world is noisy, the number may fall between whole bits, according to how much of the action survives into the outcome.

This is one elementary capacity beneath purposeful behaviour: *how much difference can an action make to what happens next?*

## A Xypher gives the question a body

Control cannot exist in the abstract. Something must have possible states. Something must act. Something must account for the physical meaning of the change.

Picture a small connected world: points joined by lines, with different routes available from different places. This is the **Graph Substrate**—the part of a Xypher that can exist, connect, and change.

Now give that world an energy account. Every complete arrangement has an energy, and the account keeps track of how energy moves when the arrangement changes. This is the **Thermodynamic Harness**.

Finally, place something inside the world that can perceive an available move and act on it. This is a **Praxion**. A base Praxion may do only one simple thing. Richer Praxions may remember, learn, plan, or share the same substrate with other Praxions.

Together, the Graph Substrate, Thermodynamic Harness, and Praxion Layer form a [**Xypher**](../../README.md): a thermodynamic graph system with something inside it that can act.

The substrate supplies possible futures. The harness gives those possibilities a physical account. The Praxion gives them consequence.

## Temperature is the price of possibility

Imagine a building with numbered floors. This building is an energy store beside the graph—a **reservoir** that can give the graph one packet of energy and receive it back.

The floor tells us how much energy the reservoir holds. Moving up one floor adds one packet. But each floor may contain a different number of rooms. The ground floor may have one room, the next two, the next four, and the next eight.

The floor is only the energy we can see from outside. A room is one exact way the reservoir can be arranged while holding that energy. Physicists call such an exact arrangement a **state**.

If an energy packet doubles the number of available rooms, and the next packet doubles them again, the possibilities are multiplying. A logarithm turns those repeated multiplications into equal steps. The natural logarithm, written **ln**, counts those steps in information units called **nats**.

Let **Ω<sub>R</sub>**, pronounced “omega R,” mean the number of reservoir states available at one energy. The small **R** reminds us which building we are counting. Let **λ** mean one packet of energy. The thermal energy scale, written **α**, comes from asking how much the logarithmic room count changes when that packet arrives.

**THE STATE-COUNT THERMOMETER**

$$
\frac{1}{\alpha} = \frac{\ln \Omega_R(E_R + \lambda) - \ln \Omega_R(E_R)}{\lambda}
$$

*Temperature is the energy price of multiplying the number of ways the reservoir can exist.*

**Ω<sub>R</sub>(E<sub>R</sub>)** is the number of ways the reservoir can be arranged while holding energy **E<sub>R</sub>**. **λ** is one energy packet. **α** is the resulting energy price per nat of reservoir-state multiplicity.

Read it as: *When one packet of energy enters, how much do the reservoir's possible arrangements multiply?*

This is the discrete-reservoir form of the standard statistical-physics relation. When total energy is fixed, temperature is read from the change in entropy as energy moves between a system and its reservoir. Physicists call this the **microcanonical** description. In ordinary physical units, **α** is Boltzmann's constant multiplied by temperature in kelvin. A digital construction can use a declared model-energy packet instead of a joule, just as a computer model of an orbit can use kilometres instead of metres.

When the graph and reservoir settle into thermal balance, this reservoir relation fixes the temperature scale shared by them.

Energy chooses the floor. Entropy counts the rooms. Temperature is the exchange rate between them.

## A small exact digital thermometer

Has a digital graph ever passed that test? Yes.

Picture three positions joined in a line:

> **0 ↔ 1 ↔ 2**

The graph has three visible positions. Position 0 has model energy 0, position 1 has energy 1, and position 2 has energy 2. The graph and the reservoir from our building picture always hold two energy packets between them. When the graph moves up, the reservoir moves down. When the graph moves down, the reservoir receives the packet back.

Each visible position hides several distinguishable internal graph arrangements: one at position 0, four at position 1, and four at position 2. The reservoir has four internal arrangements when it holds two packets, two when it holds one, and one when it is empty.

Count the complete arrangements of graph and reservoir together:

- At position 0, there are four.
- At position 1, there are eight.
- At position 2, there are four.

That makes sixteen complete states in total. A “state” here means one whole graph-and-reservoir arrangement. Because there are only sixteen, a short program can list every state and every permitted transition. There is no need to estimate the answer from a sample or wait for a long simulation to settle down.

The construction then reads temperature in two independent ways.

The first thermometer counts how the reservoir's internal possibilities multiply when it gains energy. Every packet doubles them.

The second watches movement. For each permitted move, the program counts all the microscopic ways that move and its exact reverse can occur. Their traffic ratio, together with the known change in energy and the change in graph-state multiplicity, returns a second reading of **α**.

Both thermometers return the same temperature. Every energy packet is accounted for along every move. Two copies prepared at that temperature can exchange energy, yet neither develops a continuing energy current into the other.

This gives us a digital system whose temperature can be read independently from state counts and reversible motion. The sixteen arrangements, the transitions between them, and every check are available to inspect.

**Read:** [the newcomer-facing account of the two thermometers](./equation-of-state-and-growth-of-a-xypher.md#two-thermometers).

**Inspect:** [the exact recorded result and its complete output](https://github.com/OneManMobile/thaim/blob/c0e32bc1fc787c5322df07422a183a4977b20857/research/physics/derivable/xypher-operational-thermodynamics-result.md).

**Run:** [the standalone verifier source](../code/xypher-thermodynamics-proof/). It has no external Rust dependencies. [Read its quick-start guide](../code/xypher-thermodynamics-proof/README.md).

With Rust installed, run these commands inside that folder:

```text
cargo test
cargo run --quiet
```

We have a digital thermometer. Now we need to distinguish possibility from control.

## Eight doors: possibility is not control

Imagine a Praxion facing eight doors. Behind them lie eight different future states.

In the first version of the experiment, the Praxion has eight commands. Each command reliably opens a different door. The same command always produces the same result. By choosing a command, the Praxion can select one future from eight.

That is three bits of control, because three yes-or-no choices can distinguish eight outcomes.

Now keep all eight doors, but disconnect the commands. A gust of wind opens one at random, with every door equally likely. The future still has eight possibilities. An observer still needs three bits to report which door opened. But the Praxion has no control over the result. Changing its command changes nothing.

The number of possible futures is the same. The causal power of action is completely different.

Between perfect command and pure wind lies noise. One command may usually open the red door, sometimes the blue, and rarely the green. We can repeat each command and measure how much knowing the command tells us about the door that eventually opens.

Information theory calls that shared knowledge **mutual information**. If we choose the most informative mixture of commands—the one that reveals the greatest control available through the interface—the result is the capacity of the action-to-future channel. In agent research, this quantity is called **empowerment**.

Let **A** stand for the command—or sequence of commands—imposed on the Praxion. Let **Y<sub>τ</sub>** stand for the future outcome we want to control after a chosen time horizon, and let **z** stand for the same fixed starting state in every trial. **Y<sub>τ</sub>** deliberately excludes the stored command itself. Otherwise, a system could appear to control its future merely by remembering which command it received. The symbol **q** describes how often each permitted command is tried.

**CAUSAL-CONTROL CAPACITY**

$$
I_\tau^{\mathrm{ctrl}}(z) = \max_q I_q(A; Y_\tau \mid z) \quad \text{bits}
$$

*Control is the information that an imposed action can transmit into the system's future.*

**A** is the selected command. **Y<sub>τ</sub>** is the controlled future outcome. **z** fixes the starting conditions. The expression **I(A; Y<sub>τ</sub>)** asks how much knowing the command tells us about the future that occurs. Maximizing over **q** finds the strongest control the action interface makes available.

Read it as: *From the same starting point, how many future choices can action actually command?*

For the eight reliable doors, the answer is three bits. For eight wind-opened doors, it is zero. For noisy doors, it lies between them.

This gives us a measurable coordinate of intelligence: not how many futures exist, but how much influence action has over which future becomes real.

## When information becomes part of the actor

A base Praxion can act without memory. A learning Praxion adds another step: information from the world can change the mechanism that chooses its next action.

Imagine two otherwise identical Praxions receiving a signal that says **left** or **right**. One discards it and behaves the same either way. The other stores it, and its next move changes with the stored value.

For the second Praxion, the information has become part of the system's state. We can test this directly: change only the stored value and watch whether the later action changes. If it does, the memory has causal consequence.

Once memory sits inside a Xypher, writing it, changing it, and clearing it are changes to the physical system. The Thermodynamic Harness must account for those changes alongside movement in the graph. This is where learning begins to meet thermodynamics—but the basic measure of control does not need to wait for a learning Praxion.

## When the rooms and doors are the same futures

We now know what two measurements must do.

The working thermometer reads **α** from the reservoir's state counts and the system's reversible motion. The action test measures how many future states the Praxion can select.

They become one physical measurement only when they refer to the same futures inside the same system.

Return to the building. The thermometer prices the rooms. The control experiment asks which rooms the Praxion can reach by choosing different doors. The command cannot remain an invisible label outside the building. Keep it in a physical command register until the future is read, and include that register, the Praxion, the graph, the reservoir, and every work source in one energy account.

This gives us two different views of the same event. **Y<sub>τ</sub>** is the outcome whose control we measure; it excludes the command register. **Z<sub>τ</sub>** is the complete physical state used for thermodynamic accounting; it includes the command register, the outcome, the graph, the reservoir, the Praxion, and every other part of the energy ledger. Keeping those views separate prevents stored command labels from masquerading as control while keeping their physical cost inside the Xypher.

Now make two copies of the record.

In the first, preserve which command went with which future. In the second, randomly re-pair the same commands and the same futures. Every command and every future appears with exactly the same frequency in both records. Only the relationship between them has been removed.

That relationship is the mutual information we measured at the doors.

At a fixed temperature, both energy and entropy determine how much capacity a state has to drive physical change. Physicists call this account **nonequilibrium free energy**. Provided linking the command and future does not itself change either part's energy, re-pairing commands and futures leaves their average energy unchanged. What changes is entropy.

In the linked record, knowing the command makes the future less uncertain. Random re-pairing removes exactly that shared information, so the entropy of the combined record rises by the mutual information between command and future. The free-energy value carried by that physical correlation is therefore **α** multiplied by the mutual information in nats.

One bit is **ln 2** nats. At the command mixture that reveals the full channel capacity, call this correlation free energy **V<sub>τ</sub><sup>ctrl</sup>(z)**.

**THE CORRELATION FREE ENERGY OF CONTROL**

$$
V_\tau^{\mathrm{ctrl}}(z) = \alpha \ln 2 \cdot I_\tau^{\mathrm{ctrl}}(z)
$$

*A physical action–future correlation carries free energy in proportion to its measured control information.*

**I<sub>τ</sub><sup>ctrl</sup>(z)** is the Praxion's action-to-future capacity from starting state **z**, measured in bits. **ln 2** converts bits into nats. **α** converts the correlation entropy into the energy units of the Thermodynamic Harness. **V<sub>τ</sub><sup>ctrl</sup>(z)** is the free-energy difference between the linked command–future record and its randomly re-paired counterpart with the same individual frequencies.

Read it as: *How much control reaches the future, and how much free energy does that physical correlation carry at this temperature?*

The equation follows from a standard thermodynamic fact: correlation lowers the entropy of a joint state while leaving its separate parts unchanged. It values the correlation present at readout. The work needed to prepare the command, and any work later extracted from the correlation, remain measurable processes in the same ledger.

The multiplication is simple. The scientific work lies in building the physical object that earns it.

The temperature must be measured independently. The action test must be measured independently. Both must describe the same complete physical system. The action test groups that system's complete states by the declared outcome **Y<sub>τ</sub>**, while explicitly ignoring the stored command when it asks what future was controlled. Both comparisons must use the same grouping of distinguishable outcomes and the same distribution over them. The action-to-future relationship determines the number of control bits. Temperature sets the conversion factor in the free-energy account.

Here is the punch: control does not require a new mystical substance to enter physics. Once command and future are physical coordinates, the information joining them already has a thermodynamic form—correlation free energy.

In the exact thermometer above, every doubling of reservoir possibilities is matched by one model-energy packet. For that reservoir, the equation reports one model-energy unit of correlation free energy per measured control bit. The experiment below tests whether an action channel can earn that shared physical account while the graph's thermodynamics remain intact.

## Where THAIM fits

The [Core Thesis](../../README.md) asks what happens when a Praxion changes the graph itself and opens more future possibility. The Thermodynamic Harness records positive expansion with a receipt called **THAIM**.

THAIM and empowerment answer different questions.

THAIM asks: *How much new future possibility did this action open?*

Empowerment asks: *How much control can action exert within the futures now available?*

A graph can gain possibilities that no Praxion can reliably select. A Praxion can also gain better control among states that already existed. When expansion and control live in the same fully accounted states at the same temperature, they can finally be compared inside one thermodynamic account.

## The next experiment

The exact digital construction already supplies the first half: a graph–reservoir system whose state counts, transition traffic, energy ledger, equilibrium behavior, and contact behavior agree on one temperature.

The next experiment adds the action instrument.

1. Enlarge the exact state list to include a physical command register, the Praxion, and any memory it uses. Include the mechanism supplying the command, or record that mechanism's contribution as external work.
2. Choose one completely specified starting state and one future horizon. For every permitted command or command sequence, enumerate the exact distribution of controlled outcomes **Y<sub>τ</sub>**. Separately enumerate every complete thermodynamic state **Z<sub>τ</sub>**, including the command register, Praxion, graph, reservoir, and work account.
3. Find the command mixture that transmits the most information into those futures. This gives the action-to-future capacity in bits.
4. Construct or exactly enumerate a second physical ensemble in which the command register and controlled outcome are independently paired, while keeping each one's individual frequencies unchanged. The possibilities and average energy remain; the control information and its correlation free energy should collapse.
5. Run the thermodynamic checks again on the enlarged object. The reservoir and reversible traffic must still return the same **α**. Every move and command must close its energy account. The action test must use **Y<sub>τ</sub>**; the thermodynamic check must include every complete **Z<sub>τ</sub>** state and its probability.

If every command produces the same spread of futures, the graph has possibility without control. If two commands are used equally and reliably select two different futures, the channel carries one bit. If those outcomes live inside the same fully energy-accounted graph–reservoir–Praxion system to which **α** applies, then one Xypher has joined thermodynamics and causal control in a single executable object.

The target is no longer vague. We know what to build, what to measure, and what result would prove us wrong.

## A physical coordinate of intelligence

The result can now be said in ordinary language.

A system has one bit of causal control when its action can reliably select between two distinguishable future states. Information theory already knows how to measure that control when the world is noisy. Statistical physics already knows how to assign an energy price to distinguishable state multiplicity. A matched Xypher can give both measurements one shared physical home.

The digital thermometer is built and exactly reproducible. The matched action-to-future experiment is the frontier immediately in front of us.

A mind can express many capacities. This places one of them—the power to make a difference to its own future—on a physical scale.

A furnace has energy. A library has information. A Praxion has consequence.

The unit is a bit of control. The temperature gives it weight.

## Research trail

- Claude E. Shannon, [“A Mathematical Theory of Communication”](https://doi.org/10.1002/j.1538-7305.1948.tb01338.x), establishes the bit, entropy, mutual information, and channel capacity used to measure the action-to-future link.
- Alexander S. Klyubin, Daniel Polani, and Chrystopher L. Nehaniv, [“Empowerment: A Universal Agent-Centric Measure of Control”](https://doi.org/10.1109/CEC.2005.1554676), develops action-to-future channel capacity as an agent's potential control.
- Takahiro Sagawa and Masahito Ueda, [“Fluctuation Theorem with Information Exchange: Role of Correlations in Stochastic Thermodynamics”](https://doi.org/10.1103/PhysRevLett.109.180602), establishes how mutual information enters the thermodynamic account of physical correlations.
- Xyphers research, [“An Equation of State and the Growth of a Xypher”](./equation-of-state-and-growth-of-a-xypher.md), builds the floors-and-rooms ladder and explains the two independent digital thermometers.
- Xyphers research, [the exact operational-thermodynamics result](https://github.com/OneManMobile/thaim/blob/c0e32bc1fc787c5322df07422a183a4977b20857/research/physics/derivable/xypher-operational-thermodynamics-result.md), contains the sixteen-state construction, checks, output, and scientific boundary.
- Xyphers research, [the version-pinned standalone source](https://github.com/OneManMobile/thaim/tree/17b57589a0bd84dfd13d94f09d7340f5d4e85392/research/physics/xypher-thermodynamics-proof), contains the dependency-free verifier used by the downloadable archive.

[Return to the research index](../README.md).
