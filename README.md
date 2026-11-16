# braveheart-selfplay

Braveheart's self-play dataset, and the pipeline that grows it: billions of
self-play positions, each labelled by the engine itself, with the audits that
check every generation before anything trains on it.

- **Datagen** plays the engine against itself and stores the games and positions.
- **Relabel** re-scores stored positions with another net or search depth.
- **The audits** (`datagen-audit`, the label-set audit) check a generation or a
  label set before training uses it.
- **The ledger**, [docs/datasets.md](docs/datasets.md), records every run,
  generation and label set, with its sources.

The engine is [Braveheart-Chess/braveheart](https://github.com/Braveheart-Chess/braveheart);
the nets are trained by [Braveheart-Chess/braveheart-trainer](https://github.com/Braveheart-Chess/braveheart-trainer).

## Moved from the engine repository

These files moved here from `Braveheart-Chess/braveheart` at
[`1a25b13`](https://github.com/Braveheart-Chess/braveheart/commit/1a25b1375049c88b2c9cf01d26c90216f5f65d88)
on 2026-11-16, with their history. Their part of the engine's `crates/tools/`
became this repository's crate, at the root. Pull request numbers in the older
commit messages refer to the engine repository.

**Interim.** Until the data move, the positions stay in
`s3://jwall-engineering-braveheart-nnue`, owned by the engine's project, and
datagen and relabel keep running from the engine repository. This repository
gets its own bucket, `jwall-engineering-braveheart-selfplay-positions`, at the
data move. Until then:

- The engine's [docs/datasets.md](https://github.com/Braveheart-Chess/braveheart/blob/main/docs/datasets.md)
  is the ledger; the copy here is a snapshot.
- The Datagen and Relabel workflows and the `ci/` scripts are not ported yet.
  The project file declares no dataset, so both workflows stop at their first
  step.

## Building

The crate depends on the engine's `braveheart-core` and `braveheart-search`,
pinned to one engine release in `Cargo.toml`. Every manifest records that
engine's version and commit, which `build.rs` reads from `Cargo.lock`.

```sh
cargo build --release --locked      # datagen, relabel, datagen-audit
cargo test --locked                 # includes tests/golden.rs
```

`tests/golden.rs` pins the data: four games from fixed seeds, hashed, with the
expected values computed by the engine's own code at the pinned release. It
fails whenever the same seed would make different data. See
[CONTRIBUTING.md](CONTRIBUTING.md#changing-what-datagen-makes).

| Path | What it is |
|:--|:--|
| `src/datagen.rs`, `src/bin/datagen.rs` | Self-play: games, filters, shards and manifests |
| `src/relabel.rs`, `src/bin/relabel.rs` | Re-scoring stored positions into a new label set |
| `src/bin/datagen_audit.rs` | The generation audit |
| `src/rng.rs` | The engine's random number generator, copied verbatim |
| `datagen.toml` | The settings datagen plays with, and the audit's bars |
| `ci/` | The task scripts and managers the workflows run |
| `docs/` | The formats, the runbooks and the ledger |
