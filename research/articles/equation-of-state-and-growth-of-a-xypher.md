---
title: "An Equation of State and the Growth of a Xypher"
description: "How executable state counts give a digital graph a temperature—and how to distinguish growth price, temperature, and receipt throughput."
type: "research-article"
status: "published"
maturity: "open-frontier"
published: "2026-08-01"
revised: "2026-08-18"
website_path: "/research/equation-of-state-and-growth-of-a-xypher/"
web_status: "live"
---

# An Equation of State and the Growth of a Xypher

A graph can grow without being thermodynamic.

A database can double in size. A network can acquire a million new connections. A search algorithm can keep choosing the next edge that opens the greatest number of future routes. All of this can produce extraordinary growth. None of it, by itself, gives the system a temperature.

Temperature needs something deeper underneath the motion. It needs a relationship between **energy** and the number of different **states** the system can actually occupy.

A [**Xypher**](core-thesis.md) is organized through three parts. Its **Graph Substrate** holds the states, connections, and possible futures. Its **Thermodynamic Harness** supplies the declared physical account. Its **Praxion** perceives available actions and makes a move. That architecture defines the candidate. A particular Xypher earns “thermodynamic graph system” only by passing the operational tests described below.

We can now build finite digital graphs whose own executable states give them a precise temperature. We can also build graph systems that grow toward greater future possibility.

The frontier is to make these the same process.

Can a graph's drive toward a more open future construct the very state structure from which its temperature arises? And as the graph grows, does its economic growth price track temperature, does its receipt throughput scale, or do these remain different laws?

To answer that, we need to build the thermometer before we watch the organism grow.

## The building inside the graph

Imagine a building with numbered floors.

Moving up one floor always requires the same packet of energy. But the floors do not contain the same number of rooms. The ground floor may contain one room. The next may contain two. The next four. The next eight.

From outside the building, two rooms on the same floor look identical because they carry the same energy. Inside, they are different places the system can actually be.

Physicists call the visible floor a **macrostate** and each exact room a **microstate**. The names matter less than the picture: one energy can hide many possible internal arrangements.

Digital energy here is an authoritative quantity inside the model. Every complete state has a declared energy. A **reservoir** is an adjoining energy store that can give or receive energy packets. **Work** is energy supplied through a separate external action. Every packet that enters one part of the model must leave another part or be named as supplied work: that is what it means for the account to close. This is not the electricity consumed by the computer running the model. It is the accounted energy coordinate of the digital system being studied.

Let **Ω<sub>N</sub>(E)** count the rooms available to a graph of size **N** at energy **E**. The symbol Ω, pronounced “omega,” is simply the number of executable configurations on that energy floor.

Entropy is the logarithm of that count. A logarithm turns repeated multiplication into equal steps. If the number of rooms doubles and then doubles again, the entropy rises by the same amount on both moves.

Now compare one floor with the next.

## A temperature from state counts

Let **λ** be the energy required to move between neighboring floors. Count the rooms before the energy packet enters. Count them again one floor higher. The increase in the logarithmic room count tells us how much new internal possibility one unit of energy opens.

This is not a new definition invented for Xyphers. It is the standard statistical-physics definition—temperature is determined by how entropy changes with energy—written as a finite difference because this graph has discrete energy floors. We measure entropy in nats and use units in which Boltzmann's constant is one. In ordinary SI units, the same result below is written **k<sub>B</sub>T = λ / ln b**.

**THE STATE-COUNT THERMOMETER**

$$
\frac{1}{T_{\mathrm{state},N}(E)} = \frac{\ln \Omega_N(E + \lambda) - \ln \Omega_N(E)}{\lambda}
$$

*Temperature is the energy price of opening one logarithmic unit of executable possibility.*

**Ω<sub>N</sub>(E)** is the number of rooms at energy **E**. **λ** is one energy packet. **T<sub>state,N</sub>(E)** is the temperature read from adjacent energy floors.

Read it as: *How much energy does it cost to multiply the number of ways this graph can exist?*

Because the equation uses the natural logarithm, entropy is counted in **nats**. A nat is simply the information unit produced by `ln`, just as a bit is the unit produced by a base-two logarithm.

