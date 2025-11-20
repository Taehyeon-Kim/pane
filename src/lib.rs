/// Pane - A blazing-fast TUI skill launcher for developers
///
/// This library provides the core functionality for skill discovery,
/// configuration management, and manifest parsing.
pub mod app;
pub mod config;
pub mod context;
pub mod i18n;
pub mod input;
pub mod search;
pub mod skills;
pub mod state;
pub mod terminal;
pub mod ui;

// Re-export commonly used types
pub use config::{load_config, Config};
pub use input::{poll_event, InputEvent};
pub use skills::{discover_skills, Skill, SkillManifest, SkillSource};
pub use state::{AppState, ViewMode};
pub use terminal::TerminalGuard;
