# ADR-009: Implementation and Migration Plan

| ADR ID | ADR-009 |
|--------|---------|
| Title | Implementation and Migration Plan |
| Status | Proposed |
| Date | 2026-01-19 |
| Authors | Architecture Team |
| Depends On | ADR-001, ADR-002, ADR-003, ADR-004, ADR-005, ADR-006 |

## Context

This ADR defines the comprehensive implementation plan and migration strategy for the robotframework-swing keyword API modernization project. The plan coordinates the implementation of all architectural decisions (ADR-001 through ADR-006) into a phased, risk-managed approach.

### Project Scope

The modernization project aims to:

1. Unify three separate library implementations (Swing, SWT, RCP) into a single codebase
2. Standardize keyword naming conventions across all technologies
3. Implement a unified locator syntax with automatic normalization
4. Create a robust error handling system with actionable messages
5. Provide a smooth migration path for existing users
6. Improve overall code quality, maintainability, and performance

### Current State Analysis

| Component | Current | Target | Gap |
|-----------|---------|--------|-----|
| SwingLibrary keywords | ~55 | ~45 unified + aliases | Refactor to traits |
| SwtLibrary keywords | ~55 | ~45 unified + aliases | Refactor to traits |
| RcpLibrary keywords | ~83 | ~73 unified + aliases | Refactor to traits |
| Code duplication | ~70% | <10% | Shared implementations |
| Test coverage | ~65% | 90%+ | Add unit/integration tests |
| Locator formats | 3 disparate | 1 unified | Normalization layer |
| Exception hierarchy | Swing-centric | Technology-agnostic | New hierarchy |

### Decision Drivers

- Minimize disruption to existing users
- Maintain backwards compatibility throughout transition
- Enable incremental delivery of value
- Ensure high quality through comprehensive testing
- Support parallel development with clear interfaces
- Enable rollback at each phase if issues arise

## Decision

We will implement a **Four-Phase Delivery Strategy** with clear milestones, quality gates, and rollback procedures.

---

## Phase 1: Foundation (Weeks 1-4)

### Objective
Establish the core infrastructure that all other components depend on.

### 1.1 Locator Chain Parser (Week 1-2)

**Deliverables:**
- `src/locator/unified.rs` - Unified locator normalizer
- `src/locator/chain.rs` - Locator chain parser with combinator support
- `src/locator/filter.rs` - Pseudo-class and filter implementation

**Technical Tasks:**

```rust
// Core components to implement
pub struct LocatorNormalizer {
    mode: GuiMode,
    cache: LruCache<String, NormalizedLocator>,
}

pub enum NormalizedLocator {
    Type(TypeSelector),
    Attribute(AttributeSelector),
    Compound(CompoundSelector),
    XPath(XPathSelector),
    TechSpecific(TechSpecificSelector),
    Chain(Vec<LocatorStep>),  // NEW: Chained locators
}

pub struct LocatorStep {
    selector: NormalizedLocator,
    combinator: Combinator,
    filters: Vec<Filter>,
}

pub enum Combinator {
    Descendant,     // space
    Child,          // >
    Adjacent,       // +
    Sibling,        // ~
}

pub enum Filter {
    Index(i32),          // :nth(2)
    First,               // :first
    Last,                // :last
    Visible,             // :visible
    Enabled,             // :enabled
    Contains(String),    // :contains('text')
}
```

**Acceptance Criteria:**
- [ ] All existing locator formats parse correctly (100% backward compatible)
- [ ] CSS-style chained locators work: `Panel > Button[name='ok']`
- [ ] Pseudo-class filters work: `Button:visible:enabled`
- [ ] XPath locators pass through correctly
- [ ] LRU cache improves repeated lookups by 10x
- [ ] Unit tests cover 95%+ of locator module

**Test Cases:**
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_css_chain_parsing() {
        let normalizer = LocatorNormalizer::new(GuiMode::Swing);
        let result = normalizer.normalize("Panel > Button[name='ok']").unwrap();
        assert!(matches!(result, NormalizedLocator::Chain(_)));
    }

    #[test]
    fn test_legacy_prefix_compatibility() {
        let normalizer = LocatorNormalizer::new(GuiMode::Swt);
        let result = normalizer.normalize("name:submitButton").unwrap();
        assert!(matches!(result, NormalizedLocator::Attribute(_)));
    }

    #[test]
    fn test_filter_parsing() {
        let normalizer = LocatorNormalizer::new(GuiMode::Auto);
        let result = normalizer.normalize("Button:visible:first").unwrap();
        // Verify filters are extracted
    }
}
```

### 1.2 Assertion Engine Core (Week 2-3)

**Deliverables:**
- `src/core/assertion.rs` - Assertion engine with fluent API
- `src/core/matchers.rs` - Built-in matchers (equals, contains, regex, etc.)
- `src/core/wait.rs` - Wait utilities with configurable polling

**Technical Design:**

```rust
/// Assertion engine for element verification
pub struct AssertionEngine {
    timeout: Duration,
    poll_interval: Duration,
    soft_assertions: Vec<AssertionResult>,
}

impl AssertionEngine {
    /// Assert element text matches expected value
    pub fn assert_text(&self, element: &dyn GuiElement, matcher: &dyn Matcher) -> PyResult<()>;

    /// Assert element property value
    pub fn assert_property(&self, element: &dyn GuiElement, property: &str, matcher: &dyn Matcher) -> PyResult<()>;

    /// Wait until assertion passes
    pub fn wait_until(&self, condition: impl Fn() -> bool) -> PyResult<()>;

    /// Soft assertion (collect failures, report at end)
    pub fn soft_assert(&mut self, assertion: impl Fn() -> AssertionResult);
}

/// Matcher trait for flexible assertions
pub trait Matcher: Send + Sync {
    fn matches(&self, actual: &str) -> MatchResult;
    fn describe(&self) -> String;
    fn describe_mismatch(&self, actual: &str) -> String;
}

