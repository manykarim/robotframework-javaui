//! Matcher implementation for evaluating locators against UI components
//!
//! This module provides the `Evaluator` that matches parsed locators against
//! UI component trees.

use std::cell::RefCell;

use lru::LruCache;
use regex::Regex;

use crate::model::UIComponent;

use super::ast::*;

/// Maximum size for the regex cache
const REGEX_CACHE_SIZE: usize = 100;

/// Result of a match operation
#[derive(Debug, Clone)]
pub struct MatchResult {
    /// Whether the component matches
    pub matches: bool,
    /// Confidence score (0.0 to 1.0)
    pub confidence: f64,
    /// Matched selectors for debugging
    pub matched_selectors: Vec<String>,
}

impl MatchResult {
    /// Create a matching result
    pub fn matched() -> Self {
        Self {
            matches: true,
            confidence: 1.0,
            matched_selectors: Vec::new(),
        }
    }

    /// Create a non-matching result
    pub fn not_matched() -> Self {
        Self {
            matches: false,
            confidence: 0.0,
            matched_selectors: Vec::new(),
        }
    }

    /// Create with confidence
    pub fn with_confidence(matches: bool, confidence: f64) -> Self {
        Self {
            matches,
            confidence,
            matched_selectors: Vec::new(),
        }
    }

    /// Add a matched selector description
    pub fn add_match(&mut self, selector: String) {
        self.matched_selectors.push(selector);
    }
}

/// Context for matching, provides tree traversal capabilities
pub struct MatchContext<'a> {
    /// The root of the tree
    pub root: &'a UIComponent,
    /// Parent component (if any)
    pub parent: Option<&'a UIComponent>,
    /// Full ancestor chain (from immediate parent to root)
    pub ancestors: Vec<&'a UIComponent>,
    /// Siblings (including self)
    pub siblings: Vec<&'a UIComponent>,
    /// Index among siblings (0-based)
    pub sibling_index: usize,
}

impl<'a> MatchContext<'a> {
    /// Create a new context
    pub fn new(root: &'a UIComponent) -> Self {
        Self {
            root,
            parent: None,
            ancestors: Vec::new(),
            siblings: vec![root],
            sibling_index: 0,
        }
    }

    /// Create a context with parent information
    pub fn with_parent(
        root: &'a UIComponent,
        parent: &'a UIComponent,
        siblings: Vec<&'a UIComponent>,
        sibling_index: usize,
    ) -> Self {
        Self {
            root,
            parent: Some(parent),
            ancestors: vec![parent],
            siblings,
            sibling_index,
        }
    }

    /// Create a context with full ancestor chain
    pub fn with_ancestors(
        root: &'a UIComponent,
        parent: Option<&'a UIComponent>,
        ancestors: Vec<&'a UIComponent>,
        siblings: Vec<&'a UIComponent>,
        sibling_index: usize,
    ) -> Self {
        Self {
            root,
            parent,
            ancestors,
            siblings,
            sibling_index,
        }
    }

    /// Get the number of siblings
    pub fn sibling_count(&self) -> usize {
        self.siblings.len()
    }

    /// Check if this is the first child
    pub fn is_first_child(&self) -> bool {
        self.sibling_index == 0
    }

    /// Check if this is the last child
    pub fn is_last_child(&self) -> bool {
        self.sibling_index == self.siblings.len().saturating_sub(1)
    }

    /// Get 1-based position among siblings
    pub fn position(&self) -> usize {
        self.sibling_index + 1
    }

    /// Get position from end (1-based)
    pub fn position_from_end(&self) -> usize {
        self.siblings.len() - self.sibling_index
    }
}

/// Evaluator for matching locators against components
pub struct Evaluator {
    /// Cache for compiled regex patterns
    regex_cache: RefCell<LruCache<String, Regex>>,
    /// Case-sensitive matching
    case_sensitive: bool,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    /// Create a new evaluator
    pub fn new() -> Self {
        Self {
            regex_cache: RefCell::new(LruCache::new(
                std::num::NonZeroUsize::new(REGEX_CACHE_SIZE).unwrap(),
            )),
            case_sensitive: false,
        }
    }

    /// Create an evaluator with case-sensitive matching
    pub fn case_sensitive() -> Self {
        Self {
            regex_cache: RefCell::new(LruCache::new(
                std::num::NonZeroUsize::new(REGEX_CACHE_SIZE).unwrap(),
            )),
            case_sensitive: true,
        }
    }

    /// Evaluate a locator against a component
    ///
    /// Returns true if any of the selectors in the locator match the component.
    pub fn evaluate(&self, locator: &Locator, component: &UIComponent, context: &MatchContext) -> MatchResult {
        for selector in &locator.selectors {
            let result = self.evaluate_complex_selector(selector, component, context);
            if result.matches {
                return result;
            }
        }
        MatchResult::not_matched()
    }

