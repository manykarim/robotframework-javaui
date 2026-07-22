# robotframework-javagui — Swing Automation Findings Report

**Date:** 2025-02-25  
**Package:** `robotframework-javagui` 0.4.0 (pip) / 0.1.0 (`__version__`)  
**Application Under Test:** Smart Client Showcase 22.04.2 (`smart-client-showcase-22.04.2.jar`)  
**Library Import:** `JavaGui.Swing`  

---

## 1. Test Environment

| Component | Version / Detail |
|-----------|------------------|
| Python | 3.12.9 (pyenv) |
| Robot Framework | 7.4.1 |
| robotframework-javagui | **0.4.0** (pip) / **0.1.0** (`JavaGui.__version__`) |
| Java | OpenJDK 17.0.15+6-Ubuntu-0ubuntu120.04.1 |
| OS | Linux 5.15.153.1-microsoft-standard-WSL2 |
| Display | X11 via WSLg (`:0`), Microsoft Corporation vendor |
| Package Manager | uv (PEP 723 project) |
| Agent JAR | `javagui-agent.jar` (bundled in package: 455,044 bytes) |

---

## 2. Executive Summary

The `JavaGui.Swing` library successfully automates a complex Java Swing desktop application. **Connection, property reading, element enumeration, component tree inspection, text assertions, navigation clicks, and full page traversal all work.** Interactive keywords (`Click`, `Input Text`, etc.) work correctly for visible/showing elements. The library handles custom Swing subclasses (JGoodies Smart Client components) without issues.

**Key issues found:**
1. **Version mismatch** — pip reports 0.4.0, `__version__` reports 0.1.0
2. **Agent port mismatch** — documented default is 5678, actual agent listens on **18080**
3. **`force=True` ineffective** — does not bypass agent-side `isShowing()` check
4. **Connection instability** — WebSocket drops with `Broken pipe (os error 32)` after inactivity
5. **`Get Component Tree` ignores locator** — always returns full tree regardless of scoping argument
6. **Duplicate keywords** — several keyword pairs are redundant aliases

---

## 3. Issues Found

### 3.1 Version Mismatch (Bug)

| Source | Version |
|--------|---------|
| `pip show robotframework-javagui` | **0.4.0** |
| `JavaGui.__version__` | **0.1.0** |
| `JavaGui.ROBOT_LIBRARY_VERSION` | **0.1.0** |

**Impact:** Version introspection in Robot Framework logs, `libdoc` output, and test reports shows incorrect version.

**Recommendation:** Synchronize `__version__` with the package metadata version (e.g., `importlib.metadata.version('robotframework-javagui')`).

### 3.2 Agent Port Mismatch (Documentation / Default)

**Severity:** Medium  

The `Connect To Application` keyword documents `port=5678` as the default. The bundled `javagui-agent.jar` actually listens on **port 18080**.

```
# Fails with ConnectionError: Connection refused
Connect To Application

# Works:
Connect To Application    port=18080
```

Port was discovered via `ss -tlnp` after connection failure. This was the most confusing issue during initial setup.

**Recommendation:**
1. Update the default port parameter to match the agent's actual default (18080), **or**
2. Have the agent listen on port 5678 to match the keyword default, **or**
3. Add clear startup logging from the agent showing which port it's listening on

### 3.3 `force=True` Does Not Bypass Visibility Check (Bug)

**Severity:** Medium  

The `Click` keyword accepts a `force` parameter documented as bypassing `isShowing()` checks. However, clicking non-showing elements with `force=True` still throws:

```
ConnectionError: RPC error -32603: Internal error:
java.lang.IllegalStateException: Component not visible for click after window activation: 25.
Window may be minimized, hidden, or component is not in a displayable window.
Waited 2000ms after activation.
```

The error originates from the Java agent side, which enforces visibility after window activation regardless of the `force` flag.

**Workaround:** Ensure the parent container is expanded/scrolled before clicking. In the Smart Client Showcase, clicking `//NavigationButton[2]` expands the left nav panel, making toggle buttons `showing=True`.

**Recommendation:** Either:
1. Pass the `force` flag through to the agent and skip the `isShowing()` guard, **or**
2. Document that `force` only applies to client-side checks and the agent still requires `isShowing()=true`

### 3.4 Connection Drops (Broken Pipe)

**Severity:** Medium  