/// Built-in matchers
pub struct EqualsMatcher { expected: String }
pub struct ContainsMatcher { substring: String }
pub struct RegexMatcher { pattern: Regex }
pub struct StartsWithMatcher { prefix: String }
pub struct EndsWithMatcher { suffix: String }
```

**Robot Framework Integration:**

```robot
*** Test Cases ***
Test With Integrated Assertions
    # Direct assertion (fails immediately)
    Click    name:button
    Get Text    name:result    ==    Success
    Get Text    name:result    contains    Succ
    Get Text    name:result    matches    ^Success.*$

    # Soft assertions (collects failures)
    Start Soft Assertions
    Get Text    name:field1    ==    expected1
    Get Text    name:field2    ==    expected2
    Get Text    name:field3    ==    expected3
    End Soft Assertions    # Reports all failures together
```

**Acceptance Criteria:**
- [ ] `Get Text` keyword supports inline assertions: `Get Text    locator    ==    expected`
- [ ] Wait utilities respect configured timeout and poll interval
- [ ] Soft assertions collect multiple failures before reporting
- [ ] Custom matchers can be registered
- [ ] Clear error messages show expected vs actual

### 1.3 Session Management Refactor (Week 3)

**Deliverables:**
- `src/core/session.rs` - Session management with proper lifecycle
- `src/connection/pool.rs` - Connection pooling for multiple apps

**Technical Design:**

```rust
/// Session manager for connection lifecycle
pub struct SessionManager {
    sessions: HashMap<String, Session>,
    active_session: Option<String>,
    config: SessionConfig,
}

pub struct Session {
    id: String,
    connection: ConnectionState,
    mode: GuiMode,
    element_cache: ElementCache,
    created_at: Instant,
    last_activity: Instant,
}

pub struct SessionConfig {
    max_sessions: usize,
    session_timeout: Duration,
    auto_reconnect: bool,
    connection_pool_size: usize,
}

impl SessionManager {
    /// Create new session with auto-detection
    pub fn create_session(&mut self, app: &str, host: &str, port: u16) -> PyResult<String>;

    /// Switch active session
    pub fn switch_session(&mut self, session_id: &str) -> PyResult<()>;

    /// Get current active session
    pub fn active(&self) -> PyResult<&Session>;

    /// Close session and cleanup
    pub fn close_session(&mut self, session_id: &str) -> PyResult<()>;
}
```

**Acceptance Criteria:**
- [ ] Multiple application sessions can be managed concurrently
- [ ] Session switching is thread-safe
- [ ] Automatic reconnection on connection loss (configurable)
- [ ] Session timeout and cleanup
- [ ] Element cache is session-scoped

### 1.4 Base Keyword Dispatcher (Week 4)

**Deliverables:**
- `src/core/dispatcher.rs` - Keyword dispatch with mode awareness
- `src/core/traits.rs` - Core traits as defined in ADR-001

**Technical Design:**

```rust
/// Core traits for element operations (from ADR-001)
pub trait ElementOperations {
    fn find_element(&self, locator: &str) -> PyResult<Box<dyn GuiElement>>;
    fn find_elements(&self, locator: &str) -> PyResult<Vec<Box<dyn GuiElement>>>;
    fn click(&self, locator: &str) -> PyResult<()>;
    fn double_click(&self, locator: &str) -> PyResult<()>;
    fn right_click(&self, locator: &str) -> PyResult<()>;
    fn input_text(&self, locator: &str, text: &str, clear: bool) -> PyResult<()>;
    fn clear_text(&self, locator: &str) -> PyResult<()>;
    fn get_text(&self, locator: &str) -> PyResult<String>;
}

/// Keyword dispatcher routes calls to appropriate backend
pub struct KeywordDispatcher {
    mode: GuiMode,
    swing_backend: Option<SwingBackend>,
    swt_backend: Option<SwtBackend>,
    session: Arc<RwLock<SessionManager>>,
    normalizer: LocatorNormalizer,
    assertion: AssertionEngine,
}

impl KeywordDispatcher {
    /// Dispatch keyword to appropriate backend based on mode
    pub fn dispatch<T>(&self, keyword: &str, args: Args) -> PyResult<T>
    where
        T: FromPyObject,
    {
        match self.mode {
            GuiMode::Swing => self.swing_backend.as_ref()
                .ok_or_else(|| error("Swing backend not available"))?
                .execute(keyword, args),
            GuiMode::Swt | GuiMode::Rcp => self.swt_backend.as_ref()
                .ok_or_else(|| error("SWT backend not available"))?
                .execute(keyword, args),
            GuiMode::Auto => Err(error("Mode not determined. Connect to application first.")),
            GuiMode::Unknown => Err(error("Unknown technology mode")),
        }
    }
}
```

**Acceptance Criteria:**
- [ ] All keywords route through dispatcher
- [ ] Mode-specific keywords validate mode before execution
- [ ] Locator normalization happens transparently
- [ ] Assertion integration works seamlessly
- [ ] Error handling follows ADR-005 guidelines

### Phase 1 Quality Gate

| Metric | Target | Measurement |
|--------|--------|-------------|
| Unit test coverage | 90%+ | `cargo tarpaulin` |
| All existing tests pass | 100% | CI pipeline |
| Performance regression | <5% | Benchmark suite |
| Documentation | Complete | Generated docs |
| No new compiler warnings | 0 warnings | `cargo build` |

**Phase 1 Rollback Procedure:**
1. Revert commits to pre-Phase 1 state
2. No user-facing changes to revert
3. Internal refactoring only

---

## Phase 2: Core Keywords (Weeks 5-8)

### Objective
Implement the unified keyword API with full backward compatibility.

### 2.1 Action Keywords (Week 5-6)

**Deliverables:**
- 5 core action keywords with unified implementation
- Backward-compatible aliases for all variations

| Unified Keyword | Aliases | Implementation |
|-----------------|---------|----------------|
| `Click` | `Click Element`, `Click Widget` | `ElementOperations::click()` |
| `Double Click` | `Double Click Element`, `Double Click Widget` | `ElementOperations::double_click()` |
| `Right Click` | `Right Click Element`, `Right Click Widget`, `Context Click` | `ElementOperations::right_click()` |
| `Input Text` | `Type Text` (with clear), `Enter Text` | `ElementOperations::input_text()` |
| `Clear Text` | `Clear Field`, `Clear Element` | `ElementOperations::clear_text()` |

**Implementation Pattern:**

```rust
/// Macro for unified keyword with aliases (from ADR-003)
macro_rules! unified_keyword {
    (
        primary: $primary:ident,
        robot_name: $robot_name:literal,
        aliases: [$($alias:ident => $alias_name:literal),*],
        impl: $impl_fn:expr
    ) => {
        #[pyo3(name = $robot_name)]
        pub fn $primary(&self, locator: &str) -> PyResult<()> {
            self.dispatcher.dispatch("click", (locator,))
        }

        $(
            #[pyo3(name = $alias_name)]
            pub fn $alias(&self, locator: &str) -> PyResult<()> {
                #[cfg(feature = "deprecation_warnings")]
                warn_deprecated($alias_name, $robot_name);

                self.dispatcher.dispatch("click", (locator,))
            }
        )*
    };
}

