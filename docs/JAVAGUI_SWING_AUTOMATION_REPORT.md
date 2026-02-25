# robotframework-javagui — Swing Automation Findings Report

**Date:** 2026-02-25  
**Version Under Test:** robotframework-javagui 0.3.1 (PyPI) / 0.1.0 (`__version__`)  
**Application Under Test:** JGoodies Smart Client Showcase 22.04.2 (`smart-client-showcase-22.04.2.jar`)  
**Library Import:** `JavaGui.Swing`  

## 1. Test Environment

| Component           | Version / Detail                                      |
|----------------------|-------------------------------------------------------|
| Python               | 3.12.9                                                |
| Robot Framework      | 7.4.1                                                 |
| robotframework-javagui | 0.3.1 (pip) / 0.1.0 (`JavaGui.__version__`)         |
| Java                 | OpenJDK 17.0.15                                       |
| OS                   | Linux 5.15.153.1-microsoft-standard-WSL2              |
| Display              | X11 via WSLg (`:0`), Xvfb available                   |
| Agent JAR            | `javagui-agent.jar` (bundled + in site-packages)      |

### Version Discrepancy (Minor)
`uv pip list` reports `robotframework-javagui==0.3.1`, but `JavaGui.__version__` returns `0.1.0`. The `__version__` attribute in `__init__.py` has not been updated to match the package distribution version.

---

## 2. Executive Summary

The `JavaGui.Swing` library successfully connects to and inspects Java Swing applications via the javaagent mechanism. **Property-reading, element enumeration, component tree inspection, element existence checks, and text assertions all work reliably.** However, a critical `isShowing()` vs `isVisible()` inconsistency within the JGoodies `SplitView` component causes **all interactive keywords (Click, Double Click, Right Click, Input Text) to fail** for components nested inside SplitView panels. This also creates contradictory behavior between `Element Should Be Visible` (fails) and `Component Should Be Visible` (passes) for the same component.

---

## 3. Keyword-by-Keyword Testing Results

### 3.1 Connection & Lifecycle Keywords

| Keyword | Status | Notes |
|---------|--------|-------|
| `Connect To Application` | **PASS** | `port=5678` works. Connection is immediate (~0.03s) |
| `Disconnect` | **PASS** | Clean disconnect, `Is Connected` returns `False` after |
| `Is Connected` | **PASS** | Returns correct boolean state |
| `Get Connection Info` | **PASS** | Returns dict: `{connected: True, application_name: 'default', host: 'localhost', port: 5678, pid: None}` |
| `List Applications` | **PASS** | Returns empty list `[]` — documented as placeholder |
| Reconnect cycle | **PASS** | Disconnect → Connect To Application works cleanly |

**Note:** `pid` is always `None` in the connection info. Consider populating this from the javaagent's JMX or from the `Start Process` return value.

### 3.2 Property & Inspection Keywords

| Keyword | Status | Notes |
|---------|--------|-------|
| `Get Property` | **PASS** | Works for `title`, `text`, `enabled`, `visible`, `showing`, `selected` |
| `Get Properties` | **PASS** | Returns dict: `{name, text, enabled, visible, selected}` |
| `Get Element Properties` | **PASS** | Same as `Get Properties` — are these intentional aliases? |
| `Get Element Property` | **PASS** | Works same as `Get Property` |
| `Get Element States` | **PASS** | Returns list: `['visible', 'enabled', 'unfocused', 'selected', 'checked', 'readonly', 'attached']` |
| `Get Element Text` | **PASS** | Returns component text |
| `Get Text` | **PASS** | Alias for `Get Element Text` |
| `Get Component Text` | **PASS** | Another alias for text retrieval |
| `Get Component Tree` | **PASS** | `format=text`, `max_depth=N` parameters work |
| `Log Ui Tree` | **PASS** | Logs tree to RF log |
| `Save Ui Tree` | **PASS** | Saves to file path |
| `Refresh Ui Tree` | **PASS** | Forces tree refresh |

### 3.3 Element Lookup Keywords

