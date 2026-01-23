//! Performance benchmarks for UI tree depth control
//!
//! This benchmark suite measures the performance impact of depth limiting
//! on UI tree traversal across different tree sizes and depths.
//!
//! Test scenarios:
//! - Component counts: 100, 500, 1000, 5000
//! - Depths: 1, 5, 10, unlimited
//! - Metrics: Time, memory, JSON size
//!
//! Run with: cargo bench --bench tree_depth_benchmark

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use javagui::model::{
    element::UIElement, tree::UITree, element::ElementState,
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
            visible: *id_counter % 5 != 0,
            enabled: *id_counter % 4 != 0,
            focused: false,
            selected: *id_counter % 7 == 0,
            editable: *id_counter % 3 == 0,
            showing: *id_counter % 5 != 0,
            focusable: *id_counter % 4 != 0,
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
        "Depth Benchmark Window".to_string(),
        "JFrame".to_string(),
        root,
        12345,
    )
}

/// Benchmark tree retrieval with different depths and sizes
fn bench_tree_depth(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_depth_performance");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(50);

    // Test configurations: (component_count, tree_depth, query_depth)
    let configs = vec![
        // Small trees
        (100, 5, Some(1)),
        (100, 5, Some(5)),
        (100, 10, Some(10)),
        (100, 10, None), // Unlimited

        // Medium trees
        (500, 8, Some(1)),
        (500, 8, Some(5)),
        (500, 10, Some(10)),
        (500, 10, None),

        // Large trees
        (1000, 10, Some(1)),
        (1000, 10, Some(5)),
        (1000, 10, Some(10)),
        (1000, 10, None),

        // Very large trees
        (5000, 10, Some(1)),
        (5000, 10, Some(5)),
        (5000, 10, Some(10)),
        (5000, 10, None),
    ];

    for (component_count, tree_depth, query_depth) in configs {
        let tree = generate_test_tree(component_count, tree_depth);
        let depth_str = query_depth.map_or("unlimited".to_string(), |d| d.to_string());
        let bench_id = BenchmarkId::from_parameter(
            format!("{}_components_depth_{}", component_count, depth_str)
        );

        group.throughput(Throughput::Elements(component_count as u64));

        group.bench_with_input(
            bench_id,
            &query_depth,
            |b, &depth| {
                b.iter(|| {
                    let result = match depth {
                        Some(d) => tree.to_summary(d),
                        None => tree.to_json_compact().unwrap(),
                    };
                    black_box(result);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark memory consumption with different depths
fn bench_tree_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_depth_memory");
    group.measurement_time(Duration::from_secs(10));

    // Measure JSON serialization size as proxy for memory
    let test_configs = vec![
        (1000, 10, Some(1)),
        (1000, 10, Some(5)),
        (1000, 10, None),
        (5000, 10, Some(1)),
        (5000, 10, Some(5)),
    ];

    for (count, tree_depth, query_depth) in test_configs {
        let tree = generate_test_tree(count, tree_depth);
        let bench_id = BenchmarkId::from_parameter(
            format!("{}_depth_{:?}", count, query_depth)
        );

        group.bench_with_input(
            bench_id,
            &query_depth,
            |b, &depth| {
                b.iter(|| {
                    let json = match depth {
                        Some(d) => tree.to_summary(d),
                        None => tree.to_json_compact().unwrap(),
                    };
                    let size = json.len();
                    black_box(size);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark repeated queries (simulating cache behavior)
fn bench_tree_caching(c: &mut Criterion) {
    let mut group = c.benchmark_group("tree_caching");
    group.measurement_time(Duration::from_secs(10));

    let tree = generate_test_tree(1000, 10);

    // Repeated unlimited depth queries
    group.bench_function("repeated_unlimited", |b| {
        b.iter(|| {
            let json1 = tree.to_json_compact().unwrap();
            let json2 = tree.to_json_compact().unwrap();
            black_box((json1, json2));
        });
    });

    // Repeated depth-limited queries
    group.bench_function("repeated_depth_5", |b| {
        b.iter(|| {
            let summary1 = tree.to_summary(5);
            let summary2 = tree.to_summary(5);
            black_box((summary1, summary2));
        });
    });

    // Single query benchmarks for comparison
    group.bench_function("single_unlimited", |b| {
        b.iter(|| {
            let json = tree.to_json_compact().unwrap();
            black_box(json);
        });
    });

    group.bench_function("single_depth_5", |b| {
        b.iter(|| {
            let summary = tree.to_summary(5);
            black_box(summary);
        });
    });

    group.finish();
}

/// Benchmark JSON serialization and parsing overhead
fn bench_json_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_operations");
    group.measurement_time(Duration::from_secs(10));

    // Generate trees with different depths
    for depth in [1, 5, 10] {
        let tree = generate_test_tree(1000, depth);

        // Benchmark serialization
        group.bench_function(&format!("serialize_depth_{}", depth), |b| {
            b.iter(|| {
                let json = tree.to_json_compact().unwrap();
                black_box(json);
            });
        });

        // Benchmark round-trip (serialize + parse)
        group.bench_function(&format!("roundtrip_depth_{}", depth), |b| {
            b.iter(|| {
                let json = tree.to_json_compact().unwrap();
                let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
                black_box(parsed);
            });
        });
    }

    group.finish();
}

/// Benchmark depth limiting efficiency
fn bench_depth_efficiency(c: &mut Criterion) {
    let mut group = c.benchmark_group("depth_efficiency");
    group.measurement_time(Duration::from_secs(10));

    let large_tree = generate_test_tree(5000, 15);

    // Compare shallow vs deep queries on large trees
    for depth in [1, 3, 5, 10, 15] {
        group.bench_function(&format!("large_tree_depth_{}", depth), |b| {
            b.iter(|| {
                let summary = large_tree.to_summary(depth);
                black_box(summary);
            });
        });
    }

    // Unlimited depth for comparison
    group.bench_function("large_tree_unlimited", |b| {
        b.iter(|| {
            let json = large_tree.to_json_compact().unwrap();
            black_box(json);
        });
    });

    group.finish();
}

criterion_group!(
    depth_benchmarks,
    bench_tree_depth,
    bench_tree_memory,
);

criterion_group!(
    operations_benchmarks,
    bench_tree_caching,
    bench_json_operations,
    bench_depth_efficiency,
);

criterion_main!(
    depth_benchmarks,
    operations_benchmarks,
);
