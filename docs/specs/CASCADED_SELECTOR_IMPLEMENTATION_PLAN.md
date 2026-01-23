# Cascaded Selector Implementation Plan

## Document Information

| Field | Value |
|-------|-------|
| Version | 1.0.0 |
| Status | Implementation Plan |
| Author | Strategic Planning Agent |
| Date | 2026-01-21 |
| Related Documents | [Specification](CASCADED_SELECTOR_SPECIFICATION.md), [Test Report](../test-plans/CASCADED_TEST_EXECUTION_REPORT.md) |

---

## Executive Summary

This document provides a comprehensive implementation plan for completing the cascaded selector feature in robotframework-swing. Based on the test execution results, **two critical features are missing**:

1. **Capture Prefix (`*`) - PRIORITY 1**: Completely unimplemented (25 test failures, 4% pass rate)
2. **Class Engine Cascade Support - PRIORITY 2**: Partially broken (3 test failures, returns empty results)

Current overall pass rate: **72% (128/178 tests passing)**
Target pass rate after implementation: **86%+ (153/178 tests passing)**

---

## 1. Current State Analysis

### 1.1 What Works ✅

Based on the test execution report:

| Feature | Pass Rate | Status |
|---------|-----------|--------|
| Table Cascaded Selectors | 100% (47/47) | ✅ Fully Working |
| Index Engine Cascades | 100% (11/11) | ✅ Fully Working |
| XPath Engine Cascades | 100% (13/13) | ✅ Fully Working |
| ID Engine Cascades | 100% (8/8) | ✅ Fully Working |
| Text Engine Cascades | 93% (14/15) | ✅ Mostly Working |
| Basic Cascade Syntax | 50% (15/30) | ⚠️ Partially Working |
| Name Engine Cascades | 77% (10/13) | ⚠️ Mostly Working |
| CSS Engine Cascades | 82% (9/11) | ⚠️ Mostly Working |

**Strengths:**
- The `>>` separator parsing works correctly
- Table-specific selectors are fully implemented
- Most selector engines integrate properly with cascading
- Whitespace handling around `>>` works as expected
- Integration with action keywords (Click, Input Text) works

### 1.2 What's Broken ❌

#### Critical Issues

**Issue #1: Capture Prefix Feature Missing**
- **Impact**: 25 test failures (96% of capture tests fail)
- **Severity**: CRITICAL
- **User Impact**: Cannot capture intermediate elements in cascades
- **Example Failure**:
  ```robot
  # This fails with ElementNotFoundError:
  ${panel}=    Get Element    *JPanel[name='formPanel'] >> JTextField
  ```

**Issue #2: Class Engine Returns Empty Results**
- **Impact**: 3 test failures
- **Severity**: HIGH
- **User Impact**: `class=` prefix doesn't work in cascaded contexts
- **Example Failure**:
  ```robot
  # This returns empty list []:
  Get Element    class=JPanel >> class=JButton
  ```

#### Medium Issues

**Issue #3: Multiple Element Ambiguity**
- **Impact**: 9 test failures
- **Severity**: MEDIUM
- **Root Cause**: Generic selectors without attributes match too many elements
- **Resolution**: Requires test updates, not code changes

**Issue #4: Element Verification Keywords**
- **Impact**: 2 test failures
- **Severity**: LOW
- **Root Cause**: Verification keywords don't handle element objects correctly

### 1.3 Existing Code Structure

The current implementation has these components:

```
src/locator/
├── ast.rs           # AST definitions (includes Combinator::Cascaded)
├── parser.rs        # Parser implementation using pest
├── grammar.pest     # Pest grammar file
├── matcher.rs       # Element matching logic
├── expression.rs    # High-level locator expressions
└── mod.rs           # Module exports
```

**Key Finding**: The `Combinator::Cascaded` variant already exists in the AST, indicating partial implementation.

---

## 2. Missing Feature #1: Capture Prefix (`*`)

### 2.1 Feature Overview

**Specification Reference**: Section 4 of CASCADED_SELECTOR_SPECIFICATION.md

**Purpose**: Allow capturing an intermediate element in a cascade chain rather than the final element.

**Syntax**:
```robot
# Without capture: returns JTextField
Get Element    JPanel >> JTextField

# With capture: returns JPanel (the container)
Get Element    *JPanel >> JTextField
```

**Use Cases**:
- Reusing container elements for multiple operations
- Accessing parent elements for validation
- Efficient element caching and context reuse

### 2.2 Current State

**Status**: ❌ NOT IMPLEMENTED AT ALL

**Evidence from Tests**:
```
18_cascaded_capture.robot: 1 PASSED / 25 FAILED (4%)
All 25 failures: ElementNotFoundError for selectors with * prefix
```

**Test Examples**:
```robot
# All of these fail:
*JPanel[name='formPanel'] >> JTextField
JDialog >> *JPanel[name='formPanel'] >> JButton
*name=formPanel >> name=submitButton
```

### 2.3 Implementation Requirements

#### 2.3.1 AST Changes

**Location**: `src/locator/ast.rs`

Add `capture` flag to selector segments:

```rust
/// A single segment in a cascaded locator chain
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CascadedSegment {
    /// Whether this segment should be captured (marked with *)
    pub capture: bool,

    /// The compound selector for this segment
    pub compound: CompoundSelector,

    /// Combinator to next segment (if any)
    pub combinator: Option<Combinator>,

    /// Original raw text of this segment
    pub raw: String,
}

/// Enhanced ComplexSelector with cascade awareness
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexSelector {
    /// All compound selectors in this chain
    pub compounds: Vec<CompoundSelector>,

    /// For cascaded selectors: structured segments with capture flags
    pub cascaded_segments: Option<Vec<CascadedSegment>>,
}
```

**Rationale**: The `capture` flag needs to be stored at parse time and used during matching.

#### 2.3.2 Grammar Changes

**Location**: `src/locator/grammar.pest`

Add capture prefix to grammar:

