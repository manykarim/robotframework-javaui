# Component Tree Filtering Guide

This guide explains how to use the advanced filtering capabilities when retrieving component trees with `Get Component Tree` and `Get UI Tree`.

## Table of Contents

1. [Overview](#overview)
2. [Type Filtering](#type-filtering)
3. [State Filtering](#state-filtering)
4. [Combined Filtering](#combined-filtering)
5. [Filter Logic](#filter-logic)
6. [Performance Considerations](#performance-considerations)
7. [Examples](#examples)

## Overview

The component tree retrieval methods support advanced filtering to help you focus on specific components. This is useful for:

- **Debugging**: Quickly find all buttons or text fields in a complex UI
- **Testing**: Verify only enabled or visible components
- **Performance**: Reduce tree size by filtering out irrelevant components
- **Analysis**: Focus on specific component types or states

### Available Filter Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `types` | String | Component types to include (comma-separated) |
| `exclude_types` | String | Component types to exclude (comma-separated) |
| `visible_only` | Boolean | Only include visible components |
| `enabled_only` | Boolean | Only include enabled components |
| `focusable_only` | Boolean | Only include focusable components |
| `max_depth` | Integer | Maximum tree depth to traverse |

## Type Filtering

Type filtering allows you to include or exclude specific component types from the tree.

### Basic Type Filtering

\`\`\`robot
# Get only buttons
\${buttons}=    Get Component Tree    types=JButton    format=json

# Get multiple types
\${inputs}=    Get Component Tree    types=JButton,JTextField,JTextArea
\`\`\`

### Wildcard Patterns

Wildcards allow flexible matching of component types:

- \`*\` matches any sequence of characters
- \`?\` matches any single character

\`\`\`robot
# Get all button types (JButton, JToggleButton, JRadioButton, etc.)
\${all_buttons}=    Get Component Tree    types=J*Button

# Get all text components (JTextField, JTextArea, JTextPane, etc.)
\${text_components}=    Get Component Tree    types=JText*
\`\`\`

### Exclusion Filtering

\`\`\`robot
# Get all components except labels
\${tree}=    Get Component Tree    exclude_types=JLabel

# Include buttons but exclude radio buttons
\${buttons}=    Get Component Tree    types=J*Button    exclude_types=JRadioButton
\`\`\`

## State Filtering

### Visibility, Enabled, and Focusable

\`\`\`robot
# Get only visible components
\${visible}=    Get Component Tree    visible_only=\${True}

# Get only enabled components
\${enabled}=    Get Component Tree    enabled_only=\${True}

# Get only focusable components
\${focusable}=    Get Component Tree    focusable_only=\${True}
\`\`\`

## Combined Filtering

\`\`\`robot
# Get visible, enabled buttons
\${buttons}=    Get Component Tree
...    types=JButton
...    visible_only=\${True}
...    enabled_only=\${True}
\`\`\`

## Filter Logic

- All filters use AND logic
- Exclusions take precedence over inclusions
- Type matching is case-sensitive
- Filters are applied during traversal for performance

For complete examples and details, see the full documentation.
