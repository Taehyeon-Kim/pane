use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// Helper function to get the tips data file path
fn get_tips_file_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("skills/claude-tips/data/claude-tips.yaml")
}

/// Load and parse tips from YAML file
/// Returns the parsed YAML as a vector of tip maps
fn load_tips_from_yaml() -> Vec<serde_yaml::Value> {
    let tips_path = get_tips_file_path();
    let content = fs::read_to_string(&tips_path)
        .unwrap_or_else(|e| panic!("Failed to read tips file at {:?}: {}", tips_path, e));

    serde_yaml::from_str(&content).unwrap_or_else(|e| panic!("Failed to parse tips YAML: {}", e))
}

#[test]
fn test_claude_tips_skill_loads_all_tips() {
    // Load tips from YAML file programmatically
    let tips = load_tips_from_yaml();

    // Assert ≥15 tips loaded
    assert!(
        tips.len() >= 15,
        "Expected at least 15 tips, but found {}",
        tips.len()
    );

    // Assert all required fields present (id, title, text, category, tags)
    for (index, tip) in tips.iter().enumerate() {
        let tip_obj = tip
            .as_mapping()
            .unwrap_or_else(|| panic!("Tip {} is not a valid mapping", index));

        // Check required fields
        assert!(
            tip_obj.contains_key(serde_yaml::Value::String("id".to_string())),
            "Tip {} missing 'id' field",
            index
        );

        assert!(
            tip_obj.contains_key(serde_yaml::Value::String("title".to_string())),
            "Tip {} missing 'title' field",
            index
        );

        assert!(
            tip_obj.contains_key(serde_yaml::Value::String("text".to_string())),
            "Tip {} missing 'text' field",
            index
        );

        assert!(
            tip_obj.contains_key(serde_yaml::Value::String("category".to_string())),
            "Tip {} missing 'category' field",
            index
        );

        assert!(
            tip_obj.contains_key(serde_yaml::Value::String("tags".to_string())),
            "Tip {} missing 'tags' field",
            index
        );
    }

    // Collect all categories
    let categories: HashSet<String> = tips
        .iter()
        .filter_map(|tip| {
            tip.as_mapping()
                .and_then(|m| m.get(serde_yaml::Value::String("category".to_string())))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .collect();

    // Assert all 5 categories represented
    let expected_categories: HashSet<String> = [
        "prompting",
        "cost",
        "workflow",
        "debugging",
        "best-practices",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();

    assert_eq!(
        categories, expected_categories,
        "Not all 5 categories are represented. Found: {:?}",
        categories
    );
}

#[test]
fn test_claude_tips_search_filters_correctly() {
    let tips = load_tips_from_yaml();

    // Test category search for each of the 5 categories
    let categories = [
        "prompting",
        "cost",
        "workflow",
        "debugging",
        "best-practices",
    ];

    for category in &categories {
        let filtered: Vec<_> = tips
            .iter()
            .filter(|tip| {
                tip.as_mapping()
                    .and_then(|m| m.get(serde_yaml::Value::String("category".to_string())))
                    .and_then(|v| v.as_str())
                    .map(|s| s == *category)
                    .unwrap_or(false)
            })
            .collect();

        assert!(
            !filtered.is_empty(),
            "Expected to find tips in category '{}', but found none",
            category
        );
    }

    // Test tag-based search
    let tag_searches = vec![
        ("prompting", true),
        ("cost", true),
        ("git", true),
        ("nonexistent_tag", false),
    ];

    for (search_tag, should_find) in tag_searches {
        let filtered: Vec<_> = tips
            .iter()
            .filter(|tip| {
                tip.as_mapping()
                    .and_then(|m| m.get(serde_yaml::Value::String("tags".to_string())))
                    .and_then(|v| v.as_sequence())
                    .map(|tags| {
                        tags.iter().any(|tag| {
                            tag.as_str()
                                .map(|s| s.to_lowercase() == search_tag.to_lowercase())
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(false)
            })
            .collect();

        if should_find {
            assert!(
                !filtered.is_empty(),
                "Expected to find tips with tag '{}', but found none",
                search_tag
            );
        } else {
            assert!(
                filtered.is_empty(),
                "Expected to find no tips with tag '{}', but found {}",
                search_tag,
                filtered.len()
            );
        }
    }

    // Test case-insensitive matching on tags
    let case_insensitive_searches = vec!["PROMPTING", "Git", "WoRkFlOw"];

    for search_tag in case_insensitive_searches {
        let filtered: Vec<_> = tips
            .iter()
            .filter(|tip| {
                tip.as_mapping()
                    .and_then(|m| m.get(serde_yaml::Value::String("tags".to_string())))
                    .and_then(|v| v.as_sequence())
                    .map(|tags| {
                        tags.iter().any(|tag| {
                            tag.as_str()
                                .map(|s| s.to_lowercase() == search_tag.to_lowercase())
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(false)
            })
            .collect();

        assert!(
            !filtered.is_empty(),
            "Case-insensitive search for '{}' should find results",
            search_tag
        );
    }
}

#[test]
fn test_claude_tips_tip_ids_unique() {
    let tips = load_tips_from_yaml();

    // Collect all tip IDs
    let ids: Vec<String> = tips
        .iter()
        .filter_map(|tip| {
            tip.as_mapping()
                .and_then(|m| m.get(serde_yaml::Value::String("id".to_string())))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        })
        .collect();

    // Check for duplicates
    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();

    for id in &ids {
        if !seen.insert(id.clone()) {
            duplicates.push(id.clone());
        }
    }

    assert!(
        duplicates.is_empty(),
        "Found duplicate tip IDs: {:?}",
        duplicates
    );

    // Validate no duplicate IDs exist
    assert_eq!(
        ids.len(),
        seen.len(),
        "Number of IDs ({}) doesn't match number of unique IDs ({})",
        ids.len(),
        seen.len()
    );

    // Assert ID format follows pattern (e.g., "cc-XXX")
    for id in &ids {
        assert!(
            id.starts_with("cc-"),
            "Tip ID '{}' doesn't follow 'cc-XXX' pattern",
            id
        );

        // Check that part after "cc-" is numeric
        let number_part = &id[3..];
        assert!(
            number_part.parse::<u32>().is_ok(),
            "Tip ID '{}' should have numeric suffix after 'cc-'",
            id
        );
    }
}
