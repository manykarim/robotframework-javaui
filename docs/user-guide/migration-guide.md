# Migration Guide: Legacy Libraries to Unified Library

This guide helps you migrate from the legacy `SwingLibrary`, `SwtLibrary`, and `RcpLibrary` to the unified `JavaGuiLibrary` (or the new `JavaGui.Swing`, `JavaGui.Swt`, `JavaGui.Rcp` modules).

## Table of Contents

- [Overview](#overview)
- [Version Compatibility](#version-compatibility)
- [Quick Migration](#quick-migration)
- [Library Import Changes](#library-import-changes)
- [Keyword Mapping](#keyword-mapping)
- [Exception Changes](#exception-changes)
- [Locator Changes](#locator-changes)
- [Automated Migration Tool](#automated-migration-tool)
- [Step-by-Step Migration](#step-by-step-migration)
- [Common Issues](#common-issues)

## Overview

### What Changed

Version 2.1 introduces a unified architecture while maintaining full backwards compatibility:

| Aspect | Old (v2.0) | New (v2.1+) |
|--------|------------|-------------|
| Libraries | 3 separate libraries | Unified with mode selection |
| Keywords | Technology-specific names | Unified naming convention |
| Exceptions | Swing-prefixed names | Technology-agnostic names |
| Locators | Similar but separate | Unified with full compatibility |

### Backwards Compatibility

**Good news**: Version 2.1 maintains full backwards compatibility. Your existing tests will continue to work without changes.

However, we recommend migrating to the new unified API to:
- Simplify test maintenance
- Use consistent naming across technologies
- Prepare for version 3.0 (which removes deprecated APIs)

### Deprecation Timeline

| Version | Status | Legacy API Behavior |
|---------|--------|---------------------|
| 2.1.x | Current | Works without warnings |
| 2.2.x | Upcoming | Deprecation warnings |
| 3.0.x | Future | Removed |

## Version Compatibility

### Supported Versions

| Python | Robot Framework | JavaGui |
|--------|-----------------|---------|
| 3.8+ | 4.0+ | 2.1+ |
| 3.9+ | 5.0+ | 2.1+ |
| 3.10+ | 6.0+ | 2.1+ |

### Checking Your Version

```bash
pip show robotframework-javagui
```

## Quick Migration

### Minimal Changes (Recommended First Step)

The simplest migration just updates imports:

**Before:**
```robotframework
*** Settings ***
Library    SwingLibrary    timeout=10
```

**After:**
```robotframework
*** Settings ***
Library    JavaGui.Swing    timeout=10
```

This works immediately - all your existing keywords continue to work.

### Full Migration

For complete migration to the new unified API:

```robotframework
*** Settings ***
# Old
Library    SwingLibrary

# New
Library    JavaGui.Swing


*** Test Cases ***
# Old keywords still work
Old Style Test
    Click Element    name:button

# New unified keywords
New Style Test
    Click    name:button
```

## Library Import Changes

### Swing Applications

| Old Import | New Import |
|------------|------------|
| `Library    SwingLibrary` | `Library    JavaGui.Swing` |
| `Library    SwingLibrary    timeout=30` | `Library    JavaGui.Swing    timeout=30` |

### SWT Applications

| Old Import | New Import |
|------------|------------|
| `Library    SwtLibrary` | `Library    JavaGui.Swt` |
| `Library    SwtLibrary    timeout=30` | `Library    JavaGui.Swt    timeout=30` |

### RCP Applications

| Old Import | New Import |
|------------|------------|
| `Library    RcpLibrary` | `Library    JavaGui.Rcp` |
| `Library    RcpLibrary    timeout=30` | `Library    JavaGui.Rcp    timeout=30` |

### Python Import Changes

If you import the library in Python code:

```python
# Old
from swing_library import SwingLibrary
from swt_library import SwtLibrary
from rcp_library import RcpLibrary

# New
from JavaGui import Swing, Swt, Rcp

# Or use the class names directly
from JavaGui import SwingLibrary, SwtLibrary, RcpLibrary  # Still works
```

## Keyword Mapping

### Click Keywords

| Old Keyword | New Keyword | Notes |
|-------------|-------------|-------|
| `Click Element` | `Click` | Swing |
| `Click Widget` | `Click` | SWT |
| `Double Click Element` | `Double Click` | Swing |
| `Double Click Widget` | `Double Click` | SWT |
| `Right Click Element` | `Right Click` | Swing |
| `Right Click Widget` | `Right Click` | SWT |
| `Click Button` | `Click Button` | Unchanged |

### Element Finding Keywords

| Old Keyword | New Keyword | Notes |
|-------------|-------------|-------|
| `Find Element` | `Find Element` | Unchanged |
| `Find Elements` | `Find Elements` | Unchanged |
| `Find Widget` | `Find Element` | SWT |
| `Find Widgets` | `Find Elements` | SWT |

### Wait Keywords

| Old Keyword | New Keyword | Notes |
|-------------|-------------|-------|
| `Wait Until Element Exists` | `Wait Until Element Exists` | Unchanged |
| `Wait Until Widget Exists` | `Wait Until Element Exists` | SWT |
| `Wait Until Element Is Visible` | `Wait Until Element Is Visible` | Unchanged |
| `Wait Until Widget Enabled` | `Wait Until Element Is Enabled` | SWT |
| `Wait Until Element Is Enabled` | `Wait Until Element Is Enabled` | Unchanged |

### Verification Keywords

| Old Keyword | New Keyword | Notes |
|-------------|-------------|-------|
| `Element Should Be Visible` | `Element Should Be Visible` | Unchanged |
| `Widget Should Be Visible` | `Element Should Be Visible` | SWT |
| `Element Should Be Enabled` | `Element Should Be Enabled` | Unchanged |
| `Widget Should Be Enabled` | `Element Should Be Enabled` | SWT |
| `Widget Text Should Be` | `Element Text Should Be` | SWT |
| `Element Text Should Be` | `Element Text Should Be` | Unchanged |

### Selection Keywords

| Old Keyword | New Keyword | Notes |
|-------------|-------------|-------|
| `Check Checkbox` | `Check Checkbox` | Unchanged |
| `Check Button` | `Check Checkbox` | SWT |
| `Uncheck Checkbox` | `Uncheck Checkbox` | Unchanged |
| `Uncheck Button` | `Uncheck Checkbox` | SWT |
| `Select Radio Button` | `Select Radio Button` | Unchanged |
| `Select Combo Item` | `Select From Combobox` | SWT |
| `Select From Combobox` | `Select From Combobox` | Unchanged |
| `Select List Item` | `Select From List` | SWT |
| `Select From List` | `Select From List` | Unchanged |

### Tree Keywords

| Old Keyword | New Keyword | Notes |
|-------------|-------------|-------|
| `Expand Tree Node` | `Expand Tree Node` | Unchanged |
| `Expand Tree Item` | `Expand Tree Node` | SWT |
| `Collapse Tree Node` | `Collapse Tree Node` | Unchanged |
| `Collapse Tree Item` | `Collapse Tree Node` | SWT |
| `Select Tree Node` | `Select Tree Node` | Unchanged |
| `Select Tree Item` | `Select Tree Node` | SWT |

### Table Keywords

| Old Keyword | New Keyword | Notes |
|-------------|-------------|-------|
| `Get Table Cell Value` | `Get Table Cell Value` | Unchanged |
| `Get Table Cell` | `Get Table Cell Value` | SWT |
| `Select Table Row` | `Select Table Row` | Unchanged |
| `Select Table Cell` | `Select Table Cell` | Unchanged |
| `Get Table Row Count` | `Get Table Row Count` | Unchanged |

### Input Keywords

| Old Keyword | New Keyword | Notes |
|-------------|-------------|-------|
| `Input Text` | `Input Text` | Unchanged |
| `Clear Text` | `Clear Text` | Unchanged |
| `Type Text` | `Type Text` | Unchanged |

### Connection Keywords

| Old Keyword | New Keyword | Notes |
|-------------|-------------|-------|
| `Connect To Application` | `Connect To Application` | Unchanged |
| `Connect To Swt Application` | `Connect To Swt Application` | SWT (still works) |
| `Disconnect` | `Disconnect` | Unchanged |
| `Is Connected` | `Is Connected` | Unchanged |

### RCP-Specific Keywords

RCP keywords remain unchanged:

| Keyword | Notes |
|---------|-------|
| `Open Perspective` | RCP only |
| `Get Active Perspective` | RCP only |
| `Reset Perspective` | RCP only |
| `Show View` | RCP only |
| `Close View` | RCP only |
| `Activate View` | RCP only |
| `Open Editor` | RCP only |
| `Close Editor` | RCP only |
| `Close All Editors` | RCP only |
| `Save Editor` | RCP only |
| `Save All Editors` | RCP only |
| `Execute Command` | RCP only |
| `Wait For Workbench` | RCP only |

## Exception Changes

### Exception Mapping

| Old Exception | New Exception |
|---------------|---------------|
| `SwingConnectionError` | `ConnectionError` |
| `SwingTimeoutError` | `ActionTimeoutError` |
| `PyLocatorParseError` | `LocatorParseError` |

### New Exception Hierarchy

```
JavaGuiError (base)
+-- ConnectionError
|   +-- ConnectionRefusedError
|   +-- ConnectionTimeoutError
|   +-- NotConnectedError
+-- ElementError
|   +-- ElementNotFoundError
|   +-- MultipleElementsFoundError
|   +-- ElementNotInteractableError
|   +-- StaleElementError
+-- LocatorError
|   +-- LocatorParseError
|   +-- InvalidLocatorSyntaxError
+-- ActionError
|   +-- ActionFailedError
|   +-- ActionTimeoutError
|   +-- ActionNotSupportedError
+-- TechnologyError
    +-- ModeNotSupportedError
    +-- RcpWorkbenchError
    +-- SwtShellError
```

### Exception Handling Examples

**Old style (still works):**
```robotframework
*** Test Cases ***
Old Exception Handling
    ${error}=    Run Keyword And Expect Error    SwingConnectionError:*
    ...    Connect To Application    invalid    localhost    9999
```

**New style:**
```robotframework
*** Test Cases ***
New Exception Handling
    ${error}=    Run Keyword And Expect Error    ConnectionError:*
    ...    Connect To Application    invalid    localhost    9999

Catch Parent Exception
    # This catches all element-related errors
    ${error}=    Run Keyword And Expect Error    ElementError:*
    ...    Click    name:nonexistent
```

## Locator Changes

Locator syntax remains **fully compatible**. All existing locators continue to work.

### Recommendations

While your existing locators work, consider these improvements:

| Pattern | Old Style | New Recommended |
|---------|-----------|-----------------|
| By name | `name:button` | `#button` (shorthand) |
| SWT widget + attr | `[text='OK']` | `Button[text='OK']` |
| Multiple attrs | Complex XPath | `JButton[name='x'][enabled='true']` |

## Automated Migration Tool

### Installation

The migration tool is included with the library:

```bash
pip install robotframework-javagui[migrate]
```

### Usage

```bash
# Preview changes (dry run)
python -m javagui.migrate --dry-run tests/

# Migrate with backup
python -m javagui.migrate --backup tests/

# Migrate single file
python -m javagui.migrate --dry-run tests/login.robot

# Generate report
python -m javagui.migrate --report tests/
```

### What the Tool Migrates

1. Library import statements
2. Deprecated keyword names
3. Old exception names in error handling
4. Generates detailed report of changes

### Example Output

```
================================================================
MIGRATION REPORT
================================================================

SUMMARY:
  Total changes: 47
  library_import: 5
  keyword_rename: 38
  exception_rename: 4

CHANGES:

tests/login.robot:
  Line 2: library_import
    - Library    SwingLibrary    timeout=10
    + Library    JavaGui.Swing    timeout=10
  Line 15: keyword_rename
    - Click Element    name:loginButton
    + Click    name:loginButton

tests/data_entry.robot:
  Line 8: keyword_rename
    - Wait Until Widget Exists    Table#data
    + Wait Until Element Exists    Table#data
...
```

## Step-by-Step Migration

### Step 1: Update Dependencies

```bash
pip install --upgrade robotframework-javagui
```

### Step 2: Run Tests (Verify Baseline)

Run your existing tests to ensure they pass before migration:

```bash
robot tests/
```

### Step 3: Run Migration Tool (Dry Run)

```bash
python -m javagui.migrate --dry-run --report tests/
```

Review the report to understand what will change.

### Step 4: Backup Your Tests

```bash
cp -r tests/ tests_backup/
```

### Step 5: Run Migration

```bash
python -m javagui.migrate tests/
```

### Step 6: Run Tests Again

```bash
robot tests/
```

All tests should pass. If any fail, check the migration report and fix manually.

### Step 7: Manual Review

Review changes for:
- Custom keywords using deprecated APIs
- Exception handling that may need updates
- Any edge cases the tool might have missed

### Step 8: Clean Up Backups

Once satisfied:

```bash
rm -rf tests_backup/
rm tests/*.robot.bak  # If individual backups were created
```

## Common Issues

### Issue: Import Not Found

**Problem:**
```
ModuleNotFoundError: No module named 'JavaGui'
```

**Solution:**
```bash
pip install --upgrade robotframework-javagui
```

### Issue: Keyword Not Found After Migration

**Problem:**
```
No keyword with name 'Click Widget' found.
```

**Solution:**
The migration tool should have renamed this. Check if:
1. The file was actually migrated
2. The keyword is defined in a resource file that wasn't migrated

```bash
python -m javagui.migrate path/to/resource/files/
```

### Issue: Exception Handler Not Working

**Problem:**
```robotframework
# This no longer catches the expected error
${error}=    Run Keyword And Expect Error    SwingConnectionError:*
...    Connect To Application    invalid    localhost    9999
```

**Solution:**
Update to new exception name:
```robotframework
${error}=    Run Keyword And Expect Error    ConnectionError:*
...    Connect To Application    invalid    localhost    9999
```

### Issue: Custom Library Extension

**Problem:** You have a custom library extending `SwingLibrary`.

**Solution:**
Update your extension:

```python
# Old
from swing_library import SwingLibrary

class MyCustomLibrary(SwingLibrary):
    pass

# New
from JavaGui import SwingLibrary  # Or: from JavaGui import Swing

class MyCustomLibrary(SwingLibrary):
    pass
```

### Issue: Variable File Using Library Types

**Problem:** Variable file references old types.

**Solution:**
Update imports:
```python
# Old
from swing_library import SwingLibrary
LIBRARY = SwingLibrary(timeout=30)

# New
from JavaGui import Swing
LIBRARY = Swing(timeout=30)
```

## Getting Help

If you encounter issues during migration:

1. Check this guide for common issues
2. Run with `--verbose` for detailed output:
   ```bash
   python -m javagui.migrate --verbose --dry-run tests/
   ```
3. Check the [GitHub Issues](https://github.com/your-org/robotframework-javagui/issues)
4. Open a new issue with:
   - JavaGui version
   - Python version
   - Robot Framework version
   - Error message
   - Relevant test code

## Related Documentation

- [Unified Library Guide](unified-library.md)
- [Locator Reference](locator-reference.md)
- [ADR-006: Backwards Compatibility](../adr/ADR-006-backwards-compatibility-approach.md)
