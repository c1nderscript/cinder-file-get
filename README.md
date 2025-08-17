# repo-harvest

**Repo Harvest** is a small CLI utility that fetches a filtered snapshot of a
GitHub repository. It resolves a commit, downloads the archive, applies your
include/exclude globs, and writes a deterministic "helper pack" that is ready
for indexing by LLM tooling.

## Quick start

### Initialize a profile

```bash
repo-harvest init my-profile \
  --repo owner/repo \
  --ref main \
  --out ~/harvests \
  --include 'src/**' --include 'docs/**' --include 'README*' \
  --exclude 'node_modules/**' --exclude 'target/**'
```

### Sync the profile

```bash
repo-harvest sync my-profile
```

Set `GITHUB_TOKEN` to raise rate limits:

```bash
export GITHUB_TOKEN=ghp_your_token_here
```

## Build (Arch Linux)

```bash
sudo pacman -S --needed base-devel git rustup
rustup default stable
```

## License

MIT

