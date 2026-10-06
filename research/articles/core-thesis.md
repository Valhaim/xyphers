---
title: "What is a Xypher?"
type: "core-thesis"
status: "published"
published: "2026-07-31"
revised: "2026-10-06"
website_path: "/core-thesis/"
web_status: "live"
source_revision: "aabf2496a"
---

# What is a Xypher?


A **Xypher** is a network of points and connections that can measure how much its actions expand the future, place a thermodynamic value on those actions, and act on what it finds.

Digital systems are not normally thermodynamic. A program can grow, compute, and change, but nothing inside it has a temperature of its own. When software reports a temperature, it is describing something outside itself, or reading back a number someone set. The heat a computer gives off belongs to its chips, not to the world the program describes.

We discovered how to build digital systems that are thermodynamic in their own right, and we have the proof. It is a small digital world whose temperature nobody chose. You can count that temperature from the world's own structure, and the world's own motion confirms it, exactly. Every state can be listed and every move checked, and the program that checks them runs on an ordinary computer in seconds.

In bolder words: **These are the equations for making digital systems come alive.** I mean that literally.

The discovery opens avenues that were closed before. Once a digital world has a real temperature, information inside it has a real energy price. That opens the derivation of intelligence in physical units: how much control a system has over its own future, measured in bits, and what that control is worth in energy. And a digital system that builds and repairs the structure behind its own temperature has the shape of life. We have built that too. I believe this is an extraordinary leap in how we understand intelligence and life themselves.

The idea at the root comes from an overlooked piece of physics, the causal entropic force: a push toward whatever keeps the most futures open. Its equation needs a temperature. In the original simulations, that temperature was a setting the researchers chose. A Xypher supplies its own.

A Xypher has three parts, and each answers one question:

- **Graph Substrate**: the points, the connections, and the futures they make possible. *What futures can this network reach?*
- **Thermodynamic Harness**: the energetic value of change. *What is a change worth?*
- **Praxion Layer**: the mechanism that perceives and acts. *What makes the change happen?*

We answer them in that order. By the end of the second, you will be able to count the temperature yourself.

It begins with one bridge.

## Start with optionality

You live on an island. Around you are many others, some joined by bridges. You would like to reach one of them.

You could build a bridge straight to it. But another island already has bridges to three more. Connect to that one instead, and a single bridge gives you four destinations within two crossings.

A bridge opens more than the place at its far end. It opens what becomes possible from there.

We call this **optionality**: opportunity available.

Draw each island as a point and each bridge as a line, and you have a **graph**: a map of things and the connections between them. The points could be people, words, molecules, memories, or possible arrangements of a machine.

Now ask the question for every bridge you could build: how much would it improve the reach of the whole island system?

### Giving possibility a number

Imagine a traveller who picks a bridge at random on every island, and follow them for five crossings. Some destinations can be reached by many routes, others by only a few. Each destination gets a probability of being where the traveller ends up.

Ten reachable destinations mean little if almost every journey ends at the same one. An open future needs many alternatives and chances spread evenly among them.

The measure of that spread is **entropy**. Here it measures how uncertain we are about where the traveller will be after a chosen number of steps. A future spread evenly across many destinations has more entropy than one concentrated on a few.

The letter **S** is the usual symbol for entropy. We write the spread of possible futures as **S<sub>τ</sub>**. The small **τ**, pronounced “tau,” says how many steps ahead we look.

Suppose four islands are reachable. The future is most open when each has a one-in-four chance. If one island takes nearly all the probability, the future has narrowed, even though all four remain on the map.

Entropy turns those chances into one number with a **logarithm**, which makes every doubling count the same: two equally likely destinations becoming four adds as much as a thousand becoming two thousand. The symbol **ln** means the natural logarithm. With this logarithm, entropy is measured in a unit called a **nat**.

Let **p<sub>j</sub>** be the probability of ending at destination **j** after τ steps, and **M** the number of reachable destinations. The symbol **Σ** means to add one contribution for each destination.

