//! Comprehensive unit tests for the core module
//!
//! Tests cover:
//! - LibraryConfig creation and builder patterns
//! - ConnectionConfig for different toolkits
//! - GuiMode/ToolkitType conversions
//! - JavaGuiElement creation and type mapping
//! - Backend trait and factory patterns
//! - ElementCondition descriptions
//! - BackendError handling

#[cfg(test)]
mod library_config_tests {
    use crate::core::config::{LibraryConfig, LogLevel};
    use std::path::PathBuf;
    use std::time::Duration;

    #[test]
    fn test_library_config_defaults() {
        let config = LibraryConfig::default();
        assert_eq!(config.timeout, Duration::from_secs(10));
        assert_eq!(config.poll_interval, Duration::from_millis(500));
        assert!(config.screenshot_on_failure);
        assert_eq!(config.screenshot_directory, PathBuf::from("."));
        assert_eq!(config.screenshot_format, "png");
        assert_eq!(config.log_level, LogLevel::Info);
        assert!(config.enable_element_cache);
        assert_eq!(config.cache_ttl, Duration::from_secs(5));
        assert!(config.log_actions);
    }

    #[test]
    fn test_library_config_new_equals_default() {
        let config1 = LibraryConfig::new();
        let config2 = LibraryConfig::default();
        assert_eq!(config1.timeout, config2.timeout);
        assert_eq!(config1.poll_interval, config2.poll_interval);
        assert_eq!(config1.screenshot_on_failure, config2.screenshot_on_failure);
    }

    #[test]
    fn test_library_config_custom_values() {
        let config = LibraryConfig::new()
            .with_timeout_secs(30.0)
            .with_poll_interval_secs(1.0)
            .with_screenshot_on_failure(false)
            .with_log_level(LogLevel::Debug)
            .with_element_cache(false)
            .with_cache_ttl(Duration::from_secs(60));

        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.poll_interval, Duration::from_secs(1));
        assert!(!config.screenshot_on_failure);
        assert_eq!(config.log_level, LogLevel::Debug);
        assert!(!config.enable_element_cache);
        assert_eq!(config.cache_ttl, Duration::from_secs(60));
    }

    #[test]
    fn test_library_config_with_timeout() {
        let config = LibraryConfig::new().with_timeout(Duration::from_secs(45));
        assert_eq!(config.timeout, Duration::from_secs(45));
    }

    #[test]
    fn test_library_config_with_poll_interval() {
        let config = LibraryConfig::new().with_poll_interval(Duration::from_millis(250));
        assert_eq!(config.poll_interval, Duration::from_millis(250));
    }

    #[test]
    fn test_library_config_with_screenshot_directory() {
        let config = LibraryConfig::new().with_screenshot_directory("/tmp/screenshots");
        assert_eq!(config.screenshot_directory, PathBuf::from("/tmp/screenshots"));
    }

    #[test]
    fn test_library_config_builder_chain() {
        // Test that builder methods can be chained in any order
        let config = LibraryConfig::new()
            .with_log_level(LogLevel::Warning)
            .with_timeout_secs(15.0)
            .with_element_cache(true)
            .with_screenshot_on_failure(true)
            .with_poll_interval_secs(0.25);

        assert_eq!(config.log_level, LogLevel::Warning);
        assert_eq!(config.timeout, Duration::from_secs(15));
        assert!(config.enable_element_cache);
        assert!(config.screenshot_on_failure);
        assert_eq!(config.poll_interval, Duration::from_millis(250));
    }
}

#[cfg(test)]
mod log_level_tests {
    use crate::core::config::LogLevel;

