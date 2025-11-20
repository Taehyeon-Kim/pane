// NOTE: This module contains foundational configuration infrastructure
// that will be integrated in future stories (Story 1.3+). Dead code
// warnings are expected and suppressed until integration occurs.
#![allow(dead_code)]

use std::env;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::state::ViewMode;
use crate::ui::theme::ThemeConfig;

/// User configuration loaded from `~/.config/pane/config.toml`
///
/// Provides customization options for skill discovery, UI behavior, and logging.
/// Falls back to sensible defaults when the config file doesn't exist or is invalid.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Config {
    /// Default view mode on launch (All/Favorites/Recent)
    #[serde(default)]
    pub default_view_mode: ViewMode,

    /// Whether to enable mouse support (default: true)
    #[serde(default = "default_enable_mouse")]
    pub enable_mouse: bool,

    /// Optional theme customization (colors, styles)
    #[serde(default)]
    pub theme: Option<ThemeConfig>,

    /// Maximum number of recent skills to track (default: 10)
    #[serde(default = "default_max_recent_skills")]
    pub max_recent_skills: usize,

    /// Whether to enable debug logging to file (default: false)
    #[serde(default)]
    pub debug_log_enabled: bool,

    /// Path to debug log file (default: ~/.config/pane/logs/pane-debug.log)
    #[serde(default = "default_debug_log_path")]
    pub debug_log_path: PathBuf,

    /// Skill discovery paths in search order (project, user, system)
    #[serde(default = "default_skill_paths")]
    pub skill_paths: Vec<PathBuf>,

    /// UI language (en, ko)
    #[serde(default = "default_language")]
    pub language: String,
}

// Helper functions for serde defaults
fn default_enable_mouse() -> bool {
    true
}

fn default_max_recent_skills() -> usize {
    10
}

fn default_debug_log_path() -> PathBuf {
    // Note: This will be expanded to actual home directory at runtime
    PathBuf::from("~/.config/pane/logs/pane-debug.log")
}

fn default_skill_paths() -> Vec<PathBuf> {
    vec![
        PathBuf::from("./.pane/skills/"), // Project (highest priority)
        PathBuf::from("~/.config/pane/skills/"), // User
        PathBuf::from("/usr/local/share/pane/skills/"), // System (lowest priority)
    ]
}

fn default_language() -> String {
    "en".to_string()
}

impl Default for Config {
    /// Creates a Config with sensible default values
    ///
    /// Default configuration:
    /// - default_view_mode: All
    /// - enable_mouse: true
    /// - theme: None
    /// - max_recent_skills: 10
    /// - debug_log_enabled: false
    /// - debug_log_path: ~/.config/pane/logs/pane-debug.log
    /// - skill_paths: [./.pane/skills/, ~/.config/pane/skills/, /usr/local/share/pane/skills/]
    fn default() -> Self {
        Config {
            default_view_mode: ViewMode::default(),
            enable_mouse: default_enable_mouse(),
            theme: None,
            max_recent_skills: default_max_recent_skills(),
            debug_log_enabled: false,
            debug_log_path: default_debug_log_path(),
            skill_paths: default_skill_paths(),
            language: default_language(),
        }
    }
}

impl Config {
    /// Validates the configuration
    ///
    /// Checks that:
    /// - skill_paths are not empty
    /// - debug_log_path parent directory exists if debug logging is enabled
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails
    pub fn validate(&self) -> Result<()> {
        // Validate skill paths exist
        if self.skill_paths.is_empty() {
            anyhow::bail!("Configuration error: skill_paths cannot be empty");
        }

        // Validate debug log path parent exists if debug logging enabled
        if self.debug_log_enabled {
            let expanded_path = expand_tilde(&self.debug_log_path.to_string_lossy());
            if let Some(parent) = expanded_path.parent() {
                if !parent.exists() {
                    eprintln!("Warning: Debug log directory does not exist: {:?}", parent);
                }
            }
        }

        Ok(())
    }
}

/// Loads configuration from `~/.config/pane/config.toml` or `PANE_CONFIG_PATH`
///
/// If the config file doesn't exist, returns default configuration.
/// If the config file exists but is invalid, returns an error with helpful context.
///
/// # Environment Variables
///
/// - `PANE_CONFIG_PATH`: Override the default config file location
///
/// # Errors
///
/// Returns an error if:
/// - Config file exists but contains invalid TOML
/// - Config file exists but cannot be read due to permissions
///
/// # Examples
///
/// ```
/// use pane::config::load_config;
///
/// fn example() -> anyhow::Result<()> {
///     let config = load_config()?;
///     println!("Loaded {} skill paths", config.skill_paths.len());
///     Ok(())
/// }
/// ```
pub fn load_config() -> Result<Config> {
    let config_path = get_config_path();

    // If config file doesn't exist, return defaults
    if !config_path.exists() {
        eprintln!("Config file not found at {:?}, using defaults", config_path);
        return Ok(Config::default());
    }

    // Read and parse config file
    let contents = fs::read_to_string(&config_path)
        .with_context(|| format!("Failed to read config file: {:?}", config_path))?;

    let config: Config = toml::from_str(&contents).with_context(|| {
        format!(
            "Config file corrupted, using defaults. Check {:?}",
            config_path
        )
    })?;

    Ok(config)
}

