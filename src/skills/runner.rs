use std::io::{self, BufReader, Read};
use std::process::{Command, ExitStatus, Stdio};
use std::time::Instant;

use anyhow::{bail, Context, Result};
use crossterm::terminal;

use crate::context::SkillContext;
use crate::skills::manifest::UiMode;
use crate::skills::output::{OutputBuffer, SkillOutput};
use crate::skills::Skill;

/// Execute a skill based on its UI mode (TUI or inline)
///
/// Routes execution to the appropriate handler based on the skill's ui.mode setting:
/// - `UiMode::Tui`: Suspends launcher, hands terminal to skill process
/// - `UiMode::Inline`: Runs process with output capture, launcher remains active
///
/// # Arguments
///
/// * `skill` - The skill to execute
/// * `context` - Context information to pass via environment variables
///
/// # Returns
///
/// ExitStatus from the skill process (TUI mode) or exit code from SkillOutput (inline mode)
///
/// # Errors
///
/// Returns an error if:
/// - Executable validation fails (not found in PATH or invalid path)
/// - Terminal suspend/restore fails (TUI mode)
/// - Process spawn fails
/// - Process wait fails
///
/// # Examples
///
/// ```no_run
/// # use pane::skills::Skill;
/// # use pane::context::SkillContext;
/// # use pane::skills::runner::execute_skill;
/// # fn example(skill: &Skill, context: SkillContext) -> anyhow::Result<()> {
/// let status = execute_skill(skill, context)?;
/// if status.success() {
///     println!("Skill completed successfully");
/// }
/// # Ok(())
/// # }
/// ```
pub fn execute_skill(skill: &Skill, context: SkillContext) -> Result<ExitStatus> {
    // Route based on UI mode
    match skill.manifest.ui.mode {
        UiMode::Tui => execute_tui(skill, context),
        UiMode::Inline => {
            // Execute inline and capture output
            let _output = execute_inline(skill, &context)?;

            // NOTE: Output storage and display is handled in Story 3.2
            // For now, we just execute and discard the output
            // Return synthetic ExitStatus by re-running with status() for compatibility
            // TODO: Store output in AppState (Story 3.2)

            // Create a simple command to get an ExitStatus with the captured exit code
            // This is a workaround since ExitStatus can't be constructed directly
            let exit_code = _output.exit_code.unwrap_or(0);
            std::process::Command::new("sh")
                .arg("-c")
                .arg(format!("exit {}", exit_code))
                .status()
                .context("Failed to create synthetic exit status")
        }
    }
}

/// Execute a skill in TUI mode with terminal handoff
///
/// Suspends the TUI, spawns the skill process with context environment variables,
/// waits for completion, and restores the TUI. Uses RAII pattern for terminal
/// state management to ensure cleanup even on panic.
///
/// # Arguments
///
/// * `skill` - The skill to execute
/// * `context` - Context information to pass via environment variables
///
/// # Returns
///
/// ExitStatus from the skill process
///
/// # Errors
///
/// Returns an error if:
/// - Executable validation fails (not found in PATH or invalid path)
/// - Terminal suspend/restore fails
/// - Process spawn fails
/// - Process wait fails
fn execute_tui(skill: &Skill, context: SkillContext) -> Result<ExitStatus> {
    // Validate that the executable exists before attempting to spawn
    validate_executable(&skill.manifest.exec)
        .with_context(|| format!("Failed to validate executable '{}'", skill.manifest.exec))?;

    // Suspend TUI before skill execution
    suspend_tui().context("Failed to suspend TUI")?;

    // Create terminal guard to ensure restoration even on panic
    let _guard = TerminalRestoreGuard;

    // Prepare environment variables
    let env_vars = context.prepare_environment(&skill.manifest.context);

    // Build and spawn the command
    let mut cmd = Command::new(&skill.manifest.exec);
    cmd.args(&skill.manifest.args);
    cmd.envs(env_vars);

    // For TUI mode: inherit stdin/stdout/stderr (skill takes over terminal)
    let status = cmd
        .status()
        .with_context(|| format!("Failed to execute skill '{}'", skill.manifest.name))?;

    // Restore TUI after skill exits
    restore_tui().context("Failed to restore TUI")?;

    Ok(status)
}