```pest
// Current grammar likely has:
complex_selector = { compound_selector ~ (combinator ~ compound_selector)* }

// Need to add:
capture_prefix = { "*" }

compound_selector_with_capture = {
    capture_prefix? ~ compound_selector
}

// Update complex selector to support capture:
complex_selector = {
    compound_selector_with_capture ~
    (combinator ~ compound_selector_with_capture)*
}
```

**Testing Strategy**: Add unit tests to verify grammar parses `*` correctly:
- `*JPanel` → type selector with capture=true
- `*JButton[name='x']` → type + attribute with capture=true
- `JPanel >> *JButton >> JLabel` → capture middle segment

#### 2.3.3 Parser Changes

**Location**: `src/locator/parser.rs`

Update parser to recognize and store capture flags:

```rust
// Pseudocode for parser changes
fn parse_compound_selector_with_capture(pair: Pair<Rule>)
    -> Result<(bool, CompoundSelector), ParseError>
{
    let mut capture = false;
    let mut compound = CompoundSelector::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::capture_prefix => {
                capture = true;
            }
            Rule::type_selector => {
                compound.type_selector = Some(parse_type_selector(inner_pair)?);
            }
            Rule::attribute_selector => {
                compound.attribute_selectors.push(parse_attribute(inner_pair)?);
            }
            // ... other selector types
            _ => {}
        }
    }

    Ok((capture, compound))
}

// Update complex selector parsing:
fn parse_complex_selector(pair: Pair<Rule>) -> Result<ComplexSelector, ParseError> {
    let mut segments = Vec::new();
    let mut current_segment = CascadedSegment::default();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::compound_selector_with_capture => {
                let (capture, compound) = parse_compound_selector_with_capture(inner_pair)?;
                current_segment.capture = capture;
                current_segment.compound = compound;
            }
            Rule::combinator => {
                current_segment.combinator = Some(parse_combinator(inner_pair)?);
                segments.push(current_segment);
                current_segment = CascadedSegment::default();
            }
            _ => {}
        }
    }

    // Push final segment
    segments.push(current_segment);

    Ok(ComplexSelector {
        compounds: segments.iter().map(|s| s.compound.clone()).collect(),
        cascaded_segments: Some(segments),
    })
}
```

**Error Handling**: Add validation for:
- `*` at end with nothing after: `JPanel >> *` (invalid)
- `*` in quotes: `text='*pattern'` (not a capture prefix)
- Multiple `*` in same segment: `**JPanel` (invalid)

#### 2.3.4 Matcher Changes

**Location**: `src/locator/matcher.rs`

Implement capture logic in the matching algorithm:

```rust
/// Find elements matching a cascaded locator with capture support
pub fn find_cascaded<'a>(
    locator: &ComplexSelector,
    root: &'a SwingComponent,
) -> Result<Vec<&'a SwingComponent>, MatchError> {

    // Get cascaded segments if this is a cascaded selector
    let segments = match &locator.cascaded_segments {
        Some(segs) if has_cascaded_combinator(locator) => segs,
        _ => {
            // Not a cascaded selector, use normal matching
            return find_standard(locator, root);
        }
    };

    let mut current_contexts: Vec<&SwingComponent> = vec![root];
    let mut captured_elements: Option<Vec<&SwingComponent>> = None;

    for (segment_idx, segment) in segments.iter().enumerate() {
        let mut next_contexts = Vec::new();

        // Search within each current context
        for context in &current_contexts {
            let matches = find_in_context(&segment.compound, context)?;
            next_contexts.extend(matches);
        }

        // If this segment has capture flag and we haven't captured yet
        if segment.capture && captured_elements.is_none() {
            captured_elements = Some(next_contexts.clone());
        }

        // Check if we found anything
        if next_contexts.is_empty() {
            return Err(MatchError::NoMatch {
                selector: format_segment(segment),
                segment_index: segment_idx,
                context: format!("{} contexts from previous segment", current_contexts.len()),
            });
        }

        // Continue with next segment using these results as new contexts
        current_contexts = next_contexts;
    }

    // Return captured elements if any, otherwise final results
    Ok(captured_elements.unwrap_or(current_contexts))
}

/// Find elements matching a compound selector within a context
fn find_in_context<'a>(
    compound: &CompoundSelector,
    context: &'a SwingComponent,
) -> Result<Vec<&'a SwingComponent>, MatchError> {
    let mut results = Vec::new();

    // Search all descendants of the context component
    search_descendants(context, compound, &mut results)?;

    Ok(results)
}

/// Recursively search descendants
fn search_descendants<'a>(
    component: &'a SwingComponent,
    selector: &CompoundSelector,
    results: &mut Vec<&'a SwingComponent>,
) -> Result<(), MatchError> {

    // Check if current component matches
    if matches_compound(component, selector)? {
        results.push(component);
    }

    // Recurse into children
    for child in component.get_children() {
        search_descendants(child, selector, results)?;
    }

    Ok(())
}
```

**Key Algorithm Points**:
1. Parse cascaded selector into segments
2. Start with root as context
3. For each segment:
   - Search within current contexts
   - If segment has `capture` flag, save these results
   - Use results as contexts for next segment
4. Return captured results if any, otherwise final results

**Edge Cases to Handle**:
- Multiple captures: Only first capture counts (per spec)
- Capture on last segment: Equivalent to no capture
- No matches at any stage: Clear error message
- Empty contexts: Fail with context information

#### 2.3.5 Python Binding Changes

**Location**: Python bindings for locator functionality

Ensure Python API properly handles captured elements:

```python
def find_element(self, locator: str) -> SwingElement:
    """
    Find a single element matching the locator.

    Supports capture prefix (*) to return intermediate elements.

    Examples:
        find_element("JButton[text='OK']")
        find_element("JPanel >> JButton[text='OK']")
        find_element("*JPanel >> JButton[text='OK']")  # Returns JPanel
    """
    result = self._rust_locator.find_single(locator)
    if result is None:
        raise ElementNotFoundError(f"No element found matching: {locator}")
    return SwingElement(result)
```

**No API changes needed** - existing functions already return elements, just need to ensure Rust implementation returns correct element.

### 2.4 Implementation Steps (Priority 1)

