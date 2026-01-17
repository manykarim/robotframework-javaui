//! Comprehensive unit tests for the unified locator system
//!
//! Tests cover:
//! - Prefix locators (name:, text:, class:, id:, index:)
//! - CSS-style locators with attributes
//! - ID shorthand (#name)
//! - XPath locators
//! - Type mappings between Swing and SWT
//! - Locator factory for different toolkits
//! - Predicate parsing and handling

#[cfg(test)]
mod locator_parsing_tests {
    use crate::locator::unified::{UnifiedLocator, LocatorType, LocatorParseError};

    #[test]
    fn test_parse_name_locator() {
        let locator = UnifiedLocator::parse("name:myButton").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Name);
        assert_eq!(locator.value, "myButton");
        assert_eq!(locator.original, "name:myButton");
    }

    #[test]
    fn test_parse_name_locator_with_spaces() {
        let locator = UnifiedLocator::parse("name:my button").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Name);
        assert_eq!(locator.value, "my button");
    }

    #[test]
    fn test_parse_name_locator_case_insensitive() {
        let locator = UnifiedLocator::parse("NAME:TestButton").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Name);
        assert_eq!(locator.value, "TestButton");
    }

    #[test]
    fn test_parse_text_locator() {
        let locator = UnifiedLocator::parse("text:Click Me").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Text);
        assert_eq!(locator.value, "Click Me");
    }

    #[test]
    fn test_parse_text_locator_with_colon() {
        let locator = UnifiedLocator::parse("text:Error: Something went wrong").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Text);
        assert_eq!(locator.value, "Error: Something went wrong");
    }

    #[test]
    fn test_parse_class_locator() {
        let locator = UnifiedLocator::parse("class:JButton").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Class);
        assert_eq!(locator.value, "JButton");
    }

    #[test]
    fn test_parse_id_locator() {
        let locator = UnifiedLocator::parse("id:12345").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Id);
        assert_eq!(locator.value, "12345");
    }

    #[test]
    fn test_parse_index_locator() {
        let locator = UnifiedLocator::parse("index:0").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Index);
        assert_eq!(locator.value, "0");
    }

    #[test]
    fn test_parse_index_locator_larger_value() {
        let locator = UnifiedLocator::parse("index:42").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Index);
        assert_eq!(locator.value, "42");
    }

    #[test]
    fn test_parse_empty_locator() {
        let result = UnifiedLocator::parse("");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.message.contains("empty"));
    }

    #[test]
    fn test_parse_whitespace_only() {
        let result = UnifiedLocator::parse("   ");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_locator_trimmed() {
        let locator = UnifiedLocator::parse("  name:test  ").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Name);
        assert_eq!(locator.value, "test");
    }
}

#[cfg(test)]
mod id_shorthand_tests {
    use crate::locator::unified::{UnifiedLocator, LocatorType};

    #[test]
    fn test_parse_id_shorthand() {
        let locator = UnifiedLocator::parse("#myButton").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Name);
        assert_eq!(locator.value, "myButton");
    }

    #[test]
    fn test_parse_id_shorthand_with_underscores() {
        let locator = UnifiedLocator::parse("#my_button_123").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Name);
        assert_eq!(locator.value, "my_button_123");
    }

    #[test]
    fn test_parse_id_shorthand_with_dashes() {
        let locator = UnifiedLocator::parse("#my-button").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Name);
        assert_eq!(locator.value, "my-button");
    }

    #[test]
    fn test_parse_id_shorthand_empty() {
        let locator = UnifiedLocator::parse("#").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Name);
        assert_eq!(locator.value, "");
    }
}

#[cfg(test)]
mod css_locator_tests {
    use crate::locator::unified::{UnifiedLocator, LocatorType, LocatorPredicate, MatchOp};

