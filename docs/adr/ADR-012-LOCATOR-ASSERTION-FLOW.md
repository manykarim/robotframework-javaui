# ADR-012: Locator-Assertion Flow Architecture

| ADR ID | ADR-012 |
|--------|---------|
| Title | Locator-Assertion Flow Architecture |
| Status | Proposed |
| Date | 2026-01-20 |
| Authors | Architecture Team |
| Related | ADR-002 (Locator Syntax), ADR-005 (Error Handling), ADR-007 (Unified Keyword API) |

## Context

The library requires a well-defined end-to-end flow from user-specified locators to assertion verification. This flow spans multiple layers:

1. **Python Keyword Layer** - Robot Framework keywords with assertion parameters
2. **Retry Wrapper** - Timeout-based retry mechanism for transient failures
3. **PyO3 Bridge** - Rust-Python interop boundary
4. **Locator Parser (Pest)** - Rust-based parser for CSS/XPath-like locators
5. **RPC Client** - JSON-RPC communication with Java agent
6. **Java Agent** - Element resolution and property extraction
7. **AssertionEngine** - Value comparison and verification

### Current Implementation State

**Locator Parsing (Rust):**
- Pest-based grammar in `src/locator/grammar.pest`
- AST definitions in `src/locator/ast.rs`
- Parser implementation in `src/locator/parser.rs`
- Matcher evaluation in `src/locator/matcher.rs`

**RPC Protocol:**
- JSON-RPC 2.0 protocol in `src/protocol/mod.rs`
- TCP connection management in `src/python/swing_library.rs`
- Java agent at `agent/src/main/java/com/robotframework/swing/RpcServer.java`

**Current Gaps:**
- No formal AssertionEngine integration
- Retry logic embedded in individual keywords
- Inconsistent timeout handling
- No unified value type handling for assertions

### Decision Drivers

- Browser Library-style assertion pattern adoption (ADR-007)
- Unified error handling across all layers (ADR-005)
- Support for locator chaining syntax (ADR-002)
- Performance optimization through caching
- Clear debugging context in error messages

## Decision

We will implement a **Layered Locator-Assertion Architecture** with explicit boundaries and well-defined data flow.

### 1. End-to-End Flow Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                           Robot Framework Test Case                              │
│  Get Text    JFrame >> JPanel >> JButton[enabled=true]    ==    Expected Text   │
└─────────────────────────────────────────────────────────────────────────────────┘
                                         │
                                         ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         Python Keyword Layer (Rust/PyO3)                         │
│  get_text(locator: str, operator: Option<str>, expected: Option<str>,           │
│           message: Option<str>, timeout: Option<f64>) -> str                    │
│                                                                                  │
│  Responsibilities:                                                               │
│  - Parse keyword arguments                                                       │
│  - Determine if assertion mode (operator provided)                              │
│  - Configure timeout from library settings or parameter                         │
│  - Delegate to retry wrapper if assertion mode                                  │
└─────────────────────────────────────────────────────────────────────────────────┘
                                         │
                      ┌──────────────────┴──────────────────┐
                      │                                     │
                      ▼                                     ▼
        ┌─────────────────────────┐           ┌─────────────────────────┐
        │   Direct Value Return   │           │   Assertion Mode        │
        │   (no operator)         │           │   (operator provided)   │
        └─────────────────────────┘           └─────────────────────────┘
                      │                                     │
                      │                                     ▼
                      │           ┌───────────────────────────────────────────────┐
                      │           │              Retry Wrapper                     │
                      │           │  AssertionRetryContext {                       │
                      │           │      timeout: Duration,                        │
                      │           │      interval: Duration,                       │
                      │           │      start_time: Instant,                      │
                      │           │  }                                             │
                      │           │                                                │
                      │           │  loop {                                        │
                      │           │      match get_and_assert() {                  │
                      │           │          Ok(value) => return Ok(value),        │
                      │           │          Err(AssertionError) => retry,         │
                      │           │          Err(ElementNotFound) => retry,        │
                      │           │          Err(other) => return Err(other),      │
                      │           │      }                                         │
                      │           │      if elapsed > timeout { break; }           │
                      │           │      sleep(interval);                          │
                      │           │  }                                             │
                      │           └───────────────────────────────────────────────┘
                      │                                     │
                      └──────────────────┬──────────────────┘
                                         │
                                         ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              PyO3 Bridge Layer                                   │
