# Pane – Product Requirements & Technical Specification

## 0. Overview

Pane is a terminal-native utility launcher for modern coding and “vibe coding” workflows.

External usage is intentionally minimal:

```bash
pane
```

This launches a full-screen TUI where users can:

- Search small utilities (“skills”)
- See descriptions, tags, and estimated duration
- Run a selected skill inside the same terminal pane
- Return back to the launcher when the skill exits

The initial flagship skill is **Claude Code Tips Viewer**, which lets users browse a curated archive of Claude Code usage tips inside the terminal.

---

## 1. Goals

### 1.1 Primary Goals

1. Provide a **Raycast-style launcher** inside the terminal with a single entry command (`pane`).
2. Turn idle coding time (builds, tests, deploys, AI calls) into **productive micro-sessions**.
3. Support a **community-driven skill ecosystem** via a simple manifest + executable model.
4. Minimize context switching by keeping utilities inside the terminal pane.

### 1.2 Secondary Goals

1. Be approachable for non-developers who are comfortable launching a terminal app but do not want to memorize complex flags.
2. Start with a strong, personally valuable skill: **Claude Code Tips Viewer**.
3. Ensure that the core is performant, small, and easy to distribute (single binary for macOS/Linux).

---

## 2. User Stories

### 2.1 Developers

- “While my build is running, I want to quickly revisit Claude Code usage tips without opening a browser or notes app.”
- “I want a terminal-native launcher that lets me search and run my small utilities.”
- “I want to pin my most-used tools and access them with a couple of keystrokes.”

### 2.2 Creators / Learners

- “I want to spend short gaps learning something small (tips, language flashcards, notes).”
- “I don't want to maintain a huge GUI app just to access a few micro-tools.”

### 2.3 Community Contributors

- “I want to create and share a new skill by writing a small script and adding a simple manifest file.”
- “I want others to be able to install and run my skill with no extra configuration.”

---

## 3. Feature Requirements (MVP)

### 3.1 Single Command Entry

- Only one main command exposed to the user:

  ```bash
  pane
  ```

- Optional flags:
  - `--version`
  - `--help`
- No subcommands required for regular use (no `pane list`, `pane run`, etc.).
- All interactions happen inside the TUI.

---

### 3.2 TUI Launcher

#### 3.2.1 Layout

- **Header**
  - Title: `Pane`
  - Optional subtitle: “What do you want to do?”

- **Search Bar**
  - Single-line input
  - Always focused by default
  - Typing filters the skills list in real time (fuzzy match on name, id, tags, description)

- **Skill List**
  - Scrollable vertical list
  - Each item shows:
    - Name
    - Tags (inline chip-style text)
    - Estimated time (e.g., `⏱ 1–3 min`)
    - One-line description (truncated if necessary)
    - (Optional later) small icon/emoji

- **Detail / Preview Area**
  - Shows more information for the selected skill:
    - Full description
    - Estimated time
    - Id
    - Tags
    - Source (system / user / project)

- **Footer**
  - Key hints, e.g.:
    - `↑/↓` or `j/k`: move
    - `Enter`: run
    - `Space`: favorite
    - `/` or plain typing: search
    - `Tab`: toggle view (all / favorites / recent)
    - `Esc`: exit

#### 3.2.2 Navigation & Interactions

- **Keyboard**
  - `↑/↓` or `j/k` – navigate the list
  - `PageUp/PageDown` – page scroll
  - `Enter` – execute selected skill
  - `Space` – toggle favorite flag
  - `Tab` – switch list mode:
    - All skills
    - Favorites only
    - Recently used
  - `Esc` – exit pane (or navigate back if future nested screens exist)
  - Text input – any printable key updates search query

- **Mouse (nice-to-have for MVP)**
  - Click on a skill item to select it.
  - Double-click (or click + Enter) to run.
  - Scroll to move list.

---

### 3.3 Skill System

Skills are the core abstraction of Pane. Each skill is:

- A **manifest file** (`pane-skill.yaml`)
- An **executable command** (binary or script)

