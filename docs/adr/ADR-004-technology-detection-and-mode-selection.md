# ADR-004: Technology Detection and Mode Selection

| ADR ID | ADR-004 |
|--------|---------|
| Title | Technology Detection and Mode Selection |
| Status | Proposed |
| Date | 2026-01-16 |
| Authors | Architecture Team |

## Context

The library supports three Java GUI technologies (Swing, SWT, RCP), each requiring different:

- Java agent code paths
- Widget class hierarchies
- Connection protocols (same JSON-RPC, different methods)
- Locator interpretation
- Available keywords

### Current Approach

Users must choose the correct library class:

```robot
# User must know which technology to use
Library    SwingLibrary    # For Swing apps
Library    SwtLibrary      # For SWT apps
Library    RcpLibrary      # For Eclipse RCP apps
```

### Problems with Current Approach

1. **User Burden**: User must know the application's technology beforehand
2. **Wrong Choice**: Using wrong library gives confusing errors
3. **Mode Switching**: Can't test hybrid applications (Swing embedded in RCP)
4. **No Flexibility**: Can't change mode mid-test

### Decision Drivers

- Minimize user configuration burden
- Support automatic detection where possible
- Allow explicit override for edge cases
- Support hybrid applications
- Maintain backwards compatibility

## Decision

We will implement **Hybrid Mode Selection** with three strategies:

1. **Auto-Detection** (default): Detect technology from connection
2. **Explicit Mode**: User specifies technology
3. **Dynamic Mode**: Query and switch modes at runtime

### 1. Mode Configuration Options

```robot
*** Settings ***
# Option 1: Auto-detect (recommended for most cases)
Library    JavaGuiLibrary

# Option 2: Auto-detect with fallback
Library    JavaGuiLibrary    mode=auto

# Option 3: Explicit mode
Library    JavaGuiLibrary    mode=swing
Library    JavaGuiLibrary    mode=swt
Library    JavaGuiLibrary    mode=rcp

# Option 4: Backwards compatible (specific library)
Library    SwingLibrary
Library    SwtLibrary
Library    RcpLibrary
```

### 2. Auto-Detection Algorithm

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuiMode {
    Swing,
    Swt,
    Rcp,
    Auto,     // Will detect on connection
    Unknown,  // Detection failed
}

pub struct TechnologyDetector {
    /// Cached detection result
    detected_mode: Option<GuiMode>,
}

impl TechnologyDetector {
    /// Detect technology from connected application
    pub fn detect(&mut self, connection: &ConnectionState) -> PyResult<GuiMode> {
        // Return cached result if available
        if let Some(mode) = self.detected_mode {
            return Ok(mode);
        }

        // Step 1: Query agent for technology info
        let tech_info = connection.send_rpc_request("system.getTechnologyInfo", json!({}))?;

        // Step 2: Parse response
        let mode = self.parse_technology_info(&tech_info)?;

        // Step 3: Cache and return
        self.detected_mode = Some(mode);
        Ok(mode)
    }

    fn parse_technology_info(&self, info: &serde_json::Value) -> PyResult<GuiMode> {
        // Priority-based detection
        let has_swing = info.get("hasSwing").and_then(|v| v.as_bool()).unwrap_or(false);
        let has_swt = info.get("hasSwt").and_then(|v| v.as_bool()).unwrap_or(false);
        let has_rcp = info.get("hasRcp").and_then(|v| v.as_bool()).unwrap_or(false);
        let primary = info.get("primaryTechnology").and_then(|v| v.as_str());

        // Use explicit primary if available
        if let Some(tech) = primary {
            return Ok(match tech.to_lowercase().as_str() {
                "swing" => GuiMode::Swing,
                "swt" => GuiMode::Swt,
                "rcp" => GuiMode::Rcp,
                _ => self.infer_from_flags(has_swing, has_swt, has_rcp),
            });
        }

        // Otherwise infer from flags
        Ok(self.infer_from_flags(has_swing, has_swt, has_rcp))
    }

    fn infer_from_flags(&self, has_swing: bool, has_swt: bool, has_rcp: bool) -> GuiMode {
        // RCP implies SWT, so check RCP first
        if has_rcp {
            return GuiMode::Rcp;
        }
        if has_swt {
            return GuiMode::Swt;
        }
        if has_swing {
            return GuiMode::Swing;
        }
        GuiMode::Unknown
    }
}
```

### 3. Java Agent Technology Detection

The Java agent implements technology detection:

```java
public class TechnologyDetector {
    public static Map<String, Object> getTechnologyInfo() {
        Map<String, Object> info = new HashMap<>();

        // Check for Swing
        info.put("hasSwing", hasSwingComponents());

        // Check for SWT
        info.put("hasSwt", hasSwtWidgets());

        // Check for RCP
        info.put("hasRcp", hasRcpWorkbench());

        // Determine primary technology
        info.put("primaryTechnology", determinePrimary());

        // Additional metadata
        info.put("javaVersion", System.getProperty("java.version"));
        info.put("osName", System.getProperty("os.name"));

        return info;
    }

