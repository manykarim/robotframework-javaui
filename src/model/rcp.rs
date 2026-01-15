//! Eclipse RCP (Rich Client Platform) model structures
//!
//! This module provides Rust representations of Eclipse RCP workbench
//! components including Workbench, WorkbenchWindow, Perspectives, Views,
//! Editors, and Commands.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Import SwtWidget from the widget module
use super::widget::SwtWidget;

/// Unique identifier for RCP components
pub type RcpId = String;

// ============================================================================
// Layout Types
// ============================================================================

/// Relationship between views in a perspective layout
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutRelationship {
    /// View is positioned to the left of the reference
    Left,
    /// View is positioned to the right of the reference
    Right,
    /// View is positioned above the reference
    Top,
    /// View is positioned below the reference
    Bottom,
    /// View is stacked with the reference (tabbed)
    Stack,
}

impl Default for LayoutRelationship {
    fn default() -> Self {
        Self::Stack
    }
}

/// Position information for a view within the layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewPosition {
    /// Reference view ID (if relative positioning)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_to: Option<RcpId>,
    /// Relationship to reference view
    pub relationship: LayoutRelationship,
    /// Ratio of space allocation (0.0 to 1.0)
    pub ratio: f32,
}

impl Default for ViewPosition {
    fn default() -> Self {
        Self {
            relative_to: None,
            relationship: LayoutRelationship::Stack,
            ratio: 0.5,
        }
    }
}

/// Layout information for a single view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewLayoutInfo {
    /// View ID
    pub view_id: RcpId,
    /// Secondary ID for multiple instances
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_id: Option<String>,
    /// Position in the layout
    pub position: ViewPosition,
    /// Whether the view is visible in this layout
    pub visible: bool,
    /// Whether the view is closeable
    pub closeable: bool,
    /// Whether the view is moveable
    pub moveable: bool,
    /// Whether the view is standalone (not in a stack)
    pub standalone: bool,
    /// Whether to show the title
    pub show_title: bool,
}

impl Default for ViewLayoutInfo {
    fn default() -> Self {
        Self {
            view_id: String::new(),
            secondary_id: None,
            position: ViewPosition::default(),
            visible: true,
            closeable: true,
            moveable: true,
            standalone: false,
            show_title: true,
        }
    }
}

/// Complete perspective layout configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PerspectiveLayout {
    /// Editor area visibility
    pub editor_area_visible: bool,
    /// View layouts in this perspective
    pub view_layouts: Vec<ViewLayoutInfo>,
    /// Fixed views that cannot be closed
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixed_views: Vec<RcpId>,
    /// Action set IDs visible in this perspective
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub action_sets: Vec<RcpId>,
    /// Perspective bar visibility
    pub perspective_bar_visible: bool,
    /// Fast view bar visibility
    pub fast_view_bar_visible: bool,
    /// Status line visibility
    pub status_line_visible: bool,
}

// ============================================================================
// Perspective Types
// ============================================================================

/// Descriptor for a perspective (metadata)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerspectiveDescriptor {
    /// Unique perspective ID (e.g., "org.eclipse.jdt.ui.JavaPerspective")
    pub id: RcpId,
    /// Display label
    pub label: String,
    /// Description text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Icon path (plugin-relative or platform URI)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_path: Option<String>,
    /// Category ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// Whether this is a fixed perspective
    pub fixed: bool,
    /// Whether this perspective is singleton
    pub singleton: bool,
}

impl Default for PerspectiveDescriptor {
    fn default() -> Self {
        Self {
            id: String::new(),
            label: String::new(),
            description: None,
            icon_path: None,
            category_id: None,
            fixed: false,
            singleton: true,
        }
    }
}

/// Active perspective instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Perspective {
    /// Perspective descriptor
    pub descriptor: PerspectiveDescriptor,
    /// Current layout configuration
    pub layout: PerspectiveLayout,
    /// Original (default) layout for reset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_layout: Option<PerspectiveLayout>,
    /// Whether the perspective has been customized
    pub customized: bool,
}

impl Default for Perspective {
    fn default() -> Self {
        Self {
            descriptor: PerspectiveDescriptor::default(),
            layout: PerspectiveLayout::default(),
            original_layout: None,
            customized: false,
        }
    }
}

// ============================================================================
// Editor Types
// ============================================================================

/// Editor input type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInputType {
    /// File-based editor input
    File,
    /// External file (outside workspace)
    ExternalFile,
    /// URI-based input
    Uri,
    /// Storage-based input
    Storage,
    /// In-memory/untitled input
    Untitled,
    /// Custom/unknown input type
    Custom,
}

impl Default for EditorInputType {
    fn default() -> Self {
        Self::File
    }
}

