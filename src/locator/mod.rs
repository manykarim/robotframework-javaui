//! Locator parsing and matching for Swing UI elements

pub mod ast;
pub mod expression;
pub mod matcher;
pub mod parser;

// Explicit exports from ast (avoid PseudoSelector conflict)
pub use ast::{
    Locator, ComplexSelector, Combinator, CompoundSelector, TypeSelector,
    AttributeSelector, AttributeMatcher, MatchOperator, AttributeValue, NthExpr,
    PseudoSelector as AstPseudoSelector,
};

// Explicit exports from expression (PseudoSelector comes from here)
pub use expression::{
    LocatorExpression, LocatorParseError, SimpleLocator, SimpleLocatorType,
    CssSelector, CssSelectorSegment, CssAttribute, AttributeOperator, PseudoSelector,
    XPathExpression, XPathStep, XPathAxis, XPathPredicate,
};

pub use matcher::{Evaluator, MatchContext, MatchResult, find_matching_components};
pub use parser::{parse_locator, ParseError};