    /// Evaluate a complex selector against a component
    fn evaluate_complex_selector(
        &self,
        selector: &ComplexSelector,
        component: &UIComponent,
        context: &MatchContext,
    ) -> MatchResult {
        // For a complex selector with combinators, we need to match from right to left
        if selector.compounds.is_empty() {
            return MatchResult::not_matched();
        }

        // If there's only one compound, just match it directly
        if selector.compounds.len() == 1 {
            return self.evaluate_compound_selector(&selector.compounds[0], component, context);
        }

        // Match the rightmost compound first
        let last_idx = selector.compounds.len() - 1;
        let last_compound = &selector.compounds[last_idx];
        let result = self.evaluate_compound_selector(last_compound, component, context);
        if !result.matches {
            return result;
        }

        // Now we need to check combinators from right to left
        // This requires tree traversal which we'll implement with context
        self.match_combinator_chain(selector, component, context)
    }

    /// Match a chain of combinators from a complex selector
    fn match_combinator_chain(
        &self,
        selector: &ComplexSelector,
        _target: &UIComponent,
        context: &MatchContext,
    ) -> MatchResult {
        // Walk through compounds from right to left
        let current_context = context;

        for i in (0..selector.compounds.len()).rev() {
            let compound = &selector.compounds[i];

            // For the rightmost compound, we already checked it matches
            if i == selector.compounds.len() - 1 {
                continue;
            }

            // Get the combinator that connects this compound to the next
            let combinator = compound.combinator.as_ref().unwrap_or(&Combinator::Descendant);

            // Find matching ancestor/sibling based on combinator
            match combinator {
                Combinator::Descendant => {
                    // Any ancestor must match - iterate through all ancestors
                    let mut found = false;
                    for ancestor in &current_context.ancestors {
                        let ancestor_ctx = MatchContext::new(ancestor);
                        if self.evaluate_compound_selector(compound, ancestor, &ancestor_ctx).matches {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return MatchResult::not_matched();
                    }
                }
                Combinator::Child => {
                    // Direct parent must match
                    if let Some(parent) = current_context.parent {
                        let parent_ctx = MatchContext::new(parent);
                        if self.evaluate_compound_selector(compound, parent, &parent_ctx).matches {
                            continue;
                        }
                    }
                    return MatchResult::not_matched();
                }
                Combinator::AdjacentSibling => {
                    // Previous sibling must match
                    if current_context.sibling_index > 0 {
                        let prev_sibling = current_context.siblings[current_context.sibling_index - 1];
                        let sibling_ctx = MatchContext::with_parent(
                            prev_sibling,
                            current_context.parent.unwrap_or(context.root),
                            current_context.siblings.clone(),
                            current_context.sibling_index - 1,
                        );
                        if self.evaluate_compound_selector(compound, prev_sibling, &sibling_ctx).matches {
                            continue;
                        }
                    }
                    return MatchResult::not_matched();
                }
                Combinator::GeneralSibling => {
                    // Any previous sibling must match
                    for idx in 0..current_context.sibling_index {
                        let sibling = current_context.siblings[idx];
                        let sibling_ctx = MatchContext::with_parent(
                            sibling,
                            current_context.parent.unwrap_or(context.root),
                            current_context.siblings.clone(),
                            idx,
                        );
                        if self.evaluate_compound_selector(compound, sibling, &sibling_ctx).matches {
                            // Found a matching sibling
                            break;
                        }
                        if idx == current_context.sibling_index - 1 {
                            return MatchResult::not_matched();
                        }
                    }
                }
                Combinator::Cascaded => {
                    // Cascaded >> is like descendant but semantically means "find within"
                    // Any ancestor must match - iterate through all ancestors
                    let mut found = false;
                    for ancestor in &current_context.ancestors {
                        let ancestor_ctx = MatchContext::new(ancestor);
                        if self.evaluate_compound_selector(compound, ancestor, &ancestor_ctx).matches {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return MatchResult::not_matched();
                    }
                }
            }
        }

        MatchResult::matched()
    }

    /// Evaluate a compound selector against a component
    fn evaluate_compound_selector(
        &self,
        compound: &CompoundSelector,
        component: &UIComponent,
        context: &MatchContext,
    ) -> MatchResult {
        let mut result = MatchResult::matched();

        // Check type selector
        if let Some(ref type_sel) = compound.type_selector {
            if !self.match_type_selector(type_sel, component) {
                return MatchResult::not_matched();
            }
            result.add_match(format!("type:{}", type_sel));
        }

        // Check ID selector
        if let Some(ref id) = compound.id_selector {
            if !self.match_id_selector(id, component) {
                return MatchResult::not_matched();
            }
            result.add_match(format!("id:{}", id));
        }

        // Check class selectors
        for class in &compound.class_selectors {
            if !self.match_class_selector(class, component) {
                return MatchResult::not_matched();
            }
            result.add_match(format!("class:{}", class));
        }

        // Check attribute selectors
        for attr in &compound.attribute_selectors {
            if !self.match_attribute_selector(attr, component) {
                return MatchResult::not_matched();
            }
            result.add_match(format!("attr:{}", attr.name));
        }

        // Check pseudo selectors
        for pseudo in &compound.pseudo_selectors {
            if !self.match_pseudo_selector(pseudo, component, context) {
                return MatchResult::not_matched();
            }
            result.add_match(format!("pseudo:{}", pseudo));
        }

        result
    }

    /// Match a type selector
    fn match_type_selector(&self, selector: &TypeSelector, component: &UIComponent) -> bool {
        match selector {
            TypeSelector::Universal => true,
            TypeSelector::TypeName(name) => {
                let simple_name = &component.component_type.simple_name;
                if self.case_sensitive {
                    simple_name == name || simple_name.strip_prefix('J') == Some(name)
                } else {
                    simple_name.eq_ignore_ascii_case(name)
                        || simple_name
                            .strip_prefix('J')
                            .map_or(false, |s| s.eq_ignore_ascii_case(name))
                }
            }
            TypeSelector::PrefixSelector { key, value } => {
                // Handle prefix-style selectors like class=JButton, name=myButton
                match key.as_str() {
                    "class" => self.match_class_selector(value, component),
                    "name" => {
                        if let Some(ref name) = component.identity.name {
                            self.string_equals(name, value)
                        } else {
                            false
                        }
                    }
                    "text" => {
                        if let Some(ref text) = component.identity.text {
                            self.string_equals(text, value)
                        } else {
                            false
                        }
                    }
                    "id" => self.match_id_selector(value, component),
                    "tooltip" => {
                        if let Some(ref tooltip) = component.identity.tooltip {
                            self.string_equals(tooltip, value)
                        } else {
                            false
                        }
                    }
                    "index" => {
                        // Index selector would need context, not supported in type selector
                        false
                    }
                    "accessible" => {
                        if let Some(ref accessible_name) = component.accessibility.accessible_name {
                            self.string_equals(accessible_name, value)
                        } else {
                            false
                        }
                    }
                    _ => false,
                }
            }
        }
    }

    /// Match an ID selector
    fn match_id_selector(&self, id: &str, component: &UIComponent) -> bool {
        // Check internal_name first, then name
        if let Some(ref internal_name) = component.identity.internal_name {
            if self.string_equals(internal_name, id) {
                return true;
            }
        }
        if let Some(ref name) = component.identity.name {
            if self.string_equals(name, id) {
                return true;
            }
        }
        false
    }

    /// Match a class selector
    fn match_class_selector(&self, class: &str, component: &UIComponent) -> bool {
        // In Swing, we treat the simple class name (without J prefix) as a "class"
        // Also check against accessible role
        let simple_name = &component.component_type.simple_name;
        let simple_without_j = simple_name.strip_prefix('J').unwrap_or(simple_name);

        // Also strip J prefix from the selector value for consistency
        let class_without_j = class.strip_prefix('J').unwrap_or(class);

        if self.string_equals(simple_without_j, class_without_j) {
            return true;
        }

        // Check accessible role
        if let Some(ref role) = component.accessibility.accessible_role {
            if self.string_equals(role, class) {
                return true;
            }
        }

        // Check accessible state
        for state in &component.accessibility.accessible_state {
            if self.string_equals(state, class) {
                return true;
            }
        }

        false
    }

    /// Match an attribute selector
    fn match_attribute_selector(&self, selector: &AttributeSelector, component: &UIComponent) -> bool {
        // Get the attribute value from the component
        let attr_value = self.get_attribute_value(&selector.name, component);

        // If no matcher, just check existence
        if selector.matcher.is_none() {
            return attr_value.is_some();
        }

        let matcher = selector.matcher.as_ref().unwrap();
        let component_value = match attr_value {
            Some(v) => v,
            None => return false,
        };

        self.match_value(&matcher.operator, &component_value, &matcher.value)
    }

    /// Get an attribute value from a component
    fn get_attribute_value(&self, attr: &str, component: &UIComponent) -> Option<String> {
        match attr.to_lowercase().as_str() {
            "name" => component.identity.name.clone(),
            "internalname" | "internal_name" | "internal-name" => {
                component.identity.internal_name.clone()
            }
            "text" => component.identity.text.clone(),
            "title" => component.identity.title.clone(),
            "tooltip" | "tooltiptext" => component.identity.tooltip.clone(),
            "actioncommand" | "action_command" | "action-command" => {
                component.identity.action_command.clone()
            }
            "label" | "labeltext" => component.identity.label_text.clone(),
            "class" | "classname" | "class_name" => {
                Some(component.component_type.class_name.clone())
            }
            "simplename" | "simple_name" | "type" => {
                Some(component.component_type.simple_name.clone())
            }
            "enabled" => Some(component.state.enabled.to_string()),
            "visible" => Some(component.state.visible.to_string()),
            "showing" => Some(component.state.showing.to_string()),
            "focused" | "focus" => Some(component.state.focused.to_string()),
            "focusable" => Some(component.state.focusable.to_string()),
            "selected" | "checked" => component.state.selected.map(|s| s.to_string()),
            "editable" => component.state.editable.map(|e| e.to_string()),
            "x" => Some(component.geometry.bounds.x.to_string()),
            "y" => Some(component.geometry.bounds.y.to_string()),
            "width" => Some(component.geometry.bounds.width.to_string()),
            "height" => Some(component.geometry.bounds.height.to_string()),
            "index" | "siblingindex" | "sibling_index" => {
                Some(component.metadata.sibling_index.to_string())
            }
            "depth" => Some(component.id.depth.to_string()),
            "childcount" | "child_count" | "children" => {
                Some(component.metadata.child_count.to_string())
            }
            "accessiblename" | "accessible_name" | "accessible-name" => {
                component.accessibility.accessible_name.clone()
            }
            "accessibledescription" | "accessible_description" | "accessible-description" => {
                component.accessibility.accessible_description.clone()
            }
            "accessiblerole" | "accessible_role" | "accessible-role" => {
                component.accessibility.accessible_role.clone()
            }
            _ => {
                // Check generic properties if available
                None
            }
        }
    }

    /// Match a value against a target value with the given operator
    fn match_value(&self, operator: &MatchOperator, component_value: &str, target: &AttributeValue) -> bool {
        match operator {
            MatchOperator::Equals => {
                let target_str = target.as_str().unwrap_or("");
                self.string_equals(component_value, target_str)
            }
            MatchOperator::NotEquals => {
                let target_str = target.as_str().unwrap_or("");
                !self.string_equals(component_value, target_str)
            }
            MatchOperator::PrefixMatch => {
                let target_str = target.as_str().unwrap_or("");
                if self.case_sensitive {
                    component_value.starts_with(target_str)
                } else {
                    component_value
                        .to_lowercase()
                        .starts_with(&target_str.to_lowercase())
                }
            }
            MatchOperator::SuffixMatch => {
                let target_str = target.as_str().unwrap_or("");
                if self.case_sensitive {
                    component_value.ends_with(target_str)
                } else {
                    component_value
                        .to_lowercase()
                        .ends_with(&target_str.to_lowercase())
                }
            }
            MatchOperator::SubstringMatch => {
                let target_str = target.as_str().unwrap_or("");
                if self.case_sensitive {
                    component_value.contains(target_str)
                } else {
                    component_value
                        .to_lowercase()
                        .contains(&target_str.to_lowercase())
                }
            }
            MatchOperator::WordMatch => {
                let target_str = target.as_str().unwrap_or("");
                component_value
                    .split_whitespace()
                    .any(|word| self.string_equals(word, target_str))
            }
            MatchOperator::DashMatch => {
                let target_str = target.as_str().unwrap_or("");
                let lower = if self.case_sensitive {
                    component_value.to_string()
                } else {
                    component_value.to_lowercase()
                };
                let target_lower = if self.case_sensitive {
                    target_str.to_string()
                } else {
                    target_str.to_lowercase()
                };
                lower == target_lower || lower.starts_with(&format!("{}-", target_lower))
            }
            MatchOperator::RegexMatch => {
                let pattern = target.as_str().unwrap_or("");
                self.regex_matches(component_value, pattern)
            }
            MatchOperator::LessThan => {
                let comp_num: f64 = component_value.parse().unwrap_or(f64::MAX);
                let target_num = target.as_number().unwrap_or(f64::MIN);
                comp_num < target_num
            }
            MatchOperator::LessOrEqual => {
                let comp_num: f64 = component_value.parse().unwrap_or(f64::MAX);
                let target_num = target.as_number().unwrap_or(f64::MIN);
                comp_num <= target_num
            }
            MatchOperator::GreaterThan => {
                let comp_num: f64 = component_value.parse().unwrap_or(f64::MIN);
                let target_num = target.as_number().unwrap_or(f64::MAX);
                comp_num > target_num
            }
            MatchOperator::GreaterOrEqual => {
                let comp_num: f64 = component_value.parse().unwrap_or(f64::MIN);
                let target_num = target.as_number().unwrap_or(f64::MAX);
                comp_num >= target_num
            }
        }
    }

    /// Match a pseudo selector
    fn match_pseudo_selector(
        &self,
        pseudo: &PseudoSelector,
        component: &UIComponent,
        context: &MatchContext,
    ) -> bool {
        match pseudo {
            // Structural pseudo-classes
            PseudoSelector::FirstChild => context.is_first_child(),
            PseudoSelector::LastChild => context.is_last_child(),
            PseudoSelector::NthChild(expr) => expr.matches(context.position() as i32),
            PseudoSelector::NthLastChild(expr) => expr.matches(context.position_from_end() as i32),
            PseudoSelector::OnlyChild => context.sibling_count() == 1,
            PseudoSelector::FirstOfType => {
                self.is_first_of_type(component, context)
            }
            PseudoSelector::LastOfType => {
                self.is_last_of_type(component, context)
            }
            PseudoSelector::NthOfType(expr) => {
                let type_index = self.get_type_index(component, context);
                expr.matches(type_index as i32)
            }
            PseudoSelector::NthLastOfType(expr) => {
                let type_index_from_end = self.get_type_index_from_end(component, context);
                expr.matches(type_index_from_end as i32)
            }
            PseudoSelector::OnlyOfType => {
                self.is_only_of_type(component, context)
            }
            PseudoSelector::Empty => {
                component.metadata.child_count == 0
            }
            PseudoSelector::Root => {
                context.parent.is_none()
            }

            // State pseudo-classes
            PseudoSelector::Enabled => component.state.enabled,
            PseudoSelector::Disabled => !component.state.enabled,
            PseudoSelector::Visible => component.state.visible,
            PseudoSelector::Hidden => !component.state.visible,
            PseudoSelector::Showing => component.state.showing,
            PseudoSelector::Focused => component.state.focused,
            PseudoSelector::Selected => component.state.selected.unwrap_or(false),
            PseudoSelector::Editable => component.state.editable.unwrap_or(false),
            PseudoSelector::ReadOnly => !component.state.editable.unwrap_or(true),

            // Functional pseudo-classes
            PseudoSelector::Not(inner) => {
                !self.evaluate_compound_selector(inner, component, context).matches
            }
            PseudoSelector::Has(inner) => {
                // Check if any descendant matches
                self.has_matching_descendant(inner, component)
            }
            PseudoSelector::Contains(text) => {
                self.component_contains_text(component, text)
            }
        }
    }

    /// Check if component is first of its type among siblings
    fn is_first_of_type(&self, component: &UIComponent, context: &MatchContext) -> bool {
        let component_type = &component.component_type.simple_name;
        for (idx, sibling) in context.siblings.iter().enumerate() {
            if sibling.component_type.simple_name == *component_type {
                return idx == context.sibling_index;
            }
        }
        true
    }

    /// Check if component is last of its type among siblings
    fn is_last_of_type(&self, component: &UIComponent, context: &MatchContext) -> bool {
        let component_type = &component.component_type.simple_name;
        for (idx, sibling) in context.siblings.iter().enumerate().rev() {
            if sibling.component_type.simple_name == *component_type {
                return idx == context.sibling_index;
            }
        }
        true
    }

    /// Get 1-based index among siblings of the same type
    fn get_type_index(&self, component: &UIComponent, context: &MatchContext) -> usize {
        let component_type = &component.component_type.simple_name;
        let mut index = 0;
        for (idx, sibling) in context.siblings.iter().enumerate() {
            if sibling.component_type.simple_name == *component_type {
                index += 1;
                if idx == context.sibling_index {
                    return index;
                }
            }
        }
        index
    }

    /// Get 1-based index from end among siblings of the same type
    fn get_type_index_from_end(&self, component: &UIComponent, context: &MatchContext) -> usize {
        let component_type = &component.component_type.simple_name;
        let total_of_type: usize = context
            .siblings
            .iter()
            .filter(|s| s.component_type.simple_name == *component_type)
            .count();
        let forward_index = self.get_type_index(component, context);
        total_of_type - forward_index + 1
    }

    /// Check if component is the only one of its type among siblings
    fn is_only_of_type(&self, component: &UIComponent, context: &MatchContext) -> bool {
        let component_type = &component.component_type.simple_name;
        context
            .siblings
            .iter()
            .filter(|s| s.component_type.simple_name == *component_type)
            .count()
            == 1
    }

    /// Check if any descendant matches the selector
    fn has_matching_descendant(&self, selector: &CompoundSelector, component: &UIComponent) -> bool {
        if let Some(ref children) = component.children {
            for (idx, child) in children.iter().enumerate() {
                let child_ctx = MatchContext::with_parent(
                    child,
                    component,
                    children.iter().collect(),
                    idx,
                );
                if self.evaluate_compound_selector(selector, child, &child_ctx).matches {
                    return true;
                }
                // Recursively check descendants
                if self.has_matching_descendant(selector, child) {
                    return true;
                }
            }
        }
        false
    }

    /// Check if component contains the given text
    fn component_contains_text(&self, component: &UIComponent, text: &str) -> bool {
        // Check text property
        if let Some(ref comp_text) = component.identity.text {
            if self.case_sensitive {
                if comp_text.contains(text) {
                    return true;
                }
            } else if comp_text.to_lowercase().contains(&text.to_lowercase()) {
                return true;
            }
        }

        // Check title
        if let Some(ref title) = component.identity.title {
            if self.case_sensitive {
                if title.contains(text) {
                    return true;
                }
            } else if title.to_lowercase().contains(&text.to_lowercase()) {
                return true;
            }
        }

        // Check accessible name
        if let Some(ref acc_name) = component.accessibility.accessible_name {
            if self.case_sensitive {
                if acc_name.contains(text) {
                    return true;
                }
            } else if acc_name.to_lowercase().contains(&text.to_lowercase()) {
                return true;
            }
        }

        false
    }

    /// Compare strings with case sensitivity setting
    fn string_equals(&self, a: &str, b: &str) -> bool {
        if self.case_sensitive {
            a == b
        } else {
            a.eq_ignore_ascii_case(b)
        }
    }

    /// Match with regex, using cache
    fn regex_matches(&self, value: &str, pattern: &str) -> bool {
        let mut cache = self.regex_cache.borrow_mut();

        // Check cache
        if let Some(regex) = cache.get(pattern) {
            return regex.is_match(value);
        }

        // Compile and cache
        match Regex::new(pattern) {
            Ok(regex) => {
                let result = regex.is_match(value);
                cache.put(pattern.to_string(), regex);
                result
            }
            Err(_) => false,
        }
    }
}

/// Find all components matching a locator in a tree
pub fn find_matching_components<'a>(
    locator: &Locator,
    root: &'a UIComponent,
    evaluator: &Evaluator,
) -> Vec<&'a UIComponent> {
    // Check if any selector has cascaded segments with capture
    for selector in &locator.selectors {
        if selector.is_cascaded() && selector.has_capture() {
            // Use cascaded matching with capture support
            if let Ok(results) = find_cascaded_with_capture(selector, root, evaluator) {
                return results;
            }
        } else if selector.is_cascaded() {
            // Use cascaded matching without capture
            if let Ok(results) = find_cascaded(selector, root, evaluator) {
                return results;
            }
        }
    }

