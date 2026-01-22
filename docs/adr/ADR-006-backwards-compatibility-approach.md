# ADR-006: Backwards Compatibility Approach

| ADR ID | ADR-006 |
|--------|---------|
| Title | Backwards Compatibility Approach |
| Status | Proposed |
| Date | 2026-01-16 |
| Authors | Architecture Team |

## Context

The unification of Swing, SWT, and RCP keywords will introduce significant API changes. Existing users have tests written against the current API that must continue to work.

### Scope of Changes

| Change Type | Count | Impact |
|-------------|-------|--------|
| Renamed keywords | ~25 | High - tests will fail |
| New unified keywords | ~45 | Low - additive |
| Deprecated keywords | ~20 | Medium - should warn |
| Removed keywords | 0 (v2.x) | None in initial release |
| Exception renames | ~6 | Medium - exception handlers |
| Library class changes | 3 | Medium - import statements |

### Affected Areas

1. **Keyword Names**: Widget -> Element, Item -> Node, etc.
2. **Library Classes**: SwingLibrary, SwtLibrary, RcpLibrary
3. **Exception Classes**: SwingConnectionError, etc.
4. **Locator Syntax**: Technology-specific variations
5. **Import Paths**: Python module structure

### Decision Drivers

- Existing tests must not break on upgrade
- Clear migration path for users
- Reasonable deprecation timeline
- Tooling support for migration
- Documentation of all changes

## Decision

We will implement a **Three-Phase Deprecation Strategy** with **Automated Migration Tools**.

### 1. Version Strategy

```
Version 2.0.x - Current (status quo)
Version 2.1.x - Unified API (with full backwards compatibility)
Version 2.2.x - Deprecation warnings enabled
Version 3.0.x - Legacy aliases removed (breaking)
```

### 2. Phase 1: Full Backwards Compatibility (v2.1)

All existing APIs work exactly as before, plus new unified APIs available.

```rust
// Macro for generating backwards-compatible keyword
macro_rules! unified_keyword {
    (
        primary: $primary:ident,
        aliases: [$($alias:ident),*],
        impl: $impl_fn:expr
    ) => {
        /// Primary (canonical) keyword name
        #[pyo3(name = stringify!($primary))]
        pub fn $primary(&self, locator: &str) -> PyResult<()> {
            $impl_fn(self, locator)
        }

        $(
            /// Backwards-compatible alias
            #[pyo3(name = stringify!($alias))]
            pub fn $alias(&self, locator: &str) -> PyResult<()> {
                // In v2.1: no warning, just works
                // In v2.2: deprecation warning
                // In v3.0: removed
                #[cfg(feature = "deprecation_warnings")]
                self.warn_deprecated(stringify!($alias), stringify!($primary));

                $impl_fn(self, locator)
            }
        )*
    };
}

impl JavaGuiLibrary {
    unified_keyword!(
        primary: click,
        aliases: [click_element, click_widget],
        impl: |s: &Self, loc: &str| s.shared.click_impl(loc)
    );

    unified_keyword!(
        primary: find_element,
        aliases: [find_widget],
        impl: |s: &Self, loc: &str| s.shared.find_element_impl(loc)
    );

    // ... more keywords
}
```

### 3. Phase 2: Deprecation Warnings (v2.2)

Enable warnings for deprecated keywords:

```rust
impl JavaGuiLibrary {
    /// Issue deprecation warning
    fn warn_deprecated(&self, old_name: &str, new_name: &str) {
        Python::with_gil(|py| {
            if let Ok(warnings) = py.import("warnings") {
                let message = format!(
                    "Keyword '{}' is deprecated and will be removed in version 3.0. \
                     Use '{}' instead. See migration guide: \
                     https://docs.example.com/migration",
                    old_name, new_name
                );

                // Use DeprecationWarning for IDE integration
                let _ = warnings.call_method1(
                    "warn",
                    (
                        message,
                        py.get_type::<pyo3::exceptions::PyDeprecationWarning>(),
                        2,  // Stack level
                    )
                );
            }
        });
    }
}
```

### 4. Library Class Backwards Compatibility

