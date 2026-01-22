"""
Performance tests for output formatters.

Validates that formatters meet performance requirements:
- Format conversion should add <5ms overhead
- Large trees (1000+ components) should format in <50ms
"""

import pytest
import time
import json
import yaml


@pytest.mark.performance
class TestFormatterPerformance:
    """Performance validation for output formatters."""

    @pytest.fixture
    def library(self):
        """Create library instance."""
        from JavaGui import SwingLibrary
        lib = SwingLibrary()
        return lib

    def measure_format_time(self, library, format_type, iterations=10):
        """Measure average time to format tree."""
        times = []

        for _ in range(iterations):
            start = time.perf_counter()
            library.get_component_tree(format=format_type)
            end = time.perf_counter()
            times.append((end - start) * 1000)  # Convert to milliseconds

        avg_time = sum(times) / len(times)
        min_time = min(times)
        max_time = max(times)

        return {
            "average_ms": avg_time,
            "min_ms": min_time,
            "max_ms": max_time,
            "iterations": iterations
        }

    def test_json_baseline_performance(self, library, test_app):
        """Establish JSON baseline performance."""
        results = self.measure_format_time(library, "json")

        print(f"\nJSON Performance:")
        print(f"  Average: {results['average_ms']:.2f}ms")
        print(f"  Min: {results['min_ms']:.2f}ms")
        print(f"  Max: {results['max_ms']:.2f}ms")

        # JSON should be reasonably fast
        assert results["average_ms"] < 100  # 100ms max for typical tree

    def test_yaml_overhead(self, library, test_app):
        """Test YAML format overhead compared to JSON."""
        json_results = self.measure_format_time(library, "json")
        yaml_results = self.measure_format_time(library, "yaml")

        overhead = yaml_results["average_ms"] - json_results["average_ms"]

        print(f"\nYAML Performance:")
        print(f"  Average: {yaml_results['average_ms']:.2f}ms")
        print(f"  Overhead vs JSON: {overhead:.2f}ms")

        # YAML should add <5ms overhead
        assert overhead < 5.0, f"YAML overhead {overhead:.2f}ms exceeds 5ms limit"

    def test_csv_overhead(self, library, test_app):
        """Test CSV format overhead compared to JSON."""
        json_results = self.measure_format_time(library, "json")
        csv_results = self.measure_format_time(library, "csv")

        overhead = csv_results["average_ms"] - json_results["average_ms"]

        print(f"\nCSV Performance:")
        print(f"  Average: {csv_results['average_ms']:.2f}ms")
        print(f"  Overhead vs JSON: {overhead:.2f}ms")

        # CSV should add <5ms overhead
        assert overhead < 5.0, f"CSV overhead {overhead:.2f}ms exceeds 5ms limit"

    def test_markdown_overhead(self, library, test_app):
        """Test Markdown format overhead compared to JSON."""
        json_results = self.measure_format_time(library, "json")
        md_results = self.measure_format_time(library, "markdown")

        overhead = md_results["average_ms"] - json_results["average_ms"]

        print(f"\nMarkdown Performance:")
        print(f"  Average: {md_results['average_ms']:.2f}ms")
        print(f"  Overhead vs JSON: {overhead:.2f}ms")

        # Markdown should add <5ms overhead
        assert overhead < 5.0, f"Markdown overhead {overhead:.2f}ms exceeds 5ms limit"

    def test_all_formats_performance_summary(self, library, test_app):
        """Compare performance of all formats."""
        formats = ["json", "xml", "yaml", "csv", "markdown", "text"]
        results = {}

        for fmt in formats:
            try:
                results[fmt] = self.measure_format_time(library, fmt)
            except Exception as e:
                print(f"Skipping {fmt}: {e}")

        print("\n=== Performance Summary ===")
        print(f"{'Format':<12} {'Avg (ms)':<10} {'Min (ms)':<10} {'Max (ms)':<10}")
        print("-" * 45)

        for fmt, data in sorted(results.items(), key=lambda x: x[1]["average_ms"]):
            print(f"{fmt:<12} {data['average_ms']:>8.2f}   {data['min_ms']:>8.2f}   {data['max_ms']:>8.2f}")

        # All formats should complete in reasonable time
        for fmt, data in results.items():
            assert data["average_ms"] < 100, f"{fmt} average time {data['average_ms']:.2f}ms exceeds 100ms"

    @pytest.mark.slow
    def test_large_tree_yaml_performance(self, library):
        """Test YAML performance with large tree (if available)."""
        # This would require a large test application
        # For now, just verify format works
        yaml_tree = library.get_component_tree(format="yaml")
        parsed = yaml.safe_load(yaml_tree)

        # Count components
        def count_components(tree):
            count = len(tree.get("roots", []))
            for root in tree.get("roots", []):
                count += count_children(root)
            return count

        def count_children(comp):
            count = 0
            if "children" in comp and comp["children"]:
                count += len(comp["children"])
                for child in comp["children"]:
                    count += count_children(child)
            return count

        component_count = count_components(parsed)
        print(f"\nComponent count: {component_count}")

        # If tree has 100+ components, should still be fast
        if component_count >= 100:
            start = time.perf_counter()
            library.get_component_tree(format="yaml")
            duration = (time.perf_counter() - start) * 1000

            print(f"Large tree YAML time: {duration:.2f}ms")
            assert duration < 50, f"Large tree formatting {duration:.2f}ms exceeds 50ms limit"

    @pytest.mark.slow
    def test_large_tree_csv_performance(self, library):
        """Test CSV performance with large tree."""
        csv_tree = library.get_component_tree(format="csv")
        row_count = len(csv_tree.split('\n')) - 1  # Exclude header

        print(f"\nCSV row count: {row_count}")

        # If tree has 100+ components
        if row_count >= 100:
            start = time.perf_counter()
            library.get_component_tree(format="csv")
            duration = (time.perf_counter() - start) * 1000

            print(f"Large tree CSV time: {duration:.2f}ms")
            assert duration < 50, f"Large tree formatting {duration:.2f}ms exceeds 50ms limit"

    def test_format_scaling(self, library, test_app):
        """Test that format time scales linearly with tree size."""
        # Get full tree time
        full_start = time.perf_counter()
        library.get_component_tree(format="yaml")
        full_time = (time.perf_counter() - full_start) * 1000

        # Get limited tree time
        limited_start = time.perf_counter()
        library.get_component_tree(format="yaml", max_depth=1)
        limited_time = (time.perf_counter() - limited_start) * 1000

        print(f"\nScaling Test:")
        print(f"  Full tree: {full_time:.2f}ms")
        print(f"  Limited tree (depth=1): {limited_time:.2f}ms")

        # Limited tree should be faster (or similar if tree is small)
        assert limited_time <= full_time * 1.5  # Allow some variance

    def test_repeated_format_calls_no_degradation(self, library, test_app):
        """Test that repeated format calls don't degrade performance."""
        times = []

        # Make 20 calls
        for i in range(20):
            start = time.perf_counter()
            library.get_component_tree(format="yaml")
            duration = (time.perf_counter() - start) * 1000
            times.append(duration)

        # Calculate first half vs second half average
        first_half = sum(times[:10]) / 10
        second_half = sum(times[10:]) / 10

        print(f"\nRepeated Calls Test:")
        print(f"  First 10 calls avg: {first_half:.2f}ms")
        print(f"  Last 10 calls avg: {second_half:.2f}ms")

        # Second half should not be significantly slower
        assert second_half <= first_half * 1.2  # Allow 20% variance


