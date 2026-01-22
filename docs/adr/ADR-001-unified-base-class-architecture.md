# ADR-001: Unified Base Class Architecture

| ADR ID | ADR-001 |
|--------|---------|
| Title | Unified Base Class Architecture |
| Status | Proposed |
| Date | 2026-01-16 |
| Authors | Architecture Team |

## Context

The robotframework-javagui library currently implements three separate library classes for different Java GUI technologies:

1. **SwingLibrary** (~55 keywords) - For Java Swing applications
2. **SwtLibrary** (~55 keywords) - For Eclipse SWT applications
3. **RcpLibrary** (~83 keywords, extends SwtLibrary) - For Eclipse RCP applications

### Current Architecture Problems

1. **Code Duplication**: Approximately 70% of keywords have identical semantics across technologies but different implementations
2. **Inconsistent APIs**: Users face different keyword names for the same operations (e.g., `Click Element` vs `Click Widget`)
3. **Maintenance Burden**: Bug fixes and enhancements must be applied to multiple locations
4. **Learning Curve**: Users switching between technologies must learn different APIs

### Current Implementation Structure

```
SwingLibrary (Rust/PyO3)
├── LibraryConfig
├── ConnectionState
├── ~55 keywords
└── RPC communication via JSON-RPC

SwtLibrary (Rust/PyO3)
├── SwtLibraryConfig
├── SwtConnectionState
├── ~55 keywords
└── RPC communication via JSON-RPC

RcpLibrary (Rust/PyO3)
├── Contains SwtLibrary instance (composition)
├── Delegates SWT keywords
└── +28 RCP-specific keywords
```

### Decision Drivers

- Need for consistent user experience across technologies
- Desire to reduce maintenance overhead
- Requirement for backwards compatibility with existing tests
- Rust/PyO3 constraints on inheritance patterns
- Performance requirements (no runtime overhead)

## Decision

We will implement a **Trait-Based Composition Architecture** using Rust traits with PyO3 bindings, following these patterns:

### 1. Core Traits for Shared Behavior

```rust
/// Core trait for element operations - shared across all technologies
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

/// Trait for connection management
pub trait ConnectionOperations {
    fn connect(&mut self, app: &str, host: &str, port: u16, timeout: f64) -> PyResult<()>;
    fn disconnect(&mut self) -> PyResult<()>;
    fn is_connected(&self) -> PyResult<bool>;
    fn send_rpc_request(&self, method: &str, params: serde_json::Value) -> PyResult<serde_json::Value>;
}

/// Trait for table operations
pub trait TableOperations {
    fn get_table_row_count(&self, locator: &str) -> PyResult<i32>;
    fn get_table_cell(&self, locator: &str, row: i32, col: i32) -> PyResult<String>;
    fn select_table_row(&self, locator: &str, row: i32) -> PyResult<()>;
    // ... other table operations
}

/// Trait for tree operations
pub trait TreeOperations {
    fn expand_tree_node(&self, locator: &str, path: &str) -> PyResult<()>;
    fn collapse_tree_node(&self, locator: &str, path: &str) -> PyResult<()>;
    fn select_tree_node(&self, locator: &str, path: &str) -> PyResult<()>;
    // ... other tree operations
}
```

### 2. Shared Implementation Module

```rust
/// Shared implementation that works with any connection provider
pub struct SharedOperations<C: ConnectionOperations> {
    connection: Arc<RwLock<C>>,
    config: Arc<RwLock<LibraryConfig>>,
    locator_normalizer: LocatorNormalizer,
}

impl<C: ConnectionOperations> SharedOperations<C> {
    pub fn click_impl(&self, locator: &str) -> PyResult<()> {
        let normalized = self.locator_normalizer.normalize(locator)?;
        let component_id = self.get_component_id(&normalized)?;
        self.connection.read()?.send_rpc_request("click", json!({
            "componentId": component_id
        }))?;
        Ok(())
    }

    // ... other shared implementations
}
```

### 3. Technology-Specific Libraries