    // Fallback to standard matching
    let mut results = Vec::new();
    find_recursive(locator, root, None, Vec::new(), &[], 0, evaluator, &mut results);
    results
}

/// Find elements matching a cascaded locator with capture support
fn find_cascaded_with_capture<'a>(
    selector: &ComplexSelector,
    root: &'a UIComponent,
    evaluator: &Evaluator,
) -> Result<Vec<&'a UIComponent>, ()> {
    // Get cascaded segments
    let segments = selector.get_cascaded_segments().ok_or(())?;

    // Track state through the cascade
    let mut current_contexts: Vec<&UIComponent> = vec![root];
    let mut captured_elements: Option<Vec<&UIComponent>> = None;

    // Process each segment
    for segment in segments.iter() {
        let mut next_contexts = Vec::new();

        // Search within each current context
        for context in &current_contexts {
            // Find all descendants matching this segment's selector
            let matches = find_in_context(&segment.compound, context, evaluator);

            // Add matches to next contexts (remove duplicates by comparing IDs)
            for match_elem in matches {
                if !next_contexts.iter().any(|e: &&UIComponent| e.id == match_elem.id) {
                    next_contexts.push(match_elem);
                }
            }
        }

        // If this segment has capture flag and we haven't captured yet
        if segment.capture && captured_elements.is_none() {
            // This is the first segment with capture flag
            captured_elements = Some(next_contexts.clone());
        }

        // Check if we found anything
        if next_contexts.is_empty() {
            // No matches found for this segment, return empty
            return Ok(Vec::new());
        }

        // Continue with next segment using these results as new contexts
        current_contexts = next_contexts;
    }

    // Return captured elements if any, otherwise final results
    Ok(captured_elements.unwrap_or(current_contexts))
}

