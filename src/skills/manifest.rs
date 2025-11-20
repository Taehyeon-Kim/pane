use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

/// UI interaction mode for skills
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UiMode {
    /// Skill takes over terminal with its own TUI
    Tui,
    /// Skill prints to stdout, results embedded in launcher
    Inline,
}

/// UI configuration for skill display
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UiConfig {
    /// How the skill interacts with the terminal
    pub mode: UiMode,
    /// Whether skill uses fullscreen mode (default: true)
    #[serde(default = "default_fullscreen")]
    pub fullscreen: bool,
}

fn default_fullscreen() -> bool {
    true
}

/// Configuration for which context fields to pass to a skill
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContextConfig {
    /// Whether to pass current working directory
    pub pass_cwd: bool,
    /// Whether to detect and pass git root
    pub pass_git_root: bool,
    /// Whether to pass project name
    pub pass_project_name: bool,
    /// Whether to send full context as JSON to stdin
    pub pass_stdin_json: bool,
}

impl Default for ContextConfig {
    fn default() -> Self {
        Self {
            pass_cwd: true,
            pass_git_root: true,
            pass_project_name: true,
            pass_stdin_json: false,
        }
    }
}

/// Skill manifest representation from pane-skill.yaml
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkillManifest {
    /// Unique identifier (lowercase alphanumeric + hyphens)
    pub id: String,
    /// Human-readable display name
    pub name: String,
    /// One or two sentence explanation
    pub description: String,
    /// Semantic version of the skill
    #[serde(default = "default_version")]
    pub version: String,
    /// Executable or script name/path to run
    pub exec: String,
    /// Command-line arguments to pass to executable
    #[serde(default)]
    pub args: Vec<String>,
    /// Searchable tags for filtering
    #[serde(default)]
    pub tags: Vec<String>,
    /// Human-readable time estimate (e.g., "1–3 min")
    pub estimated_time: Option<String>,
    /// UI configuration
    pub ui: UiConfig,
    /// Context configuration
    #[serde(default)]
    pub context: ContextConfig,
}

fn default_version() -> String {
    "0.1.0".to_string()
}

