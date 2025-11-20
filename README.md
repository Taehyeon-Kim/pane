# Pane - Terminal Skill Launcher

A fast, keyboard-driven TUI launcher for terminal skills and micro-tools.

## Features

- **Dual Execution Modes**: TUI mode for interactive tools, inline mode for quick commands
- **Fuzzy Search**: Quick skill discovery with fuzzy matching
- **Skill Discovery**: Three-tier loading from bundled, user, and project locations
- **Output Display**: View inline skill output directly in the launcher
- **Recent Skills**: Track and quickly access recently executed skills
- **Favorites**: Mark and filter your most-used skills
- **Fast Startup**: Sub-100ms launch time for instant access

## Installation

### Homebrew (Recommended for macOS/Linux)

Install Pane via Homebrew:

```bash
brew tap Taehyeon-Kim/pane
brew install pane
```

Or install directly without tapping:

```bash
brew install Taehyeon-Kim/pane/pane
```

After installation, run:

```bash
pane
```

### Prerequisites (for source installation)

- Rust toolchain (1.75+ stable)
- macOS or Linux operating system

### Installation from Source

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/pane.git
   cd pane
   ```

2. **Build release binaries:**
   ```bash
   cargo build --release
   ```

3. **Install using the installation script:**

   **System-wide installation** (requires sudo):
   ```bash
   sudo ./scripts/install.sh
   ```

   **User installation** (no sudo required):
   ```bash
   PREFIX=$HOME/.local ./scripts/install.sh
   ```

   **Custom installation prefix:**
   ```bash
   PREFIX=/custom/path ./scripts/install.sh
   ```

   **Preview installation** (dry-run mode):
   ```bash
   ./scripts/install.sh --dry-run
   ```

4. **Ensure installation path is in your PATH:**

   For user installations, add to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):
   ```bash
   export PATH="$HOME/.local/bin:$PATH"
   ```

### What Gets Installed

- **Binaries:**
  - `pane` - Main launcher executable
  - `claude-tips` - Bundled Claude Code Tips Viewer skill

- **Bundled Skills:**
  - Claude Code Tips skill with 17 curated tips
  - Skill manifest and data files

- **Installation Locations:**
  - Binaries: `$PREFIX/bin/` (default: `/usr/local/bin/`)
  - Skills: `$PREFIX/share/pane/skills/` (default: `/usr/local/share/pane/skills/`)

### Uninstallation

To remove Pane and all bundled skills:

```bash
# Interactive uninstall (with confirmation)
sudo ./scripts/uninstall.sh

# Automatic uninstall (no confirmation)
sudo ./scripts/uninstall.sh --yes

# Preview uninstallation
./scripts/uninstall.sh --dry-run

# Uninstall from custom prefix
PREFIX=$HOME/.local ./scripts/uninstall.sh
```

### Homebrew Installation

Coming soon in v1.0 release:
```bash
brew install pane
```

### Bundled Skills

Pane comes with bundled skills that are installed automatically:

#### Claude Code Tips

A curated collection of tips and best practices for working with Claude Code.

**Installation Details:**
- Binary: `claude-tips` (installed to `/usr/local/bin/` or system PATH)
- Manifest: `/usr/local/share/pane/skills/claude-tips/pane-skill.yaml`
- Data: `/usr/local/share/pane/skills/claude-tips/data/claude-tips.yaml`

**Standalone Usage:**
The `claude-tips` binary can be run independently of Pane:
```bash
# For development (from project root):
cd skills/claude-tips && cargo run --release

