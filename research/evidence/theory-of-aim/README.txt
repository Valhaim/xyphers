XYPHERS THEORY OF AIM EVIDENCE MAP

Article
  Can a Theory Predict What Energy Buys?

Read
  theory/THEORY.md is the cavity mean-field theory of the price of open
  future: its construction, its retrospective validation on CAL-CEF-4 (all
  48 edges within 0.62%), what it explains, and its predictions.
  theory/cavity.py implements it; theory/validation.txt and
  theory/scaling.txt are its outputs on CAL-CEF-4 and out to twenty islands;
  theory/predictions_cef5.py and theory/predictions_cef5.txt are the 56
  predictions frozen for this experiment.
  BOUNDARY.md is the preregistration of CAL-CEF-5: the CAL-CEF-4 world
  extended by a traveller speed v and a measure radius R, two new pairs of
  stores, the frozen predictions with their SHA-256, twelve acceptance
  gates, nine deliberately broken controls, and five hypotheses that could
  fail. It was frozen before any evaluator ran.
  RESULT.md records the single frozen execution: verdict, every prediction
  against its exact value, review chronology, file digests, toolchain, the
  complete report, and the independent check.
  expected-report.txt is the report that run printed.

Inspect and run
  source/xypher-theory-of-aim-proof/ is the dependency-free Rust verifier,
  with the exact arithmetic of CAL-CEF-3 and CAL-CEF-4 reused unchanged and
  an embedded copy of the frozen predictions checked against their SHA-256.
  RUN.txt gives the commands and the expected result.
  independent_check.py is a separate implementation written from the
  boundary alone; expected-independent-check.txt is its output.

Provenance
  theory/THEORY.md  thaim commit 665da9b276bb0f3c5b89a21c06500502aa39ae44,
                    research/physics/derivable/
                    xypher-agency-cavity-theory.md
  theory/ (other)   thaim commit c06ffaebbf3fad3a07742549f93cb8a369db29c6,
                    research/physics/xypher-agency-theory/
  BOUNDARY.md       thaim commit 665da9b276bb0f3c5b89a21c06500502aa39ae44,
                    research/physics/derivable/
                    xypher-theory-of-aim-boundary.md
  RESULT.md         thaim commit 45d6e047018174ed770b5ac83ea14e8168f48c41,
                    research/physics/derivable/
                    xypher-theory-of-aim-result.md
  source/           thaim commit d91de4702a887f8b251dd273836acc1a518edce1,
                    research/physics/xypher-theory-of-aim-proof/
                    (Git tree f66ebffafc8f39bb34bcd25fab2273127abd54e9)
  independent_check.py and expected-independent-check.txt
                    thaim commit 45d6e047018174ed770b5ac83ea14e8168f48c41,
                    research/physics/xypher-theory-of-aim-check/
  These files are exact copies of the originals at those commits.
  README.txt, RUN.txt, and SHA256SUMS were written for this packet;
  expected-report.txt is copied from the report in RESULT.md section 3.1.

Scope
  Exact finite worlds and a floating-point theory whose predictions were
  frozen as six-decimal values before the run; every comparison between them
  is exact. Results: at traveller speed 1 all 32 predictions lie within
  0.36% of the exact edges (new pairs of stores, and a three-step measure);
  at speeds 4 and 16, 14 of 24 lie outside 1%, by up to 8.78%, always below
  the exact value; every qualitative prediction holds (faster travellers
  gain less from aim, blind foresight lowers the edge, the nearest aim leads,
  and the one-line law over-predicts local aim by at most 3%).

  Next: a theory that keeps pairs of bridges exactly; and informed plans.

Hashes
  SHA256SUMS covers this directory except the checksum file itself.
