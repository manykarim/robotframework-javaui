# Cascaded Selector Test Plan
## Robot Framework Test Suite for CASCADED_SELECTOR_SPECIFICATION.md

**Version:** 1.0.0
**Date:** 2026-01-21
**Target:** Swing Test Application
**Test Runner:** `uv run`

---

## Test Organization

### Directory Structure
```
tests/robot/swing/
├── resources/
│   └── cascaded_selectors.resource      # Shared resources for cascaded tests
├── 16_cascaded_basic.robot              # Basic cascaded selector tests
├── 17_cascaded_engines.robot            # Selector engine tests
├── 18_cascaded_capture.robot            # Capture prefix (*) tests
├── 19_cascaded_tables.robot             # Table-specific cascaded selectors
├── 20_cascaded_trees.robot              # Tree-specific cascaded selectors
├── 21_cascaded_tabs.robot               # Tab-specific cascaded selectors
├── 22_cascaded_menus.robot              # Menu-specific cascaded selectors
├── 23_cascaded_complex.robot            # Complex combination tests
└── 24_cascaded_performance.robot        # Performance and stress tests
```

---

## 1. Basic Cascaded Selector Tests (16_cascaded_basic.robot)

### 1.1 Simple Chaining (10 tests)
**Coverage:** Section 2.1 of specification

| Test Case | Locator | Description | Tags |
|-----------|---------|-------------|------|
| Two-Segment Cascade | `JPanel >> JButton` | Basic parent-child | smoke, positive |
| Three-Segment Cascade | `JFrame >> JPanel >> JButton` | Multi-level chain | smoke, positive |
| Four-Segment Cascade | `JFrame >> JTabbedPane >> JPanel >> JButton` | Deep hierarchy | positive |
| Cascade With Name Attributes | `JPanel[name='main'] >> JButton[name='submit']` | Attribute matching | smoke, positive |
| Cascade With Text Attributes | `JPanel >> JButton[text='Submit']` | Text matching | positive |
| Cascade Mixed Attributes | `JDialog[title='Settings'] >> JPanel[name='form'] >> JButton` | Multiple attrs | positive |
| Direct Child Only | `JPanel > JButton` | CSS child combinator | positive |
| Descendant Any Level | `JPanel JButton` | Space combinator | positive |
| Cascade With Type Only | `JDialog >> JPanel >> JButton >> JLabel` | Type-based chain | positive |
| Empty Result Cascade | `JDialog[name='nonexistent'] >> JButton` | No match scenario | negative |

### 1.2 Whitespace Handling (5 tests)
**Coverage:** Section 2.3 of specification

| Test Case | Variants Tested | Tags |
|-----------|----------------|------|
| No Whitespace Around Separator | `JPanel>>JButton` | edge-case |
| Single Space Around Separator | `JPanel >> JButton` | positive |
| Multiple Spaces Around Separator | `JPanel  >>  JButton` | edge-case |
| Tab Characters Around Separator | `JPanel\t>>\tJButton` | edge-case |
| Mixed Whitespace | Various combinations | edge-case |

---

## 2. Selector Engine Tests (17_cascaded_engines.robot)

### 2.1 CSS Engine (Default) - 15 tests
**Coverage:** Section 3.2 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Type Selector Cascade | `JButton >> JLabel` | Basic type | smoke, css-engine |
| ID Selector Cascade | `#mainPanel >> #submitBtn` | ID syntax | smoke, css-engine |
| Class Selector Cascade | `.primary >> .secondary` | Class syntax | css-engine |
| Attribute Equals | `JButton[text='OK'] >> JLabel` | Exact match | css-engine |
| Attribute Contains | `JButton[text*='Log'] >> JLabel` | Substring match | css-engine |
| Attribute Starts With | `JButton[text^='Log'] >> JLabel` | Prefix match | css-engine |
| Attribute Ends With | `JLabel[text$=':'] >> JTextField` | Suffix match | css-engine |
| Pseudo Enabled | `JButton:enabled >> JLabel` | State pseudo | css-engine |
| Pseudo Visible | `JButton:visible >> JLabel` | Visibility pseudo | css-engine |
| Pseudo First Child | `JButton:first-child >> JLabel` | Position pseudo | css-engine |
| Pseudo Nth Child | `JButton:nth-child(2) >> JLabel` | Index pseudo | css-engine |
| Child Combinator | `JPanel > JButton >> JLabel` | Direct child | css-engine |
| Descendant Combinator | `JPanel JButton >> JLabel` | Any descendant | css-engine |
| Multiple Pseudo | `JButton:enabled:visible >> JLabel` | Combined pseudo | css-engine |
| Complex CSS Chain | Full CSS feature cascade | All features | css-engine, complex |

### 2.2 Class Engine - 8 tests
**Coverage:** Section 3.3 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Simple Class Cascade | `class=JDialog >> class=JPanel >> class=JButton` | Basic class match | smoke, class-engine |
| Class Without J Prefix | `class=Dialog >> class=Panel >> class=Button` | J prefix optional | class-engine |
| Mixed Case Class | `class=jButton >> class=JLabel` | Case insensitive | class-engine |
| Class Then CSS | `class=JDialog >> JButton[text='OK']` | Engine mixing | class-engine |
| CSS Then Class | `JDialog >> class=JButton` | Reverse mixing | class-engine |
| Class With Explicit Prefix | `css=class=JButton` | Explicit CSS | edge-case |
| Class Invalid Component | `class=NonExistentComponent >> JButton` | No match | negative |
| Multiple Class Segments | `class=JFrame >> class=JPanel >> class=JButton >> class=JLabel` | Long chain | class-engine |