# After installation (system-wide):
claude-tips
```

**Note:** During development, run from the `skills/claude-tips` directory so the binary can locate the data file at `data/claude-tips.yaml`.

**From Pane Launcher:**
Launch Pane and search for "claude" or "tips" to find and execute the skill interactively.

**Contributing New Tips:**
The Claude Code Tips skill includes a curated collection of tips that can be expanded by contributors. To add new tips:

1. Review the [Tips Authoring Guide](skills/claude-tips/AUTHORING_GUIDE.md) for format and quality guidelines
2. Edit `skills/claude-tips/data/claude-tips.yaml` to add your tip following the required schema
3. Test your changes:
   ```bash
   # Run tip parser tests
   cargo test --package claude-tips

   # Run E2E integration tests
   cargo test --package pane --test claude_tips_e2e

   # Test in the TUI
   cargo run --package claude-tips --release
   ```
4. Verify your tip displays correctly in various terminal sizes (80, 120, and 60 columns)
5. Submit a pull request with your new tip

See the [Tips Authoring Guide](skills/claude-tips/AUTHORING_GUIDE.md) for detailed instructions on:
- Tip YAML format and required fields
- Writing guidelines (clear titles, concise text, terminal-friendly formatting)
- Category descriptions (prompting, cost, workflow, debugging, best-practices)
- Validation rules and testing procedures
- Examples of good vs poor tips

## Usage

### Basic Workflow

1. Launch Pane: `pane`
2. Search for skills using fuzzy search (just start typing)
3. Navigate with ↑/↓ or j/k
4. Press Enter to execute the selected skill
5. Press Esc to close output panel (inline mode) or quit (skill list)

### Keyboard Shortcuts

**Skill List Navigation:**
- `↑/↓` or `j/k` - Move selection up/down
- `PageUp/PageDown` - Jump by page
- `Tab` - Cycle view modes (All/Favorites/Recent)
- `Enter` - Execute selected skill
- `Esc` - Quit application

**Output Panel (Inline Mode):**
- `↑/↓` or `j/k` - Scroll output up/down
- `Esc` - Close output panel and return to skill list

## Skill Execution Modes

Pane supports two skill execution modes:

### TUI Mode (Default)

Skills with `ui.mode: tui` take over the terminal completely, ideal for interactive applications.

**Use cases:**
- Full-screen TUI applications
- Interactive prompts and editors
- Complex workflows requiring full terminal control

**Example:**
```yaml
id: claude-tips
name: Claude Code Tips
ui:
  mode: tui
  fullscreen: true
```

When executed, the launcher suspends and the skill takes full terminal control. After the skill exits, the launcher automatically restores.

### Inline Mode

Skills with `ui.mode: inline` execute with output captured and displayed in a panel within the launcher.

**Use cases:**
- Quick status checks (git status, system info)
- Non-interactive utilities
- Short command outputs that fit in a panel

**Example:**
```yaml
id: git-status-inline
name: Git Status
ui:
  mode: inline
  fullscreen: false
exec: git
args:
  - status
  - --short
```

When executed, output appears in a scrollable panel. Press Esc to dismiss and return to the skill list.

## Creating Skills

### Skill Manifest Structure

Create a `pane-skill.yaml` file in any of these locations:
- **Bundled**: `skills/` (packaged with Pane)
- **User**: `~/.pane/skills/` (user-wide skills)
- **Project**: `.pane/skills/` (project-specific skills)

### Minimal Skill Example

```yaml
id: my-skill
name: My Skill
description: A simple example skill
version: 1.0.0
exec: echo
args:
  - "Hello from my skill!"
ui:
  mode: inline
tags:
  - example
  - utility
```

### Inline Mode Best Practices

**When to use inline mode:**
- Output is reasonably sized (<10MB)
- Skill is non-interactive
- Quick execution time (<few seconds typical)
- Output is useful to review without leaving the launcher

**Inline mode features:**
- Output captured to stdout and stderr
- 10MB size limit (truncation warning if exceeded)
- Automatic panel display after execution
- Scrollable output with visual indicators
- Execution time and exit code display
- Error output highlighted separately

**Example inline skills:**

**Git Status:**
```yaml
id: git-status-inline
name: Git Status
description: Show current git repository status inline
exec: git
args: [status, --short]
ui:
  mode: inline
context:
  pass_cwd: true
  pass_git_root: true
```

**System Info:**
```yaml
id: system-info
name: System Info
description: Display system information
exec: ./system-info.sh
ui:
  mode: inline
tags: [system, monitoring]
```

### Context Variables

Skills can request environment variables:

```yaml
context:
  pass_cwd: true           # PANE_CWD - current working directory
  pass_git_root: true      # PANE_GIT_ROOT - git repository root
  pass_git_branch: true    # PANE_GIT_BRANCH - current branch
  pass_project_root: true  # PANE_PROJECT_ROOT - project root
