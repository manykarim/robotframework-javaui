//! Unified locator normalization - Anti-Corruption Layer
//!
//! This module provides a unified locator syntax that works across
//! Swing, SWT, and RCP toolkits with automatic normalization.
//!
//! # Supported Locator Formats
//!
//! ## CSS-style selectors (canonical, recommended)
//! ```text
//! Button                           # Type selector
//! Button[name='submit']            # Type + attribute
//! [name='submit']                  # Attribute only
//! #submitButton                    # ID shorthand (name attribute)
//! Button:enabled                   # Type + pseudo-class
//! Panel > Button                   # Child combinator
//! ```
//!
//! ## Prefix-style locators (SWT-compatible)
//! ```text
//! name:submitButton                # By name
//! text:Click Me                    # By text content
//! class:JButton                    # By class name
//! id:myId                          # By ID
//! tooltip:Save file                # By tooltip
//! index:0                          # By index
//! accessible:Button name           # By accessible name
//! ```
//!
//! ## XPath expressions
//! ```text
//! //JButton[@text='OK']            # XPath absolute
//! .//Button                        # XPath relative
//! ```
//!
//! ## Toolkit-specific prefixes
//! ```text
//! swing:JButton[name='x']          # Swing-specific
//! swt:Button[text='OK']            # SWT-specific
//! rcp:org.eclipse.ui.view          # Eclipse RCP view/editor
//! ```

use crate::core::backend::ToolkitType;
use lru::LruCache;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt;
use std::num::NonZeroUsize;
use std::sync::Mutex;

// =============================================================================
// Constants and Static Data
// =============================================================================

/// Default LRU cache size for parsed locators
const DEFAULT_CACHE_SIZE: usize = 1000;

/// Widget type mappings between Swing and SWT
pub static WIDGET_TYPE_MAPPINGS: &[WidgetTypeMapping] = &[
    // Buttons
    WidgetTypeMapping {
        swing_class: "javax.swing.JButton",
        swing_simple: "JButton",
        swt_class: "org.eclipse.swt.widgets.Button",
        swt_simple: "Button",
        canonical_name: "Button",
        category: WidgetCategory::Button,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JToggleButton",
        swing_simple: "JToggleButton",
        swt_class: "org.eclipse.swt.widgets.Button",
        swt_simple: "ToggleButton",
        canonical_name: "ToggleButton",
        category: WidgetCategory::Button,
    },
    // Text inputs
    WidgetTypeMapping {
        swing_class: "javax.swing.JTextField",
        swing_simple: "JTextField",
        swt_class: "org.eclipse.swt.widgets.Text",
        swt_simple: "Text",
        canonical_name: "TextField",
        category: WidgetCategory::TextInput,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JTextArea",
        swing_simple: "JTextArea",
        swt_class: "org.eclipse.swt.custom.StyledText",
        swt_simple: "StyledText",
        canonical_name: "TextArea",
        category: WidgetCategory::TextInput,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JPasswordField",
        swing_simple: "JPasswordField",
        swt_class: "org.eclipse.swt.widgets.Text",
        swt_simple: "Text",
        canonical_name: "PasswordField",
        category: WidgetCategory::TextInput,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JFormattedTextField",
        swing_simple: "JFormattedTextField",
        swt_class: "org.eclipse.swt.widgets.Text",
        swt_simple: "Text",
        canonical_name: "FormattedTextField",
        category: WidgetCategory::TextInput,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JEditorPane",
        swing_simple: "JEditorPane",
        swt_class: "org.eclipse.swt.custom.StyledText",
        swt_simple: "StyledText",
        canonical_name: "EditorPane",
        category: WidgetCategory::TextInput,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JTextPane",
        swing_simple: "JTextPane",
        swt_class: "org.eclipse.swt.custom.StyledText",
        swt_simple: "StyledText",
        canonical_name: "TextPane",
        category: WidgetCategory::TextInput,
    },
    // Selection controls
    WidgetTypeMapping {
        swing_class: "javax.swing.JCheckBox",
        swing_simple: "JCheckBox",
        swt_class: "org.eclipse.swt.widgets.Button",
        swt_simple: "CheckBox",
        canonical_name: "CheckBox",
        category: WidgetCategory::Selection,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JRadioButton",
        swing_simple: "JRadioButton",
        swt_class: "org.eclipse.swt.widgets.Button",
        swt_simple: "RadioButton",
        canonical_name: "RadioButton",
        category: WidgetCategory::Selection,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JComboBox",
        swing_simple: "JComboBox",
        swt_class: "org.eclipse.swt.widgets.Combo",
        swt_simple: "Combo",
        canonical_name: "ComboBox",
        category: WidgetCategory::Selection,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JList",
        swing_simple: "JList",
        swt_class: "org.eclipse.swt.widgets.List",
        swt_simple: "List",
        canonical_name: "List",
        category: WidgetCategory::Selection,
    },
    // Data display
    WidgetTypeMapping {
        swing_class: "javax.swing.JTable",
        swing_simple: "JTable",
        swt_class: "org.eclipse.swt.widgets.Table",
        swt_simple: "Table",
        canonical_name: "Table",
        category: WidgetCategory::DataDisplay,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JTree",
        swing_simple: "JTree",
        swt_class: "org.eclipse.swt.widgets.Tree",
        swt_simple: "Tree",
        canonical_name: "Tree",
        category: WidgetCategory::DataDisplay,
    },
    // Labels and display
    WidgetTypeMapping {
        swing_class: "javax.swing.JLabel",
        swing_simple: "JLabel",
        swt_class: "org.eclipse.swt.widgets.Label",
        swt_simple: "Label",
        canonical_name: "Label",
        category: WidgetCategory::Display,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JProgressBar",
        swing_simple: "JProgressBar",
        swt_class: "org.eclipse.swt.widgets.ProgressBar",
        swt_simple: "ProgressBar",
        canonical_name: "ProgressBar",
        category: WidgetCategory::Display,
    },
    // Containers
    WidgetTypeMapping {
        swing_class: "javax.swing.JPanel",
        swing_simple: "JPanel",
        swt_class: "org.eclipse.swt.widgets.Composite",
        swt_simple: "Composite",
        canonical_name: "Panel",
        category: WidgetCategory::Container,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JScrollPane",
        swing_simple: "JScrollPane",
        swt_class: "org.eclipse.swt.custom.ScrolledComposite",
        swt_simple: "ScrolledComposite",
        canonical_name: "ScrollPane",
        category: WidgetCategory::Container,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JSplitPane",
        swing_simple: "JSplitPane",
        swt_class: "org.eclipse.swt.custom.SashForm",
        swt_simple: "SashForm",
        canonical_name: "SplitPane",
        category: WidgetCategory::Container,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JTabbedPane",
        swing_simple: "JTabbedPane",
        swt_class: "org.eclipse.swt.widgets.TabFolder",
        swt_simple: "TabFolder",
        canonical_name: "TabFolder",
        category: WidgetCategory::Container,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JLayeredPane",
        swing_simple: "JLayeredPane",
        swt_class: "org.eclipse.swt.widgets.Composite",
        swt_simple: "Composite",
        canonical_name: "LayeredPane",
        category: WidgetCategory::Container,
    },
    // Menus
    WidgetTypeMapping {
        swing_class: "javax.swing.JMenuBar",
        swing_simple: "JMenuBar",
        swt_class: "org.eclipse.swt.widgets.Menu",
        swt_simple: "MenuBar",
        canonical_name: "MenuBar",
        category: WidgetCategory::Menu,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JMenu",
        swing_simple: "JMenu",
        swt_class: "org.eclipse.swt.widgets.Menu",
        swt_simple: "Menu",
        canonical_name: "Menu",
        category: WidgetCategory::Menu,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JMenuItem",
        swing_simple: "JMenuItem",
        swt_class: "org.eclipse.swt.widgets.MenuItem",
        swt_simple: "MenuItem",
        canonical_name: "MenuItem",
        category: WidgetCategory::Menu,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JPopupMenu",
        swing_simple: "JPopupMenu",
        swt_class: "org.eclipse.swt.widgets.Menu",
        swt_simple: "PopupMenu",
        canonical_name: "PopupMenu",
        category: WidgetCategory::Menu,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JCheckBoxMenuItem",
        swing_simple: "JCheckBoxMenuItem",
        swt_class: "org.eclipse.swt.widgets.MenuItem",
        swt_simple: "CheckMenuItem",
        canonical_name: "CheckMenuItem",
        category: WidgetCategory::Menu,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JRadioButtonMenuItem",
        swing_simple: "JRadioButtonMenuItem",
        swt_class: "org.eclipse.swt.widgets.MenuItem",
        swt_simple: "RadioMenuItem",
        canonical_name: "RadioMenuItem",
        category: WidgetCategory::Menu,
    },
    // Toolbars
    WidgetTypeMapping {
        swing_class: "javax.swing.JToolBar",
        swing_simple: "JToolBar",
        swt_class: "org.eclipse.swt.widgets.ToolBar",
        swt_simple: "ToolBar",
        canonical_name: "ToolBar",
        category: WidgetCategory::Toolbar,
    },
    // Sliders and spinners
    WidgetTypeMapping {
        swing_class: "javax.swing.JSlider",
        swing_simple: "JSlider",
        swt_class: "org.eclipse.swt.widgets.Slider",
        swt_simple: "Slider",
        canonical_name: "Slider",
        category: WidgetCategory::Range,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JSpinner",
        swing_simple: "JSpinner",
        swt_class: "org.eclipse.swt.widgets.Spinner",
        swt_simple: "Spinner",
        canonical_name: "Spinner",
        category: WidgetCategory::Range,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JScrollBar",
        swing_simple: "JScrollBar",
        swt_class: "org.eclipse.swt.widgets.ScrollBar",
        swt_simple: "ScrollBar",
        canonical_name: "ScrollBar",
        category: WidgetCategory::Range,
    },
    // Windows and dialogs
    WidgetTypeMapping {
        swing_class: "javax.swing.JFrame",
        swing_simple: "JFrame",
        swt_class: "org.eclipse.swt.widgets.Shell",
        swt_simple: "Shell",
        canonical_name: "Window",
        category: WidgetCategory::Window,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JDialog",
        swing_simple: "JDialog",
        swt_class: "org.eclipse.swt.widgets.Shell",
        swt_simple: "Shell",
        canonical_name: "Dialog",
        category: WidgetCategory::Window,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JInternalFrame",
        swing_simple: "JInternalFrame",
        swt_class: "org.eclipse.swt.widgets.Shell",
        swt_simple: "Shell",
        canonical_name: "InternalFrame",
        category: WidgetCategory::Window,
    },
    // File/Color choosers
    WidgetTypeMapping {
        swing_class: "javax.swing.JFileChooser",
        swing_simple: "JFileChooser",
        swt_class: "org.eclipse.swt.widgets.FileDialog",
        swt_simple: "FileDialog",
        canonical_name: "FileChooser",
        category: WidgetCategory::Dialog,
    },
    WidgetTypeMapping {
        swing_class: "javax.swing.JColorChooser",
        swing_simple: "JColorChooser",
        swt_class: "org.eclipse.swt.widgets.ColorDialog",
        swt_simple: "ColorDialog",
        canonical_name: "ColorChooser",
        category: WidgetCategory::Dialog,
    },
    // Option panes
    WidgetTypeMapping {
        swing_class: "javax.swing.JOptionPane",
        swing_simple: "JOptionPane",
        swt_class: "org.eclipse.swt.widgets.MessageBox",
        swt_simple: "MessageBox",
        canonical_name: "MessageDialog",
        category: WidgetCategory::Dialog,
    },
    // Separators
    WidgetTypeMapping {
        swing_class: "javax.swing.JSeparator",
        swing_simple: "JSeparator",
        swt_class: "org.eclipse.swt.widgets.Label",
        swt_simple: "Separator",
        canonical_name: "Separator",
        category: WidgetCategory::Display,
    },
];