### 2.3 Name Engine - 10 tests
**Coverage:** Section 3.4 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Simple Name Cascade | `name=mainDialog >> name=buttonPanel >> name=okButton` | Basic name | smoke, name-engine |
| Name With Quotes | `name='my-component' >> name='sub-component'` | Quoted names | name-engine |
| Name Wildcard Prefix | `name=user* >> JButton` | Wildcard match | name-engine |
| Name Case Sensitive | `name=MyButton` vs `name=mybutton` | Case sensitivity | name-engine |
| Name Then CSS | `name=formPanel >> JButton[text='Submit']` | Engine mixing | name-engine |
| CSS Then Name | `JPanel >> name=submitButton` | Reverse mixing | name-engine |
| Name With Spaces | `name='form panel' >> name='ok button'` | Spaced names | name-engine |
| Name Nonexistent | `name=nonexistent >> JButton` | No match | negative |
| Name Empty String | `name='' >> JButton` | Empty name | edge-case |
| Deep Name Chain | 5+ name segments | Long chain | name-engine |

### 2.4 Text Engine - 12 tests
**Coverage:** Section 3.5 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Simple Text Cascade | `text=File >> text=Save` | Basic text | smoke, text-engine |
| Text With Spaces | `text='Log Out' >> JLabel` | Spaced text | text-engine |
| Text Regex Pattern | `text=/Log.*/ >> JButton` | Regex support | text-engine |
| Text Partial Match | `text=*partial* >> JButton` | Wildcard match | text-engine |
| Text Exact Match | `text=ExactText >> JLabel` | Exact matching | text-engine |
| Text Case Sensitive | `text=Submit` vs `text=submit` | Case handling | text-engine |
| Text Then CSS | `text=OK >> JLabel[name='status']` | Engine mixing | text-engine |
| CSS Then Text | `JButton >> text=Submit` | Reverse mixing | text-engine |
| Text In Menu | `JMenu >> text=File >> JMenuItem >> text=Save` | Menu navigation | text-engine |
| Text Empty | `text='' >> JButton` | Empty text | edge-case |
| Text Special Chars | `text='Name:' >> JTextField` | Special chars | text-engine |
| Text Nonexistent | `text=NonexistentText >> JButton` | No match | negative |

### 2.5 Index Engine - 10 tests
**Coverage:** Section 3.6 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Index First Element | `JButton >> index=0` | First index | smoke, index-engine |
| Index Middle Element | `JButton >> index=2` | Middle index | index-engine |
| Index Last Element | `JButton >> index=-1` | Negative index | index-engine |
| Index Second Last | `JButton >> index=-2` | Negative offset | index-engine |
| Index Out Of Range | `JButton >> index=9999` | Invalid index | negative |
| Index Then CSS | `JPanel >> index=0 >> JButton[text='OK']` | Engine mixing | index-engine |
| CSS Then Index | `JPanel >> JButton >> index=1` | Select nth result | index-engine |
| Table Row Index | `JTable >> row >> index=5 >> cell >> index=0` | Table specific | index-engine |
| Multiple Index | `JPanel >> index=0 >> JButton >> index=0` | Repeated index | index-engine |
| Index Zero | `JButton >> index=0` | First element | index-engine |

### 2.6 XPath Engine - 12 tests
**Coverage:** Section 3.7 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| XPath Relative Child | `xpath=./JButton >> JLabel` | Child axis | smoke, xpath-engine |
| XPath Parent | `xpath=../JButton` | Parent axis | xpath-engine |
| XPath Descendant | `xpath=descendant::JButton >> JLabel` | Descendant axis | xpath-engine |
| XPath Ancestor | `xpath=ancestor::JPanel >> JButton` | Ancestor axis | xpath-engine |
| XPath Following Sibling | `xpath=following-sibling::JButton` | Sibling axis | xpath-engine |
| XPath Preceding Sibling | `xpath=preceding-sibling::JButton` | Preceding sibling | xpath-engine |
| XPath With Predicate | `xpath=.//JButton[@text='OK']` | Attribute filter | xpath-engine |
| XPath Index | `xpath=.//JButton[1]` | XPath index | xpath-engine |
| XPath Then CSS | `xpath=.//JPanel >> JButton[text='OK']` | Engine mixing | xpath-engine |
| CSS Then XPath | `JPanel >> xpath=.//JButton` | Reverse mixing | xpath-engine |
| Table Cell XPath | `JTable >> xpath=.//td[1] >> text=Active` | Table context | xpath-engine |
| XPath Invalid | `xpath=///invalid//` | Invalid syntax | negative |

