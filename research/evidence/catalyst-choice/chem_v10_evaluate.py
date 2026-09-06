#!/usr/bin/env python3
"""CHEM-V10 required-slot repair and target-free preflight.

CHEM-V10 reuses the frozen CHEM-V9 features and inferential core.  The sole
analytical change is that each held-out experiment requests cross-fitted slot
scores only for the eligible coded slots present in that experiment's frozen
primary panel.  Training ranks still use all 47 rows of every loaded block.
"""

from __future__ import annotations

import argparse
import collections
import json
import math
import sys
from pathlib import Path
from typing import Iterable, Mapping, Sequence

import numpy as np


V9_DIR = Path(__file__).resolve().parents[1] / "chem_v9"
if str(V9_DIR) not in sys.path:
    sys.path.insert(0, str(V9_DIR))

import chem_v9_evaluate as v9


EXPECTED_GLOBAL_SLOTS = tuple(range(1, 49))
EXPECTED_REQUIRED_SLOTS = tuple(range(1, 48))
EXPECTED_GLOBALLY_OBSERVED_NOT_REQUIRED = (48,)
EXPECTED_TARGET_SHA256 = "58dae5feac0aad193b3a8d6590120b5abfea02447aed093e4b146ccaf9a8f407"


def _block_maps(frozen_dir: Path) -> tuple[dict[str, str], dict[str, str]]:
    rows = v9.read_tsv(
        frozen_dir / "blocks.tsv",
        ("block_id", "experiment", "ligand_rows", "primary_included"),
    )
    loaded = {row["block_id"]: row["experiment"] for row in rows if int(row["ligand_rows"]) > 0}
    primary = {row["block_id"]: row["experiment"] for row in rows if row["primary_included"] == "1"}
    return loaded, primary


def required_slots_by_experiment(
    eligible_memberships: Sequence[v9.Membership],
    primary_blocks: Mapping[str, str],
) -> dict[str, frozenset[int]]:
    """Return only the coded slots used by each frozen eligible primary panel."""
    result: dict[str, set[int]] = collections.defaultdict(set)
    seen_blocks: set[str] = set()
    for row in eligible_memberships:
        if row.block_id not in primary_blocks:
            continue
        if primary_blocks[row.block_id] != row.experiment:
            raise ValueError(f"primary block/experiment mismatch for {row.block_id}")
        result[row.experiment].add(row.coded_ligand_slot)
        seen_blocks.add(row.block_id)
    if seen_blocks != set(primary_blocks):
        raise ValueError("primary blocks and eligible membership blocks do not match")
    return {experiment: frozenset(slots) for experiment, slots in sorted(result.items())}


