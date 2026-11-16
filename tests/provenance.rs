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

/// A scratch directory, emptied first.
fn scratch(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("provenance-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// One small shard from the datagen binary, with `extra` flags.
fn datagen_shard(dir: &std::path::Path, extra: &[&str]) -> serde_json::Value {
    let settings = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/datagen.toml"))
        .unwrap()
        .lines()
        .map(|l| match l.split('=').next().map(str::trim) {
            Some("nodes") => "nodes = 300".to_string(),
            Some("hash_mb") => "hash_mb = 1".to_string(),
            Some("shard_positions") => "shard_positions = 40".to_string(),
            _ => l.to_string(),
        })
        .collect::<Vec<_>>()
        .join("\n");
    let config = dir.join("datagen.toml");
    std::fs::write(&config, settings).unwrap();
    let out = dir.join("shards");
    let status = std::process::Command::new(env!("CARGO_BIN_EXE_datagen"))
        .arg("--config")
        .arg(&config)
        .arg("--out")
        .arg(&out)
        .args([
            "--run",
            "r",
            "--task",
            "t0",
            "--threads",
            "1",
            "--seed",
            "7",
        ])
        .args(["--max-positions", "40"])
        .args(extra)
        .output()
        .unwrap();
    assert!(
        status.status.success(),
        "{}",
        String::from_utf8_lossy(&status.stderr)
    );
    let manifest = out.join("t0-w000-s00000.json");
    serde_json::from_str(&std::fs::read_to_string(manifest).unwrap()).unwrap()
}

const SELFPLAY_SHA: &str = "0123456789abcdef0123456789abcdef01234567";

#[test]
fn a_shard_records_the_pinned_engine_and_the_commit_that_made_it() {
    let dir = scratch("datagen");
    let m = datagen_shard(&dir, &["--selfplay-sha", SELFPLAY_SHA]);
    let p = &m["provenance"];
    assert_eq!(p["engine_version"], locked("version"));
    assert_eq!(p["engine_sha"], env!("BRAVEHEART_ENGINE_SHA"));
    assert_eq!(p["selfplay_sha"], SELFPLAY_SHA);
    std::fs::remove_dir_all(&dir).ok();
}

/// Without the flag the manifest has no `selfplay_sha` at all, as every shard
/// made in the engine repository: readers tell the two apart by its presence.
#[test]
fn a_shard_without_the_flag_has_no_selfplay_sha() {
    let dir = scratch("datagen-bare");
    let m = datagen_shard(&dir, &[]);
    assert!(
        m["provenance"].get("selfplay_sha").is_none(),
        "{}",
        m["provenance"]
    );
    std::fs::remove_dir_all(&dir).ok();
}

#[test]
fn a_relabelled_shard_records_the_pinned_engine_and_the_commit_that_made_it() {
    let dir = scratch("relabel");
    datagen_shard(&dir, &[]);
    let task = dir.join("in/gen=7/run=r/task=t0");
    for sub in ["manifests", "positions", "games"] {
        std::fs::create_dir_all(task.join(sub)).unwrap();
    }
    let stem = "t0-w000-s00000";
    let shards = dir.join("shards");
    std::fs::copy(
        shards.join(format!("{stem}.json")),
        task.join(format!("manifests/{stem}.json")),
    )
    .unwrap();
    std::fs::copy(
        shards.join(format!("{stem}.positions.txt")),
        task.join(format!("positions/{stem}.txt")),
    )
    .unwrap();
    std::fs::copy(
        shards.join(format!("{stem}.games.jsonl")),
        task.join(format!("games/{stem}.jsonl")),
    )
    .unwrap();
    let relabel = |out: &str, extra: &[&str]| -> serde_json::Value {
        let status = std::process::Command::new(env!("CARGO_BIN_EXE_relabel"))
            .args(["--net", "default", "--nodes", "300", "--threads", "1"])
            .arg("--in")
            .arg(dir.join("in"))
            .arg("--out")
            .arg(dir.join(out))
            .args(extra)
            .output()
            .unwrap();
        assert!(
            status.status.success(),
            "{}",
            String::from_utf8_lossy(&status.stderr)
        );
        let path = dir
            .join(out)
            .join(format!("gen=7/run=r/task=t0/manifests/{stem}.json"));
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    };
    let with = relabel("out", &["--selfplay-sha", SELFPLAY_SHA]);
    assert_eq!(with["relabel"]["engine_sha"], env!("BRAVEHEART_ENGINE_SHA"));
    assert_eq!(with["relabel"]["engine_version"], locked("version"));
    assert_eq!(with["relabel"]["selfplay_sha"], SELFPLAY_SHA);
    let without = relabel("out-bare", &[]);
    assert!(
        without["relabel"].get("selfplay_sha").is_none(),
        "{}",
        without["relabel"]
    );
    std::fs::remove_dir_all(&dir).ok();
}

/// The task scripts read the pin with ci/engine-pin.sh; it must name exactly
/// the engine build.rs records.
#[test]
fn the_task_scripts_read_the_same_pin() {
    let out = std::process::Command::new("bash")
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/ci/engine-pin.sh"))
        .output()
        .expect("bash runs ci/engine-pin.sh");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(
        String::from_utf8(out.stdout).unwrap().trim(),
        format!(
            "{} {}",
            env!("BRAVEHEART_ENGINE_VERSION"),
            env!("BRAVEHEART_ENGINE_SHA")
        )
    );
}
