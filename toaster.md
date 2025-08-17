# Architecture Overview
```mermaid
flowchart TD
    A[Entry Points] --> B[Core Logic] --> C[External Systems]
    D[codex.sh] --> B
```

**Interfaces**: `repo-harvest` and `finishes` CLI, filesystem, GitHub API
**Critical paths**: build (`cargo build`), testing (`cargo test`), release (`codex.sh bootstrap`)

**Interfaces**: [key integration points]
**Critical paths**: [deployment, testing, release flows]