def target_free_slot_preflight(
    frozen_dir: Path,
    *,
    enforce_frozen_shape: bool = True,
) -> dict[str, object]:
    """Audit required LOEO coverage without opening any outcome field or table."""
    if enforce_frozen_shape:
        v9.verify_freeze_manifest(frozen_dir)
    loaded_memberships = v9.load_memberships(frozen_dir / "loaded_memberships.tsv")
    eligible_memberships = v9.load_memberships(frozen_dir / "eligible_memberships.tsv")
    loaded_blocks, primary_blocks = _block_maps(frozen_dir)
    required = required_slots_by_experiment(eligible_memberships, primary_blocks)

    reaction_ids = [row.reaction_id for row in loaded_memberships]
    duplicate_reaction_ids = len(reaction_ids) - len(set(reaction_ids))

    primary_panel_mappings: dict[str, dict[str, int]] = {}
    panel_mapping_changes: dict[str, list[str]] = collections.defaultdict(list)
    eligible_by_block: dict[str, list[v9.Membership]] = collections.defaultdict(list)
    for row in eligible_memberships:
        if row.block_id in primary_blocks:
            eligible_by_block[row.block_id].append(row)
    for block_id, rows in sorted(eligible_by_block.items()):
        experiment = primary_blocks[block_id]
        mapping: dict[str, int] = {}
        for row in rows:
            previous = mapping.setdefault(row.structure_id, row.coded_ligand_slot)
            if previous != row.coded_ligand_slot:
                raise ValueError(f"structure changes coded slot within primary block {block_id}")
        if experiment not in primary_panel_mappings:
            primary_panel_mappings[experiment] = mapping
        elif primary_panel_mappings[experiment] != mapping:
            panel_mapping_changes[experiment].append(block_id)

    rows_by_block: dict[str, list[v9.Membership]] = collections.defaultdict(list)
    for row in loaded_memberships:
        if row.block_id not in loaded_blocks:
            raise ValueError(f"loaded membership references nonloaded block {row.block_id}")
        if loaded_blocks[row.block_id] != row.experiment:
            raise ValueError(f"loaded block/experiment mismatch for {row.block_id}")
        rows_by_block[row.block_id].append(row)
    if set(rows_by_block) != set(loaded_blocks):
        raise ValueError("loaded blocks and loaded membership blocks do not match")
    malformed_blocks = {
        block_id: len(rows)
        for block_id, rows in rows_by_block.items()
        if len(rows) != 47 or len({row.coded_ligand_slot for row in rows}) != 47
    }
    anomalous_slot_blocks: list[dict[str, object]] = []
    standard_slots = set(range(1, 48))
    for block_id, rows in sorted(rows_by_block.items()):
        slots = {row.coded_ligand_slot for row in rows}
        if slots != standard_slots:
            anomalous_slot_blocks.append(
                {
                    "block_id": block_id,
                    "experiment": loaded_blocks[block_id],
                    "missing_standard_slots": sorted(standard_slots - slots),
                    "extra_slots": sorted(slots - standard_slots),
                }
            )

    global_slots = sorted({row.coded_ligand_slot for row in loaded_memberships})
    required_union = sorted(set().union(*required.values()))
    globally_observed_not_required = sorted(set(global_slots) - set(required_union))
    slot_source_experiments: dict[int, set[str]] = collections.defaultdict(set)
    slot_row_counts: collections.Counter[int] = collections.Counter()
    for row in loaded_memberships:
        slot_source_experiments[row.coded_ligand_slot].add(row.experiment)
        slot_row_counts[row.coded_ligand_slot] += 1

    coverage: dict[str, object] = {}
    total_missing = 0
    for held_out, slots in sorted(required.items()):
        training_counts = {
            slot: sum(
                1
                for row in loaded_memberships
                if row.experiment != held_out and row.coded_ligand_slot == slot
            )
            for slot in sorted(slots)
        }
        missing = [slot for slot, count in training_counts.items() if count == 0]
        total_missing += len(missing)
        coverage[held_out] = {
            "required_slots": sorted(slots),
            "required_slot_count": len(slots),
            "missing_required_training_slots": missing,
            "minimum_training_rows_per_required_slot": min(training_counts.values()),
        }

    report = {
        "protocol": "CHEM-V10",
        "repair": "score only held-out eligible primary-panel slots; rank all 47 loaded rows",
        "blindness": {
            "outcome_fields_decoded": 0,
            "outcome_tables_opened": 0,
            "inputs": ["blocks.tsv", "eligible_memberships.tsv", "loaded_memberships.tsv"],
        },
        "loaded_blocks": len(loaded_blocks),
        "loaded_memberships": len(loaded_memberships),
        "duplicate_loaded_reaction_ids": duplicate_reaction_ids,
        "primary_blocks": len(primary_blocks),
        "malformed_47_row_loaded_blocks": malformed_blocks,
        "anomalous_slot_blocks": anomalous_slot_blocks,
        "primary_experiments": len(required),
        "primary_panel_mapping_changes": dict(panel_mapping_changes),
        "globally_observed_slots": global_slots,
        "required_slot_union": required_union,
        "globally_observed_but_not_required": globally_observed_not_required,
        "required_missing_total": total_missing,
        "coverage_by_held_out_experiment": coverage,
        "slot_48_audit": {
            "globally_observed": 48 in global_slots,
            "required_by_any_primary_experiment": 48 in required_union,
            "loaded_rows": slot_row_counts[48],
            "source_experiments": sorted(slot_source_experiments[48]),
        },
    }
    if enforce_frozen_shape:
        if tuple(global_slots) != EXPECTED_GLOBAL_SLOTS:
            raise ValueError("frozen globally observed slot set changed")
        if tuple(required_union) != EXPECTED_REQUIRED_SLOTS:
            raise ValueError("frozen required slot union changed")
        if tuple(globally_observed_not_required) != EXPECTED_GLOBALLY_OBSERVED_NOT_REQUIRED:
            raise ValueError("frozen nonrequired observed slot set changed")
        if malformed_blocks:
            raise ValueError("a frozen loaded block is not a unique 47-slot ranking block")
        if len(loaded_memberships) != v9.EXPECTED_LOADED_MEMBERSHIPS or duplicate_reaction_ids:
            raise ValueError("frozen loaded membership ID coverage changed")
        if len(primary_blocks) != v9.EXPECTED_PRIMARY_BLOCKS:
            raise ValueError("frozen primary block count changed")
        if panel_mapping_changes:
            raise ValueError("a frozen primary experiment changes structure-to-slot mapping")
        if len(required) != v9.EXPECTED_PRIMARY_EXPERIMENTS:
            raise ValueError("frozen primary experiment count changed")
        if total_missing:
            raise ValueError(f"{total_missing} required held-out slot scores lack training coverage")
        if report["slot_48_audit"] != {
            "globally_observed": True,
            "required_by_any_primary_experiment": False,
            "loaded_rows": 1,
            "source_experiments": ["XZ-01-1"],
        }:
            raise ValueError("frozen slot 48 audit changed")
        if anomalous_slot_blocks != [
            {
                "block_id": "00ab912e0eff6a901dbc3314af67dd92b65640918126f7036976b4f64a90b0c8",
                "experiment": "XZ-01-1",
                "missing_standard_slots": [16],
                "extra_slots": [48],
            }
        ]:
            raise ValueError("frozen slot 48/missing-slot-16 anomaly changed")
    report["pass"] = not malformed_blocks and total_missing == 0
    return report


