/// Skills module - handles skill manifest parsing and management
pub mod loader;
pub mod manifest;
pub mod model;
pub mod output;
pub mod runner;

// Re-export manifest types
#[allow(unused_imports)]
pub use manifest::{ContextConfig, SkillManifest, UiConfig, UiMode};

// Re-export model types
#[allow(unused_imports)]
pub use model::{Skill, SkillSource};

// Re-export loader functions
#[allow(unused_imports)]
pub use loader::discover_skills;