```rust
// Primary unified library
#[pyclass(name = "JavaGuiLibrary")]
pub struct JavaGuiLibrary { /* ... */ }

// Backwards-compatible wrappers
#[pyclass(name = "SwingLibrary")]
pub struct SwingLibrary {
    inner: JavaGuiLibrary,
}

#[pymethods]
impl SwingLibrary {
    #[new]
    #[pyo3(signature = (timeout=10.0, poll_interval=0.5, screenshot_directory="."))]
    pub fn new(timeout: f64, poll_interval: f64, screenshot_directory: &str) -> PyResult<Self> {
        #[cfg(feature = "deprecation_warnings")]
        warn_library_deprecated("SwingLibrary", "JavaGuiLibrary with mode='swing'");

        Ok(Self {
            inner: JavaGuiLibrary::new_with_mode("swing", timeout)?,
        })
    }

    // Delegate all methods to inner
    pub fn click(&self, locator: &str) -> PyResult<()> {
        self.inner.click(locator)
    }

    // Legacy method names also work
    pub fn click_element(&self, locator: &str) -> PyResult<()> {
        self.inner.click_element(locator)
    }

    // ... all other methods
}

#[pyclass(name = "SwtLibrary")]
pub struct SwtLibrary {
    inner: JavaGuiLibrary,
}

#[pymethods]
impl SwtLibrary {
    #[new]
    pub fn new(timeout: Option<f64>) -> PyResult<Self> {
        #[cfg(feature = "deprecation_warnings")]
        warn_library_deprecated("SwtLibrary", "JavaGuiLibrary with mode='swt'");

        Ok(Self {
            inner: JavaGuiLibrary::new_with_mode("swt", timeout.unwrap_or(10.0))?,
        })
    }

    // Legacy SWT method names
    pub fn click_widget(&self, locator: &str) -> PyResult<()> {
        self.inner.click(locator)
    }

    pub fn find_widget(&self, locator: &str) -> PyResult<SwtElement> {
        self.inner.find_element(locator)
    }

    // ... all other methods
}

#[pyclass(name = "RcpLibrary")]
pub struct RcpLibrary {
    inner: JavaGuiLibrary,
}

#[pymethods]
impl RcpLibrary {
    #[new]
    pub fn new(timeout: Option<f64>) -> PyResult<Self> {
        #[cfg(feature = "deprecation_warnings")]
        warn_library_deprecated("RcpLibrary", "JavaGuiLibrary with mode='rcp'");

        Ok(Self {
            inner: JavaGuiLibrary::new_with_mode("rcp", timeout.unwrap_or(10.0))?,
        })
    }

    // RCP-specific keywords
    pub fn open_perspective(&self, perspective_id: &str) -> PyResult<()> {
        self.inner.open_perspective(perspective_id)
    }

    // Inherited SWT keywords
    pub fn click_widget(&self, locator: &str) -> PyResult<()> {
        self.inner.click(locator)
    }

    // ... all other methods
}
```

### 5. Exception Backwards Compatibility

```rust
pub fn register_exceptions(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // New unified exceptions (primary)
    m.add("JavaGuiError", py.get_type::<JavaGuiError>())?;
    m.add("ConnectionError", py.get_type::<ConnectionError>())?;
    m.add("ElementNotFoundError", py.get_type::<ElementNotFoundError>())?;
    m.add("ActionTimeoutError", py.get_type::<ActionTimeoutError>())?;
    // ... other new exceptions

    // Legacy exception aliases (deprecated in v2.2, removed in v3.0)
    m.add("SwingConnectionError", py.get_type::<ConnectionError>())?;
    m.add("SwingTimeoutError", py.get_type::<ActionTimeoutError>())?;
    m.add("PyLocatorParseError", py.get_type::<LocatorParseError>())?;

    Ok(())
}
```

### 6. Automated Migration Tool

Create a migration script that updates test files:

```python
#!/usr/bin/env python3
"""
Migration tool for robotframework-javagui v2.x to v3.x

Usage:
    python -m javagui.migrate [options] <path>

Options:
    --dry-run       Show changes without modifying files
    --backup        Create backup files before modification
    --verbose       Show detailed output
    --report        Generate migration report
"""

import re
import sys
from pathlib import Path
from dataclasses import dataclass
from typing import List, Tuple, Optional

# Keyword mapping: old -> new
KEYWORD_MAPPINGS = {
    # SWT -> Unified
    "Click Widget": "Click",
    "Double Click Widget": "Double Click",
    "Right Click Widget": "Right Click",
    "Find Widget": "Find Element",
    "Find Widgets": "Find Elements",
    "Wait Until Widget Exists": "Wait Until Element Exists",
    "Wait Until Widget Enabled": "Wait Until Element Is Enabled",
    "Widget Should Be Visible": "Element Should Be Visible",
    "Widget Should Be Enabled": "Element Should Be Enabled",
    "Widget Text Should Be": "Element Text Should Be",
    "Check Button": "Check",
    "Uncheck Button": "Uncheck",
    "Expand Tree Item": "Expand Tree Node",
    "Collapse Tree Item": "Collapse Tree Node",
    "Select Tree Item": "Select Tree Node",
    "Get Table Cell": "Get Table Cell Value",

    # Minor Swing variations
    "Check Checkbox": "Check",
    "Uncheck Checkbox": "Uncheck",
    "Select Radio Button": "Select Radio",
}

# Library import mapping
LIBRARY_MAPPINGS = {
    "SwingLibrary": ("JavaGuiLibrary", "mode=swing"),
    "SwtLibrary": ("JavaGuiLibrary", "mode=swt"),
    "RcpLibrary": ("JavaGuiLibrary", "mode=rcp"),
}

# Exception mapping
EXCEPTION_MAPPINGS = {
    "SwingConnectionError": "ConnectionError",
    "SwingTimeoutError": "ActionTimeoutError",
    "PyLocatorParseError": "LocatorParseError",
}

@dataclass
class MigrationChange:
    file: Path
    line_number: int
    old_text: str
    new_text: str
    change_type: str

class Migrator:
    def __init__(self, dry_run: bool = False, backup: bool = True):
        self.dry_run = dry_run
        self.backup = backup
        self.changes: List[MigrationChange] = []

    def migrate_file(self, file_path: Path) -> List[MigrationChange]:
        """Migrate a single Robot Framework file."""
        changes = []

        if not file_path.exists():
            return changes

        content = file_path.read_text()
        lines = content.split('\n')
        modified_lines = []

        for i, line in enumerate(lines, 1):
            new_line, line_changes = self._migrate_line(line, file_path, i)
            modified_lines.append(new_line)
            changes.extend(line_changes)

        if changes and not self.dry_run:
            if self.backup:
                backup_path = file_path.with_suffix(file_path.suffix + '.bak')
                backup_path.write_text(content)

            file_path.write_text('\n'.join(modified_lines))

        self.changes.extend(changes)
        return changes

    def _migrate_line(self, line: str, file_path: Path, line_num: int) -> Tuple[str, List[MigrationChange]]:
        """Migrate a single line."""
        changes = []
        new_line = line

        # Check for library imports
        for old_lib, (new_lib, args) in LIBRARY_MAPPINGS.items():
            pattern = rf'Library\s+{old_lib}(\s|$)'
            if re.search(pattern, line):
                new_line = re.sub(
                    pattern,
                    f'Library    {new_lib}    {args}',
                    new_line
                )
                changes.append(MigrationChange(
                    file=file_path,
                    line_number=line_num,
                    old_text=line.strip(),
                    new_text=new_line.strip(),
                    change_type="library_import"
                ))

        # Check for keyword usage
        for old_kw, new_kw in KEYWORD_MAPPINGS.items():
            # Match keyword at start of line (after spaces/pipes)
            pattern = rf'(\s*\|?\s*){re.escape(old_kw)}(\s|\||$)'
            if re.search(pattern, new_line, re.IGNORECASE):
                new_line = re.sub(
                    pattern,
                    rf'\1{new_kw}\2',
                    new_line,
                    flags=re.IGNORECASE
                )
                changes.append(MigrationChange(
                    file=file_path,
                    line_number=line_num,
                    old_text=line.strip(),
                    new_text=new_line.strip(),
                    change_type="keyword_rename"
                ))

        # Check for exception handling
        for old_exc, new_exc in EXCEPTION_MAPPINGS.items():
            if old_exc in new_line:
                new_line = new_line.replace(old_exc, new_exc)
                changes.append(MigrationChange(
                    file=file_path,
                    line_number=line_num,
                    old_text=line.strip(),
                    new_text=new_line.strip(),
                    change_type="exception_rename"
                ))

        return new_line, changes

    def migrate_directory(self, directory: Path, pattern: str = "**/*.robot") -> List[MigrationChange]:
        """Migrate all Robot Framework files in directory."""
        all_changes = []
        for file_path in directory.glob(pattern):
            changes = self.migrate_file(file_path)
            all_changes.extend(changes)
        return all_changes

    def generate_report(self) -> str:
        """Generate migration report."""
        report = ["=" * 60]
        report.append("MIGRATION REPORT")
        report.append("=" * 60)
        report.append("")

        # Summary
        report.append("SUMMARY:")
        report.append(f"  Total changes: {len(self.changes)}")

        by_type = {}
        for change in self.changes:
            by_type[change.change_type] = by_type.get(change.change_type, 0) + 1

        for change_type, count in sorted(by_type.items()):
            report.append(f"  {change_type}: {count}")

        report.append("")

        # Details
        if self.changes:
            report.append("CHANGES:")
            current_file = None
            for change in self.changes:
                if change.file != current_file:
                    report.append(f"\n{change.file}:")
                    current_file = change.file
                report.append(f"  Line {change.line_number}: {change.change_type}")
                report.append(f"    - {change.old_text}")
                report.append(f"    + {change.new_text}")

        return '\n'.join(report)


def main():
    import argparse
    parser = argparse.ArgumentParser(description="Migrate robotframework-javagui tests")
    parser.add_argument("path", help="Path to file or directory")
    parser.add_argument("--dry-run", action="store_true", help="Show changes without modifying")
    parser.add_argument("--backup", action="store_true", default=True, help="Create backups")
    parser.add_argument("--no-backup", action="store_false", dest="backup")
    parser.add_argument("--verbose", action="store_true", help="Verbose output")
    parser.add_argument("--report", action="store_true", help="Generate report")

    args = parser.parse_args()
    path = Path(args.path)

    migrator = Migrator(dry_run=args.dry_run, backup=args.backup)

    if path.is_file():
        changes = migrator.migrate_file(path)
    else:
        changes = migrator.migrate_directory(path)

    if args.verbose or args.report:
        print(migrator.generate_report())
    else:
        print(f"Migrated {len(changes)} items")
        if args.dry_run:
            print("(dry run - no files modified)")


if __name__ == "__main__":
    main()
```

