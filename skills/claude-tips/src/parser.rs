//! YAML parser for Claude Code tips.
//!
//! This module handles loading and validating tips from YAML files.
//! It ensures all tips have required fields and unique IDs before
//! returning the parsed collection.

use anyhow::{anyhow, Context, Result};
use std::collections::HashSet;
use std::path::Path;

use crate::model::Tip;

/// Loads and validates tips from a YAML file.
///
/// This function reads the YAML file, parses it into a collection of tips,
/// and validates that all tips meet the requirements:
/// - All required fields (id, title, text) must be non-empty
/// - All tip IDs must be unique across the collection
///
/// # Arguments
///
/// * `file_path` - Path to the YAML file containing tips
///
/// # Returns
///
/// Returns `Ok(Vec<Tip>)` if the file is successfully loaded and validated.
/// Returns `Err` if:
/// - The file cannot be read
/// - The YAML is malformed
/// - Any tip has empty required fields
/// - Duplicate tip IDs are found
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use claude_tips::parser::load_tips;
///
/// let tips = load_tips(Path::new("data/claude-tips.yaml"))?;
/// println!("Loaded {} tips", tips.len());
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn load_tips(file_path: &Path) -> Result<Vec<Tip>> {
    // Read file contents
    let contents = std::fs::read_to_string(file_path)
        .with_context(|| format!("Tips file not found at {}", file_path.display()))?;

    // Parse YAML into Vec<Tip>
    let tips: Vec<Tip> = serde_yaml::from_str(&contents)
        .with_context(|| format!("Failed to parse tips file at {}", file_path.display()))?;

    // Validate tips
    validate_tips(&tips)?;

    Ok(tips)
}

/// Validates a collection of tips.
///
/// Ensures all tips have:
/// - Non-empty required fields (id, title, text)
/// - Unique IDs across the collection
///
/// # Arguments
///
/// * `tips` - Slice of tips to validate
///
/// # Returns
///
/// Returns `Ok(())` if all validations pass.
/// Returns `Err` with a descriptive error message if validation fails.
fn validate_tips(tips: &[Tip]) -> Result<()> {
    let mut seen_ids = HashSet::new();

    for (index, tip) in tips.iter().enumerate() {
        // Validate required fields are non-empty
        if tip.id.trim().is_empty() {
            return Err(anyhow!(
                "Tip at index {} is missing required field 'id'",
                index
            ));
        }

        if tip.title.trim().is_empty() {
            return Err(anyhow!(
                "Tip at index {} is missing required field 'title'",
                index
            ));
        }

        if tip.text.trim().is_empty() {
            return Err(anyhow!(
                "Tip at index {} is missing required field 'text'",
                index
            ));
        }

        // Check for duplicate IDs
        if !seen_ids.insert(tip.id.clone()) {
            return Err(anyhow!(
                "Duplicate tip ID '{}' found (tips must have unique IDs)",
                tip.id
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Tip;

    #[test]
    fn test_load_tips_valid_file_success() {
        let yaml = r#"
- id: "cc-001"
  title: "First Tip"
  category: "testing"
  text: "This is the first tip."
  tags: ["test", "example"]

- id: "cc-002"
  title: "Second Tip"
  text: "This is the second tip."
"#;

        let tips: Vec<Tip> = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
        assert_eq!(tips.len(), 2);

        assert_eq!(tips[0].id, "cc-001");
        assert_eq!(tips[0].title, "First Tip");
        assert_eq!(tips[0].category, Some("testing".to_string()));
        assert_eq!(tips[0].text, "This is the first tip.");
        assert_eq!(tips[0].tags, vec!["test", "example"]);

        assert_eq!(tips[1].id, "cc-002");
        assert_eq!(tips[1].title, "Second Tip");
        assert_eq!(tips[1].category, None);
        assert_eq!(tips[1].text, "This is the second tip.");
        assert_eq!(tips[1].tags, Vec::<String>::new());
    }

    #[test]
    fn test_load_tips_missing_required_field_fails() {
        // Missing 'id' field
        let yaml_missing_id = r#"
- title: "Test Tip"
  text: "Test content"
"#;

        let result: Result<Vec<Tip>, _> = serde_yaml::from_str(yaml_missing_id);
        assert!(result.is_err());

        // Missing 'title' field
        let yaml_missing_title = r#"
- id: "cc-001"
  text: "Test content"
"#;

        let result: Result<Vec<Tip>, _> = serde_yaml::from_str(yaml_missing_title);
        assert!(result.is_err());

        // Missing 'text' field
        let yaml_missing_text = r#"
- id: "cc-001"
  title: "Test Tip"
"#;

        let result: Result<Vec<Tip>, _> = serde_yaml::from_str(yaml_missing_text);
        assert!(result.is_err());
    }

    #[test]
    fn test_load_tips_duplicate_ids_fails() {
        let yaml = r#"
- id: "cc-001"
  title: "First Tip"
  text: "First content"

- id: "cc-001"
  title: "Duplicate ID"
  text: "Second content"
"#;

        let tips: Vec<Tip> = serde_yaml::from_str(yaml).expect("YAML parsing should succeed");
        let result = validate_tips(&tips);

        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Duplicate tip ID 'cc-001'"));
    }

    #[test]
    fn test_load_tips_optional_fields_defaults() {
        let yaml = r#"
- id: "cc-001"
  title: "Minimal Tip"
  text: "Just the required fields."
"#;

        let tips: Vec<Tip> = serde_yaml::from_str(yaml).expect("Failed to parse YAML");
        assert_eq!(tips.len(), 1);

        assert_eq!(tips[0].id, "cc-001");
        assert_eq!(tips[0].title, "Minimal Tip");
        assert_eq!(tips[0].category, None);
        assert_eq!(tips[0].text, "Just the required fields.");
        assert_eq!(tips[0].tags, Vec::<String>::new());
    }

    #[test]
    fn test_load_tips_invalid_yaml_fails() {
        let yaml = r#"
- id: "cc-001"
  title: "Test"
  text: "Content"
  invalid_field: [unclosed array
"#;

        let result: Result<Vec<Tip>, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_tips_empty_id_fails() {
        let tips = vec![Tip {
            id: "".to_string(),
            title: "Test".to_string(),
            category: None,
            text: "Content".to_string(),
            tags: vec![],
        }];

        let result = validate_tips(&tips);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing required field 'id'"));
    }

    #[test]
    fn test_validate_tips_empty_title_fails() {
        let tips = vec![Tip {
            id: "cc-001".to_string(),
            title: "   ".to_string(),
            category: None,
            text: "Content".to_string(),
            tags: vec![],
        }];

        let result = validate_tips(&tips);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing required field 'title'"));
    }

    #[test]
    fn test_validate_tips_empty_text_fails() {
        let tips = vec![Tip {
            id: "cc-001".to_string(),
            title: "Test".to_string(),
            category: None,
            text: "".to_string(),
            tags: vec![],
        }];

        let result = validate_tips(&tips);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing required field 'text'"));
    }
}
