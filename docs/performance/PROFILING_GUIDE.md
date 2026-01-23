# Performance Profiling Guide

## Overview

This guide explains how to profile the robotframework-javagui component tree implementation to identify performance bottlenecks and optimization opportunities.

## Quick Start

```bash
# 1. Install profiling tools
cargo install flamegraph cargo-flamegraph

# 2. Run profiling
cargo flamegraph --bench component_tree_benchmark

# 3. View results
open flamegraph.svg
```

## Profiling Tools

### 1. Flamegraph (Recommended)

**Installation:**
```bash
cargo install flamegraph
```

**Usage:**
```bash
# Profile benchmark
cargo flamegraph --bench component_tree_benchmark

# Profile specific test
cargo flamegraph --bench component_tree_benchmark -- bench_tree_retrieval_sizes

# With specific benchmark filter
cargo flamegraph --bench component_tree_benchmark -- --bench "1000_components"
```

**Output:**
- `flamegraph.svg`: Interactive flame graph
- Shows call stack and time distribution
- Identify hot paths visually

### 2. Perf (Linux)

**Installation:**
```bash
# Ubuntu/Debian
sudo apt-get install linux-tools-common linux-tools-generic

# Fedora/RHEL
sudo dnf install perf
```

**Usage:**
```bash
# Build benchmark in release mode
cargo bench --bench component_tree_benchmark --no-run

# Profile with perf
perf record --call-graph dwarf ./target/release/deps/component_tree_benchmark-* --bench

# View report
perf report

# Generate flamegraph from perf data
perf script | stackcollapse-perf.pl | flamegraph.pl > flamegraph.svg
```

### 3. Valgrind/Cachegrind

**Installation:**
```bash
sudo apt-get install valgrind kcachegrind
```

**Usage:**
```bash
# Profile cache behavior
valgrind --tool=cachegrind --cachegrind-out-file=cachegrind.out \
    ./target/release/deps/component_tree_benchmark-*

# Visualize
kcachegrind cachegrind.out
```

### 4. Massif (Memory Profiling)

**Usage:**
```bash
# Profile memory usage
valgrind --tool=massif --massif-out-file=massif.out \
    ./target/release/deps/component_tree_benchmark-*

# Visualize
ms_print massif.out > massif.txt
less massif.txt
```

### 5. Criterion Built-in Profiling

Criterion supports profiling integration:

```bash
# With flamegraph
cargo bench --bench component_tree_benchmark -- --profile-time=30

# Check target/criterion/*/profile for results
```

## Key Areas to Profile

### 1. Tree Traversal

**Focus:**
- `collect_stats` function
- Recursive children iteration
- Depth tracking

**Expected Hotspots:**
- Element iteration loops
- Depth comparison
- Type count HashMap updates

**Commands:**
```bash
cargo flamegraph --bench component_tree_benchmark -- --bench "tree_retrieval"
```

### 2. Output Formatters

**Focus:**
- `to_json`, `to_yaml`, `to_text_tree` functions
- String formatting and concatenation
- Serialization overhead

**Expected Hotspots:**
- `format!` macro calls
- String allocations
- Serde serialization

**Commands:**
```bash
cargo flamegraph --bench component_tree_benchmark -- --bench "output_formats"
```

### 3. Filtering

**Focus:**
- `TreeFilter::matches` function
- Regex compilation
- Type matching

**Expected Hotspots:**
- Regex::new calls
- String comparisons
- Predicate evaluation

**Commands:**
```bash
cargo flamegraph --bench component_tree_benchmark -- --bench "filtering"
```

### 4. JSON Parsing/Serialization

**Focus:**
- Serde JSON operations
- Value extraction
- Type conversions

**Expected Hotspots:**
- `serde_json::to_string`
- `serde_json::from_str`
- Value::get operations

## Profiling Workflow

### Step 1: Establish Baseline

```bash
# Run benchmarks to get baseline
cargo bench --bench component_tree_benchmark > baseline.txt

# Profile overall execution
cargo flamegraph --bench component_tree_benchmark
```

### Step 2: Identify Bottlenecks

Review the flamegraph:
1. Look for wide bars (high time consumption)
2. Identify deep call stacks (potential optimization)
3. Note unexpected function calls
4. Check for allocation hotspots

### Step 3: Drill Down

Profile specific areas:
```bash
# Tree retrieval only
cargo flamegraph --bench component_tree_benchmark -- --bench "tree_retrieval"

# Filtering only
cargo flamegraph --bench component_tree_benchmark -- --bench "filtering"

# Formatters only
cargo flamegraph --bench component_tree_benchmark -- --bench "output_formats"
```

### Step 4: Optimize

Common optimizations:
- Replace String concatenation with `write!`
- Cache compiled regexes
- Use iterators instead of recursion
- Pre-allocate buffers
- Reduce cloning

### Step 5: Validate

```bash
# Re-run benchmarks
cargo bench --bench component_tree_benchmark > optimized.txt

# Compare results
diff baseline.txt optimized.txt

# Re-profile
cargo flamegraph --bench component_tree_benchmark
```

