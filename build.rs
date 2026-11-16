//! The engine this crate is built against, for every manifest's provenance.
//!
//! In the engine repository, `datagen` and `relabel` recorded
//! `CARGO_PKG_VERSION`, which was the engine's version. Here that is this
//! crate's own, so the engine's version and commit are read from the pinned
//! `braveheart-search` entry of `Cargo.lock` instead and passed
//! to the binaries as `BRAVEHEART_ENGINE_VERSION` and `BRAVEHEART_ENGINE_SHA`.

use std::path::Path;

fn main() {
    let lock = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock");
    println!("cargo:rerun-if-changed={}", lock.display());
    let text = std::fs::read_to_string(&lock).expect("Cargo.lock");
    let (version, sha) = engine(&text).expect("braveheart-search, from git, in Cargo.lock");
    println!("cargo:rustc-env=BRAVEHEART_ENGINE_VERSION={version}");
    println!("cargo:rustc-env=BRAVEHEART_ENGINE_SHA={sha}");
}

/// The version and git commit of the `braveheart-search` package.
fn engine(lock: &str) -> Option<(String, String)> {
    let block = lock.split("[[package]]").find(|b| {
        b.lines()
            .any(|l| l.trim() == "name = \"braveheart-search\"")
    })?;
    let field = |key: &str| {
        block.lines().find_map(|l| {
            l.trim()
                .strip_prefix(key)
                .and_then(|v| v.trim().strip_prefix("= \""))
                .and_then(|v| v.strip_suffix('"'))
                .map(str::to_string)
        })
    };
    let version = field("version")?;
    let source = field("source")?;
    let sha = source.rsplit_once('#')?.1.to_string();
    (sha.len() == 40 && sha.bytes().all(|b| b.is_ascii_hexdigit())).then_some((version, sha))
}
