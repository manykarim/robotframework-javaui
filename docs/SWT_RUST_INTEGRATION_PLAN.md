# SWT Rust Integration Plan

## Overview

This document details the plan to integrate SWT support into the Rust layer of the robotframework-swing library.

## Current Architecture

### Rust Layer (`src/python/swing_library.rs`)

```rust
pub struct SwingLibrary {
    config: Arc<RwLock<LibraryConfig>>,
    connection: Arc<RwLock<ConnectionState>>,
    ui_tree: Arc<RwLock<Option<UITree>>>,
    element_cache: Arc<RwLock<HashMap<String, SwingElement>>>,
}

impl SwingLibrary {
    // RPC communication
    fn send_rpc_request(&self, method: &str, params: serde_json::Value) -> Result<serde_json::Value>;

    // Current Swing methods
    pub fn get_component_tree(&self, ...) -> PyResult<String>;
    // ... other Swing-specific methods
}
```

### Java RPC Server

**Swing**: `com.robotframework.swing.RpcServer` (port 18080)
**SWT**: `com.robotframework.swt.SwtRpcServer` (port 18081)

## Integration Strategy

### Option 1: Dual-Framework Approach (Recommended)

Create a framework-agnostic interface that detects and delegates to the appropriate backend:

```rust
#[derive(Clone, Copy, PartialEq, Debug)]
enum UIFramework {
    Swing,
    SWT,
    Unknown,
}

struct ConnectionState {
    // ... existing fields
    framework: UIFramework,  // NEW FIELD
    swt_port: Option<u16>,   // NEW FIELD (default: 18081)
}

impl SwingLibrary {
    /// Auto-detect UI framework (Swing or SWT)
    fn detect_framework(&self) -> Result<UIFramework> {
        // Try SWT first (call getShells)
        if self.send_rpc_request("getShells", serde_json::json!({})).is_ok() {
            return Ok(UIFramework::SWT);
        }

        // Try Swing (call getComponentTree)
        if self.send_rpc_request("getComponentTree", serde_json::json!({})).is_ok() {
            return Ok(UIFramework::Swing);
        }

        Ok(UIFramework::Unknown)
    }

    /// Get component tree (framework-agnostic)
    pub fn get_component_tree(&self, ...) -> PyResult<String> {
        let framework = self.detect_framework()?;

        match framework {
            UIFramework::Swing => self.get_swing_component_tree(...),
            UIFramework::SWT => self.get_swt_widget_tree(...),
            UIFramework::Unknown => Err(SwingError::connection("Unknown UI framework")),
        }
    }

    /// Get Swing component tree
    fn get_swing_component_tree(&self, ...) -> PyResult<String> {
        // Existing implementation
        let result = self.send_rpc_request("getComponentTree", params)?;
        // ...
    }

    /// Get SWT widget tree
    fn get_swt_widget_tree(&self, ...) -> PyResult<String> {
        // NEW: Call SWT RPC method
        let result = self.send_rpc_request("getWidgetTree", params)?;
        // Convert to UITree format
        // ...
    }
}
```

### Option 2: Separate SWT Library (Not Recommended)

Create a completely separate `SwtLibrary` class. This would:
- Duplicate code
- Confuse users
- Increase maintenance burden

## Implementation Steps

### Phase 1: Framework Detection

1. **Add framework field to ConnectionState**
   ```rust
   struct ConnectionState {
       // ... existing fields
       framework: UIFramework,
       swt_port: Option<u16>,
   }
   ```

2. **Implement detection logic**
   ```rust
   fn detect_framework(&self) -> Result<UIFramework> {
       // Try SWT getShells call
       // Try Swing getComponentTree call
       // Return detected framework
   }
   ```

3. **Add connect methods**
   ```rust
   pub fn connect_to_swt(&mut self, host: &str, port: u16) -> PyResult<()>;
   pub fn connect_to_swing(&mut self, host: &str, port: u16) -> PyResult<()>;
   ```

### Phase 2: SWT Widget Tree Support

1. **Add SWT-specific data structures**
   ```rust
   #[derive(Clone, Debug)]
   struct SwtWidget {
       id: i32,
       class: String,
       text: Option<String>,
       bounds: Option<Bounds>,
       visible: bool,
       enabled: bool,
       children: Vec<SwtWidget>,
   }
   ```

2. **Add SWT tree retrieval**
   ```rust
   fn get_swt_widget_tree(&self, max_depth: Option<u32>) -> Result<UITree> {
       let params = serde_json::json!({
           "maxDepth": max_depth.unwrap_or(10)
       });

       let result = self.send_rpc_request("getWidgetTree", params)?;
       self.swt_json_to_ui_tree(&result)
   }
   ```

