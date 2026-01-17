# Quick Test Guide - RPC Hang Fix Validation

## TL;DR - Run These Tests

### 1. Quick Validation (30 seconds)
```bash
# Run all Python experiments
xvfb-run -a uv run python tests/experiments/multi_call_test.py
xvfb-run -a uv run python tests/experiments/robot_simulation_test.py
xvfb-run -a uv run python tests/experiments/trace_hang.py
```

### 2. Robot Framework Validation (2 minutes)
```bash
# Single run
xvfb-run -a uv run robot --outputdir /tmp/validation tests/experiments/validation_no_hang.robot

# 10x loop (comprehensive)
for i in {1..10}; do
  xvfb-run -a uv run robot --outputdir /tmp/loop$i tests/experiments/validation_no_hang.robot
done
```

### 3. Full Stress Test (5 minutes)
```bash
# All SWT tests
xvfb-run -a uv run robot --outputdir /tmp/stress tests/robot/swt/
```

## Expected Results

### ✅ Success Criteria
- All Python tests pass (9/9 experiments)
- Robot validation passes all 4 tests
- 10x loop completes with ZERO hangs
- Stress test shows ZERO timeouts (some feature failures OK)

### ⚠️ Known Issues (Not Hang-Related)
- Some tests fail due to missing `getWidgetProperties` method
- Some combo/list selection tests fail (parameter handling)
- These are feature gaps, NOT hang issues

## Test File Locations

```
tests/experiments/
├── multi_call_test.py           # 6 RPC experiments
├── robot_simulation_test.py     # 3 RF simulation tests
├── trace_hang.py                # Hang detection with timing
├── validation_no_hang.robot     # Robot Framework validation suite
├── VALIDATION_REPORT.md         # Full test report
└── QUICK_TEST_GUIDE.md          # This file

tests/robot/swt/
├── 01_connection.robot          # Connection tests
├── 02_widgets.robot             # Widget interaction tests
├── 03_*.robot                   # Additional test suites
└── resources/common.resource    # Shared resources
```

## What Each Test Validates

### multi_call_test.py
- Basic RPC connection (ping)
- Multiple sequential calls
- Different methods in sequence
- Rapid fire (no delay between calls)
- Library-level API usage

### robot_simulation_test.py
- Robot Framework GLOBAL scope behavior
- Cache management between tests
- Connection reuse vs. new connections

### trace_hang.py
- Execution timing
- Thread state monitoring
- Hang detection with timeout

### validation_no_hang.robot
- Multiple findWidgets calls
- Mixed RPC methods
- Rapid sequential calls (10 iterations)
- Sequential test simulation (10 tests)

## Troubleshooting

### If Tests Fail
1. Check if SWT test app is running:
   ```bash
   ps aux | grep SwtTestApp
   ```

2. Verify port 5679 is available:
   ```bash
   netstat -an | grep 5679
   ```

3. Check logs in `/tmp/validation*/log.html`

### If Tests Hang
**This should NOT happen with the fix!**

If hangs occur:
1. Save thread dumps: `jstack <pid>`
2. Check RPC server logs
3. Report to development team

## Performance Benchmarks

Expected execution times (with fix):

| Test | Expected Time |
|------|---------------|
| multi_call_test.py | < 5 seconds |
| robot_simulation_test.py | < 3 seconds |
| trace_hang.py | < 1 second |
| validation_no_hang.robot | < 5 seconds |
| 10x loop | < 60 seconds |
| Full stress test | < 5 minutes |

## CI/CD Integration

### Pre-commit Hook
```bash
#!/bin/bash
# Run quick validation before commit
xvfb-run -a uv run python tests/experiments/multi_call_test.py
xvfb-run -a uv run robot tests/experiments/validation_no_hang.robot
```

### PR Validation
```bash
#!/bin/bash
# Run full validation for PR
for i in {1..10}; do
  xvfb-run -a uv run robot --outputdir /tmp/pr_validation$i \
    tests/experiments/validation_no_hang.robot
  if [ $? -ne 0 ]; then exit 1; fi
done
```

## Test Maintenance

### When to Update Tests
- Adding new RPC methods → Update multi_call_test.py
- Changing connection logic → Update robot_simulation_test.py
- Adding new widgets → Update validation_no_hang.robot

### When to Add New Tests
- New synchronization patterns
- New threading models
- Performance-critical features
