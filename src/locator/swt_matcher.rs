//! SWT-specific locator matcher for Eclipse SWT/RCP widgets
//!
//! This module extends the locator system to support SWT-specific widget types,
//! views, editors, perspectives, and menus common in Eclipse RCP applications.

use crate::model::widget::{SwtWidget, SwtWidgetType};
use std::fmt;

// =============================================================================
// Error Types
// =============================================================================

/// Error type for SWT locator operations
#[derive(Debug, Clone)]
pub struct LocatorError {
    /// Error message
    pub message: String,
    /// Position in input where error occurred
    pub position: Option<usize>,
    /// Kind of error
    pub kind: LocatorErrorKind,
}

impl LocatorError {
    /// Create a new locator error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            position: None,
            kind: LocatorErrorKind::InvalidSyntax,
        }
    }

    /// Create an error with position information
    pub fn with_position(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position: Some(position),
            kind: LocatorErrorKind::InvalidSyntax,
        }
    }

    /// Create an error with kind
    pub fn with_kind(message: impl Into<String>, kind: LocatorErrorKind) -> Self {
        Self {
            message: message.into(),
            position: None,
            kind,
        }
    }
}

impl fmt::Display for LocatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(pos) = self.position {
            write!(f, "{} at position {}", self.message, pos)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for LocatorError {}

/// Kind of locator error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocatorErrorKind {
    /// Invalid syntax in locator string
    InvalidSyntax,
    /// Unknown locator type
    UnknownType,
    /// Invalid attribute name
    InvalidAttribute,
    /// Invalid pseudo-class
    InvalidPseudoClass,
    /// Empty locator
    EmptyLocator,
    /// Unsupported operation
    Unsupported,
}

// =============================================================================
// SWT-Specific Locator Types
// =============================================================================

/// SWT-specific locator types for Eclipse RCP applications
#[derive(Debug, Clone)]
pub enum SwtLocator {
    /// Basic widget selector
    Widget(WidgetSelector),
    /// Eclipse view selector
    View(ViewSelector),
    /// Eclipse editor selector
    Editor(EditorSelector),
    /// Eclipse perspective selector
    Perspective(PerspectiveSelector),
    /// Menu selector (menu bar, popup, context)
    Menu(MenuSelector),
    /// Compound locator (AND combination)
    Compound(Box<SwtLocator>, Box<SwtLocator>),
    /// Union locator (OR combination)
    Union(Vec<SwtLocator>),
}

impl SwtLocator {
    /// Create a widget locator
    pub fn widget() -> WidgetSelector {
        WidgetSelector::default()
    }

    /// Create a view locator
    pub fn view() -> ViewSelector {
        ViewSelector::default()
    }

    /// Create an editor locator
    pub fn editor() -> EditorSelector {
        EditorSelector::default()
    }

    /// Create a perspective locator
    pub fn perspective() -> PerspectiveSelector {
        PerspectiveSelector::default()
    }

    /// Create a menu locator
    pub fn menu() -> MenuSelector {
        MenuSelector::default()
    }

    /// Combine two locators with AND logic
    pub fn and(self, other: SwtLocator) -> SwtLocator {
        SwtLocator::Compound(Box::new(self), Box::new(other))
    }

    /// Combine multiple locators with OR logic
    pub fn or(self, other: SwtLocator) -> SwtLocator {
        match self {
            SwtLocator::Union(mut vec) => {
                vec.push(other);
                SwtLocator::Union(vec)
            }
            _ => SwtLocator::Union(vec![self, other]),
        }
    }
}

// =============================================================================
// Widget Selector
// =============================================================================

/// Selector for SWT widgets with various matching criteria
#[derive(Debug, Clone, Default)]
pub struct WidgetSelector {
    /// Widget type to match
    pub widget_type: Option<SwtWidgetType>,
    /// Widget ID from getData("id")
    pub id: Option<String>,
    /// Widget text (getText())
    pub text: Option<String>,
    /// Widget tooltip
    pub tooltip: Option<String>,
    /// Style string to match
    pub style: Option<String>,
    /// Data key to check
    pub data_key: Option<String>,
    /// Data value to match for the data key
    pub data_value: Option<String>,
    /// Index among matching widgets (0-based)
    pub index: Option<usize>,
    /// Pseudo-class selector
    pub pseudo_class: Option<SwtPseudoClass>,
    /// Class name pattern (partial match)
    pub class_name: Option<String>,
    /// Accessible name
    pub accessible_name: Option<String>,
    /// Accessible role
    pub accessible_role: Option<String>,
    /// Text matching mode
    pub text_match_mode: TextMatchMode,
}

impl WidgetSelector {
    /// Create a new widget selector
    pub fn new() -> Self {
        Self::default()
    }

    /// Match by widget type
    pub fn with_type(mut self, widget_type: SwtWidgetType) -> Self {
        self.widget_type = Some(widget_type);
        self
    }

    /// Match by ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Match by text
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Match by text containing substring
    pub fn with_text_containing(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self.text_match_mode = TextMatchMode::Contains;
        self
    }

    /// Match by text with regex
    pub fn with_text_matching(mut self, pattern: impl Into<String>) -> Self {
        self.text = Some(pattern.into());
        self.text_match_mode = TextMatchMode::Regex;
        self
    }

    /// Match by tooltip
    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Match by style
    pub fn with_style(mut self, style: impl Into<String>) -> Self {
        self.style = Some(style.into());
        self
    }

    /// Match by data key existence
    pub fn with_data_key(mut self, key: impl Into<String>) -> Self {
        self.data_key = Some(key.into());
        self
    }

    /// Match by data key and value
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data_key = Some(key.into());
        self.data_value = Some(value.into());
        self
    }

    /// Match by index among results
    pub fn at_index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Match by pseudo-class
    pub fn with_pseudo(mut self, pseudo: SwtPseudoClass) -> Self {
        self.pseudo_class = Some(pseudo);
        self
    }

    /// Match by class name pattern
    pub fn with_class_name(mut self, class_name: impl Into<String>) -> Self {
        self.class_name = Some(class_name.into());
        self
    }

    /// Match by accessible name
    pub fn with_accessible_name(mut self, name: impl Into<String>) -> Self {
        self.accessible_name = Some(name.into());
        self
    }

    /// Build into an SwtLocator
    pub fn build(self) -> SwtLocator {
        SwtLocator::Widget(self)
    }
}

/// Text matching mode for string comparisons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextMatchMode {
    /// Exact match
    #[default]
    Exact,
    /// Contains substring
    Contains,
    /// Starts with prefix
    StartsWith,
    /// Ends with suffix
    EndsWith,
    /// Regular expression match
    Regex,
    /// Case-insensitive exact match
    CaseInsensitive,
}

// =============================================================================
// SWT Pseudo-Classes
// =============================================================================

/// SWT-specific pseudo-class selectors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwtPseudoClass {
    /// First child among siblings
    FirstChild,
    /// Last child among siblings
    LastChild,
    /// Nth child (1-based index)
    NthChild(usize),
    /// Nth child from end (1-based)
    NthLastChild(usize),
    /// Only child
    OnlyChild,
    /// Widget is enabled
    Enabled,
    /// Widget is disabled
    Disabled,
    /// Widget is visible
    Visible,
    /// Widget is hidden
    Hidden,
    /// Widget is selected
    Selected,
    /// Widget has focus
    Focused,
    /// Widget is checked (checkbox, toggle)
    Checked,
    /// Widget is not checked
    Unchecked,
    /// Widget has been disposed
    Disposed,
    /// Widget is active (for shells)
    Active,
    /// Widget is expanded (tree items, sections)
    Expanded,
    /// Widget is collapsed
    Collapsed,
    /// Widget is editable (text widgets)
    Editable,
    /// Widget is read-only
    ReadOnly,
    /// Widget is maximized (shells)
    Maximized,
    /// Widget is minimized (shells)
    Minimized,
    /// Widget has children
    HasChildren,
    /// Widget is empty (no children)
    Empty,
    /// Custom pseudo-class
    Custom(String),
}

