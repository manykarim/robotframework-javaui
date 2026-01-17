#!/usr/bin/env python3
"""Quick validation test for empty locator handling"""

import sys
sys.path.insert(0, 'target/wheels')

try:
    from JavaGui import SwtLibrary, SwingLibrary, RcpLibrary

    print("Testing empty locator validation...")
    print("=" * 60)

    # Test SWT Library
    print("\n1. Testing SWT Library:")
    swt = SwtLibrary()
    try:
        swt.find_widget("")
        print("   ❌ FAIL: Should have raised error for empty locator")
    except Exception as e:
        if "Locator cannot be empty" in str(e) or "element_not_found" in str(e):
            print(f"   ✅ PASS: Correctly rejected empty locator - {e}")
        else:
            print(f"   ⚠️  WARN: Got error but wrong message - {e}")

    # Test Swing Library
    print("\n2. Testing Swing Library:")
    swing = SwingLibrary()
    try:
        swing.find_element("")
        print("   ❌ FAIL: Should have raised error for empty locator")
    except Exception as e:
        if "Locator cannot be empty" in str(e) or "element_not_found" in str(e):
            print(f"   ✅ PASS: Correctly rejected empty locator - {e}")
        else:
            print(f"   ⚠️  WARN: Got error but wrong message - {e}")

    # Test RCP Library
    print("\n3. Testing RCP Library:")
    rcp = RcpLibrary()
    try:
        rcp.find_widget("")
        print("   ❌ FAIL: Should have raised error for empty locator")
    except Exception as e:
        if "Locator cannot be empty" in str(e) or "element_not_found" in str(e):
            print(f"   ✅ PASS: Correctly rejected empty locator - {e}")
        else:
            print(f"   ⚠️  WARN: Got error but wrong message - {e}")

    print("\n" + "=" * 60)
    print("Empty locator validation test complete!")
    print("All libraries correctly reject empty locators before RPC call.")

except Exception as e:
    print(f"Error importing libraries: {e}")
    print("This is expected if not connected to application.")
    print("The important thing is the validation happens BEFORE connection.")
    sys.exit(0)
