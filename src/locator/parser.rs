//! Parser implementation for locator expressions using pest
//!
//! This module provides the parser that converts locator strings into AST structures.

use pest::Parser;
use pest_derive::Parser;
use std::fmt;

use super::ast::*;

/// Pest parser for locator grammar
#[derive(Parser)]
#[grammar = "locator/grammar.pest"]
pub struct LocatorParser;

/// Parse error with position information
#[derive(Debug, Clone)]
pub struct ParseError {
    /// Error message
    pub message: String,
    /// Error kind
    pub kind: ParseErrorKind,
    /// Position in input (byte offset)
    pub position: usize,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// The problematic input fragment
    pub fragment: Option<String>,
}

impl ParseError {
    /// Create a new parse error
    pub fn new(message: String, kind: ParseErrorKind, position: usize) -> Self {
        Self {
            message,
            kind,
            position,
            line: 1,
            column: position + 1,
            fragment: None,
        }
    }

    /// Create an error with line/column information
    pub fn with_location(
        message: String,
        kind: ParseErrorKind,
        position: usize,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            message,
            kind,
            position,
            line,
            column,
            fragment: None,
        }
    }

    /// Add a fragment to the error
    pub fn with_fragment(mut self, fragment: String) -> Self {
        self.fragment = Some(fragment);
        self
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error at line {}, column {}: {}",
            self.line, self.column, self.message
        )?;
        if let Some(ref fragment) = self.fragment {
            write!(f, " near '{}'", fragment)?;
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {}

/// Kind of parse error
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// Empty input
    EmptyInput,
    /// Unexpected character
    UnexpectedChar,
    /// Unexpected end of input
    UnexpectedEof,
    /// Invalid selector syntax
    InvalidSelector,
    /// Invalid attribute syntax
    InvalidAttribute,
    /// Invalid pseudo selector
    InvalidPseudo,
    /// Invalid XPath expression
    InvalidXPath,
    /// Unclosed bracket or quote
    Unclosed,
    /// General syntax error
    SyntaxError,
}

/// Parse a locator string into an AST
///
/// # Arguments
///
/// * `input` - The locator string to parse
///
/// # Returns
///
/// A `Locator` AST on success, or a `ParseError` on failure
///
/// # Examples
///
/// ```rust,ignore
/// use robotframework_swing::locator::parse_locator;
///
/// let locator = parse_locator("JButton#submit:enabled").unwrap();
/// let locator = parse_locator("//JButton[@text='Save']").unwrap();
/// ```
pub fn parse_locator(input: &str) -> Result<Locator, ParseError> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err(ParseError::new(
            "Empty locator expression".to_string(),
            ParseErrorKind::EmptyInput,
            0,
        ));
    }

    // Try to parse with pest
    let pairs = LocatorParser::parse(Rule::locator, trimmed).map_err(|e| {
        let (line, column) = match e.line_col {
            pest::error::LineColLocation::Pos((l, c)) => (l, c),
            pest::error::LineColLocation::Span((l, c), _) => (l, c),
        };
        ParseError::with_location(
            format!("Syntax error: {}", e.variant.message()),
            ParseErrorKind::SyntaxError,
            0,
            line,
            column,
        )
    })?;

    // Build AST from parsed pairs
    let mut selectors = Vec::new();
    let mut is_xpath = false;

    for pair in pairs {
        match pair.as_rule() {
            Rule::css_selector_list => {
                for selector_pair in pair.into_inner() {
                    if let Rule::complex_selector = selector_pair.as_rule() {
                        selectors.push(parse_complex_selector(selector_pair)?);
                    }
                }
            }
            Rule::xpath_expr => {
                is_xpath = true;
                selectors.push(parse_xpath_expr(pair)?);
            }
            Rule::EOI => {}
            _ => {}
        }
    }

    if selectors.is_empty() {
        return Err(ParseError::new(
            "No valid selectors found".to_string(),
            ParseErrorKind::InvalidSelector,
            0,
        ));
    }

    Ok(Locator::new(selectors, input.to_string(), is_xpath))
}