    #[test]
    fn test_log_level_from_str_valid() {
        assert_eq!(LogLevel::from_str("debug"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("info"), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str("warning"), Some(LogLevel::Warning));
        assert_eq!(LogLevel::from_str("warn"), Some(LogLevel::Warning));
        assert_eq!(LogLevel::from_str("error"), Some(LogLevel::Error));
    }

    #[test]
    fn test_log_level_from_str_case_insensitive() {
        assert_eq!(LogLevel::from_str("DEBUG"), Some(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("Info"), Some(LogLevel::Info));
        assert_eq!(LogLevel::from_str("WARNING"), Some(LogLevel::Warning));
        assert_eq!(LogLevel::from_str("WARN"), Some(LogLevel::Warning));
        assert_eq!(LogLevel::from_str("ERROR"), Some(LogLevel::Error));
    }

    #[test]
    fn test_log_level_from_str_invalid() {
        assert_eq!(LogLevel::from_str("unknown"), None);
        assert_eq!(LogLevel::from_str("trace"), None);
        assert_eq!(LogLevel::from_str("fatal"), None);
        assert_eq!(LogLevel::from_str(""), None);
    }

    #[test]
    fn test_log_level_as_str() {
        assert_eq!(LogLevel::Debug.as_str(), "debug");
        assert_eq!(LogLevel::Info.as_str(), "info");
        assert_eq!(LogLevel::Warning.as_str(), "warning");
        assert_eq!(LogLevel::Error.as_str(), "error");
    }

    #[test]
    fn test_log_level_display_roundtrip() {
        for level in [LogLevel::Debug, LogLevel::Info, LogLevel::Warning, LogLevel::Error] {
            let s = level.as_str();
            let parsed = LogLevel::from_str(s);
            assert_eq!(parsed, Some(level));
        }
    }

    #[test]
    fn test_log_level_default() {
        let level = LogLevel::default();
        assert_eq!(level, LogLevel::Info);
    }
}

#[cfg(test)]
mod connection_config_tests {
    use crate::core::config::ConnectionConfig;
    use crate::core::backend::ToolkitType;
    use std::time::Duration;

    #[test]
    fn test_connection_config_defaults() {
        let config = ConnectionConfig::default();
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5678);
        assert_eq!(config.timeout, Duration::from_secs(30));
        assert_eq!(config.application, "");
        assert_eq!(config.toolkit, ToolkitType::Swing);
    }

    #[test]
    fn test_connection_config_swing() {
        let config = ConnectionConfig::swing("myapp.jar");
        assert_eq!(config.application, "myapp.jar");
        assert_eq!(config.port, 5678);
        assert_eq!(config.toolkit, ToolkitType::Swing);
        assert_eq!(config.host, "localhost");
    }

    #[test]
    fn test_connection_config_swt() {
        let config = ConnectionConfig::swt("com.example.SwtApp");
        assert_eq!(config.application, "com.example.SwtApp");
        assert_eq!(config.port, 5679);
        assert_eq!(config.toolkit, ToolkitType::Swt);
    }

    #[test]
    fn test_connection_config_rcp() {
        let config = ConnectionConfig::rcp("eclipse");
        assert_eq!(config.application, "eclipse");
        assert_eq!(config.port, 5679);
        assert_eq!(config.toolkit, ToolkitType::Rcp);
    }

    #[test]
    fn test_connection_config_builder() {
        let config = ConnectionConfig::new()
            .with_host("192.168.1.100")
            .with_port(9999)
            .with_timeout_secs(60.0)
            .with_application("test-app");

        assert_eq!(config.host, "192.168.1.100");
        assert_eq!(config.port, 9999);
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.application, "test-app");
    }

    #[test]
    fn test_connection_config_with_toolkit() {
        let config = ConnectionConfig::new()
            .with_toolkit(ToolkitType::Swt);

        assert_eq!(config.toolkit, ToolkitType::Swt);
        // Port should update to SWT default
        assert_eq!(config.port, 5679);
    }

    #[test]
    fn test_connection_config_socket_addr() {
        let config = ConnectionConfig::new()
            .with_host("127.0.0.1")
            .with_port(8080);

        assert_eq!(config.socket_addr(), "127.0.0.1:8080");
    }

    #[test]
    fn test_connection_config_socket_addr_with_hostname() {
        let config = ConnectionConfig::new()
            .with_host("myserver.example.com")
            .with_port(5678);

        assert_eq!(config.socket_addr(), "myserver.example.com:5678");
    }
}