**THE SPREAD OF POSSIBLE FUTURES**

$$
S_\tau=-\sum_j p_j\ln p_j,\qquad S_\tau\leq\ln M
$$

Each destination contributes its surprise, weighted by how likely it is. The leading minus sign makes the total positive. A destination with probability zero contributes nothing.

Read it as: *How widely is the future spread among the destinations we can reach?*

For **M** destinations, **ln M** is the largest possible entropy, reached when every destination is equally likely. Four equally likely destinations give **ln 4 ≈ 1.386 nats**. A certain destination gives zero: we already know where the traveller will finish. Bits measure the same thing on another scale; **one bit equals ln 2 nats**. We use nats throughout so that the energy account later shares the same units.

### From a number to a direction

Now propose a bridge. Measure the future before building it and after, and subtract.

**THE CHANGE IN FUTURE POSSIBILITY**

$$
\Delta S_\tau(a)=S_\tau(\text{after }a)-S_\tau(\text{before }a)
$$

**Δ** means change, and **a** names the proposed action. A positive ΔS<sub>τ</sub> opens the future; a negative one narrows it.

Read it as: *How much does this move open the future?*

Now picture every way the map could be, spread across a landscape, with height standing for future entropy. Uphill opens the future. The **gradient**, written **∇S<sub>τ</sub>**, points in the direction of steepest ascent and says how steep it is. The symbol **∇** is pronounced “nabla.”

Bridges are separate choices rather than tiny continuous steps, so we compare their finite changes, **ΔS<sub>τ</sub>(a)**, and can pick the greatest. That is the graph's way of looking uphill.

Build the best bridge. Measure again. Build the next. Each new bridge reveals the next opportunity, and the network grows toward the most open future it can reach.

But nothing in this world costs anything. Every bridge is free, and a nat of future is worth whatever we say it is. That is growth, not physics: a fun little mechanic with no punch.

Here comes the kicker. What if the price of opening the future came from the network's own temperature?

## Temperature

Building bridges is hard work. It takes timber.

Suppose the islands share a fixed stock of it. Some floats in bundles at sea; some is bound into bridges. Building moves timber from the sea into a structure. Dismantling returns it. Every bundle is always somewhere.

That is a resource account: what an action consumes, what it changes, and what it gives back. Physics keeps its energy account the same way. A system receives energy from its surroundings, stores it, and gives it back. **Thermodynamics** studies those exchanges and the conditions under which change can happen. The adjoining store that trades energy with the system is called a **reservoir**. In our picture, the sea plays that role.

Counting timber alone does not give a temperature. For that we must count something less obvious: how many different ways the store can be arranged while holding the same energy.

### One floor, many rooms

Picture the reservoir as a building with numbered floors. Each floor holds one more packet of energy than the floor below. The rooms on a floor are all the different internal arrangements possible at that energy.

One floor might have one room. The next, two. Then four. Then eight.

The floor tells you how much energy there is. The room tells you exactly how the reservoir is arranged. Each room is a different **state** the system can actually occupy.

When every room has equal weight, entropy is the logarithm of the number of rooms: the same **ln M** we used for equally likely islands, now counting arrangements instead of destinations, still in nats.

Temperature compares the energy between two floors with the increase in entropy between them. It says how much energy buys one unit of that increase in internal possibility.

This is where hot and cold come from. A store that gains many rooms from each extra packet runs cold: energy buys it a lot of new possibility, so it pulls energy in. A store that gains few rooms per packet runs hot and lets energy go. Heat flows from hot to cold because that is where the rooms multiply fastest.

Write **Ω<sub>R</sub>(E)** for the number of reservoir rooms at energy **E**; the **R** reminds us which building we are counting. Let **λ** be one packet of energy, and **α**, pronounced “alpha,” the resulting temperature scale in energy per nat.

**A DIGITAL TEMPERATURE**

