# cinder-file-get


`cinder-file-get` retrieves selected files from a GitHub repository to build a lightweight helper pack for LLM tooling.

## Arch Linux setup

```bash
sudo pacman -S --needed base-devel git rustup
rustup default stable
```

## Build and test

```bash
cd finishes
cargo test
cargo build --release
```

## Example usage

```bash
finishes init
# follow prompts for repo path, destination, and file types

finishes sync --dry-run
# preview export without writing files

finishes config --source /path/to/repo --dest /tmp/out --include rs --include md
# update saved paths and include extensions

finishes doctor
# validate configuration and estimate copied files
```

Configuration is saved to `~/.config/finishes/config.json` and a `.finishesignore`
template is written to the chosen repository if absent.

The `codex.sh` script offers dry-run helpers for bootstrap and validation. See [AGENTS.md](AGENTS.md) for detailed concepts and options.

## Development
Run repository checks with `./codex.sh fast-validate`. Commands default to a dry-run; pass `--confirm` to execute.