impl SwtPseudoClass {
    /// Parse a pseudo-class from string
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "first-child" | "firstchild" => Some(Self::FirstChild),
            "last-child" | "lastchild" => Some(Self::LastChild),
            "only-child" | "onlychild" => Some(Self::OnlyChild),
            "enabled" => Some(Self::Enabled),
            "disabled" => Some(Self::Disabled),
            "visible" => Some(Self::Visible),
            "hidden" => Some(Self::Hidden),
            "selected" => Some(Self::Selected),
            "focused" | "focus" => Some(Self::Focused),
            "checked" => Some(Self::Checked),
            "unchecked" => Some(Self::Unchecked),
            "disposed" => Some(Self::Disposed),
            "active" => Some(Self::Active),
            "expanded" => Some(Self::Expanded),
            "collapsed" => Some(Self::Collapsed),
            "editable" => Some(Self::Editable),
            "readonly" | "read-only" => Some(Self::ReadOnly),
            "maximized" => Some(Self::Maximized),
            "minimized" => Some(Self::Minimized),
            "has-children" | "haschildren" => Some(Self::HasChildren),
            "empty" => Some(Self::Empty),
            _ => None,
        }
    }

    /// Parse nth-child expression
    pub fn parse_nth(expr: &str) -> Option<Self> {
        let trimmed = expr.trim().to_lowercase();

        if trimmed == "odd" {
            return Some(Self::NthChild(1)); // Will be handled specially
        }
        if trimmed == "even" {
            return Some(Self::NthChild(2)); // Will be handled specially
        }

        if let Ok(n) = trimmed.parse::<usize>() {
            return Some(Self::NthChild(n));
        }

        None
    }
}

// =============================================================================
// Eclipse RCP Selectors
// =============================================================================

/// Selector for Eclipse views
#[derive(Debug, Clone, Default)]
pub struct ViewSelector {
    /// View ID (e.g., "org.eclipse.ui.views.ProblemView")
    pub id: Option<String>,
    /// View title
    pub title: Option<String>,
    /// View secondary ID
    pub secondary_id: Option<String>,
    /// View is active
    pub active: Option<bool>,
    /// View is visible
    pub visible: Option<bool>,
    /// View is dirty (has unsaved changes)
    pub dirty: Option<bool>,
}

impl ViewSelector {
    /// Create a new view selector
    pub fn new() -> Self {
        Self::default()
    }

    /// Match by view ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Match by title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Match by secondary ID
    pub fn with_secondary_id(mut self, id: impl Into<String>) -> Self {
        self.secondary_id = Some(id.into());
        self
    }

    /// Match active views only
    pub fn active(mut self) -> Self {
        self.active = Some(true);
        self
    }

    /// Match visible views only
    pub fn visible(mut self) -> Self {
        self.visible = Some(true);
        self
    }

    /// Match dirty views only
    pub fn dirty(mut self) -> Self {
        self.dirty = Some(true);
        self
    }

    /// Build into an SwtLocator
    pub fn build(self) -> SwtLocator {
        SwtLocator::View(self)
    }
}

/// Selector for Eclipse editors
#[derive(Debug, Clone, Default)]
pub struct EditorSelector {
    /// Editor ID
    pub id: Option<String>,
    /// Editor title
    pub title: Option<String>,
    /// Editor input name (file name)
    pub input_name: Option<String>,
    /// Editor input path
    pub input_path: Option<String>,
    /// Content type
    pub content_type: Option<String>,
    /// Editor is active
    pub active: Option<bool>,
    /// Editor is dirty
    pub dirty: Option<bool>,
    /// Editor index in tab folder
    pub index: Option<usize>,
}

impl EditorSelector {
    /// Create a new editor selector
    pub fn new() -> Self {
        Self::default()
    }

    /// Match by editor ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Match by title
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Match by input name (file name)
    pub fn with_input_name(mut self, name: impl Into<String>) -> Self {
        self.input_name = Some(name.into());
        self
    }

    /// Match by input path
    pub fn with_input_path(mut self, path: impl Into<String>) -> Self {
        self.input_path = Some(path.into());
        self
    }

    /// Match by content type
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> Self {
        self.content_type = Some(content_type.into());
        self
    }

    /// Match active editors only
    pub fn active(mut self) -> Self {
        self.active = Some(true);
        self
    }

    /// Match dirty editors only
    pub fn dirty(mut self) -> Self {
        self.dirty = Some(true);
        self
    }

    /// Match by index
    pub fn at_index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Build into an SwtLocator
    pub fn build(self) -> SwtLocator {
        SwtLocator::Editor(self)
    }
}

/// Selector for Eclipse perspectives
#[derive(Debug, Clone, Default)]
pub struct PerspectiveSelector {
    /// Perspective ID
    pub id: Option<String>,
    /// Perspective label
    pub label: Option<String>,
    /// Perspective is active
    pub active: Option<bool>,
}

impl PerspectiveSelector {
    /// Create a new perspective selector
    pub fn new() -> Self {
        Self::default()
    }

    /// Match by perspective ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Match by label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Match active perspective only
    pub fn active(mut self) -> Self {
        self.active = Some(true);
        self
    }

    /// Build into an SwtLocator
    pub fn build(self) -> SwtLocator {
        SwtLocator::Perspective(self)
    }
}

/// Selector for menus (menu bar, popup, context menus)
#[derive(Debug, Clone, Default)]
pub struct MenuSelector {
    /// Menu path (e.g., "File/New/Project")
    pub path: Option<String>,
    /// Menu item text
    pub text: Option<String>,
    /// Menu item ID
    pub id: Option<String>,
    /// Menu type
    pub menu_type: Option<MenuType>,
    /// Menu item index
    pub index: Option<usize>,
    /// Menu item mnemonic
    pub mnemonic: Option<char>,
    /// Menu item accelerator
    pub accelerator: Option<String>,
    /// Menu item is enabled
    pub enabled: Option<bool>,
    /// Menu item is checked
    pub checked: Option<bool>,
}

impl MenuSelector {
    /// Create a new menu selector
    pub fn new() -> Self {
        Self::default()
    }

    /// Match by menu path
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    /// Match by text
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Match by ID
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Match by menu type
    pub fn with_type(mut self, menu_type: MenuType) -> Self {
        self.menu_type = Some(menu_type);
        self
    }

    /// Match by index
    pub fn at_index(mut self, index: usize) -> Self {
        self.index = Some(index);
        self
    }

    /// Match by mnemonic
    pub fn with_mnemonic(mut self, mnemonic: char) -> Self {
        self.mnemonic = Some(mnemonic);
        self
    }

    /// Match enabled items only
    pub fn enabled(mut self) -> Self {
        self.enabled = Some(true);
        self
    }

    /// Match checked items only
    pub fn checked(mut self) -> Self {
        self.checked = Some(true);
        self
    }

    /// Build into an SwtLocator
    pub fn build(self) -> SwtLocator {
        SwtLocator::Menu(self)
    }
}

/// Type of menu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuType {
    /// Menu bar
    Bar,
    /// Popup/context menu
    Popup,
    /// Dropdown menu
    Dropdown,
    /// Cascade (submenu)
    Cascade,
}

// =============================================================================
// SWT Matcher Implementation
// =============================================================================

/// SWT widget matcher for finding widgets by selector
pub struct SwtMatcher;