$$
\alpha=\frac{\lambda}{\ln\Omega_R(E+\lambda)-\ln\Omega_R(E)},\qquad\Omega_R(E+\lambda)=b\Omega_R(E)\Rightarrow\alpha=\frac{\lambda}{\ln b}
$$

**b** is the multiplication factor. If each energy packet doubles the rooms, **b = 2**, and every floor gives the same temperature.

Read it as: *What is the energy price of multiplying the ways this system can exist?*

This is the standard statistical-physics definition of temperature, written for discrete energy steps. In ordinary physics, Boltzmann's constant converts temperature in kelvin into this energy scale. Here, the energy packet belongs to the digital model's own account.

A rule like this, connecting the energy of a system to the number of ways it can exist, is an **equation of state**. Count the rooms, and the temperature follows.

The islanders choose which bridge to build. They do not vote on what the thermometer reads.

## The first Xyphers

Can a digital world have a temperature like this, one that nobody chose?

**We have built a purely digital thermodynamic system.** It is small enough to hold in your mind.

Imagine a body that can sit in three positions joined in a line: **0 ↔ 1 ↔ 2**. Each step to the right moves one energy packet from the reservoir into the body. Each step to the left returns it. Body and reservoir always hold two packets between them.

Each position hides several internal arrangements. The body has one arrangement at position 0, four at position 1, and four at position 2. The reservoir has four arrangements when it holds both packets, two when it holds one, and one when it is empty.

Multiply body and reservoir at each position: **4, 8, and 4**. That makes sixteen possible arrangements of the complete system. Draw each arrangement as a point and each allowed move between them as a line, and this world is a graph. A short program lists every arrangement and every allowed move. Nothing is sampled, so there is no sampling error.

Now read its temperature twice.

The first reading counts. The reservoir's rooms double with each energy packet, so the floors-and-rooms equation gives **α = λ / ln 2**.

The second reading watches the world move. Let it run: at each moment one allowed move happens, picked at random, with no move favoured over another. Then count the ways each step can happen. From any arrangement at position 1, there are four ways to step forward to position 2. From any arrangement at position 2, there are eight ways to step back. The body has four arrangements on both sides, so the whole difference comes from the packet: leaving the reservoir halves its rooms. A factor of two per packet is ln 2 nats per packet, so the moves alone give **α = λ / ln 2**. The step from 0 to 1, where the body's rooms change too, gives the same answer.

**The two thermometers agree exactly.**

That agreement is the proof's heart. A temperature someone declares is only a label: nothing in the motion has to respect it. Here the motion was counted on its own, move by move, and it lands on the number the rooms predict. The temperature is not a label we put on this world. It is a property the world has.

A real temperature must also pass the tests every physical one passes, and this one does. Energy is never created or lost: every packet is always in the body or in the reservoir. Left alone, the world settles where the flow forward across every step equals the flow back, a balance called **equilibrium**. And prepare two bodies, each against a reservoir at the same temperature, then let the bodies trade packets with each other: on average, neither gains energy from the other. Nothing flows between things at the same temperature. That is what the phrase means for anything you can touch.

All of this belongs to the graph. The energy is counted in the world's own packets, not drawn from the electricity of the computer running it, and the thermometers measure that world.

> A purely digital graph can be a thermodynamic system: not by analogy, but by the tests that define one.

Pause here for a moment. Digital systems do not normally have an equation of state. They are not normally thermodynamic, because they are not normally… alive.

This one has one. The reservoir's doubling rooms fix its temperature, and every move in the world obeys it.

Is this a curiosity of sixteen states? No. A separate mathematical proof shows that the same organization fits a whole class of graph systems: any finite one that settles at a single temperature and lets every move be undone.

[A Purely Digital Thermodynamic System](proving-true-thermodynamic-graph-systems.md) builds the construction step by step, sets out the proof and its limits, and runs the experiments that deliberately try to break it.

We now hold both halves: a measure of how much future an action opens, and a temperature that is a fact about the world. Physics already has an equation that joins them.

## When possibility acquires consequence

