use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

use anyhow::Result;
use walkdir::WalkDir;

use crate::skills::manifest::SkillManifest;
use crate::skills::model::{Skill, SkillSource};

/// Discover skills from project, user, and system locations
///
/// Skills are discovered in the following order (highest to lowest precedence):
/// 1. Project: `./.pane/skills/` (current working directory)
/// 2. User: `~/.config/pane/skills/` (user's config directory)
/// 3. System: `/usr/local/share/pane/skills/` (system-wide installation)
///
/// When skills with duplicate IDs are found, the skill from the higher precedence
/// source is used. Missing directories are skipped gracefully without errors.
///
/// # Returns
///
/// * `Result<Vec<Skill>>` - Vector of discovered skills with unique IDs
///
/// # Errors
///
/// Returns an error only if critical failures occur. Individual skill loading
/// failures are logged as warnings and skipped.
#[allow(dead_code)]
pub fn discover_skills() -> Result<Vec<Skill>> {
    let mut skill_map: HashMap<String, Skill> = HashMap::new();

    // Define discovery paths in precedence order (process in reverse for HashMap)
    let discovery_paths = vec![
        (
            PathBuf::from("/usr/local/share/pane/skills"),
            SkillSource::System,
        ),
        (expand_tilde("~/.config/pane/skills"), SkillSource::User),
        (PathBuf::from("./.pane/skills"), SkillSource::Project),
    ];

    // Discover skills from each location
    for (path, source) in discovery_paths {
        let skills = discover_in_directory(path, source);
        for skill in skills {
            let id = skill.manifest.id.clone();
            if let Some(existing) = skill_map.get(&id) {
                tracing::info!(
                    "Skill '{}' from {:?} overridden by {:?}",
                    id,
                    existing.source,
                    skill.source
                );
            }
            skill_map.insert(id, skill);
        }
    }

    Ok(skill_map.into_values().collect())
}

/// Discover skills in a specific directory
///
/// Recursively searches the given directory for `pane-skill.yaml` files,
/// loads and validates each manifest, and returns a vector of discovered skills.
///
/// # Arguments
///
/// * `path` - Directory path to search
/// * `source` - Source type for discovered skills
///
/// # Returns
///
/// Vector of successfully loaded skills. Parse failures are logged and skipped.
#[allow(dead_code)]
fn discover_in_directory(path: PathBuf, source: SkillSource) -> Vec<Skill> {
    // Check if directory exists
    if !path.exists() {
        tracing::debug!("Skill directory not found: {:?}, skipping", path);
        return Vec::new();
    }

    let mut skills = Vec::new();

    // Recursively walk directory looking for pane-skill.yaml files
    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();

        // Only process files named pane-skill.yaml
        if entry_path.is_file()
            && entry_path.file_name().and_then(|n| n.to_str()) == Some("pane-skill.yaml")
        {
            match SkillManifest::from_yaml_file(entry_path.to_path_buf()) {
                Ok(manifest) => {
                    skills.push(Skill {
                        manifest,
                        source: source.clone(),
                        manifest_path: entry_path.to_path_buf(),
                    });
                }
                Err(e) => {
                    tracing::warn!("Failed to load skill manifest from {:?}: {}", entry_path, e);
                }
            }
        }
    }

    skills
}

