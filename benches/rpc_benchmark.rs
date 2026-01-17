//! Benchmarks for RPC serialization and protocol operations
//!
//! These benchmarks test serialization performance without network overhead.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use serde_json::{json, Value};
use javagui::core::element::JavaGuiElement;
use javagui::core::backend::ToolkitType;

/// Benchmark JSON-RPC request serialization
fn benchmark_request_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("rpc_request_serialization");

    // Simple requests
    group.bench_function("serialize_ping", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "ping",
            "params": {},
            "id": 1
        })))
    });

    group.bench_function("serialize_click", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "click",
            "params": {"componentId": black_box(12345)},
            "id": 1
        })))
    });

    group.bench_function("serialize_get_text", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "getText",
            "params": {"componentId": black_box(12345)},
            "id": 2
        })))
    });

    group.bench_function("serialize_type_text", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "typeText",
            "params": {
                "componentId": black_box(12345),
                "text": black_box("Hello, World!"),
                "clearFirst": true
            },
            "id": 3
        })))
    });

    // Find element requests with different locator types
    group.bench_function("serialize_find_by_name", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "findElement",
            "params": {
                "locatorType": "name",
                "value": black_box("submitButton")
            },
            "id": 4
        })))
    });

    group.bench_function("serialize_find_by_class", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "findElement",
            "params": {
                "locatorType": "class",
                "value": black_box("JButton"),
                "predicates": [
                    {"type": "attribute", "name": "text", "op": "=", "value": "OK"}
                ]
            },
            "id": 5
        })))
    });

    group.bench_function("serialize_find_by_xpath", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "findElement",
            "params": {
                "locatorType": "xpath",
                "xpath": black_box("//JButton[@text='Save']")
            },
            "id": 6
        })))
    });

    // Complex requests
    group.bench_function("serialize_get_component_tree", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "getComponentTree",
            "params": {
                "rootHashCode": black_box(12345),
                "maxDepth": 10,
                "includeInvisible": false
            },
            "id": 7
        })))
    });

    group.bench_function("serialize_table_operation", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "jsonrpc": "2.0",
            "method": "selectTableCell",
            "params": {
                "componentId": black_box(12345),
                "row": 5,
                "column": 3,
                "extend": false
            },
            "id": 8
        })))
    });

    group.finish();
}

/// Benchmark JSON-RPC response parsing
fn benchmark_response_parsing(c: &mut Criterion) {
    let mut group = c.benchmark_group("rpc_response_parsing");

    // Simple responses
    let ping_response = r#"{"jsonrpc":"2.0","result":"pong","id":1}"#;
    group.bench_function("parse_ping_response", |b| {
        b.iter(|| serde_json::from_str::<Value>(black_box(ping_response)))
    });

    // Boolean response
    let bool_response = r#"{"jsonrpc":"2.0","result":true,"id":2}"#;
    group.bench_function("parse_bool_response", |b| {
        b.iter(|| serde_json::from_str::<Value>(black_box(bool_response)))
    });

    // String response
    let text_response = r#"{"jsonrpc":"2.0","result":"Hello, World!","id":3}"#;
    group.bench_function("parse_text_response", |b| {
        b.iter(|| serde_json::from_str::<Value>(black_box(text_response)))
    });

    // Element response
    let element_response = r#"{
        "jsonrpc":"2.0",
        "result":{
            "hashCode":12345,
            "className":"javax.swing.JButton",
            "name":"okButton",
            "text":"OK",
            "x":100,"y":200,"width":80,"height":25,
            "visible":true,"enabled":true,"focused":false
        },
        "id":4
    }"#;
    group.bench_function("parse_element_response", |b| {
        b.iter(|| serde_json::from_str::<Value>(black_box(element_response)))
    });

    // Error response
    let error_response = r##"{
        "jsonrpc":"2.0",
        "error":{"code":-32001,"message":"Element not found","data":{"locator":"#missing"}},
        "id":5
    }"##;
    group.bench_function("parse_error_response", |b| {
        b.iter(|| serde_json::from_str::<Value>(black_box(error_response)))
    });

    // Large tree response (simulated)
    let tree_response = generate_tree_response(50);
    group.bench_function("parse_tree_response_50", |b| {
        b.iter(|| serde_json::from_str::<Value>(black_box(&tree_response)))
    });

    let tree_response_200 = generate_tree_response(200);
    group.bench_function("parse_tree_response_200", |b| {
        b.iter(|| serde_json::from_str::<Value>(black_box(&tree_response_200)))
    });

    group.finish();
}

