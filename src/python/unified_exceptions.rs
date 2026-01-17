//! Unified exception hierarchy for robotframework-javagui
//!
//! This module implements a technology-agnostic exception hierarchy with rich error context,
//! following ADR-005 (Error Handling Strategy).
//!
//! # Exception Hierarchy
//!
//! ```text
//! JavaGuiError (base)
//! +-- ConnectionError
//! |   +-- ConnectionRefusedError
//! |   +-- ConnectionTimeoutError
//! |   +-- NotConnectedError
//! +-- ElementError
//! |   +-- ElementNotFoundError
//! |   +-- MultipleElementsFoundError
//! |   +-- ElementNotInteractableError
//! |   +-- StaleElementError
//! +-- LocatorError
//! |   +-- LocatorParseError
//! |   +-- InvalidLocatorSyntaxError
//! +-- ActionError
//! |   +-- ActionFailedError
//! |   +-- ActionTimeoutError
//! |   +-- ActionNotSupportedError
//! +-- TechnologyError
//! |   +-- ModeNotSupportedError
//! |   +-- RcpWorkbenchError
//! |   +-- SwtShellError
//! +-- InternalError
//! ```
//!
//! # Usage
//!
//! ```rust,ignore
//! use crate::python::unified_exceptions::*;
//!
//! // Create an error with rich context
//! let err = ErrorBuilder::element_not_found("button[name='submit']")
//!     .with_context("mode", "swing")
//!     .with_context("window", "Main Window")
//!     .with_suggestion("Use 'Log UI Tree' keyword to inspect available elements")
//!     .with_suggestion("Verify the application window is visible")
//!     .build();
//! ```

use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::collections::HashMap;

// ============================================================
// Base Exception
// ============================================================

pyo3::create_exception!(
    javagui,
    JavaGuiError,
    PyException,
    "Base exception for all Java GUI library errors.\n\nAll library exceptions inherit from this class, allowing you to catch all library errors with a single except clause."
);

// ============================================================
// Connection Exceptions
// ============================================================

pyo3::create_exception!(
    javagui,
    ConnectionError,
    JavaGuiError,
    "Base exception for connection-related errors.\n\nRaised when there are problems establishing or maintaining a connection to the Java application."
);

pyo3::create_exception!(
    javagui,
    ConnectionRefusedError,
    ConnectionError,
    "Connection to the Java application was refused.\n\nThis typically means the Java agent is not loaded or the port is incorrect."
);

pyo3::create_exception!(
    javagui,
    ConnectionTimeoutError,
    ConnectionError,
    "Connection attempt timed out.\n\nThe Java application did not respond within the configured timeout period."
);

pyo3::create_exception!(
    javagui,
    NotConnectedError,
    ConnectionError,
    "Operation requires an active connection.\n\nYou must connect to an application using 'Connect To Application' before performing this operation."
);

// ============================================================
// Element Exceptions
// ============================================================

pyo3::create_exception!(
    javagui,
    ElementError,
    JavaGuiError,
    "Base exception for element-related errors.\n\nRaised when there are problems finding or interacting with UI elements."
);

pyo3::create_exception!(
    javagui,
    ElementNotFoundError,
    ElementError,
    "No element found matching the given locator.\n\nThe locator did not match any visible element in the current UI tree."
);

pyo3::create_exception!(
    javagui,
    MultipleElementsFoundError,
    ElementError,
    "Multiple elements found when only one was expected.\n\nThe locator matched more than one element. Use a more specific locator or index."
);

pyo3::create_exception!(
    javagui,
    ElementNotInteractableError,
    ElementError,
    "Element exists but cannot be interacted with.\n\nThe element may be disabled, hidden, or covered by another element."
);

pyo3::create_exception!(
    javagui,
    StaleElementError,
    ElementError,
    "Element reference is stale (no longer in UI).\n\nThe element was found previously but is no longer present in the UI tree."
);

// ============================================================
// Locator Exceptions
// ============================================================

pyo3::create_exception!(
    javagui,
    LocatorError,
    JavaGuiError,
    "Base exception for locator-related errors.\n\nRaised when there are problems parsing or validating locator expressions."
);

pyo3::create_exception!(
    javagui,
    LocatorParseError,
    LocatorError,
    "Failed to parse the locator expression.\n\nThe locator syntax is invalid or malformed."
);

pyo3::create_exception!(
    javagui,
    InvalidLocatorSyntaxError,
    LocatorError,
    "Locator syntax is invalid.\n\nThe locator uses an unsupported format or contains syntax errors."
);

// ============================================================
// Action Exceptions
// ============================================================

pyo3::create_exception!(
    javagui,
    ActionError,
    JavaGuiError,
    "Base exception for action-related errors.\n\nRaised when an action on an element fails to complete."
);