#[pymethods]
impl JavaGuiLibrary {
    unified_keyword!(
        primary: click,
        robot_name: "Click",
        aliases: [
            click_element => "Click Element",
            click_widget => "Click Widget"
        ],
        impl: |d: &KeywordDispatcher, loc: &str| d.element_ops().click(loc)
    );
}
```

**Unit Tests per Keyword:**

```rust
#[cfg(test)]
mod click_tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_click_with_name_locator() {
        let mut mock = MockElementOperations::new();
        mock.expect_click()
            .with(eq("name:button"))
            .times(1)
            .returning(|_| Ok(()));

        let lib = create_test_library(mock);
        assert!(lib.click("name:button").is_ok());
    }

    #[test]
    fn test_click_element_alias() {
        let lib = create_test_library_with_mock();
        // Verify Click Element calls the same implementation
        assert!(lib.click_element("name:button").is_ok());
    }

    #[test]
    fn test_click_not_found_error() {
        let mut mock = MockElementOperations::new();
        mock.expect_click()
            .returning(|_| Err(ElementNotFoundError::new_err("Element not found")));

        let lib = create_test_library(mock);
        let result = lib.click("name:nonexistent");
        assert!(result.is_err());
        // Verify error message format
    }
}
```

### 2.2 Get Keywords with Assertion Support (Week 6-7)

**Deliverables:**
- 6 getter keywords with optional assertion parameter
- Assertion matchers integrated

| Unified Keyword | Return Type | Assertion Support |
|-----------------|-------------|-------------------|
| `Get Text` | `str` | `==`, `contains`, `matches`, `starts_with`, `ends_with` |
| `Get Property` | `Any` | `==`, `is_true`, `is_false`, `is_none` |
| `Get Element Count` | `int` | `==`, `>`, `<`, `>=`, `<=` |
| `Get Selected Value` | `str` | `==`, `contains`, `matches` |
| `Get Table Cell Value` | `str` | `==`, `contains`, `matches` |
| `Get List Items` | `List[str]` | `contains`, `length` |

**Implementation with Assertion:**

```rust
/// Get Text keyword with optional assertion
///
/// Arguments:
///     locator: Element locator
///     assertion: Optional assertion operator (==, contains, matches, etc.)
///     expected: Expected value for assertion
///     timeout: Timeout for wait (if assertion provided)
///
/// Returns:
///     Element text if no assertion, None if assertion passes
///
/// Raises:
///     ElementNotFoundError: If element not found
///     AssertionError: If assertion fails
#[pyo3(name = "Get Text", signature = (locator, assertion=None, expected=None, timeout=None))]
pub fn get_text(
    &self,
    locator: &str,
    assertion: Option<&str>,
    expected: Option<&str>,
    timeout: Option<f64>,
) -> PyResult<Option<String>> {
    let text = self.dispatcher.get_text(locator)?;

    match (assertion, expected) {
        (Some(op), Some(exp)) => {
            let matcher = self.create_matcher(op, exp)?;

            if let Some(t) = timeout {
                // Wait for assertion to pass
                self.assertion.wait_until_matches(&text, &matcher, Duration::from_secs_f64(t))?;
            } else {
                // Immediate assertion
                self.assertion.assert_matches(&text, &matcher)?;
            }
            Ok(None)
        }
        _ => Ok(Some(text)),
    }
}
```

**Robot Framework Usage:**

```robot
*** Test Cases ***
Test Get Text Without Assertion
    ${text}=    Get Text    name:label
    Should Be Equal    ${text}    Expected Value

Test Get Text With Inline Assertion
    Get Text    name:label    ==    Expected Value
    Get Text    name:label    contains    Expected
    Get Text    name:label    matches    ^Expected.*$

Test Get Text With Wait
    Get Text    name:result    ==    Success    timeout=10

Test Get Property
    Get Property    name:button    enabled    ==    true
    Get Property    name:checkbox    selected    is_true
```

### 2.3 Selection Keywords (Week 7)

**Deliverables:**
- Table, Tree, List, Combobox selection keywords
- Unified naming with technology-specific handling

| Unified Keyword | Description | Mode Support |
|-----------------|-------------|--------------|
| `Select Table Row` | Select row by index | All |
| `Select Table Cell` | Select specific cell | All |
| `Select Tree Node` | Select tree node by path | All |
| `Expand Tree Node` | Expand tree node | All |
| `Collapse Tree Node` | Collapse tree node | All |
| `Select From Combobox` | Select combobox item | All |
| `Select From List` | Select list item | All |
| `Check` | Check checkbox/toggle | All |
| `Uncheck` | Uncheck checkbox/toggle | All |

### 2.4 Integration Tests (Week 8)

**Test Applications:**
- Swing test app: `tests/apps/swing/swing-test-app-1.0.0.jar`
- SWT test app: `tests/apps/swt/swt-test-app.jar` (create if needed)
- RCP test app: `tests/apps/rcp/rcp-test-app/` (Eclipse product)

**Integration Test Suite:**

```robot
*** Settings ***
Suite Setup       Start Test Applications
Suite Teardown    Stop Test Applications
Library           JavaGuiLibrary
Resource          common.resource