/// Parse a complex selector (chain of compound selectors with combinators)
fn parse_complex_selector(pair: pest::iterators::Pair<Rule>) -> Result<ComplexSelector, ParseError> {
    let mut compounds: Vec<CompoundSelector> = Vec::new();
    let mut current_combinator: Option<Combinator> = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::compound_selector => {
                let mut compound = parse_compound_selector(inner)?;
                if let Some(comb) = current_combinator.take() {
                    // Set the combinator on the previous compound
                    if let Some(prev) = compounds.last_mut() {
                        prev.combinator = Some(comb);
                    }
                }
                compounds.push(compound);
            }
            Rule::combinator => {
                current_combinator = Some(parse_combinator(inner)?);
            }
            _ => {}
        }
    }

    Ok(ComplexSelector::from_compounds(compounds))
}

/// Parse a compound selector
fn parse_compound_selector(pair: pest::iterators::Pair<Rule>) -> Result<CompoundSelector, ParseError> {
    let mut compound = CompoundSelector::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::type_selector => {
                compound.type_selector = Some(parse_type_selector(inner)?);
            }
            Rule::id_selector => {
                compound.id_selector = Some(parse_id_selector(inner)?);
            }
            Rule::class_selector => {
                compound.class_selectors.push(parse_class_selector(inner)?);
            }
            Rule::attribute_selector => {
                compound.attribute_selectors.push(parse_attribute_selector(inner)?);
            }
            Rule::pseudo_selector => {
                compound.pseudo_selectors.push(parse_pseudo_selector(inner)?);
            }
            _ => {}
        }
    }

    Ok(compound)
}

/// Parse a type selector
fn parse_type_selector(pair: pest::iterators::Pair<Rule>) -> Result<TypeSelector, ParseError> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::universal_selector => return Ok(TypeSelector::Universal),
            Rule::type_name => return Ok(TypeSelector::TypeName(inner.as_str().to_string())),
            _ => {}
        }
    }
    Err(ParseError::new(
        "Invalid type selector".to_string(),
        ParseErrorKind::InvalidSelector,
        0,
    ))
}

/// Parse an ID selector
fn parse_id_selector(pair: pest::iterators::Pair<Rule>) -> Result<String, ParseError> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::identifier {
            return Ok(inner.as_str().to_string());
        }
    }
    Err(ParseError::new(
        "Invalid ID selector".to_string(),
        ParseErrorKind::InvalidSelector,
        0,
    ))
}

/// Parse a class selector
fn parse_class_selector(pair: pest::iterators::Pair<Rule>) -> Result<String, ParseError> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::identifier {
            return Ok(inner.as_str().to_string());
        }
    }
    Err(ParseError::new(
        "Invalid class selector".to_string(),
        ParseErrorKind::InvalidSelector,
        0,
    ))
}

/// Parse an attribute selector
fn parse_attribute_selector(pair: pest::iterators::Pair<Rule>) -> Result<AttributeSelector, ParseError> {
    let mut name = String::new();
    let mut matcher: Option<AttributeMatcher> = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::attribute_name => {
                name = inner.as_str().to_string();
            }
            Rule::attribute_matcher => {
                matcher = Some(parse_attribute_matcher(inner)?);
            }
            _ => {}
        }
    }

    if name.is_empty() {
        return Err(ParseError::new(
            "Missing attribute name".to_string(),
            ParseErrorKind::InvalidAttribute,
            0,
        ));
    }

    Ok(AttributeSelector { name, matcher })
}

/// Parse an attribute matcher (operator + value)
fn parse_attribute_matcher(pair: pest::iterators::Pair<Rule>) -> Result<AttributeMatcher, ParseError> {
    let mut operator: Option<MatchOperator> = None;
    let mut value: Option<AttributeValue> = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::match_operator => {
                operator = Some(parse_match_operator(inner)?);
            }
            Rule::attribute_value => {
                value = Some(parse_attribute_value(inner)?);
            }
            _ => {}
        }
    }

    match (operator, value) {
        (Some(op), Some(val)) => Ok(AttributeMatcher {
            operator: op,
            value: val,
        }),
        _ => Err(ParseError::new(
            "Invalid attribute matcher".to_string(),
            ParseErrorKind::InvalidAttribute,
            0,
        )),
    }
}

