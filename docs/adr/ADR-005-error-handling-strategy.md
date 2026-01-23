# ADR-005: Error Handling Strategy

| ADR ID | ADR-005 |
|--------|---------|
| Title | Error Handling Strategy |
| Status | Proposed |
| Date | 2026-01-16 |
| Authors | Architecture Team |

## Context

The library currently has error handling spread across multiple locations with inconsistent patterns:

### Current Error Implementation

**Rust Error Types** (`src/error.rs`):
```rust
pub enum SwingError {
    JvmNotFound { identifier: String },
    AttachFailed { pid: u32, reason: String },
    AgentInjectionFailed { reason: String },
    ConnectionLost,
    ConnectionTimeout { timeout_ms: u64 },
    NotConnected,
    ElementNotFound { locator: String, context: Option<Box<ElementNotFoundContext>> },
    MultipleElementsFound { locator: String, count: usize },
    ElementNotInteractable { element_id: String, reason: String },
    StaleElement { element_id: String },
    InvalidLocator { locator: String, message: String, position: Option<usize> },
    ActionFailed { action: String, element_id: String, reason: String },
    WaitTimeout { condition: String, timeout_ms: u64 },
    RpcError { code: i32, message: String },
    ProtocolError { message: String },
    SerializationError { message: String },
    Internal { message: String },
    Io(std::io::Error),
}
```

**Python Exceptions** (`src/python/exceptions.rs`):
```rust
// Current exceptions
SwingConnectionError
ElementNotFoundError
MultipleElementsFoundError
PyLocatorParseError
ActionFailedError
SwingTimeoutError
```

### Problems with Current Approach

1. **Naming Inconsistency**: Mix of "Swing" and generic prefixes
2. **Technology-Specific**: All exceptions use "Swing" naming, even for SWT/RCP
3. **Missing Categories**: No distinct exception for RCP-specific errors
4. **Limited Context**: Error messages don't always include enough debugging info
5. **Inconsistent Messages**: Different parts of code format errors differently

### Decision Drivers

- Unified error handling across all technologies
- Clear, actionable error messages
- Helpful debugging context
- Robot Framework compatibility
- Backwards compatibility with existing exception handlers

## Decision

We will implement a **Unified Exception Hierarchy** with **Technology-Agnostic Naming** and **Rich Error Context**.

### 1. Exception Hierarchy

```
JavaGuiError (base)
├── ConnectionError
│   ├── ConnectionRefusedError
│   ├── ConnectionTimeoutError
│   └── NotConnectedError
├── ElementError
│   ├── ElementNotFoundError
│   ├── MultipleElementsFoundError
│   ├── ElementNotInteractableError
│   └── StaleElementError
├── LocatorError
│   ├── LocatorParseError
│   └── InvalidLocatorSyntaxError
├── ActionError
│   ├── ActionFailedError
│   ├── ActionTimeoutError
│   └── ActionNotSupportedError
├── TechnologyError
│   ├── ModeNotSupportedError
│   ├── RcpWorkbenchError
│   └── SwtShellError
└── InternalError
```

### 2. Rust Implementation

```rust
//! Unified error types for robotframework-javagui

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
    "Base exception for all Java GUI library errors."
);

// ============================================================
// Connection Exceptions
// ============================================================

pyo3::create_exception!(
    javagui,
    ConnectionError,
    JavaGuiError,
    "Base exception for connection-related errors."
);

pyo3::create_exception!(
    javagui,
    ConnectionRefusedError,
    ConnectionError,
    "Connection to the Java application was refused."
);

pyo3::create_exception!(
    javagui,
    ConnectionTimeoutError,
    ConnectionError,
    "Connection attempt timed out."
);

pyo3::create_exception!(
    javagui,
    NotConnectedError,
    ConnectionError,
    "Operation requires an active connection."
);

// ============================================================
// Element Exceptions
// ============================================================

pyo3::create_exception!(
    javagui,
    ElementError,
    JavaGuiError,
    "Base exception for element-related errors."
);

pyo3::create_exception!(
    javagui,
    ElementNotFoundError,
    ElementError,
    "No element found matching the given locator."
);

pyo3::create_exception!(
    javagui,
    MultipleElementsFoundError,
    ElementError,
    "Multiple elements found when only one was expected."
);

pyo3::create_exception!(
    javagui,
    ElementNotInteractableError,
    ElementError,
    "Element exists but cannot be interacted with."
);

pyo3::create_exception!(
    javagui,
    StaleElementError,
    ElementError,
    "Element reference is stale (no longer in UI)."
);

// ============================================================
// Locator Exceptions
// ============================================================

pyo3::create_exception!(
    javagui,
    LocatorError,
    JavaGuiError,
    "Base exception for locator-related errors."
);

pyo3::create_exception!(
    javagui,
    LocatorParseError,
    LocatorError,
    "Failed to parse the locator expression."
);

pyo3::create_exception!(
    javagui,
    InvalidLocatorSyntaxError,
    LocatorError,
    "Locator syntax is invalid."
);

// ============================================================
// Action Exceptions
// ============================================================

pyo3::create_exception!(
    javagui,
    ActionError,
    JavaGuiError,
    "Base exception for action-related errors."
);

pyo3::create_exception!(
    javagui,
    ActionFailedError,
    ActionError,
    "Failed to perform the requested action."
);

pyo3::create_exception!(
    javagui,
    ActionTimeoutError,
    ActionError,
    "Action timed out waiting for condition."
);

pyo3::create_exception!(
    javagui,
    ActionNotSupportedError,
    ActionError,
    "Action is not supported for this element type."
);

// ============================================================
// Technology-Specific Exceptions
// ============================================================

pyo3::create_exception!(
    javagui,
    TechnologyError,
    JavaGuiError,
    "Base exception for technology-specific errors."
);

pyo3::create_exception!(
    javagui,
    ModeNotSupportedError,
    TechnologyError,
    "Operation not supported in current mode."
);

pyo3::create_exception!(
    javagui,
    RcpWorkbenchError,
    TechnologyError,
    "Eclipse RCP workbench operation failed."
);

pyo3::create_exception!(
    javagui,
    SwtShellError,
    TechnologyError,
    "SWT shell operation failed."
);
```