/// Widget category for grouping
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WidgetCategory {
    Button,
    TextInput,
    Selection,
    DataDisplay,
    Display,
    Container,
    Menu,
    Toolbar,
    Range,
    Window,
    Dialog,
}

/// Widget type mapping between Swing and SWT
#[derive(Debug, Clone)]
pub struct WidgetTypeMapping {
    /// Full Swing class name (e.g., "javax.swing.JButton")
    pub swing_class: &'static str,
    /// Simple Swing class name (e.g., "JButton")
    pub swing_simple: &'static str,
    /// Full SWT class name (e.g., "org.eclipse.swt.widgets.Button")
    pub swt_class: &'static str,
    /// Simple SWT class name (e.g., "Button")
    pub swt_simple: &'static str,
    /// Canonical name (e.g., "Button")
    pub canonical_name: &'static str,
    /// Widget category
    pub category: WidgetCategory,
}

// =============================================================================
// Error Types
// =============================================================================

/// Error type for locator parsing
#[derive(Debug, Clone)]
pub struct LocatorParseError {
    /// Error message
    pub message: String,
    /// Position in input where error occurred
    pub position: Option<usize>,
    /// Suggestions for correction
    pub suggestions: Vec<String>,
}

impl fmt::Display for LocatorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.position {
            Some(pos) => write!(f, "{} at position {}", self.message, pos),
            None => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for LocatorParseError {}

impl LocatorParseError {
    /// Create a new parse error
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self {
            message: message.into(),
            position: None,
            suggestions: vec![],
        }
    }

    /// Create an error at a specific position
    pub fn at_position<S: Into<String>>(message: S, position: usize) -> Self {
        Self {
            message: message.into(),
            position: Some(position),
            suggestions: vec![],
        }
    }

    /// Add a suggestion
    pub fn with_suggestion<S: Into<String>>(mut self, suggestion: S) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    /// Add multiple suggestions
    pub fn with_suggestions<I, S>(mut self, suggestions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.suggestions.extend(suggestions.into_iter().map(|s| s.into()));
        self
    }
}

// =============================================================================
// Core Locator Types
// =============================================================================

/// Unified locator that works across toolkits
#[derive(Debug, Clone)]
pub struct UnifiedLocator {
    /// Original locator string
    pub original: String,
    /// Parsed locator type
    pub locator_type: LocatorType,
    /// Primary value
    pub value: String,
    /// Additional predicates (attributes, pseudo-classes)
    pub predicates: Vec<LocatorPredicate>,
}

/// Locator type enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LocatorType {
    /// By name attribute: name:myButton, #myButton
    Name,
    /// By text content: text:Click Me
    Text,
    /// By text containing substring: text*=partial
    TextContains,
    /// By text matching regex: text~=pattern
    TextRegex,
    /// By class name: class:JButton, Button
    Class,
    /// By index: index:0
    Index,
    /// By ID (hash code): id:12345
    Id,
    /// By tooltip: tooltip:Help text
    Tooltip,
    /// By accessible name: accessible:Button name
    AccessibleName,
    /// CSS-like selector: JButton[text="Save"]
    Css,
    /// XPath: //JButton[@text='Save']
    XPath,
    /// Toolkit-specific: swing:JButton, swt:Button
    Toolkit { toolkit: String, selector: String },
}

/// Locator predicate for filtering
#[derive(Debug, Clone, PartialEq)]
pub enum LocatorPredicate {
    /// Attribute match: [name="value"]
    Attribute {
        name: String,
        op: MatchOp,
        value: String,
    },
    /// Pseudo-class: :visible, :enabled
    PseudoClass(PseudoClass),
    /// Index selection: :nth(2)
    Index(usize),
    /// Combinator to parent/child
    Combinator(Combinator),
}

/// Match operation for attribute predicates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchOp {
    /// Exact match: =
    Equals,
    /// Not equal: !=
    NotEquals,
    /// Contains: *=
    Contains,
    /// Starts with: ^=
    StartsWith,
    /// Ends with: $=
    EndsWith,
    /// Regex match: ~=
    Regex,
    /// Word match: |= (word boundary)
    WordMatch,
}

impl fmt::Display for MatchOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatchOp::Equals => write!(f, "="),
            MatchOp::NotEquals => write!(f, "!="),
            MatchOp::Contains => write!(f, "*="),
            MatchOp::StartsWith => write!(f, "^="),
            MatchOp::EndsWith => write!(f, "$="),
            MatchOp::Regex => write!(f, "~="),
            MatchOp::WordMatch => write!(f, "|="),
        }
    }
}

/// Pseudo-class selectors (universal across toolkits)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoClass {
    // UI State
    Enabled,
    Disabled,
    Visible,
    Hidden,
    Focused,
    Selected,
    Checked,
    Unchecked,
    Editable,
    ReadOnly,

    // Position-based
    First,
    Last,
    NthChild(NthExpr),
    NthLastChild(NthExpr),
    OnlyChild,
    FirstOfType,
    LastOfType,
    NthOfType(NthExpr),
    OnlyOfType,

    // Content-based
    Empty,
    HasChildren,

    // Window state
    Active,
    Maximized,
    Minimized,

    // Tree/expandable state
    Expanded,
    Collapsed,

    // Custom pseudo-class
    Custom(String),
}

impl PseudoClass {
    /// Parse a pseudo-class from string
    pub fn parse(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "enabled" => Some(Self::Enabled),
            "disabled" => Some(Self::Disabled),
            "visible" => Some(Self::Visible),
            "hidden" => Some(Self::Hidden),
            "focused" | "focus" => Some(Self::Focused),
            "selected" => Some(Self::Selected),
            "checked" => Some(Self::Checked),
            "unchecked" => Some(Self::Unchecked),
            "editable" => Some(Self::Editable),
            "readonly" | "read-only" => Some(Self::ReadOnly),
            "first" | "first-child" => Some(Self::First),
            "last" | "last-child" => Some(Self::Last),
            "only-child" => Some(Self::OnlyChild),
            "first-of-type" => Some(Self::FirstOfType),
            "last-of-type" => Some(Self::LastOfType),
            "only-of-type" => Some(Self::OnlyOfType),
            "empty" => Some(Self::Empty),
            "has-children" => Some(Self::HasChildren),
            "active" => Some(Self::Active),
            "maximized" => Some(Self::Maximized),
            "minimized" => Some(Self::Minimized),
            "expanded" => Some(Self::Expanded),
            "collapsed" => Some(Self::Collapsed),
            _ => None,
        }
    }

    /// Parse nth expression (e.g., nth-child(2), nth-child(odd))
    pub fn parse_nth(pseudo_name: &str, expr: &str) -> Option<Self> {
        let nth_expr = NthExpr::parse(expr)?;
        match pseudo_name {
            "nth-child" | "nth" => Some(Self::NthChild(nth_expr)),
            "nth-last-child" => Some(Self::NthLastChild(nth_expr)),
            "nth-of-type" => Some(Self::NthOfType(nth_expr)),
            _ => None,
        }
    }
}

