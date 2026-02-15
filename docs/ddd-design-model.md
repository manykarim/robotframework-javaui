# Domain-Driven Design Model for robotframework-javaui

## 1. Bounded Context Map

### BC1: Connection Management

**Purpose**: Manages the lifecycle of connections to Java Virtual Machines, including JVM discovery, agent injection, and TCP socket communication.

**Aggregate Root**: `ConnectionManager`

| Concept | Role | Current Location |
|---------|------|-----------------|
| `ConnectionManager` | Aggregate Root | `src/connection/mod.rs` |
| `SwingConnection` | Entity | `src/connection/mod.rs` |
| `JvmInfo` | Value Object | `src/connection/mod.rs` |
| `ConnectionConfig` | Value Object | `src/core/config.rs` |
| `ConnectionInfo` | Value Object | `src/core/backend.rs` |
| `GenericBackend` | Domain Service | `src/core/backend.rs` |
| `BackendFactory` | Factory | `src/core/backend.rs` |

**Value Objects**:
- `JvmInfo` (pid, main_class, args, window_titles)
- `ConnectionConfig` (host, port, timeout, retry_count, application, toolkit, auto_reconnect)
- `ConnectionInfo` (host, port, toolkit_type)

**Domain Services**:
- JVM Discovery Service (wraps `jps` command execution)
- Agent Injection Service (wraps `jattach` for agent loading)
- Backend Factory (creates toolkit-specific backend instances)

**Invariants**:
1. A connection must have a valid JVM PID before agent injection can occur.
2. Agent injection must succeed before any RPC communication is attempted.
3. Only one active connection per JVM PID is permitted within a ConnectionManager.
4. Auto-reconnect attempts must respect retry_count and timeout bounds.
5. A disconnected backend must not accept send_request calls.

**Current File Mapping**:
- `src/connection/mod.rs` - SwingConnection, ConnectionManager, JvmInfo
- `src/core/backend.rs` - Backend trait, GenericBackend, BackendFactory, ConnectionInfo
- `src/core/config.rs` - ConnectionConfig (lines defining host, port, timeout fields)

**Proposed Reorganization**:
```
src/connection/
    mod.rs            -- re-exports
    manager.rs        -- ConnectionManager aggregate
    swing_connection.rs -- SwingConnection entity
    jvm_discovery.rs  -- JVM discovery domain service
    agent_injection.rs -- Agent injection domain service
    config.rs         -- ConnectionConfig value object (moved from core/)
```

---

### BC2: Element Model

**Purpose**: Represents UI elements across all toolkits (Swing, SWT, RCP) with a unified abstraction layer and toolkit-specific projections.

**Aggregate Root**: `JavaGuiElement`

| Concept | Role | Current Location |
|---------|------|-----------------|
| `JavaGuiElement` | Aggregate Root | `src/core/element.rs` |
| `ElementId` | Value Object / Identity | `src/core/element.rs` |
| `ElementType` | Value Object | `src/core/element.rs` (33 enum variants) |
| `Bounds` | Value Object | `src/core/element.rs` |
| `ElementState` | Value Object | `src/core/element.rs` |
| `UIComponent` | Entity (Swing) | `src/model/component.rs` |
| `UITree` | Entity (Swing) | `src/model/component.rs` |
| `SwtWidget` | Entity (SWT) | `src/model/widget.rs` |
| `SwtWidgetTree` | Entity (SWT) | `src/model/widget.rs` |
| `ElementCache` | Domain Service | `src/core/element_cache.rs` |
| `FinderCache` | Domain Service | `src/core/element_cache.rs` |

**Value Objects**:
- `ElementId` (handle: i64, tree_path: Vec<usize>, depth: usize)
- `ElementType` (33 variants: Button, CheckBox, TextField, Table, Tree, etc.)
- `Bounds` (x, y, width, height) with geometric operations (contains, intersects, center, distance_to)
- `ElementState` (visible, enabled, focused, selected, editable)
- `ComponentId`, `ComponentIdentity`, `ComponentGeometry` (Swing-specific)
- `WidgetId`, `WidgetBounds`, `SwtWidgetState`, `SwtStyle` (SWT-specific)
- `AccessibilityInfo` / `AccessibleInfo`

**Invariants**:
1. Every JavaGuiElement must have a non-empty hash_code (identity).
2. ElementType must map to a valid toolkit-specific class name via from_swing_class or from_swt_class.
3. Bounds must have non-negative width and height.
4. An element's toolkit field must match the toolkit that produced it.
5. Cache entries must be invalidated when their TTL expires.
6. A stale element (no longer present in the UI tree) must raise StaleElementError on access.

**Current File Mapping**:
- `src/core/element.rs` - JavaGuiElement, ElementType, Bounds, ElementState, ElementId
- `src/core/element_cache.rs` - ElementCache, FinderCache, ElementCacheStats
- `src/model/component.rs` - Swing UIComponent, UITree, ComponentType, FilterBuilder
- `src/model/element.rs` - Lower-level UIElement, SwingComponentType
- `src/model/widget.rs` - SWT SwtWidget, SwtWidgetTree, SwtWidgetType, SwtFilterBuilder
- `src/model/tree.rs` - TreeFilter
- `src/model/rcp.rs` - RCP-specific element types (View, Editor, Perspective)

