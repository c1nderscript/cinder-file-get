# Plan

## Goals
- Ensure `codex.sh` commands operate on the `finishes` crate using `cargo --manifest-path finishes/Cargo.toml`.
- Preserve dry-run defaults and root safety while keeping the script lintable and formatted.

## Tests
- `shellcheck codex.sh`
- `shfmt -d codex.sh`
- `cargo fmt --all --check`
- `cargo clippy --manifest-path finishes/Cargo.toml -- -D warnings`
- `cargo test --manifest-path finishes/Cargo.toml`

## SemVer Impact
- Patch release: tooling script adjustment.

## Rollback Strategy
- `git revert <commit>` to restore the previous script.