/// Parse a match operator
fn parse_match_operator(pair: pest::iterators::Pair<Rule>) -> Result<MatchOperator, ParseError> {
    for inner in pair.into_inner() {
        return match inner.as_rule() {
            Rule::equals => Ok(MatchOperator::Equals),
            Rule::not_equals => Ok(MatchOperator::NotEquals),
            Rule::prefix_match => Ok(MatchOperator::PrefixMatch),
            Rule::suffix_match => Ok(MatchOperator::SuffixMatch),
            Rule::substring_match => Ok(MatchOperator::SubstringMatch),
            Rule::word_match => Ok(MatchOperator::WordMatch),
            Rule::dash_match => Ok(MatchOperator::DashMatch),
            Rule::regex_match => Ok(MatchOperator::RegexMatch),
            _ => Err(ParseError::new(
                format!("Unknown operator: {:?}", inner.as_rule()),
                ParseErrorKind::InvalidAttribute,
                0,
            )),
        };
    }
    Err(ParseError::new(
        "Missing operator".to_string(),
        ParseErrorKind::InvalidAttribute,
        0,
    ))
}

/// Parse an attribute value
fn parse_attribute_value(pair: pest::iterators::Pair<Rule>) -> Result<AttributeValue, ParseError> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::quoted_string => {
                return Ok(AttributeValue::String(parse_quoted_string(inner)?));
            }
            Rule::unquoted_value => {
                let text = inner.as_str();
                // Try to parse as number
                if let Ok(n) = text.parse::<f64>() {
                    return Ok(AttributeValue::Number(n));
                }
                return Ok(AttributeValue::String(text.to_string()));
            }
            _ => {}
        }
    }
    Err(ParseError::new(
        "Invalid attribute value".to_string(),
        ParseErrorKind::InvalidAttribute,
        0,
    ))
}

/// Parse a quoted string (extract content without quotes)
fn parse_quoted_string(pair: pest::iterators::Pair<Rule>) -> Result<String, ParseError> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::single_quoted => {
                for content in inner.into_inner() {
                    if content.as_rule() == Rule::single_quoted_inner {
                        return Ok(content.as_str().to_string());
                    }
                }
            }
            Rule::double_quoted => {
                for content in inner.into_inner() {
                    if content.as_rule() == Rule::double_quoted_inner {
                        return Ok(content.as_str().to_string());
                    }
                }
            }
            _ => {}
        }
    }
    Err(ParseError::new(
        "Invalid quoted string".to_string(),
        ParseErrorKind::InvalidAttribute,
        0,
    ))
}

