//! Python exception types for Robot Framework Java GUI library
//!
//! This module provides the legacy exception interface that delegates to the unified
//! exception hierarchy in `unified_exceptions.rs`. It maintains backwards compatibility
//! while allowing gradual migration to the new exception types.
//!
//! # Migration Guide
//!
//! The following legacy exceptions map to new unified exceptions:
//!
//! | Legacy Exception         | Unified Exception       |
//! |--------------------------|-------------------------|
//! | `SwingConnectionError`   | `ConnectionError`       |
//! | `SwingTimeoutError`      | `ActionTimeoutError`    |
//! | `PyLocatorParseError`    | `LocatorParseError`     |
//! | `ElementNotFoundError`   | `ElementNotFoundError`  |
//! | `MultipleElementsFoundError` | `MultipleElementsFoundError` |
//! | `ActionFailedError`      | `ActionFailedError`     |
//!
//! The new exceptions are organized in a hierarchy:
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

use pyo3::prelude::*;

// Re-export unified exceptions for internal use
pub use crate::python::unified_exceptions::{
    // Base
    JavaGuiError,
    // Connection
    ConnectionError, ConnectionRefusedError, ConnectionTimeoutError, NotConnectedError,
    // Element
    ElementError, ElementNotFoundError, MultipleElementsFoundError,
    ElementNotInteractableError, StaleElementError,
    // Locator
    LocatorError, LocatorParseError, InvalidLocatorSyntaxError,
    // Action
    ActionError, ActionFailedError, ActionTimeoutError, ActionNotSupportedError,
    // Technology
    TechnologyError, ModeNotSupportedError, RcpWorkbenchError, SwtShellError,
    // Internal
    InternalError,
    // Builder and helpers
    ErrorBuilder, ErrorMessages, ErrorType, GuiMode, SimilarElement, UnifiedError,
};

// ============================================================
// Legacy Exception Types (for backwards compatibility)
// ============================================================

// Note: The following type aliases are provided for backwards compatibility.
// New code should use the unified exception types directly.

/// Legacy alias for ConnectionError
///
/// **Deprecated**: Use `ConnectionError` instead.
pub type SwingConnectionError = ConnectionError;

/// Legacy alias for ActionTimeoutError
///
/// **Deprecated**: Use `ActionTimeoutError` instead.
pub type SwingTimeoutError = ActionTimeoutError;

/// Legacy alias for LocatorParseError
///
/// **Deprecated**: Use `LocatorParseError` instead.
pub type PyLocatorParseError = LocatorParseError;

// ============================================================
// Legacy Internal Error Type (SwingError)
// ============================================================

/// Internal error type for library operations
///
/// This type is maintained for backwards compatibility. New code should use
/// `UnifiedError` from `unified_exceptions` module instead.
#[derive(Debug, Clone)]
pub struct SwingError {
    pub kind: SwingErrorKind,
    pub message: String,
    pub details: Option<String>,
}

/// Error kind enumeration (legacy)
///
/// **Deprecated**: Use `ErrorType` from `unified_exceptions` instead.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwingErrorKind {
    Connection,
    ElementNotFound,
    MultipleElementsFound,
    LocatorParse,
    ActionFailed,
    Timeout,
    Internal,
    // New kinds mapped from unified exceptions
    NotConnected,
    ElementNotInteractable,
    StaleElement,
    ModeNotSupported,
    RcpWorkbench,
    SwtShell,
}