#[cfg(test)]
mod toolkit_type_tests {
    use crate::core::backend::ToolkitType;

    #[test]
    fn test_toolkit_type_from_str() {
        assert_eq!(ToolkitType::from_str("swing"), Some(ToolkitType::Swing));
        assert_eq!(ToolkitType::from_str("swt"), Some(ToolkitType::Swt));
        assert_eq!(ToolkitType::from_str("rcp"), Some(ToolkitType::Rcp));
    }

    #[test]
    fn test_toolkit_type_from_str_case_insensitive() {
        assert_eq!(ToolkitType::from_str("SWING"), Some(ToolkitType::Swing));
        assert_eq!(ToolkitType::from_str("SWT"), Some(ToolkitType::Swt));
        assert_eq!(ToolkitType::from_str("RCP"), Some(ToolkitType::Rcp));
        assert_eq!(ToolkitType::from_str("Swing"), Some(ToolkitType::Swing));
    }

    #[test]
    fn test_toolkit_type_from_str_invalid() {
        assert_eq!(ToolkitType::from_str("unknown"), None);
        assert_eq!(ToolkitType::from_str("awt"), None);
        assert_eq!(ToolkitType::from_str("javafx"), None);
        assert_eq!(ToolkitType::from_str(""), None);
    }

    #[test]
    fn test_toolkit_type_display() {
        assert_eq!(format!("{}", ToolkitType::Swing), "swing");
        assert_eq!(format!("{}", ToolkitType::Swt), "swt");
        assert_eq!(format!("{}", ToolkitType::Rcp), "rcp");
    }

    #[test]
    fn test_toolkit_type_name() {
        assert_eq!(ToolkitType::Swing.name(), "swing");
        assert_eq!(ToolkitType::Swt.name(), "swt");
        assert_eq!(ToolkitType::Rcp.name(), "rcp");
    }

    #[test]
    fn test_toolkit_type_default_port() {
        assert_eq!(ToolkitType::Swing.default_port(), 5678);
        assert_eq!(ToolkitType::Swt.default_port(), 5679);
        assert_eq!(ToolkitType::Rcp.default_port(), 5679);
    }

    #[test]
    fn test_toolkit_type_equality() {
        assert_eq!(ToolkitType::Swing, ToolkitType::Swing);
        assert_ne!(ToolkitType::Swing, ToolkitType::Swt);
        assert_ne!(ToolkitType::Swt, ToolkitType::Rcp);
    }

    #[test]
    fn test_toolkit_type_clone() {
        let original = ToolkitType::Swing;
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_toolkit_type_copy() {
        let original = ToolkitType::Swt;
        let copied: ToolkitType = original;
        assert_eq!(original, copied);
    }
}

#[cfg(test)]
mod element_type_tests {
    use crate::core::element::ElementType;
    use crate::core::backend::ToolkitType;

    #[test]
    fn test_element_type_from_swing_class() {
        assert_eq!(
            ElementType::from_class_name("javax.swing.JButton", ToolkitType::Swing),
            ElementType::Button
        );
        assert_eq!(
            ElementType::from_class_name("JTextField", ToolkitType::Swing),
            ElementType::TextField
        );
        assert_eq!(
            ElementType::from_class_name("JTable", ToolkitType::Swing),
            ElementType::Table
        );
        assert_eq!(
            ElementType::from_class_name("JTree", ToolkitType::Swing),
            ElementType::Tree
        );
    }

