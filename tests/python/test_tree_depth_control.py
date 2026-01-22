"""
Tests for UI Tree Depth Control and Performance Optimization.

This test suite verifies:
1. Depth limiting works correctly
2. Performance targets are met
3. Caching strategy functions properly
4. Memory consumption is reasonable
"""

import pytest
import json
import time
from typing import Optional, Dict, Any


def count_tree_depth(tree_json: str) -> int:
    """Calculate the maximum depth of a JSON tree."""
    tree = json.loads(tree_json)

    def _depth(node: Dict[str, Any], current: int = 0) -> int:
        if "children" not in node or not node["children"]:
            return current

        max_child_depth = current
        for child in node["children"]:
            child_depth = _depth(child, current + 1)
            max_child_depth = max(max_child_depth, child_depth)

        return max_child_depth

    if "roots" in tree:
        return max(_depth(root) for root in tree["roots"])
    return _depth(tree)


def count_components(tree_json: str) -> int:
    """Count total components in a JSON tree."""
    tree = json.loads(tree_json)

    def _count(node: Dict[str, Any]) -> int:
        count = 1
        if "children" in node:
            count += sum(_count(child) for child in node["children"])
        return count

    if "roots" in tree:
        return sum(_count(root) for root in tree["roots"])
    return _count(tree)


class TestDepthLimiting:
    """Test depth limiting functionality."""

    def test_depth_1_only_immediate_children(self, swing_library):
        """Depth 1 should return only immediate children."""
        tree_json = swing_library.get_component_tree(format="json", max_depth=1)
        actual_depth = count_tree_depth(tree_json)

        assert actual_depth <= 1, f"Expected depth <= 1, got {actual_depth}"

    def test_depth_5_moderate_tree(self, swing_library):
        """Depth 5 should return tree up to 5 levels deep."""
        tree_json = swing_library.get_component_tree(format="json", max_depth=5)
        actual_depth = count_tree_depth(tree_json)

        assert actual_depth <= 5, f"Expected depth <= 5, got {actual_depth}"

    def test_depth_10_deep_tree(self, swing_library):
        """Depth 10 should return tree up to 10 levels deep."""
        tree_json = swing_library.get_component_tree(format="json", max_depth=10)
        actual_depth = count_tree_depth(tree_json)

        assert actual_depth <= 10, f"Expected depth <= 10, got {actual_depth}"

    def test_unlimited_depth_full_tree(self, swing_library):
        """Unlimited depth should return complete tree."""
        tree_unlimited = swing_library.get_component_tree(format="json")
        tree_limited = swing_library.get_component_tree(format="json", max_depth=5)

        count_unlimited = count_components(tree_unlimited)
        count_limited = count_components(tree_limited)

        # Unlimited should have more or equal components
        assert count_unlimited >= count_limited, \
            f"Unlimited ({count_unlimited}) should have >= limited ({count_limited}) components"

    def test_depth_0_returns_roots_only(self, swing_library):
        """Depth 0 should return only root windows with no children."""
        tree_json = swing_library.get_component_tree(format="json", max_depth=0)
        tree = json.loads(tree_json)

        # Should have roots
        assert "roots" in tree
        roots = tree["roots"]
        assert len(roots) > 0, "Should have at least one root window"

        # No root should have children
        for root in roots:
            children = root.get("children", [])
            assert len(children) == 0, \
                f"Depth 0 should have no children, but root has {len(children)}"

    @pytest.mark.parametrize("depth", [0, 1, 2, 3, 5, 10, 20])
    def test_various_depths(self, swing_library, depth):
        """Test various depth values."""
        tree_json = swing_library.get_component_tree(format="json", max_depth=depth)
        actual_depth = count_tree_depth(tree_json)

        assert actual_depth <= depth, \
            f"Expected depth <= {depth}, got {actual_depth}"

    def test_negative_depth_raises_error(self, swing_library):
        """Negative depth should raise ValueError."""
        with pytest.raises(ValueError, match="must be >= 0"):
            swing_library.get_component_tree(format="json", max_depth=-1)

    def test_non_integer_depth_raises_error(self, swing_library):
        """Non-integer depth should raise TypeError."""
        with pytest.raises(TypeError, match="must be an integer or None"):
            swing_library.get_component_tree(format="json", max_depth="5")

        with pytest.raises(TypeError, match="must be an integer or None"):
            swing_library.get_component_tree(format="json", max_depth=5.5)


