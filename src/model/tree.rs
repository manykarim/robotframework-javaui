//! UI Tree model for complete component hierarchy

use super::element::UIElement;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Complete UI tree for a window/application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UITree {
    /// Window title
    pub window_title: String,
    /// Window class name
    pub window_class: String,
    /// Root element of the tree
    pub root: UIElement,
    /// When the tree was captured
    pub timestamp: DateTime<Utc>,
    /// JVM process ID
    pub jvm_pid: u32,
    /// Tree statistics
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats: Option<TreeStats>,
}

/// Statistics about the UI tree
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TreeStats {
    /// Total number of elements
    pub total_elements: usize,
    /// Maximum depth of the tree
    pub max_depth: usize,
    /// Count by component type
    pub type_counts: HashMap<String, usize>,
    /// Number of visible elements
    pub visible_count: usize,
    /// Number of enabled elements
    pub enabled_count: usize,
}

impl UITree {
    /// Create a new UITree
    pub fn new(window_title: String, window_class: String, root: UIElement, jvm_pid: u32) -> Self {
        Self {
            window_title,
            window_class,
            root,
            timestamp: Utc::now(),
            jvm_pid,
            stats: None,
        }
    }

    /// Calculate and store statistics
    pub fn calculate_stats(&mut self) {
        let mut stats = TreeStats::default();
        self.collect_stats(&self.root, 0, &mut stats);
        self.stats = Some(stats);
    }

    fn collect_stats(&self, element: &UIElement, depth: usize, stats: &mut TreeStats) {
        stats.total_elements += 1;
        stats.max_depth = stats.max_depth.max(depth);

        *stats
            .type_counts
            .entry(element.simple_class_name.clone())
            .or_insert(0) += 1;

        if element.state.visible {
            stats.visible_count += 1;
        }
        if element.state.enabled {
            stats.enabled_count += 1;
        }

        for child in &element.children {
            self.collect_stats(child, depth + 1, stats);
        }
    }

    /// Serialize to JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Serialize to compact JSON
    pub fn to_json_compact(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize to YAML
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(self)
    }

    /// Serialize to human-readable tree format
    pub fn to_text_tree(&self) -> String {
        let mut output = format!("Window: {} [{}]\n", self.window_title, self.window_class);
        output.push_str(&format!("Captured: {}\n", self.timestamp.format("%Y-%m-%d %H:%M:%S")));
        output.push_str(&format!("PID: {}\n\n", self.jvm_pid));
        self.format_element(&self.root, "", true, &mut output);
        output
    }

    fn format_element(&self, elem: &UIElement, prefix: &str, last: bool, output: &mut String) {
        let connector = if last { "└── " } else { "├── " };
        let child_prefix = if last { "    " } else { "│   " };

        // Build element description
        let mut desc = elem.simple_class_name.clone();

        if let Some(ref name) = elem.name {
            desc.push_str(&format!(" [{}]", name));
        }

        if let Some(ref text) = elem.text {
            let display_text = if text.len() > 30 {
                format!("\"{}...\"", &text[..27])
            } else {
                format!("\"{}\"", text)
            };
            desc.push_str(&format!(" {}", display_text));
        }

        if !elem.state.visible {
            desc.push_str(" (hidden)");
        }
        if !elem.state.enabled {
            desc.push_str(" (disabled)");
        }

        output.push_str(&format!("{}{}{}\n", prefix, connector, desc));

        let new_prefix = format!("{}{}", prefix, child_prefix);
        let child_count = elem.children.len();

        for (i, child) in elem.children.iter().enumerate() {
            let is_last = i == child_count - 1;
            self.format_element(child, &new_prefix, is_last, output);
        }
    }

    /// Format as Robot Framework log-friendly output
    pub fn to_robot_log(&self) -> String {
        let mut output = String::new();
        output.push_str("*HTML* <pre>\n");
        output.push_str(&self.to_text_tree());
        output.push_str("</pre>");
        output
    }

    /// Find all elements matching a predicate
    pub fn find_all<F>(&self, predicate: F) -> Vec<&UIElement>
    where
        F: Fn(&UIElement) -> bool,
    {
        let mut results = Vec::new();
        self.collect_matching(&self.root, &predicate, &mut results);
        results
    }

    fn collect_matching<'a, F>(
        &'a self,
        element: &'a UIElement,
        predicate: &F,
        results: &mut Vec<&'a UIElement>,
    ) where
        F: Fn(&UIElement) -> bool,
    {
        if predicate(element) {
            results.push(element);
        }
        for child in &element.children {
            self.collect_matching(child, predicate, results);
        }
    }

    /// Get a summary of the tree (limited depth)
    pub fn to_summary(&self, max_depth: usize) -> String {
        let mut output = format!("Window: {}\n", self.window_title);
        self.format_element_summary(&self.root, "", true, 0, max_depth, &mut output);
        output
    }

    fn format_element_summary(
        &self,
        elem: &UIElement,
        prefix: &str,
        last: bool,
        depth: usize,
        max_depth: usize,
        output: &mut String,
    ) {
        if depth > max_depth {
            return;
        }

        let connector = if last { "└── " } else { "├── " };
        let child_prefix = if last { "    " } else { "│   " };

        let name_part = elem.name.as_deref().unwrap_or("");
        output.push_str(&format!(
            "{}{}{} [{}]\n",
            prefix, connector, elem.simple_class_name, name_part
        ));

        if depth == max_depth && !elem.children.is_empty() {
            let new_prefix = format!("{}{}", prefix, child_prefix);
            output.push_str(&format!("{}... ({} more children)\n", new_prefix, elem.children.len()));
            return;
        }

        let new_prefix = format!("{}{}", prefix, child_prefix);
        let child_count = elem.children.len();

        for (i, child) in elem.children.iter().enumerate() {
            let is_last = i == child_count - 1;
            self.format_element_summary(child, &new_prefix, is_last, depth + 1, max_depth, output);
        }
    }
}

