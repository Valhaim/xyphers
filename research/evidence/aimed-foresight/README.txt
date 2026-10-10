XYPHERS AIMED FORESIGHT EVIDENCE MAP

Article
  Does Seeing Further Ahead Buy More Open Future?

Read
  BOUNDARY.md is the preregistration of CAL-CEF-4: a driven archipelago with
  no free protection (the weather builds and wears every bridge, and a
  missing bridge blocks the traveller's step), routes held as intentions,
  agents that fuel every bridge (warming), every bridge at the traveller's
  island (LOCAL), or the route's bridges within k moves (AIM-k), the edge E
  at equal power against random placement of the same mean number of
  bridges, the values fixed in advance, eleven acceptance gates, seven
  deliberately broken controls, five hypotheses that could fail, the
  blinding, and fifteen verified references. It was frozen before any
  evaluator ran.
  RESULT.md records the single frozen execution: verdict, hypothesis
  outcomes, review chronology, file digests, toolchain, the complete report,
  every steady state, and the independent check.
  expected-report.txt is the report that run printed; compare your own run
  with it.

Inspect and run
  source/xypher-aimed-foresight-proof/ is the dependency-free Rust verifier,
  with the exact arithmetic of CAL-CEF-3 reused unchanged, symmetry-orbit
  solving checked state by state on the full chain, and the prospective
  Futuruna declaration (xypher.runa). RUN.txt gives the commands and the
  expected result.
  independent_check.py is a separate implementation written from the
  boundary alone. It rebuilds every world, solves it exactly, compares H and
  J with expected-report.txt, and re-decides the five hypotheses.
  expected-independent-check.txt is its output.

Provenance
  BOUNDARY.md       thaim commit cec8eb84034144a6adae018fd4df8cefbce64b20,
                    research/physics/derivable/
                    xypher-aimed-foresight-boundary.md
  RESULT.md         thaim commit bca2736c6efe79b11ef01ee432cda9dc1dd7f8ad,
                    research/physics/derivable/
                    xypher-aimed-foresight-result.md
  source/           thaim commit 0bb4b46c6fed69ff109ba6523177851f8ac510c2,
                    research/physics/xypher-aimed-foresight-proof/
                    (Git tree 917c4afd1ff7b0505ce01ffb53aa5020efc0f739)
  independent_check.py and expected-independent-check.txt
                    thaim commit bca2736c6efe79b11ef01ee432cda9dc1dd7f8ad,
                    research/physics/xypher-aimed-foresight-check/
  These files are exact copies of the originals at those commits.
  README.txt, RUN.txt, and SHA256SUMS were written for this packet;
  expected-report.txt is copied from the report in RESULT.md section 3.1.

Scope
  An exact finite construction, not a fitted simulation. No random seed,
  sampling, simulation length, tolerance, or floating-point acceptance path
  is involved. Logarithms enter only through the price floor and the
  efficiency, as certified rational enclosures.

  Results within the declared worlds, with the open future counted as
  two-step walks from where the traveller stands: fuel buys open future;
  at equal power, fuel at the traveller's island holds 1.35 to 2.21 times
  the open future of random placement, in all eighteen worlds; that edge
  rises by nearly equal steps from three to five islands; aiming fuel
  further along the route holds more open future in total but has a
  smaller edge at every step, in all nine ladders; the whole route never
  beats fuel at the traveller's island; the agent that fuels only its next
  crossing has the largest edge in every world.

  Next: count the open future as far ahead as the plan looks, and slow the
  weather relative to the traveller.

Hashes
  SHA256SUMS covers this directory except the checksum file itself.
