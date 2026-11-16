# Contributing

The short version: one-line Conventional Commits, every change through a pull request, squash or rebase merges only, releases cut by Release Please, and the same seed always makes the same data. [AGENTS.md](AGENTS.md) has the rules for the data and the research record.

## Workflow

1. Branch from `main`.
2. Make the change. Keep the code formatted (`cargo fmt`) and run the local checks below.
3. Open a PR with a typed template (see below). The PR title must be a valid Conventional Commit.
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
type(scope)?: subject
```

- At most 100 characters.
- `type` is one of `feat`, `fix`, `docs`, `chore`, `refactor`, `ci`, `build`, `perf`, `style`, `test`, `revert`.
- Breaking changes are marked with `!`, for example `feat!: drop the v1 record format`.
- The only content allowed after the header is a blank line followed by `Co-authored-by: Name <email>` trailers.
- No body text. Details belong in the PR description.

commitlint enforces this with `@commitlint/config-conventional` plus a local rule, `only-coauthor-trailers` (`tools/commitlint/only-coauthor-trailers.mjs`), which fails on any line after the header that is not blank or a `Co-authored-by` trailer. CI runs it over every commit in the PR range, so a rebase merge never lands a non-conforming commit on `main`.

### Why the PR title matters

Squash merges use the PR title as the commit message on `main`, and Release Please reads those messages to decide the next version and to write `CHANGELOG.md`. A `feat` bumps the minor version, a `fix` bumps the patch, and `!` bumps the major (the minor while pre-1.0). CI lints the title with the same rules.

### Dependabot PRs

Dependabot cannot write one-line, lowercase commit messages, so its PRs skip the commit message check and the title check. Merge them with **squash** and a lowercase Conventional Commit subject, for example:

```sh
gh pr merge <n> --squash --subject "ci: bump actions/checkout from 6 to 7"
```

A Dependabot PR that moves `braveheart-core` or `braveheart-search` is moving the engine pin: see [Changing what datagen makes](#changing-what-datagen-makes).

## Pull request templates

Templates live in `.github/PULL_REQUEST_TEMPLATE/`, one per type: `feat.md`, `perf.md`, `fix.md`, `docs.md`, `chore.md`, `refactor.md`, `ci.md`. Pick one with

```sh
gh pr create --template fix.md
```

## Releases

Release Please keeps a release PR open on `main`. Merging it tags `vX.Y.Z`, writes `CHANGELOG.md` and bumps the version in `Cargo.toml` and `Cargo.lock`. Nobody edits those by hand. Release Please only cuts a release when a commit of a type that appears in `CHANGELOG.md` lands on `main`: `feat`, `fix`, `perf`, `revert` or `docs`. `chore`, `ci`, `build`, `refactor`, `style` and `test` never release on their own.