/// Editor input descriptor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorInput {
    /// Display name of the input
    pub name: String,
    /// Full path or URI
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Input type
    pub input_type: EditorInputType,
    /// Whether the resource exists
    pub exists: bool,
    /// Tool tip text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_tip: Option<String>,
    /// Factory ID for persistence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factory_id: Option<String>,
    /// Content type ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type_id: Option<String>,
    /// Encoding (for text files)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}

impl Default for EditorInput {
    fn default() -> Self {
        Self {
            name: String::new(),
            path: None,
            input_type: EditorInputType::File,
            exists: false,
            tool_tip: None,
            factory_id: None,
            content_type_id: None,
            encoding: None,
        }
    }
}

/// Editor part representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Editor {
    /// Editor ID (editor type, e.g., "org.eclipse.ui.DefaultTextEditor")
    pub id: RcpId,
    /// Display title
    pub title: String,
    /// Title tooltip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_tool_tip: Option<String>,
    /// Title image path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_image: Option<String>,
    /// Whether the editor has unsaved changes
    pub dirty: bool,
    /// Whether this editor is currently active
    pub active: bool,
    /// Whether the editor is pinned
    pub pinned: bool,
    /// Editor input
    pub input: EditorInput,
    /// Reference to the underlying SWT widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget: Option<SwtWidget>,
    /// Editor-specific properties
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
    /// Site ID for this editor instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            title_tool_tip: None,
            title_image: None,
            dirty: false,
            active: false,
            pinned: false,
            input: EditorInput::default(),
            widget: None,
            properties: HashMap::new(),
            site_id: None,
        }
    }
}

// ============================================================================
// View Types
// ============================================================================

/// View part representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct View {
    /// View ID (e.g., "org.eclipse.ui.views.ProblemView")
    pub id: RcpId,
    /// Secondary ID for multiple instances
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_id: Option<String>,
    /// Display title
    pub title: String,
    /// Title tooltip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_tool_tip: Option<String>,
    /// Title image path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_image: Option<String>,
    /// Whether the view has unsaved state
    pub dirty: bool,
    /// Whether the view is pinned
    pub pinned: bool,
    /// Whether the view is visible
    pub visible: bool,
    /// Whether this view is currently active (has focus)
    pub active: bool,
    /// Reference to the underlying SWT widget
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget: Option<SwtWidget>,
    /// View-specific properties
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<String, String>,
    /// Content description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_description: Option<String>,
    /// Part name (may differ from title)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_name: Option<String>,
    /// Site ID for this view instance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_id: Option<String>,
}

impl Default for View {
    fn default() -> Self {
        Self {
            id: String::new(),
            secondary_id: None,
            title: String::new(),
            title_tool_tip: None,
            title_image: None,
            dirty: false,
            pinned: false,
            visible: true,
            active: false,
            widget: None,
            properties: HashMap::new(),
            content_description: None,
            part_name: None,
            site_id: None,
        }
    }
}

impl View {
    /// Create a new view with the given ID and title
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            ..Default::default()
        }
    }

    /// Get the full compound ID (id:secondaryId)
    pub fn compound_id(&self) -> String {
        match &self.secondary_id {
            Some(secondary) => format!("{}:{}", self.id, secondary),
            None => self.id.clone(),
        }
    }
}

// ============================================================================
// Command Types
// ============================================================================

/// Eclipse command representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EclipseCommand {
    /// Command ID (e.g., "org.eclipse.ui.file.save")
    pub id: RcpId,
    /// Command name
    pub name: String,
    /// Description text
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Category ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Category name (resolved)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_name: Option<String>,
    /// Whether the command is defined
    pub defined: bool,
    /// Whether the command is currently enabled
    pub enabled: bool,
    /// Whether the command has a handler
    pub handled: bool,
    /// Default key binding
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_key_binding: Option<String>,
    /// All key bindings
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub key_bindings: Vec<String>,
    /// Command parameters
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub parameters: HashMap<String, CommandParameter>,
}

impl Default for EclipseCommand {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            description: None,
            category: None,
            category_name: None,
            defined: false,
            enabled: false,
            handled: false,
            default_key_binding: None,
            key_bindings: Vec::new(),
            parameters: HashMap::new(),
        }
    }
}

impl EclipseCommand {
    /// Create a new command with basic info
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            defined: true,
            ..Default::default()
        }
    }

    /// Check if the command can be executed
    pub fn can_execute(&self) -> bool {
        self.defined && self.enabled && self.handled
    }
}

/// Command parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandParameter {
    /// Parameter ID
    pub id: String,
    /// Parameter name
    pub name: String,
    /// Whether the parameter is optional
    pub optional: bool,
    /// Parameter type ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_id: Option<String>,
    /// Default value
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
}

