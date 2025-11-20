# 4. Technical Specification (Deeper)

## 4.1 Language & Libraries

- **Language**: Rust
- **Terminal backend**: `crossterm`
- **TUI framework**: `ratatui`
- **Config/manifest parsing**: `serde`, `serde_yaml`
- **Fuzzy search**: `fuzzy-matcher` (e.g., `skim`-style) or a lightweight custom scorer
- **Process management**: Rust `std::process::Command`

## 4.2 Core Modules

Proposed Rust module layout:

```text
src/
  main.rs
  app.rs            # main event loop & high-level state
  ui/
    mod.rs
    home.rs         # launcher screen
    components.rs   # reusable widgets
  skills/
    mod.rs
    loader.rs       # discover and parse manifests
    model.rs        # skill struct definitions
    runner.rs       # spawn child processes
  config.rs         # global config handling
  context.rs        # context collection (cwd, git root, etc.)
  input.rs          # key & mouse handling
  logging.rs        # (optional) debug logging
```

### 4.2.1 `app.rs`

- Maintains global `AppState`:
  - current screen (launcher, settings, etc.)
  - list of loaded skills
  - current search query
  - selection index
  - favorites & recent skills (in-memory, optionally persisted)
- Handles main loop:
  - Poll input
  - Update state
  - Render via UI layer

### 4.2.2 `skills/loader.rs`

- Recursively scans the three skill roots:
  - system, user, project
- Finds `pane-skill.yaml` files.
- Parses into `Skill` structs.
- Applies override rules by id (project > user > system).
- Validates critical fields (id, name, exec, ui mode).

### 4.2.3 `skills/runner.rs`

- Given a `Skill`, builds a `Command`:
  - `Command::new(skill.exec)` + `skill.args`
  - Sets env vars from `context` flags.
- For `ui.mode = "tui"`:
  - Drops to raw mode off (if needed).
  - Spawns the child.
  - Waits for completion.
  - Re-initializes TUI.
- For `inline`:
  - Captures stdout and shows it within launcher.

## 4.3 Config Handling

- Global config file (optional for MVP):

  - Path: `~/.config/pane/config.toml`
  - Content examples:
    - Default skill view mode (all/favorites/recent)
    - Whether to enable mouse support
    - Theme options (colors, light/dark)
    - Experimental flags (for future features)

- Config is:
  - Loaded once on startup.
  - Passed down to modules (read-only in MVP).

## 4.4 Error Handling & Logging

- Graceful handling when:
  - No skills are found → show friendly message + hint where to put skills.
  - A skill manifest is invalid → skip with warning (optional debug log).
  - A skill fails to start (exec not found) → show error toast in launcher.

- Logging strategy:
  - By default, minimal/no logging to stdout (to keep TUI clean).
  - Optional `PANE_DEBUG=1` environment variable:
    - Writes debug logs to a file: `~/.config/pane/logs/pane-debug.log`.

## 4.5 Performance & Constraints

- Target startup time:
  - < 100 ms for normal user setups (with a moderate number of skills).
- Skill discovery:
  - Cached per session; no need for file system watch in MVP.
- Memory:
  - Keep structs simple, no heavy caches.

---