│                                                                                  │
│  pub fn get_element_text(&self, locator: &str) -> PyResult<String>              │
│                                                                                  │
│  Responsibilities:                                                               │
│  - Convert Python strings to Rust &str                                          │
│  - Convert Rust errors to Python exceptions                                     │
│  - Manage GIL appropriately                                                     │
└─────────────────────────────────────────────────────────────────────────────────┘
                                         │
                                         ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         Locator Parser (Pest Grammar)                            │
│                                                                                  │
│  Input:  "JFrame >> JPanel >> JButton[enabled=true]"                            │
│                                                                                  │
│  Parsing Pipeline:                                                               │
│  1. Lexical analysis (Pest rules in grammar.pest)                               │
│  2. Build AST (Locator { selectors: Vec<ComplexSelector> })                     │
│  3. Return structured representation                                             │
│                                                                                  │
│  Output: Locator {                                                               │
│      selectors: [                                                                │
│          ComplexSelector {                                                       │
│              compounds: [                                                        │
│                  CompoundSelector { type: "JFrame", combinator: Cascaded },     │
│                  CompoundSelector { type: "JPanel", combinator: Cascaded },     │
│                  CompoundSelector {                                              │
│                      type: "JButton",                                            │
│                      attributes: [{ name: "enabled", value: true }],            │
│                      combinator: None                                            │
│                  }                                                                │
│              ]                                                                   │
│          }                                                                       │
│      ],                                                                          │
│      original: "JFrame >> JPanel >> JButton[enabled=true]",                     │
│      is_xpath: false                                                             │
│  }                                                                               │
└─────────────────────────────────────────────────────────────────────────────────┘
                                         │
                                         ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              RPC Client Layer                                    │
│                                                                                  │
│  JSON-RPC 2.0 Request:                                                           │
│  {                                                                               │
│      "jsonrpc": "2.0",                                                           │
│      "method": "getElementText",                                                 │
│      "params": {                                                                 │
│          "locator": {                                                            │
│              "type": "cascaded",                                                 │
│              "segments": [                                                       │
│                  { "type": "JFrame" },                                           │
│                  { "type": "JPanel" },                                           │
│                  { "type": "JButton", "enabled": true }                          │
│              ]                                                                   │
│          }                                                                       │
│      },                                                                          │
│      "id": 42                                                                    │
│  }                                                                               │
│                                                                                  │
│  Responsibilities:                                                               │
│  - Serialize locator AST to JSON                                                │
│  - Manage TCP connection and timeouts                                           │
│  - Handle JSON-RPC error responses                                              │
│  - Deserialize response to Rust types                                           │
└─────────────────────────────────────────────────────────────────────────────────┘
                                         │
                                    TCP/IP
                                         │
                                         ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                         Java Agent (ActionExecutor)                              │
│                                                                                  │
│  public String getElementText(LocatorChain locator) {                           │
│      Component element = resolveElement(locator);                               │
│      return extractText(element);                                               │
│  }                                                                               │
│                                                                                  │
│  Element Resolution Algorithm:                                                   │
│  1. Start from root (all top-level windows)                                     │
│  2. For each cascaded segment:                                                  │
│     a. Find matching children of current context                                │
│     b. Apply attribute filters                                                   │
│     c. Apply state filters (enabled, visible, etc.)                             │
│     d. Apply index selection if specified                                       │
│  3. Return final matching element or throw ElementNotFoundException             │
│                                                                                  │
│  JSON-RPC 2.0 Response:                                                          │
│  {                                                                               │
│      "jsonrpc": "2.0",                                                           │
│      "result": "Submit",                                                         │
│      "id": 42                                                                    │
│  }                                                                               │
│                                                                                  │
│  Or Error Response:                                                              │
│  {                                                                               │
│      "jsonrpc": "2.0",                                                           │
│      "error": {                                                                  │
│          "code": -32000,                                                         │
│          "message": "Element not found",                                         │
│          "data": { "locator": "...", "searched": 15 }                           │
│      },                                                                          │
│      "id": 42                                                                    │
│  }                                                                               │
└─────────────────────────────────────────────────────────────────────────────────┘
                                         │
                                         ▼
