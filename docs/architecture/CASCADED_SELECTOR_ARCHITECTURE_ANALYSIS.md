# Cascaded Selector Architecture Analysis

## Document Information

| Field | Value |
|-------|-------|
| Version | 1.0.0 |
| Status | Technical Architecture Analysis |
| Author | System Architecture Designer |
| Date | 2026-01-21 |
| Related Documents | [Implementation Plan](../specs/CASCADED_SELECTOR_IMPLEMENTATION_PLAN.md), [Specification](../specs/CASCADED_SELECTOR_SPECIFICATION.md) |

---

## Executive Summary

This document provides a comprehensive technical architecture analysis of the robotframework-swing locator/selector implementation, focusing on the areas that need modification to implement:

1. **Capture Prefix (`*`) Feature** - Returns intermediate elements in cascade chains
2. **Class Engine Bug Fix** - Fixes empty result issue in cascaded contexts

The analysis reveals that the basic cascaded selector infrastructure (`>>` combinator) is already implemented but **two critical features are completely missing**:
- No AST support for capture flags
- No specialized cascaded matching algorithm

---

## 1. Current Architecture Overview

### 1.1 Component Structure

The locator subsystem consists of six main modules:

```
src/locator/
├── mod.rs              # Module exports and public API
├── ast.rs              # Abstract Syntax Tree definitions
├── grammar.pest        # PEG grammar for parsing
├── parser.rs           # Parser implementation (pest-based)
├── matcher.rs          # Element matching and evaluation logic
├── expression.rs       # High-level locator expressions
├── unified.rs          # Cross-toolkit locator support
└── swt_matcher.rs      # SWT-specific matching logic
```

### 1.2 Data Flow Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Locator String Input                      │
│              "JPanel >> *JButton[name='x']"                 │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  PARSER (parser.rs)                          │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Grammar (grammar.pest) - PEG Rules                    │  │
│  │ • Tokenization via pest parser                        │  │
│  │ • Validates syntax                                    │  │
│  │ • Produces parse tree (Pairs)                         │  │
│  └───────────────────────────────────────────────────────┘  │
│                           │                                  │
│                           ▼                                  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Parse Functions (parser.rs)                           │  │
│  │ • parse_locator()         - Entry point               │  │
│  │ • parse_complex_selector() - Builds ComplexSelector   │  │
│  │ • parse_compound_selector() - Builds CompoundSelector │  │
│  │ • parse_combinator()       - Identifies >> vs > etc   │  │
│  └───────────────────────────────────────────────────────┘  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                    AST (ast.rs)                              │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Locator                                                │  │
│  │ └─ selectors: Vec<ComplexSelector>                    │  │
│  │    └─ compounds: Vec<CompoundSelector>                │  │
│  │       ├─ type_selector: Option<TypeSelector>          │  │
│  │       ├─ id_selector: Option<String>                  │  │
│  │       ├─ class_selectors: Vec<String>                 │  │
│  │       ├─ attribute_selectors: Vec<AttributeSelector>  │  │
│  │       ├─ pseudo_selectors: Vec<PseudoSelector>        │  │
│  │       └─ combinator: Option<Combinator>               │  │
│  │          └─ Cascaded variant EXISTS ✓                 │  │
│  └───────────────────────────────────────────────────────┘  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  MATCHER (matcher.rs)                        │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Evaluator                                              │  │
│  │ • evaluate() - Main entry point                       │  │
│  │ • evaluate_complex_selector()                         │  │
│  │ • match_combinator_chain()                            │  │
│  │   ├─ Combinator::Descendant  ✓                        │  │
│  │   ├─ Combinator::Child  ✓                             │  │
│  │   ├─ Combinator::AdjacentSibling  ✓                   │  │
│  │   ├─ Combinator::GeneralSibling  ✓                    │  │
│  │   └─ Combinator::Cascaded  ⚠️ (same as Descendant)   │  │
│  │                                                        │  │
│  │ • evaluate_compound_selector()                        │  │
│  │   ├─ match_type_selector()                            │  │
│  │   ├─ match_id_selector()                              │  │
│  │   ├─ match_class_selector()                           │  │
│  │   ├─ match_attribute_selector()                       │  │
│  │   └─ match_pseudo_selector()                          │  │
│  └───────────────────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────────────────┐  │
│  │ Public API                                             │  │
│  │ • find_matching_components() - Tree traversal         │  │
│  │ • find_recursive() - Recursive search                 │  │
│  └───────────────────────────────────────────────────────┘  │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│              Results: Vec<&UIComponent>                      │
└─────────────────────────────────────────────────────────────┘
```

---

## 2. Detailed Component Analysis

### 2.1 AST Module (`ast.rs`)

**File**: `/mnt/c/workspace/robotframework-swing/src/locator/ast.rs`
**Lines**: 441 lines
**Current State**: ✅ Partial Support for Cascaded

#### Key Structures

```rust
/// Root locator type (lines 9-56)
pub struct Locator {
    pub selectors: Vec<ComplexSelector>,
    pub original: String,
    pub is_xpath: bool,
}

/// Complex selector: chain of compounds with combinators (lines 64-83)
pub struct ComplexSelector {
    pub compounds: Vec<CompoundSelector>,
    // ❌ MISSING: pub cascaded_segments: Option<Vec<CascadedSegment>>
}

/// Compound selector: type + modifiers (lines 112-143)
pub struct CompoundSelector {
    pub type_selector: Option<TypeSelector>,
    pub id_selector: Option<String>,
    pub class_selectors: Vec<String>,
    pub attribute_selectors: Vec<AttributeSelector>,
    pub pseudo_selectors: Vec<PseudoSelector>,
    pub combinator: Option<Combinator>,
    // ❌ MISSING: No capture flag
}

