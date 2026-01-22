//! Comprehensive performance benchmarks for component tree operations
//!
//! This benchmark suite measures performance across all implementation phases:
//! - Phase 1: Fixed bugs (baseline)
//! - Phase 2: Depth control
//! - Phase 3: Filtering
//! - Phase 4: New formats (YAML, CSV, Markdown)
//! - Phase 5: SWT backend
//! - Phase 6: RCP backend
//!
//! Performance Targets:
//! - Tree retrieval: <100ms for 1000 components
//! - Memory usage: <50MB for 10,000 components
//! - Depth 1: <10ms for any UI size
//! - Depth 5: <50ms for 1000 components
//!
//! Run with: cargo bench --bench component_tree_benchmark

use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput,
};
use javagui::model::{
    element::UIElement, tree::UITree, tree::TreeFilter, element::ElementState,
};
use std::time::Duration;

// ========================================
// Test Data Generation
// ========================================

/// Generate a synthetic UI tree with specified component count and depth
fn generate_test_tree(total_components: usize, max_depth: usize) -> UITree {
    fn build_balanced_tree(
        components_left: &mut usize,
        current_depth: usize,
        max_depth: usize,
        id_counter: &mut usize,
    ) -> Option<UIElement> {
        if *components_left == 0 || current_depth > max_depth {
            return None;
        }

        *components_left -= 1;
        *id_counter += 1;

        let mut element = UIElement::new(
            id_counter.to_string(),
            if current_depth == 0 {
                "javax.swing.JFrame".to_string()
            } else if *id_counter % 3 == 0 {
                "javax.swing.JButton".to_string()
            } else if *id_counter % 3 == 1 {
                "javax.swing.JLabel".to_string()
            } else {
                "javax.swing.JPanel".to_string()
            },
        );

        element.name = Some(format!("component_{}", id_counter));
        element.text = Some(format!("Text {}", id_counter));
        element.state = ElementState {
            visible: *id_counter % 5 != 0, // 20% hidden
            enabled: *id_counter % 4 != 0,  // 25% disabled
            focused: false,
            selected: *id_counter % 7 == 0,  // ~14% selected
            editable: *id_counter % 3 == 0,
            showing: *id_counter % 5 != 0,  // Same as visible
            focusable: *id_counter % 4 != 0, // Same as enabled
        };

        // Add children if we haven't reached max depth
        if current_depth < max_depth && *components_left > 0 {
            let children_per_node = (*components_left).min(3);
            for _ in 0..children_per_node {
                if let Some(child) = build_balanced_tree(
                    components_left,
                    current_depth + 1,
                    max_depth,
                    id_counter,
                ) {
                    element.children.push(child);
                }
            }
        }

        Some(element)
    }

    let mut components_left = total_components;
    let mut id_counter = 0;
    let root = build_balanced_tree(&mut components_left, 0, max_depth, &mut id_counter)
        .unwrap_or_else(|| UIElement::new("1".to_string(), "javax.swing.JFrame".to_string()));

    UITree::new(
        "Benchmark Test Window".to_string(),
        "JFrame".to_string(),
        root,
        12345,
    )
}

// ========================================
// Tree Traversal Benchmarks
// ========================================

fn bench_tree_retrieval_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_retrieval_by_size");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);

    for size in [10, 100, 500, 1000, 5000].iter() {
        let tree = generate_test_tree(*size, 10);

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_components", size)),
            size,
            |b, _| {
                b.iter(|| {
                    let json = tree.to_json_compact().unwrap();
                    black_box(json);
                });
            },
        );
    }

    group.finish();
}

fn bench_tree_depth_control(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_depth_control");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);

    let tree = generate_test_tree(1000, 10);

    for depth in [1, 3, 5, 10].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("depth_{}", depth)),
            depth,
            |b, &max_depth| {
                b.iter(|| {
                    let summary = tree.to_summary(max_depth);
                    black_box(summary);
                });
            },
        );
    }

    // Benchmark unlimited depth
    group.bench_function("depth_unlimited", |b| {
        b.iter(|| {
            let json = tree.to_json_compact().unwrap();
            black_box(json);
        });
    });

    group.finish();
}

// ========================================
// Output Format Benchmarks
// ========================================

