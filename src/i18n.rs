/// Internationalization (i18n) support for Pane
///
/// This module provides a simple, lightweight translation system supporting
/// English and Korean. The design prioritizes simplicity and compile-time
/// safety over external dependencies.
///
/// # Supported Languages
///
/// - English (`en`) - Default
/// - Korean (`ko`)
///
/// # Example
///
/// ```
/// use pane::i18n::{Language, Translations};
///
/// let lang = Language::from_code("ko");
/// let translations = Translations::load(lang);
/// assert_eq!(translations.app_title, "페인");
/// ```
/// Supported UI languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    /// English (default)
    #[default]
    En,
    /// Korean (한국어)
    Ko,
}

impl Language {
    /// Parse language code with fallback to English
    ///
    /// Accepts various formats (case-insensitive):
    /// - "en", "EN", "english" → English
    /// - "ko", "KO", "kor", "korean" → Korean
    /// - Any other value → English (fallback)
    ///
    /// # Arguments
    ///
    /// * `code` - Language code string
    ///
    /// # Returns
    ///
    /// Language enum variant, defaulting to En for unknown codes
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::i18n::Language;
    ///
    /// assert_eq!(Language::from_code("ko"), Language::Ko);
    /// assert_eq!(Language::from_code("KO"), Language::Ko);
    /// assert_eq!(Language::from_code("fr"), Language::En); // Fallback
    /// ```
    pub fn from_code(code: &str) -> Self {
        match code.trim().to_lowercase().as_str() {
            "ko" | "kor" | "korean" | "한국어" => Language::Ko,
            _ => Language::En, // Fallback to English for all other codes
        }
    }

    /// Get the standard language code
    ///
    /// # Returns
    ///
    /// Two-letter ISO 639-1 language code
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::i18n::Language;
    ///
    /// assert_eq!(Language::En.code(), "en");
    /// assert_eq!(Language::Ko.code(), "ko");
    /// ```
    pub fn code(&self) -> &'static str {
        match self {
            Language::En => "en",
            Language::Ko => "ko",
        }
    }
}

/// All translatable UI strings
///
/// Contains static string references for every piece of user-facing text
/// in the application. Organized by UI component for maintainability.
#[derive(Debug, Clone)]
pub struct Translations {
    // Header
    /// Application title displayed in header
    pub app_title: &'static str,

    // Search bar
    /// Placeholder text in search input
    pub search_placeholder: &'static str,

    // Footer - Normal mode
    /// Key hints shown in Normal mode
    pub footer_normal_hints: &'static str,
    /// Key hints shown in Insert mode
    pub footer_insert_hints: &'static str,
    /// Insert mode indicator text
    pub footer_insert_mode: &'static str,

    // Footer - View modes
    /// "All" view mode label
    pub footer_view_all: &'static str,
    /// "Favorites" view mode label
    pub footer_view_favorites: &'static str,
    /// "Recent" view mode label
    pub footer_view_recent: &'static str,

    // Skill list
    /// Message shown when no skills are available
    pub empty_skills_message: &'static str,

    // Detail pane
    /// Detail pane title
    pub detail_pane_title: &'static str,
    /// "Description:" label
    pub detail_description_label: &'static str,
    /// "Tags:" label
    pub detail_tags_label: &'static str,
    /// "Estimated Time:" label
    pub detail_estimated_time_label: &'static str,
    /// "Version:" label
    pub detail_version_label: &'static str,
    /// "Source:" label
    pub detail_source_label: &'static str,

    // Output panel
    /// Output panel title
    pub output_panel_title: &'static str,
    /// "Standard Output:" label
    pub output_panel_stdout_label: &'static str,
    /// "Standard Error:" label
    pub output_panel_stderr_label: &'static str,
    /// "Exit Code:" label
    pub output_panel_exit_code_label: &'static str,
    /// "Execution Time:" label
    pub output_panel_execution_time_label: &'static str,
    /// Hint for closing output panel
    pub output_panel_close_hint: &'static str,
}

impl Translations {
    /// Load translations for the specified language
    ///
    /// # Arguments
    ///
    /// * `language` - The language to load translations for
    ///
    /// # Returns
    ///
    /// Translations struct with all strings in the requested language
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::i18n::{Language, Translations};
    ///
    /// let en = Translations::load(Language::En);
    /// assert_eq!(en.app_title, "Pane");
    ///
    /// let ko = Translations::load(Language::Ko);
    /// assert_eq!(ko.app_title, "페인");
    /// ```
    pub fn load(language: Language) -> Self {
        match language {
            Language::En => Self::english(),
            Language::Ko => Self::korean(),
        }
    }

    /// English translations
    fn english() -> Self {
        Translations {
            // Header
            app_title: "Pane",

            // Search
            search_placeholder: "Type to search...",

            // Footer
            footer_normal_hints: "j/k Move | / Search | Enter Run | Esc Quit",
            footer_insert_hints: "Type to search | Esc Normal mode",
            footer_insert_mode: "-- INSERT --",
            footer_view_all: "All",
            footer_view_favorites: "Favorites",
            footer_view_recent: "Recent",

            // Skill list
            empty_skills_message: "No skills available",

            // Detail pane
            detail_pane_title: "Details",
            detail_description_label: "Description:",
            detail_tags_label: "Tags:",
            detail_estimated_time_label: "Estimated Time:",
            detail_version_label: "Version:",
            detail_source_label: "Source:",

            // Output panel
            output_panel_title: "Output",
            output_panel_stdout_label: "Standard Output:",
            output_panel_stderr_label: "Standard Error:",
            output_panel_exit_code_label: "Exit Code:",
            output_panel_execution_time_label: "Execution Time:",
            output_panel_close_hint: "Press Esc to close",
        }
    }