impl fmt::Display for PseudoClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Enabled => write!(f, ":enabled"),
            Self::Disabled => write!(f, ":disabled"),
            Self::Visible => write!(f, ":visible"),
            Self::Hidden => write!(f, ":hidden"),
            Self::Focused => write!(f, ":focused"),
            Self::Selected => write!(f, ":selected"),
            Self::Checked => write!(f, ":checked"),
            Self::Unchecked => write!(f, ":unchecked"),
            Self::Editable => write!(f, ":editable"),
            Self::ReadOnly => write!(f, ":readonly"),
            Self::First => write!(f, ":first"),
            Self::Last => write!(f, ":last"),
            Self::NthChild(expr) => write!(f, ":nth-child({})", expr),
            Self::NthLastChild(expr) => write!(f, ":nth-last-child({})", expr),
            Self::OnlyChild => write!(f, ":only-child"),
            Self::FirstOfType => write!(f, ":first-of-type"),
            Self::LastOfType => write!(f, ":last-of-type"),
            Self::NthOfType(expr) => write!(f, ":nth-of-type({})", expr),
            Self::OnlyOfType => write!(f, ":only-of-type"),
            Self::Empty => write!(f, ":empty"),
            Self::HasChildren => write!(f, ":has-children"),
            Self::Active => write!(f, ":active"),
            Self::Maximized => write!(f, ":maximized"),
            Self::Minimized => write!(f, ":minimized"),
            Self::Expanded => write!(f, ":expanded"),
            Self::Collapsed => write!(f, ":collapsed"),
            Self::Custom(name) => write!(f, ":{}", name),
        }
    }
}

/// Nth expression for positional pseudo-classes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NthExpr {
    /// Specific index (1-based)
    Index(i32),
    /// Odd indices (1, 3, 5, ...)
    Odd,
    /// Even indices (2, 4, 6, ...)
    Even,
    /// Formula an+b
    Formula { a: i32, b: i32 },
}

impl NthExpr {
    /// Parse an nth expression
    pub fn parse(expr: &str) -> Option<Self> {
        let trimmed = expr.trim().to_lowercase();

        if trimmed == "odd" {
            return Some(Self::Odd);
        }
        if trimmed == "even" {
            return Some(Self::Even);
        }
        if let Ok(n) = trimmed.parse::<i32>() {
            return Some(Self::Index(n));
        }

        // Parse formula an+b or an-b
        if trimmed.contains('n') {
            let parts: Vec<&str> = if trimmed.contains('+') {
                trimmed.split('+').collect()
            } else if trimmed.contains('-') && !trimmed.starts_with('-') {
                trimmed.split('-').collect()
            } else {
                vec![&trimmed[..]]
            };

            if !parts.is_empty() {
                let a_str = parts[0].trim().replace('n', "");
                let a = if a_str.is_empty() || a_str == "-" {
                    if a_str == "-" { -1 } else { 1 }
                } else {
                    a_str.parse().ok()?
                };

                let b = if parts.len() > 1 {
                    let b_val: i32 = parts[1].trim().parse().ok()?;
                    if trimmed.contains('-') && !trimmed.contains('+') {
                        -b_val
                    } else {
                        b_val
                    }
                } else {
                    0
                };

                return Some(Self::Formula { a, b });
            }
        }

        None
    }

    /// Check if a 1-based index matches this expression
    pub fn matches(&self, index: i32) -> bool {
        match self {
            Self::Index(i) => index == *i,
            Self::Odd => index % 2 == 1,
            Self::Even => index % 2 == 0,
            Self::Formula { a, b } => {
                if *a == 0 {
                    index == *b
                } else {
                    let n = index - b;
                    n % a == 0 && n / a >= 0
                }
            }
        }
    }
}

impl fmt::Display for NthExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Index(i) => write!(f, "{}", i),
            Self::Odd => write!(f, "odd"),
            Self::Even => write!(f, "even"),
            Self::Formula { a, b } => {
                if *b >= 0 {
                    write!(f, "{}n+{}", a, b)
                } else {
                    write!(f, "{}n{}", a, b)
                }
            }
        }
    }
}

/// CSS-style combinator
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Combinator {
    /// Descendant (space)
    Descendant,
    /// Direct child (>)
    Child,
    /// Adjacent sibling (+)
    AdjacentSibling,
    /// General sibling (~)
    GeneralSibling,
}

impl fmt::Display for Combinator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Descendant => write!(f, " "),
            Self::Child => write!(f, " > "),
            Self::AdjacentSibling => write!(f, " + "),
            Self::GeneralSibling => write!(f, " ~ "),
        }
    }
}

// =============================================================================
// Normalized Locator (internal representation)
// =============================================================================

/// Normalized locator ready for toolkit-specific execution
#[derive(Debug, Clone)]
pub struct NormalizedLocator {
    /// Normalized locator type
    pub locator_type: LocatorType,
    /// Normalized value (toolkit-specific class name)
    pub value: String,
    /// Predicates to apply
    pub predicates: Vec<LocatorPredicate>,
    /// Target toolkit
    pub toolkit: Option<ToolkitType>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl NormalizedLocator {
    /// Create a new normalized locator
    pub fn new(locator_type: LocatorType, value: String) -> Self {
        Self {
            locator_type,
            value,
            predicates: vec![],
            toolkit: None,
            metadata: HashMap::new(),
        }
    }

    /// Set target toolkit
    pub fn with_toolkit(mut self, toolkit: ToolkitType) -> Self {
        self.toolkit = Some(toolkit);
        self
    }

    /// Add a predicate
    pub fn with_predicate(mut self, predicate: LocatorPredicate) -> Self {
        self.predicates.push(predicate);
        self
    }

    /// Add metadata
    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.to_string());
        self
    }
}

// =============================================================================
// Locator Normalizer with Caching
// =============================================================================

/// Locator normalizer with LRU caching
pub struct LocatorNormalizer {
    /// LRU cache for parsed locators
    cache: Mutex<LruCache<String, NormalizedLocator>>,
    /// Type mappings lookup (canonical -> mapping)
    type_lookup: HashMap<String, &'static WidgetTypeMapping>,
    /// Current toolkit mode
    mode: ToolkitType,
}

impl LocatorNormalizer {
    /// Create a new normalizer with default cache size
    pub fn new(mode: ToolkitType) -> Self {
        Self::with_cache_size(mode, DEFAULT_CACHE_SIZE)
    }

    /// Create a normalizer with custom cache size
    pub fn with_cache_size(mode: ToolkitType, cache_size: usize) -> Self {
        let cache_size = NonZeroUsize::new(cache_size).unwrap_or(NonZeroUsize::new(1).unwrap());

        // Build type lookup table
        let mut type_lookup = HashMap::new();
        for mapping in WIDGET_TYPE_MAPPINGS {
            // Add all variations for lookup
            type_lookup.insert(mapping.canonical_name.to_lowercase(), mapping);
            type_lookup.insert(mapping.swing_simple.to_lowercase(), mapping);
            type_lookup.insert(mapping.swt_simple.to_lowercase(), mapping);
            // Also add without J prefix for Swing
            if mapping.swing_simple.starts_with('J') {
                type_lookup.insert(mapping.swing_simple[1..].to_lowercase(), mapping);
            }
        }

        Self {
            cache: Mutex::new(LruCache::new(cache_size)),
            type_lookup,
            mode,
        }
    }

    /// Set the toolkit mode
    pub fn set_mode(&mut self, mode: ToolkitType) {
        self.mode = mode;
    }

    /// Get the current toolkit mode
    pub fn mode(&self) -> ToolkitType {
        self.mode
    }

    /// Normalize a locator string
    pub fn normalize(&self, locator: &str) -> Result<NormalizedLocator, LocatorParseError> {
        // Check cache first
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(locator) {
                return Ok(cached.clone());
            }
        }

        // Parse and normalize
        let normalized = self.parse_and_normalize(locator)?;

        // Store in cache
        {
            let mut cache = self.cache.lock().unwrap();
            cache.put(locator.to_string(), normalized.clone());
        }