*** Test Cases ***
# Action Keywords
Test Click In Swing Mode
    [Tags]    swing    action
    Switch To Swing Application
    Click    name:testButton
    Get Text    name:result    ==    Button clicked

Test Click In SWT Mode
    [Tags]    swt    action
    Switch To SWT Application
    Click    name:testButton
    Get Text    name:result    ==    Button clicked

# Locator Tests
Test CSS Locator Chain
    [Tags]    locator
    Click    Panel[name='main'] > Button[text='Submit']

Test Pseudo-Class Filter
    [Tags]    locator
    Click    Button:visible:first

# Assertion Tests
Test Inline Assertion Pass
    [Tags]    assertion
    Get Text    name:label    ==    Expected

Test Inline Assertion Fail
    [Tags]    assertion
    Run Keyword And Expect Error    AssertionError:*
    ...    Get Text    name:label    ==    Wrong

# Error Handling Tests
Test Element Not Found Error
    [Tags]    error
    Run Keyword And Expect Error    ElementNotFoundError:*
    ...    Click    name:nonexistent
```

### Phase 2 Quality Gate

| Metric | Target | Measurement |
|--------|--------|-------------|
| Unit test coverage | 90%+ | `cargo tarpaulin` |
| Integration test pass rate | 100% | Robot Framework tests |
| All legacy keywords work | 100% | Backward compatibility tests |
| Deprecation warnings fire | Verified | Log inspection |
| Performance | No regression | Benchmark comparison |

**Phase 2 Rollback Procedure:**
1. Disable new keywords via feature flag
2. Keep legacy implementations active
3. User-facing impact: None (aliases still work)

---

## Phase 3: Advanced Features (Weeks 9-11)

### Objective
Implement advanced locator features, specialized component handling, and debugging tools.

### 3.1 Locator Chaining and Filtering (Week 9)

**Advanced Locator Features:**

```robot
*** Test Cases ***
Test Chained Locator
    # Find button inside specific panel
    Click    Panel[name='toolbar'] > Button[text='Save']

Test Descendant Locator
    # Find button anywhere inside panel
    Click    Panel[name='main'] Button[name='submit']

Test Index Filter
    # Click second button
    Click    Button:nth(2)

Test Combined Filters
    # Click first visible enabled button
    Click    Button:visible:enabled:first

Test Contains Filter
    # Click button containing text
    Click    Button:contains('Save')

Test Parent Traversal
    # Find parent panel of button
    ${panel}=    Find Element    Button[name='ok'] << Panel
```

**Implementation:**

```rust
pub struct LocatorChain {
    steps: Vec<LocatorStep>,
}

impl LocatorChain {
    pub fn evaluate(&self, root: &dyn GuiElement) -> PyResult<Vec<Box<dyn GuiElement>>> {
        let mut current = vec![root.clone_box()];

        for step in &self.steps {
            let mut next = Vec::new();
            for element in &current {
                let matches = self.find_matches(element, &step.selector, &step.combinator)?;
                let filtered = self.apply_filters(&matches, &step.filters)?;
                next.extend(filtered);
            }
            current = next;
        }

        Ok(current)
    }
}
```

### 3.2 Table/Tree/List Specialized Handling (Week 9-10)

**Table Keywords:**

```robot
*** Test Cases ***
Test Table Operations
    # Get cell value
    ${value}=    Get Table Cell Value    name:dataTable    2    3

    # Select with verification
    Select Table Row    name:dataTable    5
    Get Table Cell Value    name:dataTable    5    0    ==    Expected

    # Get entire row
    @{row}=    Get Table Row    name:dataTable    3

    # Get column values
    @{column}=    Get Table Column    name:dataTable    Name

    # Find row by content
    ${row_index}=    Find Table Row    name:dataTable    column=Name    value=John

    # Sort table
    Click Table Header    name:dataTable    Name
```

**Tree Keywords:**

```robot
*** Test Cases ***
Test Tree Operations
    # Expand to node
    Expand Tree Path    name:fileTree    Root/Folder1/SubFolder

    # Select node
    Select Tree Node    name:fileTree    Root/Folder1/File.txt

    # Get children
    @{children}=    Get Tree Node Children    name:fileTree    Root/Folder1

    # Check node state
    Tree Node Should Be Expanded    name:fileTree    Root/Folder1
    Tree Node Should Be Selected    name:fileTree    Root/Folder1/File.txt
```

### 3.3 Screenshot Enhancements (Week 10)

**Deliverables:**
- `src/core/screenshot.rs` - Screenshot capture and comparison

**Features:**

```robot
*** Test Cases ***
Test Screenshot Features
    # Capture full window
    Take Screenshot    full_window.png

    # Capture specific element
    Take Element Screenshot    name:chart    chart.png

    # Capture with highlighting
    Highlight Element    name:button
    Take Screenshot    highlighted.png

    # Visual comparison
    ${diff}=    Compare Screenshots    expected.png    actual.png    threshold=0.98
    Should Be True    ${diff} >= 0.98
