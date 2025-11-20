use pane::skills::manifest::SkillManifest;
use pane::skills::UiMode;
use std::path::PathBuf;

/// Test that the claude-tips skill manifest is valid and contains all required fields
#[test]
fn test_claude_tips_manifest_valid() {
    let manifest_path = PathBuf::from("skills/claude-tips/pane-skill.yaml");

    // Load and parse the manifest
    let manifest =
        SkillManifest::from_yaml_file(manifest_path).expect("Failed to load claude-tips manifest");

    // Assert all required fields are present and correct
    assert_eq!(
        manifest.id, "claude-tips",
        "Manifest ID should be 'claude-tips'"
    );
    assert_eq!(
        manifest.name, "Claude Code Tips",
        "Manifest name should be 'Claude Code Tips'"
    );
    assert_eq!(
        manifest.description, "Browse a curated archive of Claude Code usage tips.",
        "Manifest description should match expected value"
    );
    assert_eq!(
        manifest.version, "0.1.0",
        "Manifest version should be '0.1.0'"
    );
    assert_eq!(manifest.exec, "claude-tips", "Exec should be 'claude-tips'");
    assert!(manifest.args.is_empty(), "Args should be empty");

    // Assert tags contain expected values
    assert_eq!(manifest.tags.len(), 3, "Should have 3 tags");
    assert!(
        manifest.tags.contains(&"tips".to_string()),
        "Tags should contain 'tips'"
    );
    assert!(
        manifest.tags.contains(&"claude".to_string()),
        "Tags should contain 'claude'"
    );
    assert!(
        manifest.tags.contains(&"coding".to_string()),
        "Tags should contain 'coding'"
    );

    // Assert estimated_time is set
    assert_eq!(
        manifest.estimated_time,
        Some("1–3 min".to_string()),
        "Estimated time should be '1–3 min'"
    );

    // Assert ui.mode is TUI
    assert_eq!(manifest.ui.mode, UiMode::Tui, "UI mode should be TUI");

    // Assert ui.fullscreen is true
    assert_eq!(manifest.ui.fullscreen, true, "UI fullscreen should be true");

    // Assert context configuration is set to false (self-contained skill)
    assert_eq!(
        manifest.context.pass_cwd, false,
        "pass_cwd should be false for self-contained skill"
    );
    assert_eq!(
        manifest.context.pass_git_root, false,
        "pass_git_root should be false for self-contained skill"
    );
    assert_eq!(
        manifest.context.pass_project_name, false,
        "pass_project_name should be false for self-contained skill"
    );
    assert_eq!(
        manifest.context.pass_stdin_json, false,
        "pass_stdin_json should be false for self-contained skill"
    );
}

/// Test that the claude-tips binary exists and is executable
#[test]
fn test_claude_tips_executable_exists() {
    // Build the project first (this test assumes cargo build has been run)
    let debug_binary = PathBuf::from("target/debug/claude-tips");
    let release_binary = PathBuf::from("target/release/claude-tips");

    // Check if at least one binary exists
    let binary_exists = debug_binary.exists() || release_binary.exists();
    assert!(
        binary_exists,
        "claude-tips binary should exist in target/debug or target/release"
    );

    // Check execute permissions on whichever binary exists
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let binary_path = if debug_binary.exists() {
            debug_binary
        } else {
            release_binary
        };

        let metadata = std::fs::metadata(&binary_path).expect("Failed to read binary metadata");
        let permissions = metadata.permissions();
        let mode = permissions.mode();

        // Check if executable bit is set (0o111 = --x--x--x)
        assert!(mode & 0o111 != 0, "Binary should have execute permissions");
    }
}
