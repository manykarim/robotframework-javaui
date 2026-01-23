# Domain-Driven Design Architecture for robotframework-javagui

## Executive Summary

This document presents a comprehensive Domain-Driven Design (DDD) architecture for unifying the robotframework-javagui library across three Java GUI toolkits: Swing, SWT, and Eclipse RCP. The design achieves approximately 70% keyword unification while maintaining backwards compatibility and technology-specific extensibility.

---

## 1. Strategic Design: Bounded Contexts

### 1.1 Context Map Overview

```
+------------------------------------------------------------------+
|                        CORE DOMAIN                                |
|  +------------------------------------------------------------+  |
|  |              Unified Automation Domain                      |  |
|  |  - Element abstraction                                      |  |
|  |  - Action execution                                         |  |
|  |  - Verification operations                                  |  |
|  |  - Wait strategies                                          |  |
|  +------------------------------------------------------------+  |
+------------------------------------------------------------------+
           |                    |                    |
     [Shared Kernel]     [Shared Kernel]      [Shared Kernel]
           |                    |                    |
+------------------+  +------------------+  +------------------+
| SUPPORTING       |  | SUPPORTING       |  | SUPPORTING       |
| DOMAIN: Swing    |  | DOMAIN: SWT      |  | DOMAIN: RCP      |
|                  |  |                  |  | (Conformist)     |
| - JComponent     |  | - Widget         |  | - Workbench      |
| - EDT handling   |  | - Display thread |  | - Perspective    |
| - SwingBaseType  |  | - SwtWidgetType  |  | - View/Editor    |
+------------------+  +------------------+  +------------------+
           |                    |                    |
           +--------------------+--------------------+
                               |
+------------------------------------------------------------------+
|                    GENERIC SUBDOMAINS                             |
|  +----------------+  +----------------+  +------------------+     |
|  | Locator        |  | Connection     |  | Protocol         |     |
|  | - CSS/XPath    |  | - RPC Client   |  | - JSON-RPC       |     |
|  | - Normalization|  | - Health Check |  | - Request/Resp   |     |
|  | - Matching     |  | - Reconnection |  | - Serialization  |     |
|  +----------------+  +----------------+  +------------------+     |
|                                                                   |
|  +----------------+  +----------------+  +------------------+     |
|  | Screenshot     |  | Logging        |  | Configuration    |     |
|  | - Capture      |  | - UI Tree      |  | - Timeout        |     |
|  | - Storage      |  | - Debug output |  | - Settings       |     |
|  +----------------+  +----------------+  +------------------+     |
+------------------------------------------------------------------+
```

### 1.2 Bounded Context Definitions

#### 1.2.1 Core Domain: Unified Automation

**Purpose**: Provides the unified, technology-agnostic API for GUI automation.

**Responsibilities**:
- Define unified keyword interfaces (Click, InputText, SelectFromList, etc.)
- Abstract element representation
- Common verification and assertion logic
- Wait strategy coordination
- Screenshot and debugging capabilities

**Key Aggregates**:
- `Element` - Unified UI element representation
- `Table` - Table data access aggregate
- `Tree` - Hierarchical navigation aggregate
- `Action` - Command execution aggregate

#### 1.2.2 Supporting Domain: Swing

**Purpose**: Provides Swing-specific implementations and extensions.

**Responsibilities**:
- Swing component type mapping
- EDT (Event Dispatch Thread) synchronization
- Swing-specific properties (AccessibleContext, ButtonModel, etc.)
- Legacy keyword compatibility

**Key Entities**:
- `SwingElement` - Swing JComponent wrapper
- `SwingConnection` - Swing agent communication
- `ComponentInspector` - UI tree traversal

#### 1.2.3 Supporting Domain: SWT

**Purpose**: Provides SWT widget toolkit implementations.

**Responsibilities**:
- SWT widget type mapping
- Display thread synchronization
- SWT style bit interpretation
- Shell management

**Key Entities**:
- `SwtWidget` - SWT Widget wrapper
- `SwtConnection` - SWT agent communication
- `WidgetInspector` - Widget tree traversal

#### 1.2.4 Supporting Domain: RCP (Conformist)

**Purpose**: Extends SWT with Eclipse RCP workbench concepts.

**Relationship**: Conformist to SWT - adopts SWT model without modification.

**Responsibilities**:
- Workbench management
- Perspective switching
- View/Editor lifecycle
- Eclipse command execution
- Preference page navigation

**Key Entities**:
- `Workbench` - Eclipse workbench state
- `Perspective` - Perspective management
- `View` / `Editor` - Part management
- `EclipseCommand` - Command execution

