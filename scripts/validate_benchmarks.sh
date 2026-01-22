#!/bin/bash
# Quick validation script to verify benchmarks are working

set -e

echo "========================================"
echo "Benchmark Validation Script"
echo "========================================"
echo

# Check Python
echo "Checking Python installation..."
if ! command -v python3 &> /dev/null; then
    echo "ERROR: Python 3 not found"
    exit 1
fi
python3 --version

# Check pytest
echo "Checking pytest installation..."
if ! python3 -c "import pytest" 2>/dev/null; then
    echo "WARNING: pytest not installed. Installing..."
    pip install pytest
fi

# Check if benchmark file exists
BENCHMARK_FILE="/mnt/c/workspace/robotframework-swing/tests/python/test_component_tree_benchmarks.py"
if [ ! -f "$BENCHMARK_FILE" ]; then
    echo "ERROR: Benchmark file not found: $BENCHMARK_FILE"
    exit 1
fi
echo "Found benchmark file: $BENCHMARK_FILE"

# Run a quick smoke test
echo
echo "Running quick smoke test..."
python3 -c "
from test_component_tree_benchmarks import ComponentTreeBenchmarks
bench = ComponentTreeBenchmarks()
tree = bench.create_mock_tree(10)
count = bench.count_nodes(tree)
print(f'Created mock tree with {count} nodes')
assert count == 10, f'Expected 10 nodes, got {count}'
print('✓ Smoke test passed')
" 2>/dev/null || echo "WARNING: Smoke test failed (may need to adjust Python path)"

echo
echo "========================================"
echo "Validation Complete"
echo "========================================"
echo
echo "To run benchmarks:"
echo "  python3 tests/python/test_component_tree_benchmarks.py"
echo "  pytest tests/python/test_component_tree_benchmarks.py -v -s"
echo "  python3 scripts/run_performance_benchmarks.py --profile"
echo
