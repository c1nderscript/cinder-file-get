# Plan

## Goals
- Introduce new `finishes` crate with CLI skeleton and logging.
- Expose foundational modules for configuration and file operations.
- Record crate addition in `CHANGELOG.md`.

## Tests
- `cargo fmt --all --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`

## SemVer Impact
- Minor release: 0.1.1 → 0.2.0 (new functionality).

## Rollback Strategy
- `git revert <commit>` to remove the new crate.
