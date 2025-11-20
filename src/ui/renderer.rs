use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{
    state::AppState,
    ui::{
        components::{
            detail_pane::render_detail_pane, footer::render_footer, search_bar::render_search_bar,
            skill_list::render_skill_list,
        },
        output_panel::render_output_panel,
    },
};

/// Render the main UI
///
/// Draws the complete TUI interface including header, search bar, skill list,
/// detail pane, and footer. Layout is responsive based on terminal width:
/// - Wide terminals (≥80 cols): Side-by-side list and detail pane
/// - Narrow terminals (<80 cols): Stacked list and detail pane
///
/// # Arguments
///
/// * `frame` - The ratatui frame to render into
/// * `state` - The current application state
pub fn render(frame: &mut Frame, state: &AppState) {
    let terminal_width = frame.size().width;
    let theme = state.theme();

    // Main vertical layout: header, search, content area, footer
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(3), // Search bar
            Constraint::Min(0),    // Content area (list + detail)
            Constraint::Length(3), // Footer
        ])
        .split(frame.size());

    // Render header with theme styling and translated title
    let header = Block::default()
        .title(state.translations().app_title)
        .borders(Borders::ALL)
        .border_type(theme.border_style)
        .border_style(theme.border_style())
        .style(theme.header_style());
    frame.render_widget(header, main_chunks[0]);

    // Render search bar (always focused for now) with translated placeholder
    render_search_bar(
        main_chunks[1],
        frame,
        state.search_query(),
        true,
        state.translations().search_placeholder,
        theme,
    );

    // Responsive layout for content area (list + detail pane)
    let (list_area, detail_area) = if terminal_width >= 80 {
        // Wide terminal: side-by-side layout
        let horizontal_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // Skill list (left)
                Constraint::Percentage(50), // Detail pane (right)
            ])
            .split(main_chunks[2]);
        (horizontal_chunks[0], horizontal_chunks[1])
    } else {
        // Narrow terminal: stacked layout
        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60), // Skill list (top)
                Constraint::Percentage(40), // Detail pane (bottom)
            ])
            .split(main_chunks[2]);
        (vertical_chunks[0], vertical_chunks[1])
    };

    // Render skill list
    let skills: Vec<_> = state.filtered_skills().collect();
    render_skill_list(
        list_area,
        frame,
        &skills,
        state.selected_index(),
        state.scroll_offset(),
        theme,
    );

    // Render detail pane (with empty state handling)
    if let Some(selected_skill) = state.selected_skill() {
        render_detail_pane(detail_area, frame, selected_skill, theme);
    } else {
        // Empty state: no skill selected - use translated message
        let empty_message = Paragraph::new(state.translations().empty_skills_message)
            .block(
                Block::default()
                    .title(state.translations().detail_pane_title)
                    .borders(Borders::ALL)
                    .border_type(theme.border_style)
                    .border_style(theme.border_style()),
            )
            .style(Style::default().fg(theme.text_dim));
        frame.render_widget(empty_message, detail_area);
    }

    // Render footer with view mode, input mode, and translations
    render_footer(
        main_chunks[3],
        frame,
        state.view_mode(),
        state.input_mode(),
        state.translations(),
        theme,
    );

    // Render output panel overlay if visible (highest z-order)
    render_output_panel(frame, state);
}
