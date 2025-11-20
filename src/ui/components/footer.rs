use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::i18n::Translations;
use crate::state::{InputMode, ViewMode};
use crate::ui::theme::ThemeConfig;

/// Render the footer with key hints and mode indicator
///
/// Displays keyboard shortcuts, current input mode, and view mode at the bottom
/// of the TUI. The footer provides users with quick reference for navigation,
/// actions, and current mode awareness.
///
/// # Arguments
///
/// * `area` - The rectangular area to render into
/// * `frame` - The ratatui frame to render into
/// * `view_mode` - The current view mode (All/Favorites/Recent)
/// * `input_mode` - The current input mode (Normal/Insert)
/// * `theme` - Theme configuration for styling
///
/// # Mode-Specific Display
///
/// **Normal Mode**: Shows navigation keys (j/k), search (/), and commands
/// **Insert Mode**: Shows "-- INSERT --" and escape to return to Normal mode
///
/// # Example
///
/// ```no_run
/// use ratatui::backend::TestBackend;
/// use ratatui::Terminal;
/// use pane::state::{ViewMode, InputMode};
/// use pane::ui::components::footer::render_footer;
/// use pane::ui::theme::ThemeConfig;
///
/// let backend = TestBackend::new(80, 3);
/// let mut terminal = Terminal::new(backend).unwrap();
/// let view_mode = ViewMode::All;
/// let input_mode = InputMode::Normal;
/// let theme = ThemeConfig::default();
///
/// terminal.draw(|frame| {
///     render_footer(frame.size(), frame, &view_mode, &input_mode, &theme);
/// }).unwrap();
/// ```
pub fn render_footer(
    area: Rect,
    frame: &mut Frame,
    view_mode: &ViewMode,
    input_mode: &InputMode,
    translations: &Translations,
    theme: &ThemeConfig,
) {
    let view_mode_text = match view_mode {
        ViewMode::All => translations.footer_view_all,
        ViewMode::Favorites => translations.footer_view_favorites,
        ViewMode::Recent => translations.footer_view_recent,
    };

    // Build mode-specific footer content
    let mut footer_spans = Vec::new();

    // Mode indicator (left side)
    match input_mode {
        InputMode::Normal => {
            // No mode indicator in Normal mode for cleaner look
        }
        InputMode::Insert => {
            footer_spans.push(Span::styled(
                translations.footer_insert_mode,
                Style::default().fg(theme.secondary),
            ));
            footer_spans.push(Span::raw("  "));
        }
    }

    // Key hints (mode-specific) - use translated hints strings
    match input_mode {
        InputMode::Normal => {
            footer_spans.push(Span::raw(translations.footer_normal_hints));
            footer_spans.push(Span::raw(" | "));
        }
        InputMode::Insert => {
            footer_spans.push(Span::raw(translations.footer_insert_hints));
            footer_spans.push(Span::raw(" | "));
        }
    }

    // View mode indicator (right side, always visible)
    footer_spans.push(Span::styled(
        format!("Tab View: {}", view_mode_text),
        Style::default().fg(theme.primary),
    ));

    let footer = Paragraph::new(Line::from(footer_spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(theme.border_style)
                .border_style(theme.border_style()),
        )
        .style(Style::default());

    frame.render_widget(footer, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::{Language, Translations};
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_render_footer_normal_mode() {
        // Arrange
        let backend = TestBackend::new(80, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let view_mode = ViewMode::All;
        let input_mode = InputMode::Normal;
        let translations = Translations::load(Language::En);
        let theme = ThemeConfig::default();

        // Act & Assert - rendering should complete without panic
        terminal
            .draw(|frame| {
                render_footer(
                    frame.size(),
                    frame,
                    &view_mode,
                    &input_mode,
                    &translations,
                    &theme,
                );
            })
            .unwrap();
    }

    #[test]
    fn test_render_footer_insert_mode() {
        // Arrange
        let backend = TestBackend::new(80, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let view_mode = ViewMode::All;
        let input_mode = InputMode::Insert;
        let translations = Translations::load(Language::En);
        let theme = ThemeConfig::default();

        // Act & Assert - rendering should complete without panic
        terminal
            .draw(|frame| {
                render_footer(
                    frame.size(),
                    frame,
                    &view_mode,
                    &input_mode,
                    &translations,
                    &theme,
                );
            })
            .unwrap();
    }

    #[test]
    fn test_render_footer_shows_current_view_mode() {
        // Arrange
        let backend = TestBackend::new(80, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let view_mode = ViewMode::Favorites;
        let input_mode = InputMode::Normal;
        let translations = Translations::load(Language::En);
        let theme = ThemeConfig::default();

        // Act & Assert - rendering should complete without panic
        terminal
            .draw(|frame| {
                render_footer(
                    frame.size(),
                    frame,
                    &view_mode,
                    &input_mode,
                    &translations,
                    &theme,
                );
            })
            .unwrap();
    }

    #[test]
    fn test_render_footer_view_mode_all() {
        // Arrange
        let backend = TestBackend::new(80, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let view_mode = ViewMode::All;
        let input_mode = InputMode::Normal;
        let translations = Translations::load(Language::En);
        let theme = ThemeConfig::default();

        // Act & Assert - rendering should complete without panic
        terminal
            .draw(|frame| {
                render_footer(
                    frame.size(),
                    frame,
                    &view_mode,
                    &input_mode,
                    &translations,
                    &theme,
                );
            })
            .unwrap();
    }

    #[test]
    fn test_render_footer_view_mode_favorites() {
        // Arrange
        let backend = TestBackend::new(80, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let view_mode = ViewMode::Favorites;
        let input_mode = InputMode::Normal;
        let translations = Translations::load(Language::En);
        let theme = ThemeConfig::default();

        // Act & Assert - rendering should complete without panic
        terminal
            .draw(|frame| {
                render_footer(
                    frame.size(),
                    frame,
                    &view_mode,
                    &input_mode,
                    &translations,
                    &theme,
                );
            })
            .unwrap();
    }

    #[test]
    fn test_render_footer_view_mode_recent() {
        // Arrange
        let backend = TestBackend::new(80, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let view_mode = ViewMode::Recent;
        let input_mode = InputMode::Normal;
        let translations = Translations::load(Language::En);
        let theme = ThemeConfig::default();

        // Act & Assert - rendering should complete without panic
        terminal
            .draw(|frame| {
                render_footer(
                    frame.size(),
                    frame,
                    &view_mode,
                    &input_mode,
                    &translations,
                    &theme,
                );
            })
            .unwrap();
    }
}
