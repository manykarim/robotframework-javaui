#!/usr/bin/env python3
"""
Performance Benchmark Suite for Multi-Test Hang Fix

This suite measures:
1. RPC call latency (min/avg/max/p95/p99)
2. Multi-test overhead
3. Memory usage
4. Throughput (calls per second)

Usage:
    python performance_benchmark.py --baseline  # Run before fix
    python performance_benchmark.py --after     # Run after fix
    python performance_benchmark.py --compare baseline.json after.json
"""

import socket
import json
import time
import sys
import os
import argparse
import statistics
from typing import List, Dict, Any, Tuple
import psutil
import subprocess
from datetime import datetime

# Configuration
HOST = "localhost"
PORT = 5679
AGENT_JAR = os.path.abspath("agent/target/robotframework-swing-agent-1.0.0-all.jar")
APP_JAR = os.path.abspath("tests/apps/swt/target/swt-test-app-1.0.0-all.jar")

class PerformanceMetrics:
    """Container for performance measurements"""

    def __init__(self):
        self.latencies: List[float] = []
        self.errors: List[str] = []
        self.memory_samples: List[int] = []
        self.start_time: float = 0
        self.end_time: float = 0

    def add_latency(self, latency_ms: float):
        """Record a latency measurement in milliseconds"""
        self.latencies.append(latency_ms)

    def add_error(self, error: str):
        """Record an error"""
        self.errors.append(error)

    def add_memory_sample(self, bytes_used: int):
        """Record memory usage in bytes"""
        self.memory_samples.append(bytes_used)

    def calculate_statistics(self) -> Dict[str, Any]:
        """Calculate statistical summary"""
        if not self.latencies:
            return {"error": "No measurements recorded"}

        sorted_latencies = sorted(self.latencies)
        n = len(sorted_latencies)

        return {
            "count": n,
            "min_ms": min(sorted_latencies),
            "max_ms": max(sorted_latencies),
            "mean_ms": statistics.mean(sorted_latencies),
            "median_ms": statistics.median(sorted_latencies),
            "stdev_ms": statistics.stdev(sorted_latencies) if n > 1 else 0,
            "p95_ms": sorted_latencies[int(n * 0.95)] if n > 1 else sorted_latencies[0],
            "p99_ms": sorted_latencies[int(n * 0.99)] if n > 1 else sorted_latencies[0],
            "throughput_per_sec": n / (self.end_time - self.start_time) if self.end_time > self.start_time else 0,
            "error_count": len(self.errors),
            "error_rate": len(self.errors) / n if n > 0 else 0,
        }

    def calculate_memory_stats(self) -> Dict[str, Any]:
        """Calculate memory usage statistics"""
        if not self.memory_samples:
            return {"error": "No memory samples"}

        return {
            "min_mb": min(self.memory_samples) / 1024 / 1024,
            "max_mb": max(self.memory_samples) / 1024 / 1024,
            "mean_mb": statistics.mean(self.memory_samples) / 1024 / 1024,
            "samples": len(self.memory_samples),
        }


def send_rpc_request_timed(sock, method: str, params: Dict = None, request_id: int = 1) -> Tuple[float, Any]:
    """
    Send RPC request and measure latency.
    Returns: (latency_ms, response)
    """
    if params is None:
        params = {}

    request = {
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": request_id
    }

    request_str = json.dumps(request) + "\n"

    # Start timing
    start_time = time.perf_counter()

    try:
        sock.sendall(request_str.encode('utf-8'))

        # Read response byte by byte, tracking JSON depth
        response_bytes = bytearray()
        depth = 0
        in_string = False
        escape_next = False
        started = False

        sock.settimeout(30.0)  # 30 second timeout

        while True:
            b = sock.recv(1)
            if not b:
                end_time = time.perf_counter()
                return (end_time - start_time) * 1000, {"error": "Connection closed"}

            c = b[0]
            char = chr(c)

            # Skip leading whitespace
            if not started and char in '\n\r \t':
                continue

            response_bytes.append(c)

            if escape_next:
                escape_next = False
                continue
            if char == '\\' and in_string:
                escape_next = True
                continue
            if char == '"':
                in_string = not in_string
            if not in_string:
                if char == '{':
                    depth += 1
                    started = True
                elif char == '}':
                    depth -= 1
                    if started and depth == 0:
                        break

        # End timing
        end_time = time.perf_counter()
        latency_ms = (end_time - start_time) * 1000

        response_str = response_bytes.decode('utf-8')
        response = json.loads(response_str)

        return latency_ms, response

    except socket.timeout:
        end_time = time.perf_counter()
        return (end_time - start_time) * 1000, {"error": "Timeout"}
    except Exception as e:
        end_time = time.perf_counter()
        return (end_time - start_time) * 1000, {"error": str(e)}


