//! Application state management for the Claude Code Tips browser.
//!
//! This module defines the core application state and behavior for the TUI,
//! including search filtering, navigation, and view mode management.

use crate::model::Tip;
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent};

/// Main application state for the tips browser TUI.
///
/// The `AppState` struct maintains all runtime state including the loaded tips,
/// current search query, filtered results, selection state, and UI mode flags.
/// State mutations are explicit and go through named methods to ensure
/// predictable behavior and easier debugging.
///
/// # State Management Pattern
///
/// - UI rendering takes `&AppState` (immutable reference)
/// - Input handling takes `&mut AppState` (mutable reference)
/// - All state changes happen through explicit methods
///
/// # Examples
///
/// ```
/// use claude_tips::model::Tip;
/// use claude_tips::app::AppState;
///
/// let tips = vec![
///     Tip {
///         id: "cc-001".to_string(),
///         title: "Test Tip".to_string(),
///         category: Some("testing".to_string()),
///         text: "Example tip text.".to_string(),
///         tags: vec!["test".to_string()],
///     },
/// ];
///
/// let mut state = AppState::new(tips);
/// assert_eq!(state.filtered_count(), 1);
/// assert_eq!(state.selected_index(), 0);
/// ```
#[derive(Debug)]
pub struct AppState {
    /// All loaded tips from YAML file.
    tips: Vec<Tip>,

    /// Indices into `tips` that match the current search query.
    /// When search is empty, contains all indices [0..tips.len()).
    filtered_tips: Vec<usize>,

    /// Currently selected index within the `filtered_tips` list.
    /// Must always be < filtered_tips.len() when filtered_tips is non-empty.
    selected_index: usize,

    /// Current search query string.
    /// Empty string means no active search (all tips visible).
    search_query: String,

    /// Whether search mode is currently active.
    /// When true, keyboard input appends to search_query.
    search_mode: bool,

    /// Whether detail view is currently shown.
    /// When true, displays full tip content instead of just list.
    detail_mode: bool,

    /// Application exit flag.
    /// When true, the main event loop should terminate.
    should_quit: bool,
}

impl AppState {
    /// Creates a new application state with the given tips.
    ///
    /// Initializes with all tips visible (no search filter applied),
    /// first tip selected, and all mode flags set to false.
    ///
    /// # Arguments
    ///
    /// * `tips` - Vector of tips to display in the browser
    ///
    /// # Examples
    ///
    /// ```
    /// use claude_tips::app::AppState;
    /// use claude_tips::model::Tip;
    ///
    /// let tips = vec![
    ///     Tip {
    ///         id: "cc-001".to_string(),
    ///         title: "Example".to_string(),
    ///         category: None,
    ///         text: "Example text".to_string(),
    ///         tags: vec![],
    ///     },
    /// ];
    ///
    /// let state = AppState::new(tips);
    /// assert!(!state.should_quit());
    /// ```
    pub fn new(tips: Vec<Tip>) -> Self {
        let filtered_tips: Vec<usize> = (0..tips.len()).collect();
        Self {
            tips,
            filtered_tips,
            selected_index: 0,
            search_query: String::new(),
            search_mode: false,
            detail_mode: false,
            should_quit: false,
        }
    }

    /// Updates the search query and re-filters the tips list.
    ///
    /// Performs case-insensitive substring matching across title, text,
    /// category, and tags fields. Resets selected_index to 0 after filtering.
    ///
    /// # Arguments
    ///
    /// * `query` - The search string to filter by
    ///
    /// # Examples
    ///
    /// ```
    /// use claude_tips::app::AppState;
    /// use claude_tips::model::Tip;
    ///
    /// let tips = vec![
    ///     Tip {
    ///         id: "cc-001".to_string(),
    ///         title: "Prompting Tips".to_string(),
    ///         category: None,
    ///         text: "Be clear and specific".to_string(),
    ///         tags: vec!["prompting".to_string()],
    ///     },
    ///     Tip {
    ///         id: "cc-002".to_string(),
    ///         title: "Debugging Workflow".to_string(),
    ///         category: None,
    ///         text: "Use systematic approach".to_string(),
    ///         tags: vec!["debugging".to_string()],
    ///     },
    /// ];
    ///
    /// let mut state = AppState::new(tips);
    /// state.update_search("prompting".to_string());
    /// assert_eq!(state.filtered_count(), 1);
    /// ```
    pub fn update_search(&mut self, query: String) {
        self.search_query = query.to_lowercase();

        if self.search_query.is_empty() {
            // No search query - show all tips
            self.filtered_tips = (0..self.tips.len()).collect();
        } else {
            // Filter tips by case-insensitive substring matching
            self.filtered_tips = self
                .tips
                .iter()
                .enumerate()
                .filter(|(_, tip)| {
                    let query = &self.search_query;
                    tip.title.to_lowercase().contains(query)
                        || tip.text.to_lowercase().contains(query)
                        || tip
                            .tags
                            .iter()
                            .any(|tag| tag.to_lowercase().contains(query))
                        || tip
                            .category
                            .as_ref()
                            .map(|c| c.to_lowercase().contains(query))
                            .unwrap_or(false)
                })
                .map(|(i, _)| i)
                .collect();
        }

        // Reset selection to first item
        self.selected_index = 0;
    }

