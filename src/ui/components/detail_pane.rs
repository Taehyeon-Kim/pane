use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::skills::{Skill, SkillSource};
use crate::ui::theme::ThemeConfig;

/// Render the skill detail pane
///
/// Displays detailed information about the selected skill including name,
/// description, estimated time, ID, tags, and source. Handles long descriptions
/// with word wrapping and gracefully displays None values.
///
/// # Arguments
///
/// * `area` - The rectangular area to render into
/// * `frame` - The ratatui frame to render into
/// * `skill` - The skill to display details for
/// * `theme` - Theme configuration for styling
///
/// # Example
///
/// ```no_run
/// use ratatui::backend::TestBackend;
/// use ratatui::Terminal;
/// use pane::skills::{Skill, SkillManifest, SkillSource};
/// use pane::skills::manifest::{UiConfig, UiMode, ContextConfig};
/// use pane::ui::components::detail_pane::render_detail_pane;
/// use pane::ui::theme::ThemeConfig;
/// use std::path::PathBuf;
///
/// let backend = TestBackend::new(80, 24);
/// let mut terminal = Terminal::new(backend).unwrap();
/// let theme = ThemeConfig::default();
/// // Create a test skill
/// let skill = Skill {
///     manifest: SkillManifest {
///         id: "test-skill".to_string(),
///         name: "Test Skill".to_string(),
///         description: "A test skill".to_string(),
///         version: "1.0.0".to_string(),
///         exec: "test".to_string(),
///         args: vec![],
///         tags: vec!["test".to_string()],
///         estimated_time: Some("1-2 min".to_string()),
///         ui: UiConfig { mode: UiMode::Tui, fullscreen: true },
///         context: ContextConfig::default(),
///     },
///     source: SkillSource::Project,
///     manifest_path: PathBuf::from("test.yaml"),
/// };
///
/// terminal.draw(|frame| {
///     render_detail_pane(frame.size(), frame, &skill, &theme);
/// }).unwrap();
/// ```
#[allow(clippy::vec_init_then_push)]
pub fn render_detail_pane(area: Rect, frame: &mut Frame, skill: &Skill, theme: &ThemeConfig) {
    // Build the detail text content
    let mut lines = vec![];

    // Skill name (bold/highlighted with theme)
    lines.push(Line::from(Span::styled(
        skill.manifest.name.as_str(),
        theme.header_style(),
    )));
    lines.push(Line::from("")); // Empty line for spacing

    // Description (wrapped)
    lines.push(Line::from(skill.manifest.description.as_str()));
    lines.push(Line::from("")); // Empty line for spacing

    // Estimated time (with clock icon and theme styling)
    let time_line = match &skill.manifest.estimated_time {
        Some(time) => Line::from(vec![Span::styled(
            format!("⏱ {}", time),
            theme.time_style(),
        )]),
        None => Line::from(vec![Span::styled("⏱ N/A", theme.time_style())]),
    };
    lines.push(time_line);

    // ID
    lines.push(Line::from(format!("ID: {}", skill.manifest.id)));

    // Tags (chip-style formatting with theme)
    if !skill.manifest.tags.is_empty() {
        let mut tag_spans = vec![Span::raw("Tags: ")];
        for (i, tag) in skill.manifest.tags.iter().enumerate() {
            if i > 0 {
                tag_spans.push(Span::raw(" "));
            }
            tag_spans.push(Span::styled(format!("[{}]", tag), theme.tag_style()));
        }
        lines.push(Line::from(tag_spans));
    } else {
        lines.push(Line::from("Tags: (none)"));
    }

    // Source (display enum variant as string)
    let source_text = match skill.source {
        SkillSource::System => "System",
        SkillSource::User => "User",
        SkillSource::Project => "Project",
    };
    lines.push(Line::from(format!("Source: {}", source_text)));

    // Create the paragraph with wrapping enabled and theme styling
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title("Details")
                .borders(Borders::ALL)
                .border_type(theme.border_style)
                .border_style(theme.border_style()),
        )
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skills::{SkillManifest, SkillSource};
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;
    use std::path::PathBuf;

    fn create_test_skill(
        id: &str,
        name: &str,
        description: &str,
        tags: Vec<String>,
        estimated_time: Option<String>,
        source: SkillSource,
    ) -> Skill {
        Skill {
            manifest: SkillManifest {
                id: id.to_string(),
                name: name.to_string(),
                description: description.to_string(),
                version: "1.0.0".to_string(),
                exec: "test".to_string(),
                args: vec![],
                tags,
                estimated_time,
                ui: crate::skills::manifest::UiConfig {
                    mode: crate::skills::manifest::UiMode::Tui,
                    fullscreen: true,
                },
                context: crate::skills::manifest::ContextConfig::default(),
            },
            source,
            manifest_path: PathBuf::from("test.yaml"),
        }
    }

    #[test]
    fn test_render_detail_pane_displays_all_fields() {
        // Arrange
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = ThemeConfig::default();
        let skill = create_test_skill(
            "claude-tips",
            "Claude Code Tips",
            "Displays helpful tips for using Claude Code",
            vec!["tips".to_string(), "claude".to_string()],
            Some("1-3 min".to_string()),
            SkillSource::System,
        );

        // Act & Assert - rendering should complete without panic
        terminal
            .draw(|frame| {
                render_detail_pane(frame.size(), frame, &skill, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_render_detail_pane_handles_long_description() {
        // Arrange
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = ThemeConfig::default();
        let long_description = "This is a very long description that should wrap across multiple lines in the detail pane. It contains enough text to ensure that word wrapping works correctly and handles long content gracefully without truncating important information.";
        let skill = create_test_skill(
            "test-skill",
            "Test Skill",
            long_description,
            vec![],
            None,
            SkillSource::User,
        );

        // Act & Assert - rendering should complete without panic
        terminal
            .draw(|frame| {
                render_detail_pane(frame.size(), frame, &skill, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_render_detail_pane_handles_none_estimated_time() {
        // Arrange
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = ThemeConfig::default();
        let skill = create_test_skill(
            "no-time-skill",
            "No Time Skill",
            "A skill without estimated time",
            vec![],
            None, // No estimated time
            SkillSource::Project,
        );

        // Act & Assert - rendering should complete without panic
        terminal
            .draw(|frame| {
                render_detail_pane(frame.size(), frame, &skill, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_render_detail_pane_displays_tags_inline() {
        // Arrange
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = ThemeConfig::default();
        let skill = create_test_skill(
            "multi-tag-skill",
            "Multi Tag Skill",
            "A skill with multiple tags",
            vec!["tag1".to_string(), "tag2".to_string(), "tag3".to_string()],
            Some("2 min".to_string()),
            SkillSource::User,
        );

        // Act & Assert - rendering should complete without panic
        terminal
            .draw(|frame| {
                render_detail_pane(frame.size(), frame, &skill, &theme);
            })
            .unwrap();
    }

    #[test]
    fn test_render_detail_pane_displays_source_correctly() {
        // Arrange
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = ThemeConfig::default();

        // Test System source
        let system_skill = create_test_skill(
            "system-skill",
            "System Skill",
            "A system skill",
            vec![],
            None,
            SkillSource::System,
        );

        // Act & Assert - rendering should complete without panic
        terminal
            .draw(|frame| {
                render_detail_pane(frame.size(), frame, &system_skill, &theme);
            })
            .unwrap();

        // Test User source
        let user_skill = create_test_skill(
            "user-skill",
            "User Skill",
            "A user skill",
            vec![],
            None,
            SkillSource::User,
        );

        terminal
            .draw(|frame| {
                render_detail_pane(frame.size(), frame, &user_skill, &theme);
            })
            .unwrap();

        // Test Project source
        let project_skill = create_test_skill(
            "project-skill",
            "Project Skill",
            "A project skill",
            vec![],
            None,
            SkillSource::Project,
        );

        terminal
            .draw(|frame| {
                render_detail_pane(frame.size(), frame, &project_skill, &theme);
            })
            .unwrap();
    }
}
