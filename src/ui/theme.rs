/// Theme configuration for TUI visual styling
///
/// Provides customizable colors and styles for all UI components.
/// Defaults work in both light and dark terminal themes.
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::BorderType;
use serde::{Deserialize, Serialize};

/// Theme configuration for TUI visual styling
///
/// Provides customizable colors and styles for all UI components.
/// Defaults are designed to work well in both light and dark terminal themes.
///
/// # Examples
///
/// ```
/// use pane::ui::theme::ThemeConfig;
///
/// // Use default theme
/// let theme = ThemeConfig::default();
///
/// // Access theme colors
/// let primary_color = theme.primary;
/// let header = theme.header_style;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThemeConfig {
    /// Primary accent color (title, highlights)
    #[serde(with = "color_serde")]
    pub primary: Color,

    /// Secondary accent color
    #[serde(with = "color_serde")]
    pub secondary: Color,

    /// Selected item background color
    #[serde(with = "color_serde")]
    pub highlight: Color,

    /// Border color for all blocks
    #[serde(with = "color_serde")]
    pub border: Color,

    /// Primary text color
    #[serde(with = "color_serde")]
    pub text: Color,

    /// Dimmed/secondary text color
    #[serde(with = "color_serde")]
    pub text_dim: Color,

    /// Tag background color
    #[serde(with = "color_serde")]
    pub tag_bg: Color,

    /// Tag foreground/text color
    #[serde(with = "color_serde")]
    pub tag_fg: Color,

    /// Border type for all blocks
    #[serde(with = "border_type_serde")]
    pub border_style: BorderType,
}

impl Default for ThemeConfig {
    /// Returns terminal-friendly defaults that work in both light and dark modes
    ///
    /// # Example
    ///
    /// ```
    /// use pane::ui::theme::ThemeConfig;
    ///
    /// let theme = ThemeConfig::default();
    /// assert_eq!(theme.primary, ratatui::style::Color::Cyan);
    /// ```
    fn default() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            highlight: Color::DarkGray,
            border: Color::Gray,
            text: Color::White,
            text_dim: Color::DarkGray,
            tag_bg: Color::Blue,
            tag_fg: Color::White,
            border_style: BorderType::Rounded,
        }
    }
}

impl ThemeConfig {
    /// Returns the header style based on theme configuration
    ///
    /// # Example
    ///
    /// ```
    /// use pane::ui::theme::ThemeConfig;
    ///
    /// let theme = ThemeConfig::default();
    /// let header_style = theme.header_style();
    /// ```
    pub fn header_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }

    /// Returns the selected item style based on theme configuration
    ///
    /// # Example
    ///
    /// ```
    /// use pane::ui::theme::ThemeConfig;
    ///
    /// let theme = ThemeConfig::default();
    /// let selected_style = theme.selected_style();
    /// ```
    pub fn selected_style(&self) -> Style {
        Style::default().bg(self.highlight)
    }

    /// Returns the tag chip style based on theme configuration
    ///
    /// # Example
    ///
    /// ```
    /// use pane::ui::theme::ThemeConfig;
    ///
    /// let theme = ThemeConfig::default();
    /// let tag_style = theme.tag_style();
    /// ```
    pub fn tag_style(&self) -> Style {
        Style::default().bg(self.tag_bg).fg(self.tag_fg)
    }

    /// Returns the time estimate style based on theme configuration
    ///
    /// # Example
    ///
    /// ```
    /// use pane::ui::theme::ThemeConfig;
    ///
    /// let theme = ThemeConfig::default();
    /// let time_style = theme.time_style();
    /// ```
    pub fn time_style(&self) -> Style {
        Style::default().fg(self.text_dim)
    }

    /// Returns the border style for blocks
    ///
    /// # Example
    ///
    /// ```
    /// use pane::ui::theme::ThemeConfig;
    ///
    /// let theme = ThemeConfig::default();
    /// let border = theme.border_style();
    /// ```
    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }
}

