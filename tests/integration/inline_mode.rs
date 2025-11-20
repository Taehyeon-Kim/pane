use pane::{
    config::Config,
    skills::{
        manifest::{ContextConfig, SkillManifest, UiConfig, UiMode},
        Skill, SkillSource,
    },
    state::AppState,
};
use std::path::PathBuf;

/// Helper function to create a test skill with specified UI mode
fn create_test_skill(id: &str, name: &str, ui_mode: UiMode) -> Skill {
    Skill {
        manifest: SkillManifest {
            id: id.to_string(),
            name: name.to_string(),
            description: format!("Test {} skill", name),
            version: "1.0.0".to_string(),
            exec: "echo".to_string(),
            args: vec!["test".to_string()],
            tags: vec!["test".to_string()],
            estimated_time: Some("<1 min".to_string()),
            ui: UiConfig {
                mode: ui_mode,
                fullscreen: true,
            },
            context: ContextConfig::default(),
        },
        source: SkillSource::Project,
        manifest_path: PathBuf::from(format!("/test/{}.yaml", id)),
    }
}

#[test]
fn test_app_state_initializes_with_inline_skill() {
    // Arrange
    let inline_skill = create_test_skill("inline-test", "Inline Test", UiMode::Inline);
    let skills = vec![inline_skill];
    let config = Config::default();

    // Act
    let state = AppState::new(skills, config);

    // Assert - output panel should not be visible initially
    assert!(!state.is_output_panel_visible());
    assert!(!state.is_executing_inline());
}

#[test]
fn test_app_state_supports_mixed_ui_modes() {
    // Arrange
    let inline_skill = create_test_skill("inline-1", "Inline Skill", UiMode::Inline);
    let tui_skill = create_test_skill("tui-1", "TUI Skill", UiMode::Tui);
    let skills = vec![inline_skill, tui_skill];
    let config = Config::default();

    // Act
    let state = AppState::new(skills, config);

    // Assert - state should be initialized correctly
    assert!(!state.is_output_panel_visible());
    assert!(!state.should_quit());
}

#[test]
fn test_selected_skill_returns_inline_skill() {
    // Arrange
    let inline_skill = create_test_skill("inline-test", "Inline Test", UiMode::Inline);
    let skills = vec![inline_skill];
    let config = Config::default();
    let state = AppState::new(skills, config);

    // Act
    let selected = state.selected_skill();

    // Assert
    assert!(selected.is_some());
    let skill = selected.unwrap();
    assert_eq!(skill.manifest.id, "inline-test");
    assert_eq!(skill.manifest.ui.mode, UiMode::Inline);
}

#[test]
fn test_output_panel_visibility_toggle() {
    // Arrange
    let inline_skill = create_test_skill("inline-test", "Inline Test", UiMode::Inline);
    let skills = vec![inline_skill];
    let config = Config::default();
    let mut state = AppState::new(skills, config);

    // Assert initial state
    assert!(!state.is_output_panel_visible());

    // Act - create mock output
    let output = pane::skills::output::SkillOutput {
        stdout: "Test output".to_string(),
        stderr: String::new(),
        exit_code: Some(0),
        truncated: false,
        execution_time: std::time::Duration::from_millis(100),
    };

    state.show_output_panel(output);

    // Assert - panel should be visible
    assert!(state.is_output_panel_visible());

    // Act - hide panel
    state.hide_output_panel();

    // Assert - panel should be hidden
    assert!(!state.is_output_panel_visible());
}

#[test]
fn test_recent_skills_tracking_api_exists() {
    // Arrange
    let inline_skill = create_test_skill("inline-test", "Inline Test", UiMode::Inline);
    let skills = vec![inline_skill];
    let config = Config::default();
    let mut state = AppState::new(skills, config);

    // Act - add skill to recent (tests that the method exists and works)
    state.add_to_recent("inline-test".to_string());

    // Assert - no panic, method executed successfully
    // Note: Recent skills tracking is internal state, tested via integration
}

