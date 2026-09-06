use jsonschema::{Draft, JSONSchema};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use valhaim_thermodynamic_kernel_runner::{
    derive_contact, derive_controls, derive_kernel_exact, sha256_hex, validate_artifact_semantics,
    Provenance, Toolchain, FREEZE_COMMIT, FREEZE_MANIFEST_SHA256, PROTOCOL_SHA256,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RunnerSeal {
    schema_version: u8,
    freeze_commit: String,
    protocol_path: String,
    protocol_sha256: String,
    freeze_manifest_path: String,
    freeze_manifest_sha256: String,
    schema_path: String,
    schema_sha256: String,
    relevant_source_paths: Vec<String>,
    relevant_source_sha256: String,
    executable_sha256: String,
    build_toolchain: SealedToolchain,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SealedToolchain {
    rustc: String,
    cargo: String,
    target: String,
    cargo_lock_sha256: String,
}

const SCHEMA_PATH: &str = "research/physics/valhaim-thermodynamic-kernel/SCHEMA.json";
const PROTOCOL_PATH: &str = "research/physics/valhaim-thermodynamic-kernel/PROTOCOL.md";
const FREEZE_MANIFEST_PATH: &str =
    "research/physics/valhaim-thermodynamic-kernel/artifacts/freeze-manifest.json";
const REQUIRED_SOURCE_PATHS: &[&str] = &[
    "research/physics/valhaim-thermodynamic-kernel/SCHEMA.json",
    "research/physics/valhaim-thermodynamic-kernel/runner/Cargo.lock",
    "research/physics/valhaim-thermodynamic-kernel/runner/Cargo.toml",
    "research/physics/valhaim-thermodynamic-kernel/runner/build.rs",
    "research/physics/valhaim-thermodynamic-kernel/runner/source-set.txt",
    "research/physics/valhaim-thermodynamic-kernel/runner/src/lib.rs",
    "research/physics/valhaim-thermodynamic-kernel/runner/src/main.rs",
    "research/physics/valhaim-thermodynamic-kernel/runner/tests/exact.rs",
];
const EMBEDDED_SOURCE_SHA256: &str = env!("VAL_KERNEL_SOURCE_SHA256");
const EMBEDDED_LOCK_SHA256: &str = env!("VAL_KERNEL_LOCK_SHA256");
const EMBEDDED_BUILD_RUSTC: &str = env!("VAL_KERNEL_BUILD_RUSTC");
const EMBEDDED_BUILD_CARGO: &str = env!("VAL_KERNEL_BUILD_CARGO");
const EMBEDDED_BUILD_TARGET: &str = env!("VAL_KERNEL_BUILD_TARGET");

struct Arguments {
    repo_root: PathBuf,
    output_dir: PathBuf,
    runner_seal: PathBuf,
    argv: Vec<String>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let arguments = parse_arguments()?;
    let provenance = preflight(&arguments)?;
    let controls = derive_controls(provenance.clone()).map_err(|error| error.to_string())?;
    let exact =
        derive_kernel_exact(provenance.clone(), &controls).map_err(|error| error.to_string())?;
    let contact = derive_contact(provenance, &exact).map_err(|error| error.to_string())?;
    validate_artifact_semantics(&exact, &controls, &contact).map_err(|error| error.to_string())?;

    let artifacts = [
        (
            "kernel-exact.json",
            serde_json::to_value(&exact).map_err(|error| error.to_string())?,
        ),
        (
            "controls.json",
            serde_json::to_value(&controls).map_err(|error| error.to_string())?,
        ),
        (
            "contact.json",
            serde_json::to_value(&contact).map_err(|error| error.to_string())?,
        ),
    ];
    validate_schema(&arguments.repo_root, &artifacts)?;
    let outputs = artifacts
        .iter()
        .map(|(name, value)| {
            serde_json::to_vec_pretty(value)
                .map(|bytes| (*name, bytes))
                .map_err(|error| error.to_string())
        })
        .collect::<Result<Vec<_>, _>>()?;
    write_new_artifacts(&arguments.output_dir, &outputs)
}

fn parse_arguments() -> Result<Arguments, String> {
    let raw = std::env::args_os().collect::<Vec<_>>();
    let argv = raw
        .iter()
        .map(|value| value.to_string_lossy().into_owned())
        .collect::<Vec<_>>();
    let mut values = raw.into_iter().skip(1);
    let mut repo_root = None;
    let mut output_dir = None;
    let mut runner_seal = None;
    while let Some(flag) = values.next() {
        let value = values.next().ok_or_else(usage)?;
        match flag.to_string_lossy().as_ref() {
            "--repo-root" => repo_root = Some(PathBuf::from(value)),
            "--output-dir" => output_dir = Some(PathBuf::from(value)),
            "--runner-seal" => runner_seal = Some(PathBuf::from(value)),
            _ => return Err(usage()),
        }
    }
    Ok(Arguments {
        repo_root: repo_root.ok_or_else(usage)?,
        output_dir: output_dir.ok_or_else(usage)?,
        runner_seal: runner_seal.ok_or_else(usage)?,
        argv,
    })
}

fn usage() -> String {
    "usage: valhaim-thermodynamic-kernel-runner --repo-root <path> --output-dir <path> --runner-seal <path>".to_owned()
}

fn preflight(arguments: &Arguments) -> Result<Provenance, String> {
    let seal_path = absolute_from(&arguments.repo_root, &arguments.runner_seal);
    let seal_bytes = fs::read(&seal_path)
        .map_err(|error| format!("cannot read {}: {error}", seal_path.display()))?;
    let seal: RunnerSeal = serde_json::from_slice(&seal_bytes)
        .map_err(|error| format!("invalid runner seal: {error}"))?;
    if seal.schema_version != 1 || seal.freeze_commit != FREEZE_COMMIT {
        return Err("runner seal has the wrong version or freeze commit".to_owned());
    }
    if seal.protocol_path != PROTOCOL_PATH
        || seal.freeze_manifest_path != FREEZE_MANIFEST_PATH
        || seal.schema_path != SCHEMA_PATH
    {
        return Err(
            "runner seal uses a noncanonical protocol, manifest, or schema path".to_owned(),
        );
    }
    require_source_set(&seal.relevant_source_paths)?;
    let runner_seal_relative = relative_to(&arguments.repo_root, &seal_path)?;
    let runner_seal_commit = git_output(
        &arguments.repo_root,
        &["log", "-1", "--format=%H", "--", &runner_seal_relative],
    )?;
    if runner_seal_commit.is_empty() {
        return Err("runner seal has no committed history".to_owned());
    }
    git_success(
        &arguments.repo_root,
        &[
            "merge-base",
            "--is-ancestor",
            FREEZE_COMMIT,
            &runner_seal_commit,
        ],
    )?;
    let head = git_output(&arguments.repo_root, &["rev-parse", "HEAD"])?;
    git_success(
        &arguments.repo_root,
        &["merge-base", "--is-ancestor", &runner_seal_commit, &head],
    )?;
    require_hash(
        &arguments.repo_root,
        &seal.protocol_path,
        &seal.protocol_sha256,
        PROTOCOL_SHA256,
    )?;
    require_commit_hash(
        &arguments.repo_root,
        FREEZE_COMMIT,
        &seal.protocol_path,
        PROTOCOL_SHA256,
    )?;
    require_hash(
        &arguments.repo_root,
        &seal.freeze_manifest_path,
        &seal.freeze_manifest_sha256,
        FREEZE_MANIFEST_SHA256,
    )?;
    require_commit_hash(
        &arguments.repo_root,
        FREEZE_COMMIT,
        &seal.freeze_manifest_path,
        FREEZE_MANIFEST_SHA256,
    )?;
    let schema_sha256 = require_hash(
        &arguments.repo_root,
        &seal.schema_path,
        &seal.schema_sha256,
        &seal.schema_sha256,
    )?;
    let relevant_source_sha256 =
        combined_source_hash(&arguments.repo_root, &seal.relevant_source_paths)?;
    if relevant_source_sha256 != seal.relevant_source_sha256 {
        return Err(format!(
            "relevant source hash mismatch: expected {}, observed {}",
            seal.relevant_source_sha256, relevant_source_sha256
        ));
    }
    let committed_source_sha256 = combined_source_hash_at_commit(
        &arguments.repo_root,
        &runner_seal_commit,
        &seal.relevant_source_paths,
    )?;
    if committed_source_sha256 != seal.relevant_source_sha256
        || EMBEDDED_SOURCE_SHA256 != seal.relevant_source_sha256
    {
        return Err(format!(
            "source receipt mismatch: seal {}, commit {}, binary {}",
            seal.relevant_source_sha256, committed_source_sha256, EMBEDDED_SOURCE_SHA256
        ));
    }
    let executable = std::env::current_exe()
        .map_err(|error| format!("cannot identify running executable: {error}"))?;
    let executable_sha256 = file_sha256(&executable)?;
    if executable_sha256 != seal.executable_sha256 {
        return Err(format!(
            "running executable hash mismatch: expected {}, observed {}",
            seal.executable_sha256, executable_sha256
        ));
    }
    require_relevant_clean(
        &arguments.repo_root,
        &seal.relevant_source_paths,
        relative_to(&arguments.repo_root, &seal_path)?,
    )?;
    let cargo_lock = "research/physics/valhaim-thermodynamic-kernel/runner/Cargo.lock";
    let cargo_lock_sha256 = file_sha256(&arguments.repo_root.join(cargo_lock))?;
    if cargo_lock_sha256 != EMBEDDED_LOCK_SHA256 {
        return Err("running binary embeds another Cargo.lock hash".to_owned());
    }
    if seal.build_toolchain.rustc != EMBEDDED_BUILD_RUSTC
        || seal.build_toolchain.cargo != EMBEDDED_BUILD_CARGO
        || seal.build_toolchain.target != EMBEDDED_BUILD_TARGET
        || seal.build_toolchain.cargo_lock_sha256 != cargo_lock_sha256
    {
        return Err(
            "binary build toolchain or lockfile differs from the sealed receipt".to_owned(),
        );
    }
    Ok(Provenance {
        freeze_commit: FREEZE_COMMIT.to_owned(),
        runner_seal_commit,
        protocol_sha256: PROTOCOL_SHA256.to_owned(),
        freeze_manifest_sha256: FREEZE_MANIFEST_SHA256.to_owned(),
        schema_sha256,
        relevant_source_clean: true,
        relevant_source_paths: seal.relevant_source_paths,
        relevant_source_sha256,
        executable_sha256,
        command_argv: arguments.argv.clone(),
        toolchain: Toolchain {
            rustc: EMBEDDED_BUILD_RUSTC.to_owned(),
            cargo: EMBEDDED_BUILD_CARGO.to_owned(),
            target: EMBEDDED_BUILD_TARGET.to_owned(),
            cargo_lock_sha256,
        },
    })
}

fn require_source_set(paths: &[String]) -> Result<(), String> {
    let mut observed = paths.to_vec();
    observed.sort();
    let expected = REQUIRED_SOURCE_PATHS
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<Vec<_>>();
    if observed != expected {
        return Err(format!(
            "runner seal source set differs from required scientific source set: expected {expected:?}, observed {observed:?}"
        ));
    }
    Ok(())
}

fn validate_schema(root: &Path, artifacts: &[(&str, serde_json::Value)]) -> Result<(), String> {
    let schema_bytes =
        fs::read(root.join(SCHEMA_PATH)).map_err(|error| format!("cannot read schema: {error}"))?;
    let schema: serde_json::Value = serde_json::from_slice(&schema_bytes)
        .map_err(|error| format!("invalid schema JSON: {error}"))?;
    let validator = JSONSchema::options()
        .with_draft(Draft::Draft202012)
        .compile(&schema)
        .map_err(|error| format!("cannot compile evidence schema: {error}"))?;
    for (name, artifact) in artifacts {
        if let Err(errors) = validator.validate(artifact) {
            let details = errors
                .take(20)
                .map(|error| error.to_string())
                .collect::<Vec<_>>()
                .join("; ");
            return Err(format!(
                "{name} failed evidence schema validation: {details}"
            ));
        }
    }
    Ok(())
}

fn require_hash(
    root: &Path,
    relative: &str,
    seal_expected: &str,
    frozen_expected: &str,
) -> Result<String, String> {
    if seal_expected != frozen_expected {
        return Err(format!(
            "sealed identity for {relative} disagrees with the frozen runner"
        ));
    }
    let observed = file_sha256(&root.join(relative))?;
    if observed != seal_expected {
        return Err(format!(
            "hash mismatch for {relative}: expected {seal_expected}, observed {observed}"
        ));
    }
    Ok(observed)
}

fn file_sha256(path: &Path) -> Result<String, String> {
    fs::read(path)
        .map(|bytes| sha256_hex(&bytes))
        .map_err(|error| format!("cannot hash {}: {error}", path.display()))
}

fn require_commit_hash(
    root: &Path,
    commit: &str,
    path: &str,
    expected: &str,
) -> Result<(), String> {
    let object = format!("{commit}:{path}");
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["show", &object])
        .output()
        .map_err(|error| format!("cannot read frozen object {object}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "cannot read frozen object {object}: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    let observed = sha256_hex(&output.stdout);
    if observed != expected {
        return Err(format!(
            "frozen object hash mismatch for {object}: expected {expected}, observed {observed}"
        ));
    }
    Ok(())
}

fn combined_source_hash(root: &Path, paths: &[String]) -> Result<String, String> {
    if paths.is_empty() {
        return Err("runner seal has no relevant source paths".to_owned());
    }
    let mut sorted = paths.to_vec();
    sorted.sort();
    sorted.dedup();
    if sorted.len() != paths.len() {
        return Err("runner seal relevant source paths are duplicated".to_owned());
    }
    let mut digest = Sha256::new();
    for path in sorted {
        let hash = file_sha256(&root.join(&path))?;
        digest.update(path.as_bytes());
        digest.update([0]);
        digest.update(hash.as_bytes());
        digest.update(b"\n");
    }
    Ok(digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn combined_source_hash_at_commit(
    root: &Path,
    commit: &str,
    paths: &[String],
) -> Result<String, String> {
    let mut sorted = paths.to_vec();
    sorted.sort();
    sorted.dedup();
    if sorted.len() != paths.len() {
        return Err("runner seal relevant source paths are duplicated".to_owned());
    }
    let mut digest = Sha256::new();
    for path in sorted {
        let object = format!("{commit}:{path}");
        let output = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["show", &object])
            .output()
            .map_err(|error| format!("cannot read committed source {object}: {error}"))?;
        if !output.status.success() {
            return Err(format!(
                "cannot read committed source {object}: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ));
        }
        let hash = sha256_hex(&output.stdout);
        digest.update(path.as_bytes());
        digest.update([0]);
        digest.update(hash.as_bytes());
        digest.update(b"\n");
    }
    Ok(digest
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

fn require_relevant_clean(root: &Path, paths: &[String], seal_path: String) -> Result<(), String> {
    let mut all = paths.to_vec();
    all.push(seal_path);
    for path in &all {
        git_success(root, &["ls-files", "--error-unmatch", "--", path])?;
    }
    let mut command = Command::new("git");
    command
        .arg("-C")
        .arg(root)
        .args(["diff", "--quiet", "HEAD", "--"]);
    command.args(&all);
    let status = command
        .status()
        .map_err(|error| format!("cannot run git diff: {error}"))?;
    if !status.success() {
        return Err("relevant source set differs from the runner-seal commit".to_owned());
    }
    Ok(())
}

fn git_output(root: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .map_err(|error| format!("cannot run git: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_owned());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn git_success(root: &Path, arguments: &[&str]) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .status()
        .map_err(|error| format!("cannot run git: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("git command failed: git {}", arguments.join(" ")))
    }
}

fn absolute_from(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_owned()
    } else {
        root.join(path)
    }
}

fn relative_to(root: &Path, path: &Path) -> Result<String, String> {
    path.strip_prefix(root)
        .map(|relative| relative.to_string_lossy().into_owned())
        .map_err(|_| format!("{} is outside repo root {}", path.display(), root.display()))
}

fn write_new_artifacts(output_dir: &Path, outputs: &[(&str, Vec<u8>)]) -> Result<(), String> {
    if !output_dir.is_dir() {
        return Err(format!(
            "output directory {} does not exist",
            output_dir.display()
        ));
    }
    for (name, _) in outputs {
        if output_dir.join(name).exists() || output_dir.join(format!(".{name}.tmp")).exists() {
            return Err(format!("refusing to replace existing output {name}"));
        }
    }
    let mut staged = Vec::new();
    for (name, bytes) in outputs {
        let temporary = output_dir.join(format!(".{name}.tmp"));
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| format!("cannot create {}: {error}", temporary.display()))?;
        file.write_all(bytes)
            .and_then(|_| file.write_all(b"\n"))
            .and_then(|_| file.sync_all())
            .map_err(|error| format!("cannot write {}: {error}", temporary.display()))?;
        staged.push((temporary, output_dir.join(name)));
    }
    for (temporary, final_path) in staged {
        fs::rename(&temporary, &final_path).map_err(|error| {
            format!(
                "cannot publish {} as {}: {error}",
                temporary.display(),
                final_path.display()
            )
        })?;
    }
    Ok(())
}
