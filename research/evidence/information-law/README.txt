XYPHERS INFORMATION LAW EVIDENCE MAP

Article
  Can Knowing the World Replace Fuel?

Read
  THEOREM.md is the information law (formula V of the formula sheet): the
  class of worlds it covers, the exact balance of fuel, information, and
  memory, the proof, the construction that makes the exchange rate exactly
  one, the demon world in closed form, what is borrowed and what is new, and
  the review record (blind referee, two rounds; independent check, two runs).

Inspect and run
  infolaw.py is the exact ledger library. Steady states are exact rationals;
  identities are decided over a pairwise coprime base of integers, where every
  coefficient must vanish; inequalities are decided by exact signs or by
  certified arb enclosures.
  verify.py checks the theorem in 500 random worlds, the demon world, and two
  three-island archipelagos. verification.txt is its recorded output.
  independent/ holds a separate program written from the theorem alone. It
  uses its own generator and solver, and it factors every integer into
  primes. output.txt is its run on the final theorem. output-run1.txt is its
  first run, on the first version of the theorem, which caught a printing
  error since corrected. README.md describes both runs.
  RUN.txt gives the commands and the expected results.

Provenance
  THEOREM.md        thaim commit 042e89d37bcf0b389a7f8adb16847e09ef1afab8,
                    research/physics/derivable/xypher-information-law.md
  infolaw.py, verify.py, verification.txt
                    thaim commit 6604006083cc0d4f6d80dd20173dc6bfacfbea81,
                    research/physics/xypher-information-law/
  independent/      thaim commit b9242e33e89ddeb4a4f52fadc5121e98811a7f04,
                    research/physics/xypher-information-law/independent/
  These files are exact copies of the originals at those commits.
  README.txt, RUN.txt, and SHA256SUMS were written for this packet.

Scope
  A theorem for every finite bipartite world (world states and memory
  states) whose undriven channels do not read the agent's memory. The
  world's fuel can fall below the Horowitz-Zhou-England floor by at most the
  information the agent's actions use; the memory pays at least the
  information it learns, which exceeds that by what the world's noise erases;
  counted in entropy the total never falls below the floor. The coefficient
  one is sharp. The CAL-CEF-1 to 5 worlds lie outside the class: their
  weather reads the plan. They are its empty-memory case (THEOREM.md
  section 8).

  Next: sensing agents with fuel of fixed strength against blind ones, with
  the exact condition of THEOREM.md section 6 written down in advance.

Hashes
  SHA256SUMS covers this directory except the checksum file itself.
