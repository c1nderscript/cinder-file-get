# AGENTS.md — Repo Helper Harvester (Arch • Rust)

**Language:** Rust
**Binary name (suggested):** `repo-harvest`
**Additional crate:** `finishes`
**Purpose:** Obtain all **relevant files** from a chosen GitHub repository to build a **GPT/Claude helper pack** (code + core docs + configs). Runs simply, then **persists its chosen include/exclude paths as a profile** so subsequent runs can **repopulate and sync to the most recent changes** with one command.

------

## 0) TL;DR (operator view)

```bash
# Arch toolchain
sudo pacman -S --needed base-devel git rustup
rustup default stable

# First-run: create a profile and harvest a snapshot
repo-harvest init octobot \
  --repo Drakkar-Software/OctoBot \
  --ref main \
  --out ~/harvests \
  --include 'src/**' --include 'docs/**' --include 'README*' \
  --include 'CONTRIBUTING*' --include 'CHANGELOG*' --include 'LICENSE*' \
  --exclude 'node_modules/**' --exclude 'target/**' --exclude '**/fixtures/**'

# Subsequent syncs: reuse the saved profile and pull only new changes
repo-harvest sync octobot
```

Optional (raise API limits):

```bash
export GITHUB_TOKEN=ghp_your_token_here
```

------

## 1) Mission & Scope

**Mission.** Produce a clean local "**LLM helper pack**" from a repo: source files that define the public surface area, essential documentation, build/run configs, and legal notices. Output is deterministic, small enough to index quickly, and suitable for ChatGPT/Claude ingestion.

**In scope**

- Archive-based fetch from GitHub for `owner/repo` at branch/tag/SHA (`--ref`).
- Optional `--subpath` to focus on a subtree.
- Gitignore-like **include/exclude** patterns tuned for LLM context utility.
- Persistent **profiles** so future `sync` runs use the same rules.
- Deterministic output layout + JSON **MANIFEST** with commit SHA and file hashes.
- Optional "**context pack**" export (flattened Markdown copy and a concatenated prompt-ready bundle).

**Out of scope (for now)**

- Non-GitHub providers; full VCS operations; building docs.

------

## 2) Target Platform & Packaging

- **Primary:** Arch Linux (x86_64).
- **Build deps:** `base-devel`, `rustup` stable, `git`.
- **TLS:** `reqwest` with **rustls** (avoid system OpenSSL headaches).
- **Packaging:** Provide `PKGBUILD` for AUR once stabilized.

------

## 3) Core Concepts

- **Profile**: Named configuration that persists repo, ref, includes/excludes, output dir, and last synced commit. Stored at `~/.config/repo-harvest/profiles/<name>.toml` and duplicated in the snapshot root for provenance.
- **Snapshot**: A timestamped, read-only harvest for a specific resolved commit SHA.
- **Context Pack (optional)**: Post-processed bundle for LLM tools: normalized Markdown copies, simple table-of-contents, and a concatenated `CONTEXT.md` capped by byte/line limits.

------

## 4) Design Overview

**Fetch strategy**

- Use GitHub archive download for `--ref` (branch/tag/SHA). Resolve and record the **exact commit SHA** in the manifest.
- Extraction is streamed and path-sanitized (prevents zip-slip).
- Apply include/exclude **glob filters** after extraction; can also gate by size (`--max-size`).

**Defaults optimized for LLM helpers**

- Include: `src/**`, `lib/**`, `include/**`, `examples/**`, `README*`, `CONTRIBUTING*`, `CHANGELOG*`, `LICENSE*`, `CODE_OF_CONDUCT*`, `SECURITY*`, `docs/**`, `*.md`, `*.rst`, `Cargo.toml`, `package.json`, `pyproject.toml`, `requirements*.txt`, `Dockerfile*`, `docker-compose*.yml`, `Makefile*`, `*.proto`, `openapi*.{json,yaml,yml}`.
- Exclude: `.git/**`, `.github/workflows/cache/**`, `node_modules/**`, `target/**`, `.venv/**`, `dist/**`, `build/**`, `**/fixtures/**`, `**/*.bin`, `**/*.mp4`, `**/*.zip`, files larger than `--max-size` MB (default 20) unless `--allow-large`.

