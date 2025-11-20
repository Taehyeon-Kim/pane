# Tech Stack

## Cloud Infrastructure

**N/A** – Pane is a **local-only, offline-first terminal application**. No cloud services, databases, or external APIs are required for core functionality. All data (skills, config, favorites) is stored locally on the user's file system.

## Technology Stack Table

| Category | Technology | Version | Purpose | Rationale |
|----------|-----------|---------|---------|-----------|
| **Language** | Rust | 1.75+ (stable) | Primary development language | Memory safety, zero-cost abstractions, excellent TUI ecosystem, single binary compilation, <100ms startup achievable |
| **Build System** | Cargo | 1.75+ | Build tool and package manager | Standard Rust tooling, handles dependencies, testing, and compilation seamlessly |
| **TUI Framework** | `ratatui` | 0.26.0 | Terminal user interface framework | Active fork of `tui-rs`, excellent widget system, production-ready, strong community, PRD-specified |
| **Terminal Backend** | `crossterm` | 0.27.0 | Cross-platform terminal manipulation | Works on macOS/Linux/Windows, event-driven input handling, mouse support, PRD-specified |
| **Serialization** | `serde` | 1.0 | Serialization/deserialization framework | Industry standard for Rust, zero-cost abstractions, excellent derive macros |
| **YAML Parsing** | `serde_yaml` | 0.9.0 | Skill manifest parsing | Human-readable manifests, good error messages, serde integration |
| **TOML Parsing** | `toml` | 0.8.0 | Config file parsing | User-friendly config format, excellent Rust support, XDG standard |
| **Fuzzy Matching** | `nucleo` | 0.2.0 | Fuzzy search for skill filtering | Fastest fuzzy matcher in Rust ecosystem (used by Helix editor), beats `sublime_fuzzy` and `fuzzy-matcher` |
| **Process Management** | `std::process::Command` | stdlib | Skill execution | Built-in Rust standard library, no extra dependencies, Unix-native |
| **Error Handling** | `anyhow` | 1.0 | Application-level error handling | Context-rich errors, simple API, perfect for CLI applications |
| **CLI Parsing** | `clap` | 4.5.0 | Command-line argument parsing | Derive macros for `--help`, `--version`, future extensibility for debug flags |
| **Testing** | `cargo test` + `rstest` | 0.18.0 | Unit and integration testing | Native Rust testing with parametric tests via `rstest`, no need for heavy frameworks |
| **Logging** | `tracing` | 0.1 | Structured logging | Optional debug logging to file, minimal overhead when disabled, excellent performance |
| **Git Detection** | `git2` | 0.18.0 | Detect git root for context | Libgit2 bindings, reliable git repository detection for `PANE_GIT_ROOT` |
| **Code Formatting** | `rustfmt` | stdlib | Code formatting | Enforced in CI, ensures consistent code style |
| **Linting** | `clippy` | stdlib | Static analysis and linting | Catches common mistakes and enforces best practices |
| **Distribution** | Homebrew + Cargo | N/A | Package distribution | Primary: Homebrew for macOS/Linux; Secondary: `cargo install` for Rust users |
| **CI/CD** | GitHub Actions | N/A | Continuous integration | Free for open source, excellent Rust support, cross-platform builds |