**Step 1: AST Changes** (2-3 hours)
- [ ] Add `CascadedSegment` struct with `capture: bool` field
- [ ] Add `cascaded_segments: Option<Vec<CascadedSegment>>` to `ComplexSelector`
- [ ] Add helper methods: `is_cascaded()`, `has_capture()`, `get_capture_index()`
- [ ] Update `Display` implementations for debugging
- [ ] Add unit tests for AST structures

**Step 2: Grammar Changes** (1-2 hours)
- [ ] Add `capture_prefix = { "*" }` rule
- [ ] Add `compound_selector_with_capture` rule
- [ ] Update `complex_selector` to use new rule
- [ ] Test grammar with pest_generator
- [ ] Add grammar test cases:
  - `*JPanel`
  - `JDialog >> *JPanel >> JButton`
  - `*name=x >> text=y`

**Step 3: Parser Implementation** (4-6 hours)
- [ ] Update `parse_complex_selector()` to recognize capture prefix
- [ ] Build `CascadedSegment` structures during parsing
- [ ] Set `cascaded_segments` field when `>>` combinator found
- [ ] Add validation for invalid capture syntax
- [ ] Add parser unit tests:
  - Valid capture patterns
  - Invalid capture patterns (error cases)
  - Capture with various selector types

**Step 4: Matcher Implementation** (6-8 hours)
- [ ] Implement `find_cascaded()` function with capture support
- [ ] Implement `find_in_context()` for context-aware searching
- [ ] Update main `find()` function to route to `find_cascaded()` when appropriate
- [ ] Handle multiple captures (first wins)
- [ ] Add comprehensive matcher unit tests:
  - Single capture
  - Multiple captures (verify first wins)
  - Capture on different segments (first, middle, last)
  - No capture (verify default behavior unchanged)

**Step 5: Integration Testing** (2-3 hours)
- [ ] Run `18_cascaded_capture.robot` test suite
- [ ] Verify all 25 tests now pass
- [ ] Test capture with different engines (class, name, text, etc.)
- [ ] Performance testing with deep cascades

**Step 6: Documentation** (1-2 hours)
- [ ] Update API documentation
- [ ] Add capture examples to user guide
- [ ] Document capture behavior and limitations

**Total Estimated Effort: 16-24 hours (2-3 developer days)**

### 2.5 Testing Strategy

#### Unit Tests

**Parser Tests** (`src/locator/parser_tests.rs`):
```rust
#[test]
fn test_parse_capture_prefix() {
    let locator = parse_locator("*JPanel").unwrap();
    assert!(locator.has_capture());
    assert_eq!(locator.get_capture_index(), Some(0));
}

#[test]
fn test_parse_capture_middle_segment() {
    let locator = parse_locator("JDialog >> *JPanel >> JButton").unwrap();
    assert!(locator.has_capture());
    assert_eq!(locator.get_capture_index(), Some(1));
}

#[test]
fn test_parse_multiple_captures_first_wins() {
    let locator = parse_locator("*JDialog >> *JPanel >> JButton").unwrap();
    assert_eq!(locator.get_capture_index(), Some(0));
}

#[test]
fn test_parse_capture_with_attributes() {
    let locator = parse_locator("*JPanel[name='form'] >> JTextField").unwrap();
    let segments = locator.cascaded_segments.unwrap();
    assert!(segments[0].capture);
    assert_eq!(segments[0].compound.attribute_selectors.len(), 1);
}
```

**Matcher Tests** (`src/locator/matcher_tests.rs`):
```rust
#[test]
fn test_capture_intermediate_element() {
    let root = create_test_hierarchy(); // JFrame > JPanel > JButton
    let locator = parse_locator("*JPanel >> JButton").unwrap();
    let results = find_cascaded(&locator, &root).unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].component_type, "JPanel");
}

#[test]
fn test_capture_with_multiple_matches() {
    let root = create_test_hierarchy(); // Multiple JButtons in JPanel
    let locator = parse_locator("*JPanel >> JButton").unwrap();
    let results = find_cascaded(&locator, &root).unwrap();

    // Should return JPanel (the parent), not the buttons
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].component_type, "JPanel");
}

#[test]
fn test_no_capture_default_behavior() {
    let root = create_test_hierarchy();
    let locator = parse_locator("JPanel >> JButton").unwrap();
    let results = find_cascaded(&locator, &root).unwrap();

    // Should return JButton (final element)
    assert!(results.iter().all(|r| r.component_type == "JButton"));
}
```

#### Integration Tests

Run existing Robot Framework test suite:
```bash
# Should go from 1/26 passing to 26/26 passing
uv run robot --outputdir results/capture_after tests/robot/18_cascaded_capture.robot
```

**Expected Results**:
- Before: 1 PASSED, 25 FAILED (4%)
- After: 26 PASSED, 0 FAILED (100%)

### 2.6 Success Criteria

✅ **Implementation Complete When**:
1. All 26 tests in `18_cascaded_capture.robot` pass
2. No regressions in other test suites (128+ tests still passing)
3. Performance acceptable for deep cascades (<100ms for 5-level cascade)
4. Documentation updated with examples
5. Code reviewed and merged

---

## 3. Missing Feature #2: Class Engine Cascade Support

### 3.1 Feature Overview

**Specification Reference**: Section 3.3 of CASCADED_SELECTOR_SPECIFICATION.md

**Purpose**: Enable `class=` selector engine to work within cascaded selector chains.

**Syntax**:
```robot
# Should work but currently returns empty results:
Get Element    class=JPanel >> class=JButton
Get Element    class=JDialog >> JButton[name='submit']
Get Element    JPanel[name='x'] >> class=JButton
```

### 3.2 Current State

**Status**: ⚠️ PARTIALLY BROKEN

**Evidence from Tests**:
```
17_cascaded_engines.robot: 65 PASSED / 10 FAILED (87%)
Class Engine: 5/8 passed (63%)
All 3 class engine failures: '[]' should not be empty
```