fn bench_output_formats(c: &mut Criterion) {
    let mut group = c.benchmark_group("output_formats");
    group.measurement_time(Duration::from_secs(10));

    let tree = generate_test_tree(500, 8);

    group.bench_function("json_pretty", |b| {
        b.iter(|| {
            let json = tree.to_json().unwrap();
            black_box(json);
        });
    });

    group.bench_function("json_compact", |b| {
        b.iter(|| {
            let json = tree.to_json_compact().unwrap();
            black_box(json);
        });
    });

    group.bench_function("yaml", |b| {
        b.iter(|| {
            let yaml = tree.to_yaml().unwrap();
            black_box(yaml);
        });
    });

    group.bench_function("text_tree", |b| {
        b.iter(|| {
            let text = tree.to_text_tree();
            black_box(text);
        });
    });

    group.bench_function("robot_log", |b| {
        b.iter(|| {
            let html = tree.to_robot_log();
            black_box(html);
        });
    });

    group.finish();
}

// ========================================
// Filtering Benchmarks
// ========================================

fn bench_filtering_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("filtering_operations");
    group.measurement_time(Duration::from_secs(10));

    let tree = generate_test_tree(1000, 10);

    group.bench_function("filter_visible_only", |b| {
        b.iter(|| {
            let filter = TreeFilter::new().visible_only(true);
            let results = tree.find_all(|e| filter.matches(e, 0));
            black_box(results);
        });
    });

    group.bench_function("filter_enabled_only", |b| {
        b.iter(|| {
            let filter = TreeFilter::new().enabled_only(true);
            let results = tree.find_all(|e| filter.matches(e, 0));
            black_box(results);
        });
    });

    group.bench_function("filter_by_type", |b| {
        b.iter(|| {
            let filter = TreeFilter::new().include_type("JButton");
            let results = tree.find_all(|e| filter.matches(e, 0));
            black_box(results);
        });
    });

    group.bench_function("filter_exclude_type", |b| {
        b.iter(|| {
            let filter = TreeFilter::new().exclude_type("JPanel");
            let results = tree.find_all(|e| filter.matches(e, 0));
            black_box(results);
        });
    });

    group.bench_function("filter_combined", |b| {
        b.iter(|| {
            let filter = TreeFilter::new()
                .visible_only(true)
                .enabled_only(true)
                .include_type("JButton");
            let results = tree.find_all(|e| filter.matches(e, 0));
            black_box(results);
        });
    });

    group.bench_function("filter_with_depth", |b| {
        b.iter(|| {
            let filter = TreeFilter::new()
                .max_depth(5)
                .visible_only(true);
            let results = tree.find_all(|e| filter.matches(e, 0));
            black_box(results);
        });
    });

    group.finish();
}

// ========================================
// Statistics Calculation Benchmarks
// ========================================

fn bench_statistics_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("statistics_calculation");
    group.measurement_time(Duration::from_secs(10));

    for size in [100, 500, 1000, 5000].iter() {
        let mut tree = generate_test_tree(*size, 10);

        group.throughput(Throughput::Elements(*size as u64));
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}_components", size)),
            size,
            |b, _| {
                b.iter(|| {
                    tree.calculate_stats();
                    black_box(&tree.stats);
                });
            },
        );
    }

    group.finish();
}

// ========================================
// Memory and Serialization Size Benchmarks
// ========================================

fn bench_serialization_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("serialization_sizes");

    for size in [100, 500, 1000, 5000].iter() {
        let tree = generate_test_tree(*size, 10);

        // JSON size
        group.bench_with_input(
            BenchmarkId::new("json_size", size),
            size,
            |b, _| {
                b.iter(|| {
                    let json = tree.to_json_compact().unwrap();
                    black_box(json.len());
                });
            },
        );

        // YAML size
        group.bench_with_input(
            BenchmarkId::new("yaml_size", size),
            size,
            |b, _| {
                b.iter(|| {
                    let yaml = tree.to_yaml().unwrap();
                    black_box(yaml.len());
                });
            },
        );

        // Text size
        group.bench_with_input(
            BenchmarkId::new("text_size", size),
            size,
            |b, _| {
                b.iter(|| {
                    let text = tree.to_text_tree();
                    black_box(text.len());
                });
            },
        );
    }

    group.finish();
}

// ========================================
// Depth vs Size Performance Matrix
// ========================================