        Ok(normalized)
    }

    /// Parse and normalize a locator
    fn parse_and_normalize(&self, locator: &str) -> Result<NormalizedLocator, LocatorParseError> {
        let trimmed = locator.trim();

        if trimmed.is_empty() {
            return Err(LocatorParseError::new("Locator cannot be empty"));
        }

        // 1. Check for XPath (starts with // or /)
        if trimmed.starts_with("//") || (trimmed.starts_with('/') && !trimmed.starts_with("/:")) {
            return self.parse_xpath(trimmed);
        }

        // 2. Check for prefix-based locators (name:, text:, class:, etc.)
        if let Some(result) = self.parse_prefix_locator(trimmed)? {
            return Ok(result);
        }

        // 3. Check for ID shorthand (#id)
        if trimmed.starts_with('#') {
            let id = &trimmed[1..];
            if id.is_empty() {
                return Err(LocatorParseError::new("Empty ID in # locator"));
            }
            return Ok(NormalizedLocator::new(
                LocatorType::Name,
                id.to_string(),
            ).with_toolkit(self.mode));
        }

        // 4. Check for Eclipse-style ID (contains dots like org.eclipse.ui.views.X)
        if self.is_eclipse_id(trimmed) {
            return Ok(NormalizedLocator::new(
                LocatorType::Toolkit {
                    toolkit: "rcp".to_string(),
                    selector: trimmed.to_string(),
                },
                trimmed.to_string(),
            ));
        }

        // 5. Parse as CSS-like selector
        self.parse_css_selector(trimmed)
    }

    /// Parse prefix-style locator (name:value, text:value, etc.)
    fn parse_prefix_locator(&self, locator: &str) -> Result<Option<NormalizedLocator>, LocatorParseError> {
        // Check for colon separator
        let colon_pos = match locator.find(':') {
            Some(pos) => pos,
            None => return Ok(None),
        };

        let prefix = &locator[..colon_pos].to_lowercase();
        let value = &locator[colon_pos + 1..];

        // Skip if value contains another colon at start (could be XPath-like)
        if value.starts_with('/') {
            return Ok(None);
        }

        match prefix.as_str() {
            "name" | "id" => {
                if value.is_empty() {
                    return Err(LocatorParseError::new("Empty value in name: locator"));
                }
                Ok(Some(NormalizedLocator::new(
                    LocatorType::Name,
                    value.to_string(),
                ).with_toolkit(self.mode)))
            }
            "text" => {
                if value.is_empty() {
                    return Err(LocatorParseError::new("Empty value in text: locator"));
                }
                // Check for match mode prefixes
                if value.starts_with('*') {
                    Ok(Some(NormalizedLocator::new(
                        LocatorType::TextContains,
                        value[1..].to_string(),
                    ).with_toolkit(self.mode)))
                } else if value.starts_with('~') {
                    Ok(Some(NormalizedLocator::new(
                        LocatorType::TextRegex,
                        value[1..].to_string(),
                    ).with_toolkit(self.mode)))
                } else {
                    Ok(Some(NormalizedLocator::new(
                        LocatorType::Text,
                        value.to_string(),
                    ).with_toolkit(self.mode)))
                }
            }
            "class" => {
                if value.is_empty() {
                    return Err(LocatorParseError::new("Empty value in class: locator"));
                }
                let normalized_class = self.normalize_class_name(value);
                Ok(Some(NormalizedLocator::new(
                    LocatorType::Class,
                    normalized_class,
                ).with_toolkit(self.mode)))
            }
            "index" => {
                let idx: usize = value.parse().map_err(|_| {
                    LocatorParseError::new(format!("Invalid index value '{}': expected non-negative integer", value))
                })?;
                Ok(Some(NormalizedLocator::new(
                    LocatorType::Index,
                    idx.to_string(),
                ).with_toolkit(self.mode)))
            }
            "tooltip" => {
                if value.is_empty() {
                    return Err(LocatorParseError::new("Empty value in tooltip: locator"));
                }
                Ok(Some(NormalizedLocator::new(
                    LocatorType::Tooltip,
                    value.to_string(),
                ).with_toolkit(self.mode)))
            }
            "accessible" => {
                if value.is_empty() {
                    return Err(LocatorParseError::new("Empty value in accessible: locator"));
                }
                Ok(Some(NormalizedLocator::new(
                    LocatorType::AccessibleName,
                    value.to_string(),
                ).with_toolkit(self.mode)))
            }
            "swing" => {
                Ok(Some(NormalizedLocator::new(
                    LocatorType::Toolkit {
                        toolkit: "swing".to_string(),
                        selector: value.to_string(),
                    },
                    value.to_string(),
                ).with_toolkit(ToolkitType::Swing)))
            }
            "swt" => {
                Ok(Some(NormalizedLocator::new(
                    LocatorType::Toolkit {
                        toolkit: "swt".to_string(),
                        selector: value.to_string(),
                    },
                    value.to_string(),
                ).with_toolkit(ToolkitType::Swt)))
            }
            "rcp" => {
                Ok(Some(NormalizedLocator::new(
                    LocatorType::Toolkit {
                        toolkit: "rcp".to_string(),
                        selector: value.to_string(),
                    },
                    value.to_string(),
                ).with_toolkit(ToolkitType::Rcp)))
            }
            "view" | "editor" | "perspective" | "menu" => {
                // Eclipse RCP specific selectors
                Ok(Some(NormalizedLocator::new(
                    LocatorType::Toolkit {
                        toolkit: format!("rcp:{}", prefix),
                        selector: value.to_string(),
                    },
                    value.to_string(),
                ).with_toolkit(ToolkitType::Rcp)))
            }
            _ => Ok(None), // Not a recognized prefix, try other parsing
        }
    }

    /// Parse CSS-style selector
    fn parse_css_selector(&self, locator: &str) -> Result<NormalizedLocator, LocatorParseError> {
        let mut remaining = locator;
        let mut predicates = Vec::new();
        let mut type_name = String::new();

        // Parse type selector (before any [ or :)
        let type_end = remaining.find(|c: char| c == '[' || c == ':' || c == '#' || c.is_whitespace())
            .unwrap_or(remaining.len());

        if type_end > 0 {
            type_name = remaining[..type_end].to_string();
            remaining = &remaining[type_end..];
        }

        // Parse ID selector (#id)
        if remaining.starts_with('#') {
            let id_end = remaining[1..].find(|c: char| c == '[' || c == ':' || c.is_whitespace())
                .map(|i| i + 1)
                .unwrap_or(remaining.len());

            let id = &remaining[1..id_end];
            predicates.push(LocatorPredicate::Attribute {
                name: "name".to_string(),
                op: MatchOp::Equals,
                value: id.to_string(),
            });
            remaining = &remaining[id_end..];
        }

        // Parse attribute selectors [attr=value]
        while remaining.starts_with('[') {
            let end = remaining.find(']').ok_or_else(|| {
                LocatorParseError::new("Missing ']' in attribute selector")
            })?;

            let attr_content = &remaining[1..end];
            if let Some(pred) = self.parse_attribute_predicate(attr_content)? {
                predicates.push(pred);
            }

            remaining = &remaining[end + 1..];
        }

        // Parse pseudo-classes :pseudo
        while remaining.starts_with(':') {
            let pseudo_end = remaining[1..].find(|c: char| c == ':' || c == '[' || c.is_whitespace())
                .map(|i| i + 1)
                .unwrap_or(remaining.len());

            let pseudo_str = &remaining[1..pseudo_end];

            // Check for functional pseudo-class like :nth-child(2)
            if pseudo_str.contains('(') {
                if let Some(paren_start) = pseudo_str.find('(') {
                    if let Some(paren_end) = pseudo_str.find(')') {
                        let pseudo_name = &pseudo_str[..paren_start];
                        let expr = &pseudo_str[paren_start + 1..paren_end];

                        if let Some(pseudo) = PseudoClass::parse_nth(pseudo_name, expr) {
                            predicates.push(LocatorPredicate::PseudoClass(pseudo));
                        }
                    }
                }
            } else if let Some(pseudo) = PseudoClass::parse(pseudo_str) {
                predicates.push(LocatorPredicate::PseudoClass(pseudo));
            }

            remaining = &remaining[pseudo_end..];
        }

        // Determine locator type and normalize class name
        let (locator_type, normalized_value) = if type_name.is_empty() {
            // Attribute-only selector
            if predicates.is_empty() {
                return Err(LocatorParseError::new("Empty locator: no type, attributes, or pseudo-classes"));
            }
            (LocatorType::Css, String::new())
        } else {
            let normalized = self.normalize_class_name(&type_name);
            (LocatorType::Css, normalized)
        };

        let mut result = NormalizedLocator::new(locator_type, normalized_value);
        result.predicates = predicates;
        result.toolkit = Some(self.mode);

        Ok(result)
    }

    /// Parse attribute predicate from [attr=value] content
    fn parse_attribute_predicate(&self, content: &str) -> Result<Option<LocatorPredicate>, LocatorParseError> {
        // Try different operators in order of specificity
        for (op_str, op) in &[
            ("!=", MatchOp::NotEquals),
            ("*=", MatchOp::Contains),
            ("^=", MatchOp::StartsWith),
            ("$=", MatchOp::EndsWith),
            ("~=", MatchOp::Regex),
            ("|=", MatchOp::WordMatch),
            ("=", MatchOp::Equals),
        ] {
            if let Some(pos) = content.find(op_str) {
                let name = content[..pos].trim().to_string();
                let value = content[pos + op_str.len()..]
                    .trim()
                    .trim_matches(|c| c == '"' || c == '\'')
                    .to_string();

                return Ok(Some(LocatorPredicate::Attribute {
                    name,
                    op: *op,
                    value,
                }));
            }
        }

        // Just attribute existence check [attr]
        if !content.is_empty() {
            Ok(Some(LocatorPredicate::Attribute {
                name: content.trim().to_string(),
                op: MatchOp::Equals,
                value: String::new(),
            }))
        } else {
            Ok(None)
        }
    }

    /// Parse XPath expression
    fn parse_xpath(&self, xpath: &str) -> Result<NormalizedLocator, LocatorParseError> {
        Ok(NormalizedLocator::new(
            LocatorType::XPath,
            xpath.to_string(),
        ).with_toolkit(self.mode))
    }

    /// Check if string looks like an Eclipse plugin ID
    fn is_eclipse_id(&self, s: &str) -> bool {
        // Eclipse IDs typically have multiple dot-separated parts
        // and start with common prefixes like org., com., etc.
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() >= 3 {
            let first = parts[0].to_lowercase();
            return first == "org" || first == "com" || first == "net" || first == "eclipse";
        }
        false
    }

    /// Normalize class name for the current toolkit
    pub fn normalize_class_name(&self, class_name: &str) -> String {
        let lookup_key = class_name.to_lowercase();

        // Look up in type mappings
        if let Some(mapping) = self.type_lookup.get(&lookup_key) {
            return match self.mode {
                ToolkitType::Swing => mapping.swing_simple.to_string(),
                ToolkitType::Swt | ToolkitType::Rcp => mapping.swt_simple.to_string(),
            };
        }

        // If not found, apply simple transformations
        match self.mode {
            ToolkitType::Swing => {
                // Add J prefix if not present for common types
                if !class_name.starts_with('J') && !class_name.contains('.') {
                    format!("J{}", class_name)
                } else {
                    class_name.to_string()
                }
            }
            ToolkitType::Swt | ToolkitType::Rcp => {
                // Remove J prefix if present
                if class_name.starts_with('J') && class_name.len() > 1 {
                    class_name[1..].to_string()
                } else {
                    class_name.to_string()
                }
            }
        }
    }

    /// Map a widget type name to the canonical form
    pub fn map_type(&self, type_name: &str) -> String {
        let lookup_key = type_name.to_lowercase();

        if let Some(mapping) = self.type_lookup.get(&lookup_key) {
            mapping.canonical_name.to_string()
        } else {
            type_name.to_string()
        }
    }

    /// Get suggestions for a failed locator
    pub fn suggest_corrections(&self, locator: &str, _error: &LocatorParseError) -> Vec<String> {
        let mut suggestions = Vec::new();

        // Check for Swing-specific types in SWT mode
        if matches!(self.mode, ToolkitType::Swt | ToolkitType::Rcp) {
            if locator.contains("JButton") {
                suggestions.push("In SWT mode, use 'Button' instead of 'JButton'".to_string());
            }
            if locator.contains("JTextField") {
                suggestions.push("In SWT mode, use 'Text' instead of 'JTextField'".to_string());
            }
            if locator.contains("JLabel") {
                suggestions.push("In SWT mode, use 'Label' instead of 'JLabel'".to_string());
            }
        }

        // Check for SWT prefix format in Swing mode
        if self.mode == ToolkitType::Swing {
            if locator.starts_with("name:") {
                let value = &locator[5..];
                suggestions.push(format!("CSS-style syntax recommended: [name='{}']", value));
            }
            if locator.starts_with("text:") {
                let value = &locator[5..];
                suggestions.push(format!("CSS-style syntax recommended: [text='{}']", value));
            }
        }

        // Check for common typos
        let lower = locator.to_lowercase();
        if lower.contains("textfield") && !lower.contains("jtextfield") {
            suggestions.push("Did you mean 'TextField' (capital F)?".to_string());
        }
        if lower.contains("checkbox") && !lower.contains("jcheckbox") {
            suggestions.push("Did you mean 'CheckBox' (capital B)?".to_string());
        }
        if lower.contains("combobox") && !lower.contains("jcombobox") {
            suggestions.push("Did you mean 'ComboBox' (capital B)?".to_string());
        }

        // Check for missing quotes in attributes
        if locator.contains('[') && locator.contains('=') {
            if !locator.contains('\'') && !locator.contains('"') {
                suggestions.push("Attribute values should be quoted: [attr='value']".to_string());
            }
        }

        suggestions
    }

    /// Clear the cache
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        let cache = self.cache.lock().unwrap();
        (cache.len(), cache.cap().get())
    }
}