/// Expand tilde (~) in path to user's home directory
///
/// # Arguments
///
/// * `path` - Path string potentially containing `~`
///
/// # Returns
///
/// PathBuf with tilde expanded to home directory
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
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_expand_tilde_with_home_env() {
        // Arrange
        let original_home = env::var("HOME").ok();
        env::set_var("HOME", "/Users/testuser");

        // Act
        let result = expand_tilde("~/.config/pane/skills");

        // Assert
        assert_eq!(result, PathBuf::from("/Users/testuser/.config/pane/skills"));

        // Cleanup
        if let Some(home) = original_home {
            env::set_var("HOME", home);
        }
    }

    #[test]
    fn test_expand_tilde_without_tilde_returns_unchanged() {
        // Arrange
        let path = "/usr/local/share/pane/skills";

        // Act
        let result = expand_tilde(path);

        // Assert
        assert_eq!(result, PathBuf::from(path));
    }

    #[test]
    fn test_discover_in_directory_empty_directory_returns_empty_vec() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();

        // Act
        let skills = discover_in_directory(temp_dir.path().to_path_buf(), SkillSource::Project);

        // Assert
        assert_eq!(skills.len(), 0);
    }

    #[test]
    fn test_discover_in_directory_missing_directory_returns_empty_vec() {
        // Arrange
        let nonexistent_path = PathBuf::from("/tmp/nonexistent-pane-test-dir-12345");

        // Act
        let skills = discover_in_directory(nonexistent_path, SkillSource::Project);

        // Assert
        assert_eq!(skills.len(), 0);
    }

    #[test]
    fn test_discover_in_directory_finds_valid_manifest() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = temp_dir.path().join("pane-skill.yaml");
        fs::write(
            &manifest_path,
            r#"
id: test-skill
name: Test Skill
description: A test skill
exec: ./test.sh
ui:
  mode: tui
"#,
        )
        .unwrap();

        // Act
        let skills = discover_in_directory(temp_dir.path().to_path_buf(), SkillSource::Project);

        // Assert
        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].manifest.id, "test-skill");
        assert_eq!(skills[0].source, SkillSource::Project);
        assert_eq!(skills[0].manifest_path, manifest_path);
    }

    #[test]
    fn test_discover_in_directory_skips_invalid_manifest() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        fs::write(
            temp_dir.path().join("pane-skill.yaml"),
            r#"
invalid yaml syntax: [unclosed
"#,
        )
        .unwrap();

        // Act
        let skills = discover_in_directory(temp_dir.path().to_path_buf(), SkillSource::User);

        // Assert
        assert_eq!(skills.len(), 0);
    }

    #[test]
    fn test_discover_in_directory_recursive_finds_nested_manifests() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        let nested_dir = temp_dir.path().join("subdir").join("nested");
        fs::create_dir_all(&nested_dir).unwrap();

        fs::write(
            temp_dir.path().join("pane-skill.yaml"),
            r#"
id: root-skill
name: Root Skill
description: Root level skill
exec: ./root.sh
ui:
  mode: tui
"#,
        )
        .unwrap();

        fs::write(
            nested_dir.join("pane-skill.yaml"),
            r#"
id: nested-skill
name: Nested Skill
description: Nested skill
exec: ./nested.sh
ui:
  mode: tui
"#,
        )
        .unwrap();

        // Act
        let skills = discover_in_directory(temp_dir.path().to_path_buf(), SkillSource::System);

        // Assert
        assert_eq!(skills.len(), 2);
        let ids: Vec<String> = skills.iter().map(|s| s.manifest.id.clone()).collect();
        assert!(ids.contains(&"root-skill".to_string()));
        assert!(ids.contains(&"nested-skill".to_string()));
    }

    #[test]
    fn test_discover_skills_duplicate_id_project_overrides_user() {
        // Arrange
        let temp_base = TempDir::new().unwrap();
        let project_dir = temp_base.path().join(".pane/skills");
        let user_dir = temp_base.path().join(".config/pane/skills");

        fs::create_dir_all(&project_dir).unwrap();
        fs::create_dir_all(&user_dir).unwrap();

        // User skill
        fs::write(
            user_dir.join("pane-skill.yaml"),
            r#"
id: duplicate-skill
name: User Skill
description: From user directory
exec: ./user.sh
ui:
  mode: tui
"#,
        )
        .unwrap();

        // Project skill (should override user)
        fs::write(
            project_dir.join("pane-skill.yaml"),
            r#"
id: duplicate-skill
name: Project Skill
description: From project directory
exec: ./project.sh
ui:
  mode: tui
"#,
        )
        .unwrap();

        // Act - We can't easily test discover_skills() without changing directories
        // so we test the logic directly
        let user_skills = discover_in_directory(user_dir, SkillSource::User);
        let project_skills = discover_in_directory(project_dir, SkillSource::Project);

        // Assert
        assert_eq!(user_skills.len(), 1);
        assert_eq!(project_skills.len(), 1);
        assert_eq!(user_skills[0].manifest.name, "User Skill");
        assert_eq!(project_skills[0].manifest.name, "Project Skill");

        // In the actual discover_skills() HashMap, project would override user
        let mut map: HashMap<String, Skill> = HashMap::new();
        for skill in user_skills {
            map.insert(skill.manifest.id.clone(), skill);
        }
        for skill in project_skills {
            map.insert(skill.manifest.id.clone(), skill);
        }

        let final_skill = map.get("duplicate-skill").unwrap();
        assert_eq!(final_skill.manifest.name, "Project Skill");
        assert_eq!(final_skill.source, SkillSource::Project);
    }
}
