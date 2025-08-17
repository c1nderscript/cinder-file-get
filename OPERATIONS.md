# Operations

## Common tasks
- `finishes init` — interactively create configuration and `.finishesignore` template.
- `finishes sync` — copy matching files to the destination; combine `.gitignore` and `.finishesignore` rules. Supports `--clean` to wipe the destination and `--force` to overwrite existing files.
- `finishes config` — display or update stored paths and file type filters.
- `finishes doctor` — diagnose configuration, list ignore rules, and estimate export changes.

## Dry run
Use `finishes sync --dry-run` to preview actions without writing files. The `codex.sh` helpers default to a dry run; pass `--confirm` to execute.

## Cleaning
`finishes sync --clean` removes the destination directory before copying. Combine with `--dry-run` to preview removal.

## Manifest
`finishes sync` writes `export.manifest.json` with the commit SHA and file hashes. `finishes doctor` reads this manifest to report new or changed files.

## Troubleshooting
- Run `finishes doctor` to validate paths and inspect ignore patterns.
- Ensure the destination directory is writable when not using `--dry-run`.
- Regenerate configuration with `finishes init` if settings become invalid.
