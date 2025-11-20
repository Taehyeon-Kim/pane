# Core Workflows

## Workflow 1: Application Startup & Skill Discovery

```mermaid
sequenceDiagram
    participant User
    participant Main as App Orchestrator
    participant CLI as CLI Parser
    participant ConfigLoader as Config Loader
    participant SkillLoader as Skill Loader
    participant GitDetector as Git Detector
    participant Logger
    participant UIRenderer as UI Renderer
    participant Terminal

    User->>Main: Runs `pane` command
    Main->>CLI: Parse arguments (--help, --version)
    CLI-->>Main: Args validated

    Main->>ConfigLoader: load_config()
    ConfigLoader->>ConfigLoader: Read ~/.config/pane/config.toml
    ConfigLoader-->>Main: Config (or defaults)

    Main->>Logger: init_logger(config)
    Logger-->>Main: Logger initialized (if enabled)

    Main->>SkillLoader: discover_skills()
    SkillLoader->>GitDetector: detect_git_root(cwd)
    GitDetector-->>SkillLoader: Optional<PathBuf>

    loop For each skill directory (Project, User, System)
        SkillLoader->>SkillLoader: Scan for pane-skill.yaml
        SkillLoader->>SkillLoader: Parse manifest with serde_yaml
        SkillLoader->>SkillLoader: Validate required fields
    end

    SkillLoader->>SkillLoader: deduplicate_skills() (Project > User > System)
    SkillLoader-->>Main: Vec<Skill>

    Main->>Main: Initialize AppState with skills & config
    Main->>Terminal: Initialize crossterm + ratatui
    Terminal-->>Main: Ready

    loop Event Loop
        Main->>Terminal: Poll for events
        Main->>UIRenderer: render(frame, state)
        UIRenderer->>Terminal: Draw TUI (list, search, details)
        Terminal-->>User: Display launcher
    end
```

## Workflow 2: User Search & Skill Selection

```mermaid
sequenceDiagram
    participant User
    participant Terminal
    participant InputHandler as Input Handler
    participant FuzzyMatcher as Fuzzy Matcher
    participant AppState as App State
    participant UIRenderer as UI Renderer

    User->>Terminal: Types search query: "clau"
    Terminal->>InputHandler: KeyEvent('c'), KeyEvent('l'), KeyEvent('a'), KeyEvent('u')

    loop For each character
        InputHandler->>AppState: update_search_query("clau")
        InputHandler->>FuzzyMatcher: filter_skills("clau", state.skills)

        FuzzyMatcher->>FuzzyMatcher: Score each skill against query
        Note over FuzzyMatcher: Match against name, id, tags, description
        FuzzyMatcher-->>InputHandler: Vec<usize> (ranked indices)

        InputHandler->>AppState: Update filtered_skills
        InputHandler->>UIRenderer: Trigger re-render
        UIRenderer->>Terminal: Draw updated skill list
        Terminal-->>User: Show filtered results
    end

    User->>Terminal: Press ↓ (Down arrow)
    Terminal->>InputHandler: KeyEvent(Down)
    InputHandler->>AppState: update_selection(Direction::Down)
    AppState->>AppState: selected_index += 1
    InputHandler->>UIRenderer: Trigger re-render
    UIRenderer->>Terminal: Highlight next skill
    Terminal-->>User: Show updated selection

    User->>Terminal: Press Enter
    Terminal->>InputHandler: KeyEvent(Enter)
    InputHandler-->>InputHandler: Return Action::ExecuteSkill
```

## Workflow 3: Skill Execution & Terminal Handoff

```mermaid
sequenceDiagram
    participant User
    participant Main as App Orchestrator
    participant SkillRunner as Skill Runner
    participant GitDetector as Git Detector
    participant Terminal
    participant ChildProcess as Skill Process
    participant ConfigLoader as Config Loader

    User->>Main: Selects skill & presses Enter
    Main->>SkillRunner: execute_skill(skill, create context)

    SkillRunner->>GitDetector: detect_git_root(cwd) (if needed)
    GitDetector-->>SkillRunner: Optional<PathBuf>

    SkillRunner->>SkillRunner: Build SkillContext
    Note over SkillRunner: cwd, git_root, project_name, etc.

    SkillRunner->>SkillRunner: prepare_environment(context)
    Note over SkillRunner: PANE_ID, PANE_CWD, PANE_GIT_ROOT, etc.

    SkillRunner->>Terminal: suspend_tui()
    Terminal->>Terminal: Disable raw mode, clear screen
    Terminal-->>SkillRunner: TUI suspended

    SkillRunner->>ChildProcess: spawn(skill.exec, skill.args, env)
    Note over ChildProcess: Skill executable takes over terminal

    alt UI Mode: TUI
        ChildProcess->>Terminal: Render skill's own TUI
        Terminal-->>User: Display skill interface
        User->>ChildProcess: Interact with skill (keyboard/mouse)
        ChildProcess->>ChildProcess: Run skill logic
    else UI Mode: Inline
        ChildProcess->>ChildProcess: Print to stdout
        ChildProcess-->>SkillRunner: Capture stdout
    end

    ChildProcess->>ChildProcess: Skill completes
    ChildProcess-->>SkillRunner: Exit with status code

    SkillRunner->>Terminal: restore_tui()
    Terminal->>Terminal: Re-enable raw mode, clear screen
    Terminal-->>SkillRunner: TUI restored

    SkillRunner->>SkillRunner: Update recent skills in AppState
    SkillRunner->>ConfigLoader: save_config(updated recent list)
    ConfigLoader->>ConfigLoader: Write to ~/.config/pane/config.toml
    ConfigLoader-->>SkillRunner: Config persisted

    SkillRunner-->>Main: ExitStatus
    Main->>Terminal: Re-render launcher
    Terminal-->>User: Back to skill list
```

## Workflow 4: Favorite Management & View Switching

```mermaid
sequenceDiagram
    participant User
    participant Terminal
    participant InputHandler as Input Handler
    participant AppState as App State
    participant ConfigLoader as Config Loader
    participant UIRenderer as UI Renderer

    User->>Terminal: Press Space (toggle favorite)
    Terminal->>InputHandler: KeyEvent(Space)
    InputHandler->>AppState: toggle_favorite()

    alt Skill not in favorites
        AppState->>AppState: favorites.insert(skill.id)
        Note over AppState: Add to HashSet
    else Skill in favorites
        AppState->>AppState: favorites.remove(skill.id)
        Note over AppState: Remove from HashSet
    end

    InputHandler->>ConfigLoader: save_config(state.config)
    ConfigLoader-->>InputHandler: Config saved
    InputHandler->>UIRenderer: Trigger re-render
    UIRenderer->>Terminal: Update UI (show ★ indicator)
    Terminal-->>User: Visual feedback

    User->>Terminal: Press Tab (switch view mode)
    Terminal->>InputHandler: KeyEvent(Tab)
    InputHandler->>AppState: Cycle view_mode

    alt Current: All
        AppState->>AppState: view_mode = Favorites
        Note over AppState: Filter skills to favorites only
    else Current: Favorites
        AppState->>AppState: view_mode = Recent
        Note over AppState: Filter skills to recent only
    else Current: Recent
        AppState->>AppState: view_mode = All
        Note over AppState: Show all skills
    end

    InputHandler->>AppState: Update filtered_skills based on view_mode
    InputHandler->>UIRenderer: Trigger re-render
    UIRenderer->>Terminal: Draw filtered list with mode indicator
    Terminal-->>User: Show updated view
```