// Custom serde implementations for ratatui types
mod color_serde {
    use ratatui::style::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(color: &Color, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match color {
            Color::Black => "Black".serialize(serializer),
            Color::Red => "Red".serialize(serializer),
            Color::Green => "Green".serialize(serializer),
            Color::Yellow => "Yellow".serialize(serializer),
            Color::Blue => "Blue".serialize(serializer),
            Color::Magenta => "Magenta".serialize(serializer),
            Color::Cyan => "Cyan".serialize(serializer),
            Color::Gray => "Gray".serialize(serializer),
            Color::DarkGray => "DarkGray".serialize(serializer),
            Color::LightRed => "LightRed".serialize(serializer),
            Color::LightGreen => "LightGreen".serialize(serializer),
            Color::LightYellow => "LightYellow".serialize(serializer),
            Color::LightBlue => "LightBlue".serialize(serializer),
            Color::LightMagenta => "LightMagenta".serialize(serializer),
            Color::LightCyan => "LightCyan".serialize(serializer),
            Color::White => "White".serialize(serializer),
            _ => "White".serialize(serializer), // Default fallback
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Black" => Ok(Color::Black),
            "Red" => Ok(Color::Red),
            "Green" => Ok(Color::Green),
            "Yellow" => Ok(Color::Yellow),
            "Blue" => Ok(Color::Blue),
            "Magenta" => Ok(Color::Magenta),
            "Cyan" => Ok(Color::Cyan),
            "Gray" => Ok(Color::Gray),
            "DarkGray" => Ok(Color::DarkGray),
            "LightRed" => Ok(Color::LightRed),
            "LightGreen" => Ok(Color::LightGreen),
            "LightYellow" => Ok(Color::LightYellow),
            "LightBlue" => Ok(Color::LightBlue),
            "LightMagenta" => Ok(Color::LightMagenta),
            "LightCyan" => Ok(Color::LightCyan),
            "White" => Ok(Color::White),
            _ => Ok(Color::White), // Default fallback
        }
    }
}

mod border_type_serde {
    use ratatui::widgets::BorderType;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(border_type: &BorderType, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match border_type {
            BorderType::Plain => "Plain".serialize(serializer),
            BorderType::Rounded => "Rounded".serialize(serializer),
            BorderType::Double => "Double".serialize(serializer),
            BorderType::Thick => "Thick".serialize(serializer),
            _ => "Rounded".serialize(serializer), // Default fallback
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<BorderType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "Plain" => Ok(BorderType::Plain),
            "Rounded" => Ok(BorderType::Rounded),
            "Double" => Ok(BorderType::Double),
            "Thick" => Ok(BorderType::Thick),
            _ => Ok(BorderType::Rounded), // Default fallback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_config_default_values() {
        // Arrange & Act
        let theme = ThemeConfig::default();

        // Assert
        assert_eq!(theme.primary, Color::Cyan);
        assert_eq!(theme.secondary, Color::Blue);
        assert_eq!(theme.highlight, Color::DarkGray);
        assert_eq!(theme.border, Color::Gray);
        assert_eq!(theme.text, Color::White);
        assert_eq!(theme.text_dim, Color::DarkGray);
        assert_eq!(theme.tag_bg, Color::Blue);
        assert_eq!(theme.tag_fg, Color::White);
        assert_eq!(theme.border_style, BorderType::Rounded);
    }

    #[test]
    fn test_theme_config_serialization() {
        // Arrange
        let theme = ThemeConfig::default();

        // Act
        let serialized = toml::to_string(&theme);

        // Assert
        assert!(serialized.is_ok());
        let toml_str = serialized.unwrap();
        assert!(toml_str.contains("primary"));
        assert!(toml_str.contains("Cyan"));
    }

    #[test]
    fn test_theme_config_deserialization() {
        // Arrange
        let toml_str = r#"
            primary = "Cyan"
            secondary = "Blue"
            highlight = "DarkGray"
            border = "Gray"
            text = "White"
            text_dim = "DarkGray"
            tag_bg = "Blue"
            tag_fg = "White"
            border_style = "Rounded"
        "#;

        // Act
        let theme: Result<ThemeConfig, _> = toml::from_str(toml_str);

        // Assert
        assert!(theme.is_ok());
        let theme = theme.unwrap();
        assert_eq!(theme.primary, Color::Cyan);
        assert_eq!(theme.border_style, BorderType::Rounded);
    }

    #[test]
    fn test_header_style_returns_correct_style() {
        // Arrange
        let theme = ThemeConfig::default();

        // Act
        let style = theme.header_style();

        // Assert
        assert_eq!(style.fg, Some(Color::Cyan));
        assert!(style.add_modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_selected_style_returns_correct_style() {
        // Arrange
        let theme = ThemeConfig::default();

        // Act
        let style = theme.selected_style();

        // Assert
        assert_eq!(style.bg, Some(Color::DarkGray));
    }

    #[test]
    fn test_tag_style_returns_correct_style() {
        // Arrange
        let theme = ThemeConfig::default();

        // Act
        let style = theme.tag_style();

        // Assert
        assert_eq!(style.bg, Some(Color::Blue));
        assert_eq!(style.fg, Some(Color::White));
    }

    #[test]
    fn test_time_style_returns_correct_style() {
        // Arrange
        let theme = ThemeConfig::default();

        // Act
        let style = theme.time_style();

        // Assert
        assert_eq!(style.fg, Some(Color::DarkGray));
    }

    #[test]
    fn test_border_style_returns_correct_style() {
        // Arrange
        let theme = ThemeConfig::default();

        // Act
        let style = theme.border_style();

        // Assert
        assert_eq!(style.fg, Some(Color::Gray));
    }
}