**Proposed Reorganization**:
```
src/element/
    mod.rs              -- re-exports, JavaGuiElement aggregate
    identity.rs         -- ElementId, ElementType
    geometry.rs         -- Bounds value object
    state.rs            -- ElementState value object
    cache.rs            -- ElementCache, FinderCache (domain service)
    swing/
        mod.rs          -- UIComponent, UITree
        component.rs    -- Component types, properties
        filter.rs       -- FilterBuilder, FilterSpecification
    swt/
        mod.rs          -- SwtWidget, SwtWidgetTree
        widget.rs       -- Widget types, properties
        filter.rs       -- SwtFilterBuilder, SwtFilterSpec
    rcp/
        mod.rs          -- View, Editor, Perspective
        workbench.rs    -- Workbench, WorkbenchWindow
        layout.rs       -- PerspectiveLayout, ViewPosition
```

---

### BC3: Locator / Query Language

**Purpose**: Parses, validates, and evaluates locator expressions (CSS-like, XPath-like, toolkit-specific) to find UI elements in component trees.

**Aggregate Root**: `UnifiedLocator`

| Concept | Role | Current Location |
|---------|------|-----------------|
| `UnifiedLocator` | Aggregate Root | `src/locator/unified.rs` |
| `LocatorExpression` | Value Object | `src/locator/expression.rs` |
| `Locator` (AST) | Value Object | `src/locator/ast.rs` |
| `SwtLocator` | Value Object | `src/locator/swt_matcher.rs` |
| `Evaluator` | Domain Service | `src/locator/matcher.rs` |
| `SwtMatcher` | Domain Service | `src/locator/swt_matcher.rs` |
| `LocatorFactory` | Factory | `src/locator/unified.rs` |