| Keyword | Status | Notes |
|---------|--------|-------|
| `Find Element` | **PASS** | Returns `SwingElement` object with rich attribute set |
| `Find Elements` | **PASS** | Returns list of all matching `SwingElement` objects |
| `Get Element Count` | **PASS** | Correct counts: 34 NavigationToggleButtons, 3 NavigationButtons, 15 JPanels |

**Locator format:** `ClassName[property='value']` — e.g., `NavigationToggleButton[text='Start']`

### 3.4 Existence & State Assertion Keywords

| Keyword | Status | Notes |
|---------|--------|-------|
| `Element Should Exist` | **PASS** | Works correctly |
| `Element Should Not Exist` | **PASS** | Works correctly for nonexistent elements |
| `Element Should Be Enabled` | **PASS** | Works correctly |
| `Element Should Be Disabled` | Not tested | (No disabled elements in default state) |
| `Element Should Be Selected` | **PASS** | Works for Start button (selected by default) |
| `Element Should Not Be Selected` | **PASS** | Works for Pages button (unselected) |
| `Component Should Be Enabled` | **PASS** | Works correctly |
| `Component Should Be Visible` | **PASS** | Uses `isVisible()` — **passes** for SplitView children |
| `Element Should Be Visible` | **FAIL** | Uses `isShowing()` — **fails** for SplitView children |
| `Element Should Not Be Visible` | **PASS** (misleading) | Passes for components that ARE visible but not "showing" |

### 3.5 Text Assertion Keywords

| Keyword | Status | Notes |
|---------|--------|-------|
| `Element Text Should Be` | **PASS** | Exact text match works |
| `Element Text Should Contain` | **PASS** | Substring match works |

### 3.6 Wait Keywords

| Keyword | Status | Notes |
|---------|--------|-------|
| `Wait For Element` | **PASS** | Timeout accepts float (seconds), NOT Robot time strings like `3s` |
| `Wait Until Element Exists` | **PASS** | Works with float timeout |
| `Wait Until Element Does Not Exist` | **PASS** | Works correctly |
| `Wait Until Element Is Visible` | **FAIL** | Times out — uses `isShowing()` internally |
| `Set Timeout` | **PASS** | Accepts float seconds |
| `Set Assertion Timeout` | **PASS** | Returns previous value |
| `Set Assertion Interval` | **PASS** | Returns previous value |

### 3.7 Interactive Keywords (Click, Input)

| Keyword | Status | Error |
|---------|--------|-------|
| `Click` | **FAIL** | `IllegalStateException: Component not visible for click after window activation` |
| `Click Button` | **FAIL** | Same `IllegalStateException` |
| `Click Element` | **FAIL** | Same `IllegalStateException` |
| `Double Click` | **FAIL** | `IllegalStateException: Component not visible for double-click after window activation` |
| `Right Click` | **FAIL** | `EDT callable failed` |
| `Input Text` | **FAIL** | `IllegalStateException: Component is not visible` |

**All interactive keywords fail** because the agent-side Java code checks `isShowing()` before dispatching events and refuses to interact with components where `isShowing()` returns `false`.

### 3.8 Screenshot Keywords

| Keyword | Status | Notes |
|---------|--------|-------|
| `Capture Screenshot` | **PASS** | Returns filename: `screenshot_YYYYMMDD_HHMMSS.png` |
| `Set Screenshot Directory` | **PASS** | Accepts absolute path |

---

## 4. Critical Issue: `isShowing()` vs `isVisible()` Inconsistency

### 4.1 The Problem

The JGoodies `SplitView` component has a known behavior where its child panels report `java.awt.Component.isShowing() == false` even though:
- They ARE in the component tree
- `isVisible()` returns `true`
- `isEnabled()` returns `true`  
- Their parent `SplitView` reports `isShowing() == true`
- They are physically painted and visible on screen

### 4.2 Hierarchy `isShowing()` Survey