### 2.7 ID Engine - 8 tests
**Coverage:** Section 3.8 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Simple ID Cascade | `id=mainWindow >> id=contentPane >> id=submitButton` | Basic ID | smoke, id-engine |
| ID With Quotes | `id='component-id' >> JButton` | Quoted ID | id-engine |
| ID Case Sensitive | `id=MyButton` vs `id=mybutton` | Case handling | id-engine |
| ID Vs CSS ID | `id=submitBtn` vs `#submitBtn` | Syntax difference | id-engine |
| ID Then CSS | `id=formPanel >> JButton[text='OK']` | Engine mixing | id-engine |
| CSS Then ID | `JPanel >> id=okButton` | Reverse mixing | id-engine |
| ID Nonexistent | `id=nonexistent >> JButton` | No match | negative |
| ID Empty | `id='' >> JButton` | Empty ID | edge-case |

---

## 3. Capture Prefix Tests (18_cascaded_capture.robot)

### 3.1 Basic Capture - 10 tests
**Coverage:** Section 4 of specification

| Test Case | Locator | Expected Result | Tags |
|-----------|---------|----------------|------|
| Capture First Segment | `*JPanel >> JTextField` | Returns JPanel | smoke, capture |
| Capture Second Segment | `JDialog >> *JPanel >> JButton` | Returns JPanel | capture |
| Capture Last Segment | `JDialog >> JPanel >> *JButton` | Returns JButton | capture |
| Multiple Captures First Wins | `*JDialog >> *JPanel >> JButton` | Returns JDialog | capture |
| No Capture Returns Last | `JDialog >> JPanel >> JButton` | Returns JButton | capture |
| Capture With Name | `*JPanel[name='form'] >> JTextField` | Returns named panel | capture |
| Capture With Text | `*JButton[text='Submit'] >> JLabel` | Returns button | capture |
| Capture Intermediate Dialog | `*JDialog[title='Settings'] >> JPanel >> JButton` | Returns dialog | capture |
| Capture Table Row | `JTable >> *row >> cell[text='Active']` | Returns row | capture, table |
| Capture Then Reuse | Store and reuse captured element | Element reuse | capture |

### 3.2 Capture With Different Engines - 8 tests
**Coverage:** Section 4 combined with Section 3

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Capture Class Engine | `*class=JDialog >> class=JButton` | Class capture | capture, class-engine |
| Capture Name Engine | `*name=formPanel >> name=okButton` | Name capture | capture, name-engine |
| Capture Text Engine | `*text=Settings >> JButton` | Text capture | capture, text-engine |
| Capture Index Engine | `JPanel >> *index=0 >> JButton` | Index capture | capture, index-engine |
| Capture XPath Engine | `*xpath=.//JDialog >> JButton` | XPath capture | capture, xpath-engine |
| Capture ID Engine | `*id=mainPanel >> id=submitBtn` | ID capture | capture, id-engine |
| Mixed Engine Capture | `*class=JDialog >> name=panel >> JButton` | Multi-engine | capture, complex |
| Capture CSS Complex | `*JButton:enabled[text*='Log'] >> JLabel` | Complex CSS | capture, css-engine |

### 3.3 Capture Workflows - 8 tests
**Coverage:** Section 4.5 of specification

| Test Case | Description | Tags |
|-----------|-------------|------|
| Capture Container Multiple Ops | Get container, perform 3+ operations | smoke, capture, workflow |
| Capture Dialog Workflow | Capture dialog, interact with children | capture, workflow |
| Capture Panel Form Fill | Capture panel, fill multiple fields | capture, workflow |
| Capture Table Row Ops | Capture row, read multiple cells | capture, table, workflow |
| Capture Tree Node Ops | Capture node, verify children | capture, tree, workflow |
| Store Multiple Captures | Capture 3+ different elements | capture, workflow |
| Nested Capture Operations | Capture within captured context | capture, workflow |
| Capture Error Handling | Verify proper error when capture fails | capture, negative |

---

## 4. Table-Specific Tests (19_cascaded_tables.robot)

### 4.1 Cell Selection - 12 tests
**Coverage:** Section 5.1 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Cell By Row Col Index | `JTable >> cell[row=0, col=1]` | Index-based | smoke, table |
| Cell By Row Index Col Name | `JTable >> cell[row=0, col='Name']` | Column name | smoke, table |
| Cell Via Row Then Cell | `JTable >> row[index=5] >> cell[index=2]` | Chained | table |
| Cell With Table Name | `JTable[name='dataTable'] >> cell[row=0, col=0]` | Named table | table |
| Cell In Nested Table | `JPanel >> JTable >> cell[row=0, col=0]` | Nested context | table |
| Multiple Cells | Get 5+ cells via cascade | Iteration | table |
| Cell XPath Style | `JTable >> xpath=.//td[1]` | XPath cells | table, xpath-engine |
| Cell Nonexistent Row | `JTable >> cell[row=9999, col=0]` | Out of bounds | negative, table |
| Cell Nonexistent Col | `JTable >> cell[row=0, col=9999]` | Out of bounds | negative, table |
| Cell Invalid Column Name | `JTable >> cell[row=0, col='InvalidCol']` | Bad column name | negative, table |
| Cell Get Value | `JTable >> cell[row=0, col=0]` with value check | Value retrieval | table |
| Cell Click | `JTable >> cell[row=0, col=0]` then click | Interaction | table |