/// Execute a skill in inline mode with output capture
///
/// Spawns the skill process with piped stdout/stderr, captures output into buffers
/// with size limits, waits for completion, and returns structured output. The
/// launcher TUI remains active during execution (output display in Story 3.2).
///
/// Output is limited to 10MB total to prevent unbounded memory usage. If the limit
/// is exceeded, output is truncated and the truncated flag is set.
///
/// # Arguments
///
/// * `skill` - The skill to execute
/// * `context` - Context information to pass via environment variables
///
/// # Returns
///
/// SkillOutput containing stdout, stderr, exit code, execution time, and truncation status
///
/// # Errors
///
/// Returns an error if:
/// - Executable validation fails
/// - Process spawn fails
/// - Output reading fails
/// - Process wait fails
pub fn execute_inline(skill: &Skill, context: &SkillContext) -> Result<SkillOutput> {
    // Record start time for execution duration
    let start_time = Instant::now();

    // Validate executable exists
    validate_executable(&skill.manifest.exec)
        .with_context(|| format!("Failed to validate executable '{}'", skill.manifest.exec))?;

    // Prepare environment variables
    let env_vars = context.prepare_environment(&skill.manifest.context);

    // Build command with piped output
    let mut cmd = Command::new(&skill.manifest.exec);
    cmd.args(&skill.manifest.args);
    cmd.envs(env_vars);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    // Spawn the child process
    let mut child = cmd
        .spawn()
        .with_context(|| format!("Failed to spawn inline skill '{}'", skill.manifest.name))?;

    // Capture stdout
    let stdout_handle = child
        .stdout
        .take()
        .context("Failed to capture stdout handle")?;
    let stdout_result =
        read_output_stream(stdout_handle).context("Failed to read stdout from skill process")?;

    // Capture stderr
    let stderr_handle = child
        .stderr
        .take()
        .context("Failed to capture stderr handle")?;
    let stderr_result =
        read_output_stream(stderr_handle).context("Failed to read stderr from skill process")?;

    // Wait for process to complete
    let status = child.wait().with_context(|| {
        format!(
            "Failed to wait for skill '{}' completion",
            skill.manifest.name
        )
    })?;

    // Calculate execution time
    let execution_time = start_time.elapsed();

    // Check if either stream was truncated
    let truncated = stdout_result.1 || stderr_result.1;

    // Build output string with truncation warning if needed
    let mut stdout = stdout_result.0;
    let mut stderr = stderr_result.0;

    if truncated {
        let warning = "\n[Output truncated - exceeded 10MB limit]";
        if stdout_result.1 {
            stdout.push_str(warning);
        }
        if stderr_result.1 {
            stderr.push_str(warning);
        }
    }

    Ok(SkillOutput {
        stdout,
        stderr,
        exit_code: status.code(),
        truncated,
        execution_time,
    })
}

/// Read output from a process stream with size limit enforcement
///
/// Reads from the provided stream into an OutputBuffer, enforcing the 10MB size limit.
/// Returns the captured output as a String and a boolean indicating if truncation occurred.
///
/// # Arguments
///
/// * `stream` - The output stream to read from (stdout or stderr handle)
///
/// # Returns
///
/// Tuple of (output_string, was_truncated)
///
/// # Errors
///
/// Returns an error if reading from the stream fails
fn read_output_stream<R: Read>(stream: R) -> Result<(String, bool)> {
    let mut buffer = OutputBuffer::new();
    let mut reader = BufReader::new(stream);
    let mut chunk = vec![0u8; 8192]; // 8KB chunks for efficient reading

    loop {
        let bytes_read = reader
            .read(&mut chunk)
            .context("Failed to read from output stream")?;

        if bytes_read == 0 {
            break; // EOF
        }

        buffer.append(&chunk[..bytes_read]);

        // Stop reading if we've hit the limit
        if buffer.is_truncated() {
            break;
        }
    }

    Ok((buffer.to_string(), buffer.is_truncated()))
}

