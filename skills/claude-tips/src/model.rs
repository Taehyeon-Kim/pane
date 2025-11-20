//! Data models for Claude Code tips.
//!
//! This module defines the core data structures for representing tips
//! that can be loaded from YAML files and displayed in the tips viewer.

use serde::{Deserialize, Serialize};

/// Represents a single Claude Code tip.
///
/// Tips provide guidance, best practices, and workflow suggestions for
/// working with Claude Code. Each tip has a unique identifier, title,
/// content text, and optional metadata for categorization and search.
///
/// # Examples
///
/// ```
/// use claude_tips::model::Tip;
///
/// let tip = Tip {
///     id: "cc-001".to_string(),
///     title: "Use clear prompts".to_string(),
///     category: Some("prompting".to_string()),
///     text: "Be specific and clear in your requests.".to_string(),
///     tags: vec!["prompting".to_string(), "best-practices".to_string()],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Tip {
    /// Unique identifier for this tip (e.g., "cc-001").
    ///
    /// IDs must be unique across all tips in a collection.
    /// Format convention: "cc-NNN" where NNN is a zero-padded number.
    pub id: String,

    /// Human-readable title displayed in the tips browser.
    ///
    /// Should be concise (ideally < 60 characters) and descriptive.
    pub title: String,

    /// Optional category for grouping related tips.
    ///
    /// Common categories: "prompting", "workflow", "debugging", "features".
    /// Defaults to `None` if not specified.
    #[serde(default)]
    pub category: Option<String>,

    /// The main content of the tip.
    ///
    /// Can be multi-line and may contain markdown formatting.
    /// Should provide actionable guidance or useful information.
    pub text: String,

    /// Optional list of searchable tags.
    ///
    /// Tags enable filtering and discovery of related tips.
    /// Defaults to an empty vector if not specified.
    #[serde(default)]
    pub tags: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tip_creation_with_all_fields() {
        let tip = Tip {
            id: "cc-001".to_string(),
            title: "Test Tip".to_string(),
            category: Some("testing".to_string()),
            text: "This is a test tip.".to_string(),
            tags: vec!["test".to_string(), "example".to_string()],
        };

        assert_eq!(tip.id, "cc-001");
        assert_eq!(tip.title, "Test Tip");
        assert_eq!(tip.category, Some("testing".to_string()));
        assert_eq!(tip.text, "This is a test tip.");
        assert_eq!(tip.tags.len(), 2);
    }

    #[test]
    fn test_tip_serde_defaults_applied() {
        let yaml = r#"
id: "cc-002"
title: "Minimal Tip"
text: "Just the required fields."
"#;

        let tip: Tip = serde_yaml::from_str(yaml).expect("Failed to parse YAML");

        assert_eq!(tip.id, "cc-002");
        assert_eq!(tip.title, "Minimal Tip");
        assert_eq!(tip.category, None);
        assert_eq!(tip.tags, Vec::<String>::new());
    }

    #[test]
    fn test_tip_serialization_roundtrip() {
        let original = Tip {
            id: "cc-003".to_string(),
            title: "Roundtrip Test".to_string(),
            category: Some("serialization".to_string()),
            text: "Testing serialization.".to_string(),
            tags: vec!["serde".to_string()],
        };

        let yaml = serde_yaml::to_string(&original).expect("Failed to serialize");
        let deserialized: Tip = serde_yaml::from_str(&yaml).expect("Failed to deserialize");

        assert_eq!(original, deserialized);
    }
}
