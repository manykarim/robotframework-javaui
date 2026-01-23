# ADR-001: Domain-Driven Design Architecture for Keyword API Simplification

**Status**: Proposed
**Date**: 2026-01-19
**Author**: System Architecture Designer
**Supersedes**: None
**Related**: docs/architecture/DDD_ARCHITECTURE_DESIGN.md, docs/research/Update_Keywords.md

---

## Context

The robotframework-swing library currently exposes approximately 60-70 keywords across Swing, SWT, and RCP toolkits. This keyword proliferation creates several challenges:

1. **Cognitive Load**: Test authors must learn many narrowly-scoped keywords
2. **Redundancy**: Separate keywords exist for actions, waits, and assertions on the same element states
3. **Control-Specific APIs**: Keywords encode control types (e.g., `Click Button`, `Check Checkbox`) rather than generic actions
4. **Technology Lock-in**: Keywords are tied to specific UI toolkits

The Robot Framework Browser Library has established a modern design pattern using:
- Assertion-engine powered `Get*` keywords with inline assertions
- Locator-driven interaction (locator determines target, not keyword name)
- Minimal keyword sets with high composability
- Built-in retry/wait semantics in assertion operations

This ADR defines the Domain-Driven Design architecture to reduce the keyword API from ~60-70 keywords to ~20-25 core keywords while maintaining full automation capabilities.

---

## Decision

We will implement a DDD-based architecture with five bounded contexts, four aggregate roots, four domain services, and supporting value objects and domain events. The architecture enables assertion-engine patterns while preserving backwards compatibility.

---

## Bounded Contexts

### Context Map

```
+------------------------------------------------------------------------+
|                          CORE SUBDOMAIN                                 |
|  +------------------------------------------------------------------+  |
|  |              KEYWORD EXECUTION CONTEXT                            |  |
|  |  Purpose: Unified action execution across all UI toolkits         |  |
|  |  Keywords: Click, TypeText, SelectItem, SetCheckbox, Clear        |  |
|  +------------------------------------------------------------------+  |
|                                    |                                    |
|               [Shared Kernel: KeywordCommand, ElementReference]         |
|                                    |                                    |
|  +------------------------------------------------------------------+  |
|  |              ASSERTION ENGINE CONTEXT                             |  |
|  |  Purpose: Get* keywords with inline assertions & auto-retry       |  |
|  |  Keywords: GetText, GetValue, GetElementStates, GetProperty       |  |
|  +------------------------------------------------------------------+  |
+------------------------------------------------------------------------+
                                     |
            +------------------------+------------------------+
            |                        |                        |
     [ACL: Locator                [Partnership]         [ACL: Session
      Normalization]                   |                 Lifecycle]
            |                        |                        |
+------------------------+  +------------------------+  +------------------------+
|  LOCATOR RESOLUTION    |  |    INTROSPECTION       |  |  SESSION MANAGEMENT    |
|  CONTEXT               |  |    CONTEXT             |  |  CONTEXT               |
|  (Generic Subdomain)   |  |  (Supporting Subdomain)|  |  (Supporting Subdomain)|
+------------------------+  +------------------------+  +------------------------+
| - CSS/XPath parsing    |  | - UI Tree traversal    |  | - Connect/Disconnect   |
| - Locator chaining     |  | - Debug logging        |  | - Connection pooling   |
| - Hierarchy filters    |  | - Element properties   |  | - Health monitoring    |
| - State predicates     |  | - Metadata extraction  |  | - Auto-reconnection    |
+------------------------+  +------------------------+  +------------------------+
            |                        |                        |
            +------------------------+------------------------+
                                     |
                    [Anti-Corruption Layer: Toolkit Adapters]
                                     |
+------------------------------------------------------------------------+
|                        INFRASTRUCTURE LAYER                             |
|  +------------------+  +------------------+  +------------------+       |
|  | Swing Adapter    |  | SWT Adapter      |  | RCP Adapter      |       |
|  | (JSON-RPC)       |  | (JSON-RPC)       |  | (extends SWT)    |       |
|  +------------------+  +------------------+  +------------------+       |
+------------------------------------------------------------------------+
```

### 1. Keyword Execution Context (Core)

**Purpose**: Provides unified, technology-agnostic action keywords.

**Ubiquitous Language**:
- **Action**: An operation performed on a UI element (click, type, select)
- **Target**: The element identified by a locator expression
- **Modifier**: Action variants (double-click, right-click, with-offset)

**Responsibilities**:
- Execute actions on resolved elements
- Handle action-specific options (click count, key modifiers)
- Coordinate with Locator Resolution Context
- Emit KeywordExecuted events

**Keywords** (reduced from ~30 action keywords):
| Keyword | Purpose | Replaces |
|---------|---------|----------|
| `Click` | Click any element | Click, Click Button, Click Cell, etc. |
| `Type Text` | Input text | Input Text, Type Into Textfield, etc. |
| `Clear Text` | Clear text content | Clear Text Field |
| `Select Item` | Select in list/combo/table/tree | Select From List, Select Table Row, etc. |
| `Set Checkbox` | Set checkbox state | Check Checkbox, Uncheck Checkbox |
| `Press Key` | Keyboard input | Type Text (special keys) |
| `Drag And Drop` | Drag operations | Drag And Drop, Drag Tree Node |

