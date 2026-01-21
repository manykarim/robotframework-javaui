"""Performance benchmarks for assertion retry mechanism.

This module benchmarks:
- Retry mechanism overhead
- Formatter performance
- Security evaluator performance
- Memory usage patterns
"""

import pytest
import time
import statistics
from typing import List, Callable

# Skip if dependencies not available
try:
    from assertionengine import AssertionOperator
    from JavaGui.assertions import (
        with_retry_assertion,
        numeric_assertion_with_retry,
        state_assertion_with_retry,
        ElementState,
    )
    from JavaGui.assertions.formatters import (
        normalize_spaces,
        strip,
        lowercase,
        uppercase,
        strip_html_tags,
        apply_formatters,
    )
    from JavaGui.assertions.security import (
        SecureExpressionEvaluator,
        secure_evaluate,
        validate_expression,
    )
    AVAILABLE = True
except ImportError:
    AVAILABLE = False


def benchmark(func: Callable, iterations: int = 1000, warmup: int = 100) -> dict:
    """Run a benchmark and return statistics.

    Args:
        func: Function to benchmark (should take no arguments).
        iterations: Number of timed iterations.
        warmup: Number of warmup iterations.

    Returns:
        Dict with min, max, mean, median, stdev times in microseconds.
    """
    # Warmup
    for _ in range(warmup):
        func()

    # Timed runs
    times: List[float] = []
    for _ in range(iterations):
        start = time.perf_counter()
        func()
        end = time.perf_counter()
        times.append((end - start) * 1_000_000)  # Convert to microseconds

    return {
        "iterations": iterations,
        "min_us": min(times),
        "max_us": max(times),
        "mean_us": statistics.mean(times),
        "median_us": statistics.median(times),
        "stdev_us": statistics.stdev(times) if len(times) > 1 else 0,
    }


def print_benchmark_result(name: str, result: dict):
    """Print benchmark results in a formatted way."""
    print(f"\n{name}:")
    print(f"  Iterations: {result['iterations']}")
    print(f"  Min:    {result['min_us']:.2f} µs")
    print(f"  Max:    {result['max_us']:.2f} µs")
    print(f"  Mean:   {result['mean_us']:.2f} µs")
    print(f"  Median: {result['median_us']:.2f} µs")
    print(f"  Stdev:  {result['stdev_us']:.2f} µs")


@pytest.mark.skipif(not AVAILABLE, reason="Dependencies not available")
class TestRetryMechanismBenchmarks:
    """Benchmarks for the retry assertion mechanism."""

    def test_immediate_success_overhead(self):
        """Benchmark overhead when assertion passes immediately."""
        value = "expected"

        def get_value():
            return value

        def run():
            with_retry_assertion(
                get_value,
                AssertionOperator["=="],
                "expected",
                timeout=1.0,
                interval=0.1,
            )

        result = benchmark(run, iterations=500, warmup=50)
        print_benchmark_result("Immediate Success Assertion", result)

        # Should be fast - less than 500µs mean
        assert result["mean_us"] < 500, f"Mean time {result['mean_us']}µs exceeds 500µs threshold"

    def test_no_operator_overhead(self):
        """Benchmark overhead when no operator is specified (value return only)."""
        value = "test_value"

        def get_value():
            return value

        def run():
            with_retry_assertion(
                get_value,
                None,
                None,
                timeout=1.0,
                interval=0.1,
            )

        result = benchmark(run, iterations=1000, warmup=100)
        print_benchmark_result("No Operator (Value Return)", result)

        # Should be very fast - less than 100µs mean
        assert result["mean_us"] < 100, f"Mean time {result['mean_us']}µs exceeds 100µs threshold"

    def test_contains_operator_overhead(self):
        """Benchmark contains operator assertion."""
        value = "hello world"

        def get_value():
            return value

        def run():
            with_retry_assertion(
                get_value,
                AssertionOperator["*="],  # contains
                "world",
                timeout=1.0,
                interval=0.1,
            )

        result = benchmark(run, iterations=500, warmup=50)
        print_benchmark_result("Contains Operator Assertion", result)

        assert result["mean_us"] < 500, f"Mean time {result['mean_us']}µs exceeds 500µs threshold"

    def test_numeric_assertion_overhead(self):
        """Benchmark numeric assertion."""
        value = 100

        def get_value():
            return value

        def run():
            numeric_assertion_with_retry(
                get_value,
                AssertionOperator[">"],
                50,
                timeout=1.0,
                interval=0.1,
            )

        result = benchmark(run, iterations=500, warmup=50)
        print_benchmark_result("Numeric Assertion (>)", result)

        assert result["mean_us"] < 500, f"Mean time {result['mean_us']}µs exceeds 500µs threshold"


