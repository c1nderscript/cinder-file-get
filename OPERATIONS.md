# Operations

## Common tasks
- `finishes init` — interactively create configuration and `.finishesignore` template.
- `finishes sync` — copy matching files to the destination; combine `.gitignore` and `.finishesignore` rules.
- `finishes config` — display or update stored paths and file type filters.
- `finishes doctor` — diagnose configuration, list ignore rules, and estimate export changes.

## Dry run
Use `finishes sync --dry-run` to preview actions without writing files. The `codex.sh` helpers default to a dry run; pass `--confirm` to execute.

## Cleaning
`finishes sync --clean` removes the destination directory before copying. Combine with `--dry-run` to preview removal.

## Troubleshooting
- Run `finishes doctor` to validate paths and inspect ignore patterns.
- Ensure the destination directory is writable when not using `--dry-run`.
- Regenerate configuration with `finishes init` if settings become invalid.
