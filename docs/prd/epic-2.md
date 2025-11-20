# Epic 2: TUI Launcher Implementation

## Epic Goal
Build a functional Terminal User Interface (TUI) that allows users to browse, search, filter, and execute skills through an intuitive keyboard-driven interface.

## Epic Scope
- TUI framework integration (ratatui + crossterm)
- Application event loop and state management
- Skill list display with scrolling and selection
- Search bar with real-time fuzzy filtering
- Detail pane showing selected skill information
- Keyboard navigation (↑/↓, j/k, Enter, Esc, Tab)
- Terminal handoff for skill execution (TUI mode)

## Success Criteria
- Users can launch `pane` and see a functional TUI launcher
- Skills are displayed in a scrollable list with metadata
- Typing in search bar filters skills in real-time using fuzzy matching
- Selected skill details are shown in a dedicated pane
- Pressing Enter executes the selected skill
- TUI suspends and restores correctly when launching TUI-mode skills
- All keyboard shortcuts work as specified in PRD
- Mouse support (optional for MVP)

---

## Story 2.1: TUI Framework & Application Structure

### Story Statement
**As a** developer,
**I want** the TUI framework integrated with proper application structure,
**so that** I have a foundation for building the interactive launcher interface.

### Acceptance Criteria
1. ratatui and crossterm dependencies are configured in Cargo.toml
2. AppState struct is defined with all necessary fields (skills, filtered_skills, selected_index, search_query, view_mode, etc.)
3. Event loop is implemented to handle keyboard/mouse input and rendering
4. Terminal state is managed with RAII guard (setup/cleanup/restore)
5. Basic TUI renders with header showing "Pane" title
6. Application exits cleanly on Esc key press
7. Unit tests exist for state management functions

### Technical Notes
- Use ratatui 0.26.0 as specified in tech stack
- Use crossterm 0.27.0 for terminal backend
- Implement TerminalGuard struct with Drop trait for cleanup
- Event loop should be non-blocking with timeout
- Follow RAII pattern for terminal mode changes

---

## Story 2.2: Skill List Display & Navigation

### Story Statement
**As a** user,
**I want** to see discovered skills in a scrollable list with navigation,
**so that** I can browse available skills and select one to view details.

### Acceptance Criteria
1. Skill list component renders all discovered skills in a vertical scrollable list
2. Each skill item displays: name, tags (inline), estimated time, one-line description
3. Selected skill is visually highlighted
4. ↑/↓ arrow keys navigate the list (with wrapping at boundaries)
5. j/k vim-style keys also navigate the list
6. PageUp/PageDown scroll by page
7. Mouse click selects a skill (nice-to-have)
8. List scrolls automatically to keep selected item visible

### Technical Notes
- Use ratatui List widget with highlighting
- Implement selection state in AppState
- Handle keyboard events in event loop
- Follow PRD layout specification (header, list, detail, footer)

---

## Story 2.3: Search & Fuzzy Filtering

### Story Statement
**As a** user,
**I want** to search and filter skills in real-time,
**so that** I can quickly find the skill I need.

### Acceptance Criteria
1. Search bar is rendered at the top with current query displayed
2. Typing updates the search query in AppState
3. Filtered skills list updates in real-time based on fuzzy matching
4. Fuzzy match searches: skill name, id, tags, description
5. Backspace removes characters from search query
6. Search is case-insensitive
7. Empty search shows all skills
8. Selected index resets to 0 when filter changes

### Technical Notes
- Use nucleo crate 0.2.0 for fuzzy matching (as specified in tech stack)
- Implement fuzzy_match function in search.rs module
- Update filtered_skills in AppState on every search query change
- Search bar should show cursor position

---

## Story 2.4: Skill Detail Pane & Footer

### Story Statement
**As a** user,
**I want** to see detailed information about the selected skill,
**so that** I can understand what it does before executing it.

### Acceptance Criteria
1. Detail pane renders on the right side or below skill list
2. Shows selected skill's: full description, estimated time, id, tags, source (project/user/system)
3. Detail updates automatically when selection changes
4. Footer displays key hints: ↑/↓ move, Enter run, Esc exit, Tab toggle view
5. Layout is responsive to terminal size
6. Truncation handles long descriptions gracefully

### Technical Notes
- Use ratatui Paragraph widget for detail content
- Use ratatui Block with borders for sections
- Footer should use Spans for formatted text
- Handle terminal resize events

---

## Story 2.5: View Mode Switching (All/Favorites/Recent)

### Story Statement
**As a** user,
**I want** to toggle between viewing all skills, favorites, and recent skills,
**so that** I can focus on the skills most relevant to me.

### Acceptance Criteria
1. Tab key cycles through view modes: All → Favorites → Recent → All
2. ViewMode enum is defined (All, Favorites, Recent)
3. Footer shows current view mode
4. Favorites view shows only skills marked as favorites
5. Recent view shows only recently executed skills (up to max_recent_skills from config)
6. Switching modes updates the displayed skill list
7. Search filters apply within the current view mode

### Technical Notes
- Define ViewMode enum in state.rs
- Implement view filtering logic
- Favorites and recent data will be persisted in future story
- For MVP, favorites/recent can be in-memory only

---

## Story 2.6: Skill Execution & Terminal Handoff

### Story Statement
**As a** user,
**I want** to execute a selected skill by pressing Enter,
**so that** I can run the skill and see its output.

### Acceptance Criteria
1. Pressing Enter on selected skill suspends the TUI
2. Terminal is restored to normal mode before skill execution
3. Skill process is spawned with correct exec and args from manifest
4. Context is passed via environment variables (PANE_ID, PANE_NAME, PANE_CWD, etc.)
5. For ui.mode = "tui", skill takes over the terminal completely
6. When skill exits, TUI is restored and redraws
7. Error messages are displayed if skill execution fails
8. Exit code from skill is captured and logged

### Technical Notes
- Implement skill runner in skills/runner.rs
- Use std::process::Command for spawning
- Implement SkillContext struct for environment variables
- Use git2 crate for git root detection
- Handle terminal suspend/restore with TerminalGuard
- Log execution to debug log if enabled

---

## Story 2.7: Theme & Styling

### Story Statement
**As a** user,
**I want** the TUI to have a polished visual design,
**so that** the interface is pleasant to use.

### Acceptance Criteria
1. Theme configuration is defined (colors, styles)
2. Header uses distinct styling (title centered or left-aligned)
3. Selected item has clear highlight color
4. Borders use consistent style throughout
5. Tags are rendered with chip-style inline formatting
6. Estimated time has an icon/prefix (⏱)
7. Theme can be customized via config.toml (optional for MVP)

### Technical Notes
- Define ThemeConfig in config.rs
- Use ratatui Style and Color types
- Follow PRD layout specification
- Default theme should be terminal-friendly (works in light/dark)

---

## Dependencies
- Epic 1 (Stories 1.1-1.4) must be complete
- All skill discovery functionality must be working

## Technical Risks
- Terminal state management complexity (suspend/restore)
- Fuzzy matching performance with large skill lists
- Terminal resize handling
- Cross-platform terminal compatibility

## Out of Scope for Epic 2
- Inline skill mode (ui.mode = "inline") - deferred to Epic 3
- Favorites/recent persistence - deferred to Phase 2
- Mouse double-click to execute - nice-to-have
- Custom themes beyond default - Phase 2