impl SwingError {
    /// Create a new SwingError with the given kind and message
    pub fn new(kind: SwingErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            details: None,
        }
    }

    pub fn connection(message: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::Connection,
            message: message.into(),
            details: None,
        }
    }

    pub fn not_connected() -> Self {
        Self {
            kind: SwingErrorKind::NotConnected,
            message: "Not connected to any application".into(),
            details: None,
        }
    }

    /// Create a validation error
    pub fn validation(message: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::ActionFailed,
            message: format!("Validation error: {}", message.into()),
            details: None,
        }
    }

    pub fn element_not_found(locator: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::ElementNotFound,
            message: format!("Element not found: {}", locator.into()),
            details: None,
        }
    }

    pub fn multiple_elements_found(locator: impl Into<String>, count: usize) -> Self {
        Self {
            kind: SwingErrorKind::MultipleElementsFound,
            message: format!(
                "Expected single element, found {} matching: {}",
                count,
                locator.into()
            ),
            details: None,
        }
    }

    pub fn element_not_interactable(element_id: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::ElementNotInteractable,
            message: format!(
                "Element '{}' is not interactable: {}",
                element_id.into(),
                reason.into()
            ),
            details: None,
        }
    }

    pub fn stale_element(element_id: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::StaleElement,
            message: format!("Element '{}' is stale (no longer in UI tree)", element_id.into()),
            details: None,
        }
    }

    pub fn locator_parse(message: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::LocatorParse,
            message: message.into(),
            details: None,
        }
    }

    pub fn action_failed(action: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::ActionFailed,
            message: format!("Action '{}' failed: {}", action.into(), reason.into()),
            details: None,
        }
    }

    pub fn rcp_error(message: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::RcpWorkbench,
            message: format!("RcpError: {}", message.into()),
            details: None,
        }
    }

    pub fn swt_shell_error(message: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::SwtShell,
            message: format!("SwtShellError: {}", message.into()),
            details: None,
        }
    }

    pub fn mode_not_supported(keyword: impl Into<String>, current_mode: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::ModeNotSupported,
            message: format!(
                "Keyword '{}' is not available in {} mode",
                keyword.into(),
                current_mode.into()
            ),
            details: None,
        }
    }

    pub fn timeout(operation: impl Into<String>, timeout_secs: f64) -> Self {
        Self {
            kind: SwingErrorKind::Timeout,
            message: format!(
                "Operation '{}' timed out after {:.1}s",
                operation.into(),
                timeout_secs
            ),
            details: None,
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::Internal,
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

impl std::fmt::Display for SwingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(details) = &self.details {
            write!(f, "{}\nDetails: {}", self.message, details)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for SwingError {}

impl From<SwingError> for PyErr {
    fn from(err: SwingError) -> PyErr {
        let msg = err.to_string();
        match err.kind {
            // Connection errors -> unified ConnectionError hierarchy
            SwingErrorKind::Connection => ConnectionError::new_err(msg),
            SwingErrorKind::NotConnected => NotConnectedError::new_err(msg),

            // Element errors -> unified ElementError hierarchy
            SwingErrorKind::ElementNotFound => ElementNotFoundError::new_err(msg),
            SwingErrorKind::MultipleElementsFound => MultipleElementsFoundError::new_err(msg),
            SwingErrorKind::ElementNotInteractable => ElementNotInteractableError::new_err(msg),
            SwingErrorKind::StaleElement => StaleElementError::new_err(msg),

            // Locator errors -> unified LocatorError hierarchy
            SwingErrorKind::LocatorParse => LocatorParseError::new_err(msg),

            // Action errors -> unified ActionError hierarchy
            SwingErrorKind::ActionFailed => ActionFailedError::new_err(msg),
            SwingErrorKind::Timeout => ActionTimeoutError::new_err(msg),

            // Technology errors -> unified TechnologyError hierarchy
            SwingErrorKind::ModeNotSupported => ModeNotSupportedError::new_err(msg),
            SwingErrorKind::RcpWorkbench => RcpWorkbenchError::new_err(msg),
            SwingErrorKind::SwtShell => SwtShellError::new_err(msg),

            // Internal errors
            SwingErrorKind::Internal => InternalError::new_err(msg),
        }
    }
}

impl From<crate::locator::LocatorParseError> for SwingError {
    fn from(err: crate::locator::LocatorParseError) -> Self {
        SwingError::locator_parse(err.to_string())
    }
}

// ============================================================
// Conversion between SwingError and UnifiedError
// ============================================================

impl From<SwingError> for UnifiedError {
    fn from(err: SwingError) -> Self {
        let error_type = match err.kind {
            SwingErrorKind::Connection => ErrorType::Connection,
            SwingErrorKind::NotConnected => ErrorType::NotConnected,
            SwingErrorKind::ElementNotFound => ErrorType::ElementNotFound,
            SwingErrorKind::MultipleElementsFound => ErrorType::MultipleElementsFound,
            SwingErrorKind::ElementNotInteractable => ErrorType::ElementNotInteractable,
            SwingErrorKind::StaleElement => ErrorType::StaleElement,
            SwingErrorKind::LocatorParse => ErrorType::LocatorParse,
            SwingErrorKind::ActionFailed => ErrorType::ActionFailed,
            SwingErrorKind::Timeout => ErrorType::ActionTimeout,
            SwingErrorKind::ModeNotSupported => ErrorType::ModeNotSupported,
            SwingErrorKind::RcpWorkbench => ErrorType::RcpWorkbench,
            SwingErrorKind::SwtShell => ErrorType::SwtShell,
            SwingErrorKind::Internal => ErrorType::Internal,
        };

        let mut unified = UnifiedError::new(error_type, err.message);
        if let Some(details) = err.details {
            unified = unified.with_details(details);
        }
        unified
    }
}

impl From<UnifiedError> for SwingError {
    fn from(err: UnifiedError) -> Self {
        let kind = match err.kind {
            ErrorType::Connection | ErrorType::ConnectionRefused | ErrorType::ConnectionTimeout => {
                SwingErrorKind::Connection
            }
            ErrorType::NotConnected => SwingErrorKind::NotConnected,
            ErrorType::Element | ErrorType::ElementNotFound => SwingErrorKind::ElementNotFound,
            ErrorType::MultipleElementsFound => SwingErrorKind::MultipleElementsFound,
            ErrorType::ElementNotInteractable => SwingErrorKind::ElementNotInteractable,
            ErrorType::StaleElement => SwingErrorKind::StaleElement,
            ErrorType::Locator | ErrorType::LocatorParse | ErrorType::InvalidLocatorSyntax => {
                SwingErrorKind::LocatorParse
            }
            ErrorType::Action | ErrorType::ActionFailed | ErrorType::ActionNotSupported => {
                SwingErrorKind::ActionFailed
            }
            ErrorType::ActionTimeout => SwingErrorKind::Timeout,
            ErrorType::Technology | ErrorType::ModeNotSupported => SwingErrorKind::ModeNotSupported,
            ErrorType::RcpWorkbench => SwingErrorKind::RcpWorkbench,
            ErrorType::SwtShell => SwingErrorKind::SwtShell,
            ErrorType::Internal => SwingErrorKind::Internal,
        };

        let mut swing_err = SwingError::new(kind, err.message);
        if let Some(details) = err.details {
            swing_err = swing_err.with_details(details);
        }
        swing_err
    }
}

// ============================================================
// Register Python Exceptions
// ============================================================

/// Register Python exception types with the module
///
/// This function registers both the unified exceptions and legacy aliases
/// for backwards compatibility.
pub fn register_exceptions(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Register all unified exceptions (includes legacy aliases)
    crate::python::unified_exceptions::register_unified_exceptions(py, m)
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swing_error_display() {
        let err = SwingError::element_not_found("button[name='submit']");
        assert!(err.to_string().contains("button[name='submit']"));
    }

    #[test]
    fn test_swing_error_with_details() {
        let err = SwingError::element_not_found("button[name='submit']")
            .with_details("Searched 50 elements");
        let msg = err.to_string();
        assert!(msg.contains("button[name='submit']"));
        assert!(msg.contains("Searched 50 elements"));
    }

    #[test]
    fn test_swing_error_kinds() {
        let conn = SwingError::connection("test");
        assert_eq!(conn.kind, SwingErrorKind::Connection);

        let not_found = SwingError::element_not_found("x");
        assert_eq!(not_found.kind, SwingErrorKind::ElementNotFound);

        let timeout = SwingError::timeout("click", 5.0);
        assert_eq!(timeout.kind, SwingErrorKind::Timeout);

        let rcp = SwingError::rcp_error("workbench failed");
        assert_eq!(rcp.kind, SwingErrorKind::RcpWorkbench);
    }

    #[test]
    fn test_swing_to_unified_conversion() {
        let swing = SwingError::element_not_found("button[name='x']")
            .with_details("extra info");
        let unified: UnifiedError = swing.into();

        assert_eq!(unified.kind, ErrorType::ElementNotFound);
        assert!(unified.message.contains("button[name='x']"));
        assert!(unified.details.is_some());
    }

    #[test]
    fn test_unified_to_swing_conversion() {
        let unified = UnifiedError::action_timeout("click", 5.0);
        let swing: SwingError = unified.into();

        assert_eq!(swing.kind, SwingErrorKind::Timeout);
        assert!(swing.message.contains("5.0s"));
    }

    #[test]
    fn test_new_error_kinds() {
        let not_interactable = SwingError::element_not_interactable("btn1", "disabled");
        assert_eq!(not_interactable.kind, SwingErrorKind::ElementNotInteractable);
        assert!(not_interactable.message.contains("disabled"));

        let stale = SwingError::stale_element("btn2");
        assert_eq!(stale.kind, SwingErrorKind::StaleElement);
        assert!(stale.message.contains("stale"));

        let mode = SwingError::mode_not_supported("Open Perspective", "swing");
        assert_eq!(mode.kind, SwingErrorKind::ModeNotSupported);
        assert!(mode.message.contains("Open Perspective"));

        let swt = SwingError::swt_shell_error("shell disposed");
        assert_eq!(swt.kind, SwingErrorKind::SwtShell);
    }
}
