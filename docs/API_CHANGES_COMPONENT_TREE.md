# Component Tree API Bug Fixes and Enhancements

## Overview

This document describes the bug fixes and API enhancements made to the component tree methods in SwingLibrary.

## Fixed Issues

### 1. `get_component_tree` - Parameter Passing Bug

**Bug Description:**
The `get_component_tree` method was incorrectly passing the `locator` parameter as the first argument to the Rust backend's `get_ui_tree` method, which expects `format` as the first parameter.

**Old Buggy Code:**
```python
def get_component_tree(self, locator: Optional[str] = None, format: str = "text", max_depth: Optional[int] = None) -> str:
    tree_str = self._lib.get_ui_tree(locator)  # BUG: locator passed as format!
    return tree_str
```

**New Fixed Code:**
```python
def get_component_tree(self, locator: Optional[str] = None, format: str = "text", max_depth: Optional[int] = None) -> str:
    # Note: locator parameter is currently not supported by the Rust backend
    if locator is not None:
        import warnings
        warnings.warn(
            "The 'locator' parameter is not yet supported in get_component_tree. "
            "Returning full component tree instead.",
            DeprecationWarning,
            stacklevel=2
        )

    # Call the correct method with proper parameters
    tree_str = self._lib.get_ui_tree(format, max_depth, False)
    return tree_str
```

**Impact:**
- ✅ `format` parameter now works correctly
- ✅ `max_depth` parameter now works correctly
- ⚠️ `locator` parameter shows deprecation warning (not yet supported by Rust backend)

### 2. `save_ui_tree` - Missing Format and Max Depth Parameters

**Bug Description:**
The `save_ui_tree` method only accepted `filename` and `locator` parameters, but did not support `format` or `max_depth` parameters, making it impossible to save trees in different formats or with depth limits.

**Old Limited API:**
```python
def save_ui_tree(self, filename: str, locator: Optional[str] = None) -> None:
    self._lib.save_ui_tree(filename, locator)
```

**New Enhanced API:**
```python
def save_ui_tree(
    self,
    filename: str,
    locator: Optional[str] = None,
    format: str = "text",
    max_depth: Optional[int] = None
) -> None:
    # Note: locator parameter is currently not supported by the Rust backend
    if locator is not None:
        import warnings
        warnings.warn(
            "The 'locator' parameter is not yet supported in save_ui_tree. "
            "Saving full component tree instead.",
            DeprecationWarning,
            stacklevel=2
        )

    # Get the tree with the specified format and depth
    tree_content = self._lib.get_ui_tree(format, max_depth, False)

    # Write to file
    with open(filename, 'w', encoding='utf-8') as f:
        f.write(tree_content)
```

**Impact:**
- ✅ Can now save trees in different formats (text, json, xml)
- ✅ Can now limit tree depth when saving
- ✅ Proper UTF-8 encoding support
- ⚠️ `locator` parameter shows deprecation warning (not yet supported by Rust backend)

## API Changes

### `get_component_tree`

**Signature:**
```python
def get_component_tree(
    self,
    locator: Optional[str] = None,
    format: str = "text",
    max_depth: Optional[int] = None,
) -> str
```

**Parameters:**
- `locator` (Optional[str]): **Deprecated** - Not yet supported. Will show warning if provided.
- `format` (str): Output format: `"text"`, `"json"`, or `"yaml"`. Default: `"text"`.
- `max_depth` (Optional[int]): Maximum depth to traverse. `None` for unlimited.

**Returns:**
- `str`: The component tree in the specified format.

**Example Usage:**
```robot
*** Test Cases ***
Get Tree in Different Formats
    ${text_tree}=    Get Component Tree                        # Default text format
    ${json_tree}=    Get Component Tree    format=json          # JSON format
    ${xml_tree}=     Get Component Tree    format=xml           # XML format
    ${limited}=      Get Component Tree    max_depth=3          # Limit depth to 3
    ${custom}=       Get Component Tree    format=json    max_depth=5
```

### `save_ui_tree`

