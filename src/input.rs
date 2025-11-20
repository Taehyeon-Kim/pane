use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::state::InputMode;

/// Input events recognized by the application
///
/// Maps terminal input events to application-level actions.
/// The set of available events depends on the current input mode
/// (Normal vs Insert).
#[derive(Debug, PartialEq, Eq)]
pub enum InputEvent {
    /// Quit the application (Esc key in Normal mode)
    Quit,
    /// Move selection up (↑ or k in Normal mode)
    MoveUp,
    /// Move selection down (↓ or j in Normal mode)
    MoveDown,
    /// Character input for search (any char in Insert mode)
    CharInput(char),
    /// Backspace key (delete character from search)
    Backspace,
    /// Enter key (execute selected skill)
    Enter,
    /// Tab key (toggle view mode)
    Tab,
    /// Page Up key (scroll up by page)
    PageUp,
    /// Page Down key (scroll down by page)
    PageDown,
    /// Enter Insert mode for search (/ key in Normal mode)
    EnterInsertMode,
    /// Enter Normal mode for navigation (Esc key in Insert mode)
    EnterNormalMode,
    /// Toggle favorite status of selected skill (f key in Normal mode)
    ToggleFavorite,
    /// Show help overlay (? key in Normal mode)
    ShowHelp,
}

/// Poll for an input event with a timeout
///
/// This function checks for terminal input events (keyboard, mouse, etc.)
/// and maps them to application-level InputEvents based on the current input mode.
/// Uses a timeout to allow the event loop to continue even when no input is available.
///
/// # Arguments
///
/// * `timeout` - Maximum time to wait for an event
/// * `input_mode` - Current input mode (Normal or Insert) for mode-aware key mapping
///
/// # Returns
///
/// - `Ok(Some(InputEvent))` if an event was received and mapped
/// - `Ok(None)` if the timeout expired with no event
/// - `Err` if reading the event failed
///
/// # Errors
///
/// Returns an error if the terminal event system fails to read events.
///
/// # Examples
///
/// ```no_run
/// use std::time::Duration;
/// use pane::input::poll_event;
/// use pane::state::InputMode;
///
/// let mode = InputMode::Normal;
/// let event = poll_event(Duration::from_millis(250), &mode).unwrap();
/// ```
pub fn poll_event(timeout: Duration, input_mode: &InputMode) -> Result<Option<InputEvent>> {
    // Check if an event is available within the timeout
    if !event::poll(timeout)? {
        return Ok(None);
    }

    // Read the event
    let event = event::read()?;

    // Map crossterm events to InputEvent based on current mode
    let input_event = match event {
        Event::Key(key_event) => {
            // Only process key press events (ignore key release on some terminals)
            if key_event.kind != KeyEventKind::Press {
                return Ok(None);
            }

            map_key_event(key_event, input_mode)
        }
        // Mouse and other events are ignored for now
        _ => None,
    };

    Ok(input_event)
}

