<!--
PR title must be a valid Conventional Commit, e.g. `perf(scope): speed up something`.
Squash merges use the title as the commit message.
-->

## Summary

<!-- One or two sentences on what this PR does. -->

## Motivation and context

<!-- Why is this change needed? Link issues, runs or the docs/datasets.md entry. -->

## What changed

<!-- Bullet the changes. Call out anything that changes the records, the manifests, the S3 layout or a workflow's inputs. -->

-

## How it was verified

<!-- e.g. cargo fmt/clippy/test, the manager tests, a local datagen or relabel run, a workflow run (link it). -->

-

## Speed

<!-- The same command before and after on the same machine; the data must be unchanged. -->

- Before: `<positions an hour, or records a second>`
- After: `<positions an hour, or records a second>`

## Data

<!-- tests/golden.rs pins what datagen makes from fixed seeds. A perf PR leaves it unchanged. -->

- [ ] The data is unchanged: `tests/golden.rs` passes untouched.

## Checklist

- [ ] PR title is a valid Conventional Commit (`perf`)
- [ ] Every commit is a single line (only `Co-authored-by` trailers may follow)
- [ ] Every run this PR made is recorded in docs/ (see AGENTS.md)
- [ ] Docs updated (README, docs/, CONTRIBUTING, AGENTS) where behaviour changed
