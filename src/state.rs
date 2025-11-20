use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::i18n::{Language, Translations};
use crate::search::filter_skills;
use crate::skills::Skill;

/// View mode for filtering the skill list
///
/// Determines which subset of skills to display in the TUI.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum ViewMode {
    /// Show all discovered skills
    #[default]
    All,
    /// Show only skills marked as favorites
    Favorites,
    /// Show only recently executed skills
    Recent,
}

/// Input mode for the TUI
///
/// Determines how keyboard input is interpreted. Modal input design
/// inspired by Vim allows for efficient navigation (Normal mode) and
/// text entry (Insert mode) without key binding conflicts.
///
/// # Mode Descriptions
///
/// - **Normal**: Navigation and commands (j/k keys move selection, / enters Insert mode)
/// - **Insert**: All character keys are treated as search input (j/k can be typed in search)
///
/// # Examples
///
/// ```
/// use pane::state::InputMode;
///
/// let mode = InputMode::Normal;
/// assert!(matches!(mode, InputMode::Normal));
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub enum InputMode {
    /// Normal mode for navigation and commands
    #[default]
    Normal,
    /// Insert mode for search text input
    Insert,
}

/// Main application state
///
/// This struct holds all mutable state for the TUI application and serves
/// as the single source of truth for the UI. State is updated in response
/// to user input events and used to render the interface.
#[derive(Debug)]
pub struct AppState {
    /// All discovered skills (immutable after initialization)
    skills: Vec<Skill>,
    /// Indices into `skills` vec after filtering (updated on search/view change)
    filtered_skills: Vec<usize>,
    /// Index into `filtered_skills` for the currently selected skill
    selected_index: usize,
    /// Current search query text
    search_query: String,
    /// Current view mode filter
    view_mode: ViewMode,
    /// Current input mode (Normal or Insert)
    input_mode: InputMode,
    /// Skill IDs marked as favorites
    favorites: HashSet<String>,
    /// Recently executed skill IDs (ordered, most recent first)
    recent: Vec<String>,
    /// User configuration
    config: Config,
    /// Resolved theme (cached from config at startup for efficient access)
    resolved_theme: crate::ui::theme::ThemeConfig,
    /// UI translations (cached from config language at startup)
    translations: Translations,
    /// Flag indicating the application should exit
    should_quit: bool,
    /// Scroll offset for the skill list (for auto-scrolling)
    scroll_offset: usize,
    /// Flag indicating an inline skill is currently executing
    executing_inline: bool,
    /// Status message for inline execution (e.g., "Executing skill-name...")
    inline_execution_status: Option<String>,
    /// Active skill output for display in output panel
    active_output: Option<crate::skills::output::SkillOutput>,
    /// Flag indicating the output panel is visible
    output_panel_visible: bool,
    /// Scroll offset for the output panel (current line position)
    output_scroll_offset: usize,
}

impl AppState {
    /// Create a new AppState with the given skills and configuration
    ///
    /// Initializes the state with all skills visible (no filter applied),
    /// selection at index 0, and view mode from config.
    ///
    /// # Arguments
    ///
    /// * `skills` - Vector of discovered skills
    /// * `config` - User configuration
    ///
    /// # Returns
    ///
    /// A new AppState ready for use in the event loop
    pub fn new(skills: Vec<Skill>, config: Config) -> Self {
        let filtered_skills: Vec<usize> = (0..skills.len()).collect();
        let view_mode = config.default_view_mode.clone();
        let resolved_theme = config.theme.clone().unwrap_or_default();

        // Load translations based on configured language
        let language = Language::from_code(&config.language);
        let translations = Translations::load(language);

        Self {
            skills,
            filtered_skills,
            selected_index: 0,
            search_query: String::new(),
            view_mode,
            input_mode: InputMode::Normal,
            favorites: HashSet::new(),
            recent: Vec::new(),
            config,
            resolved_theme,
            translations,
            should_quit: false,
            scroll_offset: 0,
            executing_inline: false,
            inline_execution_status: None,
            active_output: None,
            output_panel_visible: false,
            output_scroll_offset: 0,
        }
    }

    /// Get the currently selected skill, if any
    ///
    /// Returns None if the filtered list is empty.
    ///
    /// # Returns
    ///
    /// Reference to the selected skill, or None if list is empty
    pub fn selected_skill(&self) -> Option<&Skill> {
        self.filtered_skills
            .get(self.selected_index)
            .and_then(|&idx| self.skills.get(idx))
    }

    /// Move selection up in the filtered list
    ///
    /// Wraps to the bottom if already at the top.
    /// Updates scroll offset to keep selection visible.
    pub fn move_selection_up(&mut self) {
        if self.filtered_skills.is_empty() {
            return;
        }

        if self.selected_index == 0 {
            self.selected_index = self.filtered_skills.len() - 1;
        } else {
            self.selected_index -= 1;
        }

        // Update scroll offset if selection moved above visible area
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
    }

    /// Move selection down in the filtered list
    ///
    /// Wraps to the top if already at the bottom.
    /// Updates scroll offset to keep selection visible.
    pub fn move_selection_down(&mut self) {
        if self.filtered_skills.is_empty() {
            return;
        }

        self.selected_index = (self.selected_index + 1) % self.filtered_skills.len();

        // Reset scroll when wrapping to top
        if self.selected_index == 0 {
            self.scroll_offset = 0;
        }
    }