    /// Moves selection to the next tip in the filtered list.
    ///
    /// Wraps around to the first tip when at the end of the list.
    /// Does nothing if the filtered list is empty.
    pub fn select_next(&mut self) {
        if !self.filtered_tips.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.filtered_tips.len();
        }
    }

    /// Moves selection to the previous tip in the filtered list.
    ///
    /// Wraps around to the last tip when at the beginning of the list.
    /// Does nothing if the filtered list is empty.
    pub fn select_prev(&mut self) {
        if !self.filtered_tips.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.filtered_tips.len() - 1;
            } else {
                self.selected_index -= 1;
            }
        }
    }

    /// Toggles search mode on/off.
    ///
    /// When entering search mode, clears any existing search query.
    /// When exiting search mode, keeps the current filter applied.
    pub fn toggle_search(&mut self) {
        self.search_mode = !self.search_mode;
        if self.search_mode {
            // Clear search when entering search mode
            self.search_query.clear();
            self.update_search(String::new());
        }
    }

    /// Toggles detail view mode on/off.
    ///
    /// Detail mode shows the full content of the selected tip
    /// instead of just the tips list.
    pub fn toggle_detail(&mut self) {
        self.detail_mode = !self.detail_mode;
    }

    /// Sets the quit flag to true, signaling the application should exit.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Returns whether the application should quit.
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Returns the currently selected tip, if any.
    ///
    /// Returns `None` if the filtered tips list is empty.
    pub fn selected_tip(&self) -> Option<&Tip> {
        if self.filtered_tips.is_empty() {
            None
        } else {
            let tip_index = self.filtered_tips[self.selected_index];
            self.tips.get(tip_index)
        }
    }

    /// Returns a slice of all tips (unfiltered).
    pub fn all_tips(&self) -> &[Tip] {
        &self.tips
    }

    /// Returns the filtered tips as a vector of references.
    pub fn filtered_tips(&self) -> Vec<&Tip> {
        self.filtered_tips
            .iter()
            .filter_map(|&idx| self.tips.get(idx))
            .collect()
    }

    /// Returns the number of tips matching the current filter.
    pub fn filtered_count(&self) -> usize {
        self.filtered_tips.len()
    }

    /// Returns the currently selected index within the filtered list.
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Returns the current search query string.
    pub fn search_query(&self) -> &str {
        &self.search_query
    }

    /// Returns whether search mode is active.
    pub fn is_search_mode(&self) -> bool {
        self.search_mode
    }

    /// Returns whether detail view mode is active.
    pub fn is_detail_mode(&self) -> bool {
        self.detail_mode
    }
}

