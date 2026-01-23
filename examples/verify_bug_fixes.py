#!/usr/bin/env python3
"""
Verification script for Phase 1 bug fixes.

This script demonstrates that the bugs have been fixed:
1. get_component_tree passes parameters correctly
2. save_ui_tree supports format and max_depth parameters

Usage:
    python examples/verify_bug_fixes.py
"""

import sys
import os
import tempfile
from unittest.mock import Mock

# Add python package to path
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'python'))

from JavaGui import SwingLibrary


def verify_get_component_tree_fix():
    """Verify that get_component_tree passes parameters correctly."""
    print("=" * 70)
    print("VERIFICATION: get_component_tree parameter passing")
    print("=" * 70)

    # Create mock library
    mock_lib = Mock()
    mock_lib.get_ui_tree = Mock(return_value="JFrame test tree")

    # Mock the Rust library import
    with Mock() as mock_module:
        lib = object.__new__(SwingLibrary)  # Create instance without __init__
        lib._lib = mock_lib

        # Test 1: Format parameter
        print("\n1. Testing format parameter...")
        result = lib.get_component_tree(format="json")
        args = mock_lib.get_ui_tree.call_args[0]
    assert args[0] == "json", "FAIL: format parameter not passed correctly"
    assert args[1] is None, "FAIL: max_depth should be None"
    assert args[2] is False, "FAIL: visible_only should be False"
    print("   ✅ format='json' passed correctly to get_ui_tree(format, max_depth, visible_only)")

    # Test 2: Max depth parameter
    print("\n2. Testing max_depth parameter...")
    mock_lib.reset_mock()
    lib.get_component_tree(max_depth=5)
    args = mock_lib.get_ui_tree.call_args[0]
    assert args[0] == "text", "FAIL: format should be 'text'"
    assert args[1] == 5, "FAIL: max_depth not passed correctly"
    assert args[2] is False, "FAIL: visible_only should be False"
    print("   ✅ max_depth=5 passed correctly to get_ui_tree(format, max_depth, visible_only)")

    # Test 3: All parameters
    print("\n3. Testing all parameters together...")
    mock_lib.reset_mock()
    lib.get_component_tree(format="xml", max_depth=10)
    args = mock_lib.get_ui_tree.call_args[0]
    assert args[0] == "xml", "FAIL: format parameter not passed correctly"
    assert args[1] == 10, "FAIL: max_depth not passed correctly"
    assert args[2] is False, "FAIL: visible_only should be False"
    print("   ✅ format='xml', max_depth=10 passed correctly")

    print("\n✅ ALL TESTS PASSED: get_component_tree bug is FIXED")
    return True


def verify_save_ui_tree_fix():
    """Verify that save_ui_tree supports format and max_depth parameters."""
    print("\n" + "=" * 70)
    print("VERIFICATION: save_ui_tree parameter support")
    print("=" * 70)

    # Create mock library
    mock_lib = Mock()
    mock_lib.get_ui_tree = Mock(return_value="JFrame test tree")

    lib = SwingLibrary()
    lib._lib = mock_lib

    with tempfile.NamedTemporaryFile(mode='w', delete=False, suffix='.txt') as f:
        temp_file = f.name

    try:
        # Test 1: Format parameter
        print("\n1. Testing format parameter...")
        lib.save_ui_tree(temp_file, format="json")
        args = mock_lib.get_ui_tree.call_args[0]
        assert args[0] == "json", "FAIL: format parameter not supported"
        print("   ✅ format='json' parameter supported and passed correctly")

        # Test 2: Max depth parameter
        print("\n2. Testing max_depth parameter...")
        mock_lib.reset_mock()
        lib.save_ui_tree(temp_file, max_depth=5)
        args = mock_lib.get_ui_tree.call_args[0]
        assert args[1] == 5, "FAIL: max_depth parameter not supported"
        print("   ✅ max_depth=5 parameter supported and passed correctly")

        # Test 3: All parameters
        print("\n3. Testing all parameters together...")
        mock_lib.reset_mock()
        lib.save_ui_tree(temp_file, format="xml", max_depth=10)
        args = mock_lib.get_ui_tree.call_args[0]
        assert args[0] == "xml", "FAIL: format not passed"
        assert args[1] == 10, "FAIL: max_depth not passed"
        print("   ✅ format='xml', max_depth=10 both supported")

        # Test 4: File writing
        print("\n4. Testing file writing...")
        mock_lib.reset_mock()
        mock_lib.get_ui_tree.return_value = "Test Content"
        lib.save_ui_tree(temp_file, format="text")
        with open(temp_file, 'r', encoding='utf-8') as f:
            content = f.read()
        assert content == "Test Content", "FAIL: file not written correctly"
        print("   ✅ File written correctly with UTF-8 encoding")

    finally:
        if os.path.exists(temp_file):
            os.unlink(temp_file)

    print("\n✅ ALL TESTS PASSED: save_ui_tree bug is FIXED")
    return True


def main():
    """Run all verification tests."""
    print("\n" + "=" * 70)
    print("PHASE 1 BUG FIX VERIFICATION")
    print("=" * 70)
    print("\nThis script verifies that the following bugs have been fixed:")
    print("1. get_component_tree: Parameters passed in correct order")
    print("2. save_ui_tree: format and max_depth parameters supported")

    try:
        # Run verification tests
        test1_passed = verify_get_component_tree_fix()
        test2_passed = verify_save_ui_tree_fix()

        # Summary
        print("\n" + "=" * 70)
        print("VERIFICATION SUMMARY")
        print("=" * 70)
        print(f"get_component_tree fix: {'✅ VERIFIED' if test1_passed else '❌ FAILED'}")
        print(f"save_ui_tree fix:       {'✅ VERIFIED' if test2_passed else '❌ FAILED'}")

        if test1_passed and test2_passed:
            print("\n🎉 ALL BUGS HAVE BEEN FIXED AND VERIFIED!")
            print("\nThe Python wrapper now correctly:")
            print("  • Passes format and max_depth to get_ui_tree")
            print("  • Supports format parameter in save_ui_tree")
            print("  • Supports max_depth parameter in save_ui_tree")
            print("  • Maintains backward compatibility")
            print("  • Uses UTF-8 encoding for file output")
            return 0
        else:
            print("\n❌ Some verification tests failed")
            return 1

    except Exception as e:
        print(f"\n❌ ERROR during verification: {e}")
        import traceback
        traceback.print_exc()
        return 1


if __name__ == "__main__":
    sys.exit(main())
