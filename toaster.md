# Architecture Overview
```mermaid
flowchart TD
    A[Entry Points] --> B[Core Logic] --> C[External Systems]
    D[codex.sh] --> B
```

**Interfaces**: `finishes` CLI, `repo-harvest` crate, filesystem, GitHub API
**Critical paths**: build (`cargo build`), testing (`cargo test`), release (`codex.sh bootstrap`)
