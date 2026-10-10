XYPHERS COMMITTED FUTURES EVIDENCE MAP

Article
  Committed Futures as Microstates

Read
  manuscript.pdf is the manuscript "Committed futures as microstates: causal
  entropic bias as configurational entropy at the bath temperature", with
  every proof. manuscript.tex and references.bib are
  its source; pdflatex, bibtex, pdflatex, pdflatex (REVTeX 4.2) rebuild it.
  The TeX source names its bibliography "references", the file beside it.

Run
  checks.py recomputes every exact check that the manuscript marks as not
  preregistered, with whole numbers and exact fractions only. It needs
  Python 3.8 or later and nothing else. From this folder:

    python3 checks.py

  It prints 52 lines ending with ALL CHECKS PASS, in a few seconds.
  expected-checks.txt is that output. RUN.txt gives the comparison command
  and how to rebuild the manuscript.

What checks.py covers (manuscript numbering)
  configuration-law    Theorem 1 and Proposition 1: the law under uniform and
                       one-step replanning, N = 2..4, tau = 1..3, b = 2, 3,
                       including N = 2, outside the preregistered scope
  primary              Table I and Fig. 1
  plan-count           enumerated walks equal matrix-power counts
  flux-and-protection  Lemma 3 and Remark 3, N = 3..5
  non-Markov           Proposition 2, both replanning rules
  many-travellers      Proposition 3: two and three travellers
  degree-law           Proposition 4, N = 3..6
  build-only           Proposition 5
  size-biased-identity Remark 2
  long-horizon         the exploratory N = 5 sweep after Proposition 6
  destination-law      open problem Q1

Relation to CAL-CEF-1
  The preregistered Rust verifier in ../causal-entropic/ checked the
  configuration law, the odds, the degree law, and same-temperature contact on
  eight cases, once, from a frozen source. checks.py is not preregistered. It
  was written after that run and reproduces the manuscript's additional
  exact checks.

Provenance
  manuscript.pdf        thaim commit e08d1aea56bedf2ec9ba25b1c7565bc1b8f13204,
  manuscript.tex        research/physics/paper-committed-futures/
  references.bib        (paper.pdf, paper.tex, references.bib, checks.py)
  checks.py
  These four files are byte-for-byte copies; the manuscript files were
  renamed from paper.*. expected-checks.txt is the output of checks.py at
  that commit under Python 3.9.6. README.txt, RUN.txt, and SHA256SUMS
  were written for this packet.

Scope
  Exact statements about a declared finite model, proved for every number of
  islands N >= 2 and every horizon tau >= 1 and checked exactly on the cases
  listed above. No random seed, sampling, tolerance, or floating-point
  acceptance path is involved.

  Established within the declared model: the long-run law of bridge
  configurations is N_tau(G) exp(-U/alpha)/Z; its causal entropic odds occur
  at T_c = T_r = alpha, whatever the replanning rate; the mechanism is
  protection; one-step replanning leaves the law unchanged; the
  configuration process is not Markov; travellers multiply their counts; the
  tau = 2 degree law; the long-horizon limit ln(Lambda).

  Open: whether the Shannon entropy of realized trajectories or of random-walk
  endpoints can play the role of ln N_tau; growth and nonequilibrium drive;
  learning; control.

Hashes
  SHA256SUMS covers this directory except the checksum file itself.