## Interpreting Results

### Flamegraph Colors
- **Red**: System/kernel functions
- **Orange**: Libraries
- **Yellow**: Application code
- **Green**: Application code (leaf functions)

### What to Look For

**Good Signs:**
- Flat, wide bars in expected places
- Balanced time distribution
- Minimal deep recursion

**Warning Signs:**
- Unexpected hot paths
- Deep call stacks
- Excessive allocations
- High time in string operations

## Common Bottlenecks

### 1. String Allocations

**Symptom:**
```
String::from
format!
String::push_str
```

**Fix:**
```rust
// Before
let mut output = String::new();
output.push_str(&format!("Value: {}", value));

// After
let mut output = String::with_capacity(expected_size);
write!(output, "Value: {}", value).unwrap();
```

### 2. Cloning

**Symptom:**
```
Clone::clone
Vec::clone
String::clone
```

**Fix:**
```rust
// Before
fn process(tree: UITree) { ... }

// After
fn process(tree: &UITree) { ... }
```

### 3. Regex Compilation

**Symptom:**
```
Regex::new
regex::compile
```

**Fix:**
```rust
use once_cell::sync::Lazy;

static PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"pattern").unwrap());
```

### 4. Repeated Serialization

**Symptom:**
```
serde_json::to_string (called many times)
```

**Fix:**
```rust
// Cache serialized results
let mut cache = HashMap::new();
cache.entry(key).or_insert_with(|| tree.to_json().unwrap());
```

## Memory Profiling

### Using Massif

```bash
# Run with massif
valgrind --tool=massif \
    --massif-out-file=massif.out \
    --time-unit=B \
    ./target/release/deps/component_tree_benchmark-*

# Analyze
ms_print massif.out | less
```

**Look for:**
- Peak memory usage
- Allocation patterns
- Memory leaks
- Unnecessary allocations

### Memory Optimization Tips

1. **Pre-allocate collections:**
   ```rust
   let mut children = Vec::with_capacity(expected_count);
   ```

2. **Use `String::with_capacity`:**
   ```rust
   let mut output = String::with_capacity(estimated_size);
   ```

3. **Avoid cloning when possible:**
   ```rust
   // Use references
   fn format_element(element: &UIElement) -> String { ... }
   ```

4. **Use `Cow` for conditional ownership:**
   ```rust
   use std::borrow::Cow;
   fn get_name(&self) -> Cow<str> { ... }
   ```

## CPU Profiling Best Practices

1. **Profile in release mode:**
   ```bash
   cargo bench --bench component_tree_benchmark
   # Not cargo test
   ```

2. **Run long enough:**
   - Minimum 10 seconds per benchmark
   - Criterion default: 5 seconds measurement + warmup

3. **Disable frequency scaling:**
   ```bash
   echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
   ```

4. **Close other applications:**
   - Minimize background noise
   - Run on dedicated system if possible

5. **Multiple iterations:**
   - Run profiling multiple times
   - Look for consistent patterns

## Automated Profiling

### Script for Continuous Profiling

```bash
#!/bin/bash
# scripts/profile_benchmarks.sh

set -e

echo "Building benchmarks..."
cargo bench --bench component_tree_benchmark --no-run

echo "Profiling tree retrieval..."
cargo flamegraph --bench component_tree_benchmark -- --bench "tree_retrieval"
mv flamegraph.svg flamegraph_tree_retrieval.svg

echo "Profiling formatters..."
cargo flamegraph --bench component_tree_benchmark -- --bench "output_formats"
mv flamegraph.svg flamegraph_formatters.svg

echo "Profiling filtering..."
cargo flamegraph --bench component_tree_benchmark -- --bench "filtering"
mv flamegraph.svg flamegraph_filtering.svg

echo "Done! Check flamegraph_*.svg files"
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Performance Profiling

on: [pull_request]

jobs:
  profile:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Install flamegraph
        run: cargo install flamegraph
      - name: Run profiling
        run: cargo flamegraph --bench component_tree_benchmark
      - name: Upload flamegraph
        uses: actions/upload-artifact@v2
        with:
          name: flamegraph
          path: flamegraph.svg
```

## Troubleshooting

### Issue: No symbols in flamegraph

**Solution:**
```toml
[profile.release]
debug = true
```

### Issue: Benchmarks take too long

**Solution:**
Reduce sample size:
```rust
group.sample_size(10);  // Instead of default 100
```

### Issue: Profiling crashes

**Solution:**
- Check stack size: `ulimit -s unlimited`
- Reduce test data size
- Profile smaller benchmarks individually

## References

- [Flamegraph Documentation](https://github.com/flamegraph-rs/flamegraph)
- [The Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [Criterion Profiling Guide](https://bheisler.github.io/criterion.rs/book/user_guide/profiling.html)
- [Valgrind Manual](https://valgrind.org/docs/manual/manual.html)

## Conclusion

Effective profiling identifies:
- Performance bottlenecks
- Optimization opportunities
- Resource consumption patterns
- Regression sources

Use this guide to systematically analyze and optimize the component tree implementation.
