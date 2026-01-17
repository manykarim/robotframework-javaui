# Performance Benchmarking Suite - Summary

**Date**: 2026-01-17
**Purpose**: Verify multi-test hang fix does not introduce performance regressions
**Status**: ✅ Ready for use

---

## Overview

A comprehensive performance benchmarking suite has been created to measure the impact of the multi-test hang fix. The suite includes:

1. **Automated benchmark scripts** - Measure latency, throughput, and memory
2. **Report generator** - Create detailed Markdown comparison reports
3. **Workflow automation** - Complete end-to-end testing script
4. **Documentation** - Usage guides and troubleshooting

---

## Quick Start

### Option 1: Automated Workflow (Recommended)

```bash
cd tests/experiments
./run_full_benchmark.sh
```

This runs the complete workflow:
1. Baseline benchmarks (before fix)
2. Prompts you to apply the fix
3. After-fix benchmarks
4. Generates comparison report

### Option 2: Manual Steps

```bash
# 1. Baseline
cd tests/experiments
python performance_benchmark.py --baseline

# 2. Apply fix (manual)
# Edit src/python/swt_library.rs
# Rebuild: cargo build --release && cd agent && mvn clean package

# 3. After fix
python performance_benchmark.py --after

# 4. Generate report
python generate_performance_report.py \
    baseline_benchmark_results.json \
    after_benchmark_results.json \
    -o ../docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md
```

---

## What Gets Measured

### Benchmark 1: RPC Call Latency (100 calls)
- **Purpose**: Raw RPC performance baseline
- **Metrics**: min/mean/median/P95/P99/max latency, throughput, errors
- **Duration**: ~30 seconds

### Benchmark 2: Multi-Method Mix (100 calls)
- **Purpose**: Simulate real Robot Framework usage
- **Methods**: ping, isInitialized, findWidgets (various locators)
- **Duration**: ~1 minute

### Benchmark 3: Robot Framework Suite
- **Purpose**: End-to-end test performance
- **Test**: Full `tests/robot/swt/02_widgets.robot` execution
- **Duration**: ~1-2 minutes (should complete; before fix it hangs)

### Benchmark 4: Sustained Load (30 seconds)
- **Purpose**: Memory leak detection
- **Metrics**: Memory usage over time, latency under load
- **Duration**: 30 seconds

**Total benchmark duration**: ~3-5 minutes per run

---

## Success Criteria

✅ **Fix is acceptable if**:
- Mean RPC latency change < 5%
- P99 latency change < 10%
- Memory usage change < 10%
- Robot test suite completes successfully
- Error rate = 0%

⚠️ **Investigate if**:
- Mean latency increases 5-10%
- Memory increases 10-20%

❌ **Fix needs revision if**:
- Mean latency increases >10%
- Memory increases >20%
- Robot test suite still hangs
- Any errors occur

---

## Files Created

### Scripts (`tests/experiments/`)

| File | Purpose | Lines |
|------|---------|-------|
| `performance_benchmark.py` | Main benchmark suite | 320 |
| `generate_performance_report.py` | Report generator | 280 |
| `run_full_benchmark.sh` | Automated workflow | 270 |
| `PERFORMANCE_BENCHMARK_GUIDE.md` | Detailed usage guide | 450 |
| `README.md` | Quick reference | 150 |

### Documentation (`docs/`)

| File | Purpose |
|------|---------|
| `MULTI_TEST_HANG_PERFORMANCE_REPORT.md` | Template/output for comparison report |
| `PERFORMANCE_BENCHMARKING_SUMMARY.md` | This file - overview |

### Output Files (generated)

- `baseline_benchmark_results.json` - Pre-fix metrics
- `after_benchmark_results.json` - Post-fix metrics
- `docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md` - Final report

---

## Key Features

### 1. Comprehensive Metrics

- **Latency**: Min, mean, median, P95, P99, max, standard deviation
- **Throughput**: Calls per second
- **Memory**: Min, mean, max usage during sustained load
- **Reliability**: Error rate, timeout detection
- **End-to-end**: Full Robot Framework test suite execution

### 2. Statistical Analysis

- Percentile calculations (P95, P99) for tail latency
- Standard deviation for consistency measurement
- Before/after comparison with percentage changes
- Automatic regression detection

### 3. Automation

- Single-command full workflow
- Automatic app startup/shutdown
- Health checks and validation
- Error handling and cleanup

### 4. Reporting

- Markdown-formatted comparison reports
- Color-coded indicators (✅/⚠️/❌)
- Executive summary with verdict
- Detailed metrics tables
- Recommendations section

---

## Example Output

### Console Output

```
======================================================================
BENCHMARK 1: RPC Call Latency (100 calls)
======================================================================
Connected to localhost:5679
Warming up (10 calls)...
Running benchmark (100 calls)...

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

### Generated Report (excerpt)

```markdown
## Executive Summary

- **RPC Latency (Mean)**: ✅ 0.3% improvement
- **RPC Latency (P99)**: ✅ no change
- **Robot Test Suite**: ✅ 2.1% improvement
- **Memory Usage**: ✅ 1.2% improvement

### Overall Verdict