/// Filter specification for UI tree queries
#[derive(Debug, Clone, Default)]
pub struct TreeFilter {
    /// Include only visible elements
    pub visible_only: bool,
    /// Include only enabled elements
    pub enabled_only: bool,
    /// Maximum depth to traverse
    pub max_depth: Option<usize>,
    /// Filter by component types (include)
    pub include_types: Vec<String>,
    /// Filter by component types (exclude)
    pub exclude_types: Vec<String>,
    /// Filter by name pattern (regex)
    pub name_pattern: Option<String>,
    /// Filter by text pattern (regex)
    pub text_pattern: Option<String>,
}

impl TreeFilter {
    /// Create a new empty filter
    pub fn new() -> Self {
        Self::default()
    }

    /// Set visible_only filter
    pub fn visible_only(mut self, value: bool) -> Self {
        self.visible_only = value;
        self
    }

    /// Set enabled_only filter
    pub fn enabled_only(mut self, value: bool) -> Self {
        self.enabled_only = value;
        self
    }

    /// Set max_depth filter
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }

    /// Add type to include
    pub fn include_type(mut self, type_name: impl Into<String>) -> Self {
        self.include_types.push(type_name.into());
        self
    }

    /// Add type to exclude
    pub fn exclude_type(mut self, type_name: impl Into<String>) -> Self {
        self.exclude_types.push(type_name.into());
        self
    }

    /// Set name pattern
    pub fn name_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.name_pattern = Some(pattern.into());
        self
    }

    /// Set text pattern
    pub fn text_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.text_pattern = Some(pattern.into());
        self
    }

    /// Check if an element matches this filter
    pub fn matches(&self, element: &UIElement, depth: usize) -> bool {
        // Check depth
        if let Some(max) = self.max_depth {
            if depth > max {
                return false;
            }
        }

        // Check visibility
        if self.visible_only && !element.state.visible {
            return false;
        }

        // Check enabled state
        if self.enabled_only && !element.state.enabled {
            return false;
        }

        // Check include types
        if !self.include_types.is_empty() {
            let matches = self.include_types.iter().any(|t| element.matches_type(t));
            if !matches {
                return false;
            }
        }

        // Check exclude types
        if self.exclude_types.iter().any(|t| element.matches_type(t)) {
            return false;
        }

        // Check name pattern
        if let Some(ref pattern) = self.name_pattern {
            if let Some(ref name) = element.name {
                if let Ok(re) = regex::Regex::new(pattern) {
                    if !re.is_match(name) {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }

        // Check text pattern
        if let Some(ref pattern) = self.text_pattern {
            if let Some(ref text) = element.text {
                if let Ok(re) = regex::Regex::new(pattern) {
                    if !re.is_match(text) {
                        return false;
                    }
                }
            } else {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::element::ElementState;

    fn create_test_tree() -> UITree {
        let mut root = UIElement::new("1".to_string(), "javax.swing.JFrame".to_string());
        root.name = Some("mainFrame".to_string());
        root.state = ElementState {
            visible: true,
            enabled: true,
            ..Default::default()
        };

        let mut button = UIElement::new("2".to_string(), "javax.swing.JButton".to_string());
        button.name = Some("submitBtn".to_string());
        button.text = Some("Submit".to_string());
        button.state = ElementState {
            visible: true,
            enabled: true,
            ..Default::default()
        };

        root.children.push(button);

        UITree::new("Test Window".to_string(), "JFrame".to_string(), root, 12345)
    }

    #[test]
    fn test_to_text_tree() {
        let tree = create_test_tree();
        let text = tree.to_text_tree();
        assert!(text.contains("Test Window"));
        assert!(text.contains("JFrame"));
        assert!(text.contains("JButton"));
        assert!(text.contains("submitBtn"));
    }

    #[test]
    fn test_calculate_stats() {
        let mut tree = create_test_tree();
        tree.calculate_stats();
        let stats = tree.stats.as_ref().unwrap();
        assert_eq!(stats.total_elements, 2);
        assert_eq!(stats.visible_count, 2);
    }

    #[test]
    fn test_filter() {
        let tree = create_test_tree();
        let filter = TreeFilter::new().include_type("JButton");
        let results = tree.find_all(|e| filter.matches(e, 0));
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].simple_class_name, "JButton");
    }
}