**Signature:**
```python
def save_ui_tree(
    self,
    filename: str,
    locator: Optional[str] = None,
    format: str = "text",
    max_depth: Optional[int] = None
) -> None
```

**Parameters:**
- `filename` (str): Path to save the tree file.
- `locator` (Optional[str]): **Deprecated** - Not yet supported. Will show warning if provided.
- `format` (str): Output format: `"text"`, `"json"`, or `"xml"`. Default: `"text"`.
- `max_depth` (Optional[int]): Maximum depth to traverse. `None` for unlimited.

**Example Usage:**
```robot
*** Test Cases ***
Save Tree in Different Formats
    Save UI Tree    tree.txt                                    # Default text format
    Save UI Tree    tree.json    format=json                    # JSON format
    Save UI Tree    tree.xml     format=xml                     # XML format
    Save UI Tree    limited.txt  max_depth=3                    # Limit depth to 3
    Save UI Tree    custom.json  format=json    max_depth=5     # JSON with depth limit
```

## Backward Compatibility

### Breaking Changes
**None** - All changes are backward compatible.

### Deprecation Warnings

The following usage patterns will show deprecation warnings:

```robot
*** Test Cases ***
Using Locator Parameter (Deprecated)
    # This will show a deprecation warning
    ${tree}=    Get Component Tree    locator=JPanel#main

    # This will also show a deprecation warning
    Save UI Tree    tree.txt    JPanel#main
```

**Warning Message:**
```
DeprecationWarning: The 'locator' parameter is not yet supported in get_component_tree.
Returning full component tree instead.
```

### Migration Guide

**Before (Old API):**
```robot
*** Test Cases ***
Old Usage
    # Could only get/save text format
    ${tree}=    Get Component Tree
    Save UI Tree    tree.txt
```

**After (New API):**
```robot
*** Test Cases ***
New Usage
    # Can now specify format and depth
    ${text}=     Get Component Tree                           # Still works as before
    ${json}=     Get Component Tree    format=json            # New: JSON format
    ${limited}=  Get Component Tree    max_depth=3            # New: Depth limit

    Save UI Tree    tree.txt                                  # Still works as before
    Save UI Tree    tree.json    format=json                  # New: JSON format
    Save UI Tree    limited.txt  max_depth=3                  # New: Depth limit
```

## Test Coverage

Comprehensive test coverage has been added:

- ✅ Parameter passing correctness
- ✅ All format types (text, json, xml)
- ✅ Depth limiting
- ✅ File I/O with UTF-8 encoding
- ✅ Deprecation warnings for unsupported parameters
- ✅ Regression tests for the specific bugs fixed

**Test File:** `tests/python/test_component_tree_unit.py`

**Run Tests:**
```bash
uv run pytest tests/python/test_component_tree_unit.py -v
```

## Future Enhancements

### Planned Features

1. **Locator Support**: Add support for scoped tree retrieval by locator
   - This requires Rust backend implementation
   - When implemented, deprecation warnings will be removed

2. **Additional Formats**: Support for more output formats
   - YAML format (currently listed but maps to text)
   - HTML format for visual documentation
   - CSV format for data analysis

3. **Filtering Options**: Add filtering capabilities
   - Filter by component type
   - Filter by visibility
   - Filter by enabled state

## Technical Details

### Rust Backend Interface

The Python wrapper calls the Rust backend's `get_ui_tree` method:

**Expected Signature:**
```rust
pub fn get_ui_tree(
    &self,
    format: &str,         // "text", "json", or "xml"
    max_depth: Option<i32>,  // None for unlimited
    visible_only: bool    // Filter to visible components only
) -> String
```

### File Encoding

All files are saved with UTF-8 encoding to support international characters and Unicode symbols in component names.

## Related Files

- Python Implementation: `/python/JavaGui/__init__.py`
- Unit Tests: `/tests/python/test_component_tree_unit.py`
- Integration Tests: `/tests/python/test_component_tree.py`
- This Documentation: `/docs/API_CHANGES_COMPONENT_TREE.md`

## Version Information

- **Fixed in Version:** 0.3.0 (unreleased)
- **Last Updated:** 2026-01-22
