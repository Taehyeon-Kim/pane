# Epic 4: Claude Code Tips Viewer (Bundled Skill)

## Epic Goal

Deliver the flagship bundled skill that allows users to browse curated Claude Code usage tips within the terminal, demonstrating the value of the Pane launcher for productive micro-sessions during idle coding time.

## Epic Scope

- Tips data model and YAML format definition
- TUI-based tips browser with search and navigation
- Skill manifest and integration with Pane launcher
- Curated content library of Claude Code usage tips
- Bundled distribution with Pane binary

## Success Criteria

- Users can launch `claude-tips` skill from Pane launcher
- Tips are browsable by category with search/filter capability
- Tips display is readable and well-formatted in terminal
- 15-20 high-quality curated tips covering key Claude Code workflows
- Skill demonstrates the value proposition: "more convenient than opening browser/notes"
- Execution time aligns with "1-3 min" estimated time for micro-sessions
- **(After Epic 5)** Skill is installable and discoverable via standard Pane installation

---

## Story 4.1: Tips Data Model & YAML Parser

### Story Statement

**As a** developer,
**I want** a structured data format for Claude Code tips with parsing capability,
**so that** tips can be loaded, validated, and displayed in the tips viewer.

### Acceptance Criteria

1. `Tip` struct is defined with fields: id, title, category, text, tags
2. Tips data format uses YAML for easy content authoring
3. YAML parser loads tips from file into Vec<Tip> collection
4. Invalid tip entries are rejected with clear validation errors
5. Required fields (id, title, text) are enforced
6. Optional fields (category, tags) have sensible defaults
7. Tips file location follows bundled skill resource conventions
8. Unit tests cover parsing valid and invalid tip files

### Technical Notes

- Define Tip struct in skill source (claude-tips/src/model.rs)
- Use serde and serde_yaml for parsing (already in project dependencies)
- Tips YAML file location: `claude-tips/data/claude-tips.yaml`
- Follow PRD example data format (Section 3.5.3):
  ```yaml
  - id: "cc-001"
    title: "Avoid vague 'do it all' prompts"
    category: "prompting"
    text: |
      Multi-line tip content...
    tags: ["claude", "prompting", "workflow"]
  ```
- Validate unique tip IDs to prevent duplicates
- Consider bundling tips file with binary or loading from skill directory

---

## Story 4.2: Tips Browser TUI Component

### Story Statement

**As a** user,
**I want** an intuitive TUI interface to browse and search Claude Code tips,
**so that** I can quickly find relevant tips during coding sessions.

### Acceptance Criteria

1. Tips list view displays all tips with title, category, and tags
2. Selected tip is visually highlighted in list
3. Detail view shows full tip content (title, category, text, tags)
4. Navigation: ↑/↓ or j/k moves between tips, Enter toggles detail view
5. Search mode: `/` activates search, typing filters tips by title/tags/text
6. Search is case-insensitive with real-time filtering
7. Esc exits search mode or closes detail view or exits skill
8. Footer shows key hints: ↑/↓ navigate, Enter view, / search, Esc back/exit
9. Layout is responsive to terminal size with proper wrapping
10. Empty search results show helpful "No tips found" message

### Technical Notes

- Create new Rust binary crate in workspace: `claude-tips/`
- Use ratatui for TUI (consistent with Pane launcher)
- Implement similar state management pattern as Pane (AppState)
- Layout structure:
  - Header: "Claude Code Tips" title
  - Main area: Tips list (left/top) + Detail pane (right/bottom)
  - Footer: Key hints
- Reuse TUI patterns from Epic 2 where applicable
- Skill runs in fullscreen TUI mode (ui.mode = "tui")

---

## Story 4.3: Tips Skill Integration & Manifest

### Story Statement

**As a** user,
**I want** the Claude Code Tips Viewer to be discoverable and executable from Pane,
**so that** I can launch it seamlessly from the launcher.

### Acceptance Criteria