**Incremental sync**

- `init` stores the profile and takes an initial snapshot, writing `MANIFEST.json` with `resolved_sha`.
- `sync` resolves the latest HEAD for the configured `ref`. If unchanged, no work. If changed, downloads and re-applies the same filters, producing a new snapshot. Optionally `--since last` can compute a change-only list for faster indexing.

**Deterministic layout**

```
{out}/{owner}-{repo}/{profile}/{resolvedSHA}/{YYYYMMDD-HHMMSSZ}/
  harvest/            # filtered files
  MANIFEST.json       # schema, repo/ref, resolved_sha, files, sizes, hashes
  PROFILE.toml        # the effective profile used
  CONTEXT/            # optional: normalized .md and CONTEXT.md
  HARVEST.log         # operation log
```

------

## 5) CLI Contract

```
repo-harvest init <profile>
    --repo <owner/repo>
    [--ref <branch|tag|sha>]       # default: default branch
    [--subpath <dir>]
    [--out <dir>]                  # default: ~/harvests
    [--include <glob>]...          # repeatable
    [--exclude <glob>]...          # repeatable
    [--max-size <MB>]              # default: 20
    [--allow-large]
    [--make-context]               # generate CONTEXT pack
    [--format human|json|both]
    [--dry-run] [--quiet] [--verbose] [--trace]

repo-harvest sync <profile>
    [--ref <branch|tag|sha>]       # overrides saved ref
    [--make-context]
    [--format human|json|both]

repo-harvest plan <profile>
    # Prints what would change vs last snapshot (new/modified/removed).

repo-harvest profile get <profile>
repo-harvest profile set <profile> --include ... --exclude ... --out ... --ref ...
repo-harvest profile list
repo-harvest profile rm <profile>

repo-harvest inspect <snapshot-path>   # show manifest summary
repo-harvest verify  <snapshot-path>   # re-hash and verify integrity
repo-harvest clean   <profile>         # prune old snapshots by count/age
```

**Exit codes**

- `0` success; `10` bad input; `20` network/API; `30` extraction/filtering; `40` filesystem; `50` internal.

------

## 6) Configuration

**Environment**

- `GITHUB_TOKEN` – increases rate limits; required for private repos.
- `HTTP_PROXY`/`HTTPS_PROXY` – proxy support.
- `RUST_LOG` – e.g., `RUST_LOG=repo_harvest=info`.

**Profile file (`PROFILE.toml`)**

```toml
name      = "octobot"
repo      = "Drakkar-Software/OctoBot"
ref       = "main"
out_dir   = "/home/cinder/harvests"
subpath   = ""
max_size  = 20
make_context = true
include   = ["src/**", "docs/**", "README*"]
exclude   = ["node_modules/**", "target/**", "**/fixtures/**"]
```

CLI flags override the profile.

------

## 7) Context Pack (optional)

When `--make-context` is used, the harvester additionally:

- Copies text-like files to `CONTEXT/` and normalizes line endings and headings.
- Generates `CONTEXT/TOC.md` and a size-limited `CONTEXT/CONTEXT.md` concatenation for quick LLM ingestion.
- Emits `CONTEXT/index.json` describing file order and byte counts.

This is intentionally simple; embedding/vectorization is out of scope.

------

## 8) Build, Test, Lint (Arch)

```bash
sudo pacman -S --needed base-devel git rustup
rustup default stable
rustup component add rustfmt clippy

cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all --locked
cargo build --release
```

**Crate features**

- `tls-rustls` (default): rustls-backed reqwest.
- `tls-native`: enable system TLS if requested.
- `git2` (future): sparse checkout mode.

------

## 9) Security & Safety

- Archive extraction is sanitized; never writes outside the output root.
- Honors `.gitattributes export-ignore` where possible; internal filters otherwise.
- Enforces size gates; logs top-N largest retained files.
- Redacts credentials from logs; no telemetry.

------

## 10) Automation