impl Default for LocatorNormalizer {
    fn default() -> Self {
        Self::new(ToolkitType::Swing)
    }
}

// =============================================================================
// UnifiedLocator Implementation
// =============================================================================

impl UnifiedLocator {
    /// Parse a locator string into a UnifiedLocator
    ///
    /// Supported formats:
    /// - `name:buttonName` - by component name
    /// - `#buttonName` - shorthand for name
    /// - `text:Click Me` - by text content
    /// - `class:JButton` - by class name
    /// - `index:0` - by index
    /// - `id:12345` - by hash code
    /// - `swing:JButton[text="Save"]` - toolkit-specific
    /// - `JButton[text="Save"]` - CSS-like selector
    /// - `//JButton[@text='Save']` - XPath
    pub fn parse(locator: &str) -> Result<Self, LocatorParseError> {
        let locator = locator.trim();

        if locator.is_empty() {
            return Err(LocatorParseError::new("Locator cannot be empty"));
        }

        // Handle explicit type:value format (but not pseudo-classes like Type:enabled)
        if let Some((type_part, value)) = locator.split_once(':') {
            // Check if this looks like a pseudo-class (Type:pseudo-class)
            let is_pseudo_class = PseudoClass::parse(value.split('(').next().unwrap_or(value)).is_some()
                || value.starts_with("nth-child")
                || value.starts_with("nth-of-type");

            // Only treat as type:value if it's a known type prefix and NOT a pseudo-class
            if !is_pseudo_class {
                match type_part.to_lowercase().as_str() {
                    "name" => return Ok(Self::name(value)),
                    "text" => return Ok(Self::text(value)),
                    "class" => return Ok(Self::class(value)),
                    "index" => {
                        let idx = value
                            .parse()
                            .map_err(|_| LocatorParseError::new("Invalid index value"))?;
                        return Ok(Self::index(idx));
                    }
                    "id" => return Ok(Self::id(value)),
                    "tooltip" => return Ok(Self::tooltip(value)),
                    "accessible" => return Ok(Self::accessible(value)),
                    "swing" | "swt" | "rcp" => {
                        return Ok(Self::toolkit(type_part, value));
                    }
                    _ => {} // Fall through to other parsing
                }
            }
        }

        // Handle #name shorthand
        if locator.starts_with('#') {
            return Ok(Self::name(&locator[1..]));
        }

        // Handle XPath
        if locator.starts_with("//") || locator.starts_with("(//") {
            return Ok(Self::xpath(locator));
        }

        // Handle CSS-like selectors with attributes or pseudo-classes
        if locator.contains('[') || locator.contains(':') {
            return Self::parse_css_extended(locator);
        }

        // Default: treat as class name
        Ok(Self::class(locator))
    }

    /// Parse CSS-like selector with optional attributes and pseudo-classes
    fn parse_css_extended(locator: &str) -> Result<Self, LocatorParseError> {
        // Find the first special character ([ or :) to split type from predicates
        let special_pos = locator.find(|c: char| c == '[' || c == ':');

        let (class_name, rest) = match special_pos {
            Some(pos) => (&locator[..pos], &locator[pos..]),
            None => (locator, ""),
        };

        let mut result = Self {
            original: locator.to_string(),
            locator_type: LocatorType::Css,
            value: class_name.to_string(),
            predicates: vec![],
        };

        // Parse attributes and pseudo-classes
        let mut remaining = rest;
        while !remaining.is_empty() {
            if remaining.starts_with('[') {
                let end = remaining
                    .find(']')
                    .ok_or_else(|| LocatorParseError::new("Missing ']' in attribute selector"))?;

                let attr_content = &remaining[1..end];
                result.predicates.push(Self::parse_attribute(attr_content)?);
                remaining = &remaining[end + 1..];
            } else if remaining.starts_with(':') {
                // Parse pseudo-class
                let pseudo_end = remaining[1..]
                    .find(|c: char| c == ':' || c == '[')
                    .map(|i| i + 1)
                    .unwrap_or(remaining.len());

                let pseudo_str = &remaining[1..pseudo_end];

                // Handle functional pseudo-class like :nth-child(2)
                if pseudo_str.contains('(') {
                    if let (Some(paren_start), Some(paren_end)) = (pseudo_str.find('('), pseudo_str.find(')')) {
                        let pseudo_name = &pseudo_str[..paren_start];
                        let expr = &pseudo_str[paren_start + 1..paren_end];

                        if let Some(pseudo) = PseudoClass::parse_nth(pseudo_name, expr) {
                            result.predicates.push(LocatorPredicate::PseudoClass(pseudo));
                        } else if pseudo_name == "nth" || pseudo_name == "index" {
                            if let Ok(idx) = expr.parse::<usize>() {
                                result.predicates.push(LocatorPredicate::Index(idx));
                            }
                        }
                    }
                } else if let Some(pseudo) = PseudoClass::parse(pseudo_str) {
                    result.predicates.push(LocatorPredicate::PseudoClass(pseudo));
                }

                remaining = &remaining[pseudo_end..];
            } else {
                break;
            }
        }

        Ok(result)
    }

    /// Create name locator
    pub fn name(value: &str) -> Self {
        Self {
            original: format!("name:{}", value),
            locator_type: LocatorType::Name,
            value: value.to_string(),
            predicates: vec![],
        }
    }

    /// Create text locator
    pub fn text(value: &str) -> Self {
        Self {
            original: format!("text:{}", value),
            locator_type: LocatorType::Text,
            value: value.to_string(),
            predicates: vec![],
        }
    }

    /// Create class locator
    pub fn class(value: &str) -> Self {
        Self {
            original: value.to_string(),
            locator_type: LocatorType::Class,
            value: value.to_string(),
            predicates: vec![],
        }
    }

    /// Create index locator
    pub fn index(idx: usize) -> Self {
        Self {
            original: format!("index:{}", idx),
            locator_type: LocatorType::Index,
            value: idx.to_string(),
            predicates: vec![],
        }
    }

    /// Create id locator (by hash code)
    pub fn id(value: &str) -> Self {
        Self {
            original: format!("id:{}", value),
            locator_type: LocatorType::Id,
            value: value.to_string(),
            predicates: vec![],
        }
    }

    /// Create tooltip locator
    pub fn tooltip(value: &str) -> Self {
        Self {
            original: format!("tooltip:{}", value),
            locator_type: LocatorType::Tooltip,
            value: value.to_string(),
            predicates: vec![],
        }
    }

    /// Create accessible name locator
    pub fn accessible(value: &str) -> Self {
        Self {
            original: format!("accessible:{}", value),
            locator_type: LocatorType::AccessibleName,
            value: value.to_string(),
            predicates: vec![],
        }
    }

    /// Create XPath locator
    pub fn xpath(value: &str) -> Self {
        Self {
            original: value.to_string(),
            locator_type: LocatorType::XPath,
            value: value.to_string(),
            predicates: vec![],
        }
    }

    /// Create toolkit-specific locator
    pub fn toolkit(toolkit: &str, selector: &str) -> Self {
        Self {
            original: format!("{}:{}", toolkit, selector),
            locator_type: LocatorType::Toolkit {
                toolkit: toolkit.to_lowercase(),
                selector: selector.to_string(),
            },
            value: selector.to_string(),
            predicates: vec![],
        }
    }