    #[test]
    fn test_element_type_from_swing_class_all_types() {
        let swing_mappings = [
            ("JButton", ElementType::Button),
            ("JToggleButton", ElementType::ToggleButton),
            ("JCheckBox", ElementType::CheckBox),
            ("JRadioButton", ElementType::RadioButton),
            ("JTextField", ElementType::TextField),
            ("JFormattedTextField", ElementType::TextField),
            ("JTextArea", ElementType::TextArea),
            ("JEditorPane", ElementType::TextArea),
            ("JTextPane", ElementType::TextArea),
            ("JPasswordField", ElementType::PasswordField),
            ("JSpinner", ElementType::Spinner),
            ("JComboBox", ElementType::ComboBox),
            ("JList", ElementType::List),
            ("JTable", ElementType::Table),
            ("JTree", ElementType::Tree),
            ("JLabel", ElementType::Label),
            ("JProgressBar", ElementType::ProgressBar),
            ("JSlider", ElementType::Slider),
            ("JPanel", ElementType::Panel),
            ("JFrame", ElementType::Frame),
            ("JDialog", ElementType::Dialog),
            ("JScrollPane", ElementType::ScrollPane),
            ("JSplitPane", ElementType::SplitPane),
            ("JTabbedPane", ElementType::TabbedPane),
            ("JMenuBar", ElementType::MenuBar),
            ("JMenu", ElementType::Menu),
            ("JMenuItem", ElementType::MenuItem),
            ("JCheckBoxMenuItem", ElementType::MenuItem),
            ("JRadioButtonMenuItem", ElementType::MenuItem),
            ("JPopupMenu", ElementType::PopupMenu),
            ("JToolBar", ElementType::ToolBar),
        ];

        for (class_name, expected_type) in swing_mappings {
            assert_eq!(
                ElementType::from_class_name(class_name, ToolkitType::Swing),
                expected_type,
                "Failed for class: {}",
                class_name
            );
        }
    }

    #[test]
    fn test_element_type_from_swt_class() {
        assert_eq!(
            ElementType::from_class_name("org.eclipse.swt.widgets.Button", ToolkitType::Swt),
            ElementType::Button
        );
        assert_eq!(
            ElementType::from_class_name("Text", ToolkitType::Swt),
            ElementType::TextField
        );
        assert_eq!(
            ElementType::from_class_name("Table", ToolkitType::Swt),
            ElementType::Table
        );
    }

    #[test]
    fn test_element_type_from_swt_class_all_types() {
        let swt_mappings = [
            ("Button", ElementType::Button),
            ("Text", ElementType::TextField),
            ("StyledText", ElementType::TextArea),
            ("Spinner", ElementType::Spinner),
            ("Combo", ElementType::ComboBox),
            ("CCombo", ElementType::ComboBox),
            ("List", ElementType::List),
            ("Table", ElementType::Table),
            ("Tree", ElementType::Tree),
            ("Label", ElementType::Label),
            ("CLabel", ElementType::Label),
            ("ProgressBar", ElementType::ProgressBar),
            ("Scale", ElementType::Slider),
            ("Slider", ElementType::Slider),
            ("Composite", ElementType::Panel),
            ("ScrolledComposite", ElementType::Panel),
            ("Group", ElementType::Group),
            ("Shell", ElementType::Shell),
            ("TabFolder", ElementType::TabbedPane),
            ("CTabFolder", ElementType::TabbedPane),
            ("SashForm", ElementType::SplitPane),
            ("Menu", ElementType::Menu),
            ("MenuItem", ElementType::MenuItem),
            ("ToolBar", ElementType::ToolBar),
            ("ToolItem", ElementType::ToolItem),
            ("ViewPart", ElementType::View),
            ("ViewSite", ElementType::View),
            ("EditorPart", ElementType::Editor),
            ("EditorSite", ElementType::Editor),
        ];

        for (class_name, expected_type) in swt_mappings {
            assert_eq!(
                ElementType::from_class_name(class_name, ToolkitType::Swt),
                expected_type,
                "Failed for class: {}",
                class_name
            );
        }
    }

    #[test]
    fn test_element_type_from_rcp_class() {
        assert_eq!(
            ElementType::from_class_name("ViewPart", ToolkitType::Rcp),
            ElementType::View
        );
        assert_eq!(
            ElementType::from_class_name("EditorPart", ToolkitType::Rcp),
            ElementType::Editor
        );
    }