// These methods will be used in future stories for skill discovery
#[allow(dead_code)]
impl SkillManifest {
    /// Parse a skill manifest from a YAML string
    ///
    /// # Arguments
    ///
    /// * `yaml_str` - YAML string containing the manifest
    ///
    /// # Returns
    ///
    /// * `Result<SkillManifest>` - Parsed and validated manifest
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - YAML syntax is invalid
    /// - Required fields are missing
    /// - Validation fails
    pub fn from_yaml_str(yaml_str: &str) -> Result<Self> {
        let manifest: SkillManifest =
            serde_yaml::from_str(yaml_str).context("Failed to parse YAML manifest")?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Parse a skill manifest from a YAML file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the manifest file
    ///
    /// # Returns
    ///
    /// * `Result<SkillManifest>` - Parsed and validated manifest
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - File cannot be read
    /// - YAML syntax is invalid
    /// - Required fields are missing
    /// - Validation fails
    pub fn from_yaml_file(path: PathBuf) -> Result<Self> {
        let file = std::fs::File::open(&path)
            .with_context(|| format!("Failed to open manifest file: {:?}", path))?;
        let manifest: SkillManifest = serde_yaml::from_reader(file)
            .with_context(|| format!("Failed to parse manifest file: {:?}", path))?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Validate the manifest fields
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Ok if validation passes
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Required fields are empty
    /// - ID format is invalid (must be lowercase alphanumeric + hyphens)
    pub fn validate(&self) -> Result<()> {
        // Validate required fields are non-empty
        if self.id.is_empty() {
            anyhow::bail!("Skill id cannot be empty");
        }
        if self.name.is_empty() {
            anyhow::bail!("Skill name cannot be empty");
        }
        if self.description.is_empty() {
            anyhow::bail!("Skill description cannot be empty");
        }
        if self.exec.is_empty() {
            anyhow::bail!("Skill exec cannot be empty");
        }

        // Validate id format: lowercase alphanumeric + hyphens only
        let id_regex =
            regex::Regex::new(r"^[a-z0-9-]+$").context("Failed to compile id validation regex")?;
        if !id_regex.is_match(&self.id) {
            anyhow::bail!(
                "Invalid skill id '{}': must be lowercase alphanumeric with hyphens only",
                self.id
            );
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    fn create_valid_manifest() -> SkillManifest {
        SkillManifest {
            id: "test-skill".to_string(),
            name: "Test Skill".to_string(),
            description: "A test skill".to_string(),
            version: "0.1.0".to_string(),
            exec: "./test.sh".to_string(),
            args: vec![],
            tags: vec![],
            estimated_time: None,
            ui: UiConfig {
                mode: UiMode::Tui,
                fullscreen: true,
            },
            context: ContextConfig::default(),
        }
    }

    #[test]
    fn test_from_yaml_str_valid_minimal_manifest_succeeds() {
        // Arrange
        let yaml = r#"
id: test-skill
name: Test Skill
description: A test skill
exec: ./test.sh
ui:
  mode: tui
"#;

        // Act
        let result = SkillManifest::from_yaml_str(yaml);

        // Assert
        assert!(result.is_ok());
        let manifest = result.unwrap();
        assert_eq!(manifest.id, "test-skill");
        assert_eq!(manifest.name, "Test Skill");
        assert_eq!(manifest.description, "A test skill");
        assert_eq!(manifest.exec, "./test.sh");
        assert_eq!(manifest.version, "0.1.0"); // default
        assert_eq!(manifest.ui.fullscreen, true); // default
        assert!(manifest.args.is_empty());
        assert!(manifest.tags.is_empty());
        assert_eq!(manifest.estimated_time, None);
    }

    #[test]
    fn test_from_yaml_str_valid_full_manifest_succeeds() {
        // Arrange
        let yaml = r#"
id: my-skill
name: My Awesome Skill
description: This is a comprehensive example skill manifest
version: 1.2.3
exec: /usr/local/bin/my-skill
args:
  - --verbose
  - --color
tags:
  - productivity
  - development
  - automation
estimated_time: 2-5 min
ui:
  mode: tui
  fullscreen: true
context:
  pass_cwd: true
  pass_git_root: true
  pass_project_name: true
  pass_stdin_json: false
"#;

        // Act
        let result = SkillManifest::from_yaml_str(yaml);

        // Assert
        assert!(result.is_ok());
        let manifest = result.unwrap();
        assert_eq!(manifest.id, "my-skill");
        assert_eq!(manifest.name, "My Awesome Skill");
        assert_eq!(manifest.version, "1.2.3");
        assert_eq!(manifest.exec, "/usr/local/bin/my-skill");
        assert_eq!(manifest.args, vec!["--verbose", "--color"]);
        assert_eq!(
            manifest.tags,
            vec!["productivity", "development", "automation"]
        );
        assert_eq!(manifest.estimated_time, Some("2-5 min".to_string()));
        assert_eq!(manifest.ui.mode, UiMode::Tui);
        assert_eq!(manifest.ui.fullscreen, true);
        assert_eq!(manifest.context.pass_cwd, true);
        assert_eq!(manifest.context.pass_git_root, true);
        assert_eq!(manifest.context.pass_project_name, true);
        assert_eq!(manifest.context.pass_stdin_json, false);
    }

    #[test]
    fn test_from_yaml_str_invalid_yaml_fails() {
        // Arrange
        let yaml = r#"
id: test-skill
name: Test Skill
invalid yaml syntax: [unclosed bracket
"#;

        // Act
        let result = SkillManifest::from_yaml_str(yaml);

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Failed to parse YAML manifest"));
    }

    #[rstest]
    #[case("MySkill", "must be lowercase")]
    #[case("my skill", "alphanumeric with hyphens only")]
    #[case("my_skill", "alphanumeric with hyphens only")]
    fn test_validate_invalid_id_format_fails(#[case] invalid_id: &str, #[case] expected_msg: &str) {
        // Arrange
        let mut manifest = create_valid_manifest();
        manifest.id = invalid_id.to_string();

        // Act
        let result = manifest.validate();

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains(expected_msg));
    }

    #[rstest]
    #[case("", "id cannot be empty")]
    fn test_validate_empty_id_fails(#[case] empty_id: &str, #[case] expected_msg: &str) {
        // Arrange
        let mut manifest = create_valid_manifest();
        manifest.id = empty_id.to_string();

        // Act
        let result = manifest.validate();

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains(expected_msg));
    }

    #[test]
    fn test_validate_empty_name_fails() {
        // Arrange
        let mut manifest = create_valid_manifest();
        manifest.name = "".to_string();

        // Act
        let result = manifest.validate();

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("name cannot be empty"));
    }

    #[test]
    fn test_validate_empty_description_fails() {
        // Arrange
        let mut manifest = create_valid_manifest();
        manifest.description = "".to_string();

        // Act
        let result = manifest.validate();

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("description cannot be empty"));
    }

    #[test]
    fn test_validate_empty_exec_fails() {
        // Arrange
        let mut manifest = create_valid_manifest();
        manifest.exec = "".to_string();

        // Act
        let result = manifest.validate();

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("exec cannot be empty"));
    }

    #[test]
    fn test_context_config_defaults() {
        // Arrange & Act
        let context = ContextConfig::default();

        // Assert
        assert_eq!(context.pass_cwd, true);
        assert_eq!(context.pass_git_root, true);
        assert_eq!(context.pass_project_name, true);
        assert_eq!(context.pass_stdin_json, false);
    }

    #[test]
    fn test_ui_config_fullscreen_default() {
        // Arrange
        let yaml = r#"
id: test-skill
name: Test Skill
description: A test skill
exec: ./test.sh
ui:
  mode: tui
"#;

        // Act
        let result = SkillManifest::from_yaml_str(yaml);

        // Assert
        assert!(result.is_ok());
        let manifest = result.unwrap();
        assert_eq!(manifest.ui.fullscreen, true);
    }

    #[test]
    fn test_from_yaml_file_valid_manifest_succeeds() {
        // Arrange
        let path = PathBuf::from("tests/fixtures/skills/minimal-valid.yaml");

        // Act
        let result = SkillManifest::from_yaml_file(path);

        // Assert
        assert!(result.is_ok());
        let manifest = result.unwrap();
        assert_eq!(manifest.id, "claude-tips");
        assert_eq!(manifest.name, "Claude Code Tips");
    }

    #[test]
    fn test_from_yaml_file_nonexistent_file_fails() {
        // Arrange
        let path = PathBuf::from("tests/fixtures/skills/nonexistent.yaml");

        // Act
        let result = SkillManifest::from_yaml_file(path);

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Failed to open manifest file"));
    }
}