3. **Add SWT to UITree conversion**
   ```rust
   fn swt_json_to_ui_tree(&self, json: &serde_json::Value) -> Result<UITree> {
       // Convert SWT JSON to common UITree format
       // Map SWT widget types to ComponentType
       // Handle SWT-specific properties
   }
   ```

### Phase 3: SWT Actions Support

1. **Add SWT-specific action methods**
   ```rust
   // Widget actions
   pub fn swt_click(&self, widget_id: i32) -> PyResult<()>;
   pub fn swt_set_text(&self, widget_id: i32, text: &str) -> PyResult<()>;
   pub fn swt_select_item(&self, widget_id: i32, item: &str) -> PyResult<()>;

   // Table operations
   pub fn swt_select_table_row(&self, widget_id: i32, row: i32) -> PyResult<()>;
   pub fn swt_get_table_data(&self, widget_id: i32) -> PyResult<String>;

   // Tree operations
   pub fn swt_select_tree_item(&self, widget_id: i32, path: &str) -> PyResult<()>;
   pub fn swt_expand_tree_item(&self, widget_id: i32, path: &str) -> PyResult<()>;
   ```

2. **Add framework-agnostic wrappers**
   ```rust
   pub fn click_element(&self, locator: &str) -> PyResult<()> {
       let framework = self.detect_framework()?;

       match framework {
           UIFramework::Swing => {
               let element = self.find_element(locator)?;
               self.swing_click(element.id)
           },
           UIFramework::SWT => {
               let widget_id = self.find_widget(locator)?;
               self.swt_click(widget_id)
           },
           _ => Err(SwingError::unsupported("Unknown framework"))
       }
   }
   ```

### Phase 4: SWT Locators Support

1. **Add SWT widget finding**
   ```rust
   fn find_swt_widget(&self, locator: &str) -> Result<i32> {
       // Parse locator (text, class, tooltip, data, etc.)
       let (locator_type, value) = parse_swt_locator(locator)?;

       let params = serde_json::json!({
           "type": locator_type,
           "value": value
       });

       let result = self.send_rpc_request("findWidget", params)?;
       Ok(result["id"].as_i64().unwrap() as i32)
   }
   ```

2. **Support SWT-specific locator types**
   - `text=Button Text`
   - `class=org.eclipse.swt.widgets.Button`
   - `tooltip=Click me`
   - `data=mykey=myvalue`

### Phase 5: Output Formatters

1. **Add SWT output formatters**
   ```rust
   pub fn format_swt_tree(&self, tree: &UITree, format: &str) -> String {
       match format {
           "json" => self.format_swt_json(tree),
           "plain" => self.format_swt_plain(tree),
           "tree" => self.format_swt_tree_view(tree),
           "html" => self.format_swt_html(tree),
           _ => self.format_swt_json(tree)
       }
   }
   ```

## SWT Widget Type Mapping

Map SWT widget classes to common ComponentType:

| SWT Class | ComponentType | Swing Equivalent |
|-----------|--------------|------------------|
| Shell | Window | JFrame, JDialog |
| Button | Button | JButton |
| Label | Label | JLabel |
| Text | TextField | JTextField |
| Combo | ComboBox | JComboBox |
| List | List | JList |
| Table | Table | JTable |
| Tree | Tree | JTree |
| TabFolder | TabbedPane | JTabbedPane |
| Group | Panel | JPanel |
| Composite | Panel | JPanel |

## Testing Strategy

### Unit Tests (Rust)

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_framework_detection() {
        // Mock RPC responses
        // Test Swing detection
        // Test SWT detection
    }

    #[test]
    fn test_swt_widget_tree_parsing() {
        // Parse sample SWT JSON
        // Verify UITree conversion
    }

    #[test]
    fn test_swt_locator_parsing() {
        // Test various SWT locator formats
    }
}
```

### Integration Tests (Python)

```python
def test_swt_connection():
    """Test connecting to SWT application"""
    lib = JavaGui()
    lib.connect_to_swt("localhost", 18081)
    assert lib.is_connected()

def test_swt_widget_tree():
    """Test getting SWT widget tree"""
    lib = JavaGui()
    lib.connect_to_swt("localhost", 18081)
    tree = lib.get_component_tree()
    assert "Shell" in tree

def test_swt_actions():
    """Test SWT widget actions"""
    lib = JavaGui()
    lib.connect_to_swt("localhost", 18081)
    lib.click_element("text=OK")
    lib.input_text("text=Name", "John Doe")
