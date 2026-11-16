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
on 2026-11-16, with their history. `crates/tools/` became `crates/selfplay/`. Pull
request numbers in the older commit messages refer to the engine repository.

**Interim.** Until the data move, the positions stay in
`s3://jwall-engineering-braveheart-nnue`, owned by the engine's project, and
datagen and relabel keep running from the engine repository. This repository
gets its own bucket, `jwall-engineering-braveheart-selfplay-positions`, at the
data move.