### 4.2 Row Selection - 10 tests
**Coverage:** Section 5.1 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Row By Index | `JTable >> row[index=0]` | Index | smoke, table |
| Row Contains Text | `JTable >> row[contains='Active']` | Text search | smoke, table |
| Row Selected | `JTable >> row:selected` | Selected state | table, pseudo |
| Row First | `JTable >> row:first` | First pseudo | table, pseudo |
| Row Last | `JTable >> row:last` | Last pseudo | table, pseudo |
| Row Then Cell | `JTable >> row[index=0] >> cell[index=1]` | Chained | table |
| Row In Named Table | `JTable[name='dataTable'] >> row[index=0]` | Named table | table |
| Row Nonexistent Index | `JTable >> row[index=9999]` | Out of bounds | negative, table |
| Row Contains No Match | `JTable >> row[contains='NoMatch']` | No match | negative, table |
| Multiple Rows | Get all rows, iterate | Iteration | table |

### 4.3 Column Selection - 8 tests
**Coverage:** Section 5.1 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Column By Name | `JTable >> column[name='Status']` | Name | smoke, table |
| Column By Index | `JTable >> column[index=3]` | Index | smoke, table |
| Column Header Cell | `JTable >> header >> cell[text='Name']` | Header | table |
| Column Then Cells | `JTable >> column[index=0] >> cell` | All column cells | table |
| Column In Named Table | `JTable[name='dataTable'] >> column[name='Name']` | Named table | table |
| Column Invalid Name | `JTable >> column[name='InvalidCol']` | No match | negative, table |
| Column Invalid Index | `JTable >> column[index=9999]` | Out of bounds | negative, table |
| All Columns | Iterate all columns | Iteration | table |

### 4.4 Table Pseudo-Classes - 6 tests
**Coverage:** Section 5.1 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Row First Pseudo | `JTable >> row:first` | First row | table, pseudo |
| Row Last Pseudo | `JTable >> row:last` | Last row | table, pseudo |
| Cell Selected Pseudo | `JTable >> cell:selected` | Selected cell | table, pseudo |
| Cell Editable Pseudo | `JTable >> cell:editable` | Editable cell | table, pseudo |
| Combined Pseudo | `JTable >> row:first >> cell:editable` | Multiple pseudo | table, pseudo |
| Pseudo No Match | `JTable >> row:selected` when none selected | No match | negative, table |

### 4.5 Table Complex Workflows - 8 tests
**Coverage:** Integration of table features

| Test Case | Description | Tags |
|-----------|-------------|------|
| Find Row By Cell Content | Search cell, get row | smoke, table, workflow |
| Navigate Table Grid | Iterate rows and columns | table, workflow |
| Edit Cell Via Cascade | Find and edit cell | table, workflow |
| Click Button In Row | `JTable >> row[contains='John'] >> JButton[text='Edit']` | table, workflow |
| Verify Column Data | Get all column values, verify | table, workflow |
| Table Sorting | Click header, verify order | table, workflow |
| Multi-Table Cascade | `JPanel >> JTable[name='orders'] >> cell[row=0, col=0]` | table, workflow |
| Table Filter Row | Filter rows by condition | table, workflow |

---

## 5. Tree-Specific Tests (20_cascaded_trees.robot)

### 5.1 Node Selection - 12 tests
**Coverage:** Section 5.2 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Node By Path Pipe | `JTree >> node[path='Root\|Child\|Leaf']` | Pipe separator | smoke, tree |
| Node By Path Slash | `JTree >> node[path='Root/Settings/Display']` | Slash separator | smoke, tree |
| Node By Text | `JTree >> node[text='Settings']` | Text match | tree |
| Node Text Contains | `JTree >> node[text*='Config']` | Partial text | tree |
| Node By Path Then Child | `JTree >> node[path='Root'] >> child[index=2]` | Child navigation | tree |
| Node Selected | `JTree >> node:selected` | Selected state | tree, pseudo |
| Node Expanded | `JTree >> node:expanded` | Expanded state | tree, pseudo |
| Node Collapsed | `JTree >> node:collapsed` | Collapsed state | tree, pseudo |
| Node By Level | `JTree >> node[level=2]` | Depth level | tree |
| Node Root | `JTree >> node:root` | Root pseudo | tree, pseudo |
| Node Leaf | `JTree >> node:leaf` | Leaf pseudo | tree, pseudo |
| Node Nonexistent Path | `JTree >> node[path='Invalid/Path']` | No match | negative, tree |

### 5.2 Tree Navigation - 10 tests
**Coverage:** Section 5.2 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Expand Parent Select Child | Multi-step navigation | smoke, tree, workflow |
| Navigate Deep Path | 4+ levels deep | tree |
| Sibling Navigation | Next/previous sibling | tree |
| Parent To Child | Parent then child access | tree |
| Root To Leaf | Full path traversal | tree |
| Named Tree Node | `JTree[name='nav'] >> node[path='Settings']` | Named tree | tree |
| Multiple Trees | Distinguish between trees | tree |
| Tree In Panel | `JPanel >> JTree >> node[path='Root']` | Nested context | tree |
| Dynamic Tree | Add node, then select | tree, dynamic |
| Tree Lazy Load | Expand lazy-loaded nodes | tree, dynamic |