    #[test]
    fn test_element_type_unknown() {
        assert_eq!(
            ElementType::from_class_name("CustomWidget", ToolkitType::Swing),
            ElementType::Widget
        );
        assert_eq!(
            ElementType::from_class_name("MyCustomComponent", ToolkitType::Swt),
            ElementType::Widget
        );
    }

    #[test]
    fn test_element_type_name() {
        assert_eq!(ElementType::Button.name(), "Button");
        assert_eq!(ElementType::TextField.name(), "TextField");
        assert_eq!(ElementType::Table.name(), "Table");
        assert_eq!(ElementType::Unknown.name(), "Unknown");
    }

    #[test]
    fn test_element_type_is_text_input() {
        assert!(ElementType::TextField.is_text_input());
        assert!(ElementType::TextArea.is_text_input());
        assert!(ElementType::PasswordField.is_text_input());
        assert!(ElementType::Spinner.is_text_input());
        assert!(ElementType::ComboBox.is_text_input());

        assert!(!ElementType::Button.is_text_input());
        assert!(!ElementType::Label.is_text_input());
        assert!(!ElementType::Table.is_text_input());
    }

    #[test]
    fn test_element_type_is_container() {
        assert!(ElementType::Panel.is_container());
        assert!(ElementType::Frame.is_container());
        assert!(ElementType::Dialog.is_container());
        assert!(ElementType::Shell.is_container());
        assert!(ElementType::Group.is_container());
        assert!(ElementType::ScrollPane.is_container());
        assert!(ElementType::SplitPane.is_container());
        assert!(ElementType::TabbedPane.is_container());

        assert!(!ElementType::Button.is_container());
        assert!(!ElementType::Label.is_container());
    }

    #[test]
    fn test_element_type_is_clickable() {
        assert!(ElementType::Button.is_clickable());
        assert!(ElementType::ToggleButton.is_clickable());
        assert!(ElementType::CheckBox.is_clickable());
        assert!(ElementType::RadioButton.is_clickable());
        assert!(ElementType::MenuItem.is_clickable());
        assert!(ElementType::ToolItem.is_clickable());

        assert!(!ElementType::Label.is_clickable());
        assert!(!ElementType::TextField.is_clickable());
        assert!(!ElementType::Panel.is_clickable());
    }

    #[test]
    fn test_element_type_display() {
        assert_eq!(format!("{}", ElementType::Button), "Button");
        assert_eq!(format!("{}", ElementType::TextField), "TextField");
    }
}

#[cfg(test)]
mod java_gui_element_tests {
    use crate::core::element::JavaGuiElement;
    use crate::core::backend::ToolkitType;
    use serde_json::json;

    #[test]
    fn test_element_creation() {
        let elem = JavaGuiElement::new(12345, "javax.swing.JButton", "swing");
        assert_eq!(elem.hash_code, 12345);
        assert_eq!(elem.class_name, "javax.swing.JButton");
        assert_eq!(elem.simple_name, "JButton");
        assert_eq!(elem.toolkit, "swing");
        assert_eq!(elem.element_type, "Button");
    }

    #[test]
    fn test_element_creation_swt() {
        let elem = JavaGuiElement::new(67890, "org.eclipse.swt.widgets.Button", "swt");
        assert_eq!(elem.hash_code, 67890);
        assert_eq!(elem.simple_name, "Button");
        assert_eq!(elem.toolkit, "swt");
        assert_eq!(elem.element_type, "Button");
    }

    #[test]
    fn test_element_with_builder_pattern() {
        let elem = JavaGuiElement::new(123, "JButton", "swing")
            .with_name("testButton")
            .with_text("Click Me")
            .with_bounds(10, 20, 100, 50)
            .with_visible(true)
            .with_enabled(true);

        assert_eq!(elem.name, Some("testButton".to_string()));
        assert_eq!(elem.text, Some("Click Me".to_string()));
        assert_eq!(elem.x, 10);
        assert_eq!(elem.y, 20);
        assert_eq!(elem.width, 100);
        assert_eq!(elem.height, 50);
        assert!(elem.visible);
        assert!(elem.enabled);
    }

