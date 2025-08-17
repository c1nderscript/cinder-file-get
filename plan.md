# Plan

## Goals
- Release `finishes` crate version `0.5.0` and move `Unreleased` notes to a dated section.
- Document `finishes` commands, configuration handling, and manifest features in docs.

## Tests
- `cargo fmt --all --check`
- `cargo clippy --manifest-path finishes/Cargo.toml -- -D warnings`
- `cargo test --manifest-path finishes/Cargo.toml`
- `cargo build --manifest-path finishes/Cargo.toml --release`

## SemVer Impact
- Minor release: `0.4.2` → `0.5.0` (new features and documentation).

## Rollback Strategy
- `git revert <commit>` to undo release changes.