    #[test]
    fn test_parse_css_type_only() {
        let locator = UnifiedLocator::parse("JButton").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Class);
        assert_eq!(locator.value, "JButton");
        assert!(locator.predicates.is_empty());
    }

    #[test]
    fn test_parse_css_with_attribute_equals() {
        let locator = UnifiedLocator::parse("JButton[text=\"Save\"]").unwrap();
        assert_eq!(locator.locator_type, LocatorType::Css);
        assert_eq!(locator.value, "JButton");
        assert_eq!(locator.predicates.len(), 1);

        if let LocatorPredicate::Attribute { name, op, value } = &locator.predicates[0] {
            assert_eq!(name, "text");
            assert_eq!(*op, MatchOp::Equals);
            assert_eq!(value, "Save");
        } else {
            panic!("Expected attribute predicate");
        }
    }

    #[test]
    fn test_parse_css_with_single_quotes() {
        let locator = UnifiedLocator::parse("JButton[text='Save']").unwrap();
        if let LocatorPredicate::Attribute { value, .. } = &locator.predicates[0] {
            assert_eq!(value, "Save");
        }
    }

    #[test]
    fn test_parse_css_with_contains() {
        let locator = UnifiedLocator::parse("JButton[text*=\"Save\"]").unwrap();
        if let LocatorPredicate::Attribute { op, .. } = &locator.predicates[0] {
            assert_eq!(*op, MatchOp::Contains);
        }
    }

    #[test]
    fn test_parse_css_with_starts_with() {
        let locator = UnifiedLocator::parse("JButton[text^=\"Save\"]").unwrap();
        if let LocatorPredicate::Attribute { op, .. } = &locator.predicates[0] {
            assert_eq!(*op, MatchOp::StartsWith);
        }
    }

    #[test]
    fn test_parse_css_with_ends_with() {
        let locator = UnifiedLocator::parse("JLabel[text$=\":\"]").unwrap();
        if let LocatorPredicate::Attribute { op, .. } = &locator.predicates[0] {
            assert_eq!(*op, MatchOp::EndsWith);
        }
    }

    #[test]
    fn test_parse_css_with_regex() {
        let locator = UnifiedLocator::parse("JButton[text~=\".*Save.*\"]").unwrap();
        if let LocatorPredicate::Attribute { op, .. } = &locator.predicates[0] {
            assert_eq!(*op, MatchOp::Regex);
        }
    }

    #[test]
    fn test_parse_css_with_multiple_attributes() {
        let locator = UnifiedLocator::parse("JButton[name=\"btn\"][text=\"OK\"]").unwrap();
        assert_eq!(locator.predicates.len(), 2);

        if let LocatorPredicate::Attribute { name, .. } = &locator.predicates[0] {
            assert_eq!(name, "name");
        }
        if let LocatorPredicate::Attribute { name, .. } = &locator.predicates[1] {
            assert_eq!(name, "text");
        }
    }

    #[test]
    fn test_parse_css_with_pseudo_class() {
        let locator = UnifiedLocator::parse("JButton[name=\"btn\"]:visible").unwrap();
        assert_eq!(locator.predicates.len(), 2);

        if let LocatorPredicate::PseudoClass(pseudo) = &locator.predicates[1] {
            assert_eq!(pseudo, "visible");
        } else {
            panic!("Expected pseudo-class predicate");
        }
    }

    #[test]
    fn test_parse_css_multiple_pseudo_classes() {
        let locator = UnifiedLocator::parse("JButton:enabled:visible").unwrap();
        assert_eq!(locator.predicates.len(), 2);

        if let LocatorPredicate::PseudoClass(p1) = &locator.predicates[0] {
            assert_eq!(p1, "enabled");
        }
        if let LocatorPredicate::PseudoClass(p2) = &locator.predicates[1] {
            assert_eq!(p2, "visible");
        }
    }

    #[test]
    fn test_parse_css_attribute_with_spaces() {
        let locator = UnifiedLocator::parse("JButton[ text = \"Save\" ]").unwrap();
        if let LocatorPredicate::Attribute { name, value, .. } = &locator.predicates[0] {
            assert_eq!(name, "text");
            assert_eq!(value, "Save");
        }
    }
}