**Test Failures**:
```robot
# These all return empty list []:
Simple Class Cascade             class=JPanel >> class=JButton
Class Then CSS Engine Mix         class=JPanel >> JButton[name='submitButton']
CSS Then Class Engine Mix         JPanel[name='formPanel'] >> class=JButton
```

**Analysis**: The class engine appears to work in non-cascaded contexts but fails when used in cascade chains. This suggests the issue is in how the class engine integrates with the cascaded matching logic.

### 3.3 Root Cause Analysis

**Hypothesis 1: Class Engine Context Issues**
- Class engine may not properly handle parent context filtering
- When used in cascade, it needs to search within provided context, not entire tree

**Hypothesis 2: Class Engine Matching Logic**
- Class matcher might use different component traversal than other engines
- May not properly implement descendant search within context

**Hypothesis 3: Engine Prefix Parsing**
- `class=` prefix might not be properly parsed in cascaded segments
- Parser may strip the prefix incorrectly

**Debugging Steps**:
1. Add debug logging to class engine matcher
2. Verify `class=JPanel` works standalone (should pass based on test results)
3. Check how context is passed to class engine in cascaded mode
4. Compare class engine implementation with working engines (name, text)

### 3.4 Implementation Requirements

#### 3.4.1 Diagnostic Code

**Location**: `src/locator/matcher.rs`

Add logging to understand what's happening:

```rust
// Add to class engine matcher
pub fn find_by_class<'a>(
    class_name: &str,
    context: &'a SwingComponent,
) -> Vec<&'a SwingComponent> {
    log::debug!("Class engine: searching for '{}' in context {:?}",
                class_name, context.component_type);

    let mut results = Vec::new();
    search_by_class_recursive(class_name, context, &mut results);

    log::debug!("Class engine: found {} matches", results.len());
    results
}

fn search_by_class_recursive<'a>(
    class_name: &str,
    component: &'a SwingComponent,
    results: &mut Vec<&'a SwingComponent>,
) {
    // Check current component
    if component_matches_class(component, class_name) {
        log::debug!("  - Match: {:?}", component.component_type);
        results.push(component);
    }

    // Recurse into children
    for child in component.get_children() {
        search_by_class_recursive(class_name, child, results);
    }
}
```

#### 3.4.2 Expected Fix #1: Context Scoping

If the issue is context scoping, fix the class engine to search within context:

```rust
pub fn find_by_class<'a>(
    class_name: &str,
    context: &'a SwingComponent,
) -> Vec<&'a SwingComponent> {
    let mut results = Vec::new();

    // IMPORTANT: Start searching from context's CHILDREN, not context itself
    // This matches behavior of CSS descendant selector
    for child in context.get_children() {
        search_by_class_recursive(class_name, child, &mut results);
    }

    results
}
```

**Rationale**: In cascaded selectors, each segment searches within the results of the previous segment. If the class engine searches from the context itself, it might include the context in results, which would be incorrect.

#### 3.4.3 Expected Fix #2: Class Name Matching

If the issue is class name matching, verify the matching logic:

```rust
fn component_matches_class(component: &SwingComponent, class_name: &str) -> bool {
    // Handle both "JButton" and "Button" (with or without J prefix)
    let component_class = component.component_type.simple_name();
    let normalized_class = class_name.trim_start_matches('J');
    let component_normalized = component_class.trim_start_matches('J');

    // Case-insensitive comparison
    normalized_class.eq_ignore_ascii_case(&component_normalized)
}
```

#### 3.4.4 Expected Fix #3: Engine Integration

Ensure class engine is properly called in cascaded context:

```rust
fn find_in_context<'a>(
    segment: &CascadedSegment,
    context: &'a SwingComponent,
) -> Result<Vec<&'a SwingComponent>, MatchError> {

    // Check if this segment uses class engine
    if let Some(class_name) = extract_class_engine_prefix(&segment) {
        return Ok(find_by_class(class_name, context));
    }

    // Otherwise use standard CSS matching
    find_css_matches(&segment.compound, context)
}

fn extract_class_engine_prefix(segment: &CascadedSegment) -> Option<&str> {
    // Check if the type selector is a class= engine prefix
    if let Some(TypeSelector::EnginePrefix { engine, value }) = &segment.compound.type_selector {
        if engine == "class" {
            return Some(value);
        }
    }
    None
}
```

### 3.5 Implementation Steps (Priority 2)

**Step 1: Diagnosis** (2-3 hours)
- [ ] Add debug logging to class engine
- [ ] Run failing tests with logging enabled
- [ ] Identify exact failure point (parsing vs matching vs context)
- [ ] Compare with working name engine implementation
- [ ] Document findings

**Step 2: Parser Verification** (1-2 hours)
- [ ] Verify `class=JPanel` is correctly parsed
- [ ] Check if engine prefix is preserved in cascaded segments
- [ ] Add parser unit tests for class engine in cascades
- [ ] Fix parser if needed

**Step 3: Matcher Fix** (3-4 hours)
- [ ] Implement correct context-aware searching for class engine
- [ ] Ensure class matching logic is consistent with other engines
- [ ] Handle edge cases (no children, multiple matches)
- [ ] Add unit tests for class engine matching

**Step 4: Integration** (2-3 hours)
- [ ] Verify class engine works with other engines in mixed cascades
- [ ] Test `class=X >> css_selector`
- [ ] Test `css_selector >> class=X`
- [ ] Test `class=X >> class=Y >> class=Z`

**Step 5: Testing** (1-2 hours)
- [ ] Run `17_cascaded_engines.robot` class engine tests
- [ ] Verify all 8 class engine tests pass
- [ ] Check for regressions in other engine tests
- [ ] Performance validation

**Total Estimated Effort: 9-14 hours (1.5-2 developer days)**

### 3.6 Testing Strategy

#### Unit Tests