def benchmark_rpc_latency(num_calls: int = 100, verbose: bool = False) -> PerformanceMetrics:
    """
    Benchmark 1: RPC Call Latency
    Measure time for sequential RPC calls
    """
    print(f"\n{'='*70}")
    print(f"BENCHMARK 1: RPC Call Latency ({num_calls} calls)")
    print(f"{'='*70}")

    metrics = PerformanceMetrics()

    # Connect once
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.connect((HOST, PORT))
        print(f"Connected to {HOST}:{PORT}")

        # Warmup
        print("Warming up (10 calls)...")
        for i in range(10):
            send_rpc_request_timed(sock, "ping", {}, i)

        # Actual benchmark
        print(f"Running benchmark ({num_calls} calls)...")
        metrics.start_time = time.time()

        for i in range(num_calls):
            latency_ms, response = send_rpc_request_timed(sock, "ping", {}, i)
            metrics.add_latency(latency_ms)

            if "error" in response:
                metrics.add_error(response["error"])

            if verbose and (i + 1) % 10 == 0:
                print(f"  Progress: {i+1}/{num_calls} calls")

        metrics.end_time = time.time()

    finally:
        sock.close()

    # Print results
    stats = metrics.calculate_statistics()
    print(f"\nResults:")
    print(f"  Total calls:     {stats['count']}")
    print(f"  Min latency:     {stats['min_ms']:.2f} ms")
    print(f"  Average latency: {stats['mean_ms']:.2f} ms")
    print(f"  Median latency:  {stats['median_ms']:.2f} ms")
    print(f"  Max latency:     {stats['max_ms']:.2f} ms")
    print(f"  P95 latency:     {stats['p95_ms']:.2f} ms")
    print(f"  P99 latency:     {stats['p99_ms']:.2f} ms")
    print(f"  Std deviation:   {stats['stdev_ms']:.2f} ms")
    print(f"  Throughput:      {stats['throughput_per_sec']:.2f} calls/sec")
    print(f"  Error count:     {stats['error_count']}")

    return metrics


def benchmark_multi_method_calls(verbose: bool = False) -> PerformanceMetrics:
    """
    Benchmark 2: Multi-Method Call Mix
    Test with different RPC methods to simulate real usage
    """
    print(f"\n{'='*70}")
    print(f"BENCHMARK 2: Multi-Method Call Mix")
    print(f"{'='*70}")

    metrics = PerformanceMetrics()

    methods = [
        ("ping", {}),
        ("isInitialized", {}),
        ("findWidgets", {"locatorType": "class", "value": "Button"}),
        ("findWidgets", {"locatorType": "name", "value": "buttonSubmit"}),
        ("findWidgets", {"locatorType": "class", "value": "Text"}),
    ]

    num_iterations = 20  # Each iteration runs all 5 methods
    total_calls = num_iterations * len(methods)

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.connect((HOST, PORT))
        print(f"Connected to {HOST}:{PORT}")
        print(f"Running {num_iterations} iterations x {len(methods)} methods = {total_calls} calls...")

        metrics.start_time = time.time()

        for iteration in range(num_iterations):
            for method, params in methods:
                latency_ms, response = send_rpc_request_timed(sock, method, params, iteration)
                metrics.add_latency(latency_ms)

                if "error" in response:
                    metrics.add_error(f"{method}: {response['error']}")

            if verbose:
                print(f"  Iteration {iteration + 1}/{num_iterations} complete")

        metrics.end_time = time.time()

    finally:
        sock.close()

    # Print results
    stats = metrics.calculate_statistics()
    print(f"\nResults:")
    print(f"  Total calls:     {stats['count']}")
    print(f"  Average latency: {stats['mean_ms']:.2f} ms")
    print(f"  P95 latency:     {stats['p95_ms']:.2f} ms")
    print(f"  P99 latency:     {stats['p99_ms']:.2f} ms")
    print(f"  Throughput:      {stats['throughput_per_sec']:.2f} calls/sec")
    print(f"  Error count:     {stats['error_count']}")

    return metrics


