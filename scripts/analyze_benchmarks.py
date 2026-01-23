#!/usr/bin/env python3
"""
Analyze Criterion benchmark results and generate performance report.
"""

import json
import os
import sys
from pathlib import Path
from typing import Dict, List, Tuple
import argparse


def parse_criterion_estimates(estimates_file: Path) -> Dict:
    """Parse Criterion estimates.json file."""
    with open(estimates_file) as f:
        return json.load(f)


def format_time(nanos: float) -> str:
    """Format nanoseconds to human-readable time."""
    if nanos < 1000:
        return f"{nanos:.2f}ns"
    elif nanos < 1_000_000:
        return f"{nanos/1000:.2f}μs"
    elif nanos < 1_000_000_000:
        return f"{nanos/1_000_000:.2f}ms"
    else:
        return f"{nanos/1_000_000_000:.2f}s"


def check_target(name: str, time_ms: float) -> Tuple[str, str]:
    """Check if benchmark meets performance target."""
    targets = {
        "depth1": (10, "Depth 1 queries"),
        "depth5_1000": (50, "Depth 5 with 1000 components"),
        "1000_components": (100, "1000 component tree"),
        "format": (10, "Format conversion"),
        "filter": (20, "Filtering operations"),
        "statistics": (5, "Statistics calculation"),
    }
    
    for key, (target_ms, description) in targets.items():
        if key in name.lower():
            if time_ms <= target_ms:
                return "✓ PASS", f"{description}: {time_ms:.2f}ms ≤ {target_ms}ms"
            else:
                return "✗ FAIL", f"{description}: {time_ms:.2f}ms > {target_ms}ms"
    
    return "- INFO", f"{time_ms:.2f}ms"


def analyze_benchmark_group(group_path: Path) -> List[Dict]:
    """Analyze all benchmarks in a group."""
    results = []
    
    for bench_dir in group_path.iterdir():
        if not bench_dir.is_dir():
            continue
            
        estimates_file = bench_dir / "base" / "estimates.json"
        if not estimates_file.exists():
            continue
        
        estimates = parse_criterion_estimates(estimates_file)
        mean_nanos = estimates.get("mean", {}).get("point_estimate", 0)
        mean_ms = mean_nanos / 1_000_000
        
        status, message = check_target(bench_dir.name, mean_ms)
        
        results.append({
            "name": bench_dir.name,
            "mean_nanos": mean_nanos,
            "mean_ms": mean_ms,
            "status": status,
            "message": message,
        })
    
    return results


def generate_report(criterion_dir: Path, output_file: Path = None):
    """Generate performance analysis report."""
    if not criterion_dir.exists():
        print(f"Error: Criterion directory not found: {criterion_dir}")
        sys.exit(1)
    
    # Collect all benchmark groups
    all_results = {}
    
    for group_dir in criterion_dir.iterdir():
        if not group_dir.is_dir():
            continue
        
        # Check if this is a benchmark group (has subdirectories with base/estimates.json)
        has_benchmarks = any(
            (subdir / "base" / "estimates.json").exists()
            for subdir in group_dir.iterdir()
            if subdir.is_dir()
        )
        
        if has_benchmarks:
            results = analyze_benchmark_group(group_dir)
            if results:
                all_results[group_dir.name] = results
    
    # Generate report
    report_lines = [
        "# Performance Benchmark Analysis Report",
        "",
        f"Generated from: {criterion_dir}",
        "",
        "## Summary",
        "",
    ]
    
    total_tests = sum(len(results) for results in all_results.values())
    passed_tests = sum(
        1 for results in all_results.values()
        for r in results if r["status"] == "✓ PASS"
    )
    failed_tests = sum(
        1 for results in all_results.values()
        for r in results if r["status"] == "✗ FAIL"
    )
    
    report_lines.extend([
        f"- Total benchmarks: {total_tests}",
        f"- Passed targets: {passed_tests}",
        f"- Failed targets: {failed_tests}",
        f"- Pass rate: {passed_tests/max(total_tests,1)*100:.1f}%",
        "",
        "## Performance Targets Status",
        "",
    ])
    
    # Group results by status
    for group_name, results in sorted(all_results.items()):
        report_lines.append(f"### {group_name}")
        report_lines.append("")
        
        # Sort by status (failures first)
        results_sorted = sorted(results, key=lambda x: (x["status"] != "✗ FAIL", x["mean_ms"]))
        
        for result in results_sorted:
            report_lines.append(f"- {result['status']} `{result['name']}`: {result['message']}")
        
        report_lines.append("")
    
    # Detailed results table
    report_lines.extend([
        "## Detailed Results",
        "",
        "| Benchmark | Mean Time | Status |",
        "|-----------|-----------|--------|",
    ])
    
    for group_name, results in sorted(all_results.items()):
        for result in sorted(results, key=lambda x: x["mean_ms"]):
            time_str = format_time(result["mean_nanos"])
            report_lines.append(
                f"| {group_name}/{result['name']} | {time_str} | {result['status']} |"
            )
    
    report_lines.append("")
    
    # Write report
    report_text = "\n".join(report_lines)
    
    if output_file:
        output_file.parent.mkdir(parents=True, exist_ok=True)
        with open(output_file, "w") as f:
            f.write(report_text)
        print(f"Report written to: {output_file}")
    else:
        print(report_text)
    
    return passed_tests == total_tests


def main():
    parser = argparse.ArgumentParser(description="Analyze Criterion benchmark results")
    parser.add_argument(
        "--criterion-dir",
        type=Path,
        default=Path("target/criterion"),
        help="Path to Criterion results directory",
    )
    parser.add_argument(
        "--output",
        type=Path,
        help="Output file path (default: print to stdout)",
    )
    parser.add_argument(
        "--fail-on-regression",
        action="store_true",
        help="Exit with non-zero status if any targets failed",
    )
    
    args = parser.parse_args()
    
    all_passed = generate_report(args.criterion_dir, args.output)
    
    if args.fail_on_regression and not all_passed:
        sys.exit(1)


if __name__ == "__main__":
    main()