┌─────────────────────────────────────────────────────────────────────────────────┐
│                            AssertionEngine                                       │
│                                                                                  │
│  fn verify_assertion(                                                            │
│      actual: &str,                                                               │
│      operator: &str,                                                             │
│      expected: &str,                                                             │
│      message: Option<&str>,                                                      │
│  ) -> Result<String, AssertionError>                                            │
│                                                                                  │
│  Operator Evaluation:                                                            │
│  - "=="           => actual == expected                                         │
│  - "!="           => actual != expected                                         │
│  - "contains"     => actual.contains(expected)                                  │
│  - "not contains" => !actual.contains(expected)                                 │
│  - "matches"      => Regex::new(expected).is_match(actual)                      │
│  - "starts with"  => actual.starts_with(expected)                               │
│  - "ends with"    => actual.ends_with(expected)                                 │
│                                                                                  │
│  On Success: Return actual value                                                │
│  On Failure: Return AssertionError with context                                 │
└─────────────────────────────────────────────────────────────────────────────────┘
```

### 2. Locator Chaining Specification

#### 2.1 Cascaded Combinator (`>>`)

Browser Library-style parent-child context navigation:

```
JFrame >> JPanel >> JButton
```

**Semantics:**
- Each segment creates a new search context
- Search is performed within the context of the previous match
- Multiple matches at intermediate levels are allowed (first match is used as context)

#### 2.2 Supported Locator Syntax

| Syntax | Example | Description |
|--------|---------|-------------|
| Type selector | `JButton` | Match by component class |
| ID selector | `#submitBtn` | Shorthand for `[name='submitBtn']` |
| Class selector | `.primary` | Match by CSS class (if applicable) |
| Attribute filter | `[name='x']` | Exact attribute match |
| Attribute operators | `[text*='Save']` | Contains, starts-with, etc. |
| State pseudo-class | `:enabled` | Match by component state |
| Structural pseudo-class | `:first-child` | Match by position |
| Index filter | `[index=0]` or `:first` | Select specific element |
| Cascaded chain | `Parent >> Child` | Context-based search |
| Direct child | `Parent > Child` | Immediate child only |
| Descendant | `Parent Child` | Any descendant |
| XPath | `//JButton[@text='OK']` | XPath expression |

#### 2.3 Attribute Filter Syntax

```rust
// Attribute operators from grammar.pest
[attr]                    // Attribute exists
[attr='value']            // Exact match
[attr!='value']           // Not equal
[attr^='prefix']          // Starts with
[attr$='suffix']          // Ends with
[attr*='substring']       // Contains
[attr~='word']            // Word match (space-separated)
[attr|='prefix']          // Dash-separated prefix
[attr/='regex']           // Regex match

// Multiple attributes
[attr1='a'][attr2='b']    // AND combination
[attr1='a', attr2='b']    // Alternate AND syntax
```

#### 2.4 State Pseudo-Classes

```rust
// From src/locator/ast.rs PseudoSelector enum
:enabled                  // Component.isEnabled() == true
:disabled                 // Component.isEnabled() == false
:visible                  // Component.isVisible() == true
:hidden                   // Component.isVisible() == false
:showing                  // Component.isShowing() == true
:focused                  // Component has focus
:selected                 // For checkboxes, radio buttons, list items
:editable                 // For text fields
:readonly                 // Read-only state
```

#### 2.5 Structural Pseudo-Classes

```rust
// Position-based selection
:first-child              // First among siblings
:last-child               // Last among siblings
:nth-child(n)             // Nth child (1-indexed)
:nth-child(odd)           // Odd-positioned children
:nth-child(even)          // Even-positioned children
:nth-child(2n+1)          // Formula-based (an+b)
:only-child               // Only child of parent

// Type-based selection
:first-of-type            // First of this type among siblings
:last-of-type             // Last of this type
:nth-of-type(n)           // Nth of type
:only-of-type             // Only element of this type

// Convenience aliases
:first                    // Alias for :first-child
:last                     // Alias for :last-child
[index=0]                 // Alias for :nth-child(1)
```

### 3. Error Flow Architecture

