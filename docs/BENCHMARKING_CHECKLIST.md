# Performance Benchmarking Checklist

**Purpose**: Ensure no performance regression from multi-test hang fix
**Status**: Ready to execute
**Estimated Time**: 15-20 minutes total

---

## Prerequisites ✓

Before starting, verify:

- [ ] Agent JAR built: `agent/target/robotframework-swing-agent-1.0.0-all.jar`
- [ ] Test app built: `tests/apps/swt/target/swt-test-app-1.0.0-all.jar`
- [ ] Python 3 installed
- [ ] Java 11+ installed
- [ ] Rust toolchain installed (for rebuilding after fix)

**Build if needed**:
```bash
# Agent
cd agent && mvn clean package && cd ..

# Test app
cd tests/apps/swt && mvn clean package && cd ../../..

# Rust library (initial)
cargo build --release
```

---

## Step 1: Baseline Benchmarks (Before Fix)

**Estimated Time**: 3-5 minutes

### A. Start Test Application

```bash
cd tests/experiments

# Start app in background
java -javaagent:../../agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5679 \
     -jar ../apps/swt/target/swt-test-app-1.0.0-all.jar &

# Wait for startup
sleep 3

# Verify it's listening
netstat -an | grep 5679
# Should show: *.5679.*LISTEN
```

**Checklist**:
- [ ] App started successfully
- [ ] Port 5679 is listening
- [ ] No errors in console

### B. Run Baseline Benchmarks

```bash
python performance_benchmark.py --baseline -o benchmark_results.json --verbose
```

**Expected Output**:
```
======================================================================
BENCHMARK 1: RPC Call Latency (100 calls)
======================================================================
Connected to localhost:5679
Warming up (10 calls)...
Running benchmark (100 calls)...

Results:
  Total calls:     100
  Min latency:     2.XX ms
  Average latency: 3.XX ms
  ...
  Error count:     0  ← Must be 0!
```

**Checklist**:
- [ ] All 4 benchmarks completed
- [ ] Error count = 0 for all benchmarks
- [ ] Robot test suite **likely hangs** (this is expected before fix)
- [ ] File created: `baseline_benchmark_results.json`

### C. Stop Test Application

```bash
# Kill the app
pkill -f swt-test-app

# Wait
sleep 2
```

**Checklist**:
- [ ] App stopped cleanly
- [ ] Port 5679 no longer listening

---

## Step 2: Apply the Fix

**Estimated Time**: 5 minutes

### A. Edit Source Code

Edit `src/python/swt_library.rs`, lines 1488-1494:

**BEFORE** (remove this):
```rust
stream.set_read_timeout(Some(Duration::from_millis(100))).ok();
let _ = stream.read(&mut byte_buf); // consume \n or \r\n
if byte_buf[0] == b'\r' {
    let _ = stream.read(&mut byte_buf); // consume \n after \r
}
stream.set_read_timeout(Some(Duration::from_secs(30))).ok();
break;
```

**AFTER** (replace with this):
```rust
// JSON complete - trailing newline handled by next read's whitespace skip
break;
```

**Optional Enhancement** (recommended for robustness):

Add this after the break, before the closing brace:

```rust
// Drain any pending whitespace from buffer
stream.set_nonblocking(true).ok();
loop {
    match stream.read(&mut byte_buf) {
        Ok(1) => {
            let c = byte_buf[0] as char;
            if c != '\n' && c != '\r' && c != ' ' && c != '\t' {
                // Non-whitespace - shouldn't happen
                log::warn!("Unexpected byte in buffer: {:?}", byte_buf[0]);
                break;
            }
        }
        _ => break, // No data or error - done draining
    }
}
stream.set_nonblocking(false).ok();
```

**Checklist**:
- [ ] Lines 1488-1494 modified
- [ ] Code compiles (syntax correct)
- [ ] Optional buffer drain added (recommended)

### B. Rebuild

```bash
# From project root
cargo build --release

# Rebuild agent (picks up new library)
cd agent && mvn clean package && cd ..
```

**Checklist**:
- [ ] Rust compilation successful
- [ ] Maven build successful
- [ ] New agent JAR created