```

### 3.4 UI Tree Introspection (Week 10-11)

**Deliverables:**
- `src/core/introspection.rs` - UI tree analysis tools

**Features:**

```robot
*** Test Cases ***
Test UI Introspection
    # Log entire UI tree
    Log UI Tree

    # Log tree starting from element
    Log UI Tree    name:mainPanel    depth=3

    # Get element hierarchy
    @{ancestors}=    Get Element Ancestors    name:button

    # Find elements by property
    @{buttons}=    Find Elements By Property    type=Button    visible=true

    # Analyze component
    ${info}=    Get Element Info    name:table
    Log    Component type: ${info}[type]
    Log    Bounds: ${info}[bounds]
    Log    Properties: ${info}[properties]
```

### Phase 3 Quality Gate

| Metric | Target | Measurement |
|--------|--------|-------------|
| Advanced locator tests | 100% pass | Integration tests |
| Specialized component tests | 100% pass | Component-specific tests |
| Screenshot functionality | Verified | Visual tests |
| Performance | <10% overhead | Benchmarks |

---

## Phase 4: Migration Support (Weeks 12-14)

### Objective
Provide comprehensive migration support for existing users.

### 4.1 Deprecation Warnings (Week 12)

**Implementation (from ADR-006):**

```rust
/// Deprecation warning with configurable severity
pub fn warn_deprecated(old_name: &str, new_name: &str, severity: DeprecationSeverity) {
    Python::with_gil(|py| {
        if let Ok(warnings) = py.import("warnings") {
            let message = match severity {
                DeprecationSeverity::Soft => format!(
                    "Keyword '{}' is deprecated. Consider using '{}' instead.",
                    old_name, new_name
                ),
                DeprecationSeverity::Warning => format!(
                    "Keyword '{}' is deprecated and will be removed in version 3.0. \
                     Use '{}' instead. See: https://docs.example.com/migration",
                    old_name, new_name
                ),
                DeprecationSeverity::PendingRemoval => format!(
                    "DEPRECATED: '{}' will be REMOVED in the next release! \
                     Migrate to '{}' immediately.",
                    old_name, new_name
                ),
            };

            let _ = warnings.call_method1(
                "warn",
                (message, py.get_type::<pyo3::exceptions::PyDeprecationWarning>(), 2)
            );
        }
    });
}
```

### 4.2 Keyword Aliases (Week 12)

**Complete Alias Mapping:**

```rust
pub const KEYWORD_ALIASES: &[KeywordAlias] = &[
    // SWT -> Unified
    KeywordAlias { old: "Click Widget", new: "Click", status: Deprecated },
    KeywordAlias { old: "Double Click Widget", new: "Double Click", status: Deprecated },
    KeywordAlias { old: "Right Click Widget", new: "Right Click", status: Deprecated },
    KeywordAlias { old: "Find Widget", new: "Find Element", status: Deprecated },
    KeywordAlias { old: "Find Widgets", new: "Find Elements", status: Deprecated },
    KeywordAlias { old: "Wait Until Widget Exists", new: "Wait Until Element Exists", status: Deprecated },
    KeywordAlias { old: "Widget Should Be Visible", new: "Element Should Be Visible", status: Deprecated },
    KeywordAlias { old: "Widget Should Be Enabled", new: "Element Should Be Enabled", status: Deprecated },
    KeywordAlias { old: "Widget Text Should Be", new: "Element Text Should Be", status: Deprecated },
    KeywordAlias { old: "Check Button", new: "Check", status: Deprecated },
    KeywordAlias { old: "Uncheck Button", new: "Uncheck", status: Deprecated },
    KeywordAlias { old: "Expand Tree Item", new: "Expand Tree Node", status: Deprecated },
    KeywordAlias { old: "Collapse Tree Item", new: "Collapse Tree Node", status: Deprecated },
    KeywordAlias { old: "Select Tree Item", new: "Select Tree Node", status: Deprecated },
    KeywordAlias { old: "Get Table Cell", new: "Get Table Cell Value", status: Deprecated },

    // Swing variations (keep as active aliases)
    KeywordAlias { old: "Click Element", new: "Click", status: Active },
    KeywordAlias { old: "Check Checkbox", new: "Check", status: Active },
    KeywordAlias { old: "Uncheck Checkbox", new: "Uncheck", status: Active },
    KeywordAlias { old: "Select Radio Button", new: "Select Radio", status: Active },
];
```

### 4.3 Migration Guide Documentation (Week 13)

**Deliverables:**
- `docs/migration/MIGRATION-GUIDE.md` - Comprehensive migration guide
- `docs/migration/KEYWORD-MAPPING.md` - Complete keyword mapping table
- `docs/migration/FAQ.md` - Frequently asked questions

**Migration Guide Structure:**

```markdown
# Migration Guide: v2.x to v3.x

## Quick Start (5 minutes)

1. Install updated library
2. Run migration tool
3. Review changes
4. Update tests

## Step-by-Step Migration

### Step 1: Update Library Import

```robot
# Before (still works, with deprecation warning)
Library    SwingLibrary

# After (recommended)
Library    JavaGuiLibrary    mode=swing
```

### Step 2: Run Migration Tool

```bash
python -m javagui.migrate --dry-run tests/
python -m javagui.migrate tests/
```

### Step 3: Review Keyword Changes

See [Keyword Mapping](KEYWORD-MAPPING.md) for complete list.

### Step 4: Update Exception Handlers

```robot
# Before
Run Keyword And Expect Error    SwingConnectionError:*    ...

# After
Run Keyword And Expect Error    ConnectionError:*    ...
```

## Detailed Changes

### Library Classes

| Old | New | Notes |
|-----|-----|-------|
| SwingLibrary | JavaGuiLibrary mode=swing | Full compatibility |
| SwtLibrary | JavaGuiLibrary mode=swt | Full compatibility |
| RcpLibrary | JavaGuiLibrary mode=rcp | Full compatibility |

### Keywords

[Complete mapping table...]

### Exceptions

[Complete exception mapping...]

## Troubleshooting

### Common Issues

...

## Timeline