impl SwtMatcher {
    /// Check if a widget matches the given selector
    pub fn matches(widget: &SwtWidget, selector: &WidgetSelector) -> bool {
        // Check widget type
        if let Some(ref expected_type) = selector.widget_type {
            if widget.widget_type != *expected_type {
                return false;
            }
        }

        // Check ID
        if let Some(ref expected_id) = selector.id {
            let widget_id = widget.data.get("id")
                .and_then(|v| v.as_str());

            if widget_id != Some(expected_id.as_str()) {
                return false;
            }
        }

        // Check text
        if let Some(ref expected_text) = selector.text {
            if let Some(ref widget_text) = widget.text {
                if !Self::matches_text(widget_text, expected_text, selector.text_match_mode) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Check tooltip
        if let Some(ref expected_tooltip) = selector.tooltip {
            match &widget.tooltip {
                Some(tooltip) if tooltip == expected_tooltip => {}
                _ => return false,
            }
        }

        // Check class name
        if let Some(ref expected_class) = selector.class_name {
            if !widget.class_name.contains(expected_class) {
                return false;
            }
        }

        // Check accessible name
        if let Some(ref expected_name) = selector.accessible_name {
            match &widget.accessible_name {
                Some(name) if name == expected_name => {}
                _ => return false,
            }
        }

        // Check accessible role
        if let Some(ref expected_role) = selector.accessible_role {
            match &widget.accessible_role {
                Some(role) if role == expected_role => {}
                _ => return false,
            }
        }

        // Check data key/value
        if let Some(ref key) = selector.data_key {
            if let Some(ref expected_value) = selector.data_value {
                let actual_value = widget.data.get(key)
                    .and_then(|v| v.as_str());

                if actual_value != Some(expected_value.as_str()) {
                    return false;
                }
            } else {
                // Just check if key exists
                if !widget.data.contains_key(key) {
                    return false;
                }
            }
        }

        // Check style
        if let Some(ref expected_style) = selector.style {
            if !widget.style.style_names.iter()
                .any(|s| s.eq_ignore_ascii_case(expected_style)) {
                return false;
            }
        }

        // Check pseudo-class
        if let Some(ref pseudo) = selector.pseudo_class {
            if !Self::matches_pseudo(widget, pseudo) {
                return false;
            }
        }

        true
    }

    /// Check if text matches according to mode
    fn matches_text(actual: &str, expected: &str, mode: TextMatchMode) -> bool {
        match mode {
            TextMatchMode::Exact => actual == expected,
            TextMatchMode::Contains => actual.contains(expected),
            TextMatchMode::StartsWith => actual.starts_with(expected),
            TextMatchMode::EndsWith => actual.ends_with(expected),
            TextMatchMode::CaseInsensitive => {
                actual.to_lowercase() == expected.to_lowercase()
            }
            TextMatchMode::Regex => {
                regex::Regex::new(expected)
                    .map(|re| re.is_match(actual))
                    .unwrap_or(false)
            }
        }
    }

    /// Check if widget matches a pseudo-class
    fn matches_pseudo(widget: &SwtWidget, pseudo: &SwtPseudoClass) -> bool {
        match pseudo {
            SwtPseudoClass::Enabled => widget.state.enabled,
            SwtPseudoClass::Disabled => !widget.state.enabled,
            SwtPseudoClass::Visible => widget.state.visible,
            SwtPseudoClass::Hidden => !widget.state.visible,
            SwtPseudoClass::Selected => widget.state.selection.unwrap_or(false),
            SwtPseudoClass::Focused => widget.state.focused,
            SwtPseudoClass::Checked => widget.state.checked.unwrap_or(false),
            SwtPseudoClass::Unchecked => !widget.state.checked.unwrap_or(true),
            SwtPseudoClass::Disposed => widget.state.disposed,
            SwtPseudoClass::Active => widget.state.active.unwrap_or(false),
            SwtPseudoClass::Expanded => widget.state.expanded.unwrap_or(false),
            SwtPseudoClass::Collapsed => !widget.state.expanded.unwrap_or(true),
            SwtPseudoClass::Editable => widget.state.editable.unwrap_or(false),
            SwtPseudoClass::ReadOnly => !widget.state.editable.unwrap_or(true),
            SwtPseudoClass::Maximized => widget.state.maximized.unwrap_or(false),
            SwtPseudoClass::Minimized => widget.state.minimized.unwrap_or(false),
            SwtPseudoClass::HasChildren => {
                widget.children.as_ref()
                    .map(|c| !c.is_empty())
                    .unwrap_or(false)
            }
            SwtPseudoClass::Empty => {
                widget.children.as_ref()
                    .map(|c| c.is_empty())
                    .unwrap_or(true)
            }
            // Position-based pseudo-classes need sibling context
            SwtPseudoClass::FirstChild => widget.sibling_index == 0,
            SwtPseudoClass::NthChild(n) => widget.sibling_index as usize == *n - 1,
            SwtPseudoClass::OnlyChild => widget.sibling_index == 0, // Simplified
            // These require more context
            SwtPseudoClass::LastChild |
            SwtPseudoClass::NthLastChild(_) => true, // Need parent context
            SwtPseudoClass::Custom(_) => true, // Custom pseudo-classes always match
        }
    }

    /// Find all widgets matching the selector
    pub fn find_all<'a>(
        widgets: &'a [SwtWidget],
        selector: &WidgetSelector
    ) -> Vec<&'a SwtWidget> {
        let mut results: Vec<&SwtWidget> = widgets
            .iter()
            .filter(|w| Self::matches(w, selector))
            .collect();

        // Also search recursively in children
        for widget in widgets {
            if let Some(ref children) = widget.children {
                results.extend(Self::find_all(children, selector));
            }
        }

        // Apply index filter if specified
        if let Some(index) = selector.index {
            if index < results.len() {
                return vec![results[index]];
            } else {
                return vec![];
            }
        }

        results
    }

    /// Find the first widget matching the selector
    pub fn find_first<'a>(
        widgets: &'a [SwtWidget],
        selector: &WidgetSelector
    ) -> Option<&'a SwtWidget> {
        // Check top-level widgets first
        for widget in widgets {
            if Self::matches(widget, selector) {
                return Some(widget);
            }
        }

        // Then check children recursively
        for widget in widgets {
            if let Some(ref children) = widget.children {
                if let Some(found) = Self::find_first(children, selector) {
                    return Some(found);
                }
            }
        }

        None
    }

    /// Find all widgets matching an SwtLocator
    pub fn find_all_by_locator<'a>(
        widgets: &'a [SwtWidget],
        locator: &SwtLocator
    ) -> Vec<&'a SwtWidget> {
        match locator {
            SwtLocator::Widget(selector) => Self::find_all(widgets, selector),
            SwtLocator::Compound(left, right) => {
                let left_matches = Self::find_all_by_locator(widgets, left);
                left_matches.into_iter()
                    .filter(|w| {
                        Self::find_all_by_locator(&[(*w).clone()], right)
                            .len() > 0
                    })
                    .collect()
            }
            SwtLocator::Union(locators) => {
                let mut results = Vec::new();
                for loc in locators {
                    results.extend(Self::find_all_by_locator(widgets, loc));
                }
                // Remove duplicates by handle
                let mut seen = std::collections::HashSet::new();
                results.retain(|w| seen.insert(w.id.handle));
                results
            }
            // View, Editor, Perspective, Menu need Eclipse-specific handling
            _ => vec![],
        }
    }

    /// Count widgets matching the selector
    pub fn count<'a>(widgets: &'a [SwtWidget], selector: &WidgetSelector) -> usize {
        Self::find_all(widgets, selector).len()
    }

    /// Check if any widget matches the selector
    pub fn exists<'a>(widgets: &'a [SwtWidget], selector: &WidgetSelector) -> bool {
        Self::find_first(widgets, selector).is_some()
    }
}

// =============================================================================
// Parser for SWT-Specific Locator Syntax
// =============================================================================

/// Parse an SWT-specific locator string
///
/// Supports formats:
/// - `swt:Button` - Widget type
/// - `swt:Button#myId` - Widget type with ID
/// - `swt:Button[text='OK']` - Widget with attribute
/// - `swt:Button:enabled` - Widget with pseudo-class
/// - `view:Problems` - Eclipse view by title
/// - `editor:MyFile.java` - Eclipse editor by name
/// - `perspective:Java` - Eclipse perspective
/// - `menu:File/New/Project` - Menu path
/// - `class:ClassName` - Match by Java class name (partial or full)
/// - `class:org.eclipse.swt.widgets.Button` - Match by fully qualified class name
/// - `index:N` - Get the Nth widget (0-based index)
/// - `name:Text` - Match by widget name/text
/// - `id:widgetId` - Match by widget ID
/// - `tooltip:Text` - Match by tooltip text
///
/// Note: Locator prefixes are case-insensitive (e.g., NAME:, Name:, name: all work)
pub fn parse_swt_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(LocatorError::with_kind(
            "Empty locator",
            LocatorErrorKind::EmptyLocator
        ));
    }

    // Check for prefix-based locators (case-insensitive prefix matching)
    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "swt:") {
        return parse_swt_widget_locator(rest);
    }

    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "view:") {
        return parse_view_locator(rest);
    }

    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "editor:") {
        return parse_editor_locator(rest);
    }

    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "perspective:") {
        return parse_perspective_locator(rest);
    }

    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "menu:") {
        return parse_menu_locator(rest);
    }

    // New locator types
    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "class:") {
        return parse_class_locator(rest);
    }

    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "index:") {
        return parse_index_locator(rest);
    }

    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "name:") {
        return parse_name_locator(rest);
    }

    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "text:") {
        return parse_name_locator(rest); // text: is alias for name:
    }

    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "id:") {
        return parse_id_locator(rest);
    }

    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "tooltip:") {
        return parse_tooltip_locator(rest);
    }

    if let Some(rest) = strip_prefix_case_insensitive(trimmed, "accessible:") {
        return parse_accessible_locator(rest);
    }

    // Default to widget locator
    parse_swt_widget_locator(trimmed)
}