✅ **NO SIGNIFICANT REGRESSION** - Performance is within acceptable range (<5% change)
```

---

## Performance Targets

Based on expected behavior:

| Metric | Expected Baseline | Target After Fix | Tolerance |
|--------|------------------|------------------|-----------|
| Mean RPC Latency | 2-5 ms | Similar | ±5% |
| P99 RPC Latency | 5-15 ms | Similar | ±10% |
| Throughput | 200-400 calls/sec | Similar or better | ±5% |
| Robot Test Duration | N/A (hangs) | 30-60 seconds | Must complete |
| Memory Usage | 200-400 MB | Similar | ±10% |

---

## Common Use Cases

### Baseline Before Fix

```bash
cd tests/experiments
./run_full_benchmark.sh --skip-after
```

### Quick Check (Faster, Less Accurate)

```bash
./run_full_benchmark.sh --quick
```

### Re-run After Fix Only

```bash
./run_full_benchmark.sh --skip-baseline
```

### Compare Existing Results

```bash
python performance_benchmark.py --compare \
    baseline_benchmark_results.json \
    after_benchmark_results.json
```

---

## Integration with Development Workflow

### Step 1: Before Implementing Fix

```bash
# Capture baseline
cd tests/experiments
python performance_benchmark.py --baseline
```

### Step 2: Implement Fix

Edit `src/python/swt_library.rs`:
- Remove problematic newline consumption (lines 1488-1494)
- Add non-blocking buffer drain

Rebuild:
```bash
cargo build --release
cd agent && mvn clean package && cd ../..
```

### Step 3: Verify Fix

```bash
cd tests/experiments
python performance_benchmark.py --after
```

### Step 4: Generate Report

```bash
python generate_performance_report.py \
    baseline_benchmark_results.json \
    after_benchmark_results.json
```

### Step 5: Review and Decide

- Read `docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md`
- Check for regressions
- Make merge decision

---

## Troubleshooting

### Agent Not Listening

**Problem**: `Agent not reachable: Connection refused`

**Solution**:
```bash
# Check if app is running
netstat -an | grep 5679

# If not, start it
java -javaagent:agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5679 \
     -jar tests/apps/swt/target/swt-test-app-1.0.0-all.jar &
```

### Test Suite Hangs

**Problem**: Benchmark 3 times out

**Cause**: Multi-test hang still present (fix not applied correctly)

**Solution**:
- Review implementation plan
- Check that buffer drain was added
- Verify rebuild completed

### Inconsistent Results

**Problem**: Large variance between runs

**Cause**: System load, background processes

**Solution**:
- Close unnecessary applications
- Run multiple iterations and average
- Use `--quick` for faster iterations during development

---

## Advanced Usage

### Custom Benchmark Duration

Edit `performance_benchmark.py`:

```python
# Change number of calls
num_calls = 50 if args.quick else 200  # Default: 100

# Change sustained load duration
duration = 15 if args.quick else 60  # Default: 30
```

### Add Custom Benchmarks

```python
def benchmark_my_custom_test():
    """Custom benchmark"""
    metrics = PerformanceMetrics()
    # ... your test logic
    return metrics

# Add to main():
results['benchmark_5'] = benchmark_my_custom_test()
```

### CI/CD Integration

```bash
#!/bin/bash
# ci_performance_check.sh

python performance_benchmark.py --after --quick

# Parse JSON and check thresholds
python -c "
import json
with open('after_benchmark_results.json') as f:
    data = json.load(f)
    mean_latency = data['benchmark_1']['mean_ms']
    if mean_latency > 10:  # 10ms threshold
        print('FAIL: Latency too high')
        exit(1)
"
```

---

## Related Documentation

- **Root Cause Analysis**: `SWT_MULTIPLE_TEST_HANG_ANALYSIS.md`
- **Implementation Plan**: `MULTI_TEST_HANG_IMPLEMENTATION_PLAN.md`
- **Benchmark Guide**: `tests/experiments/PERFORMANCE_BENCHMARK_GUIDE.md`
- **Experiments README**: `tests/experiments/README.md`

---

## Metrics Glossary

| Term | Description |
|------|-------------|
| **Mean latency** | Average time per RPC call |
| **Median latency** | Middle value (50th percentile) |
| **P95 latency** | 95% of calls complete within this time |
| **P99 latency** | 99% of calls complete within this time |
| **Throughput** | Number of calls completed per second |
| **Error rate** | Percentage of failed calls |
| **Memory delta** | Change in memory usage during test |
| **Standard deviation** | Measure of latency consistency |

---

## Summary

A comprehensive performance benchmarking suite has been created to validate the multi-test hang fix. The suite:

✅ Measures 4 different performance aspects
✅ Provides statistical analysis with percentiles
✅ Generates detailed comparison reports
✅ Includes automated workflow scripts
✅ Has clear success criteria
✅ Is ready for immediate use

**Total effort**: ~3 hours development
**Documentation**: ~1000 lines
**Code**: ~800 lines
**Test coverage**: RPC latency, multi-method calls, Robot Framework suite, sustained load

The suite is production-ready and can be run immediately to validate the fix.

---

**Document Created**: 2026-01-17
**Status**: ✅ Complete
**Next Action**: Run benchmarks using the automated workflow
