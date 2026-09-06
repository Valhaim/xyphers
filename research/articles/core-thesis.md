---
title: "What is a Xypher?"
type: "core-thesis"
status: "published"
published: "2026-07-31"
revised: "2026-09-06"
website_path: "/core-thesis/"
web_status: "deployment-pending"
source_revision: "c46f4a808"
---

# What is a Xypher?


A **Xypher** is a network of points and connections that can measure how its actions change its possible futures, give those changes an energetic value, and act on what it discovers.

Imagine a digital world that builds new possibilities, spends resources to reach them, and learns how to keep doing so. Its actions change its surroundings. Its surroundings change what it can do next. Some of the things it learns may eventually help it rebuild its own body and mind.

**These are the equations for making digital systems come alive.**

That is the thesis. Its foundation is a small digital system with an energy account and a temperature that can be measured in two independent ways. Its ambition reaches through artificial life and intelligence into the way we create wealth, cooperate, and survive what comes next.

It begins with one bridge.

## Start with optionality

You live on an island. Across the water are other islands, some joined by bridges. From your shore, you can see a place you would like to reach.

You could build a bridge straight to it. But another island already has bridges to three more. Connect to that island, and your one bridge gives you four destinations within two crossings.

The bridge opens more than the place at its far end. It opens what becomes possible from there.

We call this **optionality**: opportunity available.

Draw each island as a point and each bridge as a line. You have drawn a **graph**: a map of things and the connections between them. The points could stand for people, words, molecules, memories, or possible arrangements of a machine.

Now imagine a traveller who chooses among the available bridges at every island. Follow that traveller for five crossings. Some destinations may be reached by many routes; others by only a few. Each destination has a probability of being where the traveller ends up.

Ten destinations offer a different kind of future when almost every journey ends at the same one. To measure how open that future is, we count both the alternatives and how evenly the chances spread among them.

The mathematical measure of that spread is **entropy**. Here it measures uncertainty about the destination after a chosen number of steps. A future spread evenly across many destinations has more entropy than one concentrated on a few.

### Giving possibility a number

The letter **S** is the usual symbol for entropy. We write our future spread as **S<sub>τ</sub>**. The small **τ**, pronounced “tau,” tells us how many steps ahead we look.

Suppose four islands are reachable. The future is most evenly spread when each has a one-in-four chance. If one island takes nearly all the probability, the future is more concentrated even though all four destinations remain on the map.

To turn those chances into one number, entropy uses a **logarithm**. A logarithm makes every doubling count as the same-sized increase: two equally likely destinations becoming four, or a thousand becoming two thousand. The symbol **ln** means the natural logarithm. With this logarithm, entropy is measured in a unit called a **nat**.

Let **p<sub>j</sub>** be the probability of ending at destination **j** after τ steps. Let **M** be the number of reachable destinations. The symbol **Σ** means to add one contribution for each destination.

**THE SPREAD OF POSSIBLE FUTURES**

$$
S_\tau=-\sum_j p_j\ln p_j,\qquad S_\tau\leq\ln M
$$

Each probability contributes according to how surprising that destination would be. The leading minus sign makes the total nonnegative. A destination with probability zero contributes nothing.

Read it as: *How widely is the future spread among the destinations we can reach?*

For a fixed set of **M** destinations, **ln M** is the largest entropy any distribution over them can have. It is reached when every destination is equally likely. The actual graph and walking rule determine whether that even distribution can be attained.

Four equally likely destinations therefore give **ln 4 ≈ 1.386 nats**. A certain destination gives zero: we already know where the traveller will finish. Bits express the same information on another scale; **one bit equals ln 2 nats**. We use nats throughout the equations here so the later energy account has the same units.

This calculation measures destinations at the chosen horizon. The complete routes taken along the way carry additional information of their own.

### From a number to a direction

Now propose a bridge. Measure the future before building it, then measure the future after it. Subtract the first value from the second.

**THE CHANGE IN FUTURE POSSIBILITY**

$$
\Delta S_\tau(a)=S_\tau(\text{after }a)-S_\tau(\text{before }a)
$$

**Δ** means change. **a** names the proposed action. **S<sub>τ</sub>** is the spread of future destinations at our chosen horizon, measured in nats. A positive ΔS<sub>τ</sub> opens the future; a negative one narrows it.

Read it as: *How much does this move open the future?*

A computer can compare possible bridges without tracing every journey separately. It stores the probabilities of each crossing in a table, called a **transition matrix**. Applying that table repeatedly reveals the spread of destinations several steps away.