### 3. Rich Error Context

```rust
/// Error builder for creating detailed error messages
pub struct ErrorBuilder {
    error_type: ErrorType,
    message: String,
    context: HashMap<String, String>,
    suggestions: Vec<String>,
    related_errors: Vec<String>,
}

impl ErrorBuilder {
    pub fn element_not_found(locator: &str) -> Self {
        Self {
            error_type: ErrorType::ElementNotFound,
            message: format!("Element not found: {}", locator),
            context: HashMap::new(),
            suggestions: Vec::new(),
            related_errors: Vec::new(),
        }
    }

    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestions.push(suggestion.to_string());
        self
    }

    pub fn with_similar_elements(mut self, similar: &[SimilarElement]) -> Self {
        for elem in similar.iter().take(3) {
            self.suggestions.push(format!(
                "Did you mean: {} ({})",
                elem.locator, elem.class_name
            ));
        }
        self
    }

    pub fn with_searched_tree(mut self, tree_summary: &str) -> Self {
        self.context.insert("searched_tree".to_string(), tree_summary.to_string());
        self
    }

    pub fn build(self) -> PyErr {
        let mut message = self.message.clone();

        // Add context
        if !self.context.is_empty() {
            message.push_str("\n\nContext:");
            for (key, value) in &self.context {
                message.push_str(&format!("\n  {}: {}", key, value));
            }
        }

        // Add suggestions
        if !self.suggestions.is_empty() {
            message.push_str("\n\nSuggestions:");
            for suggestion in &self.suggestions {
                message.push_str(&format!("\n  - {}", suggestion));
            }
        }

        // Return appropriate exception type
        match self.error_type {
            ErrorType::ElementNotFound => ElementNotFoundError::new_err(message),
            ErrorType::ConnectionTimeout => ConnectionTimeoutError::new_err(message),
            ErrorType::ActionFailed => ActionFailedError::new_err(message),
            // ... other types
            _ => JavaGuiError::new_err(message),
        }
    }
}
```

### 4. Error Message Templates

```rust
/// Standardized error message templates
pub struct ErrorMessages;

impl ErrorMessages {
    pub fn element_not_found(locator: &str, mode: GuiMode) -> String {
        format!(
            "Element not found: '{}'\n\
             Mode: {:?}\n\
             \n\
             Troubleshooting:\n\
             1. Verify the element exists using 'Log UI Tree'\n\
             2. Check if the application window is visible\n\
             3. Wait for the element to appear using 'Wait Until Element Exists'\n\
             4. Verify the locator syntax is correct for {} mode",
            locator,
            mode,
            mode
        )
    }

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

    pub fn mode_not_supported(keyword: &str, current_mode: GuiMode, required_modes: &[GuiMode]) -> String {
        let required_str = required_modes.iter()
            .map(|m| format!("{:?}", m).to_lowercase())
            .collect::<Vec<_>>()
            .join(", ");

        format!(
            "Keyword '{}' is not available in {:?} mode.\n\
             This keyword requires: {}\n\
             \n\
             To use this keyword:\n\
             1. Connect to an application that supports {} mode, or\n\
             2. Use 'Set Mode' to switch to a supported mode",
            keyword,
            current_mode,
            required_str,
            required_str
        )
    }

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
}
```

### 5. Backwards Compatibility Aliases

