# Plan

## Goals
- Add `finishes config` subcommand to display or modify stored configuration values (source, destination, includes).
- Add `finishes doctor` subcommand to validate paths, report ignore rules, count candidate files, and estimate export changes.
- Bump `finishes` crate version to 0.4.0 and document new commands.

## Tests
- `cargo fmt --all --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`

## SemVer Impact
- Minor release: 0.3.0 → 0.4.0 (new features).

## Rollback Strategy
- `git revert <commit>` to remove new subcommands and version bump.
