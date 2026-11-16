# Guidance for coding agents

This repository makes Braveheart's self-play training data: `datagen` plays the engine against itself, `relabel` re-scores stored positions, and the audits check a generation or a label set before anything trains on it. Read [README.md](README.md) for the layout, [docs/datagen.md](docs/datagen.md) for the formats and the S3 layout, [docs/datasets.md](docs/datasets.md) for the ledger and [CONTRIBUTING.md](CONTRIBUTING.md) for the full rules. The rules below are the ones you MUST follow.

## Rules

- **Never push to `main`.** Every change goes through a pull request on a branch. There are no exceptions after the initial scaffold.
- **Commits are a single line**: `type(scope)?: subject`, at most 100 characters. Allowed types: `feat`, `fix`, `docs`, `chore`, `refactor`, `ci`, `build`, `perf`, `style`, `test`, `revert`. The only thing allowed after the header is a blank line and `Co-authored-by: Name <email>` trailers. No body text. Put details in the PR description.
- **Open PRs with a typed template** and fill in every section:

  ```sh
  gh pr create --template <feat|perf|fix|docs|chore|refactor|ci>.md
  ```

  The PR title MUST be a valid Conventional Commit; squash merges use it as the commit message.
- **Run the repo's checks locally before opening the PR** (see below).
- **Never delete or overwrite stored data.** Generations and label sets are append-only and kept forever. A relabel writes a new label set beside the original; it never replaces one. No workflow, script or command in this repository deletes from the dataset bucket.
- **The same seed makes the same data.** `tests/golden.rs` pins what datagen makes from fixed seeds, with values computed by the engine's own code. `src/rng.rs` is a verbatim copy of the engine's generator and never changes. A change that alters the data is a `feat` (or a `fix`); the PR says why, updates the fingerprints and records from which run or generation the new data applies. Never update a fingerprint to make CI pass. A `perf` or `refactor` PR leaves the data unchanged.
- **The engine pin moves deliberately.** `Cargo.toml` pins `braveheart-core` and `braveheart-search` to one engine release, and every manifest records that engine's version and commit (`build.rs` reads them from `Cargo.lock`). Moving the pin changes who generates the data: it is its own PR, it updates the golden fingerprints, and it says which generation the new engine plays.
- **Every experiment updates `docs/`.** Data generation, relabelling, audits and one-off measurements all get recorded, including smoke tests and failed, cancelled, interrupted and dropped runs. See [Experiments and the research record](#experiments-and-the-research-record).

## Until the data move

The positions still live in `s3://jwall-engineering-braveheart-nnue`, owned by the engine's project, and Datagen and Relabel still run from [Braveheart-Chess/braveheart](https://github.com/Braveheart-Chess/braveheart).

- **The engine's [docs/datasets.md](https://github.com/Braveheart-Chess/braveheart/blob/main/docs/datasets.md) is the ledger until then.** The copy here is a snapshot from engine commit `1a25b13`; record nothing in it. It is refreshed from the engine's at the data move.
- **The workflows and `ci/` scripts here are not ported yet.** They came over with their history and still build `-p braveheart-tools` and record the repository commit as the engine's. They still run as the engine's project (`project: braveheart`), whose task launch role trusts only the engine repository, so they fail before launching anything. Don't run them; porting them is part of the data move.

## Experiments and the research record

This project is research: a result counts only when someone else can reproduce it and check it from the record.

| When you run | Record it in |
|:--|:--|
| Datagen, a Relabel run, a generation's or label set's audit | [docs/datasets.md](docs/datasets.md) (the engine's until the data move) |
| A design measurement (relabelling, datagen settings) | That topic's page in `docs/` |
| Training, a net's SPRT or rating | [braveheart-trainer's docs/nets.md](https://github.com/Braveheart-Chess/braveheart-trainer/blob/main/docs/nets.md) (the engine's until the data move) |

- **Record it promptly.** Use the same PR when the experiment has one. Otherwise open a docs PR when the run ends, before another experiment builds on the result.
- **Decide before you run.** Write down the question, the one thing that changes, what stays the same and what result would keep or drop it. Don't move the criterion after seeing the result.
- **Make it reproducible.** Record the commit or tag of this repository and of the engine it pins, the workflow inputs or exact command, the run id, seeds, the generations and label sets, each net's id and SHA-256, and the hardware and market.
- **Use primary sources.** Take numbers from shard manifests, `task.json` or `datagen-audit`, never from progress lines, memory or another document. Link the source. Label estimates, such as costs, and local measurements as what they are.
- **Audit before you build on it.** A generation needs a full audit with a GO, and a label set its recorded audit, before anything trains on it. Where a check could pass without testing anything, add a negative control.
- **Say what wasn't verified.** Name any check you skipped or couldn't run, and why. When you check a ledger against its sources, add the result to its audit log, gaps included.
- **Keep every result and correct in the open.** Never delete or reword a failed or disappointing result. Record every attempt, including reclaimed legs and re-runs, and the gate in force when the decision was made. A later rule doesn't rewrite an earlier decision; note how the two differ.

## Before opening a PR

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --locked
python3 ci/test-relabel-manager.py
python3 ci/test-datagen-manager.py
npm ci && npm test
```

## Things that need the owner

Repository secrets and variables, the release bot app installation, repository settings, AWS (applies happen through the infra repository's workflows) and anything that writes to the dataset bucket. List them in the PR instead of attempting them.
