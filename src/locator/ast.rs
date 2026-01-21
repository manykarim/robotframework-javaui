//! Abstract Syntax Tree for locator expressions
//!
//! This module defines the AST structures used by the parser and matcher.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Root locator type containing one or more selectors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Locator {
    /// List of complex selectors (comma-separated in CSS)
    pub selectors: Vec<ComplexSelector>,
    /// Original input string
    pub original: String,
    /// Whether this is XPath-style
    pub is_xpath: bool,
}

impl Locator {
    /// Create a new locator from parsed selectors
    pub fn new(selectors: Vec<ComplexSelector>, original: String, is_xpath: bool) -> Self {
        Self {
            selectors,
            original,
            is_xpath,
        }
    }

    /// Create a simple type selector locator
    pub fn type_selector(type_name: impl Into<String>) -> Self {
        let name = type_name.into();
        Self {
            selectors: vec![ComplexSelector::simple(CompoundSelector {
                type_selector: Some(TypeSelector::TypeName(name.clone())),
                ..Default::default()
            })],
            original: name,
            is_xpath: false,
        }
    }

    /// Check if locator is empty
    pub fn is_empty(&self) -> bool {
        self.selectors.is_empty()
    }

    /// Check if this is a universal selector
    pub fn is_universal(&self) -> bool {
        self.selectors.len() == 1
            && self.selectors[0].compounds.len() == 1
            && matches!(
                self.selectors[0].compounds[0].type_selector,
                Some(TypeSelector::Universal)
            )
    }
}

impl fmt::Display for Locator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.original)
    }
}

/// A single segment in a cascaded locator chain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CascadedSegment {
    /// Whether this segment should be captured (marked with *)
    pub capture: bool,

    /// The compound selector for this segment
    pub compound: CompoundSelector,

    /// Combinator to next segment (if any)
    pub combinator: Option<Combinator>,

    /// Original raw text of this segment (for error messages)
    pub raw: String,
}

impl CascadedSegment {
    /// Create a new cascaded segment
    pub fn new(
        capture: bool,
        compound: CompoundSelector,
        combinator: Option<Combinator>,
        raw: String,
    ) -> Self {
        Self {
            capture,
            compound,
            combinator,
            raw,
        }
    }

    /// Check if this segment has the capture flag
    pub fn is_captured(&self) -> bool {
        self.capture
    }
}

/// Complex selector: chain of compound selectors connected by combinators
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSelector {
    /// All compound selectors in this chain
    pub compounds: Vec<CompoundSelector>,

    /// For cascaded selectors: structured segments with capture flags
    pub cascaded_segments: Option<Vec<CascadedSegment>>,
}

impl ComplexSelector {
    /// Create a simple complex selector with one compound
    pub fn simple(compound: CompoundSelector) -> Self {
        Self {
            compounds: vec![compound],
            cascaded_segments: None,
        }
    }

    /// Create from a list of compounds
    pub fn from_compounds(compounds: Vec<CompoundSelector>) -> Self {
        Self {
            compounds,
            cascaded_segments: None,
        }
    }

    /// Check if this is a cascaded selector (contains >> combinator)
    pub fn is_cascaded(&self) -> bool {
        self.compounds
            .iter()
            .any(|c| matches!(c.combinator, Some(Combinator::Cascaded)))
    }

    /// Get cascaded segments if this is a cascaded selector
    pub fn get_cascaded_segments(&self) -> Option<&Vec<CascadedSegment>> {
        self.cascaded_segments.as_ref()
    }

    /// Check if any segment has capture flag
    pub fn has_capture(&self) -> bool {
        self.cascaded_segments
            .as_ref()
            .map(|segs| segs.iter().any(|s| s.capture))
            .unwrap_or(false)
    }

    /// Get index of first captured segment (0-based)
    pub fn get_capture_index(&self) -> Option<usize> {
        self.cascaded_segments
            .as_ref()
            .and_then(|segs| segs.iter().position(|s| s.capture))
    }
}

/// Combinator types connecting compound selectors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Combinator {
    /// Whitespace - descendant
    Descendant,
    /// > - child
    Child,
    /// + - adjacent sibling
    AdjacentSibling,
    /// ~ - general sibling
    GeneralSibling,
    /// >> - cascaded (Browser Library style: find within parent context)
    Cascaded,
}

