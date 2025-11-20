use std::time::Duration;

use anyhow::{Context, Result};

use crate::{
    context::SkillContext, discover_skills, input::poll_event, load_config,
    skills::manifest::UiMode, skills::runner, state::AppState, terminal::TerminalGuard, ui::render,
    InputEvent,
};

/// Run the main TUI application
///
/// This is the main entry point for the TUI. It:
/// 1. Loads user configuration
/// 2. Discovers skills from all configured locations
/// 3. Initializes the terminal and application state
/// 4. Runs the event loop
/// 5. Cleans up the terminal on exit
///
/// # Returns
///
/// Ok(()) on successful exit, or an error if initialization or the event loop fails.
///
/// # Errors
///
/// Returns an error if:
/// - Configuration loading fails
/// - Skill discovery fails
/// - Terminal initialization fails
/// - Terminal rendering fails
pub fn run() -> Result<()> {
    tracing::info!("Starting Pane TUI application");

    // Load user configuration
    let config = load_config().context("Failed to load configuration")?;
    tracing::debug!("Configuration loaded: {:?}", config);

    // Discover all available skills
    let skills = discover_skills().context("Failed to discover skills")?;
    tracing::info!("Discovered {} skills", skills.len());

    // Initialize application state
    let mut state = AppState::new(skills, config);

    // Initialize terminal (RAII guard handles cleanup)
    let mut term_guard = TerminalGuard::new().context("Failed to initialize terminal")?;
    let terminal = term_guard.terminal();

    // Main event loop
    loop {
        // Render current state
        terminal
            .draw(|frame| render(frame, &state))
            .context("Failed to render UI")?;

        // Poll for input events (250ms timeout for responsive rendering)
        // Pass current input mode for mode-aware key mapping
        if let Some(event) = poll_event(Duration::from_millis(250), state.input_mode())? {
            handle_event(event, &mut state);
        }

        // Check exit condition
        if state.should_quit() {
            break;
        }
    }

    tracing::info!("Pane TUI application exiting");
    Ok(())
}

/// Handle an input event and update application state
///
/// Routes events based on application context:
/// - If output panel is visible: scroll output or close panel (Esc)
/// - Otherwise: normal skill list navigation and search
///
/// # Arguments
///
/// * `event` - The input event to handle
/// * `state` - The application state to update
fn handle_event(event: InputEvent, state: &mut AppState) {
    // Output panel is visible - handle output panel navigation
    if state.is_output_panel_visible() {
        match event {
            InputEvent::Quit => {
                // Esc closes output panel
                state.hide_output_panel();
            }
            InputEvent::MoveUp | InputEvent::CharInput('k') => {
                // Scroll output up
                state.scroll_output_up();
            }
            InputEvent::MoveDown | InputEvent::CharInput('j') => {
                // Scroll output down (max_lines calculated internally from content)
                state.scroll_output_down();
            }
            // Other keys ignored when output panel is visible
            _ => {}
        }
        return;
    }

    // Normal skill list navigation
    match event {
        InputEvent::Quit => {
            // Double-Esc behavior: Clear search first, then quit
            if !state.search_query().is_empty() {
                state.set_search_query(String::new());
                state.apply_view_filter();
                tracing::debug!("Cleared search query");
            } else {
                state.quit();
                tracing::debug!("Quitting application");
            }
        }
        InputEvent::MoveUp => state.move_selection_up(),
        InputEvent::MoveDown => state.move_selection_down(),
        InputEvent::CharInput(c) => {
            state.append_to_search(c);
        }
        InputEvent::Backspace => {
            state.remove_from_search();
        }
        InputEvent::PageDown => {
            // TODO: Calculate page size dynamically based on terminal height
            // page_size = terminal_height - header(3) - footer(3) - borders(2)
            const DEFAULT_PAGE_SIZE: usize = 10;
            state.move_selection_page_down(DEFAULT_PAGE_SIZE);
        }
        InputEvent::PageUp => {
            const DEFAULT_PAGE_SIZE: usize = 10;
            state.move_selection_page_up(DEFAULT_PAGE_SIZE);
        }
        InputEvent::Enter => {
            // Execute the selected skill
            if let Some(selected_skill) = state.selected_skill() {
                // Clone data we need before execution to avoid borrow issues
                let skill_id = selected_skill.manifest.id.clone();
                let skill_name = selected_skill.manifest.name.clone();
                let ui_mode = selected_skill.manifest.ui.mode.clone();

                // Build context for skill execution
                match SkillContext::build(selected_skill, state.config()) {
                    Ok(context) => {
                        // Route execution based on UI mode
                        match ui_mode {
                            UiMode::Inline => {
                                // Inline execution - capture output and display in panel
                                tracing::info!("Executing inline skill: {}", skill_name);

                                match runner::execute_inline(selected_skill, &context) {
                                    Ok(output) => {
                                        // Update recent skills list
                                        state.add_to_recent(skill_id);

                                        // Log exit status if debug logging enabled
                                        if let Some(code) = output.exit_code {
                                            tracing::debug!(
                                                "Inline skill '{}' exited with code {}",
                                                skill_name,
                                                code
                                            );
                                        }

                                        // Show output panel automatically
                                        state.show_output_panel(output);
                                    }
                                    Err(e) => {
                                        // Log error and display in output panel
                                        tracing::error!(
                                            "Failed to execute inline skill '{}': {:?}",
                                            skill_name,
                                            e
                                        );

                                        // Create error output for display
                                        let error_output = crate::skills::output::SkillOutput {
                                            stdout: String::new(),
                                            stderr: format!("Failed to execute skill: {:#}", e),
                                            exit_code: None,
                                            truncated: false,
                                            execution_time: std::time::Duration::from_secs(0),
                                        };
                                        state.show_output_panel(error_output);
                                    }
                                }
                            }
                            UiMode::Tui => {
                                // TUI execution - use existing terminal handoff flow
                                tracing::info!("Executing TUI skill: {}", skill_name);

                                match runner::execute_skill(selected_skill, context) {
                                    Ok(status) => {
                                        // Update recent skills list
                                        state.add_to_recent(skill_id);

                                        // Log exit status if debug logging enabled
                                        if let Some(code) = status.code() {
                                            tracing::debug!(
                                                "TUI skill '{}' exited with code {}",
                                                skill_name,
                                                code
                                            );
                                        }
                                    }
                                    Err(e) => {
                                        // Log error
                                        tracing::error!(
                                            "Failed to execute TUI skill '{}': {:?}",
                                            skill_name,
                                            e
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!(
                            "Failed to build context for skill '{}': {:?}",
                            skill_name,
                            e
                        );
                    }
                }
            }
        }
        InputEvent::Tab => {
            state.cycle_view_mode();
            state.apply_view_filter();
        }
        // Modal input system events (Story 6.1)
        InputEvent::EnterInsertMode => {
            state.enter_insert_mode();
            tracing::debug!("Entered Insert mode");
        }
        InputEvent::EnterNormalMode => {
            state.enter_normal_mode();
            tracing::debug!("Entered Normal mode");
        }
        InputEvent::ToggleFavorite => {
            // TODO: Implement favorite toggling in Task 6
            tracing::debug!("Toggle favorite (not yet implemented)");
        }
        InputEvent::ShowHelp => {
            // TODO: Implement help overlay in Story 6.3
            tracing::debug!("Show help (not yet implemented)");
        }
    }
}