**Systemd user units (example)**

```
~/.config/systemd/user/repo-harvest@.service
[Unit]
Description=Repo Harvest sync for profile %i

[Service]
Type=oneshot
Environment=RUST_LOG=repo_harvest=info
ExecStart=%h/.local/bin/repo-harvest sync %i --make-context --format human
~/.config/systemd/user/repo-harvest@.timer
[Unit]
Description=Hourly Repo Harvest sync for profile %i

[Timer]
OnCalendar=hourly
Persistent=true

[Install]
WantedBy=timers.target
```

Enable for a profile:

```bash
systemctl --user enable --now repo-harvest@octobot.timer
```

------

## 11) Examples

**Initialize and sync a profile for a monorepo subdir**

```bash
repo-harvest init custom-octobot \
  --repo c1nderscript/custom-octobot \
  --subpath STRIDE-Tentacle \
  --include 'src/**' --include 'README*' --include 'docs/**' \
  --exclude 'target/**' 'node_modules/**' --make-context

repo-harvest plan custom-octobot   # show what changed since last
repo-harvest sync custom-octobot   # pull latest with same rules
```

**Produce a compact helper pack for evaluators**

```bash
repo-harvest init evaluators \
  --repo Drakkar-Software/OctoBot-Evaluators \
  --include 'octobot_evaluators/**' --include 'README*' --include 'docs/**' \
  --exclude '**/tests/**' --exclude '**/*.png' --make-context
```

------

## 12) Versioning & Releases

- Semantic Versioning.
- `CHANGELOG.md` with Keep a Changelog categories.
- Tag `vX.Y.Z`; attach Linux x86_64 binary.

------

## 13) Troubleshooting (Arch)

- **403 rate limit**: set `GITHUB_TOKEN`.
- **OpenSSL link errors**: ensure rustls build; disable native TLS features.
- **Linker missing**: `sudo pacman -S --needed base-devel`.
- **Huge repo**: tighten includes/excludes; raise `--max-size` carefully.

------

## Appendix A — Minimal `Cargo.toml`

```toml
[package]
name = "repo-harvest"
version = "0.1.0"
edition = "2021"
license = "MIT"

[dependencies]
anyhow = "1"
clap = { version = "4", features = ["derive"] }
globset = "0.4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
camino = "1"
sha2 = "0.10"
hex = "0.4"
time = { version = "0.3", features = ["formatting"] }
reqwest = { version = "0.12", default-features = false, features = ["json", "rustls-tls"] }
tokio   = { version = "1", features = ["rt-multi-thread", "macros"] }
tar = "0.4"
flate2 = "1"
walkdir = "2"
```

------

## Appendix B — Profile example (`~/.config/repo-harvest/profiles/octobot.toml`)

```toml
name = "octobot"
repo = "Drakkar-Software/OctoBot"
ref = "main"
out_dir = "/home/cinder/harvests"
subpath = ""
max_size = 20
make_context = true
include = [
  "src/**", "lib/**", "include/**",
  "docs/**", "README*", "CHANGELOG*", "LICENSE*",
  "CONTRIBUTING*", "SECURITY*",
  "Cargo.toml", "pyproject.toml", "package.json",
  "Dockerfile*", "docker-compose*.yml", "Makefile*"
]
exclude = [
  "node_modules/**", "target/**", ".venv/**",
  "dist/**", "build/**", "**/fixtures/**",
  "**/*.bin", "**/*.mp4", "**/*.zip"
]
```

------

## Appendix C — Manifest schema (excerpt)

```json
{
  "schema": 1,
  "spec": {
    "repo": "owner/repo",
    "ref": "main",
    "resolved_sha": "abc123...",
    "generated_at": "2025-08-17T09:00:00Z",
    "profile": "octobot"
  },
  "filters": {
    "include": ["src/**", "docs/**"],
    "exclude": ["node_modules/**"]
  },
  "files": [
    {"path": "src/main.rs", "bytes": 1234, "sha256": "..."}
  ],
  "stats": { "file_count": 1, "total_bytes": 1234 }
}
```
