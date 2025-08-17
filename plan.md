# Plan

## Goals
- Rewrite `README.md` to document the `finishes` CLI, installation steps, quick start, configuration schema, and ignore rules.
- Create `CONFIGURATION.md` describing per-OS config file locations.
- Add `OPERATIONS.md` documenting common tasks, dry-run usage, cleaning, and troubleshooting.
- Update `AGENTS.md` and `toaster.md` to reference the `finishes` CLI and current architecture overview.
- Bump crate version to `0.4.2` and record documentation updates in `CHANGELOG.md`.

## Tests
- `cargo fmt --all --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`

## SemVer Impact
- Patch release: `0.4.1` → `0.4.2` (documentation and metadata only).

## Rollback Strategy
- `git revert <commit>` to undo documentation changes and version bump.