pyo3::create_exception!(
    javagui,
    ActionFailedError,
    ActionError,
    "Failed to perform the requested action.\n\nThe action could not be completed on the target element."
);

pyo3::create_exception!(
    javagui,
    ActionTimeoutError,
    ActionError,
    "Action timed out waiting for condition.\n\nThe expected condition was not met within the timeout period."
);

pyo3::create_exception!(
    javagui,
    ActionNotSupportedError,
    ActionError,
    "Action is not supported for this element type.\n\nThe requested action is not available for this type of UI component."
);

// ============================================================
// Technology-Specific Exceptions
// ============================================================

pyo3::create_exception!(
    javagui,
    TechnologyError,
    JavaGuiError,
    "Base exception for technology-specific errors.\n\nRaised when there are problems specific to Swing, SWT, or RCP."
);

pyo3::create_exception!(
    javagui,
    ModeNotSupportedError,
    TechnologyError,
    "Operation not supported in current mode.\n\nThe requested keyword is only available in certain modes (Swing, SWT, or RCP)."
);

pyo3::create_exception!(
    javagui,
    RcpWorkbenchError,
    TechnologyError,
    "Eclipse RCP workbench operation failed.\n\nFailed to interact with the Eclipse RCP workbench, perspective, view, or editor."
);

pyo3::create_exception!(
    javagui,
    SwtShellError,
    TechnologyError,
    "SWT shell operation failed.\n\nFailed to interact with an SWT shell, dialog, or window."
);

// ============================================================
// Internal Exceptions
// ============================================================

pyo3::create_exception!(
    javagui,
    InternalError,
    JavaGuiError,
    "Internal library error.\n\nAn unexpected error occurred within the library. Please report this issue."
);

// ============================================================
// Error Types Enumeration
// ============================================================

/// Enumeration of all error types for the ErrorBuilder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorType {
    // Connection errors
    Connection,
    ConnectionRefused,
    ConnectionTimeout,
    NotConnected,

    // Element errors
    Element,
    ElementNotFound,
    MultipleElementsFound,
    ElementNotInteractable,
    StaleElement,

    // Locator errors
    Locator,
    LocatorParse,
    InvalidLocatorSyntax,

    // Action errors
    Action,
    ActionFailed,
    ActionTimeout,
    ActionNotSupported,

    // Technology errors
    Technology,
    ModeNotSupported,
    RcpWorkbench,
    SwtShell,

    // Internal
    Internal,
}

// ============================================================
// GUI Mode Enumeration
// ============================================================

/// GUI mode enumeration for error context
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiMode {
    Swing,
    Swt,
    Rcp,
    Auto,
}

impl std::fmt::Display for GuiMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuiMode::Swing => write!(f, "Swing"),
            GuiMode::Swt => write!(f, "SWT"),
            GuiMode::Rcp => write!(f, "RCP"),
            GuiMode::Auto => write!(f, "Auto"),
        }
    }
}

// ============================================================
// Similar Element (for error diagnostics)
// ============================================================

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

// ============================================================
// Error Builder
// ============================================================

/// Error builder for creating detailed error messages with rich context.
///
/// The builder pattern allows you to construct error messages with:
/// - Clear description of the problem
/// - Context information (mode, window, element state, etc.)
/// - Troubleshooting suggestions
/// - Similar elements that might be what the user wanted
///
/// # Example
///
/// ```rust,ignore
/// let err = ErrorBuilder::element_not_found("button[name='ok']")
///     .with_context("mode", "swing")
///     .with_context("window", "Settings Dialog")
///     .with_suggestion("Check if the button is named 'OK' (uppercase)")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct ErrorBuilder {
    error_type: ErrorType,
    message: String,
    context: HashMap<String, String>,
    suggestions: Vec<String>,
    related_errors: Vec<String>,
}

impl ErrorBuilder {
    /// Create a new ErrorBuilder with the given type and message
    pub fn new(error_type: ErrorType, message: impl Into<String>) -> Self {
        Self {
            error_type,
            message: message.into(),
            context: HashMap::new(),
            suggestions: Vec::new(),
            related_errors: Vec::new(),
        }
    }

    // --------------------------------------------------------
    // Connection Error Builders
    // --------------------------------------------------------

    /// Create a ConnectionRefused error builder
    pub fn connection_refused(host: &str, port: u16) -> Self {
        Self::new(
            ErrorType::ConnectionRefused,
            format!("Connection refused to {}:{}", host, port),
        )
        .with_suggestion("Verify the Java application is running")
        .with_suggestion("Check that the Java agent is loaded (java -javaagent:javagui-agent.jar ...)")
        .with_suggestion("Verify the port number matches the agent configuration")
        .with_suggestion("Check firewall settings")
    }