#### 3.3.1 Manifest Format

File name convention: `pane-skill.yaml`

Example:

```yaml
id: "claude-tips"
name: "Claude Code Tips"
description: "Browse a curated archive of Claude Code usage tips."
version: "0.1.0"

exec: "claude-tips"
args: []
tags: ["tips", "claude", "coding"]
estimated_time: "1–3 min"

ui:
  mode: "tui"          # tui | inline
  fullscreen: true

context:
  pass_cwd: true
  pass_git_root: true
  pass_project_name: true
```

##### Required fields

- `id` (string)
  - Unique identifier, used internally and for analytics.
- `name` (string)
  - Human-readable display name.
- `description` (string)
  - One or two sentences explaining what the skill does.
- `exec` (string)
  - The executable or script name to run (must be resolvable via `$PATH` or absolute/relative path).
- `ui.mode` (enum)
  - `tui`: the skill takes over the terminal and draws its own TUI.
  - `inline`: the skill prints to stdout in a non-interactive way, and Pane can embed the results.

##### Optional fields

- `version` (string)
- `args` (array of strings)
- `tags` (array of strings)
- `estimated_time` (string, e.g., `"<1 min"`, `"1–3 min"`)
- `ui.fullscreen` (boolean, default: true)
- `context.pass_*` (booleans) – indicates which context Pane should pass via environment variables or stdin.

#### 3.3.2 Skill Discovery

Pane searches for skills in the following locations **in order of priority**:

1. Project-specific skills:
   - `./.pane/skills/**/pane-skill.yaml`
2. User-level skills:
   - `~/.config/pane/skills/**/pane-skill.yaml`
3. System/bundled skills:
   - `/usr/local/share/pane/skills/**/pane-skill.yaml`  
     (or similar, depending on installation prefix)

If multiple manifests share the same `id`, the nearer scope overrides:

- Project > User > System

---

### 3.4 Skill Execution Runtime

#### 3.4.1 Process Model

- Pane spawns the skill as a child process using the `exec` + `args` fields.
- Working directory:
  - Default: the cwd where `pane` was launched.
  - Can be overridden in the future but not needed for MVP.

#### 3.4.2 Context Passing

Pane passes context in two ways:

1. **Environment Variables**
   - `PANE_ID` – skill id
   - `PANE_NAME` – skill name
   - `PANE_CWD` – current working directory
   - `PANE_GIT_ROOT` – git repository root (if detected)
   - `PANE_PROJECT_NAME` – project/repo name (if detected)
   - `PANE_CONFIG` – path to main config file (e.g., `~/.config/pane/config.toml`)

2. **Stdin JSON (optional for MVP but recommended)**
   - When `context` is enabled in the manifest, Pane can also send a JSON payload to stdin:

   ```json
   {
     "id": "claude-tips",
     "name": "Claude Code Tips",
     "cwd": "/Users/you/dev/project",
     "git_root": "/Users/you/dev/project",
     "project_name": "project",
     "args": []
   }
   ```

- Skills are free to ignore this context if they don’t need it.

#### 3.4.3 UI Behavior

- For `ui.mode = "tui"`:
  - Pane **temporarily suspends its own TUI**.
  - Clears screen (optional; may leave to child).
  - Hands the terminal over to the child skill.
  - When the child exits, Pane restores its TUI and redraws the launcher.

- For `ui.mode = "inline"`:
  - Pane can:
    - Clear the main area and display the child’s stdout, or
    - Show the output in a modal/secondary panel.
  - After the child exits, Pane remains in launcher mode.

---

### 3.5 Claude Code Tips Viewer (Bundled Skill)

#### 3.5.1 Purpose

The first highly practical, dogfooded skill that:

- Displays a personal or shared archive of “Claude Code usage tips”.
- Replaces the need to open a browser or a note app during coding.
- Demonstrates the skill model and TUI integration.

#### 3.5.2 Behavior

- Data source:
  - Local YAML or JSON file containing tips, e.g., `claude-tips.yaml`.
