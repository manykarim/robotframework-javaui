//! Locator expression types for high-level parsing API
//!
//! This module provides the high-level expression types used by the Python bindings
//! for simplified locator handling.

use super::ast::{Locator, CompoundSelector, AttributeSelector, TypeSelector, PseudoSelector as AstPseudoSelector};
use super::parser;
use std::fmt;

/// Error type for locator parsing
#[derive(Debug, Clone)]
pub struct LocatorParseError {
    pub message: String,
    pub position: Option<usize>,
}

impl LocatorParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            position: None,
        }
    }

    pub fn with_position(message: impl Into<String>, position: usize) -> Self {
        Self {
            message: message.into(),
            position: Some(position),
        }
    }
}

impl fmt::Display for LocatorParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(pos) = self.position {
            write!(f, "{} at position {}", self.message, pos)
        } else {
            write!(f, "{}", self.message)
        }
    }
}

impl std::error::Error for LocatorParseError {}

impl From<parser::ParseError> for LocatorParseError {
    fn from(err: parser::ParseError) -> Self {
        Self::new(err.to_string())
    }
}

/// High-level locator expression type
#[derive(Debug, Clone)]
pub enum LocatorExpression {
    /// Simple locator (e.g., "name:button", "text:Submit")
    Simple(SimpleLocator),
    /// CSS-style selector
    Css(CssSelector),
    /// XPath-style expression
    XPath(XPathExpression),
}

impl LocatorExpression {
    /// Parse a locator string into an expression
    pub fn parse(input: &str) -> Result<Self, LocatorParseError> {
        let trimmed = input.trim();

        // Check for simple locator syntax (prefix:value)
        if let Some(simple) = Self::try_parse_simple(trimmed) {
            return Ok(LocatorExpression::Simple(simple));
        }

        // Check for XPath syntax (starts with / or //)
        if trimmed.starts_with('/') {
            return Self::parse_xpath(trimmed);
        }

        // Default to CSS selector
        Self::parse_css(trimmed)
    }

    /// Try to parse as a simple locator
    fn try_parse_simple(input: &str) -> Option<SimpleLocator> {
        // Simple locators have the format "prefix:value"
        // But NOT if it looks like a pseudo-selector (:enabled, :visible, etc.)
        if input.starts_with(':') {
            return None;
        }

        // Must contain a colon and not have CSS-specific characters before it
        if let Some(colon_pos) = input.find(':') {
            let prefix = &input[..colon_pos];
            let value = &input[colon_pos + 1..];

            // Don't treat as simple if prefix contains CSS special chars
            if prefix.contains('[') || prefix.contains('.') || prefix.contains('#') || prefix.contains(' ') {
                return None;
            }

            // Check if it's a known simple locator prefix
            match prefix.to_lowercase().as_str() {
                "name" => Some(SimpleLocator::new(SimpleLocatorType::Name, value)),
                "internalname" | "internal_name" | "internal-name" => {
                    Some(SimpleLocator::new(SimpleLocatorType::InternalName, value))
                }
                "text" => Some(SimpleLocator::new(SimpleLocatorType::Text, value)),
                "tooltip" => Some(SimpleLocator::new(SimpleLocatorType::Tooltip, value)),
                "class" => Some(SimpleLocator::new(SimpleLocatorType::Class, value)),
                "index" => Some(SimpleLocator::new(SimpleLocatorType::Index, value)),
                "id" => Some(SimpleLocator::new(SimpleLocatorType::Id, value)),
                "label" => Some(SimpleLocator::new(SimpleLocatorType::Label, value)),
                "accessiblename" | "accessible_name" | "accessible-name" => {
                    Some(SimpleLocator::new(SimpleLocatorType::AccessibleName, value))
                }
                _ => None,
            }
        } else {
            None
        }
    }

    /// Parse as CSS selector
    fn parse_css(input: &str) -> Result<Self, LocatorParseError> {
        // Use the pest parser
        let locator = parser::parse_locator(input)?;
        Ok(LocatorExpression::Css(CssSelector::from_locator(&locator)))
    }

    /// Parse as XPath expression
    fn parse_xpath(input: &str) -> Result<Self, LocatorParseError> {
        // Use the pest parser
        let locator = parser::parse_locator(input)?;
        Ok(LocatorExpression::XPath(XPathExpression::from_locator(&locator)))
    }
}

/// Simple locator type (prefix:value format)
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleLocator {
    pub locator_type: SimpleLocatorType,
    pub value: String,
}

impl SimpleLocator {
    pub fn new(locator_type: SimpleLocatorType, value: impl Into<String>) -> Self {
        Self {
            locator_type,
            value: value.into(),
        }
    }
}

/// Simple locator type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleLocatorType {
    Name,
    InternalName,
    Text,
    Tooltip,
    Class,
    Index,
    Id,
    Label,
    AccessibleName,
}

/// CSS-style selector representation
#[derive(Debug, Clone)]
pub struct CssSelector {
    /// Segments connected by combinators
    pub segments: Vec<CssSelectorSegment>,
}