```rust
/// Unified Java GUI Library with mode selection
#[pyclass(name = "JavaGuiLibrary")]
pub struct JavaGuiLibrary {
    mode: GuiMode,
    shared: SharedOperations<ConnectionState>,
    swing_specific: Option<SwingSpecificOps>,
    swt_specific: Option<SwtSpecificOps>,
    rcp_specific: Option<RcpSpecificOps>,
}

#[derive(Clone, Copy)]
pub enum GuiMode {
    Swing,
    Swt,
    Rcp,
    Auto, // Detect from connection
}

#[pymethods]
impl JavaGuiLibrary {
    #[new]
    #[pyo3(signature = (mode="auto", timeout=10.0))]
    pub fn new(mode: &str, timeout: f64) -> PyResult<Self> {
        let gui_mode = match mode.to_lowercase().as_str() {
            "swing" => GuiMode::Swing,
            "swt" => GuiMode::Swt,
            "rcp" => GuiMode::Rcp,
            "auto" | _ => GuiMode::Auto,
        };
        // Initialize appropriate specific operations based on mode
        // ...
    }

    // Unified keywords available in all modes
    pub fn click(&self, locator: &str) -> PyResult<()> {
        self.shared.click_impl(locator)
    }

    // Mode-specific keywords return error if wrong mode
    pub fn open_perspective(&self, perspective_id: &str) -> PyResult<()> {
        match self.mode {
            GuiMode::Rcp => self.rcp_specific.as_ref()
                .ok_or_else(|| SwingError::action_failed("open_perspective", "Not in RCP mode"))?
                .open_perspective(perspective_id),
            _ => Err(SwingError::action_failed(
                "open_perspective",
                "This keyword is only available in RCP mode"
            ).into())
        }
    }
}

/// Backwards-compatible SwingLibrary wrapping unified library
#[pyclass(name = "SwingLibrary")]
pub struct SwingLibrary {
    inner: JavaGuiLibrary,
}

#[pymethods]
impl SwingLibrary {
    #[new]
    pub fn new(timeout: Option<f64>) -> PyResult<Self> {
        Ok(Self {
            inner: JavaGuiLibrary::new("swing", timeout.unwrap_or(10.0))?
        })
    }

    // Delegate to unified implementation
    pub fn click_element(&self, locator: &str) -> PyResult<()> {
        self.inner.click(locator)
    }
}
```

### 4. Class Hierarchy (Python View)

```
JavaGuiLibrary (unified, mode-selectable)
    |
    +-- SwingLibrary (swing mode, backwards compatible)
    +-- SwtLibrary (swt mode, backwards compatible)
    +-- RcpLibrary (rcp mode, backwards compatible)
```

### 5. Robot Framework Usage

```robot
*** Settings ***
# New unified approach
Library    JavaGui    mode=swing

# Or backwards-compatible specific libraries
Library    SwingLibrary
Library    SwtLibrary
Library    RcpLibrary

*** Test Cases ***
Test With Unified Library
    Connect To Application    myapp    localhost    5678
    Click    name:submitButton
    Input Text    name:username    testuser

Test With Legacy Swing Library
    Connect To Application    myapp
    Click Element    name:submitButton
    Input Text    name:username    testuser
```

## Consequences

### Positive

1. **Single Source of Truth**: Core operations implemented once, reducing bugs
2. **Consistent API**: Users get the same keywords regardless of technology
3. **Reduced Maintenance**: Bug fixes apply to all technologies automatically
4. **Backwards Compatible**: Existing tests continue to work via wrapper classes
5. **Clear Extension Points**: Technology-specific features have designated locations
6. **Better Performance**: Rust trait dispatch is zero-cost abstraction
7. **Easier Testing**: Shared implementations can be tested once

### Negative

1. **Initial Complexity**: Trait-based architecture is more complex than current approach
2. **Migration Effort**: Existing code must be refactored into traits
3. **PyO3 Constraints**: Some Rust patterns don't map cleanly to Python
4. **Mode Detection Overhead**: Auto-detection adds slight complexity

### Risks

1. **Breaking Changes**: Despite backwards compatibility efforts, edge cases may break
2. **Performance Regression**: Additional abstraction layer could impact performance
3. **Feature Parity**: Ensuring all technology-specific features remain accessible

## Alternatives Considered

### Alternative 1: Inheritance via Macro Generation

Use Rust macros to generate Python classes with shared implementations.

**Rejected because**:
- PyO3 doesn't support Python-style inheritance well
- Macro-generated code is harder to debug
- Limited flexibility for technology-specific overrides

### Alternative 2: Python-Level Inheritance

Keep Rust implementations separate, create Python base class that delegates.

**Rejected because**:
- Adds Python layer overhead
- Complicates the build process
- Loses Rust's type safety benefits

### Alternative 3: Keep Separate Libraries (Status Quo)

Continue with three independent implementations.

**Rejected because**:
- Doesn't address code duplication
- Doesn't address API inconsistency
- Increases maintenance burden over time

### Alternative 4: Pure Composition (No Traits)

Use struct composition without traits.

**Rejected because**:
- Loses ability to swap implementations
- Makes testing harder
- Less flexible for future extensions

## Implementation Plan

1. **Phase 1**: Define core traits and shared structs (2 weeks)
2. **Phase 2**: Implement SharedOperations with Swing backend (2 weeks)
3. **Phase 3**: Add SWT backend to SharedOperations (1 week)
4. **Phase 4**: Add RCP-specific operations (1 week)
5. **Phase 5**: Create backwards-compatible wrapper classes (1 week)
6. **Phase 6**: Update tests and documentation (2 weeks)

## References

- [PyO3 Documentation - Classes](https://pyo3.rs/v0.20.0/class)
- [Rust Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [robotframework-javagui Unified Keywords Research](/docs/unify_keywords_research.md)