```
┌─────────────────────────────────────────────────────────────────────────────────┐
│                              Error Origin Points                                 │
└─────────────────────────────────────────────────────────────────────────────────┘

                     ┌────────────────┐
                     │ Locator Parse  │
                     │    Error       │
                     └────────┬───────┘
                              │
                              ▼
              ┌─────────────────────────────────────┐
              │ LocatorParseError (Rust)            │
              │                                     │
              │ ParseError {                        │
              │   message: "Invalid syntax",        │
              │   kind: InvalidSelector,            │
              │   position: 15,                     │
              │   line: 1,                          │
              │   column: 16,                       │
              │   fragment: Some("JButton[")        │
              │ }                                   │
              └───────────────┬─────────────────────┘
                              │ PyO3 conversion
                              ▼
              ┌─────────────────────────────────────┐
              │ LocatorParseError (Python)          │
              │                                     │
              │ "Failed to parse locator:           │
              │  'JFrame >> JButton['              │
              │                                     │
              │  Parse error at line 1, column 16:  │
              │  Unexpected end of input            │
              │                                     │
              │  Valid examples:                    │
              │  - JButton[name='submit']           │
              │  - #submitButton                    │
              │  - //JButton[@text='OK']"           │
              └─────────────────────────────────────┘

                     ┌────────────────┐
                     │ Element Not    │
                     │   Found        │
                     └────────┬───────┘
                              │
                              ▼
              ┌─────────────────────────────────────┐
              │ Java Agent Response                 │
              │                                     │
              │ {                                   │
              │   "error": {                        │
              │     "code": -32000,                 │
              │     "message": "Element not found", │
              │     "data": {                       │
              │       "locator": "JButton#submit",  │
              │       "searchedCount": 47,          │
              │       "partialMatches": [           │
              │         "JButton[name='submit2']",  │
              │         "JButton[name='submitAll']" │
              │       ]                             │
              │     }                               │
              │   }                                 │
              │ }                                   │
              └───────────────┬─────────────────────┘
                              │
                              ▼
              ┌─────────────────────────────────────┐
              │ Retry Wrapper Behavior              │
              │                                     │
              │ if timeout_remaining > 0 {          │
              │   sleep(poll_interval);             │
              │   continue; // Retry element search │
              │ } else {                            │
              │   return ElementNotFoundError       │
              │ }                                   │
              └───────────────┬─────────────────────┘
                              │ Final error after timeout
                              ▼
              ┌─────────────────────────────────────┐
              │ ElementNotFoundError (Python)       │
              │                                     │
              │ "Element not found: 'JButton#submit'│
              │                                     │
              │  Searched 47 components in 5.0s     │
              │                                     │
              │  Similar elements found:            │
              │  - JButton[name='submit2']          │
              │  - JButton[name='submitAll']        │
              │                                     │
              │  Troubleshooting:                   │
              │  - Use 'Log UI Tree' to inspect     │
              │  - Check element visibility         │
              │  - Verify locator syntax"           │
              └─────────────────────────────────────┘

                     ┌────────────────┐
                     │  Assertion     │
                     │   Failed       │
                     └────────┬───────┘
                              │
                              ▼
              ┌─────────────────────────────────────┐
              │ AssertionEngine Result              │
              │                                     │
              │ AssertionError {                    │
              │   operator: "==",                   │
              │   expected: "Ready",                │
              │   actual: "Loading",                │
              │   locator: "JLabel#status"          │
              │ }                                   │
              └───────────────┬─────────────────────┘
                              │
                              ▼
              ┌─────────────────────────────────────┐
              │ Retry Wrapper Behavior              │
              │                                     │
              │ if timeout_remaining > 0 {          │
              │   sleep(poll_interval);             │
              │   continue; // Re-fetch and assert  │
              │ } else {                            │
              │   return AssertionFailedError       │
              │ }                                   │
              └───────────────┬─────────────────────┘
                              │ Final error after timeout
                              ▼
              ┌─────────────────────────────────────┐
              │ AssertionFailedError (Python)       │
              │                                     │
              │ "Assertion failed after 5.0s        │
              │                                     │
              │  Operator: ==                       │
              │  Expected: 'Ready'                  │
              │  Actual:   'Loading'                │
              │  Locator:  JLabel#status            │
              │                                     │
              │  Suggestions:                       │
              │  - Increase timeout if value changes│
              │  - Verify expected value is correct │
              │  - Check element is updating"       │
              └─────────────────────────────────────┘
```

### 4. Timeout and Retry Configuration

#### 4.1 Configuration Hierarchy