/// Strip a prefix from a string in a case-insensitive manner
/// Returns the remaining part of the string if the prefix matches
fn strip_prefix_case_insensitive<'a>(input: &'a str, prefix: &str) -> Option<&'a str> {
    let input_lower = input.to_lowercase();
    let prefix_lower = prefix.to_lowercase();

    if input_lower.starts_with(&prefix_lower) {
        Some(&input[prefix.len()..])
    } else {
        None
    }
}

/// Parse a class locator (class:ClassName or class:org.eclipse.swt.widgets.Button)
fn parse_class_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(LocatorError::with_kind(
            "Empty class name in class: locator",
            LocatorErrorKind::InvalidSyntax
        ));
    }

    let mut selector = WidgetSelector::new();

    // Check if additional selectors are present (e.g., class:Button[text='OK'])
    if let Some(bracket_pos) = trimmed.find('[') {
        let class_name = &trimmed[..bracket_pos];
        selector.class_name = Some(class_name.to_string());

        // Also try to infer widget type from class name
        if let Some(widget_type) = infer_widget_type_from_class(class_name) {
            selector.widget_type = Some(widget_type);
        }

        // Parse remaining attributes
        let remaining = &trimmed[bracket_pos..];
        return parse_widget_attributes_and_pseudos(remaining, selector);
    }

    if let Some(pseudo_pos) = trimmed.find(':') {
        let class_name = &trimmed[..pseudo_pos];
        selector.class_name = Some(class_name.to_string());

        if let Some(widget_type) = infer_widget_type_from_class(class_name) {
            selector.widget_type = Some(widget_type);
        }

        let remaining = &trimmed[pseudo_pos..];
        return parse_widget_attributes_and_pseudos(remaining, selector);
    }

    // Just the class name
    selector.class_name = Some(trimmed.to_string());

    // Try to infer widget type from class name
    if let Some(widget_type) = infer_widget_type_from_class(trimmed) {
        selector.widget_type = Some(widget_type);
    }

    Ok(SwtLocator::Widget(selector))
}

/// Infer SwtWidgetType from a class name (simple or fully qualified)
///
/// This function takes a class name (either simple like "Button" or fully qualified
/// like "org.eclipse.swt.widgets.Button") and returns the corresponding SwtWidgetType
/// if it matches a known SWT widget type.
///
/// # Arguments
///
/// * `class_name` - The class name to match (e.g., "Button" or "org.eclipse.swt.widgets.Button")
///
/// # Returns
///
/// * `Some(SwtWidgetType)` if the class name matches a known widget type
/// * `None` if the class name is not recognized
pub fn infer_widget_type_from_class(class_name: &str) -> Option<SwtWidgetType> {
    // Get simple class name (after last '.')
    let simple_name = class_name.rsplit('.').next().unwrap_or(class_name);

    // Try to match common SWT widget types (case-insensitive)
    match simple_name.to_lowercase().as_str() {
        // Top-level shells
        "shell" => Some(SwtWidgetType::Shell),
        "decorations" => Some(SwtWidgetType::Decorations),

        // Containers
        "composite" => Some(SwtWidgetType::Composite),
        "group" => Some(SwtWidgetType::Group),
        "tabfolder" => Some(SwtWidgetType::TabFolder),
        "tabitem" => Some(SwtWidgetType::TabItem),
        "ctabfolder" => Some(SwtWidgetType::CTabFolder),
        "ctabitem" => Some(SwtWidgetType::CTabItem),
        "sashform" => Some(SwtWidgetType::SashForm),
        "scrolledcomposite" => Some(SwtWidgetType::ScrolledComposite),
        "expandbar" => Some(SwtWidgetType::ExpandBar),
        "expanditem" => Some(SwtWidgetType::ExpandItem),
        "canvas" => Some(SwtWidgetType::Canvas),
        "coolbar" => Some(SwtWidgetType::CoolBar),
        "coolitem" => Some(SwtWidgetType::CoolItem),
        "toolbar" => Some(SwtWidgetType::ToolBar),
        "toolitem" => Some(SwtWidgetType::ToolItem),

        // Basic controls
        "button" => Some(SwtWidgetType::Button),
        "label" => Some(SwtWidgetType::Label),
        "text" => Some(SwtWidgetType::Text),
        "styledtext" => Some(SwtWidgetType::StyledText),
        "combo" => Some(SwtWidgetType::Combo),
        "ccombo" => Some(SwtWidgetType::CCombo),
        "list" => Some(SwtWidgetType::List),
        "link" => Some(SwtWidgetType::Link),

        // Complex controls
        "table" => Some(SwtWidgetType::Table),
        "tablecolumn" => Some(SwtWidgetType::TableColumn),
        "tableitem" => Some(SwtWidgetType::TableItem),
        "tree" => Some(SwtWidgetType::Tree),
        "treecolumn" => Some(SwtWidgetType::TreeColumn),
        "treeitem" => Some(SwtWidgetType::TreeItem),
        "datetime" => Some(SwtWidgetType::DateTime),
        "spinner" => Some(SwtWidgetType::Spinner),
        "scale" => Some(SwtWidgetType::Scale),
        "slider" => Some(SwtWidgetType::Slider),
        "progressbar" => Some(SwtWidgetType::ProgressBar),
        "browser" => Some(SwtWidgetType::Browser),

        // Menu system
        "menu" => Some(SwtWidgetType::Menu),
        "menuitem" => Some(SwtWidgetType::MenuItem),

        // Dialogs
        "messagebox" => Some(SwtWidgetType::MessageBox),
        "colordialog" => Some(SwtWidgetType::ColorDialog),
        "directorydialog" => Some(SwtWidgetType::DirectoryDialog),
        "filedialog" => Some(SwtWidgetType::FileDialog),
        "fontdialog" => Some(SwtWidgetType::FontDialog),
        "printdialog" => Some(SwtWidgetType::PrintDialog),

        // Miscellaneous
        "separator" => Some(SwtWidgetType::Separator),
        "tooltip" => Some(SwtWidgetType::ToolTip),
        "trayitem" => Some(SwtWidgetType::TrayItem),
        "tray" => Some(SwtWidgetType::Tray),
        "caret" => Some(SwtWidgetType::Caret),
        "tracker" => Some(SwtWidgetType::Tracker),

        // JFace/RCP Viewers
        "tableviewer" => Some(SwtWidgetType::TableViewer),
        "treeviewer" => Some(SwtWidgetType::TreeViewer),
        "listviewer" => Some(SwtWidgetType::ListViewer),
        "comboviewer" => Some(SwtWidgetType::ComboViewer),

        // Eclipse Forms
        "form" => Some(SwtWidgetType::Form),
        "scrolledform" => Some(SwtWidgetType::ScrolledForm),
        "section" => Some(SwtWidgetType::Section),
        "hyperlink" => Some(SwtWidgetType::Hyperlink),
        "imagehyperlink" => Some(SwtWidgetType::ImageHyperlink),
        "formtext" => Some(SwtWidgetType::FormText),

        // Nebula widgets
        "grid" => Some(SwtWidgetType::Grid),
        "griditem" => Some(SwtWidgetType::GridItem),
        "gridcolumn" => Some(SwtWidgetType::GridColumn),
        "gallery" => Some(SwtWidgetType::Gallery),
        "galleryitem" => Some(SwtWidgetType::GalleryItem),

        _ => None,
    }
}

