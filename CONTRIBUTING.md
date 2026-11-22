# Contributing

The short version: one-line Conventional Commits, every change through a pull request, squash or rebase merges only, releases cut by Release Please, and the same seed always makes the same data. [AGENTS.md](AGENTS.md) has the rules for the data and the research record.

## Workflow

1. Branch from `main`.
2. Make the change. Keep the code formatted (`cargo fmt`) and run the local checks below.
3. Open a PR from the template (see below) and fill in every section. The PR title must be a valid commit header, scope included.
4. Merge with **squash** or **rebase**. Merge commits are disabled and history stays linear.

Direct pushes to `main` are not allowed. The only exception was the initial scaffold commit.

## Local checks

```sh
cargo fmt --all --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --locked
python3 ci/test-relabel-manager.py
python3 ci/test-datagen-manager.py
npm ci && npm test
```

CI runs the same checks on every pull request. Optional local hooks catch mistakes before they are pushed:

```sh
pre-commit install --hook-type commit-msg --hook-type pre-commit
```

## Changing what datagen makes

`tests/golden.rs` hashes four games from fixed seeds, at settings written out in the test, with their training records. The expected values come from the engine repository's own `datagen` code at v0.54.0. The test fails when the data changes, whatever the cause: the random number generator, the engine pin, the search, the filters or the record format.

- A `perf` or `refactor` PR leaves it passing untouched.
- A `feat` or `fix` that changes the data on purpose updates the fingerprints, says why in the PR and records in [docs/datasets.md](docs/datasets.md) from which run or generation the new data applies.
- Moving the engine pin in `Cargo.toml` is its own PR, for the same reasons.

## Commit messages: one line, nothing else

Every commit message is a single line:

```
type(scope): subject
```

- At most 100 characters, with an imperative, lowercase subject and no trailing period.
- `type` is one of `feat`, `fix`, `perf`, `revert`, `docs`, `refactor`, `test`, `build`, `ci`, `style`, `chore`: the same set in every repository.
- `scope` is required: exactly one of the scopes below. A change that spans two is two pull requests, or takes the scope it is mostly about.
- Breaking changes are marked with `!`, for example `feat(datagen)!: drop the v1 record format`.
- The only content allowed after the header is a blank line followed by `Co-authored-by: Name <email>` trailers, in any letter case. No body text. Details belong in the PR description.

commitlint enforces this with `@commitlint/config-conventional` plus three local rules in `tools/commitlint/`: `only-coauthor-trailers` fails on any line after the header that is not blank or a `Co-authored-by` trailer, `single-scope` fails on a list such as `feat(a,b)`, and `subject-lowercase` requires a lowercase subject except in Dependabot's fixed `chore(deps): Bump ...`. The fixtures in `tests/commitlint/` show accepted and rejected messages. CI lints every commit in the PR range, so a rebase merge never lands a non-conforming commit on `main`, and lints the PR title through the same config. The standard behind it is the [repository conventions](https://github.com/jwallace145/repository-template/blob/main/repository-conventions.md).

### Scopes

Every commit names exactly one of this repository's scopes. They are `SCOPES` in `commitlint.config.mjs`; this table mirrors it, in the same order, and `npm run test:commitlint` fails if they drift. Adding, renaming or removing a scope goes through a pull request like any other change.

| Scope | Covers |
| --- | --- |
| `datagen` | Self-play generation: `src/datagen.rs`, `datagen.toml`, the Datagen workflow and its manager |
| `relabel` | Relabelling: `src/relabel.rs`, the Relabel workflow and its manager |
| `audit` | The dataset, label-set and bucket-copy audits |
| `datasets` | `docs/datasets.md` and the record of every generation and label set |
| `infra` | `jaws.yaml` and the Jaws workflows |
| `repo` | README, CONTRIBUTING, AGENTS, commitlint, the PR and issue templates, and repository tooling |
| `deps` | Dependency updates, mostly Dependabot's |
| `release` | Release Please's release pull requests and its config |

### Why the PR title matters

Squash merges use the PR title as the commit message on `main`, and Release Please reads those messages to decide the next version and to write `CHANGELOG.md`. A `feat` bumps the minor version, a `fix` bumps the patch, and `!` bumps the major (the minor while pre-1.0). CI lints the title through the same commitlint config as a commit, scope included. Release Please's own pull requests are titled `chore(release): release X.Y.Z`.

### Dependabot PRs

Dependabot writes its titles as `chore(deps): bump ...`, so they pass the title check as they are. Its commits carry a release-notes body, so its PRs skip the commit message check; merge them with **squash**, which lands only the title on `main`.

A Dependabot PR that moves `braveheart-core` or `braveheart-search` is moving the engine pin: see [Changing what datagen makes](#changing-what-datagen-makes).

## Pull request template

One template, `.github/pull_request_template.md`: Summary, Why, Changes, Verification, Data, and a checklist. `gh pr create` and the compare page both start from it. Fill in every section; write "n/a" rather than deleting one. For a fix, Why names the bug, its root cause and how to reproduce it.

## Issues

Open issues from the forms: **Bug**, **Feature** or **Task**. Blank issues are off.
## Releases

Release Please keeps a release PR open on `main`. Merging it tags `vX.Y.Z`, writes `CHANGELOG.md` and bumps the version in `Cargo.toml` and `Cargo.lock`. Nobody edits those by hand. Release Please only cuts a release when a commit of a type that appears in `CHANGELOG.md` lands on `main`: `feat`, `fix`, `perf`, `revert` or `docs`. `chore`, `ci`, `build`, `refactor`, `style` and `test` never release on their own.
