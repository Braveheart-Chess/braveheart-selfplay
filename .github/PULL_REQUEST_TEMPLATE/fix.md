<!--
PR title must be a valid Conventional Commit, e.g. `fix(scope): correct something`.
Squash merges use the title as the commit message.
-->

## Summary

<!-- One or two sentences on what this PR does. -->

## Motivation and context

<!-- Why is this change needed? Link issues, runs or the docs/datasets.md entry. -->

## Bug

<!-- What is wrong, as observed. Include the output, a manifest or a failing record if short. -->

## Root cause

<!-- Why it happens. -->

## Reproduction

<!-- Steps or a command that shows the bug before this change. -->

## What changed

<!-- Bullet the changes. Call out anything that changes the records, the manifests, the S3 layout or a workflow's inputs. -->

-

## How it was verified

<!-- e.g. cargo fmt/clippy/test, the manager tests, a local datagen or relabel run, a workflow run (link it). -->

-

## Data

<!-- tests/golden.rs pins what datagen makes from fixed seeds. Pick one. -->

- [ ] The data is unchanged: `tests/golden.rs` passes untouched.
- [ ] The data changes on purpose: the fingerprints are updated, the reason is
      above, and docs/datasets.md says from which run or generation it applies.

## Checklist

- [ ] PR title is a valid Conventional Commit (`fix`)
- [ ] Every commit is a single line (only `Co-authored-by` trailers may follow)
- [ ] Every run this PR made is recorded in docs/ (see AGENTS.md)
- [ ] Docs updated (README, docs/, CONTRIBUTING, AGENTS) where behaviour changed