```

All skills automatically receive:
- `PANE_ID` - Skill ID from manifest
- `PANE_NAME` - Skill name from manifest

## Troubleshooting

### Inline Mode Issues

**Output not appearing:**
- Verify `ui.mode: inline` is set in manifest
- Check executable path is correct
- Ensure executable has proper permissions

**Output truncated:**
- Skill produces >10MB output
- Consider filtering output in the skill itself
- Or switch to TUI mode for large outputs

**Execution errors:**
- Check error output in the output panel
- Verify context variables are correct
- Test skill execution manually: `./skill-exec args...`

### General Issues

**Skill not found:**
- Verify manifest is in a valid location
- Check manifest filename is `pane-skill.yaml`
- Ensure YAML syntax is valid

**Performance issues:**
- Check skill discovery locations
- Reduce number of skills if necessary
- Use fuzzy search to filter quickly

## Building from Source

### Prerequisites

- Rust 1.75+ (stable channel)
- Git (for build metadata)
- tar (for archive creation)
- sha256sum or shasum (for checksums)
- **Optional**: `cross` tool for cross-compilation (`cargo install cross`)

### Build Release Binaries

Pane includes an automated build script for creating optimized release binaries:

```bash
# Build for host platform (auto-detected)
./scripts/build-release.sh

# Build for specific target platform
./scripts/build-release.sh --target x86_64-apple-darwin
./scripts/build-release.sh --target aarch64-apple-darwin
./scripts/build-release.sh --target x86_64-unknown-linux-gnu

# Clean build from scratch
./scripts/build-release.sh --clean

# Preview build actions without executing
./scripts/build-release.sh --dry-run
```

### Build Script Features

The `build-release.sh` script:
- Builds workspace binaries (`pane` and `claude-tips`) with release optimizations
- Organizes artifacts in `dist/v<version>/<target>/` directory structure
- Creates compressed tar.gz archives
- Generates SHA256 checksum files for verification
- Creates build metadata JSON with version, git commit, and build date
- Supports cross-compilation to macOS (x86_64, ARM64) and Linux (x86_64)
- Compatible with CI/CD environments (GitHub Actions)

### Build Outputs

After running the build script, artifacts are organized in `dist/`:

```
dist/
└── v0.1.0/
    ├── aarch64-apple-darwin/
    │   ├── pane
    │   └── claude-tips
    ├── pane-v0.1.0-aarch64-apple-darwin.tar.gz
    ├── pane-v0.1.0-aarch64-apple-darwin.tar.gz.sha256
    └── build-metadata.json
```

### Cross-Compilation

**For same-OS cross-compilation (e.g., x86_64 to ARM64 on macOS):**
```bash
rustup target add aarch64-apple-darwin
./scripts/build-release.sh --target aarch64-apple-darwin
```

**For cross-platform builds (e.g., Linux from macOS):**
```bash
cargo install cross
./scripts/build-release.sh --target x86_64-unknown-linux-gnu
```

### CI/CD Integration

The build script is designed for GitHub Actions workflows:

```yaml
- name: Build release binaries
  run: ./scripts/build-release.sh --target ${{ matrix.target }}

- name: Upload artifacts
  uses: actions/upload-artifact@v3
  with:
    name: release-binaries
    path: dist/v${{ github.ref_name }}/*.tar.gz
```

## Development

```bash
# Build for development
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Lint
cargo clippy -- -D warnings
```

## Architecture

- **TUI Framework**: ratatui 0.26.0
- **Terminal Backend**: crossterm 0.27.0
- **Fuzzy Matching**: nucleo 0.2.0
- **Language**: Rust 1.75+ (stable)
- **Local-only**: No network, cloud, or external dependencies

## License

MIT OR Apache-2.0

## Contributing

Contributions welcome! Please see CONTRIBUTING.md for guidelines.

## Roadmap

- [ ] Homebrew distribution
- [ ] Plugin system for custom skill sources
- [ ] ANSI color support in output panel
- [ ] Skill marketplace/registry
- [ ] Cloud sync for favorites/recent (opt-in)