#[cfg(test)]
mod xpath_locator_tests {
    use crate::locator::unified::{UnifiedLocator, LocatorType};

    #[test]
    fn test_parse_xpath_simple() {
        let locator = UnifiedLocator::parse("//JButton").unwrap();
        assert_eq!(locator.locator_type, LocatorType::XPath);
        assert_eq!(locator.value, "//JButton");
    }

    #[test]
    fn test_parse_xpath_with_attribute() {
        let locator = UnifiedLocator::parse("//JButton[@name='test']").unwrap();
        assert_eq!(locator.locator_type, LocatorType::XPath);
        assert_eq!(locator.value, "//JButton[@name='test']");
    }

    #[test]
    fn test_parse_xpath_with_text() {
        let locator = UnifiedLocator::parse("//JButton[@text='Save']").unwrap();
        assert_eq!(locator.locator_type, LocatorType::XPath);
    }

    #[test]
    fn test_parse_xpath_descendant() {
        let locator = UnifiedLocator::parse("//JPanel//JButton").unwrap();
        assert_eq!(locator.locator_type, LocatorType::XPath);
    }

    #[test]
    fn test_parse_xpath_with_parentheses() {
        let locator = UnifiedLocator::parse("(//JButton)[1]").unwrap();
        assert_eq!(locator.locator_type, LocatorType::XPath);
        assert_eq!(locator.value, "(//JButton)[1]");
    }

    #[test]
    fn test_parse_xpath_complex() {
        let locator = UnifiedLocator::parse("//JPanel[@name='main']//JButton[contains(@text, 'Save')]").unwrap();
        assert_eq!(locator.locator_type, LocatorType::XPath);
    }
}

#[cfg(test)]
mod toolkit_specific_locator_tests {
    use crate::locator::unified::{UnifiedLocator, LocatorType};

    #[test]
    fn test_parse_swing_toolkit_specific() {
        let locator = UnifiedLocator::parse("swing:JButton").unwrap();
        if let LocatorType::Toolkit { toolkit, selector } = &locator.locator_type {
            assert_eq!(toolkit, "swing");
            assert_eq!(selector, "JButton");
        } else {
            panic!("Expected toolkit locator type");
        }
    }

    #[test]
    fn test_parse_swt_toolkit_specific() {
        let locator = UnifiedLocator::parse("swt:Button").unwrap();
        if let LocatorType::Toolkit { toolkit, selector } = &locator.locator_type {
            assert_eq!(toolkit, "swt");
            assert_eq!(selector, "Button");
        }
    }

    #[test]
    fn test_parse_rcp_toolkit_specific() {
        let locator = UnifiedLocator::parse("rcp:ViewPart").unwrap();
        if let LocatorType::Toolkit { toolkit, selector } = &locator.locator_type {
            assert_eq!(toolkit, "rcp");
            assert_eq!(selector, "ViewPart");
        }
    }

    #[test]
    fn test_parse_toolkit_with_complex_selector() {
        let locator = UnifiedLocator::parse("swing:JButton[text='Save']").unwrap();
        if let LocatorType::Toolkit { selector, .. } = &locator.locator_type {
            assert_eq!(selector, "JButton[text='Save']");
        }
    }
}

#[cfg(test)]
mod locator_factory_tests {
    use crate::locator::unified::{UnifiedLocator, LocatorFactory};
    use crate::core::backend::ToolkitType;

    #[test]
    fn test_locator_factory_name_swing() {
        let locator = UnifiedLocator::name("testButton");
        let params = LocatorFactory::to_swing_params(&locator);

        assert_eq!(params["locatorType"], "name");
        assert_eq!(params["value"], "testButton");
    }

    #[test]
    fn test_locator_factory_name_swt() {
        let locator = UnifiedLocator::name("testButton");
        let params = LocatorFactory::to_swt_params(&locator);

        assert_eq!(params["locatorType"], "name");
        assert_eq!(params["value"], "testButton");
    }