    /// Update the search query and re-filter skills
    ///
    /// Updates the search query, applies fuzzy filtering combined with view mode filtering,
    /// and resets selection to the first filtered result.
    ///
    /// # Arguments
    ///
    /// * `query` - New search query text
    ///
    /// # Behavior
    ///
    /// - Empty query shows all skills in current view mode
    /// - Non-empty query filters using fuzzy matching across name, ID, tags, and description
    /// - Search filtering is applied AFTER view mode filtering
    /// - Results are ranked by match score (best matches first)
    /// - Selection and scroll offset reset to 0
    pub fn set_search_query(&mut self, query: String) {
        self.search_query = query;
        self.apply_view_filter();
    }

    /// Append a character to the search query
    ///
    /// Adds a character to the end of the current search query and re-filters
    /// the skill list within the current view mode. Selection resets to the first result.
    ///
    /// # Arguments
    ///
    /// * `ch` - Character to append
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pane::{AppState, Config};
    /// # let mut state = AppState::new(vec![], Config::default());
    /// state.append_to_search('c');
    /// state.append_to_search('l');
    /// state.append_to_search('a');
    /// state.append_to_search('u');
    /// assert_eq!(state.search_query(), "clau");
    /// ```
    pub fn append_to_search(&mut self, ch: char) {
        self.search_query.push(ch);
        self.apply_view_filter();
    }

    /// Remove the last character from the search query
    ///
    /// Removes the last character from the current search query (backspace behavior)
    /// and re-filters the skill list within the current view mode. If the query is
    /// already empty, this is a no-op. Selection resets to the first result.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pane::{AppState, Config};
    /// # let mut state = AppState::new(vec![], Config::default());
    /// state.set_search_query("clau".to_string());
    /// state.remove_from_search(); // "clau" -> "cla"
    /// assert_eq!(state.search_query(), "cla");
    /// ```
    pub fn remove_from_search(&mut self) {
        if !self.search_query.is_empty() {
            self.search_query.pop();
            self.apply_view_filter();
        }
    }

    /// Get the current search query
    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    /// Get the current view mode
    pub fn view_mode(&self) -> &ViewMode {
        &self.view_mode
    }

    /// Check if the application should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Set the quit flag
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Check if an inline skill is currently executing
    pub fn is_executing_inline(&self) -> bool {
        self.executing_inline
    }

    /// Get the current inline execution status message
    pub fn inline_execution_status(&self) -> Option<&str> {
        self.inline_execution_status.as_deref()
    }

    /// Start inline skill execution
    ///
    /// Sets the executing_inline flag to true and stores a status message
    /// indicating which skill is executing. This is used to display an
    /// "Executing..." indicator in the TUI.
    ///
    /// # Arguments
    ///
    /// * `skill_name` - Name of the skill being executed
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pane::{AppState, Config};
    /// # let mut state = AppState::new(vec![], Config::default());
    /// state.start_inline_execution("my-skill".to_string());
    /// assert!(state.is_executing_inline());
    /// ```
    pub fn start_inline_execution(&mut self, skill_name: String) {
        self.executing_inline = true;
        self.inline_execution_status = Some(format!("Executing {}...", skill_name));
    }

    /// Finish inline skill execution
    ///
    /// Clears the executing_inline flag and status message. The output is stored
    /// for later display (Story 3.2).
    ///
    /// # Arguments
    ///
    /// * `_output` - The captured skill output (stored in Story 3.2)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pane::{AppState, Config};
    /// # use pane::skills::output::SkillOutput;
    /// # use std::time::Duration;
    /// # let mut state = AppState::new(vec![], Config::default());
    /// # let output = SkillOutput {
    /// #     stdout: "test".to_string(),
    /// #     stderr: String::new(),
    /// #     exit_code: Some(0),
    /// #     truncated: false,
    /// #     execution_time: Duration::from_secs(1),
    /// # };
    /// state.start_inline_execution("my-skill".to_string());
    /// state.finish_inline_execution(output);
    /// assert!(!state.is_executing_inline());
    /// ```
    pub fn finish_inline_execution(&mut self, output: crate::skills::output::SkillOutput) {
        self.executing_inline = false;
        self.inline_execution_status = None;
        // Show output panel automatically after execution completes
        self.show_output_panel(output);
    }

    /// Show the output panel with the given skill output
    ///
    /// Sets the output panel visible, stores the output, and resets scroll position.
    ///
    /// # Arguments
    ///
    /// * `output` - The skill output to display
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pane::{AppState, Config};
    /// # use pane::skills::output::SkillOutput;
    /// # use std::time::Duration;
    /// # let mut state = AppState::new(vec![], Config::default());
    /// # let output = SkillOutput {
    /// #     stdout: "test".to_string(),
    /// #     stderr: String::new(),
    /// #     exit_code: Some(0),
    /// #     truncated: false,
    /// #     execution_time: Duration::from_secs(1),
    /// # };
    /// state.show_output_panel(output);
    /// assert!(state.is_output_panel_visible());
    /// ```
    pub fn show_output_panel(&mut self, output: crate::skills::output::SkillOutput) {
        self.active_output = Some(output);
        self.output_panel_visible = true;
        self.output_scroll_offset = 0;
    }