class TestPerformance:
    """Test performance characteristics of depth control."""

    @pytest.mark.performance
    def test_depth_1_performance_100_components(self, swing_library_100):
        """Depth 1 query should be very fast (<10ms) for 100 components."""
        start = time.time()
        tree_json = swing_library_100.get_component_tree(format="json", max_depth=1)
        duration_ms = (time.time() - start) * 1000

        assert duration_ms < 10, f"Expected <10ms, got {duration_ms:.2f}ms"
        assert count_tree_depth(tree_json) <= 1

    @pytest.mark.performance
    def test_depth_5_performance_1000_components(self, swing_library_1000):
        """Depth 5 query should be fast (<100ms) for 1000 components."""
        start = time.time()
        tree_json = swing_library_1000.get_component_tree(format="json", max_depth=5)
        duration_ms = (time.time() - start) * 1000

        assert duration_ms < 100, f"Expected <100ms, got {duration_ms:.2f}ms"

    @pytest.mark.performance
    def test_unlimited_performance_1000_components(self, swing_library_1000):
        """Unlimited depth should meet performance target (<100ms for 1000)."""
        start = time.time()
        tree_json = swing_library_1000.get_component_tree(format="json")
        duration_ms = (time.time() - start) * 1000

        assert duration_ms < 100, f"Expected <100ms, got {duration_ms:.2f}ms"

    @pytest.mark.performance
    @pytest.mark.slow
    def test_large_tree_performance_5000_components(self, swing_library_5000):
        """Large tree (5000 components) should meet targets."""
        # Depth 1: <30ms
        start = time.time()
        swing_library_5000.get_component_tree(format="json", max_depth=1)
        duration_d1 = (time.time() - start) * 1000
        assert duration_d1 < 30, f"Depth 1: Expected <30ms, got {duration_d1:.2f}ms"

        # Depth 5: <100ms
        start = time.time()
        swing_library_5000.get_component_tree(format="json", max_depth=5)
        duration_d5 = (time.time() - start) * 1000
        assert duration_d5 < 100, f"Depth 5: Expected <100ms, got {duration_d5:.2f}ms"

        # Unlimited: <500ms
        start = time.time()
        swing_library_5000.get_component_tree(format="json")
        duration_unlimited = (time.time() - start) * 1000
        assert duration_unlimited < 500, \
            f"Unlimited: Expected <500ms, got {duration_unlimited:.2f}ms"


class TestCaching:
    """Test caching strategy for tree queries."""

    def test_unlimited_depth_uses_cache(self, swing_library):
        """Unlimited depth queries should use cache on repeat."""
        # First call - fetch fresh
        start1 = time.time()
        tree1 = swing_library.get_component_tree(format="json")
        time1 = time.time() - start1

        # Second call - should be cached (much faster)
        start2 = time.time()
        tree2 = swing_library.get_component_tree(format="json")
        time2 = time.time() - start2

        # Cached call should be at least 2x faster
        assert time2 < time1 / 2, \
            f"Cached call ({time2:.4f}s) should be much faster than first ({time1:.4f}s)"

        # Content should be identical
        assert tree1 == tree2, "Cached tree should match original"

    def test_depth_limited_no_cache(self, swing_library):
        """Depth-limited queries should not use cache."""
        # Two calls with same depth - both should fetch fresh
        start1 = time.time()
        tree1 = swing_library.get_component_tree(format="json", max_depth=5)
        time1 = time.time() - start1

        start2 = time.time()
        tree2 = swing_library.get_component_tree(format="json", max_depth=5)
        time2 = time.time() - start2

        # Times should be similar (both fetch fresh)
        time_ratio = max(time1, time2) / min(time1, time2)
        assert time_ratio < 2, \
            f"Both calls should be similar speed (ratio: {time_ratio:.2f})"

    def test_different_depths_independent(self, swing_library):
        """Different depths should be independent queries."""
        tree_d1 = swing_library.get_component_tree(format="json", max_depth=1)
        tree_d5 = swing_library.get_component_tree(format="json", max_depth=5)

        count_d1 = count_components(tree_d1)
        count_d5 = count_components(tree_d5)

        # Depth 5 should have more components than depth 1
        assert count_d5 > count_d1, \
            f"Depth 5 ({count_d5}) should have more components than depth 1 ({count_d1})"


