#!/usr/bin/env python3
"""
Comprehensive Test Report Generator

Analyzes Robot Framework test results and generates detailed reports.
"""

import json
import sys
from pathlib import Path
from robot.api import ExecutionResult


def analyze_test_results(output_xml_path):
    """Analyze Robot Framework test results from output.xml."""
    result = ExecutionResult(output_xml_path)

    stats = {
        "total": result.statistics.total.all.total,
        "passed": result.statistics.total.all.passed,
        "failed": result.statistics.total.all.failed,
        "skipped": result.statistics.total.all.skipped,
        "pass_rate": 0.0,
        "empty_locator_tests": {"passed": 0, "failed": 0, "total": 0},
        "shell_tests": {"passed": 0, "failed": 0, "total": 0},
        "crash_tests": {"passed": 0, "failed": 0, "total": 0},
        "failures_by_type": {},
        "test_details": []
    }

    if stats["total"] > 0:
        stats["pass_rate"] = (stats["passed"] / stats["total"]) * 100

    # Analyze individual tests
    for suite in result.suite.suites:
        analyze_suite(suite, stats)

    return stats


def analyze_suite(suite, stats):
    """Recursively analyze test suites."""
    for test in suite.tests:
        test_name = test.name.lower()
        test_status = test.status

        # Track test details
        test_detail = {
            "name": test.name,
            "status": test_status,
            "message": test.message if test.message else "",
            "elapsed_time": test.elapsedtime / 1000.0  # Convert to seconds
        }
        stats["test_details"].append(test_detail)

        # Categorize tests
        if "empty locator" in test_name or "empty" in test_name:
            stats["empty_locator_tests"]["total"] += 1
            if test_status == "PASS":
                stats["empty_locator_tests"]["passed"] += 1
            else:
                stats["empty_locator_tests"]["failed"] += 1

        if "shell" in test_name or "get all shells" in test_name:
            stats["shell_tests"]["total"] += 1
            if test_status == "PASS":
                stats["shell_tests"]["passed"] += 1
            else:
                stats["shell_tests"]["failed"] += 1

        if "widget by text" in test_name or "find widget" in test_name:
            stats["crash_tests"]["total"] += 1
            if test_status == "PASS":
                stats["crash_tests"]["passed"] += 1
            else:
                stats["crash_tests"]["failed"] += 1

        # Track failure types
        if test_status == "FAIL" and test.message:
            msg_lower = test.message.lower()
            if "connection" in msg_lower:
                failure_type = "Connection Error"
            elif "not found" in msg_lower or "element not found" in msg_lower:
                failure_type = "Element Not Found"
            elif "timeout" in msg_lower:
                failure_type = "Timeout"
            elif "locator" in msg_lower and "empty" in msg_lower:
                failure_type = "Empty Locator"
            elif "signal" in msg_lower or "terminated" in msg_lower:
                failure_type = "Crash/Signal"
            else:
                failure_type = "Other"

            stats["failures_by_type"][failure_type] = \
                stats["failures_by_type"].get(failure_type, 0) + 1

    # Recursively analyze sub-suites
    for subsuite in suite.suites:
        analyze_suite(subsuite, stats)


def print_report(stats, title="Test Results"):
    """Print formatted test report."""
    print(f"\n{'='*80}")
    print(f"{title:^80}")
    print(f"{'='*80}\n")

    # Overall stats
    print("Overall Results:")
    print(f"  Total Tests:    {stats['total']}")
    print(f"  Passed:         {stats['passed']} ({stats['pass_rate']:.1f}%)")
    print(f"  Failed:         {stats['failed']}")
    print(f"  Skipped:        {stats['skipped']}")
    print()

    # Empty locator tests
    if stats["empty_locator_tests"]["total"] > 0:
        el_stats = stats["empty_locator_tests"]
        el_pass_rate = (el_stats["passed"] / el_stats["total"]) * 100 if el_stats["total"] > 0 else 0
        print("Empty Locator Tests:")
        print(f"  Total:          {el_stats['total']}")
        print(f"  Passed:         {el_stats['passed']} ({el_pass_rate:.1f}%)")
        print(f"  Failed:         {el_stats['failed']}")
        print()

    # Shell tests
    if stats["shell_tests"]["total"] > 0:
        sh_stats = stats["shell_tests"]
        sh_pass_rate = (sh_stats["passed"] / sh_stats["total"]) * 100 if sh_stats["total"] > 0 else 0
        print("Shell Tests (listShells validation):")
        print(f"  Total:          {sh_stats['total']}")
        print(f"  Passed:         {sh_stats['passed']} ({sh_pass_rate:.1f}%)")
        print(f"  Failed:         {sh_stats['failed']}")
        print()

    # Crash tests
    if stats["crash_tests"]["total"] > 0:
        cr_stats = stats["crash_tests"]
        cr_pass_rate = (cr_stats["passed"] / cr_stats["total"]) * 100 if cr_stats["total"] > 0 else 0
        print("Crash-Prone Tests (Find Widget By Text, etc.):")
        print(f"  Total:          {cr_stats['total']}")
        print(f"  Passed:         {cr_stats['passed']} ({cr_pass_rate:.1f}%)")
        print(f"  Failed:         {cr_stats['failed']}")
        print()

    # Failure breakdown
    if stats["failures_by_type"]:
        print("Failure Types:")
        for failure_type, count in sorted(stats["failures_by_type"].items(), key=lambda x: x[1], reverse=True):
            print(f"  {failure_type:30s} {count:>5d}")
        print()

    print(f"{'='*80}\n")


def main():
    if len(sys.argv) < 2:
        print("Usage: python comprehensive_test_report.py <output.xml> [title]")
        sys.exit(1)

    output_xml = sys.argv[1]
    title = sys.argv[2] if len(sys.argv) > 2 else "Test Results"

    if not Path(output_xml).exists():
        print(f"Error: File not found: {output_xml}")
        sys.exit(1)

    stats = analyze_test_results(output_xml)
    print_report(stats, title)

    # Save JSON report
    json_path = Path(output_xml).parent / "test_report.json"
    with open(json_path, 'w') as f:
        json.dump(stats, f, indent=2)

    print(f"Detailed JSON report saved to: {json_path}")

    return 0 if stats["failed"] == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