    #[test]
    fn test_locator_factory_text() {
        let locator = UnifiedLocator::text("Click Me");
        let params = LocatorFactory::to_swing_params(&locator);

        assert_eq!(params["locatorType"], "text");
        assert_eq!(params["value"], "Click Me");
    }

    #[test]
    fn test_locator_factory_class_swing_normalization() {
        let locator = UnifiedLocator::class("Button");
        let params = LocatorFactory::to_swing_params(&locator);

        // For Swing, Button should become JButton
        assert_eq!(params["value"], "JButton");
    }

    #[test]
    fn test_locator_factory_class_swt_normalization() {
        let locator = UnifiedLocator::class("JButton");
        let params = LocatorFactory::to_swt_params(&locator);

        // For SWT, JButton should become Button
        assert_eq!(params["value"], "Button");
    }

    #[test]
    fn test_locator_factory_to_params_swing() {
        let locator = UnifiedLocator::name("test");
        let params = LocatorFactory::to_params(&locator, ToolkitType::Swing);
        assert_eq!(params["locatorType"], "name");
    }

    #[test]
    fn test_locator_factory_to_params_swt() {
        let locator = UnifiedLocator::name("test");
        let params = LocatorFactory::to_params(&locator, ToolkitType::Swt);
        assert_eq!(params["locatorType"], "name");
    }

    #[test]
    fn test_locator_factory_to_params_rcp() {
        let locator = UnifiedLocator::name("test");
        let params = LocatorFactory::to_params(&locator, ToolkitType::Rcp);
        assert_eq!(params["locatorType"], "name");
    }

    #[test]
    fn test_locator_factory_index() {
        let locator = UnifiedLocator::index(5);
        let params = LocatorFactory::to_swing_params(&locator);

        assert_eq!(params["locatorType"], "index");
        assert_eq!(params["value"], 5);
    }

    #[test]
    fn test_locator_factory_id() {
        let locator = UnifiedLocator::id("12345");
        let params = LocatorFactory::to_swing_params(&locator);

        assert_eq!(params["locatorType"], "hashCode");
        assert_eq!(params["value"], 12345);
    }

    #[test]
    fn test_locator_factory_xpath() {
        let locator = UnifiedLocator::xpath("//JButton[@text='Save']");
        let params = LocatorFactory::to_swing_params(&locator);

        assert_eq!(params["locatorType"], "xpath");
        assert_eq!(params["xpath"], "//JButton[@text='Save']");
    }
}

#[cfg(test)]
mod swing_normalization_tests {
    use crate::locator::unified::UnifiedLocator;
    use crate::core::backend::ToolkitType;

    fn normalize_for_swing(value: &str) -> String {
        let locator = UnifiedLocator::class(value);
        let normalized = locator.normalize_for_toolkit(ToolkitType::Swing);
        normalized.value
    }

    #[test]
    fn test_swing_to_unified_button() {
        assert_eq!(normalize_for_swing("Button"), "JButton");
    }

    #[test]
    fn test_swing_to_unified_textfield() {
        assert_eq!(normalize_for_swing("TextField"), "JTextField");
    }

    #[test]
    fn test_swing_to_unified_textarea() {
        assert_eq!(normalize_for_swing("TextArea"), "JTextField");
    }

    #[test]
    fn test_swing_to_unified_text() {
        assert_eq!(normalize_for_swing("Text"), "JTextField");
    }

    #[test]
    fn test_swing_to_unified_label() {
        assert_eq!(normalize_for_swing("Label"), "JLabel");
    }

    #[test]
    fn test_swing_to_unified_combobox() {
        assert_eq!(normalize_for_swing("ComboBox"), "JComboBox");
    }

    #[test]
    fn test_swing_to_unified_combo() {
        assert_eq!(normalize_for_swing("Combo"), "JComboBox");
    }

    #[test]
    fn test_swing_to_unified_list() {
        assert_eq!(normalize_for_swing("List"), "JList");
    }

    #[test]
    fn test_swing_to_unified_table() {
        assert_eq!(normalize_for_swing("Table"), "JTable");
    }