/// Find elements matching a cascaded locator (without capture)
fn find_cascaded<'a>(
    selector: &ComplexSelector,
    root: &'a UIComponent,
    evaluator: &Evaluator,
) -> Result<Vec<&'a UIComponent>, ()> {
    // Get cascaded segments
    let segments = selector.get_cascaded_segments().ok_or(())?;

    // Track state through the cascade
    let mut current_contexts: Vec<&UIComponent> = vec![root];

    // Process each segment
    for segment in segments.iter() {
        let mut next_contexts = Vec::new();

        // Search within each current context
        for context in &current_contexts {
            // Find all descendants matching this segment's selector
            let matches = find_in_context(&segment.compound, context, evaluator);

            // Add matches to next contexts (remove duplicates by comparing IDs)
            for match_elem in matches {
                if !next_contexts.iter().any(|e: &&UIComponent| e.id == match_elem.id) {
                    next_contexts.push(match_elem);
                }
            }
        }

        // Check if we found anything
        if next_contexts.is_empty() {
            // No matches found for this segment, return empty
            return Ok(Vec::new());
        }

        // Continue with next segment using these results as new contexts
        current_contexts = next_contexts;
    }

    // Return final results
    Ok(current_contexts)
}

/// Find elements matching a compound selector within a specific context
fn find_in_context<'a>(
    compound: &CompoundSelector,
    context: &'a UIComponent,
    evaluator: &Evaluator,
) -> Vec<&'a UIComponent> {
    let mut results = Vec::new();

    // Search all descendants of the context component
    // (but not the context itself - that was matched by previous segment)
    if let Some(ref children) = context.children {
        for child in children {
            search_descendants_recursive(child, compound, evaluator, &mut results);
        }
    }

    results
}

