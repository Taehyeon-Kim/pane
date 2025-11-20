# Source Tree

**Implementation Status Legend**:
- ✅ Implemented and tested
- 🔶 Partially implemented or in progress
- ❌ Not yet implemented (planned in Epic 5)

```
pane/
├── .github/
│   └── workflows/
│       ├── ci.yml                  ❌ Epic 5.4
│       ├── release.yml             ❌ Epic 5.4
│       └── security-audit.yml      ❌ Epic 5.4 (optional)
│
├── src/
│   ├── main.rs                     # Application entry point, CLI parsing
│   ├── app.rs                      # App Orchestrator - event loop & state management
│   │
│   ├── skills/
│   │   ├── mod.rs                  # Skills module exports
│   │   ├── loader.rs               # Skill Loader - discovery & parsing
│   │   ├── runner.rs               # Skill Runner - execution & terminal handoff
│   │   ├── model.rs                # Skill, SkillSource, UiMode, ContextConfig types
│   │   └── manifest.rs             # YAML manifest parsing logic
│   │
│   ├── ui/
│   │   ├── mod.rs                  # UI module exports
│   │   ├── renderer.rs             # UI Renderer - main render function
│   │   ├── components/
│   │   │   ├── mod.rs
│   │   │   ├── header.rs           # Header component (title, subtitle)
│   │   │   ├── search_bar.rs       # Search input component
│   │   │   ├── skill_list.rs       # Scrollable skill list component
│   │   │   ├── detail_pane.rs      # Skill detail viewer component
│   │   │   └── footer.rs           # Key hints footer component
│   │   └── theme.rs                # Theme and styling configuration
│   │
│   ├── input.rs                    # Input Handler - keyboard & mouse events
│   ├── search.rs                   # Fuzzy Matcher - nucleo integration
│   ├── state.rs                    # AppState, ViewMode enums
│   ├── config.rs                   # Config Loader - TOML parsing & persistence
│   ├── context.rs                  # SkillContext - git detection & context gathering
│   ├── logging.rs                  # Logger - tracing setup
│   └── error.rs                    # Custom error types and Result aliases
│
├── skills/
│   └── claude-tips/                🔶 Built, bundling pending Epic 5.5
│       ├── pane-skill.yaml         ✅ Skill manifest
│       ├── Cargo.toml              ✅ Skill's own Rust project
│       ├── src/
│       │   └── main.rs             ✅ Tips viewer TUI implementation
│       └── data/
│           └── claude-tips.yaml    ✅ Tips content database
│
├── tests/
│   ├── integration/
│   │   ├── mod.rs
│   │   ├── skill_discovery_test.rs # Test three-tier skill loading
│   │   ├── search_test.rs          # Test fuzzy matching
│   │   └── config_test.rs          # Test config loading & persistence
│   │
│   └── fixtures/
│       ├── skills/                 # Sample skill manifests for testing
│       └── configs/                # Sample config files for testing
│
├── examples/
│   ├── simple-skill.sh             # Example bash skill
│   ├── python-skill/               # Example Python skill
│   │   ├── pane-skill.yaml
│   │   └── main.py
│   └── rust-skill-template/        # cargo-generate template for Rust skills
│       ├── pane-skill.yaml
│       └── Cargo.toml
│
├── docs/
│   ├── prd.md                      # Product Requirements Document
│   ├── architecture.md             # This architecture document
│   └── skill-development-guide.md  # How to create skills (Phase 2)
│
├── scripts/
│   ├── install.sh                  ❌ Epic 5.1
│   ├── uninstall.sh                ❌ Epic 5.1
│   └── build-release.sh            ❌ Epic 5.2
│
├── .cargo/
│   └── config.toml                 # Cargo build configuration (optimization flags)
│
├── Cargo.toml                      # Project dependencies & metadata
├── Cargo.lock                      # Locked dependency versions
├── rust-toolchain.toml             # Rust version specification
├── .gitignore                      # Git ignore patterns
├── LICENSE                         # MIT or Apache 2.0
├── README.md                       # Project overview & quick start
└── CHANGELOG.md                    # Version history
```
