# Tasks

Status: implemented + verified live against DBeaver. Reachability flipped from **all-0** to
**Text=4, Button=4, Composite=41, Label=27, Tree=5** — F9 fixed.

## 1. Fix
- [x] 1.1 Added `"type"` to recognized prefixes in `src/python/swt_library.rs::parse_locator` (both `=` and `:` branches)
- [x] 1.2 Added `"type"` to recognized prefixes in `src/python/base_library.rs::parse_locator` (both `=` and `:` branches)
- [x] 1.3 Rebuilt `_core` (`maturin develop --release`)

## 2. Verify
- [~] 2.1 Rust unit test for `parse_locator("type:X")` — `parse_locator` is a private `&self` method with no existing test harness; covered instead by the live reachability guard (2.2). Left as follow-up if a test seam is added.
- [x] 2.2 Live harness: `Find Widgets type:Shell` 0→3 (limbo / "DBeaver 26.1.2" main window / "Statistics collection" modal); reachability guard asserts Composite/Text/Button > 0
- [x] 2.3 Live harness: text field reachable (4 Texts found, was 0); combo/table/toolitem documented as different SWT classes in DBeaver
- [x] 2.4 Regression: `cargo test` + `pytest` (see run) + widget suite 6/6 green
- [x] 2.5 Non-`type:` locators unchanged (text:/name: still work — modal automation still green)

## 3. Follow-up notes
- [x] 3.1 Updated the F9 note in `rcp-real-app-automation` to the true root cause (client-side `type` omission), not "main window unreachable"
