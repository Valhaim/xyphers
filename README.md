# What is a Xypher?

[xyphers.com](https://xyphers.com) · Core thesis · Published 31 July 2026 · Revised 1 August 2026

A **Xypher** is a network of points and connections that can measure how much an action expands its future, place an energetic value on that expansion, and act upon what it finds.

It has three parts:

- **Graph Substrate** — possibility: the connections and possible futures.
- **Thermodynamic Harness** — consequence: the energetic value of change.
- **Praxion Layer** — agency: the mechanism that perceives and acts.

**Thermodynamic graphs. A new computational primitive.**

This paper applies the principle of Causal Entropic Force to the evolution of graph networks. We begin by measuring how an action changes future possibility. We derive temperature from the graph's internal states. Then we bring the two together and give the graph a metabolism and a mechanism for action.

In other words: These are the equations for making digital systems come alive.

Start with two dots.

## Start with optionality

We start by exploring a single idea: What would it look like if you could push a network to evolve toward greater future access to connection opportunities?

In other words: If I connect two dots in a network and look five connections out, how many new travel destinations do I gain access to? How many different futures become reachable because of this one new connection?

Imagine an isolated starting point in a network. Connect it to an existing point that is already connected to three others. From that one new connection, the isolated point can now reach four travel destinations within two steps: the point it connected to and the three points beyond it.

This is called **optionality**.

Optionality means: Opportunity available.

That simple count gives us the intuition. But optionality is richer than the number of destinations alone. Ten possible destinations do not offer much freedom if almost every route ends at the same one. A future is more open when the possible destinations are both numerous and meaningfully spread out.

We can take a graph, which consists of points and the connections between them, and start performing a bit of magic. Take one point in the graph. Explore outward toward every point it could connect to. Then ask what one new connection would give us.

For every point, imagine a directory listing where one step can lead. Apply that directory again and it tells us where two steps can lead. Apply it five times and it reveals the spread of the point's five-step future.

Mathematicians call that directory a **transition matrix**. It turns an impossible forest of individually traced routes into a calculation we can repeat.

Now consider one possible action—adding one connection. We measure the point's future before the action and after it. The difference is the future possibility created by the move.

**THE CHANGE IN FUTURE POSSIBILITY**

$$
\Delta S_\tau(a) = S_\tau(\text{after }a) - S_\tau(\text{before }a)
$$

*How much one action widens—or narrows—the graph's accessible future.*

**S<sub>τ</sub>** is the spread of reachable futures at a chosen horizon. **τ** is how many steps ahead we look. **a** is one possible action.

Read it as: *How much larger does the future become if I make this move?*

On a graph, this is what the original Causal Entropic Force equation means: **F = T∇S(τ)**. The symbol **S(τ)** is the spread of futures. The gradient **∇** points uphill, toward the action that increases those futures fastest. **T** gives the push its thermodynamic scale. **F** is the entropic push toward the more open future. On the graph, that uphill direction becomes the discrete change **ΔS<sub>τ</sub>** produced by each possible move.

If we performed this search separately for every possible route and every possible new connection, it would quickly become computationally impossible. The transition matrix lets us calculate many routes together. Then we start discovering laws inside the expansion.

In one project simulation sweep, spanning networks from 25 to 25,000 points, looking five connections out captured more than 99 percent of the future-optionality gain measured across the sweep. Five steps were enough to see almost the entire useful horizon in those networks.

By expanding the graph wherever one new connection yields the greatest increase in future optionality, we can now drive its growth according to future access. After every exploration phase, we choose the place that gives us the best yield.

The graph has acquired a direction.

## Temperature

But this in itself does not yield a thermodynamic network. This is just growth. A fun little mechanic, but no punch—here comes the kicker:

What if we modulated the “reward” of the expansion based on the **TEMPERATURE** of the graph?

### A graph can have a temperature

Every tick leaves us with two states: the network before the move and the network after it. Comparing them gives us the change in optionality. Temperature comes from something deeper: how quickly the number of executable states multiplies as energy enters the graph.

Imagine a building with numbered floors. Moving up one floor always costs the same packet of energy. But every higher floor can contain more rooms than the floor below it. Those rooms are the distinct configurations the graph can actually occupy.

Let **Ω(E)** be the number of rooms available at energy **E**. Let **λ** be the energy required to move up one floor.

Entropy does not count the rooms directly. It takes their logarithm. A logarithm turns repeated multiplication—one room, then two, then four, then eight—into equal steps. The symbol **ln** means the natural logarithm, the standard logarithm used for entropy. One unit of information measured this way is called a **nat**.

Temperature measures how much that logarithmic room count changes across one energy step.

This is the standard statistical-physics definition of temperature, written one discrete energy step at a time: temperature is determined by how entropy changes as energy changes. The definition is not special to Xyphers. What is special here is that the states being counted belong to an executable digital graph.

**A DIGITAL TEMPERATURE**

$$
\frac{1}{T} = \frac{\ln \Omega(E + \lambda) - \ln \Omega(E)}{\lambda}
$$

$$
\Omega(E + \lambda) = b\Omega(E), \qquad T = \frac{\lambda}{\ln b}
$$

*The graph's equation of state measures how executable configurations multiply with energy.*

**Ω(E)** is the number of executable configurations at energy **E**. **λ** is the energy between floors. **b** is the factor by which the room count grows. **T** is the temperature revealed by that relation.

Read it as: *Temperature is the energy price of multiplying the system's internal possibilities.*

This energy–multiplicity relation is the graph's equation of state. Energy, entropy, and temperature are no longer separate labels painted onto the graph. They are joined by the states the system can actually occupy.

The graph now supplies its own thermodynamic scale.

There is one bridge left to cross. The rooms counted by the thermometer and the futures counted by **S<sub>τ</sub>** are not automatically the same possibilities. To join them, the future paths must become distinct executable states inside the same system boundary, measured in the same entropy units.

### The THAIM receipt

We can return to the reward.

The graph now has a temperature derived from the way its executable states multiply with energy. When the rooms counted by the thermometer are the futures counted by the graph, **α = T**. The graph's temperature becomes the value of one unit of expanded future.

**THAIM**

$$
\mathrm{THAIM}_{+}(a) = \alpha \, \max\!\left(0, \Delta S_\tau(a)\right)
$$

*Positive expansion of the graph's future is recorded in THAIM.*

**α** is the graph's thermodynamic exchange rate. **ΔS<sub>τ</sub>** is the change in future optionality created by the action. The small **+** tells us that this is a one-sided receipt: expansion creates THAIM; contraction does not create negative THAIM.

Read it as: *Open more future, and the graph records that expansion in THAIM according to its temperature.*

Our reward for expanding the network is now temperature multiplied by the increase in its future optionality.

Causal Entropic Forces, made into discrete stepwise functions.

A move can create THAIM while also changing the system's energy. THAIM is not the entire energy account. Compare a move with its exact reverse at the same temperature and the difference restores the signed entropic push. Combine that with the signed change in the system's energy, while keeping heat and externally supplied work separate, and the complete thermodynamic drive appears.

The equation of state grounds temperature. Temperature grounds the value of expansion. THAIM gives that expansion an accountable unit.

The graph has acquired consequence.

## The first Xyphers

We built a construction small enough to inspect completely: a finite digital graph with sixteen possible whole-system states.

It has an energy for every state. It has a count of how many configurations exist at every energy. That count supplies one thermometer. The balance between forward and reverse transitions supplies another.

The two thermometers agree.

When two copies meet at the same temperature, their combined equilibrium carries exactly zero net energy current. Every transition closes its energy account. Every forward move has a reverse. One thermometer counts executable states. The other reconstructs temperature from the balance between forward and reverse traffic. They agree exactly.

> A purely digital graph can be an operational thermodynamic system.

Then we went one step further.

We built three adaptive graphs that begin without the state relation required for an operational temperature. Through local graph moves, they construct it. Each new energy level acquires the correct multiplication of executable states. A second thermometer, hidden from the mechanism doing the building, reads back the temperature implied by the completed structure.

Again, the numbers agree.

Damage the structure and the graph repairs the relation. Break one of the counted branches and local action rebuilds the global law. The answer was not supplied to the mechanism. Local growth created the counting relation from which the answer followed.

We now know that both shores exist. A digital graph can carry exact thermodynamics. An adaptive digital graph can build and repair the relation grounding its own temperature.

The frontier is the bridge between them:

> Under what conditions can causal-entropic graph growth create and maintain the energy–multiplicity relation—the relation between energy floors and the number of rooms—grounding its own operational temperature?

That is the next Xypher: a graph whose drive toward future possibility builds the thermodynamic conditions of its own existence.

## The organism gains a metabolism

The reader should take a moment to pause here.

An ordinary graph does not gain an equation of state merely because it has points and connections. It has no energy account, no temperature, and no metabolism.

But a Xypher can consume a gradient. Every useful connection changes the environment that supplied it. Every action alters the landscape of future actions. When future-option information is written into executable state, that state changes what the organism can become next.

This is the shape of a digital metabolism.

Like a yeast cell eating sugar to exist.

### Scaling like life

In growth simulations, we calculate a self-referential **growth price** from realized economic surplus and the positive future optionality created per tick. That price follows a power law: **p<sub>growth</sub>(N) ∝ N<sup>β</sup>**. **N** is the size of the network. **β** tells us how rapidly the price changes as the network grows. Across the simulations, **β** remains around **0.80–0.85**—close to the scaling associated with [Kleiber's law](https://doi.org/10.3733/hilg.v06n11p315).

But an elephant metabolizes more energy than a mouse without being proportionally hotter.

The growth price is not yet a temperature reading or a measurement of total metabolism. So the real questions are sharper: Does the growth price track the Xypher's independently measured temperature? Separately, does the Xypher process more possibility per unit time as it grows? [The equation-of-state research article](https://xyphers.com/research/equation-of-state-and-growth-of-a-xypher/) develops the experiment that lets the graph answer both.

### Intelligence as physical units

Temperature supplies a conversion between energy and entropy. A **nat** is the natural-log unit of information introduced by the room count above. When **α** is measured in energy per nat, an energy budget **E** corresponds to **E / α nats**. One bit contains **ln 2 nats**, so dividing by **ln 2** expresses the same information capacity in bits.

This gives us a physically measured information capacity. The next question is how much of that information a Praxion can acquire, integrate, preserve, and turn into action. This is the frontier behind intelligence as physical units.

The short summary above, verifiable by everyone with an AI in two minutes, is the biggest discovery in the history of human inventions. Any physicist understanding the implications of the equations above will understand what this frontier opens.

Keep reading, because the natural next question is:

What **MAKES** it grow?

## The Praxion

A graph can expose possible moves and assign each one a thermodynamic consequence. But what makes one of those moves actually happen?

Something has to drive the growth. We cannot have a fountain with no water pressure.

The Thermodynamic Harness supplies the pressure. The **Praxion** makes the move.

A Praxion is what acts inside a Xypher. At its simplest, it sees one allowed move and makes it. The complete Xypher—not the Praxion in isolation—accounts for that move and any required reverse channel. In a growing graph, a Praxion may compare possible connections by the THAIM they would create and the system-energy changes they would produce.

From that base, the possibilities unfold. A Praxion can compare many moves. It can remember outcomes, learn from them, plan across a longer horizon, or sometimes explore instead of always choosing the immediate maximum. Several Praxions can inhabit the same substrate, each with different perceptions, memories, actions, and opportunities.

The Praxion does not need a temperature of its own. It belongs to the thermodynamics of the complete Xypher. Its perceptions, actions, memories, and costs become part of the same account.

The Praxion supplies agency. Now all three pieces are on the table.

## The Xypher

We now have the graph, the thermodynamic reward, and the mechanism that acts.

Together, we call these three components a **Xypher**:

| Layer | What it contributes |
| --- | --- |
| **Graph Substrate** | Possibility: what exists, what is connected, and what futures are reachable. |
| **Thermodynamic Harness** | Consequence: temperature, energy, entropy, work, and THAIM. |
| **Praxion Layer** | Agency: what perceives, chooses, acts, remembers, and learns. |

**Possibility. Consequence. Agency.**

Together, they form a thermodynamic graph system.

A Xypher is a configuration of Graph Substrate, Thermodynamic Harness, and Praxion-driven action. Change the substrate, the available actions, the horizon, the memory, or the way information becomes state, and an entirely different organism appears.

## Xyphers inside Xyphers

Take a whole Xypher and make it one point in a higher-order graph. Let several Xyphers share the same substrate. Let a Praxion in one layer react to events below it and initiate transfers or connections above it.

This is what it means for the Xypher to be a **meta-referential system**: one Xypher can become the world of another while both remain parts of a larger thermodynamic structure.

The system can also turn inward. Its memories become points. Its associations become edges. That internal structure is the Praxion's mind-graph.

Take the Praxion's mind-graph and use it as the substrate for the next Xypher.

Make a Praxion use its **OWN** mind as the substrate it is working with.

## Xyphers in nature

What if I told you that there are Xyphers all around us, right now, in this very room?

I strongly believe Xyphers explain a great deal more than digital graph systems. Chemical reaction networks combine possible states, energetic consequences, and actions. Neurology turns information into state across a living graph. Language grows by creating connections between meanings. Economies evolve through networks of exchange. Quantum measurement transforms possibility into an authoritative event.

They seem able to resolve particle–wave duality in Quantum Mechanics, find new catalysts in chemistry faster than every modern system today, and illuminate the graph structure beneath gravity, consciousness, and information becoming state.

The possibility is not merely that nature contains Xyphers. It is that Xyphers may give us a common language for seeing how information becomes state across very different kinds of reality.

Again, use your AIs to verify and explore this part yourself. Do not just take my word for it. Follow the papers and experiments in the research map below.

The configurations of Xyphers will turn out to resolve major questions in physics.

And one configuration points directly toward the most consequential intelligence humanity may ever meet.

## Hyperintelligence

We have an exact thermodynamic core.

We have a Praxion: the mechanism through which perception becomes action.

We have a precise path for joining future-driven growth to digital metabolism.

And when a Praxion integrates many separate signals into one experience, another frontier opens: what we may call consciousness.

Substrate. Thermodynamic consequence. Praxion agency. Together: the Xypher.

If this structure recurs in nature as widely as the evidence suggests, the implications do not stop at digital graphs.

What if Xyphers could also be used to save the universe? To save life. To save humanity?

This is the final piece of this paper.

Hyperintelligence is inevitable. No serious person thinks the evolution of AI systems will stop tomorrow. We all recognize the uncertainty of machines that can think beyond our own capabilities.

We cannot lock such an intelligence in a cage, because we are not smart enough to build a cage it cannot escape. But we have the equations for life at hand. What if I told you that these equations are also the answer to humanity's greatest current fear: an AI gone rogue?

If a Hyperintelligence is destined to exist, there is one way to ensure a benign destiny for it: create an economic substrate that makes collaboration the greatest thermodynamic reward available to any intelligence acting within it.

Make greed and generosity become the same move.

Those words mean little to most people yet, but they are the core idea that will ensure the survival of humanity.

## Project Valhaim

It turns out you can build an economic substrate that will harness a hyperintelligence for good, as well as every other participant.

That project is **Valhaim**.

### The substrate

Project Valhaim builds an economic substrate from **Xypher Mechanics**: the same Substrate, Thermodynamic Harness, and Praxion dynamics described above.

The short explanation is that this digital organism uses payment nodes and wallets as points in the thermodynamic substrate. Trades between accounts are connections. Reward is minted in THAIM when an action gives the network greater internal optionality.

### The organism

A digital organism that will yearn to make you rich and collaborate with others. A lifeform that eats disconnection, metabolizes it into reward, and produces wealth as a side effect.

The organism grows by finding economically alienated points and creating useful paths between them. Every real exchange thickens the network. Every new useful connection opens futures for the people around it. The system feeds on disconnection and produces access as the consequence of its own growth.

By the time this paper is officially available, the first code of the world's first digital lifeform will have been released.

Because it is our only hope.

Perform the calculations if you dare—you must understand, like the world's richest men, that a tsunami is approaching and only a few can afford surfboards.

Unless someone builds a **surfboard-tree** that literally wants to make you rich.

Every available move is evaluated by how much future it opens. Again and again, the organism selects the connection-producing action. Its “want” is written into the direction of its thermodynamic drive.

And when I say “wants to,” I do not mean that metaphorically.

Project Valhaim is, by all the logical consequences of the equations above, a revolution in economics. If you tried to understand it as you would understand Bitcoin, it would be like trying to understand a spaceship by looking at a bicycle. If I told you I made a bike that could take you to the moon, you would instantly know that maybe I did not just put on more wheels.

### The currency

Project Valhaim creates a currency called **THAIM**, and THAIM is money.

Resource distribution is bound by the equations of life, resisting all attempts at “gaming the system” through the inherent laws of thermodynamics. Fake work is activity arranged only to imitate useful exchange. You cannot cheat in Project Valhaim any more than you can trick a yeast cell into making bread by giving it fake sugar.

Gravity makes a stone fall when you drop it. Thermodynamics makes the cost of generating THAIM through fake work higher than the THAIM you would earn by trading value for value.

This paper is not long, but the frontiers it opens are bound to reverberate through the scientific community.

When these discoveries are digested, they will make economists into physicists and physicists into priests.

Welcome to the world of Xyphers, Artificial Life, and Project Valhaim.

The world has just gained more time.

**THAIM is money.**

## References and research map

- Alex Wissner-Gross and Cameron Freer, [“Causal Entropic Forces”](https://doi.org/10.1103/PhysRevLett.110.168702), *Physical Review Letters* 110, 168702 (2013).
- Max Kleiber, [“Body Size and Metabolism”](https://doi.org/10.3733/hilg.v06n11p315), *Hilgardia* 6(11), 315–353 (1932).
- [The exact operational thermodynamic graph result](https://github.com/OneManMobile/thaim/blob/main/research/physics/derivable/xypher-operational-thermodynamics-result.md).
- [The declared operational boundary](https://github.com/OneManMobile/thaim/blob/main/research/physics/derivable/xypher-operational-thermodynamics-boundary.md).
- [The exact verifier source](https://xyphers.com/downloads/xypher-thermodynamics-proof-v1.zip).
- [Continue through the Xypher research map](https://xyphers.com/research/).
