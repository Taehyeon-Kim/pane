use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::theme::ThemeConfig;

/// Render a search bar for filtering skills
///
/// Displays a bordered input area showing the current search query with a cursor
/// indicator when focused. The search bar allows users to type queries that filter
/// the skill list in real-time using fuzzy matching.
///
/// # Arguments
///
/// * `area` - The rectangular area to render the search bar into
/// * `frame` - The ratatui frame to render into
/// * `query` - The current search query string
/// * `is_focused` - Whether the search bar is currently focused (shows cursor)
/// * `theme` - Theme configuration for styling
///
/// # Layout
///
/// ```text
/// ┌─ Search ───────────────────────────────────────┐
/// │ Search: query_                                 │
/// └────────────────────────────────────────────────┘
/// ```
///
/// - Border: Thin border with "Search" title, styled with theme colors
/// - Label: "Search: " prefix before query text
/// - Cursor: `_` character at end of query when focused
/// - Placeholder: "Type to search..." shown when query is empty and focused
/// - Height: 3 lines (border top + content + border bottom)
///
/// # Example
///
/// ```no_run
/// use ratatui::Frame;
/// use ratatui::layout::Rect;
/// # use pane::ui::components::search_bar::render_search_bar;
/// # use pane::ui::theme::ThemeConfig;
///
/// fn render(frame: &mut Frame, query: &str, area: Rect) {
///     let theme = ThemeConfig::default();
///     render_search_bar(area, frame, query, true, &theme);
/// }
/// ```
pub fn render_search_bar(
    area: Rect,
    frame: &mut Frame,
    query: &str,
    is_focused: bool,
    placeholder: &str,
    theme: &ThemeConfig,
) {
    // Format the search text with cursor when focused
    let search_text = if query.is_empty() && is_focused {
        format!("Search: {}_", placeholder)
    } else if is_focused {
        format!("Search: {}_", query)
    } else {
        format!("Search: {}", query)
    };

    // Create text line with appropriate styling
    let line = Line::from(vec![Span::styled(
        search_text,
        if is_focused {
            Style::default().fg(theme.text)
        } else {
            Style::default().fg(theme.text_dim)
        },
    )]);

    // Create paragraph widget with border
    let paragraph = Paragraph::new(line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(theme.border_style)
            .title("Search")
            .border_style(if is_focused {
                Style::default().fg(theme.primary)
            } else {
                theme.border_style()
            }),
    );

    // Render the search bar
    frame.render_widget(paragraph, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_render_search_bar_empty_query_focused() {
        // Arrange
        let backend = TestBackend::new(60, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = ThemeConfig::default();

        // Act
        terminal
            .draw(|frame| {
                let area = Rect::new(0, 0, 60, 3);
                render_search_bar(area, frame, "", true, "Type to search...", &theme);
            })
            .unwrap();

        // Assert - should render without panicking
        // More detailed assertions would require inspecting terminal buffer
    }

    #[test]
    fn test_render_search_bar_with_query_focused() {
        // Arrange
        let backend = TestBackend::new(60, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = ThemeConfig::default();

        // Act
        terminal
            .draw(|frame| {
                let area = Rect::new(0, 0, 60, 3);
                render_search_bar(area, frame, "clau", true, "Type to search...", &theme);
            })
            .unwrap();

        // Assert - should render without panicking
    }

    #[test]
    fn test_render_search_bar_with_query_unfocused() {
        // Arrange
        let backend = TestBackend::new(60, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = ThemeConfig::default();

        // Act
        terminal
            .draw(|frame| {
                let area = Rect::new(0, 0, 60, 3);
                render_search_bar(area, frame, "test", false, "Type to search...", &theme);
            })
            .unwrap();

        // Assert - should render without panicking
    }

    #[test]
    fn test_render_search_bar_empty_query_unfocused() {
        // Arrange
        let backend = TestBackend::new(60, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = ThemeConfig::default();

        // Act
        terminal
            .draw(|frame| {
                let area = Rect::new(0, 0, 60, 3);
                render_search_bar(area, frame, "", false, "Type to search...", &theme);
            })
            .unwrap();

        // Assert - should render without panicking
    }

    #[test]
    fn test_render_search_bar_long_query() {
        // Arrange
        let backend = TestBackend::new(60, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = ThemeConfig::default();
        let long_query = "this is a very long search query that might overflow";

        // Act
        terminal
            .draw(|frame| {
                let area = Rect::new(0, 0, 60, 3);
                render_search_bar(area, frame, long_query, true, "Type to search...", &theme);
            })
            .unwrap();

        // Assert - should render without panicking (truncation handled by ratatui)
    }
}