/// Resolves the config file path, checking environment variable override first
///
/// Priority:
/// 1. `PANE_CONFIG_PATH` environment variable
/// 2. `~/.config/pane/config.toml` (default)
fn get_config_path() -> PathBuf {
    if let Ok(path) = env::var("PANE_CONFIG_PATH") {
        return expand_tilde(&path);
    }

    expand_tilde("~/.config/pane/config.toml")
}

/// Expands tilde (~) in path to home directory
///
/// If tilde expansion fails (no HOME env var), returns path unchanged.
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
    use serial_test::serial;
    use std::env;

    #[test]
    fn test_config_default_has_correct_values() {
        // Arrange & Act
        let config = Config::default();

        // Assert
        assert_eq!(config.default_view_mode, ViewMode::All);
        assert!(config.enable_mouse);
        assert_eq!(config.theme, None);
        assert_eq!(config.max_recent_skills, 10);
        assert!(!config.debug_log_enabled);
        assert_eq!(
            config.debug_log_path,
            PathBuf::from("~/.config/pane/logs/pane-debug.log")
        );
        assert_eq!(config.skill_paths.len(), 3);
        assert_eq!(config.skill_paths[0], PathBuf::from("./.pane/skills/"));
        assert_eq!(
            config.skill_paths[1],
            PathBuf::from("~/.config/pane/skills/")
        );
        assert_eq!(
            config.skill_paths[2],
            PathBuf::from("/usr/local/share/pane/skills/")
        );
    }

    #[test]
    fn test_viewmode_default_is_all() {
        // Arrange & Act
        let mode = ViewMode::default();

        // Assert
        assert_eq!(mode, ViewMode::All);
    }

    #[test]
    #[serial]
    fn test_load_config_missing_file_returns_defaults() {
        // Arrange
        env::set_var("PANE_CONFIG_PATH", "/nonexistent/path/config.toml");

        // Act
        let result = load_config();

        // Assert
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.enable_mouse);
        assert_eq!(config.max_recent_skills, 10);

        // Cleanup
        env::remove_var("PANE_CONFIG_PATH");
    }

    #[test]
    #[serial]
    fn test_load_config_valid_toml_parses_correctly() {
        // Arrange
        let fixture_path = env::current_dir()
            .unwrap()
            .join("tests/fixtures/configs/valid.toml");
        env::set_var("PANE_CONFIG_PATH", fixture_path.to_str().unwrap());

        // Act
        let result = load_config();

        // Assert
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(!config.enable_mouse);
        assert_eq!(config.max_recent_skills, 20);
        assert!(config.debug_log_enabled);

        // Cleanup
        env::remove_var("PANE_CONFIG_PATH");
    }

    #[test]
    #[serial]
    fn test_load_config_invalid_toml_produces_error() {
        // Arrange
        let fixture_path = env::current_dir()
            .unwrap()
            .join("tests/fixtures/configs/invalid.toml");
        env::set_var("PANE_CONFIG_PATH", fixture_path.to_str().unwrap());

        // Act
        let result = load_config();

        // Assert
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Config file corrupted"));

        // Cleanup
        env::remove_var("PANE_CONFIG_PATH");
    }

    #[test]
    #[serial]
    fn test_load_config_env_var_override_works() {
        // Arrange
        let custom_path = "/custom/path/config.toml";
        env::set_var("PANE_CONFIG_PATH", custom_path);

        // Act
        let path = get_config_path();

        // Assert
        assert_eq!(path, PathBuf::from(custom_path));

        // Cleanup
        env::remove_var("PANE_CONFIG_PATH");
    }

    #[test]
    fn test_config_validate_empty_skill_paths_fails() {
        // Arrange
        let config = Config {
            skill_paths: vec![],
            ..Config::default()
        };

        // Act
        let result = config.validate();

        // Assert
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("skill_paths cannot be empty"));
    }

    #[test]
    fn test_config_validate_valid_config_passes() {
        // Arrange
        let config = Config::default();

        // Act
        let result = config.validate();

        // Assert
        assert!(result.is_ok());
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
        }
    }

    #[test]
    fn test_expand_tilde_without_tilde_returns_unchanged() {
        // Arrange & Act
        let path = expand_tilde("/absolute/path/config.toml");

        // Assert
        assert_eq!(path, PathBuf::from("/absolute/path/config.toml"));
    }

    #[test]
    #[serial]
    fn test_config_with_custom_theme_loads_correctly() {
        // Arrange
        let fixture_path = env::current_dir()
            .unwrap()
            .join("tests/fixtures/configs/with_theme.toml");
        env::set_var("PANE_CONFIG_PATH", fixture_path.to_str().unwrap());

        // Act
        let result = load_config();

        // Assert
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.theme.is_some());

        // Cleanup
        env::remove_var("PANE_CONFIG_PATH");
    }

    #[test]
    fn test_config_without_theme_uses_defaults() {
        // Arrange
        let config = Config::default();

        // Act
        let theme_option = &config.theme;

        // Assert
        assert!(theme_option.is_none());
    }
}