**Value Objects**:
- `LocatorExpression` with variants: Simple (type#id, type.name, etc.), Css, XPath
- `SimpleLocator` (locator_type, value) with SimpleLocatorType enum
- `CssSelector` (segments of type/class/attribute/pseudo selectors)
- `XPathExpression` (steps with axis, node test, predicates)
- `Locator` AST (ComplexSelector, CompoundSelector, Combinator)
- `SwtLocator` (WidgetSelector, ViewSelector, EditorSelector, PerspectiveSelector, MenuSelector)
- `NormalizedLocator` (canonical form for cache keying)

**Domain Services**:
- `Evaluator` - Matches Swing components against locator AST
- `SwtMatcher` - Matches SWT widgets against SWT-specific locators
- `LocatorFactory` - Creates appropriate locator type from string input
- `parse_locator()` - Parser function (Swing CSS-like syntax)
- `parse_swt_locator()` - Parser function (SWT-specific syntax)

**Invariants**:
1. A locator string must parse successfully or produce a descriptive LocatorParseError.
2. CSS selectors must follow the compound selector grammar (type, class, attribute, pseudo).
3. XPath expressions must have valid axis and predicate syntax.
4. SWT locators must reference valid SwtWidgetType values.
5. NormalizedLocator must produce identical output for semantically equivalent locators.
6. Locator matching must be deterministic given the same tree and locator.

**Current File Mapping**:
- `src/locator/ast.rs` - AST types (Locator, ComplexSelector, Combinator, etc.)
- `src/locator/expression.rs` - LocatorExpression, SimpleLocator, CssSelector, XPathExpression
- `src/locator/parser.rs` - parse_locator function, ParseError
- `src/locator/matcher.rs` - Evaluator, MatchContext, find_matching_components
- `src/locator/swt_matcher.rs` - SwtLocator, SwtMatcher, parse_swt_locator
- `src/locator/unified.rs` - UnifiedLocator, NormalizedLocator, LocatorFactory

**Proposed Reorganization** (minimal changes -- already well-structured):
```
src/locator/
    mod.rs              -- re-exports (current structure is good)
    ast.rs              -- AST types
    expression.rs       -- Expression types
    parser.rs           -- Swing parser
    matcher.rs          -- Swing matcher/evaluator
    swt_matcher.rs      -- SWT matcher
    unified.rs          -- Cross-toolkit unified locator
```

---

### BC4: Protocol / Communication

**Purpose**: Defines and manages the JSON-RPC 2.0 protocol for communication between the Rust core and the Java agent running inside the target JVM.

**Aggregate Root**: None (this is a pure infrastructure/supporting context)

| Concept | Role | Current Location |
|---------|------|-----------------|
| `JsonRpcRequest` | Value Object | `src/protocol/mod.rs` |
| `JsonRpcResponse` | Value Object | `src/protocol/mod.rs` |
| `JsonRpcError` | Value Object | `src/protocol/mod.rs` |
| `RpcMethod` | Value Object | `src/protocol/mod.rs` (30 methods) |
| error_codes module | Constants | `src/protocol/mod.rs` |

**Value Objects**:
- `JsonRpcRequest` (jsonrpc, method, params, id)
- `JsonRpcResponse` (jsonrpc, result, error, id)
- `JsonRpcError` (code, message, data)
- `RpcMethod` enum (30 variants grouped into Discovery, Element Location, Inspection, Actions, Table Ops, Tree Ops, Waits, Screenshots)

**Invariants**:
1. All requests must use JSON-RPC version "2.0".
2. Request IDs must be unique within a session (monotonically incrementing u64).
3. A response must contain either `result` or `error`, never both.
4. Error codes must use standard JSON-RPC codes (-32700 to -32603) or custom application codes (-32000 to -32004).
5. Method names must map 1:1 to RpcMethod enum variants via as_str().

**Current File Mapping**:
- `src/protocol/mod.rs` - All protocol types

**Proposed Reorganization** (already minimal and clean):
```
src/protocol/
    mod.rs          -- re-exports
    request.rs      -- JsonRpcRequest
    response.rs     -- JsonRpcResponse
    error.rs        -- JsonRpcError, error_codes
    methods.rs      -- RpcMethod enum
```

---

### BC5: Assertion / Verification

**Purpose**: Provides retry-based assertion patterns for verifying UI element states, text values, and numeric properties. Integrates with Robot Framework's assertion model.

**Aggregate Root**: `AssertionConfig`

| Concept | Role | Current Location |
|---------|------|-----------------|
| `AssertionConfig` | Aggregate Root | `python/JavaGui/assertions/__init__.py` |
| `ElementState` | Value Object (Flag enum) | `python/JavaGui/assertions/__init__.py` |
| `with_retry_assertion` | Domain Service | `python/JavaGui/assertions/__init__.py` |
| `state_assertion_with_retry` | Domain Service | `python/JavaGui/assertions/__init__.py` |
| `numeric_assertion_with_retry` | Domain Service | `python/JavaGui/assertions/__init__.py` |
| `SecureExpressionEvaluator` | Domain Service | `python/JavaGui/assertions/security.py` |
| Formatters | Value Objects | `python/JavaGui/assertions/formatters.py` |

**Value Objects**:
- `ElementState` Flag enum (visible, hidden, enabled, disabled, focused, unfocused, selected, unselected, editable, readonly, checked, unchecked)
- Text Formatters (normalize_spaces, strip, lowercase, uppercase, strip_html_tags)
- FORMATTERS registry dict

**Domain Services**:
- `with_retry_assertion(getter_fn, assertion_operator, expected, message, formatter)` - Retry loop with configurable timeout and interval
- `state_assertion_with_retry(getter_fn, expected_states, message)` - State flag matching
- `numeric_assertion_with_retry(getter_fn, assertion_operator, expected, message)` - Numeric comparison
- `SecureExpressionEvaluator` - AST-based safe evaluation for `validate` operator
- `apply_formatters(value, formatter_string)` - Text transformation pipeline

**Invariants**:
1. Assertion timeout must be positive.
2. Poll interval must be less than timeout.
3. SecureExpressionEvaluator must reject any AST node not in SAFE_BUILTINS.
4. SecureExpressionEvaluator must reject attribute access to DANGEROUS_ATTRIBUTES.
5. Formatters must be applied in the order specified in the pipe-delimited string.
6. ElementState flags must be consistent (cannot be both visible and hidden simultaneously).

**Current File Mapping**:
- `python/JavaGui/assertions/__init__.py` - AssertionConfig, ElementState, retry functions
- `python/JavaGui/assertions/formatters.py` - Text formatters
- `python/JavaGui/assertions/security.py` - SecureExpressionEvaluator

**Proposed Reorganization**:
```
python/JavaGui/assertions/
    __init__.py         -- AssertionConfig, ElementState (keep)
    retry.py            -- Extract retry functions from __init__.py
    formatters.py       -- Text formatters (keep)
    security.py         -- SecureExpressionEvaluator (keep)
    operators.py        -- Extract assertion operator matching logic
```

---

### BC6: Toolkit Adaptation (Python Keyword Layer)

**Purpose**: Adapts the Rust core's capabilities into Robot Framework keyword libraries, providing toolkit-specific keyword sets (Swing, SWT, RCP) and a unified library. This is the primary public API surface.

**Aggregate Root**: `JavaGuiLibrary` (unified), with `SwingLibrary`, `SwtLibrary`, `RcpLibrary` as toolkit-specific adapters.

| Concept | Role | Current Location |
|---------|------|-----------------|
| `JavaGuiLibrary` | Aggregate Root (unified) | `src/python/base_library.rs` |
| `SwingLibrary` | Adapter Entity | `python/JavaGui/__init__.py` (lines 200-1737) |
| `SwtLibrary` | Adapter Entity | `python/JavaGui/__init__.py` (lines 1809-2614) |
| `RcpLibrary` | Adapter Entity | `python/JavaGui/__init__.py` (lines 2617-3503) |
| `SwingElement` | Adapter Entity | `python/JavaGui/__init__.py` (lines 1746-1807) |
| Keyword Mixins | Domain Services | `python/JavaGui/keywords/*.py` |
| Deprecation System | Infrastructure Service | `python/JavaGui/deprecation.py` |

**Keyword Mixin Classes** (Domain Services providing grouped keyword behavior):
- `GetterKeywords` - Element property getters with assertion support
- `TableKeywords` - Swing table operations
- `TreeKeywords` - Swing tree operations
- `ListKeywords` - Swing list operations
- `SwtGetterKeywords` - SWT widget property getters
- `SwtTableKeywords` - SWT table operations
- `SwtTreeKeywords` - SWT tree operations
- `RcpKeywords` - RCP workbench, view, editor, perspective operations

**Invariants**:
1. ROBOT_LIBRARY_SCOPE must be "GLOBAL" for all library classes.
2. Every library must initialize its Rust backing class (_SwingLibrary, _SwtLibrary, _RcpLibrary) before keyword use.
3. Keyword names must be unique within a library (Robot Framework constraint).
4. Deprecated keywords must produce warnings but remain functional.
5. The unified JavaGuiLibrary must support all keywords from all toolkit-specific libraries.
6. Connection must be established before any element-interacting keyword is called.

**Current File Mapping**:
- `python/JavaGui/__init__.py` - SwingLibrary (1537 lines), SwtLibrary (805 lines), RcpLibrary (886 lines), SwingElement (61 lines)
- `python/JavaGui/keywords/getters.py` - GetterKeywords mixin
- `python/JavaGui/keywords/tables.py` - TableKeywords, TreeKeywords, ListKeywords mixins
- `python/JavaGui/keywords/rcp_keywords.py` - RcpKeywords mixin
- `python/JavaGui/keywords/swt_getters.py` - SwtGetterKeywords mixin
- `python/JavaGui/keywords/swt_tables.py` - SwtTableKeywords mixin
- `python/JavaGui/keywords/swt_trees.py` - SwtTreeKeywords mixin
- `python/JavaGui/deprecation.py` - deprecated decorator, KeywordAliasRegistry

**Proposed Reorganization** (primary refactoring target):
```
python/JavaGui/
    __init__.py              -- Minimal: imports and re-exports only
    _unified.py              -- JavaGuiLibrary (unified, delegates to Rust)
    swing/
        __init__.py          -- SwingLibrary class definition
        _element.py          -- SwingElement wrapper
        _connection.py       -- Swing connection keywords
        _interaction.py      -- Click, type, select keywords
        _inspection.py       -- Get text, get properties keywords
        _tables.py           -- Table-specific keywords
        _trees.py            -- Tree-specific keywords
        _lists.py            -- List-specific keywords
        _menus.py            -- Menu-specific keywords
        _waits.py            -- Wait keywords
        _screenshots.py      -- Screenshot keywords
        _verification.py     -- Should-exist, should-contain keywords
        _config.py           -- Configuration keywords
    swt/
        __init__.py          -- SwtLibrary class definition
        _element.py          -- SwtElement wrapper
        _connection.py       -- SWT connection keywords
        _interaction.py      -- Widget interaction keywords
        _inspection.py       -- Widget property getters
        _tables.py           -- SWT table keywords
        _trees.py            -- SWT tree keywords
        _shells.py           -- Shell management keywords
    rcp/
        __init__.py          -- RcpLibrary class definition
        _workbench.py        -- Workbench/window keywords
        _perspectives.py     -- Perspective keywords
        _views.py            -- View keywords
        _editors.py          -- Editor keywords
        _commands.py         -- Command execution keywords
        _preferences.py      -- Preference keywords
    keywords/                -- Shared keyword mixins (keep existing)
    assertions/              -- Assertion framework (keep existing)
    deprecation.py           -- Deprecation system (keep existing)
```

---

### BC7: Error / Exception Hierarchy

**Purpose**: Defines a unified, technology-agnostic exception hierarchy that maps between Rust errors, JSON-RPC error codes, and Python exceptions for Robot Framework consumption.

**Aggregate Root**: None (cross-cutting concern)

| Concept | Role | Current Location |
|---------|------|-----------------|
| `SwingError` | Rust Error Enum | `src/error.rs` |
| `BackendError` | Rust Error Enum | `src/core/backend.rs` |
| `JavaGuiError` | Python Base Exception | `src/python/unified_exceptions.rs` |
| `ConnectionError` | Python Exception | `src/python/unified_exceptions.rs` |
| `ElementError` | Python Exception | `src/python/unified_exceptions.rs` |
| `LocatorError` | Python Exception | `src/python/unified_exceptions.rs` |
| `ActionError` | Python Exception | `src/python/unified_exceptions.rs` |
| `TechnologyError` | Python Exception | `src/python/unified_exceptions.rs` |
| Legacy exceptions | Python Aliases | `src/python/exceptions.rs` |
| `ElementNotFoundContext` | Value Object | `src/error.rs` |
| `SimilarElement` | Value Object | `src/error.rs` |

**Error Code Mapping** (JSON-RPC to Domain):
| JSON-RPC Code | Domain Meaning | Python Exception |
|---------------|----------------|-----------------|
| -32000 | Element not found | ElementNotFoundError |
| -32001 | Multiple elements found | MultipleElementsFoundError |
| -32002 | Not interactable | ElementNotInteractableError |
| -32003 | Timeout | ActionTimeoutError |
| -32004 | Stale element | StaleElementError |
| -32700 | Parse error | InternalError |
| -32600 | Invalid request | InternalError |
| -32601 | Method not found | ActionNotSupportedError |
| -32602 | Invalid params | LocatorParseError |
| -32603 | Internal error | InternalError |

**Invariants**:
1. Every Rust SwingError variant must map to exactly one Python exception type.
2. Legacy exception names must resolve to their unified equivalents.
3. ElementNotFoundContext must include the searched tree snapshot and similar elements for diagnostics.
4. Error propagation must preserve the original error message and context through all layers (Rust -> PyO3 -> Python).

**Current File Mapping**:
- `src/error.rs` - SwingError, ElementNotFoundContext, SimilarElement
- `src/core/backend.rs` - BackendError
- `src/python/exceptions.rs` - Legacy Python exceptions
- `src/python/unified_exceptions.rs` - Unified Python exception hierarchy

---

### BC8: Configuration / Library Lifecycle

**Purpose**: Manages library-wide configuration, initialization, mode selection, and lifecycle (connect/disconnect/reconnect).

**Aggregate Root**: `LibraryConfig`

| Concept | Role | Current Location |
|---------|------|-----------------|
| `LibraryConfig` | Aggregate Root | `src/core/config.rs` |
| `GuiMode` | Value Object | `src/core/config.rs` |
| `LogLevel` | Value Object | `src/core/config.rs` |
| `ToolkitType` | Value Object | `src/core/backend.rs` |

**Value Objects**:
- `LibraryConfig` (timeout, poll_interval, screenshot settings, log_level, mode, cache settings, retry settings)
- `GuiMode` enum (Swing, Swt, Rcp, Auto)
- `LogLevel` enum (Debug, Info, Warning, Error)
- `ToolkitType` enum (Swing, Swt, Rcp)
- Environment variable overrides (JAVAGUI_TIMEOUT, JAVAGUI_HOST, JAVAGUI_PORT, etc.)

**Invariants**:
1. Timeout must be positive and greater than poll_interval.
2. GuiMode must be compatible with the connected JVM's available toolkits.
3. Cache TTL must be non-negative (0 disables caching).
4. Environment variables override programmatic configuration.
5. Configuration changes after connection are limited to non-connection settings.

**Current File Mapping**:
- `src/core/config.rs` - LibraryConfig, ConnectionConfig, GuiMode, LogLevel
- `src/core/mod.rs` - Module re-exports

---

## 2. Context Relationships

```
+------------------+          +-------------------+
|   Connection     |  Shared  |    Protocol /     |
|   Management     |  Kernel  |  Communication    |
|    (BC1)         |<-------->|     (BC4)         |
+--------+---------+          +-------------------+
         |                              ^
         | Customer-Supplier            | Used by
         v                              |
+--------+---------+          +---------+---------+
|   Element        |  ACL     |   Locator /       |
|   Model          |<-------->|   Query Language   |
|    (BC2)         |          |    (BC3)          |
+--------+---------+          +-------------------+
         |
         | Conformist
         v
+--------+---------+          +-------------------+
|   Toolkit        |  ACL     |   Assertion /     |
|   Adaptation     |<-------->|   Verification    |
|    (BC6)         |          |    (BC5)          |
+--------+---------+          +-------------------+
         |
         | Uses
         v
+--------+---------+          +-------------------+
|   Error /        |  Shared  |  Configuration /  |
|   Exception      |  Kernel  |  Lifecycle        |
|    (BC7)         |<-------->|    (BC8)          |
+------------------+          +-------------------+
```

### Relationship Details

**BC1 <-> BC4: Shared Kernel**
- Connection Management and Protocol share the JSON-RPC types directly.
- `GenericBackend` in BC1 constructs `JsonRpcRequest` from BC4 and interprets `JsonRpcResponse`.
- Change coupling: Modifications to RpcMethod enum require coordinated changes in both contexts.

**BC1 -> BC2: Customer-Supplier**
- Connection Management is the supplier providing connectivity.
- Element Model is the customer consuming connection services to populate element data.
- The Backend trait (BC1) returns raw JSON that Element Model (BC2) deserializes into UIComponent/SwtWidget.
- Element Model depends on Connection Management but not vice versa.

**BC2 <-> BC3: Anti-Corruption Layer**
- The Locator context (BC3) produces locator ASTs and matchers.
- The Element Model context (BC2) provides component trees.
- The ACL translates between locator match results and element identity.
- `find_matching_components()` in `src/locator/matcher.rs` takes UIComponent trees and returns matched components -- this is the ACL boundary.
- `SwtMatcher` performs the same role for SWT widget trees.

**BC6 <-> BC5: Anti-Corruption Layer**
- Toolkit Adaptation (BC6) uses Assertion (BC5) for retry-based verification keywords.
- The ACL is the keyword mixin pattern: `GetterKeywords`, `SwtGetterKeywords`, etc. call `with_retry_assertion()` passing getter lambdas.
- BC6 provides the getter functions; BC5 provides the retry/assertion logic.
- Neither context knows the internal details of the other.

**BC6 -> BC2: Conformist**
- Toolkit Adaptation (BC6) conforms to Element Model (BC2) data structures.
- Python wrapper classes (SwingElement, SwtElement) directly wrap Rust element types.
- BC6 cannot change BC2's element model -- it adapts to whatever BC2 provides.

**BC7 <-> BC8: Shared Kernel**
- Error hierarchy and Configuration share fundamental types (ToolkitType, GuiMode).
- Error messages reference configuration state (timeouts, connection settings).
- Both are consumed by all other contexts.

**BC7 -> All: Published Language**
- The exception hierarchy (BC7) is a Published Language consumed by all other contexts.
- Every context raises exceptions from BC7's hierarchy.
- The JSON-RPC error code mapping is the translation layer between Protocol (BC4) and the published exception language.

---

## 3. Domain Events

### Connection Events

| Event | Trigger | Producer | Consumers | Data |
|-------|---------|----------|-----------|------|
| `ConnectionEstablished` | Successful connect | BC1 | BC2, BC6 | host, port, toolkit_type, jvm_pid |
| `ConnectionLost` | Socket disconnect | BC1 | BC2, BC6, BC7 | reason, last_request_id |
| `ConnectionReconnected` | Auto-reconnect success | BC1 | BC2, BC6 | attempt_count, downtime_ms |
| `JvmDiscovered` | JVM found via jps | BC1 | BC1 (internal) | JvmInfo |
| `AgentInjected` | Agent loaded into JVM | BC1 | BC1 (internal) | jvm_pid, agent_port |

### Element Events

| Event | Trigger | Producer | Consumers | Data |
|-------|---------|----------|-----------|------|
| `ElementFound` | Successful find | BC2, BC3 | BC6, BC5 | element_id, locator, match_count |
| `ElementNotFound` | Find returns empty | BC2, BC3 | BC6, BC7 | locator, searched_tree, similar_elements, suggestions |
| `MultipleElementsFound` | Find returns > 1 | BC2, BC3 | BC6, BC7 | locator, count, elements |
| `ElementStale` | Cached element invalid | BC2 | BC6, BC7 | element_id, reason |
| `TreeRetrieved` | Component tree fetched | BC2 | BC3, BC6 | root_component, node_count, depth |
| `TreeFiltered` | Tree filtered by criteria | BC2 | BC6 | filter_spec, matched_count, total_count |
| `CacheHit` | Element found in cache | BC2 | (monitoring) | element_id, cache_age_ms |
| `CacheEviction` | LRU/TTL eviction | BC2 | (monitoring) | element_id, reason |

### Assertion Events

| Event | Trigger | Producer | Consumers | Data |
|-------|---------|----------|-----------|------|
| `AssertionPassed` | Retry assertion succeeds | BC5 | BC6 | keyword, operator, expected, actual, attempts, duration_ms |
| `AssertionFailed` | Retry assertion exhausted | BC5 | BC6, BC7 | keyword, operator, expected, actual, attempts, timeout_ms |
| `AssertionRetry` | Single retry attempt | BC5 | (monitoring) | attempt_number, actual_value |

### Action Events

| Event | Trigger | Producer | Consumers | Data |
|-------|---------|----------|-----------|------|
| `ActionPerformed` | Click/type/select succeeds | BC4, BC6 | (monitoring) | action_type, element_id, duration_ms |
| `ActionFailed` | Action RPC returns error | BC4, BC7 | BC6 | action_type, element_id, error_code, message |
| `ScreenshotCaptured` | Screenshot taken | BC6 | (monitoring) | file_path, element_id, dimensions |

### Lifecycle Events

| Event | Trigger | Producer | Consumers | Data |
|-------|---------|----------|-----------|------|
| `LibraryInitialized` | Library class created | BC8 | BC1 | config, mode, toolkit |
| `ModeChanged` | GuiMode switched | BC8 | BC1, BC6 | old_mode, new_mode |
| `ConfigurationUpdated` | Runtime config change | BC8 | All | changed_keys, new_values |

---

## 4. Aggregate Invariants

### ConnectionManager Aggregate (BC1)

```
INV-C1: connection.state IN {Disconnected, Connecting, Connected, Reconnecting}
INV-C2: connection.state = Connected => connection.socket IS NOT NULL
INV-C3: connection.retry_count >= 0 AND connection.retry_count <= config.max_retries
INV-C4: FORALL c1, c2 IN connections: c1.jvm_pid != c2.jvm_pid (unique per JVM)
INV-C5: connection.state = Disconnected => send_request() RAISES NotConnectedError
INV-C6: agent_injection REQUIRES jvm_pid > 0 AND agent_jar_path EXISTS
```

### JavaGuiElement Aggregate (BC2)

```
INV-E1: element.hash_code != 0 (valid identity)
INV-E2: element.toolkit IN {Swing, Swt, Rcp}
INV-E3: element.bounds.width >= 0 AND element.bounds.height >= 0
INV-E4: element.element_type maps to a known class in the element's toolkit
INV-E5: cache.size <= cache.max_capacity
INV-E6: cache_entry.age <= cache.ttl_seconds OR cache_entry IS evicted
INV-E7: finder_cache[locator] = element_id => element_id EXISTS in element_cache
```

### UnifiedLocator Aggregate (BC3)

```
INV-L1: locator.raw_string IS NOT EMPTY
INV-L2: locator.parsed IS valid AST OR locator.error IS descriptive
INV-L3: CSS selector segments follow: type? (#id | .class | [attr] | :pseudo)*
INV-L4: XPath steps follow: axis::node_test[predicate]*
INV-L5: normalized(locator_a) = normalized(locator_b) => semantics(a) = semantics(b)
INV-L6: SWT locator widget_type IN SwtWidgetType enum values
```

### AssertionConfig Aggregate (BC5)

```
INV-A1: config.timeout > 0
INV-A2: config.interval > 0 AND config.interval < config.timeout
INV-A3: assertion_operator IN {equal, not_equal, contain, not_contain, starts_with,
         ends_with, matches, validate, greater_than, less_than, >=, <=}
INV-A4: validate operator expression passes SecureExpressionEvaluator.validate()
INV-A5: formatter_names are all present in FORMATTERS registry
INV-A6: ElementState flags: NOT (visible AND hidden) simultaneously
INV-A7: ElementState flags: NOT (enabled AND disabled) simultaneously
```

### LibraryConfig Aggregate (BC8)

```
INV-CF1: config.timeout > config.poll_interval
INV-CF2: config.gui_mode IN {Swing, Swt, Rcp, Auto}
INV-CF3: config.log_level IN {Debug, Info, Warning, Error}
INV-CF4: config.cache_ttl >= 0 (0 = disabled)
INV-CF5: config.retry_count >= 0
INV-CF6: env_var_override(key) TAKES PRECEDENCE over programmatic config
INV-CF7: config.screenshot_dir IS writable directory when screenshots enabled
```

---

## 5. Proposed Module Structure

### Current State (Problems)

The current `python/JavaGui/__init__.py` is a **3523-line god file** containing 4 classes:
- `SwingLibrary` (lines 200-1737) -- 1537 lines
- `SwingElement` (lines 1746-1807) -- 61 lines
- `SwtLibrary` (lines 1809-2614) -- 805 lines
- `RcpLibrary` (lines 2617-3503) -- 886 lines

Problems:
1. **Single Responsibility Violation**: One file handles connection, element finding, clicking, typing, table ops, tree ops, menus, waits, screenshots, configuration, and assertions.
2. **Code Duplication**: Swing and SWT libraries duplicate patterns for tables, trees, getters with only the Rust backing class differing.
3. **Monolithic Coupling**: Changes to any keyword category require touching the same file.
4. **Testing Difficulty**: Cannot unit-test keyword categories in isolation.

### Proposed Python Module Structure

```
python/JavaGui/
    __init__.py                     # Minimal: 50-80 lines, imports and re-exports
    _version.py                     # Version constant

    # Unified Library (new projects)
    unified/
        __init__.py                 # JavaGuiLibrary (wraps Rust base_library)
        _keywords.py                # Unified cross-toolkit keyword methods

    # Swing Bounded Context
    swing/
        __init__.py                 # SwingLibrary class (~100 lines, mixin composition)
        _element.py                 # SwingElement wrapper (~70 lines)
        _connection.py              # connect_to_application, disconnect, etc.
        _finding.py                 # find_element, find_elements, wait_for_element
        _interaction.py             # click, double_click, right_click, type_text, etc.
        _selection.py               # select_from_combo, select_from_list, etc.
        _tables.py                  # Table keywords (move from keywords/tables.py)
        _trees.py                   # Tree keywords (move from keywords/tables.py)
        _lists.py                   # List keywords (move from keywords/tables.py)
        _menus.py                   # Menu keywords
        _waits.py                   # Wait until visible/enabled/etc.
        _verification.py            # element_should_exist, text_should_be, etc.
        _screenshots.py             # capture_screenshot, capture_element_screenshot
        _config.py                  # set_timeout, set_poll_interval, etc.
        _ui_tree.py                 # get_component_tree, print_component_tree

    # SWT Bounded Context
    swt/
        __init__.py                 # SwtLibrary class (~80 lines, mixin composition)
        _element.py                 # SwtElement wrapper
        _connection.py              # SWT connection keywords
        _finding.py                 # find_widget, find_widgets
        _interaction.py             # Widget interaction keywords
        _shells.py                  # Shell management keywords
        _tables.py                  # SWT table keywords (from keywords/swt_tables.py)
        _trees.py                   # SWT tree keywords (from keywords/swt_trees.py)
        _waits.py                   # SWT wait keywords
        _verification.py            # SWT verification keywords

    # RCP Bounded Context
    rcp/
        __init__.py                 # RcpLibrary class (~80 lines, mixin composition)
        _workbench.py               # Workbench/window keywords
        _perspectives.py            # Perspective keywords
        _views.py                   # View keywords
        _editors.py                 # Editor keywords
        _commands.py                # Command execution keywords
        _preferences.py             # Preference keywords
        _extensions.py              # Extension point keywords (future)

    # Shared Keyword Infrastructure
    keywords/
        __init__.py                 # Mixin exports (keep existing)
        getters.py                  # GetterKeywords mixin (shared by Swing/SWT)
        rcp_keywords.py             # RcpKeywords mixin (keep)
        swt_getters.py              # SwtGetterKeywords mixin (keep)
        swt_tables.py               # SwtTableKeywords mixin (keep)
        swt_trees.py                # SwtTreeKeywords mixin (keep)

    # Assertion Bounded Context (keep as-is, well-structured)
    assertions/
        __init__.py                 # AssertionConfig, ElementState, retry functions
        formatters.py               # Text formatters
        security.py                 # SecureExpressionEvaluator

    # Cross-Cutting
    deprecation.py                  # Deprecation system (keep)
    _compat.py                      # Backward compatibility aliases
```

### Proposed Rust Module Structure

```
src/
    lib.rs                          # PyO3 module entry point (keep, already clean)

    # BC1: Connection Management
    connection/
        mod.rs                      # Re-exports
        manager.rs                  # ConnectionManager aggregate
        swing_connection.rs         # SwingConnection entity
        jvm_discovery.rs            # Extract JVM discovery service
        agent_injection.rs          # Extract agent injection service

    # BC2: Element Model
    core/
        mod.rs                      # Re-exports
        element.rs                  # JavaGuiElement aggregate (keep)
        element_cache.rs            # Cache service (keep)
        backend.rs                  # Backend trait, GenericBackend (keep)
        config.rs                   # LibraryConfig, ConnectionConfig (keep)

    model/
        mod.rs                      # Re-exports (keep)
        component.rs               # Swing UIComponent (keep)
        element.rs                  # Lower-level elements (keep)
        tree.rs                     # TreeFilter (keep)
        widget.rs                   # SWT widgets (keep)
        rcp.rs                      # RCP types (keep)

    # BC3: Locator / Query Language (already well-structured)
    locator/
        mod.rs                      # Re-exports
        ast.rs                      # AST types
        expression.rs               # Expression types
        parser.rs                   # Parser
        matcher.rs                  # Swing matcher
        swt_matcher.rs              # SWT matcher
        unified.rs                  # Unified locator

    # BC4: Protocol
    protocol/
        mod.rs                      # All protocol types (keep, already minimal)

    # BC7: Error Hierarchy
    error.rs                        # SwingError enum (keep)

    # BC6: Python Bindings
    python/
        mod.rs                      # Module declarations
        base_library.rs             # JavaGuiLibrary (unified)
        swing_library.rs            # SwingLibrary (Rust backing)
        swt_library.rs              # SwtLibrary (Rust backing)
        rcp_library.rs              # RcpLibrary (Rust backing)
        element.rs                  # SwingElement binding
        swt_element.rs              # SwtElement binding
        exceptions.rs               # Legacy exceptions
        unified_exceptions.rs       # Unified exception hierarchy
```

### Migration Strategy for __init__.py Decomposition

**Phase 1: Extract without breaking changes**
1. Create `swing/`, `swt/`, `rcp/` subdirectories.
2. Move keyword groups into separate files within each subdirectory.
3. Convert each keyword group into a mixin class.
4. Reconstruct `SwingLibrary`, `SwtLibrary`, `RcpLibrary` as mixin compositions in their respective `__init__.py`.
5. Re-export from the top-level `__init__.py` to maintain backward compatibility.

**Phase 2: Keyword consolidation**
1. Identify duplicated patterns between Swing and SWT keyword implementations.
2. Extract shared logic into `keywords/` base mixins parameterized by toolkit.
3. Toolkit-specific modules (`swing/`, `swt/`) inherit from shared mixins, adding only toolkit-specific behavior.

**Phase 3: Deprecation of direct imports**
1. Add deprecation warnings for `from JavaGui import SwingLibrary` (suggest `from JavaGui.swing import SwingLibrary`).
2. Maintain backward-compatible imports in top-level `__init__.py` for at least 2 major versions.

---

## 6. Cross-Cutting Concerns

### Logging

**Current State**: No structured logging framework observed in the Python layer. Rust side uses standard error propagation.

**Proposed Placement**:
- **BC1 (Connection)**: Log connection lifecycle events (connect, disconnect, reconnect attempts, agent injection).
- **BC3 (Locator)**: Log locator parse results and match statistics at DEBUG level.
- **BC5 (Assertion)**: Log retry attempts, assertion outcomes at DEBUG/INFO level.
- **BC6 (Toolkit Adaptation)**: Log keyword entry/exit at DEBUG level for Robot Framework log integration.
- **Implementation**: Use Python `logging` module with library logger (`logging.getLogger("JavaGui")`). Follow Robot Framework logging conventions (`robot.api.logger` for keyword-level output).

### Validation (Input Validation at System Boundaries)

**Boundary Points**:
1. **Robot Framework -> Python Keywords** (BC6): Validate keyword arguments (locator strings, timeout values, operator names).
2. **Python -> Rust Core** (BC6 -> BC2): PyO3 type conversion provides basic validation. Add explicit validation for semantic correctness.
3. **Rust -> Java Agent** (BC1 -> BC4): Validate RPC method parameters before serialization.
4. **Java Agent -> Rust** (BC4 -> BC1): Validate response structure, error codes.
5. **User Expressions -> SecureExpressionEvaluator** (BC5): AST-based security validation for `validate` operator.

**Proposed Pattern**: Validate at the outermost boundary (keyword entry points in BC6) and at the protocol boundary (BC4). Internal boundaries between Rust modules can trust typed interfaces.

### Caching

**Current State**: `ElementCache` and `FinderCache` in `src/core/element_cache.rs` with LRU eviction and TTL.

**Proposed Enhancements**:
- **BC2 (Element Model)**: Element cache with configurable TTL per element type (stable elements like labels can have longer TTL, dynamic elements like table cells should have shorter TTL).
- **BC3 (Locator)**: Cache parsed locator ASTs keyed by raw string (locators are immutable value objects, so cache indefinitely).
- **BC1 (Connection)**: Cache JVM discovery results with short TTL (JVMs can start/stop).
- **Cache Invalidation Strategy**: Connection loss event (from BC1) must invalidate all element caches (BC2) since element handles become stale.

### Error Handling

**Current Layering**:
```
Java Agent (throws Java exceptions)
    -> JSON-RPC error response (error codes)
        -> Rust SwingError enum (src/error.rs)
            -> PyO3 Python exception (src/python/unified_exceptions.rs)
                -> Robot Framework keyword failure
```

**Placement by Context**:
- **BC4 (Protocol)**: Translate JSON-RPC error codes to domain error types. Map standard codes (-327xx) and custom codes (-320xx).
- **BC1 (Connection)**: Handle transport errors (socket, timeout). Trigger reconnection on recoverable errors.
- **BC2 (Element)**: Handle stale elements, not-found with diagnostic context (similar elements, suggestions).
- **BC3 (Locator)**: Handle parse errors with position information and suggestion.
- **BC5 (Assertion)**: Handle retry exhaustion, format assertion failure messages.
- **BC6 (Toolkit Adaptation)**: Catch all exceptions at keyword boundary, translate to Robot Framework failure format.
- **BC7 (Exception Hierarchy)**: Define the canonical exception types. All other contexts raise these.

### Security

**Current State**: `SecureExpressionEvaluator` in `python/JavaGui/assertions/security.py`.

**Placement**:
- **BC5 (Assertion)**: SecureExpressionEvaluator guards the `validate` assertion operator against code injection.
- **BC1 (Connection)**: Agent injection mechanism should validate agent JAR integrity (future).
- **BC6 (Toolkit Adaptation)**: Sanitize locator strings to prevent injection into the Java agent.

### Thread Safety

**Current State**: Element cache uses `RwLock` in Rust. Python GIL provides basic thread safety.

**Placement**:
- **BC2 (Element Cache)**: Thread-safe via `RwLock` (already implemented).
- **BC1 (Connection)**: Connection state transitions must be atomic.
- **BC8 (Configuration)**: Config reads should be lock-free; writes should be synchronized.

---

## Appendix: Glossary of Domain Terms

| Term | Definition | Context |
|------|-----------|---------|
| **Component** | A Swing UI element (JButton, JTextField, etc.) | BC2 |
| **Widget** | An SWT UI element (Button, Text, Table, etc.) | BC2 |
| **Element** | Unified abstraction over Component and Widget | BC2 |
| **Locator** | A string expression identifying UI elements (CSS-like, XPath-like) | BC3 |
| **Keyword** | A Robot Framework test step (Python method exposed as RF keyword) | BC6 |
| **Backend** | Abstraction over toolkit-specific communication | BC1 |
| **Agent** | Java JAR injected into target JVM for inspection | BC1 |
| **Perspective** | Eclipse RCP workspace layout configuration | BC2 (RCP) |
| **View** | Eclipse RCP panel within a perspective | BC2 (RCP) |
| **Editor** | Eclipse RCP document editor | BC2 (RCP) |
| **Shell** | SWT top-level window | BC2 (SWT) |
| **Assertion** | Verification with retry that expected condition holds | BC5 |
| **Mixin** | Python class providing keyword methods via multiple inheritance | BC6 |
