# Plan

## Goals
- Provide usable documentation and setup instructions, especially for Arch Linux.
- Supply missing project scaffolding files: `toaster.md` for architecture overview and `codex.sh` utility script.
- Update README with build and usage instructions.
- Bump crate version to 0.1.1 and record changes in CHANGELOG.

## Tests
- `cargo test`
- `shellcheck codex.sh`
- `shfmt -w codex.sh`
- `cargo audit` (dependency security check)

## SemVer Impact
- Patch release: 0.1.0 → 0.1.1 (documentation and tooling only).

## Rollback Strategy
- Revert commit: `git revert <commit>` if issues arise.