// ============================================================================
// Workbench Window Types
// ============================================================================

/// Coolbar item representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoolbarItem {
    /// Item ID
    pub id: RcpId,
    /// Item type (toolbar, separator, etc.)
    pub item_type: String,
    /// Display label
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Whether the item is visible
    pub visible: bool,
}

/// Menu item representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    /// Item ID
    pub id: RcpId,
    /// Menu label (with accelerator)
    pub label: String,
    /// Associated command ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_id: Option<RcpId>,
    /// Whether the item is enabled
    pub enabled: bool,
    /// Whether the item is visible
    pub visible: bool,
    /// Whether the item is checked (for toggle items)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    /// Child menu items (for submenus)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<MenuItem>,
}

/// Menu representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Menu {
    /// Menu ID
    pub id: RcpId,
    /// Menu label
    pub label: String,
    /// Menu items
    pub items: Vec<MenuItem>,
    /// Whether the menu is visible
    pub visible: bool,
}

/// Workbench window representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbenchWindow {
    /// Window shell reference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shell: Option<SwtWidget>,
    /// Available perspectives
    pub perspectives: Vec<Perspective>,
    /// Active perspective index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_perspective_index: Option<usize>,
    /// Open views
    pub views: Vec<View>,
    /// Open editors
    pub editors: Vec<Editor>,
    /// Active editor index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_editor_index: Option<usize>,
    /// Coolbar configuration
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub coolbar: Vec<CoolbarItem>,
    /// Menu bar
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub menubar: Vec<Menu>,
    /// Window title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Window bounds
    pub bounds: WindowBounds,
    /// Whether the window is maximized
    pub maximized: bool,
    /// Whether the window is minimized
    pub minimized: bool,
    /// Whether the coolbar is visible
    pub coolbar_visible: bool,
    /// Whether the perspective bar is visible
    pub perspective_bar_visible: bool,
    /// Whether the status line is visible
    pub status_line_visible: bool,
}

impl Default for WorkbenchWindow {
    fn default() -> Self {
        Self {
            shell: None,
            perspectives: Vec::new(),
            active_perspective_index: None,
            views: Vec::new(),
            editors: Vec::new(),
            active_editor_index: None,
            coolbar: Vec::new(),
            menubar: Vec::new(),
            title: None,
            bounds: WindowBounds::default(),
            maximized: false,
            minimized: false,
            coolbar_visible: true,
            perspective_bar_visible: true,
            status_line_visible: true,
        }
    }
}

impl WorkbenchWindow {
    /// Get the active perspective
    pub fn active_perspective(&self) -> Option<&Perspective> {
        self.active_perspective_index
            .and_then(|idx| self.perspectives.get(idx))
    }

    /// Get the active perspective mutably
    pub fn active_perspective_mut(&mut self) -> Option<&mut Perspective> {
        self.active_perspective_index
            .and_then(|idx| self.perspectives.get_mut(idx))
    }

    /// Get the active editor
    pub fn active_editor(&self) -> Option<&Editor> {
        self.active_editor_index
            .and_then(|idx| self.editors.get(idx))
    }

    /// Get a view by ID
    pub fn find_view(&self, id: &str) -> Option<&View> {
        self.views.iter().find(|v| v.id == id)
    }

    /// Get a view by compound ID (id:secondaryId)
    pub fn find_view_by_compound_id(&self, compound_id: &str) -> Option<&View> {
        self.views.iter().find(|v| v.compound_id() == compound_id)
    }

    /// Get all dirty editors
    pub fn dirty_editors(&self) -> Vec<&Editor> {
        self.editors.iter().filter(|e| e.dirty).collect()
    }

    /// Check if any editor has unsaved changes
    pub fn has_dirty_editors(&self) -> bool {
        self.editors.iter().any(|e| e.dirty)
    }
}

/// Window bounds
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl WindowBounds {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }
}

// ============================================================================
// Workbench Types
// ============================================================================

/// Top-level workbench representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workbench {
    /// Open workbench windows
    pub windows: Vec<WorkbenchWindow>,
    /// Active window index
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_window_index: Option<usize>,
    /// Registered perspective descriptors
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub perspective_registry: Vec<PerspectiveDescriptor>,
    /// Available commands
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub commands: Vec<EclipseCommand>,
    /// Workbench state
    pub state: WorkbenchState,
    /// Product name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    /// Product ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Workspace location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_location: Option<String>,
}

impl Default for Workbench {
    fn default() -> Self {
        Self {
            windows: Vec::new(),
            active_window_index: None,
            perspective_registry: Vec::new(),
            commands: Vec::new(),
            state: WorkbenchState::Running,
            product_name: None,
            product_id: None,
            workspace_location: None,
        }
    }
}

