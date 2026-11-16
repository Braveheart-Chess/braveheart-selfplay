//! Every manifest records the engine that made it: the pinned `braveheart-search`
//! in `Cargo.lock`, never this crate's own version.

const LOCK: &str = include_str!("../Cargo.lock");

fn locked(field: &str) -> String {
    let block = LOCK
        .split("[[package]]")
        .find(|b| {
            b.lines()
                .any(|l| l.trim() == "name = \"braveheart-search\"")
        })
        .expect("braveheart-search in Cargo.lock");
    let line = block
        .lines()
        .find(|l| l.starts_with(field))
        .unwrap_or_else(|| panic!("{field} in its block"));
    line.split('"').nth(1).expect("a quoted value").to_string()
}

#[test]
fn the_recorded_engine_version_is_the_pinned_engines() {
    assert_eq!(env!("BRAVEHEART_ENGINE_VERSION"), locked("version"));
    assert_ne!(env!("BRAVEHEART_ENGINE_VERSION"), env!("CARGO_PKG_VERSION"));
}

#[test]
fn the_recorded_engine_commit_is_the_pinned_one() {
    let source = locked("source");
    assert!(source.starts_with("git+https://github.com/Braveheart-Chess/braveheart?tag="));
    assert_eq!(
        env!("BRAVEHEART_ENGINE_SHA"),
        source.rsplit_once('#').unwrap().1
    );
}