#### 1.2.5 Generic Subdomain: Locator

**Purpose**: CSS/XPath-like locator parsing and matching.

**Responsibilities**:
- Locator syntax parsing
- AST representation
- Technology-neutral matching algorithm
- Locator normalization (Anti-Corruption Layer)

**Value Objects**:
- `Locator` - Parsed locator AST
- `AttributeSelector` - Attribute matching
- `PseudoSelector` - State-based selection
- `Combinator` - Structural relationships

#### 1.2.6 Generic Subdomain: Connection

**Purpose**: Agent communication management.

**Responsibilities**:
- RPC client lifecycle
- Connection pooling
- Health monitoring
- Automatic reconnection

---

## 2. Tactical Design: Domain Model

### 2.1 Core Domain Aggregates

#### 2.1.1 Element Aggregate (Aggregate Root: UnifiedElement)

```
+------------------------------------------------------------------+
|                     ELEMENT AGGREGATE                             |
+------------------------------------------------------------------+
|                                                                   |
|  +------------------------+                                       |
|  | <<Aggregate Root>>     |                                       |
|  | UnifiedElement         |                                       |
|  +------------------------+                                       |
|  | + id: ElementId        |                                       |
|  | + type: ElementType    |                                       |
|  | + identity: Identity   |                                       |
|  | + state: ElementState  |                                       |
|  | + geometry: Geometry   |                                       |
|  +------------------------+                                       |
|  | + click()              |                                       |
|  | + inputText(text)      |                                       |
|  | + getText()            |                                       |
|  | + isEnabled()          |                                       |
|  | + isVisible()          |                                       |
|  +------------------------+                                       |
|              |                                                    |
|              | contains                                           |
|              v                                                    |
|  +------------------------+  +------------------------+           |
|  | <<Value Object>>       |  | <<Value Object>>       |           |
|  | ElementId              |  | Identity               |           |
|  +------------------------+  +------------------------+           |
|  | + handle: i64          |  | + name: Option<String> |           |
|  | + treePath: String     |  | + text: Option<String> |           |
|  | + depth: u32           |  | + tooltip: Option<Str> |           |
|  +------------------------+  +------------------------+           |
|                                                                   |
|  +------------------------+  +------------------------+           |
|  | <<Value Object>>       |  | <<Value Object>>       |           |
|  | ElementState           |  | Geometry               |           |
|  +------------------------+  +------------------------+           |
|  | + visible: bool        |  | + bounds: Bounds       |           |
|  | + enabled: bool        |  | + center: (i32, i32)   |           |
|  | + focused: bool        |  +------------------------+           |
|  | + selected: Option     |                                       |
|  +------------------------+                                       |
|                                                                   |
+------------------------------------------------------------------+
```

#### 2.1.2 Table Aggregate (Aggregate Root: Table)

```
+------------------------------------------------------------------+
|                      TABLE AGGREGATE                              |
+------------------------------------------------------------------+
|                                                                   |
|  +------------------------+                                       |
|  | <<Aggregate Root>>     |                                       |
|  | Table                  |                                       |
|  +------------------------+                                       |
|  | + element: UnifiedElem |                                       |
|  | + columns: Vec<Column> |                                       |
|  | + rowCount: usize      |                                       |
|  | + selectedRows: Vec    |                                       |
|  +------------------------+                                       |
|  | + getCell(row, col)    |                                       |
|  | + selectRow(row)       |                                       |
|  | + selectCell(row,col)  |                                       |
|  | + getRowCount()        |                                       |
|  | + getColumnCount()     |                                       |
|  +------------------------+                                       |
|              |                                                    |
|              | contains                                           |
|              v                                                    |
|  +------------------------+  +------------------------+           |
|  | <<Entity>>             |  | <<Value Object>>       |           |
|  | TableRow               |  | TableColumn            |           |
|  +------------------------+  +------------------------+           |
|  | + index: usize         |  | + index: usize         |           |
|  | + cells: Vec<Cell>     |  | + header: String       |           |
|  | + selected: bool       |  | + width: i32           |           |
|  +------------------------+  +------------------------+           |
|                                                                   |
|  +------------------------+                                       |
|  | <<Value Object>>       |                                       |
|  | TableCell              |                                       |
|  +------------------------+                                       |
|  | + row: usize           |                                       |
|  | + column: usize        |                                       |
|  | + value: String        |                                       |
|  +------------------------+                                       |
|                                                                   |
+------------------------------------------------------------------+
```