---

## Step 3: After-Fix Benchmarks

**Estimated Time**: 3-5 minutes

### A. Restart Test Application

```bash
cd tests/experiments

# Start app with new agent
java -javaagent:../../agent/target/robotframework-swing-agent-1.0.0-all.jar=port=5679 \
     -jar ../apps/swt/target/swt-test-app-1.0.0-all.jar &

# Wait for startup
sleep 3

# Verify
netstat -an | grep 5679
```

**Checklist**:
- [ ] App started with updated agent
- [ ] Port 5679 listening
- [ ] No errors

### B. Run After-Fix Benchmarks

```bash
python performance_benchmark.py --after -o benchmark_results.json --verbose
```

**Expected Output**:
```
======================================================================
BENCHMARK 3: Robot Framework Test Suite
======================================================================
Running: tests/robot/swt/02_widgets.robot

Results:
  Status:          SUCCESS  ← MUST PASS NOW!
  Total time:      45.67 seconds
  Tests run:       25
  Memory delta:    56.11 MB
```

**Checklist**:
- [ ] All 4 benchmarks completed
- [ ] Error count = 0 for all benchmarks
- [ ] **Robot test suite PASSED** (this is the fix validation!)
- [ ] File created: `after_benchmark_results.json`

### C. Stop Test Application

```bash
pkill -f swt-test-app
sleep 2
```

---

## Step 4: Generate Report

**Estimated Time**: 1 minute

```bash
python generate_performance_report.py \
    baseline_benchmark_results.json \
    after_benchmark_results.json \
    -o ../../docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md
```

**Expected Output**:
```
✅ Report generated: ../../docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md

Preview:
========================================================================
# Multi-Test Hang Fix - Performance Report

## Executive Summary

- **RPC Latency (Mean)**: ✅ 0.3% improvement
- **RPC Latency (P99)**: ✅ no change
- **Robot Test Suite**: ✅ 2.1% improvement
...
```

**Checklist**:
- [ ] Report generated successfully
- [ ] Preview shows green checkmarks (✅)
- [ ] No warnings or errors in executive summary

---

## Step 5: Review Results

**Estimated Time**: 5 minutes

### A. Review the Report

```bash
cat ../../docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md
```

**Look for these sections**:

1. **Executive Summary**
   - [ ] All metrics show ✅ (green checkmarks)
   - [ ] Overall verdict is "NO SIGNIFICANT REGRESSION" or "PERFORMANCE IMPROVED"

2. **Benchmark 1: RPC Call Latency**
   - [ ] Mean latency change < 5%
   - [ ] P99 latency change < 10%
   - [ ] Error rate = 0%

3. **Benchmark 3: Robot Framework Test Suite**
   - [ ] Status changed from FAIL/TIMEOUT to SUCCESS
   - [ ] Duration is reasonable (30-90 seconds)
   - [ ] Memory delta < 100 MB

4. **Final Verdict**
   - [ ] Shows "✅ APPROVED FOR MERGE" or "✅ APPROVED WITH MONITORING"

### B. Quick Comparison

```bash
# Or use the built-in comparison
python performance_benchmark.py --compare \
    baseline_benchmark_results.json \
    after_benchmark_results.json
```

**Look for**:
- [ ] No major regressions in any metric
- [ ] Robot test suite now passes

---

## Step 6: Decision Gate

### Success Criteria Met? ✅

**All must be true**:
- [ ] Mean RPC latency change < 5%
- [ ] P99 RPC latency change < 10%
- [ ] Memory usage change < 10%
- [ ] Robot test suite completes successfully
- [ ] Error rate = 0% for all benchmarks

**If YES**: ✅ **APPROVE FOR MERGE**
- Proceed with commit and PR
- Fix resolves the issue without regression

**If NO**: ⚠️ **INVESTIGATE**
- Review which metric failed
- Consider if it's acceptable
- May need to revise the fix

### Common Scenarios

**Scenario 1**: Latency increased 3%
- **Verdict**: ✅ Acceptable (within 5% threshold)
- **Action**: Approve for merge