```

## File Changes Required

### New Files
- `/src/swt/mod.rs` - SWT-specific module
- `/src/swt/widgets.rs` - SWT widget types
- `/src/swt/locators.rs` - SWT locator parsing
- `/src/swt/formatters.rs` - SWT output formatters
- `/tests/swt/test_integration.py` - SWT integration tests

### Modified Files
- `/src/python/swing_library.rs` - Add framework detection and SWT methods
- `/src/lib.rs` - Add SWT module
- `/src/model.rs` - Extend ComponentType for SWT widgets
- `/python/JavaGui/__init__.py` - Add SWT Python keywords
- `/Cargo.toml` - Add SWT feature flag (optional)

## API Design

### Python Keywords (Framework-Agnostic)

```python
# Connection
Connect To Application    host    port    framework=auto
Connect To Swing          host    port
Connect To SWT            host    port

# Framework detection
${framework}=    Get Framework Type
Should Be Swing Framework
Should Be SWT Framework

# Component tree (auto-detects framework)
${tree}=    Get Component Tree    format=json    max_depth=10
${tree}=    Get Component Tree    output=plain

# Element interaction (works with both frameworks)
Click Element        text=OK
Input Text           text=Name    John Doe
Select From List     text=Country    United States

# Framework-specific (if needed)
Click Swing Component     xpath=//JButton[@text='OK']
Click SWT Widget          text=OK    # Uses SWT RPC
```

## Performance Considerations

1. **Caching**: Cache framework detection result
2. **Connection pooling**: Reuse TCP connections
3. **JSON parsing**: Use serde_json for efficient parsing
4. **Memory**: Clear widget caches periodically

## Error Handling

```rust
#[derive(Debug)]
enum SwtError {
    WidgetNotFound(String),
    WidgetDisposed(i32),
    NotOnDisplayThread,
    DisplayNotAvailable,
    InvalidLocator(String),
}

impl From<SwtError> for SwingError {
    fn from(err: SwtError) -> SwingError {
        match err {
            SwtError::WidgetNotFound(msg) => SwingError::element_not_found(&msg),
            SwtError::WidgetDisposed(id) => SwingError::element_not_found(&format!("Widget {} disposed", id)),
            // ... other conversions
        }
    }
}
```

## Configuration

### Library Initialization

```python
# Auto-detect framework
lib = JavaGui()
lib.connect_to_application("localhost", 18080)  # Tries Swing port
# OR
lib.connect_to_application("localhost", 18081)  # Tries SWT port

# Explicit framework
lib = JavaGui(framework="swt")
lib.connect("localhost", 18081)

# Configure SWT-specific settings
lib.set_swt_timeout(30)  # Timeout for Display thread operations
lib.enable_swt_caching(True)  # Cache widget IDs
```

## Timeline

| Phase | Task | Estimated Effort | Dependencies |
|-------|------|-----------------|--------------|
| 1 | Framework detection | 4 hours | None |
| 2 | SWT widget tree | 6 hours | Phase 1 |
| 3 | SWT actions | 8 hours | Phase 2 |
| 4 | SWT locators | 4 hours | Phase 3 |
| 5 | Output formatters | 4 hours | Phase 2 |
| 6 | Testing | 8 hours | All phases |
| 7 | Documentation | 4 hours | All phases |

**Total**: 38 hours (~5 days)

## Risk Mitigation

| Risk | Mitigation |
|------|-----------|
| Widget ID conflicts between Swing and SWT | Use framework-specific ID spaces |
| Thread safety issues | Use proper locking, respect Display thread |
| Memory leaks from caching | Use WeakHashMap pattern, periodic cleanup |
| Type conversion errors | Comprehensive error handling, type validation |

## Success Criteria

1. ✅ Can detect Swing vs SWT applications automatically
2. ✅ Can retrieve SWT widget tree with all properties
3. ✅ Can perform actions on SWT widgets (click, text input, selection)
4. ✅ Can locate SWT widgets using multiple locator strategies
5. ✅ All output formatters work with SWT trees
6. ✅ Integration tests pass on all platforms
7. ✅ Performance comparable to Swing backend (<100ms overhead)

## Next Steps

1. Create `/src/swt/mod.rs` module structure
2. Implement framework detection in `swing_library.rs`
3. Add SWT widget tree retrieval
4. Implement SWT action methods
5. Add comprehensive tests
6. Update Python keywords
7. Write user documentation

---

*Version: 1.0*
*Last Updated: 2026-01-22*
*Status: Planning Phase*