#### 2.1.3 Tree Aggregate (Aggregate Root: Tree)

```
+------------------------------------------------------------------+
|                       TREE AGGREGATE                              |
+------------------------------------------------------------------+
|                                                                   |
|  +------------------------+                                       |
|  | <<Aggregate Root>>     |                                       |
|  | Tree                   |                                       |
|  +------------------------+                                       |
|  | + element: UnifiedElem |                                       |
|  | + roots: Vec<TreeNode> |                                       |
|  | + selectedPaths: Vec   |                                       |
|  +------------------------+                                       |
|  | + expandNode(path)     |                                       |
|  | + collapseNode(path)   |                                       |
|  | + selectNode(path)     |                                       |
|  | + getSelectedNode()    |                                       |
|  | + findNode(path)       |                                       |
|  +------------------------+                                       |
|              |                                                    |
|              | contains                                           |
|              v                                                    |
|  +------------------------+                                       |
|  | <<Entity>>             |                                       |
|  | TreeNode               |                                       |
|  +------------------------+                                       |
|  | + path: NodePath       |                                       |
|  | + text: String         |                                       |
|  | + expanded: bool       |                                       |
|  | + children: Vec<Node>  |                                       |
|  | + parent: Option<Path> |                                       |
|  +------------------------+                                       |
|                                                                   |
|  +------------------------+                                       |
|  | <<Value Object>>       |                                       |
|  | NodePath               |                                       |
|  +------------------------+                                       |
|  | + segments: Vec<String>|                                       |
|  | + separator: char      |                                       |
|  +------------------------+                                       |
|                                                                   |
+------------------------------------------------------------------+
```

### 2.2 Value Objects

#### 2.2.1 Locator Value Object

```
+------------------------------------------------------------------+
|                    LOCATOR VALUE OBJECTS                          |
+------------------------------------------------------------------+
|                                                                   |
|  +------------------------+                                       |
|  | <<Value Object>>       |                                       |
|  | Locator                |       Immutable, equality by value    |
|  +------------------------+                                       |
|  | + selectors: Vec       |                                       |
|  | + original: String     |                                       |
|  | + isXPath: bool        |                                       |
|  +------------------------+                                       |
|  | + parse(str) -> Locator|                                       |
|  | + isUniversal() -> bool|                                       |
|  | + toString() -> String |                                       |
|  +------------------------+                                       |
|              |                                                    |
|              | composed of                                        |
|              v                                                    |
|  +------------------------+  +------------------------+           |
|  | <<Value Object>>       |  | <<Value Object>>       |           |
|  | ComplexSelector        |  | CompoundSelector       |           |
|  +------------------------+  +------------------------+           |
|  | + compounds: Vec       |  | + typeSelector: Opt    |           |
|  +------------------------+  | + idSelector: Option   |           |
|                              | + attributes: Vec      |           |
|                              | + pseudos: Vec         |           |
|                              | + combinator: Option   |           |
|                              +------------------------+           |
|                                                                   |
|  +------------------------+  +------------------------+           |
|  | <<Value Object>>       |  | <<Enumeration>>        |           |
|  | AttributeSelector      |  | MatchOperator          |           |
|  +------------------------+  +------------------------+           |
|  | + name: String         |  | Equals                 |           |
|  | + operator: MatchOp    |  | PrefixMatch (^=)       |           |
|  | + value: AttrValue     |  | SuffixMatch ($=)       |           |
|  +------------------------+  | SubstringMatch (*=)    |           |
|                              | NotEquals (!=)         |           |
|                              +------------------------+           |
|                                                                   |
+------------------------------------------------------------------+
```

#### 2.2.2 Timeout and Configuration Value Objects

```
+------------------------------------------------------------------+
|                 CONFIGURATION VALUE OBJECTS                       |
+------------------------------------------------------------------+
|                                                                   |
|  +------------------------+  +------------------------+           |
|  | <<Value Object>>       |  | <<Value Object>>       |           |
|  | Timeout                |  | Coordinates            |           |
|  +------------------------+  +------------------------+           |
|  | + duration: Duration   |  | + x: i32               |           |
|  | + pollInterval: Dur    |  | + y: i32               |           |
|  +------------------------+  +------------------------+           |
|  | + isExpired() -> bool  |  | + offset(dx, dy)       |           |
|  | + remaining() -> Dur   |  | + distance(other)      |           |
|  +------------------------+  +------------------------+           |
|                                                                   |
|  +------------------------+  +------------------------+           |
|  | <<Value Object>>       |  | <<Value Object>>       |           |
|  | Bounds                 |  | ConnectionConfig       |           |
|  +------------------------+  +------------------------+           |
|  | + x: i32               |  | + host: String         |           |
|  | + y: i32               |  | + port: u16            |           |
|  | + width: i32           |  | + timeout: Timeout     |           |
|  | + height: i32          |  | + retryPolicy: Policy  |           |
|  +------------------------+  +------------------------+           |
|  | + center() -> Coords   |                                       |
|  | + contains(point)      |                                       |
|  | + intersects(other)    |                                       |
|  +------------------------+                                       |
|                                                                   |
+------------------------------------------------------------------+
```

