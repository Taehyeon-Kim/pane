# Epic 1: Core Infrastructure & CLI Foundation

## Epic Goal
Establish the foundational Rust project structure, basic CLI entry point, configuration system, and skill manifest data structures that will support the TUI launcher and skill execution runtime.

## Epic Scope
- Rust project scaffolding with proper module organization
- Single command CLI (`pane`) with basic flags
- Configuration file loading and management
- Skill manifest format definition and parsing

## Success Criteria
- Project compiles successfully to a single binary
- `pane --version` and `pane --help` work correctly
- Configuration can be loaded from standard locations
- Skill manifests can be parsed and validated

---

## Story 1.1: Project Scaffolding & Basic CLI Entry Point

### Story Statement
**As a** developer,
**I want** a properly scaffolded Rust project with a working CLI entry point,
**so that** I can build upon a solid foundation with proper dependencies and project structure.

### Acceptance Criteria
1. Rust project is initialized with proper Cargo.toml configuration
2. Project name is "pane" with appropriate metadata (version, authors, description)
3. Running `pane --version` displays the version number
4. Running `pane --help` displays usage information including available flags
5. Running `pane` without arguments exits cleanly with a helpful message (TUI not yet implemented)
6. Project compiles without errors or warnings
7. Basic error handling exists for unknown flags
8. Dependencies for CLI parsing (clap) are configured

### Technical Notes
- Use Rust 2021 edition
- Use clap for CLI argument parsing
- Follow Rust project conventions for directory structure
- Set up for single binary output

---

## Story 1.2: Configuration System Foundation

### Story Statement
**As a** user,
**I want** pane to load configuration from standard locations,
**so that** I can customize behavior and skill discovery paths.

### Acceptance Criteria
1. Config file format is defined using TOML
2. Config file can be loaded from `~/.config/pane/config.toml`
3. Default configuration values are used when config file doesn't exist
4. Configuration struct includes skill discovery paths (system, user, project)
5. Configuration loading errors are handled gracefully with helpful messages
6. Config can be validated and logged at debug level

### Technical Notes
- Use serde and toml crate for config parsing
- Store config in `~/.config/pane/config.toml` (XDG Base Directory spec)
- Support environment variable override for config location
- Include skill_paths in config structure

---

## Story 1.3: Skill Manifest Data Structures & Parser

### Story Statement
**As a** developer,
**I want** skill manifest format defined with parsing and validation,
**so that** skills can be discovered and loaded from YAML files.

### Acceptance Criteria
1. `SkillManifest` struct is defined with all required fields (id, name, description, exec, ui.mode)
2. Optional fields are supported (version, args, tags, estimated_time, ui.fullscreen, context.*)
3. YAML parser can load valid `pane-skill.yaml` files into SkillManifest struct
4. Invalid manifests are rejected with clear error messages
5. Validation ensures required fields are present
6. Unit tests exist for manifest parsing (valid and invalid cases)

### Technical Notes
- Use serde and serde_yaml for manifest parsing
- Define nested structs for `ui` and `context` sections
- Implement validation in the manifest struct
- Store manifest structs in `src/skill/manifest.rs`

---

## Story 1.4: Skill Discovery System

### Story Statement
**As a** user,
**I want** pane to discover skills from project, user, and system locations,
**so that** I can access skills from multiple sources with proper precedence.

### Acceptance Criteria
1. Skill discovery searches in order: project (`./.pane/skills/`), user (`~/.config/pane/skills/`), system (`/usr/local/share/pane/skills/`)
2. Discovery recursively finds all `pane-skill.yaml` files in each location
3. Skills with duplicate IDs are resolved by precedence (project > user > system)
4. Discovery errors (missing directories, unreadable files) are logged but don't crash the application
5. Discovered skills are returned as a collection of SkillManifest objects
6. Unit tests cover discovery from multiple locations and precedence rules

### Technical Notes
- Use walkdir crate for recursive directory traversal
- Implement skill loading in `src/skill/discovery.rs`
- Cache discovered skills for performance
- Handle missing directories gracefully (not all locations must exist)
