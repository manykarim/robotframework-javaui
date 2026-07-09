# Smart Client Showcase — action-validation proof

This opt-in suite proves the library drives a **real third-party** Java Swing app — the
JGoodies **Smart Client Showcase 24.09.0** — and that **every performed action is
independently confirmed by reading the application's state back**. A green run is evidence
the action truly happened on the UI, not merely that the keyword returned.

## How to run

The showcase jar is a third-party binary kept out of git (`example-apps/*.jar` is
gitignored). Place it at `example-apps/smart-client-showcase-24.09.0.jar`, then:

```bash
# build the agent once
mvn -f agent/pom.xml package

# headless (Linux/CI)
xvfb-run -a uv run robot -d results/showcase tests/robot/showcase/

# with a display
uv run robot -d results/showcase tests/robot/showcase/
```

If the jar (or the built agent) is absent, the suite **self-skips** — it never fails a
default run.

## What is proven

Each test performs an action and then reads the app back through an *independent* keyword:

| Category | Action | Independent validation |
|----------|--------|------------------------|
| **Text entry** | `Input Text` / `Clear Text` on the search field | `Get Element Text` equals the typed text / is empty |
| **Navigation** | click a `NavigationToggleButton` | target toggle `Element Should Be Selected`, other one not |
| **Checkbox** | `Check` / `Uncheck` on the Settings checkbox | `Element Should Be Selected` / `... Not Be Selected` |
| **Combo box** | `Select From Combobox` | `Element Text Should Be` the selected value |
| **Deep reach** | — | `Find Elements //*` sees the full deep tree (regression guard for the depth fix below) |

## Findings surfaced during this work

Per the proof's contract, defects found are reported, not hidden:

1. **FIXED — locator engine was capped at tree depth 10.** The Swing locator fetched the
   component tree with no depth argument, and the agent defaulted to depth 10
   (`ComponentInspector.java`). Real apps nest interactive widgets well below depth 10, so
   `find`/`click` could reach only ~44 of 279+ components — the showcase's content was
   unreachable. The agent now returns the **full tree** when no depth is requested, which
   also fixes the surprisingly shallow default of `Get UI Tree`. This is the enabling fix
   for everything above.

2. **Limitation — the showcase's tile navigation is not activatable via the locator.** The
   Start "hub" navigates to demo pages through custom, unnamed nested `JPanel` tile cards
   (with a `com.jgoodies…$FormsLabel` label). Clicking the label — including double-click —
   does not trigger navigation, and the cards carry no stable name/id. As a result the
   descriptive demo pages (which might host radio buttons, tables, or trees) could not be
   reached on this particular app. The categories proven above are those reachable via the
   working `NavigationToggleButton` navigation and the top-level search field. Radio/table/
   tree validation was therefore not possible against *this* app; it is covered against the
   library's own test apps in `tests/robot/swing/`.

3. **Minor — `Element Text Should Be` and `Get Element Text` diverge on the search field.**
   `Get Element Text` returns the entered value; `Element Text Should Be` read empty. The
   suite uses `Get Element Text` for text validation. Worth reconciling in the library.