def crossfit_required_slot_rank_percentiles(
    loaded_memberships: Sequence[v9.Membership],
    outcomes: Mapping[str, float],
    loaded_blocks: Mapping[str, str],
    required_slots: Mapping[str, Iterable[int]],
    *,
    expected_rows_per_block: int | None = 47,
) -> dict[str, dict[int, float]]:
    """Build LOEO scores for required slots while ranking every loaded row."""
    by_block: dict[str, list[v9.Membership]] = collections.defaultdict(list)
    for row in loaded_memberships:
        if row.block_id not in loaded_blocks:
            raise ValueError(f"loaded membership references nonloaded block {row.block_id}")
        if loaded_blocks[row.block_id] != row.experiment:
            raise ValueError(f"loaded block/experiment mismatch for {row.block_id}")
        by_block[row.block_id].append(row)
    if set(by_block) != set(loaded_blocks):
        raise ValueError("loaded blocks and loaded membership blocks do not match")

    source_values: dict[str, dict[int, list[float]]] = collections.defaultdict(
        lambda: collections.defaultdict(list)
    )
    for block_id, rows in by_block.items():
        if expected_rows_per_block is not None and len(rows) != expected_rows_per_block:
            raise ValueError(f"loaded block {block_id} does not have {expected_rows_per_block} rows")
        slots = [row.coded_ligand_slot for row in rows]
        if len(set(slots)) != len(slots):
            raise ValueError(f"loaded block {block_id} repeats a coded ligand slot")
        ranks = v9.average_ranks([outcomes[row.reaction_id] for row in rows])
        percentiles = (ranks - 1.0) / max(len(rows) - 1, 1)
        experiment = loaded_blocks[block_id]
        for row, percentile in zip(rows, percentiles):
            source_values[experiment][row.coded_ligand_slot].append(float(percentile))

    result: dict[str, dict[int, float]] = {}
    source_experiments = set(source_values)
    for held_out, held_out_slots in sorted(required_slots.items()):
        training_experiments = source_experiments - {held_out}
        slot_scores: dict[int, float] = {}
        for slot in sorted(set(held_out_slots)):
            values = [
                value
                for experiment in training_experiments
                for value in source_values[experiment].get(slot, ())
            ]
            if not values:
                raise ValueError(f"required slot {slot} has no cross-fit training coverage for {held_out}")
            slot_scores[slot] = math.fsum(values) / len(values)
        result[held_out] = slot_scores
    return result