    private static boolean hasSwingComponents() {
        try {
            // Check if any JFrame windows exist
            Frame[] frames = Frame.getFrames();
            for (Frame f : frames) {
                if (f instanceof JFrame) {
                    return true;
                }
            }
            return false;
        } catch (Throwable t) {
            return false;
        }
    }

    private static boolean hasSwtWidgets() {
        try {
            // Check if Display.getDefault() returns non-null
            Class<?> displayClass = Class.forName("org.eclipse.swt.widgets.Display");
            Method getDefault = displayClass.getMethod("getDefault");
            Object display = getDefault.invoke(null);
            return display != null;
        } catch (Throwable t) {
            return false;
        }
    }

    private static boolean hasRcpWorkbench() {
        try {
            // Check if PlatformUI.getWorkbench() returns non-null
            Class<?> platformClass = Class.forName("org.eclipse.ui.PlatformUI");
            Method getWorkbench = platformClass.getMethod("getWorkbench");
            Object workbench = getWorkbench.invoke(null);
            return workbench != null;
        } catch (Throwable t) {
            return false;
        }
    }

    private static String determinePrimary() {
        // RCP > SWT > Swing (priority order)
        if (hasRcpWorkbench()) return "rcp";
        if (hasSwtWidgets()) return "swt";
        if (hasSwingComponents()) return "swing";
        return "unknown";
    }
}
```

### 4. Connection Management with Mode

```rust
#[pyclass]
pub struct JavaGuiLibrary {
    mode: GuiMode,
    explicit_mode: bool,  // True if user specified mode
    detector: TechnologyDetector,
    connection: Arc<RwLock<ConnectionState>>,
    // ... other fields
}

#[pymethods]
impl JavaGuiLibrary {
    #[new]
    #[pyo3(signature = (mode="auto", timeout=10.0))]
    pub fn new(mode: &str, timeout: f64) -> PyResult<Self> {
        let (gui_mode, explicit) = match mode.to_lowercase().as_str() {
            "swing" => (GuiMode::Swing, true),
            "swt" => (GuiMode::Swt, true),
            "rcp" => (GuiMode::Rcp, true),
            "auto" | _ => (GuiMode::Auto, false),
        };

        Ok(Self {
            mode: gui_mode,
            explicit_mode: explicit,
            detector: TechnologyDetector::new(),
            connection: Arc::new(RwLock::new(ConnectionState::default())),
            // ...
        })
    }

    /// Connect to application with optional mode override
    #[pyo3(signature = (application, host="localhost", port=5678, timeout=30.0, mode=None))]
    pub fn connect_to_application(
        &mut self,
        application: &str,
        host: &str,
        port: u16,
        timeout: f64,
        mode: Option<&str>,
    ) -> PyResult<()> {
        // Establish connection first
        self.establish_connection(application, host, port, timeout)?;

        // Handle mode
        if let Some(m) = mode {
            // Mode override in connect call
            self.mode = self.parse_mode(m)?;
            self.explicit_mode = true;
        } else if !self.explicit_mode {
            // Auto-detect if not explicitly set
            let conn = self.connection.read().map_err(|_| {
                SwingError::connection("Failed to acquire connection lock")
            })?;
            self.mode = self.detector.detect(&conn)?;
        }

        // Verify mode is valid for this connection
        self.verify_mode_compatibility()?;

        Ok(())
    }

    /// Get current technology mode
    pub fn get_mode(&self) -> String {
        match self.mode {
            GuiMode::Swing => "swing".to_string(),
            GuiMode::Swt => "swt".to_string(),
            GuiMode::Rcp => "rcp".to_string(),
            GuiMode::Auto => "auto".to_string(),
            GuiMode::Unknown => "unknown".to_string(),
        }
    }

    /// Set technology mode at runtime
    pub fn set_mode(&mut self, mode: &str) -> PyResult<()> {
        let new_mode = self.parse_mode(mode)?;

        // Verify mode is compatible with connected application
        if self.is_connected()? {
            self.verify_mode_compatibility_for(new_mode)?;
        }

        self.mode = new_mode;
        self.explicit_mode = true;
        Ok(())
    }

    /// Query available modes for connected application
    pub fn get_available_modes(&self) -> PyResult<Vec<String>> {
        if !self.is_connected()? {
            return Err(SwingError::connection("Not connected").into());
        }

        let conn = self.connection.read().map_err(|_| {
            SwingError::connection("Failed to acquire connection lock")
        })?;

        let tech_info = conn.send_rpc_request("system.getTechnologyInfo", json!({}))?;

        let mut modes = Vec::new();
        if tech_info.get("hasSwing").and_then(|v| v.as_bool()).unwrap_or(false) {
            modes.push("swing".to_string());
        }
        if tech_info.get("hasSwt").and_then(|v| v.as_bool()).unwrap_or(false) {
            modes.push("swt".to_string());
        }
        if tech_info.get("hasRcp").and_then(|v| v.as_bool()).unwrap_or(false) {
            modes.push("rcp".to_string());
        }

        Ok(modes)
    }
}
```

### 5. Mode-Aware Keyword Execution

```rust
impl JavaGuiLibrary {
    /// Ensure keyword is available for current mode
    fn require_mode(&self, required: &[GuiMode], keyword: &str) -> PyResult<()> {
        if !required.contains(&self.mode) {
            let available = required.iter()
                .map(|m| format!("{:?}", m).to_lowercase())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(SwingError::action_failed(
                keyword,
                format!(
                    "This keyword is only available in {} mode(s). Current mode: {:?}",
                    available,
                    self.mode
                )
            ).into());
        }
        Ok(())
    }