/// Combinator types (lines 85-110)
pub enum Combinator {
    Descendant,      // whitespace
    Child,           // >
    AdjacentSibling, // +
    GeneralSibling,  // ~
    Cascaded,        // >> ✓ EXISTS
}
```

#### Analysis: What's Missing

**Critical Missing Structure**:

```rust
// ❌ NOT PRESENT - NEEDS TO BE ADDED
pub struct CascadedSegment {
    pub capture: bool,               // ⭐ KEY FIELD FOR CAPTURE PREFIX
    pub compound: CompoundSelector,
    pub combinator: Option<Combinator>,
    pub raw: String,                 // For error messages
}
```

**Required Additions to ComplexSelector**:

```rust
pub struct ComplexSelector {
    pub compounds: Vec<CompoundSelector>,

    // ⭐ ADD THIS FIELD:
    pub cascaded_segments: Option<Vec<CascadedSegment>>,
}

impl ComplexSelector {
    // ⭐ ADD THESE HELPER METHODS:
    pub fn is_cascaded(&self) -> bool { /* ... */ }
    pub fn has_capture(&self) -> bool { /* ... */ }
    pub fn get_capture_index(&self) -> Option<usize> { /* ... */ }
}
```

#### Modification Plan for AST

**Priority 1: Add CascadedSegment struct**

Location: After `CompoundSelector` definition (around line 143)

```rust
/// A segment in a cascaded selector chain with capture support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CascadedSegment {
    /// Whether this segment should be captured (marked with *)
    pub capture: bool,

    /// The compound selector for this segment
    pub compound: CompoundSelector,

    /// Combinator to next segment (None for last segment)
    pub combinator: Option<Combinator>,

    /// Original raw text (for debugging/error messages)
    pub raw: String,
}
```

**Priority 2: Enhance ComplexSelector**

Location: Lines 64-83

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSelector {
    /// All compound selectors in this chain
    pub compounds: Vec<CompoundSelector>,

    /// For cascaded selectors: structured segments with capture flags
    pub cascaded_segments: Option<Vec<CascadedSegment>>,
}
```

**Priority 3: Add helper methods**

Location: After existing `ComplexSelector` impl blocks

```rust
impl ComplexSelector {
    /// Check if this selector uses the cascaded combinator (>>)
    pub fn is_cascaded(&self) -> bool {
        self.compounds.iter().any(|c| {
            matches!(c.combinator, Some(Combinator::Cascaded))
        })
    }

    /// Check if any segment has the capture flag
    pub fn has_capture(&self) -> bool {
        self.cascaded_segments
            .as_ref()
            .map(|segs| segs.iter().any(|s| s.capture))
            .unwrap_or(false)
    }

    /// Get the index of the first captured segment
    pub fn get_capture_index(&self) -> Option<usize> {
        self.cascaded_segments
            .as_ref()
            .and_then(|segs| {
                segs.iter().position(|s| s.capture)
            })
    }
}
```

---

### 2.2 Grammar Module (`grammar.pest`)

**File**: `/mnt/c/workspace/robotframework-swing/src/locator/grammar.pest`
**Lines**: 341 lines
**Current State**: ✅ Has Cascaded Combinator, ❌ Missing Capture Prefix

#### Current Grammar Structure

```pest
// Top-level (lines 9-15)
locator = _{ SOI ~ (xpath_expr | css_selector_list) ~ EOI }
css_selector_list = { complex_selector ~ ("," ~ explicit_ws* ~ complex_selector)* }
complex_selector = { compound_selector ~ (combinator ~ compound_selector)* }

// Compound selector (lines 18-21) - NO CAPTURE SUPPORT
compound_selector = {
    type_selector? ~
    (id_selector | class_selector | attribute_selector | pseudo_selector)*
}

// Type selector (lines 28-34)
type_selector = { universal_selector | type_name }
universal_selector = { "*" }  // ⚠️ CONFLICTS WITH CAPTURE PREFIX
type_name = @{ (ASCII_ALPHA | "_") ~ (ASCII_ALPHANUMERIC | "_" | "-")* }

// Combinators (lines 171-193)
combinator = {
    cascaded_combinator |      // >> ✓ EXISTS
    child_combinator |         // >
    adjacent_sibling |         // +
    general_sibling |          // ~
    descendant_combinator      // whitespace
}

cascaded_combinator = { explicit_ws* ~ ">>" ~ explicit_ws* }  // ✓ WORKS
```

#### Analysis: Grammar Conflict Issue

**CRITICAL PROBLEM**: The `*` character is currently used for:
1. Universal selector (`*` matches any element)
2. Substring match operator (`*=` in attributes)

**Solution**: Context-sensitive parsing
- `*` at the **start of a compound selector** before any other token → Capture prefix
- `*` alone or after whitespace/combinator → Universal selector
- `*=` → Substring match operator (different context)

#### Modification Plan for Grammar

**Priority 1: Add capture_prefix rule**

Location: Before `compound_selector` (around line 17)

```pest
/// Capture prefix for cascaded segments
capture_prefix = { "*" ~ &(type_name | id_selector | class_selector | attribute_selector) }
```

The `&` lookahead ensures `*` is only treated as capture if followed by a selector component.

**Priority 2: Update compound_selector rule**

Location: Lines 18-21

```pest
/// Compound selector with optional capture prefix
compound_selector = {
    capture_prefix? ~
    type_selector? ~
    (id_selector | class_selector | attribute_selector | pseudo_selector)*
}
```