    /// Create a ConnectionTimeout error builder
    pub fn connection_timeout(timeout_ms: u64) -> Self {
        Self::new(
            ErrorType::ConnectionTimeout,
            format!("Connection timeout after {}ms", timeout_ms),
        )
        .with_suggestion("Increase the connection timeout")
        .with_suggestion("Check if the application is responding")
        .with_suggestion("Verify network connectivity")
    }

    /// Create a NotConnected error builder
    pub fn not_connected() -> Self {
        Self::new(
            ErrorType::NotConnected,
            "Not connected to any application".to_string(),
        )
        .with_suggestion("Use 'Connect To Application' keyword first")
        .with_suggestion("Check if a previous connection was lost")
    }

    // --------------------------------------------------------
    // Element Error Builders
    // --------------------------------------------------------

    /// Create an ElementNotFound error builder
    pub fn element_not_found(locator: &str) -> Self {
        Self::new(
            ErrorType::ElementNotFound,
            format!("Element not found: '{}'", locator),
        )
        .with_suggestion("Use 'Log UI Tree' keyword to inspect available elements")
        .with_suggestion("Verify the application window is visible and fully loaded")
        .with_suggestion("Wait for the element to appear using 'Wait Until Element Exists'")
    }

    /// Create a MultipleElementsFound error builder
    pub fn multiple_elements_found(locator: &str, count: usize) -> Self {
        Self::new(
            ErrorType::MultipleElementsFound,
            format!(
                "Expected single element, found {} matching: '{}'",
                count, locator
            ),
        )
        .with_suggestion("Use a more specific locator (add more attributes)")
        .with_suggestion("Use index syntax: locator[0], locator[1], etc.")
        .with_suggestion("Use 'Get Elements' keyword to get all matching elements")
    }

    /// Create an ElementNotInteractable error builder
    pub fn element_not_interactable(element_id: &str, reason: &str) -> Self {
        Self::new(
            ErrorType::ElementNotInteractable,
            format!("Element '{}' is not interactable: {}", element_id, reason),
        )
        .with_suggestion("Wait for the element to become enabled")
        .with_suggestion("Check if the element is hidden or covered")
        .with_suggestion("Verify the element supports this action")
    }

    /// Create a StaleElement error builder
    pub fn stale_element(element_id: &str) -> Self {
        Self::new(
            ErrorType::StaleElement,
            format!(
                "Element '{}' is stale (no longer in UI tree)",
                element_id
            ),
        )
        .with_suggestion("Re-find the element using the original locator")
        .with_suggestion("The UI may have been refreshed or navigated")
    }

    // --------------------------------------------------------
    // Locator Error Builders
    // --------------------------------------------------------

    /// Create a LocatorParse error builder
    pub fn locator_parse(locator: &str, error: &str) -> Self {
        Self::new(
            ErrorType::LocatorParse,
            format!("Failed to parse locator '{}': {}", locator, error),
        )
    }

    /// Create a LocatorParse error builder with position information
    pub fn locator_parse_at_position(locator: &str, error: &str, position: usize) -> Self {
        let pointer = format!("{}\n{}^", locator, " ".repeat(position));
        Self::new(
            ErrorType::LocatorParse,
            format!("Failed to parse locator at position {}:\n{}\nError: {}", position, pointer, error),
        )
        .with_suggestion("Check the locator syntax")
        .with_suggestion("Valid formats: Button[name='x'], name:x, #x, //Button[@name='x']")
    }

    /// Create an InvalidLocatorSyntax error builder
    pub fn invalid_locator_syntax(locator: &str, reason: &str) -> Self {
        Self::new(
            ErrorType::InvalidLocatorSyntax,
            format!("Invalid locator syntax '{}': {}", locator, reason),
        )
        .with_suggestion("CSS-style: Button[name='x']")
        .with_suggestion("Prefix-style: name:x, text:x, class:x")
        .with_suggestion("ID shorthand: #element_id")
        .with_suggestion("XPath: //Button[@name='x']")
    }

    // --------------------------------------------------------
    // Action Error Builders
    // --------------------------------------------------------

    /// Create an ActionFailed error builder
    pub fn action_failed(action: &str, element: &str, reason: &str) -> Self {
        Self::new(
            ErrorType::ActionFailed,
            format!("Action '{}' failed on element '{}': {}", action, element, reason),
        )
        .with_suggestion("Verify the element is visible and enabled")
        .with_suggestion("Check if another window is blocking the element")
        .with_suggestion("Try waiting for the element to be ready first")
    }

