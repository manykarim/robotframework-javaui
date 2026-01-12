//! Python exception types for Robot Framework Swing library
//!
//! This module defines custom Python exceptions that map to library error conditions.

use pyo3::exceptions::PyException;
use pyo3::prelude::*;

/// Error raised when connection to a Java Swing application fails
pyo3::create_exception!(
    _swing_library,
    SwingConnectionError,
    PyException,
    "Error connecting to or communicating with Java Swing application."
);

/// Error raised when no element matches the given locator
pyo3::create_exception!(
    _swing_library,
    ElementNotFoundError,
    PyException,
    "No element found matching the given locator."
);

/// Error raised when multiple elements match a locator that expects a single element
pyo3::create_exception!(
    _swing_library,
    MultipleElementsFoundError,
    PyException,
    "Multiple elements found when only one was expected."
);

/// Error raised when a locator expression cannot be parsed
pyo3::create_exception!(
    _swing_library,
    PyLocatorParseError,
    PyException,
    "Failed to parse the locator expression."
);

/// Error raised when an action on an element fails
pyo3::create_exception!(
    _swing_library,
    ActionFailedError,
    PyException,
    "Failed to perform the requested action on the element."
);

/// Error raised when an operation times out
pyo3::create_exception!(
    _swing_library,
    SwingTimeoutError,
    PyException,
    "Operation timed out waiting for condition."
);

/// Internal error type for library operations
#[derive(Debug, Clone)]
pub struct SwingError {
    pub kind: SwingErrorKind,
    pub message: String,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwingErrorKind {
    Connection,
    ElementNotFound,
    MultipleElementsFound,
    LocatorParse,
    ActionFailed,
    Timeout,
    Internal,
}

impl SwingError {
    pub fn connection(message: impl Into<String>) -> Self {
        Self {
            kind: SwingErrorKind::Connection,
            message: message.into(),
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
            SwingErrorKind::Connection => SwingConnectionError::new_err(msg),
            SwingErrorKind::ElementNotFound => ElementNotFoundError::new_err(msg),
            SwingErrorKind::MultipleElementsFound => MultipleElementsFoundError::new_err(msg),
            SwingErrorKind::LocatorParse => PyLocatorParseError::new_err(msg),
            SwingErrorKind::ActionFailed => ActionFailedError::new_err(msg),
            SwingErrorKind::Timeout => SwingTimeoutError::new_err(msg),
            SwingErrorKind::Internal => PyException::new_err(msg),
        }
    }
}

impl From<crate::locator::LocatorParseError> for SwingError {
    fn from(err: crate::locator::LocatorParseError) -> Self {
        SwingError::locator_parse(err.to_string())
    }
}

/// Register Python exception types with the module
pub fn register_exceptions(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("SwingConnectionError", py.get_type::<SwingConnectionError>())?;
    m.add("ElementNotFoundError", py.get_type::<ElementNotFoundError>())?;
    m.add("MultipleElementsFoundError", py.get_type::<MultipleElementsFoundError>())?;
    m.add("LocatorParseError", py.get_type::<PyLocatorParseError>())?;
    m.add("ActionFailedError", py.get_type::<ActionFailedError>())?;
    m.add("TimeoutError", py.get_type::<SwingTimeoutError>())?;
    Ok(())
}
