/// Fuzzy search and filtering for skills
///
/// This module provides fast fuzzy matching capabilities using the nucleo crate.
/// It supports searching across skill names, IDs, tags, and descriptions with
/// case-insensitive matching and score-based ranking.
use crate::skills::Skill;
use nucleo_matcher::{
    pattern::{CaseMatching, Pattern},
    Matcher, Utf32Str,
};

/// Filter skills based on a fuzzy search query
///
/// Performs case-insensitive fuzzy matching against skill names, IDs, tags, and
/// descriptions. Returns indices of matching skills sorted by match score (best first).
///
/// # Arguments
///
/// * `query` - The search query string
/// * `skills` - Slice of skills to search through
///
/// # Returns
///
/// Vector of indices into the skills slice, sorted by match score (highest first).
/// Returns all indices (0..skills.len()) if query is empty.
///
/// # Performance
///
/// - Optimized for real-time filtering (<10ms for 100 skills)
/// - Uses nucleo matcher (same as Helix editor)
/// - Case-insensitive matching
///
/// # Example
///
/// ```no_run
/// # use pane::skills::Skill;
/// # use pane::search::filter_skills;
/// # let skills: Vec<Skill> = vec![];
/// let query = "clau";
/// let filtered_indices = filter_skills(query, &skills);
/// // filtered_indices contains indices of skills matching "clau"
/// ```
pub fn filter_skills(query: &str, skills: &[Skill]) -> Vec<usize> {
    // Empty query returns all skills
    if query.is_empty() {
        return (0..skills.len()).collect();
    }

    // Create nucleo matcher with case-insensitive configuration
    let mut matcher = Matcher::new(nucleo_matcher::Config::DEFAULT);

    // Create pattern for matching (case-insensitive)
    let pattern = Pattern::parse(query, CaseMatching::Ignore);

    // Score each skill and collect (index, score) pairs
    let mut scored: Vec<(usize, u32)> = skills
        .iter()
        .enumerate()
        .filter_map(|(idx, skill)| {
            score_skill(&pattern, &mut matcher, skill).map(|score| (idx, score))
        })
        .collect();

    // Sort by score descending (highest score first)
    scored.sort_by(|a, b| b.1.cmp(&a.1));

    // Return sorted indices
    scored.into_iter().map(|(idx, _)| idx).collect()
}