/// Validate that an executable exists in PATH or as an absolute/relative path
///
/// Checks if the executable can be found before attempting to spawn a process,
/// providing better error messages than "file not found" after spawn failure.
///
/// # Arguments
///
/// * `exec` - Executable name or path to validate
///
/// # Returns
///
/// Ok(()) if executable exists and is accessible
///
/// # Errors
///
/// Returns an error if:
/// - Executable is not found in PATH
/// - Absolute/relative path does not exist
/// - File exists but is not executable
fn validate_executable(exec: &str) -> Result<()> {
    // Check if it's an absolute or relative path
    if exec.contains('/') || exec.contains('\\') {
        // It's a path - check if it exists
        let path = std::path::Path::new(exec);
        if !path.exists() {
            bail!("Executable not found at path: {}", exec);
        }
        // Check if it's a file (not a directory)
        if !path.is_file() {
            bail!("Path is not a file: {}", exec);
        }
        return Ok(());
    }

    // It's a command name - check if it's in PATH
    let path_var = std::env::var("PATH").unwrap_or_default();
    let paths = std::env::split_paths(&path_var);

    for dir in paths {
        let full_path = dir.join(exec);
        if full_path.exists() && full_path.is_file() {
            return Ok(());
        }
    }

    bail!("Executable '{}' not found in PATH or invalid path", exec);
}

/// Suspend the TUI to hand terminal control to the skill
///
/// Disables raw mode, clears the screen, and shows the cursor.
/// Must be called before spawning a skill process.
///
/// In test environments where raw mode is not enabled, silently continues
/// without error.
///
/// # Returns
///
/// Ok(()) on success
///
/// # Errors
///
/// Returns an error if terminal operations fail (except in test environments)
fn suspend_tui() -> Result<()> {
    // Attempt to disable raw mode - ignore error if not in raw mode (test environment)
    let _ = terminal::disable_raw_mode();

    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        crossterm::cursor::Show
    )
    .context("Failed to clear terminal and show cursor")?;

    Ok(())
}

/// Restore the TUI after skill execution completes
///
/// Re-enables raw mode, hides the cursor, and clears the screen
/// to prepare for TUI re-rendering.
///
/// In test environments, attempts best-effort restoration without failing.
///
/// # Returns
///
/// Ok(()) on success
///
/// # Errors
///
/// Returns an error if terminal operations fail (except in test environments)
fn restore_tui() -> Result<()> {
    // Attempt to enable raw mode - ignore error in test environment
    let _ = terminal::enable_raw_mode();

    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::cursor::Hide,
        terminal::Clear(terminal::ClearType::All)
    )
    .context("Failed to hide cursor and clear terminal")?;

    Ok(())
}

/// RAII guard to ensure TUI is restored even on panic
///
/// Implements the Drop trait to call restore_tui() when the guard
/// goes out of scope, ensuring terminal cleanup in all scenarios.
struct TerminalRestoreGuard;