    #[test]
    fn test_element_best_identifier_with_name() {
        let elem = JavaGuiElement::new(123, "JButton", "swing")
            .with_name("okButton");
        assert_eq!(elem.best_identifier(), "okButton");
    }

    #[test]
    fn test_element_best_identifier_with_text() {
        let elem = JavaGuiElement::new(456, "JButton", "swing")
            .with_text("Submit");
        assert_eq!(elem.best_identifier(), "Submit");
    }

    #[test]
    fn test_element_best_identifier_fallback() {
        let elem = JavaGuiElement::new(789, "JButton", "swing");
        assert_eq!(elem.best_identifier(), "JButton@789");
    }

    #[test]
    fn test_element_get_bounds() {
        let elem = JavaGuiElement::new(123, "JButton", "swing")
            .with_bounds(10, 20, 100, 50);
        assert_eq!(elem.get_bounds(), (10, 20, 100, 50));
    }

    #[test]
    fn test_element_get_center() {
        let elem = JavaGuiElement::new(123, "JButton", "swing")
            .with_bounds(10, 20, 100, 50);
        // Center should be (10 + 100/2, 20 + 50/2) = (60, 45)
        assert_eq!(elem.get_center(), (60, 45));
    }

    #[test]
    fn test_element_from_json_basic() {
        let json = json!({
            "hashCode": 12345,
            "className": "javax.swing.JButton",
            "name": "okButton",
            "text": "OK",
            "x": 10,
            "y": 20,
            "width": 100,
            "height": 30,
            "visible": true,
            "enabled": true,
        });

        let elem = JavaGuiElement::from_json(&json, ToolkitType::Swing).unwrap();
        assert_eq!(elem.hash_code, 12345);
        assert_eq!(elem.simple_name, "JButton");
        assert_eq!(elem.name, Some("okButton".to_string()));
        assert_eq!(elem.text, Some("OK".to_string()));
        assert_eq!(elem.element_type, "Button");
        assert_eq!(elem.x, 10);
        assert_eq!(elem.y, 20);
    }

    #[test]
    fn test_element_from_json_alternate_keys() {
        let json = json!({
            "id": 12345,
            "class": "javax.swing.JLabel",
            "simpleClass": "JLabel",
        });

        let elem = JavaGuiElement::from_json(&json, ToolkitType::Swing).unwrap();
        assert_eq!(elem.hash_code, 12345);
        assert_eq!(elem.simple_name, "JLabel");
    }

    #[test]
    fn test_element_from_json_missing_required() {
        let json = json!({
            "name": "test"
        });

        let elem = JavaGuiElement::from_json(&json, ToolkitType::Swing);
        assert!(elem.is_none());
    }

    #[test]
    fn test_element_to_json() {
        let elem = JavaGuiElement::new(123, "JButton", "swing")
            .with_name("btn")
            .with_text("Click")
            .with_bounds(5, 10, 80, 25);

        let json = elem.to_json();
        assert_eq!(json["hashCode"], 123);
        assert_eq!(json["className"], "JButton");
        assert_eq!(json["name"], "btn");
        assert_eq!(json["text"], "Click");
        assert_eq!(json["x"], 5);
        assert_eq!(json["y"], 10);
        assert_eq!(json["width"], 80);
        assert_eq!(json["height"], 25);
    }

    #[test]
    fn test_element_json_roundtrip() {
        let original = JavaGuiElement::new(999, "javax.swing.JTextField", "swing")
            .with_name("inputField")
            .with_text("Hello")
            .with_bounds(0, 0, 200, 25);

        let json = original.to_json();
        let restored = JavaGuiElement::from_json(&json, ToolkitType::Swing).unwrap();

        assert_eq!(original.hash_code, restored.hash_code);
        assert_eq!(original.name, restored.name);
        assert_eq!(original.text, restored.text);
    }

