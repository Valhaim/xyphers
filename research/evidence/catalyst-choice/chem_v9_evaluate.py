#!/usr/bin/env python3
"""Frozen CHEM-V9 target reveal and confirmatory evaluator.

This file is frozen before target access.  Its ``extract-targets`` command is
the only CHEM-V9 code path that decodes ``Reaction.outcomes``.  Do not run that
command until the feature freeze has been committed and pushed.
"""

from __future__ import annotations

import argparse
import collections
import csv
import hashlib
import itertools
import json
import math
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Mapping, Sequence

import numpy as np
import pyarrow.parquet as pq
from ord_schema.proto import reaction_pb2


DATASET_ID = "ord_dataset-805ad863feef48579d95d86a728035f4"
DATASET_SHA256 = "31b7c4762145e5dad973050373cd967bdd85caf3a9858767c83a17fed6c4af36"
DATASET_BYTES = 5_088_296
SOURCE_ROWS = 50_688
SOURCE_ROW_GROUPS = 51

EXPECTED_STRUCTURES = 45
EXPECTED_LOADED_MEMBERSHIPS = 49_632
EXPECTED_ELIGIBLE_MEMBERSHIPS = 17_632
EXPECTED_LOADED_BLOCKS = 1_056
EXPECTED_PRIMARY_BLOCKS = 864
EXPECTED_PRIMARY_EXPERIMENTS = 14
EXPECTED_PERMUTATIONS = 6_912

CANDIDATE = "s_tau_p_rh_tau5"
SECONDARY = "s_tau_p_free_tau5"
POSITION_CONTROL = "coded_ligand_slot"
CROSSFIT_POSITION_CONTROL = "crossfit_slot_rank_percentile"
DESCRIPTOR_BASELINES = (
    "all_atom_count",
    "heavy_atom_count",
    "all_atom_edge_count",
    "molecular_weight",
    "wiener_index",
    "randic_index",
    "zagreb_m1",
    "balaban_j",
    "shell_r1",
    "shell_r2",
    "shell_r3",
    "shell_r4",
    "shell_r5",
    "ball_r1",
    "ball_r2",
    "ball_r3",
    "ball_r4",
    "ball_r5",
    "endpoint_support_rh_tau5",
    "endpoint_support_free_tau5",
    "branch_excess_r5",
    "mean_p_distance",
)
DESCRIPTOR_PREDICTORS = (CANDIDATE, SECONDARY, *DESCRIPTOR_BASELINES)
FIXED_POSITION_CONTROLS = (POSITION_CONTROL, CROSSFIT_POSITION_CONTROL)
ALL_PREDICTORS = (*DESCRIPTOR_PREDICTORS, *FIXED_POSITION_CONTROLS)
BASELINES = (*DESCRIPTOR_BASELINES, *FIXED_POSITION_CONTROLS)

BOOTSTRAP_REPLICATES = 10_000
BOOTSTRAP_SEED = 0x4348454D5639B007
BOOTSTRAP_LOWER_INDEX = 249
BOOTSTRAP_UPPER_INDEX = 9_749
REQUIRED_FROZEN_ARTIFACTS = {
    "alias_audit.tsv",
    "blocks.tsv",
    "cohort_audit.tsv",
    "eligible_memberships.tsv",
    "experiment_panels.tsv",
    "ligand_descriptors.tsv",
    "loaded_memberships.tsv",
    "permutation_strata.tsv",
    "plate_layout_audit.tsv",
}
REQUIRED_FREEZE_INPUTS = {
    "CHEM-V9-Preregistration.md",
    "README.md",
    "chem_v9_evaluate.py",
    "chem_v9_prepare.py",
    "requirements.txt",
    "test_chem_v9_evaluate.py",
    "test_chem_v9_prepare.py",
}


@dataclass(frozen=True)
class Membership:
    reaction_id: str
    block_id: str
    experiment: str
    structure_id: str
    coded_ligand_slot: int
    sample_well_row: int


@dataclass(frozen=True)
class AggregatedBlock:
    block_id: str
    experiment: str
    structure_ids: tuple[str, ...]
    yields: tuple[float, ...]
    coded_ligand_slots: tuple[float, ...]
    sample_well_rows: tuple[int, ...]
    source_rows: int


@dataclass(frozen=True)
class ExperimentData:
    name: str
    block_ids: tuple[str, ...]
    panel_indices: np.ndarray
    outcomes: np.ndarray
    outcome_rank_centers: np.ndarray
    outcome_rank_norms: np.ndarray
    coded_ligand_slots: np.ndarray
    crossfit_slot_rank_percentiles: np.ndarray


@dataclass(frozen=True)
class PreparedEvaluation:
    structure_ids: tuple[str, ...]
    descriptor_values: np.ndarray
    experiments: tuple[ExperimentData, ...]
    strata: tuple[tuple[int, ...], ...]
    loaded_membership_count: int
    eligible_membership_count: int
    loaded_block_count: int
    primary_block_count: int
    duplicate_block_structure_groups: int
    mover_structure_indices: tuple[int, ...]


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_tsv(path: Path, required: Iterable[str]) -> list[dict[str, str]]:
    with path.open(encoding="utf-8", newline="") as handle:
        reader = csv.DictReader(handle, delimiter="\t")
        fields = set(reader.fieldnames or ())
        missing = set(required) - fields
        if missing:
            raise ValueError(f"{path}: missing columns {sorted(missing)}")
        return list(reader)


