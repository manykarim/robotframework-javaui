#!/bin/bash
# Test runner script for robotframework-swing

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

echo "========================================"
echo "Robot Framework Swing Library Tests"
echo "========================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Parse command line arguments
RUN_PYTHON=true
RUN_ROBOT=true
RUN_RUST=true
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --python-only)
            RUN_ROBOT=false
            RUN_RUST=false
            shift
            ;;
        --robot-only)
            RUN_PYTHON=false
            RUN_RUST=false
            shift
            ;;
        --rust-only)
            RUN_PYTHON=false
            RUN_ROBOT=false
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --python-only   Run only Python unit tests"
            echo "  --robot-only    Run only Robot Framework tests"
            echo "  --rust-only     Run only Rust tests"
            echo "  -v, --verbose   Verbose output"
            echo "  -h, --help      Show this help"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

cd "$PROJECT_ROOT"

# Run Rust tests
if [ "$RUN_RUST" = true ]; then
    echo -e "\n${YELLOW}Running Rust tests...${NC}"
    if command -v cargo &> /dev/null; then
        if [ "$VERBOSE" = true ]; then
            cargo test --verbose
        else
            cargo test
        fi
        echo -e "${GREEN}✓ Rust tests passed${NC}"
    else
        echo -e "${YELLOW}⚠ Cargo not found, skipping Rust tests${NC}"
    fi
fi

# Run Python unit tests
if [ "$RUN_PYTHON" = true ]; then
    echo -e "\n${YELLOW}Running Python unit tests...${NC}"
    cd "$PROJECT_ROOT/tests/python"

    if [ "$VERBOSE" = true ]; then
        python -m pytest -v --tb=long
    else
        python -m pytest -v --tb=short
    fi

    echo -e "${GREEN}✓ Python tests passed${NC}"
    cd "$PROJECT_ROOT"
fi

# Run Robot Framework tests
if [ "$RUN_ROBOT" = true ]; then
    echo -e "\n${YELLOW}Running Robot Framework tests...${NC}"

    # Check if demo app JAR exists
    DEMO_JAR="$PROJECT_ROOT/demo/target/swing-demo-1.0.0.jar"
    if [ ! -f "$DEMO_JAR" ]; then
        echo -e "${YELLOW}⚠ Demo app JAR not found. Building...${NC}"
        cd "$PROJECT_ROOT/demo"
        if command -v mvn &> /dev/null; then
            mvn package -DskipTests
            cd "$PROJECT_ROOT"
        else
            echo -e "${RED}✗ Maven not found, cannot build demo app${NC}"
            echo "  Please install Maven or build manually."
            cd "$PROJECT_ROOT"
        fi
    fi

    # Run Robot tests
    cd "$PROJECT_ROOT/tests/robot"

    if [ "$VERBOSE" = true ]; then
        robot --loglevel DEBUG .
    else
        robot --loglevel INFO .
    fi

    echo -e "${GREEN}✓ Robot Framework tests passed${NC}"
    cd "$PROJECT_ROOT"
fi

echo -e "\n${GREEN}========================================"
echo "All tests completed successfully!"
echo -e "========================================${NC}"
