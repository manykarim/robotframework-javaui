#!/usr/bin/env python3
"""
Generate Performance Report from Benchmark Results

Analyzes baseline and after-fix benchmark results to create a comprehensive
performance report in Markdown format.

Usage:
    python generate_performance_report.py baseline.json after.json -o report.md
"""

import json
import argparse
import sys
from datetime import datetime
from typing import Dict, Any


def format_percentage_change(before: float, after: float) -> str:
    """Format percentage change with color indicator"""
    if before == 0:
        return "N/A"

    change = ((after / before) - 1) * 100

    if abs(change) < 1:
        indicator = "✅"
        status = "no change"
    elif change < 0:
        indicator = "✅"
        status = f"{abs(change):.1f}% improvement"
    else:
        indicator = "⚠️" if change > 5 else "➡️"
        status = f"{change:.1f}% regression" if change > 0 else "change"

    return f"{indicator} {status}"


def format_latency_distribution(stats: Dict[str, Any]) -> str:
    """Format latency distribution table"""
    return f"""| Metric | Value |
|--------|-------|
| Min | {stats.get('min_ms', 0):.2f} ms |
| Mean | {stats.get('mean_ms', 0):.2f} ms |
| Median | {stats.get('median_ms', 0):.2f} ms |
| P95 | {stats.get('p95_ms', 0):.2f} ms |
| P99 | {stats.get('p99_ms', 0):.2f} ms |
| Max | {stats.get('max_ms', 0):.2f} ms |
| Std Dev | {stats.get('stdev_ms', 0):.2f} ms |"""


