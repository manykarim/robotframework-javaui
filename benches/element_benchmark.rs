//! Benchmarks for element operations
//!
//! Performance targets:
//! - Element creation: <10us
//! - Type mapping: <100ns

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use javagui::core::element::{ElementType, JavaGuiElement};
use javagui::core::backend::ToolkitType;
use javagui::model::element::{UIElement, SwingComponentType, Rectangle, ElementState};
use javagui::model::component::{
    UIComponent, ComponentId, ComponentType, SwingBaseType,
    ComponentIdentity, ComponentGeometry, ComponentState, ComponentProperties,
    AccessibilityInfo, TraversalMetadata, Bounds,
};

/// Benchmark ElementType creation and mapping
fn benchmark_element_type_mapping(c: &mut Criterion) {
    let mut group = c.benchmark_group("ElementType");

    // Test Swing class name mapping
    group.bench_function("from_swing_class_simple", |b| {
        b.iter(|| ElementType::from_class_name(black_box("JButton"), ToolkitType::Swing))
    });

    group.bench_function("from_swing_class_fqn", |b| {
        b.iter(|| ElementType::from_class_name(black_box("javax.swing.JButton"), ToolkitType::Swing))
    });

    group.bench_function("from_swing_class_unknown", |b| {
        b.iter(|| ElementType::from_class_name(black_box("CustomComponent"), ToolkitType::Swing))
    });

    // Test SWT class name mapping
    group.bench_function("from_swt_class_simple", |b| {
        b.iter(|| ElementType::from_class_name(black_box("Button"), ToolkitType::Swt))
    });

    group.bench_function("from_swt_class_fqn", |b| {
        b.iter(|| ElementType::from_class_name(black_box("org.eclipse.swt.widgets.Button"), ToolkitType::Swt))
    });

    // Parameterized benchmark for all Swing component types
    let swing_classes = [
        "JButton", "JTextField", "JTextArea", "JLabel", "JComboBox",
        "JList", "JTable", "JTree", "JCheckBox", "JRadioButton",
        "JPanel", "JFrame", "JDialog", "JScrollPane", "JTabbedPane",
        "JMenu", "JMenuItem", "JToolBar", "JProgressBar", "JSlider",
    ];

    for class_name in swing_classes {
        group.bench_with_input(
            BenchmarkId::new("swing_map", class_name),
            class_name,
            |b, class| {
                b.iter(|| ElementType::from_class_name(black_box(class), ToolkitType::Swing))
            },
        );
    }

    // Parameterized benchmark for all SWT component types
    let swt_classes = [
        "Button", "Text", "StyledText", "Label", "Combo",
        "List", "Table", "Tree", "Composite", "Shell",
        "TabFolder", "Menu", "MenuItem", "ToolBar", "ProgressBar",
    ];

    for class_name in swt_classes {
        group.bench_with_input(
            BenchmarkId::new("swt_map", class_name),
            class_name,
            |b, class| {
                b.iter(|| ElementType::from_class_name(black_box(class), ToolkitType::Swt))
            },
        );
    }

    group.finish();
}

/// Benchmark ElementType property checks
fn benchmark_element_type_properties(c: &mut Criterion) {
    let mut group = c.benchmark_group("ElementType_properties");

    let types = [
        ElementType::Button,
        ElementType::TextField,
        ElementType::Panel,
        ElementType::Table,
        ElementType::Menu,
    ];

    for elem_type in types {
        group.bench_with_input(
            BenchmarkId::new("is_text_input", elem_type.name()),
            &elem_type,
            |b, t| b.iter(|| t.is_text_input()),
        );

        group.bench_with_input(
            BenchmarkId::new("is_container", elem_type.name()),
            &elem_type,
            |b, t| b.iter(|| t.is_container()),
        );

        group.bench_with_input(
            BenchmarkId::new("is_clickable", elem_type.name()),
            &elem_type,
            |b, t| b.iter(|| t.is_clickable()),
        );
    }

    group.finish();
}

/// Benchmark JavaGuiElement creation
fn benchmark_java_gui_element_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("JavaGuiElement");

    group.bench_function("new_swing", |b| {
        b.iter(|| JavaGuiElement::new(black_box(12345), black_box("javax.swing.JButton"), "swing"))
    });

    group.bench_function("new_swt", |b| {
        b.iter(|| JavaGuiElement::new(black_box(12345), black_box("org.eclipse.swt.widgets.Button"), "swt"))
    });

    group.bench_function("new_with_builder", |b| {
        b.iter(|| {
            JavaGuiElement::new(black_box(12345), black_box("JButton"), "swing")
                .with_name("testButton")
                .with_text("Click Me")
                .with_bounds(10, 20, 100, 30)
                .with_visible(true)
                .with_enabled(true)
        })
    });

    group.finish();
}

