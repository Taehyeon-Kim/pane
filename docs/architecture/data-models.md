# Data Models

## Skill

**Purpose:** Represents a discoverable skill with its manifest metadata and execution configuration.

**Key Attributes:**
- `id: String` – Unique identifier for the skill (e.g., "claude-tips")
- `name: String` – Human-readable display name (e.g., "Claude Code Tips")
- `description: String` – One or two sentence explanation of what the skill does
- `version: String` – Semantic version of the skill
- `exec: String` – Executable or script name/path to run
- `args: Vec<String>` – Command-line arguments to pass to the executable
- `tags: Vec<String>` – Searchable tags for filtering (e.g., ["tips", "claude", "coding"])
- `estimated_time: Option<String>` – Human-readable time estimate (e.g., "1–3 min")
- `ui_mode: UiMode` – Enum: `Tui` or `Inline`
- `ui_fullscreen: bool` – Whether skill uses fullscreen mode (default: true)
- `context_config: ContextConfig` – Which context fields to pass to the skill
- `source: SkillSource` – Enum: `System`, `User`, or `Project`
- `manifest_path: PathBuf` – Path to the source `pane-skill.yaml` file

**Relationships:**
- Owned by `SkillRegistry` collection
- Referenced by `AppState` for current selection
- Used by `SkillRunner` for execution

## AppState

**Purpose:** Central application state for the TUI launcher, managing UI state, loaded skills, and user interactions.

**Key Attributes:**
- `skills: Vec<Skill>` – All discovered skills after loading and deduplication
- `filtered_skills: Vec<usize>` – Indices into `skills` matching current search query
- `selected_index: usize` – Currently selected skill index in `filtered_skills`
- `search_query: String` – Current fuzzy search input
- `view_mode: ViewMode` – Enum: `All`, `Favorites`, `Recent`
- `favorites: HashSet<String>` – Skill IDs marked as favorites
- `recent: Vec<String>` – Recently executed skill IDs (ordered, most recent first)
- `config: Config` – User configuration loaded from TOML
- `should_quit: bool` – Flag to exit the application

**Relationships:**
- Contains collection of `Skill` objects
- References `Config` for user preferences
- Modified by input handlers and event loop
- Used by UI rendering functions

## Config

**Purpose:** User configuration loaded from `~/.config/pane/config.toml` with sensible defaults.

**Key Attributes:**
- `default_view_mode: ViewMode` – Default view on launch (All/Favorites/Recent)
- `enable_mouse: bool` – Whether to enable mouse support (default: true)
- `theme: Option<ThemeConfig>` – Optional theme customization (colors, styles)
- `max_recent_skills: usize` – Maximum number of recent skills to track (default: 10)
- `debug_log_enabled: bool` – Whether to enable debug logging to file (default: false)
- `debug_log_path: PathBuf` – Path to debug log file (default: `~/.config/pane/logs/pane-debug.log`)

**Relationships:**
- Loaded once at startup
- Owned by `AppState`
- Serialized/deserialized via `serde` + `toml`

## SkillContext

**Purpose:** Context data passed to skills via environment variables and optional stdin JSON.

**Key Attributes:**
- `skill_id: String` – ID of the skill being executed
- `skill_name: String` – Name of the skill
- `cwd: PathBuf` – Current working directory where `pane` was launched
- `git_root: Option<PathBuf>` – Git repository root (if detected)
- `project_name: Option<String>` – Project/repo name derived from git root or cwd
- `config_path: PathBuf` – Path to Pane's config file
- `args: Vec<String>` – Additional arguments passed to the skill

**Relationships:**
- Created by `SkillRunner` before skill execution
- Derived from `AppState` and system inspection (git detection)
- Passed to child process via `PANE_*` environment variables
- Optionally serialized to JSON and sent to skill's stdin

## Supporting Enums and Structs

**UiMode (Enum)** – Defines how a skill interacts with the terminal
- `Tui` – Skill takes over the terminal with its own TUI
- `Inline` – Skill prints to stdout, results embedded in launcher

**ViewMode (Enum)** – Defines which skills are displayed in the launcher list
- `All` – Show all discovered skills
- `Favorites` – Show only favorited skills
- `Recent` – Show only recently executed skills

**SkillSource (Enum)** – Indicates where a skill was discovered (for override precedence)
- `System` – Discovered in `/usr/local/share/pane/skills/`
- `User` – Discovered in `~/.config/pane/skills/`
- `Project` – Discovered in `./.pane/skills/`
- Precedence: Project > User > System (for ID collision resolution)

**ContextConfig (Struct)** – Configuration for which context fields to pass to a skill
- `pass_cwd: bool` – Whether to pass current working directory
- `pass_git_root: bool` – Whether to detect and pass git root
- `pass_project_name: bool` – Whether to pass project name
- `pass_stdin_json: bool` – Whether to send full context as JSON to stdin