impl fmt::Display for Combinator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Combinator::Descendant => write!(f, " "),
            Combinator::Child => write!(f, " > "),
            Combinator::AdjacentSibling => write!(f, " + "),
            Combinator::GeneralSibling => write!(f, " ~ "),
            Combinator::Cascaded => write!(f, " >> "),
        }
    }
}

/// Compound selector: type + modifiers without combinators
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CompoundSelector {
    /// Type selector (element name or *)
    pub type_selector: Option<TypeSelector>,
    /// ID selector (#id)
    pub id_selector: Option<String>,
    /// Class selectors (.class)
    pub class_selectors: Vec<String>,
    /// Attribute selectors ([attr=value])
    pub attribute_selectors: Vec<AttributeSelector>,
    /// Pseudo selectors (:pseudo)
    pub pseudo_selectors: Vec<PseudoSelector>,
    /// Combinator to next compound (set during parsing)
    pub combinator: Option<Combinator>,
}

impl CompoundSelector {
    /// Create an empty compound selector
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if this selector matches anything (has at least one condition)
    pub fn is_empty(&self) -> bool {
        self.type_selector.is_none()
            && self.id_selector.is_none()
            && self.class_selectors.is_empty()
            && self.attribute_selectors.is_empty()
            && self.pseudo_selectors.is_empty()
    }
}

impl fmt::Display for CompoundSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref ts) = self.type_selector {
            write!(f, "{}", ts)?;
        }
        if let Some(ref id) = self.id_selector {
            write!(f, "#{}", id)?;
        }
        for class in &self.class_selectors {
            write!(f, ".{}", class)?;
        }
        for attr in &self.attribute_selectors {
            write!(f, "{}", attr)?;
        }
        for pseudo in &self.pseudo_selectors {
            write!(f, "{}", pseudo)?;
        }
        Ok(())
    }
}

/// Type selector: element type name, universal, or prefix-style selector
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeSelector {
    /// Named type (e.g., JButton)
    TypeName(String),
    /// Universal selector (*)
    Universal,
    /// Prefix-style selector (e.g., class=JButton, name=myButton)
    PrefixSelector { key: String, value: String },
}

impl fmt::Display for TypeSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeSelector::TypeName(name) => write!(f, "{}", name),
            TypeSelector::Universal => write!(f, "*"),
            TypeSelector::PrefixSelector { key, value } => write!(f, "{}={}", key, value),
        }
    }
}

/// Attribute selector: [attr], [attr=value], [attr*=value], etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeSelector {
    /// Attribute name
    pub name: String,
    /// Optional matcher (operator + value)
    pub matcher: Option<AttributeMatcher>,
}

impl AttributeSelector {
    /// Create an existence check selector [attr]
    pub fn exists(name: String) -> Self {
        Self { name, matcher: None }
    }

    /// Create an equality selector [attr=value]
    pub fn equals(name: String, value: AttributeValue) -> Self {
        Self {
            name,
            matcher: Some(AttributeMatcher {
                operator: MatchOperator::Equals,
                value,
            }),
        }
    }

    /// Create a selector with custom operator
    pub fn with_operator(name: String, operator: MatchOperator, value: AttributeValue) -> Self {
        Self {
            name,
            matcher: Some(AttributeMatcher { operator, value }),
        }
    }
}

impl fmt::Display for AttributeSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}", self.name)?;
        if let Some(ref matcher) = self.matcher {
            write!(f, "{}", matcher)?;
        }
        write!(f, "]")
    }
}

/// Attribute matcher: operator and value
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeMatcher {
    /// Comparison operator
    pub operator: MatchOperator,
    /// Value to compare against
    pub value: AttributeValue,
}

impl fmt::Display for AttributeMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.operator, self.value)
    }
}

/// Match operators for attribute comparison
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MatchOperator {
    /// = exact match
    Equals,
    /// != not equal
    NotEquals,
    /// ^= prefix match
    PrefixMatch,
    /// $= suffix match
    SuffixMatch,
    /// *= substring match
    SubstringMatch,
    /// ~= word match
    WordMatch,
    /// |= dash match
    DashMatch,
    /// /= regex match
    RegexMatch,
    /// < less than (numeric)
    LessThan,
    /// <= less or equal
    LessOrEqual,
    /// > greater than
    GreaterThan,
    /// >= greater or equal
    GreaterOrEqual,
}

