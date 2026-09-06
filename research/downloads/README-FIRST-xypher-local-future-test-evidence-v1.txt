XYPHERS LOCAL-FUTURE-TEST EVIDENCE v1

1. Download xypher-local-future-test-evidence-v1.zip and its .sha256 file
   into the same directory.
2. Verify the archive:
     shasum -a 256 -c xypher-local-future-test-evidence-v1.zip.sha256
3. Extract it and read README.txt.
4. Verify its contents from the extracted directory:
     shasum -a 256 -c SHA256SUMS
5. Follow RUN.txt. The practical reproduction is the N=3..6 classifier;
   the N=7 and N=8 exhaustive sources have much larger run costs.

The packet proves a bounded endpoint-entropy result. It does not prove an
actor-payoff, welfare, thermodynamic, or AI-alignment theorem.