    /// Create an ActionTimeout error builder
    pub fn action_timeout(operation: &str, timeout_secs: f64, condition: &str) -> Self {
        Self::new(
            ErrorType::ActionTimeout,
            format!(
                "Timeout after {:.1}s waiting for: {}\nOperation: {}",
                timeout_secs, condition, operation
            ),
        )
        .with_suggestion("Increase timeout using 'Set Timeout' keyword")
        .with_suggestion("Verify the condition will eventually be met")
        .with_suggestion("Check if the application is responding")
    }

    /// Create an ActionNotSupported error builder
    pub fn action_not_supported(action: &str, element_type: &str) -> Self {
        Self::new(
            ErrorType::ActionNotSupported,
            format!(
                "Action '{}' is not supported for element type '{}'",
                action, element_type
            ),
        )
        .with_suggestion("Check the element type supports this action")
        .with_suggestion("Use 'Get Element Type' keyword to verify the element type")
    }

    // --------------------------------------------------------
    // Technology Error Builders
    // --------------------------------------------------------

    /// Create a ModeNotSupported error builder
    pub fn mode_not_supported(keyword: &str, current_mode: GuiMode, required_modes: &[GuiMode]) -> Self {
        let required_str = required_modes
            .iter()
            .map(|m| format!("{}", m))
            .collect::<Vec<_>>()
            .join(", ");

        Self::new(
            ErrorType::ModeNotSupported,
            format!(
                "Keyword '{}' is not available in {} mode. Requires: {}",
                keyword, current_mode, required_str
            ),
        )
        .with_suggestion(format!(
            "Connect to an application that supports {} mode",
            required_str
        ))
        .with_suggestion("Use 'Set Mode' to switch to a supported mode (if available)")
    }

    /// Create an RcpWorkbench error builder
    pub fn rcp_workbench(operation: &str, reason: &str) -> Self {
        Self::new(
            ErrorType::RcpWorkbench,
            format!("RCP workbench operation '{}' failed: {}", operation, reason),
        )
        .with_suggestion("Verify the Eclipse RCP workbench is active")
        .with_suggestion("Check if the perspective/view/editor exists")
    }

    /// Create a SwtShell error builder
    pub fn swt_shell(shell_name: &str, reason: &str) -> Self {
        Self::new(
            ErrorType::SwtShell,
            format!("SWT shell '{}' operation failed: {}", shell_name, reason),
        )
        .with_suggestion("Verify the SWT shell is open and visible")
        .with_suggestion("Check if the shell was disposed")
    }

    // --------------------------------------------------------
    // Internal Error Builders
    // --------------------------------------------------------

    /// Create an Internal error builder
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorType::Internal, message.into())
            .with_suggestion("This is an internal error. Please report it at: https://github.com/robotframework/robotframework-javagui/issues")
    }

    // --------------------------------------------------------
    // Builder Methods
    // --------------------------------------------------------

    /// Add context information to the error
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }

    /// Add a troubleshooting suggestion
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    /// Add similar elements that might be what the user wanted
    pub fn with_similar_elements(mut self, similar: &[SimilarElement]) -> Self {
        for elem in similar.iter().take(3) {
            let suggestion = if let Some(ref name) = elem.name {
                format!(
                    "Did you mean: {} ({}, name='{}')",
                    elem.locator, elem.class_name, name
                )
            } else if let Some(ref text) = elem.text {
                let truncated = if text.len() > 30 {
                    format!("{}...", &text[..30])
                } else {
                    text.clone()
                };
                format!(
                    "Did you mean: {} ({}, text='{}')",
                    elem.locator, elem.class_name, truncated
                )
            } else {
                format!("Did you mean: {} ({})", elem.locator, elem.class_name)
            };
            self.suggestions.push(suggestion);
        }
        self
    }

    /// Add a searched tree summary for element-not-found errors
    pub fn with_searched_tree(mut self, tree_summary: &str) -> Self {
        self.context
            .insert("searched_tree".to_string(), tree_summary.to_string());
        self
    }

    /// Add a related error message
    pub fn with_related_error(mut self, error: impl Into<String>) -> Self {
        self.related_errors.push(error.into());
        self
    }

    /// Set the GUI mode context
    pub fn with_mode(self, mode: GuiMode) -> Self {
        self.with_context("mode", format!("{}", mode))
    }

    /// Build the final PyErr
    pub fn build(self) -> PyErr {
        let mut message = self.message.clone();

        // Add context section
        if !self.context.is_empty() {
            message.push_str("\n\nContext:");
            for (key, value) in &self.context {
                if key == "searched_tree" {
                    // Format tree separately
                    message.push_str(&format!("\n  UI Tree (summary):\n{}", indent_lines(value, 4)));
                } else {
                    message.push_str(&format!("\n  {}: {}", key, value));
                }
            }
        }

        // Add suggestions section
        if !self.suggestions.is_empty() {
            message.push_str("\n\nTroubleshooting:");
            for (i, suggestion) in self.suggestions.iter().enumerate() {
                message.push_str(&format!("\n  {}. {}", i + 1, suggestion));
            }
        }

        // Add related errors section
        if !self.related_errors.is_empty() {
            message.push_str("\n\nRelated errors:");
            for error in &self.related_errors {
                message.push_str(&format!("\n  - {}", error));
            }
        }

        // Return appropriate exception type
        match self.error_type {
            // Connection errors
            ErrorType::Connection => ConnectionError::new_err(message),
            ErrorType::ConnectionRefused => ConnectionRefusedError::new_err(message),
            ErrorType::ConnectionTimeout => ConnectionTimeoutError::new_err(message),
            ErrorType::NotConnected => NotConnectedError::new_err(message),

            // Element errors
            ErrorType::Element => ElementError::new_err(message),
            ErrorType::ElementNotFound => ElementNotFoundError::new_err(message),
            ErrorType::MultipleElementsFound => MultipleElementsFoundError::new_err(message),
            ErrorType::ElementNotInteractable => ElementNotInteractableError::new_err(message),
            ErrorType::StaleElement => StaleElementError::new_err(message),

            // Locator errors
            ErrorType::Locator => LocatorError::new_err(message),
            ErrorType::LocatorParse => LocatorParseError::new_err(message),
            ErrorType::InvalidLocatorSyntax => InvalidLocatorSyntaxError::new_err(message),

            // Action errors
            ErrorType::Action => ActionError::new_err(message),
            ErrorType::ActionFailed => ActionFailedError::new_err(message),
            ErrorType::ActionTimeout => ActionTimeoutError::new_err(message),
            ErrorType::ActionNotSupported => ActionNotSupportedError::new_err(message),

            // Technology errors
            ErrorType::Technology => TechnologyError::new_err(message),
            ErrorType::ModeNotSupported => ModeNotSupportedError::new_err(message),
            ErrorType::RcpWorkbench => RcpWorkbenchError::new_err(message),
            ErrorType::SwtShell => SwtShellError::new_err(message),

            // Internal
            ErrorType::Internal => InternalError::new_err(message),
        }
    }
}