**Priority 3: Ensure universal_selector doesn't conflict**

Current universal_selector is fine because:
- `*` alone → universal selector (matches type_selector rule)
- `*JButton` → capture prefix + type selector (capture_prefix matches first due to lookahead)
- `* JButton` → universal + descendant + JButton (space breaks capture context)

**Test Cases for Grammar Validation**:

```pest
// Should parse as: capture=true, type=JPanel
*JPanel

// Should parse as: capture=true, type=JButton, attr=[name='x']
*JButton[name='x']

// Should parse as: universal selector, descendant, type=JButton
* JButton

// Should parse as: type=JPanel, cascaded, capture=true, type=JButton
JPanel >> *JButton

// Should parse as: universal selector
*

// Should parse as: attr with substring match
[text*='hello']
```

---

### 2.3 Parser Module (`parser.rs`)

**File**: `/mnt/c/workspace/robotframework-swing/src/locator/parser.rs`
**Lines**: ~1000+ lines
**Current State**: ✅ Parses `>>` combinator, ❌ Doesn't build cascaded segments

#### Current Parsing Flow

```rust
// Main entry point (lines 128-185)
pub fn parse_locator(input: &str) -> Result<Locator, ParseError> {
    // 1. Tokenize with pest
    let pairs = LocatorParser::parse(Rule::locator, trimmed)?;

    // 2. Build AST
    for pair in pairs {
        match pair.as_rule() {
            Rule::css_selector_list => {
                for selector_pair in pair.into_inner() {
                    selectors.push(parse_complex_selector(selector_pair)?);
                }
            }
            // ...
        }
    }

    Ok(Locator::new(selectors, input, is_xpath))
}

// Complex selector parsing (lines 188-212)
fn parse_complex_selector(pair: Pair<Rule>) -> Result<ComplexSelector, ParseError> {
    let mut compounds: Vec<CompoundSelector> = Vec::new();
    let mut current_combinator: Option<Combinator> = None;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::compound_selector => {
                let compound = parse_compound_selector(inner)?;
                if let Some(comb) = current_combinator.take() {
                    // Store combinator in previous compound
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

    // ❌ PROBLEM: Just returns basic ComplexSelector
    Ok(ComplexSelector::from_compounds(compounds))
}
```

#### Analysis: Missing Capture Logic

The parser currently:
1. ✅ Recognizes `>>` combinator correctly
2. ✅ Stores combinator in `CompoundSelector.combinator` field
3. ❌ **Does NOT check for capture prefix**
4. ❌ **Does NOT build `CascadedSegment` structures**
5. ❌ **Does NOT populate `cascaded_segments` field**

#### Modification Plan for Parser

**Priority 1: Update parse_compound_selector to detect capture**

Location: Lines 214-240

```rust
fn parse_compound_selector(pair: Pair<Rule>) -> Result<(bool, CompoundSelector), ParseError> {
    let mut compound = CompoundSelector::new();
    let mut has_capture = false;

    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::capture_prefix => {
                has_capture = true;
            }
            Rule::type_selector => {
                compound.type_selector = Some(parse_type_selector(inner)?);
            }
            // ... rest of parsing
            _ => {}
        }
    }

    Ok((has_capture, compound))
}
```

**Priority 2: Update parse_complex_selector to build cascaded segments**

Location: Lines 188-212

```rust
fn parse_complex_selector(pair: Pair<Rule>) -> Result<ComplexSelector, ParseError> {
    let mut compounds: Vec<CompoundSelector> = Vec::new();
    let mut cascaded_segments: Vec<CascadedSegment> = Vec::new();
    let mut is_cascaded = false;
    let mut current_combinator: Option<Combinator> = None;
    let mut current_raw = String::new();

    for inner in pair.into_inner() {
        let raw_text = inner.as_str();

        match inner.as_rule() {
            Rule::compound_selector => {
                let (has_capture, compound) = parse_compound_selector(inner)?;
                current_raw.push_str(raw_text);

                if let Some(comb) = current_combinator.take() {
                    // This is NOT the first segment
                    if matches!(comb, Combinator::Cascaded) {
                        is_cascaded = true;
                    }

                    // Store previous segment
                    if is_cascaded {
                        let prev_compound = compounds.last().unwrap().clone();
                        let prev_capture = /* track this */ false;
                        cascaded_segments.push(CascadedSegment {
                            capture: prev_capture,
                            compound: prev_compound,
                            combinator: Some(comb),
                            raw: current_raw.clone(),
                        });
                    }

                    // Update previous compound's combinator
                    if let Some(prev) = compounds.last_mut() {
                        prev.combinator = Some(comb);
                    }
                }

                compounds.push(compound);
                current_raw.clear();
            }
            Rule::combinator => {
                current_combinator = Some(parse_combinator(inner)?);
                current_raw.push_str(raw_text);
            }
            _ => {}
        }
    }

    // Push final segment if cascaded
    if is_cascaded {
        let last_compound = compounds.last().unwrap().clone();
        cascaded_segments.push(CascadedSegment {
            capture: /* track from last compound */ false,
            compound: last_compound,
            combinator: None,
            raw: current_raw,
        });
    }

    Ok(ComplexSelector {
        compounds,
        cascaded_segments: if is_cascaded { Some(cascaded_segments) } else { None },
    })
}
```

**Complexity Note**: Parser needs to track capture flags per segment, requiring state management during parsing.

---

### 2.4 Matcher Module (`matcher.rs`)

**File**: `/mnt/c/workspace/robotframework-swing/src/locator/matcher.rs`
**Lines**: ~1100 lines
**Current State**: ⚠️ **CRITICAL ISSUE** - Cascaded matching is WRONG

