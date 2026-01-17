//! Benchmarks for locator parsing and caching
//!
//! Performance targets:
//! - Locator parsing: <100us for simple locators
//! - Cached lookup: <1us

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use javagui::locator::unified::{UnifiedLocator, LocatorFactory, LocatorType, PseudoClass, MatchOp};
use javagui::locator::{parse_locator, Locator};
use javagui::core::backend::ToolkitType;

/// Benchmark unified locator parsing
fn benchmark_unified_locator_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnifiedLocator::parse");

    // Simple prefix locators
    group.bench_function("parse_name_locator", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("name:testButton")))
    });

    group.bench_function("parse_text_locator", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("text:Click Me")))
    });

    group.bench_function("parse_class_locator", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("class:JButton")))
    });

    group.bench_function("parse_index_locator", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("index:0")))
    });

    group.bench_function("parse_id_locator", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("id:12345")))
    });

    // Shorthand locators
    group.bench_function("parse_id_shorthand", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("#submitButton")))
    });

    // CSS-like locators
    group.bench_function("parse_css_simple", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("Button#submit")))
    });

    group.bench_function("parse_css_with_attribute", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("Button[text='OK']")))
    });

    group.bench_function("parse_css_with_contains", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("Button[text*='Save']")))
    });

    group.bench_function("parse_css_complex", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("JPanel > JButton[text='OK']:visible")))
    });

    // XPath locators
    group.bench_function("parse_xpath_simple", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("//JButton")))
    });

    group.bench_function("parse_xpath_with_attribute", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("//JButton[@text='Save']")))
    });

    // Toolkit-specific locators
    group.bench_function("parse_toolkit_swing", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("swing:JButton")))
    });

    group.bench_function("parse_toolkit_swt", |b| {
        b.iter(|| UnifiedLocator::parse(black_box("swt:Button")))
    });

    group.finish();
}

/// Benchmark full AST-based locator parsing (pest parser)
fn benchmark_ast_locator_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_locator");

    group.bench_function("ast_type_only", |b| {
        b.iter(|| parse_locator(black_box("JButton")))
    });

    group.bench_function("ast_type_with_id", |b| {
        b.iter(|| parse_locator(black_box("JButton#myButton")))
    });

    group.bench_function("ast_type_with_class", |b| {
        b.iter(|| parse_locator(black_box("JButton.primary.active")))
    });

    group.bench_function("ast_attribute_equals", |b| {
        b.iter(|| parse_locator(black_box("[text='Hello']")))
    });

    group.bench_function("ast_pseudo_selector", |b| {
        b.iter(|| parse_locator(black_box("JButton:enabled:visible")))
    });

    group.bench_function("ast_nth_child", |b| {
        b.iter(|| parse_locator(black_box("JButton:nth-child(3)")))
    });

    group.bench_function("ast_child_combinator", |b| {
        b.iter(|| parse_locator(black_box("JPanel > JButton")))
    });

    group.bench_function("ast_descendant_combinator", |b| {
        b.iter(|| parse_locator(black_box("JFrame JButton")))
    });

    group.bench_function("ast_complex_selector", |b| {
        b.iter(|| parse_locator(black_box("JPanel.main > JButton#submit[text='OK']:enabled:visible")))
    });

    group.bench_function("ast_xpath_simple", |b| {
        b.iter(|| parse_locator(black_box("//JButton")))
    });

    group.bench_function("ast_xpath_with_attribute", |b| {
        b.iter(|| parse_locator(black_box("//JButton[@text='Save']")))
    });

    group.bench_function("ast_xpath_multi_step", |b| {
        b.iter(|| parse_locator(black_box("//JPanel//JButton")))
    });

    group.bench_function("ast_multiple_selectors", |b| {
        b.iter(|| parse_locator(black_box("JButton, JTextField, JLabel")))
    });

    group.finish();
}