def benchmark_robot_test_suite(verbose: bool = False) -> Dict[str, Any]:
    """
    Benchmark 3: Robot Framework Test Suite Execution
    Measure execution time for 02_widgets.robot
    """
    print(f"\n{'='*70}")
    print(f"BENCHMARK 3: Robot Framework Test Suite")
    print(f"{'='*70}")

    test_file = "tests/robot/swt/02_widgets.robot"
    output_dir = "/tmp/perf_benchmark_robot"

    # Ensure output directory exists
    os.makedirs(output_dir, exist_ok=True)

    print(f"Running: {test_file}")
    print(f"Output: {output_dir}")

    # Get memory before
    process = psutil.Process()
    mem_before = process.memory_info().rss

    # Run Robot Framework
    start_time = time.time()

    cmd = [
        "xvfb-run", "-a",
        "uv", "run", "robot",
        "--outputdir", output_dir,
        "--loglevel", "INFO",
        test_file
    ]

    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=600)
        end_time = time.time()

        # Get memory after
        mem_after = process.memory_info().rss

        duration = end_time - start_time

        # Parse output for test count
        output_lines = result.stdout.split('\n')
        test_count = 0
        pass_count = 0
        fail_count = 0

        for line in output_lines:
            if 'test total' in line.lower():
                # Extract numbers from Robot output
                parts = line.split()
                for i, part in enumerate(parts):
                    if part.isdigit():
                        test_count = int(part)
                        break

        success = result.returncode == 0

        print(f"\nResults:")
        print(f"  Status:          {'SUCCESS' if success else 'FAILED'}")
        print(f"  Total time:      {duration:.2f} seconds")
        print(f"  Tests run:       {test_count}")
        print(f"  Memory before:   {mem_before / 1024 / 1024:.2f} MB")
        print(f"  Memory after:    {mem_after / 1024 / 1024:.2f} MB")
        print(f"  Memory delta:    {(mem_after - mem_before) / 1024 / 1024:.2f} MB")

        if verbose:
            print(f"\nStdout preview:")
            print(result.stdout[:500])

        return {
            "success": success,
            "duration_sec": duration,
            "test_count": test_count,
            "memory_before_mb": mem_before / 1024 / 1024,
            "memory_after_mb": mem_after / 1024 / 1024,
            "memory_delta_mb": (mem_after - mem_before) / 1024 / 1024,
            "return_code": result.returncode,
        }

    except subprocess.TimeoutExpired:
        print("ERROR: Test suite timed out after 10 minutes")
        return {
            "success": False,
            "error": "Timeout",
            "duration_sec": 600,
        }


def benchmark_memory_usage(duration_sec: int = 30, verbose: bool = False) -> PerformanceMetrics:
    """
    Benchmark 4: Memory Usage During Sustained Load
    Monitor memory while making continuous RPC calls
    """
    print(f"\n{'='*70}")
    print(f"BENCHMARK 4: Memory Usage (sustained load for {duration_sec}s)")
    print(f"{'='*70}")

    metrics = PerformanceMetrics()
    process = psutil.Process()

    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    try:
        sock.connect((HOST, PORT))
        print(f"Connected to {HOST}:{PORT}")

        metrics.start_time = time.time()
        call_count = 0

        print(f"Making RPC calls for {duration_sec} seconds...")

        while (time.time() - metrics.start_time) < duration_sec:
            # Make RPC call
            latency_ms, response = send_rpc_request_timed(sock, "ping", {}, call_count)
            metrics.add_latency(latency_ms)
            call_count += 1

            # Sample memory every 10 calls
            if call_count % 10 == 0:
                mem_info = process.memory_info()
                metrics.add_memory_sample(mem_info.rss)

                if verbose and call_count % 100 == 0:
                    print(f"  Calls: {call_count}, Memory: {mem_info.rss / 1024 / 1024:.2f} MB")

        metrics.end_time = time.time()

    finally:
        sock.close()

    # Print results
    stats = metrics.calculate_statistics()
    mem_stats = metrics.calculate_memory_stats()

    print(f"\nPerformance Results:")
    print(f"  Total calls:     {stats['count']}")
    print(f"  Average latency: {stats['mean_ms']:.2f} ms")
    print(f"  Throughput:      {stats['throughput_per_sec']:.2f} calls/sec")

    print(f"\nMemory Results:")
    print(f"  Min memory:      {mem_stats['min_mb']:.2f} MB")
    print(f"  Max memory:      {mem_stats['max_mb']:.2f} MB")
    print(f"  Average memory:  {mem_stats['mean_mb']:.2f} MB")
    print(f"  Samples taken:   {mem_stats['samples']}")

    return metrics


def save_results(results: Dict[str, Any], filename: str):
    """Save benchmark results to JSON file"""
    results['timestamp'] = datetime.now().isoformat()
    results['hostname'] = socket.gethostname()

    with open(filename, 'w') as f:
        json.dump(results, f, indent=2)

    print(f"\nResults saved to: {filename}")


