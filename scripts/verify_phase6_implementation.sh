#!/bin/bash
# Phase 6 RCP Implementation Verification Script
#
# Checks that all Phase 6 deliverables are in place and properly structured

set -e

echo "========================================="
echo "Phase 6 RCP Implementation Verification"
echo "========================================="
echo ""

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS=0
FAIL=0
WARN=0

check_file() {
    local file=$1
    local description=$2

    if [ -f "$file" ]; then
        echo -e "${GREEN}✓${NC} $description: $file"
        ((PASS++))
    else
        echo -e "${RED}✗${NC} $description: $file (MISSING)"
        ((FAIL++))
    fi
}

check_content() {
    local file=$1
    local pattern=$2
    local description=$3

    if grep -q "$pattern" "$file" 2>/dev/null; then
        echo -e "${GREEN}✓${NC} $description"
        ((PASS++))
    else
        echo -e "${RED}✗${NC} $description (NOT FOUND)"
        ((FAIL++))
    fi
}

check_warning() {
    local message=$1
    echo -e "${YELLOW}⚠${NC} $message"
    ((WARN++))
}

echo "1. Checking Java Implementation..."
echo "-----------------------------------"

check_file "agent/src/main/java/com/robotframework/swt/RcpComponentInspector.java" \
    "RCP Component Inspector"

check_file "agent/src/main/java/com/robotframework/swt/EclipseWorkbenchHelper.java" \
    "Eclipse Workbench Helper"

check_file "agent/src/main/java/com/robotframework/swt/WidgetInspector.java" \
    "Widget Inspector"

check_file "agent/src/main/java/com/robotframework/swt/SwtRpcServer.java" \
    "SWT RPC Server"

echo ""
echo "2. Checking RPC Method Registration..."
echo "---------------------------------------"

check_content "agent/src/main/java/com/robotframework/swt/SwtRpcServer.java" \
    'case "rcp.getComponentTree"' \
    "rcp.getComponentTree method registered"

check_content "agent/src/main/java/com/robotframework/swt/SwtRpcServer.java" \
    'case "rcp.getAllViews"' \
    "rcp.getAllViews method registered"

check_content "agent/src/main/java/com/robotframework/swt/SwtRpcServer.java" \
    'case "rcp.getAllEditors"' \
    "rcp.getAllEditors method registered"

check_content "agent/src/main/java/com/robotframework/swt/SwtRpcServer.java" \
    'case "rcp.getComponent"' \
    "rcp.getComponent method registered"

echo ""
echo "3. Checking Rust Implementation..."
echo "-----------------------------------"

check_file "src/python/swing_library.rs" \
    "Swing Library Rust implementation"

check_content "src/python/swing_library.rs" \
    "get_rcp_component_tree" \
    "get_rcp_component_tree method"

check_content "src/python/swing_library.rs" \
    "get_all_rcp_views" \
    "get_all_rcp_views method"

check_content "src/python/swing_library.rs" \
    "get_all_rcp_editors" \
    "get_all_rcp_editors method"

check_content "src/python/swing_library.rs" \
    "rcp_tree_to_text" \
    "RCP tree formatting helper"

echo ""
echo "4. Checking Test Suite..."
echo "--------------------------"

check_file "tests/python/test_rcp_component_tree.py" \
    "RCP component tree tests"

check_content "tests/python/test_rcp_component_tree.py" \
    "class TestRcpComponentTree" \
    "RCP component tree test class"

check_content "tests/python/test_rcp_component_tree.py" \
    "class TestRcpViewsAndEditors" \
    "RCP views and editors test class"

check_content "tests/python/test_rcp_component_tree.py" \
    "class TestRcpSwtOperations" \
    "RCP SWT operations test class"

check_content "tests/python/test_rcp_component_tree.py" \
    "def test_rcp_swt_widget_inheritance" \
    "SWT widget inheritance test"

echo ""
echo "5. Checking Documentation..."
echo "-----------------------------"

check_file "docs/RCP_COMPONENT_TREE_GUIDE.md" \
    "RCP Component Tree Guide"

check_file "docs/PHASE_6_RCP_IMPLEMENTATION_SUMMARY.md" \
    "Phase 6 Implementation Summary"

check_content "docs/RCP_COMPONENT_TREE_GUIDE.md" \
    "get_rcp_component_tree" \
    "API documentation for get_rcp_component_tree"

check_content "docs/RCP_COMPONENT_TREE_GUIDE.md" \
    "SWT Operation Inheritance" \
    "SWT operation inheritance documentation"

echo ""
echo "6. Checking Implementation Details..."
echo "--------------------------------------"

check_content "agent/src/main/java/com/robotframework/swt/RcpComponentInspector.java" \
    "buildWorkbenchWindowNode" \
    "Workbench window traversal"

check_content "agent/src/main/java/com/robotframework/swt/RcpComponentInspector.java" \
    "buildViewNode" \
    "View node building"

check_content "agent/src/main/java/com/robotframework/swt/RcpComponentInspector.java" \
    "buildEditorNode" \
    "Editor node building"

check_content "agent/src/main/java/com/robotframework/swt/RcpComponentInspector.java" \
    "swtControlId" \
    "SWT widget ID exposure"

check_content "agent/src/main/java/com/robotframework/swt/RcpComponentInspector.java" \
    "DisplayHelper.syncExecAndReturn" \
    "Thread-safe execution"

echo ""
echo "7. Checking SWT Integration..."
echo "-------------------------------"

check_content "agent/src/main/java/com/robotframework/swt/RcpComponentInspector.java" \
    "WidgetInspector.getOrCreateId" \
    "Widget ID management integration"

check_content "agent/src/main/java/com/robotframework/swt/RcpComponentInspector.java" \
    "WidgetInspector.getWidgetTree" \
    "SWT widget tree integration"

check_content "agent/src/main/java/com/robotframework/swt/RcpComponentInspector.java" \
    "EclipseWorkbenchHelper" \
    "Eclipse Workbench Helper usage"

echo ""
echo "8. Checking Error Handling..."
echo "------------------------------"

check_content "agent/src/main/java/com/robotframework/swt/RcpComponentInspector.java" \
    "isEclipseAvailable" \
    "RCP availability check"

check_content "agent/src/main/java/com/robotframework/swt/RcpComponentInspector.java" \
    "available.*false" \
    "Graceful degradation"

check_content "tests/python/test_rcp_component_tree.py" \
    "TestRcpErrorHandling" \
    "Error handling tests"

echo ""
echo "========================================="
echo "Verification Summary"
echo "========================================="
echo -e "${GREEN}Passed:${NC}  $PASS"
echo -e "${RED}Failed:${NC}  $FAIL"
echo -e "${YELLOW}Warnings:${NC} $WARN"
echo ""

if [ $FAIL -eq 0 ]; then
    echo -e "${GREEN}✓ Phase 6 implementation is complete and properly structured${NC}"
    exit 0
else
    echo -e "${RED}✗ Phase 6 implementation has missing components${NC}"
    exit 1
fi
