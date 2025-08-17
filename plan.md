# Plan

## Goals
- Add unit tests for path validation, ignore unions, extension filters, and manifest hashing.
- Add integration tests using temporary repositories with `venv/`, `target/`, and `node_modules/` directories to ensure they are excluded.
- Add property tests generating random file trees to confirm no traversal outside the repository and that the size limit is enforced.
- Bump `finishes` crate version to `0.4.1` and document new tests.

## Tests
- `cargo fmt --all --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`

## SemVer Impact
- Patch release: `0.4.0` → `0.4.1` (tests only).

## Rollback Strategy
- `git revert <commit>` to undo the version bump and test additions.
