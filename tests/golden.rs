//! The data this repository makes, pinned.
//!
//! Four games from fixed seeds, at fixed settings written out here (not
//! `datagen.toml`, which changes when a new net ships), hashed with their
//! training records. The expected values were computed by the engine
//! repository's own `datagen` code at v0.54.0 (`braveheart-tools`,
//! commit 3fd0223), the code this crate was moved from.
//!
//! A change here means datagen no longer makes the same data from the same
//! seed: the random number generator, the engine pin, the search, the filters
//! or the record format changed. That may be intended (moving the engine pin
//! is the usual reason); then the PR says so, records it in docs/datasets.md
//! and updates the value. It is never updated to make CI pass.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use braveheart_search::search::Searcher;
use braveheart_selfplay::datagen::{Settings, play_game, record_line};
use braveheart_selfplay::rng::Rng;
use sha2::{Digest, Sha256};

const SETTINGS: &str = r#"
[datagen]
generation = GENERATION
nodes = 300
random_plies = 8
max_opening_cp = 250
hash_mb = 1
shard_positions = 25000
win_plies = 4
win_cp = 1200
draw_after_ply = 80
draw_plies = 12
draw_cp = 10
max_plies = 400
"#;

/// SHA-256 over four games from seeds 0 to 3, played by one searcher as a
/// datagen worker plays them: each game's JSON line, then its training records.
fn fingerprint(generation: u32) -> String {
    let settings =
        Settings::parse(&SETTINGS.replace("GENERATION", &generation.to_string())).unwrap();
    braveheart_core::attacks::init();
    let mut searcher = Searcher::new(Arc::new(AtomicBool::new(false)));
    let mut hasher = Sha256::new();
    for seed in 0..4u64 {
        let game = play_game(&mut Rng::seeded(seed), &mut searcher, &settings);
        hasher.update(serde_json::to_string(&game).unwrap());
        hasher.update(b"\n");
        for sample in &game.samples {
            hasher.update(record_line(sample, game.result));
            hasher.update(b"\n");
        }
    }
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// PeSTO: the generator, the search, the filters and the record format.
#[test]
fn generation_0_games_are_the_engines() {
    assert_eq!(
        fingerprint(0),
        "23b3f49bf0adb2858ba8eb7e4e263c2a986c0ece8c3c80667426159e18cbad22"
    );
}

/// The net generation 7's data is played with, embedded in the pinned engine.
#[test]
fn generation_7_games_are_the_engines() {
    assert_eq!(
        fingerprint(7),
        "1e5181a0ed481eca63a98551909359882e7cabb5ab5e3e433acb54ec8dd8d4ce"
    );
}