### 2. Assertion Engine Context (Core)

**Purpose**: Provides `Get*` keywords with inline assertion capabilities and auto-retry.

**Ubiquitous Language**:
- **Assertion Operator**: Comparison operation (should be, contains, greater than)
- **Expected Value**: The value to assert against
- **Auto-Retry**: Automatic polling until assertion passes or timeout
- **Soft Assertion**: Continue execution on failure (for reporting)

**Responsibilities**:
- Retrieve element values/states
- Evaluate assertions with configurable operators
- Implement retry logic with configurable timeout
- Emit AssertionEvaluated events

**Keywords** (replaces ~25 assertion/wait keywords):
| Keyword | Returns | Assertion Example |
|---------|---------|-------------------|
| `Get Text` | Element text | `Get Text    locator    should be    Expected` |
| `Get Value` | Control value | `Get Value    locator    contains    partial` |
| `Get Element States` | List of states | `Get Element States    locator    contains    visible` |
| `Get Element Count` | Count of matches | `Get Element Count    locator    >    0` |
| `Get Property` | Specific property | `Get Property    locator    tooltip    equals    Help` |
| `Get Table Cell` | Cell value | `Get Table Cell    locator    row=1    col=Name    should be    John` |
| `Get Selected Item` | Selected value | `Get Selected Item    locator    should be    Option A` |

**Assertion Operators**:
```
+------------------------------------------------------------------+
|                    ASSERTION OPERATORS                            |
+------------------------------------------------------------------+
| Equality          | should be, equals, ==                         |
| Inequality        | should not be, not equals, !=                 |
| Containment       | contains, should contain                       |
| Non-containment   | not contains, should not contain               |
| Comparison        | >, <, >=, <=, greater than, less than         |
| Pattern           | matches, should match (regex)                  |
| Collection        | contains item, has length                      |
| State             | is visible, is enabled, is selected            |
+------------------------------------------------------------------+
```

### 3. Locator Resolution Context (Generic Subdomain)

**Purpose**: Parse, normalize, and evaluate locator expressions.

**Ubiquitous Language**:
- **Locator**: CSS/XPath-like expression identifying elements
- **Locator Chain**: Hierarchical locator using `>>` combinator
- **State Filter**: Predicate applied to element state (`:visible`, `:enabled`)
- **Attribute Filter**: Predicate applied to attributes (`[text="OK"]`)

**Responsibilities**:
- Parse locator syntax (CSS, XPath, shorthand)
- Normalize locators across toolkits (JButton -> Button)
- Evaluate locators against UI tree
- Support chaining and filtering
- Emit ElementResolved events

**Locator Syntax**:
```
+------------------------------------------------------------------+
|                     LOCATOR SYNTAX                                |
+------------------------------------------------------------------+
| Type Selector     | JButton, Button, TextField                   |
| ID Selector       | #submitBtn, name:submitBtn                   |
| Text Selector     | text:Save, [text="Save"]                     |
| Index Selector    | index:0, :nth(2)                             |
| Attribute         | [tooltip="Help"], [enabled=true]             |
| Pseudo-class      | :visible, :enabled, :focused, :selected      |
| Chaining          | JPanel >> JButton, parent >> child           |
| Hierarchy Filter  | JPanel[name="toolbar"] >> JButton            |
| State Filter      | JButton:visible:enabled                      |
| Combined          | JPanel >> JButton[text="OK"]:enabled         |
+------------------------------------------------------------------+
```

### 4. Session Management Context (Supporting Subdomain)

**Purpose**: Manage application connections and lifecycle.

**Ubiquitous Language**:
- **Session**: Active connection to a Java application
- **Application**: Target Java process (Swing/SWT/RCP)
- **Health Check**: Periodic verification of connection status

**Responsibilities**:
- Establish and manage TCP connections
- Monitor connection health
- Handle reconnection logic
- Emit SessionStateChanged events

**Keywords** (unchanged - explicit lifecycle):
| Keyword | Purpose |
|---------|---------|
| `Connect To Application` | Establish connection |
| `Disconnect` | Close connection |
| `Is Connected` | Check connection state |
| `Get Connection Info` | Retrieve connection details |

### 5. Introspection Context (Supporting Subdomain)

**Purpose**: UI tree inspection and debugging capabilities.

**Ubiquitous Language**:
- **UI Tree**: Hierarchical structure of UI components
- **Element Metadata**: Properties, accessibility info, bounds
- **Debug Output**: Human-readable tree representation

**Responsibilities**:
- Traverse and cache UI component tree
- Extract element metadata and properties
- Provide debugging output
- Support test development tooling

**Keywords** (unchanged - diagnostic tools):
| Keyword | Purpose |
|---------|---------|
| `Get UI Tree` | Retrieve full tree structure |
| `Log UI Tree` | Output tree to log |
| `Refresh UI Tree` | Force tree refresh |
| `Get Element Properties` | All properties for debugging |
| `Find Element` | Advanced element lookup |
| `Find Elements` | Multiple element lookup |

---

## Aggregate Roots

### 1. KeywordCommand Aggregate