fn bench_depth_size_matrix(c: &mut Criterion) {
    let mut group = c.benchmark_group("depth_size_matrix");
    group.measurement_time(Duration::from_secs(15));
    group.sample_size(30);

    // Test various combinations of size and depth
    let test_configs = vec![
        // (total_components, tree_depth, query_depth)
        (100, 5, 1),
        (100, 5, 3),
        (100, 5, 5),
        (500, 8, 1),
        (500, 8, 5),
        (500, 8, 8),
        (1000, 10, 1),
        (1000, 10, 5),
        (1000, 10, 10),
        (5000, 10, 1),
        (5000, 10, 5),
    ];

    for (size, tree_depth, query_depth) in test_configs {
        let tree = generate_test_tree(size, tree_depth);

        group.bench_with_input(
            BenchmarkId::from_parameter(format!("size{}_depth{}", size, query_depth)),
            &(size, query_depth),
            |b, &(_, depth)| {
                b.iter(|| {
                    let summary = tree.to_summary(depth);
                    black_box(summary);
                });
            },
        );
    }

    group.finish();
}

// ========================================
// Real-world Scenario Benchmarks
// ========================================

fn bench_realistic_scenarios(c: &mut Criterion) {
    let mut group = c.benchmark_group("realistic_scenarios");
    group.measurement_time(Duration::from_secs(15));

    // Scenario 1: Quick inspection (depth 1, text format)
    group.bench_function("quick_inspection", |b| {
        let tree = generate_test_tree(1000, 10);
        b.iter(|| {
            let text = tree.to_summary(1);
            black_box(text);
        });
    });

    // Scenario 2: Full tree export (JSON)
    group.bench_function("full_export_json", |b| {
        let tree = generate_test_tree(1000, 10);
        b.iter(|| {
            let json = tree.to_json_compact().unwrap();
            black_box(json);
        });
    });

    // Scenario 3: Filtered button search
    group.bench_function("find_all_buttons", |b| {
        let tree = generate_test_tree(1000, 10);
        b.iter(|| {
            let filter = TreeFilter::new()
                .include_type("JButton")
                .visible_only(true)
                .enabled_only(true);
            let results = tree.find_all(|e| filter.matches(e, 0));
            black_box(results);
        });
    });

    // Scenario 4: Debug logging (depth 3, text)
    group.bench_function("debug_logging", |b| {
        let tree = generate_test_tree(1000, 10);
        b.iter(|| {
            let text = tree.to_summary(3);
            black_box(text);
        });
    });

    // Scenario 5: Statistics calculation for reporting
    group.bench_function("calculate_statistics", |b| {
        let mut tree = generate_test_tree(1000, 10);
        b.iter(|| {
            tree.calculate_stats();
            black_box(&tree.stats);
        });
    });

    group.finish();
}

// ========================================
// Performance Target Validation
// ========================================

fn bench_performance_targets(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_targets");
    group.measurement_time(Duration::from_secs(20));
    group.sample_size(100);

    // Target: Tree retrieval <100ms for 1000 components
    group.bench_function("target_1000_components", |b| {
        let tree = generate_test_tree(1000, 10);
        b.iter(|| {
            let json = tree.to_json_compact().unwrap();
            black_box(json);
        });
    });

    // Target: Depth 1 <10ms for any UI size
    group.bench_function("target_depth1_large", |b| {
        let tree = generate_test_tree(5000, 10);
        b.iter(|| {
            let summary = tree.to_summary(1);
            black_box(summary);
        });
    });

    // Target: Depth 5 <50ms for 1000 components
    group.bench_function("target_depth5_1000", |b| {
        let tree = generate_test_tree(1000, 10);
        b.iter(|| {
            let summary = tree.to_summary(5);
            black_box(summary);
        });
    });

    group.finish();
}

// ========================================
// Benchmark Groups
// ========================================

criterion_group!(
    basic_benchmarks,
    bench_tree_retrieval_sizes,
    bench_tree_depth_control,
    bench_output_formats,
);

criterion_group!(
    filtering_benchmarks,
    bench_filtering_operations,
    bench_statistics_calculation,
);

criterion_group!(
    advanced_benchmarks,
    bench_serialization_sizes,
    bench_depth_size_matrix,
    bench_realistic_scenarios,
);

criterion_group!(
    target_validation,
    bench_performance_targets,
);

criterion_main!(
    basic_benchmarks,
    filtering_benchmarks,
    advanced_benchmarks,
    target_validation,
);