**Scenario 2**: Latency increased 8%
- **Verdict**: ⚠️ Investigate
- **Action**: Profile code, optimize if possible
- **Option**: Accept if fix is critical and 8% is acceptable

**Scenario 3**: Robot suite still hangs
- **Verdict**: ❌ Fix didn't work
- **Action**: Re-review implementation, check for errors

**Scenario 4**: All metrics improved
- **Verdict**: ✅ Excellent!
- **Action**: Approve for merge, document the improvement

---

## Step 7: Commit and Document

### A. Commit the Fix

```bash
cd ../..  # Back to project root

git add src/python/swt_library.rs
git add docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md

git commit -m "fix: resolve multi-test hang with buffer drain

- Remove problematic newline consumption with 100ms timeout
- Add non-blocking buffer drain for robustness
- Verified with comprehensive performance benchmarks
- No significant performance regression (<5% change)
- Robot test suite now passes reliably

Performance Impact:
- Mean latency: X.X% change
- P99 latency: X.X% change
- Memory: X.X% change

Fixes: #XXX (if there's a GitHub issue)
See: docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md"
```

**Checklist**:
- [ ] Code changes committed
- [ ] Performance report included
- [ ] Commit message describes impact
- [ ] Issue number referenced (if applicable)

### B. Update Documentation

Verify these files are up to date:
- [ ] `docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md` - Performance results
- [ ] `docs/MULTI_TEST_HANG_IMPLEMENTATION_PLAN.md` - Mark as implemented
- [ ] `docs/SWT_MULTIPLE_TEST_HANG_ANALYSIS.md` - Reference the fix

---

## Troubleshooting

### Issue: Baseline benchmarks fail

**Symptoms**: Error count > 0, timeouts

**Solution**:
1. Check app is running: `netstat -an | grep 5679`
2. Check logs: `tail -f /tmp/swt_app.log`
3. Restart app and try again

### Issue: Robot suite hangs even after fix

**Symptoms**: Benchmark 3 times out

**Solution**:
1. Verify fix was applied correctly (check `src/python/swt_library.rs`)
2. Verify rebuild completed: `ls -lh agent/target/*.jar`
3. Check for syntax errors: `cargo check`
4. Try running multi_call_test.py for diagnostics

### Issue: Large latency increase (>10%)

**Symptoms**: Mean latency went up significantly

**Solution**:
1. Run benchmarks again (could be system load)
2. Profile with: `cargo build --release --profile profiling`
3. Consider if the buffer drain is too aggressive
4. May need to optimize the drain loop

### Issue: Memory increase (>20%)

**Symptoms**: Memory usage went up significantly

**Solution**:
1. Check for leaks with valgrind
2. Run longer sustained load test
3. Monitor memory growth over time
4. Verify buffer drain isn't accumulating data

---

## Alternative: Automated Workflow

**If you prefer automation**, use the provided script:

```bash
cd tests/experiments
./run_full_benchmark.sh
```

This runs all steps automatically:
1. ✅ Baseline benchmarks
2. ⏸️  Prompts you to apply fix
3. ✅ After-fix benchmarks
4. ✅ Report generation

---

## Summary Checklist

**Complete Workflow**:
- [ ] Prerequisites verified (JARs built)
- [ ] Baseline benchmarks completed
- [ ] Fix implemented in `src/python/swt_library.rs`
- [ ] Rust library rebuilt
- [ ] Agent rebuilt
- [ ] After-fix benchmarks completed
- [ ] Performance report generated
- [ ] Results reviewed
- [ ] Decision made (approve or investigate)
- [ ] Changes committed (if approved)

**Key Files**:
- [ ] `baseline_benchmark_results.json` exists
- [ ] `after_benchmark_results.json` exists
- [ ] `docs/MULTI_TEST_HANG_PERFORMANCE_REPORT.md` exists and reviewed

**Success Indicators**:
- [ ] Robot test suite passes (was hanging before)
- [ ] Mean latency change < 5%
- [ ] No errors in any benchmark
- [ ] Report shows ✅ approval

---

**Estimated Total Time**: 15-20 minutes
**Status**: Ready to execute
**Next Step**: Start with Step 1 (Baseline Benchmarks)