```
+------------------------------------------------------------------+
|                    KEYWORD COMMAND AGGREGATE                      |
+------------------------------------------------------------------+
|                                                                    |
|  +---------------------------+                                     |
|  | <<Aggregate Root>>        |                                     |
|  | KeywordCommand            |                                     |
|  +---------------------------+                                     |
|  | + command_id: CommandId   |  Identity                           |
|  | + keyword: KeywordType    |  Which keyword                      |
|  | + target: LocatorChain    |  Target element(s)                  |
|  | + arguments: Arguments    |  Keyword-specific args              |
|  | + options: CommandOptions |  Timeout, retry, etc.               |
|  | + status: CommandStatus   |  Pending/Executing/Complete/Failed  |
|  | + result: Option<Result>  |  Execution result                   |
|  | + events: Vec<DomainEvent>|  Raised events                      |
|  +---------------------------+                                     |
|  | + execute(ctx) -> Result  |  Execute the command                |
|  | + validate() -> Result    |  Validate before execution          |
|  | + with_timeout(dur)       |  Builder pattern                    |
|  | + with_retry(policy)      |  Builder pattern                    |
|  +---------------------------+                                     |
|              |                                                     |
|              | contains                                            |
|              v                                                     |
|  +------------------------+  +------------------------+            |
|  | <<Value Object>>       |  | <<Value Object>>       |            |
|  | CommandId              |  | Arguments              |            |
|  +------------------------+  +------------------------+            |
|  | + value: Uuid           |  | + positional: Vec<Arg>|            |
|  | + timestamp: Instant    |  | + named: HashMap       |            |
|  +------------------------+  +------------------------+            |
|                                                                    |
|  +------------------------+  +------------------------+            |
|  | <<Value Object>>       |  | <<Enumeration>>        |            |
|  | CommandOptions         |  | KeywordType            |            |
|  +------------------------+  +------------------------+            |
|  | + timeout: Duration     |  | Click                  |            |
|  | + retry_policy: Policy  |  | TypeText               |            |
|  | + soft_assert: bool     |  | SelectItem             |            |
|  | + screenshot_on_fail    |  | GetText                |            |
|  +------------------------+  | GetValue ...            |            |
|                              +------------------------+            |
+------------------------------------------------------------------+
```

**Invariants**:
- Command must have valid target locator
- Arguments must match keyword signature
- Timeout must be positive duration

### 2. AssertionSpec Aggregate

```
+------------------------------------------------------------------+
|                    ASSERTION SPEC AGGREGATE                       |
+------------------------------------------------------------------+
|                                                                    |
|  +---------------------------+                                     |
|  | <<Aggregate Root>>        |                                     |
|  | AssertionSpec             |                                     |
|  +---------------------------+                                     |
|  | + assertion_id: AssertId  |  Identity                           |
|  | + operator: AssertOp      |  Comparison operator                |
|  | + expected: ExpectedValue |  Expected value/pattern             |
|  | + actual: Option<Actual>  |  Captured actual value              |
|  | + timeout: Duration       |  Max wait for assertion             |
|  | + poll_interval: Duration |  Retry interval                     |
|  | + soft: bool              |  Soft assertion flag                |
|  | + result: AssertResult    |  Pass/Fail/Timeout                  |
|  +---------------------------+                                     |
|  | + evaluate(actual) -> bool|  Check if assertion passes          |
|  | + with_timeout(dur)       |  Configure timeout                  |
|  | + as_soft()               |  Mark as soft assertion             |
|  | + format_message() -> Str |  Human-readable result              |
|  +---------------------------+                                     |
|              |                                                     |
|              | contains                                            |
|              v                                                     |
|  +------------------------+  +------------------------+            |
|  | <<Value Object>>       |  | <<Enumeration>>        |            |
|  | ExpectedValue          |  | AssertionOperator      |            |
|  +------------------------+  +------------------------+            |
|  | + value: Value          |  | ShouldBe               |            |
|  | + pattern: Option<Regex>|  | ShouldNotBe            |            |
|  | + comparator: Option    |  | Contains               |            |
|  +------------------------+  | NotContains            |            |
|                              | GreaterThan            |            |
|  +------------------------+  | LessThan               |            |
|  | <<Value Object>>       |  | Matches                |            |
|  | AssertionResult        |  | HasLength              |            |
|  +------------------------+  +------------------------+            |
|  | + passed: bool          |                                       |
|  | + message: String       |                                       |
|  | + attempts: u32         |                                       |
|  | + duration: Duration    |                                       |
|  +------------------------+                                       |
|                                                                    |
+------------------------------------------------------------------+
```

**Invariants**:
- Operator must be compatible with expected value type
- Poll interval must be less than timeout
- Regex patterns must be valid

### 3. LocatorChain Aggregate