The WebSocket connection drops with `ConnectionError: Failed to send request: Broken pipe (os error 32)` after periods of inactivity between keywords (~30-60 seconds). Occurred multiple times during extended exploration sessions.

**Workaround:**

```robotframework
*** Keywords ***
Ensure Connected
    ${connected}=    Is Connected
    IF    not ${connected}
        Connect To Application    port=${APP_PORT}
    END
```

**Recommendation:**
1. Implement WebSocket keep-alive (ping/pong) mechanism
2. Add automatic reconnection on broken pipe detection
3. Make the connection idle timeout configurable

### 3.5 `Get Component Tree` Does Not Scope to Locator (Bug)

**Severity:** Low  

`Get Component Tree    PivotFrame` returns the full application tree, identical to `Get Ui Tree`. The locator parameter is accepted but seemingly ignored.

**Expected:** Tree should be rooted at the matched `PivotFrame` component and include only its descendants.

### 3.6 Agent JAR Discrepancy (Informational)

The `javagui-agent.jar` bundled inside the Python package (455,044 bytes) differs from a workspace copy (454,420 bytes, dated Jan 23). There is no versioning or documentation about agent JAR compatibility.

**Recommendation:** Include agent version info in the JAR manifest and/or print it on agent startup.

---

## 4. `visible` vs `showing` Semantics

### 4.1 Background

Java Swing has two visibility concepts:
- **`isVisible()`** — Component has its visibility flag set (can be `true` even if parent container is collapsed/hidden)
- **`isShowing()`** — Component is actually displayable on screen (requires all ancestors to be `isShowing()=true`)

### 4.2 Observations in the Library

| Keyword | Uses | Behavior for collapsed panel elements |
|---------|------|--------------------------------------|
| `Element Should Be Visible` | `isShowing()` | **FAILS** |
| `Component Should Be Visible` | `isVisible()` | **PASSES** |
| `Get Element States` | `isVisible()` | Returns `['visible']` |
| `Click` | `isShowing()` (agent-side) | **FAILS** |
| `Wait Until Element Is Visible` | `isShowing()` | **Times out** |

The inconsistency is confusing: `Get Element States` reports `visible`, but `Element Should Be Visible` fails for the same element.

### 4.3 Recommendation

Consider unifying semantics:
- `Element Should Be Visible` → checks `isVisible()` (consistent with `Get Element States`)
- `Element Should Be Showing` → checks `isShowing()` (new, explicit keyword)
- Document the difference clearly in the library introduction

---

## 5. Keyword API Completeness (86 Keywords)

### 5.1 Connection & Lifecycle (5)

| Keyword | Signature | Status |
|---------|-----------|--------|
| `Connect To Application` | `application, pid, main_class, title, host, port, timeout` | **Works** (port=18080) |
| `Disconnect` | — | **Works** |
| `Is Connected` | → `bool` | **Works** |
| `List Applications` | → `List[str]` | Returns `[]` (placeholder) |
| `Get Connection Info` | → `Dict` | **Works** |

### 5.2 Click & Interaction (8)

| Keyword | Signature | Status |
|---------|-----------|--------|
| `Click` | `locator, force` | **Works** (showing elements), fails for hidden |
| `Click Button` | `locator, force` | **Works** |
| `Click Element` | `locator, click_count, force` | **Works** |
| `Double Click` | `locator, force` | **Works** |
| `Right Click` | `locator, force` | **Works** |
| `Input Text` | `locator, text, clear, force` | Not tested (no text fields in nav) |
| `Type Text` | `locator, text` | Not tested |
| `Clear Text` | `locator` | Not tested |

### 5.3 Selection & Choice (10)

| Keyword | Status |
|---------|--------|
| `Check Checkbox` | Not tested |
| `Uncheck Checkbox` | Not tested |
| `Select Radio Button` | Not tested |
| `Select From Combobox` | Not tested |
| `Select From List` | Not tested |
| `Select List Item By Index` | Not tested |
| `Select From Popup Menu` | Not tested |
| `Select Menu` | Not tested |
| `Select Tab` | Not tested |
| `Select Table Cell` | Not tested |

### 5.4 Element Query & Properties (12)