| Version | Status | Action Required |
|---------|--------|-----------------|
| 2.1.x | Current | No action |
| 2.2.x | Warnings | Update recommended |
| 3.0.x | Breaking | Update required |
```

### 4.4 Test Conversion Utilities (Week 13-14)

**Migration Tool Enhancement:**

```python
#!/usr/bin/env python3
"""
Enhanced migration tool for robotframework-javagui

Features:
- Keyword renaming
- Library import updates
- Exception handler updates
- Locator syntax suggestions
- Dry-run mode
- Backup creation
- Detailed reporting
"""

class EnhancedMigrator:
    def __init__(self, config: MigrationConfig):
        self.config = config
        self.changes = []
        self.warnings = []

    def migrate_project(self, path: Path) -> MigrationReport:
        """Migrate entire project directory."""
        for robot_file in path.glob("**/*.robot"):
            self.migrate_file(robot_file)
        for resource_file in path.glob("**/*.resource"):
            self.migrate_file(resource_file)
        for python_file in path.glob("**/*.py"):
            self.migrate_python_file(python_file)

        return self.generate_report()

    def migrate_file(self, file_path: Path) -> List[Change]:
        """Migrate a single Robot Framework file."""
        changes = []

        # Update library imports
        changes.extend(self.update_library_imports(file_path))

        # Update keywords
        changes.extend(self.update_keywords(file_path))

        # Update exception references
        changes.extend(self.update_exceptions(file_path))

        # Suggest locator improvements
        self.warnings.extend(self.analyze_locators(file_path))

        return changes

    def generate_report(self) -> MigrationReport:
        """Generate detailed migration report."""
        return MigrationReport(
            total_files=self.stats['files'],
            total_changes=len(self.changes),
            changes_by_type=self.group_changes_by_type(),
            warnings=self.warnings,
            estimated_impact=self.calculate_impact(),
        )
```

**CLI Interface:**

```bash
# Analyze project (dry run)
javagui-migrate analyze ./tests

# Preview changes
javagui-migrate preview ./tests --output report.html

# Apply migration
javagui-migrate apply ./tests --backup

# Verify migration
javagui-migrate verify ./tests

# Rollback (if backup exists)
javagui-migrate rollback ./tests
```

### Phase 4 Quality Gate

| Metric | Target | Measurement |
|--------|--------|-------------|
| Migration tool accuracy | 99%+ | Test on sample projects |
| Documentation completeness | 100% | Review checklist |
| Backward compatibility | 100% | Legacy test suite |
| User acceptance | Positive feedback | Beta testing |

---

## Testing Strategy

### Unit Testing (90%+ Coverage Target)

**Test Organization:**

```
tests/
+-- unit/
|   +-- locator/
|   |   +-- test_normalizer.rs
|   |   +-- test_chain_parser.rs
|   |   +-- test_filters.rs
|   +-- core/
|   |   +-- test_assertion.rs
|   |   +-- test_dispatcher.rs
|   |   +-- test_session.rs
|   +-- keywords/
|       +-- test_click.rs
|       +-- test_get_text.rs
|       +-- test_table.rs
+-- integration/
|   +-- robot/
|   |   +-- swing/
|   |   +-- swt/
|   |   +-- rcp/
|   +-- apps/
|       +-- swing-test-app/
|       +-- swt-test-app/
+-- performance/
    +-- benchmarks.rs
```

**Mock Strategy:**

```rust
use mockall::automock;

#[automock]
pub trait ElementOperations {
    fn find_element(&self, locator: &str) -> PyResult<Box<dyn GuiElement>>;
    fn click(&self, locator: &str) -> PyResult<()>;
    // ...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_click_finds_and_clicks() {
        let mut mock = MockElementOperations::new();
        mock.expect_click()
            .with(eq("name:button"))
            .times(1)
            .returning(|_| Ok(()));

        let dispatcher = KeywordDispatcher::with_mock(mock);
        assert!(dispatcher.dispatch("click", ("name:button",)).is_ok());
    }
}
```

### Integration Testing

**Test Matrix:**

| Keyword Category | Swing | SWT | RCP | Total Tests |
|------------------|-------|-----|-----|-------------|
| Connection | 5 | 5 | 5 | 15 |
| Element Finding | 10 | 10 | 10 | 30 |
| Clicking | 8 | 8 | 8 | 24 |
| Text Input | 6 | 6 | 6 | 18 |
| Selection | 10 | 10 | 10 | 30 |
| Tables | 12 | 12 | 12 | 36 |
| Trees | 10 | 10 | 10 | 30 |
| Verification | 8 | 8 | 8 | 24 |
| **Total** | **69** | **69** | **69** | **207** |

### Performance Benchmarks

**Benchmark Suite:**

```rust
use criterion::{criterion_group, criterion_main, Criterion};

fn locator_parsing_benchmark(c: &mut Criterion) {
    let normalizer = LocatorNormalizer::new(GuiMode::Swing);

    c.bench_function("parse_simple_name", |b| {
        b.iter(|| normalizer.normalize("name:button"))
    });

    c.bench_function("parse_css_chain", |b| {
        b.iter(|| normalizer.normalize("Panel > Button[name='ok']"))
    });

    c.bench_function("parse_xpath", |b| {
        b.iter(|| normalizer.normalize("//Button[@name='ok']"))
    });
}

fn keyword_dispatch_benchmark(c: &mut Criterion) {
    let lib = create_connected_library();

    c.bench_function("click_keyword", |b| {
        b.iter(|| lib.click("name:button"))
    });

    c.bench_function("get_text_keyword", |b| {
        b.iter(|| lib.get_text("name:label", None, None, None))
    });
}

criterion_group!(
    benches,
    locator_parsing_benchmark,
    keyword_dispatch_benchmark
);
criterion_main!(benches);
```

**Performance Targets:**

| Operation | Target | Acceptable |
|-----------|--------|------------|
| Locator parsing (simple) | <1ms | <5ms |
| Locator parsing (complex) | <5ms | <10ms |
| Keyword dispatch overhead | <0.5ms | <1ms |
| Element cache hit | <0.1ms | <0.5ms |
| RPC round-trip | <50ms | <100ms |

### Regression Test Suite

**Backward Compatibility Tests:**

```robot
*** Settings ***
Documentation     Verify all legacy keywords still work
Library           SwingLibrary    # Legacy import