    /// Hide the output panel
    ///
    /// Clears the output panel visibility flag and stored output.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pane::{AppState, Config};
    /// # use pane::skills::output::SkillOutput;
    /// # use std::time::Duration;
    /// # let mut state = AppState::new(vec![], Config::default());
    /// # let output = SkillOutput {
    /// #     stdout: "test".to_string(),
    /// #     stderr: String::new(),
    /// #     exit_code: Some(0),
    /// #     truncated: false,
    /// #     execution_time: Duration::from_secs(1),
    /// # };
    /// state.show_output_panel(output);
    /// state.hide_output_panel();
    /// assert!(!state.is_output_panel_visible());
    /// ```
    pub fn hide_output_panel(&mut self) {
        self.output_panel_visible = false;
        self.active_output = None;
    }

    /// Scroll the output panel up by one line
    ///
    /// Decrements the scroll offset with boundary check at 0.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pane::{AppState, Config};
    /// # use pane::skills::output::SkillOutput;
    /// # use std::time::Duration;
    /// # let mut state = AppState::new(vec![], Config::default());
    /// # let output = SkillOutput {
    /// #     stdout: "line1\nline2\nline3".to_string(),
    /// #     stderr: String::new(),
    /// #     exit_code: Some(0),
    /// #     truncated: false,
    /// #     execution_time: Duration::from_secs(1),
    /// # };
    /// state.show_output_panel(output);
    /// state.scroll_output_down(10); // Scroll down first
    /// state.scroll_output_up();
    /// ```
    pub fn scroll_output_up(&mut self) {
        self.output_scroll_offset = self.output_scroll_offset.saturating_sub(1);
    }

    /// Scroll the output panel down by one line
    ///
    /// Increments the scroll offset if not at maximum. The maximum is calculated
    /// automatically from the active output content (total lines of stdout + stderr).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pane::{AppState, Config};
    /// # use pane::skills::output::SkillOutput;
    /// # use std::time::Duration;
    /// # let mut state = AppState::new(vec![], Config::default());
    /// # let output = SkillOutput {
    /// #     stdout: "line1\nline2\nline3".to_string(),
    /// #     stderr: String::new(),
    /// #     exit_code: Some(0),
    /// #     truncated: false,
    /// #     execution_time: Duration::from_secs(1),
    /// # };
    /// state.show_output_panel(output);
    /// state.scroll_output_down();
    /// ```
    pub fn scroll_output_down(&mut self) {
        // Calculate max_offset from active output content
        let max_offset = if let Some(output) = &self.active_output {
            let stdout_lines = if output.stdout.is_empty() {
                0
            } else {
                output.stdout.lines().count()
            };
            let stderr_lines = if output.stderr.is_empty() {
                0
            } else {
                // +1 for separator line if stderr present
                output.stderr.lines().count() + 1
            };
            let total_lines = stdout_lines + stderr_lines;
            // Assume typical visible height of 20 lines (will be clamped by renderer anyway)
            total_lines.saturating_sub(20)
        } else {
            0
        };

        if self.output_scroll_offset < max_offset {
            self.output_scroll_offset += 1;
        }
    }

    /// Check if output panel is visible
    ///
    /// # Returns
    ///
    /// true if the output panel is currently visible, false otherwise
    pub fn is_output_panel_visible(&self) -> bool {
        self.output_panel_visible
    }

    /// Get the active output for display
    ///
    /// # Returns
    ///
    /// Reference to the active SkillOutput, or None if no output is stored
    pub fn active_output(&self) -> Option<&crate::skills::output::SkillOutput> {
        self.active_output.as_ref()
    }

    /// Get the current output panel scroll offset
    ///
    /// # Returns
    ///
    /// The current scroll offset (line number)
    pub fn output_scroll_offset(&self) -> usize {
        self.output_scroll_offset
    }

    /// Get the filtered skills for rendering
    ///
    /// Returns an iterator over the filtered skills in display order.
    pub fn filtered_skills(&self) -> impl Iterator<Item = &Skill> {
        self.filtered_skills
            .iter()
            .filter_map(move |&idx| self.skills.get(idx))
    }

    /// Get the number of filtered skills
    pub fn filtered_count(&self) -> usize {
        self.filtered_skills.len()
    }

    /// Get the selected index in the filtered list
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Get the scroll offset for the skill list
    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    /// Get a reference to the user configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the theme configuration
    ///
    /// Returns the resolved theme (user-configured or default).
    /// This method is efficient and does not clone the theme on each access.
    ///
    /// # Returns
    ///
    /// Reference to the ThemeConfig to use for rendering
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::state::AppState;
    /// use pane::config::Config;
    ///
    /// let config = Config::default();
    /// let state = AppState::new(vec![], config);
    /// let theme = state.theme();
    /// ```
    pub fn theme(&self) -> &crate::ui::theme::ThemeConfig {
        &self.resolved_theme
    }

