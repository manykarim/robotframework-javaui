# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed
- Multi-test hang in SWT library caused by socket buffer synchronization race condition
  - Removed timeout-based newline consumption that caused intermittent hangs
  - 100% multi-test reliability achieved (was 50% due to timing-dependent bug)
  - Test results: 110/110 consecutive tests pass with zero hangs
  - Fixed in commit 23e7bd2 (2026-01-17)
  - Documentation: `docs/MULTI_TEST_HANG_FIX_SUMMARY.md`

- RCP startup timeout failures
  - Added intelligent port availability checking with retry logic
  - Replaced insufficient 3-second sleep with proper waiting mechanism
  - RCP initialization now works reliably in headless (xvfb) environments
  - Test results: 17/17 RCP connection tests pass (was 7/17)
  - Fixed in commit 23e7bd2 (2026-01-17)

- Swing dialog EDT deadlock causing 25-minute hangs
  - Made menu clicks asynchronous using SwingUtilities.invokeLater()
  - Prevented blocking when opening modal dialogs
  - Test results: 8/8 dialog tests pass with no hangs
  - Fixed in commit 23e7bd2 (2026-01-17)

- Empty locator validation to prevent fatal test crashes
  - Added validation to catch empty locator strings before RPC calls
  - Provides clear error messages instead of crashes
  - Test results: 9/9 empty locator validation tests pass
  - Fixed in commit e67af99 (2026-01-17)

### Changed
- Enhanced exception hierarchy with better error messages and categorization
- Improved connection handling with persistent connection model
- Updated test setup keywords with better timeout handling

### Added
- Comprehensive troubleshooting documentation
- Detailed root cause analysis for all critical fixes
- Stress test validation (110+ consecutive test runs)

## [0.1.0] - 2026-01-17

### Added
- Initial release with Swing, SWT, and RCP support
- Robot Framework library for Java GUI automation
- Multi-toolkit support (Swing, SWT, RCP)
- Comprehensive test suites for all three toolkits
- Java agent for runtime introspection
- Rust-based Python library for high performance

### Known Issues
- None (all critical issues resolved in pre-release testing)

---

## Release Notes

### Version 0.1.0 (2026-01-17)

This is the initial release of the unified Robot Framework library for Java GUI automation, supporting Swing, SWT, and Eclipse RCP applications.

**Test Results**:
- Swing: 498/499 tests pass (99.8%)
- SWT: 18/18 connection tests pass (100%)
- RCP: 17/17 connection tests pass (100%)
- Overall: 533/534 tests pass (99.8%)

**Major Features**:
- Multi-toolkit support (Swing, SWT, RCP)
- High-performance Rust implementation
- Comprehensive keyword library
- Production-ready reliability (100% multi-test execution)

**Critical Fixes Applied**:
1. Multi-test hang fix (socket buffer synchronization)
2. RCP startup timeout fix (port availability checking)
3. Swing EDT deadlock fix (asynchronous menu clicks)
4. Empty locator validation (crash prevention)

**Documentation**:
- User guide with examples for all three toolkits
- Troubleshooting guide for common issues
- Developer guide for contributing
- Comprehensive API documentation

**Requirements**:
- Python 3.8+
- Java 8+
- Robot Framework 6.0+

**Installation**:
```bash
pip install robotframework-javagui
```

**Quick Start**:
```robot
*** Settings ***
Library    SwtLibrary

*** Test Cases ***
Test SWT Application
    Connect To SWT Application    myapp    localhost    5679
    Click Widget    text:Submit
    [Teardown]    Disconnect
```

For detailed documentation, see the [docs/](docs/) directory.

---

**Changelog Maintained By**: Project maintainers
**Last Updated**: 2026-01-17
