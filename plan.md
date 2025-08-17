# Plan

## Goals
- Add GitHub Actions workflow to build and test the `finishes` crate on `ubuntu-latest` and `macos-latest`.
- Ensure CI runs `cargo clippy`, `cargo test`, and `cargo dist` packaging.

## Tests
- `cargo fmt --all --check`
- `cargo clippy --manifest-path finishes/Cargo.toml -- -D warnings`
- `cargo test --manifest-path finishes/Cargo.toml`
- `cargo dist build --manifest-path finishes/Cargo.toml`

## SemVer Impact
- Patch release: workflow update only.

## Rollback Strategy
- `git revert <commit>` to restore the previous workflow.
