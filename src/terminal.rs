use std::io::{self, Stdout};

use anyhow::{Context, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

/// RAII guard for terminal state management
///
/// Automatically sets up the terminal for TUI mode on creation and restores
/// it to normal mode on drop (even if a panic occurs). This ensures the
/// terminal is always left in a usable state.
///
/// # Examples
///
/// ```no_run
/// use pane::terminal::TerminalGuard;
///
/// fn main() -> anyhow::Result<()> {
///     let mut term_guard = TerminalGuard::new()?;
///     let terminal = term_guard.terminal();
///
///     // Use the terminal for TUI rendering
///     // ...
///
///     // Terminal is automatically restored when term_guard goes out of scope
///     Ok(())
/// }
/// ```
pub struct TerminalGuard {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalGuard {
    /// Create a new TerminalGuard and initialize the terminal for TUI mode
    ///
    /// This function:
    /// - Enables raw mode (disables line buffering and echoing)
    /// - Enters the alternate screen buffer
    /// - Enables mouse capture (if available)
    /// - Clears the terminal
    ///
    /// # Returns
    ///
    /// A new TerminalGuard ready for rendering, or an error if terminal
    /// initialization fails.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Raw mode cannot be enabled
    /// - Alternate screen cannot be entered
    /// - Terminal backend cannot be created
    pub fn new() -> Result<Self> {
        // Enable raw mode (disable line buffering and echoing)
        enable_raw_mode().context("Failed to enable raw mode")?;

        // Enter alternate screen and enable mouse capture
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .context("Failed to enter alternate screen")?;

        // Create terminal with crossterm backend
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).context("Failed to create terminal backend")?;

        // Clear terminal
        terminal.clear().context("Failed to clear terminal")?;

        Ok(Self { terminal })
    }

    /// Get a mutable reference to the underlying terminal
    ///
    /// Use this to call `terminal.draw()` for rendering the TUI.
    pub fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }
}

impl Drop for TerminalGuard {
    /// Restore terminal to normal mode
    ///
    /// This is called automatically when the TerminalGuard goes out of scope,
    /// ensuring the terminal is always restored even if a panic occurs.
    ///
    /// Errors during cleanup are logged but not propagated, since Drop cannot
    /// return a Result.
    fn drop(&mut self) {
        // Disable mouse capture and leave alternate screen
        if let Err(e) = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        ) {
            eprintln!("Failed to restore terminal screen: {}", e);
        }

        // Disable raw mode
        if let Err(e) = disable_raw_mode() {
            eprintln!("Failed to disable raw mode: {}", e);
        }

        // Show cursor
        if let Err(e) = self.terminal.show_cursor() {
            eprintln!("Failed to show cursor: {}", e);
        }
    }
}
