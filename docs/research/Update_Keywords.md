Here is a **concise but complete Summary, Background, and Purpose section** that you can place at the **top of the `.md` document** (or use as an executive overview in a PR, RFC, or community discussion).

---

## Summary

This research proposes a **comprehensive simplification and unification of the keyword API** in the `robotframework-javaui` library, aligning it with the modern design principles established by the **Robot Framework Browser Library**.

The proposal focuses on:

* Reducing keyword count through generalization
* Introducing assertion-engine–driven “Get” keywords
* Supporting locator chaining for expressive element targeting
* Preserving full interaction capabilities across all Java UI technologies
* Clearly identifying keywords that **must remain explicit** for technical or diagnostic reasons

The result is a **smaller, more powerful, and more consistent keyword set** that improves usability, maintainability, and long-term extensibility without sacrificing control or debuggability.

---

## Background

`robotframework-javaui` provides UI automation for Java-based desktop applications (e.g. Swing, JavaFX, SWT).
Over time, its keyword set has grown organically, leading to:

* A large number of narrowly-scoped keywords
* Control- and technology-specific naming
* Separate keywords for interaction, waiting, and assertions
* Redundant or overlapping functionality

In parallel, the **Robot Framework Browser Library** has established a modern and widely adopted keyword design based on:

* Assertion-engine–powered keywords
* Locator-driven interaction
* Minimal but expressive keyword sets
* Reduced need for explicit waiting and verification keywords

This research examines how the **philosophy and patterns** of the Browser Library can be applied to JavaUI automation—*not by copying its surface API*, but by adopting its underlying design principles.

---

## Purpose of the Research

The purpose of this research is to define a **clear, technically sound direction** for evolving the JavaUI keyword API.

Specifically, it aims to:

1. **Improve Test Author Experience**

   * Fewer keywords to learn
   * More readable and expressive test cases
   * Consistent mental model across web and desktop automation

2. **Reduce Maintenance and API Surface Area**

   * Eliminate redundant keywords
   * Avoid technology- or control-specific APIs
   * Make future extensions easier and safer

3. **Enable Modern Assertion Patterns**

   * Embed assertions directly into “Get” keywords
   * Reduce explicit `Wait Until` and `Should Be` keywords
   * Support retryable, assertion-driven synchronization

4. **Preserve Advanced and Diagnostic Capabilities**

   * Explicitly identify keywords that cannot or should not be simplified
   * Protect debugging, introspection, and tooling use cases

5. **Provide a Foundation for Future Evolution**

   * Support new Java UI technologies without new keywords
   * Align JavaUI with the broader Robot Framework ecosystem

This research is intended to serve as:

* A **design reference**
* A **discussion baseline** for maintainers and contributors
* A **migration guide foundation** for future API evolution

It intentionally avoids implementation details and focuses solely on **API design, usability, and long-term sustainability**.


````markdown
# JavaUI Keyword Simplification & Unification Proposal

## Goal

This document proposes a **simplified, unified, and future-proof keyword API** for
`robotframework-javaui`, aligned with the design principles of the
**Robot Framework Browser Library**.

The primary objectives are:

- Reduce the overall number of keywords
- Unify keyword naming across all UI technologies
- Adopt assertion-engine based keywords
- Support locator chaining
- Enable “Get” keywords to perform assertions
- Preserve **full control and flexibility** for all Java UI technologies
- Explicitly identify keywords that **must not** be simplified

This is a **conceptual and API-level proposal only**.  
No implementation details are included.

---

## Design Principles

### 1. Fewer, More Powerful Keywords

Instead of many narrowly-scoped keywords, provide a **small set of generic, composable keywords** that:

- Work for all controls (buttons, fields, tables, trees, etc.)
- Are driven by locators and arguments, not keyword names
- Scale naturally to new UI technologies

---

### 2. Assertion Engine First

All “Get” keywords should support **inline assertions** using Robot Framework’s assertion engine.

This removes the need for separate:
- `Element Should Be ...`
- `Element Text Should ...`
- `Wait Until ...`

