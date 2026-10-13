# CAL-THAIM-1 apparatus: Can THAIM train an agent?

This apparatus measures the boundary
`research/physics/derivable/xypher-thaim-reward-boundary.md`. It does the
following:
1. Trains small policies on three forms of THAIM in an exact driven archipelago.
2. Freezes each trained policy as an exact rational rate table.
3. Evaluates every table in exact arithmetic.
4. Runs gates G01–G13 and controls C01–C06, and decides hypotheses H1–H5.

It is a measurement apparatus, not a Xypher runtime.

## Files

| file | contents |
|---|---|
| `thaimworld.py` | The world (WEATHER, WALK, fuel pairs, observation keys, walk-count ratios). Also the float steady-state solve, the adjoint gradient, the three rewards in their training forms, the tabular and MLP trainers, and freezing. |
| `exact.py` | Exact steady states (python-flint `fmpq_mat`) and linear forms in logarithms. Identities are decided over a coprime base; signs by certified enclosures with precision doubling. Also the warming rival, the floor, and rule agreement. |
| `run.py` | The single entry point: agents, gates in the declared order, controls on their witnesses, hypotheses, and the report. |
| `tests.py` | Embargo-safe tests, on non-declared worlds only (N = 2, N = 3, and N = 4 with `tau = 2`). |
| `pilots/` | Design-pilot and calibration scripts and outputs. These are disclosed in the boundary, section 11, with their SHA-256. |
| `requirements.txt` | numpy 2.0.2, scipy 1.13.1, python-flint 0.6.0. |

## Run

```sh
python3 -m venv venv && venv/bin/pip install -r requirements.txt
venv/bin/python tests.py            # embargo-safe; about one minute
venv/bin/python run.py > report.txt # the single run; declared worlds
```

The report ends with the five hypothesis verdicts and `OVERALL PASS` or
`OVERALL FAIL`.

## Embargo

Until three static reviewers approve the exact commit, only
`python -m py_compile` and `tests.py` may run. The section 11 calibration
exception of the boundary is closed. No driven steady state of a declared world
(N = 4 with `tau` in {3, 4}) is solved before the run.

The run writes every frozen table to `tables/<world>/<agent>.txt` (canonical
text, one `index<TAB>numerator/denominator` line per key) and prints its
SHA-256 in the report.
