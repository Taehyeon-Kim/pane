use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::config::Config;
use crate::skills::{manifest::ContextConfig, Skill};

/// Context information passed to skills during execution
///
/// Contains environment details gathered from the user's working directory,
/// git repository (if present), and application configuration.
#[derive(Debug, Clone, PartialEq)]
pub struct SkillContext {
    /// ID of the skill being executed
    pub skill_id: String,
    /// Name of the skill
    pub skill_name: String,
    /// Current working directory where pane was launched
    pub cwd: PathBuf,
    /// Git repository root (if detected)
    pub git_root: Option<PathBuf>,
    /// Project/repo name derived from git root or cwd
    pub project_name: Option<String>,
    /// Path to Pane's config file
    pub config_path: PathBuf,
    /// Additional arguments passed to the skill
    pub args: Vec<String>,
}

impl SkillContext {
    /// Build a SkillContext from the given skill and configuration
    ///
    /// Gathers context information including git repository detection,
    /// project name extraction, and config path resolution.
    ///
    /// # Arguments
    ///
    /// * `skill` - The skill to build context for
    /// * `config` - User configuration
    ///
    /// # Returns
    ///
    /// A SkillContext containing gathered environment details
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Current working directory cannot be determined
    /// - Config path cannot be resolved
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use pane::context::SkillContext;
    /// # use pane::{Config, skills::Skill};
    /// # fn example(skill: &Skill, config: &Config) -> anyhow::Result<()> {
    /// let context = SkillContext::build(skill, config)?;
    /// println!("Project: {:?}", context.project_name);
    /// # Ok(())
    /// # }
    /// ```
    pub fn build(skill: &Skill, _config: &Config) -> Result<Self> {
        // Get current working directory
        let cwd = env::current_dir().context("Failed to determine current working directory")?;

        // Detect git repository root
        let git_root = detect_git_root(&cwd);

        // Extract project name from git root or cwd
        let project_name = if let Some(ref root) = git_root {
            extract_project_name(root)
        } else {
            extract_project_name(&cwd)
        };

        // Get config path
        let config_path = get_config_path();

        Ok(SkillContext {
            skill_id: skill.manifest.id.clone(),
            skill_name: skill.manifest.name.clone(),
            cwd,
            git_root,
            project_name,
            config_path,
            args: skill.manifest.args.clone(),
        })
    }

    /// Prepare environment variables for skill execution
    ///
    /// Creates PANE_* environment variables based on the context and
    /// the skill's context configuration flags.
    ///
    /// # Arguments
    ///
    /// * `context_config` - Configuration controlling which context fields to pass
    ///
    /// # Returns
    ///
    /// HashMap of environment variable name to value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use pane::context::SkillContext;
    /// # use pane::skills::manifest::ContextConfig;
    /// # fn example(context: &SkillContext) -> anyhow::Result<()> {
    /// let context_config = ContextConfig::default();
    /// let env_vars = context.prepare_environment(&context_config);
    /// assert!(env_vars.contains_key("PANE_ID"));
    /// assert!(env_vars.contains_key("PANE_NAME"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn prepare_environment(&self, context_config: &ContextConfig) -> HashMap<String, String> {
        let mut env_vars = HashMap::new();

        // Always pass skill ID and name
        env_vars.insert("PANE_ID".to_string(), self.skill_id.clone());
        env_vars.insert("PANE_NAME".to_string(), self.skill_name.clone());

        // Pass config path
        env_vars.insert(
            "PANE_CONFIG_PATH".to_string(),
            self.config_path.to_string_lossy().to_string(),
        );

        // Conditionally pass context fields based on context_config flags
        if context_config.pass_cwd {
            env_vars.insert(
                "PANE_CWD".to_string(),
                self.cwd.to_string_lossy().to_string(),
            );
        }

        if context_config.pass_git_root {
            if let Some(ref git_root) = self.git_root {
                env_vars.insert(
                    "PANE_GIT_ROOT".to_string(),
                    git_root.to_string_lossy().to_string(),
                );
            }
        }

        if context_config.pass_project_name {
            if let Some(ref project_name) = self.project_name {
                env_vars.insert("PANE_PROJECT_NAME".to_string(), project_name.clone());
            }
        }

        env_vars
    }
}

