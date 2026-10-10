XYPHERS OPEN FUTURE PRICE EVIDENCE MAP

Article
  How Much Open Future Can a Unit of Energy Buy?

Read
  BOUNDARY.md is the preregistration of CAL-CEF-3: the driven archipelago
  (weather from a cold store everywhere, fuel from a hot store only beside
  the traveller), the measured open future per unit of fuel flow, the
  Horowitz-Zhou-England price floor, the values fixed in advance, ten
  acceptance gates with their check order, six deliberately broken controls
  with their expected first failures, four hypotheses that could fail, and
  the pilot that came before. It was frozen before any evaluator ran.
  RESULT.md records the single frozen execution: verdict, hypothesis
  outcomes, review chronology, the arithmetic test before approval, file
  digests, toolchain, the complete report, and the independent check.
  expected-report.txt is the report that run printed; compare your own run
  with it.

Inspect and run
  source/xypher-open-future-price-proof/ is the dependency-free Rust
  verifier, with its own exact big-integer arithmetic, and the prospective
  Futuruna declaration (xypher.runa). RUN.txt gives the commands and the
  expected result.
  independent_check.py is a separate implementation written after the run
  from the boundary alone. It rebuilds every world, solves it exactly with
  the Python standard library, compares H and J with expected-report.txt,
  and re-decides the four hypotheses. expected-independent-check.txt is its
  output.

Provenance
  BOUNDARY.md       thaim commit e5474cb773c3da74ce4162d20e07f37d904a945e,
                    research/physics/derivable/
                    xypher-open-future-price-boundary.md
  RESULT.md         thaim commit 978d2374e7af0b45354d138652deaec16622838c,
                    research/physics/derivable/
                    xypher-open-future-price-result.md
  source/           thaim commit 33b96e33ebdced33befdb72dbeb42d6ff54f68bf,
                    research/physics/xypher-open-future-price-proof/
                    (Git tree 7ad5e7e3edab89549b1b6e6ecfd203e43ef9d61b)
  independent_check.py and expected-independent-check.txt
                    thaim commit 978d2374e7af0b45354d138652deaec16622838c,
                    research/physics/xypher-open-future-price-check/
  These files are exact copies of the originals at those commits.
  README.txt, RUN.txt, and SHA256SUMS were written for this packet;
  expected-report.txt is copied from the report in RESULT.md section 3.1.

Scope
  An exact finite construction, not a fitted simulation. No random seed,
  sampling, simulation length, tolerance, or floating-point acceptance path
  is involved. Logarithms enter only through the price floor and the
  efficiency, as certified rational enclosures.

  Results within the declared worlds: fuel buys open future in all twelve
  worlds; fuel spent where the traveller stands holds 1.21 to 1.79 times as
  much open future per unit of fuel flow as the same fuel spread over every
  bridge; a longer planning horizon did not reliably raise that yield
  (refuted at stores x3 and x2) and shrank the edge of targeting at every
  pair of stores (refuted everywhere); every agent pays 2.2 to 3.0 times the
  price floor for the state it holds.

  Next: fuel aimed along the traveller's own plan, and protection of the
  plan's bridges that has to be paid for.

Hashes
  SHA256SUMS covers this directory except the checksum file itself.