```
+------------------------------------------------------------------+
|                    LOCATOR CHAIN AGGREGATE                        |
+------------------------------------------------------------------+
|                                                                    |
|  +---------------------------+                                     |
|  | <<Aggregate Root>>        |                                     |
|  | LocatorChain              |                                     |
|  +---------------------------+                                     |
|  | + original: String        |  Original locator string            |
|  | + segments: Vec<Segment>  |  Parsed chain segments              |
|  | + toolkit_hint: Option    |  Target toolkit if specified        |
|  +---------------------------+                                     |
|  | + parse(str) -> Result    |  Parse locator string               |
|  | + normalize(toolkit)      |  Normalize for toolkit              |
|  | + evaluate(tree) -> Elems |  Find matching elements             |
|  | + chain(other) -> Self    |  Append another locator             |
|  | + with_filter(pred)       |  Add filter predicate               |
|  +---------------------------+                                     |
|              |                                                     |
|              | contains                                            |
|              v                                                     |
|  +------------------------+                                        |
|  | <<Entity>>             |                                        |
|  | LocatorSegment         |                                        |
|  +------------------------+                                        |
|  | + segment_type: SegType |                                       |
|  | + selector: Selector    |                                       |
|  | + filters: Vec<Filter>  |                                       |
|  | + combinator: Combinator|                                       |
|  +------------------------+                                        |
|              |                                                     |
|              | contains                                            |
|              v                                                     |
|  +------------------------+  +------------------------+            |
|  | <<Value Object>>       |  | <<Value Object>>       |            |
|  | Selector               |  | Filter                 |            |
|  +------------------------+  +------------------------+            |
|  | + type: SelectorType    |  | + filter_type: FType   |            |
|  | + value: String         |  | + attribute: Option    |            |
|  | + modifiers: Vec        |  | + operator: MatchOp    |            |
|  +------------------------+  | + value: FilterValue   |            |
|                              +------------------------+            |
|                                                                    |
|  +------------------------+  +------------------------+            |
|  | <<Enumeration>>        |  | <<Enumeration>>        |            |
|  | SelectorType           |  | Combinator             |            |
|  +------------------------+  +------------------------+            |
|  | TypeSelector (Button)   |  | Descendant (space)     |            |
|  | IdSelector (#name)      |  | Child (>)              |            |
|  | TextSelector (text:)    |  | Chain (>>)             |            |
|  | IndexSelector (index:)  |  | Adjacent (+)           |            |
|  | AttributeSelector ([])  |  +------------------------+            |
|  | HashCodeSelector (id:)  |                                       |
|  +------------------------+                                       |
|                                                                    |
+------------------------------------------------------------------+
```

**Invariants**:
- Locator string must be parseable
- Chain must have at least one segment
- Filters must reference valid attributes

### 4. ApplicationSession Aggregate

```
+------------------------------------------------------------------+
|                  APPLICATION SESSION AGGREGATE                    |
+------------------------------------------------------------------+
|                                                                    |
|  +---------------------------+                                     |
|  | <<Aggregate Root>>        |                                     |
|  | ApplicationSession        |                                     |
|  +---------------------------+                                     |
|  | + session_id: SessionId   |  Identity                           |
|  | + connection: Connection  |  Active connection                  |
|  | + toolkit: ToolkitType    |  Swing/SWT/RCP                      |
|  | + state: SessionState     |  Connected/Disconnected/Error       |
|  | + metadata: Metadata      |  App info, version, etc.            |
|  | + health: HealthStatus    |  Last health check result           |
|  +---------------------------+                                     |
|  | + connect(config) -> Res  |  Establish connection               |
|  | + disconnect() -> Result  |  Close connection                   |
|  | + send_request(req)       |  Send RPC request                   |
|  | + check_health() -> Health|  Verify connection                  |
|  | + reconnect() -> Result   |  Attempt reconnection               |
|  +---------------------------+                                     |
|              |                                                     |
|              | contains                                            |
|              v                                                     |
|  +------------------------+  +------------------------+            |
|  | <<Value Object>>       |  | <<Value Object>>       |            |
|  | Connection             |  | SessionMetadata        |            |
|  +------------------------+  +------------------------+            |
|  | + host: String          |  | + app_name: String     |            |
|  | + port: u16             |  | + process_id: Option   |            |
|  | + stream: TcpStream     |  | + toolkit_version: Str |            |
|  | + request_id: u64       |  | + connected_at: Instant|            |
|  +------------------------+  +------------------------+            |
|                                                                    |
|  +------------------------+  +------------------------+            |
|  | <<Enumeration>>        |  | <<Value Object>>       |            |
|  | SessionState           |  | HealthStatus           |            |
|  +------------------------+  +------------------------+            |
|  | Disconnected           |  | + healthy: bool        |            |
|  | Connecting             |  | + latency_ms: u64      |            |
|  | Connected              |  | + last_check: Instant  |            |
|  | Reconnecting           |  | + error: Option<String>|            |
|  | Error(reason)          |  +------------------------+            |
|  +------------------------+                                       |
|                                                                    |
+------------------------------------------------------------------+
```

**Invariants**:
- Only one active session per library instance
- Session must be connected before sending requests
- Health check interval must be positive

---

## Domain Services

### 1. KeywordDispatcherService

