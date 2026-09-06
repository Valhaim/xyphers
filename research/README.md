# Xyphers research

This directory collects the public Xyphers research record: readable articles, explicit evidence boundaries, and—where available—the code or data needed to inspect a result.

The labels describe the maturity of the scientific claim, not the polish of the prose:

- **Published · bounded result** — the article is public and its strongest claims are limited to named evidence.
- **Negative validation** — the tested claim did not pass its controlling comparison; the failure and surviving scope are both public.
- **Formal-framework draft** — the standard mathematical core is exact, while the Xypher-specific synthesis remains a working framework.
- **Open frontier** — the article is public, but it contains a question or proposed bridge that remains unresolved.
- **Early speculation** — a conjecture released before the argument is complete.

## Foundations

- [What is a Xypher?](articles/core-thesis.md) — the Core Thesis and the map of the research program.

## Research articles

| Article | Scientific boundary | Website route |
| --- | --- | --- |
| [A Purely Digital Thermodynamic System](articles/proving-true-thermodynamic-graph-systems.md) | Exact sixteen-state minimal thermodynamic construction; separate scoped representation theorem. | `/research/proving-true-thermodynamic-graph-systems/` · deployment pending |
| [Project Valhaim](articles/project-valhaim.md) | Economic architecture, bounded kernel evidence, and an early-alpha application. | `/research/project-valhaim/` · deployment pending |
| [Can a Xypher Help Choose a Catalyst?](articles/can-a-xypher-help-choose-a-catalyst.md) | Negative validation: the molecular readout lost to a simpler ligand-graph count and showed no distinctive yield-ordering advantage. | `/research/can-a-xypher-help-choose-a-catalyst/` · deployment pending |
| [When Does Information Become State in a Xypher?](articles/when-does-information-become-state-in-a-xypher.md) | Formal-framework draft: complete-state and exact-projection tests are established; shared adaptive-Xypher state remains open. | `/research/when-does-information-become-state-in-a-xypher/` · deployment pending |
| [Can a Local Future Test Protect a Xypher Network?](articles/can-a-local-future-test-protect-a-xypher-network.md) | Published bounded graph result: exact at two steps under the declared rule; the universal threshold and alignment readings fail. | `/research/can-a-local-future-test-protect-a-xypher-network/` · deployment pending |
| [An Equation of State and the Growth of a Xypher](articles/equation-of-state-and-growth-of-a-xypher.md) | Open frontier: separates executable-state temperature from growth price and receipt throughput. | `/research/equation-of-state-and-growth-of-a-xypher/` · deployment pending |
| [Intelligence as Physical Units](articles/intelligence-as-physical-units.md) | Open frontier: derives the control-to-information ladder and states what remains to connect it physically. | `/research/intelligence-as-physical-units/` · deployment pending |

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
- `website_path` records the intended route.
- `web_status` says whether that route has been deployed.

Supporting source, data, and runnable code should sit beside the article it supports or in a clearly linked subdirectory. A reader should be able to move from claim to evidence without guessing which artifact controls the result.