/// Detect git repository root from the given directory
///
/// Walks up the directory tree to find a git repository root.
/// Returns None if no git repository is found.
///
/// # Arguments
///
/// * `cwd` - Current working directory to start search from
///
/// # Returns
///
/// Some(PathBuf) if a git repository root is found, None otherwise
///
/// # Examples
///
/// ```no_run
/// # use std::path::Path;
/// # use pane::context::detect_git_root;
/// let cwd = Path::new("/home/user/projects/myproject");
/// if let Some(git_root) = detect_git_root(cwd) {
///     println!("Git root: {:?}", git_root);
/// }
/// ```
pub fn detect_git_root(cwd: &Path) -> Option<PathBuf> {
    // Use git2::Repository::discover() to find git root
    // Returns None if no git repository is found
    git2::Repository::discover(cwd)
        .ok()
        .and_then(|repo| repo.workdir().map(|p| p.to_path_buf()))
}

/// Extract project name from a path
///
/// Gets the final directory name from the given path.
///
/// # Arguments
///
/// * `path` - Path to extract project name from (typically git root or cwd)
///
/// # Returns
///
/// Some(String) containing the directory name, None if extraction fails
///
/// # Examples
///
/// ```no_run
/// # use std::path::Path;
/// # use pane::context::extract_project_name;
/// let path = Path::new("/home/user/projects/myproject");
/// assert_eq!(extract_project_name(path), Some("myproject".to_string()));
/// ```
pub fn extract_project_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
}

/// Get the config file path
///
/// Returns the path to the Pane config file, checking environment variable
/// override first, then falling back to default location.
///
/// # Returns
///
/// PathBuf pointing to the config file location
fn get_config_path() -> PathBuf {
    if let Ok(path) = env::var("PANE_CONFIG_PATH") {
        return PathBuf::from(path);
    }

    // Default config path (will be expanded to home directory)
    expand_tilde("~/.config/pane/config.toml")
}