### 2.3 Domain Services

```
+------------------------------------------------------------------+
|                      DOMAIN SERVICES                              |
+------------------------------------------------------------------+
|                                                                   |
|  +----------------------------+                                   |
|  | <<Domain Service>>         |                                   |
|  | ElementFinder              |                                   |
|  +----------------------------+                                   |
|  | + find(locator): Element   |  Single element lookup            |
|  | + findAll(locator): Vec    |  Multiple element lookup          |
|  | + exists(locator): bool    |  Existence check                  |
|  | + waitFor(loc, timeout)    |  Wait with polling                |
|  +----------------------------+                                   |
|                                                                   |
|  +----------------------------+                                   |
|  | <<Domain Service>>         |                                   |
|  | ActionExecutor             |                                   |
|  +----------------------------+                                   |
|  | + click(element, opts)     |  Click actions                    |
|  | + input(element, text)     |  Text input                       |
|  | + select(element, value)   |  Selection actions                |
|  | + drag(from, to)           |  Drag and drop                    |
|  +----------------------------+                                   |
|                                                                   |
|  +----------------------------+                                   |
|  | <<Domain Service>>         |                                   |
|  | LocatorMatcher             |                                   |
|  +----------------------------+                                   |
|  | + matches(elem, loc): bool |  Test if element matches          |
|  | + evaluate(ctx, loc): Vec  |  Evaluate against tree            |
|  | + normalize(loc): Locator  |  Normalize syntax                 |
|  +----------------------------+                                   |
|                                                                   |
|  +----------------------------+                                   |
|  | <<Domain Service>>         |                                   |
|  | Verifier                   |                                   |
|  +----------------------------+                                   |
|  | + shouldBeVisible(elem)    |  Visibility assertion             |
|  | + shouldBeEnabled(elem)    |  Enabled assertion                |
|  | + textShouldBe(elem, txt)  |  Text assertion                   |
|  | + shouldExist(locator)     |  Existence assertion              |
|  +----------------------------+                                   |
|                                                                   |
+------------------------------------------------------------------+
```

---

## 3. Context Mapping

### 3.1 Context Relationships

```
+------------------------------------------------------------------+
|                      CONTEXT MAP                                  |
+------------------------------------------------------------------+

                    +-------------------+
                    | CORE DOMAIN       |
                    | Unified Automation|
                    +-------------------+
                           /|\
          _________________|_________________
         /                 |                 \
        /                  |                  \
  [Shared Kernel]    [Shared Kernel]    [Shared Kernel]
       /                   |                   \
      v                    v                    v
+------------+      +------------+      +------------+
| SWING      |      | SWT        |<-----| RCP        |
| Supporting |      | Supporting |      | (Conformist)|
| Domain     |      | Domain     |      | Domain     |
+------------+      +------------+      +------------+
      \                   |                   /
       \                  |                  /
        \                 |                 /
    [Anti-Corruption Layer: Locator Normalization]
                          |
                    +------------+
                    | LOCATOR    |
                    | Generic    |
                    | Subdomain  |
                    +------------+

Legend:
  -----> : Conformist (downstream adopts upstream model)
  <----> : Shared Kernel (shared model)
  [ACL]  : Anti-Corruption Layer (translation)
```

### 3.2 Shared Kernel

The Shared Kernel contains abstractions used across all technology contexts:

```rust
// Shared Kernel: Common Traits

/// Unified element trait - implemented by Swing, SWT, RCP
pub trait GuiElement {
    fn id(&self) -> &ElementId;
    fn element_type(&self) -> ElementType;
    fn text(&self) -> Option<&str>;
    fn name(&self) -> Option<&str>;
    fn is_visible(&self) -> bool;
    fn is_enabled(&self) -> bool;
    fn bounds(&self) -> &Bounds;
}

/// Unified table trait
pub trait TableElement: GuiElement {
    fn row_count(&self) -> usize;
    fn column_count(&self) -> usize;
    fn cell_value(&self, row: usize, col: usize) -> String;
    fn select_row(&self, row: usize) -> Result<()>;
    fn select_cell(&self, row: usize, col: usize) -> Result<()>;
}

/// Unified tree trait
pub trait TreeElement: GuiElement {
    fn expand(&self, path: &NodePath) -> Result<()>;
    fn collapse(&self, path: &NodePath) -> Result<()>;
    fn select(&self, path: &NodePath) -> Result<()>;
    fn selected_path(&self) -> Option<NodePath>;
}

/// Unified element finder
pub trait ElementFinder {
    type Element: GuiElement;
    fn find(&self, locator: &Locator) -> Result<Self::Element>;
    fn find_all(&self, locator: &Locator) -> Result<Vec<Self::Element>>;
}
```

### 3.3 Customer-Supplier Relationships

The Core Domain is the **downstream customer** of the Supporting Domains:

```
                 Core Domain (Customer)
                        |
         +------+------+------+
         |      |      |      |
         v      v      v      v
      Swing   SWT   RCP   Locator
    (Supplier)(Supplier)(Supplier)(Supplier)
```

Each Supporting Domain **supplies** concrete implementations:

```rust
// Swing supplies SwingElement implementing GuiElement
impl GuiElement for SwingElement { ... }

// SWT supplies SwtWidget implementing GuiElement
impl GuiElement for SwtWidget { ... }

// RCP extends SWT (Conformist relationship)
// RcpElement wraps SwtWidget but adds workbench context
pub struct RcpElement {
    widget: SwtWidget,
    view_id: Option<String>,
    editor_title: Option<String>,
}
```

### 3.4 Anti-Corruption Layer: Locator Normalization

The ACL translates between technology-specific locator syntax:

```
+------------------------------------------------------------------+
|             ANTI-CORRUPTION LAYER: LOCATOR NORMALIZATION          |
+------------------------------------------------------------------+
|                                                                   |
|  Input Locators (Technology-Specific)                             |
|  +------------------------+                                       |
|  | Swing: JButton#submit  | --> Normalizer --> Button#submit      |
|  | SWT:   Button#submit   | --> (pass-through)                    |
|  | RCP:   view:Navigator  | --> [type='view'][id='Navigator']     |
|  +------------------------+                                       |
|                                                                   |
|  Type Mappings:                                                   |
|  +-------------------+-------------------+-------------------+    |
|  | Swing             | SWT               | Unified           |    |
|  +-------------------+-------------------+-------------------+    |
|  | JButton           | Button            | Button            |    |
|  | JTextField        | Text              | TextField         |    |
|  | JComboBox         | Combo             | ComboBox          |    |
|  | JList             | List              | List              |    |
|  | JTable            | Table             | Table             |    |
|  | JTree             | Tree              | Tree              |    |
|  | JTabbedPane       | TabFolder         | TabFolder         |    |
|  | JCheckBox         | Button(CHECK)     | CheckBox          |    |
|  | JRadioButton      | Button(RADIO)     | RadioButton       |    |
|  | JProgressBar      | ProgressBar       | ProgressBar       |    |
|  | JSpinner          | Spinner           | Spinner           |    |
|  | JSlider           | Scale/Slider      | Slider            |    |
|  +-------------------+-------------------+-------------------+    |
|                                                                   |
|  Attribute Mappings:                                              |
|  +-------------------+-------------------+-------------------+    |
|  | Swing             | SWT               | Unified           |    |
|  +-------------------+-------------------+-------------------+    |
|  | getName()         | getData("name")   | @name             |    |
|  | getText()         | getText()         | @text             |    |
|  | isEnabled()       | isEnabled()       | :enabled          |    |
|  | isVisible()       | isVisible()       | :visible          |    |
|  | getToolTipText()  | getToolTipText()  | @tooltip          |    |
|  +-------------------+-------------------+-------------------+    |
|                                                                   |
+------------------------------------------------------------------+
```

---

## 4. Layered Architecture

### 4.1 Hexagonal Architecture Overview