/// Parse an index locator (index:N)
fn parse_index_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(LocatorError::with_kind(
            "Empty index in index: locator",
            LocatorErrorKind::InvalidSyntax
        ));
    }

    // Parse the index value
    let index: usize = trimmed.parse().map_err(|_| {
        LocatorError::with_kind(
            format!("Invalid index value '{}': expected a non-negative integer", trimmed),
            LocatorErrorKind::InvalidSyntax
        )
    })?;

    let mut selector = WidgetSelector::new();
    selector.index = Some(index);

    Ok(SwtLocator::Widget(selector))
}

/// Parse a name/text locator (name:Text or text:Text)
fn parse_name_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(LocatorError::with_kind(
            "Empty name in name: locator",
            LocatorErrorKind::InvalidSyntax
        ));
    }

    let mut selector = WidgetSelector::new();

    // Check for match mode prefixes
    if let Some(rest) = trimmed.strip_prefix("~") {
        // Regex match
        selector.text = Some(rest.to_string());
        selector.text_match_mode = TextMatchMode::Regex;
    } else if let Some(rest) = trimmed.strip_prefix("*") {
        // Contains match
        selector.text = Some(rest.to_string());
        selector.text_match_mode = TextMatchMode::Contains;
    } else if let Some(rest) = trimmed.strip_prefix("^") {
        // Starts with match
        selector.text = Some(rest.to_string());
        selector.text_match_mode = TextMatchMode::StartsWith;
    } else if let Some(rest) = trimmed.strip_prefix("$") {
        // Ends with match
        selector.text = Some(rest.to_string());
        selector.text_match_mode = TextMatchMode::EndsWith;
    } else {
        // Exact match (default)
        selector.text = Some(trimmed.to_string());
        selector.text_match_mode = TextMatchMode::Exact;
    }

    Ok(SwtLocator::Widget(selector))
}

/// Parse an ID locator (id:widgetId)
fn parse_id_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(LocatorError::with_kind(
            "Empty ID in id: locator",
            LocatorErrorKind::InvalidSyntax
        ));
    }

    let mut selector = WidgetSelector::new();
    selector.id = Some(trimmed.to_string());

    Ok(SwtLocator::Widget(selector))
}

/// Parse a tooltip locator (tooltip:Text)
fn parse_tooltip_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(LocatorError::with_kind(
            "Empty tooltip in tooltip: locator",
            LocatorErrorKind::InvalidSyntax
        ));
    }

    let mut selector = WidgetSelector::new();
    selector.tooltip = Some(trimmed.to_string());

    Ok(SwtLocator::Widget(selector))
}

/// Parse an accessible locator (accessible:name)
fn parse_accessible_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(LocatorError::with_kind(
            "Empty accessible name in accessible: locator",
            LocatorErrorKind::InvalidSyntax
        ));
    }

    let mut selector = WidgetSelector::new();
    selector.accessible_name = Some(trimmed.to_string());

    Ok(SwtLocator::Widget(selector))
}

/// Parse widget attributes and pseudo-classes from remaining input
fn parse_widget_attributes_and_pseudos(
    input: &str,
    mut selector: WidgetSelector
) -> Result<SwtLocator, LocatorError> {
    let mut remaining = input;

    // Parse attributes ([attr='value'])
    while let Some(rest) = remaining.strip_prefix('[') {
        if let Some(end_pos) = rest.find(']') {
            let attr_content = &rest[..end_pos];
            parse_attribute_into_selector(attr_content, &mut selector)?;
            remaining = &rest[end_pos + 1..];
        } else {
            return Err(LocatorError::with_kind(
                "Unclosed attribute bracket",
                LocatorErrorKind::InvalidSyntax
            ));
        }
    }

    // Parse pseudo-classes (:enabled)
    while let Some(rest) = remaining.strip_prefix(':') {
        let end_pos = rest.find(|c: char| c == ':' || c == '[' || c.is_whitespace())
            .unwrap_or(rest.len());

        let pseudo_name = &rest[..end_pos];

        if pseudo_name.starts_with("nth-child(") {
            if let Some(expr_end) = pseudo_name.find(')') {
                let expr = &pseudo_name[10..expr_end];
                if let Some(pseudo) = SwtPseudoClass::parse_nth(expr) {
                    selector.pseudo_class = Some(pseudo);
                }
            }
        } else if let Some(pseudo) = SwtPseudoClass::parse(pseudo_name) {
            selector.pseudo_class = Some(pseudo);
        }

        remaining = &rest[end_pos..];
    }

    Ok(SwtLocator::Widget(selector))
}

/// Parse widget locator syntax
fn parse_swt_widget_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let mut selector = WidgetSelector::new();
    let mut remaining = input;

    // Parse widget type (at the start)
    if let Some(space_pos) = remaining.find(|c: char| c == '#' || c == '[' || c == ':' || c.is_whitespace()) {
        let type_name = &remaining[..space_pos];
        if !type_name.is_empty() {
            selector.widget_type = Some(SwtWidgetType::from_class_name(type_name));
        }
        remaining = &remaining[space_pos..];
    } else if !remaining.is_empty() && !remaining.starts_with('#') && !remaining.starts_with('[') && !remaining.starts_with(':') {
        selector.widget_type = Some(SwtWidgetType::from_class_name(remaining));
        return Ok(SwtLocator::Widget(selector));
    }

    // Parse ID (#myId)
    if let Some(rest) = remaining.strip_prefix('#') {
        if let Some(end_pos) = rest.find(|c: char| c == '[' || c == ':' || c.is_whitespace()) {
            selector.id = Some(rest[..end_pos].to_string());
            remaining = &rest[end_pos..];
        } else {
            selector.id = Some(rest.to_string());
            return Ok(SwtLocator::Widget(selector));
        }
    }

    // Parse attributes ([attr='value'])
    while let Some(rest) = remaining.strip_prefix('[') {
        if let Some(end_pos) = rest.find(']') {
            let attr_content = &rest[..end_pos];
            parse_attribute_into_selector(attr_content, &mut selector)?;
            remaining = &rest[end_pos + 1..];
        } else {
            return Err(LocatorError::with_kind(
                "Unclosed attribute bracket",
                LocatorErrorKind::InvalidSyntax
            ));
        }
    }

    // Parse pseudo-classes (:enabled)
    while let Some(rest) = remaining.strip_prefix(':') {
        // Find end of pseudo-class
        let end_pos = rest.find(|c: char| c == ':' || c == '[' || c.is_whitespace())
            .unwrap_or(rest.len());

        let pseudo_name = &rest[..end_pos];

        // Handle nth-child(n) syntax
        if pseudo_name.starts_with("nth-child(") {
            if let Some(expr_end) = pseudo_name.find(')') {
                let expr = &pseudo_name[10..expr_end];
                if let Some(pseudo) = SwtPseudoClass::parse_nth(expr) {
                    selector.pseudo_class = Some(pseudo);
                }
            }
        } else if let Some(pseudo) = SwtPseudoClass::parse(pseudo_name) {
            selector.pseudo_class = Some(pseudo);
        }

        remaining = &rest[end_pos..];
    }

    Ok(SwtLocator::Widget(selector))
}

