# Epic 3: Inline Skill Mode Implementation

## Epic Goal

Enable skills to execute within the Pane launcher and display their output inline (without terminal takeover), allowing users to quickly view command results, status checks, and informational utilities while maintaining the launcher context.

## Epic Scope

- Inline skill execution runtime (ui.mode = "inline")
- Output capture mechanism (stdout/stderr buffering)
- Output display UI component (modal or panel)
- State management for inline execution results
- Example inline skill for validation

## Success Criteria

- Skills with ui.mode = "inline" execute without suspending the launcher
- Stdout/stderr are captured and displayed within the launcher interface
- Users can view, scroll, and dismiss inline output via keyboard
- Existing TUI mode skills continue to function without regression
- Output buffering handles large outputs gracefully (with limits)

---

## Story 3.1: Inline Execution Runtime & Output Capture

### Story Statement

**As a** developer,
**I want** the skill runner to support inline mode execution with output capture,
**so that** inline skills can execute without suspending the launcher and their output can be displayed within the TUI.

### Acceptance Criteria

1. Skill runner detects `ui.mode = "inline"` from manifest
2. For inline mode, child process is spawned with piped stdout and stderr
3. Output is captured into a buffer (String or Vec<u8>) during execution
4. Process completion is detected and exit code is captured
5. Launcher TUI remains active during inline skill execution
6. Long-running inline skills show "executing..." indicator
7. Output buffer has size limit (10MB) with truncation warning if exceeded
8. Errors during execution are captured and displayed in output panel
9. Existing TUI mode execution path remains unaffected
10. Unit tests cover inline execution and output capture

### Technical Notes

- Extend `skills/runner.rs` with new execution path for inline mode
- Use `Command::stdout(Stdio::piped())` and `stderr(Stdio::piped())`
- Read from child stdout/stderr using BufReader or similar
- Store captured output in AppState or separate OutputBuffer struct
- Implement timeout handling (optional for MVP, useful for hanging processes)
- Consider async execution for non-blocking behavior (optional for MVP)

---

## Story 3.2: Output Display UI Component

### Story Statement

**As a** user,
**I want** to see inline skill output in a dedicated panel or modal,
**so that** I can review results without leaving the launcher.

### Acceptance Criteria

1. Output display component renders captured text from inline skill execution
2. Component shows skill name, execution status (running/completed/failed), and exit code
3. Output text is scrollable (↑/↓ or j/k)
4. Long output is wrapped or truncated with scroll indicator
5. Esc key dismisses the output panel and returns to skill list
6. Panel uses consistent styling with launcher (borders, colors)
7. Output display handles ANSI color codes (optional for MVP, nice-to-have)
8. Empty output shows "No output" message
9. Error output (stderr) is visually distinguished from stdout (optional color coding)
10. Component integrates with ratatui layout system

### Technical Notes

- Create new UI component in `ui/components.rs` or `ui/output_panel.rs`
- Use ratatui Paragraph widget with scrolling state
- Consider modal (centered overlay) or side panel layout
- Implement scroll state tracking in AppState
- Handle terminal resize for output panel
- Strip or render ANSI codes (use `ansi-parser` crate if rendering, strip for MVP)

---

## Story 3.3: Inline Mode Integration & Testing

### Story Statement

**As a** user,
**I want** seamless execution of both TUI and inline skills,
**so that** I can use the appropriate skill mode without friction.

### Acceptance Criteria

1. AppState includes fields for inline output tracking (active_output, output_visible)
2. Pressing Enter on inline skill triggers inline execution flow
3. Output panel appears automatically when inline skill completes
4. Pressing Esc dismisses output and returns to skill list
5. Inline execution updates recent skills list (same as TUI mode)
6. Example inline skill is created (e.g., git-status-inline or system-info)
7. Example skill manifest uses ui.mode = "inline" and validates correctly
8. Regression tests confirm TUI mode skills still function correctly
9. Navigation shortcuts work consistently across TUI and inline modes
10. Documentation updated to explain inline mode usage

### Technical Notes

- Extend AppState with `active_inline_output: Option<SkillOutput>` struct
- Add `output_panel_visible: bool` flag to control display
- Create example skill script (bash or Python) that prints structured output
- Update event handling in `app.rs` to route inline vs TUI execution
- Add integration test that executes both TUI and inline skills
- Update user documentation with inline skill creation guide

---

## Dependencies

- Epic 1 (Stories 1.1-1.4) must be complete - skill discovery and manifest parsing
- Epic 2 (Stories 2.1-2.7) must be complete - TUI launcher and TUI mode execution

## Technical Risks

- **Large output handling**: Skills with megabytes of output could cause memory issues
  - Mitigation: Implement output size limits with truncation
- **Long-running inline skills**: Blocking execution could freeze launcher
  - Mitigation: Consider async execution or timeout handling
- **Output formatting**: Raw terminal output may not render well in panel
  - Mitigation: Strip ANSI codes for MVP, add rendering in future iteration

## Out of Scope for Epic 3

- Background/async execution of inline skills (all execution is synchronous for MVP)
- ANSI color code rendering (strip colors for MVP)
- Output export or saving to file
- Real-time streaming output during execution (show only after completion)
- Inline skill argument prompting (pass args from manifest only)
