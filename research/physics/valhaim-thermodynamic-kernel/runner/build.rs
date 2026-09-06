use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn version(program: PathBuf, argument: &str) -> String {
    let output = Command::new(&program)
        .arg(argument)
        .output()
        .unwrap_or_else(|error| panic!("run {} {argument}: {error}", program.display()));
    assert!(
        output.status.success(),
        "{} {argument} failed: {}",
        program.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("toolchain version must be UTF-8")
        .trim()
        .to_owned()
}

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest directory"));
    let repo = manifest
        .ancestors()
        .nth(4)
        .expect("runner must remain four levels below repository root");
    let source_set_path = manifest.join("source-set.txt");
    let source_set = fs::read_to_string(&source_set_path).expect("read source-set.txt");
    let paths = source_set
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let mut sorted = paths.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(paths, sorted, "source-set.txt must be sorted and unique");
    let mut combined = Sha256::new();
    for path in &paths {
        let absolute = repo.join(path);
        let bytes = fs::read(&absolute).unwrap_or_else(|error| panic!("read {path}: {error}"));
        let hash = sha256_hex(&bytes);
        combined.update(path.as_bytes());
        combined.update([0]);
        combined.update(hash.as_bytes());
        combined.update(b"\n");
        println!("cargo:rerun-if-changed={}", absolute.display());
    }
    let source_hash = combined
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let lock_path = manifest.join("Cargo.lock");
    let lock_hash = sha256_hex(&fs::read(&lock_path).expect("read Cargo.lock"));
    let rustc = version(
        PathBuf::from(env::var_os("RUSTC").expect("RUSTC path")),
        "--version",
    );
    let cargo = version(
        PathBuf::from(env::var_os("CARGO").expect("CARGO path")),
        "--version",
    );
    let target = env::var("TARGET").expect("TARGET triple");
    println!("cargo:rustc-env=VAL_KERNEL_SOURCE_SHA256={source_hash}");
    println!("cargo:rustc-env=VAL_KERNEL_LOCK_SHA256={lock_hash}");
    println!("cargo:rustc-env=VAL_KERNEL_BUILD_RUSTC={rustc}");
    println!("cargo:rustc-env=VAL_KERNEL_BUILD_CARGO={cargo}");
    println!("cargo:rustc-env=VAL_KERNEL_BUILD_TARGET={target}");
    println!("cargo:rerun-if-env-changed=RUSTC");
    println!("cargo:rerun-if-env-changed=CARGO");
    println!("cargo:rerun-if-env-changed=TARGET");
}