    // RCP-only keyword
    pub fn open_perspective(&self, perspective_id: &str) -> PyResult<()> {
        self.require_mode(&[GuiMode::Rcp], "Open Perspective")?;
        // ... implementation
    }

    // SWT and RCP keyword
    pub fn get_shells(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.require_mode(&[GuiMode::Swt, GuiMode::Rcp], "Get Shells")?;
        // ... implementation
    }

    // Universal keyword
    pub fn click(&self, locator: &str) -> PyResult<()> {
        // Works in all modes, just routes to appropriate backend
        match self.mode {
            GuiMode::Swing => self.swing_click(locator),
            GuiMode::Swt | GuiMode::Rcp => self.swt_click(locator),
            _ => Err(SwingError::connection("Mode not determined").into()),
        }
    }
}
```

### 6. Robot Framework Usage Examples

```robot
*** Settings ***
Library    JavaGuiLibrary

*** Test Cases ***
Test Auto Detection
    [Documentation]    Let library auto-detect the technology
    Connect To Application    myapp    localhost    5678
    ${mode}=    Get Mode
    Log    Detected mode: ${mode}
    Click    name:button1

Test Explicit Mode
    [Documentation]    Use explicit mode for specific technology
    Connect To Application    eclipse    localhost    5679    mode=rcp
    Open Perspective    org.eclipse.jdt.ui.JavaPerspective
    Show View    org.eclipse.ui.views.ProblemView

Test Mode Query
    [Documentation]    Query and switch modes for hybrid app
    Connect To Application    hybrid_app    localhost    5678
    ${modes}=    Get Available Modes
    Log Many    @{modes}
    # Switch to specific mode if needed
    Set Mode    swing
    Click    JButton[name='swingButton']
    Set Mode    swt
    Click Widget    name:swtButton

Test Mode Verification
    [Documentation]    Verify mode-specific keywords fail gracefully
    Connect To Application    swing_app    localhost    5678
    ${mode}=    Get Mode
    Should Be Equal    ${mode}    swing
    # This should fail with clear error
    Run Keyword And Expect Error    *only available in rcp*
    ...    Open Perspective    some.perspective.id
```

## Consequences

### Positive

1. **Reduced Configuration**: Auto-detection eliminates most user configuration
2. **Clear Errors**: Mode-aware keywords give clear errors when misused
3. **Flexibility**: Users can override auto-detection when needed
4. **Hybrid Support**: Can switch modes for hybrid applications
5. **Backwards Compatible**: Explicit library classes still work

### Negative

1. **Detection Overhead**: Auto-detection adds connection overhead
2. **Edge Cases**: Some applications may be incorrectly detected
3. **Complexity**: Mode management adds implementation complexity

### Risks

1. **Detection Failures**: Some edge-case applications may fail detection
2. **Mode Confusion**: Users may not understand mode implications
3. **Performance**: Detection queries add latency on connect

## Alternatives Considered

### Alternative 1: Always Require Explicit Mode

Force users to always specify the technology mode.

**Rejected because**:
- Increases user burden
- Most applications have obvious single technology
- Doesn't leverage connection information

### Alternative 2: Class-per-Technology Only

Keep only separate SwingLibrary/SwtLibrary/RcpLibrary classes.

**Rejected because**:
- Doesn't support hybrid applications
- Doesn't support runtime mode switching
- Forces user to know technology upfront

### Alternative 3: Detection Without Mode Switching

Detect once on connection, never allow changes.

**Rejected because**:
- Doesn't support hybrid applications
- Can't recover from wrong detection

## Implementation Plan

1. **Phase 1**: Implement TechnologyDetector in Rust (3 days)
2. **Phase 2**: Add Java agent detection methods (3 days)
3. **Phase 3**: Implement mode-aware keyword execution (3 days)
4. **Phase 4**: Add runtime mode switching (2 days)
5. **Phase 5**: Update connection workflow (2 days)
6. **Phase 6**: Add tests and documentation (3 days)

## References

- [Eclipse SWT Display Class](https://www.eclipse.org/swt/javadoc.php)
- [Eclipse PlatformUI](https://help.eclipse.org/latest/index.jsp)
- [Java Swing Frame Class](https://docs.oracle.com/javase/8/docs/api/java/awt/Frame.html)
- [ADR-001: Unified Base Class Architecture](/docs/adr/ADR-001-unified-base-class-architecture.md)