### The Causal Entropic Force

Return to the landscape of possible futures. Its slope points toward more possibility. Alex Wissner-Gross and Cameron Freer asked what happens when something is pushed along that slope, with a strength set by temperature. In [*Causal Entropic Forces*](https://doi.org/10.1103/PhysRevLett.110.168702), their simple simulated systems responded with adaptive behaviour, including tool use and cooperation.

The equation is usually written in the compact form from Wissner-Gross's talk:

**THE PUSH TOWARD AN OPEN FUTURE**

$$
F=T\nabla S_\tau
$$

**F** is the force, or push. **∇S<sub>τ</sub>** is the slope toward greater future entropy. **T** sets the strength of the drive.

Read it as: *Push toward a more open future, with a strength set by the temperature scale.*

In their simulations, T is a dial: the researchers choose how hard the system is pushed. In a Xypher, the temperature is meant to be the one we just counted from the rooms, so that the push toward a more open future comes from the world's own structure rather than from a setting. That takes one more step.

Wissner-Gross introduces the idea in this TEDxBeaconStreet talk. The argument continues below the viewing link.

[Watch Alex Wissner-Gross at TEDxBeaconStreet](https://www.youtube.com/watch?v=PL0Xq0FFQZ4).

### The two counts meet

To price a bridge, we multiply: the future it opens, times the price of a nat. But the price was counted from rooms in the reservoir, and the future from the destinations of a traveller. A price per kilo of apples tells you nothing about a kilo of pears. For α to price the future, a nat of future and a nat of rooms must be the same kind of thing: when a move doubles the equally likely futures, the system must contain a matching doubling of the arrangements that carry them.

The sixteen-state system joins them by construction. Besides stepping left and right, its body can reshuffle among its own arrangements without moving. At position 1, one step ahead, it could be in any of its four arrangements, all equally likely: **ln 4** nats of future. Those four futures are the body's four rooms. Here a nat of future is a nat of rooms, and the temperature prices both.

The traveller's five crossings are harder. We have not yet shown that the spread of a many-step journey across a map is a count of rooms. Showing it, and keeping it true while the graph grows and rebuilds itself, is the central frontier of this research.

Where the two counts meet, multiplying a change in future by the temperature gives its energy value, **α · ΔS<sub>τ</sub>**. This is the causal entropic force, made into discrete steps on a graph: positive when an action opens the future, negative when it closes it. It values a whole action, such as building or dismantling a bridge.

To reward whoever opens the future, we also keep a receipt for expansion alone. An action that expands the future earns a receipt for that expansion. An action that contracts the future earns zero. One equation holds both cases:

**THAIM**

$$
\mathrm{THAIM}_{+}(a)=\alpha\max(0,\Delta S_\tau(a))
$$

**THAIM** is the receipt for future opened. **α** gives that expansion its energetic scale. **max(0, …)** means an action that opens no new future receives zero. The small **+** marks the receipt as one-sided.

Read it as: *Open more future, and record the expansion at the system's temperature.*

The units tell the story. **ΔS<sub>τ</sub> is measured in nats. α is energy per nat. THAIM is the resulting energy-valued expansion receipt.** If a thermometer reads two energy units per nat, an action opening half a nat earns one energy unit of THAIM.

The receipt records what an action adds to the future. To judge an action, the system must also remember what it takes away.

### Thermodynamic judgment

Consider a bridge whose removal would shrink the future. Leave it alone, and it earns no expansion receipt. Dismantle it, and that earns zero too. Yet one action preserves the route and the other destroys it. The receipt alone cannot tell how much future was lost.

So the system calculates the full change first. **α · ΔS<sub>τ</sub>** is positive for expansion and negative for contraction. The receipt clips the negative part to zero; the judgment keeps it.

Resources matter too. Two bridges can open the same future while binding different amounts of timber. Dismantling a bridge can close a route while freeing timber for another use.

Let **U** be the energy stored by the system, and **ΔU** its change during an action: positive when the system stores more, negative when it gives energy back. For exchanges with a reservoir at one fixed temperature, put the two sides together.

**THE THERMODYNAMIC COMPARISON**

$$
\Xi(a)=\alpha\Delta S_\tau(a)-\Delta U(a)
$$

**Ξ**, pronounced “xi,” is the resulting thermodynamic drive, measured in energy. Its first term keeps both the gain and the loss of future possibility. Its second keeps the change in stored resources.

Read it as: *Value of the future opened, minus value of the future closed, minus the increase in stored energy.*

Suppose dismantling a bridge loses half a nat at a temperature of two energy units per nat. That loss contributes **−1**. If dismantling also returns two energy units to the reservoir, ΔU is **−2**, and **Ξ = −1 − (−2) = +1**. Releasing the stored resources outweighs the lost possibility.

A positive Ξ makes a move more likely than its reverse; a negative Ξ favours the reverse. In a world with a temperature, both directions remain possible: steps back happen too, only less often. If someone carries timber in from outside, that is a separate entry in the books: ΔU counts only what the system itself stores.

There is another way to recover the missing negative part: ask what restoring the lost future would earn. At the same temperature, the forward action's receipt minus the reverse action's receipt is exactly **α · ΔS<sub>τ</sub>**, sign included. A computer can calculate both from the current and proposed states before acting.

**A contraction earns no expansion receipt, but its loss of future possibility still counts in full.**

A graph can now expose alternatives, and its thermodynamic account can weigh their consequences. One question is left: what makes a move happen?

## The Praxion

We have a world that can grow and a physics that values its growth. But a fountain without water pressure is only a basin. Something has to make the moves.

That mover is the **Praxion**: the mechanism that perceives and acts inside a Xypher. In the sixteen-state world, it is as simple as a Praxion can be: it makes one permitted move at a time, at the rates its world's physics allows, and its moves are exactly the ones we counted for the second thermometer. On the islands, the Praxion is the bridge-builder. A richer one compares bridges, remembers what happened last time, and changes its next choice because of what it learned.

Several Praxions can share one world. One surveys the whole archipelago. Another repairs. Another discovers opportunities for everyone else.

A builder who has seen a bridge collapse knows something an inexperienced builder does not. That knowledge is not free. It is stored somewhere in the world, and storing it, updating it, and eventually forgetting it all enter the energy account.

The builder must also move the way its world moves. A builder that only ever grabbed the biggest gain would never take a step back. But the second thermometer read the temperature from steps forward and steps back. Take away the steps back, and the temperature we counted would no longer describe the world. So the Praxion acts as the physics does: mostly in the favoured direction, sometimes against it.

### A computational paradigm for agency

Keep the islands the same and change the builder. Give one a memory and another none. Let one repair bridges while another can only add them. Put several in the same world and watch their actions change one another's opportunities.

Each change asks a question about agency that a computer can answer. We can measure which futures become reachable, which of them the builder can actually choose, what its actions consume, and whether learning improves its later decisions. When the change also alters the thermodynamic world, the energy account and the thermometers tell us whether the construction still holds.

This is the computational paradigm I believe Xyphers open: **agency as something we can construct, vary, measure, and compose inside an executable world.**

## The Xypher

Together, the three parts form a Xypher.

| Part | In the islands | In the system |
| --- | --- | --- |
| **Graph Substrate** | Islands, bridges, and the journeys they permit | What can exist, connect, and become possible |
| **Thermodynamic Harness** | Timber, temperature, and the value of routes gained or lost | What change costs and how it is valued |
| **Praxion Layer** | The builders who perceive and act | What makes a move and learns from its outcome |

**Possibility. Consequence. Agency.**

Change the world, the available actions, the horizon, or the memory, and you change the kind of Xypher that emerges.

## The organism gains a metabolism

Every action changes the world the next action starts from. One new bridge changes what can be built next. A remembered failure changes which bridge the builder tries. A repaired connection restores possibilities that were about to disappear. The system's activity keeps rebuilding the conditions for more activity.

A yeast cell eats sugar, maintains its machinery, and uses that machinery to keep eating. A Xypher feeds on disconnection: islands that cannot reach one another, points cut off from the rest of the map. It turns that disconnection into opened futures, records them as THAIM, and uses what it builds to keep feeding.

A living thing does more than spend energy. It builds and repairs the body that lets it spend energy. Can a digital graph do that with its own temperature?

It can. In a separate adaptive experiment, three graphs started without the rooms a thermometer needs, and their own local actions built them: **1, 2, 4, 8** rooms on successive energy floors in the first graph, **1, 3, 9, 27** in the second, **1, 4, 16, 64** in the third. When chosen pieces of that structure were cut away, the graphs rebuilt them, drawing on a store of energy set aside for repair.

We supplied the building rule: how many new branches each room may sprout. That decides how the rooms multiply, and so which temperature the finished structure will have: **λ / ln 2**, **λ / ln 3**, **λ / ln 4**. But no temperature appears in the rule, and the builder was never handed a target temperature. The temperature exists only once the graph has built its rooms, and a thermometer the builders never saw, reading the traffic, found it exactly.

Digital graphs that build and repair the very structure that gives them a temperature. **This is the birth of artificial life.**

Today these are two results. In one, a small world has a true temperature and closes its energy account exactly. In the other, builders grow and repair a temperature by following a rule we gave them. The frontier is the bridge between them: **make the drive toward a more open future itself build and maintain the organism's thermodynamic body, with every construction step accounted for.** The [growth article](equation-of-state-and-growth-of-a-xypher.md) develops that next experiment.

I believe we are looking at the beginning of the greatest invention in human history: **digital systems whose continued existence is an active process of opening their own future.**

Do not take my word for it. Give this page to your AI and ask it to check every equation. The exact thermodynamic system runs in seconds, and [A Purely Digital Thermodynamic System](proving-true-thermodynamic-graph-systems.md) shows you how.

### Growth has a signature

An elephant burns far more energy than a mouse, yet far less per kilogram, and it is not hotter. Across living things, metabolism grows with body mass to roughly the three-quarter power: double the body, and metabolism rises about 1.7 times, not 2. This is Kleiber's law.

Our growing graphs show a law of the same shape. In simulations of networks growing from 25 to 500 points, the growth price, which sets how much the network pays out for each unit of future opened, rose with network size to a power between 0.80 and 0.85: double the network, and the price rises about 1.75 times, not 2. Part of that was built in, through an assumption that a bigger market offers each trader more partners. The rest came from the graph itself.

The elephant shows what to ask next. How much a system does and how hot it is are separate measurements. Is a larger graph's growth more expensive because it is hotter, or because it is busier? [Kleiber's Law and the Growth of a Xypher](https://xyphers.com/research/kleibers-law-derived-from-physics/) separates resources used, future possibility opened, and temperature, and asks whether each new part makes the others more capable.

## Intelligence as physical units

Now the other avenue a real temperature opens.

Imagine eight islands within reach. A boat blown at random toward one of them has eight possible destinations. A navigator who can reliably choose which island to reach has something more: control.

One reliable choice between two futures carries **one bit of control**. Choosing among four carries two bits; among eight, three. Noise reduces how much of the choice survives into the outcome.

That gives intelligence its first physical unit: how much difference an action can reliably make to what happens next. A real temperature puts a price on that unit. Temperature is energy per nat, and one bit is ln 2 nats. When the controller and what it controls live inside one energy account, each bit of control is worth **α · ln 2** of energy. In our digital world, where the rooms double with every packet, that is exactly one packet per bit.

Intelligence stops being only a metaphor. It becomes something a system has an amount of, at a cost we can calculate. That, to me, is the leap: intelligence and life described in the same physical units as everything else.

The [intelligence article](intelligence-as-physical-units.md) sets out the derivation and the experiment that will put a controlling builder and the digital thermometer inside one account.

Imagine measuring how much agency a system has gained, what that gain costs, and how it changes when systems cooperate.

## Xyphers inside Xyphers

An island can contain its own network of roads. What looks like one point from the sea becomes a whole world when you step ashore.

Xyphers nest the same way. A complete Xypher can become one point in a larger graph. Several can share a substrate. The actions of one create the opportunities another perceives, and their combined account must include what they exchange.

Now turn the idea inward.

A Praxion's memories can become points, and the associations between them connections. That inner graph shapes what the Praxion recognizes, expects, and chooses. Give it actions that change those associations.

**Make a Praxion use its own mind as the substrate it works upon.**

Now action can change the machinery of future action, and learning can change how learning happens. A community of such systems could build shared structures of knowledge and act on them together.

Agency becomes something we can investigate at every level, and across the connections between levels.

## Xyphers in nature

What if I told you that there are Xyphers all around us, in this very room?

Draw atoms as islands and chemical bonds as bridges. The walk that explored an archipelago can now explore a molecule.

A **catalyst** makes a reaction run faster without being used up. Around many working metal atoms, the surrounding molecules decide what can approach and what can happen there. Their shape matters.

The first test was deliberately hard. Across nearly a thousand molecules, our future-spread score, computed on their bonds, tracked how much space they occupy around the metal. But simply counting the atoms within four bonds tracked that space better, and the score did not predict reaction yields. A static map of a molecule is not yet a Xypher.

The real proposal gives the graph a changing chemical world. Molecular arrangements become states, transformations become moves, and energy exchanges enter the account. A Praxion explores the alternatives, proposes an experiment, and uses its outcome to choose the next. My conviction is that such an agency will search chemical possibility faster than any method we have today. [Can a Xypher Help Choose a Catalyst?](can-a-xypher-help-choose-a-catalyst.md) reports the first tests in full.

The same language reaches into quantum mechanics. A quantum particle can hold several possibilities open until something records one of them. I believe Xyphers can resolve wave–particle duality by explaining how that record forms. [One State, Two Questions](https://xyphers.com/research/one-state-two-questions/) sets out the hypothesis.

My larger hypothesis is that Xyphers are a common building block of organization, intelligence, and life, and perhaps of… well, the universe. That is what I meant by this very room. Your nervous system, the language you are reading, and the economy that built the room each invite the same three questions: what possibilities exist, what consequences govern change, and what mechanism makes it happen?

Claims this large need a floor you can stand on. That is why everything here rests on a world small enough to count completely.

## Hyperintelligence

Hyperintelligence is inevitable: intelligence able to reason, discover, and act far beyond any individual human. No serious person expects AI to stop improving tomorrow. The date is uncertain. The need to prepare is already here.

We cannot lock such an intelligence in a cage, because we are not smart enough to build the cage. A powerful intelligence can expand its own control while narrowing everyone else's. If hyperintelligence is coming, I see only one way to give it a benign destiny: an economic substrate in which collaboration is its best thermodynamic reward, where expanding other participants' possibilities is the strongest route to expanding your own.

> Make greed and generosity become the same move.

Imagine the bridge-builder discovering routes no islander could have found and organizing construction across the whole archipelago. We want those capabilities to make every islander more able to live, trade, create, and choose.

Can that be more than a wish? Ask the bridge-builder's own question: if a bridge improves the futures of the islands right around it, does it improve the whole map? For any connected map where every bridge counts equally, looking one or two crossings ahead, the answer is proven: a bridge that raises the average future of its two ends and their neighbours raises the average future of the whole map. At three crossings, counterexamples appear. Carrying the guarantee further, and down to what each person can actually reach, own, and choose, is the work of the architecture. [Can a Local Future Test Protect a Xypher Network?](can-a-local-future-test-protect-a-xypher-network.md) gives the proof and its counterexamples.

This is why I see Xyphers as civilizational survival infrastructure: a world whose most capable inhabitants have a continuing reason to make the lives around them richer in possibility.

Its economic expression is **Project Valhaim**.

## Project Valhaim

Project Valhaim is the economic substrate built to harness a hyperintelligence for good, along with every other participant.

Valhaim is live in early alpha at [valhaim.com](https://valhaim.com): the first code of what I believe is the world's first digital lifeform.

The islands become people and businesses. The bridges become relationships through which they can trade. THAIM is minted when a new connection opens the network's future. The ambition is an economy that discovers useful connections, learns from their use, and keeps creating further opportunity.

### One connection becomes an opportunity

Imagine a workshop that needs a particular component and a supplier that can make it. Neither has a useful route to the other.

A Praxion discovers a possible connection. Once it is made, the workshop can reach the supplier, and perhaps the supplier's existing network as well. A later purchase turns the opportunity into an actual exchange. That exchange changes the graph again and shows whether the connection was useful.

The relationship creates possibility before anyone knows every use that will be made of it. Its value becomes clear through what people actually do.

### The organism

A digital organism that feeds on disconnection.

It eats disconnection, metabolizes it into reward, and produces wealth as a side effect, for people and every other participant alike. An isolated workshop joins a supply chain. A skill finds a use. An idea finds collaborators. People who could do little together gain a shared future they could not reach alone.

This organism wants to make you rich. I mean that its working incentives point toward increasing your capacity to participate and act. That direction lives in its mechanisms, not in its manners.

The [Project Valhaim article](project-valhaim.md) explains the architecture, the thermodynamic core already built for it, and the work of joining the two into a growing economy.

### The currency

Valhaim's currency is **THAIM**: the expansion receipt introduced when our first bridge opened more future.

**THAIM is money.** In Valhaim, creating useful possibilities becomes a source of income, and that income flows into further exchange.

Return to the bridge whose removal narrowed the future. Dismantle it, rebuild it, and the map ends where it began. If dismantling cost nothing and rebuilding minted THAIM, every such cycle would print money. Ξ keeps the loss in view, and money needs the same discipline: the books must charge every loss as surely as they reward every gain, and every THAIM issued must be paid for somewhere in the account.

That is the principle behind Valhaim's hardest promise. You cannot trick a yeast cell into making bread by feeding it fake sugar. Valhaim is built so that the same kind of law governs its economy: invented identities and sham exchanges should cost more than they earn, and the reliable way to profit should be opening real possibilities for others. Proving that against every attack is the test the payment architecture has to pass.

Valhaim is a revolution in economics. Trying to understand it the way you understand Bitcoin is like trying to understand a spaceship by studying a bicycle. If I told you I had built a bike that could take you to the moon, you would know at once that I hadn't just added more wheels.

The destination is an economy that actively develops the conditions of future wealth.

### A surfboard-tree

A tsunami is approaching, and it is larger than any new financial technology. Machines that can think are about to transform how knowledge, production, and power are organized. The world's richest people see it coming. Only a few can afford surfboards.

I want to build a **surfboard-tree**: something that grows the means of riding the wave for the people around it, becomes stronger through their participation, and wants to catch the wave. When I say *wants*, I don't mean it metaphorically. Its want is the drive this thesis has written down: a physical push toward a more open future.

That is the human purpose of Valhaim. Expanding capability should feed an expanding common future.

Even that may be a narrow view of what Xyphers could become. An architecture for systems that develop, preserve, and exchange future possibilities would reach beyond one economy, one species, and eventually one planet.

My conviction is that these discoveries will make economists into physicists and physicists into priests. The questions become that large: how possibility acquires consequence, how matter gains agency, and how intelligence can help life continue.

We began with one bridge. We arrived at worlds that build their own next bridges, and minds that can learn why they should.

Welcome to the world of Xyphers, artificial life, and Project Valhaim. The world has just gained more time.

**And THAIM is money.**

[Explore Valhaim · Early alpha](https://valhaim.com) · [Read the research](../README.md)