/// Map a crossterm KeyEvent to an InputEvent based on input mode
///
/// This function implements mode-aware key mapping, where the same key can
/// trigger different events depending on whether the user is in Normal mode
/// (navigation/commands) or Insert mode (text input).
///
/// # Arguments
///
/// * `key_event` - The crossterm key event to map
/// * `input_mode` - Current input mode (Normal or Insert)
///
/// # Returns
///
/// The corresponding InputEvent, or None if the key should be ignored
///
/// # Mode-Specific Behavior
///
/// **Normal Mode:**
/// - `j`/`k` → Navigation (MoveDown/MoveUp)
/// - `/` → Enter Insert mode
/// - `f` → Toggle favorite
/// - `?` → Show help
/// - `Esc` → Quit (or clear search if present, handled in app.rs)
///
/// **Insert Mode:**
/// - All characters (including j/k//) → CharInput for search
/// - `Esc` → Enter Normal mode
/// - `Backspace` → Remove character from search
fn map_key_event(key_event: KeyEvent, input_mode: &InputMode) -> Option<InputEvent> {
    match input_mode {
        InputMode::Normal => {
            // Normal mode: Navigation and commands
            match key_event.code {
                KeyCode::Char('j') => Some(InputEvent::MoveDown),
                KeyCode::Char('k') => Some(InputEvent::MoveUp),
                KeyCode::Char('/') => Some(InputEvent::EnterInsertMode),
                KeyCode::Char('f') => Some(InputEvent::ToggleFavorite),
                KeyCode::Char('?') => Some(InputEvent::ShowHelp),
                KeyCode::Up => Some(InputEvent::MoveUp),
                KeyCode::Down => Some(InputEvent::MoveDown),
                KeyCode::Esc => Some(InputEvent::Quit),
                KeyCode::Enter => Some(InputEvent::Enter),
                KeyCode::Tab => Some(InputEvent::Tab),
                KeyCode::PageUp => Some(InputEvent::PageUp),
                KeyCode::PageDown => Some(InputEvent::PageDown),
                // Other character keys ignored in Normal mode
                _ => None,
            }
        }
        InputMode::Insert => {
            // Insert mode: All characters are search input
            match key_event.code {
                KeyCode::Esc => Some(InputEvent::EnterNormalMode),
                KeyCode::Backspace => Some(InputEvent::Backspace),
                KeyCode::Char(c) => Some(InputEvent::CharInput(c)),
                // Arrow keys still work for navigation in Insert mode (optional UX decision)
                KeyCode::Up => Some(InputEvent::MoveUp),
                KeyCode::Down => Some(InputEvent::MoveDown),
                KeyCode::Enter => Some(InputEvent::Enter),
                KeyCode::Tab => Some(InputEvent::Tab),
                KeyCode::PageUp => Some(InputEvent::PageUp),
                KeyCode::PageDown => Some(InputEvent::PageDown),
                _ => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    fn create_key_event(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    // Tests for Normal mode key mapping
    #[test]
    fn test_esc_key_quits_in_normal_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Esc);

        // Act
        let result = map_key_event(key_event, &InputMode::Normal);

        // Assert
        assert_eq!(result, Some(InputEvent::Quit));
    }

    #[test]
    fn test_j_key_moves_down_in_normal_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Char('j'));

        // Act
        let result = map_key_event(key_event, &InputMode::Normal);

        // Assert
        assert_eq!(result, Some(InputEvent::MoveDown));
    }

    #[test]
    fn test_k_key_moves_up_in_normal_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Char('k'));

        // Act
        let result = map_key_event(key_event, &InputMode::Normal);

        // Assert
        assert_eq!(result, Some(InputEvent::MoveUp));
    }

    #[test]
    fn test_slash_enters_insert_mode_in_normal_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Char('/'));

        // Act
        let result = map_key_event(key_event, &InputMode::Normal);

        // Assert
        assert_eq!(result, Some(InputEvent::EnterInsertMode));
    }

    #[test]
    fn test_f_key_toggles_favorite_in_normal_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Char('f'));

        // Act
        let result = map_key_event(key_event, &InputMode::Normal);

        // Assert
        assert_eq!(result, Some(InputEvent::ToggleFavorite));
    }

    #[test]
    fn test_question_mark_shows_help_in_normal_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Char('?'));

        // Act
        let result = map_key_event(key_event, &InputMode::Normal);

        // Assert
        assert_eq!(result, Some(InputEvent::ShowHelp));
    }

    // Tests for Insert mode key mapping
    #[test]
    fn test_j_key_inputs_char_in_insert_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Char('j'));

        // Act
        let result = map_key_event(key_event, &InputMode::Insert);

        // Assert
        assert_eq!(result, Some(InputEvent::CharInput('j')));
    }

    #[test]
    fn test_k_key_inputs_char_in_insert_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Char('k'));

        // Act
        let result = map_key_event(key_event, &InputMode::Insert);

        // Assert
        assert_eq!(result, Some(InputEvent::CharInput('k')));
    }

    #[test]
    fn test_slash_key_inputs_char_in_insert_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Char('/'));

        // Act
        let result = map_key_event(key_event, &InputMode::Insert);

        // Assert
        assert_eq!(result, Some(InputEvent::CharInput('/')));
    }

    #[test]
    fn test_esc_enters_normal_mode_in_insert_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Esc);

        // Act
        let result = map_key_event(key_event, &InputMode::Insert);

        // Assert
        assert_eq!(result, Some(InputEvent::EnterNormalMode));
    }

    #[test]
    fn test_backspace_in_insert_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Backspace);

        // Act
        let result = map_key_event(key_event, &InputMode::Insert);

        // Assert
        assert_eq!(result, Some(InputEvent::Backspace));
    }

    // Tests for arrow keys (work in both modes)
    #[test]
    fn test_arrow_up_in_normal_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Up);

        // Act
        let result = map_key_event(key_event, &InputMode::Normal);

        // Assert
        assert_eq!(result, Some(InputEvent::MoveUp));
    }

    #[test]
    fn test_arrow_down_in_normal_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Down);

        // Act
        let result = map_key_event(key_event, &InputMode::Normal);

        // Assert
        assert_eq!(result, Some(InputEvent::MoveDown));
    }

    #[test]
    fn test_arrow_up_in_insert_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Up);

        // Act
        let result = map_key_event(key_event, &InputMode::Insert);

        // Assert
        assert_eq!(result, Some(InputEvent::MoveUp));
    }

    #[test]
    fn test_enter_in_both_modes() {
        // Arrange
        let key_event = create_key_event(KeyCode::Enter);

        // Act & Assert - works in both modes
        assert_eq!(
            map_key_event(key_event, &InputMode::Normal),
            Some(InputEvent::Enter)
        );
        assert_eq!(
            map_key_event(key_event, &InputMode::Insert),
            Some(InputEvent::Enter)
        );
    }

    #[test]
    fn test_tab_in_both_modes() {
        // Arrange
        let key_event = create_key_event(KeyCode::Tab);

        // Act & Assert - works in both modes
        assert_eq!(
            map_key_event(key_event, &InputMode::Normal),
            Some(InputEvent::Tab)
        );
        assert_eq!(
            map_key_event(key_event, &InputMode::Insert),
            Some(InputEvent::Tab)
        );
    }

    #[test]
    fn test_page_up_in_both_modes() {
        // Arrange
        let key_event = create_key_event(KeyCode::PageUp);

        // Act & Assert - works in both modes
        assert_eq!(
            map_key_event(key_event, &InputMode::Normal),
            Some(InputEvent::PageUp)
        );
        assert_eq!(
            map_key_event(key_event, &InputMode::Insert),
            Some(InputEvent::PageUp)
        );
    }

    #[test]
    fn test_page_down_in_both_modes() {
        // Arrange
        let key_event = create_key_event(KeyCode::PageDown);

        // Act & Assert - works in both modes
        assert_eq!(
            map_key_event(key_event, &InputMode::Normal),
            Some(InputEvent::PageDown)
        );
        assert_eq!(
            map_key_event(key_event, &InputMode::Insert),
            Some(InputEvent::PageDown)
        );
    }

    #[test]
    fn test_regular_char_ignored_in_normal_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Char('a'));

        // Act
        let result = map_key_event(key_event, &InputMode::Normal);

        // Assert - regular chars (not j/k/f/?) are ignored in Normal mode
        assert_eq!(result, None);
    }

    #[test]
    fn test_regular_char_captured_in_insert_mode() {
        // Arrange
        let key_event = create_key_event(KeyCode::Char('a'));

        // Act
        let result = map_key_event(key_event, &InputMode::Insert);

        // Assert
        assert_eq!(result, Some(InputEvent::CharInput('a')));
    }
}