#### Current Matching Architecture

```rust
// Main public API (lines 818-826)
pub fn find_matching_components<'a>(
    locator: &Locator,
    root: &'a UIComponent,
    evaluator: &Evaluator,
) -> Vec<&'a UIComponent> {
    let mut results = Vec::new();
    find_recursive(locator, root, None, Vec::new(), &[], 0, evaluator, &mut results);
    results
}

// Recursive tree traversal (lines 828-871)
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
    let context = MatchContext::with_ancestors(/* ... */);

    // ⚠️ PROBLEM: Tests EVERY component against the FULL selector
    if evaluator.evaluate(locator, component, &context).matches {
        results.push(component);
    }

    // Recurse into children
    if let Some(ref children) = component.children {
        for (idx, child) in children.iter().enumerate() {
            find_recursive(/* recurse with same selector */);
        }
    }
}
```

#### Critical Bug Analysis: Why Cascaded Matching is Wrong

**Current Algorithm**:
```
FOR each component in tree:
    IF component matches full selector "JPanel >> JButton":
        add to results
```

**Problem**: For cascaded selectors like `JPanel >> JButton`:
- Tests `JButton` (final) against full selector
- Checks: "Does JButton match AND does an ancestor match JPanel?"
- ✅ Returns JButton (correct for non-captured)
- ❌ **Cannot return JPanel** (no mechanism for capture)

**What's Needed**: Context-aware progressive matching
```
START with root as context
FOR each segment in cascade:
    FIND elements matching this segment within current contexts
    IF segment has capture flag:
        SAVE these elements as captured
    SET current contexts = found elements
RETURN captured elements OR final elements
```

#### Modification Plan for Matcher

**Priority 1: Add cascaded-specific matching function**

Location: After `find_matching_components` (around line 827)

```rust
/// Find elements using cascaded selector with capture support
pub fn find_cascaded<'a>(
    locator: &Locator,
    root: &'a UIComponent,
    evaluator: &Evaluator,
) -> Result<Vec<&'a UIComponent>, MatchError> {
    // Assume locator has exactly one selector (validated elsewhere)
    let selector = &locator.selectors[0];

    // Get cascaded segments (if any)
    let segments = match &selector.cascaded_segments {
        Some(segs) => segs,
        None => {
            // Not a cascaded selector, use standard matching
            return Ok(find_matching_components(locator, root, evaluator));
        }
    };

    // Start with root as initial context
    let mut current_contexts: Vec<&UIComponent> = vec![root];
    let mut captured_elements: Option<Vec<&UIComponent>> = None;

    // Process each segment in the cascade chain
    for (segment_idx, segment) in segments.iter().enumerate() {
        let mut next_contexts = Vec::new();

        // Search for matches within each current context
        for context in &current_contexts {
            let matches = find_in_context(&segment.compound, context, evaluator)?;

            // Deduplicate and add to next contexts
            for matched in matches {
                if !next_contexts.contains(&matched) {
                    next_contexts.push(matched);
                }
            }
        }

        // Check for capture flag on THIS segment
        if segment.capture && captured_elements.is_none() {
            // This is the first capture - save these results
            captured_elements = Some(next_contexts.clone());
        }

        // Verify we found something
        if next_contexts.is_empty() {
            return Err(MatchError::NoMatchInSegment {
                segment_index: segment_idx,
                selector: segment.raw.clone(),
                previous_matches: current_contexts.len(),
            });
        }

        // Use these results as contexts for next segment
        current_contexts = next_contexts;
    }

    // Return captured elements if any, otherwise final results
    Ok(captured_elements.unwrap_or(current_contexts))
}

/// Find elements matching a compound selector within a specific context
fn find_in_context<'a>(
    compound: &CompoundSelector,
    context: &'a UIComponent,
    evaluator: &Evaluator,
) -> Result<Vec<&'a UIComponent>, MatchError> {
    let mut results = Vec::new();

    // Search descendants of the context (not the context itself)
    if let Some(ref children) = context.children {
        for child in children {
            search_descendants_for_match(child, compound, evaluator, &mut results);
        }
    }

    Ok(results)
}

/// Recursively search descendants for matches
fn search_descendants_for_match<'a>(
    component: &'a UIComponent,
    compound: &CompoundSelector,
    evaluator: &Evaluator,
    results: &mut Vec<&'a UIComponent>,
) {
    // Create a temporary single-compound locator for matching
    let test_locator = Locator {
        selectors: vec![ComplexSelector::simple(compound.clone())],
        original: String::new(),
        is_xpath: false,
    };

    let context = MatchContext::new(component);
    if evaluator.evaluate(&test_locator, component, &context).matches {
        results.push(component);
    }

    // Recurse into children
    if let Some(ref children) = component.children {
        for child in children {
            search_descendants_for_match(child, compound, evaluator, results);
        }
    }
}
```

**Priority 2: Add error type for match failures**

Location: In `matcher.rs` or `error.rs`

```rust
#[derive(Debug, Clone)]
pub enum MatchError {
    NoMatchInSegment {
        segment_index: usize,
        selector: String,
        previous_matches: usize,
    },
    InvalidSelector(String),
}
```

**Priority 3: Update public API to route to cascaded matching**

Location: Modify `find_matching_components` (line 818)

