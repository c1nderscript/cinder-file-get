# Plan

## Goal
Add Rust CI workflow running format, lint, tests, and release build while creating required project scaffolding.

## Steps
1. Create `.github/workflows/rust.yml` that runs `cargo fmt -- --check`, `cargo clippy -- -D warnings`, `cargo test --all --locked`, and `cargo build --release` on `push` and `pull_request`.
2. Add `codex.sh` script with dry-run default and required subcommands.
3. Add `toaster.md` architecture overview document.
4. Update `README.md` to mention `codex.sh`.
5. Record changes in `CHANGELOG.md` under **Unreleased**.

## Testing
- `cargo fmt -- --check`
- `cargo clippy -- -D warnings`
- `cargo test --all --locked`
- `cargo build --release`
- `shellcheck codex.sh`
- `shfmt -d codex.sh`

## SemVer
Patch: documentation and continuous integration only.

## Rollback
Revert the commit with `git revert` to remove workflow and scaffolding files.