impl CssSelector {
    pub fn from_locator(locator: &Locator) -> Self {
        let mut segments = Vec::new();

        if let Some(first_selector) = locator.selectors.first() {
            for compound in &first_selector.compounds {
                segments.push(CssSelectorSegment::from_compound(compound));
            }
        }

        Self { segments }
    }
}

/// A segment of a CSS selector (between combinators)
#[derive(Debug, Clone)]
pub struct CssSelectorSegment {
    pub element: String,
    pub id: Option<String>,
    pub classes: Vec<String>,
    pub attributes: Vec<CssAttribute>,
    pub pseudos: Vec<PseudoSelector>,
}

impl CssSelectorSegment {
    pub fn from_compound(compound: &CompoundSelector) -> Self {
        let element = match &compound.type_selector {
            Some(TypeSelector::TypeName(name)) => name.clone(),
            Some(TypeSelector::Universal) => "*".to_string(),
            None => String::new(),
        };

        let attributes: Vec<CssAttribute> = compound.attribute_selectors
            .iter()
            .map(CssAttribute::from_ast)
            .collect();

        let pseudos: Vec<PseudoSelector> = compound.pseudo_selectors
            .iter()
            .map(PseudoSelector::from_ast)
            .collect();

        Self {
            element,
            id: compound.id_selector.clone(),
            classes: compound.class_selectors.clone(),
            attributes,
            pseudos,
        }
    }
}

/// CSS attribute selector
#[derive(Debug, Clone)]
pub struct CssAttribute {
    pub name: String,
    pub operator: Option<AttributeOperator>,
    pub value: Option<String>,
}

impl CssAttribute {
    pub fn from_ast(attr: &AttributeSelector) -> Self {
        let (operator, value) = if let Some(ref matcher) = attr.matcher {
            let op = AttributeOperator::from_match_operator(&matcher.operator);
            let val = match &matcher.value {
                super::ast::AttributeValue::String(s) => Some(s.clone()),
                super::ast::AttributeValue::Number(n) => Some(n.to_string()),
            };
            (Some(op), val)
        } else {
            (None, None)
        };

        Self {
            name: attr.name.clone(),
            operator,
            value,
        }
    }
}

/// Attribute comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeOperator {
    Equals,
    NotEquals,
    Contains,
    StartsWith,
    EndsWith,
    Matches,
}

impl AttributeOperator {
    pub fn from_match_operator(op: &super::ast::MatchOperator) -> Self {
        match op {
            super::ast::MatchOperator::Equals => Self::Equals,
            super::ast::MatchOperator::NotEquals => Self::NotEquals,
            super::ast::MatchOperator::SubstringMatch => Self::Contains,
            super::ast::MatchOperator::PrefixMatch => Self::StartsWith,
            super::ast::MatchOperator::SuffixMatch => Self::EndsWith,
            super::ast::MatchOperator::RegexMatch => Self::Matches,
            super::ast::MatchOperator::WordMatch => Self::Contains,
            super::ast::MatchOperator::DashMatch => Self::StartsWith,
            super::ast::MatchOperator::LessThan => Self::Equals,
            super::ast::MatchOperator::LessOrEqual => Self::Equals,
            super::ast::MatchOperator::GreaterThan => Self::Equals,
            super::ast::MatchOperator::GreaterOrEqual => Self::Equals,
        }
    }
}

/// Pseudo-class selector for CSS matching
#[derive(Debug, Clone)]
pub enum PseudoSelector {
    FirstChild,
    LastChild,
    NthChild(i32),
    OnlyChild,
    FirstOfType,
    LastOfType,
    OnlyOfType,
    Empty,
    Root,
    Enabled,
    Disabled,
    Visible,
    Hidden,
    Showing,
    Focus,
    Selected,
    Editable,
    ReadOnly,
    Not(Box<CssSelectorSegment>),
    Has(Box<CssSelectorSegment>),
    Contains(String),
}