    #[test]
    fn test_element_properties() {
        let mut elem = JavaGuiElement::new(123, "JButton", "swing");

        elem.set_property("custom", "value");
        assert!(elem.has_property("custom"));
        assert!(!elem.has_property("nonexistent"));

        let names = elem.property_names();
        assert!(names.contains(&"custom".to_string()));
    }

    #[test]
    fn test_element_equality() {
        let elem1 = JavaGuiElement::new(123, "JButton", "swing");
        let elem2 = JavaGuiElement::new(123, "JButton", "swing");
        let elem3 = JavaGuiElement::new(456, "JButton", "swing");
        let elem4 = JavaGuiElement::new(123, "JButton", "swt");

        // Same hash_code and toolkit should be equal
        assert_eq!(elem1, elem2);

        // Different hash_code should not be equal
        assert_ne!(elem1, elem3);

        // Different toolkit should not be equal
        assert_ne!(elem1, elem4);
    }

    #[test]
    fn test_element_is_container() {
        let panel = JavaGuiElement::new(1, "JPanel", "swing");
        let button = JavaGuiElement::new(2, "JButton", "swing");

        assert!(panel.is_container());
        assert!(!button.is_container());
    }

    #[test]
    fn test_element_is_text_input() {
        let text_field = JavaGuiElement::new(1, "JTextField", "swing");
        let button = JavaGuiElement::new(2, "JButton", "swing");

        assert!(text_field.is_text_input());
        assert!(!button.is_text_input());
    }

    #[test]
    fn test_element_is_clickable() {
        let button = JavaGuiElement::new(1, "JButton", "swing");
        let label = JavaGuiElement::new(2, "JLabel", "swing");

        assert!(button.is_clickable());
        assert!(!label.is_clickable());
    }
}

#[cfg(test)]
mod backend_tests {
    use crate::core::backend::{
        Backend, BackendError, BackendFactory, GenericBackend, ToolkitType,
        ElementCondition,
    };

    #[test]
    fn test_generic_backend_creation() {
        let backend = GenericBackend::new(ToolkitType::Swing);
        assert_eq!(backend.toolkit_type(), ToolkitType::Swing);
        assert!(!backend.is_connected());
    }

    #[test]
    fn test_generic_backend_default_port() {
        let swing_backend = GenericBackend::new(ToolkitType::Swing);
        let swt_backend = GenericBackend::new(ToolkitType::Swt);
        let rcp_backend = GenericBackend::new(ToolkitType::Rcp);

        assert_eq!(swing_backend.default_port(), 5678);
        assert_eq!(swt_backend.default_port(), 5679);
        assert_eq!(rcp_backend.default_port(), 5679);
    }

    #[test]
    fn test_backend_factory_swing() {
        let backend = BackendFactory::create(ToolkitType::Swing);
        assert_eq!(backend.toolkit_type(), ToolkitType::Swing);
        assert!(!backend.is_connected());
    }

    #[test]
    fn test_backend_factory_swt() {
        let backend = BackendFactory::create(ToolkitType::Swt);
        assert_eq!(backend.toolkit_type(), ToolkitType::Swt);
    }

    #[test]
    fn test_backend_factory_rcp() {
        let backend = BackendFactory::create(ToolkitType::Rcp);
        assert_eq!(backend.toolkit_type(), ToolkitType::Rcp);
    }

    #[test]
    fn test_backend_connection_info_when_not_connected() {
        let backend = GenericBackend::new(ToolkitType::Swing);
        assert!(backend.connection_info().is_none());
    }

    #[test]
    fn test_backend_disconnect_when_not_connected() {
        let mut backend = GenericBackend::new(ToolkitType::Swing);
        // Should not error when disconnecting while not connected
        let result = backend.disconnect();
        assert!(result.is_ok());
    }

    #[test]
    fn test_element_condition_exists() {
        let condition = ElementCondition::Exists;
        assert_eq!(condition.description(), "exists");
    }