*** Test Cases ***
# Verify all deprecated keywords work with warnings
Legacy Click Element
    Click Element    name:button

Legacy Find Widget
    ${element}=    Find Widget    name:button
    Should Not Be Empty    ${element}

Legacy Widget Should Be Visible
    Widget Should Be Visible    name:button

Legacy Check Button
    Check Button    name:checkbox

# Verify exception compatibility
Legacy Exception Handling
    Run Keyword And Expect Error    SwingConnectionError:*
    ...    Connect To Application    nonexistent    localhost    9999
```

---

## Claude-Flow Integration

### Pre-Task Hooks for Planning

```bash
# Before starting any implementation task
npx @claude-flow/cli@latest hooks pre-task \
    --task-id "phase1-locator" \
    --description "Implement locator chain parser"

# Route task to optimal agent
npx @claude-flow/cli@latest hooks route \
    --task "Implement locator chain parser with CSS combinator support"
```

### Post-Task Hooks for Learning

```bash
# After completing a task successfully
npx @claude-flow/cli@latest hooks post-task \
    --task-id "phase1-locator" \
    --success true \
    --quality 0.95

# Train neural patterns on successful implementation
npx @claude-flow/cli@latest hooks post-edit \
    --file "src/locator/chain.rs" \
    --success true \
    --train-neural true
```

### Memory Storage for Patterns

```bash
# Store successful implementation patterns
npx @claude-flow/cli@latest memory store \
    --namespace patterns \
    --key "rust-trait-dispatch" \
    --value "Use trait objects with Box<dyn Trait> for dynamic dispatch in keyword handlers"

npx @claude-flow/cli@latest memory store \
    --namespace patterns \
    --key "locator-cache-strategy" \
    --value "LRU cache with 1000 entry limit, clear on connection change"

# Search for relevant patterns before implementation
npx @claude-flow/cli@latest memory search \
    --query "error handling rust pyresult" \
    --namespace patterns
```

### Neural Training on Successful Implementations

```bash
# After major feature completion
npx @claude-flow/cli@latest neural train \
    --pattern-type coordination \
    --epochs 10

# Predict optimal approach for new tasks
npx @claude-flow/cli@latest neural predict \
    --input "Implement table cell selection with row/column validation"
```

### Security Scanning Hooks

```bash
# Before merging any phase
npx @claude-flow/cli@latest security scan \
    --path src/ \
    --depth full

# Audit for common vulnerabilities
npx @claude-flow/cli@latest hooks worker dispatch \
    --trigger audit \
    --context "src/python/"
```

### Background Workers for Analysis

```bash
# Run test coverage analysis
npx @claude-flow/cli@latest hooks worker dispatch \
    --trigger testgaps \
    --context "src/"

# Performance optimization suggestions
npx @claude-flow/cli@latest hooks worker dispatch \
    --trigger optimize \
    --context "src/locator/"

# Update codebase map after changes
npx @claude-flow/cli@latest hooks worker dispatch \
    --trigger map
```

---

## Risk Assessment

### Breaking Changes Identification

| Change | Risk Level | Impact | Mitigation |
|--------|------------|--------|------------|
| Keyword renames | Medium | Test failures | Alias system (ADR-003) |
| Exception renames | Medium | Exception handlers break | Alias exceptions (ADR-005) |
| Library class changes | Low | Import statements | Wrapper classes (ADR-006) |
| Locator behavior changes | High | Element not found | Extensive testing |
| Mode detection changes | Medium | Wrong mode selected | Explicit mode override |
| RPC protocol changes | High | Communication failures | Version negotiation |

### Mitigation Strategies

**1. Comprehensive Alias System**
- All deprecated keywords have working aliases
- Deprecation warnings are configurable (can be disabled)
- Aliases remain for at least one major version

**2. Feature Flags**
```rust
// Cargo.toml
[features]
default = ["all-toolkits", "deprecation_warnings"]
deprecation_warnings = []
strict_mode = []  // Fail on deprecated usage
legacy_exceptions = []
```

**3. Version Negotiation**
```rust
impl Connection {
    pub fn negotiate_version(&mut self) -> PyResult<ProtocolVersion> {
        let server_version = self.query_server_version()?;
        let client_version = PROTOCOL_VERSION;

        // Find compatible version
        let negotiated = find_compatible_version(server_version, client_version)?;

        Ok(negotiated)
    }
}
```

**4. Gradual Rollout**
- Phase 1-2: Internal changes only, no user impact
- Phase 3: New features, full backward compatibility
- Phase 4: Deprecation warnings, migration tools
- v3.0: Breaking changes (with 1-year notice)

### Rollback Procedures

**Phase 1 Rollback:**
```bash
# No user-facing changes, simple git revert
git revert --no-commit HEAD~N..HEAD
git commit -m "Rollback Phase 1 changes"
```

**Phase 2 Rollback:**
```bash
# Disable new keywords via feature flag
cargo build --no-default-features --features "legacy_mode"

# Or revert commits
git revert --no-commit <phase2-commits>
```

**Phase 3 Rollback:**
```bash
# Feature flags control advanced features
cargo build --features "basic_locators"  # Disable chain locators
```

**Phase 4 Rollback:**
```bash
# Disable deprecation warnings
cargo build --no-default-features --features "no_deprecation_warnings"

# Users can rollback via migration tool
javagui-migrate rollback ./tests
```

---

## Timeline and Milestones

### Gantt Chart Overview

```
Week    1  2  3  4  5  6  7  8  9 10 11 12 13 14
        |--|--|--|--|--|--|--|--|--|--|--|--|--|--|