    /// Get the UI translations
    ///
    /// Returns the translations loaded based on the configured language.
    /// This method is efficient and does not clone the translations on each access.
    ///
    /// # Returns
    ///
    /// Reference to the Translations to use for rendering UI text
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::state::AppState;
    /// use pane::config::Config;
    ///
    /// let config = Config::default();
    /// let state = AppState::new(vec![], config);
    /// let translations = state.translations();
    /// assert_eq!(translations.app_title, "Pane");
    /// ```
    pub fn translations(&self) -> &Translations {
        &self.translations
    }

    /// Move selection down by one page
    ///
    /// Advances the selection by `page_size` items, stopping at the end
    /// of the filtered list. Updates scroll offset to keep selection visible.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Number of items to advance (typically terminal height - header - footer)
    pub fn move_selection_page_down(&mut self, page_size: usize) {
        if self.filtered_skills.is_empty() {
            return;
        }

        let max_index = self.filtered_skills.len() - 1;
        self.selected_index = (self.selected_index + page_size).min(max_index);
        self.update_scroll_offset(page_size);
    }

    /// Move selection up by one page
    ///
    /// Moves the selection backward by `page_size` items, stopping at the start
    /// of the filtered list. Updates scroll offset to keep selection visible.
    ///
    /// # Arguments
    ///
    /// * `page_size` - Number of items to move back (typically terminal height - header - footer)
    pub fn move_selection_page_up(&mut self, page_size: usize) {
        if self.filtered_skills.is_empty() {
            return;
        }

        self.selected_index = self.selected_index.saturating_sub(page_size);
        self.update_scroll_offset(page_size);
    }

