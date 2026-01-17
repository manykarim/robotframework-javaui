# Performance Experiments and Benchmarks

This directory contains diagnostic and performance testing tools for the robotframework-swing library.

## Contents

### Main Benchmark Suite

| File | Purpose |
|------|---------|
| `performance_benchmark.py` | Comprehensive performance benchmark suite |
| `generate_performance_report.py` | Generates Markdown comparison reports |
| `PERFORMANCE_BENCHMARK_GUIDE.md` | Complete usage documentation |

### Diagnostic Tools

| File | Purpose |
|------|---------|
| `multi_call_test.py` | Original diagnostic experiments for multi-test hang issue |

## Quick Start

### 1. Run Baseline Benchmarks

Before applying the multi-test hang fix:

```bash
# Start SWT test app
java -javaagent:../../agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5679 \
     -jar ../apps/swt/target/swt-test-app-1.0.0-all.jar &

sleep 3

# Run benchmarks
python performance_benchmark.py --baseline
```

### 2. Apply Fix and Re-benchmark

After implementing the fix:

```bash
# Rebuild
cargo build --release
cd ../../agent && mvn clean package && cd ../tests/experiments

# Restart app
pkill -f swt-test-app
sleep 2
java -javaagent:../../agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5679 \
     -jar ../apps/swt/target/swt-test-app-1.0.0-all.jar &

sleep 3

# Run benchmarks
python performance_benchmark.py --after
```

### 3. Generate Report

```bash
python generate_performance_report.py \
    baseline_benchmark_results.json \
    after_benchmark_results.json \
    -o ../../docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md
```

## Benchmarks Included

### Benchmark 1: RPC Call Latency
- 100 sequential `ping` calls
- Measures: min/mean/median/P95/P99/max latency
- Purpose: Raw RPC performance baseline

### Benchmark 2: Multi-Method Mix
- Mixed RPC calls (ping, isInitialized, findWidgets)
- 20 iterations × 5 methods = 100 calls
- Purpose: Real-world usage simulation

### Benchmark 3: Robot Framework Suite
- Full `tests/robot/swt/02_widgets.robot` execution
- Measures: duration, memory usage, success rate
- Purpose: End-to-end performance validation

### Benchmark 4: Sustained Load
- Continuous calls for 30 seconds
- Memory sampling every 10 calls
- Purpose: Memory leak detection

## Expected Results

| Metric | Baseline | Acceptable Range |
|--------|----------|------------------|
| Mean latency | 2-5 ms | ±5% |
| P99 latency | 5-15 ms | ±10% |
| Throughput | 200-400 calls/sec | ±5% |
| Memory | 200-400 MB | ±10% |

## Command Reference

### performance_benchmark.py

```bash
# Basic usage
python performance_benchmark.py [--baseline|--after] [OPTIONS]

# Options
--baseline          # Run as baseline (pre-fix)
--after            # Run after fix
--quick            # Quick mode (fewer iterations)
--verbose, -v      # Verbose output
--output, -o FILE  # Output JSON file
--compare B A      # Compare two results
```

### generate_performance_report.py

```bash
python generate_performance_report.py BASELINE AFTER [-o OUTPUT]

# Example
python generate_performance_report.py \
    baseline_benchmark_results.json \
    after_benchmark_results.json \
    -o ../../docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md
```

### multi_call_test.py

```bash
# Run all diagnostic experiments
python multi_call_test.py

# Individual experiments available:
# - Experiment 1: Basic connection (ping)
# - Experiment 2: Multiple pings
# - Experiment 3: Two findWidgets calls
# - Experiment 4: Different methods in sequence
# - Experiment 5: Rapid fire (no delays)
# - Experiment 6: Library-level testing
```

## Troubleshooting

### Agent not listening

```bash
# Check if app is running
netstat -an | grep 5679

# If not, start it
java -javaagent:../../agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5679 \
     -jar ../apps/swt/target/swt-test-app-1.0.0-all.jar &
```

### Test suite times out

- Indicates multi-test hang is still present
- Check if fix was applied correctly
- Review `MULTI_TEST_HANG_IMPLEMENTATION_PLAN.md`

### High memory usage

- This is normal for Python processes
- Focus on memory **delta** (growth), not absolute values
- Compare baseline vs after-fix

## Documentation

- **Usage Guide**: `PERFORMANCE_BENCHMARK_GUIDE.md` (detailed)
- **Implementation Plan**: `../../docs/MULTI_TEST_HANG_IMPLEMENTATION_PLAN.md`
- **Root Cause Analysis**: `../../docs/SWT_MULTIPLE_TEST_HANG_ANALYSIS.md`
- **Generated Report**: `../../docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md`

## Output Files

All benchmark runs generate JSON files:

- `baseline_benchmark_results.json` - Pre-fix results
- `after_benchmark_results.json` - Post-fix results
- Custom names supported via `-o` option

These files can be compared using:
```bash
python performance_benchmark.py --compare baseline.json after.json
```

Or generate a full Markdown report with `generate_performance_report.py`.

---

For detailed instructions, see `PERFORMANCE_BENCHMARK_GUIDE.md`.