```
+------------------------------------------------------------------+
|                  KEYWORD DISPATCHER SERVICE                       |
+------------------------------------------------------------------+
|                                                                    |
|  +---------------------------+                                     |
|  | <<Domain Service>>        |                                     |
|  | KeywordDispatcherService  |                                     |
|  +---------------------------+                                     |
|  | - session: Session        |  Active session reference           |
|  | - locator_svc: LocatorSvc |  Locator resolver                   |
|  | - assertion_svc: AssertSvc|  Assertion engine                   |
|  +---------------------------+                                     |
|  | + dispatch(cmd) -> Result |  Execute keyword command            |
|  | + dispatch_click(...)     |  Click action                       |
|  | + dispatch_type_text(...) |  Text input action                  |
|  | + dispatch_select(...)    |  Selection action                   |
|  | + dispatch_get_text(...)  |  Get with optional assertion        |
|  | + dispatch_get_value(...) |  Get with optional assertion        |
|  +---------------------------+                                     |
|                                                                    |
|  Responsibilities:                                                 |
|  - Route keyword commands to appropriate handlers                  |
|  - Coordinate locator resolution                                   |
|  - Integrate assertion evaluation                                  |
|  - Handle retry logic                                              |
|  - Emit domain events                                              |
|                                                                    |
+------------------------------------------------------------------+
```

**Dispatch Flow**:
```
Robot Framework Keyword
         |
         v
+-------------------+
| KeywordCommand    |  1. Create command from keyword args
| (Aggregate)       |
+-------------------+
         |
         v
+-------------------+
| Dispatcher        |  2. Validate and route
| Service           |
+-------------------+
         |
    +----+----+
    |         |
    v         v
+-------+ +-------+
|Locator| |Assert |  3. Resolve target, evaluate assertion
|Service| |Service|
+-------+ +-------+
    |         |
    +----+----+
         |
         v
+-------------------+
| Session           |  4. Send RPC to Java agent
| (Aggregate)       |
+-------------------+
         |
         v
+-------------------+
| Result + Events   |  5. Return result, emit events
+-------------------+
```

### 2. AssertionEngineService

```
+------------------------------------------------------------------+
|                  ASSERTION ENGINE SERVICE                         |
+------------------------------------------------------------------+
|                                                                    |
|  +---------------------------+                                     |
|  | <<Domain Service>>        |                                     |
|  | AssertionEngineService    |                                     |
|  +---------------------------+                                     |
|  | - default_timeout: Dur    |  Default assertion timeout          |
|  | - default_poll: Duration  |  Default poll interval              |
|  +---------------------------+                                     |
|  | + evaluate(spec, actual)  |  Evaluate assertion                 |
|  | + evaluate_with_retry(...)| Evaluate with auto-retry            |
|  | + parse_assertion(args)   |  Parse keyword assertion args       |
|  | + create_spec(op, expect) |  Create AssertionSpec               |
|  +---------------------------+                                     |
|                                                                    |
|  Assertion Evaluation Flow:                                        |
|                                                                    |
|  Get Text    locator    should be    Expected    timeout=10s       |
|       |          |          |            |             |           |
|       v          v          v            v             v           |
|   [keyword]  [target]  [operator]   [expected]   [options]         |
|                                                                    |
|  1. Parse assertion arguments                                      |
|  2. Create AssertionSpec with operator + expected                  |
|  3. Loop until timeout:                                            |
|     a. Resolve element via locator                                 |
|     b. Get actual value                                            |
|     c. Evaluate assertion                                          |
|     d. If pass -> return success                                   |
|     e. If fail -> wait poll_interval, retry                        |
|  4. On timeout -> raise AssertionError                             |
|                                                                    |
+------------------------------------------------------------------+
```

**Operator Implementations**:
```rust
impl AssertionOperator {
    fn evaluate(&self, actual: &Value, expected: &Value) -> bool {
        match self {
            ShouldBe => actual == expected,
            ShouldNotBe => actual != expected,
            Contains => actual.as_str()
                .map(|s| s.contains(expected.as_str().unwrap_or("")))
                .unwrap_or(false),
            GreaterThan => actual.as_f64() > expected.as_f64(),
            LessThan => actual.as_f64() < expected.as_f64(),
            Matches => Regex::new(expected.as_str().unwrap_or(""))
                .map(|r| r.is_match(actual.as_str().unwrap_or("")))
                .unwrap_or(false),
            // ... other operators
        }
    }
}
```

### 3. LocatorResolverService

```
+------------------------------------------------------------------+
|                  LOCATOR RESOLVER SERVICE                         |
+------------------------------------------------------------------+
|                                                                    |
|  +---------------------------+                                     |
|  | <<Domain Service>>        |                                     |
|  | LocatorResolverService    |                                     |
|  +---------------------------+                                     |
|  | - parser: LocatorParser   |  Locator syntax parser              |
|  | - normalizer: Normalizer  |  Toolkit-specific normalization     |
|  | - cache: ElementCache     |  Resolved element cache             |
|  +---------------------------+                                     |
|  | + resolve(locator, tree)  |  Resolve single element             |
|  | + resolve_all(loc, tree)  |  Resolve all matching elements      |
|  | + parse(str) -> Chain     |  Parse locator string               |
|  | + normalize(chain, kit)   |  Normalize for toolkit              |
|  | + validate(locator)       |  Check locator validity             |
|  +---------------------------+                                     |
|                                                                    |
|  Resolution Algorithm:                                             |
|                                                                    |
|  1. Parse locator string to LocatorChain                           |
|  2. Normalize type selectors for target toolkit                    |
|     - JButton (Swing) <-> Button (SWT)                             |
|     - JTextField <-> Text                                          |
|  3. For each segment in chain:                                     |
|     a. Find elements matching selector                             |
|     b. Apply attribute filters                                     |
|     c. Apply state filters (:visible, :enabled)                    |
|     d. Apply index filter if specified                             |
|  4. Apply combinator to connect segments:                          |
|     - >> (chain): child/descendant relationship                    |
|     - >  (child): direct child only                                |
|     - (space): descendant search                                   |
|  5. Return matching element(s) or ElementNotFound error            |
|                                                                    |
+------------------------------------------------------------------+
```