```
+------------------------------------------------------------------+
|                    HEXAGONAL ARCHITECTURE                         |
+------------------------------------------------------------------+

                         +-------------------+
                         |  Robot Framework  |
                         |  (Primary Port)   |
                         +-------------------+
                                  |
                                  v
+------------------------------------------------------------------+
|                        APPLICATION LAYER                          |
|  +------------------------------------------------------------+  |
|  |  SwingLibrary    |  SwtLibrary      |  RcpLibrary          |  |
|  |  (Python API)    |  (Python API)    |  (Python API)        |  |
|  +------------------------------------------------------------+  |
+------------------------------------------------------------------+
                                  |
                                  v
+------------------------------------------------------------------+
|                         DOMAIN LAYER                              |
|  +------------------------------------------------------------+  |
|  |  Unified Keywords  |  Domain Services  |  Aggregates       |  |
|  |  - Click           |  - ElementFinder  |  - Element        |  |
|  |  - InputText       |  - ActionExecutor |  - Table          |  |
|  |  - SelectFromList  |  - Verifier       |  - Tree           |  |
|  +------------------------------------------------------------+  |
+------------------------------------------------------------------+
                                  |
                                  v
+------------------------------------------------------------------+
|                     INFRASTRUCTURE LAYER                          |
|  +--------------------+  +------------------+  +---------------+  |
|  | Swing Adapter      |  | SWT Adapter      |  | RCP Adapter   |  |
|  | - RpcClient        |  | - SwtRpcClient   |  | - WorkbenchAPI|  |
|  | - ComponentInsp    |  | - WidgetInsp     |  | - CommandExec |  |
|  +--------------------+  +------------------+  +---------------+  |
|                                                                   |
|  +--------------------+  +------------------+  +---------------+  |
|  | Locator Adapter    |  | Config Adapter   |  | Screenshot    |  |
|  | - Parser           |  | - Settings       |  | - Capture     |  |
|  | - Matcher          |  | - Timeouts       |  | - Storage     |  |
|  +--------------------+  +------------------+  +---------------+  |
+------------------------------------------------------------------+
                                  |
                                  v
                         +-------------------+
                         |  Java Agents      |
                         |  (Secondary Port) |
                         +-------------------+
```

### 4.2 Module Structure

```
robotframework-javagui/
+-- src/
|   +-- lib.rs                      # PyO3 module entry point
|   |
|   +-- domain/                     # DOMAIN LAYER
|   |   +-- mod.rs
|   |   +-- element/                # Element aggregate
|   |   |   +-- mod.rs
|   |   |   +-- unified_element.rs  # Aggregate root
|   |   |   +-- element_id.rs       # Value object
|   |   |   +-- element_state.rs    # Value object
|   |   |   +-- element_type.rs     # Enumeration
|   |   |   +-- geometry.rs         # Value object
|   |   |
|   |   +-- table/                  # Table aggregate
|   |   |   +-- mod.rs
|   |   |   +-- table.rs            # Aggregate root
|   |   |   +-- table_row.rs        # Entity
|   |   |   +-- table_cell.rs       # Value object
|   |   |
|   |   +-- tree/                   # Tree aggregate
|   |   |   +-- mod.rs
|   |   |   +-- tree.rs             # Aggregate root
|   |   |   +-- tree_node.rs        # Entity
|   |   |   +-- node_path.rs        # Value object
|   |   |
|   |   +-- services/               # Domain services
|   |   |   +-- mod.rs
|   |   |   +-- element_finder.rs
|   |   |   +-- action_executor.rs
|   |   |   +-- verifier.rs
|   |   |
|   |   +-- traits.rs               # Shared kernel traits
|   |
|   +-- application/                # APPLICATION LAYER
|   |   +-- mod.rs
|   |   +-- unified_keywords.rs     # Technology-agnostic keywords
|   |   +-- swing_keywords.rs       # Swing-specific keywords
|   |   +-- swt_keywords.rs         # SWT-specific keywords
|   |   +-- rcp_keywords.rs         # RCP-specific keywords
|   |
|   +-- infrastructure/             # INFRASTRUCTURE LAYER
|   |   +-- mod.rs
|   |   +-- adapters/
|   |   |   +-- swing_adapter.rs    # Swing implementation
|   |   |   +-- swt_adapter.rs      # SWT implementation
|   |   |   +-- rcp_adapter.rs      # RCP implementation
|   |   |
|   |   +-- rpc/
|   |   |   +-- client.rs           # RPC client
|   |   |   +-- protocol.rs         # Wire protocol
|   |   |
|   |   +-- locator/
|   |   |   +-- parser.rs           # Locator parsing
|   |   |   +-- matcher.rs          # Element matching
|   |   |   +-- normalizer.rs       # ACL: syntax normalization
|   |   |
|   |   +-- screenshot.rs           # Screenshot capture
|   |
|   +-- python/                     # PYTHON BINDINGS
|       +-- mod.rs
|       +-- swing_library.rs        # Robot Framework library
|       +-- swt_library.rs
|       +-- rcp_library.rs
|       +-- element.rs              # Python element wrapper
|       +-- exceptions.rs           # Exception types
|
+-- agent/                          # JAVA AGENTS
    +-- src/main/java/
        +-- com/robotframework/
            +-- swing/              # Swing agent
            +-- swt/                # SWT agent
            +-- rcp/                # RCP extensions
```

