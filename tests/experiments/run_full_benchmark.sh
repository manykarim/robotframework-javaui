#!/bin/bash
#
# Complete Performance Benchmark Workflow
#
# This script runs the full benchmark workflow:
# 1. Baseline benchmarks (before fix)
# 2. Prompt for fix implementation
# 3. After-fix benchmarks
# 4. Generate comparison report
#
# Usage:
#   ./run_full_benchmark.sh [--quick] [--skip-baseline]
#

set -e  # Exit on error

# Configuration
AGENT_JAR="../../agent/target/robotframework-swing-agent-1.0.0-all.jar"
APP_JAR="../apps/swt/target/swt-test-app-1.0.0-all.jar"
PORT=5679
QUICK_MODE=false
SKIP_BASELINE=false

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --quick)
            QUICK_MODE=true
            shift
            ;;
        --skip-baseline)
            SKIP_BASELINE=true
            shift
            ;;
        *)
            echo "Unknown option: $1"
            echo "Usage: $0 [--quick] [--skip-baseline]"
            exit 1
            ;;
    esac
done

# Helper functions
print_header() {
    echo -e "\n${BLUE}======================================================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}======================================================================${NC}\n"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

check_prerequisites() {
    print_header "Checking Prerequisites"

    # Check Java
    if ! command -v java &> /dev/null; then
        print_error "Java not found. Please install Java 11 or higher."
        exit 1
    fi
    print_success "Java found: $(java -version 2>&1 | head -n 1)"

    # Check Python
    if ! command -v python3 &> /dev/null && ! command -v python &> /dev/null; then
        print_error "Python not found. Please install Python 3."
        exit 1
    fi
    print_success "Python found: $(python3 --version 2>&1 || python --version 2>&1)"

    # Check agent JAR
    if [ ! -f "$AGENT_JAR" ]; then
        print_error "Agent JAR not found: $AGENT_JAR"
        echo "Please build the agent: cd ../../agent && mvn clean package"
        exit 1
    fi
    print_success "Agent JAR found"

    # Check app JAR
    if [ ! -f "$APP_JAR" ]; then
        print_error "Test app JAR not found: $APP_JAR"
        echo "Please build the app: cd ../apps/swt && mvn clean package"
        exit 1
    fi
    print_success "Test app JAR found"

    # Check benchmark script
    if [ ! -f "performance_benchmark.py" ]; then
        print_error "Benchmark script not found: performance_benchmark.py"
        exit 1
    fi
    print_success "Benchmark script found"
}

start_test_app() {
    print_header "Starting SWT Test Application"

    # Kill any existing instance
    pkill -f swt-test-app 2>/dev/null || true
    sleep 2

    # Start app
    java -javaagent:"$AGENT_JAR=port=$PORT" -jar "$APP_JAR" > /tmp/swt_app.log 2>&1 &
    APP_PID=$!
    echo $APP_PID > /tmp/swt_app.pid

    print_success "Started test app (PID: $APP_PID)"

    # Wait for app to be ready
    echo -n "Waiting for app to start"
    for i in {1..10}; do
        sleep 1
        echo -n "."
        if netstat -an 2>/dev/null | grep -q ":$PORT.*LISTEN" || \
           ss -an 2>/dev/null | grep -q ":$PORT.*LISTEN"; then
            echo ""
            print_success "App is listening on port $PORT"
            return 0
        fi
    done

    echo ""
    print_error "App failed to start within 10 seconds"
    print_warning "Check logs: /tmp/swt_app.log"
    exit 1
}

stop_test_app() {
    print_header "Stopping SWT Test Application"

    if [ -f /tmp/swt_app.pid ]; then
        APP_PID=$(cat /tmp/swt_app.pid)
        kill $APP_PID 2>/dev/null || true
        rm /tmp/swt_app.pid
        print_success "Stopped test app (PID: $APP_PID)"
    else
        pkill -f swt-test-app 2>/dev/null || true
        print_success "Stopped any running test apps"
    fi

    sleep 2
}

run_benchmark() {
    local mode=$1
    local output_file=$2

    print_header "Running $mode Benchmarks"

    local quick_flag=""
    if [ "$QUICK_MODE" = true ]; then
        quick_flag="--quick"
        print_warning "Quick mode enabled (faster but less accurate)"
    fi

    # Determine python command
    local python_cmd="python3"
    if ! command -v python3 &> /dev/null; then
        python_cmd="python"
    fi

    # Run benchmark
    if $python_cmd performance_benchmark.py --$mode $quick_flag -o benchmark_results.json --verbose; then
        print_success "$mode benchmarks completed"
        print_success "Results saved to: $output_file"
    else
        print_error "$mode benchmarks failed"
        exit 1
    fi
}

prompt_for_fix() {
    print_header "Apply the Fix"

    echo ""
    echo "Before running after-fix benchmarks, you need to:"
    echo ""
    echo "1. Implement the fix in: src/python/swt_library.rs"
    echo "   - Remove newline consumption code (lines 1488-1494)"
    echo "   - Add non-blocking buffer drain"
    echo ""
    echo "2. Rebuild the library:"
    echo "   cd ../.."
    echo "   cargo build --release"
    echo "   cd agent && mvn clean package && cd .."
    echo "   cd tests/experiments"
    echo ""
    echo "3. Press Enter when ready to continue..."
    echo ""

    read -r

    print_success "Continuing with after-fix benchmarks"
}

generate_report() {
    print_header "Generating Performance Report"

    local baseline=$1
    local after=$2
    local output="../../docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md"

    # Determine python command
    local python_cmd="python3"
    if ! command -v python3 &> /dev/null; then
        python_cmd="python"
    fi

    if $python_cmd generate_performance_report.py "$baseline" "$after" -o "$output"; then
        print_success "Report generated: $output"
        echo ""
        echo "Preview:"
        echo "========================================================================"
        head -n 30 "$output"
        echo "..."
        echo "========================================================================"
        echo ""
        echo "View full report: cat $output"
    else
        print_error "Report generation failed"
        exit 1
    fi
}

cleanup() {
    print_header "Cleanup"
    stop_test_app
    print_success "Cleanup complete"
}

# Trap to ensure cleanup on exit
trap cleanup EXIT

# Main workflow
main() {
    print_header "Multi-Test Hang Fix - Performance Benchmark Workflow"

    if [ "$QUICK_MODE" = true ]; then
        print_warning "Running in QUICK mode (less accurate, faster)"
    fi

    # Check prerequisites
    check_prerequisites

    # Baseline benchmarks
    if [ "$SKIP_BASELINE" = false ]; then
        start_test_app
        run_benchmark "baseline" "baseline_benchmark_results.json"
        stop_test_app
    else
        print_warning "Skipping baseline benchmarks (--skip-baseline)"
        if [ ! -f "baseline_benchmark_results.json" ]; then
            print_error "baseline_benchmark_results.json not found"
            exit 1
        fi
    fi

    # Prompt for fix implementation
    if [ "$SKIP_BASELINE" = false ]; then
        prompt_for_fix
    fi

    # After-fix benchmarks
    start_test_app
    run_benchmark "after" "after_benchmark_results.json"
    stop_test_app

    # Generate report
    generate_report "baseline_benchmark_results.json" "after_benchmark_results.json"

    # Summary
    print_header "Benchmark Workflow Complete"

    echo ""
    echo "Files generated:"
    echo "  - baseline_benchmark_results.json"
    echo "  - after_benchmark_results.json"
    echo "  - ../../docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md"
    echo ""
    echo "Next steps:"
    echo "  1. Review the performance report"
    echo "  2. Check for regressions (mean latency change < 5%)"
    echo "  3. Verify Robot test suite completed successfully"
    echo "  4. Make go/no-go decision for merge"
    echo ""

    print_success "All done!"
}

# Run main workflow
main