/// Parse attribute content into selector
fn parse_attribute_into_selector(
    content: &str,
    selector: &mut WidgetSelector
) -> Result<(), LocatorError> {
    // Handle different operators
    let (name, value) = if let Some(pos) = content.find("*=") {
        let name = content[..pos].trim();
        let value = extract_quoted_value(&content[pos + 2..])?;
        selector.text_match_mode = TextMatchMode::Contains;
        (name, value)
    } else if let Some(pos) = content.find("^=") {
        let name = content[..pos].trim();
        let value = extract_quoted_value(&content[pos + 2..])?;
        selector.text_match_mode = TextMatchMode::StartsWith;
        (name, value)
    } else if let Some(pos) = content.find("$=") {
        let name = content[..pos].trim();
        let value = extract_quoted_value(&content[pos + 2..])?;
        selector.text_match_mode = TextMatchMode::EndsWith;
        (name, value)
    } else if let Some(pos) = content.find("~=") {
        let name = content[..pos].trim();
        let value = extract_quoted_value(&content[pos + 2..])?;
        selector.text_match_mode = TextMatchMode::Regex;
        (name, value)
    } else if let Some(pos) = content.find('=') {
        let name = content[..pos].trim();
        let value = extract_quoted_value(&content[pos + 1..])?;
        (name, value)
    } else {
        // Just attribute existence
        return Ok(());
    };

    // Apply to appropriate field
    match name {
        "text" => selector.text = Some(value),
        "tooltip" => selector.tooltip = Some(value),
        "id" => selector.id = Some(value),
        "class" => selector.class_name = Some(value),
        "accessible-name" | "accessibleName" => selector.accessible_name = Some(value),
        "accessible-role" | "accessibleRole" => selector.accessible_role = Some(value),
        "style" => selector.style = Some(value),
        attr if attr.starts_with("data-") => {
            selector.data_key = Some(attr[5..].to_string());
            selector.data_value = Some(value);
        }
        _ => {
            // Treat as data key
            selector.data_key = Some(name.to_string());
            selector.data_value = Some(value);
        }
    }

    Ok(())
}

/// Extract value from quoted or unquoted string
fn extract_quoted_value(input: &str) -> Result<String, LocatorError> {
    let trimmed = input.trim();

    if trimmed.starts_with('\'') {
        if let Some(end) = trimmed[1..].find('\'') {
            return Ok(trimmed[1..end + 1].to_string());
        }
    } else if trimmed.starts_with('"') {
        if let Some(end) = trimmed[1..].find('"') {
            return Ok(trimmed[1..end + 1].to_string());
        }
    }

    // Unquoted value
    Ok(trimmed.to_string())
}

/// Parse view locator
fn parse_view_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let mut selector = ViewSelector::new();

    // Check for ID (starts with org. or similar package prefix)
    if input.contains('.') {
        selector.id = Some(input.to_string());
    } else {
        selector.title = Some(input.to_string());
    }

    Ok(SwtLocator::View(selector))
}

/// Parse editor locator
fn parse_editor_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let mut selector = EditorSelector::new();

    // Check if it looks like a path
    if input.contains('/') || input.contains('\\') {
        selector.input_path = Some(input.to_string());
    } else {
        selector.input_name = Some(input.to_string());
    }

    Ok(SwtLocator::Editor(selector))
}

/// Parse perspective locator
fn parse_perspective_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let mut selector = PerspectiveSelector::new();

    // Check for ID (starts with org. or similar package prefix)
    if input.contains('.') {
        selector.id = Some(input.to_string());
    } else {
        selector.label = Some(input.to_string());
    }

    Ok(SwtLocator::Perspective(selector))
}