1. `pane-skill.yaml` manifest is created for claude-tips skill
2. Manifest specifies: id="claude-tips", name="Claude Code Tips", ui.mode="tui"
3. Manifest includes tags: ["tips", "claude", "coding"]
4. Estimated time is set to "1–3 min"
5. Skill executable is built as standalone binary: `claude-tips`
6. Skill is bundled in system skills location during Pane installation
7. Skill is discoverable via Pane skill discovery system
8. Pressing Enter on claude-tips in Pane launches the TUI successfully
9. Skill exits cleanly and returns to Pane launcher
10. Skill works standalone (can be run via `claude-tips` command directly)

### Technical Notes

- Create workspace member in `Cargo.toml`: `members = [".", "claude-tips"]`
- Build both binaries: `pane` and `claude-tips`
- Manifest location: `claude-tips/pane-skill.yaml`
- Install manifest to system skills directory (e.g., `/usr/local/share/pane/skills/claude-tips/`)
- No special context needed (skill is self-contained)
- Manifest example:
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
    mode: "tui"
    fullscreen: true
  ```

---

## Story 4.4: Sample Tips Content & Testing

### Story Statement

**As a** user,
**I want** high-quality curated tips about Claude Code workflows,
**so that** I can learn best practices and improve my productivity.

### Acceptance Criteria

1. Tips content file contains 15-20 curated Claude Code tips
2. Tips cover categories: prompting, cost optimization, workflow, debugging, best practices
3. Each tip has clear title, actionable text, and relevant tags
4. Tip text is concise (2-5 sentences) and terminal-friendly formatting
5. Tips are validated for accuracy and usefulness
6. End-to-end test: Launch skill, browse tips, search, view details, exit
7. Manual testing confirms tips display correctly in various terminal sizes
8. Documentation added: tips authoring guide for future content additions

### Technical Notes

- Curate tips from:
  - Claude Code official documentation
  - Common user questions/pain points
  - Best practices from development workflow
- Categories to cover:
  - **Prompting**: How to write effective prompts for Claude Code
  - **Cost**: Token optimization, context management
  - **Workflow**: Efficient usage patterns, shortcuts
  - **Debugging**: Common issues and solutions
  - **Best Practices**: Code review, testing, error handling
- Sample tip structure:
  ```yaml
  - id: "cc-001"
    title: "Avoid vague 'do it all' prompts"
    category: "prompting"
    text: |
      Instead of asking Claude Code to "just build everything",
      give it: 1) context, 2) current state, 3) desired change, 4) constraints.
      This helps Claude understand scope and deliver focused results.
    tags: ["claude", "prompting", "workflow"]
  ```
- Test tips readability in 80-column and 120-column terminals

---

## Dependencies

- **Epic 1** (Stories 1.1-1.4) - Skill discovery and manifest parsing ✅ Complete
- **Epic 2** (Stories 2.1-2.7) - TUI framework and skill execution ✅ Complete
- **Epic 3** (Stories 3.1-3.3) - Inline skill mode implementation ✅ Complete
- **Epic 5** (Installation & Distribution) - ⚠️ **Required for full Epic 4 completion**

**Note**: Epic 4 Stories are implemented but cannot be fully validated until Epic 5 provides the installation mechanism.

## Technical Risks

- **Content quality**: Tips must provide real value to justify the skill
  - Mitigation: Curate from real usage patterns, iterate based on feedback
- **Workspace build complexity**: Multi-binary workspace may complicate build/distribution
  - Mitigation: Test Cargo workspace configuration early, document build process
- **Tips file bundling**: Distribution mechanism for tips YAML file
  - Mitigation: Embed tips in binary with `include_str!` macro or bundle in skill directory

## Out of Scope for Epic 4

- Tips editing/adding via UI (future Phase 2)
- Remote tips synchronization or updates (future Phase 3)
- Tips categories as separate sections (simple flat list is sufficient for MVP)
- User-contributed tips submission system (future feature)
- Tips favoriting/bookmarking (can be added in Phase 2)
- Multi-language tips support (English only for MVP)