/// Expands tilde (~) in path to home directory
///
/// If tilde expansion fails (no HOME env var), returns path unchanged.
///
/// # Arguments
///
/// * `path` - Path string potentially containing tilde prefix
///
/// # Returns
///
/// PathBuf with tilde expanded to home directory if possible
fn expand_tilde(path: &str) -> PathBuf {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Ok(home) = env::var("HOME") {
            return PathBuf::from(home).join(stripped);
        }
    }
    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::skills::{
        manifest::{SkillManifest, UiConfig, UiMode},
        SkillSource,
    };
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_skill(id: &str, name: &str) -> Skill {
        Skill {
            manifest: SkillManifest {
                id: id.to_string(),
                name: name.to_string(),
                description: "Test skill".to_string(),
                version: "1.0.0".to_string(),
                exec: "test".to_string(),
                args: vec!["--flag".to_string(), "value".to_string()],
                tags: vec![],
                estimated_time: None,
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
    #[serial_test::serial]
    fn test_build_context_without_git_repo() {
        // Arrange
        let skill = create_test_skill("test-skill", "Test Skill");
        let config = Config::default();
        let temp_dir = TempDir::new().unwrap();
        env::set_current_dir(temp_dir.path()).unwrap();

        // Act
        let result = SkillContext::build(&skill, &config);

        // Assert
        assert!(result.is_ok());
        let context = result.unwrap();
        assert_eq!(context.skill_id, "test-skill");
        assert_eq!(context.skill_name, "Test Skill");
        // Use canonicalize to handle symlink resolution on macOS
        assert_eq!(
            context.cwd.canonicalize().unwrap(),
            temp_dir.path().canonicalize().unwrap()
        );
        assert_eq!(context.git_root, None);
        assert!(context.project_name.is_some());
        assert_eq!(context.args, vec!["--flag", "value"]);
    }

    #[test]
    fn test_detect_git_root_no_repo_returns_none() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();

        // Act
        let result = detect_git_root(temp_dir.path());

        // Assert
        assert_eq!(result, None);
    }

    #[test]
    fn test_detect_git_root_finds_repo() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        git2::Repository::init(temp_dir.path()).unwrap();

        // Act
        let result = detect_git_root(temp_dir.path());

        // Assert
        assert!(result.is_some());
        // Use canonicalize to handle symlink resolution on macOS
        assert_eq!(
            result.unwrap().canonicalize().unwrap(),
            temp_dir.path().canonicalize().unwrap()
        );
    }

    #[test]
    fn test_extract_project_name_from_path() {
        // Arrange
        let path = PathBuf::from("/home/user/projects/myproject");

        // Act
        let result = extract_project_name(&path);

        // Assert
        assert_eq!(result, Some("myproject".to_string()));
    }

    #[test]
    fn test_extract_project_name_from_git_root() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        let git_root = temp_dir.path();

        // Act
        let result = extract_project_name(git_root);

        // Assert
        assert!(result.is_some());
        // Result will be temp directory name
    }

    #[test]
    fn test_prepare_environment_all_fields_present() {
        // Arrange
        let _temp_dir = TempDir::new().unwrap();
        let context = SkillContext {
            skill_id: "test-skill".to_string(),
            skill_name: "Test Skill".to_string(),
            cwd: PathBuf::from("/home/user/project"),
            git_root: Some(PathBuf::from("/home/user/project")),
            project_name: Some("project".to_string()),
            config_path: PathBuf::from("/home/user/.config/pane/config.toml"),
            args: vec![],
        };
        let context_config = ContextConfig::default();

        // Act
        let env_vars = context.prepare_environment(&context_config);

        // Assert
        assert_eq!(env_vars.get("PANE_ID"), Some(&"test-skill".to_string()));
        assert_eq!(env_vars.get("PANE_NAME"), Some(&"Test Skill".to_string()));
        assert_eq!(
            env_vars.get("PANE_CWD"),
            Some(&"/home/user/project".to_string())
        );
        assert_eq!(
            env_vars.get("PANE_GIT_ROOT"),
            Some(&"/home/user/project".to_string())
        );
        assert_eq!(
            env_vars.get("PANE_PROJECT_NAME"),
            Some(&"project".to_string())
        );
        assert_eq!(
            env_vars.get("PANE_CONFIG_PATH"),
            Some(&"/home/user/.config/pane/config.toml".to_string())
        );
    }

    #[test]
    fn test_prepare_environment_respects_context_config() {
        // Arrange
        let context = SkillContext {
            skill_id: "test-skill".to_string(),
            skill_name: "Test Skill".to_string(),
            cwd: PathBuf::from("/home/user/project"),
            git_root: Some(PathBuf::from("/home/user/project")),
            project_name: Some("project".to_string()),
            config_path: PathBuf::from("/home/user/.config/pane/config.toml"),
            args: vec![],
        };
        let context_config = ContextConfig {
            pass_cwd: false,
            pass_git_root: false,
            pass_project_name: false,
            pass_stdin_json: false,
        };

        // Act
        let env_vars = context.prepare_environment(&context_config);

        // Assert - Only ID, NAME, and CONFIG_PATH should be present
        assert!(env_vars.contains_key("PANE_ID"));
        assert!(env_vars.contains_key("PANE_NAME"));
        assert!(env_vars.contains_key("PANE_CONFIG_PATH"));
        assert!(!env_vars.contains_key("PANE_CWD"));
        assert!(!env_vars.contains_key("PANE_GIT_ROOT"));
        assert!(!env_vars.contains_key("PANE_PROJECT_NAME"));
    }

    #[test]
    fn test_prepare_environment_omits_missing_optional_fields() {
        // Arrange
        let context = SkillContext {
            skill_id: "test-skill".to_string(),
            skill_name: "Test Skill".to_string(),
            cwd: PathBuf::from("/home/user/project"),
            git_root: None,     // No git root
            project_name: None, // No project name
            config_path: PathBuf::from("/home/user/.config/pane/config.toml"),
            args: vec![],
        };
        let context_config = ContextConfig::default();

        // Act
        let env_vars = context.prepare_environment(&context_config);

        // Assert
        assert!(env_vars.contains_key("PANE_ID"));
        assert!(env_vars.contains_key("PANE_NAME"));
        assert!(env_vars.contains_key("PANE_CWD"));
        assert!(!env_vars.contains_key("PANE_GIT_ROOT")); // Optional field not present
        assert!(!env_vars.contains_key("PANE_PROJECT_NAME")); // Optional field not present
    }

    #[test]
    fn test_expand_tilde_with_home_env() {
        // Arrange
        let original_home = env::var("HOME").ok();
        env::set_var("HOME", "/test/home");

        // Act
        let expanded = expand_tilde("~/config/file.toml");

        // Assert
        assert_eq!(expanded, PathBuf::from("/test/home/config/file.toml"));

        // Cleanup
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        } else {
            env::remove_var("HOME");
        }
    }

    #[test]
    fn test_expand_tilde_without_tilde_returns_unchanged() {
        // Arrange & Act
        let path = expand_tilde("/absolute/path/config.toml");

        // Assert
        assert_eq!(path, PathBuf::from("/absolute/path/config.toml"));
    }
}
