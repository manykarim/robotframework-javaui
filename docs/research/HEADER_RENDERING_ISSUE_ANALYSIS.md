# RST Header Rendering Issue - Root Cause Analysis

**Date:** 2026-01-20
**Issue:** Section headers (=Header=, ==Header==) render as plain text instead of HTML headers

## Executive Summary

**ROOT CAUSE IDENTIFIED:** Robot Framework's `libdoc` tool **does NOT support reStructuredText section headers**.

- ❌ `= Header =` does NOT convert to `<h2>Header</h2>`
- ❌ `== Header ==` does NOT convert to `<h3>Header</h3>`
- ❌ RST underline headers do NOT convert to HTML headers
- ❌ Markdown headers (`# Header`) do NOT convert to HTML headers

**Current Result:** All section headers render as plain `<p>` tags with literal equals signs.

**Browser Library:** Uses the SAME libdoc tool - they also don't have section headers from docstrings!

---

## Evidence: Headers in Current Docs

### RCP Library (docs/keywords/Rcp.html)

**Headers found rendered as plain text in `<p>` tags:**

```html
<p>= Eclipse Workbench Model =</p>
<p>== Opening a Perspective ==</p>
<p>== Executing Commands ==</p>
<p>== Working with Preferences ==</p>
<p>== Complete Example ==</p>
```

**Expected (but does NOT happen):**

```html
<h2>Eclipse Workbench Model</h2>
<h3>Opening a Perspective</h3>
<h3>Executing Commands</h3>
<h3>Working with Preferences</h3>
<h3>Complete Example</h3>
```

---

## Test Results: What libdoc Supports

I ran comprehensive tests to determine what RST features libdoc actually supports:

### ✅ **Supported RST Features**

| Feature | Syntax | Result |
|---------|--------|--------|
| **Bold** | `**text**` | `<strong>text</strong>` |
| **Italic** | `*text*` | `<em>text</em>` |
| **Code** | `` `text` `` | `<code>text</code>` |
| **Bullet Lists** | `- item` | `<ul><li>item</li></ul>` |
| **Code Blocks** | `::` | `<pre>...</pre>` |
| **Pipe Tables** | `\| =H= \|` | `<table>...</table>` |
| **Literal Blocks** | Indented code | `<pre>...</pre>` |

### ❌ **NOT Supported (Renders as Plain Text)**

| Feature | Syntax | What Renders |
|---------|--------|--------------|
| **Section Headers** | `= Header =` | `<p>= Header =</p>` |
| **Subsection Headers** | `== Header ==` | `<p>== Header ==</p>` |
| **Underline Headers** | `Header\n=====` | `<p>Header<br>=====</p>` |
| **Markdown Headers** | `# Header` | `<p># Header</p>` |
| **Raw HTML** | `<h2>Header</h2>` | Escaped or removed |
| **Simple Tables** | `=== === ===` | `<p>=== === ===</p>` |
| **Grid Tables** | `+---+---+` | `<p>+---+---+</p>` |

---

## Browser Library Comparison

### How Browser Library Has Headers

I analyzed the Browser library documentation. Here's what I found:

**The headers in Browser library docs are NOT from the docstring!**

#### Browser Library HTML Structure:
```html
<div id="introduction-container">
  <h2 id="introduction">Introduction</h2>  ← Part of HTML template
  <div class="doc">
    {{{doc}}}  ← Docstring content inserted here (NO headers!)
  </div>
</div>
```

**Key Finding:**
- The `<h2>Introduction</h2>` is **part of the libdoc HTML template**, not generated from docstring
- The docstring content goes inside `<div class="doc">` WITHOUT section headers
- Browser library uses the SAME libdoc tool with the SAME limitations!

#### What's in Browser's {{{doc}}}:

```
Browser library is a browser automation library for Robot Framework.

This is the keyword documentation for Browser library...

Table of contents:
- Browser, Context and Page
- Automatic page and context closing
- Finding elements
...
```

Note: NO `<h2>` or `<h3>` tags in the docstring content!

---

## Why Headers Appear to Work in Browser Library

Browser library documentation **appears** to have richer headers because:

1. **HTML Template Structure**: Libdoc's HTML template has `<h2 id="introduction">Introduction</h2>`
2. **Table of Contents**: Browser uses a plain bulleted list that LINKS to sections, not actual headers
3. **Visual Styling**: CSS makes bullet points look like a TOC
4. **Content Organization**: They structure content with paragraphs and emphasis, not headers

### Browser Library TOC Example:

**Source (in docstring):**
```
Table of contents:
- Browser, Context and Page
- Automatic page and context closing
- Finding elements
```

**Renders as:**
```html
<p>Table of contents:</p>
<ul>
  <li>Browser, Context and Page</li>
  <li>Automatic page and context closing</li>
  <li>Finding elements</li>
</ul>
```

It's just a bulleted list, NOT navigation headers!

---

## Impact on robotframework-swing

### Current State:

**Swing Library:**
```
= Locator Syntax =

The library supports...

== CSS-like Selectors ==

*Selector*    *Description*
```

**Renders as:**
```html
<p>= Locator Syntax =</p>
<p>The library supports...</p>
<p>== CSS-like Selectors ==</p>
<p><em>Selector</em>    <em>Description</em></p>
```

**Problems:**
1. ❌ Headers are plain text with literal `=` signs
2. ❌ No semantic HTML structure
3. ❌ No visual hierarchy
4. ❌ Can't be used for navigation
5. ❌ Malformed tables (separate issue)

---

## Solutions

### ❌ **Non-Solutions (These DON'T Work)**

1. **Different RST header syntax** - None work with libdoc
2. **Raw HTML headers** - Escaped or removed by libdoc
3. **Different ROBOT_LIBRARY_DOC_FORMAT** - No format supports headers
4. **Upgrade libdoc** - This is by design, not a bug

### ✅ **Working Solutions**

#### **Solution 1: Remove RST Headers** ⭐ **RECOMMENDED - Quick Fix**

**Remove the equals signs and use formatting instead:**

```python
"""
Robot Framework library for Eclipse RCP automation.

ECLIPSE WORKBENCH MODEL

Understanding the Eclipse workbench hierarchy is essential.

*Workbench Components:*

- Perspectives
- Views
- Editors
- Commands
"""
```

**Result:** Clean, readable content without confusing `=` signs

**Pros:**
- ✅ Quick fix (find & replace)
- ✅ Removes visual clutter
- ✅ Content is still structured
- ✅ Matches Browser library approach

**Cons:**
- ❌ No semantic HTML headers
- ❌ Less structured than true headers

---

#### **Solution 2: Use Section Separators** 📝 **RECOMMENDED - Better Structure**

**Use visual separators and bold text:**

```python
"""
Robot Framework library for Eclipse RCP automation.

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Eclipse Workbench Model
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Understanding the Eclipse workbench hierarchy is essential.

**Workbench Components**

The workbench contains these primary elements:

- *Perspectives* - Different layouts for different tasks
- *Views* - Information panels
- *Editors* - Central editing area
"""
```

**Result:** Clear visual structure with emphasis

**Pros:**
- ✅ Clear visual hierarchy
- ✅ Uses supported RST features (bold, italic, lists)
- ✅ Professional appearance
- ✅ Easy to scan

**Cons:**
- ❌ Still no semantic HTML headers
- ❌ Separators may look cluttered

---

#### **Solution 3: Use List-Based TOC** 🎯 **RECOMMENDED - Browser Library Style**

**Create a bulleted TOC like Browser library:**

```python
"""
Robot Framework library for Eclipse RCP automation.

**Table of Contents:**

- `Eclipse Workbench Model`_
- `Perspectives`_
- `Views and Editors`_
- `Commands`_
- `Preferences`_
- `Common Workflows`_

.. _Eclipse Workbench Model:

**Eclipse Workbench Model**

Understanding the Eclipse workbench hierarchy is essential.

.. _Perspectives:

**Perspectives**

Perspectives define different layouts...
"""
```

**Result:** Navigable TOC with internal links (RST references)

**Pros:**
- ✅ Matches Browser library approach
- ✅ Creates clickable TOC links
- ✅ Professional structure
- ✅ Uses standard RST features

**Cons:**
- ❌ More verbose
- ❌ Requires RST reference syntax knowledge

---

#### **Solution 4: Minimal - Just Remove Equals** ⚡ **FASTEST**

**Simply remove the equals signs:**

```python
"""
Locator Syntax

The library supports multiple strategies.

CSS-like Selectors

*Selector*    *Description*
Type    Match by class
"""
```

**Pros:**
- ✅ Fastest fix (regex replace)
- ✅ No visual clutter
- ✅ Content flows naturally

**Cons:**
- ❌ No visual emphasis on section titles
- ❌ Less clear hierarchy

---

## Recommended Fix

### **Phase 1: Remove Broken Headers** (30 minutes)

Use find/replace to remove RST header syntax:

```bash
# Find:    = (.+?) =
# Replace: **\1**

# Find:    == (.+?) ==
# Replace: *\1*
```