```rust
pub fn find_matching_components<'a>(
    locator: &Locator,
    root: &'a UIComponent,
    evaluator: &Evaluator,
) -> Vec<&'a UIComponent> {
    // Check if this is a cascaded selector
    if locator.selectors.len() == 1 {
        let selector = &locator.selectors[0];
        if selector.is_cascaded() {
            // Use cascaded matching algorithm
            return find_cascaded(locator, root, evaluator)
                .unwrap_or_else(|_| Vec::new());
        }
    }

    // Standard matching for non-cascaded selectors
    let mut results = Vec::new();
    find_recursive(locator, root, None, Vec::new(), &[], 0, evaluator, &mut results);
    results
}
```

---

### 2.5 Current Cascaded Combinator Handling

#### Lines 308-322 in matcher.rs

```rust
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
```

**Analysis**: This code treats `Cascaded` exactly like `Descendant`, which is **incorrect** for:
1. Progressive context narrowing (segments should search within previous results)
2. Capture prefix handling (no mechanism to return intermediate elements)

**This is why the specification calls for a specialized matching algorithm**.

---

## 3. Class Engine Bug Analysis

### 3.1 Problem Statement

From the implementation plan:
```
Class Engine Tests: 5/8 passed (63%)
All 3 failures: '[]' should not be empty
```

Test cases failing:
```robot
class=JPanel >> class=JButton          → Returns []
class=JPanel >> JButton[name='x']      → Returns []
JPanel[name='y'] >> class=JButton      → Returns []
```

### 3.2 Root Cause Hypothesis

**Issue**: The `class=` engine prefix is not being handled correctly in cascaded contexts.

**Evidence Needed**: Search for class engine implementation

```bash
# No results for class engine in matcher.rs
grep -n "class=\|class_engine\|find_by_class" matcher.rs
# (returned empty)
```

**Conclusion**: The `class=` prefix engine is likely handled in:
1. **expression.rs** - High-level locator expressions
2. **unified.rs** - Cross-toolkit unified locators

### 3.3 Investigation Plan

**Step 1**: Examine how engine prefixes are parsed

```rust
// Check parser.rs for engine prefix handling
// Look for patterns like "class=", "name=", "text=", etc.
```

**Step 2**: Find where simple locators are converted

The architecture suggests there are two locator systems:
1. **CSS/XPath locators** (ast.rs, parser.rs, matcher.rs) - What we've analyzed
2. **Simple locators** (expression.rs) - Robot Framework style `name=value`, `class=value`

**Step 3**: Check unified.rs for class engine

```rust
// File: src/locator/unified.rs
// Look for class engine implementation and cascaded handling
```

### 3.4 Likely Root Cause

**Hypothesis**: The `class=` prefix is:
1. Parsed as a simple locator (not CSS)
2. Converted to a CSS selector internally
3. Conversion fails or doesn't preserve cascaded structure

**Fix Strategy**: Ensure simple locators with `class=` are properly converted to CSS type selectors when used in cascaded contexts.

---

## 4. Implementation Roadmap

### 4.1 Phase 1: Capture Prefix Implementation (Priority 1)

**Timeline**: 16-24 hours (2-3 developer days)

#### Task Breakdown

| Task | File | Effort | Dependencies | Critical Path |
|------|------|--------|--------------|---------------|
| 1. Add CascadedSegment to AST | `ast.rs` | 2-3h | None | ✅ Yes |
| 2. Update grammar with capture_prefix | `grammar.pest` | 1-2h | None | ✅ Yes |
| 3. Update parser to detect capture | `parser.rs` | 4-6h | Tasks 1, 2 | ✅ Yes |
| 4. Implement cascaded matching | `matcher.rs` | 6-8h | Tasks 1-3 | ✅ Yes |
| 5. Add unit tests | `*_tests.rs` | 2-3h | Tasks 1-4 | ⚠️ Parallel |
| 6. Integration testing | Test suites | 1-2h | Task 4 | ⚠️ Parallel |

#### Critical Path: 13-19 hours (sequential work)

**Parallel Opportunities**: Unit tests can be written alongside implementation (TDD approach).

### 4.2 Phase 2: Class Engine Fix (Priority 2)

**Timeline**: 9-14 hours (1.5-2 developer days)

#### Task Breakdown

| Task | File | Effort | Dependencies | Critical Path |
|------|------|--------|--------------|---------------|
| 1. Diagnose class engine issue | Multiple | 2-3h | None | ✅ Yes |
| 2. Locate class engine code | `expression.rs`, `unified.rs` | 1h | None | ✅ Yes |
| 3. Verify parser handles class= | `parser.rs` | 1-2h | Phase 1 complete | ✅ Yes |
| 4. Fix class engine matching | `matcher.rs` or `expression.rs` | 3-4h | Tasks 1-3 | ✅ Yes |
| 5. Add unit tests | `*_tests.rs` | 1-2h | Task 4 | ⚠️ Parallel |
| 6. Integration testing | Test suites | 1-2h | Task 4 | ⚠️ Parallel |

#### Critical Path: 7-12 hours (sequential work)

### 4.3 Risk Factors

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Grammar conflicts with `*` | MEDIUM | HIGH | Use lookahead assertions, comprehensive grammar tests |
| Parser state tracking complexity | MEDIUM | HIGH | Incremental development, extensive unit tests |
| Performance degradation with deep trees | LOW | MEDIUM | Early benchmarking, optimize if needed |
| Class engine deeply integrated | MEDIUM | MEDIUM | Thorough diagnosis phase, allow extra time |
| Breaking existing tests | LOW | HIGH | Continuous regression testing after each change |

---

## 5. Technical Specifications

### 5.1 AST Changes Specification