    /// Parse CSS-like selector
    fn parse_css(locator: &str) -> Result<Self, LocatorParseError> {
        // Simple CSS parsing: Type[attr="value"]
        let bracket_pos = locator
            .find('[')
            .ok_or_else(|| LocatorParseError::new("Expected '[' in CSS selector"))?;

        let class_name = &locator[..bracket_pos];
        let attr_part = &locator[bracket_pos..];

        let mut result = Self {
            original: locator.to_string(),
            locator_type: LocatorType::Css,
            value: class_name.to_string(),
            predicates: vec![],
        };

        // Parse attributes: [attr="value"]
        let mut remaining = attr_part;
        while remaining.starts_with('[') {
            let end = remaining
                .find(']')
                .ok_or_else(|| LocatorParseError::new("Missing ']' in attribute selector"))?;

            let attr_content = &remaining[1..end];
            result.predicates.push(Self::parse_attribute(attr_content)?);

            remaining = &remaining[end + 1..];
        }

        // Parse pseudo-classes: :visible, :enabled
        while remaining.starts_with(':') {
            let pseudo_end = remaining[1..]
                .find(|c: char| !c.is_alphanumeric() && c != '-' && c != '(' && c != ')')
                .map(|i| i + 1)
                .unwrap_or(remaining.len());

            let pseudo_str = &remaining[1..pseudo_end];

            // Handle functional pseudo-class
            if pseudo_str.contains('(') {
                if let Some(paren_start) = pseudo_str.find('(') {
                    if let Some(paren_end) = pseudo_str.find(')') {
                        let pseudo_name = &pseudo_str[..paren_start];
                        let expr = &pseudo_str[paren_start + 1..paren_end];

                        if let Some(pseudo) = PseudoClass::parse_nth(pseudo_name, expr) {
                            result.predicates.push(LocatorPredicate::PseudoClass(pseudo));
                        } else if pseudo_name == "nth" || pseudo_name == "index" {
                            if let Ok(idx) = expr.parse::<usize>() {
                                result.predicates.push(LocatorPredicate::Index(idx));
                            }
                        }
                    }
                }
            } else if let Some(pseudo) = PseudoClass::parse(pseudo_str) {
                result.predicates.push(LocatorPredicate::PseudoClass(pseudo));
            }

            remaining = &remaining[pseudo_end..];
        }

        Ok(result)
    }

    /// Parse attribute predicate
    fn parse_attribute(content: &str) -> Result<LocatorPredicate, LocatorParseError> {
        // Try different operators
        for (op_str, op) in &[
            ("!=", MatchOp::NotEquals),
            ("*=", MatchOp::Contains),
            ("^=", MatchOp::StartsWith),
            ("$=", MatchOp::EndsWith),
            ("~=", MatchOp::Regex),
            ("|=", MatchOp::WordMatch),
            ("=", MatchOp::Equals),
        ] {
            if let Some(pos) = content.find(op_str) {
                let name = content[..pos].trim().to_string();
                let value = content[pos + op_str.len()..]
                    .trim()
                    .trim_matches(|c| c == '"' || c == '\'')
                    .to_string();

                return Ok(LocatorPredicate::Attribute {
                    name,
                    op: *op,
                    value,
                });
            }
        }

        Err(LocatorParseError::new(format!(
            "Invalid attribute selector: {}",
            content
        )))
    }

    /// Normalize locator for specific toolkit
    pub fn normalize_for_toolkit(&self, toolkit: ToolkitType) -> NormalizedLocator {
        NormalizedLocator {
            locator_type: self.locator_type.clone(),
            value: self.normalize_class_name(toolkit),
            predicates: self.predicates.clone(),
            toolkit: Some(toolkit),
            metadata: HashMap::new(),
        }
    }

    /// Normalize class name for toolkit
    fn normalize_class_name(&self, toolkit: ToolkitType) -> String {
        if !matches!(self.locator_type, LocatorType::Class | LocatorType::Css) {
            return self.value.clone();
        }

        match toolkit {
            ToolkitType::Swing => normalize_for_swing(&self.value),
            ToolkitType::Swt | ToolkitType::Rcp => normalize_for_swt(&self.value),
        }
    }

    /// Add a predicate to this locator
    pub fn with_predicate(mut self, predicate: LocatorPredicate) -> Self {
        self.predicates.push(predicate);
        self
    }

    /// Add attribute predicate
    pub fn with_attribute(self, name: &str, op: MatchOp, value: &str) -> Self {
        self.with_predicate(LocatorPredicate::Attribute {
            name: name.to_string(),
            op,
            value: value.to_string(),
        })
    }

    /// Add pseudo-class predicate
    pub fn with_pseudo_class(self, pseudo: PseudoClass) -> Self {
        self.with_predicate(LocatorPredicate::PseudoClass(pseudo))
    }

    /// Add index predicate
    pub fn with_index(self, index: usize) -> Self {
        self.with_predicate(LocatorPredicate::Index(index))
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Normalize class name for Swing toolkit
pub fn normalize_for_swing(value: &str) -> String {
    // Add J prefix if not present for common types
    if !value.starts_with('J') && !value.contains('.') {
        match value {
            "Button" => "JButton".to_string(),
            "TextField" | "Text" => "JTextField".to_string(),
            "TextArea" => "JTextArea".to_string(),
            "Label" => "JLabel".to_string(),
            "ComboBox" | "Combo" => "JComboBox".to_string(),
            "List" => "JList".to_string(),
            "Table" => "JTable".to_string(),
            "Tree" => "JTree".to_string(),
            "CheckBox" => "JCheckBox".to_string(),
            "RadioButton" => "JRadioButton".to_string(),
            "Panel" | "Composite" => "JPanel".to_string(),
            "Frame" | "Shell" | "Window" => "JFrame".to_string(),
            "Dialog" => "JDialog".to_string(),
            "ScrollPane" | "ScrolledComposite" => "JScrollPane".to_string(),
            "SplitPane" | "SashForm" => "JSplitPane".to_string(),
            "TabbedPane" | "TabFolder" => "JTabbedPane".to_string(),
            "MenuBar" => "JMenuBar".to_string(),
            "Menu" => "JMenu".to_string(),
            "MenuItem" => "JMenuItem".to_string(),
            "PopupMenu" => "JPopupMenu".to_string(),
            "ToolBar" => "JToolBar".to_string(),
            "Slider" | "Scale" => "JSlider".to_string(),
            "Spinner" => "JSpinner".to_string(),
            "ProgressBar" => "JProgressBar".to_string(),
            _ => value.to_string(),
        }
    } else {
        value.to_string()
    }
}

/// Normalize class name for SWT/RCP toolkit
pub fn normalize_for_swt(value: &str) -> String {
    // Remove J prefix for SWT
    if value.starts_with('J') && value.len() > 1 {
        let without_j = &value[1..];
        match without_j {
            "Button" => "Button".to_string(),
            "TextField" | "TextArea" | "TextPane" | "EditorPane" => "Text".to_string(),
            "Label" => "Label".to_string(),
            "ComboBox" => "Combo".to_string(),
            "List" => "List".to_string(),
            "Table" => "Table".to_string(),
            "Tree" => "Tree".to_string(),
            "CheckBox" => "Button".to_string(), // SWT uses Button with SWT.CHECK
            "RadioButton" => "Button".to_string(), // SWT uses Button with SWT.RADIO
            "Panel" => "Composite".to_string(),
            "Frame" | "Dialog" => "Shell".to_string(),
            "TabbedPane" => "TabFolder".to_string(),
            "SplitPane" => "SashForm".to_string(),
            "ScrollPane" => "ScrolledComposite".to_string(),
            "ProgressBar" => "ProgressBar".to_string(),
            "Slider" => "Slider".to_string(),
            "Spinner" => "Spinner".to_string(),
            "Menu" => "Menu".to_string(),
            "MenuBar" => "Menu".to_string(),
            "MenuItem" => "MenuItem".to_string(),
            "ToolBar" => "ToolBar".to_string(),
            _ => without_j.to_string(),
        }
    } else {
        value.to_string()
    }
}

// =============================================================================
// Locator Factory for RPC
// =============================================================================

/// Factory for creating toolkit-specific locator queries
pub struct LocatorFactory;

impl LocatorFactory {
    /// Convert unified locator to RPC parameters for Swing
    pub fn to_swing_params(locator: &UnifiedLocator) -> Value {
        match &locator.locator_type {
            LocatorType::Name => json!({
                "locatorType": "name",
                "value": locator.value
            }),
            LocatorType::Text => json!({
                "locatorType": "text",
                "value": locator.value
            }),
            LocatorType::TextContains => json!({
                "locatorType": "text",
                "value": locator.value,
                "matchMode": "contains"
            }),
            LocatorType::TextRegex => json!({
                "locatorType": "text",
                "value": locator.value,
                "matchMode": "regex"
            }),
            LocatorType::Class | LocatorType::Css => {
                let normalized = normalize_for_swing(&locator.value);
                let mut params = json!({
                    "locatorType": "class",
                    "value": normalized
                });

                // Add predicates
                if !locator.predicates.is_empty() {
                    params["predicates"] = Self::predicates_to_json(&locator.predicates);
                }

                params
            }
            LocatorType::Index => json!({
                "locatorType": "index",
                "value": locator.value.parse::<usize>().unwrap_or(0)
            }),
            LocatorType::Id => json!({
                "locatorType": "hashCode",
                "value": locator.value.parse::<i64>().unwrap_or(0)
            }),
            LocatorType::Tooltip => json!({
                "locatorType": "tooltip",
                "value": locator.value
            }),
            LocatorType::AccessibleName => json!({
                "locatorType": "accessible",
                "value": locator.value
            }),
            LocatorType::XPath => json!({
                "locatorType": "xpath",
                "xpath": locator.value
            }),
            LocatorType::Toolkit { selector, .. } => json!({
                "locator": selector
            }),
        }
    }

    /// Convert unified locator to RPC parameters for SWT
    pub fn to_swt_params(locator: &UnifiedLocator) -> Value {
        match &locator.locator_type {
            LocatorType::Name => json!({
                "locatorType": "name",
                "value": locator.value
            }),
            LocatorType::Text => json!({
                "locatorType": "text",
                "value": locator.value
            }),
            LocatorType::TextContains => json!({
                "locatorType": "text",
                "value": locator.value,
                "matchMode": "contains"
            }),
            LocatorType::TextRegex => json!({
                "locatorType": "text",
                "value": locator.value,
                "matchMode": "regex"
            }),
            LocatorType::Class | LocatorType::Css => {
                let normalized = normalize_for_swt(&locator.value);
                let mut params = json!({
                    "locatorType": "class",
                    "value": normalized
                });

                // Add predicates
                if !locator.predicates.is_empty() {
                    params["predicates"] = Self::predicates_to_json(&locator.predicates);
                }

                params
            }
            LocatorType::Index => json!({
                "locatorType": "index",
                "value": locator.value.parse::<usize>().unwrap_or(0)
            }),
            LocatorType::Id => json!({
                "locatorType": "id",
                "value": locator.value.parse::<i64>().unwrap_or(0)
            }),
            LocatorType::Tooltip => json!({
                "locatorType": "tooltip",
                "value": locator.value
            }),
            LocatorType::AccessibleName => json!({
                "locatorType": "accessible",
                "value": locator.value
            }),
            LocatorType::XPath => json!({
                "locatorType": "xpath",
                "xpath": locator.value
            }),
            LocatorType::Toolkit { selector, .. } => json!({
                "locator": selector
            }),
        }
    }

    /// Convert locator to RPC parameters for any toolkit
    pub fn to_params(locator: &UnifiedLocator, toolkit: ToolkitType) -> Value {
        match toolkit {
            ToolkitType::Swing => Self::to_swing_params(locator),
            ToolkitType::Swt | ToolkitType::Rcp => Self::to_swt_params(locator),
        }
    }

    /// Convert predicates to JSON
    fn predicates_to_json(predicates: &[LocatorPredicate]) -> Value {
        let json_predicates: Vec<Value> = predicates
            .iter()
            .map(|p| match p {
                LocatorPredicate::Attribute { name, op, value } => json!({
                    "type": "attribute",
                    "name": name,
                    "op": op.to_string(),
                    "value": value
                }),
                LocatorPredicate::PseudoClass(pseudo) => json!({
                    "type": "pseudo",
                    "value": pseudo.to_string()
                }),
                LocatorPredicate::Index(idx) => json!({
                    "type": "index",
                    "value": idx
                }),
                LocatorPredicate::Combinator(comb) => json!({
                    "type": "combinator",
                    "value": comb.to_string()
                }),
            })
            .collect();

        Value::Array(json_predicates)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // UnifiedLocator Tests
    // =========================================================================

    #[test]
    fn test_parse_name_locator() {
        let locator = UnifiedLocator::parse("name:myButton").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Name);
        assert_eq!(locator.value, "myButton");
    }

    #[test]
    fn test_parse_hash_shorthand() {
        let locator = UnifiedLocator::parse("#myButton").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Name);
        assert_eq!(locator.value, "myButton");
    }

    #[test]
    fn test_parse_text_locator() {
        let locator = UnifiedLocator::parse("text:Click Me").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Text);
        assert_eq!(locator.value, "Click Me");
    }