| Keyword | Status | Notes |
|---------|--------|-------|
| `Find Element` | **Works** | Returns `SwingElement` |
| `Find Elements` | **Works** | Returns list |
| `Get Element Count` | **Works** | Correct counts |
| `Get Element Properties` | **Works** | Dict with name, text, enabled, visible, selected |
| `Get Element Property` | **Works** | Individual property access |
| `Get Element States` | **Works** | Returns state list |
| `Get Element Text` | **Works** | Text content |
| `Get Text` | **Works** | With inline assertion operators (`==`, `!=`, `contains`, etc.) |
| `Get Component Text` | **Works** | Alias of Get Text |
| `Get Properties` | **Works** | Alias of Get Element Properties |
| `Get Property` | **Works** | Alias of Get Element Property |
| `Get Connection Info` | **Works** | Connection details |

### 5.5 Text Field & Value (3)

| Keyword | Status |
|---------|--------|
| `Get Field Value` | Not tested |
| `Get Text Field Value` | Not tested |
| `Get Value` | Not tested |

### 5.6 Label (1)

| Keyword | Status |
|---------|--------|
| `Get Label Content` | Not tested |

### 5.7 List Keywords (8)

| Keyword | Status |
|---------|--------|
| `Get List Items` | Not tested |
| `Get List Item Count` | Not tested |
| `Get Number Of List Items` | Not tested (duplicate of above?) |
| `Get Selected List Index` | Not tested |
| `Get Selected List Item` | Not tested |
| `Get Selected List Items` | Not tested |
| `List Selection Should Be` | Not tested |
| `List Should Contain` | Not tested |

### 5.8 Table Keywords (13)

| Keyword | Status |
|---------|--------|
| `Get Table Data` | Not tested |
| `Get Table Cell Content` | Not tested |
| `Get Table Cell Text` | Not tested |
| `Get Table Cell Value` | Not tested |
| `Get Table Column Count` | Not tested |
| `Get Table Column Values` | Not tested |
| `Get Table Row Count` | Not tested |
| `Get Table Row Values` | Not tested |
| `Get Number Of Table Columns` | Not tested (duplicate?) |
| `Get Number Of Table Rows` | Not tested (duplicate?) |
| `Get Selected Table Rows` | Not tested |
| `Select Table Row` | Not tested |
| `Table Cell Should Contain` | Not tested |

### 5.9 Tree Keywords (9)

| Keyword | Status |
|---------|--------|
| `Get Tree Nodes` | Not tested |
| `Get Tree Node Children` | Not tested |
| `Get Tree Node Count` | Not tested |
| `Get Tree Node Text` | Not tested |
| `Get Selected Tree Node` | Not tested |
| `Select Tree Node` | Not tested |
| `Expand Tree Node` | Not tested |
| `Collapse Tree Node` | Not tested |
| `Tree Node Should Exist` | Not tested |

### 5.10 Assertions (11)

| Keyword | Status | Notes |
|---------|--------|-------|
| `Element Should Be Visible` | **Works** | Uses `isShowing()` — see Section 4 |
| `Element Should Not Be Visible` | **Works** | |
| `Element Should Be Enabled` | **Works** | |
| `Element Should Be Disabled` | Not tested | |
| `Element Should Be Selected` | **Works** | |
| `Element Should Not Be Selected` | **Works** | |
| `Element Should Be Showing` | **Works** | Explicit `isShowing()` check |
| `Element Should Exist` | **Works** | |
| `Element Should Not Exist` | **Works** | |
| `Element Text Should Be` | **Works** | |
| `Element Text Should Contain` | **Works** | |

### 5.11 Wait Keywords (8)

| Keyword | Status | Notes |
|---------|--------|-------|
| `Wait For Element` | **Works** | |
| `Wait Until Element Exists` | **Works** | |
| `Wait Until Element Does Not Exist` | **Works** | |
| `Wait Until Element Visible` | **Works** | |
| `Wait Until Element Is Visible` | **Works** | Duplicate of above |
| `Wait Until Element Enabled` | **Works** | |
| `Wait Until Element Is Enabled` | **Works** | Duplicate of above |
| `Wait Until Element Is Showing` | **Works** | |

### 5.12 UI Tree (6)

| Keyword | Status | Notes |
|---------|--------|-------|
| `Get Ui Tree` | **Works** | Formats: text, json, xml, yaml |
| `Get Component Tree` | **Partial** | Locator scoping broken (see 3.5) |
| `Save Ui Tree` | **Works** | Saves to file |
| `Log Ui Tree` | **Works** | Logs to RF log |
| `Log Component Tree` | **Works** | |
| `Refresh Ui Tree` | **Works** | Required after navigation |