- Features:
  - Searchable list of tips (title + tags).
  - Short/long description view.
  - Scrollable content with basic formatting.
- Navigation:
  - `↑/↓` or `j/k` – move between tips
  - `Enter` – open / toggle detail view
  - `/` – filter by keyword
  - `Esc` – close detail / exit skill

#### 3.5.3 Data Format (example)

```yaml
- id: "cc-001"
  title: "Avoid vague 'do it all' prompts"
  category: "prompting"
  text: |
    Instead of asking Claude Code to "just build everything",
    give it:
    1) context
    2) current state
    3) desired change
    4) constraints.
  tags: ["claude", "prompting", "workflow"]

- id: "cc-002"
  title: "Use diffs for small changes"
  category: "cost"
  text: |
    For small changes, send diffs instead of entire files. This
    reduces tokens and avoids losing context.
  tags: ["claude", "cost", "diff"]
```

---

## 4. Technical Specification (Deeper)

### 4.1 Language & Libraries

- **Language**: Rust
- **Terminal backend**: `crossterm`
- **TUI framework**: `ratatui`
- **Config/manifest parsing**: `serde`, `serde_yaml`
- **Fuzzy search**: `fuzzy-matcher` (e.g., `skim`-style) or a lightweight custom scorer
- **Process management**: Rust `std::process::Command`

### 4.2 Core Modules

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

#### 4.2.1 `app.rs`

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

#### 4.2.2 `skills/loader.rs`

- Recursively scans the three skill roots:
  - system, user, project
- Finds `pane-skill.yaml` files.
- Parses into `Skill` structs.
- Applies override rules by id (project > user > system).
- Validates critical fields (id, name, exec, ui mode).

#### 4.2.3 `skills/runner.rs`

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

### 4.3 Config Handling

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

### 4.4 Error Handling & Logging

- Graceful handling when:
  - No skills are found → show friendly message + hint where to put skills.
  - A skill manifest is invalid → skip with warning (optional debug log).
  - A skill fails to start (exec not found) → show error toast in launcher.

- Logging strategy:
  - By default, minimal/no logging to stdout (to keep TUI clean).
  - Optional `PANE_DEBUG=1` environment variable:
    - Writes debug logs to a file: `~/.config/pane/logs/pane-debug.log`.

### 4.5 Performance & Constraints

- Target startup time:
  - < 100 ms for normal user setups (with a moderate number of skills).
- Skill discovery:
  - Cached per session; no need for file system watch in MVP.
- Memory:
  - Keep structs simple, no heavy caches.

---

## 5. Non-Goals (MVP)

- No background daemon or long-running agent.
- No network-based skill registry (only local manifests).
- No advanced layout engine or multi-pane orchestration inside Pane itself.
- No built-in AI inference; individual skills may call AI APIs on their own.
- No configuration UI (config file only, for now).

---

## 6. Success Criteria

### Qualitative

- The author finds that **Claude Code Tips Viewer** is more convenient than opening Threads/notes during coding.
- Early users feel that Pane “fills the idle time” in a pleasant, lightweight way.
- Contributors can add at least one skill in under 10–15 minutes.

### Quantitative (longer term)

- Typical user launches `pane` multiple times per day.
- Skill executions per day per active user ≥ 10.
- At least 3 community-created skills within the first iteration of opening the format.

---

## 7. Roadmap (High-Level)

### Phase 1 – MVP

- Core CLI + TUI launcher
- Skill manifest loader
- Skill runner
- Bundled Claude Code Tips Viewer skill
- Simple config file
- Basic packaging (single binary, Homebrew tap)

### Phase 2 – Early Ecosystem

- Favorites/recent persistence
- Simple “pane doctor” skill to check setup
- Documentation for creating skills
- Example skill templates (Node, Python, Rust)

### Phase 3 – Community & Integrations

- Git-based skill registry pattern
- Editor awareness (integration hints for Cursor/Zed/VSCode)
- Optional AI-powered skills (log analyzer, daily log writer, micro-learning)
