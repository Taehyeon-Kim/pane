# Pane – Terminal Sidecar Utility (Brief)

**Pane** is a terminal-based sidecar utility launcher designed for modern “vibe coding” workflows.

Users now spend long, continuous periods looking at terminals (Cursor, Zed, VSCode terminal, Warp, Ghostty, Neovim + tmux, etc.). During this visual attention time, Pane turns idle or waiting moments into productive, lightweight micro-sessions.

Pane provides:

- A single launch command: `pane`
- A full-screen terminal UI (TUI) launcher
- Searchable, keyboard-first (and optionally mouse-friendly) utility catalog
- Community-extensible “skills” (small, focused utilities)
- Seamless execution inside an existing terminal pane (tmux split, IDE terminal panel, etc.)

Pane is conceptually similar to **Raycast/Alfred for the terminal**, but optimized for:

- Short, focused tasks (30 seconds–3 minutes)
- Vibe coding scenarios (builds, tests, deployments, LLM calls, etc.)
- Both developers and non-developers who are comfortable living in the terminal

---

## Problem

1. **Terminal attention time has exploded.**
   - People keep terminals open as a primary workspace (logs, dev servers, AI tools, CLIs).
   - During long waits (builds, tests, remote commands, LLM calls), users stare at the terminal.

2. **Context switching is still expensive.**
   - Opening separate note apps, browsers, or GUI tools interrupts flow.
   - Even “small” actions (open notes → search → read → come back) cause mental swaps.

3. **Micro-tasks and reference content have no dedicated home.**
   - Quick tips, coding patterns, personal archives (like “Claude Code Tips”) stay in scattered places (Threads, Notion, notes apps).
   - They are useful **exactly** during idle coding time, but hard to access without context switching.

---

## Solution

Pane lives *inside* the terminal and provides:

- A **single entry point**: `pane`
- A **TUI launcher** with:
  - Search bar
  - Utility list with tags and estimated duration
  - Short descriptions and key hints
- A **Skill system** where each utility is a “skill” defined by a simple manifest + executable
- A **first bundled skill**: **Claude Code Tips Viewer**
  - Shows archived Claude Code usage tips
  - Searchable and scrollable
  - Perfect as a right-side pane during coding

---

## MVP in One Sentence

> Ship a single-binary TUI launcher (`pane`) with a clean UX and one high-value skill (**Claude Code Tips Viewer**), plus a minimal skill system that makes it easy to add more utilities later.