**Class Engine Tests** (`src/locator/matcher_tests.rs`):
```rust
#[test]
fn test_class_engine_standalone() {
    let root = create_test_hierarchy();
    let results = find_by_class("JButton", &root);
    assert!(!results.is_empty());
}

#[test]
fn test_class_engine_in_cascade() {
    let root = create_test_hierarchy(); // JPanel > JButton
    let locator = parse_locator("class=JPanel >> class=JButton").unwrap();
    let results = find_cascaded(&locator, &root).unwrap();

    assert!(!results.is_empty());
    assert!(results.iter().all(|r| r.component_type == "JButton"));
}

#[test]
fn test_class_engine_mixed_with_css() {
    let root = create_test_hierarchy();
    let locator = parse_locator("class=JPanel >> JButton[name='submit']").unwrap();
    let results = find_cascaded(&locator, &root).unwrap();

    assert!(!results.is_empty());
}
```

#### Integration Tests

**Expected Results**:
- Before: 5/8 class engine tests passing (63%)
- After: 8/8 class engine tests passing (100%)
- Before: 65/75 total engine tests passing (87%)
- After: 68/75 total engine tests passing (91%)

### 3.7 Success Criteria

✅ **Implementation Complete When**:
1. All 8 class engine tests in `17_cascaded_engines.robot` pass
2. No regressions in other engine tests (65+ tests still passing)
3. Class engine works in all cascade positions (first, middle, last)
4. Mixed engine cascades work correctly
5. Code reviewed and merged

---

## 4. Implementation Priorities

### 4.1 Priority Matrix

| Feature | Priority | Impact | Effort | Pass Rate Gain |
|---------|----------|--------|--------|----------------|
| **Capture Prefix** | P1 - CRITICAL | 25 tests | 16-24h | +14% (+25 tests) |
| **Class Engine** | P2 - HIGH | 3 tests | 9-14h | +2% (+3 tests) |
| Test Updates | P3 - MEDIUM | 9 tests | 4-6h | +5% (+9 tests) |
| Verification Fix | P4 - LOW | 2 tests | 2-3h | +1% (+2 tests) |
| Documentation | P5 - LOW | 0 tests | 2-4h | 0% (quality improvement) |

### 4.2 Recommended Implementation Order

**Phase 1: Critical Features** (3-5 days)
1. Implement Capture Prefix (`*`) - 16-24 hours
   - Highest impact: 25 test failures → 0 failures
   - Most requested feature per specification
   - Enables advanced use cases

2. Fix Class Engine - 9-14 hours
   - Closes remaining engine gaps
   - 3 test failures → 0 failures

**Expected Result After Phase 1**:
- Pass rate: 72% → 86%+
- Tests passing: 128 → 156+

**Phase 2: Test Quality** (1 day)
3. Update Ambiguous Tests - 4-6 hours
   - Make selectors more specific
   - 9 test failures → 0 failures

**Expected Result After Phase 2**:
- Pass rate: 86% → 91%
- Tests passing: 156 → 165

**Phase 3: Polish** (1 day)
4. Fix Element Verification - 2-3 hours
5. Update Documentation - 2-4 hours
6. Performance Optimization - 2-4 hours

**Expected Final Result**:
- Pass rate: 91% → 95%+
- Tests passing: 165 → 169+
- Production-ready quality

### 4.3 Timeline Estimate

**Aggressive Timeline** (1 week):
- Day 1-3: Implement Capture Prefix
- Day 4: Fix Class Engine
- Day 5: Test Updates and Verification Fixes
- Day 6-7: Documentation and Polish

**Conservative Timeline** (2 weeks):
- Week 1: Implement both P1 and P2 features with thorough testing
- Week 2: Test quality improvements, documentation, and final validation

---

## 5. Technical Implementation Guide

### 5.1 Development Environment Setup

**Prerequisites**:
```bash
# Ensure Rust toolchain is installed
rustc --version  # Should be 1.70+

# Ensure Python environment is set up
uv --version

# Install test dependencies
cd /mnt/c/workspace/robotframework-swing
uv sync
```

**Run Existing Tests**:
```bash
# Run specific test suite
uv run robot --outputdir results/capture tests/robot/18_cascaded_capture.robot

# Run all cascaded tests
uv run robot --outputdir results/all tests/robot/16_cascaded_*.robot
```

### 5.2 Code Modification Locations

**For Capture Prefix Implementation**:

1. **AST Definition** - `src/locator/ast.rs`
   - Add `CascadedSegment` struct
   - Add `capture: bool` field
   - Lines to modify: Around line 113 (CompoundSelector definition)

2. **Grammar** - `src/locator/grammar.pest`
   - Add `capture_prefix` rule
   - Update `compound_selector` rule
   - Location: After type selector rules

3. **Parser** - `src/locator/parser.rs`
   - Update `parse_complex_selector()`
   - Add `parse_cascaded_segment()`
   - Lines to modify: Around line 200+ (complex selector parsing)

4. **Matcher** - `src/locator/matcher.rs`
   - Add `find_cascaded()` function
   - Update main `find()` routing
   - Lines to modify: Core matching logic

**For Class Engine Fix**:

1. **Matcher** - `src/locator/matcher.rs`
   - Find `find_by_class()` function
   - Update context handling
   - Add proper descendant search

### 5.3 Debugging Tips

**Enable Rust Logging**:
```rust
// Add to code
use log::{debug, info, warn};

debug!("Parsing segment: {}", segment_text);
debug!("Capture flag: {}", capture);
debug!("Found {} matches in context", results.len());
```

**Run with Logging**:
```bash
RUST_LOG=debug cargo test test_name -- --nocapture
```

**Debug Robot Framework Tests**:
```robot
# Add to test
Log To Console    Searching for: ${selector}
${result}=    Get Element    ${selector}
Log To Console    Found: ${result}
```

### 5.4 Testing Procedure

**Unit Test Workflow**:
```bash
# Run specific test module
cargo test parser::tests

# Run with output
cargo test matcher::tests::test_capture -- --nocapture

# Run all locator tests
cargo test locator::
```

**Integration Test Workflow**:
```bash
# Test capture feature
uv run robot --outputdir results/capture --test "Capture*" \
    tests/robot/18_cascaded_capture.robot

# Test class engine
uv run robot --outputdir results/class --test "*Class*" \
    tests/robot/17_cascaded_engines.robot

# Full regression test
uv run robot --outputdir results/full tests/robot/1[6-9]_cascaded_*.robot
```

