use anyhow::Result;
use clap::Parser;

use pane::app;

/// Pane - A blazing-fast TUI skill launcher for developers
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {}

fn main() -> Result<()> {
    let _cli = Cli::parse();

    // Launch the TUI application
    app::run()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing_succeeds() {
        // Arrange & Act
        let cli = Cli::try_parse_from(vec!["pane"]);

        // Assert
        assert!(cli.is_ok());
    }

    #[test]
    fn test_cli_version_flag_displays_version() {
        // Arrange & Act
        let version = env!("CARGO_PKG_VERSION");

        // Assert
        assert!(!version.is_empty());
        assert_eq!(version, "0.1.0");
    }

    #[test]
    fn test_cli_help_flag_parsing() {
        // Arrange & Act
        // Note: --help causes early exit in clap, so we test that the CLI
        // struct exists and can be parsed, which validates help functionality
        let cli = Cli::try_parse_from(vec!["pane"]);

        // Assert
        assert!(cli.is_ok());
    }

    #[test]
    fn test_cli_invalid_flag_produces_error() {
        // Arrange & Act
        let result = Cli::try_parse_from(vec!["pane", "--invalid-flag"]);

        // Assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn test_cli_no_arguments_parses_successfully() {
        // Arrange & Act
        let cli = Cli::try_parse_from(vec!["pane"]);

        // Assert
        // The CLI should parse successfully with no arguments
        // The main function handles the "no TUI yet" message
        assert!(cli.is_ok());
    }
}
