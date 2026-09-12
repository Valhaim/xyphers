---
title: "Can a Xypher Help Choose a Catalyst?"
description: "A chemist can test only a handful of the molecules that tune a catalyst. Can a Xypher use their structure to identify which ones deserve the next experiment?"
type: "research-article"
status: "published"
maturity: "negative-validation"
published: "2026-08-18"
revised: "2026-08-18"
website_path: "/research/can-a-xypher-help-choose-a-catalyst/"
web_status: "live"
---

# Can a Xypher Help Choose a Catalyst?

A catalyst screen is a rationing problem. Hundreds of plausible molecules may be available, while time and material allow only a few experiments.

The molecular readout tested here did not earn the right to choose those experiments. In a blinded test it tracked one measure of ligand bulk, but a count of nearby atoms tracked that measure better. In reaction-yield data, it failed every predefined decision gate.

That answer is narrower than saying a Xypher cannot help with catalysis. The test did not contain a complete Xypher. It tested one graph readout proposed as part of one.

The distinction matters at the bench. A failed part should not be promoted into an instrument. It should tell us which part of the problem we have actually represented.

## The decision at the bench

A **catalyst** helps a chemical reaction proceed without being consumed by the overall reaction. Many industrial catalysts place a metal atom near the centre of the action. Other molecules bind around that metal and change the space and electronic environment in which it works. These surrounding molecules are called **ligands**.

Change the ligand and the same metal can become faster, slower, more selective, or effectively useless. The useful choice also depends on the substrate being transformed, the other reagents, solvent, temperature, and the route the reaction takes.

A chemist therefore faces a practical question:

> Which ligand deserves the next experiment?