### 4. UITreeIntrospectionService

```
+------------------------------------------------------------------+
|                  UI TREE INTROSPECTION SERVICE                    |
+------------------------------------------------------------------+
|                                                                    |
|  +---------------------------+                                     |
|  | <<Domain Service>>        |                                     |
|  | UITreeIntrospectionService|                                     |
|  +---------------------------+                                     |
|  | - tree_cache: TreeCache   |  Cached UI tree                     |
|  | - cache_ttl: Duration     |  Cache time-to-live                 |
|  +---------------------------+                                     |
|  | + get_tree() -> UITree    |  Get full UI tree                   |
|  | + refresh_tree() -> Tree  |  Force refresh                      |
|  | + get_element(id) -> Elem |  Get element by ID                  |
|  | + get_properties(elem)    |  Get all element properties         |
|  | + format_tree() -> String |  Debug tree output                  |
|  | + find_by_predicate(...)  |  Advanced search                    |
|  +---------------------------+                                     |
|                                                                    |
|  Tree Caching Strategy:                                            |
|                                                                    |
|  - Cache full tree on first request                                |
|  - Invalidate cache on:                                            |
|    * Explicit refresh request                                      |
|    * Cache TTL expiration                                          |
|    * Action that may modify tree (dialog open, navigation)         |
|  - Partial refresh for known subtrees                              |
|                                                                    |
+------------------------------------------------------------------+
```

---

## Value Objects

### Locator Value Object

```rust
/// Immutable locator with equality by value
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Locator {
    /// Original locator string
    pub original: String,
    /// Locator type (CSS, XPath, shorthand)
    pub locator_type: LocatorType,
    /// Primary selector value
    pub selector: String,
    /// Additional filters/predicates
    pub filters: Vec<LocatorFilter>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum LocatorType {
    /// CSS-like: JButton[text="OK"]
    Css,
    /// XPath: //JButton[@text='OK']
    XPath,
    /// Name shorthand: name:submitBtn or #submitBtn
    Name,
    /// Text shorthand: text:Submit
    Text,
    /// Index shorthand: index:0
    Index,
    /// Hash code: id:12345
    HashCode,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct LocatorFilter {
    pub attribute: String,
    pub operator: MatchOperator,
    pub value: String,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum MatchOperator {
    Equals,        // =
    NotEquals,     // !=
    Contains,      // *=
    StartsWith,    // ^=
    EndsWith,      // $=
    Regex,         // ~=
}
```

### AssertionOperator Value Object

```rust
/// Assertion comparison operators
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum AssertionOperator {
    // Equality
    ShouldBe,
    ShouldNotBe,
    Equals,
    NotEquals,

    // String operations
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    Matches,  // Regex

    // Numeric comparisons
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,

    // Collection operations
    HasLength,
    IsEmpty,
    ContainsItem,

    // State checks
    IsVisible,
    IsEnabled,
    IsSelected,
    IsFocused,
}

impl AssertionOperator {
    /// Parse operator from string (case-insensitive)
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "should be" | "shouldbe" | "==" | "equals" => Some(Self::ShouldBe),
            "should not be" | "!=" | "not equals" => Some(Self::ShouldNotBe),
            "contains" | "should contain" => Some(Self::Contains),
            ">" | "greater than" => Some(Self::GreaterThan),
            "<" | "less than" => Some(Self::LessThan),
            ">=" | "greater or equal" => Some(Self::GreaterOrEqual),
            "<=" | "less or equal" => Some(Self::LessOrEqual),
            "matches" | "should match" => Some(Self::Matches),
            "has length" | "length" => Some(Self::HasLength),
            "is empty" | "empty" => Some(Self::IsEmpty),
            _ => None,
        }
    }
}
```

### ElementState Value Object

```rust
/// Immutable element state snapshot
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ElementState {
    pub visible: bool,
    pub enabled: bool,
    pub focused: bool,
    pub selected: Option<bool>,
    pub editable: Option<bool>,
    pub expanded: Option<bool>,  // For trees
}

impl ElementState {
    /// Convert to list of state strings (for Get Element States keyword)
    pub fn to_state_list(&self) -> Vec<String> {
        let mut states = Vec::new();
        if self.visible { states.push("visible".into()); }
        if self.enabled { states.push("enabled".into()); }
        if self.focused { states.push("focused".into()); }
        if self.selected == Some(true) { states.push("selected".into()); }
        if self.editable == Some(true) { states.push("editable".into()); }
        if self.expanded == Some(true) { states.push("expanded".into()); }
        states
    }

    /// Check if state contains specific state
    pub fn has_state(&self, state: &str) -> bool {
        match state.to_lowercase().as_str() {
            "visible" => self.visible,
            "enabled" => self.enabled,
            "focused" => self.focused,
            "selected" => self.selected.unwrap_or(false),
            "editable" => self.editable.unwrap_or(false),
            "expanded" => self.expanded.unwrap_or(false),
            _ => false,
        }
    }
}
```