/// Recursively search descendants for matches
fn search_descendants_recursive<'a>(
    component: &'a UIComponent,
    compound: &CompoundSelector,
    evaluator: &Evaluator,
    results: &mut Vec<&'a UIComponent>,
) {
    // Create a temporary context for evaluation
    let context = MatchContext::new(component);

    // Check if current component matches
    if evaluator.evaluate_compound_selector(compound, component, &context).matches {
        results.push(component);
    }

    // Recurse into children
    if let Some(ref children) = component.children {
        for child in children {
            search_descendants_recursive(child, compound, evaluator, results);
        }
    }
}

/// Recursive helper for finding matching components
fn find_recursive<'a>(
    locator: &Locator,
    component: &'a UIComponent,
    parent: Option<&'a UIComponent>,
    ancestors: Vec<&'a UIComponent>,
    siblings: &[&'a UIComponent],
    sibling_index: usize,
    evaluator: &Evaluator,
    results: &mut Vec<&'a UIComponent>,
) {
    let context = MatchContext::with_ancestors(
        component,
        parent,
        ancestors.clone(),
        siblings.to_vec(),
        sibling_index,
    );

    if evaluator.evaluate(locator, component, &context).matches {
        results.push(component);
    }

    // Recurse into children - build ancestor chain
    if let Some(ref children) = component.children {
        let child_refs: Vec<&UIComponent> = children.iter().collect();
        // Build new ancestor chain: current component + existing ancestors
        let mut child_ancestors = vec![component];
        child_ancestors.extend(ancestors.iter().copied());

        for (idx, child) in children.iter().enumerate() {
            find_recursive(
                locator,
                child,
                Some(component),
                child_ancestors.clone(),
                &child_refs,
                idx,
                evaluator,
                results,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::*;

    fn create_test_component(name: &str, type_name: &str) -> UIComponent {
        UIComponent {
            id: ComponentId::new(1, "0".to_string(), 0),
            component_type: ComponentType {
                class_name: format!("javax.swing.{}", type_name),
                simple_name: type_name.to_string(),
                base_type: SwingBaseType::from_class_name(type_name),
                interfaces: vec![],
                class_hierarchy: vec![],
            },
            identity: ComponentIdentity {
                name: Some(name.to_string()),
                text: Some(format!("{} Text", name)),
                internal_name: None,
                title: None,
                tooltip: None,
                action_command: None,
                label_text: None,
            },
            geometry: ComponentGeometry::default(),
            state: ComponentState::default(),
            properties: ComponentProperties::default(),
            accessibility: AccessibilityInfo::default(),
            children: None,
            parent_id: None,
            metadata: TraversalMetadata::default(),
        }
    }

    #[test]
    fn test_match_type_selector() {
        let evaluator = Evaluator::new();
        let component = create_test_component("btn1", "JButton");
        let type_sel = TypeSelector::TypeName("JButton".to_string());

        assert!(evaluator.match_type_selector(&type_sel, &component));

        let type_sel2 = TypeSelector::TypeName("Button".to_string());
        assert!(evaluator.match_type_selector(&type_sel2, &component));

        let type_sel3 = TypeSelector::TypeName("JTextField".to_string());
        assert!(!evaluator.match_type_selector(&type_sel3, &component));

        let universal = TypeSelector::Universal;
        assert!(evaluator.match_type_selector(&universal, &component));
    }

    #[test]
    fn test_match_id_selector() {
        let evaluator = Evaluator::new();
        let mut component = create_test_component("btn1", "JButton");
        component.identity.internal_name = Some("submitBtn".to_string());

        assert!(evaluator.match_id_selector("submitBtn", &component));
        assert!(evaluator.match_id_selector("btn1", &component));
        assert!(!evaluator.match_id_selector("unknown", &component));
    }

    #[test]
    fn test_match_attribute_selector() {
        let evaluator = Evaluator::new();
        let component = create_test_component("btn1", "JButton");

        // Existence check
        let exists_sel = AttributeSelector::exists("text".to_string());
        assert!(evaluator.match_attribute_selector(&exists_sel, &component));

        // Equals check
        let equals_sel = AttributeSelector::equals(
            "name".to_string(),
            AttributeValue::String("btn1".to_string()),
        );
        assert!(evaluator.match_attribute_selector(&equals_sel, &component));

        // Contains check
        let contains_sel = AttributeSelector::with_operator(
            "text".to_string(),
            MatchOperator::SubstringMatch,
            AttributeValue::String("Text".to_string()),
        );
        assert!(evaluator.match_attribute_selector(&contains_sel, &component));
    }

    #[test]
    fn test_match_pseudo_selector_state() {
        let evaluator = Evaluator::new();

        // Test enabled (default)
        {
            let component = create_test_component("btn1", "JButton");
            let context = MatchContext::new(&component);
            assert!(evaluator.match_pseudo_selector(&PseudoSelector::Enabled, &component, &context));
            assert!(!evaluator.match_pseudo_selector(&PseudoSelector::Disabled, &component, &context));
        }

        // Test disabled
        {
            let mut component = create_test_component("btn1", "JButton");
            component.state.enabled = false;
            let context = MatchContext::new(&component);
            assert!(!evaluator.match_pseudo_selector(&PseudoSelector::Enabled, &component, &context));
            assert!(evaluator.match_pseudo_selector(&PseudoSelector::Disabled, &component, &context));
        }

        // Test visible/hidden
        {
            let component = create_test_component("btn1", "JButton");
            let context = MatchContext::new(&component);
            assert!(evaluator.match_pseudo_selector(&PseudoSelector::Visible, &component, &context));
            assert!(!evaluator.match_pseudo_selector(&PseudoSelector::Hidden, &component, &context));
        }
    }

    #[test]
    fn test_match_pseudo_selector_structural() {
        let evaluator = Evaluator::new();
        let component = create_test_component("btn1", "JButton");
        let sibling1 = create_test_component("btn2", "JButton");
        let sibling2 = create_test_component("lbl1", "JLabel");

        let siblings: Vec<&UIComponent> = vec![&component, &sibling1, &sibling2];

        // First child context
        let first_ctx = MatchContext {
            root: &component,
            parent: None,
            ancestors: vec![],
            siblings: siblings.clone(),
            sibling_index: 0,
        };
        assert!(evaluator.match_pseudo_selector(&PseudoSelector::FirstChild, &component, &first_ctx));
        assert!(!evaluator.match_pseudo_selector(&PseudoSelector::LastChild, &component, &first_ctx));

        // Last child context
        let last_ctx = MatchContext {
            root: &sibling2,
            parent: None,
            ancestors: vec![],
            siblings: siblings.clone(),
            sibling_index: 2,
        };
        assert!(evaluator.match_pseudo_selector(&PseudoSelector::LastChild, &sibling2, &last_ctx));
        assert!(!evaluator.match_pseudo_selector(&PseudoSelector::FirstChild, &sibling2, &last_ctx));

        // nth-child
        let middle_ctx = MatchContext {
            root: &sibling1,
            parent: None,
            ancestors: vec![],
            siblings: siblings.clone(),
            sibling_index: 1,
        };
        assert!(evaluator.match_pseudo_selector(
            &PseudoSelector::NthChild(NthExpr::Index(2)),
            &sibling1,
            &middle_ctx
        ));
    }

    #[test]
    fn test_match_contains() {
        let evaluator = Evaluator::new();
        let component = create_test_component("btn1", "JButton");
        let context = MatchContext::new(&component);

        assert!(evaluator.match_pseudo_selector(
            &PseudoSelector::Contains("Text".to_string()),
            &component,
            &context
        ));
        assert!(!evaluator.match_pseudo_selector(
            &PseudoSelector::Contains("NotFound".to_string()),
            &component,
            &context
        ));
    }

    #[test]
    fn test_regex_matching() {
        let evaluator = Evaluator::new();

        assert!(evaluator.regex_matches("Hello World", r"Hello.*"));
        assert!(evaluator.regex_matches("test123", r"\d+"));
        assert!(!evaluator.regex_matches("abc", r"\d+"));
    }

    #[test]
    fn test_value_matching_operators() {
        let evaluator = Evaluator::new();

        // Prefix match
        assert!(evaluator.match_value(
            &MatchOperator::PrefixMatch,
            "Hello World",
            &AttributeValue::String("Hello".to_string())
        ));

        // Suffix match
        assert!(evaluator.match_value(
            &MatchOperator::SuffixMatch,
            "Hello World",
            &AttributeValue::String("World".to_string())
        ));

        // Word match
        assert!(evaluator.match_value(
            &MatchOperator::WordMatch,
            "foo bar baz",
            &AttributeValue::String("bar".to_string())
        ));

        // Numeric comparisons
        assert!(evaluator.match_value(
            &MatchOperator::GreaterThan,
            "100",
            &AttributeValue::Number(50.0)
        ));
        assert!(evaluator.match_value(
            &MatchOperator::LessThan,
            "25",
            &AttributeValue::Number(50.0)
        ));
    }
}
