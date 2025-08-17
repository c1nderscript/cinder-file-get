# Plan

## Goals
- Add `finishes init` subcommand using `clap`.
- Collect repo path, destination, and file types via `inquire` prompts.
- Persist configuration JSON and create `.finishesignore` template.

## Tests
- `cargo fmt --all --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`

## SemVer Impact
- Minor release: 0.1.0 → 0.2.0 (new functionality).

## Rollback Strategy
- `git revert <commit>` to remove the subcommand and related files.
