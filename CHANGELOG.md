# Changelog
All notable changes to this project will be documented in this file.

## [Unreleased]

### Added
- CI workflow to build and test `finishes` on `ubuntu-latest` and `macos-latest` with `cargo clippy`, `cargo test`, and `cargo dist` packaging.

### Fixed
- `codex.sh` commands now target the `finishes` crate and honor dry-run mode by default.

## [0.5.0] - 2025-08-17
### Added
- `finishes sync` supports `--clean` to wipe the destination and `--force` to overwrite existing files.
- `finishes config` updates saved paths and include patterns.
- `finishes sync` writes `export.manifest.json` with commit SHA and file hashes; `finishes doctor` reports new or changed files.

## [0.4.2] - 2025-08-17
### Added
- Overhauled `README.md` for the `finishes` CLI including install, quick start, configuration, and ignore rules.
- Added `CONFIGURATION.md` and `OPERATIONS.md` guides.
- Updated `AGENTS.md` and `toaster.md` to reference the `finishes` architecture.

## [0.4.1] - 2025-08-17
### Added
- Unit tests for path validation, ignore unions, extension filters, and manifest hashing
- Integration tests ensuring venv/, target/, and node_modules/ directories are excluded
- Property tests verifying no traversal outside the repository and enforcing the size limit

## [0.4.0] - 2025-08-17
### Added
- `finishes config` subcommand to display or modify stored values
- `finishes doctor` subcommand to validate paths and estimate export changes

### Changed
- Bump `finishes` crate version to 0.4.0

## [0.3.0] - 2025-08-17
### Added
- Initial `finishes` crate with logging and CLI skeleton
- Interactive `finishes init` subcommand with config persistence
  and `.finishesignore` template
- `finishes sync` subcommand with filtering, copy, and manifest export

### Changed
- Bump `repo-harvest` crate version to 0.2.0
- Bump `finishes` crate version to 0.3.0

## [0.1.1] - 2025-08-17
### Added
- Arch Linux setup and usage instructions in README
- `toaster.md` architecture overview
- `codex.sh` helper script
### Changed
- Bump crate version to 0.1.1

## [0.1.0] - 2025-08-17
### Added
- MIT license
- Initialize `repo-harvest` Rust crate
- Add `.gitignore` for Rust build artifacts
- Rust CI workflow and repository scaffolding (`codex.sh`, `toaster.md`)