The [Core Thesis](core-thesis.md#xyphers-in-nature) proposes that a Xypher could search chemical possibility space. To make that proposal testable, we reduced it to a cheaper question. Can the structure of a ligand produce a score that orders useful chemical properties, and eventually reaction outcomes, better than simple alternatives?

No experiment in this sequence selected a prospective top-ten list, discovered a new molecule, or compared a complete system with density-functional theory. The tested decisions were narrower: association with a three-dimensional steric target, then association with observed yield while the non-ligand recipe was held fixed.

## Looking out from phosphorus

Many ligands in these tests are phosphines. They contain a phosphorus atom that binds to the metal. Draw one as a graph and the atoms become points while the bonds become connections.

Now release a walker at phosphorus. At every step it chooses uniformly among the bonds leaving its current atom. After five steps, some destinations will be reached often and others rarely. A ligand whose five-step destinations are widely spread gives the walker a more even set of endings. A ligand whose walks repeatedly funnel into the same region gives it a more concentrated set.

The readout measures that spread.

$$
S_5(P) = -\sum_j p_j \log_2 p_j
$$

*The score is high when five-step destinations from phosphorus are numerous and evenly likely.*

**p<sub>j</sub>** is the probability that the walker ends at atom **j** after five moves. The logarithm uses base two, so the score is measured in bits. The calculation is Shannon entropy applied to the endpoint distribution.

Read it as: *How widely have the possible destinations spread after five bond choices from phosphorus?*

This is endpoint entropy. It does not count every complete five-step route. It also knows only what the molecular graph supplies. The primary score in both controlling tests treated every bond as one unit-weight connection; the blinded shape test also compared a separate bond-energy-weighted version. Neither calculation saw a molecule's three-dimensional conformation, orbital geometry, solvent, reaction temperature, or the actual course of a catalytic cycle.

The score was evaluated on a standardized **Rh(CO)(H)(L)** complex: one common rhodium scaffold holding the ligand **L**. That placed different ligands into the same graph context. It did not place them into the palladium, nickel, copper, or metal-free contexts of the later reactions.

## A first measure of shape

Chemists have long described phosphine size using the **Tolman cone angle**: picture the metal at the tip of a cone and the ligand filling the space inside it. A wider cone means the ligand occupies more room around the metal. [Tolman's original review](https://doi.org/10.1021/cr60307a002) made this steric language practical across organometallic chemistry.

An early development set contained 50 ligands. Ten additional ligands were kept out of that development work. Before those ten were calculated, the stated predictions concerned the implementation's **S<sub>5</sub>/atom**, where the 5 again means five bond choices: first take the mean five-step, bond-energy-weighted endpoint entropy across the connected atoms in **Rh(CO)(H)(L)**, then divide that mean once more by the total atom count.

The two phosphorus halides were expected to fall between the earlier PCl₃ and PPh₃ values. Five para-substituted ligands with the same cone angle were expected to cluster with a standard deviation below **0.005**. Across all ten ligands, the rank association was expected to remain near **ρ = −0.80** if the score generalized; even **−0.60** had already been marked as possible overfit.

The symbol **ρ**, pronounced rho, measures whether two rankings rise together. A value of **+1** means they have the same order, **0** means no monotonic ordering, and **−1** means one ranking runs exactly opposite to the other.

All three expectations failed. Both halide values landed outside the stated interval, the para-substituted group had a standard deviation of **0.0116**, and the rank association with cone angle moved from **ρ = −0.752** in development to **ρ = −0.442** on the ten excluded ligands.

A different result looked better. The bond-energy-weighted score rooted specifically at phosphorus reached **ρ = +0.733** on the ten cases, close to **+0.730** in development. But that site-rooted score had no prespecified decision gate. The first run also exposed missing phosphorus–bromine and phosphorus–iodine bond energies, and the reported values came after those entries were repaired. There was no pre-reveal manifest separating the test from that repair.

The ten ligands were outside development, but the favorable number was exploratory. It was also a different score from the unweighted readout later tested against buried volume and yield.

That was enough reason to ask a harder question. It was not enough to call the descriptor validated.

## A simpler ruler

Cone angle is one view of ligand bulk. The Kraken project supplies a much larger and richer steric map: quantum-chemical calculations for 1,558 ligands, accompanied by machine-learned estimates across a broader virtual library. Its purpose is to make ligand property space searchable rather than leave chemists with a shelf of names. The dataset and its construction are described by [Gensch and colleagues](https://doi.org/10.1021/jacs.1c09718).

For the Xyphers test, the values of the target property were hidden while scores were generated. After all exclusions, **934 ligand graphs** remained.

A flexible ligand can settle into several three-dimensional shapes. The Kraken calculation estimates how much space each shape occupies near the metal, gives the lower-energy shapes more weight because they are more thermally plausible, and averages them. Chemists call the result **Boltzmann-average absolute buried volume**. It is measured in cubic ångströms, a unit of molecular volume.

The target was hidden. The ligand structures were not; the graph had to be built from them.

Before looking at the comparison, the protocol set several gates. The graph score needed **ρ ≥ 0.50** with a permutation **p ≤ 0.001**. It then had to beat the strongest simple baseline by at least **0.10**, with the lower end of its bootstrap interval above zero. Inside each of two ligand families it needed **ρ ≥ 0.70**, **p ≤ 0.025**, and the same margin over the strongest baseline. Bond-energy weighting had its own comparison.

A permutation test rearranges the target labels in the ways allowed by the protocol and asks how often a rearrangement matches or exceeds the observed result; **p** is that fraction. A paired bootstrap repeatedly resamples the same ligands and recalculates the difference between two scores. Its interval shows the range compatible with those resampled comparisons.

The central results are below. The reader should compare the graph score with the deliberately plain ruler in the next row.

| Readout | Rank association with buried volume |
|---|---:|
| Unweighted five-step endpoint entropy, **S<sub>5</sub>(P)** | **ρ = 0.5252** |
| Atoms one to four bonds from phosphorus in the parsed ligand graph, including expanded hydrogens | **ρ = 0.7255** |
| Bond-energy-weighted **S<sub>5</sub>(P)** | **ρ = 0.5195** |

The entropy score did associate with the target. Repeated resampling placed the compatible range for **ρ** between **0.4705 and 0.5754**, and the frozen permutation test gave **p = 0.000050**, so the association subtest passed. But the simpler radius-four count—atoms one to four bonds from phosphorus in the parsed ligand alone, including hydrogens added by the parser—ordered buried volume much more closely. Their prespecified difference was **−0.2003**; its **95% paired-bootstrap interval was [−0.2394, −0.1631]**.

The same relationship also had to appear inside two structural groups defined at phosphorus. In 141 ligands, all three carbons bound directly to phosphorus were nonaromatic and had only single incident bonds. In 329 ligands, all three directly bound carbons were aromatic. The score reached **ρ = 0.6327** in the first group and **ρ = 0.3483** in the second. Both fell below the required **0.70**, and neither beat its strongest simple baseline. Bond-energy weighting was slightly worse than leaving every connection unweighted: **0.5195 − 0.5252 = −0.0057**.

The full descriptor claim therefore failed.

This is a useful kind of failure. Buried volume is a local measure of spatial extent. Counting atoms within four bonds is almost embarrassingly direct, but direct is exactly what a good baseline should be. A more elaborate score earns its complexity only if it adds information the simple ruler misses.

Here it did not.

## When the comparison has no answer

Steric bulk is not catalytic performance. The next test moved to reaction yield: how much desired product was obtained under a particular recipe.

[Das and colleagues](https://doi.org/10.1021/jacs.6c05959) ran **50,688 carbon–nitrogen coupling reactions** on two substrate pairs while varying metal precatalysts, ligands, bases, solvents, temperature, and other conditions across palladium, nickel, copper, and metal-free regimes. The dense overlap makes the Open Reaction Database record unusually valuable for comparisons inside a fixed recipe.

Reaction outcomes remained sealed while the ligand structures and analysis plan were prepared. After the outcomes were decoded mechanically, **49,632 ligand-loaded yields** were available.

The plan separated examples used to establish the comparison from examples used to judge it. Every ligand position eligible for judgment therefore needed corresponding coverage in the training side. One coded position on the experimental plate appeared once in an anomalous loaded row but in none of the primary reaction panels that could enter the result. The frozen check nevertheless treated it as eligible and demanded training coverage that could not exist.

The program stopped before calculating any hypothesis statistic. This stopped run is recorded as **CHEM-V9**. It provides neither positive nor negative evidence about the molecular score.

The repair restricted the coverage check to positions that could actually enter the result. By then V9 had extracted and processed the outcome table, and two reaction-ID-sorted rows—both with zero yield—had been displayed. The mechanical repair was frozen and pushed before any hypothesis statistic was calculated. It came from target-free membership and plate-layout records; no yield value selected a model, threshold, or descriptor. The repaired analysis, **CHEM-V10**, then ran once.

That history makes CHEM-V10 useful, but not pristine confirmation. Its proper description is a repaired one-shot negative validation.

## Yield changes the question

The reactions were arranged into comparison groups called **blocks**. Within one block, substrate, base, solvent, temperature, concentration, and other non-ligand recipe fields stayed fixed. The analysis asked whether the yield ordering was associated with the graph-score ordering. It used the absolute association, so the direction was allowed to reverse between recipe contexts. Ligand identity and its coded position on the experimental plate moved together; the source record does not document random assignment of ligands to those positions.

The final analysis contained **17,632 eligible yield observations**, **45 ligand structures**, **14 experiments with enough variation to evaluate**, and **864 fixed-recipe blocks**.

Each block produced a rank association between graph score and yield. The protocol took its absolute size, then averaged the block values so that one large experiment could not dominate the rest. Call that final average **T**. The molecular score reached **T = 0.156148**. The strongest simple baseline—the ligand's coded plate position—reached **T = 0.196401**.

To ask whether the molecular value was larger than chance, the test rearranged ligand labels only in ways that preserved the frozen comparison structure. Those legal rearrangements were tightly constrained: only **19 of the 45 structures** could move. The fraction of rearrangements producing a result at least as large as the observed one is the exact permutation p-value in the table below.

The protocol also asked whether the molecular score beat the strongest baseline by a meaningful margin. A **95% cluster-bootstrap interval**, resampled across the 14 experiments rather than treating 17,632 rows as independent experiments, had to remain above zero rather than merely touching it.

Five gates had been fixed in advance. The table shows the observed value beside the value needed to pass.

| Decision gate | Observed | Required |
|---|---:|---:|
| Extra association beyond the rearranged-label average | **0.005544** | at least **0.05** |
| Exact permutation p-value | **0.039063** | at most **0.001** |
| Advantage over the strongest baseline | **−0.040253** | at least **0.05** |
| Exact permutation p-value for that competition | **0.039063** | at most **0.001** |
| 95% 14-experiment cluster-bootstrap interval for baseline advantage | **[−0.063511, −0.016179]** | lower bound above **0** |

Every gate failed. The score did not separate itself from the exact null by the required amount, and it ranked below the coded-position baseline rather than above it.

The signs tell the same story more gently but less decisively. Across 864 blocks, 287 associations were positive, 284 were negative, and 293 were zero. Of the zeros, 288 came from blocks where every recorded yield was tied. Among nonzero blocks, the positive fraction was **0.502627**. This descriptive split does not replace the formal gates, but it gives no directional rescue.

Because ligand and plate position co-varied, the result cannot identify a causal ligand effect or a causal plate effect. It says something more limited and sufficient for the question at hand: this score showed no distinctive yield-ordering advantage in the declared observational comparison.

## The reaction the graph never saw

The two failures do not identify a cause. They are consistent with a representational limit worth making explicit.

The score saw an unweighted molecular graph on a standardized rhodium complex. The buried-volume target saw a three-dimensional cloud of conformations. The reaction screen saw actual palladium, nickel, copper, or metal-free precursors; different substrates and bases; solvent and temperature; and reactions capable of following different mechanisms.

Those are not small corrections around one complete state. Most of the reaction was absent.

A separate, post hoc comparison suggests one possible applicability warning. In eight bidentate hydroformylation ligands, the average phosphorus-rooted score associated with the logarithm of linear-to-branched selectivity at **ρ = 0.452**, while the chemically direct bite angle gave **ρ = 1.000**. PBn₃, P(neoPent)₃, and BISBI also placed bulky groups behind a methylene bridge. A five-step walker may spend part of its reach crossing that bridge and underrepresent the remote group. That is a rational reconstruction from a small separate set, not an identified cause of the V8 or V10 failures. Those three examples were outside V10's eligible cohort.

Bond-energy weighting did not solve the blinded steric test. That makes sense once the target is clear: changing the strength assigned to a connection need not improve a measure of three-dimensional extent. Electronics, orbital orientation, metal identity, and competing mechanisms demand representations that actually contain them.

This explanation does not authorize a new score fitted after the failure. It marks the boundary of the score that was frozen and tested.

## A descriptor is not a catalyst Xypher

The title asks whether a Xypher can help choose a catalyst. The experiments reached only one proposed readout.

The ligand graph supplied part of a **Graph Substrate** and one endpoint-entropy measurement. It had no independently grounded **Thermodynamic Harness** and no **Praxion** choosing experiments inside a reaction state. No action loop observed an outcome, updated state, and selected the next ligand.

A reaction-level Xypher would first have to say what its complete state contains. The actual metal, ligand geometry, substrate, reagents, solvent, temperature, and allowed reaction channels cannot remain outside the model if its action depends on them. Its Praxion would need a declared choice—perhaps which experiment to run—and a result that can change later choices. Any thermodynamic claim would additionally need an independently grounded energy and temperature account, not a chemical label attached to a graph score.

None of that architecture can be inferred from **ρ = 0.5252**. It has to be built and tested on its own terms.

## Should the score choose?

If the bench question is whether unweighted **S<sub>5</sub>(P)** should choose the next ligand, the present evidence says no.

It tracked a blinded measure of ligand bulk, but the direct atom count tracked that target better. It failed the required family comparisons. Bond-energy weighting did not help. In the repaired reaction-yield analysis, it failed all five gates and ranked below a coded-position baseline.

Molecular graphs may still be useful, and other Xypher designs remain open. The measured conclusion is that this topology readout has not demonstrated an advantage for ligand selection.

A complete catalyst Xypher remains an untested proposal.

## Evidence

The external chemical records are the [Tolman steric review](https://doi.org/10.1021/cr60307a002), the [Kraken ligand-space paper](https://doi.org/10.1021/jacs.1c09718), and the [systematic coupling-screen paper](https://doi.org/10.1021/jacs.6c05959). The underlying ORD dataset is available as an [immutable repository object](https://github.com/open-reaction-database/ord-data/blob/ddb0d25770c80a0a6fcf9948c26e1c8f828cb8ad/data/80/ord_dataset-805ad863feef48579d95d86a728035f4.parquet).

**Read:** [the blinded shape result](../evidence/catalyst-choice/CHEM-V8-Result.md) and [the repaired yield result](../evidence/catalyst-choice/CHEM-V10-Result.md).

**Inspect:** [the evidence map and provenance record](../evidence/catalyst-choice/README.txt), including the early Tolman implementation, the separate bidentate comparison, the frozen descriptor source, and each manifest.

**Download:** [the checksummed Xyphers validation packet](../downloads/xypher-catalyst-validation-v1.zip) ([SHA-256](../downloads/xypher-catalyst-validation-v1.zip.sha256)). The archive does not redistribute the Kraken supplement, its derived 934-row SMILES table, or the raw ORD parquet. Its retained ORD-derived artifacts carry attribution and a CC BY-SA 4.0 notice.

**Inspect and reproduce:** [follow the evidence paths](../evidence/catalyst-choice/RUN.txt). CHEM-V8 requires rebuilding licensed source tables and matching the frozen hashes; CHEM-V10 has a numerical replay from the attributed ORD-derived freeze.