### 5.3 Tree Pseudo-Classes - 6 tests
**Coverage:** Section 5.2 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Node Root Pseudo | `JTree >> node:root` | Root node | tree, pseudo |
| Node Leaf Pseudo | `JTree >> node:leaf` | Leaf nodes | tree, pseudo |
| Node Expanded Pseudo | `JTree >> node:expanded` | Expanded nodes | tree, pseudo |
| Node Collapsed Pseudo | `JTree >> node:collapsed` | Collapsed nodes | tree, pseudo |
| Node Selected Pseudo | `JTree >> node:selected` | Selected node | tree, pseudo |
| Combined Tree Pseudo | `JTree >> node:expanded:selected` | Multiple states | tree, pseudo |

### 5.4 Tree Workflows - 8 tests
**Coverage:** Integration of tree features

| Test Case | Description | Tags |
|-----------|-------------|------|
| Full Tree Expansion | Expand all nodes recursively | smoke, tree, workflow |
| Tree Search By Text | Search and select node | tree, workflow |
| Tree Multi-Select | Select multiple nodes | tree, workflow |
| Tree Drag Drop | Drag node to new parent | tree, workflow |
| Tree Context Menu | Right-click node, select menu | tree, workflow |
| Tree Filter | Filter visible nodes | tree, workflow |
| Tree With Icons | Verify icon presence | tree, workflow |
| Tree Edit Node | Edit node text inline | tree, workflow |

---

## 6. Tab-Specific Tests (21_cascaded_tabs.robot)

### 6.1 Tab Selection - 10 tests
**Coverage:** Section 5.3 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Tab By Title | `JTabbedPane >> tab[title='Settings']` | Title match | smoke, tabs |
| Tab By Index | `JTabbedPane >> tab[index=2]` | Index | smoke, tabs |
| Tab Selected | `JTabbedPane >> tab:selected` | Selected state | tabs, pseudo |
| Tab Then Content | `JTabbedPane >> tab[title='Login'] >> JPanel` | Content access | tabs |
| Named TabbedPane Tab | `JTabbedPane[name='main'] >> tab[title='Settings']` | Named pane | tabs |
| Nested Tabs | `JTabbedPane >> tab[title='Advanced'] >> JTabbedPane >> tab[title='Network']` | Nested tabs | tabs, complex |
| Tab Invalid Title | `JTabbedPane >> tab[title='Nonexistent']` | No match | negative, tabs |
| Tab Invalid Index | `JTabbedPane >> tab[index=9999]` | Out of bounds | negative, tabs |
| Multiple TabbedPanes | Distinguish between panes | tabs |
| Tab Icon Verification | Verify tab has icon | tabs |

### 6.2 Tab Content Access - 8 tests
**Coverage:** Section 5.3 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| Tab Content Panel | `JTabbedPane >> tab[title='Form'] >> JPanel` | Panel access | smoke, tabs |
| Tab Content Button | `JTabbedPane >> tab[title='Settings'] >> JButton[text='Apply']` | Button in tab | tabs |
| Tab Content TextField | `JTabbedPane >> tab[title='Login'] >> JTextField[name='user']` | Field in tab | tabs |
| Tab Content Table | `JTabbedPane >> tab[title='Data'] >> JTable` | Table in tab | tabs |
| Tab Content Tree | `JTabbedPane >> tab[title='Nav'] >> JTree` | Tree in tab | tabs |
| Nested Tab Content | `JTabbedPane >> tab >> JTabbedPane >> tab >> JButton` | Nested content | tabs, complex |
| Tab Content Named | `JTabbedPane >> tab[title='Settings'] >> JPanel[name='form']` | Named content | tabs |
| Tab Content Cascade Chain | 4+ segments ending in tab content | tabs, complex |

### 6.3 Tab Workflows - 6 tests
**Coverage:** Integration of tab features

| Test Case | Description | Tags |
|-----------|-------------|------|
| Navigate All Tabs | Iterate through all tabs | smoke, tabs, workflow |
| Fill Form In Tab | Select tab, fill form, submit | tabs, workflow |
| Switch Tabs Workflow | Switch between tabs multiple times | tabs, workflow |
| Tab Content Verification | Verify content in each tab | tabs, workflow |
| Close Tab | Close dynamically added tab | tabs, workflow, dynamic |
| Add Tab | Add new tab dynamically | tabs, workflow, dynamic |

---

## 7. Menu-Specific Tests (22_cascaded_menus.robot)

### 7.1 Menu Navigation - 12 tests
**Coverage:** Section 5.4 of specification

| Test Case | Locator | Feature Tested | Tags |
|-----------|---------|----------------|------|
| MenuBar Menu Item | `JMenuBar >> JMenu[text='File'] >> JMenuItem[text='Save']` | Basic menu | smoke, menus |
| Menu Text Cascade | `JMenuBar >> menu[text='Edit'] >> menu[text='Find'] >> item[text='Find Next']` | Submenu | smoke, menus |
| Menu By Index | `JMenuBar >> menu[index=0] >> item[index=1]` | Index-based | menus |
| Nested Menu | `JMenuBar >> JMenu >> JMenu >> JMenuItem` | Deep nesting | menus |
| PopupMenu Item | `JPopupMenu >> JMenuItem[text='Copy']` | Context menu | menus |
| PopupMenu Index | `JPopupMenu >> item[index=0]` | Popup by index | menus |
| Menu Separator | Navigate past separator | menus |
| Disabled Menu Item | `JMenu >> JMenuItem[text='Disabled']:disabled` | Disabled state | menus, pseudo |
| Menu Checkbox | `JMenu >> JCheckBoxMenuItem[text='Option']` | Checkbox item | menus |
| Menu Radio | `JMenu >> JRadioButtonMenuItem[text='View']` | Radio item | menus |
| Menu Invalid Path | `JMenuBar >> JMenu[text='Nonexistent']` | No match | negative, menus |
| Multiple MenuBars | Distinguish between menubars | menus |