---

## 5. Ubiquitous Language

### 5.1 Core Terms

| Term | Definition | Context |
|------|------------|---------|
| **Element** | A unified representation of a UI component, abstracting Swing JComponent and SWT Widget differences | Core Domain |
| **Locator** | A CSS/XPath-like expression used to find elements in the UI tree | Generic Subdomain |
| **Action** | An operation performed on an element (click, input, select) | Core Domain |
| **Verification** | An assertion about element state (visible, enabled, text equals) | Core Domain |
| **Wait Strategy** | A polling mechanism to wait for element conditions | Core Domain |

### 5.2 Element Terms

| Term | Definition | Swing | SWT |
|------|------------|-------|-----|
| **Name** | The component identifier set by the developer | getName() | getData("name") |
| **Text** | The displayed text content | getText() | getText() |
| **Enabled** | Whether the element can receive user input | isEnabled() | isEnabled() |
| **Visible** | Whether the element is currently visible | isVisible() | isVisible() |
| **Focused** | Whether the element has keyboard focus | hasFocus() | isFocusControl() |
| **Selected** | Selection state (for toggleable elements) | isSelected() | getSelection() |

### 5.3 Control Terms

| Term | Definition | Swing | SWT |
|------|------------|-------|-----|
| **Button** | A clickable push button | JButton | Button(PUSH) |
| **CheckBox** | A toggleable checkbox | JCheckBox | Button(CHECK) |
| **RadioButton** | A mutually exclusive radio button | JRadioButton | Button(RADIO) |
| **TextField** | A single-line text input | JTextField | Text(SINGLE) |
| **TextArea** | A multi-line text input | JTextArea | Text(MULTI) |
| **ComboBox** | A dropdown selection control | JComboBox | Combo |
| **List** | A scrollable list of items | JList | List |
| **Table** | A grid of rows and columns | JTable | Table |
| **Tree** | A hierarchical node structure | JTree | Tree |
| **TabFolder** | A container with tabbed panels | JTabbedPane | TabFolder |
| **Menu** | A dropdown or context menu | JMenu/JPopupMenu | Menu |
| **ProgressBar** | A progress indicator | JProgressBar | ProgressBar |
| **Slider** | A sliding value selector | JSlider | Scale/Slider |
| **Spinner** | A numeric input with increment buttons | JSpinner | Spinner |

### 5.4 RCP-Specific Terms

| Term | Definition |
|------|------------|
| **Workbench** | The Eclipse RCP application window container |
| **Perspective** | A predefined arrangement of views and editors |
| **View** | A non-editor part showing information (e.g., Navigator, Outline) |
| **Editor** | A part for editing documents with save/dirty state |
| **Part** | Generic term for View or Editor |
| **Command** | An Eclipse action that can be executed programmatically |
| **Preference Page** | A settings configuration panel |

### 5.5 Locator Terms

| Term | Definition | Example |
|------|------------|---------|
| **Type Selector** | Matches by component class name | `JButton`, `Button` |
| **ID Selector** | Matches by component name | `#submitBtn` |
| **Attribute Selector** | Matches by property value | `[text='OK']` |
| **Pseudo Selector** | Matches by state | `:enabled`, `:visible` |
| **Child Combinator** | Direct parent-child relationship | `JPanel > JButton` |
| **Descendant Combinator** | Any ancestor-descendant relationship | `JFrame JButton` |

---

## 6. Implementation Guidelines

### 6.1 Keyword Unification Strategy

**Phase 1: Core Keywords (70% unified)**

These keywords have identical semantics across all technologies:

```
Unified Keywords (implemented once, adapters per technology):
- Click / Double Click / Right Click
- Input Text / Clear Text / Type Text
- Get Element Text / Get Element Property
- Element Should Be Visible / Enabled / Exist
- Wait Until Element Exists / Is Visible / Is Enabled
- Find Element / Find Elements
- Capture Screenshot
```

**Phase 2: Control-Specific Keywords (technology-aware)**

These keywords adapt to control differences:

