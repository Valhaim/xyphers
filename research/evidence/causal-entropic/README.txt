XYPHERS CAUSAL ENTROPIC EVIDENCE MAP

Article
  When Futures Become Entropy

General proofs
  ../committed-futures/ holds the manuscript that proves these laws for every
  N >= 2 and tau >= 1, with an exact script for its additional checks.

Read
  BOUNDARY.md is the preregistration: the construction, the derived
  predictions, twelve acceptance gates with their check order, eight
  deliberately broken controls with their expected first failures, and the
  stated claims and non-claims. It was frozen before any evaluator ran.
  RESULT.md records the single frozen execution: verdict, review chronology,
  file digests, toolchain, and the complete report.
  expected-report.txt is that report, byte for byte.

Inspect and run
  source/xypher-causal-entropic-proof/ is the dependency-free Rust verifier
  and the prospective Futuruna construction (xypher.runa). RUN.txt gives the
  commands and the expected result.

Provenance
  BOUNDARY.md       thaim commit 3a6405c43390400eff43ba722273c96f76e29dac,
                    research/physics/derivable/
                    xypher-causal-entropic-thermodynamics-boundary.md
  RESULT.md         thaim commit f61fbd43,
                    research/physics/derivable/
                    xypher-causal-entropic-thermodynamics-result.md
  source/           thaim commit 31307b6171bead8c502fb767b8dca2a398f1d178,
                    research/physics/xypher-causal-entropic-proof/
                    (Git tree e170da4e0e240abe027dac85e577b88ee0fd44d5)
  These files are byte-for-byte copies. README.txt, RUN.txt, and SHA256SUMS
  were written for this packet; expected-report.txt is copied from the
  report in RESULT.md section 3.1.

Scope
  An exact finite construction, not a fitted simulation. No random seed,
  sampling, simulation length, tolerance, or floating-point acceptance path
  is involved.

  Established within the declared model: a traveller that holds one walk
  from home as executable state, walks it, replans uniformly at home, and
  builds or dismantles only bridges at its own island satisfies the
  operational thermodynamic tests T1-T5 across an eight-case family, and T6
  for two contact pairs; its equilibrium law over bridge configurations is
  N_tau(G) exp(-U/alpha)/Z; the equilibrium-averaged build/dismantle odds
  equal exp(Xi/alpha) with Xi = alpha * Delta ln N_tau - Delta U, although no
  rate depends on N_tau.

  Open: whether the Shannon entropy of realized trajectories or of random-walk
  endpoints can play the role of ln N_tau; growth with new islands or a
  nonequilibrium drive; learning and control. The odds hold as averages;
  there is no configuration-level rate rule.

Hashes
  SHA256SUMS covers this directory except the checksum file itself.
