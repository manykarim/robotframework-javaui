//! Error types for the robotframework-swing library

use std::fmt;
use thiserror::Error;

/// Main error type for Swing automation operations
#[derive(Error, Debug)]
pub enum SwingError {
    // Connection Errors
    #[error("No JVM found matching identifier '{identifier}'")]
    JvmNotFound { identifier: String },

    #[error("Failed to attach to JVM (PID: {pid}): {reason}")]
    AttachFailed { pid: u32, reason: String },

    #[error("Agent injection failed: {reason}")]
    AgentInjectionFailed { reason: String },

    #[error("Connection lost to application")]
    ConnectionLost,

    #[error("Connection timeout after {timeout_ms}ms")]
    ConnectionTimeout { timeout_ms: u64 },

    #[error("Not connected to any application")]
    NotConnected,

    // Element Errors
    #[error("Element not found: {locator}")]
    ElementNotFound {
        locator: String,
        #[source]
        context: Option<Box<ElementNotFoundContext>>,
    },

    #[error("Multiple elements found for locator '{locator}' (found {count})")]
    MultipleElementsFound { locator: String, count: usize },

    #[error("Element '{element_id}' is not interactable: {reason}")]
    ElementNotInteractable { element_id: String, reason: String },

    #[error("Element '{element_id}' is stale (no longer in UI tree)")]
    StaleElement { element_id: String },

    // Locator Errors
    #[error("Invalid locator syntax: {message}")]
    InvalidLocator {
        locator: String,
        message: String,
        position: Option<usize>,
    },

    // Action Errors
    #[error("Action '{action}' failed on element '{element_id}': {reason}")]
    ActionFailed {
        action: String,
        element_id: String,
        reason: String,
    },

    #[error("Timeout waiting for condition: {condition}")]
    WaitTimeout { condition: String, timeout_ms: u64 },

    // Protocol Errors
    #[error("RPC error (code {code}): {message}")]
    RpcError { code: i32, message: String },

    #[error("Protocol error: {message}")]
    ProtocolError { message: String },

    #[error("Serialization error: {message}")]
    SerializationError { message: String },

    // Internal Errors
    #[error("Internal error: {message}")]
    Internal { message: String },

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Context information for element not found errors
#[derive(Debug)]
pub struct ElementNotFoundContext {
    /// Simplified tree structure showing what was searched
    pub searched_tree: String,
    /// Similar elements that might be what the user wanted
    pub similar_elements: Vec<SimilarElement>,
    /// Suggestions for fixing the locator
    pub suggestions: Vec<String>,
}

impl std::error::Error for ElementNotFoundContext {}

impl fmt::Display for ElementNotFoundContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\nSearched tree (summary):")?;
        writeln!(f, "{}", self.searched_tree)?;

        if !self.similar_elements.is_empty() {
            writeln!(f, "\nSimilar elements found:")?;
            for elem in &self.similar_elements {
                writeln!(
                    f,
                    "  - {} ({}) - similarity: {:.0}%",
                    elem.locator,
                    elem.class_name,
                    elem.similarity_score * 100.0
                )?;
            }
        }

        if !self.suggestions.is_empty() {
            writeln!(f, "\nSuggestions:")?;
            for suggestion in &self.suggestions {
                writeln!(f, "  - {}", suggestion)?;
            }
        }

        Ok(())
    }
}

/// Information about a similar element for error diagnostics
#[derive(Debug, Clone)]
pub struct SimilarElement {
    /// Suggested locator for this element
    pub locator: String,
    /// Java class name
    pub class_name: String,
    /// Component name if set
    pub name: Option<String>,
    /// Text content if available
    pub text: Option<String>,
    /// Similarity score (0.0 to 1.0)
    pub similarity_score: f32,
}

/// Result type alias for Swing operations
pub type SwingResult<T> = Result<T, SwingError>;

impl SwingError {
    /// Create an ElementNotFound error with diagnostic context
    pub fn element_not_found_with_context(
        locator: &str,
        searched_tree: String,
        similar_elements: Vec<SimilarElement>,
    ) -> Self {
        let suggestions = Self::generate_suggestions(locator, &similar_elements);

        SwingError::ElementNotFound {
            locator: locator.to_string(),
            context: Some(Box::new(ElementNotFoundContext {
                searched_tree,
                similar_elements,
                suggestions,
            })),
        }
    }

    /// Generate helpful suggestions based on what was found
    fn generate_suggestions(locator: &str, similar: &[SimilarElement]) -> Vec<String> {
        let mut suggestions = Vec::new();

        if similar.is_empty() {
            suggestions.push(
                "No similar elements found. Use 'Log UI Tree' keyword to inspect available elements."
                    .to_string(),
            );
            suggestions.push("Verify the application window is visible and fully loaded.".to_string());
        } else {
            for elem in similar.iter().take(3) {
                suggestions.push(format!("Did you mean: {}", elem.locator));
            }
        }

        // Check for common mistakes
        if locator.contains("Button") && !locator.contains("JButton") {
            suggestions.push("Note: Java Swing uses 'JButton', not 'Button'.".to_string());
        }

        if locator.contains("TextField") && !locator.contains("JTextField") {
            suggestions.push("Note: Java Swing uses 'JTextField', not 'TextField'.".to_string());
        }

        suggestions
    }

    /// Check if this error is recoverable (can be retried)
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            SwingError::ConnectionTimeout { .. }
                | SwingError::WaitTimeout { .. }
                | SwingError::ElementNotFound { .. }
        )
    }

    /// Check if this error indicates a connection problem
    pub fn is_connection_error(&self) -> bool {
        matches!(
            self,
            SwingError::JvmNotFound { .. }
                | SwingError::AttachFailed { .. }
                | SwingError::AgentInjectionFailed { .. }
                | SwingError::ConnectionLost
                | SwingError::ConnectionTimeout { .. }
                | SwingError::NotConnected
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = SwingError::ElementNotFound {
            locator: "button[name='submit']".to_string(),
            context: None,
        };
        assert!(err.to_string().contains("button[name='submit']"));
    }

    #[test]
    fn test_is_recoverable() {
        let timeout = SwingError::WaitTimeout {
            condition: "element visible".to_string(),
            timeout_ms: 5000,
        };
        assert!(timeout.is_recoverable());

        let internal = SwingError::Internal {
            message: "fatal".to_string(),
        };
        assert!(!internal.is_recoverable());
    }

    #[test]
    fn test_is_connection_error() {
        let not_connected = SwingError::NotConnected;
        assert!(not_connected.is_connection_error());

        let not_found = SwingError::ElementNotFound {
            locator: "test".to_string(),
            context: None,
        };
        assert!(!not_found.is_connection_error());
    }
}