```
JFrame          → showing=True  ✓
  JRootPane     → showing=True  ✓
    SplitView   → showing=True  ✓
      JPanel (left sidebar)
        NavigationToggleButton[text='Start']  → showing=False  ✗
        NavigationToggleButton[text='Pages']  → showing=False  ✗
        JGSearchField                         → showing=False  ✗
        NavigationToggleButton[text='Settings'] → showing=False ✗
      JPanel (right content) 
        PageFrame → showing=True  ✓
        JLabel    → showing=False  ✗
        JProgressBar → showing=False ✗
```

The `isShowing()` breakpoint occurs at the JPanel children directly inside SplitView. `SplitView` itself reports `showing=True`, but its immediate JPanel children (which contain all the UI elements) do not. `PageFrame` is an exception — it does report `showing=True`.

### 4.3 Impact on the Library

This creates **two categories of inconsistency** in the library:

#### Inconsistency 1: `Element Should Be Visible` vs `Component Should Be Visible`
```
Component Should Be Visible    NavigationToggleButton[text='Start']    → PASS (uses isVisible)
Element Should Be Visible      NavigationToggleButton[text='Start']    → FAIL (uses isShowing)
```

#### Inconsistency 2: `Get Element States` vs `Element Should Be Visible`
```
Get Element States    NavigationToggleButton[text='Start']    → includes 'visible'
Element Should Be Visible    NavigationToggleButton[text='Start']    → FAIL
```

`Get Element States` reports `visible` (from `isVisible()`), but `Element Should Be Visible` checks `isShowing()` and fails. Users see a component listed as `visible` in states but cannot assert it as visible.

### 4.4 Recommendations for the Maintainer

1. **Add a `force_click` or `ignore_showing` parameter** to `Click`, `Double Click`, `Right Click`, and `Input Text` keywords that bypasses the `isShowing()` check when the user knows the component is in the tree and functional. The agent already has the component reference — it can dispatch events via `SwingUtilities.invokeLater()` directly.

2. **Unify `isVisible()` vs `isShowing()` semantics** across all keywords. Either:
   - (a) **Preferred:** Make `Element Should Be Visible` use `isVisible()` like `Component Should Be Visible` and `Get Element States` do, and add a separate `Element Should Be Showing` keyword for strict `isShowing()` checks.
   - (b) Document the distinction clearly and add an `Element Should Be Visible Or In Tree` keyword.

3. **Add a click strategy fallback** in the agent: When `isShowing()` is false but the component IS in the tree and `isVisible()` is true, try dispatching a synthetic `ActionEvent` or `MouseEvent` via `component.dispatchEvent()` instead of using `Robot.mousePress()` which requires screen coordinates.

4. **Fix the `Wait Until Element Is Visible` keyword** to accept an optional strategy parameter (`showing` vs `visible`), or default to `isVisible()` for consistency with `Get Element States`.

5. **Consider adding `Wait Until Component Is Visible`** as a counterpart to `Component Should Be Visible`.

---

## 5. Timeout Parameter Format Issue

### Problem
Wait keywords accept only float/int seconds, not Robot Framework time strings:
```
Wait For Element    locator    3s     → ValueError: cannot be converted to float
Wait For Element    locator    3      → PASS
```

### Recommendation
Use Robot Framework's built-in `robot.utils.timestr_to_secs()` to parse timeout arguments. This would allow users to write `3s`, `500ms`, `1 min`, `1:30`, etc., consistent with other RF libraries.

---

## 6. `__version__` Mismatch

`uv pip list` reports `robotframework-javagui==0.3.1`, but `JavaGui.__version__` returns `0.1.0`. The `__version__` string in the Python package should be updated to match the distribution metadata, or be dynamically derived from `importlib.metadata`.

---

## 7. Minor Observations

### 7.1 `List Applications` Returns Empty
`List Applications` always returns `[]` and is documented as a "placeholder". If JVM enumeration is not feasible, consider removing the keyword or raising `NotImplementedError` to avoid confusion.

### 7.2 `pid` Always `None` in Connection Info
`Get Connection Info` returns `pid: None`. The agent could report the JVM PID via the RPC handshake.