```rust
// NEW STRUCTURE
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CascadedSegment {
    /// Whether this segment returns its matches (capture flag)
    pub capture: bool,

    /// The selector for this segment
    pub compound: CompoundSelector,

    /// Combinator to next segment (None for last)
    pub combinator: Option<Combinator>,

    /// Raw text for error messages
    pub raw: String,
}

// MODIFIED STRUCTURE
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSelector {
    /// All compound selectors (backward compatibility)
    pub compounds: Vec<CompoundSelector>,

    /// ⭐ NEW FIELD: Structured cascaded segments
    pub cascaded_segments: Option<Vec<CascadedSegment>>,
}

// NEW METHODS
impl ComplexSelector {
    pub fn is_cascaded(&self) -> bool;
    pub fn has_capture(&self) -> bool;
    pub fn get_capture_index(&self) -> Option<usize>;
    pub fn get_cascaded_segments(&self) -> Option<&Vec<CascadedSegment>>;
}
```

### 5.2 Grammar Changes Specification

```pest
// NEW RULE
capture_prefix = { "*" ~ &(type_name | id_selector | class_selector | attribute_selector) }

// MODIFIED RULE
compound_selector = {
    capture_prefix? ~
    type_selector? ~
    (id_selector | class_selector | attribute_selector | pseudo_selector)*
}
```

**Grammar Precedence**:
1. `capture_prefix` - Highest (due to lookahead)
2. `type_selector` - Including `universal_selector`
3. Other selectors - Standard precedence

**Disambiguation Examples**:
- `*JButton` → capture_prefix matches (lookahead sees type_name)
- `* JButton` → universal_selector + descendant combinator (space breaks lookahead)
- `*` → universal_selector (no lookahead match)
- `[text*='x']` → attribute with substring operator (different context)

### 5.3 Parser Changes Specification

**Function Signatures**:

```rust
// MODIFIED - returns capture flag
fn parse_compound_selector(
    pair: Pair<Rule>
) -> Result<(bool, CompoundSelector), ParseError>;

// MODIFIED - builds cascaded segments
fn parse_complex_selector(
    pair: Pair<Rule>
) -> Result<ComplexSelector, ParseError>;
```

**State Tracking Requirements**:
- Track current segment's capture flag
- Track current segment's raw text
- Detect when combinator is `Cascaded` (>>)
- Build `CascadedSegment` structures when cascaded detected

### 5.4 Matcher Changes Specification

**New Public Functions**:

```rust
/// Find elements using cascaded selector with capture support
pub fn find_cascaded<'a>(
    locator: &Locator,
    root: &'a UIComponent,
    evaluator: &Evaluator,
) -> Result<Vec<&'a UIComponent>, MatchError>;

/// Find elements within a specific context
fn find_in_context<'a>(
    compound: &CompoundSelector,
    context: &'a UIComponent,
    evaluator: &Evaluator,
) -> Result<Vec<&'a UIComponent>, MatchError>;
```

**Algorithm Specification**:

```
FUNCTION find_cascaded(locator, root, evaluator):
    selector = locator.selectors[0]
    segments = selector.cascaded_segments

    contexts = [root]
    captured = None

    FOR EACH segment IN segments:
        next_contexts = []

        FOR EACH context IN contexts:
            matches = find_in_context(segment.compound, context, evaluator)
            next_contexts.extend(matches)

        IF segment.capture AND captured IS None:
            captured = next_contexts.clone()

        IF next_contexts IS EMPTY:
            RETURN Error("No match at segment {segment.index}")

        contexts = next_contexts

    RETURN captured OR contexts
```

**Performance Characteristics**:
- Time: O(S × C × D) where:
  - S = number of segments in cascade
  - C = average context size per segment
  - D = average depth of component tree
- Space: O(C × S) for storing contexts

**Optimization Opportunities**:
- Early termination when no matches found
- Deduplication of contexts between segments
- Caching of commonly used selectors

---

## 6. Testing Strategy

### 6.1 Unit Test Coverage

**AST Tests** (`ast.rs` or `ast_tests.rs`):
```rust
#[test]
fn test_cascaded_segment_creation();

#[test]
fn test_complex_selector_is_cascaded();

#[test]
fn test_complex_selector_has_capture();

#[test]
fn test_complex_selector_get_capture_index();
```

**Parser Tests** (`parser.rs` tests):
```rust
#[test]
fn test_parse_capture_prefix_simple();

#[test]
fn test_parse_capture_prefix_with_attributes();

#[test]
fn test_parse_capture_middle_segment();

#[test]
fn test_parse_multiple_captures_first_wins();

#[test]
fn test_parse_universal_vs_capture();

#[test]
fn test_parse_capture_invalid_positions();
```

**Matcher Tests** (`matcher.rs` tests):
```rust
#[test]
fn test_find_cascaded_basic();

#[test]
fn test_find_cascaded_with_capture();

#[test]
fn test_find_cascaded_capture_first_segment();

#[test]
fn test_find_cascaded_capture_middle_segment();

#[test]
fn test_find_cascaded_capture_last_segment();

#[test]
fn test_find_cascaded_no_match_in_segment();

#[test]
fn test_find_in_context();
```

### 6.2 Integration Test Coverage

**Robot Framework Test Files**:
- `tests/robot/swing/18_cascaded_capture.robot` - 26 tests
- `tests/robot/swing/17_cascaded_engines.robot` - 8 class engine tests
- `tests/robot/swing/16_cascaded_basic.robot` - Regression tests

**Expected Results**:
| Test Suite | Before | After | Gain |
|------------|--------|-------|------|
| Capture tests | 1/26 (4%) | 26/26 (100%) | +96% |
| Class engine tests | 5/8 (63%) | 8/8 (100%) | +37% |
| Overall cascaded | 128/178 (72%) | 156+/178 (87%+) | +15% |