def load_prepared(
    frozen_dir: Path,
    outcome_table: Path,
    *,
    enforce_frozen_shape: bool = True,
) -> v9.PreparedEvaluation:
    """Prepare V9's analysis with only the V10 required-slot repair."""
    if enforce_frozen_shape:
        v9.verify_freeze_manifest(frozen_dir)
        target_free_slot_preflight(frozen_dir, enforce_frozen_shape=True)
        if v9.sha256_file(outcome_table) != EXPECTED_TARGET_SHA256:
            raise ValueError("CHEM-V10 target artifact hash differs from the preserved V9 reveal")
    descriptors = v9.read_tsv(
        frozen_dir / "ligand_descriptors.tsv", ("structure_id", *v9.DESCRIPTOR_PREDICTORS)
    )
    descriptors.sort(key=lambda row: row["structure_id"])
    structure_ids = tuple(row["structure_id"] for row in descriptors)
    if len(set(structure_ids)) != len(structure_ids):
        raise ValueError("descriptor structure IDs are not unique")
    descriptor_values = np.asarray(
        [[float(row[predictor]) for predictor in v9.DESCRIPTOR_PREDICTORS] for row in descriptors],
        dtype=np.float64,
    )
    if not np.all(np.isfinite(descriptor_values)):
        raise ValueError("descriptor table contains a nonfinite value")
    structure_index = {structure_id: index for index, structure_id in enumerate(structure_ids)}

    eligible = v9.load_memberships(frozen_dir / "eligible_memberships.tsv")
    loaded = v9.load_memberships(frozen_dir / "loaded_memberships.tsv")
    if {row.structure_id for row in eligible} != set(structure_ids):
        raise ValueError("membership and descriptor structure sets differ")
    outcomes = v9.load_exact_targets(outcome_table, loaded)
    loaded_blocks, primary_blocks = _block_maps(frozen_dir)
    blocks, duplicate_groups = v9.aggregate_primary_blocks(eligible, outcomes, primary_blocks)
    by_experiment: dict[str, list[v9.AggregatedBlock]] = collections.defaultdict(list)
    for block in blocks:
        by_experiment[block.experiment].append(block)
    required = required_slots_by_experiment(eligible, primary_blocks)
    crossfit_scores = crossfit_required_slot_rank_percentiles(
        loaded,
        outcomes,
        loaded_blocks,
        required,
        expected_rows_per_block=47 if enforce_frozen_shape else None,
    )

    experiments: list[v9.ExperimentData] = []
    for experiment, experiment_blocks in sorted(by_experiment.items()):
        panel = experiment_blocks[0].structure_ids
        slots = experiment_blocks[0].coded_ligand_slots
        if any(block.structure_ids != panel for block in experiment_blocks[1:]):
            raise ValueError(f"canonical panel changes across blocks in {experiment}")
        if any(block.coded_ligand_slots != slots for block in experiment_blocks[1:]):
            raise ValueError(f"coded ligand slot changes across blocks in {experiment}")
        if set(map(int, slots)) != set(required[experiment]):
            raise ValueError(f"required slot set differs from the frozen panel in {experiment}")
        values = np.asarray([block.yields for block in experiment_blocks], dtype=np.float64)
        ranks = np.vstack([v9.average_ranks(row) for row in values])
        centers = ranks - ranks.mean(axis=1, keepdims=True)
        experiments.append(
            v9.ExperimentData(
                name=experiment,
                block_ids=tuple(block.block_id for block in experiment_blocks),
                panel_indices=np.asarray([structure_index[value] for value in panel], dtype=np.int64),
                outcomes=values,
                outcome_rank_centers=centers,
                outcome_rank_norms=np.linalg.norm(centers, axis=1),
                coded_ligand_slots=np.asarray(slots, dtype=np.float64),
                crossfit_slot_rank_percentiles=np.asarray(
                    [crossfit_scores[experiment][int(slot)] for slot in slots], dtype=np.float64
                ),
            )
        )

    strata_rows = v9.read_tsv(
        frozen_dir / "permutation_strata.tsv", ("structure_id", "stratum_id")
    )
    if {row["structure_id"] for row in strata_rows} != set(structure_ids):
        raise ValueError("permutation strata and descriptor structure sets differ")
    grouped_strata: dict[str, list[int]] = collections.defaultdict(list)
    for row in strata_rows:
        grouped_strata[row["stratum_id"]].append(structure_index[row["structure_id"]])
    strata = tuple(tuple(sorted(indices)) for _, indices in sorted(grouped_strata.items()))

    experiment_names = sorted({row.experiment for row in eligible})
    incidence = collections.Counter((row.structure_id, row.experiment) for row in eligible)
    signatures = {
        index: tuple(incidence[(structure_ids[index], experiment)] for experiment in experiment_names)
        for index in range(len(structure_ids))
    }
    for stratum in strata:
        if len({signatures[index] for index in stratum}) != 1:
            raise ValueError("permutation stratum crosses observed experiment incidence")

    rows_by_structure: dict[str, set[int]] = collections.defaultdict(set)
    for row in eligible:
        rows_by_structure[row.structure_id].add(row.sample_well_row)
    movers = tuple(
        structure_index[structure_id]
        for structure_id in structure_ids
        if len(rows_by_structure[structure_id]) >= 2
    )
    prepared = v9.PreparedEvaluation(
        structure_ids=structure_ids,
        descriptor_values=descriptor_values,
        experiments=tuple(experiments),
        strata=strata,
        loaded_membership_count=len(loaded),
        eligible_membership_count=len(eligible),
        loaded_block_count=len(loaded_blocks),
        primary_block_count=len(blocks),
        duplicate_block_structure_groups=duplicate_groups,
        mover_structure_indices=movers,
    )
    if enforce_frozen_shape:
        checks = (
            (len(structure_ids), v9.EXPECTED_STRUCTURES, "structure count"),
            (len(loaded), v9.EXPECTED_LOADED_MEMBERSHIPS, "loaded membership count"),
            (len(eligible), v9.EXPECTED_ELIGIBLE_MEMBERSHIPS, "eligible membership count"),
            (len(loaded_blocks), v9.EXPECTED_LOADED_BLOCKS, "loaded block count"),
            (len(blocks), v9.EXPECTED_PRIMARY_BLOCKS, "primary block count"),
            (len(experiments), v9.EXPECTED_PRIMARY_EXPERIMENTS, "primary experiment count"),
            (len(movers), 39, "slot-row mover cohort"),
        )
        for actual, expected, label in checks:
            if actual != expected:
                raise ValueError(f"frozen {label} changed: {actual} != {expected}")
        if any(len(experiment.panel_indices) < 9 for experiment in experiments):
            raise ValueError("a frozen primary panel has fewer than nine structures")
    return prepared


