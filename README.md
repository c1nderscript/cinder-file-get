# cinder-file-get

`cinder-file-get` retrieves selected files from a GitHub repository to build a lightweight helper pack for LLM tooling.

## Arch Linux setup

```bash
sudo pacman -S --needed base-devel git rustup
rustup default stable
```

## Build and test

```bash
cd repo-harvest
cargo test
cargo build --release
```

## Example usage

```bash
repo-harvest init example \
  --repo owner/repo \
  --out ~/harvests \
  --include 'src/**' --exclude 'target/**'
```

The `codex.sh` script offers dry-run helpers for bootstrap and validation. See [AGENTS.md](AGENTS.md) for detailed concepts and options.