def compare_results(baseline_file: str, after_file: str):
    """Compare two benchmark result files"""
    print(f"\n{'='*70}")
    print(f"COMPARING RESULTS")
    print(f"{'='*70}")

    with open(baseline_file) as f:
        baseline = json.load(f)

    with open(after_file) as f:
        after = json.load(f)

    print(f"Baseline: {baseline.get('timestamp', 'unknown')}")
    print(f"After:    {after.get('timestamp', 'unknown')}")

    # Compare latency
    if 'benchmark_1' in baseline and 'benchmark_1' in after:
        b1_base = baseline['benchmark_1']
        b1_after = after['benchmark_1']

        print(f"\nBenchmark 1 - RPC Latency:")
        print(f"  Mean:   {b1_base['mean_ms']:.2f} ms -> {b1_after['mean_ms']:.2f} ms ({((b1_after['mean_ms'] / b1_base['mean_ms']) - 1) * 100:+.1f}%)")
        print(f"  P95:    {b1_base['p95_ms']:.2f} ms -> {b1_after['p95_ms']:.2f} ms ({((b1_after['p95_ms'] / b1_base['p95_ms']) - 1) * 100:+.1f}%)")
        print(f"  P99:    {b1_base['p99_ms']:.2f} ms -> {b1_after['p99_ms']:.2f} ms ({((b1_after['p99_ms'] / b1_base['p99_ms']) - 1) * 100:+.1f}%)")

    # Compare Robot test suite
    if 'benchmark_3' in baseline and 'benchmark_3' in after:
        b3_base = baseline['benchmark_3']
        b3_after = after['benchmark_3']

        print(f"\nBenchmark 3 - Robot Test Suite:")
        print(f"  Duration: {b3_base['duration_sec']:.2f}s -> {b3_after['duration_sec']:.2f}s ({((b3_after['duration_sec'] / b3_base['duration_sec']) - 1) * 100:+.1f}%)")
        print(f"  Memory:   {b3_base.get('memory_delta_mb', 0):.2f} MB -> {b3_after.get('memory_delta_mb', 0):.2f} MB")

    # Overall verdict
    print(f"\n{'='*70}")
    print(f"VERDICT:")
    if 'benchmark_1' in baseline and 'benchmark_1' in after:
        latency_change = ((b1_after['mean_ms'] / b1_base['mean_ms']) - 1) * 100
        if abs(latency_change) < 5:
            print(f"  ✅ No significant latency regression (<5% change)")
        elif latency_change < 0:
            print(f"  ✅ Performance IMPROVED by {abs(latency_change):.1f}%")
        else:
            print(f"  ⚠️  Performance DEGRADED by {latency_change:.1f}%")


def main():
    parser = argparse.ArgumentParser(description='Performance benchmark suite for multi-test hang fix')
    parser.add_argument('--baseline', action='store_true', help='Run baseline benchmarks (before fix)')
    parser.add_argument('--after', action='store_true', help='Run benchmarks after fix')
    parser.add_argument('--compare', nargs=2, metavar=('BASELINE', 'AFTER'), help='Compare two result files')
    parser.add_argument('--quick', action='store_true', help='Quick mode (fewer iterations)')
    parser.add_argument('--verbose', '-v', action='store_true', help='Verbose output')
    parser.add_argument('--output', '-o', default='benchmark_results.json', help='Output file for results')

    args = parser.parse_args()

    if args.compare:
        compare_results(args.compare[0], args.compare[1])
        return 0

    # Check if agent is running
    print("Checking if agent is listening...")
    try:
        test_sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        test_sock.settimeout(2)
        test_sock.connect((HOST, PORT))
        test_sock.close()
        print("✅ Agent is listening!")
    except Exception as e:
        print(f"❌ Agent not reachable: {e}")
        print("Please start the SWT test app first.")
        return 1

    # Run benchmarks
    num_calls = 50 if args.quick else 100

    results = {}

    # Benchmark 1: RPC Latency
    metrics_1 = benchmark_rpc_latency(num_calls, args.verbose)
    results['benchmark_1'] = metrics_1.calculate_statistics()

    # Benchmark 2: Multi-method calls
    metrics_2 = benchmark_multi_method_calls(args.verbose)
    results['benchmark_2'] = metrics_2.calculate_statistics()

    # Benchmark 3: Robot test suite
    results['benchmark_3'] = benchmark_robot_test_suite(args.verbose)

    # Benchmark 4: Memory usage
    duration = 15 if args.quick else 30
    metrics_4 = benchmark_memory_usage(duration, args.verbose)
    results['benchmark_4'] = {
        'latency': metrics_4.calculate_statistics(),
        'memory': metrics_4.calculate_memory_stats(),
    }

    # Save results
    if args.baseline:
        filename = "baseline_" + args.output
    elif args.after:
        filename = "after_" + args.output
    else:
        filename = args.output

    save_results(results, filename)

    print(f"\n{'='*70}")
    print(f"BENCHMARK COMPLETE")
    print(f"{'='*70}")
    print(f"Results saved to: {filename}")
    print(f"\nTo compare with baseline:")
    print(f"  python performance_benchmark.py --compare baseline_{args.output} after_{args.output}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
