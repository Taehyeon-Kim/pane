use std::path::PathBuf;

use crate::skills::manifest::SkillManifest;

/// Indicates where a skill was discovered (for override precedence)
///
/// Skills are discovered from three locations with the following precedence:
/// Project > User > System
///
/// When skills with duplicate IDs are found, the skill from the higher
/// precedence source is used.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum SkillSource {
    /// Discovered in /usr/local/share/pane/skills/ (lowest precedence)
    System,
    /// Discovered in ~/.config/pane/skills/ (medium precedence)
    User,
    /// Discovered in ./.pane/skills/ (highest precedence)
    Project,
}

/// Represents a discovered skill with its manifest and source metadata
///
/// A `Skill` wraps a parsed `SkillManifest` along with information about
/// where it was discovered and the path to its manifest file.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub struct Skill {
    /// Parsed skill manifest from pane-skill.yaml
    pub manifest: SkillManifest,
    /// Where this skill was discovered (for precedence resolution)
    pub source: SkillSource,
    /// Absolute path to the source pane-skill.yaml file
    pub manifest_path: PathBuf,
}