/// Handles keyboard input events and updates application state accordingly.
///
/// Processes key events based on the current mode (normal, search, or detail)
/// and updates the state appropriately. Returns an error if an unexpected
/// event occurs that cannot be handled.
///
/// # Arguments
///
/// * `state` - Mutable reference to the application state
/// * `key` - The keyboard event to process
///
/// # Returns
///
/// * `Ok(())` if the event was handled successfully
/// * `Err` if an error occurred during event processing
///
/// # Key Bindings
///
/// **Normal Mode:**
/// - `Down` or `j`: Move selection down
/// - `Up` or `k`: Move selection up
/// - `Enter`: Toggle detail view
/// - `/`: Activate search mode
/// - `Esc`: Quit application
///
/// **Search Mode:**
/// - Any character: Append to search query
/// - `Backspace`: Remove last character from query
/// - `Esc`: Exit search mode
///
/// **Detail Mode:**
/// - `Esc`: Exit detail view
///
/// # Examples
///
/// ```
/// use claude_tips::app::{AppState, handle_key_event};
/// use claude_tips::model::Tip;
/// use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
///
/// let tips = vec![
///     Tip {
///         id: "cc-001".to_string(),
///         title: "Test".to_string(),
///         category: None,
///         text: "Test tip".to_string(),
///         tags: vec![],
///     },
/// ];
///
/// let mut state = AppState::new(tips);
/// let key = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);
/// handle_key_event(&mut state, key).unwrap();
/// ```
pub fn handle_key_event(state: &mut AppState, key: KeyEvent) -> Result<()> {
    match (state.is_search_mode(), state.is_detail_mode(), key.code) {
        // Detail mode - only Esc closes it
        (_, true, KeyCode::Esc) => {
            state.toggle_detail();
        }

        // Search mode input handling
        (true, false, KeyCode::Char(c)) => {
            let mut query = state.search_query().to_string();
            query.push(c);
            state.update_search(query);
        }
        (true, false, KeyCode::Backspace) => {
            let mut query = state.search_query().to_string();
            query.pop();
            state.update_search(query);
        }
        (true, false, KeyCode::Esc) => {
            state.toggle_search();
        }

        // Normal mode navigation
        (false, false, KeyCode::Down) | (false, false, KeyCode::Char('j')) => {
            state.select_next();
        }
        (false, false, KeyCode::Up) | (false, false, KeyCode::Char('k')) => {
            state.select_prev();
        }
        (false, false, KeyCode::Enter) => {
            state.toggle_detail();
        }
        (false, false, KeyCode::Char('/')) => {
            state.toggle_search();
        }
        (false, false, KeyCode::Esc) => {
            state.quit();
        }

        // Ignore other keys
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sample_tips() -> Vec<Tip> {
        vec![
            Tip {
                id: "cc-001".to_string(),
                title: "Use Clear Prompts".to_string(),
                category: Some("prompting".to_string()),
                text: "Be specific and clear in your requests.".to_string(),
                tags: vec!["prompting".to_string(), "best-practices".to_string()],
            },
            Tip {
                id: "cc-002".to_string(),
                title: "Debugging Workflow".to_string(),
                category: Some("debugging".to_string()),
                text: "Use systematic debugging approach.".to_string(),
                tags: vec!["debugging".to_string(), "workflow".to_string()],
            },
            Tip {
                id: "cc-003".to_string(),
                title: "Keyboard Shortcuts".to_string(),
                category: Some("features".to_string()),
                text: "Learn keyboard shortcuts for efficiency.".to_string(),
                tags: vec!["shortcuts".to_string(), "productivity".to_string()],
            },
        ]
    }

    #[test]
    fn test_app_state_initialization() {
        let tips = create_sample_tips();
        let state = AppState::new(tips);

        assert_eq!(state.filtered_count(), 3);
        assert_eq!(state.selected_index(), 0);
        assert_eq!(state.search_query(), "");
        assert!(!state.is_search_mode());
        assert!(!state.is_detail_mode());
        assert!(!state.should_quit());
    }

    #[test]
    fn test_search_filters_by_title() {
        let tips = create_sample_tips();
        let mut state = AppState::new(tips);

        state.update_search("Debugging".to_string());
        assert_eq!(state.filtered_count(), 1);
        assert_eq!(state.selected_tip().unwrap().id, "cc-002");
    }

    #[test]
    fn test_search_filters_by_tags() {
        let tips = create_sample_tips();
        let mut state = AppState::new(tips);

        state.update_search("workflow".to_string());
        assert_eq!(state.filtered_count(), 1);
        assert_eq!(state.selected_tip().unwrap().id, "cc-002");
    }

    #[test]
    fn test_search_filters_by_text() {
        let tips = create_sample_tips();
        let mut state = AppState::new(tips);

        state.update_search("systematic".to_string());
        assert_eq!(state.filtered_count(), 1);
        assert_eq!(state.selected_tip().unwrap().id, "cc-002");
    }

    #[test]
    fn test_search_is_case_insensitive() {
        let tips = create_sample_tips();
        let mut state1 = AppState::new(tips.clone());
        let mut state2 = AppState::new(tips);

        state1.update_search("DEBUGGING".to_string());
        state2.update_search("debugging".to_string());

        assert_eq!(state1.filtered_count(), state2.filtered_count());
        assert_eq!(
            state1.selected_tip().unwrap().id,
            state2.selected_tip().unwrap().id
        );
    }

    #[test]
    fn test_search_no_results_empty() {
        let tips = create_sample_tips();
        let mut state = AppState::new(tips);

        state.update_search("nonexistentterm".to_string());
        assert_eq!(state.filtered_count(), 0);
        assert!(state.selected_tip().is_none());
    }

    #[test]
    fn test_navigation_wraps_correctly() {
        let tips = create_sample_tips();
        let mut state = AppState::new(tips);

        // At start, select_prev wraps to end
        assert_eq!(state.selected_index(), 0);
        state.select_prev();
        assert_eq!(state.selected_index(), 2);

        // At end, select_next wraps to start
        state.select_next();
        assert_eq!(state.selected_index(), 0);
    }

    #[test]
    fn test_mode_toggles() {
        let tips = create_sample_tips();
        let mut state = AppState::new(tips);

        // Toggle search mode
        assert!(!state.is_search_mode());
        state.toggle_search();
        assert!(state.is_search_mode());
        state.toggle_search();
        assert!(!state.is_search_mode());

        // Toggle detail mode
        assert!(!state.is_detail_mode());
        state.toggle_detail();
        assert!(state.is_detail_mode());
        state.toggle_detail();
        assert!(!state.is_detail_mode());

        // Quit flag
        assert!(!state.should_quit());
        state.quit();
        assert!(state.should_quit());
    }
}
