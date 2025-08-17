# Plan

## Goals
- Add `finishes sync` subcommand supporting `--dry-run`, `--clean`, and `--force`.
- Traverse source using `ignore` + `walkdir` with `.gitignore` and `.finishesignore` unioned.
- Copy allowed files (`.md`, `.mdx`, `.markdown`, `.go`, `.rs`, `.py`) to destination, enforcing 25 MB limit and rejecting unsafe symlinks.
- Produce `export.manifest.json` containing commit SHA and file hashes.

## Tests
- `cargo fmt --all --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`

## SemVer Impact
- Minor release: 0.2.0 → 0.3.0 (new functionality).

## Rollback Strategy
- `git revert <commit>` to remove the sync subcommand and related files.