```rust
/// Library-level configuration (lowest precedence)
pub struct LibraryConfig {
    /// Default assertion timeout (seconds)
    pub assertion_timeout: f64,    // Default: 5.0

    /// Assertion retry interval (seconds)
    pub assertion_interval: f64,   // Default: 0.1

    /// Implicit wait for element existence (seconds)
    pub implicit_wait: f64,        // Default: 0.0

    /// RPC call timeout (seconds)
    pub rpc_timeout: f64,          // Default: 30.0
}

/// Keyword-level override (highest precedence)
#[pyo3(signature = (locator, operator=None, expected=None, message=None, timeout=None))]
pub fn get_text(
    &self,
    locator: &str,
    operator: Option<&str>,
    expected: Option<&str>,
    message: Option<&str>,
    timeout: Option<f64>,  // Overrides library default
) -> PyResult<String>
```

#### 4.2 Robot Framework Keywords

```python
*** Settings ***
Library    JavaGuiLibrary    assertion_timeout=10.0    assertion_interval=0.2

*** Keywords ***
Set Assertion Timeout
    [Documentation]    Set global timeout for assertion retries.
    [Arguments]    ${timeout}
    # Modifies LibraryConfig.assertion_timeout

Set Assertion Interval
    [Documentation]    Set polling interval for assertion retries.
    [Arguments]    ${interval}
    # Modifies LibraryConfig.assertion_interval

Set Implicit Wait
    [Documentation]    Set implicit wait for element existence.
    [Arguments]    ${timeout}
    # Modifies LibraryConfig.implicit_wait

*** Test Cases ***
Test With Custom Timeout
    # Use library default (10.0s)
    Get Text    JLabel#status    ==    Ready

    # Override timeout for single call
    Get Text    JLabel#slowLoader    ==    Complete    timeout=60.0

    # Change library default mid-test
    Set Assertion Timeout    30.0
    Get Text    JLabel#status    ==    Ready    # Now uses 30.0s
```

#### 4.3 Retry Decision Matrix

| Error Type | Retry? | Rationale |
|------------|--------|-----------|
| `ElementNotFoundError` | Yes | Element may appear after UI update |
| `AssertionError` | Yes | Value may change to expected |
| `StaleElementError` | Yes | Element reference may be refreshed |
| `LocatorParseError` | No | Syntax error, won't change |
| `ConnectionError` | No | Infrastructure failure |
| `ActionNotSupportedError` | No | Invalid operation |
| `RpcTimeoutError` | No | Agent unresponsive |

### 5. Value Type Handling

#### 5.1 Verification Functions by Return Type

| Get Keyword | Return Type | Rust Type | AssertionEngine Function |
|-------------|-------------|-----------|--------------------------|
| `Get Text` | `str` | `String` | `verify_assertion` |
| `Get Value` | `str` | `String` | `verify_assertion` |
| `Get Element Count` | `int` | `i64` | `float_str_verify_assertion` |
| `Get Element States` | `List[str]` | `Vec<String>` | `flag_verify_assertion` |
| `Get Property` | `Any` | `serde_json::Value` | Type-dependent dispatch |
| `Get Properties` | `Dict` | `HashMap<String, Value>` | `dict_verify_assertion` |
| `Get Table Data` | `List[List[str]]` | `Vec<Vec<String>>` | `list_verify_assertion` |
| `Get List Items` | `List[str]` | `Vec<String>` | `list_verify_assertion` |

#### 5.2 AssertionEngine Implementation