    #[test]
    fn test_swing_to_unified_tree() {
        assert_eq!(normalize_for_swing("Tree"), "JTree");
    }

    #[test]
    fn test_swing_to_unified_checkbox() {
        assert_eq!(normalize_for_swing("CheckBox"), "JCheckBox");
    }

    #[test]
    fn test_swing_to_unified_radiobutton() {
        assert_eq!(normalize_for_swing("RadioButton"), "JRadioButton");
    }

    #[test]
    fn test_swing_to_unified_panel() {
        assert_eq!(normalize_for_swing("Panel"), "JPanel");
    }

    #[test]
    fn test_swing_to_unified_frame() {
        assert_eq!(normalize_for_swing("Frame"), "JFrame");
    }

    #[test]
    fn test_swing_to_unified_dialog() {
        assert_eq!(normalize_for_swing("Dialog"), "JDialog");
    }

    #[test]
    fn test_swing_to_unified_scrollpane() {
        assert_eq!(normalize_for_swing("ScrollPane"), "JScrollPane");
    }

    #[test]
    fn test_swing_to_unified_splitpane() {
        assert_eq!(normalize_for_swing("SplitPane"), "JSplitPane");
    }

    #[test]
    fn test_swing_to_unified_tabbedpane() {
        assert_eq!(normalize_for_swing("TabbedPane"), "JTabbedPane");
    }

    #[test]
    fn test_swing_to_unified_menubar() {
        assert_eq!(normalize_for_swing("MenuBar"), "JMenuBar");
    }

    #[test]
    fn test_swing_to_unified_menu() {
        assert_eq!(normalize_for_swing("Menu"), "JMenu");
    }

    #[test]
    fn test_swing_to_unified_menuitem() {
        assert_eq!(normalize_for_swing("MenuItem"), "JMenuItem");
    }

    #[test]
    fn test_swing_to_unified_toolbar() {
        assert_eq!(normalize_for_swing("ToolBar"), "JToolBar");
    }

    #[test]
    fn test_swing_preserves_jprefix() {
        assert_eq!(normalize_for_swing("JButton"), "JButton");
        assert_eq!(normalize_for_swing("JTextField"), "JTextField");
    }

    #[test]
    fn test_swing_preserves_custom_widgets() {
        assert_eq!(normalize_for_swing("CustomWidget"), "CustomWidget");
        assert_eq!(normalize_for_swing("MyButton"), "MyButton");
    }

    #[test]
    fn test_swing_preserves_fqn() {
        // Fully qualified names should not be modified
        let locator = UnifiedLocator::class("javax.swing.JButton");
        let normalized = locator.normalize_for_toolkit(ToolkitType::Swing);
        assert_eq!(normalized.value, "javax.swing.JButton");
    }
}

#[cfg(test)]
mod swt_normalization_tests {
    use crate::locator::unified::UnifiedLocator;
    use crate::core::backend::ToolkitType;

    fn normalize_for_swt(value: &str) -> String {
        let locator = UnifiedLocator::class(value);
        let normalized = locator.normalize_for_toolkit(ToolkitType::Swt);
        normalized.value
    }

    #[test]
    fn test_swt_from_jbutton() {
        assert_eq!(normalize_for_swt("JButton"), "Button");
    }

    #[test]
    fn test_swt_from_jtextfield() {
        assert_eq!(normalize_for_swt("JTextField"), "Text");
    }

    #[test]
    fn test_swt_from_jtextarea() {
        assert_eq!(normalize_for_swt("JTextArea"), "Text");
    }

    #[test]
    fn test_swt_from_jlabel() {
        assert_eq!(normalize_for_swt("JLabel"), "Label");
    }

    #[test]
    fn test_swt_from_jcombobox() {
        assert_eq!(normalize_for_swt("JComboBox"), "Combo");
    }

    #[test]
    fn test_swt_from_jlist() {
        assert_eq!(normalize_for_swt("JList"), "List");
    }