    #[test]
    fn test_parse_class_locator() {
        let locator = UnifiedLocator::parse("class:JButton").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Class);
        assert_eq!(locator.value, "JButton");
    }

    #[test]
    fn test_parse_default_class() {
        let locator = UnifiedLocator::parse("JButton").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Class);
        assert_eq!(locator.value, "JButton");
    }

    #[test]
    fn test_parse_xpath() {
        let locator = UnifiedLocator::parse("//JButton[@text='Save']").unwrap();
        assert_eq!(locator.locator_type, LocatorType::XPath);
        assert_eq!(locator.value, "//JButton[@text='Save']");
    }

    #[test]
    fn test_parse_css_with_attribute() {
        let locator = UnifiedLocator::parse("JButton[text=\"Save\"]").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Css);
        assert_eq!(locator.value, "JButton");
        assert_eq!(locator.predicates.len(), 1);

        if let LocatorPredicate::Attribute { name, op, value } = &locator.predicates[0] {
            assert_eq!(name, "text");
            assert_eq!(*op, MatchOp::Equals);
            assert_eq!(value, "Save");
        } else {
            panic!("Expected attribute predicate");
        }
    }

    #[test]
    fn test_parse_css_with_contains() {
        let locator = UnifiedLocator::parse("JButton[text*=\"Save\"]").unwrap();
        if let LocatorPredicate::Attribute { op, .. } = &locator.predicates[0] {
            assert_eq!(*op, MatchOp::Contains);
        }
    }

    #[test]
    fn test_parse_css_with_starts_with() {
        let locator = UnifiedLocator::parse("JButton[text^=\"Save\"]").unwrap();
        if let LocatorPredicate::Attribute { op, .. } = &locator.predicates[0] {
            assert_eq!(*op, MatchOp::StartsWith);
        }
    }

    #[test]
    fn test_parse_css_with_ends_with() {
        let locator = UnifiedLocator::parse("JButton[text$=\"Save\"]").unwrap();
        if let LocatorPredicate::Attribute { op, .. } = &locator.predicates[0] {
            assert_eq!(*op, MatchOp::EndsWith);
        }
    }

    #[test]
    fn test_parse_css_with_regex() {
        let locator = UnifiedLocator::parse("JButton[text~=\"Save.*\"]").unwrap();
        if let LocatorPredicate::Attribute { op, .. } = &locator.predicates[0] {
            assert_eq!(*op, MatchOp::Regex);
        }
    }

    #[test]
    fn test_parse_toolkit_specific() {
        let locator = UnifiedLocator::parse("swing:JButton").unwrap();
        if let LocatorType::Toolkit { toolkit, selector } = &locator.locator_type {
            assert_eq!(toolkit, "swing");
            assert_eq!(selector, "JButton");
        } else {
            panic!("Expected toolkit locator type");
        }
    }

    #[test]
    fn test_parse_tooltip_locator() {
        let locator = UnifiedLocator::parse("tooltip:Help text").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Tooltip);
        assert_eq!(locator.value, "Help text");
    }

    #[test]
    fn test_parse_accessible_locator() {
        let locator = UnifiedLocator::parse("accessible:Submit button").unwrap();
        assert_eq!(locator.locator_type, LocatorType::AccessibleName);
        assert_eq!(locator.value, "Submit button");
    }

    #[test]
    fn test_normalize_for_swing() {
        assert_eq!(normalize_for_swing("Button"), "JButton");
        assert_eq!(normalize_for_swing("TextField"), "JTextField");
        assert_eq!(normalize_for_swing("JButton"), "JButton");
        assert_eq!(normalize_for_swing("CustomWidget"), "CustomWidget");
        assert_eq!(normalize_for_swing("Text"), "JTextField");
        assert_eq!(normalize_for_swing("Composite"), "JPanel");
        assert_eq!(normalize_for_swing("Shell"), "JFrame");
    }

    #[test]
    fn test_normalize_for_swt() {
        assert_eq!(normalize_for_swt("JButton"), "Button");
        assert_eq!(normalize_for_swt("JTextField"), "Text");
        assert_eq!(normalize_for_swt("JTextArea"), "Text");
        assert_eq!(normalize_for_swt("JPanel"), "Composite");
        assert_eq!(normalize_for_swt("JFrame"), "Shell");
        assert_eq!(normalize_for_swt("JTabbedPane"), "TabFolder");
        assert_eq!(normalize_for_swt("Button"), "Button");
    }

    #[test]
    fn test_locator_factory_swing_params() {
        let locator = UnifiedLocator::name("testButton");
        let params = LocatorFactory::to_swing_params(&locator);

        assert_eq!(params["locatorType"], "name");
        assert_eq!(params["value"], "testButton");
    }

    #[test]
    fn test_locator_factory_class_normalization() {
        let locator = UnifiedLocator::class("Button");

        // For Swing, should become JButton
        let swing_params = LocatorFactory::to_swing_params(&locator);
        assert_eq!(swing_params["value"], "JButton");

        // For SWT, should stay Button
        let swt_params = LocatorFactory::to_swt_params(&locator);
        assert_eq!(swt_params["value"], "Button");
    }

    #[test]
    fn test_locator_with_predicates() {
        let locator = UnifiedLocator::class("JButton")
            .with_attribute("text", MatchOp::Equals, "Save")
            .with_pseudo_class(PseudoClass::Visible);

        assert_eq!(locator.predicates.len(), 2);
    }

    // =========================================================================
    // LocatorNormalizer Tests
    // =========================================================================

    #[test]
    fn test_normalizer_basic() {
        let normalizer = LocatorNormalizer::new(ToolkitType::Swing);

        let result = normalizer.normalize("name:testButton").unwrap();
        assert_eq!(result.locator_type, LocatorType::Name);
        assert_eq!(result.value, "testButton");
    }

    #[test]
    fn test_normalizer_caching() {
        let normalizer = LocatorNormalizer::new(ToolkitType::Swing);

        // First call parses
        let _result1 = normalizer.normalize("name:testButton").unwrap();
        let (size1, _) = normalizer.cache_stats();
        assert_eq!(size1, 1);

        // Second call hits cache
        let _result2 = normalizer.normalize("name:testButton").unwrap();
        let (size2, _) = normalizer.cache_stats();
        assert_eq!(size2, 1);
    }

    #[test]
    fn test_normalizer_class_normalization_swing() {
        let normalizer = LocatorNormalizer::new(ToolkitType::Swing);

        let result = normalizer.normalize("Button[text='Save']").unwrap();
        assert_eq!(result.value, "JButton");
    }

    #[test]
    fn test_normalizer_class_normalization_swt() {
        let normalizer = LocatorNormalizer::new(ToolkitType::Swt);

        let result = normalizer.normalize("JButton[text='Save']").unwrap();
        assert_eq!(result.value, "Button");
    }

    #[test]
    fn test_normalizer_xpath() {
        let normalizer = LocatorNormalizer::new(ToolkitType::Swing);

        let result = normalizer.normalize("//JButton[@name='submit']").unwrap();
        assert_eq!(result.locator_type, LocatorType::XPath);
    }

    #[test]
    fn test_normalizer_id_shorthand() {
        let normalizer = LocatorNormalizer::new(ToolkitType::Swing);

        let result = normalizer.normalize("#submitButton").unwrap();
        assert_eq!(result.locator_type, LocatorType::Name);
        assert_eq!(result.value, "submitButton");
    }

    #[test]
    fn test_normalizer_eclipse_id() {
        let normalizer = LocatorNormalizer::new(ToolkitType::Rcp);

        let result = normalizer.normalize("org.eclipse.ui.views.ProblemView").unwrap();
        if let LocatorType::Toolkit { toolkit, .. } = result.locator_type {
            assert_eq!(toolkit, "rcp");
        } else {
            panic!("Expected toolkit locator");
        }
    }

    #[test]
    fn test_normalizer_suggestions() {
        let normalizer = LocatorNormalizer::new(ToolkitType::Swt);
        let err = LocatorParseError::new("test");

        let suggestions = normalizer.suggest_corrections("JButton[text='Save']", &err);
        assert!(suggestions.iter().any(|s| s.contains("Button")));
    }

    // =========================================================================
    // PseudoClass Tests
    // =========================================================================

    #[test]
    fn test_pseudo_class_parsing() {
        assert_eq!(PseudoClass::parse("enabled"), Some(PseudoClass::Enabled));
        assert_eq!(PseudoClass::parse("disabled"), Some(PseudoClass::Disabled));
        assert_eq!(PseudoClass::parse("visible"), Some(PseudoClass::Visible));
        assert_eq!(PseudoClass::parse("hidden"), Some(PseudoClass::Hidden));
        assert_eq!(PseudoClass::parse("focused"), Some(PseudoClass::Focused));
        assert_eq!(PseudoClass::parse("focus"), Some(PseudoClass::Focused));
        assert_eq!(PseudoClass::parse("selected"), Some(PseudoClass::Selected));
        assert_eq!(PseudoClass::parse("checked"), Some(PseudoClass::Checked));
        assert_eq!(PseudoClass::parse("first"), Some(PseudoClass::First));
        assert_eq!(PseudoClass::parse("last"), Some(PseudoClass::Last));
        assert_eq!(PseudoClass::parse("unknown"), None);
    }

    #[test]
    fn test_nth_expr_parsing() {
        assert_eq!(NthExpr::parse("odd"), Some(NthExpr::Odd));
        assert_eq!(NthExpr::parse("even"), Some(NthExpr::Even));
        assert_eq!(NthExpr::parse("3"), Some(NthExpr::Index(3)));
        assert_eq!(NthExpr::parse("2n"), Some(NthExpr::Formula { a: 2, b: 0 }));
        assert_eq!(NthExpr::parse("2n+1"), Some(NthExpr::Formula { a: 2, b: 1 }));
    }

    #[test]
    fn test_nth_expr_matching() {
        assert!(NthExpr::Odd.matches(1));
        assert!(NthExpr::Odd.matches(3));
        assert!(!NthExpr::Odd.matches(2));

        assert!(NthExpr::Even.matches(2));
        assert!(NthExpr::Even.matches(4));
        assert!(!NthExpr::Even.matches(1));

        assert!(NthExpr::Index(3).matches(3));
        assert!(!NthExpr::Index(3).matches(2));

        let formula = NthExpr::Formula { a: 2, b: 1 };
        assert!(formula.matches(1)); // 2*0+1
        assert!(formula.matches(3)); // 2*1+1
        assert!(formula.matches(5)); // 2*2+1
        assert!(!formula.matches(2));
    }

    // =========================================================================
    // Widget Type Mapping Tests
    // =========================================================================

    #[test]
    fn test_widget_type_mappings_exist() {
        assert!(!WIDGET_TYPE_MAPPINGS.is_empty());

        // Check some key mappings
        let button = WIDGET_TYPE_MAPPINGS.iter().find(|m| m.canonical_name == "Button");
        assert!(button.is_some());
        let button = button.unwrap();
        assert_eq!(button.swing_simple, "JButton");
        assert_eq!(button.swt_simple, "Button");

        let text_field = WIDGET_TYPE_MAPPINGS.iter().find(|m| m.canonical_name == "TextField");
        assert!(text_field.is_some());
        let text_field = text_field.unwrap();
        assert_eq!(text_field.swing_simple, "JTextField");
        assert_eq!(text_field.swt_simple, "Text");
    }

    #[test]
    fn test_normalizer_type_lookup() {
        let normalizer = LocatorNormalizer::new(ToolkitType::Swing);

        // Test canonical -> swing
        assert_eq!(normalizer.normalize_class_name("Button"), "JButton");
        assert_eq!(normalizer.normalize_class_name("TextField"), "JTextField");
        assert_eq!(normalizer.normalize_class_name("ComboBox"), "JComboBox");

        let normalizer = LocatorNormalizer::new(ToolkitType::Swt);

        // Test swing -> swt
        assert_eq!(normalizer.normalize_class_name("JButton"), "Button");
        assert_eq!(normalizer.normalize_class_name("JTextField"), "Text");
        assert_eq!(normalizer.normalize_class_name("JComboBox"), "Combo");
    }

    #[test]
    fn test_map_type() {
        let normalizer = LocatorNormalizer::new(ToolkitType::Swing);

        assert_eq!(normalizer.map_type("JButton"), "Button");
        assert_eq!(normalizer.map_type("Button"), "Button");
        assert_eq!(normalizer.map_type("jbutton"), "Button");
        assert_eq!(normalizer.map_type("CustomWidget"), "CustomWidget");
    }

    // =========================================================================
    // CSS Selector Tests
    // =========================================================================

    #[test]
    fn test_css_selector_with_multiple_attributes() {
        let locator = UnifiedLocator::parse("JButton[name='submit'][text='OK']").unwrap();
        assert_eq!(locator.predicates.len(), 2);
    }

    #[test]
    fn test_css_selector_with_pseudo_class() {
        let locator = UnifiedLocator::parse("JButton:enabled").unwrap();
        assert_eq!(locator.predicates.len(), 1);
        if let LocatorPredicate::PseudoClass(pseudo) = &locator.predicates[0] {
            assert_eq!(*pseudo, PseudoClass::Enabled);
        }
    }

    #[test]
    fn test_css_selector_with_nth_child() {
        let locator = UnifiedLocator::parse("JButton:nth-child(2)").unwrap();
        assert_eq!(locator.predicates.len(), 1);
        if let LocatorPredicate::PseudoClass(PseudoClass::NthChild(expr)) = &locator.predicates[0] {
            assert_eq!(*expr, NthExpr::Index(2));
        }
    }

    #[test]
    fn test_css_selector_combined() {
        let locator = UnifiedLocator::parse("JButton[text='OK']:visible:first").unwrap();
        assert_eq!(locator.predicates.len(), 3);
    }

    // =========================================================================
    // Error Handling Tests
    // =========================================================================

    #[test]
    fn test_empty_locator_error() {
        let result = UnifiedLocator::parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_index_error() {
        let result = UnifiedLocator::parse("index:abc");
        assert!(result.is_err());
    }

    #[test]
    fn test_unclosed_bracket_error() {
        let result = UnifiedLocator::parse("JButton[text='Save'");
        assert!(result.is_err());
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_whitespace_handling() {
        let locator = UnifiedLocator::parse("  name:testButton  ").unwrap();
        assert_eq!(locator.value, "testButton");
    }

    #[test]
    fn test_case_insensitive_prefix() {
        let normalizer = LocatorNormalizer::new(ToolkitType::Swing);

        let result1 = normalizer.normalize("NAME:test").unwrap();
        let result2 = normalizer.normalize("name:test").unwrap();
        let result3 = normalizer.normalize("Name:test").unwrap();

        assert_eq!(result1.value, result2.value);
        assert_eq!(result2.value, result3.value);
    }

    #[test]
    fn test_attribute_with_single_quotes() {
        let locator = UnifiedLocator::parse("JButton[text='Save']").unwrap();
        if let LocatorPredicate::Attribute { value, .. } = &locator.predicates[0] {
            assert_eq!(value, "Save");
        }
    }

    #[test]
    fn test_attribute_with_double_quotes() {
        let locator = UnifiedLocator::parse("JButton[text=\"Save\"]").unwrap();
        if let LocatorPredicate::Attribute { value, .. } = &locator.predicates[0] {
            assert_eq!(value, "Save");
        }
    }
}