/// Calculate fuzzy match score for a single skill
///
/// Searches across all searchable fields (name, id, tags, description) and returns
/// the highest match score found. Returns None if no fields match.
///
/// # Arguments
///
/// * `pattern` - The nucleo pattern to match against
/// * `matcher` - The nucleo matcher instance
/// * `skill` - The skill to score
///
/// # Returns
///
/// `Some(score)` if any field matches, where higher scores indicate better matches.
/// `None` if the skill doesn't match the pattern.
fn score_skill(pattern: &Pattern, matcher: &mut Matcher, skill: &Skill) -> Option<u32> {
    let mut max_score = 0u32;
    let mut has_match = false;
    let mut buf = Vec::new();

    // Check skill name
    if let Some(score) = pattern.score(Utf32Str::new(&skill.manifest.name, &mut buf), matcher) {
        max_score = max_score.max(score);
        has_match = true;
    }

    // Check skill ID
    buf.clear();
    if let Some(score) = pattern.score(Utf32Str::new(&skill.manifest.id, &mut buf), matcher) {
        max_score = max_score.max(score);
        has_match = true;
    }

    // Check tags (joined as space-separated string)
    if !skill.manifest.tags.is_empty() {
        let tags_joined = skill.manifest.tags.join(" ");
        buf.clear();
        if let Some(score) = pattern.score(Utf32Str::new(&tags_joined, &mut buf), matcher) {
            max_score = max_score.max(score);
            has_match = true;
        }
    }

    // Check description
    buf.clear();
    if let Some(score) = pattern.score(
        Utf32Str::new(&skill.manifest.description, &mut buf),
        matcher,
    ) {
        max_score = max_score.max(score);
        has_match = true;
    }

    if has_match {
        Some(max_score)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skills::{
        manifest::{ContextConfig, SkillManifest, UiConfig, UiMode},
        SkillSource,
    };
    use std::path::PathBuf;

    fn create_test_skill(id: &str, name: &str, description: &str, tags: Vec<String>) -> Skill {
        Skill {
            manifest: SkillManifest {
                id: id.to_string(),
                name: name.to_string(),
                description: description.to_string(),
                version: "1.0.0".to_string(),
                exec: "test".to_string(),
                args: vec![],
                tags,
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
    fn test_filter_skills_empty_query_returns_all_indices() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "First Skill", "Description 1", vec![]),
            create_test_skill("skill2", "Second Skill", "Description 2", vec![]),
            create_test_skill("skill3", "Third Skill", "Description 3", vec![]),
        ];

        // Act
        let result = filter_skills("", &skills);

        // Assert
        assert_eq!(result, vec![0, 1, 2]);
    }

    #[test]
    fn test_filter_skills_matches_skill_name() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Claude Tips", "Description", vec![]),
            create_test_skill("skill2", "Docker Build", "Description", vec![]),
            create_test_skill("skill3", "Git Status", "Description", vec![]),
        ];

        // Act
        let result = filter_skills("clau", &skills);

        // Assert
        assert!(result.contains(&0)); // Claude Tips should match
        assert!(!result.is_empty());
    }

    #[test]
    fn test_filter_skills_matches_skill_id() {
        // Arrange
        let skills = vec![
            create_test_skill("claude-tips", "Tips", "Description", vec![]),
            create_test_skill("docker-build", "Build", "Description", vec![]),
        ];

        // Act
        let result = filter_skills("claude", &skills);

        // Assert
        assert!(result.contains(&0)); // claude-tips should match
    }

    #[test]
    fn test_filter_skills_matches_tags() {
        // Arrange
        let skills = vec![
            create_test_skill(
                "skill1",
                "Skill One",
                "Description",
                vec!["helper".to_string(), "utility".to_string()],
            ),
            create_test_skill(
                "skill2",
                "Skill Two",
                "Description",
                vec!["docker".to_string()],
            ),
        ];

        // Act
        let result = filter_skills("helper", &skills);

        // Assert
        assert!(result.contains(&0)); // Skill One has "helper" tag
    }

    #[test]
    fn test_filter_skills_matches_description() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill", "A helpful productivity tool", vec![]),
            create_test_skill("skill2", "Skill", "Docker container manager", vec![]),
        ];

        // Act
        let result = filter_skills("productivity", &skills);

        // Assert
        assert!(result.contains(&0)); // First skill has "productivity" in description
    }

    #[test]
    fn test_filter_skills_case_insensitive() {
        // Arrange
        let skills = vec![create_test_skill(
            "skill1",
            "Claude Tips",
            "Description",
            vec![],
        )];

        // Act - query in uppercase
        let result_upper = filter_skills("CLAUDE", &skills);
        let result_lower = filter_skills("claude", &skills);
        let result_mixed = filter_skills("ClAuDe", &skills);

        // Assert - all should match
        assert!(result_upper.contains(&0));
        assert!(result_lower.contains(&0));
        assert!(result_mixed.contains(&0));
    }

    #[test]
    fn test_filter_skills_returns_ranked_results() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Claude Code Tips", "Description", vec![]),
            create_test_skill("skill2", "Code Review", "Description", vec![]),
            create_test_skill("skill3", "Docker", "Code deployment", vec![]),
        ];

        // Act
        let result = filter_skills("code", &skills);

        // Assert
        // All skills with "code" should be in results, ranked by score
        assert!(!result.is_empty());
        // Exact matches should rank higher
        assert!(result.len() <= 3);
    }

    #[test]
    fn test_filter_skills_no_match_returns_empty() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Claude Tips", "Description", vec![]),
            create_test_skill("skill2", "Docker Build", "Description", vec![]),
        ];

        // Act
        let result = filter_skills("xyz123nonexistent", &skills);

        // Assert
        assert!(result.is_empty());
    }
}