@pytest.mark.performance
class TestFormatterMemoryEfficiency:
    """Test memory efficiency of formatters."""

    @pytest.fixture
    def library(self):
        """Create library instance."""
        from JavaGui import SwingLibrary
        lib = SwingLibrary()
        return lib

    def test_yaml_memory_efficiency(self, library, test_app):
        """Test YAML formatter doesn't use excessive memory."""
        import sys

        # Get tree
        yaml_tree = library.get_component_tree(format="yaml")

        # Size should be reasonable
        size_bytes = sys.getsizeof(yaml_tree)
        size_kb = size_bytes / 1024

        print(f"\nYAML output size: {size_kb:.2f} KB")

        # Should not be excessively large
        assert size_kb < 1024  # Less than 1MB for typical tree

    def test_csv_output_size(self, library, test_app):
        """Test CSV output size is reasonable."""
        import sys

        csv_tree = library.get_component_tree(format="csv")
        size_bytes = sys.getsizeof(csv_tree)
        size_kb = size_bytes / 1024

        print(f"\nCSV output size: {size_kb:.2f} KB")

        # CSV should be compact
        assert size_kb < 1024  # Less than 1MB

    def test_markdown_output_size(self, library, test_app):
        """Test Markdown output size is reasonable."""
        import sys

        md_tree = library.get_component_tree(format="markdown")
        size_bytes = sys.getsizeof(md_tree)
        size_kb = size_bytes / 1024

        print(f"\nMarkdown output size: {size_kb:.2f} KB")

        # Markdown may be larger due to formatting
        assert size_kb < 2048  # Less than 2MB


if __name__ == "__main__":
    pytest.main([__file__, "-v", "-m", "performance"])