/// Parse a pseudo selector
fn parse_pseudo_selector(pair: pest::iterators::Pair<Rule>) -> Result<PseudoSelector, ParseError> {
    let mut name = String::new();
    let mut arg: Option<pest::iterators::Pair<Rule>> = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::pseudo_name => {
                name = inner.as_str().to_string();
            }
            Rule::pseudo_arg => {
                arg = Some(inner);
            }
            _ => {}
        }
    }

    match name.as_str() {
        "first-child" => Ok(PseudoSelector::FirstChild),
        "last-child" => Ok(PseudoSelector::LastChild),
        "nth-child" => {
            let expr = arg
                .map(|a| parse_nth_expr(a))
                .transpose()?
                .unwrap_or(NthExpr::Index(1));
            Ok(PseudoSelector::NthChild(expr))
        }
        "nth-last-child" => {
            let expr = arg
                .map(|a| parse_nth_expr(a))
                .transpose()?
                .unwrap_or(NthExpr::Index(1));
            Ok(PseudoSelector::NthLastChild(expr))
        }
        "only-child" => Ok(PseudoSelector::OnlyChild),
        "first-of-type" => Ok(PseudoSelector::FirstOfType),
        "last-of-type" => Ok(PseudoSelector::LastOfType),
        "nth-of-type" => {
            let expr = arg
                .map(|a| parse_nth_expr(a))
                .transpose()?
                .unwrap_or(NthExpr::Index(1));
            Ok(PseudoSelector::NthOfType(expr))
        }
        "nth-last-of-type" => {
            let expr = arg
                .map(|a| parse_nth_expr(a))
                .transpose()?
                .unwrap_or(NthExpr::Index(1));
            Ok(PseudoSelector::NthLastOfType(expr))
        }
        "only-of-type" => Ok(PseudoSelector::OnlyOfType),
        "empty" => Ok(PseudoSelector::Empty),
        "root" => Ok(PseudoSelector::Root),
        "enabled" => Ok(PseudoSelector::Enabled),
        "disabled" => Ok(PseudoSelector::Disabled),
        "visible" => Ok(PseudoSelector::Visible),
        "hidden" => Ok(PseudoSelector::Hidden),
        "showing" => Ok(PseudoSelector::Showing),
        "focused" | "focus" => Ok(PseudoSelector::Focused),
        "selected" | "checked" => Ok(PseudoSelector::Selected),
        "editable" => Ok(PseudoSelector::Editable),
        "readonly" => Ok(PseudoSelector::ReadOnly),
        "not" => {
            let nested = arg
                .map(|a| parse_nested_selector(a))
                .transpose()?
                .unwrap_or_else(CompoundSelector::new);
            Ok(PseudoSelector::Not(Box::new(nested)))
        }
        "has" => {
            let nested = arg
                .map(|a| parse_nested_selector(a))
                .transpose()?
                .unwrap_or_else(CompoundSelector::new);
            Ok(PseudoSelector::Has(Box::new(nested)))
        }
        "contains" => {
            let text = arg
                .map(|a| parse_string_arg(a))
                .transpose()?
                .unwrap_or_default();
            Ok(PseudoSelector::Contains(text))
        }
        _ => Err(ParseError::new(
            format!("Unknown pseudo selector: {}", name),
            ParseErrorKind::InvalidPseudo,
            0,
        )),
    }
}

/// Parse nth expression (number, odd, even, or formula)
fn parse_nth_expr(pair: pest::iterators::Pair<Rule>) -> Result<NthExpr, ParseError> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::nth_expr => {
                return parse_nth_expr(inner);
            }
            Rule::nth_odd => return Ok(NthExpr::Odd),
            Rule::nth_even => return Ok(NthExpr::Even),
            Rule::nth_number => {
                let n: i32 = inner.as_str().parse().map_err(|_| {
                    ParseError::new(
                        "Invalid number in nth expression".to_string(),
                        ParseErrorKind::InvalidPseudo,
                        0,
                    )
                })?;
                return Ok(NthExpr::Index(n));
            }
            Rule::nth_formula => {
                return parse_nth_formula(inner);
            }
            _ => {}
        }
    }
    Err(ParseError::new(
        "Invalid nth expression".to_string(),
        ParseErrorKind::InvalidPseudo,
        0,
    ))
}

/// Parse nth formula (an+b)
fn parse_nth_formula(pair: pest::iterators::Pair<Rule>) -> Result<NthExpr, ParseError> {
    let mut a: i32 = 1;
    let mut b: i32 = 0;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::nth_a => {
                let s = inner.as_str();
                if s.is_empty() || s == "+" {
                    a = 1;
                } else if s == "-" {
                    a = -1;
                } else {
                    a = s.parse().unwrap_or(1);
                }
            }
            Rule::nth_b => {
                b = inner.as_str().parse().unwrap_or(0);
            }
            _ => {}
        }
    }

    Ok(NthExpr::Formula { a, b })
}