@pytest.mark.skipif(not AVAILABLE, reason="Dependencies not available")
class TestFormatterBenchmarks:
    """Benchmarks for formatter functions."""

    def test_normalize_spaces_benchmark(self):
        """Benchmark normalize_spaces formatter."""
        text = "  hello   world   with   multiple   spaces  "

        def run():
            normalize_spaces(text)

        result = benchmark(run, iterations=10000, warmup=1000)
        print_benchmark_result("normalize_spaces", result)

        # Should be very fast - less than 10µs mean
        assert result["mean_us"] < 10, f"Mean time {result['mean_us']}µs exceeds 10µs threshold"

    def test_strip_html_tags_benchmark(self):
        """Benchmark strip_html_tags formatter."""
        html = "<div><span class='test'>Hello</span> <b>World</b></div>"

        def run():
            strip_html_tags(html)

        result = benchmark(run, iterations=10000, warmup=1000)
        print_benchmark_result("strip_html_tags", result)

        # Should be reasonably fast - less than 50µs mean
        assert result["mean_us"] < 50, f"Mean time {result['mean_us']}µs exceeds 50µs threshold"

    def test_chained_formatters_benchmark(self):
        """Benchmark applying multiple formatters in chain."""
        text = "  <b>HELLO</b>   <i>WORLD</i>  "

        def run():
            # apply_formatters expects string names, not functions
            apply_formatters(text, ["strip_html_tags", "normalize_spaces", "lowercase"])

        result = benchmark(run, iterations=5000, warmup=500)
        print_benchmark_result("Chained formatters (3 formatters)", result)

        # Should be reasonably fast - less than 100µs mean
        assert result["mean_us"] < 100, f"Mean time {result['mean_us']}µs exceeds 100µs threshold"


@pytest.mark.skipif(not AVAILABLE, reason="Dependencies not available")
class TestSecurityEvaluatorBenchmarks:
    """Benchmarks for the secure expression evaluator."""

    def test_simple_equality_benchmark(self):
        """Benchmark simple equality expression."""
        evaluator = SecureExpressionEvaluator()

        def run():
            evaluator.evaluate("value == 'expected'", {"value": "expected"})

        result = benchmark(run, iterations=1000, warmup=100)
        print_benchmark_result("Simple equality expression", result)

        # Should complete in reasonable time - less than 100µs mean
        assert result["mean_us"] < 100, f"Mean time {result['mean_us']}µs exceeds 100µs threshold"

    def test_string_method_benchmark(self):
        """Benchmark expression with string methods."""
        evaluator = SecureExpressionEvaluator()

        def run():
            evaluator.evaluate(
                "value.startswith('Hello') and len(value) > 5",
                {"value": "Hello World"},
            )

        result = benchmark(run, iterations=1000, warmup=100)
        print_benchmark_result("String method expression", result)

        assert result["mean_us"] < 150, f"Mean time {result['mean_us']}µs exceeds 150µs threshold"

    def test_regex_expression_benchmark(self):
        """Benchmark expression with regex."""
        evaluator = SecureExpressionEvaluator()

        def run():
            evaluator.evaluate(
                "re.search(r'\\d+', value) is not None",
                {"value": "test123value"},
            )

        result = benchmark(run, iterations=1000, warmup=100)
        print_benchmark_result("Regex expression", result)

        # Regex is slower - allow up to 200µs
        assert result["mean_us"] < 200, f"Mean time {result['mean_us']}µs exceeds 200µs threshold"

    def test_validation_only_benchmark(self):
        """Benchmark expression validation (AST parsing)."""
        def run():
            validate_expression("value == 'test' and len(value) > 3")

        result = benchmark(run, iterations=1000, warmup=100)
        print_benchmark_result("Expression validation (AST)", result)

        # AST parsing should be fast - less than 50µs
        assert result["mean_us"] < 50, f"Mean time {result['mean_us']}µs exceeds 50µs threshold"

    def test_dangerous_expression_detection_benchmark(self):
        """Benchmark detection of dangerous expressions."""
        def run():
            # Should quickly detect and return errors
            errors = validate_expression("__import__('os').system('ls')")
            assert len(errors) > 0

        result = benchmark(run, iterations=1000, warmup=100)
        print_benchmark_result("Dangerous expression detection", result)

        # Should be fast to detect - less than 50µs
        assert result["mean_us"] < 50, f"Mean time {result['mean_us']}µs exceeds 50µs threshold"