---

## 7. Success Metrics

### 7.1 Code Quality Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Unit test coverage | >80% | `cargo tarpaulin` |
| Integration test pass rate | 87%+ | Robot Framework test run |
| No clippy warnings | 0 | `cargo clippy -- -D warnings` |
| No compile warnings | 0 | `cargo build` |
| Documentation coverage | 100% of public APIs | `cargo doc` |

### 7.2 Performance Metrics

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Parse time (simple cascade) | <1ms | Benchmark suite |
| Match time (3-segment cascade) | <10ms | Benchmark suite |
| Match time (5-segment cascade) | <100ms | Benchmark suite |
| Memory overhead per segment | <100 bytes | Memory profiler |

### 7.3 Functional Metrics

| Feature | Target | Measurement Method |
|---------|--------|-------------------|
| Capture prefix works | 100% | 26/26 capture tests pass |
| Class engine works | 100% | 8/8 class tests pass |
| No regressions | 100% | 128+ tests still pass |
| All combinators work | 100% | All combinator tests pass |

---

## 8. Deployment Considerations

### 8.1 Backward Compatibility

**Breaking Changes**: None

- New AST field is `Option<Vec<CascadedSegment>>` (None for non-cascaded)
- Existing `compounds` field remains for backward compatibility
- Existing non-cascaded selectors continue to work unchanged
- New capture prefix (`*`) is optional

**API Compatibility**:
- All existing public functions remain
- Function signatures unchanged
- Return types unchanged
- New functionality is additive only

### 8.2 Migration Path

**For Users**: No migration needed
- Existing selectors continue to work
- New features are opt-in (use `*` for capture)
- No changes to Robot Framework keyword APIs

**For Developers**: Minor AST changes
- Code reading `ComplexSelector.compounds` continues to work
- Code can optionally use new `cascaded_segments` field
- No changes to matcher public API (`find_matching_components`)

### 8.3 Documentation Updates

**Required Documentation**:
1. API docs (`///` comments in Rust code)
2. User guide section on cascaded selectors
3. User guide section on capture prefix
4. Examples in Robot Framework keyword docs
5. CHANGELOG.md entry

---

## 9. Alternative Approaches Considered

### 9.1 Alternative 1: Extend Combinator Enum

**Approach**: Add `CascadedWithCapture` variant to `Combinator`

```rust
pub enum Combinator {
    Descendant,
    Child,
    AdjacentSibling,
    GeneralSibling,
    Cascaded,
    CascadedWithCapture,  // NEW
}
```

**Pros**:
- Minimal AST changes
- Capture is encoded in combinator

**Cons**:
- Breaks the concept (capture is on segment, not combinator)
- Can't capture first segment (no combinator before it)
- Confusing semantics: "capture applies to LEFT segment"
- Multiple captures harder to handle

**Decision**: REJECTED - Semantic mismatch

### 9.2 Alternative 2: Post-Processing Transform

**Approach**: Parse normally, then transform AST to add capture info

```rust
pub fn transform_ast_for_capture(locator: &mut Locator) {
    // Scan for * prefixes in original text
    // Add capture flags post-parse
}
```

**Pros**:
- Parser stays simpler
- Grammar doesn't need capture_prefix rule

**Cons**:
- Error-prone: relies on string position matching
- Loses parsed structure information
- Harder to debug
- Fragile if whitespace handling changes

**Decision**: REJECTED - Too fragile

### 9.3 Alternative 3: Separate Capture Locator Type

**Approach**: Create a new locator type for captured selectors

```rust
pub enum LocatorType {
    Standard(Locator),
    Captured(CapturedLocator),
}
```

**Pros**:
- Clear separation of concerns
- Doesn't pollute existing AST

**Cons**:
- Massive refactoring required
- Breaking change to all code using `Locator`
- Over-engineering for a single feature
- Harder to maintain two parallel systems

**Decision**: REJECTED - Too invasive

### 9.4 Chosen Approach: Enhanced AST with Optional Field

**Approach**: Add optional `cascaded_segments` field to `ComplexSelector`

**Pros**:
- ✅ Backward compatible (field is `Option`)
- ✅ Semantic correctness (capture is property of segment)
- ✅ Clean separation (only populated for cascaded selectors)
- ✅ Easy to test and validate
- ✅ Clear debugging (segments have raw text)

**Cons**:
- Slightly more complex AST
- Parser needs to build two representations (`compounds` and `cascaded_segments`)

**Decision**: ACCEPTED - Best balance of correctness and simplicity

---

## 10. Open Questions and Risks

### 10.1 Open Technical Questions

1. **Q: How does the class engine actually work?**
   - **Status**: Needs investigation
   - **Action**: Phase 2 will diagnose this
   - **Files to check**: `expression.rs`, `unified.rs`, `swt_matcher.rs`

2. **Q: Are there other selector engines that might have the same bug?**
   - **Status**: Unknown until class engine is understood
   - **Action**: Once class engine is fixed, check `name=`, `text=`, `id=` engines

3. **Q: Should capture work with XPath-style locators?**
   - **Status**: Spec doesn't mention XPath
   - **Action**: Limit to CSS-style locators initially, document limitation

4. **Q: What happens with capture on last segment?**
   - **Status**: Spec says "semantically equivalent to no capture"
   - **Action**: Implement as specified, add test case

### 10.2 Known Risks

1. **Grammar Lookahead Performance**
   - **Risk**: Lookahead in grammar could slow parsing
   - **Probability**: LOW (pest is optimized for lookahead)
   - **Mitigation**: Benchmark parser performance, optimize if needed