/// Parse nested selector for :not() and :has()
fn parse_nested_selector(pair: pest::iterators::Pair<Rule>) -> Result<CompoundSelector, ParseError> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::string_arg => {
                // Handle :not(selector) as a string argument
                let text = parse_string_arg(inner)?;
                // Try to parse as a simple type selector
                let mut compound = CompoundSelector::new();
                compound.type_selector = Some(TypeSelector::TypeName(text));
                return Ok(compound);
            }
            Rule::quoted_string => {
                let text = parse_quoted_string(inner)?;
                let mut compound = CompoundSelector::new();
                compound.type_selector = Some(TypeSelector::TypeName(text));
                return Ok(compound);
            }
            Rule::unquoted_arg => {
                let text = inner.as_str().to_string();
                let mut compound = CompoundSelector::new();
                compound.type_selector = Some(TypeSelector::TypeName(text));
                return Ok(compound);
            }
            Rule::compound_selector => {
                return parse_compound_selector(inner);
            }
            _ => {
                // Recursively try to parse inner content
                if let Ok(result) = parse_nested_selector(inner) {
                    return Ok(result);
                }
            }
        }
    }
    // Return empty compound if nothing found
    Ok(CompoundSelector::new())
}

/// Parse string argument for :contains()
fn parse_string_arg(pair: pest::iterators::Pair<Rule>) -> Result<String, ParseError> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::string_arg => {
                return parse_string_arg(inner);
            }
            Rule::quoted_string => {
                return parse_quoted_string(inner);
            }
            Rule::unquoted_arg => {
                return Ok(inner.as_str().to_string());
            }
            _ => {}
        }
    }
    Err(ParseError::new(
        "Invalid string argument".to_string(),
        ParseErrorKind::InvalidPseudo,
        0,
    ))
}

/// Parse a combinator
fn parse_combinator(pair: pest::iterators::Pair<Rule>) -> Result<Combinator, ParseError> {
    for inner in pair.into_inner() {
        return match inner.as_rule() {
            Rule::child_combinator => Ok(Combinator::Child),
            Rule::adjacent_sibling => Ok(Combinator::AdjacentSibling),
            Rule::general_sibling => Ok(Combinator::GeneralSibling),
            Rule::descendant_combinator => Ok(Combinator::Descendant),
            _ => Err(ParseError::new(
                format!("Unknown combinator: {:?}", inner.as_rule()),
                ParseErrorKind::SyntaxError,
                0,
            )),
        };
    }
    // Default to descendant if no specific combinator found
    Ok(Combinator::Descendant)
}

/// Parse XPath expression
fn parse_xpath_expr(pair: pest::iterators::Pair<Rule>) -> Result<ComplexSelector, ParseError> {
    let mut compounds: Vec<CompoundSelector> = Vec::new();
    let mut is_descendant = false;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::xpath_start => {
                // // means descendant-or-self
                is_descendant = inner.as_str() == "//";
            }
            Rule::xpath_step => {
                let compound = parse_xpath_step(inner, is_descendant)?;
                if !compounds.is_empty() {
                    // Set combinator on previous
                    if let Some(prev) = compounds.last_mut() {
                        prev.combinator = Some(if is_descendant {
                            Combinator::Descendant
                        } else {
                            Combinator::Child
                        });
                    }
                }
                compounds.push(compound);
                // After first step, subsequent / means child
                is_descendant = false;
            }
            _ => {}
        }
    }

    Ok(ComplexSelector::from_compounds(compounds))
}

/// Parse a single XPath step
fn parse_xpath_step(pair: pest::iterators::Pair<Rule>, is_descendant: bool) -> Result<CompoundSelector, ParseError> {
    let mut compound = CompoundSelector::new();
    let mut axis_descendant = is_descendant;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::xpath_axis => {
                let axis = inner.as_str();
                if axis.contains("descendant") {
                    axis_descendant = true;
                }
                // Handle parent axis, etc. - for now we just track descendant
            }
            Rule::xpath_node_test => {
                for node_inner in inner.into_inner() {
                    match node_inner.as_rule() {
                        Rule::universal_selector => {
                            compound.type_selector = Some(TypeSelector::Universal);
                        }
                        Rule::type_name => {
                            compound.type_selector = Some(TypeSelector::TypeName(node_inner.as_str().to_string()));
                        }
                        Rule::xpath_node_type => {
                            // node(), text(), etc. - treat as universal for now
                            compound.type_selector = Some(TypeSelector::Universal);
                        }
                        _ => {}
                    }
                }
            }
            Rule::xpath_predicate => {
                parse_xpath_predicate_into_compound(inner, &mut compound)?;
            }
            _ => {}
        }
    }

    Ok(compound)
}

