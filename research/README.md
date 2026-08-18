# Xyphers research

This directory collects the public Xyphers research record: readable articles, explicit evidence boundaries, and—where available—the code or data needed to inspect a result.

The labels describe the maturity of the scientific claim, not the polish of the prose:

- **Published · bounded result** — the article is public and its strongest claims are limited to named evidence.
- **Open frontier** — the article is public, but it contains a question or proposed bridge that remains unresolved.
- **Early speculation** — a conjecture released before the argument is complete.

## Foundations

- [What is a Xypher?](../README.md) — the Core Thesis and the map of the research program.

## Published articles

| Article | Scientific boundary | Website route |
| --- | --- | --- |
| [An Equation of State and the Growth of a Xypher](articles/equation-of-state-and-growth-of-a-xypher.md) | Open frontier: separates executable-state temperature from growth price and receipt throughput. | `/research/equation-of-state-and-growth-of-a-xypher/` · deployment pending |
| [Intelligence as Physical Units](articles/intelligence-as-physical-units.md) | Open frontier: derives the control-to-information ladder and states what remains to connect it physically. | `/research/intelligence-as-physical-units/` · deployment pending |

## Runnable code

- [Minimal Thermodynamic Xypher Proof](code/xypher-thermodynamics-proof/) — the standalone, dependency-free Rust verifier for the exact 16-state result used by both articles. It includes the prospective Futuruna source, the independent Rust enumeration, and the source-contract test.

## Repository convention

Each article is ordinary Markdown with portable metadata:

- `status` records whether the article is published.
- `maturity` records the scientific boundary.
- `website_path` records the intended route.
- `web_status` says whether that route has been deployed.

Supporting source, data, and runnable code should sit beside the article it supports or in a clearly linked subdirectory. A reader should be able to move from claim to evidence without guessing which artifact controls the result.
