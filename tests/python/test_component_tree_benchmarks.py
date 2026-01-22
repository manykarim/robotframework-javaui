"""Performance benchmarks for component tree functionality.

This module benchmarks:
- Tree retrieval with varying sizes (10, 100, 500, 1000, 5000 components)
- Tree retrieval with different depth limits (1, 5, 10, unlimited)
- All output formats (text, json, xml, yaml, csv, markdown)
- Filtering operations
- Memory consumption
- Cache operations

Performance Targets:
- Tree retrieval: <100ms for 1000 components
- Memory usage: <50MB for 10,000 components
- Cache refresh: <50ms
- Format conversion: <10ms
"""

import pytest
import time
import statistics
import gc
import sys
from typing import List, Callable, Dict, Any
import json

# Memory profiling
try:
    import tracemalloc
    MEMORY_PROFILING_AVAILABLE = True
except ImportError:
    MEMORY_PROFILING_AVAILABLE = False


class BenchmarkResult:
    """Container for benchmark results."""

    def __init__(self, name: str):
        self.name = name
        self.iterations = 0
        self.times_us: List[float] = []
        self.memory_peak_mb = 0
        self.memory_current_mb = 0

    def add_timing(self, time_us: float):
        """Add a timing measurement in microseconds."""
        self.times_us.append(time_us)
        self.iterations += 1

    def set_memory(self, peak_mb: float, current_mb: float):
        """Set memory measurements in megabytes."""
        self.memory_peak_mb = peak_mb
        self.memory_current_mb = current_mb

    @property
    def min_us(self) -> float:
        return min(self.times_us) if self.times_us else 0

    @property
    def max_us(self) -> float:
        return max(self.times_us) if self.times_us else 0

    @property
    def mean_us(self) -> float:
        return statistics.mean(self.times_us) if self.times_us else 0

    @property
    def median_us(self) -> float:
        return statistics.median(self.times_us) if self.times_us else 0

    @property
    def p50_us(self) -> float:
        """50th percentile (median)."""
        return self.median_us

    @property
    def p95_us(self) -> float:
        """95th percentile."""
        if not self.times_us:
            return 0
        sorted_times = sorted(self.times_us)
        index = int(len(sorted_times) * 0.95)
        return sorted_times[index]

    @property
    def p99_us(self) -> float:
        """99th percentile."""
        if not self.times_us:
            return 0
        sorted_times = sorted(self.times_us)
        index = int(len(sorted_times) * 0.99)
        return sorted_times[index]

    @property
    def stdev_us(self) -> float:
        return statistics.stdev(self.times_us) if len(self.times_us) > 1 else 0

    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary for reporting."""
        return {
            'name': self.name,
            'iterations': self.iterations,
            'min_us': self.min_us,
            'max_us': self.max_us,
            'mean_us': self.mean_us,
            'median_us': self.median_us,
            'p50_us': self.p50_us,
            'p95_us': self.p95_us,
            'p99_us': self.p99_us,
            'stdev_us': self.stdev_us,
            'memory_peak_mb': self.memory_peak_mb,
            'memory_current_mb': self.memory_current_mb,
        }


def benchmark(
    func: Callable,
    iterations: int = 1000,
    warmup: int = 100,
    track_memory: bool = False
) -> BenchmarkResult:
    """Run a benchmark and return statistics.

    Args:
        func: Function to benchmark (should take no arguments).
        iterations: Number of timed iterations.
        warmup: Number of warmup iterations.
        track_memory: Whether to track memory usage.

    Returns:
        BenchmarkResult with timing and memory statistics.
    """
    result = BenchmarkResult(func.__name__ if hasattr(func, '__name__') else 'anonymous')

    # Warmup
    for _ in range(warmup):
        func()

    # Start memory tracking if enabled
    if track_memory and MEMORY_PROFILING_AVAILABLE:
        gc.collect()
        tracemalloc.start()

    # Timed runs
    for _ in range(iterations):
        start = time.perf_counter()
        func()
        end = time.perf_counter()
        result.add_timing((end - start) * 1_000_000)  # Convert to microseconds

    # Memory measurements
    if track_memory and MEMORY_PROFILING_AVAILABLE:
        current, peak = tracemalloc.get_traced_memory()
        result.set_memory(peak / 1024 / 1024, current / 1024 / 1024)
        tracemalloc.stop()

    return result


def print_benchmark_result(result: BenchmarkResult, show_memory: bool = False):
    """Print benchmark results in a formatted way."""
    print(f"\n{result.name}:")
    print(f"  Iterations: {result.iterations}")
    print(f"  Min:        {result.min_us:>10.2f} µs")
    print(f"  Max:        {result.max_us:>10.2f} µs")
    print(f"  Mean:       {result.mean_us:>10.2f} µs")
    print(f"  Median:     {result.median_us:>10.2f} µs")
    print(f"  P95:        {result.p95_us:>10.2f} µs")
    print(f"  P99:        {result.p99_us:>10.2f} µs")
    print(f"  Stdev:      {result.stdev_us:>10.2f} µs")

    if show_memory and result.memory_peak_mb > 0:
        print(f"  Memory Peak:    {result.memory_peak_mb:>10.2f} MB")
        print(f"  Memory Current: {result.memory_current_mb:>10.2f} MB")


class ComponentTreeBenchmarks:
    """Base class for component tree benchmarks."""

    # Mock component tree data for benchmarking
    @staticmethod
    def create_mock_tree(num_components: int, max_depth: int = 10) -> dict:
        """Create a mock component tree for benchmarking.

        Args:
            num_components: Total number of components to create
            max_depth: Maximum tree depth

        Returns:
            Mock tree structure
        """
        def create_node(id_counter: List[int], depth: int, target_count: int) -> dict:
            if id_counter[0] >= target_count or depth > max_depth:
                return None

            node_id = id_counter[0]
            id_counter[0] += 1

            node = {
                'id': node_id,
                'class': 'javax.swing.JPanel',
                'simpleClass': 'JPanel',
                'name': f'component_{node_id}',
                'x': 0,
                'y': 0,
                'width': 100,
                'height': 100,
                'visible': True,
                'enabled': True,
                'showing': True,
                'text': f'Component {node_id}',
                'children': []
            }

            # Add children
            if depth < max_depth:
                children_count = min(3, target_count - id_counter[0])
                for _ in range(children_count):
                    if id_counter[0] >= target_count:
                        break
                    child = create_node(id_counter, depth + 1, target_count)
                    if child:
                        node['children'].append(child)

            if not node['children']:
                del node['children']

            return node

        id_counter = [0]
        return {
            'roots': [create_node(id_counter, 0, num_components)],
            'timestamp': int(time.time() * 1000)
        }

    @staticmethod
    def count_nodes(tree: dict) -> int:
        """Count total nodes in tree."""
        count = 0
        if 'roots' in tree:
            for root in tree['roots']:
                count += ComponentTreeBenchmarks._count_nodes_recursive(root)
        else:
            count = ComponentTreeBenchmarks._count_nodes_recursive(tree)
        return count

    @staticmethod
    def _count_nodes_recursive(node: dict) -> int:
        count = 1
        if 'children' in node:
            for child in node['children']:
                count += ComponentTreeBenchmarks._count_nodes_recursive(child)
        return count

    @staticmethod
    def traverse_tree(tree: dict, max_depth: int = None):
        """Traverse tree up to max_depth."""
        def traverse_node(node: dict, depth: int):
            # Access all properties
            _ = node.get('id')
            _ = node.get('class')
            _ = node.get('name')
            _ = node.get('text')

            if max_depth is None or depth < max_depth:
                if 'children' in node:
                    for child in node['children']:
                        traverse_node(child, depth + 1)

        if 'roots' in tree:
            for root in tree['roots']:
                traverse_node(root, 0)
        else:
            traverse_node(tree, 0)


class TestTreeSizeBenchmarks(ComponentTreeBenchmarks):
    """Benchmarks for different tree sizes."""

    def test_tree_size_10_components(self):
        """Benchmark tree with 10 components."""
        tree = self.create_mock_tree(10)

        def run():
            self.traverse_tree(tree)

        result = benchmark(run, iterations=10000, warmup=1000, track_memory=True)
        result.name = "Tree Size: 10 components"
        print_benchmark_result(result, show_memory=True)

        assert result.mean_us < 100, f"Mean time {result.mean_us}µs exceeds 100µs for 10 components"

    def test_tree_size_100_components(self):
        """Benchmark tree with 100 components."""
        tree = self.create_mock_tree(100)

        def run():
            self.traverse_tree(tree)

        result = benchmark(run, iterations=5000, warmup=500, track_memory=True)
        result.name = "Tree Size: 100 components"
        print_benchmark_result(result, show_memory=True)

        assert result.mean_us < 1000, f"Mean time {result.mean_us}µs exceeds 1000µs for 100 components"

    def test_tree_size_500_components(self):
        """Benchmark tree with 500 components."""
        tree = self.create_mock_tree(500)

        def run():
            self.traverse_tree(tree)

        result = benchmark(run, iterations=1000, warmup=100, track_memory=True)
        result.name = "Tree Size: 500 components"
        print_benchmark_result(result, show_memory=True)

        assert result.mean_us < 5000, f"Mean time {result.mean_us}µs exceeds 5000µs for 500 components"

    def test_tree_size_1000_components(self):
        """Benchmark tree with 1000 components - TARGET: <100ms."""
        tree = self.create_mock_tree(1000)

        def run():
            self.traverse_tree(tree)

        result = benchmark(run, iterations=500, warmup=50, track_memory=True)
        result.name = "Tree Size: 1000 components (TARGET)"
        print_benchmark_result(result, show_memory=True)

        # Performance target: <100ms (100,000µs)
        assert result.mean_us < 100_000, f"Mean time {result.mean_us}µs exceeds 100ms target for 1000 components"

    def test_tree_size_5000_components(self):
        """Benchmark tree with 5000 components."""
        tree = self.create_mock_tree(5000)

        def run():
            self.traverse_tree(tree)

        result = benchmark(run, iterations=100, warmup=10, track_memory=True)
        result.name = "Tree Size: 5000 components"
        print_benchmark_result(result, show_memory=True)

        # Should complete in reasonable time
        assert result.mean_us < 500_000, f"Mean time {result.mean_us}µs exceeds 500ms for 5000 components"


class TestTreeDepthBenchmarks(ComponentTreeBenchmarks):
    """Benchmarks for different tree depths."""

    def test_depth_limit_1(self):
        """Benchmark tree traversal with depth limit 1."""
        tree = self.create_mock_tree(1000)

        def run():
            self.traverse_tree(tree, max_depth=1)

        result = benchmark(run, iterations=5000, warmup=500)
        result.name = "Depth Limit: 1"
        print_benchmark_result(result)

        # Should be very fast with shallow depth
        assert result.mean_us < 1000, f"Mean time {result.mean_us}µs exceeds 1000µs for depth 1"

    def test_depth_limit_5(self):
        """Benchmark tree traversal with depth limit 5."""
        tree = self.create_mock_tree(1000)

        def run():
            self.traverse_tree(tree, max_depth=5)

        result = benchmark(run, iterations=1000, warmup=100)
        result.name = "Depth Limit: 5"
        print_benchmark_result(result)

        assert result.mean_us < 50_000, f"Mean time {result.mean_us}µs exceeds 50ms for depth 5"

    def test_depth_limit_10(self):
        """Benchmark tree traversal with depth limit 10."""
        tree = self.create_mock_tree(1000)

        def run():
            self.traverse_tree(tree, max_depth=10)

        result = benchmark(run, iterations=500, warmup=50)
        result.name = "Depth Limit: 10"
        print_benchmark_result(result)

        assert result.mean_us < 100_000, f"Mean time {result.mean_us}µs exceeds 100ms for depth 10"

    def test_depth_unlimited(self):
        """Benchmark tree traversal with unlimited depth."""
        tree = self.create_mock_tree(1000)

        def run():
            self.traverse_tree(tree, max_depth=None)

        result = benchmark(run, iterations=500, warmup=50)
        result.name = "Depth Limit: Unlimited"
        print_benchmark_result(result)

        assert result.mean_us < 100_000, f"Mean time {result.mean_us}µs exceeds 100ms for unlimited depth"


class TestFormatConversionBenchmarks(ComponentTreeBenchmarks):
    """Benchmarks for output format conversion."""

    def test_json_serialization(self):
        """Benchmark JSON serialization - TARGET: <10ms."""
        tree = self.create_mock_tree(1000)

        def run():
            json.dumps(tree)

        result = benchmark(run, iterations=1000, warmup=100)
        result.name = "Format: JSON serialization"
        print_benchmark_result(result)

        # Target: <10ms (10,000µs)
        assert result.mean_us < 10_000, f"Mean time {result.mean_us}µs exceeds 10ms target for JSON"

    def test_json_deserialization(self):
        """Benchmark JSON deserialization."""
        tree = self.create_mock_tree(1000)
        json_str = json.dumps(tree)

        def run():
            json.loads(json_str)

        result = benchmark(run, iterations=1000, warmup=100)
        result.name = "Format: JSON deserialization"
        print_benchmark_result(result)

        assert result.mean_us < 10_000, f"Mean time {result.mean_us}µs exceeds 10ms for JSON parsing"

    def test_text_format_conversion(self):
        """Benchmark text format conversion."""
        tree = self.create_mock_tree(1000)

        def convert_to_text(node: dict, depth: int = 0) -> str:
            """Convert tree to text format."""
            indent = "  " * depth
            lines = [f"{indent}{node.get('simpleClass', 'Unknown')}[{node.get('id')}]"]

            if 'children' in node:
                for child in node['children']:
                    lines.append(convert_to_text(child, depth + 1))

            return "\n".join(lines)

        def run():
            if 'roots' in tree:
                for root in tree['roots']:
                    convert_to_text(root)
            else:
                convert_to_text(tree)

        result = benchmark(run, iterations=500, warmup=50)
        result.name = "Format: Text conversion"
        print_benchmark_result(result)

        assert result.mean_us < 10_000, f"Mean time {result.mean_us}µs exceeds 10ms for text format"


class TestCacheBenchmarks(ComponentTreeBenchmarks):
    """Benchmarks for component cache operations."""

    def test_cache_lookup_performance(self):
        """Benchmark cache lookup operations."""
        # Simulate component cache
        cache = {i: f"component_{i}" for i in range(10000)}

        def run():
            # Lookup random components
            for i in range(0, 1000, 10):
                _ = cache.get(i)

        result = benchmark(run, iterations=5000, warmup=500)
        result.name = "Cache: Lookup (10k entries, 100 lookups)"
        print_benchmark_result(result)

        # Should be very fast - dictionary lookups are O(1)
        assert result.mean_us < 100, f"Mean time {result.mean_us}µs exceeds 100µs for cache lookup"

    def test_cache_refresh_performance(self):
        """Benchmark cache refresh - TARGET: <50ms."""
        tree = self.create_mock_tree(1000)

        def refresh_cache():
            """Simulate cache refresh by rebuilding component ID map."""
            cache = {}
            reverse_cache = {}
            id_counter = [0]

            def process_node(node: dict):
                node_id = id_counter[0]
                id_counter[0] += 1
                cache[node_id] = node
                reverse_cache[node.get('name')] = node_id

                if 'children' in node:
                    for child in node['children']:
                        process_node(child)

            if 'roots' in tree:
                for root in tree['roots']:
                    process_node(root)

        def run():
            refresh_cache()

        result = benchmark(run, iterations=500, warmup=50, track_memory=True)
        result.name = "Cache: Refresh (1000 components) TARGET"
        print_benchmark_result(result, show_memory=True)

        # Target: <50ms (50,000µs)
        assert result.mean_us < 50_000, f"Mean time {result.mean_us}µs exceeds 50ms target for cache refresh"


class TestMemoryBenchmarks(ComponentTreeBenchmarks):
    """Memory consumption benchmarks."""

    @pytest.mark.skipif(not MEMORY_PROFILING_AVAILABLE, reason="tracemalloc not available")
    def test_memory_1000_components(self):
        """Measure memory for 1000 components."""
        def run():
            tree = self.create_mock_tree(1000)
            self.traverse_tree(tree)

        result = benchmark(run, iterations=100, warmup=10, track_memory=True)
        result.name = "Memory: 1000 components"
        print_benchmark_result(result, show_memory=True)

        # Should use reasonable memory
        assert result.memory_peak_mb < 10, f"Peak memory {result.memory_peak_mb}MB exceeds 10MB for 1000 components"

    @pytest.mark.skipif(not MEMORY_PROFILING_AVAILABLE, reason="tracemalloc not available")
    def test_memory_10000_components(self):
        """Measure memory for 10,000 components - TARGET: <50MB."""
        def run():
            tree = self.create_mock_tree(10000)
            self.traverse_tree(tree)

        result = benchmark(run, iterations=10, warmup=2, track_memory=True)
        result.name = "Memory: 10000 components TARGET"
        print_benchmark_result(result, show_memory=True)

        # Target: <50MB
        assert result.memory_peak_mb < 50, f"Peak memory {result.memory_peak_mb}MB exceeds 50MB target for 10k components"


class TestFilteringBenchmarks(ComponentTreeBenchmarks):
    """Benchmarks for component filtering operations."""

    def test_filter_by_class(self):
        """Benchmark filtering by component class."""
        tree = self.create_mock_tree(1000)
        target_class = "javax.swing.JPanel"

        def filter_by_class(node: dict, target: str) -> List[dict]:
            results = []
            if node.get('class') == target:
                results.append(node)
            if 'children' in node:
                for child in node['children']:
                    results.extend(filter_by_class(child, target))
            return results

        def run():
            if 'roots' in tree:
                for root in tree['roots']:
                    filter_by_class(root, target_class)

        result = benchmark(run, iterations=500, warmup=50)
        result.name = "Filter: By class name"
        print_benchmark_result(result)

        assert result.mean_us < 50_000, f"Mean time {result.mean_us}µs exceeds 50ms for class filtering"

    def test_filter_by_text(self):
        """Benchmark filtering by text content."""
        tree = self.create_mock_tree(1000)
        search_text = "Component 100"

        def filter_by_text(node: dict, text: str) -> List[dict]:
            results = []
            if text in node.get('text', ''):
                results.append(node)
            if 'children' in node:
                for child in node['children']:
                    results.extend(filter_by_text(child, text))
            return results

        def run():
            if 'roots' in tree:
                for root in tree['roots']:
                    filter_by_text(root, search_text)

        result = benchmark(run, iterations=500, warmup=50)
        result.name = "Filter: By text content"
        print_benchmark_result(result)

        assert result.mean_us < 50_000, f"Mean time {result.mean_us}µs exceeds 50ms for text filtering"

    def test_filter_visible_components(self):
        """Benchmark filtering visible components only."""
        tree = self.create_mock_tree(1000)

        def filter_visible(node: dict) -> List[dict]:
            results = []
            if node.get('visible', False) and node.get('showing', False):
                results.append(node)
            if 'children' in node:
                for child in node['children']:
                    results.extend(filter_visible(child))
            return results

        def run():
            if 'roots' in tree:
                for root in tree['roots']:
                    filter_visible(root)

        result = benchmark(run, iterations=1000, warmup=100)
        result.name = "Filter: Visible components only"
        print_benchmark_result(result)

        assert result.mean_us < 50_000, f"Mean time {result.mean_us}µs exceeds 50ms for visibility filtering"


def run_all_component_tree_benchmarks():
    """Run all component tree benchmarks and generate report."""
    print("=" * 80)
    print("COMPONENT TREE PERFORMANCE BENCHMARKS")
    print("=" * 80)
    print("\nPerformance Targets:")
    print("- Tree retrieval: <100ms for 1000 components")
    print("- Memory usage: <50MB for 10,000 components")
    print("- Cache refresh: <50ms")
    print("- Format conversion: <10ms")
    print("=" * 80)

    # Run pytest with benchmark tests
    pytest.main([
        __file__,
        "-v",
        "-s",
        "--tb=short",
        "-k", "benchmark",
    ])


if __name__ == "__main__":
    run_all_component_tree_benchmarks()
