//! Comprehensive unit tests for Python bindings
//!
//! Tests cover:
//! - Exception types and error creation
//! - SwingError conversion to PyErr
//! - Error builder pattern with context and suggestions
//! - Error kind classification

#[cfg(test)]
mod swing_error_tests {
    use crate::python::exceptions::{SwingError, SwingErrorKind};

    #[test]
    fn test_swing_error_new() {
        let err = SwingError::new(SwingErrorKind::Connection, "Failed to connect");
        assert_eq!(err.kind, SwingErrorKind::Connection);
        assert_eq!(err.message, "Failed to connect");
        assert!(err.details.is_none());
    }

    #[test]
    fn test_swing_error_connection() {
        let err = SwingError::connection("Connection refused");
        assert_eq!(err.kind, SwingErrorKind::Connection);
        assert!(err.message.contains("Connection refused"));
    }

    #[test]
    fn test_swing_error_element_not_found() {
        let err = SwingError::element_not_found("name:testButton");
        assert_eq!(err.kind, SwingErrorKind::ElementNotFound);
        assert!(err.message.contains("testButton"));
        assert!(err.message.contains("not found"));
    }

    #[test]
    fn test_swing_error_multiple_elements_found() {
        let err = SwingError::multiple_elements_found("JButton", 5);
        assert_eq!(err.kind, SwingErrorKind::MultipleElementsFound);
        assert!(err.message.contains("5"));
        assert!(err.message.contains("JButton"));
    }

    #[test]
    fn test_swing_error_locator_parse() {
        let err = SwingError::locator_parse("Invalid syntax in locator");
        assert_eq!(err.kind, SwingErrorKind::LocatorParse);
        assert!(err.message.contains("Invalid syntax"));
    }

    #[test]
    fn test_swing_error_action_failed() {
        let err = SwingError::action_failed("click", "element is disabled");
        assert_eq!(err.kind, SwingErrorKind::ActionFailed);
        assert!(err.message.contains("click"));
        assert!(err.message.contains("disabled"));
    }

    #[test]
    fn test_swing_error_timeout() {
        let err = SwingError::timeout("wait_until_element_exists", 10.0);
        assert_eq!(err.kind, SwingErrorKind::Timeout);
        assert!(err.message.contains("10"));
        assert!(err.message.contains("wait_until_element_exists"));
    }

    #[test]
    fn test_swing_error_validation() {
        let err = SwingError::validation("Value must be positive");
        assert_eq!(err.kind, SwingErrorKind::ActionFailed);
        assert!(err.message.contains("Validation"));
        assert!(err.message.contains("positive"));
    }

    #[test]
    fn test_swing_error_rcp_error() {
        let err = SwingError::rcp_error("Workbench not available");
        assert_eq!(err.kind, SwingErrorKind::ActionFailed);
        assert!(err.message.contains("RcpError"));
        assert!(err.message.contains("Workbench"));
    }

    #[test]
    fn test_swing_error_with_details() {
        let err = SwingError::connection("Connection failed")
            .with_details("Host: localhost:5678, Error: ECONNREFUSED");

        assert!(err.details.is_some());
        let details = err.details.unwrap();
        assert!(details.contains("localhost:5678"));
        assert!(details.contains("ECONNREFUSED"));
    }

    #[test]
    fn test_swing_error_display_without_details() {
        let err = SwingError::element_not_found("name:btn");
        let display = format!("{}", err);
        assert!(display.contains("not found"));
        assert!(!display.contains("Details:"));
    }

    #[test]
    fn test_swing_error_display_with_details() {
        let err = SwingError::element_not_found("name:btn")
            .with_details("Searched in window 'Main'");
        let display = format!("{}", err);
        assert!(display.contains("not found"));
        assert!(display.contains("Details:"));
        assert!(display.contains("Main"));
    }
}

#[cfg(test)]
mod swing_error_kind_tests {
    use crate::python::exceptions::SwingErrorKind;

    #[test]
    fn test_error_kind_equality() {
        assert_eq!(SwingErrorKind::Connection, SwingErrorKind::Connection);
        assert_ne!(SwingErrorKind::Connection, SwingErrorKind::Timeout);
    }

    #[test]
    fn test_error_kind_clone() {
        let kind = SwingErrorKind::ElementNotFound;
        let cloned = kind.clone();
        assert_eq!(kind, cloned);
    }