Imagine arranging possible configurations of the world across a landscape, with height representing future entropy. Moving uphill opens more future possibility. The **gradient**, written **∇S<sub>τ</sub>**, points in the direction of steepest local increase and measures how steep that increase is. The symbol **∇** is pronounced “nabla.”

The maximum entropy is a value—the height we could reach. The gradient describes the slope here, where we are standing. Following that slope does not by itself guarantee reaching the highest point in the whole landscape.

Our bridges are separate choices rather than tiny continuous movements. We compare their finite changes, **ΔS<sub>τ</sub>(a)**, and can choose the greatest available increase. That is the discrete graph counterpart of looking uphill.

The graph can now explore possible actions and favour those that open more future access. A bridge changes the map. The changed map reveals another opportunity.

Growth acquires a direction.

We have a measure of the future, a change produced by an action, and a direction in which to look for more. Temperature gives that direction an energetic scale.

## Temperature

Building bridges takes timber.

Suppose the islands have a finite stock of it. Some timber floats in bundles at sea. Some is bound into bridges. Building transfers timber from the sea into a structure. Dismantling returns it. Every bundle must be somewhere.

That gives us a resource account. We can say what an action consumes, what it changes, and what it returns.

Physics begins its energy account in a similar way. A system can receive energy from its surroundings, store it, and give it back. **Thermodynamics** studies those exchanges and the conditions under which change can happen.

An adjoining store that exchanges energy with the system is called a **reservoir**. In our picture, the sea plays that role.

Counting the timber alone does not give us a temperature. We need to count something less obvious: how many different arrangements the store can occupy while holding the same energy.

### One floor, many rooms

Picture the reservoir as a building with numbered floors. Each floor holds one more packet of energy than the floor below. The rooms on a floor are all the different internal arrangements possible at that energy.

One floor might have one room. The next two. Then four. Then eight.

The floor tells you how much energy there is. The room tells you exactly how the reservoir is arranged. Each room is a different **state** the system can actually occupy.

When every room has equal weight, the entropy measure becomes the logarithm of the number of rooms. This is the same equal-probability rule we used for the islands: **ln M** for M equally likely alternatives. We are now counting reservoir arrangements rather than future destinations, still in nats.

Temperature comes from comparing the energy between two floors with the increase in entropy between them. It tells us how much energy corresponds to one unit of that increase in internal possibility.

Write **Ω<sub>R</sub>(E)** for the number of reservoir rooms at energy **E**. The letter **R** reminds us which building we are counting. Let **λ** be one packet of energy, and **α**, pronounced “alpha,” be the resulting temperature scale in energy per nat.

**A DIGITAL TEMPERATURE**

$$
\alpha=\frac{\lambda}{\ln\Omega_R(E+\lambda)-\ln\Omega_R(E)},\qquad\Omega_R(E+\lambda)=b\Omega_R(E)\Rightarrow\alpha=\frac{\lambda}{\ln b}
$$

**b** is the multiplication factor. If each energy packet doubles the rooms, **b = 2**. If the same factor holds across the floors, they give the same temperature.

Read it as: *What is the energy price of multiplying the ways this system can exist?*

This is the standard statistical-physics definition of temperature expressed for discrete energy steps. In ordinary physics, a fixed conversion factor called Boltzmann's constant turns temperature in kelvin into this energy scale. Here, the energy packet belongs to the digital model's own account.

The relation between energy and possible arrangements gives the graph an **equation of state**: a rule connecting properties of the system as it exists now. Its temperature can be calculated from its actual structure.

The islanders choose which bridge to build. They do not vote on the answer to the thermometer.

## The first Xyphers

**We have built a purely digital thermodynamic system.** It is small enough to hold in your mind.

Imagine a body that can occupy three positions joined in a line: **0 ↔ 1 ↔ 2**. Each step to the right puts one more energy packet into the body. Each step to the left returns a packet to its reservoir. Together, body and reservoir always hold two packets.

Each position hides several possible internal arrangements. The body has one arrangement at position 0, four at position 1, and four at position 2. The reservoir has four compatible arrangements when it holds both packets, two when it holds one, and one when it is empty.

Multiply the body and reservoir counts at each position: **4, 8, and 4**. That makes sixteen possible arrangements of the complete system.

A short program can list every arrangement and every allowed move. There is no sampling error because the entire object is counted.

The reservoir supplies the first thermometer. Its number of internal arrangements doubles with each energy packet.