def evaluate_paths(frozen_dir: Path, outcome_table: Path, report_path: Path) -> dict[str, object]:
    prepared = load_prepared(frozen_dir, outcome_table, enforce_frozen_shape=True)
    report = v9.run_evaluation(prepared)
    report["analysis"]["protocol"] = (
        "CHEM-V10 post-reveal/pre-statistic target-independent mechanical repair"
    )
    report["analysis"]["position_controls"]["crossfit_method"] = (
        "For each held-out experiment, rank all 47 rows in every loaded block from the other "
        "16 source experiments, then construct scores only for coded slots required by the "
        "held-out eligible primary panel."
    )
    report["analysis"]["claim_boundary"] = (
        "Repaired corroboration, not a pristine confirmation; all V9 associational and causal limits remain."
    )
    report_path.parent.mkdir(parents=True, exist_ok=True)
    with report_path.open("w", encoding="utf-8", newline="\n") as handle:
        json.dump(report, handle, indent=2, sort_keys=True, ensure_ascii=False)
        handle.write("\n")
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    preflight = commands.add_parser("preflight")
    preflight.add_argument("frozen_dir", type=Path)
    preflight.add_argument("report", type=Path, nargs="?")
    evaluate = commands.add_parser("evaluate")
    evaluate.add_argument("frozen_dir", type=Path)
    evaluate.add_argument("outcome_table", type=Path)
    evaluate.add_argument("report", type=Path)
    arguments = parser.parse_args()
    if arguments.command == "preflight":
        report = target_free_slot_preflight(arguments.frozen_dir)
        if arguments.report:
            arguments.report.parent.mkdir(parents=True, exist_ok=True)
            arguments.report.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
        print(json.dumps(report, indent=2, sort_keys=True))
    else:
        report = evaluate_paths(arguments.frozen_dir, arguments.outcome_table, arguments.report)
        print(json.dumps(report["decision"], indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