    /// Update scroll offset to keep selected item visible
    ///
    /// Adjusts the scroll offset so the selected item is always within
    /// the visible area. Centers the selection when possible.
    ///
    /// # Arguments
    ///
    /// * `visible_height` - Number of items visible in the list area
    fn update_scroll_offset(&mut self, visible_height: usize) {
        if visible_height == 0 {
            return;
        }

        // If selected item is above visible area, scroll up
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        }
        // If selected item is below visible area, scroll down
        else if self.selected_index >= self.scroll_offset + visible_height {
            self.scroll_offset = self.selected_index - visible_height + 1;
        }
    }

    /// Cycle to the next view mode
    ///
    /// Transitions through the view modes in order: All → Favorites → Recent → All.
    /// After cycling, caller should call `apply_view_filter()` to update the displayed skills.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pane::{AppState, Config};
    /// # let mut state = AppState::new(vec![], Config::default());
    /// state.cycle_view_mode(); // All → Favorites
    /// state.cycle_view_mode(); // Favorites → Recent
    /// state.cycle_view_mode(); // Recent → All
    /// ```
    pub fn cycle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::All => ViewMode::Favorites,
            ViewMode::Favorites => ViewMode::Recent,
            ViewMode::Recent => ViewMode::All,
        };
    }

    /// Get the current input mode
    ///
    /// # Returns
    ///
    /// Reference to the current input mode (Normal or Insert)
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::state::{AppState, InputMode};
    /// use pane::config::Config;
    ///
    /// let state = AppState::new(vec![], Config::default());
    /// assert_eq!(state.input_mode(), &InputMode::Normal);
    /// ```
    pub fn input_mode(&self) -> &InputMode {
        &self.input_mode
    }

    /// Enter Insert mode for search text input
    ///
    /// Switches the input mode from Normal to Insert, enabling all character
    /// keys to be captured as search input (including j, k, etc.).
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::state::{AppState, InputMode};
    /// use pane::config::Config;
    ///
    /// let mut state = AppState::new(vec![], Config::default());
    /// state.enter_insert_mode();
    /// assert_eq!(state.input_mode(), &InputMode::Insert);
    /// ```
    pub fn enter_insert_mode(&mut self) {
        self.input_mode = InputMode::Insert;
    }

    /// Enter Normal mode for navigation and commands
    ///
    /// Switches the input mode from Insert to Normal, enabling navigation
    /// keys (j/k) and command keys (/, f, ?).
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::state::{AppState, InputMode};
    /// use pane::config::Config;
    ///
    /// let mut state = AppState::new(vec![], Config::default());
    /// state.enter_insert_mode();
    /// state.enter_normal_mode();
    /// assert_eq!(state.input_mode(), &InputMode::Normal);
    /// ```
    pub fn enter_normal_mode(&mut self) {
        self.input_mode = InputMode::Normal;
    }

    /// Check if currently in Insert mode
    ///
    /// # Returns
    ///
    /// true if in Insert mode, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::state::AppState;
    /// use pane::config::Config;
    ///
    /// let mut state = AppState::new(vec![], Config::default());
    /// assert!(!state.is_insert_mode());
    /// state.enter_insert_mode();
    /// assert!(state.is_insert_mode());
    /// ```
    pub fn is_insert_mode(&self) -> bool {
        matches!(self.input_mode, InputMode::Insert)
    }

    /// Check if currently in Normal mode
    ///
    /// # Returns
    ///
    /// true if in Normal mode, false otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::state::AppState;
    /// use pane::config::Config;
    ///
    /// let state = AppState::new(vec![], Config::default());
    /// assert!(state.is_normal_mode());
    /// ```
    pub fn is_normal_mode(&self) -> bool {
        matches!(self.input_mode, InputMode::Normal)
    }

    /// Add a skill to the recent list
    ///
    /// Adds the skill ID to the front of the recent list (most recent first).
    /// If the skill is already in the list, it is moved to the front.
    /// Enforces the max_recent_skills limit by removing the oldest entries.
    ///
    /// # Arguments
    ///
    /// * `skill_id` - ID of the skill to add to recent list
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pane::{AppState, Config};
    /// # let mut state = AppState::new(vec![], Config::default());
    /// state.add_to_recent("claude-tips".to_string());
    /// ```
    pub fn add_to_recent(&mut self, skill_id: String) {
        // Remove skill from list if already present
        self.recent.retain(|id| id != &skill_id);

        // Add to front of list
        self.recent.insert(0, skill_id);

        // Enforce max_recent_skills limit
        if self.recent.len() > self.config.max_recent_skills {
            self.recent.truncate(self.config.max_recent_skills);
        }
    }

    /// Apply view mode and search filters to update the filtered skills list
    ///
    /// Filters skills based on the current view mode, then applies the search query filter
    /// on top of the view-filtered results. Resets selection to the first result.
    ///
    /// # Filter Order
    ///
    /// 1. **View Mode Filter**: Filter skills by current view mode
    ///    - `ViewMode::All` - Show all skills
    ///    - `ViewMode::Favorites` - Show only favorited skills
    ///    - `ViewMode::Recent` - Show only recently executed skills (up to `max_recent_skills`)
    /// 2. **Search Filter**: Apply fuzzy search query to view-filtered results
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use pane::{AppState, Config};
    /// # let mut state = AppState::new(vec![], Config::default());
    /// state.cycle_view_mode(); // Change to Favorites
    /// state.apply_view_filter(); // Update filtered list
    /// ```
    pub fn apply_view_filter(&mut self) {
        // Step 1: Filter by view mode
        let view_filtered: Vec<usize> = match self.view_mode {
            ViewMode::All => (0..self.skills.len()).collect(),
            ViewMode::Favorites => self
                .skills
                .iter()
                .enumerate()
                .filter(|(_, skill)| self.favorites.contains(&skill.manifest.id))
                .map(|(idx, _)| idx)
                .collect(),
            ViewMode::Recent => {
                let max_recent = self.config.max_recent_skills;
                let recent_set: HashSet<&String> = self.recent.iter().take(max_recent).collect();
                self.skills
                    .iter()
                    .enumerate()
                    .filter(|(_, skill)| recent_set.contains(&skill.manifest.id))
                    .map(|(idx, _)| idx)
                    .collect()
            }
        };

        // Step 2: Apply search query filter on view-filtered results
        if self.search_query.is_empty() {
            self.filtered_skills = view_filtered;
        } else {
            // Use existing search filtering on view-filtered subset
            let search_filtered = filter_skills(&self.search_query, &self.skills);
            self.filtered_skills = search_filtered
                .into_iter()
                .filter(|idx| view_filtered.contains(idx))
                .collect();
        }

        // Step 3: Reset selection and scroll offset to avoid out-of-bounds
        self.selected_index = 0;
        self.scroll_offset = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skills::{SkillManifest, SkillSource};
    use std::path::PathBuf;

    fn create_test_skill(id: &str, name: &str) -> Skill {
        Skill {
            manifest: SkillManifest {
                id: id.to_string(),
                name: name.to_string(),
                description: "Test skill".to_string(),
                version: "1.0.0".to_string(),
                exec: "test".to_string(),
                args: vec![],
                tags: vec![],
                estimated_time: None,
                ui: crate::skills::manifest::UiConfig {
                    mode: crate::skills::manifest::UiMode::Tui,
                    fullscreen: true,
                },
                context: crate::skills::manifest::ContextConfig::default(),
            },
            source: SkillSource::Project,
            manifest_path: PathBuf::from("test.yaml"),
        }
    }

    fn create_test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_app_state_new_initializes_correctly() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
        ];
        let config = create_test_config();

        // Act
        let state = AppState::new(skills, config);

        // Assert
        assert_eq!(state.filtered_count(), 2);
        assert_eq!(state.selected_index(), 0);
        assert_eq!(state.search_query(), "");
        assert_eq!(state.view_mode(), &ViewMode::All);
        assert!(!state.should_quit());
    }

    #[test]
    fn test_move_selection_down_increments_index() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
        ];
        let mut state = AppState::new(skills, create_test_config());

        // Act
        state.move_selection_down();

        // Assert
        assert_eq!(state.selected_index(), 1);
    }

    #[test]
    fn test_move_selection_down_wraps_at_end() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
        ];
        let mut state = AppState::new(skills, create_test_config());
        state.selected_index = 1;

        // Act
        state.move_selection_down();

        // Assert
        assert_eq!(state.selected_index(), 0);
    }

    #[test]
    fn test_move_selection_up_decrements_index() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
        ];
        let mut state = AppState::new(skills, create_test_config());
        state.selected_index = 2;

        // Act
        state.move_selection_up();

        // Assert
        assert_eq!(state.selected_index(), 1);
    }

    #[test]
    fn test_move_selection_up_wraps_at_start() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
        ];
        let mut state = AppState::new(skills, create_test_config());

        // Act
        state.move_selection_up();

        // Assert
        assert_eq!(state.selected_index(), 1);
    }

    #[test]
    fn test_set_search_query_resets_selection() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
        ];
        let mut state = AppState::new(skills, create_test_config());
        state.selected_index = 1;

        // Act
        state.set_search_query("test".to_string());

        // Assert
        assert_eq!(state.search_query(), "test");
        assert_eq!(state.selected_index(), 0);
    }

    #[test]
    fn test_selected_skill_returns_correct_skill() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
        ];
        let mut state = AppState::new(skills, create_test_config());
        state.selected_index = 1;

        // Act
        let selected = state.selected_skill();

        // Assert
        assert!(selected.is_some());
        assert_eq!(selected.unwrap().manifest.id, "skill2");
    }

    #[test]
    fn test_selected_skill_returns_none_when_empty() {
        // Arrange
        let state = AppState::new(vec![], create_test_config());

        // Act
        let selected = state.selected_skill();

        // Assert
        assert!(selected.is_none());
    }

    #[test]
    fn test_move_selection_page_down_advances_by_page_size() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
            create_test_skill("skill4", "Skill 4"),
            create_test_skill("skill5", "Skill 5"),
        ];
        let mut state = AppState::new(skills, create_test_config());

        // Act
        state.move_selection_page_down(3);

        // Assert
        assert_eq!(state.selected_index(), 3);
    }

    #[test]
    fn test_move_selection_page_down_stops_at_end() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
        ];
        let mut state = AppState::new(skills, create_test_config());

        // Act
        state.move_selection_page_down(10);

        // Assert
        assert_eq!(state.selected_index(), 2); // Last index
    }

    #[test]
    fn test_move_selection_page_up_goes_back_by_page_size() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
            create_test_skill("skill4", "Skill 4"),
            create_test_skill("skill5", "Skill 5"),
        ];
        let mut state = AppState::new(skills, create_test_config());
        state.selected_index = 4;

        // Act
        state.move_selection_page_up(3);

        // Assert
        assert_eq!(state.selected_index(), 1);
    }

    #[test]
    fn test_move_selection_page_up_stops_at_start() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
        ];
        let mut state = AppState::new(skills, create_test_config());
        state.selected_index = 1;

        // Act
        state.move_selection_page_up(10);

        // Assert
        assert_eq!(state.selected_index(), 0);
    }

    #[test]
    fn test_scroll_offset_updates_when_selection_moves() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
            create_test_skill("skill4", "Skill 4"),
            create_test_skill("skill5", "Skill 5"),
        ];
        let mut state = AppState::new(skills, create_test_config());

        // Act - move beyond visible area (assuming page size of 2)
        state.move_selection_page_down(2);

        // Assert - scroll offset should be non-zero
        assert_eq!(state.selected_index(), 2);
        assert_eq!(state.scroll_offset(), 1); // Adjusted to keep selection visible
    }

    #[test]
    fn test_append_to_search_updates_query_and_filters() {
        // Arrange
        let skills = vec![
            create_test_skill("claude-tips", "Claude Tips"),
            create_test_skill("docker-build", "Docker Build"),
        ];
        let mut state = AppState::new(skills, create_test_config());

        // Act
        state.append_to_search('c');
        state.append_to_search('l');
        state.append_to_search('a');
        state.append_to_search('u');

        // Assert
        assert_eq!(state.search_query(), "clau");
        // Filtered list should contain claude-tips (fuzzy matches)
        assert!(state.filtered_count() > 0);
    }

    #[test]
    fn test_remove_from_search_handles_backspace() {
        // Arrange
        let skills = vec![
            create_test_skill("claude-tips", "Claude Tips"),
            create_test_skill("docker-build", "Docker Build"),
        ];
        let mut state = AppState::new(skills, create_test_config());
        state.set_search_query("clau".to_string());

        // Act
        state.remove_from_search();

        // Assert
        assert_eq!(state.search_query(), "cla");
    }

    #[test]
    fn test_remove_from_search_handles_empty_query() {
        // Arrange
        let skills = vec![create_test_skill("skill1", "Skill 1")];
        let mut state = AppState::new(skills, create_test_config());

        // Act
        state.remove_from_search();

        // Assert - should be no-op
        assert_eq!(state.search_query(), "");
    }

    #[test]
    fn test_update_search_query_resets_selected_index() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
        ];
        let mut state = AppState::new(skills, create_test_config());
        state.selected_index = 1;

        // Act
        state.set_search_query("test".to_string());

        // Assert
        assert_eq!(state.selected_index(), 0);
        assert_eq!(state.scroll_offset(), 0);
    }

    #[test]
    fn test_update_search_query_empty_shows_all_skills() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
        ];
        let mut state = AppState::new(skills, create_test_config());
        state.set_search_query("test".to_string()); // Filter

        // Act
        state.set_search_query("".to_string()); // Clear filter

        // Assert
        assert_eq!(state.filtered_count(), 3); // All skills visible
    }

    #[test]
    fn test_cycle_view_mode_all_to_favorites() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());
        assert_eq!(state.view_mode(), &ViewMode::All);

        // Act
        state.cycle_view_mode();

        // Assert
        assert_eq!(state.view_mode(), &ViewMode::Favorites);
    }

    #[test]
    fn test_cycle_view_mode_favorites_to_recent() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());
        state.cycle_view_mode(); // All -> Favorites

        // Act
        state.cycle_view_mode();

        // Assert
        assert_eq!(state.view_mode(), &ViewMode::Recent);
    }

    #[test]
    fn test_cycle_view_mode_recent_to_all() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());
        state.cycle_view_mode(); // All -> Favorites
        state.cycle_view_mode(); // Favorites -> Recent

        // Act
        state.cycle_view_mode();

        // Assert
        assert_eq!(state.view_mode(), &ViewMode::All);
    }

    #[test]
    fn test_apply_view_filter_all_shows_all_skills() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
        ];
        let mut state = AppState::new(skills, create_test_config());

        // Act
        state.apply_view_filter();

        // Assert
        assert_eq!(state.filtered_count(), 3);
        assert_eq!(state.view_mode(), &ViewMode::All);
    }

    #[test]
    fn test_apply_view_filter_favorites_shows_only_favorited() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
        ];
        let mut state = AppState::new(skills, create_test_config());
        state.favorites.insert("skill1".to_string());
        state.favorites.insert("skill3".to_string());

        // Act - Switch to Favorites mode and apply filter
        state.cycle_view_mode(); // All -> Favorites
        state.apply_view_filter();

        // Assert
        assert_eq!(state.filtered_count(), 2);
        assert_eq!(state.view_mode(), &ViewMode::Favorites);
    }

    #[test]
    fn test_apply_view_filter_recent_shows_only_recent_skills() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
        ];
        let mut state = AppState::new(skills, create_test_config());
        state.recent.push("skill2".to_string());
        state.recent.push("skill1".to_string());

        // Act - Switch to Recent mode and apply filter
        state.cycle_view_mode(); // All -> Favorites
        state.cycle_view_mode(); // Favorites -> Recent
        state.apply_view_filter();

        // Assert
        assert_eq!(state.filtered_count(), 2);
        assert_eq!(state.view_mode(), &ViewMode::Recent);
    }

    #[test]
    fn test_apply_view_filter_recent_respects_max_recent_limit() {
        // Arrange
        let skills = vec![
            create_test_skill("skill1", "Skill 1"),
            create_test_skill("skill2", "Skill 2"),
            create_test_skill("skill3", "Skill 3"),
            create_test_skill("skill4", "Skill 4"),
            create_test_skill("skill5", "Skill 5"),
        ];
        let mut config = create_test_config();
        config.max_recent_skills = 3; // Limit to 3 recent skills
        let mut state = AppState::new(skills, config);

        // Add 5 recent skills (more than max_recent_skills limit)
        state.recent.push("skill5".to_string());
        state.recent.push("skill4".to_string());
        state.recent.push("skill3".to_string());
        state.recent.push("skill2".to_string());
        state.recent.push("skill1".to_string());

        // Act - Switch to Recent mode and apply filter
        state.cycle_view_mode(); // All -> Favorites
        state.cycle_view_mode(); // Favorites -> Recent
        state.apply_view_filter();

        // Assert - Only first 3 recent skills should be shown
        assert_eq!(state.filtered_count(), 3);
        assert_eq!(state.view_mode(), &ViewMode::Recent);
    }

    #[test]
    fn test_view_and_search_filters_combine_correctly() {
        // Arrange
        let skills = vec![
            create_test_skill("claude-tips", "Claude Tips"),
            create_test_skill("docker-build", "Docker Build"),
            create_test_skill("git-helper", "Git Helper"),
        ];
        let mut state = AppState::new(skills, create_test_config());

        // Mark only claude-tips and git-helper as favorites
        state.favorites.insert("claude-tips".to_string());
        state.favorites.insert("git-helper".to_string());

        // Switch to Favorites mode
        state.cycle_view_mode(); // All -> Favorites

        // Act - Apply search query within Favorites view
        state.set_search_query("git".to_string());
        state.apply_view_filter();

        // Assert - Should only show git-helper (in favorites AND matches search)
        assert_eq!(state.filtered_count(), 1);
        let filtered_skills: Vec<&Skill> = state.filtered_skills().collect();
        assert_eq!(filtered_skills[0].manifest.id, "git-helper");
    }

    #[test]
    fn test_app_state_theme_returns_custom_theme() {
        // Arrange
        let mut config = create_test_config();
        let custom_theme = crate::ui::theme::ThemeConfig {
            primary: ratatui::style::Color::Red,
            ..crate::ui::theme::ThemeConfig::default()
        };
        config.theme = Some(custom_theme.clone());
        let state = AppState::new(vec![], config);

        // Act
        let theme = state.theme();

        // Assert
        assert_eq!(theme.primary, ratatui::style::Color::Red);
    }

    #[test]
    fn test_app_state_theme_returns_default_when_none() {
        // Arrange
        let config = create_test_config();
        let state = AppState::new(vec![], config);

        // Act
        let theme = state.theme();

        // Assert
        assert_eq!(theme.primary, ratatui::style::Color::Cyan);
        assert_eq!(*theme, crate::ui::theme::ThemeConfig::default());
    }

    // Output panel state management tests
    #[test]
    fn test_show_output_panel_sets_visibility() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());
        let output = crate::skills::output::SkillOutput {
            stdout: "test output".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
            truncated: false,
            execution_time: std::time::Duration::from_millis(100),
        };

        // Act
        state.show_output_panel(output);

        // Assert
        assert!(state.is_output_panel_visible());
        assert!(state.active_output().is_some());
        assert_eq!(state.output_scroll_offset(), 0);
    }

    #[test]
    fn test_hide_output_panel_clears_state() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());
        let output = crate::skills::output::SkillOutput {
            stdout: "test output".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
            truncated: false,
            execution_time: std::time::Duration::from_millis(100),
        };
        state.show_output_panel(output);

        // Act
        state.hide_output_panel();

        // Assert
        assert!(!state.is_output_panel_visible());
        assert!(state.active_output().is_none());
    }

    #[test]
    fn test_scroll_output_up_decrements_offset() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());
        let output = crate::skills::output::SkillOutput {
            stdout: "line1\nline2\nline3".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
            truncated: false,
            execution_time: std::time::Duration::from_millis(100),
        };
        state.show_output_panel(output);
        state.scroll_output_down(); // Scroll down first

        // Act
        state.scroll_output_up();

        // Assert
        assert_eq!(state.output_scroll_offset(), 0);
    }

    #[test]
    fn test_scroll_output_up_stops_at_zero() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());
        let output = crate::skills::output::SkillOutput {
            stdout: "line1\nline2\nline3".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
            truncated: false,
            execution_time: std::time::Duration::from_millis(100),
        };
        state.show_output_panel(output);

        // Act
        state.scroll_output_up();
        state.scroll_output_up();

        // Assert
        assert_eq!(state.output_scroll_offset(), 0);
    }

    #[test]
    fn test_scroll_output_down_increments_offset() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());
        // Create output with 25 lines (more than visible height of 20)
        let mut lines = Vec::new();
        for i in 1..=25 {
            lines.push(format!("line{}", i));
        }
        let output = crate::skills::output::SkillOutput {
            stdout: lines.join("\n"),
            stderr: String::new(),
            exit_code: Some(0),
            truncated: false,
            execution_time: std::time::Duration::from_millis(100),
        };
        state.show_output_panel(output);

        // Act
        state.scroll_output_down();

        // Assert
        assert_eq!(state.output_scroll_offset(), 1);
    }

    #[test]
    fn test_scroll_output_down_stops_at_max() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());
        let output = crate::skills::output::SkillOutput {
            stdout: "line1\nline2\nline3".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
            truncated: false,
            execution_time: std::time::Duration::from_millis(100),
        };
        state.show_output_panel(output);

        // Act - scroll down 10 times (but output only has 3 lines, so max_offset = 0)
        for _ in 0..10 {
            state.scroll_output_down();
        }

        // Assert - should stop at 0 (3 lines - 20 visible height assumption = 0, clamped)
        assert_eq!(state.output_scroll_offset(), 0);
    }

    #[test]
    fn test_finish_inline_execution_shows_output_panel() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());
        let output = crate::skills::output::SkillOutput {
            stdout: "test output".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
            truncated: false,
            execution_time: std::time::Duration::from_millis(100),
        };
        state.start_inline_execution("test-skill".to_string());

        // Act
        state.finish_inline_execution(output);

        // Assert
        assert!(!state.is_executing_inline());
        assert!(state.is_output_panel_visible());
        assert!(state.active_output().is_some());
    }

    // InputMode tests
    #[test]
    fn test_input_mode_defaults_to_normal() {
        // Arrange & Act
        let state = AppState::new(vec![], create_test_config());

        // Assert
        assert_eq!(state.input_mode(), &InputMode::Normal);
        assert!(state.is_normal_mode());
        assert!(!state.is_insert_mode());
    }

    #[test]
    fn test_enter_insert_mode_changes_state() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());

        // Act
        state.enter_insert_mode();

        // Assert
        assert_eq!(state.input_mode(), &InputMode::Insert);
        assert!(state.is_insert_mode());
        assert!(!state.is_normal_mode());
    }

    #[test]
    fn test_enter_normal_mode_changes_state() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());
        state.enter_insert_mode();

        // Act
        state.enter_normal_mode();

        // Assert
        assert_eq!(state.input_mode(), &InputMode::Normal);
        assert!(state.is_normal_mode());
        assert!(!state.is_insert_mode());
    }

    #[test]
    fn test_is_insert_mode_returns_correct_state() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());

        // Assert initial state
        assert!(!state.is_insert_mode());

        // Act
        state.enter_insert_mode();

        // Assert after mode change
        assert!(state.is_insert_mode());
    }

    #[test]
    fn test_is_normal_mode_returns_correct_state() {
        // Arrange
        let mut state = AppState::new(vec![], create_test_config());

        // Assert initial state
        assert!(state.is_normal_mode());

        // Act
        state.enter_insert_mode();

        // Assert after mode change
        assert!(!state.is_normal_mode());
    }
}