### 7. Deprecation Timeline

| Version | Release | Status | Behavior |
|---------|---------|--------|----------|
| 2.1.0 | Q1 2026 | Current | All old + new APIs work, no warnings |
| 2.2.0 | Q2 2026 | Warnings | Old APIs emit DeprecationWarning |
| 2.3.0 | Q3 2026 | Final warnings | Stronger warnings, migration tool promoted |
| 3.0.0 | Q1 2027 | Breaking | Old APIs removed |

### 8. Documentation Requirements

Create comprehensive migration documentation:

```markdown
# Migration Guide: v2.x to v3.x

## Quick Start

1. Install the migration tool:
   ```bash
   pip install robotframework-javagui[migrate]
   ```

2. Run migration on your test directory:
   ```bash
   python -m javagui.migrate --dry-run tests/
   python -m javagui.migrate tests/
   ```

3. Update library imports in your settings:
   ```robot
   # Old:
   Library    SwingLibrary

   # New:
   Library    JavaGuiLibrary    mode=swing
   ```

## Keyword Changes

| Old Keyword | New Keyword | Notes |
|------------|-------------|-------|
| Click Widget | Click | SWT terminology to unified |
| Find Widget | Find Element | SWT terminology to unified |
| ... | ... | ... |

## Exception Changes

| Old Exception | New Exception |
|--------------|---------------|
| SwingConnectionError | ConnectionError |
| ... | ... |

## Common Issues

### Q: My exception handlers stopped working
A: Update exception names per the table above, or use base `JavaGuiError`.

### Q: Locators stopped working
A: All locator formats are still supported. Check for typos.
```

## Consequences

### Positive

1. **Zero Breaking Changes**: v2.1 works with all existing tests
2. **Gradual Migration**: Users have ~1 year to migrate
3. **Automated Migration**: Tool handles most changes automatically
4. **Clear Communication**: Deprecation warnings guide users
5. **Documentation**: Comprehensive migration guide available

### Negative

1. **Code Duplication**: Must maintain aliases during transition
2. **Longer Transition**: Full deprecation cycle takes ~1 year
3. **Testing Burden**: Must test both old and new APIs
4. **User Confusion**: Two ways to do same thing temporarily

### Risks

1. **Users Ignore Warnings**: May hit breaking changes in v3.0 unprepared
2. **Migration Tool Bugs**: Automated migration may miss edge cases
3. **Documentation Lag**: Docs must be kept in sync with changes

## Alternatives Considered

### Alternative 1: Big Bang Breaking Change

Release v3.0 immediately with all changes, no compatibility.

**Rejected because**:
- Would break all existing tests
- Users have no migration path
- Poor user experience

### Alternative 2: Permanent Aliases

Keep all old names forever as permanent aliases.

**Rejected because**:
- Code bloat and maintenance burden
- Confusing documentation
- Two "correct" ways to do everything

### Alternative 3: Longer Deprecation (2+ years)

Extend deprecation period to 2+ years.

**Rejected because**:
- Delays benefits of unification
- Longer maintenance burden
- Users may never migrate

## Implementation Plan

1. **Phase 1**: Implement keyword aliases (2 weeks)
2. **Phase 2**: Implement library wrapper classes (1 week)
3. **Phase 3**: Implement exception aliases (3 days)
4. **Phase 4**: Create migration tool (1 week)
5. **Phase 5**: Write migration documentation (1 week)
6. **Phase 6**: Add deprecation warning infrastructure (3 days)
7. **Phase 7**: Test backwards compatibility (1 week)
8. **Phase 8**: Release v2.1 (1 day)

## References

- [Python Deprecation Policy](https://www.python.org/dev/peps/pep-0387/)
- [Semantic Versioning](https://semver.org/)
- [Robot Framework Library Development](https://robotframework.org/robotframework/latest/RobotFrameworkUserGuide.html#extending-robot-framework)
- [ADR-003: Keyword Naming Convention](/docs/adr/ADR-003-keyword-naming-convention.md)
- [ADR-005: Error Handling Strategy](/docs/adr/ADR-005-error-handling-strategy.md)
