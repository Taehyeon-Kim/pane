# Error Handling Strategy

## General Approach

**Error Model:** Result-based error handling with `anyhow` for application code

**Exception Hierarchy:**
- **Application Errors** (`anyhow::Error`) - Main error type for user-facing errors
- **Component-Specific Errors** - Custom error types where needed (e.g., `SkillLoadError`, `ConfigError`)
- **Library Errors** - Errors from dependencies, wrapped with context

**Error Propagation:**
- Use `?` operator for automatic propagation with context
- Add `.context()` or `.with_context()` for rich error messages
- Display user-friendly error messages, log technical details

## Logging Standards

**Library:** `tracing` 0.1

**Format:** Structured JSON logging (when enabled)

**Levels:** ERROR (critical failures), WARN (recoverable issues), INFO (state changes), DEBUG (detailed flow), TRACE (verbose)

**Required Context:**
- **Correlation ID:** Generated per-session UUID for tracing related logs
- **Service Context:** Component name (e.g., "skill_loader", "ui_renderer")
- **User Context:** Never log personally identifiable information

**Log Output:**
- **Production:** File-based logging to `~/.config/pane/logs/pane-debug.log` (opt-in via config)
- **Development:** stdout with pretty-printing
- **Disabled by Default:** Zero overhead when logging is off

## Error Handling Patterns

### Skill Discovery and Loading Errors

**Scenario:** Skill manifest not found, invalid YAML, missing required fields

**Retry Policy:** No retry - skip skill and continue discovery

**Error Translation:** Log warning, skip individual skill failures, continue loading other skills

**User-Facing Error:** "Some skills could not be loaded. Run with `PANE_DEBUG=1` for details."

### Skill Execution Errors

**Scenario:** Executable not found, execution fails, timeout

**Retry Policy:** No automatic retry - return to launcher with error message

**Timeout Configuration:** No timeout for MVP (skills are user-trusted)

**User-Facing Error:** Toast notification in launcher: "Failed to run '{skill.name}': executable not found"

### Configuration Errors

**Scenario:** Config file corrupted, invalid TOML, permission denied

**Retry Policy:** Fallback to default configuration

**User-Facing Error:** "Config file corrupted, using defaults. Check `~/.config/pane/config.toml`"

### Terminal Handoff Errors

**Scenario:** Failed to suspend/restore TUI, terminal state corrupted

**Retry Policy:** Attempt restore once, then exit gracefully

**Compensation Logic:** TerminalGuard RAII pattern ensures terminal restore even on panic

**User-Facing Error:** "Terminal handoff failed. Please restart your terminal."

### Git Detection Errors

**Scenario:** Git repository detection fails, libgit2 errors

**Retry Policy:** No retry - gracefully degrade, continue without git context

**User-Facing Error:** None - silently degrades, git context not passed to skill

## Data Consistency

**Transaction Strategy:** Atomic file writes using temp file + rename pattern

**Compensation Logic:** If save fails, old config remains untouched

**Idempotency:** Config saves are idempotent - safe to retry

## Error Codes

**Exit Codes:** Unix conventions
- **0** - Success
- **1** - General application error
- **2** - CLI argument parsing error
- **64** - Skill not found
- **65** - Config error
- **70** - Internal error (bug)