/// Helper function to indent lines
fn indent_lines(text: &str, spaces: usize) -> String {
    let indent = " ".repeat(spaces);
    text.lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n")
}

// ============================================================
// Standardized Error Message Templates
// ============================================================

/// Standardized error message templates for consistent error formatting
pub struct ErrorMessages;

impl ErrorMessages {
    /// Format an element-not-found error message
    pub fn element_not_found(locator: &str, mode: GuiMode) -> String {
        format!(
            "Element not found: '{}'\n\
             Mode: {}\n\
             \n\
             Troubleshooting:\n\
             1. Verify the element exists using 'Log UI Tree'\n\
             2. Check if the application window is visible\n\
             3. Wait for the element to appear using 'Wait Until Element Exists'\n\
             4. Verify the locator syntax is correct for {} mode",
            locator, mode, mode
        )
    }

    /// Format a connection-refused error message
    pub fn connection_refused(host: &str, port: u16) -> String {
        format!(
            "Connection refused to {}:{}\n\
             \n\
             Troubleshooting:\n\
             1. Verify the Java application is running\n\
             2. Check that the Java agent is loaded (java -javaagent:javagui-agent.jar ...)\n\
             3. Verify the port number matches the agent configuration\n\
             4. Check firewall settings",
            host, port
        )
    }