@pytest.mark.skipif(not AVAILABLE, reason="Dependencies not available")
class TestElementStateBenchmarks:
    """Benchmarks for ElementState operations."""

    def test_state_from_string_benchmark(self):
        """Benchmark ElementState.from_string."""
        def run():
            ElementState.from_string("visible")

        result = benchmark(run, iterations=10000, warmup=1000)
        print_benchmark_result("ElementState.from_string", result)

        # Should be very fast - less than 5µs
        assert result["mean_us"] < 5, f"Mean time {result['mean_us']}µs exceeds 5µs threshold"

    def test_state_from_strings_benchmark(self):
        """Benchmark ElementState.from_strings."""
        states = ["visible", "enabled", "focused"]

        def run():
            ElementState.from_strings(states)

        result = benchmark(run, iterations=5000, warmup=500)
        print_benchmark_result("ElementState.from_strings (3 states)", result)

        # Should be fast - less than 20µs
        assert result["mean_us"] < 20, f"Mean time {result['mean_us']}µs exceeds 20µs threshold"

    def test_state_to_list_benchmark(self):
        """Benchmark ElementState.to_list."""
        state = ElementState.visible | ElementState.enabled | ElementState.focused

        def run():
            state.to_list()

        result = benchmark(run, iterations=10000, warmup=1000)
        print_benchmark_result("ElementState.to_list (3 states)", result)

        # Should be reasonably fast - less than 50µs
        assert result["mean_us"] < 50, f"Mean time {result['mean_us']}µs exceeds 50µs threshold"


@pytest.mark.skipif(not AVAILABLE, reason="Dependencies not available")
class TestMemoryBenchmarks:
    """Memory-related benchmarks."""

    def test_evaluator_reuse_vs_create(self):
        """Compare performance of reusing vs creating evaluators."""
        # Reusing evaluator
        evaluator = SecureExpressionEvaluator()

        def run_reuse():
            evaluator.evaluate("value == 'test'", {"value": "test"})

        # Creating new evaluator each time
        def run_create():
            e = SecureExpressionEvaluator()
            e.evaluate("value == 'test'", {"value": "test"})

        result_reuse = benchmark(run_reuse, iterations=1000, warmup=100)
        result_create = benchmark(run_create, iterations=1000, warmup=100)

        print_benchmark_result("Evaluator reuse", result_reuse)
        print_benchmark_result("Evaluator create each time", result_create)

        # Reuse should be faster
        assert result_reuse["mean_us"] < result_create["mean_us"], (
            "Reusing evaluator should be faster than creating new one"
        )


def run_all_benchmarks():
    """Run all benchmarks and print summary."""
    print("=" * 60)
    print("ASSERTION ENGINE PERFORMANCE BENCHMARKS")
    print("=" * 60)

    # Run pytest with benchmark tests
    pytest.main([
        __file__,
        "-v",
        "-s",
        "--tb=short",
        "-k", "benchmark",
    ])


if __name__ == "__main__":
    run_all_benchmarks()