def write_tsv(path: Path, fieldnames: Sequence[str], rows: Iterable[Mapping[str, object]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="") as handle:
        writer = csv.DictWriter(handle, fieldnames=fieldnames, delimiter="\t", lineterminator="\n")
        writer.writeheader()
        writer.writerows(rows)


def verify_freeze_manifest(frozen_dir: Path) -> dict[str, object]:
    manifest_path = frozen_dir / "extraction_summary.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    if manifest.get("blindness", {}).get("decoded_outcomes") != 0:
        raise ValueError("freeze manifest does not attest zero decoded outcomes")
    source = manifest.get("source", {})
    if source.get("dataset_id") != DATASET_ID or source.get("parquet_sha256") != DATASET_SHA256:
        raise ValueError("freeze manifest source identity changed")
    if set(manifest.get("files", {})) != REQUIRED_FROZEN_ARTIFACTS:
        raise ValueError("freeze manifest artifact set changed")
    if set(manifest.get("freeze_inputs", {})) != REQUIRED_FREEZE_INPUTS:
        raise ValueError("freeze manifest input set changed")
    for name, expected in manifest["files"].items():
        actual = sha256_file(frozen_dir / name)
        if actual != expected:
            raise ValueError(f"frozen artifact hash mismatch for {name}: {actual}")
    freeze_root = frozen_dir.parent
    for name, expected in manifest["freeze_inputs"].items():
        actual = sha256_file(freeze_root / name)
        if actual != expected:
            raise ValueError(f"freeze input hash mismatch for {name}: {actual}")
    return manifest


def extract_desired_product_yield(reaction: reaction_pb2.Reaction) -> float:
    """Return the unique finite value at the preregistered ORD outcome path."""
    matches: list[float] = []
    for outcome in reaction.outcomes:
        for product in outcome.products:
            if not product.is_desired_product:
                continue
            for measurement in product.measurements:
                if measurement.type != reaction_pb2.ProductMeasurement.YIELD:
                    continue
                if not measurement.HasField("percentage"):
                    raise ValueError("desired-product YIELD has no percentage")
                matches.append(float(measurement.percentage.value))
    if len(matches) != 1:
        raise ValueError(f"expected exactly one desired-product percentage yield, found {len(matches)}")
    value = matches[0]
    if not math.isfinite(value):
        raise ValueError("desired-product percentage yield is nonfinite")
    return value


def verify_official_source(parquet_path: Path) -> pq.ParquetFile:
    if parquet_path.stat().st_size != DATASET_BYTES:
        raise ValueError("source byte count mismatch")
    digest = sha256_file(parquet_path)
    if digest != DATASET_SHA256:
        raise ValueError(f"source SHA-256 mismatch: {digest}")
    parquet = pq.ParquetFile(parquet_path)
    metadata = parquet.metadata
    if metadata.num_rows != SOURCE_ROWS or metadata.num_row_groups != SOURCE_ROW_GROUPS:
        raise ValueError("source Parquet dimensions changed")
    if parquet.schema_arrow.names != ["reaction_id", "reaction"]:
        raise ValueError(f"unexpected physical columns: {parquet.schema_arrow.names}")
    schema_metadata = parquet.schema_arrow.metadata or {}
    if schema_metadata.get(b"ord.dataset_id", b"").decode() != DATASET_ID:
        raise ValueError("ORD dataset ID metadata mismatch")
    return parquet


def extract_targets(parquet_path: Path, memberships_path: Path, output_path: Path) -> None:
    """Reveal only preregistered membership targets after the freeze is pushed."""
    if output_path.exists():
        raise FileExistsError(f"refusing to overwrite existing target file {output_path}")
    manifest = verify_freeze_manifest(memberships_path.parent)
    if memberships_path.name != "loaded_memberships.tsv":
        raise ValueError("target extraction requires the canonical loaded_memberships.tsv")
    if sha256_file(memberships_path) != manifest["files"]["loaded_memberships.tsv"]:
        raise ValueError("target extraction membership hash does not match the freeze manifest")
    membership_rows = read_tsv(memberships_path, ("reaction_id",))
    required_ids = [row["reaction_id"] for row in membership_rows]
    if len(required_ids) != EXPECTED_LOADED_MEMBERSHIPS or len(set(required_ids)) != len(required_ids):
        raise ValueError("frozen membership reaction IDs are not the expected unique set")
    required = set(required_ids)
    parquet = verify_official_source(parquet_path)
    targets: dict[str, float] = {}
    for batch in parquet.iter_batches(batch_size=1_024, columns=["reaction_id", "reaction"]):
        for row_index in range(batch.num_rows):
            reaction_id = batch.column(0)[row_index].as_py()
            if reaction_id not in required:
                continue
            if reaction_id in targets:
                raise ValueError(f"duplicate source reaction ID {reaction_id}")
            reaction = reaction_pb2.Reaction()
            reaction.ParseFromString(batch.column(1)[row_index].as_py())
            if reaction.reaction_id != reaction_id:
                raise ValueError(f"protobuf/source reaction ID mismatch for {reaction_id}")
            targets[reaction_id] = extract_desired_product_yield(reaction)
    missing = required - set(targets)
    if missing:
        raise ValueError(f"{len(missing)} frozen memberships have no source target")
    write_tsv(
        output_path,
        ("reaction_id", "yield_percentage"),
        ({"reaction_id": reaction_id, "yield_percentage": format(targets[reaction_id], ".17g")}
         for reaction_id in required_ids),
    )


def average_ranks(values: Sequence[float] | np.ndarray) -> np.ndarray:
    array = np.asarray(values, dtype=np.float64)
    if array.ndim != 1 or not np.all(np.isfinite(array)):
        raise ValueError("rank input must be a finite one-dimensional array")
    order = np.argsort(array, kind="mergesort")
    ranks = np.empty(len(array), dtype=np.float64)
    start = 0
    while start < len(array):
        end = start + 1
        while end < len(array) and array[order[end]] == array[order[start]]:
            end += 1
        ranks[order[start:end]] = (start + 1 + end) / 2.0
        start = end
    return ranks


def spearman_tie_aware(left: Sequence[float], right: Sequence[float]) -> float:
    if len(left) != len(right) or not left:
        raise ValueError("Spearman inputs must have equal nonzero length")
    x = average_ranks(left)
    y = average_ranks(right)
    x -= x.mean()
    y -= y.mean()
    denominator = float(np.linalg.norm(x) * np.linalg.norm(y))
    return 0.0 if denominator == 0.0 else float(np.dot(x, y) / denominator)


def load_memberships(path: Path) -> list[Membership]:
    rows = read_tsv(
        path,
        (
            "reaction_id",
            "block_id",
            "experiment",
            "structure_id",
            "coded_ligand_slot",
            "sample_well_row",
        ),
    )
    memberships: list[Membership] = []
    for row in rows:
        try:
            slot = int(row["coded_ligand_slot"])
            well_row = int(row["sample_well_row"])
        except ValueError as error:
            raise ValueError(f"invalid coded ligand slot or well row for {row['reaction_id']}") from error
        if slot <= 0 or well_row <= 0:
            raise ValueError(f"nonpositive coded ligand slot or well row for {row['reaction_id']}")
        memberships.append(
            Membership(
                reaction_id=row["reaction_id"],
                block_id=row["block_id"],
                experiment=row["experiment"],
                structure_id=row["structure_id"],
                coded_ligand_slot=slot,
                sample_well_row=well_row,
            )
        )
    if len({row.reaction_id for row in memberships}) != len(memberships):
        raise ValueError("membership reaction IDs are not unique")
    return memberships


def load_exact_targets(path: Path, memberships: Sequence[Membership]) -> dict[str, float]:
    rows = read_tsv(path, ("reaction_id", "yield_percentage"))
    targets: dict[str, float] = {}
    for row in rows:
        reaction_id = row["reaction_id"]
        if reaction_id in targets:
            raise ValueError(f"duplicate target row for {reaction_id}")
        try:
            value = float(row["yield_percentage"])
        except ValueError as error:
            raise ValueError(f"invalid target for {reaction_id}") from error
        if not math.isfinite(value):
            raise ValueError(f"nonfinite target for {reaction_id}")
        targets[reaction_id] = value
    required = {row.reaction_id for row in memberships}
    observed = set(targets)
    if required != observed:
        raise ValueError(
            f"target coverage is not exact: missing={len(required - observed)}, extra={len(observed - required)}"
        )
    return targets


def aggregate_primary_blocks(
    memberships: Sequence[Membership],
    targets: Mapping[str, float],
    primary_blocks: Mapping[str, str],
) -> tuple[list[AggregatedBlock], int]:
    grouped: dict[tuple[str, str], list[tuple[float, int, int, str]]] = collections.defaultdict(list)
    for row in memberships:
        if row.block_id in primary_blocks:
            if primary_blocks[row.block_id] != row.experiment:
                raise ValueError(f"block/experiment mismatch for {row.block_id}")
            grouped[(row.block_id, row.structure_id)].append(
                (
                    targets[row.reaction_id],
                    row.coded_ligand_slot,
                    row.sample_well_row,
                    row.experiment,
                )
            )
    by_block: dict[str, list[tuple[str, float, int, int, int]]] = collections.defaultdict(list)
    duplicate_groups = 0
    for (block_id, structure_id), values in grouped.items():
        experiments = {value[3] for value in values}
        slots = {value[1] for value in values}
        well_rows = {value[2] for value in values}
        if len(experiments) != 1 or len(slots) != 1 or len(well_rows) != 1:
            raise ValueError(f"canonical alias aggregation changes experiment or placement in {block_id}")
        if len(values) > 1:
            duplicate_groups += 1
        by_block[block_id].append(
            (
                structure_id,
                math.fsum(value[0] for value in values) / len(values),
                values[0][1],
                values[0][2],
                len(values),
            )
        )
    if set(by_block) != set(primary_blocks):
        raise ValueError("primary blocks and eligible membership blocks do not match")
    blocks: list[AggregatedBlock] = []
    for block_id in sorted(by_block):
        values = sorted(by_block[block_id])
        blocks.append(
            AggregatedBlock(
                block_id=block_id,
                experiment=primary_blocks[block_id],
                structure_ids=tuple(value[0] for value in values),
                yields=tuple(value[1] for value in values),
                coded_ligand_slots=tuple(float(value[2]) for value in values),
                sample_well_rows=tuple(value[3] for value in values),
                source_rows=sum(value[4] for value in values),
            )
        )
    return blocks, duplicate_groups


def crossfit_slot_rank_percentiles(
    loaded_memberships: Sequence[Membership],
    targets: Mapping[str, float],
    loaded_blocks: Mapping[str, str],
    held_out_experiments: Iterable[str],
    *,
    expected_rows_per_block: int | None = 47,
) -> dict[str, dict[int, float]]:
    """Build leave-one-experiment-out flexible slot controls.

    Targets are ranked only within loaded blocks.  For held-out experiment
    ``e``, each slot's predictor is its mean rank-percentile in every block
    from the other source experiments.  No target from ``e`` enters its score.
    """
    by_block: dict[str, list[Membership]] = collections.defaultdict(list)
    for membership in loaded_memberships:
        if membership.block_id not in loaded_blocks:
            raise ValueError(f"loaded membership references nonloaded block {membership.block_id}")
        if loaded_blocks[membership.block_id] != membership.experiment:
            raise ValueError(f"loaded block/experiment mismatch for {membership.block_id}")
        by_block[membership.block_id].append(membership)
    if set(by_block) != set(loaded_blocks):
        raise ValueError("loaded blocks and loaded membership blocks do not match")

    source_slot_values: dict[str, dict[int, list[float]]] = collections.defaultdict(
        lambda: collections.defaultdict(list)
    )
    all_slots: set[int] = set()
    for block_id, rows in by_block.items():
        if expected_rows_per_block is not None and len(rows) != expected_rows_per_block:
            raise ValueError(f"loaded block {block_id} does not have {expected_rows_per_block} rows")
        slots = [row.coded_ligand_slot for row in rows]
        if len(set(slots)) != len(slots):
            raise ValueError(f"loaded block {block_id} repeats a coded ligand slot")
        all_slots.update(slots)
        ranks = average_ranks([targets[row.reaction_id] for row in rows])
        denominator = max(len(rows) - 1, 1)
        percentiles = (ranks - 1.0) / denominator
        experiment = loaded_blocks[block_id]
        for row, percentile in zip(rows, percentiles):
            source_slot_values[experiment][row.coded_ligand_slot].append(float(percentile))

    result: dict[str, dict[int, float]] = {}
    source_experiments = set(source_slot_values)
    for held_out in sorted(set(held_out_experiments)):
        training_experiments = source_experiments - {held_out}
        slot_scores: dict[int, float] = {}
        for slot in sorted(all_slots):
            training_values = [
                value
                for experiment in training_experiments
                for value in source_slot_values[experiment].get(slot, ())
            ]
            if not training_values:
                raise ValueError(f"slot {slot} has no cross-fit training coverage for {held_out}")
            slot_scores[slot] = math.fsum(training_values) / len(training_values)
        result[held_out] = slot_scores
    return result


def _rank_columns(values: np.ndarray) -> np.ndarray:
    return np.column_stack([average_ranks(values[:, column]) for column in range(values.shape[1])])


def load_prepared(
    frozen_dir: Path,
    targets_path: Path,
    *,
    enforce_frozen_shape: bool = True,
) -> PreparedEvaluation:
    if enforce_frozen_shape:
        verify_freeze_manifest(frozen_dir)
    descriptors = read_tsv(
        frozen_dir / "ligand_descriptors.tsv",
        ("structure_id", *DESCRIPTOR_PREDICTORS),
    )
    descriptors.sort(key=lambda row: row["structure_id"])
    structure_ids = tuple(row["structure_id"] for row in descriptors)
    if len(set(structure_ids)) != len(structure_ids):
        raise ValueError("descriptor structure IDs are not unique")
    descriptor_values = np.asarray(
        [[float(row[predictor]) for predictor in DESCRIPTOR_PREDICTORS] for row in descriptors],
        dtype=np.float64,
    )
    if not np.all(np.isfinite(descriptor_values)):
        raise ValueError("descriptor table contains a nonfinite value")
    structure_index = {structure_id: index for index, structure_id in enumerate(structure_ids)}

    eligible_memberships = load_memberships(frozen_dir / "eligible_memberships.tsv")
    loaded_memberships = load_memberships(frozen_dir / "loaded_memberships.tsv")
    if {row.structure_id for row in eligible_memberships} != set(structure_ids):
        raise ValueError("membership and descriptor structure sets differ")
    targets = load_exact_targets(targets_path, loaded_memberships)

    block_rows = read_tsv(
        frozen_dir / "blocks.tsv",
        ("block_id", "experiment", "ligand_rows", "primary_included"),
    )
    loaded_blocks = {
        row["block_id"]: row["experiment"] for row in block_rows if int(row["ligand_rows"]) > 0
    }
    primary_blocks = {
        row["block_id"]: row["experiment"] for row in block_rows if row["primary_included"] == "1"
    }
    blocks, duplicate_groups = aggregate_primary_blocks(
        eligible_memberships, targets, primary_blocks
    )
    by_experiment: dict[str, list[AggregatedBlock]] = collections.defaultdict(list)
    for block in blocks:
        by_experiment[block.experiment].append(block)
    crossfit_scores = crossfit_slot_rank_percentiles(
        loaded_memberships,
        targets,
        loaded_blocks,
        by_experiment,
        expected_rows_per_block=47 if enforce_frozen_shape else None,
    )

    experiments: list[ExperimentData] = []
    for experiment, experiment_blocks in sorted(by_experiment.items()):
        panel = experiment_blocks[0].structure_ids
        slots = experiment_blocks[0].coded_ligand_slots
        if any(block.structure_ids != panel for block in experiment_blocks[1:]):
            raise ValueError(f"canonical panel changes across blocks in {experiment}")
        if any(block.coded_ligand_slots != slots for block in experiment_blocks[1:]):
            raise ValueError(f"coded ligand slot changes across blocks in {experiment}")
        outcomes = np.asarray([block.yields for block in experiment_blocks], dtype=np.float64)
        outcome_ranks = np.vstack([average_ranks(row) for row in outcomes])
        outcome_rank_centers = outcome_ranks - outcome_ranks.mean(axis=1, keepdims=True)
        experiments.append(
            ExperimentData(
                name=experiment,
                block_ids=tuple(block.block_id for block in experiment_blocks),
                panel_indices=np.asarray([structure_index[value] for value in panel], dtype=np.int64),
                outcomes=outcomes,
                outcome_rank_centers=outcome_rank_centers,
                outcome_rank_norms=np.linalg.norm(outcome_rank_centers, axis=1),
                coded_ligand_slots=np.asarray(slots, dtype=np.float64),
                crossfit_slot_rank_percentiles=np.asarray(
                    [crossfit_scores[experiment][int(slot)] for slot in slots],
                    dtype=np.float64,
                ),
            )
        )

    strata_rows = read_tsv(frozen_dir / "permutation_strata.tsv", ("structure_id", "stratum_id"))
    if {row["structure_id"] for row in strata_rows} != set(structure_ids):
        raise ValueError("permutation strata and descriptor structure sets differ")
    grouped_strata: dict[str, list[int]] = collections.defaultdict(list)
    for row in strata_rows:
        grouped_strata[row["stratum_id"]].append(structure_index[row["structure_id"]])
    strata = tuple(
        tuple(sorted(indices)) for _, indices in sorted(grouped_strata.items())
    )
    all_experiment_names = sorted({membership.experiment for membership in eligible_memberships})
    incidence_counts = collections.Counter(
        (membership.structure_id, membership.experiment) for membership in eligible_memberships
    )
    experiment_membership = {
        index: tuple(
            incidence_counts[(structure_ids[index], experiment)]
            for experiment in all_experiment_names
        )
        for index in range(len(structure_ids))
    }
    for stratum in strata:
        signatures = {experiment_membership[index] for index in stratum}
        if len(signatures) != 1:
            raise ValueError("permutation stratum crosses observed experiment incidence")

    rows_by_structure: dict[str, set[int]] = collections.defaultdict(set)
    for membership in eligible_memberships:
        rows_by_structure[membership.structure_id].add(membership.sample_well_row)
    mover_structure_indices = tuple(
        structure_index[structure_id]
        for structure_id in structure_ids
        if len(rows_by_structure[structure_id]) >= 2
    )

    prepared = PreparedEvaluation(
        structure_ids=structure_ids,
        descriptor_values=descriptor_values,
        experiments=tuple(experiments),
        strata=strata,
        loaded_membership_count=len(loaded_memberships),
        eligible_membership_count=len(eligible_memberships),
        loaded_block_count=len(loaded_blocks),
        primary_block_count=len(blocks),
        duplicate_block_structure_groups=duplicate_groups,
        mover_structure_indices=mover_structure_indices,
    )
    if enforce_frozen_shape:
        if len(structure_ids) != EXPECTED_STRUCTURES:
            raise ValueError("frozen structure count changed")
        if len(loaded_memberships) != EXPECTED_LOADED_MEMBERSHIPS:
            raise ValueError("frozen loaded membership count changed")
        if len(eligible_memberships) != EXPECTED_ELIGIBLE_MEMBERSHIPS:
            raise ValueError("frozen eligible membership count changed")
        if len(loaded_blocks) != EXPECTED_LOADED_BLOCKS:
            raise ValueError("frozen loaded block count changed")
        if len(blocks) != EXPECTED_PRIMARY_BLOCKS:
            raise ValueError("frozen primary block count changed")
        if len(experiments) != EXPECTED_PRIMARY_EXPERIMENTS:
            raise ValueError("frozen primary experiment count changed")
        if any(len(experiment.panel_indices) < 9 for experiment in experiments):
            raise ValueError("a frozen primary panel has fewer than nine structures")
        if len(mover_structure_indices) != 39:
            raise ValueError("frozen slot-row mover cohort changed")
    return prepared


def enumerate_joint_row_mappings(prepared: PreparedEvaluation) -> np.ndarray:
    identity = np.arange(len(prepared.structure_ids), dtype=np.int64)
    choices = [tuple(itertools.permutations(stratum)) for stratum in prepared.strata]
    mappings: list[np.ndarray] = []
    for combination in itertools.product(*choices):
        mapping = identity.copy()
        for destinations, sources in zip(prepared.strata, combination):
            mapping[np.asarray(destinations, dtype=np.int64)] = sources
        mappings.append(mapping)
    result = np.vstack(mappings)
    if not np.array_equal(result[0], identity):
        raise ValueError("identity is not the first exact permutation")
    return result


def _experiment_correlations(
    prepared: PreparedEvaluation, mapping: np.ndarray
) -> tuple[np.ndarray, np.ndarray]:
    absolute = np.empty((len(prepared.experiments), len(ALL_PREDICTORS)), dtype=np.float64)
    signed = np.empty_like(absolute)
    mapped_descriptors = prepared.descriptor_values[mapping]
    for experiment_index, experiment in enumerate(prepared.experiments):
        descriptor_ranks = _rank_columns(mapped_descriptors[experiment.panel_indices, :])
        fixed_position_ranks = np.column_stack(
            (
                average_ranks(experiment.coded_ligand_slots),
                average_ranks(experiment.crossfit_slot_rank_percentiles),
            )
        )
        predictor_ranks = np.column_stack((descriptor_ranks, fixed_position_ranks))
        predictor_centers = predictor_ranks - predictor_ranks.mean(axis=0, keepdims=True)
        predictor_norms = np.linalg.norm(predictor_centers, axis=0)
        denominators = np.outer(experiment.outcome_rank_norms, predictor_norms)
        numerators = experiment.outcome_rank_centers @ predictor_centers
        correlations = np.divide(
            numerators,
            denominators,
            out=np.zeros_like(numerators),
            where=denominators != 0.0,
        )
        absolute[experiment_index, :] = np.mean(np.abs(correlations), axis=0)
        signed[experiment_index, :] = np.mean(correlations, axis=0)
    return absolute, signed


def _observed_diagnostics(prepared: PreparedEvaluation) -> dict[str, dict[str, object]]:
    diagnostics = {
        predictor: {
            "positive_blocks": 0,
            "negative_blocks": 0,
            "zero_blocks": 0,
            "per_experiment": {},
        }
        for predictor in ALL_PREDICTORS
    }
    identity = np.arange(len(prepared.structure_ids), dtype=np.int64)
    mapped_descriptors = prepared.descriptor_values[identity]
    for experiment in prepared.experiments:
        descriptor_ranks = _rank_columns(mapped_descriptors[experiment.panel_indices, :])
        fixed_position_ranks = np.column_stack(
            (
                average_ranks(experiment.coded_ligand_slots),
                average_ranks(experiment.crossfit_slot_rank_percentiles),
            )
        )
        predictor_ranks = np.column_stack((descriptor_ranks, fixed_position_ranks))
        predictor_centers = predictor_ranks - predictor_ranks.mean(axis=0, keepdims=True)
        predictor_norms = np.linalg.norm(predictor_centers, axis=0)
        denominators = np.outer(experiment.outcome_rank_norms, predictor_norms)
        correlations = np.divide(
            experiment.outcome_rank_centers @ predictor_centers,
            denominators,
            out=np.zeros((len(experiment.block_ids), len(ALL_PREDICTORS))),
            where=denominators != 0.0,
        )
        for predictor_index, predictor in enumerate(ALL_PREDICTORS):
            values = correlations[:, predictor_index]
            positive = int(np.count_nonzero(values > 0.0))
            negative = int(np.count_nonzero(values < 0.0))
            zero = int(np.count_nonzero(values == 0.0))
            diagnostics[predictor]["positive_blocks"] += positive
            diagnostics[predictor]["negative_blocks"] += negative
            diagnostics[predictor]["zero_blocks"] += zero
            diagnostics[predictor]["per_experiment"][experiment.name] = {
                "blocks": len(values),
                "T": float(np.mean(np.abs(values))),
                "signed_mean_rho": float(np.mean(values)),
                "positive_blocks": positive,
                "negative_blocks": negative,
                "zero_blocks": zero,
            }
    for predictor in ALL_PREDICTORS:
        positive = diagnostics[predictor]["positive_blocks"]
        negative = diagnostics[predictor]["negative_blocks"]
        denominator = positive + negative
        diagnostics[predictor]["positive_fraction_nonzero"] = (
            None if denominator == 0 else positive / denominator
        )
    return diagnostics


def _exact_p(values: np.ndarray, observed: float) -> dict[str, float | int]:
    numerator = int(np.count_nonzero(values >= observed))
    denominator = int(len(values))
    return {"numerator": numerator, "denominator": denominator, "value": numerator / denominator}


def _fixed_percentile_interval(values: np.ndarray, lower_index: int, upper_index: int) -> dict[str, float]:
    ordered = np.sort(values)
    if lower_index < 0 or upper_index >= len(ordered) or lower_index > upper_index:
        raise ValueError("bootstrap percentile indices are outside the replicate array")
    return {"lower": float(ordered[lower_index]), "upper": float(ordered[upper_index])}


def movers_only_sensitivity(prepared: PreparedEvaluation) -> dict[str, object]:
    movers = set(prepared.mover_structure_indices)
    experiment_absolute: list[float] = []
    experiment_signed: list[float] = []
    retained_blocks = 0
    per_experiment: dict[str, object] = {}
    candidate_index = DESCRIPTOR_PREDICTORS.index(CANDIDATE)
    for experiment in prepared.experiments:
        retained = np.asarray(
            [index for index, structure in enumerate(experiment.panel_indices) if structure in movers],
            dtype=np.int64,
        )
        if len(retained) < 2:
            continue
        predictor = prepared.descriptor_values[
            experiment.panel_indices[retained], candidate_index
        ]
        correlations = np.asarray(
            [
                spearman_tie_aware(predictor.tolist(), outcome[retained].tolist())
                for outcome in experiment.outcomes
            ],
            dtype=np.float64,
        )
        absolute = float(np.mean(np.abs(correlations)))
        signed = float(np.mean(correlations))
        experiment_absolute.append(absolute)
        experiment_signed.append(signed)
        retained_blocks += len(correlations)
        per_experiment[experiment.name] = {
            "structures": len(retained),
            "blocks": len(correlations),
            "T": absolute,
            "signed_mean_rho": signed,
        }
    return {
        "global_mover_structures": len(movers),
        "retained_experiments": len(experiment_absolute),
        "retained_blocks": retained_blocks,
        "T": float(np.mean(experiment_absolute)) if experiment_absolute else None,
        "signed_experiment_balanced_mean_rho": (
            float(np.mean(experiment_signed)) if experiment_signed else None
        ),
        "per_experiment": per_experiment,
        "confirmatory_rescue_permitted": False,
        "causal_interpretation_permitted": False,
    }


def run_evaluation(
    prepared: PreparedEvaluation,
    *,
    expected_permutations: int | None = EXPECTED_PERMUTATIONS,
    bootstrap_replicates: int = BOOTSTRAP_REPLICATES,
    bootstrap_seed: int = BOOTSTRAP_SEED,
    bootstrap_lower_index: int = BOOTSTRAP_LOWER_INDEX,
    bootstrap_upper_index: int = BOOTSTRAP_UPPER_INDEX,
) -> dict[str, object]:
    mappings = enumerate_joint_row_mappings(prepared)
    if expected_permutations is not None and len(mappings) != expected_permutations:
        raise ValueError(f"expected {expected_permutations} exact permutations, found {len(mappings)}")

    permutation_scores = np.empty((len(mappings), len(ALL_PREDICTORS)), dtype=np.float64)
    observed_absolute, observed_signed = _experiment_correlations(prepared, mappings[0])
    for permutation_index, mapping in enumerate(mappings):
        absolute, _ = _experiment_correlations(prepared, mapping)
        permutation_scores[permutation_index, :] = absolute.mean(axis=0)
    observed_scores = permutation_scores[0, :]

    predictor_index = {name: index for index, name in enumerate(ALL_PREDICTORS)}
    candidate_index = predictor_index[CANDIDATE]
    baseline_indices = np.asarray([predictor_index[name] for name in BASELINES], dtype=np.int64)
    observed_baseline_values = observed_scores[baseline_indices]
    champion_offset = int(np.argmax(observed_baseline_values))
    champion_name = BASELINES[champion_offset]
    champion_score = float(observed_baseline_values[champion_offset])
    observed_delta = float(observed_scores[candidate_index] - champion_score)
    permutation_delta = (
        permutation_scores[:, candidate_index]
        - np.max(permutation_scores[:, baseline_indices], axis=1)
    )
    candidate_null_mean = float(np.mean(permutation_scores[:, candidate_index]))
    candidate_excess = float(observed_scores[candidate_index] - candidate_null_mean)

    diagnostics = _observed_diagnostics(prepared)
    predictor_report: dict[str, object] = {}
    for name in ALL_PREDICTORS:
        index = predictor_index[name]
        report = {
            "T": float(observed_scores[index]),
            "signed_experiment_balanced_mean_rho": float(observed_signed[:, index].mean()),
            **diagnostics[name],
        }
        if name not in FIXED_POSITION_CONTROLS:
            report["exact_permutation_p"] = _exact_p(permutation_scores[:, index], observed_scores[index])
            report["exact_null_mean_T"] = float(np.mean(permutation_scores[:, index]))
        predictor_report[name] = report

    rng = np.random.default_rng(bootstrap_seed)
    sampled_experiments = rng.integers(
        0,
        len(prepared.experiments),
        size=(bootstrap_replicates, len(prepared.experiments)),
    )
    bootstrap_scores = observed_absolute[sampled_experiments, :].mean(axis=1)
    bootstrap_delta = (
        bootstrap_scores[:, candidate_index]
        - np.max(bootstrap_scores[:, baseline_indices], axis=1)
    )
    bootstrap_candidate = bootstrap_scores[:, candidate_index]
    candidate_ci = _fixed_percentile_interval(
        bootstrap_candidate, bootstrap_lower_index, bootstrap_upper_index
    )
    delta_ci = _fixed_percentile_interval(bootstrap_delta, bootstrap_lower_index, bootstrap_upper_index)

    slot_correlations: dict[str, dict[str, float]] = {}
    for experiment in prepared.experiments:
        candidate_values = prepared.descriptor_values[experiment.panel_indices, candidate_index]
        slot_correlations[experiment.name] = {
            POSITION_CONTROL: spearman_tie_aware(
                candidate_values.tolist(), experiment.coded_ligand_slots.tolist()
            ),
            CROSSFIT_POSITION_CONTROL: spearman_tie_aware(
                candidate_values.tolist(), experiment.crossfit_slot_rank_percentiles.tolist()
            ),
        }

    candidate_p = _exact_p(
        permutation_scores[:, candidate_index], float(observed_scores[candidate_index])
    )
    delta_p = _exact_p(permutation_delta, observed_delta)
    gates = {
        "candidate_exact_p_le_0_001": candidate_p["value"] <= 0.001,
        "candidate_null_centered_excess_ge_0_05": candidate_excess >= 0.05,
        "baseline_margin_ge_0_05": observed_delta >= 0.05,
        "baseline_competition_exact_p_le_0_001": delta_p["value"] <= 0.001,
        "cluster_bootstrap_delta_lower_gt_zero": delta_ci["lower"] > 0.0,
        "complete_target_and_shape_qa": True,
    }
    return {
        "analysis": {
            "candidate": CANDIDATE,
            "secondary_non_rescue": SECONDARY,
            "baselines": list(BASELINES),
            "position_controls": {
                "names": list(FIXED_POSITION_CONTROLS),
                "permutation_treatment": "fixed to observed structure/placement; not moved with descriptor rows",
                "crossfit_method": (
                    "For each held-out experiment, average within-block rank-percentile by coded slot "
                    "over all loaded blocks in the other 16 source experiments."
                ),
                "scope": "stable slot patterns only; experiment-specific well bias remains unidentified",
            },
            "null_assumption": (
                "Exact only under exchangeability of structure/outcome profiles within identical "
                "experiment-incidence strata; molecular descriptors were not randomized treatments."
            ),
        },
        "qa": {
            "structures": len(prepared.structure_ids),
            "loaded_memberships_with_exact_finite_target": prepared.loaded_membership_count,
            "eligible_memberships_with_exact_finite_target": prepared.eligible_membership_count,
            "loaded_blocks_used_to_train_position_control": prepared.loaded_block_count,
            "primary_blocks": prepared.primary_block_count,
            "primary_experiments": len(prepared.experiments),
            "canonical_duplicate_block_structure_groups_averaged": prepared.duplicate_block_structure_groups,
            "exact_joint_descriptor_row_permutations": len(mappings),
            "movable_structures": sum(len(stratum) for stratum in prepared.strata if len(stratum) > 1),
            "slot_row_mover_structures": len(prepared.mover_structure_indices),
        },
        "predictors": predictor_report,
        "primary": {
            "T": float(observed_scores[candidate_index]),
            "exact_null_mean_T": candidate_null_mean,
            "null_centered_excess_E": candidate_excess,
            "exact_permutation_p": candidate_p,
            "cluster_bootstrap_95_percent_T": candidate_ci,
        },
        "baseline_competition": {
            "observed_champion": champion_name,
            "observed_champion_T": champion_score,
            "delta": observed_delta,
            "exact_permutation_p": delta_p,
            "cluster_bootstrap_95_percent_delta": delta_ci,
            "champion_reselected_per_permutation": True,
            "champion_reselected_per_bootstrap_replicate": True,
        },
        "position_diagnostics": {
            "s_tau_p_rh_tau5_vs_position_controls_spearman_by_experiment": slot_correlations,
            "movers_only_non_rescue_sensitivity": movers_only_sensitivity(prepared),
            "causal_interpretation_permitted": False,
            "unidentified_layout_risk": "experiment-specific and other unstable well bias",
        },
        "bootstrap": {
            "clusters": len(prepared.experiments),
            "replicates": bootstrap_replicates,
            "seed_hex": hex(bootstrap_seed),
            "percentile_zero_based_indices": [bootstrap_lower_index, bootstrap_upper_index],
        },
        "decision": {"gates": gates, "pass": all(gates.values())},
    }


def evaluate_paths(frozen_dir: Path, targets_path: Path, report_path: Path) -> dict[str, object]:
    prepared = load_prepared(frozen_dir, targets_path, enforce_frozen_shape=True)
    report = run_evaluation(prepared)
    report_path.parent.mkdir(parents=True, exist_ok=True)
    with report_path.open("w", encoding="utf-8", newline="\n") as handle:
        json.dump(report, handle, indent=2, sort_keys=True, ensure_ascii=False)
        handle.write("\n")
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    subparsers = parser.add_subparsers(dest="command", required=True)
    extract_parser = subparsers.add_parser("extract-targets")
    extract_parser.add_argument("parquet", type=Path)
    extract_parser.add_argument("memberships", type=Path)
    extract_parser.add_argument("output", type=Path)
    evaluate_parser = subparsers.add_parser("evaluate")
    evaluate_parser.add_argument("frozen_dir", type=Path)
    evaluate_parser.add_argument("targets", type=Path)
    evaluate_parser.add_argument("report", type=Path)
    arguments = parser.parse_args()
    if arguments.command == "extract-targets":
        extract_targets(arguments.parquet, arguments.memberships, arguments.output)
    else:
        report = evaluate_paths(arguments.frozen_dir, arguments.targets, arguments.report)
        print(json.dumps(report["decision"], indent=2, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
