# Xyphers research

This directory connects the articles at [xyphers.com](https://xyphers.com/research/) to their code and evidence. Start with the [thermodynamic graph you can run](code/xypher-thermodynamics-proof/). The later articles develop the larger programme: a physics for digital agency.

The labels describe the maturity of the scientific claim, not the polish of the prose:

- **Published · bounded result** — the article is public and its strongest claims are limited to named evidence.
- **Negative validation** — the tested claim did not pass its controlling comparison; the failure and surviving scope are both public.
- **Formal-framework draft** — the standard mathematical core is exact, while the Xypher-specific synthesis remains a working framework.
- **Open frontier** — the article is public, but it contains a question or proposed bridge that remains unresolved.
- **Early speculation** — a conjecture released before the argument is complete.

## Foundations

- [What is a Xypher?](https://xyphers.com/core-thesis/) — the Core Thesis and the map of the research program. A [portable edition](articles/core-thesis.md) is included here.

## Research articles

| Read the article | Inspect or run its evidence |
| --- | --- |
| [A Purely Digital Thermodynamic System](https://xyphers.com/research/proving-true-thermodynamic-graph-systems/) · [portable text](articles/proving-true-thermodynamic-graph-systems.md) | [Run the exact sixteen-state study](code/xypher-thermodynamics-proof/). Its source, expected report, boundary, and separate representation argument are included. |
| [Can a Xypher Help Choose a Catalyst?](https://xyphers.com/research/can-a-xypher-help-choose-a-catalyst/) · [portable text](articles/can-a-xypher-help-choose-a-catalyst.md) | [Run the molecular graph example](code/molecular-properties/). The [blinded comparisons](evidence/catalyst-choice/README.txt) did not establish a ligand-selection advantage; replaying them requires the named external datasets. |
| [When Does Information Become State in a Xypher?](https://xyphers.com/research/when-does-information-become-state-in-a-xypher/) · [portable text](articles/when-does-information-become-state-in-a-xypher.md) | [Read the complete-state and projection records](evidence/information-state/README.txt), or use the [source packet](downloads/xypher-information-state-evidence-v1.zip). The adaptive-Xypher synthesis remains a working framework. |
| [Can a Local Future Test Protect a Xypher Network?](https://xyphers.com/research/can-a-local-future-test-protect-a-xypher-network/) · [portable text](articles/can-a-local-future-test-protect-a-xypher-network.md) | [Inspect the proof and reproduction sources](evidence/local-future-test/README.txt). The two-step guarantee and later counterexamples have separate records; large enumerations are outside the default checks. |
| [Project Valhaim](https://xyphers.com/research/project-valhaim/) · [portable text](articles/project-valhaim.md) | [Inspect the archived kernel and contact experiment](physics/README.md). These are sealed execution records, not a standalone payment node. The node lives in [Valhaim/valhaim-node](https://github.com/Valhaim/valhaim-node). |
| [An Equation of State and the Growth of a Xypher](https://xyphers.com/research/equation-of-state-and-growth-of-a-xypher/) · [portable text](articles/equation-of-state-and-growth-of-a-xypher.md) | [Run the finite thermodynamic foundation](code/xypher-thermodynamics-proof/). The proposed connection between operational temperature and graph growth remains an open experiment. |
| [Intelligence as Physical Units](https://xyphers.com/research/intelligence-as-physical-units/) · [portable text](articles/intelligence-as-physical-units.md) | [Run the exact digital thermometer](code/xypher-thermodynamics-proof/). This does not execute the article's proposed action-to-future control experiment. |

## Early speculation on the website

- [Kleiber's Law and the Growth of a Xypher](https://xyphers.com/research/kleibers-law-derived-from-physics/) explores the relationship between measured graph-growth scaling and biological metabolism.
- [One State, Two Questions](https://xyphers.com/research/one-state-two-questions/) develops a Xypher hypothesis for wave–particle complementarity.

These essays are linked for the complete research picture. Their own reproduction code is not packaged in this repository.

## Runnable code

- [Minimal Thermodynamic Xypher Proof](code/xypher-thermodynamics-proof/) — the standalone, dependency-free Rust verifier for the exact 16-state result used by the equation-of-state, intelligence, and information-state articles. It includes the prospective Futuruna source, the independent Rust enumeration, and the source-contract test.

- [Molecular graph properties](code/molecular-properties/) — runnable phosphine example, frozen molecular implementation, and blinded shape evaluator.

## Evidence packets

- [Catalyst choice](evidence/catalyst-choice/) — frozen CHEM-V8/V9/V10 records, source inspection, licensing boundaries, and reproduction paths.
- [Information to state](evidence/information-state/) — CAL-XTHERM and CAL-ADAPT boundaries, exact results, and run guide.
- [Local future test](evidence/local-future-test/) — proof records, standalone Rust verifiers, expected-result dictionary, and source-status warnings.
- [Checksummed archives](downloads/) — portable copies of the downloadable packets linked from the articles.

## Repository convention

Each article is ordinary Markdown with portable metadata:

- `status` records whether the article is published.
- `maturity` records the scientific boundary.
- `website_path` records the website route.
- `web_status` says whether that route has been deployed.

Supporting source, data, and runnable code should sit beside the article it supports or in a clearly linked subdirectory. A reader should be able to move from claim to evidence without guessing which artifact controls the result.
