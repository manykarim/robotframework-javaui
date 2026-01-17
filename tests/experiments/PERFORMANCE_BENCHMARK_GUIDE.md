# Performance Benchmark Guide

This guide explains how to use the performance benchmarking suite to verify that the multi-test hang fix does not introduce performance regressions.

## Overview

The benchmark suite consists of three components:

1. **performance_benchmark.py** - Runs comprehensive performance tests
2. **generate_performance_report.py** - Generates comparison reports
3. **multi_call_test.py** - Original experiment script (still useful for manual testing)

## Quick Start

### 1. Capture Baseline (Before Fix)

Before implementing the fix, run:

```bash
# Start the SWT test application
cd tests/apps/swt
java -javaagent:../../../agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5679 \
     -jar target/swt-test-app-1.0.0-all.jar &

# Wait for app to start (3 seconds)
sleep 3

# Run baseline benchmarks
cd ../../..
python tests/experiments/performance_benchmark.py --baseline -o benchmark_results.json
```

This will create `baseline_benchmark_results.json` with pre-fix metrics.

### 2. Apply the Fix

Implement the buffer drain fix in `src/python/swt_library.rs` as described in `MULTI_TEST_HANG_IMPLEMENTATION_PLAN.md`.

### 3. Capture After-Fix Results

After implementing the fix:

```bash
# Restart the SWT test application
pkill -f swt-test-app
sleep 2

java -javaagent:agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5679 \
     -jar tests/apps/swt/target/swt-test-app-1.0.0-all.jar &

sleep 3

# Run after-fix benchmarks
python tests/experiments/performance_benchmark.py --after -o benchmark_results.json
```

This will create `after_benchmark_results.json` with post-fix metrics.

### 4. Generate Comparison Report

```bash
python tests/experiments/generate_performance_report.py \
    baseline_benchmark_results.json \
    after_benchmark_results.json \
    -o docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md
```

This creates a comprehensive Markdown report in `docs/`.

### 5. Review Results

```bash
cat docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md
```

Look for:
- ✅ No significant regression (<5% change in mean latency)
- ✅ Stable memory usage
- ✅ All tests passing

## Benchmark Details

### Benchmark 1: RPC Call Latency

**Purpose**: Measure raw RPC performance with sequential calls

**Metrics**:
- Min/Mean/Median/Max latency
- P95 and P99 latency (tail performance)
- Standard deviation (consistency)
- Throughput (calls per second)
- Error rate

**What to Look For**:
- Mean latency should not increase by more than 5%
- P99 latency should remain stable (indicates no tail latency issues)
- Error rate should be 0%

**Example Output**:
```
Results:
  Total calls:     100
  Min latency:     2.45 ms
  Average latency: 3.21 ms
  Median latency:  3.18 ms
  Max latency:     8.42 ms
  P95 latency:     4.12 ms
  P99 latency:     5.67 ms
  Std deviation:   0.89 ms
  Throughput:      311.53 calls/sec
  Error count:     0
```

### Benchmark 2: Multi-Method Call Mix

**Purpose**: Simulate real Robot Framework usage with different RPC methods

**Methods Tested**:
- `ping` - Simple health check
- `isInitialized` - State query
- `findWidgets` - Widget search (class, name locators)

**What to Look For**:
- Similar or better performance compared to simple ping
- No errors or timeouts
- Consistent throughput

**Example Output**:
```
Results:
  Total calls:     100
  Average latency: 8.45 ms
  P95 latency:     12.34 ms
  P99 latency:     18.21 ms
  Throughput:      118.34 calls/sec
  Error count:     0
```

### Benchmark 3: Robot Framework Test Suite

**Purpose**: Measure end-to-end performance of real test execution

**Test**: `tests/robot/swt/02_widgets.robot`

**What to Look For**:
- **Before fix**: Likely hangs on second test
- **After fix**: Should complete successfully
- Duration should be similar or better
- Memory delta should be reasonable (<100 MB)

**Example Output**:
```
Results:
  Status:          SUCCESS
  Total time:      45.67 seconds
  Tests run:       25
  Memory before:   256.34 MB
  Memory after:    312.45 MB
  Memory delta:    56.11 MB
```

### Benchmark 4: Memory Usage Under Sustained Load

**Purpose**: Detect memory leaks or excessive memory growth

**What to Look For**:
- Stable memory usage over time
- No significant growth trend
- Mean memory within expected range

**Example Output**:
```
Performance Results:
  Total calls:     2847
  Average latency: 3.34 ms
  Throughput:      94.90 calls/sec

Memory Results:
  Min memory:      245.23 MB
  Max memory:      256.89 MB
  Average memory:  251.45 MB
  Samples taken:   284
```

## Command-Line Options

### performance_benchmark.py

```bash
# Full options
python performance_benchmark.py [OPTIONS]

Options:
  --baseline          Run as baseline (saves to baseline_*.json)
  --after            Run after fix (saves to after_*.json)
  --quick            Quick mode (50 calls instead of 100, 15s instead of 30s)
  --verbose, -v      Verbose output with progress
  --output, -o FILE  Output JSON file (default: benchmark_results.json)
  --compare B A      Compare two result files
```

**Examples**:

```bash
# Quick baseline run with verbose output
python performance_benchmark.py --baseline --quick --verbose

# Full after-fix run
python performance_benchmark.py --after

# Compare results directly
python performance_benchmark.py --compare baseline_results.json after_results.json
```

### generate_performance_report.py

```bash
# Usage
python generate_performance_report.py BASELINE AFTER [-o OUTPUT]

# Example
python generate_performance_report.py \
    baseline_benchmark_results.json \
    after_benchmark_results.json \
    -o docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md
```

## Interpreting Results

### Performance Change Indicators

In the generated report, you'll see indicators:

- ✅ **Green checkmark**: No regression or improvement
- ➡️ **Right arrow**: Minor change (1-5%)
- ⚠️ **Warning**: Potential regression (>5%)

### Acceptable Ranges

| Metric | Acceptable Change |
|--------|-------------------|
| Mean latency | ±5% |
| P99 latency | ±10% |
| Throughput | ±5% |
| Memory usage | ±10% |

### When to Be Concerned

🚨 **Red flags**:
- Mean latency increases by >10%
- P99 latency increases by >20%
- Memory increases by >20%
- Error rate >0%
- Robot test suite fails

⚠️ **Yellow flags**:
- Mean latency increases by 5-10%
- Memory increases by 10-20%
- Throughput decreases by >5%

✅ **Green flags**:
- All changes <5%
- Robot test suite passes
- Error rate = 0%

## Troubleshooting

### Agent Not Reachable

**Error**: `Agent not reachable: [Errno 111] Connection refused`

**Solution**:
```bash
# Start the SWT test app
java -javaagent:agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5679 \
     -jar tests/apps/swt/target/swt-test-app-1.0.0-all.jar &

# Wait for startup
sleep 3

# Verify it's listening
netstat -an | grep 5679
```

### Robot Test Times Out

**Error**: Test suite timed out after 10 minutes

**Solution**:
- Check if multi-test hang is still present
- Increase timeout in benchmark script
- Run tests individually to isolate issue

### Memory Benchmark Shows High Usage

**Cause**: Python process memory includes everything (libraries, test framework, etc.)

**Solution**:
- Focus on memory **delta** (growth over time), not absolute values
- Compare before/after, not absolute numbers
- Watch for continuous growth (indicates leak)

### Inconsistent Results

**Cause**: System load, other processes, CPU throttling

**Solution**:
- Run benchmarks multiple times
- Close other applications
- Use `--quick` mode for faster iterations
- Average results from 3-5 runs

## Advanced Usage

### Running Multiple Iterations

```bash
# Run 5 iterations and average results
for i in {1..5}; do
    echo "=== Iteration $i ==="
    python performance_benchmark.py --after -o after_run_$i.json
done

# Manually average the results or write a script to aggregate
```

### Profiling Specific Operations

Modify `performance_benchmark.py` to test specific scenarios:

```python
# Add custom benchmark
def benchmark_specific_operation():
    """Test specific operation that concerns you"""
    metrics = PerformanceMetrics()
    # ... custom test logic
    return metrics
```

### Continuous Integration

Add to your CI pipeline:

```bash
#!/bin/bash
# ci_performance_check.sh

# Run benchmarks
python tests/experiments/performance_benchmark.py --after --quick

# Check if results are within acceptable range
# (You'd implement this based on your threshold policy)
```

## Example Workflow

Here's a complete example workflow:

```bash
# 1. Build everything
cd agent && mvn clean package && cd ..
cd tests/apps/swt && mvn clean package && cd ../../..

# 2. Start app
java -javaagent:agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5679 \
     -jar tests/apps/swt/target/swt-test-app-1.0.0-all.jar &
APP_PID=$!
sleep 3

# 3. Baseline benchmarks
echo "Running baseline benchmarks..."
python tests/experiments/performance_benchmark.py --baseline

# 4. Kill app
kill $APP_PID
sleep 2

# 5. Apply fix (manual step - edit src/python/swt_library.rs)
echo "Apply your fix now, then press Enter..."
read

# 6. Rebuild
cargo build --release
cd agent && mvn clean package && cd ..

# 7. Restart app
java -javaagent:agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5679 \
     -jar tests/apps/swt/target/swt-test-app-1.0.0-all.jar &
APP_PID=$!
sleep 3

# 8. After benchmarks
echo "Running after-fix benchmarks..."
python tests/experiments/performance_benchmark.py --after

# 9. Generate report
python tests/experiments/generate_performance_report.py \
    baseline_benchmark_results.json \
    after_benchmark_results.json \
    -o docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md

# 10. Cleanup
kill $APP_PID

# 11. View report
cat docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md
```

## Reference Data

Expected baseline performance (pre-fix, unoptimized):

| Metric | Expected Range |
|--------|---------------|
| Mean RPC latency | 2-5 ms |
| P99 RPC latency | 5-15 ms |
| Throughput | 200-400 calls/sec |
| Robot test suite | 30-60 seconds |
| Memory usage | 200-400 MB |

Post-fix should be similar or better.

---

**Note**: These benchmarks focus on **regression detection**, not absolute performance. The goal is to ensure the fix doesn't make things worse, not to achieve specific performance targets.