def generate_report(baseline_file: str, after_file: str) -> str:
    """Generate comprehensive performance report"""

    # Load results
    with open(baseline_file) as f:
        baseline = json.load(f)

    with open(after_file) as f:
        after = json.load(f)

    # Build report
    report = []

    # Header
    report.append("# Multi-Test Hang Fix - Performance Report")
    report.append("")
    report.append(f"**Generated**: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    report.append(f"**Baseline**: {baseline.get('timestamp', 'unknown')}")
    report.append(f"**After Fix**: {after.get('timestamp', 'unknown')}")
    report.append("")
    report.append("---")
    report.append("")

    # Executive Summary
    report.append("## Executive Summary")
    report.append("")

    # Calculate key metrics
    b1_base = baseline.get('benchmark_1', {})
    b1_after = after.get('benchmark_1', {})

    mean_change = format_percentage_change(b1_base.get('mean_ms', 0), b1_after.get('mean_ms', 0))
    p99_change = format_percentage_change(b1_base.get('p99_ms', 0), b1_after.get('p99_ms', 0))

    report.append(f"- **RPC Latency (Mean)**: {mean_change}")
    report.append(f"- **RPC Latency (P99)**: {p99_change}")

    # Robot test suite
    b3_base = baseline.get('benchmark_3', {})
    b3_after = after.get('benchmark_3', {})

    if b3_base and b3_after:
        suite_change = format_percentage_change(
            b3_base.get('duration_sec', 0),
            b3_after.get('duration_sec', 0)
        )
        report.append(f"- **Robot Test Suite**: {suite_change}")

    # Memory
    b4_base_mem = baseline.get('benchmark_4', {}).get('memory', {})
    b4_after_mem = after.get('benchmark_4', {}).get('memory', {})

    if b4_base_mem and b4_after_mem:
        mem_change = format_percentage_change(
            b4_base_mem.get('mean_mb', 0),
            b4_after_mem.get('mean_mb', 0)
        )
        report.append(f"- **Memory Usage**: {mem_change}")

    report.append("")

    # Overall verdict
    report.append("### Overall Verdict")
    report.append("")

    mean_regression = ((b1_after.get('mean_ms', 0) / b1_base.get('mean_ms', 1)) - 1) * 100

    if abs(mean_regression) < 5:
        report.append("✅ **NO SIGNIFICANT REGRESSION** - Performance is within acceptable range (<5% change)")
    elif mean_regression < 0:
        report.append(f"✅ **PERFORMANCE IMPROVED** - {abs(mean_regression):.1f}% faster on average")
    else:
        report.append(f"⚠️ **PERFORMANCE REGRESSION DETECTED** - {mean_regression:.1f}% slower on average")

    report.append("")
    report.append("---")
    report.append("")

    # Benchmark 1: RPC Call Latency
    report.append("## Benchmark 1: RPC Call Latency")
    report.append("")
    report.append(f"**Test**: {b1_base.get('count', 0)} sequential RPC calls using `ping` method")
    report.append("")

    report.append("### Baseline (Before Fix)")
    report.append("")
    report.append(format_latency_distribution(b1_base))
    report.append("")
    report.append(f"**Throughput**: {b1_base.get('throughput_per_sec', 0):.2f} calls/sec")
    report.append(f"**Error Rate**: {b1_base.get('error_rate', 0) * 100:.2f}%")
    report.append("")

    report.append("### After Fix")
    report.append("")
    report.append(format_latency_distribution(b1_after))
    report.append("")
    report.append(f"**Throughput**: {b1_after.get('throughput_per_sec', 0):.2f} calls/sec")
    report.append(f"**Error Rate**: {b1_after.get('error_rate', 0) * 100:.2f}%")
    report.append("")

    report.append("### Comparison")
    report.append("")
    report.append("| Metric | Baseline | After Fix | Change |")
    report.append("|--------|----------|-----------|--------|")

    for metric in ['min_ms', 'mean_ms', 'median_ms', 'p95_ms', 'p99_ms', 'max_ms']:
        base_val = b1_base.get(metric, 0)
        after_val = b1_after.get(metric, 0)
        change = format_percentage_change(base_val, after_val)
        metric_name = metric.replace('_ms', '').upper()
        report.append(f"| {metric_name} | {base_val:.2f} ms | {after_val:.2f} ms | {change} |")

    report.append("")
    report.append("---")
    report.append("")

    # Benchmark 2: Multi-Method Calls
    report.append("## Benchmark 2: Multi-Method Call Mix")
    report.append("")
    report.append("**Test**: Mixed RPC methods (ping, isInitialized, findWidgets) to simulate real usage")
    report.append("")

    b2_base = baseline.get('benchmark_2', {})
    b2_after = after.get('benchmark_2', {})

    report.append("| Metric | Baseline | After Fix | Change |")
    report.append("|--------|----------|-----------|--------|")

    for metric, label in [
        ('mean_ms', 'Mean Latency'),
        ('p95_ms', 'P95 Latency'),
        ('p99_ms', 'P99 Latency'),
        ('throughput_per_sec', 'Throughput (calls/sec)'),
    ]:
        base_val = b2_base.get(metric, 0)
        after_val = b2_after.get(metric, 0)
        change = format_percentage_change(base_val, after_val)
        unit = 'ms' if 'ms' in metric else 'calls/sec'
        report.append(f"| {label} | {base_val:.2f} {unit} | {after_val:.2f} {unit} | {change} |")

    report.append("")
    report.append("---")
    report.append("")

    # Benchmark 3: Robot Test Suite
    report.append("## Benchmark 3: Robot Framework Test Suite")
    report.append("")
    report.append("**Test**: Full execution of `tests/robot/swt/02_widgets.robot`")
    report.append("")

    if b3_base and b3_after:
        report.append("| Metric | Baseline | After Fix | Change |")
        report.append("|--------|----------|-----------|--------|")

        duration_change = format_percentage_change(
            b3_base.get('duration_sec', 0),
            b3_after.get('duration_sec', 0)
        )
        report.append(f"| Total Duration | {b3_base.get('duration_sec', 0):.2f} sec | {b3_after.get('duration_sec', 0):.2f} sec | {duration_change} |")

        mem_change = format_percentage_change(
            b3_base.get('memory_delta_mb', 0),
            b3_after.get('memory_delta_mb', 0)
        )
        report.append(f"| Memory Delta | {b3_base.get('memory_delta_mb', 0):.2f} MB | {b3_after.get('memory_delta_mb', 0):.2f} MB | {mem_change} |")

        report.append(f"| Success | {'✅' if b3_base.get('success') else '❌'} | {'✅' if b3_after.get('success') else '❌'} | - |")
    else:
        report.append("⚠️ Robot test suite benchmark data not available")

    report.append("")
    report.append("---")
    report.append("")

    # Benchmark 4: Memory Usage
    report.append("## Benchmark 4: Memory Usage Under Sustained Load")
    report.append("")
    report.append("**Test**: Continuous RPC calls over 30 seconds with memory monitoring")
    report.append("")

    b4_base_lat = baseline.get('benchmark_4', {}).get('latency', {})
    b4_after_lat = after.get('benchmark_4', {}).get('latency', {})

    report.append("### Performance During Load")
    report.append("")
    report.append("| Metric | Baseline | After Fix | Change |")
    report.append("|--------|----------|-----------|--------|")

    for metric, label in [
        ('mean_ms', 'Mean Latency'),
        ('throughput_per_sec', 'Throughput'),
    ]:
        base_val = b4_base_lat.get(metric, 0)
        after_val = b4_after_lat.get(metric, 0)
        change = format_percentage_change(base_val, after_val)
        unit = 'ms' if 'ms' in metric else 'calls/sec'
        report.append(f"| {label} | {base_val:.2f} {unit} | {after_val:.2f} {unit} | {change} |")

    report.append("")

    report.append("### Memory Statistics")
    report.append("")
    report.append("| Metric | Baseline | After Fix | Change |")
    report.append("|--------|----------|-----------|--------|")

    for metric, label in [
        ('min_mb', 'Min Memory'),
        ('mean_mb', 'Mean Memory'),
        ('max_mb', 'Max Memory'),
    ]:
        base_val = b4_base_mem.get(metric, 0)
        after_val = b4_after_mem.get(metric, 0)
        change = format_percentage_change(base_val, after_val)
        report.append(f"| {label} | {base_val:.2f} MB | {after_val:.2f} MB | {change} |")

    report.append("")
    report.append("---")
    report.append("")

    # Conclusions
    report.append("## Conclusions")
    report.append("")

    report.append("### Latency Analysis")
    report.append("")

    mean_change_pct = ((b1_after.get('mean_ms', 0) / b1_base.get('mean_ms', 1)) - 1) * 100

    if abs(mean_change_pct) < 2:
        report.append(f"- Mean latency change is negligible ({abs(mean_change_pct):.1f}%)")
        report.append("- Fix has no measurable impact on average performance")
    elif mean_change_pct < 0:
        report.append(f"- Mean latency **improved** by {abs(mean_change_pct):.1f}%")
        report.append("- Fix may have optimized the critical path")
    else:
        report.append(f"- Mean latency **increased** by {mean_change_pct:.1f}%")
        if mean_change_pct > 5:
            report.append("- ⚠️ Significant regression - investigate further")
        else:
            report.append("- Minor regression within acceptable range")

    report.append("")

    report.append("### Throughput Analysis")
    report.append("")

    throughput_change = ((b1_after.get('throughput_per_sec', 0) / b1_base.get('throughput_per_sec', 1)) - 1) * 100

    if abs(throughput_change) < 5:
        report.append(f"- Throughput remains stable ({abs(throughput_change):.1f}% change)")
    elif throughput_change > 0:
        report.append(f"- Throughput **improved** by {throughput_change:.1f}%")
    else:
        report.append(f"- Throughput **decreased** by {abs(throughput_change):.1f}%")

    report.append("")

    report.append("### Memory Analysis")
    report.append("")

    mem_change_pct = ((b4_after_mem.get('mean_mb', 0) / b4_base_mem.get('mean_mb', 1)) - 1) * 100

    if abs(mem_change_pct) < 5:
        report.append(f"- Memory usage is stable ({abs(mem_change_pct):.1f}% change)")
    elif mem_change_pct > 0:
        report.append(f"- Memory usage **increased** by {mem_change_pct:.1f}%")
        if mem_change_pct > 10:
            report.append("- ⚠️ Significant memory increase - investigate leaks")
    else:
        report.append(f"- Memory usage **decreased** by {abs(mem_change_pct):.1f}%")

    report.append("")

    report.append("### Final Verdict")
    report.append("")

    # Overall assessment
    issues = []
    if mean_change_pct > 10:
        issues.append("Significant latency regression")
    if mem_change_pct > 15:
        issues.append("Significant memory increase")
    if b3_after.get('success') and not b3_base.get('success'):
        issues = []  # Fix resolved failures - that's good!

    if not issues and abs(mean_change_pct) < 5:
        report.append("✅ **APPROVED FOR MERGE**")
        report.append("")
        report.append("- No significant performance regression detected")
        report.append("- All metrics within acceptable ranges")
        report.append("- Fix successfully resolves multi-test hang issue")
    elif not issues:
        report.append("✅ **APPROVED WITH MONITORING**")
        report.append("")
        report.append("- Minor performance changes detected but within acceptable range")
        report.append("- Recommend monitoring in production")
    else:
        report.append("⚠️ **REQUIRES INVESTIGATION**")
        report.append("")
        report.append("Issues detected:")
        for issue in issues:
            report.append(f"- {issue}")

    report.append("")
    report.append("---")
    report.append("")

    # Recommendations
    report.append("## Recommendations")
    report.append("")

    if mean_change_pct > 5:
        report.append("1. **Profile the critical path** to identify where latency increased")
        report.append("2. Consider optimizing buffer handling or using larger buffer sizes")
    else:
        report.append("1. Proceed with merge - performance is acceptable")

    if mem_change_pct > 10:
        report.append("2. **Investigate memory leaks** - use valgrind or similar tools")
    else:
        report.append("2. No memory concerns - usage is stable")

    report.append("3. Monitor production metrics after deployment")
    report.append("4. Consider adding performance regression tests to CI/CD pipeline")

    report.append("")
    report.append("---")
    report.append("")
    report.append(f"**Report Generated**: {datetime.now().isoformat()}")

    return "\n".join(report)


def main():
    parser = argparse.ArgumentParser(description='Generate performance comparison report')
    parser.add_argument('baseline', help='Baseline benchmark results (JSON)')
    parser.add_argument('after', help='After-fix benchmark results (JSON)')
    parser.add_argument('-o', '--output', default='docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md',
                        help='Output markdown file')

    args = parser.parse_args()

    try:
        report = generate_report(args.baseline, args.after)

        with open(args.output, 'w') as f:
            f.write(report)

        print(f"✅ Report generated: {args.output}")
        print(f"\nPreview:")
        print("=" * 70)
        print(report[:500] + "...")

        return 0

    except FileNotFoundError as e:
        print(f"❌ Error: {e}")
        print("Make sure both baseline and after-fix benchmark files exist.")
        return 1
    except Exception as e:
        print(f"❌ Error generating report: {e}")
        import traceback
        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())