    /// Format a mode-not-supported error message
    pub fn mode_not_supported(
        keyword: &str,
        current_mode: GuiMode,
        required_modes: &[GuiMode],
    ) -> String {
        let required_str = required_modes
            .iter()
            .map(|m| format!("{}", m).to_lowercase())
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "Keyword '{}' is not available in {} mode.\n\
             This keyword requires: {}\n\
             \n\
             To use this keyword:\n\
             1. Connect to an application that supports {} mode, or\n\
             2. Use 'Set Mode' to switch to a supported mode",
            keyword, current_mode, required_str, required_str
        )
    }

    /// Format an action-failed error message
    pub fn action_failed(action: &str, element: &str, reason: &str) -> String {
        format!(
            "Action '{}' failed on element '{}'\n\
             Reason: {}\n\
             \n\
             Troubleshooting:\n\
             1. Verify the element is visible and enabled\n\
             2. Check if another window is blocking the element\n\
             3. Try waiting for the element to be ready first",
            action, element, reason
        )
    }

    /// Format a locator-parse error message
    pub fn locator_parse_error(locator: &str, error: &str, position: Option<usize>) -> String {
        let mut msg = format!(
            "Failed to parse locator: '{}'\n\
             Error: {}",
            locator, error
        );

        if let Some(pos) = position {
            msg.push_str(&format!("\n\nPosition {}:\n", pos));
            msg.push_str(locator);
            msg.push('\n');
            msg.push_str(&" ".repeat(pos));
            msg.push('^');
        }

        msg.push_str("\n\nValid locator formats:\n");
        msg.push_str("  Button[name='x']     - CSS-style with attribute\n");
        msg.push_str("  name:x               - Prefix-style\n");
        msg.push_str("  #x                   - ID shorthand\n");
        msg.push_str("  //Button[@name='x']  - XPath");

        msg
    }

    /// Format a timeout error message
    pub fn timeout(operation: &str, timeout_secs: f64, condition: &str) -> String {
        format!(
            "Timeout after {:.1}s waiting for: {}\n\
             Operation: {}\n\
             \n\
             Troubleshooting:\n\
             1. Increase timeout using 'Set Timeout' keyword\n\
             2. Verify the condition will eventually be met\n\
             3. Check if the application is responding",
            timeout_secs, condition, operation
        )
    }

    /// Format a multiple-elements error message
    pub fn multiple_elements(locator: &str, count: usize) -> String {
        format!(
            "Expected single element, found {} matching: '{}'\n\
             \n\
             Troubleshooting:\n\
             1. Use a more specific locator (add more attributes)\n\
             2. Use index syntax: {}[0], {}[1], etc.\n\
             3. Use 'Get Elements' keyword to get all matching elements",
            count, locator, locator, locator
        )
    }
}

// ============================================================
// Legacy Exception Type (Unified Internal Error)
// ============================================================

/// Unified internal error type that maps to Python exceptions
///
/// This type provides a Rust-friendly way to create errors that will be
/// converted to the appropriate Python exception type.
#[derive(Debug, Clone)]
pub struct UnifiedError {
    pub kind: ErrorType,
    pub message: String,
    pub details: Option<String>,
    pub context: HashMap<String, String>,
    pub suggestions: Vec<String>,
}

impl UnifiedError {
    /// Create a new UnifiedError
    pub fn new(kind: ErrorType, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            details: None,
            context: HashMap::new(),
            suggestions: Vec::new(),
        }
    }

    // Convenience constructors for common error types

    pub fn connection(message: impl Into<String>) -> Self {
        Self::new(ErrorType::Connection, message)
    }

    pub fn connection_refused(host: &str, port: u16) -> Self {
        Self::new(
            ErrorType::ConnectionRefused,
            format!("Connection refused to {}:{}", host, port),
        )
    }

    pub fn connection_timeout(timeout_ms: u64) -> Self {
        Self::new(
            ErrorType::ConnectionTimeout,
            format!("Connection timeout after {}ms", timeout_ms),
        )
    }

    pub fn not_connected() -> Self {
        Self::new(ErrorType::NotConnected, "Not connected to any application")
    }

    pub fn element_not_found(locator: impl Into<String>) -> Self {
        Self::new(
            ErrorType::ElementNotFound,
            format!("Element not found: {}", locator.into()),
        )
    }

    pub fn multiple_elements_found(locator: impl Into<String>, count: usize) -> Self {
        Self::new(
            ErrorType::MultipleElementsFound,
            format!(
                "Expected single element, found {} matching: {}",
                count,
                locator.into()
            ),
        )
    }

    pub fn element_not_interactable(element_id: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::new(
            ErrorType::ElementNotInteractable,
            format!(
                "Element '{}' is not interactable: {}",
                element_id.into(),
                reason.into()
            ),
        )
    }

    pub fn stale_element(element_id: impl Into<String>) -> Self {
        Self::new(
            ErrorType::StaleElement,
            format!("Element '{}' is stale (no longer in UI tree)", element_id.into()),
        )
    }

    pub fn locator_parse(message: impl Into<String>) -> Self {
        Self::new(ErrorType::LocatorParse, message)
    }

    pub fn invalid_locator_syntax(locator: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::new(
            ErrorType::InvalidLocatorSyntax,
            format!("Invalid locator '{}': {}", locator.into(), reason.into()),
        )
    }

    pub fn action_failed(action: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::new(
            ErrorType::ActionFailed,
            format!("Action '{}' failed: {}", action.into(), reason.into()),
        )
    }

    pub fn action_timeout(operation: impl Into<String>, timeout_secs: f64) -> Self {
        Self::new(
            ErrorType::ActionTimeout,
            format!(
                "Operation '{}' timed out after {:.1}s",
                operation.into(),
                timeout_secs
            ),
        )
    }

    pub fn action_not_supported(action: impl Into<String>, element_type: impl Into<String>) -> Self {
        Self::new(
            ErrorType::ActionNotSupported,
            format!(
                "Action '{}' is not supported for element type '{}'",
                action.into(),
                element_type.into()
            ),
        )
    }

    pub fn mode_not_supported(keyword: impl Into<String>, current_mode: GuiMode) -> Self {
        Self::new(
            ErrorType::ModeNotSupported,
            format!(
                "Keyword '{}' is not available in {} mode",
                keyword.into(),
                current_mode
            ),
        )
    }

    pub fn rcp_error(message: impl Into<String>) -> Self {
        Self::new(ErrorType::RcpWorkbench, message)
    }

    pub fn swt_shell_error(message: impl Into<String>) -> Self {
        Self::new(ErrorType::SwtShell, message)
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorType::Internal, message)
    }

    /// Add details to the error
    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Add context to the error
    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key.into(), value.into());
        self
    }

    /// Add a suggestion to the error
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }
}