    #[test]
    fn test_element_condition_not_exists() {
        let condition = ElementCondition::NotExists;
        assert_eq!(condition.description(), "does not exist");
    }

    #[test]
    fn test_element_condition_visible() {
        let condition = ElementCondition::Visible;
        assert_eq!(condition.description(), "is visible");
    }

    #[test]
    fn test_element_condition_not_visible() {
        let condition = ElementCondition::NotVisible;
        assert_eq!(condition.description(), "is not visible");
    }

    #[test]
    fn test_element_condition_enabled() {
        let condition = ElementCondition::Enabled;
        assert_eq!(condition.description(), "is enabled");
    }

    #[test]
    fn test_element_condition_disabled() {
        let condition = ElementCondition::Disabled;
        assert_eq!(condition.description(), "is disabled");
    }

    #[test]
    fn test_element_condition_has_text() {
        let condition = ElementCondition::HasText("hello".to_string());
        assert_eq!(condition.description(), "has text 'hello'");
    }

    #[test]
    fn test_element_condition_text_contains() {
        let condition = ElementCondition::TextContains("world".to_string());
        assert_eq!(condition.description(), "text contains 'world'");
    }

    #[test]
    fn test_element_condition_focused() {
        let condition = ElementCondition::Focused;
        assert_eq!(condition.description(), "is focused");
    }

    #[test]
    fn test_element_condition_custom() {
        let condition = ElementCondition::custom("my custom condition");
        assert_eq!(condition.description(), "my custom condition");
    }
}

#[cfg(test)]
mod backend_error_tests {
    use crate::core::backend::BackendError;

    #[test]
    fn test_backend_error_connection() {
        let err = BackendError::connection("Failed to connect");
        assert!(matches!(err, BackendError::Connection { .. }));
        assert!(err.is_connection_error());
        assert!(!err.is_recoverable());
    }

    #[test]
    fn test_backend_error_not_connected() {
        let err = BackendError::NotConnected;
        assert!(err.is_connection_error());
        assert!(!err.is_recoverable());
    }

    #[test]
    fn test_backend_error_timeout() {
        let err = BackendError::Timeout { timeout_ms: 5000 };
        assert!(err.is_connection_error());
        assert!(err.is_recoverable());
    }

    #[test]
    fn test_backend_error_element_not_found() {
        let err = BackendError::ElementNotFound {
            locator: "name:myButton".to_string(),
        };
        assert!(!err.is_connection_error());
        assert!(err.is_recoverable());
    }

    #[test]
    fn test_backend_error_multiple_elements() {
        let err = BackendError::MultipleElements {
            locator: "JButton".to_string(),
            count: 5,
        };
        assert!(!err.is_connection_error());
        assert!(!err.is_recoverable());
    }

    #[test]
    fn test_backend_error_rpc() {
        let err = BackendError::Rpc {
            code: -32600,
            message: "Invalid request".to_string(),
        };
        assert!(!err.is_connection_error());
        assert!(!err.is_recoverable());
    }

    #[test]
    fn test_backend_error_protocol() {
        let err = BackendError::protocol("Invalid JSON");
        assert!(matches!(err, BackendError::Protocol { .. }));
    }

    #[test]
    fn test_backend_error_internal() {
        let err = BackendError::internal("Something went wrong");
        assert!(matches!(err, BackendError::Internal { .. }));
    }

    #[test]
    fn test_backend_error_display() {
        let err = BackendError::connection("Connection refused");
        let msg = format!("{}", err);
        assert!(msg.contains("Connection"));
        assert!(msg.contains("refused"));
    }

    #[test]
    fn test_backend_error_timeout_display() {
        let err = BackendError::Timeout { timeout_ms: 10000 };
        let msg = format!("{}", err);
        assert!(msg.contains("10000"));
    }

    #[test]
    fn test_backend_error_element_not_found_display() {
        let err = BackendError::ElementNotFound {
            locator: "name:test".to_string(),
        };
        let msg = format!("{}", err);
        assert!(msg.contains("name:test"));
    }
}
