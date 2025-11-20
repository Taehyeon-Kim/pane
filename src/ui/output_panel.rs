use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::state::AppState;

/// Render the output panel as a modal overlay
///
/// Displays the output from an inline skill execution in a centered modal panel.
/// The panel shows:
/// - Title bar with skill name
/// - Status header (execution status, exit code, execution time)
/// - Scrollable stdout output
/// - Stderr output (if present, in red)
/// - Footer with key hints
///
/// # Arguments
///
/// * `frame` - The ratatui Frame to render into
/// * `state` - Application state containing the active output
///
/// # Returns
///
/// Returns early if output panel is not visible or no output is stored.
pub fn render_output_panel(frame: &mut Frame, state: &AppState) {
    // Early return if panel not visible
    if !state.is_output_panel_visible() {
        return;
    }

    // Get the active output or return if none
    let Some(output) = state.active_output() else {
        return;
    };

    // Create centered modal area (80% width, 80% height)
    let area = centered_rect(80, 80, frame.size());

    // Create main panel block with title
    let title = " Output ";
    let panel_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(Color::Cyan));

    // Calculate inner area for content
    let inner_area = panel_block.inner(area);

    // Render the panel border
    frame.render_widget(panel_block, area);

    // Split inner area into sections
    let sections = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Status header
            Constraint::Min(1),    // Output content
            Constraint::Length(1), // Footer
        ])
        .split(inner_area);

    let header_area = sections[0];
    let content_area = sections[1];
    let footer_area = sections[2];

    // Render status header
    render_status_header(frame, header_area, output);

    // Render scrollable output content
    render_output_content(frame, content_area, state, output);

    // Render footer with key hints
    render_footer(frame, footer_area);
}

/// Render the status header section
///
/// Displays execution status, exit code, and execution time.
fn render_status_header(
    frame: &mut Frame,
    area: Rect,
    output: &crate::skills::output::SkillOutput,
) {
    let mut lines = Vec::new();

    // Status line
    let (status_text, status_color) = match output.exit_code {
        Some(0) => ("Completed ✓", Color::Green),
        Some(_) => ("Failed ✗", Color::Red),
        None => ("Error", Color::Red),
    };

    let status_line = Line::from(vec![
        Span::raw("Status: "),
        Span::styled(status_text, Style::default().fg(status_color)),
        Span::raw("    Exit Code: "),
        Span::raw(match output.exit_code {
            Some(code) => code.to_string(),
            None => "N/A".to_string(),
        }),
    ]);
    lines.push(status_line);

    // Execution time line
    let time_ms = output.execution_time.as_millis();
    let time_line = Line::from(vec![
        Span::raw("Execution Time: "),
        Span::raw(format!("{}ms", time_ms)),
    ]);
    lines.push(time_line);

    // Truncation warning if applicable
    if output.truncated {
        let warning_line = Line::from(vec![Span::styled(
            "⚠️  Output truncated (exceeded 10MB limit)",
            Style::default().fg(Color::Yellow),
        )]);
        lines.push(warning_line);
    }

    let header = Paragraph::new(lines);
    frame.render_widget(header, area);
}

/// Render the scrollable output content
///
/// Displays stdout and stderr with scrolling support.
fn render_output_content(
    frame: &mut Frame,
    area: Rect,
    state: &AppState,
    output: &crate::skills::output::SkillOutput,
) {
    let scroll_offset = state.output_scroll_offset();

    // Handle empty output
    if output.stdout.is_empty() && output.stderr.is_empty() {
        let empty_msg = Paragraph::new("No output")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(empty_msg, area);
        return;
    }

    // Collect all output lines
    let mut all_lines: Vec<Line> = Vec::new();

    // Add stdout lines
    if !output.stdout.is_empty() {
        for line in output.stdout.lines() {
            all_lines.push(Line::from(line));
        }
    }

    // Add stderr section if present
    if !output.stderr.is_empty() {
        // Add separator
        all_lines.push(Line::from(Span::styled(
            "─── Error Output ───",
            Style::default().fg(Color::Red),
        )));

        // Add stderr lines in red
        for line in output.stderr.lines() {
            all_lines.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::Red),
            )));
        }
    }

    // Calculate visible window
    let total_lines = all_lines.len();
    let visible_height = area.height.saturating_sub(2) as usize; // -2 for potential scroll indicators
    let max_offset = total_lines.saturating_sub(visible_height);
    let clamped_offset = scroll_offset.min(max_offset);

    // Add scroll indicators if needed
    let has_more_above = clamped_offset > 0;
    let has_more_below = clamped_offset + visible_height < total_lines;

    let mut display_lines = Vec::new();

    // Top scroll indicator
    if has_more_above {
        display_lines.push(Line::from(Span::styled(
            "▲ More above",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM),
        )));
    }

    // Visible content lines
    let visible_lines = all_lines
        .iter()
        .skip(clamped_offset)
        .take(visible_height)
        .cloned();
    display_lines.extend(visible_lines);

    // Bottom scroll indicator
    if has_more_below {
        display_lines.push(Line::from(Span::styled(
            "▼ More below",
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM),
        )));
    }

    let content = Paragraph::new(display_lines).wrap(Wrap { trim: false });
    frame.render_widget(content, area);
}

/// Render the footer with key hints
fn render_footer(frame: &mut Frame, area: Rect) {
    let hints = Line::from(vec![
        Span::styled("↑/↓ or j/k", Style::default().fg(Color::Cyan)),
        Span::raw(": scroll | "),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::raw(": close"),
    ]);

    let footer = Paragraph::new(hints).style(Style::default().fg(Color::Gray));
    frame.render_widget(footer, area);
}

/// Create a centered rect using percentage-based constraints
///
/// # Arguments
///
/// * `percent_x` - Percentage of parent width (0-100)
/// * `percent_y` - Percentage of parent height (0-100)
/// * `r` - Parent rect
///
/// # Returns
///
/// A centered Rect with the specified percentage dimensions
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_centered_rect_calculates_correct_dimensions() {
        // Arrange
        let parent = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };

        // Act
        let centered = centered_rect(80, 80, parent);

        // Assert
        assert_eq!(centered.width, 80);
        assert_eq!(centered.height, 80);
        assert_eq!(centered.x, 10); // (100 - 80) / 2
        assert_eq!(centered.y, 10); // (100 - 80) / 2
    }

    #[test]
    fn test_centered_rect_handles_small_percentages() {
        // Arrange
        let parent = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };

        // Act
        let centered = centered_rect(50, 50, parent);

        // Assert
        assert_eq!(centered.width, 50);
        assert_eq!(centered.height, 50);
        assert_eq!(centered.x, 25); // (100 - 50) / 2
        assert_eq!(centered.y, 25); // (100 - 50) / 2
    }
}