/// Benchmark locator factory JSON conversion
fn benchmark_locator_factory(c: &mut Criterion) {
    let mut group = c.benchmark_group("LocatorFactory");

    let name_locator = UnifiedLocator::name("testButton");
    let text_locator = UnifiedLocator::text("Click Me");
    let class_locator = UnifiedLocator::class("JButton");
    let xpath_locator = UnifiedLocator::xpath("//JButton[@text='Save']");

    group.bench_function("to_swing_params_name", |b| {
        b.iter(|| LocatorFactory::to_swing_params(black_box(&name_locator)))
    });

    group.bench_function("to_swing_params_text", |b| {
        b.iter(|| LocatorFactory::to_swing_params(black_box(&text_locator)))
    });

    group.bench_function("to_swing_params_class", |b| {
        b.iter(|| LocatorFactory::to_swing_params(black_box(&class_locator)))
    });

    group.bench_function("to_swing_params_xpath", |b| {
        b.iter(|| LocatorFactory::to_swing_params(black_box(&xpath_locator)))
    });

    group.bench_function("to_swt_params_class", |b| {
        b.iter(|| LocatorFactory::to_swt_params(black_box(&class_locator)))
    });

    // Parameterized benchmark for different toolkits
    let locator = UnifiedLocator::class("Button");
    for toolkit in [ToolkitType::Swing, ToolkitType::Swt, ToolkitType::Rcp] {
        group.bench_with_input(
            BenchmarkId::new("to_params", toolkit.name()),
            &toolkit,
            |b, &toolkit| {
                b.iter(|| LocatorFactory::to_params(black_box(&locator), toolkit))
            },
        );
    }

    group.finish();
}

/// Benchmark locator normalization for toolkits
fn benchmark_locator_normalization(c: &mut Criterion) {
    let mut group = c.benchmark_group("locator_normalization");

    let locator = UnifiedLocator::class("Button");

    group.bench_function("normalize_for_swing", |b| {
        b.iter(|| locator.normalize_for_toolkit(black_box(ToolkitType::Swing)))
    });

    group.bench_function("normalize_for_swt", |b| {
        b.iter(|| locator.normalize_for_toolkit(black_box(ToolkitType::Swt)))
    });

    group.bench_function("normalize_for_rcp", |b| {
        b.iter(|| locator.normalize_for_toolkit(black_box(ToolkitType::Rcp)))
    });

    // Benchmark with different class names
    let class_names = [
        "Button", "TextField", "Label", "ComboBox", "Table", "Tree",
        "JButton", "JTextField", "CustomWidget",
    ];

    for class_name in class_names {
        let locator = UnifiedLocator::class(class_name);
        group.bench_with_input(
            BenchmarkId::new("normalize_class", class_name),
            &locator,
            |b, locator| {
                b.iter(|| locator.normalize_for_toolkit(black_box(ToolkitType::Swing)))
            },
        );
    }

    group.finish();
}

/// Benchmark locator builder pattern
fn benchmark_locator_builder(c: &mut Criterion) {
    let mut group = c.benchmark_group("locator_builder");

    group.bench_function("build_with_predicate", |b| {
        b.iter(|| {
            UnifiedLocator::class(black_box("JButton"))
                .with_attribute("text", MatchOp::Equals, "OK")
        })
    });

    group.bench_function("build_with_multiple_predicates", |b| {
        b.iter(|| {
            UnifiedLocator::class(black_box("JButton"))
                .with_attribute("text", MatchOp::Equals, "OK")
                .with_pseudo_class(PseudoClass::Visible)
                .with_pseudo_class(PseudoClass::Enabled)
        })
    });

    group.finish();
}

/// Benchmark parsing varying complexity locators
fn benchmark_locator_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("locator_complexity");
    group.sample_size(100);

    // Test locators of increasing complexity
    let locators = [
        ("simple_type", "JButton"),
        ("with_id", "JButton#submit"),
        ("with_class", "JButton.primary"),
        ("with_attribute", "JButton[text='OK']"),
        ("with_pseudo", "JButton:enabled"),
        ("child_selector", "JPanel > JButton"),
        ("descendant_selector", "JFrame JPanel JButton"),
        ("complex_1", "JPanel > JButton#submit:enabled"),
        ("complex_2", "JPanel.main > JButton#submit[text='OK']:enabled"),
        ("complex_3", "JFrame JPanel.main > JButton#submit[text='OK'][enabled='true']:enabled:visible"),
    ];

    for (name, locator) in locators {
        group.bench_with_input(
            BenchmarkId::new("unified", name),
            locator,
            |b, locator| {
                b.iter(|| UnifiedLocator::parse(black_box(locator)))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("ast", name),
            locator,
            |b, locator| {
                b.iter(|| parse_locator(black_box(locator)))
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    benchmark_unified_locator_parsing,
    benchmark_ast_locator_parsing,
    benchmark_locator_factory,
    benchmark_locator_normalization,
    benchmark_locator_builder,
    benchmark_locator_complexity,
);

criterion_main!(benches);