    #[test]
    fn test_error_kind_copy() {
        let kind = SwingErrorKind::ActionFailed;
        let copied: SwingErrorKind = kind;
        assert_eq!(kind, copied);
    }

    #[test]
    fn test_all_error_kinds_exist() {
        // Ensure all error kinds can be constructed
        let kinds = [
            SwingErrorKind::Connection,
            SwingErrorKind::ElementNotFound,
            SwingErrorKind::MultipleElementsFound,
            SwingErrorKind::LocatorParse,
            SwingErrorKind::ActionFailed,
            SwingErrorKind::Timeout,
            SwingErrorKind::Internal,
        ];

        assert_eq!(kinds.len(), 7);
    }
}

#[cfg(test)]
mod swing_error_builder_tests {
    use crate::python::exceptions::{SwingError, SwingErrorKind};

    #[test]
    fn test_error_builder_with_context() {
        let err = SwingError::new(SwingErrorKind::Connection, "Failed")
            .with_details("Context: trying to connect to localhost");

        assert!(err.details.is_some());
        assert!(err.details.as_ref().unwrap().contains("Context"));
    }

    #[test]
    fn test_error_builder_chain() {
        let err = SwingError::element_not_found("name:btn")
            .with_details("Window: Main, Searched: 100 elements");

        assert_eq!(err.kind, SwingErrorKind::ElementNotFound);
        assert!(err.details.is_some());
    }

    #[test]
    fn test_error_builder_preserves_kind() {
        let err = SwingError::timeout("wait", 5.0)
            .with_details("Additional info");

        assert_eq!(err.kind, SwingErrorKind::Timeout);
    }

    #[test]
    fn test_error_builder_preserves_message() {
        let err = SwingError::action_failed("click", "disabled")
            .with_details("Details");

        assert!(err.message.contains("click"));
        assert!(err.message.contains("disabled"));
    }
}

#[cfg(test)]
mod locator_error_conversion_tests {
    use crate::locator::unified::LocatorParseError;
    use crate::python::exceptions::SwingError;

    #[test]
    fn test_locator_parse_error_conversion() {
        let locator_err = LocatorParseError::new("Invalid bracket");
        let swing_err: SwingError = locator_err.into();

        assert_eq!(swing_err.kind, crate::python::exceptions::SwingErrorKind::LocatorParse);
        assert!(swing_err.message.contains("Invalid bracket"));
    }

    #[test]
    fn test_locator_parse_error_at_position_conversion() {
        let locator_err = LocatorParseError::at_position("Unexpected character", 5);
        let swing_err: SwingError = locator_err.into();

        assert!(swing_err.message.contains("5"));
        assert!(swing_err.message.contains("Unexpected"));
    }
}

#[cfg(test)]
mod error_context_tests {
    use crate::python::exceptions::SwingError;

    #[test]
    fn test_error_with_suggestions() {
        let err = SwingError::element_not_found("name:submitBtn")
            .with_details("Did you mean: name:submitButton, name:submitBtnOK?");

        let display = format!("{}", err);
        assert!(display.contains("submitButton"));
    }

    #[test]
    fn test_error_with_locator_context() {
        let err = SwingError::element_not_found("JButton[text='Save'][name='saveBtn']")
            .with_details("Found 3 elements matching JButton[text='Save'], none with name='saveBtn'");

        let display = format!("{}", err);
        assert!(display.contains("3 elements"));
    }

    #[test]
    fn test_error_with_timeout_context() {
        let err = SwingError::timeout("wait_until_element_exists", 30.0)
            .with_details("Element: name:loadingSpinner, Last state: visible=true, enabled=false");

        let display = format!("{}", err);
        assert!(display.contains("loadingSpinner"));
        assert!(display.contains("enabled=false"));
    }
}

#[cfg(test)]
mod error_chaining_tests {
    use crate::python::exceptions::SwingError;

    #[test]
    fn test_error_is_std_error() {
        let err = SwingError::connection("test");
        let _: &dyn std::error::Error = &err;
    }

    #[test]
    fn test_error_source_is_none() {
        let err = SwingError::connection("test");
        // SwingError doesn't chain to source errors
        use std::error::Error;
        assert!(err.source().is_none());
    }
}