/// Helper to generate a mock tree response with n elements
fn generate_tree_response(n: usize) -> String {
    let mut children = Vec::new();
    for i in 0..n {
        children.push(json!({
            "hashCode": 10000 + i,
            "className": format!("javax.swing.J{}", if i % 2 == 0 { "Button" } else { "Label" }),
            "name": format!("element_{}", i),
            "text": format!("Text {}", i),
            "x": 10 + (i % 10) * 100,
            "y": 10 + (i / 10) * 30,
            "width": 80,
            "height": 25,
            "visible": true,
            "enabled": i % 3 != 0,
            "children": []
        }));
    }

    serde_json::to_string(&json!({
        "jsonrpc": "2.0",
        "result": {
            "hashCode": 1,
            "className": "javax.swing.JPanel",
            "name": "mainPanel",
            "children": children
        },
        "id": 100
    })).unwrap()
}

/// Benchmark element extraction from JSON response
fn benchmark_element_extraction(c: &mut Criterion) {
    let mut group = c.benchmark_group("element_extraction");

    let element_json = json!({
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
            "rolloverEnabled": true
        }
    });

    group.bench_function("extract_element", |b| {
        b.iter(|| JavaGuiElement::from_json(black_box(&element_json), ToolkitType::Swing))
    });

    // Batch extraction
    let elements_json: Vec<Value> = (0..50)
        .map(|i| json!({
            "hashCode": 10000 + i,
            "className": "javax.swing.JButton",
            "name": format!("button_{}", i),
            "text": format!("Button {}", i),
            "x": i * 10,
            "y": i * 5,
            "width": 80,
            "height": 25,
            "visible": true,
            "enabled": true,
        }))
        .collect();

    group.bench_function("extract_elements_batch_50", |b| {
        b.iter(|| {
            elements_json
                .iter()
                .filter_map(|j| JavaGuiElement::from_json(j, ToolkitType::Swing))
                .collect::<Vec<_>>()
        })
    });

    group.finish();
}

/// Benchmark serialization of different param sizes
fn benchmark_param_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("param_sizes");

    // Empty params
    group.bench_function("serialize_empty_params", |b| {
        b.iter(|| serde_json::to_string(&json!({})))
    });

    // Small params (1-3 fields)
    group.bench_function("serialize_small_params", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "id": black_box(12345)
        })))
    });

    // Medium params (5-10 fields)
    group.bench_function("serialize_medium_params", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "id": black_box(12345),
            "text": black_box("Hello"),
            "x": 100,
            "y": 200,
            "modifiers": ["shift", "ctrl"]
        })))
    });

    // Large params (complex nested structure)
    group.bench_function("serialize_large_params", |b| {
        b.iter(|| serde_json::to_string(&json!({
            "locator": {
                "type": "css",
                "value": "JPanel > JButton",
                "predicates": [
                    {"type": "attribute", "name": "text", "op": "=", "value": "OK"},
                    {"type": "pseudo", "value": "visible"},
                    {"type": "pseudo", "value": "enabled"}
                ]
            },
            "options": {
                "timeout": 5000,
                "retries": 3,
                "interval": 500
            }
        })))
    });

    group.finish();
}

/// Benchmark Value operations commonly used in RPC
fn benchmark_value_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("value_operations");

    let response: Value = serde_json::from_str(r#"{
        "jsonrpc": "2.0",
        "result": {
            "hashCode": 12345,
            "className": "javax.swing.JButton",
            "name": "okButton",
            "visible": true,
            "enabled": true
        },
        "id": 1
    }"#).unwrap();

    group.bench_function("get_field", |b| {
        b.iter(|| response.get("result"))
    });

    group.bench_function("get_nested_field", |b| {
        b.iter(|| response.get("result").and_then(|r| r.get("className")))
    });

    group.bench_function("as_str", |b| {
        let result = response.get("result").unwrap();
        b.iter(|| result.get("className").and_then(|v| v.as_str()))
    });

    group.bench_function("as_i64", |b| {
        let result = response.get("result").unwrap();
        b.iter(|| result.get("hashCode").and_then(|v| v.as_i64()))
    });

    group.bench_function("as_bool", |b| {
        let result = response.get("result").unwrap();
        b.iter(|| result.get("visible").and_then(|v| v.as_bool()))
    });

    group.bench_function("is_object", |b| {
        b.iter(|| response.get("result").map(|v| v.is_object()))
    });

    // Check for error field (common operation)
    group.bench_function("check_error", |b| {
        b.iter(|| response.get("error").is_some())
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_request_serialization,
    benchmark_response_parsing,
    benchmark_element_extraction,
    benchmark_param_sizes,
    benchmark_value_operations,
);

criterion_main!(benches);