```
Table Keywords (unified interface, adapted implementation):
- Get Table Cell Value
- Select Table Row / Select Table Cell
- Get Table Row Count / Get Table Column Count

Tree Keywords (unified interface, adapted implementation):
- Expand Tree Node / Collapse Tree Node
- Select Tree Node
- Get Selected Tree Node

Selection Keywords (control-type aware):
- Select From ComboBox (JComboBox/Combo)
- Select From List (JList/List)
- Check Checkbox / Uncheck Checkbox
- Select Radio Button
```

**Phase 3: Technology-Specific Keywords (not unified)**

```
Swing-Only:
- Select Menu (JMenuBar path traversal)
- Select From Popup Menu

SWT-Only:
- Get Shells / Activate Shell / Close Shell

RCP-Only:
- Open Perspective / Reset Perspective
- Show View / Close View / Activate View
- Open Editor / Close Editor / Save Editor
- Execute Command
- Open Preferences / Navigate To Preference Page
```

### 6.2 Backwards Compatibility

To maintain backwards compatibility:

1. **Preserve existing keyword names** - Old tests continue to work
2. **Support legacy locator syntax** - `name=foo` alongside `#foo`
3. **Deprecate gracefully** - Log warnings for deprecated patterns
4. **Version aliases** - `SwingLibrary` still works, maps to `Swing`

```python
# Legacy support in Python wrapper
class SwingLibrary:
    """Backwards-compatible alias for Swing"""
    def __init__(self, *args, **kwargs):
        warnings.warn("SwingLibrary is deprecated, use Swing", DeprecationWarning)
        self._lib = Swing(*args, **kwargs)
```

### 6.3 Error Handling

Domain-specific exceptions:

```rust
// Domain Exceptions
pub enum GuiAutomationError {
    // Element Errors
    ElementNotFound { locator: String },
    MultipleElementsFound { locator: String, count: usize },
    ElementNotVisible { locator: String },
    ElementNotEnabled { locator: String },

    // Connection Errors
    ConnectionFailed { host: String, port: u16, cause: String },
    ConnectionLost { cause: String },
    AgentNotResponding { timeout_ms: u64 },

    // Locator Errors
    LocatorParseError { locator: String, position: usize, message: String },

    // Action Errors
    ActionFailed { action: String, element: String, cause: String },

    // Timeout Errors
    TimeoutError { operation: String, timeout_ms: u64 },

    // Verification Errors
    AssertionFailed { expected: String, actual: String },
}
```

---

## 7. Migration Path

### 7.1 Phase 1: Foundation (Weeks 1-2)

1. Implement shared kernel traits
2. Create unified Element aggregate
3. Build locator normalization ACL
4. Refactor existing code to use new domain model

### 7.2 Phase 2: Unification (Weeks 3-4)

1. Implement unified keywords in application layer
2. Create technology-specific adapters
3. Add backwards compatibility wrappers
4. Migrate tests to use unified API

### 7.3 Phase 3: Enhancement (Weeks 5-6)

1. Complete RCP conformist implementation
2. Add remaining technology-specific keywords
3. Performance optimization
4. Documentation and examples

---

## 8. Appendix: Diagrams

### 8.1 Element Type Hierarchy

```
                    GuiElement (trait)
                         |
          +------+-------+-------+------+
          |      |       |       |      |
       Button  Text   Select   Table   Tree
          |      |       |       |      |
    +-----+    +---+   +---+   +---+  +---+
    |     |    |   |   |   |   |   |  |   |
  Push  Toggle Single Multi Combo List Row  Node
  Check Radio
```

### 8.2 Request Flow

```
Robot Framework Test
        |
        v
+------------------+
| SwingLibrary     |  Python API
| (Application)    |
+------------------+
        |
        v
+------------------+
| Unified Keywords |  Domain Layer
| ElementFinder    |
| ActionExecutor   |
+------------------+
        |
        v
+------------------+
| SwingAdapter     |  Infrastructure
| RpcClient        |
+------------------+
        |
        v (JSON-RPC)
+------------------+
| SwingAgent       |  Java Agent
| ActionExecutor   |
| ComponentInsp    |
+------------------+
        |
        v
+------------------+
| Swing Application|  Target App
+------------------+
```

---

## 9. Glossary

| Abbreviation | Full Term |
|--------------|-----------|
| DDD | Domain-Driven Design |
| ACL | Anti-Corruption Layer |
| EDT | Event Dispatch Thread |
| RCP | Rich Client Platform |
| SWT | Standard Widget Toolkit |
| API | Application Programming Interface |
| RPC | Remote Procedure Call |
| AST | Abstract Syntax Tree |

---

*Document Version: 1.0*
*Last Updated: 2026-01-16*
*Author: System Architecture Designer*