Assertions become **part of the keyword**, not separate steps.

---

### 3. Locator-Driven Interaction

The locator determines:
- Which element is targeted
- Which UI technology is used
- Which control type is interacted with

Keywords must **never encode UI technology or control type** in their name.

---

### 4. Locator Chaining

Locators should support **hierarchical and conditional chaining**, similar to Browser Library:

- Parent → child relationships
- State filters (visible, enabled, selected)
- Attribute-based filtering

This reduces the need for navigation-specific keywords.

---

## Current Problems in JavaUI Keywords

### Keyword Explosion

Examples:
- `Click`
- `Click Button`
- `Element Should Be Visible`
- `Wait Until Element Visible`
- `Element Should Exist`
- `Wait For Element`

All of these can be replaced with **fewer generic keywords**.

---

### Redundant Assertions

Assertions are currently:
- Split into many dedicated keywords
- Separated from data retrieval
- Hard to compose or extend

---

### Control-Specific Keywords

Examples:
- `Check Checkbox`
- `Uncheck Checkbox`
- `Select Radio Button`
- `Select From List`
- `Select List Item By Index`

These encode *what* the element is instead of *what you want to do*.

---

## Proposed Keyword Categories

### 1. Session & Application Control

Minimal and explicit.

```robot
Connect To Application    path=/app/myapp.jar
Disconnect
````

---

### 2. Generic Actions

| Keyword        | Purpose                                     |
| -------------- | ------------------------------------------- |
| `Click`        | Click any clickable element                 |
| `Type Text`    | Input text into editable elements           |
| `Clear Text`   | Clear editable elements                     |
| `Select Item`  | Select item in list, combo box, table, tree |
| `Set Checkbox` | Set checkbox state (true/false)             |

#### Examples

```robot
Click    JButton[name="Save"]

Type Text    JTextField[name="username"]    admin