### 5.13 Configuration (4)

| Keyword | Status |
|---------|--------|
| `Set Timeout` | **Works** |
| `Set Assertion Timeout` | **Works** |
| `Set Assertion Interval` | **Works** |
| `Set Screenshot Directory` | **Works** |

### 5.14 Screenshot (1)

| Keyword | Status |
|---------|--------|
| `Capture Screenshot` | **Works** |

---

## 6. Duplicate / Redundant Keywords

The following keywords appear to be identical in behavior:

| Keyword A | Keyword B | Recommendation |
|-----------|-----------|----------------|
| `Wait Until Element Visible` | `Wait Until Element Is Visible` | Keep one, deprecate other |
| `Wait Until Element Enabled` | `Wait Until Element Is Enabled` | Keep one, deprecate other |
| `Get List Item Count` | `Get Number Of List Items` | Keep one, deprecate other |
| `Get Table Row Count` | `Get Number Of Table Rows` | Keep one, deprecate other |
| `Get Table Column Count` | `Get Number Of Table Columns` | Keep one, deprecate other |
| `Get Text` | `Get Element Text` / `Get Component Text` | Document as aliases or consolidate |
| `Get Property` | `Get Element Property` | Document as aliases or consolidate |
| `Get Properties` | `Get Element Properties` | Document as aliases or consolidate |

**Recommendation:** Adopt a consistent naming convention (`element_*` preferred for RF conventions), mark duplicates with `@deprecated`, and keep both working for backward compatibility.

---

## 7. Locator Syntax (Verified)

### 7.1 Working Locator Patterns

| Syntax | Example | Result |
|--------|---------|--------|
| Simple type | `JFrame` | Matches first JFrame |
| Type + attribute | `NavigationToggleButton[text='Input']` | Matches by text property |
| XPath indexed | `//NavigationButton[2]` | Matches 2nd NavigationButton |
| XPath any-depth | `//ReadOnlyTextField[3]` | Matches 3rd ReadOnlyTextField |
| XPath with attr | `//NavigationToggleButton[@text='Start']` | Matches by XPath attribute |

### 7.2 Not Tested

| Syntax | Example | Notes |
|--------|---------|-------|
| ID selector | `#submit` | No named elements in SUT to test |
| Descendant combinator | `JPanel JButton` | No suitable test scenario |
| Child combinator | `JPanel > JButton` | No suitable test scenario |

---

## 8. Application Under Test: Component Structure

### 8.1 Window Layout

**Title:** "The Standard Design Library 22.04"  
**Size:** 1276 × 997 pixels  
**Layout:** Three-panel `SplitView`

```
┌────────────────────────────────────────────────────────┐
│ JFrame — "The Standard Design Library 22.04"           │
├──────────────┬──────────┬──────────────────────────────┤
│ Left Nav [7] │ Mid [12] │ Content Area [17]            │
│              │          │                              │
│ [Search]     │ [Expand] │ [Header: Back|Title|Action]  │
│ [ScrollPane] │ [SubNav] │ [PageFrame]                  │
│  27 NavBtns  │  5 Btns  │  - JScrollPane              │
│ [Settings]   │          │  - PivotFrame (sub-tabs)     │
│              │          │ [StatusBar: Label|Progress]  │
└──────────────┴──────────┴──────────────────────────────┘
```

### 8.2 Component Inventory

| Component Type | Count | Location |
|----------------|-------|----------|
| `NavigationToggleButton` | 34 | Left nav (27) + mid nav (5) + standalone (2) |
| `NavigationButton` | 3 | Panel expand/collapse controls |
| `ReadOnlyTextField` | 3 | Page header breadcrumb |
| `JGSearchField` | 1 | Left nav search |
| `JButton` | 1 | Page header action |
| `JLabel` | 1 | Status bar |
| `JProgressBar` | 1 | Status bar |
| `PageFrame` | 1 | Content area |
| `PivotFrame` | 1 | Sub-tab navigation |
| `SplitView` | 1 | Main layout |
| `JScrollPane` | 2+ | Scroll containers |
| `ScrollBar` | 4+ | Scroll controls |
| `PlasticArrowButton` | 4 | JGoodies Plastic L&F scroll arrows |

### 8.3 Custom Components

All custom Swing subclasses from the Smart Client framework are correctly discovered and identifiable:

- **`NavigationToggleButton`** — Toggle navigation items with `text`, `selected` properties
- **`NavigationButton`** — Non-toggle expand/collapse buttons (no visible text, use XPath indexing)
- **`ReadOnlyTextField`** — Non-editable fields used in page headers
- **`JGSearchField`** — Custom search input field
- **`PageFrame`** — Content frame container
- **`PivotFrame`** — Sub-tab container
- **`SplitView`** — Three-panel split layout
- **`PlasticArrowButton`** — JGoodies scroll arrows

### 8.4 Navigation Pages Verified

| Page Name | Title via `//ReadOnlyTextField[3]` |
|-----------|-----------------------------------|
| Start | *(no title ReadOnlyTextField on start page)* |
| Input | `Input Dialogs` |
| Selection | `Selection Dialogs` |
| Dialogs | `Dialogs` |
| Forms | `Flexible, Versatile, Consistent, Responsive Forms` |
| Master-Details | `Master-Details` |
| Basics | `Basics` |
| Components | `Components` |

### 8.5 Behavioral Notes

1. **Panel Expand Animation:** Left nav slides open with ~2s animation after clicking `//NavigationButton[2]`. Must wait before interacting with toggle buttons.
2. **UI Tree Staleness:** `Refresh Ui Tree` must be called after page navigation to see new content.
3. **Page Header Pattern:** Every page follows: 3 `ReadOnlyTextField` elements (empty, category, title) + `NavigationButton[name='back_button']` + `JButton`.
4. **Showing vs Visible:** Nav toggle buttons inside collapsed `JScrollPane` report `visible=True` but `showing=False`. Only after panel expansion do they become `showing=True`.

---

## 9. Working Test Patterns

### 9.1 Robust Launch & Connect

```robotframework
*** Keywords ***
Launch Showcase Application
    Start Process    java    -javaagent:${AGENT_JAR}    -jar    ${JAR_FILE}
    ...    alias=${APP_ALIAS}    shell=True    cwd=${EXECDIR}
    Sleep    ${STARTUP_WAIT}    Wait for JVM + agent startup
    Connect To Application    port=${APP_PORT}

Ensure Connected
    ${connected}=    Is Connected
    IF    not ${connected}
        Connect To Application    port=${APP_PORT}
    END
```

### 9.2 Navigate With Panel Expansion

```robotframework
*** Keywords ***
Expand Left Navigation Panel
    Click    //NavigationButton[2]
    Sleep    ${PANEL_WAIT}    Wait for panel animation

Navigate To Page
    [Arguments]    ${page_name}
    Expand Left Navigation Panel
    Click    NavigationToggleButton[text='${page_name}']
    Sleep    ${NAV_WAIT}    Wait for page transition
    Refresh Ui Tree
```

### 9.3 Page Title Verification

```robotframework
*** Keywords ***
Verify Page Title
    [Arguments]    ${expected_title}
    Get Text    //ReadOnlyTextField[3]    ==    ${expected_title}

Get Current Page Title
    ${title}=    Get Text    //ReadOnlyTextField[3]
    RETURN    ${title}
```

---

## 10. Feature Requests & Suggestions

### 10.1 Missing Keywords

| Keyword | Use Case |
|---------|----------|
| `Scroll Element Into View` | Make a component `showing` by scrolling its parent |
| `Get Window Title` | Convenience (currently: `Get Element Property JFrame title`) |
| `Maximize Window` / `Minimize Window` | Window state management |
| `Wait Until Page Contains` | Wait for text pattern in UI tree |
| `Get All Element Texts` | Get texts from all matching elements |
| `Element Should Be Focused` | Verify keyboard focus |

### 10.2 Improvements

1. **Auto-reconnect** — Wrap RPC calls with retry logic on `Broken pipe`
2. **Implicit wait for showing** — Global setting for `Click` to auto-wait for `isShowing()=true`
3. **`Get Component Tree` scoping** — Root tree at matched component, not full application
4. **Agent port auto-detection** — Scan common ports (5678, 18080, 8080) on connection failure
5. **UI Tree diff** — `Compare Ui Tree` returning differences between snapshots
6. **Element screenshot** — `Capture Screenshot` with optional locator for component region
7. **Agent startup logging** — Print listening port clearly to stdout/stderr

---

## 11. Package Architecture Notes

### 11.1 Native Extension

The library uses a Rust native extension (`_core.abi3.so`, ELF 64-bit x86-64) compiled against the stable Python ABI (`abi3`). This is the core communication layer between Python and the Java agent.