Phase 1 [============]                              Foundation
        |  Locator   |
           |  Assertion  |
              |  Session  |
                 | Dispatch|
                    [QG1]                           Quality Gate 1

Phase 2                [============]               Core Keywords
                       | Action KWs |
                          | Get KWs    |
                             | Select KWs |
                                | Int Tests|
                                   [QG2]            Quality Gate 2

Phase 3                               [=========]   Advanced
                                      |Locator++|
                                         |Table/Tree|
                                            |Screen+|
                                               [QG3] Quality Gate 3

Phase 4                                        [=======] Migration
                                               |Deprec|
                                                  |Docs|
                                                     |Tools|
                                                        [QG4]
```

### Detailed Milestones

| Milestone | Week | Deliverables | Success Criteria |
|-----------|------|--------------|------------------|
| M1: Foundation Complete | 4 | Locator, Assertion, Session, Dispatcher | All Phase 1 tests pass |
| M2: Core Keywords | 8 | Action, Get, Selection keywords | Integration tests pass |
| M3: Advanced Features | 11 | Chaining, Tables, Screenshots, Introspection | Feature tests pass |
| M4: Migration Ready | 14 | Warnings, Docs, Tools | User acceptance |

### Quality Gates Between Phases

**Quality Gate 1 (End of Phase 1):**
- [ ] Unit test coverage >= 90%
- [ ] All existing tests pass
- [ ] Performance benchmarks within targets
- [ ] Code review approved
- [ ] No critical/high security issues

**Quality Gate 2 (End of Phase 2):**
- [ ] All unified keywords implemented
- [ ] All aliases work correctly
- [ ] Integration tests pass on all platforms
- [ ] Documentation updated
- [ ] Performance regression < 5%

**Quality Gate 3 (End of Phase 3):**
- [ ] Advanced locator features work
- [ ] Specialized component handling complete
- [ ] Screenshot and introspection tools work
- [ ] All edge cases handled

**Quality Gate 4 (End of Phase 4):**
- [ ] Migration tool tested on sample projects
- [ ] Documentation complete and reviewed
- [ ] Deprecation warnings work correctly
- [ ] Beta user feedback incorporated

### Review Checkpoints

| Checkpoint | Week | Attendees | Agenda |
|------------|------|-----------|--------|
| Kickoff | 0 | All | Project overview, roles, timeline |
| Phase 1 Review | 4 | Tech Lead, QA | Foundation review, test coverage |
| Phase 2 Review | 8 | All | Keyword API review, integration |
| Phase 3 Review | 11 | Tech Lead, QA | Feature completeness |
| Final Review | 14 | All, Stakeholders | Release readiness |

---

## Appendix A: Complete Keyword Mapping

### Action Keywords

| Unified | Swing Alias | SWT Alias | Status |
|---------|-------------|-----------|--------|
| Click | Click Element | Click Widget | Primary |
| Double Click | Double Click Element | Double Click Widget | Primary |
| Right Click | Right Click Element | Right Click Widget | Primary |
| Input Text | Type Text | Input Text | Primary |
| Clear Text | Clear Element | Clear Widget | Primary |
| Append Text | Append To Text | - | Primary |

### Getter Keywords

| Unified | Swing Alias | SWT Alias | Status |
|---------|-------------|-----------|--------|
| Get Text | Get Element Text | Get Widget Text | Primary |
| Get Property | Get Element Property | Get Widget Property | Primary |
| Get Element Count | Count Elements | Count Widgets | Primary |
| Get Selected Value | - | - | Primary |

### Selection Keywords

| Unified | Swing Alias | SWT Alias | Status |
|---------|-------------|-----------|--------|
| Select Table Row | - | - | Primary |
| Select Table Cell | - | - | Primary |
| Select Tree Node | - | Select Tree Item | Primary |
| Expand Tree Node | - | Expand Tree Item | Primary |
| Select From Combobox | - | - | Primary |
| Select From List | - | - | Primary |
| Check | Check Checkbox | Check Button | Primary |
| Uncheck | Uncheck Checkbox | Uncheck Button | Primary |

### Verification Keywords

| Unified | Swing Alias | SWT Alias | Status |
|---------|-------------|-----------|--------|
| Element Should Exist | - | Widget Should Exist | Primary |
| Element Should Be Visible | - | Widget Should Be Visible | Primary |
| Element Should Be Enabled | - | Widget Should Be Enabled | Primary |
| Element Text Should Be | - | Widget Text Should Be | Primary |

---

## Appendix B: Exception Mapping

| New Exception | Legacy Alias | Description |
|---------------|--------------|-------------|
| JavaGuiError | - | Base exception |
| ConnectionError | SwingConnectionError | Connection issues |
| ConnectionRefusedError | - | Connection refused |
| ConnectionTimeoutError | SwingTimeoutError | Connection timeout |
| NotConnectedError | - | Not connected |
| ElementNotFoundError | - | Element not found |
| MultipleElementsFoundError | - | Multiple elements |
| LocatorParseError | PyLocatorParseError | Invalid locator |
| ActionFailedError | - | Action failed |
| ActionTimeoutError | - | Action timeout |
| ModeNotSupportedError | - | Wrong mode |

---

## References

- [ADR-001: Unified Base Class Architecture](./ADR-001-unified-base-class-architecture.md)
- [ADR-002: Locator Syntax Strategy](./ADR-002-locator-syntax-strategy.md)
- [ADR-003: Keyword Naming Convention](./ADR-003-keyword-naming-convention.md)
- [ADR-004: Technology Detection and Mode Selection](./ADR-004-technology-detection-and-mode-selection.md)
- [ADR-005: Error Handling Strategy](./ADR-005-error-handling-strategy.md)
- [ADR-006: Backwards Compatibility Approach](./ADR-006-backwards-compatibility-approach.md)
- [Robot Framework User Guide](https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html)
- [PyO3 Documentation](https://pyo3.rs/)
- [Semantic Versioning](https://semver.org/)