```rust
/// Core assertion engine with type-specific verification
pub struct AssertionEngine {
    /// Formatter for value normalization
    formatter: Box<dyn Formatter>,
}

impl AssertionEngine {
    /// String assertion (Get Text, Get Value)
    pub fn verify_assertion(
        &self,
        actual: &str,
        operator: &str,
        expected: &str,
        message: Option<&str>,
    ) -> Result<String, AssertionError> {
        let normalized_actual = self.formatter.normalize(actual);
        let normalized_expected = self.formatter.normalize(expected);

        let result = match operator {
            "==" | "equals" | "equal" | "eq" =>
                normalized_actual == normalized_expected,
            "!=" | "not equals" | "ne" =>
                normalized_actual != normalized_expected,
            "contains" | "has" =>
                normalized_actual.contains(&normalized_expected),
            "not contains" | "not has" =>
                !normalized_actual.contains(&normalized_expected),
            "starts with" | "startswith" =>
                normalized_actual.starts_with(&normalized_expected),
            "ends with" | "endswith" =>
                normalized_actual.ends_with(&normalized_expected),
            "matches" | "regex" => {
                let re = Regex::new(expected)?;
                re.is_match(&normalized_actual)
            }
            "not matches" => {
                let re = Regex::new(expected)?;
                !re.is_match(&normalized_actual)
            }
            _ => return Err(AssertionError::invalid_operator(operator)),
        };

        if result {
            Ok(actual.to_string())
        } else {
            Err(AssertionError::failed(operator, expected, actual, message))
        }
    }

    /// Numeric assertion (Get Element Count)
    pub fn float_str_verify_assertion(
        &self,
        actual: i64,
        operator: &str,
        expected: &str,
        message: Option<&str>,
    ) -> Result<i64, AssertionError> {
        let expected_num: i64 = expected.parse()
            .map_err(|_| AssertionError::invalid_expected_type("integer", expected))?;

        let result = match operator {
            "==" | "equals" => actual == expected_num,
            "!=" | "not equals" => actual != expected_num,
            ">" | "greater than" | "gt" => actual > expected_num,
            "<" | "less than" | "lt" => actual < expected_num,
            ">=" | "gte" => actual >= expected_num,
            "<=" | "lte" => actual <= expected_num,
            _ => return Err(AssertionError::invalid_operator(operator)),
        };

        if result {
            Ok(actual)
        } else {
            Err(AssertionError::failed(operator, expected, &actual.to_string(), message))
        }
    }

    /// Flag/state assertion (Get Element States)
    pub fn flag_verify_assertion(
        &self,
        actual: &[String],
        operator: &str,
        expected: &str,
        message: Option<&str>,
    ) -> Result<Vec<String>, AssertionError> {
        // Parse expected - can be single state or list
        let expected_states: Vec<String> = if expected.starts_with('[') {
            serde_json::from_str(expected)?
        } else {
            vec![expected.to_string()]
        };

        let result = match operator {
            "contains" | "has" =>
                expected_states.iter().all(|e| actual.contains(e)),
            "not contains" | "not has" =>
                expected_states.iter().all(|e| !actual.contains(e)),
            "==" | "equals" => {
                let mut actual_sorted = actual.to_vec();
                let mut expected_sorted = expected_states.clone();
                actual_sorted.sort();
                expected_sorted.sort();
                actual_sorted == expected_sorted
            }
            _ => return Err(AssertionError::invalid_operator(operator)),
        };

        if result {
            Ok(actual.to_vec())
        } else {
            Err(AssertionError::failed(
                operator,
                &format!("{:?}", expected_states),
                &format!("{:?}", actual),
                message,
            ))
        }
    }

    /// List assertion (Get Table Data, Get List Items)
    pub fn list_verify_assertion(
        &self,
        actual: &[Vec<String>],
        operator: &str,
        expected: &str,
        message: Option<&str>,
    ) -> Result<Vec<Vec<String>>, AssertionError> {
        let expected_list: Vec<Vec<String>> = serde_json::from_str(expected)?;

        let result = match operator {
            "==" | "equals" => actual == expected_list.as_slice(),
            "!=" | "not equals" => actual != expected_list.as_slice(),
            "contains" => actual.iter().any(|row| expected_list.contains(row)),
            "length ==" => actual.len() == expected_list.len(),
            _ => return Err(AssertionError::invalid_operator(operator)),
        };

        if result {
            Ok(actual.to_vec())
        } else {
            Err(AssertionError::failed(operator, expected, &format!("{:?}", actual), message))
        }
    }
}
```

### 6. Formatter Integration

#### 6.1 Formatter Trait

```rust
/// Trait for value normalization before comparison
pub trait Formatter: Send + Sync {
    /// Normalize a string value
    fn normalize(&self, value: &str) -> String;

    /// Get formatter name for logging
    fn name(&self) -> &'static str;
}

/// Default formatter - no modification
pub struct IdentityFormatter;

impl Formatter for IdentityFormatter {
    fn normalize(&self, value: &str) -> String {
        value.to_string()
    }

    fn name(&self) -> &'static str {
        "identity"
    }
}

/// Normalize whitespace (collapse multiple spaces to single)
pub struct NormalizeSpacesFormatter;

impl Formatter for NormalizeSpacesFormatter {
    fn normalize(&self, value: &str) -> String {
        value.split_whitespace().collect::<Vec<_>>().join(" ")
    }

    fn name(&self) -> &'static str {
        "normalize_spaces"
    }
}

/// Strip leading/trailing whitespace
pub struct StripFormatter;

impl Formatter for StripFormatter {
    fn normalize(&self, value: &str) -> String {
        value.trim().to_string()
    }

    fn name(&self) -> &'static str {
        "strip"
    }
}

/// Case-insensitive comparison
pub struct LowerCaseFormatter;

impl Formatter for LowerCaseFormatter {
    fn normalize(&self, value: &str) -> String {
        value.to_lowercase()
    }

    fn name(&self) -> &'static str {
        "lowercase"
    }
}

/// Composed formatter (chain multiple formatters)
pub struct ComposedFormatter {
    formatters: Vec<Box<dyn Formatter>>,
}

impl Formatter for ComposedFormatter {
    fn normalize(&self, value: &str) -> String {
        self.formatters.iter().fold(value.to_string(), |v, f| f.normalize(&v))
    }

    fn name(&self) -> &'static str {
        "composed"
    }
}
```