impl Workbench {
    /// Create a new empty workbench
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the active workbench window
    pub fn active_window(&self) -> Option<&WorkbenchWindow> {
        self.active_window_index
            .and_then(|idx| self.windows.get(idx))
    }

    /// Get the active workbench window mutably
    pub fn active_window_mut(&mut self) -> Option<&mut WorkbenchWindow> {
        self.active_window_index
            .and_then(|idx| self.windows.get_mut(idx))
    }

    /// Find a command by ID
    pub fn find_command(&self, id: &str) -> Option<&EclipseCommand> {
        self.commands.iter().find(|c| c.id == id)
    }

    /// Find a perspective descriptor by ID
    pub fn find_perspective_descriptor(&self, id: &str) -> Option<&PerspectiveDescriptor> {
        self.perspective_registry.iter().find(|p| p.id == id)
    }

    /// Get all active views across all windows
    pub fn all_views(&self) -> Vec<&View> {
        self.windows.iter().flat_map(|w| w.views.iter()).collect()
    }

    /// Get all open editors across all windows
    pub fn all_editors(&self) -> Vec<&Editor> {
        self.windows.iter().flat_map(|w| w.editors.iter()).collect()
    }

    /// Check if any editor has unsaved changes
    pub fn has_dirty_editors(&self) -> bool {
        self.windows.iter().any(|w| w.has_dirty_editors())
    }
}

/// Workbench state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkbenchState {
    /// Workbench is starting up
    Starting,
    /// Workbench is running normally
    Running,
    /// Workbench is shutting down
    Closing,
    /// Workbench has closed
    Closed,
}

impl Default for WorkbenchState {
    fn default() -> Self {
        Self::Running
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_view_compound_id() {
        let view = View {
            id: "org.eclipse.ui.views.ProblemView".to_string(),
            secondary_id: Some("secondary1".to_string()),
            ..Default::default()
        };
        assert_eq!(
            view.compound_id(),
            "org.eclipse.ui.views.ProblemView:secondary1"
        );

        let view_no_secondary = View {
            id: "org.eclipse.ui.views.ProblemView".to_string(),
            secondary_id: None,
            ..Default::default()
        };
        assert_eq!(
            view_no_secondary.compound_id(),
            "org.eclipse.ui.views.ProblemView"
        );
    }

    #[test]
    fn test_eclipse_command_can_execute() {
        let mut cmd = EclipseCommand::new("test.command", "Test Command");
        assert!(!cmd.can_execute()); // not enabled or handled

        cmd.enabled = true;
        assert!(!cmd.can_execute()); // not handled

        cmd.handled = true;
        assert!(cmd.can_execute()); // now can execute
    }

    #[test]
    fn test_workbench_window_find_view() {
        let mut window = WorkbenchWindow::default();
        window.views.push(View::new("view1", "View 1"));
        window.views.push(View::new("view2", "View 2"));

        assert!(window.find_view("view1").is_some());
        assert!(window.find_view("view3").is_none());
    }

    #[test]
    fn test_workbench_dirty_editors() {
        let mut workbench = Workbench::new();
        let mut window = WorkbenchWindow::default();

        let mut editor1 = Editor::default();
        editor1.dirty = true;
        window.editors.push(editor1);

        let editor2 = Editor::default();
        window.editors.push(editor2);

        workbench.windows.push(window);
        workbench.active_window_index = Some(0);

        assert!(workbench.has_dirty_editors());
        assert_eq!(workbench.active_window().unwrap().dirty_editors().len(), 1);
    }

    #[test]
    fn test_perspective_layout_default() {
        let layout = PerspectiveLayout::default();
        assert!(!layout.editor_area_visible);
        assert!(layout.view_layouts.is_empty());
    }

    #[test]
    fn test_editor_input_types() {
        let input = EditorInput {
            name: "test.txt".to_string(),
            path: Some("/workspace/test.txt".to_string()),
            input_type: EditorInputType::File,
            exists: true,
            ..Default::default()
        };
        assert_eq!(input.input_type, EditorInputType::File);
        assert!(input.exists);
    }

    #[test]
    fn test_layout_relationship_serialization() {
        let rel = LayoutRelationship::Left;
        let json = serde_json::to_string(&rel).unwrap();
        assert_eq!(json, "\"left\"");

        let parsed: LayoutRelationship = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, LayoutRelationship::Left);
    }

    #[test]
    fn test_workbench_state_serialization() {
        let state = WorkbenchState::Running;
        let json = serde_json::to_string(&state).unwrap();
        assert_eq!(json, "\"running\"");
    }
}