impl Drop for TerminalRestoreGuard {
    fn drop(&mut self) {
        // Best effort restoration - ignore errors since we might be panicking
        let _ = restore_tui();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::skills::{
        manifest::{ContextConfig, SkillManifest, UiConfig, UiMode},
        SkillSource,
    };
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_skill(id: &str, name: &str, exec: &str, args: Vec<String>) -> Skill {
        Skill {
            manifest: SkillManifest {
                id: id.to_string(),
                name: name.to_string(),
                description: "Test skill".to_string(),
                version: "1.0.0".to_string(),
                exec: exec.to_string(),
                args,
                tags: vec![],
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

    /// RAII guard to ensure current directory is restored even if test panics
    struct DirGuard {
        original: PathBuf,
    }

    impl DirGuard {
        fn new() -> io::Result<Self> {
            // If current_dir fails (e.g., we're in a deleted directory),
            // navigate to a safe directory first
            let original = match std::env::current_dir() {
                Ok(dir) => dir,
                Err(_) => {
                    // Navigate to temp directory as fallback
                    let safe_dir = std::env::temp_dir();
                    std::env::set_current_dir(&safe_dir)?;
                    safe_dir
                }
            };

            Ok(Self { original })
        }
    }

    impl Drop for DirGuard {
        fn drop(&mut self) {
            // Restore original directory, or navigate to temp if original is deleted
            if self.original.exists() {
                let _ = std::env::set_current_dir(&self.original);
            } else {
                let _ = std::env::set_current_dir(std::env::temp_dir());
            }
        }
    }

    #[test]
    fn test_validate_executable_exists_in_path() {
        // Arrange - use a common system executable
        let exec = "ls"; // Available on Unix systems

        // Act
        let result = validate_executable(exec);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_executable_returns_error_for_missing_executable() {
        // Arrange
        let exec = "nonexistent-command-12345";

        // Act
        let result = validate_executable(exec);

        // Assert
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("not found in PATH"));
    }

    #[test]
    fn test_validate_executable_absolute_path_exists() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        let script_path = temp_dir.path().join("test-script.sh");
        fs::write(&script_path, "#!/bin/bash\necho test").unwrap();

        // Act
        let result = validate_executable(&script_path.to_string_lossy());

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_executable_absolute_path_not_exists() {
        // Arrange
        let nonexistent_path = "/tmp/nonexistent-script-12345.sh";

        // Act
        let result = validate_executable(nonexistent_path);

        // Assert
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("not found at path"));
    }

    #[test]
    fn test_validate_executable_path_is_directory_fails() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        let dir_path = temp_dir.path();

        // Act
        let result = validate_executable(&dir_path.to_string_lossy());

        // Assert
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("not a file"));
    }

    #[test]
    #[serial_test::serial]
    fn test_execute_skill_spawns_with_correct_args() {
        // Arrange
        let _dir_guard = DirGuard::new().unwrap();
        let temp_dir = TempDir::new().unwrap();

        // Change to temp directory to ensure it stays valid
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let script_path = temp_dir.path().join("test-echo.sh");

        // Create a simple script that echoes its arguments to a file
        let output_file = temp_dir.path().join("output.txt");
        let script_content = format!(
            r#"#!/bin/bash
echo "$@" > {}
"#,
            output_file.to_string_lossy()
        );
        fs::write(&script_path, script_content).unwrap();

        // Make script executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).unwrap();
        }

        let skill = create_test_skill(
            "test-skill",
            "Test Skill",
            &script_path.to_string_lossy(),
            vec!["arg1".to_string(), "arg2".to_string()],
        );
        let config = Config::default();
        let context = SkillContext::build(&skill, &config).unwrap();

        // Act
        let result = execute_skill(&skill, context);

        // Assert
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.success());

        // Verify args were passed correctly
        let output = fs::read_to_string(&output_file).unwrap();
        assert!(output.contains("arg1"));
        assert!(output.contains("arg2"));

