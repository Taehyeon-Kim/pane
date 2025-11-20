//! UI rendering for the Claude Code Tips browser.
//!
//! This module handles all terminal UI rendering using ratatui widgets
//! and layout management. The UI consists of a header, main content area
//! (tips list or detail view), optional search bar, and footer with key hints.

use crate::app::AppState;
use crate::model::Tip;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Main render function for the tips browser UI.
///
/// Renders the complete UI including header, main content area (tips list
/// or detail view), optional search bar, and footer with contextual key hints.
/// Layout adapts based on current application state and mode.
///
/// # Arguments
///
/// * `frame` - Mutable reference to the terminal frame
/// * `state` - Immutable reference to application state
///
/// # Layout
///
/// The UI is divided into vertical sections:
/// - Header (3 lines): Title and subtitle
/// - Search bar (3 lines, if search mode active)
/// - Main area (flexible): Tips list or detail view
/// - Footer (1 line): Key hints
pub fn render(frame: &mut Frame, state: &AppState) {
    let size = frame.size();

    // Calculate layout constraints based on search mode
    let main_chunks = if state.is_search_mode() {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Length(3), // Search bar
                Constraint::Min(0),    // Main content
                Constraint::Length(1), // Footer
            ])
            .split(size)
    } else {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main content
                Constraint::Length(1), // Footer
            ])
            .split(size)
    };

    // Render header
    render_header(main_chunks[0], frame);

    // Render search bar if in search mode
    let (main_area, footer_area) = if state.is_search_mode() {
        render_search_bar(
            main_chunks[1],
            frame,
            state.search_query(),
            state.filtered_count(),
        );
        (main_chunks[2], main_chunks[3])
    } else {
        (main_chunks[1], main_chunks[2])
    };

    // Render main content area (list or detail view)
    if state.is_detail_mode() {
        if let Some(tip) = state.selected_tip() {
            render_detail_view(main_area, frame, tip);
        }
    } else {
        render_tips_list(main_area, frame, state);
    }

    // Render footer
    render_footer(footer_area, frame, state);
}

/// Renders the header section with application title.
///
/// # Arguments
///
/// * `area` - The rectangular area to render the header in
/// * `frame` - Mutable reference to the terminal frame
fn render_header(area: Rect, frame: &mut Frame) {
    let header = Paragraph::new(vec![
        Line::from(Span::styled(
            "Claude Code Tips",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "Browse and search helpful tips for using Claude Code",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(Block::default().borders(Borders::BOTTOM));

    frame.render_widget(header, area);
}

/// Renders the search bar with current query and result count.
///
/// # Arguments
///
/// * `area` - The rectangular area to render the search bar in
/// * `frame` - Mutable reference to the terminal frame
/// * `query` - The current search query string
/// * `result_count` - Number of tips matching the search
fn render_search_bar(area: Rect, frame: &mut Frame, query: &str, result_count: usize) {
    let search_text = if result_count == 0 && !query.is_empty() {
        format!("Search: {} (No tips found)", query)
    } else {
        format!("Search: {} ({} tips)", query, result_count)
    };

    let search_bar = Paragraph::new(search_text)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Search Mode")
                .style(Style::default().fg(Color::Yellow)),
        );

    frame.render_widget(search_bar, area);
}

/// Renders the scrollable tips list with highlighting.
///
/// Displays all filtered tips with title, category, and tags. The selected
/// tip is highlighted with a different style. Handles scrolling automatically
/// when the list exceeds the visible area.
///
/// # Arguments
///
/// * `area` - The rectangular area to render the list in
/// * `frame` - Mutable reference to the terminal frame
/// * `state` - Immutable reference to application state
fn render_tips_list(area: Rect, frame: &mut Frame, state: &AppState) {
    let tips = state.filtered_tips();

    if tips.is_empty() {
        let empty_msg = Paragraph::new("No tips found. Press Esc to exit search mode.")
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title("Tips List"));
        frame.render_widget(empty_msg, area);
        return;
    }

    let items: Vec<ListItem> = tips
        .iter()
        .enumerate()
        .map(|(idx, tip)| {
            let category = tip
                .category
                .as_ref()
                .map(|c| format!("[{}]", c))
                .unwrap_or_else(|| String::from("[uncategorized]"));

            let tags = if tip.tags.is_empty() {
                String::new()
            } else {
                format!(" #{}", tip.tags.join(" #"))
            };

            let content = format!("{} {} {}", tip.title, category, tags);

            let style = if idx == state.selected_index() {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title(format!(
        "Tips ({}/{})",
        state.filtered_count(),
        state.all_tips().len()
    )));

    frame.render_widget(list, area);
}

/// Renders the detail view for a single tip.
///
/// Displays the full tip content including title, category, text (with wrapping),
/// and tags. The text content is wrapped to fit the terminal width.
///
/// # Arguments
///
/// * `area` - The rectangular area to render the detail view in
/// * `frame` - Mutable reference to the terminal frame
/// * `tip` - The tip to display in detail
fn render_detail_view(area: Rect, frame: &mut Frame, tip: &Tip) {
    let category = tip
        .category
        .as_ref()
        .map(|c| format!("Category: {}", c))
        .unwrap_or_else(|| String::from("Category: uncategorized"));

    let tags = if tip.tags.is_empty() {
        String::from("Tags: none")
    } else {
        format!("Tags: {}", tip.tags.join(", "))
    };

    let content = vec![
        Line::from(Span::styled(
            &tip.title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(category, Style::default().fg(Color::Yellow))),
        Line::from(Span::styled(tags, Style::default().fg(Color::Yellow))),
        Line::from(""),
        Line::from("─".repeat(area.width as usize)),
        Line::from(""),
        Line::from(tip.text.as_str()),
    ];

    let paragraph = Paragraph::new(content)
        .block(Block::default().borders(Borders::ALL).title("Tip Detail"))
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

/// Renders the footer with contextual key hints.
///
/// Displays different key hints based on the current application mode:
/// - Normal mode: Navigation, view, search, exit keys
/// - Search mode: Search input instructions
/// - Detail mode: Return to list instruction
///
/// # Arguments
///
/// * `area` - The rectangular area to render the footer in
/// * `frame` - Mutable reference to the terminal frame
/// * `state` - Immutable reference to application state
fn render_footer(area: Rect, frame: &mut Frame, state: &AppState) {
    let key_hints = if state.is_detail_mode() {
        "Esc: back to list"
    } else if state.is_search_mode() {
        "Type to search | Esc: cancel"
    } else {
        "↑/↓ or j/k: navigate | Enter: view detail | /: search | Esc: exit"
    };

    let footer = Paragraph::new(key_hints).style(Style::default().fg(Color::DarkGray));

    frame.render_widget(footer, area);
}