**Performance Testing**:
```rust
#[test]
fn benchmark_capture_cascade() {
    let root = create_large_hierarchy(1000); // 1000 components
    let locator = parse_locator("*JPanel >> JButton >> JLabel").unwrap();

    let start = std::time::Instant::now();
    let results = find_cascaded(&locator, &root).unwrap();
    let duration = start.elapsed();

    assert!(duration.as_millis() < 100, "Cascade took too long: {:?}", duration);
}
```

### 5.5 Code Review Checklist

**Before Submitting PR**:

- [ ] All unit tests pass (`cargo test`)
- [ ] All Robot Framework tests pass (target tests + regression)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] Documentation comments added (/** ... */)
- [ ] CHANGELOG.md updated
- [ ] Test coverage >80% for new code
- [ ] Performance benchmarks pass
- [ ] No unsafe code added (or justified in comments)
- [ ] Error messages are user-friendly

---

## 6. Risk Assessment

### 6.1 Technical Risks

**Risk 1: Parser Complexity**
- **Probability**: MEDIUM
- **Impact**: HIGH
- **Description**: Grammar changes might conflict with existing rules
- **Mitigation**:
  - Test grammar changes incrementally
  - Use pest_generator for validation
  - Add comprehensive parser tests first
  - Review pest documentation for precedence rules

**Risk 2: Performance Degradation**
- **Probability**: LOW
- **Impact**: MEDIUM
- **Description**: Cascaded matching with capture might be slow for deep hierarchies
- **Mitigation**:
  - Add performance benchmarks early
  - Optimize if >100ms for typical cases
  - Consider caching parsed locators
  - Profile with real-world component trees

**Risk 3: Breaking Changes**
- **Probability**: LOW
- **Impact**: HIGH
- **Description**: Changes might break existing functionality
- **Mitigation**:
  - Run full regression suite after each change
  - Maintain 128+ tests passing throughout
  - Add tests for edge cases before implementing
  - Use feature flags if needed

**Risk 4: Class Engine Root Cause**
- **Probability**: MEDIUM
- **Impact**: MEDIUM
- **Description**: Root cause of class engine failure might be deeper than expected
- **Mitigation**:
  - Dedicate time to diagnosis first
  - Compare with working engines
  - Consider refactoring engine abstraction if needed
  - Document findings thoroughly

### 6.2 Project Risks

**Risk 5: Scope Creep**
- **Probability**: MEDIUM
- **Impact**: MEDIUM
- **Description**: Finding additional issues during implementation
- **Mitigation**:
  - Focus on P1 and P2 only initially
  - Log other issues for future work
  - Don't fix unrelated bugs in same PR
  - Time-box investigation phases

**Risk 6: Integration Issues**
- **Probability**: LOW
- **Impact**: MEDIUM
- **Description**: Changes might not integrate cleanly with Python bindings
- **Mitigation**:
  - Test Python API after Rust changes
  - Verify PyO3 bindings work correctly
  - Run Python-side tests
  - Check memory management (reference counting)

### 6.3 Mitigation Summary

| Risk | Mitigation Strategy |
|------|---------------------|
| Parser conflicts | Incremental testing, pest validation |
| Performance | Early benchmarking, optimization budget |
| Breaking changes | Continuous regression testing |
| Class engine | Thorough diagnosis before fixing |
| Scope creep | Strict priority adherence |
| Integration | Python binding validation |

---

## 7. Success Metrics

### 7.1 Quantitative Metrics

| Metric | Baseline | Target | Measurement |
|--------|----------|--------|-------------|
| **Overall Pass Rate** | 72% (128/178) | 86%+ (153+/178) | Robot Framework tests |
| **Capture Tests** | 4% (1/26) | 100% (26/26) | 18_cascaded_capture.robot |
| **Class Engine Tests** | 63% (5/8) | 100% (8/8) | 17_cascaded_engines.robot |
| **Code Coverage** | TBD | 80%+ | cargo tarpaulin |
| **Performance** | N/A | <100ms | Cascade with 5 segments |
| **Documentation** | Spec only | Complete | API docs, examples |

### 7.2 Qualitative Metrics

**Code Quality**:
- ✅ No clippy warnings
- ✅ Consistent with existing style
- ✅ Clear error messages
- ✅ Comprehensive comments
- ✅ Maintainable abstractions

**User Experience**:
- ✅ Intuitive capture syntax
- ✅ Clear error messages when capture fails
- ✅ Expected behavior matches specification
- ✅ Performance acceptable for real-world use
- ✅ Documentation includes examples

**Testing**:
- ✅ Unit tests for all new functions
- ✅ Integration tests pass
- ✅ Edge cases covered
- ✅ Error conditions tested
- ✅ Performance benchmarks exist

### 7.3 Acceptance Criteria

**Definition of Done**:

1. **Feature Complete**:
   - [ ] Capture prefix (`*`) implemented and tested
   - [ ] Class engine works in cascaded contexts
   - [ ] All priority 1 and 2 tests pass

2. **Quality Assured**:
   - [ ] Code review approved
   - [ ] All tests pass (unit + integration)
   - [ ] No regressions introduced
   - [ ] Performance benchmarks pass
   - [ ] Documentation updated

3. **Production Ready**:
   - [ ] CHANGELOG.md updated
   - [ ] Version bumped appropriately
   - [ ] Release notes drafted
   - [ ] Known limitations documented
   - [ ] Migration guide provided (if needed)

---

## 8. Example Implementation (Capture Prefix)

### 8.1 AST Example

```rust
// src/locator/ast.rs

/// A segment in a cascaded selector chain with capture support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CascadedSegment {
    /// Whether this segment should be captured and returned
    pub capture: bool,

    /// The compound selector for this segment
    pub compound: CompoundSelector,

    /// Combinator to the next segment (None for last segment)
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

impl ComplexSelector {
    /// Check if this is a cascaded selector
    pub fn is_cascaded(&self) -> bool {
        self.compounds.iter().any(|c| {
            matches!(c.combinator, Some(Combinator::Cascaded))
        })
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

    /// Get index of first captured segment
    pub fn get_capture_index(&self) -> Option<usize> {
        self.cascaded_segments
            .as_ref()
            .and_then(|segs| {
                segs.iter()
                    .position(|s| s.capture)
            })
    }
}
```

