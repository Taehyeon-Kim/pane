use std::time::Duration;

/// Maximum allowed output size (10MB)
///
/// If a skill's combined stdout + stderr exceeds this limit,
/// the output will be truncated with a warning message.
pub const MAX_OUTPUT_SIZE: usize = 10 * 1024 * 1024; // 10MB

/// Output captured from a skill execution
///
/// Contains the stdout, stderr, exit code, execution time, and truncation status
/// from running a skill in inline mode. This struct is returned after the skill
/// process completes and all output has been captured.
///
/// # Examples
///
/// ```no_run
/// use pane::skills::output::SkillOutput;
/// use std::time::Duration;
///
/// let output = SkillOutput {
///     stdout: "Hello, world!".to_string(),
///     stderr: String::new(),
///     exit_code: Some(0),
///     truncated: false,
///     execution_time: Duration::from_millis(42),
/// };
///
/// assert!(output.exit_code == Some(0));
/// assert!(!output.truncated);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct SkillOutput {
    /// Captured stdout from the skill process
    pub stdout: String,
    /// Captured stderr from the skill process
    pub stderr: String,
    /// Exit code from the skill process (None if process was killed/interrupted)
    pub exit_code: Option<i32>,
    /// Whether output was truncated due to exceeding size limit
    pub truncated: bool,
    /// Time taken to execute the skill
    pub execution_time: Duration,
}

/// Buffer for capturing output with size limit enforcement
///
/// Accumulates bytes from a process output stream up to a maximum size limit.
/// Once the limit is reached, further data is discarded and the truncated flag
/// is set. This prevents unbounded memory usage from skills with large output.
///
/// # Examples
///
/// ```
/// use pane::skills::output::OutputBuffer;
///
/// let mut buffer = OutputBuffer::new();
/// assert!(buffer.is_empty());
///
/// buffer.append(b"Hello ").unwrap();
/// buffer.append(b"world!").unwrap();
///
/// assert_eq!(buffer.to_string(), "Hello world!");
/// assert!(!buffer.is_truncated());
/// ```
#[derive(Debug, Clone)]
pub struct OutputBuffer {
    /// Underlying byte buffer
    buffer: Vec<u8>,
    /// Whether the buffer has been truncated due to size limit
    truncated: bool,
    /// Maximum allowed size in bytes
    size_limit: usize,
}

impl OutputBuffer {
    /// Create a new output buffer with the default size limit
    ///
    /// The default limit is 10MB. Use `with_limit()` to specify a custom limit.
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::skills::output::OutputBuffer;
    ///
    /// let buffer = OutputBuffer::new();
    /// assert!(buffer.is_empty());
    /// ```
    pub fn new() -> Self {
        Self::with_limit(MAX_OUTPUT_SIZE)
    }

    /// Create a new output buffer with a specific size limit
    ///
    /// # Arguments
    ///
    /// * `size_limit` - Maximum number of bytes to store
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::skills::output::OutputBuffer;
    ///
    /// let buffer = OutputBuffer::with_limit(1024); // 1KB limit
    /// ```
    pub fn with_limit(size_limit: usize) -> Self {
        Self {
            buffer: Vec::new(),
            truncated: false,
            size_limit,
        }
    }

    /// Append data to the buffer, enforcing size limit
    ///
    /// Adds the provided data to the buffer. If adding the data would exceed
    /// the size limit, only the bytes that fit are added, and the truncated
    /// flag is set.
    ///
    /// # Arguments
    ///
    /// * `data` - Bytes to append to the buffer
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::skills::output::OutputBuffer;
    ///
    /// let mut buffer = OutputBuffer::with_limit(10);
    /// buffer.append(b"Hello");
    /// buffer.append(b" World!"); // Exceeds limit
    /// assert!(buffer.is_truncated());
    /// ```
    pub fn append(&mut self, data: &[u8]) {
        // Check if we're already at capacity
        if self.buffer.len() >= self.size_limit {
            self.truncated = true;
            return;
        }

        // Calculate how much space is left
        let remaining = self.size_limit - self.buffer.len();

        if data.len() <= remaining {
            // All data fits
            self.buffer.extend_from_slice(data);
        } else {
            // Partial data fits - take only what we can
            self.buffer.extend_from_slice(&data[..remaining]);
            self.truncated = true;
        }
    }

    /// Convert the buffer contents to a UTF-8 string
    ///
    /// Invalid UTF-8 sequences are replaced with the Unicode replacement character.
    ///
    /// # Returns
    ///
    /// The buffer contents as a String
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::skills::output::OutputBuffer;
    ///
    /// let mut buffer = OutputBuffer::new();
    /// buffer.append(b"Hello, world!").unwrap();
    /// assert_eq!(buffer.to_string(), "Hello, world!");
    /// ```
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.buffer).into_owned()
    }

    /// Check if output was truncated
    ///
    /// # Returns
    ///
    /// true if the buffer exceeded the size limit
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::skills::output::OutputBuffer;
    ///
    /// let mut buffer = OutputBuffer::with_limit(5);
    /// buffer.append(b"Hello world").unwrap();
    /// assert!(buffer.is_truncated());
    /// ```
    pub fn is_truncated(&self) -> bool {
        self.truncated
    }

    /// Check if the buffer is empty
    ///
    /// # Returns
    ///
    /// true if no data has been appended
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::skills::output::OutputBuffer;
    ///
    /// let buffer = OutputBuffer::new();
    /// assert!(buffer.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    /// Get the current size of the buffer in bytes
    ///
    /// # Returns
    ///
    /// Number of bytes currently stored
    ///
    /// # Examples
    ///
    /// ```
    /// use pane::skills::output::OutputBuffer;
    ///
    /// let mut buffer = OutputBuffer::new();
    /// buffer.append(b"Hello").unwrap();
    /// assert_eq!(buffer.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