Set Checkbox    JCheckBox[name="rememberMe"]    true
```

---

### 3. Generic Get Keywords (Assertion-Enabled)

These keywords **return values** and **optionally assert them**.

| Keyword              | Returns                     |
| -------------------- | --------------------------- |
| `Get Text`           | Element text                |
| `Get Value`          | Control value               |
| `Get Element States` | List of states              |
| `Get Element Count`  | Number of matching elements |
| `Get Property`       | Specific property           |
| `Get Properties`     | All properties              |

---

## Assertion Engine Usage

All `Get` keywords accept optional assertion arguments:

```
<operator>    <expected>
```

### Example: Text Assertion

```robot
Get Text    JLabel[name="status"]    should be    Ready
```

Replaces:

```robot
Element Text Should Be    JLabel[name="status"]    Ready
```

---

### Example: Existence Check

```robot
Get Element Count    JButton[name="Delete"]    >    0
```

Replaces:

```robot
Element Should Exist    JButton[name="Delete"]
```

---

### Example: Visibility Check

```robot
Get Element States    JTextField[name="email"]    contains    visible
```

Replaces:

```robot
Element Should Be Visible    JTextField[name="email"]
```

---

## Locator Chaining

### Basic Hierarchy

```robot
Click    JPanel[name="toolbar"] >> JButton[text="Save"]
```

---

### State Filtering

```robot
Click    JButton[text="Submit"] >> enabled=true
```

---

### Combined Filters

```robot
Get Text
...    JFrame[name="Main"] >> JPanel[name="content"] >> JLabel >> visible=true
...    should contain
...    Welcome
```

---

## Waiting via Assertions

Explicit wait keywords can be replaced with **retryable assertions**.

```robot
Get Element States
...    JButton[name="Login"]
...    contains
...    visible
```

This replaces:

```robot
Wait Until Element Visible    JButton[name="Login"]
```

---

## Tables

### Reading Data

```robot
Get Value
...    JTable[name="users"] >> row=1 >> column=Name
...    should be
...    John Doe
```

---

### Selecting Rows

```robot
Select Item    JTable[name="users"] >> row[text="John Doe"]
```

---

## Trees

```robot
Select Item    JTree[name="navigation"] >> path="Settings/Users"
```

---

## Lists & Combo Boxes

Unified keyword:

```robot
Select Item    JComboBox[name="country"]    Germany
Select Item    JList[name="roles"]    Admin
```

Index-based selection:

```robot
Select Item    JList[name="roles"]    index=2
```

---

## Screenshots

```robot
Capture Screenshot
Capture Screenshot    name=after-login
```

---

## UI Tree Inspection (Unchanged)

```robot
Get UI Tree
Log UI Tree
Refresh UI Tree
```

---

# Keywords That Must NOT Be Simplified

Some JavaUI keywords **cannot or should not** be migrated to Browser-style syntax
without losing clarity, power, or debuggability.

These keywords serve **structural, diagnostic, or session-level purposes**.

---

## 1. Application / Session Lifecycle

| Keyword                  | Reason                      |
| ------------------------ | --------------------------- |
| `Connect To Application` | JVM / process-level control |
| `Disconnect`             | Session teardown            |
| `Is Connected`           | Session state               |

These do **not operate on locators** and must remain explicit.

---

## 2. UI Tree Introspection & Debugging

| Keyword           | Reason            |
| ----------------- | ----------------- |
| `Get UI Tree`     | Structural model  |
| `Log UI Tree`     | Debug output      |
| `Refresh UI Tree` | Forces UI re-scan |

These are **debugging tools**, not interaction or assertion keywords.

---

## 3. Global Configuration

| Keyword                    | Reason               |
| -------------------------- | -------------------- |
| `Set Screenshot Directory` | Global configuration |

Configuration keywords cannot be meaningfully expressed via locators or assertions.

---

## 4. Raw Metadata Access

| Keyword                  | Reason                                |
| ------------------------ | ------------------------------------- |
| `Get Element Properties` | Tool- and framework-specific metadata |

This keyword is primarily for:

* Debugging
* Exploration
* Tooling support

Assertions must remain user-driven.

---

## 5. Advanced Querying (Partial Migration Only)

| Keyword         | Status                 |
| --------------- | ---------------------- |
| `Find Element`  | Advanced / tooling use |
| `Find Elements` | Advanced / tooling use |

These should remain available but **not encouraged for test logic**.

---

## 6. Bulk Table Data Extraction

| Keyword                  | Reason          |
| ------------------------ | --------------- |
| `Get Table Data`         | Full data model |
| `Get Table Row Count`    | Structural      |
| `Get Table Column Count` | Structural      |

Assertions on these results are often ambiguous and should remain explicit.

---

## Summary: What Should NOT Be Migrated

| Category          | Keywords                                                          |
| ----------------- | ----------------------------------------------------------------- |
| Session lifecycle | `Connect To Application`, `Disconnect`, `Is Connected`            |
| UI diagnostics    | `Get UI Tree`, `Log UI Tree`, `Refresh UI Tree`                   |
| Global config     | `Set Screenshot Directory`                                        |
| Debug metadata    | `Get Element Properties`                                          |
| Advanced querying | `Find Element`, `Find Elements`                                   |
| Bulk table reads  | `Get Table Data`, `Get Table Row Count`, `Get Table Column Count` |

---

## Final Reduction Summary

| Aspect              | Before | After       |
| ------------------- | ------ | ----------- |
| Total keywords      | ~60–70 | ~20–25 core |
| Assertion keywords  | Many   | Embedded    |
| Control-specific    | Yes    | No          |
| Technology-specific | Yes    | No          |
| Debug capability    | Mixed  | Preserved   |

---

## Final Takeaway

* **~80–85%** of JavaUI keywords can be unified and simplified
* **~15–20%** must remain explicit
* The goal is **Browser Library philosophy**, not blind imitation

This approach:

* Simplifies test authoring
* Preserves advanced use cases
* Keeps JavaUI extensible and debuggable
* Aligns desktop automation with modern Robot Framework design

---

```

---