### 8.2 Parser Example

```rust
// src/locator/parser.rs

fn parse_complex_selector(pair: Pair<Rule>) -> Result<ComplexSelector, ParseError> {
    let mut compounds = Vec::new();
    let mut cascaded_segments = Vec::new();
    let mut current_compound = CompoundSelector::new();
    let mut current_capture = false;
    let mut current_raw = String::new();
    let mut current_combinator: Option<Combinator> = None;
    let mut is_cascaded = false;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::capture_prefix => {
                current_capture = true;
                current_raw.push('*');
            }

            Rule::type_selector => {
                current_compound.type_selector = Some(parse_type_selector(inner_pair)?);
                current_raw.push_str(inner_pair.as_str());
            }

            Rule::combinator => {
                let comb = parse_combinator(&inner_pair)?;

                // Save current segment if we're building cascaded segments
                if matches!(comb, Combinator::Cascaded) {
                    is_cascaded = true;
                    cascaded_segments.push(CascadedSegment::new(
                        current_capture,
                        current_compound.clone(),
                        Some(comb),
                        current_raw.clone(),
                    ));
                } else if is_cascaded {
                    // Continue building cascaded segments
                    cascaded_segments.push(CascadedSegment::new(
                        current_capture,
                        current_compound.clone(),
                        Some(comb),
                        current_raw.clone(),
                    ));
                }

                // Save compound for backward compatibility
                compounds.push(current_compound.clone());

                // Reset for next segment
                current_compound = CompoundSelector::new();
                current_capture = false;
                current_raw.clear();
                current_combinator = Some(comb);
            }

            Rule::attribute_selector => {
                let attr = parse_attribute_selector(inner_pair)?;
                current_compound.attribute_selectors.push(attr);
                current_raw.push_str(inner_pair.as_str());
            }

            // ... handle other rules

            _ => {}
        }
    }

    // Push final segment
    if is_cascaded {
        cascaded_segments.push(CascadedSegment::new(
            current_capture,
            current_compound.clone(),
            None,
            current_raw,
        ));
    }
    compounds.push(current_compound);

    Ok(ComplexSelector {
        compounds,
        cascaded_segments: if is_cascaded { Some(cascaded_segments) } else { None },
    })
}
```

### 8.3 Matcher Example

```rust
// src/locator/matcher.rs

/// Find elements matching a cascaded locator with capture support
pub fn find_cascaded<'a>(
    locator: &ComplexSelector,
    root: &'a SwingComponent,
) -> Result<Vec<&'a SwingComponent>, MatchError> {

    // Get cascaded segments
    let segments = match locator.get_cascaded_segments() {
        Some(segs) => segs,
        None => return Err(MatchError::NotCascaded),
    };

    // Track state through the cascade
    let mut current_contexts: Vec<&SwingComponent> = vec![root];
    let mut captured_elements: Option<Vec<&SwingComponent>> = None;

    // Process each segment
    for (segment_idx, segment) in segments.iter().enumerate() {
        let mut next_contexts = Vec::new();

        // Search within each current context
        for context in &current_contexts {
            // Find all descendants matching this segment's selector
            let matches = find_in_context(&segment.compound, context)?;

            // Add matches to next contexts (remove duplicates)
            for match_elem in matches {
                if !next_contexts.contains(&match_elem) {
                    next_contexts.push(match_elem);
                }
            }
        }

        // Handle capture flag
        if segment.capture && captured_elements.is_none() {
            // This is the first segment with capture flag
            captured_elements = Some(next_contexts.clone());
        }

        // Check if we found anything
        if next_contexts.is_empty() {
            return Err(MatchError::CascadeNoMatch {
                segment_index: segment_idx,
                segment_text: segment.raw.clone(),
                previous_matches: current_contexts.len(),
            });
        }

        // Move to next segment using these results as new contexts
        current_contexts = next_contexts;
    }

    // Return captured elements if any, otherwise final results
    Ok(captured_elements.unwrap_or(current_contexts))
}

/// Find elements matching a compound selector within a specific context
fn find_in_context<'a>(
    compound: &CompoundSelector,
    context: &'a SwingComponent,
) -> Result<Vec<&'a SwingComponent>, MatchError> {
    let mut results = Vec::new();

    // Search all descendants of the context component
    // (but not the context itself - that was matched by previous segment)
    for child in context.get_children() {
        search_descendants_recursive(child, compound, &mut results)?;
    }

    Ok(results)
}

/// Recursively search descendants for matches
fn search_descendants_recursive<'a>(
    component: &'a SwingComponent,
    selector: &CompoundSelector,
    results: &mut Vec<&'a SwingComponent>,
) -> Result<(), MatchError> {

    // Check if current component matches
    if matches_compound_selector(component, selector)? {
        results.push(component);
    }

    // Recurse into children
    for child in component.get_children() {
        search_descendants_recursive(child, selector, results)?;
    }

    Ok(())
}
```

---

## 9. Documentation Plan

### 9.1 User Documentation

**Location**: `docs/user-guide/CASCADED_SELECTORS.md`

**Content Outline**:
```markdown
# Cascaded Selectors User Guide

## Introduction
- What are cascaded selectors?
- When to use them vs simple selectors
- Browser Library inspiration

## Basic Usage
- Syntax: `parent >> child`
- Examples with different component types
- Chaining multiple segments

## Capture Prefix
- Syntax: `*element >> child`
- Use cases
- Examples
- Performance considerations

## Selector Engines
- Using different engines in cascades
- Mixing engines
- Best practices

## Common Patterns
- Form navigation
- Table operations
- Tree operations
- Dialog handling

## Troubleshooting
- Common errors
- Performance tips
- Debugging strategies
```

### 9.2 API Documentation

Update Rust doc comments:

```rust
/// Find elements matching a cascaded locator.
///
/// Cascaded selectors use the `>>` separator to chain multiple selector
/// segments. Each segment is evaluated within the context of the previous
/// segment's matches.
///
/// # Capture Prefix
///
/// The `*` prefix marks a segment to be captured (returned) instead of
/// continuing to the final segment:
///
/// ```robot
/// # Returns JPanel, not JButton:
/// Get Element    *JPanel >> JButton
/// ```
///
/// Only the first `*` in a chain is effective.
///
/// # Examples
///
/// ```rust
/// use robotframework_swing::locator::{parse_locator, find_cascaded};
///
/// // Basic cascade
/// let loc = parse_locator("JDialog >> JPanel >> JButton").unwrap();
/// let results = find_cascaded(&loc, &root).unwrap();
///
/// // With capture
/// let loc = parse_locator("*JPanel[name='form'] >> JButton").unwrap();
/// let panel = find_cascaded(&loc, &root).unwrap();
/// ```
///
/// # Errors
///
/// Returns [`MatchError::CascadeNoMatch`] if any segment finds no matches.
pub fn find_cascaded<'a>(
    locator: &ComplexSelector,
    root: &'a SwingComponent,
) -> Result<Vec<&'a SwingComponent>, MatchError> {
    // ...
}
```

### 9.3 Migration Guide

**For Users**:

No breaking changes - all existing selectors continue to work. New features are additive.

**For Developers**:

```markdown
# Developer Migration Guide

## AST Changes
- `ComplexSelector` now has optional `cascaded_segments` field
- New `CascadedSegment` struct for capture flag storage

## Parser Changes
- `parse_complex_selector()` now builds cascaded segments
- Grammar supports `*` prefix

## Matcher Changes
- New `find_cascaded()` function for cascaded selectors
- Main `find()` routes to `find_cascaded()` when appropriate

## Compatibility
- All existing code continues to work
- Non-cascaded selectors unchanged
- Binary compatible (no ABI breaks)
```

---

## 10. Future Enhancements

### 10.1 Planned Features (Not in Scope)

These features are mentioned in the specification but NOT included in this implementation plan:

**Frame Support (`>>>`)**:
- Separate combinator for crossing frame boundaries
- Similar to Browser Library's iframe support
- Requires Java Agent support for frame detection

**Wait Conditions in Chain**:
- Syntax: `JDialog:visible >> JButton:enabled`
- Requires integration with wait/polling logic
- Would need retry mechanism in matcher

**Relative Position Selectors**:
- Syntax: `JLabel[text='Username:'] >> right-of >> JTextField`
- Requires spatial analysis of components
- Complex layout calculations needed

**Multiple Captures**:
- Return multiple captured elements: `[dialog, panel]`
- Requires API change (return value type)
- Breaking change for Python bindings

### 10.2 Optimization Opportunities

**Locator Caching**:
- Cache parsed locators for reuse
- Significant performance gain for repeated queries
- Implement as separate enhancement

**Parallel Search**:
- Search multiple contexts in parallel
- Use rayon for parallel iteration
- Benchmark before implementing

**Smart Context Pruning**:
- Skip branches that can't possibly match
- Requires lookahead analysis
- Complex but potentially high impact

**Query Optimization**:
- Reorder segments for efficiency
- Use most specific segments first
- Automatic optimization pass

---

## 11. Appendix

### 11.1 Related Documents

- **Specification**: [CASCADED_SELECTOR_SPECIFICATION.md](CASCADED_SELECTOR_SPECIFICATION.md)
- **Test Report**: [CASCADED_TEST_EXECUTION_REPORT.md](../test-plans/CASCADED_TEST_EXECUTION_REPORT.md)
- **Test Suites**:
  - `tests/robot/16_cascaded_basic.robot` (50% passing)
  - `tests/robot/17_cascaded_engines.robot` (87% passing)
  - `tests/robot/18_cascaded_capture.robot` (4% passing)
  - `tests/robot/19_cascaded_tables.robot` (100% passing)

### 11.2 References

- [Robot Framework Browser Library](https://marketsquare.github.io/robotframework-browser/)
- [Pest Parser Documentation](https://pest.rs/)
- [CSS Selectors Specification](https://www.w3.org/TR/selectors/)

### 11.3 Glossary

| Term | Definition |
|------|------------|
| **Cascaded Selector** | Selector using `>>` to chain multiple segments |
| **Segment** | One part of a cascaded selector (between `>>` separators) |
| **Capture Prefix** | The `*` marker indicating which segment to return |
| **Context** | The set of elements within which the next segment searches |
| **Compound Selector** | Selector with type, attributes, and pseudo-selectors |
| **Complex Selector** | One or more compound selectors with combinators |
| **Selector Engine** | Strategy for matching elements (CSS, XPath, name, etc.) |

### 11.4 Contact Information

**Implementation Questions**: Refer to specification and test report
**Bug Reports**: Create issue in robotframework-swing repository
**Feature Requests**: Discuss in project issues

---

## Document Revision History

| Version | Date | Author | Changes |
|---------|------|--------|---------|
| 1.0.0 | 2026-01-21 | Strategic Planning Agent | Initial implementation plan |

---

**END OF IMPLEMENTATION PLAN**

---

## Quick Reference

### Implementation Checklist

**Phase 1: Capture Prefix (P1 - Critical)**
- [ ] AST: Add CascadedSegment struct
- [ ] Grammar: Add capture_prefix rule
- [ ] Parser: Implement capture parsing
- [ ] Matcher: Implement find_cascaded()
- [ ] Tests: 26/26 capture tests passing

**Phase 2: Class Engine (P2 - High)**
- [ ] Diagnose: Find root cause
- [ ] Fix: Implement solution
- [ ] Tests: 8/8 class tests passing

**Success Criteria**
- [ ] Pass rate: 72% → 86%+
- [ ] Tests passing: 128 → 156+
- [ ] Performance: <100ms for 5-segment cascade
- [ ] Documentation: Complete
- [ ] Code review: Approved

**Estimated Timeline**: 1-2 weeks
**Estimated Effort**: 25-38 developer hours