```rust
/// Register both new and legacy exception names
pub fn register_exceptions(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // New unified exceptions
    m.add("JavaGuiError", py.get_type::<JavaGuiError>())?;
    m.add("ConnectionError", py.get_type::<ConnectionError>())?;
    m.add("ConnectionRefusedError", py.get_type::<ConnectionRefusedError>())?;
    m.add("ConnectionTimeoutError", py.get_type::<ConnectionTimeoutError>())?;
    m.add("NotConnectedError", py.get_type::<NotConnectedError>())?;
    m.add("ElementNotFoundError", py.get_type::<ElementNotFoundError>())?;
    m.add("MultipleElementsFoundError", py.get_type::<MultipleElementsFoundError>())?;
    m.add("ElementNotInteractableError", py.get_type::<ElementNotInteractableError>())?;
    m.add("StaleElementError", py.get_type::<StaleElementError>())?;
    m.add("LocatorParseError", py.get_type::<LocatorParseError>())?;
    m.add("ActionFailedError", py.get_type::<ActionFailedError>())?;
    m.add("ActionTimeoutError", py.get_type::<ActionTimeoutError>())?;
    m.add("ModeNotSupportedError", py.get_type::<ModeNotSupportedError>())?;
    m.add("RcpWorkbenchError", py.get_type::<RcpWorkbenchError>())?;
    m.add("SwtShellError", py.get_type::<SwtShellError>())?;

    // Legacy aliases for backwards compatibility
    m.add("SwingConnectionError", py.get_type::<ConnectionError>())?;  // Alias
    m.add("SwingTimeoutError", py.get_type::<ActionTimeoutError>())?;  // Alias

    Ok(())
}
```

### 6. Robot Framework Usage

```robot
*** Settings ***
Library    JavaGuiLibrary

*** Test Cases ***
Test Element Not Found Error
    [Documentation]    Demonstrates error handling for element not found
    Connect To Application    myapp
    ${error}=    Run Keyword And Expect Error    ElementNotFoundError:*
    ...    Click    name:nonexistent
    Should Contain    ${error}    Element not found
    Should Contain    ${error}    Suggestions

Test Connection Error Handling
    [Documentation]    Demonstrates connection error handling
    ${error}=    Run Keyword And Expect Error    ConnectionRefusedError:*
    ...    Connect To Application    myapp    localhost    9999
    Should Contain    ${error}    Connection refused
    Should Contain    ${error}    Troubleshooting

Test Mode Error
    [Documentation]    Demonstrates mode-specific error
    Connect To Application    swing_app
    ${error}=    Run Keyword And Expect Error    ModeNotSupportedError:*
    ...    Open Perspective    some.id
    Should Contain    ${error}    not available in swing mode

Test Timeout Error
    [Documentation]    Demonstrates timeout handling
    Connect To Application    myapp
    Set Timeout    1
    ${error}=    Run Keyword And Expect Error    ActionTimeoutError:*
    ...    Wait Until Element Exists    name:slow_element
    Should Contain    ${error}    Timeout after 1.0s

Test Exception Hierarchy
    [Documentation]    Test catching parent exceptions
    Connect To Application    myapp
    # Catching parent catches all child exceptions
    ${error}=    Run Keyword And Expect Error    ElementError:*
    ...    Click    name:nonexistent
    # More specific catch
    ${error}=    Run Keyword And Expect Error    ElementNotFoundError:*
    ...    Click    name:nonexistent
```

## Consequences

### Positive

1. **Clear Hierarchy**: Easy to catch specific or general errors
2. **Rich Context**: Error messages include debugging information
3. **Technology-Agnostic**: No "Swing" in exception names for unified library
4. **Actionable Messages**: Suggestions help users fix issues
5. **Backwards Compatible**: Legacy exception names still work
6. **Consistent Format**: All errors follow same structure

### Negative

1. **Migration Effort**: Tests catching old exception names may need updates
2. **Verbose Messages**: Longer error messages may clutter logs
3. **More Exception Types**: More classes to maintain

### Risks

1. **Message Maintenance**: Error templates need updating with library changes
2. **Performance**: Rich context collection may impact error path performance
3. **Breaking Changes**: Some exception renames may break existing handlers

## Alternatives Considered

### Alternative 1: Keep Technology-Specific Exceptions

Keep SwingError, SwtError, RcpError separate hierarchies.

**Rejected because**:
- Doesn't support unified library goal
- Users must handle different exceptions per technology
- More code to maintain

### Alternative 2: Single Generic Exception

Use one exception type with error code/category attribute.

**Rejected because**:
- Loses Python exception hierarchy benefits
- Harder to catch specific error types
- Less idiomatic Python

### Alternative 3: No Rich Context

Keep simple error messages without suggestions/context.

**Rejected because**:
- Users spend more time debugging
- Doesn't leverage library knowledge
- Poor developer experience

## Implementation Plan

1. **Phase 1**: Define new exception hierarchy (2 days)
2. **Phase 2**: Implement ErrorBuilder and templates (3 days)
3. **Phase 3**: Update all error sites to use new system (1 week)
4. **Phase 4**: Add backwards compatibility aliases (1 day)
5. **Phase 5**: Update tests and documentation (3 days)

## References

- [Python Exception Hierarchy](https://docs.python.org/3/library/exceptions.html)
- [PyO3 Exception Handling](https://pyo3.rs/v0.20.0/exception)
- [Robot Framework Error Handling](https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html#handling-errors)
- [Current Error Implementation](/src/error.rs)
- [Current Python Exceptions](/src/python/exceptions.rs)