impl fmt::Display for MatchOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MatchOperator::Equals => write!(f, "="),
            MatchOperator::NotEquals => write!(f, "!="),
            MatchOperator::PrefixMatch => write!(f, "^="),
            MatchOperator::SuffixMatch => write!(f, "$="),
            MatchOperator::SubstringMatch => write!(f, "*="),
            MatchOperator::WordMatch => write!(f, "~="),
            MatchOperator::DashMatch => write!(f, "|="),
            MatchOperator::RegexMatch => write!(f, "/="),
            MatchOperator::LessThan => write!(f, "<"),
            MatchOperator::LessOrEqual => write!(f, "<="),
            MatchOperator::GreaterThan => write!(f, ">"),
            MatchOperator::GreaterOrEqual => write!(f, ">="),
        }
    }
}

/// Attribute value types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeValue {
    /// String value
    String(String),
    /// Numeric value
    Number(f64),
}

impl AttributeValue {
    /// Get as string reference if string
    pub fn as_str(&self) -> Option<&str> {
        match self {
            AttributeValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get as number if numeric
    pub fn as_number(&self) -> Option<f64> {
        match self {
            AttributeValue::Number(n) => Some(*n),
            _ => None,
        }
    }
}

impl fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeValue::String(s) => write!(f, "'{}'", s),
            AttributeValue::Number(n) => write!(f, "{}", n),
        }
    }
}

/// Pseudo-class selectors
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PseudoSelector {
    // Structural pseudo-classes
    FirstChild,
    LastChild,
    NthChild(NthExpr),
    NthLastChild(NthExpr),
    OnlyChild,
    FirstOfType,
    LastOfType,
    NthOfType(NthExpr),
    NthLastOfType(NthExpr),
    OnlyOfType,
    Empty,
    Root,

    // UI state pseudo-classes
    Enabled,
    Disabled,
    Visible,
    Hidden,
    Showing,
    Focused,
    Selected,
    Editable,
    ReadOnly,

    // Functional pseudo-classes
    Not(Box<CompoundSelector>),
    Has(Box<CompoundSelector>),
    Contains(String),
}

impl fmt::Display for PseudoSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PseudoSelector::FirstChild => write!(f, ":first-child"),
            PseudoSelector::LastChild => write!(f, ":last-child"),
            PseudoSelector::NthChild(expr) => write!(f, ":nth-child({})", expr),
            PseudoSelector::NthLastChild(expr) => write!(f, ":nth-last-child({})", expr),
            PseudoSelector::OnlyChild => write!(f, ":only-child"),
            PseudoSelector::FirstOfType => write!(f, ":first-of-type"),
            PseudoSelector::LastOfType => write!(f, ":last-of-type"),
            PseudoSelector::NthOfType(expr) => write!(f, ":nth-of-type({})", expr),
            PseudoSelector::NthLastOfType(expr) => write!(f, ":nth-last-of-type({})", expr),
            PseudoSelector::OnlyOfType => write!(f, ":only-of-type"),
            PseudoSelector::Empty => write!(f, ":empty"),
            PseudoSelector::Root => write!(f, ":root"),
            PseudoSelector::Enabled => write!(f, ":enabled"),
            PseudoSelector::Disabled => write!(f, ":disabled"),
            PseudoSelector::Visible => write!(f, ":visible"),
            PseudoSelector::Hidden => write!(f, ":hidden"),
            PseudoSelector::Showing => write!(f, ":showing"),
            PseudoSelector::Focused => write!(f, ":focused"),
            PseudoSelector::Selected => write!(f, ":selected"),
            PseudoSelector::Editable => write!(f, ":editable"),
            PseudoSelector::ReadOnly => write!(f, ":readonly"),
            PseudoSelector::Not(sel) => write!(f, ":not({})", sel),
            PseudoSelector::Has(sel) => write!(f, ":has({})", sel),
            PseudoSelector::Contains(text) => write!(f, ":contains('{}')", text),
        }
    }
}

/// Nth expression for :nth-child, :nth-of-type, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// Check if a 1-based index matches this expression
    pub fn matches(&self, index: i32) -> bool {
        match self {
            NthExpr::Index(i) => index == *i,
            NthExpr::Odd => index % 2 == 1,
            NthExpr::Even => index % 2 == 0,
            NthExpr::Formula { a, b } => {
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
            NthExpr::Index(i) => write!(f, "{}", i),
            NthExpr::Odd => write!(f, "odd"),
            NthExpr::Even => write!(f, "even"),
            NthExpr::Formula { a, b } => {
                if *b >= 0 {
                    write!(f, "{}n+{}", a, b)
                } else {
                    write!(f, "{}n{}", a, b)
                }
            }
        }
    }
}