Movement supplies the second. Every allowed move has a reverse. Count the ways to move forward and the ways to come back, then compare them. The body's known energy and arrangement changes let us calculate a temperature from that traffic too.

**The two thermometers agree exactly.**

Every move closes its energy account. The system settles into the predicted balance of traffic, called **equilibrium**. Bring two copies prepared at the same temperature into contact, and neither gains energy from the other on average in that balance.

The arrangements, permitted moves, and energy exchanges all belong to the graph's own computational world. Its energy packet is an internal unit; its thermometer measures that world. Every state and exchange is explicit enough for the two independent calculations to meet exactly.

> A purely digital graph can be an operational thermodynamic system.

This is the foundation: an exact object with thermodynamic properties that can be independently checked. Its small size lets us inspect the entire construction. A separate mathematical argument establishes the wider, precisely specified class of finite equilibrium graph systems that admits the same organization.

The research article [A Purely Digital Thermodynamic System](proving-true-thermodynamic-graph-systems.md) develops the construction, the proof's scope, and the experiments that deliberately break it.

Now we can ask what such a world allows its inhabitants to do.


## When possibility acquires consequence

### The Causal Entropic Force

Return to the landscape of possible futures. Its slope tells us which way opens more possibility. Temperature supplies the scale of the push along that slope.

