XYPHERS THAIM REWARD EVIDENCE MAP

Article
  Can THAIM Train an Agent?

Read
  BOUNDARY.md is the preregistration: the driven archipelago, the three ways
  of paying THAIM (receipts, the agent's own net flow, the world's net flow)
  with the theory of each, the agents and the frozen training protocol, exact
  evaluation, the six worlds with every value fixed in advance, thirteen
  acceptance gates, six deliberately broken controls, five hypotheses that
  could fail, and every number seen during the pilots and the calibration.
  It was frozen after four rounds of blind review.
  RESULT.md records the single run: verdicts with exact values, what was
  learned, the reported results, the review chronology, file digests, and the
  independent check.
  report.txt is the complete report the run printed.

Inspect and run
  source/ is the apparatus: the world, the trainers, the exact evaluator,
  the gates, controls and hypotheses, and embargo-safe tests.
  tables/<world>/<agent>.txt are the 78 frozen agents: one line per
  observation key, "index<TAB>numerator/denominator". Each file's SHA-256 is
  printed on its agent's line of report.txt. In file names, spaces become _
  and / becomes -.
  pilots/ holds the design-pilot and calibration scripts and outputs listed
  in BOUNDARY.md section 11.
  independent/ is a separate program written from the boundary and the
  tables alone, with its own exact solver; output.txt ends OVERALL AGREE.
  RUN.txt gives the commands and the expected results.

Provenance
  BOUNDARY.md   thaim commit 6af3757852beb0a5594f45872482a1265bfea624,
                research/physics/derivable/xypher-thaim-reward-boundary.md
  pilots/       thaim commit 6af3757852beb0a5594f45872482a1265bfea624,
                research/physics/xypher-thaim-reward-proof/pilots/
  source/       thaim commit 637d8e0ae1ae0f9df0f4609c1969efe2bcf1629a,
                research/physics/xypher-thaim-reward-proof/
                (Git tree da4f86d667b907b0a8349a9ba4030bdbbb12c2ff)
  report.txt and tables/
                thaim commit 6b770e074927226f0ebbd0ecc3229e0a81aefcc8,
                research/physics/xypher-thaim-reward-proof/
  RESULT.md     thaim commit 2328e895efcab168db9daf3666cc29639fbb552c,
                research/physics/derivable/xypher-thaim-reward-result.md
  independent/  thaim commit 2328e895efcab168db9daf3666cc29639fbb552c,
                research/physics/xypher-thaim-reward-check/
  These files are exact copies of the originals at those commits.
  README.txt, RUN.txt, and SHA256SUMS were written for this packet.

Scope
  Exact finite worlds: four islands, a walking traveller, weather from a cold
  store, fuel from a hot store. Small policies were trained in floating point
  and frozen as exact rational tables; every verdict is computed exactly on
  the frozen tables. Results: trained on its own net THAIM flow, an agent
  learned to fuel exactly the builds whose futures ratio exceeds the cold
  store's factor (agreement 1.0000 in six worlds), earned 0.9996 or more of the
  oracle's value where some build pays, and did nothing where none pays.
  Trained on THAIM receipts it fuelled builds worth less than their timber and
  lost net THAIM. The prediction that it would beat hand-written aim on the
  matched-power edge failed in one of four worlds. A small neural network
  trained on the same reward mostly collapsed to doing nothing.

  Next: a network that does not collapse, a memory priced by the information
  law, and several agents trained on their own net THAIM in one world.

Hashes
  SHA256SUMS covers this directory except the checksum file itself.