impl PseudoSelector {
    pub fn from_ast(pseudo: &AstPseudoSelector) -> Self {
        match pseudo {
            AstPseudoSelector::FirstChild => Self::FirstChild,
            AstPseudoSelector::LastChild => Self::LastChild,
            AstPseudoSelector::NthChild(expr) => {
                match expr {
                    super::ast::NthExpr::Index(i) => Self::NthChild(*i),
                    super::ast::NthExpr::Odd => Self::NthChild(1),
                    super::ast::NthExpr::Even => Self::NthChild(2),
                    super::ast::NthExpr::Formula { a, b } => Self::NthChild(*b),
                }
            }
            AstPseudoSelector::NthLastChild(expr) => {
                match expr {
                    super::ast::NthExpr::Index(i) => Self::NthChild(-*i),
                    _ => Self::NthChild(-1),
                }
            }
            AstPseudoSelector::OnlyChild => Self::OnlyChild,
            AstPseudoSelector::FirstOfType => Self::FirstOfType,
            AstPseudoSelector::LastOfType => Self::LastOfType,
            AstPseudoSelector::NthOfType(_) => Self::FirstOfType,
            AstPseudoSelector::NthLastOfType(_) => Self::LastOfType,
            AstPseudoSelector::OnlyOfType => Self::OnlyOfType,
            AstPseudoSelector::Empty => Self::Empty,
            AstPseudoSelector::Root => Self::Root,
            AstPseudoSelector::Enabled => Self::Enabled,
            AstPseudoSelector::Disabled => Self::Disabled,
            AstPseudoSelector::Visible => Self::Visible,
            AstPseudoSelector::Hidden => Self::Hidden,
            AstPseudoSelector::Showing => Self::Showing,
            AstPseudoSelector::Focused => Self::Focus,
            AstPseudoSelector::Selected => Self::Selected,
            AstPseudoSelector::Editable => Self::Editable,
            AstPseudoSelector::ReadOnly => Self::ReadOnly,
            AstPseudoSelector::Not(inner) => {
                Self::Not(Box::new(CssSelectorSegment::from_compound(inner)))
            }
            AstPseudoSelector::Has(inner) => {
                Self::Has(Box::new(CssSelectorSegment::from_compound(inner)))
            }
            AstPseudoSelector::Contains(text) => Self::Contains(text.clone()),
        }
    }
}

/// XPath-style expression representation
#[derive(Debug, Clone)]
pub struct XPathExpression {
    /// Steps in the XPath
    pub steps: Vec<XPathStep>,
    /// Whether it's absolute (starts with /)
    pub absolute: bool,
    /// Whether it's a descendant search (//)
    pub descendant_search: bool,
}

impl XPathExpression {
    pub fn from_locator(locator: &Locator) -> Self {
        let mut steps = Vec::new();
        let is_descendant_search = locator.original.starts_with("//");

        if let Some(first_selector) = locator.selectors.first() {
            for (i, compound) in first_selector.compounds.iter().enumerate() {
                // For the first step in a // xpath, use Descendant axis
                // For subsequent steps, check the combinator from the previous compound
                let axis = if i == 0 && is_descendant_search {
                    XPathAxis::Descendant
                } else if i > 0 {
                    // Get combinator from previous compound
                    if let Some(prev) = first_selector.compounds.get(i - 1) {
                        match prev.combinator {
                            Some(super::ast::Combinator::Descendant) => XPathAxis::Descendant,
                            Some(super::ast::Combinator::Child) => XPathAxis::Child,
                            _ => XPathAxis::Descendant, // Default to descendant for XPath-style
                        }
                    } else {
                        XPathAxis::Child
                    }
                } else {
                    XPathAxis::Child
                };
                steps.push(XPathStep::from_compound_with_axis(compound, axis));
            }
        }

        Self {
            steps,
            absolute: locator.is_xpath,
            descendant_search: is_descendant_search,
        }
    }
}

/// A step in an XPath expression
#[derive(Debug, Clone)]
pub struct XPathStep {
    pub axis: XPathAxis,
    pub node_test: String,
    pub predicates: Vec<XPathPredicate>,
}

impl XPathStep {
    pub fn from_compound(compound: &CompoundSelector) -> Self {
        Self::from_compound_with_axis(compound, XPathAxis::Child)
    }

    pub fn from_compound_with_axis(compound: &CompoundSelector, axis: XPathAxis) -> Self {
        let node_test = match &compound.type_selector {
            Some(TypeSelector::TypeName(name)) => name.clone(),
            Some(TypeSelector::Universal) => "*".to_string(),
            None => "*".to_string(),
        };

        let mut predicates = Vec::new();

        // Convert attribute selectors to predicates
        for attr in &compound.attribute_selectors {
            if let Some(ref matcher) = attr.matcher {
                let value = match &matcher.value {
                    super::ast::AttributeValue::String(s) => s.clone(),
                    super::ast::AttributeValue::Number(n) => n.to_string(),
                };
                predicates.push(XPathPredicate::AttributeEquals(attr.name.clone(), value));
            } else {
                predicates.push(XPathPredicate::AttributeExists(attr.name.clone()));
            }
        }

        Self {
            axis,
            node_test,
            predicates,
        }
    }
}

/// XPath axis types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XPathAxis {
    Child,
    Descendant,
    Parent,
    Ancestor,
    Following,
    Preceding,
    FollowingSibling,
    PrecedingSibling,
    Self_,
    DescendantOrSelf,
    AncestorOrSelf,
}

/// XPath predicate types
#[derive(Debug, Clone)]
pub enum XPathPredicate {
    /// Attribute exists check [@attr]
    AttributeExists(String),
    /// Attribute equals check [@attr='value']
    AttributeEquals(String, String),
    /// Contains function [contains(@attr, 'value')]
    Contains(String, String),
    /// Starts-with function [starts-with(@attr, 'value')]
    StartsWith(String, String),
    /// Numeric index [1], [2], etc.
    Index(usize),
    /// Complex expression
    Expression(String),
}