#[test]
fn test_recent_skills_tracking_multiple_executions() {
    // Arrange
    let inline_skill_1 = create_test_skill("inline-1", "Inline 1", UiMode::Inline);
    let inline_skill_2 = create_test_skill("inline-2", "Inline 2", UiMode::Inline);
    let skills = vec![inline_skill_1, inline_skill_2];
    let config = Config::default();
    let mut state = AppState::new(skills, config);

    // Act - execute multiple skills
    state.add_to_recent("inline-1".to_string());
    state.add_to_recent("inline-2".to_string());

    // Assert - no panic, both adds succeeded
    // Note: Recent skills list is internal state, observable via view mode filter
}

#[test]
fn test_inline_skill_manifest_validation() {
    // Arrange
    let skill = create_test_skill("inline-valid", "Valid Inline", UiMode::Inline);

    // Assert
    assert_eq!(skill.manifest.ui.mode, UiMode::Inline);
    assert_eq!(skill.manifest.id, "inline-valid");
    assert_eq!(skill.manifest.name, "Valid Inline");
    assert!(!skill.manifest.exec.is_empty());
}

#[test]
fn test_tui_skill_manifest_validation() {
    // Arrange
    let skill = create_test_skill("tui-valid", "Valid TUI", UiMode::Tui);

    // Assert
    assert_eq!(skill.manifest.ui.mode, UiMode::Tui);
    assert_eq!(skill.manifest.id, "tui-valid");
    assert_eq!(skill.manifest.name, "Valid TUI");
    assert!(!skill.manifest.exec.is_empty());
}

#[test]
fn test_context_aware_esc_handling_with_output_panel() {
    // Arrange
    let inline_skill = create_test_skill("inline-test", "Inline Test", UiMode::Inline);
    let skills = vec![inline_skill];
    let config = Config::default();
    let mut state = AppState::new(skills, config);

    // Show output panel
    let output = pane::skills::output::SkillOutput {
        stdout: "Test".to_string(),
        stderr: String::new(),
        exit_code: Some(0),
        truncated: false,
        execution_time: std::time::Duration::from_millis(50),
    };
    state.show_output_panel(output);

    assert!(state.is_output_panel_visible());
    assert!(!state.should_quit());

    // Act - hide output panel (Esc when panel visible)
    state.hide_output_panel();

    // Assert - panel hidden but app still running
    assert!(!state.is_output_panel_visible());
    assert!(!state.should_quit());
}

#[test]
fn test_context_aware_esc_handling_without_output_panel() {
    // Arrange
    let inline_skill = create_test_skill("inline-test", "Inline Test", UiMode::Inline);
    let skills = vec![inline_skill];
    let config = Config::default();
    let mut state = AppState::new(skills, config);

    assert!(!state.is_output_panel_visible());
    assert!(!state.should_quit());

    // Act - quit when panel not visible (Esc in skill list)
    state.quit();

    // Assert - app should quit
    assert!(state.should_quit());
}

#[test]
fn test_scroll_output_panel_content() {
    // Arrange
    let inline_skill = create_test_skill("inline-test", "Inline Test", UiMode::Inline);
    let skills = vec![inline_skill];
    let config = Config::default();
    let mut state = AppState::new(skills, config);

    // Create output with multiple lines for scrolling
    let multi_line_output = (0..30)
        .map(|i| format!("Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let output = pane::skills::output::SkillOutput {
        stdout: multi_line_output,
        stderr: String::new(),
        exit_code: Some(0),
        truncated: false,
        execution_time: std::time::Duration::from_millis(100),
    };
    state.show_output_panel(output);

    // Act & Assert - scroll down
    state.scroll_output_down();
    // Note: We can't directly test scroll offset as it's private,
    // but we can verify the method doesn't panic

    // Act & Assert - scroll up
    state.scroll_output_up();
    // Verify no panic
}