/// Parse XPath predicate and add to compound selector
fn parse_xpath_predicate_into_compound(
    pair: pest::iterators::Pair<Rule>,
    compound: &mut CompoundSelector,
) -> Result<(), ParseError> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::xpath_predicate_expr {
            parse_xpath_predicate_expr_into_compound(inner, compound)?;
        }
    }
    Ok(())
}

/// Parse XPath predicate expression
fn parse_xpath_predicate_expr_into_compound(
    pair: pest::iterators::Pair<Rule>,
    compound: &mut CompoundSelector,
) -> Result<(), ParseError> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::xpath_index => {
                let idx: i32 = inner.as_str().parse().map_err(|_| {
                    ParseError::new(
                        "Invalid XPath index".to_string(),
                        ParseErrorKind::InvalidXPath,
                        0,
                    )
                })?;
                compound.pseudo_selectors.push(PseudoSelector::NthChild(NthExpr::Index(idx)));
            }
            Rule::xpath_attr_exists => {
                for attr_inner in inner.into_inner() {
                    if attr_inner.as_rule() == Rule::attribute_name {
                        compound.attribute_selectors.push(AttributeSelector::exists(
                            attr_inner.as_str().to_string(),
                        ));
                    }
                }
            }
            Rule::xpath_comparison => {
                let attr_sel = parse_xpath_comparison(inner)?;
                compound.attribute_selectors.push(attr_sel);
            }
            Rule::xpath_function_call => {
                // Handle functions like contains(), starts-with(), etc.
                let pseudo = parse_xpath_function_to_pseudo(inner)?;
                if let Some(p) = pseudo {
                    compound.pseudo_selectors.push(p);
                }
            }
            Rule::xpath_or_expr | Rule::xpath_and_expr | Rule::xpath_primary_expr => {
                // Recursively handle complex predicates
                for sub in inner.into_inner() {
                    parse_xpath_predicate_expr_into_compound(sub, compound)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Parse XPath comparison (@attr='value')
fn parse_xpath_comparison(pair: pest::iterators::Pair<Rule>) -> Result<AttributeSelector, ParseError> {
    let mut attr_name: Option<String> = None;
    let mut operator = MatchOperator::Equals;
    let mut value: Option<AttributeValue> = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::xpath_lhs => {
                for lhs in inner.into_inner() {
                    if lhs.as_rule() == Rule::attribute_name {
                        attr_name = Some(lhs.as_str().to_string());
                    }
                }
            }
            Rule::xpath_comp_op => {
                operator = match inner.as_str() {
                    "=" => MatchOperator::Equals,
                    "!=" => MatchOperator::NotEquals,
                    "<" => MatchOperator::LessThan,
                    "<=" => MatchOperator::LessOrEqual,
                    ">" => MatchOperator::GreaterThan,
                    ">=" => MatchOperator::GreaterOrEqual,
                    _ => MatchOperator::Equals,
                };
            }
            Rule::xpath_rhs => {
                for rhs in inner.into_inner() {
                    match rhs.as_rule() {
                        Rule::quoted_string => {
                            value = Some(AttributeValue::String(parse_quoted_string(rhs)?));
                        }
                        Rule::xpath_number => {
                            let n: f64 = rhs.as_str().parse().unwrap_or(0.0);
                            value = Some(AttributeValue::Number(n));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    match (attr_name, value) {
        (Some(name), Some(val)) => Ok(AttributeSelector::with_operator(name, operator, val)),
        (Some(name), None) => Ok(AttributeSelector::exists(name)),
        _ => Err(ParseError::new(
            "Invalid XPath comparison".to_string(),
            ParseErrorKind::InvalidXPath,
            0,
        )),
    }
}

/// Parse XPath function call into pseudo selector
fn parse_xpath_function_to_pseudo(pair: pest::iterators::Pair<Rule>) -> Result<Option<PseudoSelector>, ParseError> {
    let mut func_name = String::new();
    let mut args: Vec<String> = Vec::new();

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::xpath_function_name => {
                func_name = inner.as_str().to_string();
            }
            Rule::xpath_function_args => {
                for arg in inner.into_inner() {
                    if arg.as_rule() == Rule::xpath_function_arg {
                        for arg_inner in arg.into_inner() {
                            match arg_inner.as_rule() {
                                Rule::quoted_string => {
                                    args.push(parse_quoted_string(arg_inner)?);
                                }
                                Rule::xpath_number => {
                                    args.push(arg_inner.as_str().to_string());
                                }
                                Rule::attribute_name => {
                                    args.push(format!("@{}", arg_inner.as_str()));
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    // Map XPath functions to pseudo selectors where applicable
    match func_name.as_str() {
        "contains" => {
            // contains(@attr, 'value') -> we need to handle this as attribute selector
            // For now, if second arg is available, use :contains()
            if args.len() >= 2 {
                Ok(Some(PseudoSelector::Contains(args[1].clone())))
            } else {
                Ok(None)
            }
        }
        "not" => {
            // XPath not() is different from CSS :not(), handle specially
            Ok(None)
        }
        "last" => Ok(Some(PseudoSelector::LastChild)),
        "position" => Ok(None), // Typically used in comparison, not standalone
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_type_only() {
        let result = parse_locator("JButton");
        assert!(result.is_ok());
        let locator = result.unwrap();
        assert_eq!(locator.selectors.len(), 1);
    }

    #[test]
    fn test_parse_universal() {
        let result = parse_locator("*");
        assert!(result.is_ok());
        let locator = result.unwrap();
        assert!(locator.is_universal());
    }

    #[test]
    fn test_parse_id() {
        let result = parse_locator("#myButton");
        assert!(result.is_ok());
        let locator = result.unwrap();
        let compound = &locator.selectors[0].compounds[0];
        assert_eq!(compound.id_selector.as_deref(), Some("myButton"));
    }

    #[test]
    fn test_parse_class() {
        let result = parse_locator(".primary.active");
        assert!(result.is_ok());
        let locator = result.unwrap();
        let compound = &locator.selectors[0].compounds[0];
        assert_eq!(compound.class_selectors.len(), 2);
        assert_eq!(compound.class_selectors[0], "primary");
        assert_eq!(compound.class_selectors[1], "active");
    }

    #[test]
    fn test_parse_attribute_equals() {
        let result = parse_locator("[text='Hello']");
        assert!(result.is_ok());
        let locator = result.unwrap();
        let compound = &locator.selectors[0].compounds[0];
        assert_eq!(compound.attribute_selectors.len(), 1);
        let attr = &compound.attribute_selectors[0];
        assert_eq!(attr.name, "text");
        assert!(matches!(
            &attr.matcher,
            Some(AttributeMatcher {
                operator: MatchOperator::Equals,
                ..
            })
        ));
    }

    #[test]
    fn test_parse_attribute_contains() {
        let result = parse_locator("[text*='Save']");
        assert!(result.is_ok());
        let locator = result.unwrap();
        let attr = &locator.selectors[0].compounds[0].attribute_selectors[0];
        assert!(matches!(
            &attr.matcher,
            Some(AttributeMatcher {
                operator: MatchOperator::SubstringMatch,
                ..
            })
        ));
    }

    #[test]
    fn test_parse_pseudo_enabled() {
        let result = parse_locator("JButton:enabled");
        assert!(result.is_ok());
        let locator = result.unwrap();
        let compound = &locator.selectors[0].compounds[0];
        assert_eq!(compound.pseudo_selectors.len(), 1);
        assert!(matches!(compound.pseudo_selectors[0], PseudoSelector::Enabled));
    }

    #[test]
    fn test_parse_pseudo_nth_child() {
        let result = parse_locator("JButton:nth-child(3)");
        assert!(result.is_ok());
        let locator = result.unwrap();
        let compound = &locator.selectors[0].compounds[0];
        assert!(matches!(
            &compound.pseudo_selectors[0],
            PseudoSelector::NthChild(NthExpr::Index(3))
        ));
    }

    #[test]
    fn test_parse_child_combinator() {
        let result = parse_locator("JPanel > JButton");
        assert!(result.is_ok());
        let locator = result.unwrap();
        let selector = &locator.selectors[0];
        assert_eq!(selector.compounds.len(), 2);
        assert!(matches!(
            selector.compounds[0].combinator,
            Some(Combinator::Child)
        ));
    }

    #[test]
    fn test_parse_descendant_combinator() {
        let result = parse_locator("JFrame JButton");
        assert!(result.is_ok());
        let locator = result.unwrap();
        let selector = &locator.selectors[0];
        assert_eq!(selector.compounds.len(), 2);
        assert!(matches!(
            selector.compounds[0].combinator,
            Some(Combinator::Descendant)
        ));
    }

    #[test]
    fn test_parse_sibling_combinators() {
        let result = parse_locator("JLabel + JTextField");
        assert!(result.is_ok());
        let locator = result.unwrap();
        assert!(matches!(
            locator.selectors[0].compounds[0].combinator,
            Some(Combinator::AdjacentSibling)
        ));

        let result = parse_locator("JLabel ~ JButton");
        assert!(result.is_ok());
        let locator = result.unwrap();
        assert!(matches!(
            locator.selectors[0].compounds[0].combinator,
            Some(Combinator::GeneralSibling)
        ));
    }

    #[test]
    fn test_parse_multiple_selectors() {
        let result = parse_locator("JButton, JTextField");
        assert!(result.is_ok());
        let locator = result.unwrap();
        assert_eq!(locator.selectors.len(), 2);
    }

    #[test]
    fn test_parse_xpath_simple() {
        let result = parse_locator("//JButton");
        assert!(result.is_ok());
        let locator = result.unwrap();
        assert!(locator.is_xpath);
    }

    #[test]
    fn test_parse_xpath_with_attribute() {
        let result = parse_locator("//JButton[@text='Save']");
        assert!(result.is_ok());
        let locator = result.unwrap();
        assert!(locator.is_xpath);
        let compound = &locator.selectors[0].compounds[0];
        assert_eq!(compound.attribute_selectors.len(), 1);
    }

    #[test]
    fn test_parse_xpath_index() {
        let result = parse_locator("//JButton[1]");
        assert!(result.is_ok());
        let locator = result.unwrap();
        let compound = &locator.selectors[0].compounds[0];
        assert!(matches!(
            &compound.pseudo_selectors[0],
            PseudoSelector::NthChild(NthExpr::Index(1))
        ));
    }

    #[test]
    fn test_parse_empty_error() {
        let result = parse_locator("");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err().kind, ParseErrorKind::EmptyInput));
    }

    #[test]
    fn test_parse_complex() {
        let result = parse_locator("JPanel.main > JButton#submit[text='OK']:enabled:visible");
        assert!(result.is_ok());
        let locator = result.unwrap();
        let selector = &locator.selectors[0];
        assert_eq!(selector.compounds.len(), 2);

        let panel = &selector.compounds[0];
        assert!(matches!(
            &panel.type_selector,
            Some(TypeSelector::TypeName(name)) if name == "JPanel"
        ));
        assert_eq!(panel.class_selectors, vec!["main"]);

        let button = &selector.compounds[1];
        assert!(matches!(
            &button.type_selector,
            Some(TypeSelector::TypeName(name)) if name == "JButton"
        ));
        assert_eq!(button.id_selector.as_deref(), Some("submit"));
        assert_eq!(button.attribute_selectors.len(), 1);
        assert_eq!(button.pseudo_selectors.len(), 2);
    }
}