/// Benchmark JavaGuiElement from JSON
fn benchmark_java_gui_element_from_json(c: &mut Criterion) {
    let mut group = c.benchmark_group("JavaGuiElement_json");

    let simple_json = serde_json::json!({
        "hashCode": 12345,
        "className": "javax.swing.JButton",
        "name": "okButton",
        "text": "OK",
    });

    let full_json = serde_json::json!({
        "hashCode": 12345,
        "className": "javax.swing.JButton",
        "simpleName": "JButton",
        "name": "okButton",
        "text": "OK",
        "tooltip": "Click to confirm",
        "x": 100,
        "y": 200,
        "width": 80,
        "height": 25,
        "visible": true,
        "enabled": true,
        "focused": false,
        "properties": {
            "borderPainted": true,
            "contentAreaFilled": true,
        }
    });

    group.bench_function("from_json_simple", |b| {
        b.iter(|| JavaGuiElement::from_json(black_box(&simple_json), ToolkitType::Swing))
    });

    group.bench_function("from_json_full", |b| {
        b.iter(|| JavaGuiElement::from_json(black_box(&full_json), ToolkitType::Swing))
    });

    // Benchmark to_json
    let element = JavaGuiElement::from_json(&full_json, ToolkitType::Swing).unwrap();
    group.bench_function("to_json", |b| {
        b.iter(|| element.to_json())
    });

    group.finish();
}

/// Benchmark SwingComponentType mapping
fn benchmark_swing_component_type(c: &mut Criterion) {
    let mut group = c.benchmark_group("SwingComponentType");

    group.bench_function("from_class_name_jbutton", |b| {
        b.iter(|| SwingComponentType::from_class_name(black_box("javax.swing.JButton")))
    });

    group.bench_function("from_class_name_simple", |b| {
        b.iter(|| SwingComponentType::from_class_name(black_box("JTable")))
    });

    group.bench_function("from_class_name_unknown", |b| {
        b.iter(|| SwingComponentType::from_class_name(black_box("MyCustomComponent")))
    });

    // Test class_name() method
    let types = [
        SwingComponentType::Button,
        SwingComponentType::Table,
        SwingComponentType::Frame,
        SwingComponentType::Unknown,
    ];

    for comp_type in types {
        group.bench_with_input(
            BenchmarkId::new("class_name", format!("{:?}", comp_type)),
            &comp_type,
            |b, t| b.iter(|| t.class_name()),
        );

        group.bench_with_input(
            BenchmarkId::new("is_container", format!("{:?}", comp_type)),
            &comp_type,
            |b, t| b.iter(|| t.is_container()),
        );

        group.bench_with_input(
            BenchmarkId::new("is_text_input", format!("{:?}", comp_type)),
            &comp_type,
            |b, t| b.iter(|| t.is_text_input()),
        );

        group.bench_with_input(
            BenchmarkId::new("is_button", format!("{:?}", comp_type)),
            &comp_type,
            |b, t| b.iter(|| t.is_button()),
        );
    }

    group.finish();
}

/// Benchmark UIElement operations
fn benchmark_ui_element(c: &mut Criterion) {
    let mut group = c.benchmark_group("UIElement");

    group.bench_function("new", |b| {
        b.iter(|| UIElement::new(
            black_box("123".to_string()),
            black_box("javax.swing.JButton".to_string()),
        ))
    });

    // Create element for other benchmarks
    let mut element = UIElement::new("123".to_string(), "javax.swing.JButton".to_string());
    element.name = Some("testButton".to_string());
    element.text = Some("Click Me".to_string());

    group.bench_function("display_name", |b| {
        b.iter(|| element.display_name())
    });

    group.bench_function("matches_type_simple", |b| {
        b.iter(|| element.matches_type(black_box("JButton")))
    });

    group.bench_function("matches_type_fqn", |b| {
        b.iter(|| element.matches_type(black_box("javax.swing.JButton")))
    });

    group.finish();
}

