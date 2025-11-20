# 3. Feature Requirements (MVP)

## 3.1 Single Command Entry

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

## 3.2 TUI Launcher

### 3.2.1 Layout

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

### 3.2.2 Navigation & Interactions

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

## 3.3 Skill System

Skills are the core abstraction of Pane. Each skill is:

- A **manifest file** (`pane-skill.yaml`)
- An **executable command** (binary or script)

### 3.3.1 Manifest Format

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

#### Required fields

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

#### Optional fields

- `version` (string)
- `args` (array of strings)
- `tags` (array of strings)
- `estimated_time` (string, e.g., `"<1 min"`, `"1–3 min"`)
- `ui.fullscreen` (boolean, default: true)
- `context.pass_*` (booleans) – indicates which context Pane should pass via environment variables or stdin.

### 3.3.2 Skill Discovery

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

## 3.4 Skill Execution Runtime

### 3.4.1 Process Model

- Pane spawns the skill as a child process using the `exec` + `args` fields.
- Working directory:
  - Default: the cwd where `pane` was launched.
  - Can be overridden in the future but not needed for MVP.

### 3.4.2 Context Passing

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

### 3.4.3 UI Behavior

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

## 3.5 Claude Code Tips Viewer (Bundled Skill)

### 3.5.1 Purpose

The first highly practical, dogfooded skill that:

- Displays a personal or shared archive of “Claude Code usage tips”.
- Replaces the need to open a browser or a note app during coding.
- Demonstrates the skill model and TUI integration.

### 3.5.2 Behavior

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

### 3.5.3 Data Format (example)

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
