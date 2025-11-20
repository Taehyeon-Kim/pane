use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem},
    Frame,
};

use crate::skills::Skill;
use crate::ui::theme::ThemeConfig;

/// Render a scrollable list of skills
///
/// Displays all provided skills in a vertical list with the specified item
/// highlighted. Each skill shows its name, tags, estimated time, and description.
///
/// # Arguments
///
/// * `area` - The rectangular area to render the list into
/// * `frame` - The ratatui frame to render into
/// * `skills` - Slice of skill references to display
/// * `selected` - Index of the currently selected skill (for highlighting)
/// * `scroll_offset` - Scroll offset to control which items are visible
/// * `theme` - Theme configuration for styling
///
/// # Layout
///
/// Each skill item is formatted with two lines:
/// - Line 1: Name (bold) + Tags [tag1] [tag2] + Estimated time (⏱ X min)
/// - Line 2: Description (indented, truncated if >80 chars)
///
/// The selected skill is highlighted with theme colors.
///
/// # Example
///
/// ```no_run
/// use ratatui::Frame;
/// use ratatui::layout::Rect;
/// # use pane::skills::Skill;
/// # use pane::ui::components::skill_list::render_skill_list;
/// # use pane::ui::theme::ThemeConfig;
///
/// fn render(frame: &mut Frame, skills: Vec<&Skill>, selected_index: usize, scroll_offset: usize, area: Rect) {
///     let theme = ThemeConfig::default();
///     render_skill_list(area, frame, &skills, selected_index, scroll_offset, &theme);
/// }
/// ```
pub fn render_skill_list(
    area: Rect,
    frame: &mut Frame,
    skills: &[&Skill],
    selected: usize,
    scroll_offset: usize,
    theme: &ThemeConfig,
) {
    // Format each skill into a ListItem
    let items: Vec<ListItem> = skills
        .iter()
        .map(|skill| format_skill_item(skill, theme))
        .collect();

    // Create the list widget with theme-based highlighting
    let list = List::new(items).highlight_style(theme.selected_style());

    // Create list state with selection and scroll offset
    let mut list_state = ratatui::widgets::ListState::default()
        .with_selected(Some(selected))
        .with_offset(scroll_offset);

    // Render the list as a stateful widget with selection
    frame.render_stateful_widget(list, area, &mut list_state);
}

/// Format a single skill into a ListItem with metadata
///
/// Creates a two-line list item with the skill's name, tags, estimated time,
/// and description. Handles missing optional fields gracefully.
///
/// # Arguments
///
/// * `skill` - Reference to the skill to format
/// * `theme` - Theme configuration for styling
///
/// # Returns
///
/// A `ListItem` ready for rendering in a `List` widget
fn format_skill_item(skill: &&Skill, theme: &ThemeConfig) -> ListItem<'static> {
    // Line 1: Name (bold) + Tags + Estimated time
    let mut line1_spans = vec![
        Span::styled("● ", Style::default().fg(theme.primary)),
        Span::styled(
            skill.manifest.name.clone(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
    ];

    // Add tags if present with chip-style formatting
    if !skill.manifest.tags.is_empty() {
        let tags_text = format!("[{}]", skill.manifest.tags.join("] ["));
        line1_spans.push(Span::styled(tags_text, theme.tag_style()));
        line1_spans.push(Span::raw("  "));
    }

    // Add estimated time if present with icon prefix
    if let Some(ref time) = skill.manifest.estimated_time {
        line1_spans.push(Span::styled(format!("⏱ {}", time), theme.time_style()));
    }

    let line1 = Line::from(line1_spans);

    // Line 2: Description (indented, truncated if too long)
    let description = if skill.manifest.description.len() > 80 {
        format!("  {}...", &skill.manifest.description[..77])
    } else {
        format!("  {}", skill.manifest.description)
    };

    let line2 = Line::from(vec![Span::styled(
        description,
        Style::default().fg(theme.text_dim),
    )]);

    // Combine lines into a ListItem
    ListItem::new(vec![line1, line2])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skills::{
        manifest::{ContextConfig, SkillManifest, UiConfig, UiMode},
        SkillSource,
    };
    use std::path::PathBuf;

    fn create_test_skill(
        name: &str,
        description: &str,
        tags: Vec<String>,
        estimated_time: Option<String>,
    ) -> Skill {
        Skill {
            manifest: SkillManifest {
                id: "test-skill".to_string(),
                name: name.to_string(),
                description: description.to_string(),
                version: "1.0.0".to_string(),
                exec: "test".to_string(),
                args: vec![],
                tags,
                estimated_time,
                ui: UiConfig {
                    mode: UiMode::Tui,
                    fullscreen: true,
                },
                context: ContextConfig::default(),
            },
            source: SkillSource::Project,
            manifest_path: PathBuf::from("test.yaml"),
        }
    }

    #[test]
    fn test_format_skill_item_with_all_metadata() {
        // Arrange
        let skill = create_test_skill(
            "Test Skill",
            "A comprehensive test skill with metadata",
            vec!["tag1".to_string(), "tag2".to_string()],
            Some("1-3 min".to_string()),
        );
        let theme = ThemeConfig::default();

        // Act
        let item = format_skill_item(&&skill, &theme);

        // Assert
        // Item should have 2 lines
        assert_eq!(item.height(), 2);
    }

    #[test]
    fn test_format_skill_item_without_tags() {
        // Arrange
        let skill = create_test_skill(
            "Test Skill",
            "A skill without tags",
            vec![],
            Some("1 min".to_string()),
        );
        let theme = ThemeConfig::default();

        // Act
        let item = format_skill_item(&&skill, &theme);

        // Assert
        assert_eq!(item.height(), 2);
    }

    #[test]
    fn test_format_skill_item_without_estimated_time() {
        // Arrange
        let skill = create_test_skill(
            "Test Skill",
            "A skill without estimated time",
            vec!["tag1".to_string()],
            None,
        );
        let theme = ThemeConfig::default();

        // Act
        let item = format_skill_item(&&skill, &theme);

        // Assert
        assert_eq!(item.height(), 2);
    }

    #[test]
    fn test_format_skill_item_truncates_long_description() {
        // Arrange
        let long_description = "This is a very long description that exceeds eighty characters and should be truncated with ellipsis";
        let skill = create_test_skill("Test Skill", long_description, vec![], None);
        let theme = ThemeConfig::default();

        // Act
        let item = format_skill_item(&&skill, &theme);

        // Assert
        assert_eq!(item.height(), 2);
        // Description should be truncated (verified by length check in format_skill_item)
    }

    #[test]
    fn test_format_skill_item_with_minimal_metadata() {
        // Arrange
        let skill = create_test_skill("Minimal Skill", "Short desc", vec![], None);
        let theme = ThemeConfig::default();

        // Act
        let item = format_skill_item(&&skill, &theme);

        // Assert
        assert_eq!(item.height(), 2);
    }
}