/// Parse menu locator
fn parse_menu_locator(input: &str) -> Result<SwtLocator, LocatorError> {
    let mut selector = MenuSelector::new();

    // Menu paths use / separator
    if input.contains('/') {
        selector.path = Some(input.to_string());
    } else {
        selector.text = Some(input.to_string());
    }

    Ok(SwtLocator::Menu(selector))
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::widget::{WidgetId, WidgetBounds, SwtStyle};
    use std::collections::HashMap;

    fn create_test_widget(
        widget_type: SwtWidgetType,
        text: Option<&str>,
        enabled: bool,
        visible: bool,
    ) -> SwtWidget {
        let id = WidgetId::new(1, "0".to_string(), 0);
        let mut widget = SwtWidget::new(
            id,
            widget_type,
            widget_type.class_name().to_string(),
        );
        widget.text = text.map(|s| s.to_string());
        widget.state.enabled = enabled;
        widget.state.visible = visible;
        widget
    }

    #[test]
    fn test_widget_selector_type_match() {
        let widget = create_test_widget(SwtWidgetType::Button, Some("OK"), true, true);

        let selector = WidgetSelector::new()
            .with_type(SwtWidgetType::Button);

        assert!(SwtMatcher::matches(&widget, &selector));

        let selector2 = WidgetSelector::new()
            .with_type(SwtWidgetType::Text);

        assert!(!SwtMatcher::matches(&widget, &selector2));
    }

    #[test]
    fn test_widget_selector_text_match() {
        let widget = create_test_widget(SwtWidgetType::Button, Some("Save Changes"), true, true);

        // Exact match
        let selector = WidgetSelector::new()
            .with_text("Save Changes");
        assert!(SwtMatcher::matches(&widget, &selector));

        // Contains match
        let selector2 = WidgetSelector::new()
            .with_text_containing("Save");
        assert!(SwtMatcher::matches(&widget, &selector2));

        // Non-match
        let selector3 = WidgetSelector::new()
            .with_text("Cancel");
        assert!(!SwtMatcher::matches(&widget, &selector3));
    }

    #[test]
    fn test_widget_selector_pseudo_class() {
        let mut widget = create_test_widget(SwtWidgetType::Button, Some("OK"), true, true);

        let enabled_selector = WidgetSelector::new()
            .with_pseudo(SwtPseudoClass::Enabled);
        assert!(SwtMatcher::matches(&widget, &enabled_selector));

        let disabled_selector = WidgetSelector::new()
            .with_pseudo(SwtPseudoClass::Disabled);
        assert!(!SwtMatcher::matches(&widget, &disabled_selector));

        widget.state.enabled = false;
        assert!(!SwtMatcher::matches(&widget, &enabled_selector));
        assert!(SwtMatcher::matches(&widget, &disabled_selector));
    }

    #[test]
    fn test_widget_selector_visible_hidden() {
        let mut widget = create_test_widget(SwtWidgetType::Label, Some("Hello"), true, true);

        let visible_selector = WidgetSelector::new()
            .with_pseudo(SwtPseudoClass::Visible);
        assert!(SwtMatcher::matches(&widget, &visible_selector));

        widget.state.visible = false;
        assert!(!SwtMatcher::matches(&widget, &visible_selector));

        let hidden_selector = WidgetSelector::new()
            .with_pseudo(SwtPseudoClass::Hidden);
        assert!(SwtMatcher::matches(&widget, &hidden_selector));
    }

    #[test]
    fn test_find_first() {
        let widgets = vec![
            create_test_widget(SwtWidgetType::Label, Some("Name:"), true, true),
            create_test_widget(SwtWidgetType::Text, None, true, true),
            create_test_widget(SwtWidgetType::Button, Some("OK"), true, true),
            create_test_widget(SwtWidgetType::Button, Some("Cancel"), true, true),
        ];

        let selector = WidgetSelector::new()
            .with_type(SwtWidgetType::Button);

        let found = SwtMatcher::find_first(&widgets, &selector);
        assert!(found.is_some());
        assert_eq!(found.unwrap().text, Some("OK".to_string()));
    }

    #[test]
    fn test_find_all() {
        let widgets = vec![
            create_test_widget(SwtWidgetType::Button, Some("OK"), true, true),
            create_test_widget(SwtWidgetType::Button, Some("Cancel"), true, true),
            create_test_widget(SwtWidgetType::Label, Some("Info"), true, true),
        ];

        let selector = WidgetSelector::new()
            .with_type(SwtWidgetType::Button);

        let found = SwtMatcher::find_all(&widgets, &selector);
        assert_eq!(found.len(), 2);
    }

    #[test]
    fn test_find_with_index() {
        let widgets = vec![
            create_test_widget(SwtWidgetType::Button, Some("First"), true, true),
            create_test_widget(SwtWidgetType::Button, Some("Second"), true, true),
            create_test_widget(SwtWidgetType::Button, Some("Third"), true, true),
        ];

        let selector = WidgetSelector::new()
            .with_type(SwtWidgetType::Button)
            .at_index(1);

        let found = SwtMatcher::find_all(&widgets, &selector);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].text, Some("Second".to_string()));
    }

    #[test]
    fn test_parse_simple_type() {
        let locator = parse_swt_locator("Button").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.widget_type, Some(SwtWidgetType::Button));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_type_with_id() {
        let locator = parse_swt_locator("Button#okButton").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.widget_type, Some(SwtWidgetType::Button));
            assert_eq!(selector.id, Some("okButton".to_string()));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_with_attribute() {
        let locator = parse_swt_locator("Button[text='OK']").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.widget_type, Some(SwtWidgetType::Button));
            assert_eq!(selector.text, Some("OK".to_string()));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_with_pseudo_class() {
        let locator = parse_swt_locator("Button:enabled").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.widget_type, Some(SwtWidgetType::Button));
            assert_eq!(selector.pseudo_class, Some(SwtPseudoClass::Enabled));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_swt_prefix() {
        let locator = parse_swt_locator("swt:Shell").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.widget_type, Some(SwtWidgetType::Shell));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_view_locator() {
        let locator = parse_swt_locator("view:Problems").unwrap();
        if let SwtLocator::View(selector) = locator {
            assert_eq!(selector.title, Some("Problems".to_string()));
        } else {
            panic!("Expected View locator");
        }
    }

    #[test]
    fn test_parse_editor_locator() {
        let locator = parse_swt_locator("editor:MyClass.java").unwrap();
        if let SwtLocator::Editor(selector) = locator {
            assert_eq!(selector.input_name, Some("MyClass.java".to_string()));
        } else {
            panic!("Expected Editor locator");
        }
    }

    #[test]
    fn test_parse_perspective_locator() {
        let locator = parse_swt_locator("perspective:Java").unwrap();
        if let SwtLocator::Perspective(selector) = locator {
            assert_eq!(selector.label, Some("Java".to_string()));
        } else {
            panic!("Expected Perspective locator");
        }
    }

    #[test]
    fn test_parse_menu_locator() {
        let locator = parse_swt_locator("menu:File/New/Project").unwrap();
        if let SwtLocator::Menu(selector) = locator {
            assert_eq!(selector.path, Some("File/New/Project".to_string()));
        } else {
            panic!("Expected Menu locator");
        }
    }

    #[test]
    fn test_pseudo_class_parsing() {
        assert_eq!(SwtPseudoClass::parse("enabled"), Some(SwtPseudoClass::Enabled));
        assert_eq!(SwtPseudoClass::parse("DISABLED"), Some(SwtPseudoClass::Disabled));
        assert_eq!(SwtPseudoClass::parse("first-child"), Some(SwtPseudoClass::FirstChild));
        assert_eq!(SwtPseudoClass::parse("visible"), Some(SwtPseudoClass::Visible));
        assert_eq!(SwtPseudoClass::parse("unknown"), None);
    }

    #[test]
    fn test_locator_error() {
        let err = parse_swt_locator("");
        assert!(err.is_err());
        assert_eq!(err.unwrap_err().kind, LocatorErrorKind::EmptyLocator);
    }

    #[test]
    fn test_widget_selector_builder() {
        let locator = WidgetSelector::new()
            .with_type(SwtWidgetType::Button)
            .with_text("Submit")
            .with_pseudo(SwtPseudoClass::Enabled)
            .build();

        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.widget_type, Some(SwtWidgetType::Button));
            assert_eq!(selector.text, Some("Submit".to_string()));
            assert_eq!(selector.pseudo_class, Some(SwtPseudoClass::Enabled));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_view_selector_builder() {
        let locator = ViewSelector::new()
            .with_id("org.eclipse.ui.views.ProblemView")
            .active()
            .build();

        if let SwtLocator::View(selector) = locator {
            assert_eq!(selector.id, Some("org.eclipse.ui.views.ProblemView".to_string()));
            assert_eq!(selector.active, Some(true));
        } else {
            panic!("Expected View locator");
        }
    }

    #[test]
    fn test_compound_locator() {
        let widget_locator = WidgetSelector::new()
            .with_type(SwtWidgetType::Button)
            .build();

        let enabled_locator = WidgetSelector::new()
            .with_pseudo(SwtPseudoClass::Enabled)
            .build();

        let compound = widget_locator.and(enabled_locator);

        assert!(matches!(compound, SwtLocator::Compound(_, _)));
    }

    #[test]
    fn test_union_locator() {
        let button_locator = WidgetSelector::new()
            .with_type(SwtWidgetType::Button)
            .build();

        let text_locator = WidgetSelector::new()
            .with_type(SwtWidgetType::Text)
            .build();

        let union = button_locator.or(text_locator);

        assert!(matches!(union, SwtLocator::Union(_)));
    }

    // =============================================================================
    // New Locator Type Tests
    // =============================================================================

    #[test]
    fn test_parse_class_locator_simple() {
        let locator = parse_swt_locator("class:Button").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.class_name, Some("Button".to_string()));
            assert_eq!(selector.widget_type, Some(SwtWidgetType::Button));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_class_locator_fully_qualified() {
        let locator = parse_swt_locator("class:org.eclipse.swt.widgets.Button").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.class_name, Some("org.eclipse.swt.widgets.Button".to_string()));
            assert_eq!(selector.widget_type, Some(SwtWidgetType::Button));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_class_locator_with_attribute() {
        let locator = parse_swt_locator("class:Button[text='OK']").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.class_name, Some("Button".to_string()));
            assert_eq!(selector.text, Some("OK".to_string()));
            assert_eq!(selector.widget_type, Some(SwtWidgetType::Button));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_class_locator_with_pseudo() {
        let locator = parse_swt_locator("class:Button:enabled").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.class_name, Some("Button".to_string()));
            assert_eq!(selector.pseudo_class, Some(SwtPseudoClass::Enabled));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_class_locator_custom_class() {
        let locator = parse_swt_locator("class:com.myapp.CustomWidget").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.class_name, Some("com.myapp.CustomWidget".to_string()));
            // Custom class should not infer a widget type
            assert_eq!(selector.widget_type, None);
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_index_locator() {
        let locator = parse_swt_locator("index:0").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.index, Some(0));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_index_locator_larger_value() {
        let locator = parse_swt_locator("index:42").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.index, Some(42));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_index_locator_invalid() {
        let result = parse_swt_locator("index:abc");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, LocatorErrorKind::InvalidSyntax);
    }

    #[test]
    fn test_parse_index_locator_empty() {
        let result = parse_swt_locator("index:");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_name_locator() {
        let locator = parse_swt_locator("name:Submit").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.text, Some("Submit".to_string()));
            assert_eq!(selector.text_match_mode, TextMatchMode::Exact);
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_name_locator_contains() {
        let locator = parse_swt_locator("name:*Save").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.text, Some("Save".to_string()));
            assert_eq!(selector.text_match_mode, TextMatchMode::Contains);
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_name_locator_starts_with() {
        let locator = parse_swt_locator("name:^Save").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.text, Some("Save".to_string()));
            assert_eq!(selector.text_match_mode, TextMatchMode::StartsWith);
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_name_locator_ends_with() {
        let locator = parse_swt_locator("name:$File").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.text, Some("File".to_string()));
            assert_eq!(selector.text_match_mode, TextMatchMode::EndsWith);
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_name_locator_regex() {
        let locator = parse_swt_locator("name:~Save.*").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.text, Some("Save.*".to_string()));
            assert_eq!(selector.text_match_mode, TextMatchMode::Regex);
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_text_locator_alias() {
        // text: should work as an alias for name:
        let locator = parse_swt_locator("text:Submit").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.text, Some("Submit".to_string()));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_id_locator() {
        let locator = parse_swt_locator("id:myButton").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.id, Some("myButton".to_string()));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_tooltip_locator() {
        let locator = parse_swt_locator("tooltip:Click to save").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.tooltip, Some("Click to save".to_string()));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_parse_accessible_locator() {
        let locator = parse_swt_locator("accessible:Save button").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.accessible_name, Some("Save button".to_string()));
        } else {
            panic!("Expected Widget locator");
        }
    }

    // =============================================================================
    // Case-Insensitive Prefix Tests
    // =============================================================================

    #[test]
    fn test_case_insensitive_prefix_lowercase() {
        let locator = parse_swt_locator("name:Test").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.text, Some("Test".to_string()));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_case_insensitive_prefix_uppercase() {
        let locator = parse_swt_locator("NAME:Test").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.text, Some("Test".to_string()));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_case_insensitive_prefix_mixed_case() {
        let locator = parse_swt_locator("Name:Test").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.text, Some("Test".to_string()));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_case_insensitive_class_prefix() {
        let locator = parse_swt_locator("CLASS:Button").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.class_name, Some("Button".to_string()));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_case_insensitive_index_prefix() {
        let locator = parse_swt_locator("INDEX:5").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.index, Some(5));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_case_insensitive_swt_prefix() {
        let locator = parse_swt_locator("SWT:Button").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.widget_type, Some(SwtWidgetType::Button));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_case_insensitive_view_prefix() {
        let locator = parse_swt_locator("VIEW:Problems").unwrap();
        if let SwtLocator::View(selector) = locator {
            assert_eq!(selector.title, Some("Problems".to_string()));
        } else {
            panic!("Expected View locator");
        }
    }

    #[test]
    fn test_case_insensitive_editor_prefix() {
        let locator = parse_swt_locator("EDITOR:MyFile.java").unwrap();
        if let SwtLocator::Editor(selector) = locator {
            assert_eq!(selector.input_name, Some("MyFile.java".to_string()));
        } else {
            panic!("Expected Editor locator");
        }
    }

    #[test]
    fn test_case_insensitive_menu_prefix() {
        let locator = parse_swt_locator("MENU:File/New").unwrap();
        if let SwtLocator::Menu(selector) = locator {
            assert_eq!(selector.path, Some("File/New".to_string()));
        } else {
            panic!("Expected Menu locator");
        }
    }

    #[test]
    fn test_case_insensitive_perspective_prefix() {
        let locator = parse_swt_locator("PERSPECTIVE:Java").unwrap();
        if let SwtLocator::Perspective(selector) = locator {
            assert_eq!(selector.label, Some("Java".to_string()));
        } else {
            panic!("Expected Perspective locator");
        }
    }

    // =============================================================================
    // Class Name Matching Tests
    // =============================================================================

    #[test]
    fn test_class_name_matching_contains() {
        let mut widget = create_test_widget(SwtWidgetType::Button, Some("OK"), true, true);
        widget.class_name = "org.eclipse.swt.widgets.Button".to_string();

        let selector = WidgetSelector::new()
            .with_class_name("Button");

        assert!(SwtMatcher::matches(&widget, &selector));
    }

    #[test]
    fn test_class_name_matching_full_class() {
        let mut widget = create_test_widget(SwtWidgetType::Button, Some("OK"), true, true);
        widget.class_name = "org.eclipse.swt.widgets.Button".to_string();

        let selector = WidgetSelector::new()
            .with_class_name("org.eclipse.swt.widgets.Button");

        assert!(SwtMatcher::matches(&widget, &selector));
    }

    #[test]
    fn test_class_name_matching_package_prefix() {
        let mut widget = create_test_widget(SwtWidgetType::Button, Some("OK"), true, true);
        widget.class_name = "org.eclipse.swt.widgets.Button".to_string();

        let selector = WidgetSelector::new()
            .with_class_name("org.eclipse.swt");

        assert!(SwtMatcher::matches(&widget, &selector));
    }

    #[test]
    fn test_class_name_no_match() {
        let mut widget = create_test_widget(SwtWidgetType::Button, Some("OK"), true, true);
        widget.class_name = "org.eclipse.swt.widgets.Button".to_string();

        let selector = WidgetSelector::new()
            .with_class_name("Label");

        assert!(!SwtMatcher::matches(&widget, &selector));
    }

    // =============================================================================
    // Index Selection Tests
    // =============================================================================

    #[test]
    fn test_index_selection_first() {
        let widgets = vec![
            create_test_widget(SwtWidgetType::Button, Some("First"), true, true),
            create_test_widget(SwtWidgetType::Button, Some("Second"), true, true),
            create_test_widget(SwtWidgetType::Button, Some("Third"), true, true),
        ];

        let selector = WidgetSelector::new()
            .with_type(SwtWidgetType::Button)
            .at_index(0);

        let found = SwtMatcher::find_all(&widgets, &selector);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].text, Some("First".to_string()));
    }

    #[test]
    fn test_index_selection_middle() {
        let widgets = vec![
            create_test_widget(SwtWidgetType::Button, Some("First"), true, true),
            create_test_widget(SwtWidgetType::Button, Some("Second"), true, true),
            create_test_widget(SwtWidgetType::Button, Some("Third"), true, true),
        ];

        let selector = WidgetSelector::new()
            .with_type(SwtWidgetType::Button)
            .at_index(1);

        let found = SwtMatcher::find_all(&widgets, &selector);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].text, Some("Second".to_string()));
    }

    #[test]
    fn test_index_selection_last() {
        let widgets = vec![
            create_test_widget(SwtWidgetType::Button, Some("First"), true, true),
            create_test_widget(SwtWidgetType::Button, Some("Second"), true, true),
            create_test_widget(SwtWidgetType::Button, Some("Third"), true, true),
        ];

        let selector = WidgetSelector::new()
            .with_type(SwtWidgetType::Button)
            .at_index(2);

        let found = SwtMatcher::find_all(&widgets, &selector);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].text, Some("Third".to_string()));
    }

    #[test]
    fn test_index_selection_out_of_bounds() {
        let widgets = vec![
            create_test_widget(SwtWidgetType::Button, Some("First"), true, true),
            create_test_widget(SwtWidgetType::Button, Some("Second"), true, true),
        ];

        let selector = WidgetSelector::new()
            .with_type(SwtWidgetType::Button)
            .at_index(10);

        let found = SwtMatcher::find_all(&widgets, &selector);
        assert_eq!(found.len(), 0);
    }

    // =============================================================================
    // Edge Case Tests
    // =============================================================================

    #[test]
    fn test_empty_class_locator() {
        let result = parse_swt_locator("class:");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_name_locator() {
        let result = parse_swt_locator("name:");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_id_locator() {
        let result = parse_swt_locator("id:");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_tooltip_locator() {
        let result = parse_swt_locator("tooltip:");
        assert!(result.is_err());
    }

    #[test]
    fn test_whitespace_handling() {
        let locator = parse_swt_locator("  class:Button  ").unwrap();
        if let SwtLocator::Widget(selector) = locator {
            assert_eq!(selector.class_name, Some("Button".to_string()));
        } else {
            panic!("Expected Widget locator");
        }
    }

    #[test]
    fn test_infer_widget_type_from_class() {
        // Basic types
        assert_eq!(infer_widget_type_from_class("Button"), Some(SwtWidgetType::Button));
        assert_eq!(infer_widget_type_from_class("button"), Some(SwtWidgetType::Button));
        assert_eq!(infer_widget_type_from_class("BUTTON"), Some(SwtWidgetType::Button));
        assert_eq!(infer_widget_type_from_class("org.eclipse.swt.widgets.Button"), Some(SwtWidgetType::Button));
        assert_eq!(infer_widget_type_from_class("Text"), Some(SwtWidgetType::Text));
        assert_eq!(infer_widget_type_from_class("Label"), Some(SwtWidgetType::Label));
        assert_eq!(infer_widget_type_from_class("Table"), Some(SwtWidgetType::Table));
        assert_eq!(infer_widget_type_from_class("Tree"), Some(SwtWidgetType::Tree));
        assert_eq!(infer_widget_type_from_class("StyledText"), Some(SwtWidgetType::StyledText));
        assert_eq!(infer_widget_type_from_class("CTabFolder"), Some(SwtWidgetType::CTabFolder));

        // Containers
        assert_eq!(infer_widget_type_from_class("Shell"), Some(SwtWidgetType::Shell));
        assert_eq!(infer_widget_type_from_class("Composite"), Some(SwtWidgetType::Composite));
        assert_eq!(infer_widget_type_from_class("Group"), Some(SwtWidgetType::Group));

        // Eclipse Forms
        assert_eq!(infer_widget_type_from_class("Section"), Some(SwtWidgetType::Section));
        assert_eq!(infer_widget_type_from_class("Hyperlink"), Some(SwtWidgetType::Hyperlink));

        // Unknown types
        assert_eq!(infer_widget_type_from_class("UnknownWidget"), None);
        assert_eq!(infer_widget_type_from_class("com.custom.MyWidget"), None);
    }
}