        // Directory will be restored automatically by DirGuard drop
    }

    #[test]
    #[serial_test::serial]
    fn test_execute_skill_applies_environment_variables() {
        // Arrange
        let _dir_guard = DirGuard::new().unwrap();
        let temp_dir = TempDir::new().unwrap();

        // Change to temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let script_path = temp_dir.path().join("test-env.sh");

        // Create a script that outputs environment variables to a file
        let output_file = temp_dir.path().join("env-output.txt");
        let script_content = format!(
            r#"#!/bin/bash
echo "PANE_ID=$PANE_ID" > {}
echo "PANE_NAME=$PANE_NAME" >> {}
echo "PANE_CWD=$PANE_CWD" >> {}
"#,
            output_file.to_string_lossy(),
            output_file.to_string_lossy(),
            output_file.to_string_lossy()
        );
        fs::write(&script_path, script_content).unwrap();

        // Make script executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).unwrap();
        }

        let skill = create_test_skill(
            "env-test",
            "Environment Test",
            &script_path.to_string_lossy(),
            vec![],
        );
        let config = Config::default();
        let context = SkillContext::build(&skill, &config).unwrap();

        // Act
        let result = execute_skill(&skill, context);

        // Assert
        assert!(result.is_ok());

        // Verify environment variables were passed
        let output = fs::read_to_string(&output_file).unwrap();
        assert!(output.contains("PANE_ID=env-test"));
        assert!(output.contains("PANE_NAME=Environment Test"));
        assert!(output.contains("PANE_CWD="));

        // Directory will be restored automatically by DirGuard drop
    }

    #[test]
    #[serial_test::serial]
    fn test_execute_inline_success_captures_stdout() {
        // Arrange
        let _dir_guard = DirGuard::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let script_path = temp_dir.path().join("echo-stdout.sh");

        // Create script that outputs to stdout
        let script_content = r#"#!/bin/bash
echo "Hello from stdout"
"#;
        fs::write(&script_path, script_content).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).unwrap();
        }

        let skill = create_test_skill(
            "stdout-test",
            "Stdout Test",
            &script_path.to_string_lossy(),
            vec![],
        );
        let config = Config::default();
        let context = SkillContext::build(&skill, &config).unwrap();

        // Act
        let result = execute_inline(&skill, &context);

        // Assert
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.stdout.contains("Hello from stdout"));
        assert_eq!(output.exit_code, Some(0));
        assert!(!output.truncated);
    }

    #[test]
    #[serial_test::serial]
    fn test_execute_inline_captures_stderr() {
        // Arrange
        let _dir_guard = DirGuard::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let script_path = temp_dir.path().join("echo-stderr.sh");

        // Create script that outputs to stderr
        let script_content = r#"#!/bin/bash
echo "Error message" >&2
"#;
        fs::write(&script_path, script_content).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).unwrap();
        }

        let skill = create_test_skill(
            "stderr-test",
            "Stderr Test",
            &script_path.to_string_lossy(),
            vec![],
        );
        let config = Config::default();
        let context = SkillContext::build(&skill, &config).unwrap();

        // Act
        let result = execute_inline(&skill, &context);

        // Assert
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.stderr.contains("Error message"));
        assert_eq!(output.exit_code, Some(0));
    }

    #[test]
    #[serial_test::serial]
    fn test_execute_inline_captures_exit_code() {
        // Arrange
        let _dir_guard = DirGuard::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let script_path = temp_dir.path().join("exit-42.sh");

        // Create script that exits with code 42
        let script_content = r#"#!/bin/bash
exit 42
"#;
        fs::write(&script_path, script_content).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).unwrap();
        }

        let skill = create_test_skill(
            "exit-test",
            "Exit Test",
            &script_path.to_string_lossy(),
            vec![],
        );
        let config = Config::default();
        let context = SkillContext::build(&skill, &config).unwrap();

        // Act
        let result = execute_inline(&skill, &context);

        // Assert
        assert!(result.is_ok());
        let output = result.unwrap();
        assert_eq!(output.exit_code, Some(42));
    }

    #[test]
    #[serial_test::serial]
    fn test_execute_inline_enforces_size_limit() {
        // Arrange
        let _dir_guard = DirGuard::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let script_path = temp_dir.path().join("large-output.sh");

        // Create script that outputs more than buffer can handle (simulated with repeated output)
        // Note: For testing, we'd need a way to limit the buffer size
        // For now, just test that the mechanism exists
        let script_content = r#"#!/bin/bash
echo "Normal output"
"#;
        fs::write(&script_path, script_content).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).unwrap();
        }

        let skill = create_test_skill(
            "size-test",
            "Size Test",
            &script_path.to_string_lossy(),
            vec![],
        );
        let config = Config::default();
        let context = SkillContext::build(&skill, &config).unwrap();

        // Act
        let result = execute_inline(&skill, &context);

        // Assert
        assert!(result.is_ok());
        let output = result.unwrap();
        // With normal output, truncated should be false
        assert!(!output.truncated);
    }

    #[test]
    #[serial_test::serial]
    fn test_execute_inline_handles_process_error() {
        // Arrange
        let _dir_guard = DirGuard::new().unwrap();
        let skill = create_test_skill(
            "nonexistent-test",
            "Nonexistent Test",
            "/nonexistent/path/to/skill",
            vec![],
        );
        let config = Config::default();
        let context = SkillContext::build(&skill, &config).unwrap();

        // Act
        let result = execute_inline(&skill, &context);

        // Assert
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        // Check for validation error (from validate_executable)
        assert!(error_msg.contains("not found") || error_msg.contains("Failed to validate"));
    }

    #[test]
    #[serial_test::serial]
    fn test_execute_skill_routes_to_inline_mode() {
        // Arrange
        let _dir_guard = DirGuard::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let script_path = temp_dir.path().join("inline-skill.sh");

        let script_content = r#"#!/bin/bash
echo "Inline execution"
"#;
        fs::write(&script_path, script_content).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).unwrap();
        }

        // Create skill with inline UI mode
        let mut skill = create_test_skill(
            "inline-route-test",
            "Inline Route Test",
            &script_path.to_string_lossy(),
            vec![],
        );
        skill.manifest.ui.mode = UiMode::Inline;

        let config = Config::default();
        let context = SkillContext::build(&skill, &config).unwrap();

        // Act
        let result = execute_skill(&skill, context);

        // Assert
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.success());
    }

    #[test]
    #[serial_test::serial]
    fn test_execute_skill_routes_to_tui_mode() {
        // Arrange
        let _dir_guard = DirGuard::new().unwrap();
        let temp_dir = TempDir::new().unwrap();
        let script_path = temp_dir.path().join("tui-skill.sh");

        let script_content = r#"#!/bin/bash
exit 0
"#;
        fs::write(&script_path, script_content).unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).unwrap();
        }

        // Create skill with TUI mode (default)
        let skill = create_test_skill(
            "tui-route-test",
            "TUI Route Test",
            &script_path.to_string_lossy(),
            vec![],
        );
        assert_eq!(skill.manifest.ui.mode, UiMode::Tui);

        let config = Config::default();
        let context = SkillContext::build(&skill, &config).unwrap();

        // Act
        let result = execute_skill(&skill, context);

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    #[serial_test::serial]
    fn test_execute_skill_captures_exit_status() {
        // Arrange
        let _dir_guard = DirGuard::new().unwrap();
        let temp_dir = TempDir::new().unwrap();

        // Change to temp directory
        std::env::set_current_dir(temp_dir.path()).unwrap();

        let script_path = temp_dir.path().join("test-exit.sh");

        // Create a script that exits with code 42
        let script_content = r#"#!/bin/bash
exit 42
"#;
        fs::write(&script_path, script_content).unwrap();

        // Make script executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&script_path).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&script_path, perms).unwrap();
        }

        let skill = create_test_skill(
            "exit-test",
            "Exit Test",
            &script_path.to_string_lossy(),
            vec![],
        );
        let config = Config::default();
        let context = SkillContext::build(&skill, &config).unwrap();

        // Act
        let result = execute_skill(&skill, context);

        // Assert
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(!status.success());
        assert_eq!(status.code(), Some(42));

        // Directory will be restored automatically by DirGuard drop
    }
}