2. **State Tracking in Parser**
   - **Risk**: Complex state management leads to bugs
   - **Probability**: MEDIUM
   - **Mitigation**: Comprehensive unit tests, incremental development

3. **Class Engine Integration**
   - **Risk**: Class engine fix requires architectural changes
   - **Probability**: MEDIUM
   - **Mitigation**: Allow extra time in Phase 2, consider refactoring if needed

4. **Performance with Deep Cascades**
   - **Risk**: O(S × C × D) could be slow for deep trees
   - **Probability**: LOW (typical UIs aren't that deep)
   - **Mitigation**: Early benchmarking, document performance characteristics

---

## 11. Appendix

### 11.1 File Locations Quick Reference

| Component | File | Lines | Purpose |
|-----------|------|-------|---------|
| AST | `src/locator/ast.rs` | 441 | Type definitions |
| Grammar | `src/locator/grammar.pest` | 341 | Parsing rules |
| Parser | `src/locator/parser.rs` | 1000+ | Parsing implementation |
| Matcher | `src/locator/matcher.rs` | 1100+ | Matching algorithm |
| Expression | `src/locator/expression.rs` | ? | High-level locators |
| Unified | `src/locator/unified.rs` | ? | Cross-toolkit support |
| Module | `src/locator/mod.rs` | 39 | Public exports |

### 11.2 Key Terminology

| Term | Definition |
|------|------------|
| **Locator** | A string that identifies UI elements (e.g., `JButton#submit`) |
| **Selector** | A parsed locator (ComplexSelector) |
| **Compound Selector** | Type + modifiers without combinators (e.g., `JButton#submit:enabled`) |
| **Complex Selector** | Chain of compounds with combinators (e.g., `JPanel > JButton`) |
| **Combinator** | Operator connecting compounds (`>`, `>>`, ` `, `+`, `~`) |
| **Cascaded Selector** | Selector using `>>` combinator |
| **Segment** | One part of a cascaded selector (between `>>` separators) |
| **Capture Prefix** | The `*` marker indicating which segment to return |
| **Context** | The set of elements within which the next segment searches |

### 11.3 Code Style Guidelines

**Rust Conventions**:
- Follow `rustfmt` defaults
- Use `clippy` for lints
- Document all public APIs with `///` comments
- Use `#[derive(Debug, Clone, PartialEq)]` for AST types
- Prefer `Option<T>` over nullable patterns

**Testing Conventions**:
- Unit tests in same file with `#[cfg(test)] mod tests`
- Integration tests in `tests/` directory
- Use descriptive test names: `test_parse_capture_prefix_with_attributes`
- Test both success and error cases

### 11.4 Useful Commands

**Development**:
```bash
# Build with warnings as errors
cargo build --all-features

# Run tests
cargo test

# Run specific test
cargo test test_parse_capture_prefix

# Check for issues
cargo clippy -- -D warnings

# Format code
cargo fmt

# Generate docs
cargo doc --open
```

**Grammar Testing**:
```bash
# Test grammar with pest_debugger (if available)
pest_debugger src/locator/grammar.pest

# Or use online pest debugger at https://pest.rs/#editor
```

**Integration Testing**:
```bash
# Run specific Robot Framework test suite
uv run robot --outputdir results tests/robot/swing/18_cascaded_capture.robot

# Run with verbose output
uv run robot --outputdir results --loglevel DEBUG tests/robot/swing/18_cascaded_capture.robot
```

---

## 12. Conclusion

### 12.1 Summary of Findings

1. **Cascaded Combinator (`>>`) Infrastructure**: ✅ EXISTS
   - AST has `Combinator::Cascaded` variant
   - Grammar has `cascaded_combinator` rule
   - Parser recognizes and stores combinator
   - Matcher handles combinator (but incorrectly)

2. **Capture Prefix (`*`) Infrastructure**: ❌ COMPLETELY MISSING
   - No AST support for capture flags
   - No grammar rule for capture prefix
   - No parser logic to detect capture
   - No matching algorithm for capture

3. **Class Engine Issue**: ⚠️ NEEDS INVESTIGATION
   - Not found in matcher.rs
   - Likely in expression.rs or unified.rs
   - Probably a conversion/integration issue

### 12.2 Recommended Implementation Order

**Phase 1 (Week 1)**: Capture Prefix - 16-24 hours
1. Day 1-2: AST + Grammar changes with tests
2. Day 2-3: Parser implementation with tests
3. Day 3-4: Matcher implementation with tests
4. Day 4-5: Integration testing and bug fixes

**Phase 2 (Week 1-2)**: Class Engine - 9-14 hours
1. Days 5-6: Diagnosis and fix planning
2. Day 6-7: Implementation and testing
3. Day 7: Integration validation

**Total Timeline**: 25-38 hours (1-2 weeks with one developer)

### 12.3 Success Probability

**Overall Risk Assessment**: MEDIUM-LOW

- ✅ Clear requirements from specification
- ✅ Well-structured existing codebase
- ✅ Good test coverage framework exists
- ✅ Changes are localized and modular
- ⚠️ Parser state tracking adds complexity
- ⚠️ Class engine unknown factor

**Confidence Level**: 85%

The implementation is well-defined and the existing codebase is clean. The main risk is the class engine issue, but even worst-case (requires refactoring) is manageable within the timeline.

---

**Document End**

**Next Steps**:
1. Review and approve architecture analysis
2. Begin Phase 1 implementation
3. Set up continuous integration for regression testing
4. Schedule code reviews after each major component

---

**Revision History**:

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2026-01-21 | System Architecture Designer | Initial architecture analysis |