    #[test]
    fn test_swt_from_jtable() {
        assert_eq!(normalize_for_swt("JTable"), "Table");
    }

    #[test]
    fn test_swt_from_jtree() {
        assert_eq!(normalize_for_swt("JTree"), "Tree");
    }

    #[test]
    fn test_swt_from_jpanel() {
        assert_eq!(normalize_for_swt("JPanel"), "Composite");
    }

    #[test]
    fn test_swt_from_jframe() {
        assert_eq!(normalize_for_swt("JFrame"), "Shell");
    }

    #[test]
    fn test_swt_from_jdialog() {
        assert_eq!(normalize_for_swt("JDialog"), "Shell");
    }

    #[test]
    fn test_swt_from_jtabbedpane() {
        assert_eq!(normalize_for_swt("JTabbedPane"), "TabFolder");
    }

    #[test]
    fn test_swt_from_jsplitpane() {
        assert_eq!(normalize_for_swt("JSplitPane"), "SashForm");
    }

    #[test]
    fn test_swt_preserves_native() {
        // Native SWT names should not be changed
        assert_eq!(normalize_for_swt("Button"), "Button");
        assert_eq!(normalize_for_swt("Text"), "Text");
        assert_eq!(normalize_for_swt("Composite"), "Composite");
    }

    #[test]
    fn test_swt_preserves_custom() {
        assert_eq!(normalize_for_swt("CustomWidget"), "CustomWidget");
    }
}

#[cfg(test)]
mod locator_predicate_tests {
    use crate::locator::unified::{UnifiedLocator, LocatorPredicate, MatchOp};

    #[test]
    fn test_locator_with_predicate() {
        let locator = UnifiedLocator::class("JButton")
            .with_predicate(LocatorPredicate::Attribute {
                name: "text".to_string(),
                op: MatchOp::Equals,
                value: "OK".to_string(),
            });

        assert_eq!(locator.predicates.len(), 1);
    }

    #[test]
    fn test_locator_with_attribute() {
        let locator = UnifiedLocator::class("JButton")
            .with_attribute("text", MatchOp::Equals, "OK");

        if let LocatorPredicate::Attribute { name, op, value } = &locator.predicates[0] {
            assert_eq!(name, "text");
            assert_eq!(*op, MatchOp::Equals);
            assert_eq!(value, "OK");
        }
    }

    #[test]
    fn test_locator_with_pseudo_class() {
        let locator = UnifiedLocator::class("JButton")
            .with_pseudo_class("visible");

        if let LocatorPredicate::PseudoClass(pseudo) = &locator.predicates[0] {
            assert_eq!(pseudo, "visible");
        }
    }

    #[test]
    fn test_locator_with_multiple_predicates() {
        let locator = UnifiedLocator::class("JButton")
            .with_attribute("name", MatchOp::Equals, "btn")
            .with_attribute("text", MatchOp::Contains, "Save")
            .with_pseudo_class("enabled")
            .with_pseudo_class("visible");

        assert_eq!(locator.predicates.len(), 4);
    }

    #[test]
    fn test_match_op_display() {
        assert_eq!(format!("{}", MatchOp::Equals), "=");
        assert_eq!(format!("{}", MatchOp::Contains), "*=");
        assert_eq!(format!("{}", MatchOp::StartsWith), "^=");
        assert_eq!(format!("{}", MatchOp::EndsWith), "$=");
        assert_eq!(format!("{}", MatchOp::Regex), "~=");
    }
}

#[cfg(test)]
mod locator_parse_error_tests {
    use crate::locator::unified::LocatorParseError;

    #[test]
    fn test_error_new() {
        let err = LocatorParseError::new("Invalid locator");
        assert_eq!(err.message, "Invalid locator");
        assert_eq!(err.position, None);
    }

    #[test]
    fn test_error_at_position() {
        let err = LocatorParseError::at_position("Unexpected character", 5);
        assert_eq!(err.message, "Unexpected character");
        assert_eq!(err.position, Some(5));
    }