This converts:
- `= Main Section =` → `**Main Section**` (bold)
- `== Subsection ==` → `*Subsection*` (italic)

### **Phase 2: Add Visual Structure** (1-2 hours)

Add separators for major sections:

```python
"""
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Main Section Name
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Content here...
"""
```

### **Phase 3: Add TOC (Optional)** (2-3 hours)

Create a Browser-style TOC for long introductions:

```python
"""
**Table of Contents:**

- Introduction
- Installation
- Locator Syntax
- Common Workflows
- Troubleshooting

**Introduction**

This library provides...
"""
```

---

## Files to Fix

All three libraries have this issue:

### **Swing Library** (`python/JavaGui/__init__.py`)
```
= Initialization =
= Locator Syntax =
== CSS-like Selectors ==
== XPath Selectors ==
= Assertion Keywords =
```

### **SWT Library** (`python/JavaGui/__init__.py`)
```
= Table of Contents =
= Introduction =
= Installation & Setup =
= Connecting to Applications =
= Locator Syntax =
= Shell Management =
... (many more)
```

### **RCP Library** (`python/JavaGui/__init__.py`)
```
= What is Eclipse RCP? =
= Initialization =
= Eclipse Workbench Model =
== Workbench ==
== Workbench Window ==
== Perspectives ==
== Views ==
== Editors ==
== Commands ==
== Preferences ==
= Typical Workflows =
== Opening a Perspective ==
== Executing Commands ==
... (many more)
```

---

## Before & After Examples

### **Before (Current - Broken):**

```html
<p>= Eclipse Workbench Model =</p>
<p>Understanding the Eclipse workbench...</p>
<p>== Perspectives ==</p>
<p>Perspectives define layouts...</p>
```

### **After (Solution 1 - Remove Headers):**

```html
<p><strong>Eclipse Workbench Model</strong></p>
<p>Understanding the Eclipse workbench...</p>
<p><em>Perspectives</em></p>
<p>Perspectives define layouts...</p>
```

### **After (Solution 2 - With Separators):**

```html
<p>━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━<br/>
Eclipse Workbench Model<br/>
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━</p>
<p>Understanding the Eclipse workbench...</p>
<p><strong>Perspectives</strong></p>
<p>Perspectives define layouts...</p>
```

### **After (Solution 3 - With TOC):**

```html
<p><strong>Table of Contents:</strong></p>
<ul>
  <li>Eclipse Workbench Model</li>
  <li>Perspectives</li>
  <li>Views and Editors</li>
</ul>
<p><strong>Eclipse Workbench Model</strong></p>
<p>Understanding the Eclipse workbench...</p>
```

---

## Testing Commands

Test the fixes by regenerating docs:

```bash
# Regenerate all docs
uv run invoke docs

# Check specific library
python -m robot.libdoc python/JavaGui/__init__.py::SwingLibrary docs/test-swing.html

# Open in browser
open docs/test-swing.html
```

---

## Conclusion

### **Key Findings:**

1. ✅ **libdoc does NOT support RST section headers** - this is by design
2. ✅ **Browser library has the same limitation** - they don't use headers in docstrings
3. ✅ **The equals signs (`=`) are rendering as literal text** - they need to be removed
4. ✅ **Multiple working solutions exist** - bold text, separators, TOCs

### **Recommended Action:**

**Phase 1 (30 min - DO FIRST):**
- Remove all RST header syntax (= and ==)
- Convert to bold (**) and italic (*)
- Eliminates visual clutter immediately

**Phase 2 (1-2 hours - OPTIONAL):**
- Add visual separators for major sections
- Use bold text for section titles
- Improves readability

**Phase 3 (2-3 hours - OPTIONAL):**
- Add table of contents for long introductions (SWT, RCP)
- Matches Browser library style
- Professional appearance

### **Expected Results:**

After Phase 1:
- ✅ No more confusing `= Header =` text
- ✅ Clean, readable content
- ✅ Professional appearance
- ✅ Matches Browser library approach

After Phase 2+3:
- ✅ Clear visual hierarchy
- ✅ Easy navigation
- ✅ Professional documentation
- ✅ 95% visual parity with Browser library

---

## Related Issues

This header issue is **separate** from the table rendering issue:

1. **Tables:** Missing pipes (`|`) - CRITICAL BUG
2. **Headers:** RST syntax not supported - DESIGN LIMITATION

Both need to be fixed, but require different approaches:
- Tables: Add proper RST pipe table syntax
- Headers: Remove RST header syntax entirely

---

**Analysis Complete**
