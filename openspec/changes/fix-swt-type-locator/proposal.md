## Why

The widget-level DBeaver experiment found (F9) that generic `Find Widget`/`Find Widgets` reached
widgets inside a modal dialog (via `text:` locators) but returned **0 results for every
`type:` locator** in the main Eclipse-4 workbench window — `type:Text`, `type:Button`,
`type:Combo`, even `type:Shell` on 3 live shells.

Root cause (isolated, not e4-specific): the Rust `parse_locator` in **both**
`src/python/swt_library.rs:1610` and `src/python/base_library.rs:1148` recognizes the prefixes
`class | name | text | index | id` but **omits `type`**. So `type:Shell` matches none of the
arms, falls through to the default branch, and is mangled into `("class", "type:Shell")` — a
class name that matches no widget. The agent supports a `type` locator (`SwtReflectionBridge`
`case "type"`), but Rust never sends it. Because `get_widget_id` uses the same parser, this also
breaks `Click Widget`/`Input Text`/`Check Button`/etc. with `type:` locators, not just finding.

`text:`, `name:`, and `class:` locators are unaffected — which is exactly the pattern the
experiment showed (modal `text:Confirm` worked; every `type:*` returned 0).

## What Changes

- Add `"type"` to the recognized locator prefixes in both `parse_locator` implementations
  (the `=` and `:` branches), so `type:Shell` → `("type", "Shell")` and reaches the agent's
  existing `type` matcher.
- Verify against real DBeaver in the Docker harness: `type:*` finders now return the main
  workbench window's widgets (the F9 reachability suite flips from 0 to non-zero).

## Capabilities

### New Capabilities
- `swt-type-locator`: `type:<SwtClass>` locators resolve for SWT/RCP widget finding and actions,
  matching on the widget's SWT class simple name.

## Impact

- **Rust:** `src/python/swt_library.rs`, `src/python/base_library.rs` (`parse_locator`). Rebuild
  `_core` (`maturin develop`).
- **Tests:** the `tests/robot/rcp/dbeaver_widgets/` reachability + widget scenarios now find
  main-window widgets; add a Rust unit test for `parse_locator("type:X")`.
- **No agent change** — the agent already handles the `type` locator; only the Rust client was
  dropping it.
- Corrects the earlier F9 framing ("main window unreachable") to the true cause (a client-side
  locator-parsing omission).