### 7.2 Menu Workflows - 8 tests
**Coverage:** Integration of menu features

| Test Case | Description | Tags |
|-----------|-------------|------|
| File Menu Save Workflow | File > Save > Confirm | smoke, menus, workflow |
| Edit Menu Copy Paste | Edit > Copy, Edit > Paste | menus, workflow |
| View Menu Toggle | Toggle view options | menus, workflow |
| Help Menu About | Help > About dialog | menus, workflow |
| Menu Keyboard Navigation | Navigate with keyboard | menus, workflow |
| Right Click Menu | Right-click, select option | menus, workflow |
| Menu Accelerator | Verify keyboard shortcuts | menus, workflow |
| Menu State Persistence | Menu state across operations | menus, workflow |

---

## 8. Complex Combination Tests (23_cascaded_complex.robot)

### 8.1 Mixed Engine Combinations - 15 tests
**Coverage:** Integration of all engines

| Test Case | Locator | Engines Used | Tags |
|-----------|---------|--------------|------|
| CSS-Name-Text | `JPanel >> name=form >> text=Submit` | 3 engines | smoke, complex |
| Class-XPath-Index | `class=JDialog >> xpath=.//JButton >> index=0` | 3 engines | complex |
| Name-CSS-Text-Index | `name=main >> JPanel >> text=OK >> index=0` | 4 engines | complex |
| ID-Class-XPath | `id=main >> class=JPanel >> xpath=.//JButton` | 3 engines | complex |
| All Engines Chain | Cascade using all 7 engines | All 7 | complex |
| Text-Capture-Index | `*text=Settings >> JButton >> index=0` | Text+capture+index | complex |
| XPath-Capture-Name | `*xpath=.//JDialog >> name=okBtn` | XPath+capture+name | complex |
| Class-CSS-Pseudo | `class=JButton >> JButton:enabled >> text=OK` | Class+CSS+pseudo | complex |
| Engine Switching | Switch engines 5+ times | Multiple | complex |
| ID-Table-Cell | `id=dataTable >> cell[row=0, col=0]` | ID+table | complex |
| Name-Tree-Node | `name=navTree >> node[path='Root/Settings']` | Name+tree | complex |
| Class-Tab-Content | `class=JTabbedPane >> tab[title='Form'] >> JButton` | Class+tab | complex |
| XPath-Menu-Item | `xpath=.//JMenuBar >> JMenu >> JMenuItem` | XPath+menu | complex |
| Mixed With Attributes | All engines with attributes | All+attributes | complex |
| Mixed With Pseudo | All engines with pseudo | All+pseudo | complex |

### 8.2 Deep Hierarchies - 10 tests
**Coverage:** Performance with deep nesting

| Test Case | Depth | Description | Tags |
|-----------|-------|-------------|------|
| Five Level Cascade | 5 | `A >> B >> C >> D >> E` | complex |
| Seven Level Cascade | 7 | `A >> B >> C >> D >> E >> F >> G` | complex |
| Ten Level Cascade | 10 | Maximum practical depth | complex, performance |
| Nested Dialogs | 3+ | Dialog within dialog | complex |
| Nested Panels | 5+ | Panel within panel | complex |
| Tab In Dialog In Frame | 4 | Mixed component types | complex |
| Table In Tab In Panel | 4 | Data components | complex |
| Tree In Dialog In Panel | 4 | Tree nesting | complex |
| Menu In Frame In Panel | 4 | Menu nesting | complex |
| Deep With Capture | 7+ with capture | Capture in deep chain | complex, capture |

### 8.3 Complex Real-World Scenarios - 12 tests
**Coverage:** Practical use cases

| Test Case | Scenario | Tags |
|-----------|----------|------|
| Preferences Dialog Complete | Open prefs, navigate tabs, change settings, save | smoke, complex, workflow |
| Data Entry Form | Multi-tab form with validation | complex, workflow |
| Tree File Browser | Tree navigation, file operations | complex, workflow |
| Table Data Management | CRUD operations via cascade | complex, workflow |
| Multi-Panel Wizard | Step through wizard panels | complex, workflow |
| Dashboard Interaction | Multiple widgets, tabs, tables | complex, workflow |
| Report Generation | Navigate menus, configure, generate | complex, workflow |
| Search And Filter | Search, filter results, select | complex, workflow |
| Settings Import Export | Complex settings dialog | complex, workflow |
| Nested Tab Forms | Forms in nested tabs | complex, workflow |
| Context Menu Operations | Right-click menu chains | complex, workflow |
| Dynamic Content Loading | Wait for lazy-loaded content | complex, workflow, dynamic |

---

## 9. Performance and Stress Tests (24_cascaded_performance.robot)

### 9.1 Performance Tests - 10 tests
**Coverage:** Section 11 of specification