### 7.3 Duplicate/Alias Keywords
Several keywords appear to be aliases of each other:
- `Get Property` / `Get Element Property`
- `Get Properties` / `Get Element Properties`
- `Get Text` / `Get Element Text` / `Get Component Text`
- `Click` / `Click Element` / `Click Button`

Consider documenting these as official aliases or consolidating.

### 7.4 `Right Click` Error Differs
While `Click` and `Double Click` report `Component not visible for click/double-click`, `Right Click` reports `EDT callable failed` — a different, less informative error message.

### 7.5 `Get Element States` Returns Undocumented States
The `readonly` and `attached` states are returned but their meaning for Swing components isn't obvious. `readonly` is not a standard Swing property. Consider documenting what each state maps to in the Swing API.

### 7.6 Screenshot Works Without Active Display?
`Capture Screenshot` passes even when components report `isShowing()=false`. This suggests it uses `Robot.createScreenCapture()` or similar AWT mechanism which may capture an empty/black screenshot when components aren't truly rendered.

---

## 8. Component Inventory of Test Application

The JGoodies Smart Client Showcase 22.04.2 has the following component structure:

| Component Type | Count | Notes |
|----------------|-------|-------|
| JFrame | 1 | Title: "The Standard Design Library 22.04" |
| SplitView | 1 | JGoodies custom split pane |
| NavigationToggleButton | 34 | 27 in left nav scroll + 5 in right panel + 2 standalone |
| NavigationButton | 3 | Toolbar-style navigation buttons |
| JGSearchField | 1 | JGoodies search input field |
| PageFrame | 1 | Right-side content frame |
| JPanel | 15 | Container panels |
| ScrollBar | 4 | Scroll bars (2 per JScrollPane) |
| PlasticArrowButton | 4 | Scroll bar arrow buttons (JGoodies Plastic L&F) |
| JLabel | 1 | Status/content label |
| JProgressBar | 1 | Progress indicator |

### Navigation Sections (Left Sidebar)
Start, Pages, Hub Page, Master-Details, List Report, Object Page, Worklist, Initial Page, Dialogs, Messages, Input, Choice, Selection, Property, Progress, Wizards, Content, Forms, Hub / Tiles, Object Lists, Facets, Basics, Layout, Components, Validation, Completion, Data Binding, Settings

---

## 9. Working Test Patterns

Given the `isShowing()` limitation, the following patterns work reliably for testing:

### Pattern 1: Property-Based Verification
```robotframework
${selected} =    Get Property    NavigationToggleButton[text='Start']    selected
Should Be Equal    ${selected}    ${True}
```

### Pattern 2: Component-Level Visibility (Not Element-Level)
```robotframework
# Use this — checks isVisible()
Component Should Be Visible    NavigationToggleButton[text='Start']

# NOT this — checks isShowing(), fails in SplitView
# Element Should Be Visible    NavigationToggleButton[text='Start']
```

### Pattern 3: Conditional Click with Fallback
```robotframework
${showing} =    Get Property    NavigationToggleButton[text='Pages']    showing
IF    ${showing}
    Click    NavigationToggleButton[text='Pages']
ELSE
    Log    Button not showing (isShowing=false) — property-only verification
    ${text} =    Get Property    NavigationToggleButton[text='Pages']    text
    Should Be Equal    ${text}    Pages
END
```

### Pattern 4: Existence + State Verification
```robotframework
Element Should Exist    NavigationToggleButton[text='Hub Page']
Element Should Be Enabled    NavigationToggleButton[text='Hub Page']
Element Text Should Be    NavigationToggleButton[text='Hub Page']    Hub Page
```

---

## 10. Conclusion

The `JavaGui.Swing` library provides a solid foundation for Java Swing UI automation. The connection mechanism, property inspection, component tree traversal, and element assertion keywords work reliably. The critical gap is the `isShowing()` gate on interactive keywords, which prevents automation of applications using JGoodies SplitView (and potentially other custom containers that break the `isShowing()` contract). Resolving the `isShowing()` vs `isVisible()` inconsistency — both in the agent's click guard and in the library's assertion keywords — would make the library significantly more robust for real-world Swing applications.