/// Benchmark UIElement tree operations
fn benchmark_ui_element_tree(c: &mut Criterion) {
    let mut group = c.benchmark_group("UIElement_tree");

    // Create a tree structure for testing
    fn create_test_tree(depth: usize, breadth: usize) -> UIElement {
        let mut root = UIElement::new("0".to_string(), "javax.swing.JPanel".to_string());
        root.depth = 0;

        if depth > 0 {
            for i in 0..breadth {
                let mut child = create_test_tree(depth - 1, breadth);
                child.id = format!("{}-{}", depth, i);
                child.sibling_index = i;
                root.children.push(child);
            }
        }
        root
    }

    // Small tree (depth=2, breadth=3, ~13 elements)
    let small_tree = create_test_tree(2, 3);
    group.bench_function("count_elements_small", |b| {
        b.iter(|| small_tree.count_elements())
    });

    group.bench_function("max_depth_small", |b| {
        b.iter(|| small_tree.max_depth())
    });

    group.bench_function("descendants_small", |b| {
        b.iter(|| small_tree.descendants())
    });

    // Medium tree (depth=3, breadth=4, ~85 elements)
    let medium_tree = create_test_tree(3, 4);
    group.bench_function("count_elements_medium", |b| {
        b.iter(|| medium_tree.count_elements())
    });

    group.bench_function("max_depth_medium", |b| {
        b.iter(|| medium_tree.max_depth())
    });

    group.bench_function("descendants_medium", |b| {
        b.iter(|| medium_tree.descendants())
    });

    // Find operations
    group.bench_function("find_descendant_small", |b| {
        let pred = |e: &UIElement| e.id == "1-2";
        b.iter(|| small_tree.find_descendant(&pred))
    });

    group.bench_function("find_descendant_medium", |b| {
        let pred = |e: &UIElement| e.id == "1-3";
        b.iter(|| medium_tree.find_descendant(&pred))
    });

    group.finish();
}

/// Benchmark Rectangle operations
fn benchmark_rectangle(c: &mut Criterion) {
    let mut group = c.benchmark_group("Rectangle");

    let rect = Rectangle::new(100, 200, 300, 150);

    group.bench_function("contains_inside", |b| {
        b.iter(|| rect.contains(black_box(200), black_box(250)))
    });

    group.bench_function("contains_outside", |b| {
        b.iter(|| rect.contains(black_box(50), black_box(50)))
    });

    group.bench_function("center", |b| {
        b.iter(|| rect.center())
    });

    let rect2 = Rectangle::new(350, 300, 100, 100);
    group.bench_function("intersects_true", |b| {
        let r1 = Rectangle::new(100, 100, 200, 200);
        let r2 = Rectangle::new(150, 150, 200, 200);
        b.iter(|| r1.intersects(black_box(&r2)))
    });

    group.bench_function("intersects_false", |b| {
        b.iter(|| rect.intersects(black_box(&rect2)))
    });

    group.finish();
}

/// Benchmark UIComponent (complex component model)
fn benchmark_ui_component(c: &mut Criterion) {
    let mut group = c.benchmark_group("UIComponent");

    group.bench_function("new", |b| {
        b.iter(|| {
            let id = ComponentId::new(black_box(12345), "0.1.2".to_string(), 3);
            let comp_type = ComponentType::default();
            UIComponent::new(id, comp_type)
        })
    });

    group.bench_function("component_id_new", |b| {
        b.iter(|| ComponentId::new(black_box(12345), black_box("0.1.2".to_string()), black_box(3)))
    });

    group.bench_function("swing_base_type_from_class", |b| {
        b.iter(|| SwingBaseType::from_class_name(black_box("javax.swing.JButton")))
    });

    group.finish();
}

/// Benchmark Bounds operations
fn benchmark_bounds(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bounds");

    group.bench_function("new", |b| {
        b.iter(|| Bounds::new(black_box(100), black_box(200), black_box(300), black_box(150)))
    });

    let bounds = Bounds::new(100, 200, 300, 150);
    group.bench_function("center", |b| {
        b.iter(|| bounds.center())
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_element_type_mapping,
    benchmark_element_type_properties,
    benchmark_java_gui_element_creation,
    benchmark_java_gui_element_from_json,
    benchmark_swing_component_type,
    benchmark_ui_element,
    benchmark_ui_element_tree,
    benchmark_rectangle,
    benchmark_ui_component,
    benchmark_bounds,
);

criterion_main!(benches);