impl std::fmt::Display for UnifiedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;

        if let Some(details) = &self.details {
            write!(f, "\nDetails: {}", details)?;
        }

        if !self.context.is_empty() {
            write!(f, "\nContext:")?;
            for (key, value) in &self.context {
                write!(f, "\n  {}: {}", key, value)?;
            }
        }

        if !self.suggestions.is_empty() {
            write!(f, "\nSuggestions:")?;
            for suggestion in &self.suggestions {
                write!(f, "\n  - {}", suggestion)?;
            }
        }

        Ok(())
    }
}

impl std::error::Error for UnifiedError {}

impl From<UnifiedError> for PyErr {
    fn from(err: UnifiedError) -> PyErr {
        let msg = err.to_string();
        match err.kind {
            // Connection errors
            ErrorType::Connection => ConnectionError::new_err(msg),
            ErrorType::ConnectionRefused => ConnectionRefusedError::new_err(msg),
            ErrorType::ConnectionTimeout => ConnectionTimeoutError::new_err(msg),
            ErrorType::NotConnected => NotConnectedError::new_err(msg),

            // Element errors
            ErrorType::Element => ElementError::new_err(msg),
            ErrorType::ElementNotFound => ElementNotFoundError::new_err(msg),
            ErrorType::MultipleElementsFound => MultipleElementsFoundError::new_err(msg),
            ErrorType::ElementNotInteractable => ElementNotInteractableError::new_err(msg),
            ErrorType::StaleElement => StaleElementError::new_err(msg),

            // Locator errors
            ErrorType::Locator => LocatorError::new_err(msg),
            ErrorType::LocatorParse => LocatorParseError::new_err(msg),
            ErrorType::InvalidLocatorSyntax => InvalidLocatorSyntaxError::new_err(msg),

            // Action errors
            ErrorType::Action => ActionError::new_err(msg),
            ErrorType::ActionFailed => ActionFailedError::new_err(msg),
            ErrorType::ActionTimeout => ActionTimeoutError::new_err(msg),
            ErrorType::ActionNotSupported => ActionNotSupportedError::new_err(msg),

            // Technology errors
            ErrorType::Technology => TechnologyError::new_err(msg),
            ErrorType::ModeNotSupported => ModeNotSupportedError::new_err(msg),
            ErrorType::RcpWorkbench => RcpWorkbenchError::new_err(msg),
            ErrorType::SwtShell => SwtShellError::new_err(msg),

            // Internal
            ErrorType::Internal => InternalError::new_err(msg),
        }
    }
}

// ============================================================
// Exception Registration
// ============================================================

