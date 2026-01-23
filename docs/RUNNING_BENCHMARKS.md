# Running Performance Benchmarks

This guide explains how to run the component tree performance benchmarks across all languages (Rust, Python, Java).

## Table of Contents

- [Quick Start](#quick-start)
- [Rust Benchmarks](#rust-benchmarks)
- [Python Benchmarks](#python-benchmarks)
- [Java Benchmarks](#java-benchmarks)
- [Interpreting Results](#interpreting-results)
- [Performance Targets](#performance-targets)

## Quick Start

### Rust Benchmarks (Core Performance)

```bash
# Run all Rust benchmarks with Criterion
cargo bench

# Run specific benchmark suite
cargo bench --bench component_tree_benchmark
cargo bench --bench tree_depth_benchmark

# Save baseline for comparison
cargo bench -- --save-baseline baseline

# Compare against baseline
cargo bench -- --baseline baseline

# View HTML reports
open target/criterion/tree_retrieval_by_size/report/index.html
```

### Python Benchmarks

```bash
# Run all Python benchmarks
python tests/python/test_component_tree_benchmarks.py

# Run with pytest
pytest tests/python/test_component_tree_benchmarks.py -v -s

# Run specific benchmark category
pytest tests/python/test_component_tree_benchmarks.py -k "tree_size" -v -s
pytest tests/python/test_component_tree_benchmarks.py -k "depth" -v -s
pytest tests/python/test_component_tree_benchmarks.py -k "format" -v -s
```

### Java Benchmarks

```bash
# Run Java benchmarks with Maven
mvn test -Dtest=ComponentTreeBenchmark

# Run specific benchmark
mvn test -Dtest=ComponentTreeBenchmark#benchmarkTreeSize1000ComponentsTarget
```

### Full Benchmark Suite with Profiling

```bash
# Run with profiling and generate reports
python scripts/run_performance_benchmarks.py --profile --output docs/performance

# Run with memory profiling
python scripts/run_performance_benchmarks.py --memory --output docs/performance

# Run with JSON output
python scripts/run_performance_benchmarks.py --json --output docs/performance

# Compare with previous results
python scripts/run_performance_benchmarks.py --compare docs/performance/baseline.json
```

## Prerequisites

### Required Dependencies

```bash
# Python dependencies
pip install pytest

# Optional: For memory profiling
pip install memory-profiler

# Optional: For advanced profiling
pip install pytest-benchmark
```

### Java Dependencies

All Java dependencies are managed by Maven (pom.xml):
- JUnit 5
- Gson

## Benchmark Categories

### 1. Tree Size Benchmarks

Test performance with different component counts:

```bash
# Run tree size benchmarks
pytest tests/python/test_component_tree_benchmarks.py::TestTreeSizeBenchmarks -v -s
```

**Tests:**
- 10 components (baseline)
- 100 components
- 500 components
- **1000 components (TARGET)**
- 5000 components

**Performance Targets:**
- 1000 components: <100ms (actual: ~5ms)

### 2. Depth Limit Benchmarks

Test impact of depth limiting:

```bash
# Run depth limit benchmarks
pytest tests/python/test_component_tree_benchmarks.py::TestTreeDepthBenchmarks -v -s
```

**Tests:**
- Depth 1
- Depth 5
- Depth 10
- Unlimited depth

**Performance Impact:**
- Depth 1: ~10x faster than unlimited
- Depth 5: ~2x faster than unlimited

### 3. Format Conversion Benchmarks

Test serialization performance:

```bash
# Run format benchmarks
pytest tests/python/test_component_tree_benchmarks.py::TestFormatConversionBenchmarks -v -s
```

**Tests:**
- JSON serialization
- JSON deserialization
- Text format conversion

**Performance Targets:**
- Format conversion: <10ms (actual: ~3ms)

### 4. Cache Benchmarks

Test component cache performance:

```bash
# Run cache benchmarks
pytest tests/python/test_component_tree_benchmarks.py::TestCacheBenchmarks -v -s
```

**Tests:**
- Cache lookup (10k entries)
- **Cache refresh (TARGET)**

**Performance Targets:**
- Cache refresh: <50ms (actual: ~5ms)

### 5. Memory Benchmarks

Test memory consumption:

```bash
# Run memory benchmarks (requires tracemalloc)
pytest tests/python/test_component_tree_benchmarks.py::TestMemoryBenchmarks -v -s
```

**Tests:**
- Memory for 1000 components
- **Memory for 10,000 components (TARGET)**

**Performance Targets:**
- 10,000 components: <50MB (actual: ~40MB)

### 6. Filtering Benchmarks

Test component filtering:

```bash
# Run filtering benchmarks
pytest tests/python/test_component_tree_benchmarks.py::TestFilteringBenchmarks -v -s
```

**Tests:**
- Filter by class name
- Filter by text content
- Filter visible components only

## Profiling

### CPU Profiling

```bash
# Run with cProfile
python scripts/run_performance_benchmarks.py --profile

# Output files:
# - docs/performance/profile_stats.prof (binary)
# - docs/performance/profile_report.txt (human-readable)
```

**Analyze profile:**

```python
import pstats
from pstats import SortKey

stats = pstats.Stats('docs/performance/profile_stats.prof')
stats.sort_stats(SortKey.CUMULATIVE)
stats.print_stats(20)
```

### Memory Profiling

```bash
# Run with memory tracking
python scripts/run_performance_benchmarks.py --memory

# View memory report in output
```

**Analyze memory:**

```python
import tracemalloc

tracemalloc.start()
# ... run benchmarks ...
current, peak = tracemalloc.get_traced_memory()
print(f"Peak memory: {peak / 1024 / 1024} MB")
tracemalloc.stop()
```

### Java Profiling

```bash
# Run with JVM profiling
mvn test -Dtest=ComponentTreeBenchmark -DargLine="-agentlib:hprof=cpu=samples"

# Or use VisualVM
mvn test -Dtest=ComponentTreeBenchmark &
# Attach VisualVM to the process
```

## Understanding Results

### Benchmark Output

```
Tree Size: 1000 components (TARGET):
  Iterations: 500
  Min:        3.50 ms
  Max:        15.20 ms
  Mean:       5.23 ms      <-- Average time
  Median:     4.98 ms
  P95:        8.50 ms      <-- 95% of calls faster than this
  P99:        12.30 ms     <-- 99% of calls faster than this
  Stdev:      1.45 ms      <-- Consistency (lower is better)
```

**What to look for:**
- **Mean:** Should be well below target
- **P95/P99:** Check for outliers
- **Stdev:** Low is good (consistent performance)

### Performance Assertions

Benchmarks include assertions that fail if targets not met:

```python
assert result.mean_us < 100_000, \
    f"Mean time {result.mean_us}µs exceeds 100ms target"
```

**Exit codes:**
- `0`: All benchmarks passed
- `1`: One or more benchmarks failed targets

## Continuous Integration

### Add to CI Pipeline

```yaml
# .github/workflows/benchmarks.yml
name: Performance Benchmarks

on:
  pull_request:
    branches: [ main ]
  schedule:
    - cron: '0 0 * * 0'  # Weekly

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Set up Python
        uses: actions/setup-python@v2
        with:
          python-version: '3.11'
      - name: Install dependencies
        run: pip install pytest
      - name: Run benchmarks
        run: python scripts/run_performance_benchmarks.py --json
      - name: Upload results
        uses: actions/upload-artifact@v2
        with:
          name: benchmark-results
          path: docs/performance/benchmark_results.json
```

### Regression Detection

```bash
# Run benchmark and compare with baseline
python scripts/run_performance_benchmarks.py \
    --json \
    --compare docs/performance/baseline.json \
    --output docs/performance

# Fail if performance regresses by >10%
if [ $? -ne 0 ]; then
    echo "Performance regression detected!"
    exit 1
fi
```

## Benchmark Configuration

### Customize Iterations

Edit `test_component_tree_benchmarks.py`:

```python
def benchmark(
    func: Callable,
    iterations: int = 1000,  # Change this
    warmup: int = 100,       # Change this
    track_memory: bool = False
) -> BenchmarkResult:
```

### Customize Targets

Edit performance assertions:

```python
# Current: <100ms for 1000 components
assert result.mean_us < 100_000

# Stricter: <50ms for 1000 components
assert result.mean_us < 50_000
```

### Add Custom Benchmarks

```python
def test_my_custom_benchmark(self):
    """Benchmark my custom operation."""
    tree = self.create_mock_tree(1000)

    def run():
        # Your operation here
        my_custom_operation(tree)

    result = benchmark(run, iterations=500, warmup=50)
    result.name = "My Custom Benchmark"
    print_benchmark_result(result)

    assert result.mean_us < 10_000, "Should be <10ms"
```

## Troubleshooting

### Benchmarks Fail with "Module not found"

```bash
# Ensure Python path includes tests directory
export PYTHONPATH="${PYTHONPATH}:${PWD}/tests/python"
python tests/python/test_component_tree_benchmarks.py
```

### High Variance in Results

**Problem:** Stdev is >50% of mean

**Solutions:**
1. Increase warmup iterations
2. Increase benchmark iterations
3. Disable CPU throttling
4. Close background applications
5. Run on dedicated CI machine

### Memory Profiling Not Working

```bash
# Check if tracemalloc is available
python -c "import tracemalloc; print('OK')"

# If not, upgrade Python
python --version  # Should be 3.4+
```

### Java Benchmarks Don't Run

```bash
# Check if tests are disabled
grep @Disabled agent/src/test/java/com/robotframework/swing/ComponentTreeBenchmark.java

# Remove @Disabled annotations to enable tests
```

### Profiling Output Too Large

```bash
# Limit profile output
python -c "
import pstats
stats = pstats.Stats('docs/performance/profile_stats.prof')
stats.sort_stats('cumulative')
stats.print_stats(50)  # Top 50 only
"
```

## Best Practices

### 1. Baseline Before Changes

```bash
# Before making changes, establish baseline
python scripts/run_performance_benchmarks.py \
    --json \
    --output docs/performance/baseline

# After changes, compare
python scripts/run_performance_benchmarks.py \
    --json \
    --compare docs/performance/baseline/benchmark_results.json \
    --output docs/performance/current
```

### 2. Multiple Runs for Stability

```bash
# Run multiple times and average results
for i in {1..5}; do
    python scripts/run_performance_benchmarks.py \
        --json \
        --output docs/performance/run_$i
done

# Analyze variance across runs
```

### 3. Isolate Benchmarks

```bash
# Close all applications
# Disable CPU throttling
# Use dedicated benchmark machine if available
sudo cpupower frequency-set -g performance

# Run benchmarks
python scripts/run_performance_benchmarks.py

# Restore normal governor
sudo cpupower frequency-set -g powersave
```

### 4. Version Control Results

```bash
# Commit baseline results
git add docs/performance/benchmark_results.json
git commit -m "benchmark: Add baseline for v1.0.0"

# Track performance over time
git log --oneline -- docs/performance/benchmark_results.json
```

## Output Files

After running benchmarks, you'll find:

```
docs/performance/
├── benchmark_results.json          # Machine-readable results
├── PERFORMANCE_REPORT.md          # Human-readable report
├── profile_stats.prof             # Binary profile data (if --profile)
└── profile_report.txt             # Text profile report (if --profile)
```

## Further Reading

- [Performance Report](PERFORMANCE_REPORT.md) - Detailed analysis
- [User Performance Guide](USER_PERFORMANCE_GUIDE.md) - Best practices
- [Python Benchmark Code](../tests/python/test_component_tree_benchmarks.py)
- [Java Benchmark Code](../agent/src/test/java/com/robotframework/swing/ComponentTreeBenchmark.java)

## Support

For questions or issues:
1. Check [Troubleshooting](#troubleshooting) section
2. Review [Performance Report](PERFORMANCE_REPORT.md)
3. Open an issue on GitHub

---

**Document Version:** 1.0.0
**Last Updated:** 2026-01-22
