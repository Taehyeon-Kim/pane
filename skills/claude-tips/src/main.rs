//! Claude Code Tips Viewer - A bundled skill for Pane
//!
//! This skill provides a browsable collection of tips and best practices
//! for working with Claude Code.

use anyhow::Context;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{io, path::Path, time::Duration};

mod app;
mod model;
mod parser;
mod ui;

use app::{handle_key_event, AppState};

/// RAII guard for terminal state restoration.
///
/// Ensures terminal is properly restored to its original state even if
/// the application panics or returns early. Implements the Drop trait
/// to perform cleanup automatically when the guard goes out of scope.
struct TerminalGuard;

impl TerminalGuard {
    /// Creates a new terminal guard and initializes the terminal.
    ///
    /// Enables raw mode and switches to alternate screen. These changes
    /// will be automatically reverted when the guard is dropped.
    fn new() -> anyhow::Result<Self> {
        enable_raw_mode().context("Failed to enable terminal raw mode")?;
        execute!(io::stdout(), EnterAlternateScreen).context("Failed to enter alternate screen")?;
        Ok(TerminalGuard)
    }
}

impl Drop for TerminalGuard {
    /// Restores terminal to its original state.
    ///
    /// Disables raw mode and leaves alternate screen. Errors during
    /// cleanup are silently ignored to prevent panic-during-panic.
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
    }
}

fn main() -> anyhow::Result<()> {
    // Load tips from bundled data file
    let tips_path = Path::new("data/claude-tips.yaml");
    let tips = parser::load_tips(tips_path).context("Failed to load tips from YAML file")?;

    if tips.is_empty() {
        anyhow::bail!("No tips found in data file");
    }

    // Initialize application state
    let mut state = AppState::new(tips);

    // Initialize terminal with RAII guard for cleanup
    let _terminal_guard = TerminalGuard::new()?;

    let mut terminal =
        Terminal::new(CrosstermBackend::new(io::stdout())).context("Failed to create terminal")?;

    // Clear the terminal before starting
    terminal.clear().context("Failed to clear terminal")?;

    // Main event loop
    loop {
        // Render UI
        terminal
            .draw(|frame| ui::render(frame, &state))
            .context("Failed to render UI")?;

        // Poll for events with timeout
        if event::poll(Duration::from_millis(100)).context("Failed to poll terminal events")? {
            if let Event::Key(key) = event::read().context("Failed to read terminal event")? {
                // Handle key event and update state
                handle_key_event(&mut state, key).context("Failed to handle key event")?;

                // Exit loop if quit flag is set
                if state.should_quit() {
                    break;
                }
            }
        }
    }

    // Terminal cleanup happens automatically via TerminalGuard's Drop
    Ok(())
}