    /// Korean translations (한국어 번역)
    fn korean() -> Self {
        Translations {
            // Header
            app_title: "페인",

            // Search
            search_placeholder: "검색어를 입력하세요...",

            // Footer
            footer_normal_hints: "j/k 이동 | / 검색 | Enter 실행 | Esc 종료",
            footer_insert_hints: "검색어 입력 | Esc 일반 모드",
            footer_insert_mode: "-- 입력 --",
            footer_view_all: "전체",
            footer_view_favorites: "즐겨찾기",
            footer_view_recent: "최근",

            // Skill list
            empty_skills_message: "사용 가능한 스킬이 없습니다",

            // Detail pane
            detail_pane_title: "상세 정보",
            detail_description_label: "설명:",
            detail_tags_label: "태그:",
            detail_estimated_time_label: "예상 시간:",
            detail_version_label: "버전:",
            detail_source_label: "소스:",

            // Output panel
            output_panel_title: "출력",
            output_panel_stdout_label: "표준 출력:",
            output_panel_stderr_label: "표준 에러:",
            output_panel_exit_code_label: "종료 코드:",
            output_panel_execution_time_label: "실행 시간:",
            output_panel_close_hint: "Esc를 눌러 닫기",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_from_code_en() {
        // Arrange & Act & Assert
        assert_eq!(Language::from_code("en"), Language::En);
        assert_eq!(Language::from_code("EN"), Language::En);
        assert_eq!(Language::from_code("english"), Language::En);
    }

    #[test]
    fn test_language_from_code_ko() {
        // Arrange & Act & Assert
        assert_eq!(Language::from_code("ko"), Language::Ko);
        assert_eq!(Language::from_code("KO"), Language::Ko);
        assert_eq!(Language::from_code("kor"), Language::Ko);
        assert_eq!(Language::from_code("korean"), Language::Ko);
        assert_eq!(Language::from_code("한국어"), Language::Ko);
    }

    #[test]
    fn test_language_from_code_fallback() {
        // Arrange & Act & Assert - Unknown codes fallback to English
        assert_eq!(Language::from_code("fr"), Language::En);
        assert_eq!(Language::from_code("de"), Language::En);
        assert_eq!(Language::from_code("ja"), Language::En);
        assert_eq!(Language::from_code(""), Language::En);
        assert_eq!(Language::from_code("invalid"), Language::En);
    }

    #[test]
    fn test_language_code() {
        // Arrange & Act & Assert
        assert_eq!(Language::En.code(), "en");
        assert_eq!(Language::Ko.code(), "ko");
    }

    #[test]
    fn test_language_default() {
        // Arrange & Act
        let default_lang = Language::default();

        // Assert
        assert_eq!(default_lang, Language::En);
    }

    #[test]
    fn test_translations_load_english() {
        // Arrange & Act
        let translations = Translations::load(Language::En);

        // Assert
        assert_eq!(translations.app_title, "Pane");
        assert_eq!(translations.search_placeholder, "Type to search...");
        assert_eq!(translations.footer_insert_mode, "-- INSERT --");
        assert_eq!(translations.footer_view_all, "All");
        assert_eq!(translations.empty_skills_message, "No skills available");
    }

    #[test]
    fn test_translations_load_korean() {
        // Arrange & Act
        let translations = Translations::load(Language::Ko);

        // Assert
        assert_eq!(translations.app_title, "페인");
        assert_eq!(translations.search_placeholder, "검색어를 입력하세요...");
        assert_eq!(translations.footer_insert_mode, "-- 입력 --");
        assert_eq!(translations.footer_view_all, "전체");
        assert_eq!(
            translations.empty_skills_message,
            "사용 가능한 스킬이 없습니다"
        );
    }

    #[test]
    fn test_translations_all_keys_present_english() {
        // Arrange
        let t = Translations::load(Language::En);

        // Assert - all strings are non-empty
        assert!(!t.app_title.is_empty());
        assert!(!t.search_placeholder.is_empty());
        assert!(!t.footer_normal_hints.is_empty());
        assert!(!t.footer_insert_hints.is_empty());
        assert!(!t.footer_insert_mode.is_empty());
        assert!(!t.footer_view_all.is_empty());
        assert!(!t.footer_view_favorites.is_empty());
        assert!(!t.footer_view_recent.is_empty());
        assert!(!t.empty_skills_message.is_empty());
        assert!(!t.detail_pane_title.is_empty());
        assert!(!t.output_panel_title.is_empty());
    }

    #[test]
    fn test_translations_all_keys_present_korean() {
        // Arrange
        let t = Translations::load(Language::Ko);

        // Assert - all strings are non-empty
        assert!(!t.app_title.is_empty());
        assert!(!t.search_placeholder.is_empty());
        assert!(!t.footer_normal_hints.is_empty());
        assert!(!t.footer_insert_hints.is_empty());
        assert!(!t.footer_insert_mode.is_empty());
        assert!(!t.footer_view_all.is_empty());
        assert!(!t.footer_view_favorites.is_empty());
        assert!(!t.footer_view_recent.is_empty());
        assert!(!t.empty_skills_message.is_empty());
        assert!(!t.detail_pane_title.is_empty());
        assert!(!t.output_panel_title.is_empty());
    }
}