---

## Domain Events

```rust
/// Domain events raised during keyword execution
#[derive(Clone, Debug)]
pub enum DomainEvent {
    /// Keyword execution completed
    KeywordExecuted {
        command_id: CommandId,
        keyword: KeywordType,
        target: String,
        duration: Duration,
        success: bool,
    },

    /// Assertion evaluated (pass or fail)
    AssertionEvaluated {
        assertion_id: AssertionId,
        operator: AssertionOperator,
        expected: String,
        actual: String,
        passed: bool,
        attempts: u32,
        duration: Duration,
    },

    /// Element resolved from locator
    ElementResolved {
        locator: String,
        element_count: usize,
        duration: Duration,
        from_cache: bool,
    },

    /// Session state changed
    SessionStateChanged {
        session_id: SessionId,
        previous_state: SessionState,
        new_state: SessionState,
        reason: Option<String>,
    },

    /// UI tree refreshed
    UITreeRefreshed {
        element_count: usize,
        duration: Duration,
    },
}
```

**Event Flow**:
```
+------------------------------------------------------------------+
|                      EVENT FLOW                                   |
+------------------------------------------------------------------+

  Keyword Call
       |
       v
  +----------+  raises   +-------------------+
  | Aggregate | -------> | KeywordExecuted   |
  +----------+           | AssertionEvaluated|
       |                 | ElementResolved   |
       |                 +-------------------+
       v                          |
  +----------+                    v
  | Domain   |           +-------------------+
  | Service  |           | Event Publisher   |
  +----------+           +-------------------+
                                  |
              +-------------------+-------------------+
              |                   |                   |
              v                   v                   v
       +------------+      +------------+      +------------+
       | Logger     |      | Metrics    |      | Screenshot |
       | Subscriber |      | Collector  |      | Handler    |
       +------------+      +------------+      +------------+
```

---

## Integration Patterns

### Robot Framework Keyword Integration

```python
# Python keyword wrapper using PyO3
@keyword("Get Text")
def get_text(
    self,
    locator: str,
    assertion_operator: Optional[str] = None,
    expected: Optional[str] = None,
    timeout: Optional[str] = None,
    message: Optional[str] = None,
) -> str:
    """
    Get element text with optional assertion.

    Examples:
        | ${text}= | Get Text | name:status |
        | Get Text | name:status | should be | Ready |
        | Get Text | name:status | contains | Success | timeout=10s |
    """
    # Create KeywordCommand
    command = KeywordCommand.new(
        keyword=KeywordType.GetText,
        target=locator,
        assertion=AssertionSpec.parse(assertion_operator, expected) if assertion_operator else None,
        options=CommandOptions.from_kwargs(timeout=timeout),
    )

    # Dispatch through domain service
    result = self.dispatcher.dispatch(command)

    # Return actual value (assertion already evaluated)
    return result.value
```

### Backwards Compatibility Layer

```rust
/// Compatibility aliases for legacy keywords
impl SwingLibrary {
    /// Legacy: Element Should Be Visible
    /// Maps to: Get Element States    locator    contains    visible
    #[pyo3(name = "element_should_be_visible")]
    pub fn element_should_be_visible_legacy(&self, locator: &str) -> PyResult<()> {
        // Emit deprecation warning
        warn_deprecated("Element Should Be Visible",
                       "Get Element States    locator    contains    visible");

        // Delegate to new implementation
        self.get_element_states(locator, Some("contains"), Some("visible"), None)?;
        Ok(())
    }

    /// Legacy: Click Button
    /// Maps to: Click    locator
    #[pyo3(name = "click_button")]
    pub fn click_button_legacy(&self, locator: &str) -> PyResult<()> {
        warn_deprecated("Click Button", "Click");
        self.click(locator, None, None)
    }

    /// Legacy: Wait Until Element Exists
    /// Maps to: Get Element Count    locator    >    0    timeout=X
    #[pyo3(name = "wait_until_element_exists")]
    pub fn wait_until_element_exists_legacy(
        &self,
        locator: &str,
        timeout: Option<f64>
    ) -> PyResult<()> {
        warn_deprecated("Wait Until Element Exists",
                       "Get Element Count    locator    >    0");
        self.get_element_count(locator, Some(">"), Some("0"), timeout)?;
        Ok(())
    }
}
```

---

## Module Boundaries