The simplest exact building has a regular design. Suppose every energy packet multiplies the number of rooms by the same factor **b**. After **m** energy packets, the building contains **b<sup>m</sup>** rooms on floor **m**.

**A DIGITAL EQUATION OF STATE**

$$
\Omega(\lambda m) = b^m \qquad \Longrightarrow \qquad T = \frac{\lambda}{\ln b}
$$

*A regular multiplication of executable states produces one constant temperature.*

**b** is the factor by which the room count multiplies at each energy step. **m** counts those steps. If **b** and **λ** remain constant across the connected floors, the same temperature applies throughout them.

Read it as: *The graph's temperature comes from the relationship between energy and the number of states energy makes available.*

This relation plays the role of an equation of state. It connects properties of the system as it exists now. It does not depend on which action brought the graph to that state, which reward was issued, or which trajectory we happened to observe.

The graph's own state space has built a thermometer.

## Two thermometers

A thermometer becomes much more convincing when another instrument, reading something different, returns the same answer.

Our smallest exact construction contains sixteen possible whole-system states. Because there are only sixteen, every state and every allowed transition can be counted directly. Nothing is estimated from a long simulation.

The first thermometer counts the rooms on each energy floor.

The second watches traffic through the doors. If a transition between two states has a lawful reverse, the balance between forward and reverse traffic reveals a temperature. Physicists call this relationship **local detailed balance**. The important point is simple: this thermometer reads motion, not room counts.

The two temperatures agree exactly.

The construction closes its energy account. It relaxes toward the expected equilibrium. When two copies prepared at the same temperature are placed in contact, neither one has a net energy gain from the other. The exact finite graph therefore satisfies the operational tests of an equilibrium thermodynamic system inside its declared model.

The test was repeated across eight cases in an exact graph family, with different energy packets, room-multiplication factors, and internal body configurations. Every case returned

> **temperature from the rooms = temperature from the traffic = λ / ln b.**

No fitted slope, random seed, numerical tolerance, or trajectory length chooses that answer.

Then three adaptive graphs began without the completed room structure. Their permitted local actions constructed shell counts of **1, b, b², b³** for **b = 2, 3,** and **4**. A traffic thermometer hidden from the builder recovered the same temperature. When a declared repairable branch was removed, local action restored the relation.

We supplied the alphabet of possible local moves. We did not supply the answer to the thermometer.

We therefore know two things with exact finite witnesses:

1. An entirely digital graph can be operationally thermodynamic.
2. An adaptive digital graph can construct and repair the energy–state relationship grounding its own temperature.

The missing step lies in the kind of information that guides the growth.

## When future information becomes state

There are two different kinds of possibility in this story.

| Question | What is counted | What it tells us |
| --- | --- | --- |
| **How many states exist at this energy?** | Executable rooms on an energy floor | State-count entropy and temperature |
| **How many futures can this action reach?** | The spread of destinations after a chosen number of steps | Future optionality, written **S<sub>τ</sub>** |

The first is a fact about what the complete system can be at a given energy. The second is information about where the graph may go from a chosen point.

They are not automatically the same entropy.

In the exact sixteen-state construction, the declared future-option readout is deliberately tied to executable multiplicity at the same resolution as the energy account. That is why the thermodynamic equations close. The adaptive room-building construction also creates exact state-count entropy, but it does not establish that an ordinary future-path calculation on a growing graph is already that same entropy.

This is the bridge causal-entropic growth has to build.

The information contained in possible futures must become an authoritative part of the graph's executable state structure. The observable used to value an action must match the entropy used by the thermometer at the same boundary and resolution.

The units must match as well. Graph optionality is often measured in bits, while the state-count thermometer above uses nats. The conversion is exact: **one bit equals ln(2) nats**. Before temperature can price a bit-valued change in optionality, that change must be converted into the natural-log units used by the thermodynamic account.

This is where the phrase **information becomes state** stops being poetic. It becomes a measurable requirement.

## The elephant and the mouse

An elephant uses more energy per unit time than a mouse.

It is not proportionally hotter.