/// Register all unified exceptions with the Python module.
///
/// This function registers both the new unified exception names and
/// backwards-compatible aliases for the legacy exception names.
pub fn register_unified_exceptions(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // --------------------------------------------------------
    // New Unified Exceptions
    // --------------------------------------------------------

    // Base exception
    m.add("JavaGuiError", py.get_type::<JavaGuiError>())?;

    // Connection exceptions
    m.add("ConnectionError", py.get_type::<ConnectionError>())?;
    m.add("ConnectionRefusedError", py.get_type::<ConnectionRefusedError>())?;
    m.add("ConnectionTimeoutError", py.get_type::<ConnectionTimeoutError>())?;
    m.add("NotConnectedError", py.get_type::<NotConnectedError>())?;

    // Element exceptions
    m.add("ElementError", py.get_type::<ElementError>())?;
    m.add("ElementNotFoundError", py.get_type::<ElementNotFoundError>())?;
    m.add("MultipleElementsFoundError", py.get_type::<MultipleElementsFoundError>())?;
    m.add("ElementNotInteractableError", py.get_type::<ElementNotInteractableError>())?;
    m.add("StaleElementError", py.get_type::<StaleElementError>())?;

    // Locator exceptions
    m.add("LocatorError", py.get_type::<LocatorError>())?;
    m.add("LocatorParseError", py.get_type::<LocatorParseError>())?;
    m.add("InvalidLocatorSyntaxError", py.get_type::<InvalidLocatorSyntaxError>())?;

    // Action exceptions
    m.add("ActionError", py.get_type::<ActionError>())?;
    m.add("ActionFailedError", py.get_type::<ActionFailedError>())?;
    m.add("ActionTimeoutError", py.get_type::<ActionTimeoutError>())?;
    m.add("ActionNotSupportedError", py.get_type::<ActionNotSupportedError>())?;

    // Technology exceptions
    m.add("TechnologyError", py.get_type::<TechnologyError>())?;
    m.add("ModeNotSupportedError", py.get_type::<ModeNotSupportedError>())?;
    m.add("RcpWorkbenchError", py.get_type::<RcpWorkbenchError>())?;
    m.add("SwtShellError", py.get_type::<SwtShellError>())?;

    // Internal exception
    m.add("InternalError", py.get_type::<InternalError>())?;

    // --------------------------------------------------------
    // Legacy Aliases (Backwards Compatibility)
    // --------------------------------------------------------

    // SwingConnectionError -> ConnectionError
    m.add("SwingConnectionError", py.get_type::<ConnectionError>())?;

    // SwingTimeoutError -> ActionTimeoutError
    m.add("SwingTimeoutError", py.get_type::<ActionTimeoutError>())?;

    // TimeoutError -> ActionTimeoutError (was registered as TimeoutError before)
    m.add("TimeoutError", py.get_type::<ActionTimeoutError>())?;

    // PyLocatorParseError -> LocatorParseError (internal name was different)
    // Note: LocatorParseError is already registered above

    Ok(())
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_builder_basic() {
        let builder = ErrorBuilder::element_not_found("button[name='ok']");
        assert!(builder.message.contains("Element not found"));
        assert!(builder.message.contains("button[name='ok']"));
    }

    #[test]
    fn test_error_builder_with_context() {
        let builder = ErrorBuilder::element_not_found("button[name='ok']")
            .with_context("mode", "swing")
            .with_context("window", "Main");

        assert!(builder.context.contains_key("mode"));
        assert!(builder.context.contains_key("window"));
    }

    #[test]
    fn test_error_builder_with_suggestions() {
        let builder = ErrorBuilder::element_not_found("button[name='ok']")
            .with_suggestion("Try this")
            .with_suggestion("Or that");

        // Default suggestions + custom suggestions
        assert!(builder.suggestions.len() >= 2);
    }

    #[test]
    fn test_error_builder_with_similar_elements() {
        let similar = vec![
            SimilarElement {
                locator: "button[name='cancel']".to_string(),
                class_name: "JButton".to_string(),
                name: Some("cancel".to_string()),
                text: None,
                similarity_score: 0.8,
            },
        ];

        let builder = ErrorBuilder::element_not_found("button[name='ok']")
            .with_similar_elements(&similar);

        assert!(builder.suggestions.iter().any(|s| s.contains("cancel")));
    }

    #[test]
    fn test_unified_error_display() {
        let err = UnifiedError::element_not_found("button[name='submit']")
            .with_details("Searched 50 elements")
            .with_context("window", "Login");

        let msg = err.to_string();
        assert!(msg.contains("Element not found"));
        assert!(msg.contains("button[name='submit']"));
        assert!(msg.contains("Searched 50 elements"));
        assert!(msg.contains("window"));
        assert!(msg.contains("Login"));
    }

    #[test]
    fn test_error_messages_element_not_found() {
        let msg = ErrorMessages::element_not_found("button[name='x']", GuiMode::Swing);
        assert!(msg.contains("Element not found"));
        assert!(msg.contains("button[name='x']"));
        assert!(msg.contains("Swing"));
        assert!(msg.contains("Log UI Tree"));
    }

    #[test]
    fn test_error_messages_connection_refused() {
        let msg = ErrorMessages::connection_refused("localhost", 9999);
        assert!(msg.contains("Connection refused"));
        assert!(msg.contains("localhost:9999"));
        assert!(msg.contains("javaagent"));
    }

    #[test]
    fn test_error_messages_locator_parse() {
        let msg = ErrorMessages::locator_parse_error("button[name=", "unclosed bracket", Some(11));
        assert!(msg.contains("Failed to parse locator"));
        assert!(msg.contains("Position 11"));
        assert!(msg.contains("^"));
        assert!(msg.contains("Valid locator formats"));
    }

    #[test]
    fn test_gui_mode_display() {
        assert_eq!(format!("{}", GuiMode::Swing), "Swing");
        assert_eq!(format!("{}", GuiMode::Swt), "SWT");
        assert_eq!(format!("{}", GuiMode::Rcp), "RCP");
        assert_eq!(format!("{}", GuiMode::Auto), "Auto");
    }
}