| Test Case | Measurement | Target | Tags |
|-----------|-------------|--------|------|
| Simple Cascade Speed | 2-segment lookup time | <100ms | performance |
| Complex Cascade Speed | 5+ segment lookup time | <500ms | performance |
| Capture Overhead | Capture vs non-capture | <10% overhead | performance |
| Engine Switch Overhead | Multiple engine switches | <50ms per switch | performance |
| Deep Hierarchy Speed | 10-level cascade | <1s | performance |
| Large Table Cell Lookup | 1000+ rows | <2s | performance, table |
| Large Tree Node Lookup | 1000+ nodes | <2s | performance, tree |
| Repeated Cascade | Same cascade 100x | Linear scaling | performance, stress |
| Memory Usage | Large cascade operations | No leaks | performance |
| Parallel Cascades | 10 concurrent cascades | No conflicts | performance, stress |

### 9.2 Stress Tests - 8 tests
**Coverage:** Robustness under load

| Test Case | Load Type | Tags |
|-----------|-----------|------|
| Rapid Fire Cascades | 1000 cascades in loop | stress |
| Maximum Depth Cascade | 20+ levels | stress, edge-case |
| Maximum Breadth | Select from 1000+ siblings | stress |
| Memory Leak Test | 10000 cascade operations | stress, memory |
| Concurrent Access | Multiple test threads | stress, concurrency |
| Cache Stress | Cache invalidation scenarios | stress |
| Error Recovery Stress | Rapid error/success cycles | stress, error-handling |
| Long Running Session | 1-hour continuous operation | stress, stability |

### 9.3 Optimization Tests - 6 tests
**Coverage:** Section 11.1 of specification

| Test Case | Optimization | Tags |
|-----------|--------------|------|
| Early Termination | No match at segment 1 | performance, optimization |
| Index Caching | Reuse parsed locator | performance, optimization |
| Context Limiting | Search only descendants | performance, optimization |
| Result Limiting | First match only | performance, optimization |
| Best Practice Patterns | Efficient cascade patterns | performance, best-practice |
| Worst Practice Patterns | Inefficient patterns | performance, anti-pattern |

---

## 10. Error Handling and Edge Cases

### 10.1 Error Handling Tests - 15 tests
**Coverage:** Section 10 of specification

| Test Case | Error Type | Expected Behavior | Tags |
|-----------|------------|-------------------|------|
| Invalid Separator Inside Quotes | `JPanel[name='x' >> y']` | Parse error with hint | negative, error-handling |
| Invalid Separator Inside Brackets | `JPanel[name='x >> y']` | Parse error | negative, error-handling |
| Unknown Engine Prefix | `unknown=value >> JButton` | Unknown engine error | negative, error-handling |
| Empty Segment | `JPanel >> >> JButton` | Empty segment error | negative, error-handling |
| Malformed Attribute | `JPanel[name=unclosed` | Parse error | negative, error-handling |
| Malformed XPath | `xpath=///invalid///` | XPath parse error | negative, error-handling |
| No Match At Segment 2 | Error shows context | negative, error-handling |
| Timeout During Cascade | Timeout with context | negative, error-handling |
| Invalid Index Format | `index=abc` | Invalid index error | negative, error-handling |
| Multiple Capture Warning | `*A >> *B >> C` | Only first capture used | edge-case, warning |
| Unclosed Quotes | `text='unclosed` | Parse error | negative, error-handling |
| Bracket Mismatch | `JPanel[name='x']` | Parse error | negative, error-handling |
| Special Char Escape | Test escape sequences | Proper handling | edge-case |
| Unicode In Selectors | Unicode text/names | Proper handling | edge-case |
| Very Long Selector | 1000+ char selector | Handle gracefully | edge-case |

### 10.2 Edge Cases - 10 tests

| Test Case | Description | Tags |
|-----------|-------------|------|
| Empty Result Set | Cascade produces no results | edge-case |
| Single Element Result | Exactly one match | edge-case |
| Duplicate Elements | Multiple identical elements | edge-case |
| Dynamic Element Removal | Element removed during cascade | edge-case, dynamic |
| Dynamic Element Addition | Element added during cascade | edge-case, dynamic |
| Circular References | Avoid infinite loops | edge-case |
| Case Sensitivity | Various case combinations | edge-case |
| Whitespace Variations | All whitespace types | edge-case |
| Special Characters | $, @, %, etc. in values | edge-case |
| Null/Empty Attributes | Elements with null values | edge-case |

---

## Test Execution Strategy

### Execution Order
1. Basic cascaded selectors (smoke tests)
2. Individual engine tests
3. Capture prefix tests
4. Component-specific tests (tables, trees, tabs, menus)
5. Complex combinations
6. Performance and stress tests
7. Error handling tests

### Test Runner Commands
```bash
# Run all cascaded selector tests
uv run robot tests/robot/swing/16_cascaded_basic.robot
uv run robot tests/robot/swing/17_cascaded_engines.robot
uv run robot tests/robot/swing/18_cascaded_capture.robot
uv run robot tests/robot/swing/19_cascaded_tables.robot
uv run robot tests/robot/swing/20_cascaded_trees.robot
uv run robot tests/robot/swing/21_cascaded_tabs.robot
uv run robot tests/robot/swing/22_cascaded_menus.robot
uv run robot tests/robot/swing/23_cascaded_complex.robot
uv run robot tests/robot/swing/24_cascaded_performance.robot

# Run smoke tests only
uv run robot --include smoke tests/robot/swing/

# Run specific test tags
uv run robot --include capture tests/robot/swing/
uv run robot --include table tests/robot/swing/
uv run robot --include complex tests/robot/swing/

# Run all cascaded tests
uv run robot --include cascaded tests/robot/swing/
```