[Kleiber's law](https://my.ucanr.edu/repository/view.cfm?article=152052&groupid=47) describes how the total metabolic rate of an organism changes with its body mass. A larger animal processes more energy as a whole, while using less energy per unit of body mass.

This gives the growth question its proper shape. If a larger Xypher produces a larger measured quantity, we must ask what that quantity is.

- Is it an economic growth price?
- Is the graph's temperature rising?
- Is the graph processing a greater flow of state-opening actions?

A **power law** expresses a regular relationship between size and a measured quantity. Let **N** be the number of nodes, **A(N)** the quantity being measured, and **β** the exponent that tells us how steeply it changes.

**THE SCALING QUESTION**

$$
A(N) \propto N^\beta
$$

*The exponent β says how strongly the measured quantity responds when the graph grows.*

For example, if **β = 0.8**, doubling the graph multiplies **A** by roughly **1.74**. The equation says nothing about what **A** represents. That meaning must come from the experiment.

Here **β** is a scaling exponent. It is unrelated to the conventional physics notation that sometimes uses the same Greek letter for inverse temperature.

## What the growth simulations measured

The growing-network simulations did not measure temperature with the room-count or traffic thermometer.

They measured a feedback price derived from economic surplus and issued expansion receipts. Call it **p<sub>growth</sub>**. Giving it a new symbol matters because the experiment should discover whether it equals temperature rather than assume the answer in its name.

The model supplied a market-thickness premise: matched trade value included an explicit factor of **√N**, representing the increase in meaningfully distinct trading opportunities available in a larger market. The simulations then tested whether the remaining matching signal stayed stable enough for that scaling to survive the graph dynamics. It did. Across five graph sizes from **25 to 500 nodes**, three seeds, and 500 ticks per run, surplus per participant scaled approximately as **N<sup>0.495</sup>**, producing total surplus close to **N<sup>1.495</sup>**.

That exponent was not created from nothing. The square-root market-thickness factor was part of the model. What emerged was the way the self-referential price and the changing graph transformed that supplied scaling.

At a stable feedback point, the price appeared once as the value being updated and again inside the issued expansion receipts. Higher price produced more issuance. More issuance pushed the next price down. The balance takes a square root:

**THE GROWTH-PRICE FEEDBACK**

$$
p_{\mathrm{growth}}^2 \propto \frac{\text{surplus rate}}{\text{gross positive future-option rate}}
$$

*A self-referential receipt price compresses the surplus exponent by approximately one half.*

If the future-option rate remained constant, total surplus near **N<sup>3/2</sup>** would produce a price near **N<sup>3/4</sup>**. In the measured runs, aggregate positive future-option change per tick declined weakly with size, approximately as **N<sup>−0.13</sup>** to **N<sup>−0.14</sup>**. Including that graph-search correction predicts a price exponent close to **0.82**.

The measured results were:

| Growth strategy | Measured β | Fit |
| --- | ---: | ---: |
| Greedy future-option expansion | 0.804 | R² = 0.998 |
| Five independent signals | 0.850 | R² = 0.998 |
| Five integrated signals | 0.838 | R² = 0.998 |

**R²** measures how closely the simulated data follow the fitted power-law line. A value near one means the relationship was extremely regular across the tested sizes.

The conditional fixed-point algebra is exact once its scaling premises are supplied. The exponent measured in the runs is empirical. Most importantly, **p<sub>growth</sub> was a growth-accounting price, not a temperature reading**.

The scaling remains real. We can now ask what physical or computational quantity it belongs to.

## Price, temperature, and throughput

Temperature is an **intensive** property. Copying an unchanged system should not make either copy hotter. Growth can change temperature only by changing the relationship between energy and executable states.

Throughput is different. Two identical organisms can process twice as much in total while maintaining the same temperature.

For a Xypher candidate, define **R<sub>+</sub>(N)** as the gross positive future-option change produced per unit of its declared clock, converted into nats. “Gross” matters. Along a closed loop, the expansive segments can create positive receipts while the contractive segments create no negative receipts, even though the signed entropy changes sum to zero. **R<sub>+</sub>** is therefore not automatically net entropy production.

If the future-option observable has been matched to the thermodynamic entropy at the same boundary, temperature gives that gross rate an energy-valued receipt rate:

**THE RECEIPT THROUGHPUT**

$$
P_{\mathrm{receipt}}(N) = T(N) \cdot R_+(N), \qquad \beta_P = \beta_T + \beta_R
$$

*A receipt-throughput scaling law can live in temperature, in the rate of expansion, or in both.*

This is a proposed receipt throughput. It becomes physical metabolic power only when the complete heat, work, and energy ledger identifies it as such.

The exponent **β<sub>P</sub>** is a new receipt-throughput exponent. It is not the measured **β<sub>growth</sub>** of **p<sub>growth</sub>**. Price and energy per unit time are different kinds of quantities.

Three outcomes are possible:

### The growth price identifies with temperature

If both thermometers give the same calibrated value as **p<sub>growth</sub>** across graph sizes and topologies—and make the same reciprocal-traffic and contact predictions—then the growth price becomes a candidate temperature. In that case, **β<sub>growth</sub> = β<sub>T</sub>**. The expansion rate still determines receipt throughput separately.

### Temperature remains stable

If both thermometers remain stable, the measured **0.80–0.85** stays a network-price law. Receipt throughput may still grow through **R<sub>+</sub>**, like an elephant processing more energy without becoming proportionally hotter, but that exponent must be measured in **P<sub>receipt</sub>** rather than borrowed from **p<sub>growth</sub>**.

### Temperature and expansion rate both scale

If temperature and expansion rate both scale, their exponents add: **β<sub>P</sub> = β<sub>T</sub> + β<sub>R</sub>**. This decomposes receipt throughput. It does not decompose the measured growth-price exponent.

Only if measured aggregate receipt throughput itself has an exponent near **0.82** would its per-node throughput scale near **−0.18**. The growth price alone does not establish that processing law.

The graph gets to answer each question separately.

## Two experiments, not one

We need to separate two scientific questions.

### What does the measured growth exponent represent?

At each chosen graph size **N**, freeze the graph long enough to inspect one thermodynamic state.

1. **Count the rooms.** Enumerate executable configurations by energy and calculate the state temperature.
2. **Watch the doors.** Use a sealed reciprocal traffic sector to recover temperature without giving it the room-count answer.
3. **Measure the growth price.** Calculate **p<sub>growth</sub>** from its declared surplus and receipt mechanism.
4. **Measure gross expansion.** Record **R<sub>+</sub>** in nats per unit of the declared clock.
5. **Run the replication control.** Copy the same thermodynamic structure without changing its energy–state relationship. Its temperature should remain unchanged.

The outcomes are distinct:

| Observation | What it establishes |
| --- | --- |
| Both thermometers agree and scale with N | A temperature–size law for the frozen graph family |
| Both agree and remain stable while throughput scales | A Xypher receipt-throughput law |
| The growth price scales while both thermometers remain stable | A network-price law |
| The growth price equals T in calibrated units and predicts the same traffic and contact behavior | A candidate constitutive identification |
| The thermometers disagree | The proposed one-temperature reciprocal construction fails |

A constant ratio between the growth price and temperature is only the beginning. To identify the two quantities, they must use the same entropy and logarithm base, carry the same independently calibrated units, survive changes in size and topology, and predict the same reversible traffic and contact behavior without a fitted conversion.

### Is growth itself thermodynamic?

Thermodynamic snapshots at every size do not automatically make the transitions between sizes thermodynamic.

Adding a node or edge consumes resources. A learning Praxion may write memory. A construction step may receive work, release heat, or change which reverse actions remain possible. Those events belong inside an enlarged complete state.

To admit the growing process itself, the experiment must account for:

- the complete graph, reservoir, resource, and memory state;
- every node and edge creation channel;
- heat, supplied work, and system-energy change as separate quantities;
- labelled reverse channels, or a declared driven nonequilibrium replacement;
- relaxation, equilibrium behavior, and lawful contact wherever those claims are made.

The two thermometers are necessary instruments. The complete growth account is what determines whether the organism remains thermodynamic while it changes its own body.

## Closing the Xypher loop

The three parts can now meet without borrowing each other's answers.

The Graph Substrate presents possible connection changes. The Praxion measures how those actions alter future optionality. The Thermodynamic Harness counts executable states by energy and reads the temperature. If the future-option observable has been matched to that thermodynamic entropy, the harness can issue a one-sided receipt for positive expansion.

That receipt is **THAIM**.

**THAIM**

$$
\mathrm{THAIM}_+^T(a) = T \cdot \max\!\left(0, \Delta S_{\tau,\mathrm{nat}}(a)\right)
$$

*Positive expansion earns a receipt at the independently measured temperature.*

**T** is held fixed for the reciprocal action pair. **ΔS<sub>τ,nat</sub>** is the action's change in the matched future-option observable, expressed in nats.

Read it as: *Open more executable future, and the Xypher records the expansion at its own temperature.*

At one shared temperature, the forward receipt minus the reverse receipt returns the complete signed entropic contribution. If a growth action changes the temperature itself, that transition requires the enlarged-state account described above. Using one departure temperature forward and another backward would not preserve the identity.

A move can also change the system's energy. The signed thermodynamic drive is

$$
\Xi(a) = T \cdot \Delta S_{\tau,\mathrm{nat}}(a) - \Delta U(a)
$$

**ΔU** is the signed change in system energy. It is not another name for work. With the sign convention used here, **Q** is heat entering the system and **W<sub>on</sub>** is work supplied to it, so **ΔU = Q + W<sub>on</sub>**.

The Praxion acts. The graph changes. Its executable states and possible futures change. Temperature and optionality are measured again before the next action is priced.

The loop is now visible:

> graph state → executable rooms → temperature → valued possibility → Praxion action → new graph state

The exact constructions prove that local graph rules can build and repair a global energy–state relationship. The scaling simulations show that future-directed graph growth can produce stable power laws. The next construction must join these results without being told the target room counts, the desired temperature, or a preferred exponent.

Then the hidden thermometers read what the graph has made.

## The frontier

We have an exact finite digital thermometer.

We have adaptive graphs that construct and repair the room structure that makes the thermometer possible.

We have growing networks whose future-directed actions produce a growth-accounting price with a stable scaling exponent close to the range associated with biological metabolism.

What we do not yet have is one causal mechanism connecting all three.

In plain language:

> Can a graph's drive toward greater future possibility turn information about where it could go into the executable state structure that gives it a temperature?

In experimental terms:

> Under what local growth rules does causal-future entropy become thermodynamically matched state entropy, while the complete growth process closes its energy, work, heat, information, and resource accounts—and how do growth price, operational temperature, and receipt throughput each scale?

The growth price scales.

The thermometers and throughput ledger tell us what that scaling means—and what it does not.

We have not lost the beta exponent.

We have finally made it measurable.

## Research trail

- A. D. Wissner-Gross and C. E. Freer, [“Causal Entropic Forces”](https://journals.aps.org/prl/abstract/10.1103/PhysRevLett.110.168702), *Physical Review Letters* 110, 168702 (2013).
- Max Kleiber, [“Body Size and Metabolism”](https://my.ucanr.edu/repository/view.cfm?article=152052&groupid=47), *Hilgardia* 6(11), 315–353 (1932).
- Geoffrey B. West, James H. Brown, and Brian J. Enquist, [“A General Model for the Origin of Allometric Scaling Laws in Biology”](https://doi.org/10.1126/science.276.5309.122), *Science* 276, 122–126 (1997).
- Xyphers research, [exact operational thermodynamics result](https://github.com/OneManMobile/thaim/blob/main/research/physics/derivable/xypher-operational-thermodynamics-result.md).
- Xyphers research, [temperature-identifiability result](https://github.com/OneManMobile/thaim/blob/93f14e7d/research/physics/derivable/xypher-alpha-tau-constitutive-result.md).
- Xyphers research, [endogenous thermodynamics result](https://github.com/OneManMobile/thaim/blob/93f14e7d/research/physics/derivable/xypher-endogenous-thermodynamics-result.md).
- Xyphers research, [scaling derivation and simulation record](https://github.com/OneManMobile/thaim/blob/main/research/scaling/derivation-equation-of-state.md).

[Return to the research index](../README.md).