```
src/
+-- lib.rs                          # PyO3 module registration
|
+-- domain/                         # DOMAIN LAYER (Pure business logic)
|   +-- mod.rs
|   +-- aggregates/
|   |   +-- mod.rs
|   |   +-- keyword_command.rs      # KeywordCommand aggregate
|   |   +-- assertion_spec.rs       # AssertionSpec aggregate
|   |   +-- locator_chain.rs        # LocatorChain aggregate
|   |   +-- application_session.rs  # ApplicationSession aggregate
|   |
|   +-- services/
|   |   +-- mod.rs
|   |   +-- keyword_dispatcher.rs   # KeywordDispatcherService
|   |   +-- assertion_engine.rs     # AssertionEngineService
|   |   +-- locator_resolver.rs     # LocatorResolverService
|   |   +-- ui_tree_introspection.rs# UITreeIntrospectionService
|   |
|   +-- value_objects/
|   |   +-- mod.rs
|   |   +-- locator.rs              # Locator, LocatorFilter
|   |   +-- assertion_operator.rs   # AssertionOperator
|   |   +-- element_state.rs        # ElementState
|   |   +-- command_options.rs      # CommandOptions, Timeout
|   |
|   +-- events/
|   |   +-- mod.rs
|   |   +-- domain_events.rs        # All domain events
|   |   +-- event_publisher.rs      # Event publishing infrastructure
|   |
|   +-- traits.rs                   # Shared kernel traits
|
+-- application/                    # APPLICATION LAYER (Use cases)
|   +-- mod.rs
|   +-- keywords/
|   |   +-- mod.rs
|   |   +-- action_keywords.rs      # Click, TypeText, etc.
|   |   +-- assertion_keywords.rs   # GetText, GetValue, etc.
|   |   +-- session_keywords.rs     # Connect, Disconnect
|   |   +-- introspection_keywords.rs # GetUITree, LogUITree
|   |
|   +-- compatibility/
|       +-- mod.rs
|       +-- legacy_keywords.rs      # Deprecated keyword aliases
|       +-- migration_guide.rs      # Migration helpers
|
+-- infrastructure/                 # INFRASTRUCTURE LAYER (Technical)
|   +-- mod.rs
|   +-- adapters/
|   |   +-- mod.rs
|   |   +-- swing_adapter.rs        # Swing RPC implementation
|   |   +-- swt_adapter.rs          # SWT RPC implementation
|   |   +-- rcp_adapter.rs          # RCP extension
|   |
|   +-- rpc/
|   |   +-- mod.rs
|   |   +-- client.rs               # JSON-RPC client
|   |   +-- protocol.rs             # Wire protocol
|   |
|   +-- locator/
|   |   +-- mod.rs
|   |   +-- parser.rs               # Locator parsing
|   |   +-- normalizer.rs           # Toolkit normalization
|   |   +-- matcher.rs              # Element matching
|   |
|   +-- screenshot.rs               # Screenshot capture
|   +-- logging.rs                  # Robot Framework logging
|
+-- python/                         # PYTHON BINDINGS
    +-- mod.rs
    +-- swing_library.rs            # SwingLibrary #[pyclass]
    +-- swt_library.rs              # SwtLibrary #[pyclass]
    +-- rcp_library.rs              # RcpLibrary #[pyclass]
    +-- element.rs                  # Element wrappers
    +-- exceptions.rs               # Python exceptions
```

---

## Consequences

### Positive Consequences

1. **Reduced Keyword Count**: From ~60-70 to ~20-25 core keywords
2. **Unified Mental Model**: Same patterns as Browser Library
3. **Built-in Assertions**: No separate wait/assert keywords needed
4. **Improved Readability**: Test cases are more concise
5. **Auto-Retry**: Assertions automatically retry until timeout
6. **Flexible Locators**: Chaining and filtering without new keywords
7. **Preserved Capabilities**: All automation features remain accessible
8. **Clear Module Boundaries**: Domain logic isolated from infrastructure
9. **Testable Design**: Domain services can be unit tested without RPC

### Negative Consequences

1. **Learning Curve**: Existing users must learn new patterns
2. **Migration Effort**: Existing tests need updates (compatibility layer helps)
3. **Implementation Complexity**: Assertion engine requires careful design
4. **Breaking Changes**: Some keyword signatures will change
5. **Documentation**: Extensive documentation updates required

### Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| User resistance to change | Comprehensive compatibility layer, migration guides |
| Assertion edge cases | Extensive test coverage for all operators |
| Performance regression | Caching, optimized locator resolution |
| Breaking existing tests | Deprecation warnings, 2-version transition period |

---

## Implementation Plan

### Phase 1: Foundation (Weeks 1-2)
- Implement domain value objects (Locator, AssertionOperator, ElementState)
- Create aggregate root skeletons
- Define domain event types
- Set up module structure

### Phase 2: Core Services (Weeks 3-4)
- Implement AssertionEngineService with all operators
- Implement LocatorResolverService with chaining
- Add auto-retry logic
- Unit test domain services

### Phase 3: Keyword Migration (Weeks 5-6)
- Implement new unified keywords
- Create compatibility layer for legacy keywords
- Integration testing with test applications
- Performance optimization

### Phase 4: Documentation & Release (Week 7-8)
- Update keyword documentation
- Create migration guide
- Write deprecation notices
- Release as beta for community feedback

---

## References

- [Robot Framework Browser Library](https://robotframework-browser.org/)
- [Domain-Driven Design by Eric Evans](https://domainlanguage.com/ddd/)
- [Existing DDD Architecture Design](../architecture/DDD_ARCHITECTURE_DESIGN.md)
- [Keyword Simplification Research](../research/Update_Keywords.md)
- [Unified Library Architecture](../architecture/UNIFIED_LIBRARY_ARCHITECTURE.md)

---

*Document Version: 1.0*
*Status: Proposed*
*Last Updated: 2026-01-19*