class TestMemoryConsumption:
    """Test memory consumption characteristics."""

    @pytest.mark.performance
    def test_depth_1_memory_small(self, swing_library_1000):
        """Depth 1 tree should have small memory footprint."""
        tree_json = swing_library_1000.get_component_tree(format="json", max_depth=1)
        size_kb = len(tree_json.encode('utf-8')) / 1024

        # Should be <500KB for 1000 component app
        assert size_kb < 500, f"Expected <500KB, got {size_kb:.2f}KB"

    @pytest.mark.performance
    def test_depth_5_memory_medium(self, swing_library_1000):
        """Depth 5 tree should have medium memory footprint."""
        tree_json = swing_library_1000.get_component_tree(format="json", max_depth=5)
        size_kb = len(tree_json.encode('utf-8')) / 1024

        # Should be <2MB for 1000 component app
        assert size_kb < 2048, f"Expected <2MB, got {size_kb:.2f}KB"

    @pytest.mark.performance
    def test_unlimited_memory_bounded(self, swing_library_1000):
        """Unlimited depth should still have reasonable memory."""
        tree_json = swing_library_1000.get_component_tree(format="json")
        size_kb = len(tree_json.encode('utf-8')) / 1024

        # Should be <10MB for 1000 component app
        assert size_kb < 10240, f"Expected <10MB, got {size_kb:.2f}KB"

    def test_memory_scales_with_depth(self, swing_library_1000):
        """Memory should scale proportionally with depth."""
        tree_d1 = swing_library_1000.get_component_tree(format="json", max_depth=1)
        tree_d5 = swing_library_1000.get_component_tree(format="json", max_depth=5)
        tree_unlimited = swing_library_1000.get_component_tree(format="json")

        size_d1 = len(tree_d1.encode('utf-8'))
        size_d5 = len(tree_d5.encode('utf-8'))
        size_unlimited = len(tree_unlimited.encode('utf-8'))

        # Verify scaling: d1 < d5 < unlimited
        assert size_d1 < size_d5 < size_unlimited, \
            f"Memory should scale: {size_d1} < {size_d5} < {size_unlimited}"


class TestFormats:
    """Test that depth control works across all formats."""

    @pytest.mark.parametrize("format", ["json", "text", "xml"])
    def test_depth_control_all_formats(self, swing_library, format):
        """Depth control should work for all output formats."""
        tree = swing_library.get_component_tree(format=format, max_depth=5)

        assert tree is not None
        assert len(tree) > 0

        # JSON format should be parseable
        if format == "json":
            parsed = json.loads(tree)
            assert "roots" in parsed or "children" in parsed


# Fixtures for test applications with different sizes

@pytest.fixture
def swing_library():
    """Standard test swing application."""
    # TODO: Connect to test app with ~200 components
    from tests.python.conftest import MockSwingLibrary
    return MockSwingLibrary()


@pytest.fixture
def swing_library_100():
    """Test application with 100 components."""
    # TODO: Create/connect to app with exactly 100 components
    from tests.python.conftest import MockSwingLibrary
    return MockSwingLibrary()


@pytest.fixture
def swing_library_1000():
    """Test application with 1000 components."""
    # TODO: Create/connect to app with exactly 1000 components
    from tests.python.conftest import MockSwingLibrary
    return MockSwingLibrary()


@pytest.fixture
def swing_library_5000():
    """Test application with 5000 components."""
    # TODO: Create/connect to app with exactly 5000 components
    from tests.python.conftest import MockSwingLibrary
    return MockSwingLibrary()
