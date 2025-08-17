# finishes

`finishes` copies selected file types from a local Git repository into a clean export directory with a manifest. It helps create lightweight helper packs for LLM tooling.

## Installation

### Arch Linux prerequisites

```bash
sudo pacman -S --needed base-devel git rustup
rustup default stable
```

### Build from source

```bash
cargo install --path finishes
```

## Quick start

```bash
finishes init
# follow prompts for repository path, destination, and file types

finishes sync --dry-run
# preview export without writing files

finishes config --source /path/to/repo --dest /tmp/out --include rs --include md
# update saved paths and include extensions

finishes doctor
# validate configuration and estimate copied files
```

## Configuration
Configuration is stored in a JSON file. See [CONFIGURATION.md](CONFIGURATION.md) for per-OS paths.

```json
{
  "source_repo": "/path/to/repo",
  "destination": "/tmp/out",
  "file_types": ["rs", "md"]
}
```

## Ignore rules

`finishes` honors patterns from both `.gitignore` and `.finishesignore` in the source repository. A `.finishesignore` template is written during `finishes init` if one is missing. Directories such as `venv/`, `target/`, and `node_modules/` are ignored automatically. Use `finishes doctor` to view active rules.

## Development
Run repository checks with `./codex.sh fast-validate`. Commands default to a dry run; pass `--confirm` to execute.