#### 6.2 Python Keyword for Formatter Selection

```python
*** Settings ***
Library    JavaGuiLibrary

*** Test Cases ***
Test With Formatter
    # Default (identity formatter)
    Get Text    JLabel#message    ==    Hello World

    # Normalize spaces (collapse whitespace)
    Get Text    JLabel#formatted    ==    Hello World    formatter=normalize_spaces

    # Strip whitespace
    Get Text    JLabel#padded    ==    Content    formatter=strip

    # Case insensitive
    Get Text    JLabel#title    ==    my title    formatter=lowercase

    # Multiple formatters
    Get Text    JLabel#complex    ==    hello world    formatter=['strip', 'normalize_spaces', 'lowercase']
```

### 7. Implementation Roadmap

#### Phase 1: Core AssertionEngine (Week 1)

1. Create `src/assertion/mod.rs` with AssertionEngine struct
2. Implement `verify_assertion` for string comparisons
3. Implement `float_str_verify_assertion` for numeric comparisons
4. Define AssertionError type with rich context

#### Phase 2: Retry Wrapper (Week 1-2)

1. Create `src/assertion/retry.rs` with RetryContext
2. Implement timeout and interval configuration
3. Add retryable error classification
4. Integrate with existing keyword implementations

#### Phase 3: Formatter System (Week 2)

1. Define Formatter trait in `src/assertion/formatter.rs`
2. Implement standard formatters (strip, normalize_spaces, lowercase)
3. Add ComposedFormatter for chaining
4. Integrate formatter selection into keywords

#### Phase 4: Enhanced Error Messages (Week 2-3)

1. Update ErrorBuilder with assertion-specific context
2. Add similar element suggestions on ElementNotFound
3. Include locator chain debugging information
4. Add timeout/retry count to timeout errors

#### Phase 5: Flag and List Assertions (Week 3)

1. Implement `flag_verify_assertion` for states
2. Implement `list_verify_assertion` for table/list data
3. Implement `dict_verify_assertion` for properties
4. Add type coercion for mixed-type comparisons

#### Phase 6: Documentation and Testing (Week 3-4)

1. Update keyword documentation with assertion examples
2. Add unit tests for all assertion functions
3. Add integration tests for retry behavior
4. Create migration guide from old wait/assert keywords

## Consequences

### Positive

1. **Unified Flow**: Clear data path from keyword to verification
2. **Retry Resilience**: Automatic handling of transient failures
3. **Rich Errors**: Debugging context in all error messages
4. **Type Safety**: Proper handling of different value types
5. **Extensibility**: Formatter system allows custom normalization
6. **Browser Library Alignment**: Familiar patterns for modern RF users

### Negative

1. **Complexity**: Multiple layers add cognitive overhead
2. **Performance**: Retry adds latency to passing assertions
3. **Memory**: Error context collection uses memory
4. **Migration**: Existing tests may need adjustment

### Risks

1. **Retry Overhead**: Unnecessary retries on deterministic failures
2. **Timeout Tuning**: Users may need to adjust defaults
3. **Format Ambiguity**: Complex formatters may hide actual failures
4. **State Dependencies**: Assertions may pass/fail due to timing

## References

- [ADR-002: Locator Syntax Strategy](./ADR-002-locator-syntax-strategy.md)
- [ADR-005: Error Handling Strategy](./ADR-005-error-handling-strategy.md)
- [ADR-007: Unified Keyword API Design](./ADR-007-UNIFIED-KEYWORD-API.md)
- [Browser Library Assertion Engine](https://robotframework-browser.org/#assertions)
- [AssertionEngine Package](https://github.com/MarketSquare/AssertionEngine)
- [Pest Parser Documentation](https://pest.rs/)
- [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