impl Default for OutputBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_buffer_new_initializes_empty() {
        // Arrange & Act
        let buffer = OutputBuffer::new();

        // Assert
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert!(!buffer.is_truncated());
    }

    #[test]
    fn test_output_buffer_append_within_limit() {
        // Arrange
        let mut buffer = OutputBuffer::new();
        let data = b"Hello, world!";

        // Act
        buffer.append(data);

        // Assert
        assert_eq!(buffer.len(), data.len());
        assert!(!buffer.is_truncated());
        assert_eq!(buffer.to_string(), "Hello, world!");
    }

    #[test]
    fn test_output_buffer_append_exceeds_limit_sets_truncated() {
        // Arrange
        let mut buffer = OutputBuffer::with_limit(10);
        let data = b"Hello, world! This is too long.";

        // Act
        buffer.append(data);

        // Assert
        assert_eq!(buffer.len(), 10); // Only first 10 bytes
        assert!(buffer.is_truncated());
        assert_eq!(buffer.to_string(), "Hello, wor");
    }

    #[test]
    fn test_output_buffer_append_multiple_within_limit() {
        // Arrange
        let mut buffer = OutputBuffer::new();

        // Act
        buffer.append(b"Hello ");
        buffer.append(b"world");
        buffer.append(b"!");

        // Assert
        assert_eq!(buffer.len(), 12);
        assert!(!buffer.is_truncated());
        assert_eq!(buffer.to_string(), "Hello world!");
    }

    #[test]
    fn test_output_buffer_append_multiple_exceeds_limit() {
        // Arrange
        let mut buffer = OutputBuffer::with_limit(10);

        // Act
        buffer.append(b"Hello"); // 5 bytes
        assert!(!buffer.is_truncated());

        buffer.append(b" world"); // +6 bytes = 11 total (exceeds limit)

        // Assert
        assert_eq!(buffer.len(), 10);
        assert!(buffer.is_truncated());
        assert_eq!(buffer.to_string(), "Hello worl");
    }

    #[test]
    fn test_output_buffer_append_after_truncated_is_noop() {
        // Arrange
        let mut buffer = OutputBuffer::with_limit(5);
        buffer.append(b"Hello");
        buffer.append(b" world"); // Truncates
        assert!(buffer.is_truncated());

        // Act - try to append more after truncation
        buffer.append(b" more data");

        // Assert - should remain at limit
        assert_eq!(buffer.len(), 5);
        assert_eq!(buffer.to_string(), "Hello");
    }

    #[test]
    fn test_output_buffer_to_string_converts_correctly() {
        // Arrange
        let mut buffer = OutputBuffer::new();
        buffer.append(b"Test output\n");
        buffer.append(b"Line 2\n");

        // Act
        let result = buffer.to_string();

        // Assert
        assert_eq!(result, "Test output\nLine 2\n");
    }

    #[test]
    fn test_output_buffer_to_string_handles_invalid_utf8() {
        // Arrange
        let mut buffer = OutputBuffer::new();
        // Invalid UTF-8 sequence
        buffer.append(&[0xFF, 0xFE, 0xFD]);

        // Act
        let result = buffer.to_string();

        // Assert - should contain replacement characters, not panic
        assert!(!result.is_empty());
        assert!(result.contains('�')); // Unicode replacement character
    }

    #[test]
    fn test_skill_output_struct_creation() {
        // Arrange & Act
        let output = SkillOutput {
            stdout: "Output text".to_string(),
            stderr: "Error text".to_string(),
            exit_code: Some(0),
            truncated: false,
            execution_time: Duration::from_millis(123),
        };

        // Assert
        assert_eq!(output.stdout, "Output text");
        assert_eq!(output.stderr, "Error text");
        assert_eq!(output.exit_code, Some(0));
        assert!(!output.truncated);
        assert_eq!(output.execution_time, Duration::from_millis(123));
    }

    #[test]
    fn test_skill_output_with_truncation() {
        // Arrange & Act
        let output = SkillOutput {
            stdout: "Truncated output".to_string(),
            stderr: String::new(),
            exit_code: Some(0),
            truncated: true,
            execution_time: Duration::from_secs(1),
        };

        // Assert
        assert!(output.truncated);
    }

    #[test]
    fn test_skill_output_with_none_exit_code() {
        // Arrange & Act - process was killed/interrupted
        let output = SkillOutput {
            stdout: String::new(),
            stderr: "Process killed".to_string(),
            exit_code: None,
            truncated: false,
            execution_time: Duration::from_millis(50),
        };

        // Assert
        assert!(output.exit_code.is_none());
    }
}