    #[test]
    fn test_error_display_without_position() {
        let err = LocatorParseError::new("Invalid syntax");
        let msg = format!("{}", err);
        assert_eq!(msg, "Invalid syntax");
    }

    #[test]
    fn test_error_display_with_position() {
        let err = LocatorParseError::at_position("Unexpected bracket", 10);
        let msg = format!("{}", err);
        assert!(msg.contains("10"));
        assert!(msg.contains("Unexpected bracket"));
    }
}

#[cfg(test)]
mod locator_constructors_tests {
    use crate::locator::unified::{UnifiedLocator, LocatorType};

    #[test]
    fn test_name_constructor() {
        let locator = UnifiedLocator::name("myButton");
        assert_eq!(locator.locator_type, LocatorType::Name);
        assert_eq!(locator.value, "myButton");
        assert_eq!(locator.original, "name:myButton");
        assert!(locator.predicates.is_empty());
    }

    #[test]
    fn test_text_constructor() {
        let locator = UnifiedLocator::text("Click Me");
        assert_eq!(locator.locator_type, LocatorType::Text);
        assert_eq!(locator.value, "Click Me");
        assert_eq!(locator.original, "text:Click Me");
    }

    #[test]
    fn test_class_constructor() {
        let locator = UnifiedLocator::class("JButton");
        assert_eq!(locator.locator_type, LocatorType::Class);
        assert_eq!(locator.value, "JButton");
        assert_eq!(locator.original, "JButton");
    }

    #[test]
    fn test_index_constructor() {
        let locator = UnifiedLocator::index(3);
        assert_eq!(locator.locator_type, LocatorType::Index);
        assert_eq!(locator.value, "3");
        assert_eq!(locator.original, "index:3");
    }

    #[test]
    fn test_id_constructor() {
        let locator = UnifiedLocator::id("67890");
        assert_eq!(locator.locator_type, LocatorType::Id);
        assert_eq!(locator.value, "67890");
        assert_eq!(locator.original, "id:67890");
    }

    #[test]
    fn test_xpath_constructor() {
        let locator = UnifiedLocator::xpath("//JButton[@text='OK']");
        assert_eq!(locator.locator_type, LocatorType::XPath);
        assert_eq!(locator.value, "//JButton[@text='OK']");
        assert_eq!(locator.original, "//JButton[@text='OK']");
    }

    #[test]
    fn test_toolkit_constructor() {
        let locator = UnifiedLocator::toolkit("swt", "Button[text='Save']");
        if let LocatorType::Toolkit { toolkit, selector } = &locator.locator_type {
            assert_eq!(toolkit, "swt");
            assert_eq!(selector, "Button[text='Save']");
        }
        assert_eq!(locator.original, "swt:Button[text='Save']");
    }
}

#[cfg(test)]
mod normalized_locator_tests {
    use crate::locator::unified::UnifiedLocator;
    use crate::core::backend::ToolkitType;

    #[test]
    fn test_normalize_name_locator() {
        let locator = UnifiedLocator::name("test");
        let normalized = locator.normalize_for_toolkit(ToolkitType::Swing);

        // Name locators should not change value
        assert_eq!(normalized.value, "test");
    }

    #[test]
    fn test_normalize_text_locator() {
        let locator = UnifiedLocator::text("Click");
        let normalized = locator.normalize_for_toolkit(ToolkitType::Swt);

        // Text locators should not change value
        assert_eq!(normalized.value, "Click");
    }

    #[test]
    fn test_normalize_id_locator() {
        let locator = UnifiedLocator::id("12345");
        let normalized = locator.normalize_for_toolkit(ToolkitType::Swing);

        // ID locators should not change value
        assert_eq!(normalized.value, "12345");
    }

    #[test]
    fn test_normalize_preserves_predicates() {
        let locator = UnifiedLocator::parse("Button[text='Save']:visible").unwrap();
        let normalized = locator.normalize_for_toolkit(ToolkitType::Swing);

        assert_eq!(normalized.predicates.len(), 2);
    }
}