---

## Test Statistics Summary

| Test Suite | Test Count | Estimated LOC |
|------------|-----------|---------------|
| 16_cascaded_basic.robot | 15 | 400 |
| 17_cascaded_engines.robot | 83 | 2200 |
| 18_cascaded_capture.robot | 26 | 700 |
| 19_cascaded_tables.robot | 44 | 1200 |
| 20_cascaded_trees.robot | 36 | 1000 |
| 21_cascaded_tabs.robot | 24 | 650 |
| 22_cascaded_menus.robot | 20 | 550 |
| 23_cascaded_complex.robot | 37 | 1000 |
| 24_cascaded_performance.robot | 24 | 650 |
| **TOTAL** | **309** | **~8,350** |

---

## Coverage Matrix

| Specification Section | Test Coverage | Test Files |
|----------------------|---------------|------------|
| 2.1 Basic Cascaded Selector | 100% | 16_cascaded_basic.robot |
| 2.2 Grammar Definition | 100% | All files |
| 2.3 Whitespace Rules | 100% | 16_cascaded_basic.robot |
| 3.2 CSS Engine | 100% | 17_cascaded_engines.robot |
| 3.3 Class Engine | 100% | 17_cascaded_engines.robot |
| 3.4 Name Engine | 100% | 17_cascaded_engines.robot |
| 3.5 Text Engine | 100% | 17_cascaded_engines.robot |
| 3.6 Index Engine | 100% | 17_cascaded_engines.robot |
| 3.7 XPath Engine | 100% | 17_cascaded_engines.robot |
| 3.8 ID Engine | 100% | 17_cascaded_engines.robot |
| 4. Capture Prefix | 100% | 18_cascaded_capture.robot |
| 5.1 Table Selectors | 100% | 19_cascaded_tables.robot |
| 5.2 Tree Selectors | 100% | 20_cascaded_trees.robot |
| 5.3 TabbedPane Selectors | 100% | 21_cascaded_tabs.robot |
| 5.4 Menu Selectors | 100% | 22_cascaded_menus.robot |
| 10. Error Handling | 100% | All files (negative tests) |
| 11. Performance | 100% | 24_cascaded_performance.robot |

---

## Tags Reference

### Primary Tags
- `smoke` - Critical path tests (run first)
- `positive` - Tests that should succeed
- `negative` - Tests that should fail gracefully
- `edge-case` - Boundary and unusual conditions
- `regression` - Tests for known bugs

### Feature Tags
- `cascaded` - All cascaded selector tests
- `css-engine` - CSS engine specific
- `class-engine` - Class engine specific
- `name-engine` - Name engine specific
- `text-engine` - Text engine specific
- `index-engine` - Index engine specific
- `xpath-engine` - XPath engine specific
- `id-engine` - ID engine specific
- `capture` - Capture prefix tests
- `table` - Table component tests
- `tree` - Tree component tests
- `tabs` - Tab component tests
- `menus` - Menu component tests
- `complex` - Complex combinations
- `pseudo` - Pseudo-class tests
- `workflow` - Multi-step workflows
- `performance` - Performance tests
- `stress` - Stress tests
- `error-handling` - Error handling tests
- `assertion-operator` - AssertionEngine tests

---

## Dependencies and Prerequisites

### Required
- Swing Test Application running
- JavaGui.Swing library v1.0.0+
- Robot Framework 7.0+
- Python 3.10+
- Java 11+

### Test Data
- Pre-configured Swing Test App with:
  - Multiple tabs (Form Input, Selections, Data View, Settings)
  - Tables with test data
  - Trees with hierarchical data
  - Menus (File, Edit, View, Help)
  - Dialogs (Settings, About)
  - Various component types

---

## Success Criteria

### Test Execution
- ✅ All smoke tests pass (100%)
- ✅ All positive tests pass (>95%)
- ✅ All negative tests fail gracefully (100%)
- ✅ Performance tests meet targets (>90%)
- ✅ No memory leaks detected
- ✅ Error messages are clear and actionable

### Code Coverage
- ✅ 100% specification coverage
- ✅ All selector engines tested
- ✅ All component types tested
- ✅ All error paths tested

### Documentation
- ✅ All tests have clear documentation
- ✅ All failures include reproduction steps
- ✅ Performance benchmarks documented

---

## Next Steps

1. ✅ **Review this test plan** with team
2. Create `resources/cascaded_selectors.resource`
3. Implement test suite 16 (basic)
4. Implement test suite 17 (engines)
5. Implement test suite 18 (capture)
6. Implement test suites 19-22 (components)
7. Implement test suite 23 (complex)
8. Implement test suite 24 (performance)
9. Execute full test suite
10. Analyze results and fix issues
11. Update specification based on findings

---

**Document Status:** Draft - Ready for Review
**Author:** Claude Code
**Last Updated:** 2026-01-21