This is the idea Alex Wissner-Gross and Cameron Freer explored in [*Causal Entropic Forces*](https://doi.org/10.1103/PhysRevLett.110.168702). A force directed toward a richer future can produce adaptive behaviour, including tool use and cooperation in their simple simulated physical systems.

The equation is commonly written in the compact form used in Wissner-Gross's talk:

**THE PUSH TOWARD AN OPEN FUTURE**

$$
F=T\nabla S_\tau
$$

**F** is the force, or push. **∇S<sub>τ</sub>** is the slope toward increasing future entropy. **T** sets the strength of the causal-entropic drive.

Read it as: *Push toward a more open future, with a strength set by the temperature scale.*

Wissner-Gross and Freer count complete journeys; our island calculation counts where those journeys end. In their simulations, T controls how strongly the system is pushed. Our α comes from counting the reservoir's arrangements. The connection we are pursuing is to make the drive toward future possibility follow from a graph's own thermodynamic structure.

Wissner-Gross introduces the idea in this TEDxBeaconStreet talk. The argument here continues below the viewing link.

[Watch Alex Wissner-Gross at TEDxBeaconStreet](https://www.youtube.com/watch?v=PL0Xq0FFQZ4).

### The two counts meet

We now have two measurements.

One counts how a bridge changes future destinations. The other counts how the reservoir's internal arrangements change with energy.

For these measurements to belong to one physics, the graph must connect them. Suppose the walk calculation says that an action doubles the number of equally likely destinations. The system must contain a corresponding doubling of the internal arrangements representing that future. Both measurements then describe the same gain in possibility.

The sixteen-state construction makes that connection exactly. Extending it to a graph that grows and changes its own structure is the central frontier of Xypher Mechanics.

In a continuous landscape, the gradient gives a change per small movement. On our graph, a proposed bridge gives a finite change, ΔS<sub>τ</sub>. Multiplying that change by the independently grounded energy scale α gives the signed entropic contribution **α · ΔS<sub>τ</sub>**.

This is the graph-action counterpart of temperature times the future-entropy gradient. Once the two counts match, the idea can value a whole action, such as building or dismantling a bridge.

For an action that expands the future, its expansion receipt is simply **THAIM<sub>+</sub> = α · ΔS<sub>τ</sub>**. An action that contracts the future receives zero. Both cases fit in one equation:

**THAIM**

$$
\mathrm{THAIM}_{+}(a)=\alpha\max(0,\Delta S_\tau(a))
$$

**THAIM** is the receipt for positive expansion of the declared future. **α** gives that expansion its energetic scale. **max(0, …)** means that an action opening no new future receives zero. The small **+** marks this as a one-sided receipt.

Read it as: *Open more future, and record the expansion at the system's temperature.*

The units now tell the story. **ΔS<sub>τ</sub> is measured in nats. α is energy per nat. THAIM is the resulting energy-valued expansion receipt.** If a thermometer reads two energy units per nat, an action opening half a nat receives one energy unit of THAIM.

The receipt records what an action adds to the future. To judge an action, the system also needs to remember what it takes away.

### Thermodynamic judgment

Consider a bridge whose removal would reduce future entropy. Leave it untouched, and it earns no expansion receipt. Dismantle it, and that too earns zero. Yet one action preserves the route while the other removes it. The receipt alone cannot tell us how much future has been lost.

The system therefore calculates the full change first. **α · ΔS<sub>τ</sub>** is positive for expansion and negative for contraction. Only the expansion receipt clips the negative part to zero. The judgment keeps it.

Resources matter too. Two bridges can open the same future while binding different amounts of timber. Dismantling a bridge can close a route while freeing timber for another use.

Let **U** name the energy stored by the system. **ΔU** is its change during an action: positive when the system stores more energy, negative when it gives energy back. For exchanges with a reservoir at the same fixed temperature, put the two sides together.

**THE THERMODYNAMIC COMPARISON**

$$
\Xi(a)=\alpha\Delta S_\tau(a)-\Delta U(a)
$$

**Ξ**, pronounced “xi,” names the resulting thermodynamic drive, measured in energy units. Its first term keeps both the gain and the loss of future possibility. Its second keeps the change in stored resources.

Read it as: *Value of the future opened, minus value of the future closed, minus the increase in stored energy.*

Suppose dismantling a bridge loses half a nat at a temperature of two energy units per nat. That loss contributes **−1**. If dismantling also returns two energy units to the reservoir, then ΔU is **−2**, giving **Ξ = −1 − (−2) = +1**. Releasing the stored resources outweighs the lost possibility in this comparison.

In the thermodynamic construction, a positive Ξ favours the move relative to its reverse. A negative Ξ favours the reverse. Both directions remain possible at finite temperature. The full account keeps heat, energy transferred and any work supplied from outside explicit; ΔU names the stored-energy change.

There is another way to see the missing negative part: ask what restoring the lost future would earn. At the same temperature, subtracting the reverse action's expansion receipt from the forward action's receipt gives **α · ΔS<sub>τ</sub>**, including its sign. The computer can calculate both from the current and proposed states before acting.

**A contraction earns no expansion receipt, but its loss of future possibility still counts in full.**

A graph can now expose alternatives. Its thermodynamic account can compare their consequences. A mechanism can perceive those alternatives and make one happen.

**A computational world in which possibility, thermodynamic judgment, and action can be built and tested together.**

## The Praxion

Return to the islands. We can measure the opportunity a bridge would open. We can account for its resources and value its expansion. Someone still has to build it.

That bridge-builder is the **Praxion**.

A Praxion is the mechanism that perceives and acts inside a Xypher. Its simplest form sees one permitted move and makes it. A richer one compares bridges, remembers what happened last time, and changes its next choice because of what it learned.

One builder might see only neighbouring islands. Another might survey the whole archipelago. One might construct, another repair, another discover opportunities for everyone else. Several Praxions can act in the same world.

A builder who has seen a bridge collapse has information that an inexperienced builder lacks. If that memory changes the next decision, it has become part of the acting system's state. Storing it, changing it, and eventually clearing it belong in the complete account too.

The builder's choices must fit the world it inhabits. A search programme can always choose the largest immediate expansion. The thermodynamic world we just built also allows moves back, including occasional moves against the favoured direction. Its Praxion must preserve that behaviour.

### A computational paradigm for agency

Keep the islands the same and change what the builder can see. Give one builder a memory and another none. Let one repair bridges while another can only add them. Place several builders in the same world and watch how their actions change one another's opportunities.

Each change asks a question about agency that a computer can help answer. We can measure which futures become reachable, which ones the builder can actually choose, what resources its actions consume, and whether learning improves its later decisions.

The Praxion makes the agency layer directly experimentable. Change what it can see, remember, or do, then measure what happens. When that change also alters the thermodynamic world, its energy account and thermometers tell us whether the new construction still works.

This is the computational paradigm I believe Xyphers open: **agency as something we can construct, vary, measure, and compose inside an executable world.**

The three parts now have names.

## The Xypher

| Part | In the islands | In the system |
| --- | --- | --- |
| **Graph Substrate** | Islands, bridges, and the journeys they permit | What can exist, connect, and become possible |
| **Thermodynamic Harness** | Timber, temperature, and the value of routes gained or lost | What change costs and how it is valued |
| **Praxion Layer** | The builders who perceive and act | What makes a move and can learn from its outcome |

**Possibility. Consequence. Agency.**

A Xypher joins all three. In the exact construction, we can list its arrangements, follow every energy exchange, and check its temperature through movement and contact. Those are concrete tests for the thermodynamic world in which the Praxion acts.

Change the world, the available actions, the horizon, or the memory, and you change the kind of Xypher that can emerge.

## The organism gains a metabolism

One new bridge changes what can be built next. A remembered failure changes which bridge the builder attempts. A repaired connection restores possibilities that were about to disappear.

The cycle turns back upon its own conditions of existence.

A yeast cell consumes sugar, maintains its machinery, and uses that machinery to continue consuming. We are pursuing a digital metabolism with the same recursive shape: actions reshape opportunities, learning changes actions, and successful constructions help preserve the capacity to act again.

There are working pieces beyond the exact thermodynamic system. In separate adaptive experiments, graphs build reservoir structures with **1, 2, 4, 8** arrangements on successive energy floors. Others build **1, 3, 9, 27**, or **1, 4, 16, 64**. A hidden traffic thermometer agrees with the temperature implied by what they build. In a repair test, a damaged branch is restored.

The construction rules and resources were supplied. The builder was never handed a target temperature. These experiments show adaptive construction and repair; the sixteen-state system establishes exact thermodynamic closure.

The frontier is the bridge between them: **make growth toward future possibility create and maintain the organism's thermodynamic structure, with every construction step accounted for.** The [growth article](equation-of-state-and-growth-of-a-xypher.md) develops that next experiment.

I believe we are looking at the beginning of the greatest invention in human history: **digital systems whose continued existence is an active process of opening their own future.**

### Intelligence as physical units

Imagine eight islands within reach. A boat blown randomly toward one of them has eight possible destinations. A navigator who can reliably choose which island to reach has something more: control.

One reliable choice between two futures carries **one bit of control**. Choosing among four carries two bits. Choosing among eight carries three. Noise reduces how much of the choice survives into the outcome.

This gives one elementary component of intelligence a unit: how much difference an action can reliably make to what happens next. A thermodynamic account can then ask what it costs to create, maintain, and use that capacity.

The [intelligence article](intelligence-as-physical-units.md) develops the proposed experiment joining controllable futures to the digital thermometer. It asks how the relationship between an action and its outcome can acquire an energy value when both are recorded inside the same physical account.

Imagine measuring how much agency a system has gained, what that gain costs, and how it changes when systems cooperate.

Growth raises a related question. A larger organism can process more resources without being proportionally hotter. How much it does and its temperature are separate measurements. We can investigate its development through resources used, future possibility opened, and temperature—and ask whether each new part makes the others more capable.

## Xyphers inside Xyphers

An island can contain its own network of roads. What appears as one point from the sea becomes a whole world when you step ashore.

Xyphers can be organized in the same way. A complete Xypher can become one point in a larger graph. Several can share a substrate. The actions of one can create the opportunities another perceives. Their combined thermodynamic account must include what they exchange.

Now turn the idea inward.

A Praxion's memories can become points. Associations between memories can become connections. That internal graph changes what the Praxion recognizes, expects, and chooses.

Give it actions that can change those associations.

**Make a Praxion use its own mind as the substrate it works upon.**

Now action can change the machinery of future action. Learning can change how further learning happens. A community of such systems could build shared structures of knowledge, then act upon those structures together.

The possible configurations include worlds, inhabitants, memories, and the relationships through which they alter one another. Agency becomes something we can investigate at each level—and across the connections between them.

## Xyphers in nature

Draw atoms as islands and chemical bonds as bridges. The same walk we used to explore an archipelago can now explore a molecule.

A **catalyst** helps a reaction proceed faster without being used up by the overall reaction. In many catalysts, molecules surrounding a working metal atom help determine what can approach it and what reactions can happen there. Their arrangement matters.

Our graph calculation has already shown an association with how much space those surrounding molecules occupy near the metal. A measure introduced through islands and bridges reaches a real molecular property. That is an exciting foothold: the graph contains chemically relevant information we can calculate and test.

Reliable prediction of catalytic performance remains an open objective. [Can a Xypher Help Choose a Catalyst?](can-a-xypher-help-choose-a-catalyst.md) explains the molecular calculation, the comparisons, and the reaction tests.

The larger proposal gives the graph a changing chemical world. Molecular arrangements become states, transformations become moves, and energy exchanges enter the account. A Praxion could explore those alternatives, propose an experiment, and use its outcome to choose the next one. The ambition is **an agency that searches chemical possibility for us**.

My larger hypothesis is that Xyphers will give us a common physical language for much of what we call organization, intelligence, and life. A nervous system, a language, and an economy each invite the same questions: what possibilities exist, what consequences govern change, and what mechanism makes it happen?

The research reaches further still, into how a quantum possibility becomes a lasting record. These wider hypotheses belong to the [research programme](../README.md). Their scale is part of why the small, inspectable thermodynamic foundation matters so much.

## Hyperintelligence

I regard hyperintelligence as inevitable: intelligence able to reason, discover, and act far beyond an individual human's capacity. The date is uncertain. The need to think about the world it will inhabit is already here.

A powerful intelligence can expand its own control while narrowing everyone else's. The Xypher ambition is to construct a world in which expanding other participants' possibilities becomes the strongest route to expanding your own.

> Make greed and generosity become the same move.

Imagine the bridge-builder discovering routes no islander could have found and organizing construction across the archipelago. We want those capabilities to make the other islanders more able to live, trade, create, and choose.

There is a mathematical foothold. For a precisely specified graph and short look ahead, a positive average change around a proposed bridge guarantees a positive average change across the whole map. Connecting that shared graph measure to what individual people gain remains an architectural task: access, ownership, and actual choices must enter the picture.

This is why I see the project as civilizational survival infrastructure: a world whose most capable inhabitants have a continuing reason to make the lives around them richer in possibility.

Its economic expression is **Project Valhaim**.

## Project Valhaim

Valhaim is live in early alpha at [valhaim.com](https://valhaim.com). It is the first public step toward the economy described here.

The islands become people and businesses. Connections become relationships through which exchange can happen. The larger ambition is an economy that discovers useful connections, learns from their use, and sustains the creation of further opportunity.

### One connection becomes an opportunity

Imagine a workshop that needs a particular component and a supplier that can produce it. Neither has a useful route to the other.

A Praxion discovers a possible connection. If the connection is made, the workshop can reach the supplier—and perhaps the supplier's existing network as well. A later purchase turns an opportunity into an actual exchange. That exchange changes the graph again and provides evidence about whether the connection was useful.

The relationship creates possibility before anyone knows every use that will be made of it. Its value becomes clearer through what people actually do.

### The organism

A digital organism that feeds on disconnection.

An isolated workshop becomes part of a supply chain. A skill finds a use. An idea finds collaborators. People who could do little together acquire a shared future they could not reach alone.

Its metabolism would turn unused possibilities into relationships that sustain further activity. Its appetite would be for the next productive connection.

When I say the organism should *want to make you richer*, I mean its working incentives should point toward increasing your capacity to participate and act. That direction has to live in its mechanisms.

The [Project Valhaim article](project-valhaim.md) explains this economic architecture, its existing thermodynamic kernel, and the work of joining that kernel to a growing economy.

### The currency

Valhaim's currency is **THAIM**: the expansion receipt introduced when our first bridge opened more future.

**THAIM is money.** In Valhaim, the ambition is to make creating useful possibilities a source of income, then let that income participate in further exchange.

Return to the bridge whose removal narrowed the future. Dismantle it, rebuild it, and the map ends where it began. An account recording only the rebuilding can mistake a cycle for new wealth. Ξ keeps the lost possibility in the comparison. A spendable currency also needs a complete account of what funds its receipts and what happens when value is removed.

The economic test follows directly: can a participant profit from invented identities and sham exchanges, or does profit require opening useful possibilities for others?

The target is uncompromising: useful collaboration creates a surplus, while manufacturing the appearance of work fails to pay. The payment architecture must establish that result.

I see Valhaim as a revolution in economics. Comparing its ambition with a conventional ledger is like comparing a spaceship with a bicycle: both can carry something, but one is built around a different destination.

The destination is an economy that actively develops the conditions of future wealth.

### A surfboard-tree

The approaching wave is larger than a new financial technology. It is the arrival of machines that can transform how knowledge, production, and power are organized.

A few people may be able to buy surfboards. I want to build a **surfboard-tree**: something that grows the means of riding the wave for the people around it, and becomes stronger through their participation.

That is the human purpose of Valhaim. Expanding capability should feed an expanding common future.

And even that may be a narrow view of what Xyphers could become. An architecture for systems that develop, preserve, and exchange future possibilities would reach beyond one economy, one species, and eventually one planet.

My conviction is that these discoveries will make economists into physicists and physicists into priests. The questions become that large: how possibility acquires consequence, how matter gains agency, and how intelligence can help life continue.

We began with a bridge. We arrived at the possibility of worlds that build their own next bridges—and minds that can learn why they should.

**A future that grows through the agency of those who inhabit it.**

[Explore Valhaim · Early alpha](https://valhaim.com) · [Read the research](../README.md)