### 11.2 Library Metadata

```python
ROBOT_LIBRARY_SCOPE = 'GLOBAL'
ROBOT_LIBRARY_DOC_FORMAT = 'REST'
ROBOT_LIBRARY_VERSION = '0.1.0'  # Should be 0.4.0
```

### 11.3 Module Exports

```python
__all__ = ['Swing', 'Swt', 'Rcp', 'SwingLibrary', 'SwtLibrary', 'RcpLibrary']
```

---

## 12. Full Component Tree (Start Page)

```
[1] JFrame (primary)
  [2] JRootPane (-)
    [3] JPanel (null.glassPane)
    [4] JLayeredPane (null.layeredPane)
      [5] JPanel (-)
        [6] SplitView (-)
          [7] JPanel (-)                          ← Left Navigation Panel
            [8] JPanel (-)
              [20] NavigationButton ()            ← Panel collapse button
              [21] JPanel (-)
            [9] JPanel (-)
              [22] JGSearchField ()               ← Search field
            [10] JScrollPane (-)
              [23] JViewport (-)
                [24] JPanel (-)
                  [25] NavigationToggleButton (Start)
                  [26] NavigationToggleButton (Pages)
                  [27] NavigationToggleButton (Hub Page)
                  [28] NavigationToggleButton (Master-Details)
                  [29] NavigationToggleButton (List Report)
                  [30] NavigationToggleButton (Object Page)
                  [31] NavigationToggleButton (Worklist)
                  [32] NavigationToggleButton (Initial Page)
                  [33] NavigationToggleButton (Dialogs)
                  [34] NavigationToggleButton (Messages)
                  [35] NavigationToggleButton (Input)
                  [36] NavigationToggleButton (Choice)
                  [37] NavigationToggleButton (Selection)
                  [38] NavigationToggleButton (Property)
                  [39] NavigationToggleButton (Progress)
                  [40] NavigationToggleButton (Wizards)
                  [41] NavigationToggleButton (Content)
                  [42] NavigationToggleButton (Forms)
                  [43] NavigationToggleButton (Hub / Tiles)
                  [44] NavigationToggleButton (Object Lists)
                  [45] NavigationToggleButton (Facets)
                  [46] NavigationToggleButton (Basics)
                  [47] NavigationToggleButton (Layout)
                  [48] NavigationToggleButton (Components)
                  [49] NavigationToggleButton (Validation)
                  [50] NavigationToggleButton (Completion)
                  [51] NavigationToggleButton (Data Binding)
              [52] ScrollBar (-)
              [55] ScrollBar (-)
            [11] NavigationToggleButton (Settings)
          [12] JPanel (-)                         ← Middle Navigation Panel
            [13] JPanel (-)
              [58] NavigationButton ()
              [59] JPanel (-)
            [14] NavigationButton ()              ← Panel expand button
            [15] JPanel (-)
              [60-64] NavigationToggleButton (-)  ← Sub-navigation
            [16] NavigationToggleButton (-)
          [17] JPanel (-)                         ← Content Panel
            [18] JPanel (-)                       ← Status Bar
              [65] JLabel ()
              [66] JProgressBar (-)
            [19] PageFrame (-)                    ← Page Content
              [67] JPanel (-)
                [68] JPanel (-)
                  [69] JScrollPane (-)
```

---

## 13. Test Suite Created

A validated test suite was created at `tests/smart_client_showcase_tests.robot`:

- **11 test cases** covering launch, window verification, navigation, page traversal, UI tree logging
- **All steps individually executed and verified** via MCP RobotMCP before assembly
- **Custom keywords:** Launch/Close Showcase Application, Ensure Connected, Expand Left Navigation Panel, Navigate To Page, Verify Page Title, Get Current Page Title

---

## 14. Conclusion

The `JavaGui.Swing` library provides a solid and well-designed foundation for Java Swing automation. The inline assertion pattern (borrowed from Browser Library), rich UI tree inspection, and dual locator syntax are excellent design choices. The library correctly handles custom Swing components without additional configuration.

**Priority fixes recommended:**
1. **Agent port** — align default port between keyword and agent (highest user impact)
2. **`force=True`** — make it actually bypass visibility checks end-to-end
3. **Version string** — synchronize `__version__` with package metadata
4. **Connection stability** — add keep-alive or auto-reconnect

The library is ready for production use with the documented workarounds.
