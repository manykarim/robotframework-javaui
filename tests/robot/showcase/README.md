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

2. **Tile navigation IS reachable — via geometry locators today; a click-fidelity gap
   otherwise.** The Start "hub" navigates through custom `JGoodies` tile *cards*
   (`com.jgoodies.fluent.tiles.AbstractTileView`). Decompilation confirmed the card's
   `MouseHandler` is registered on the **card `JPanel`**, not on the visible
   `com.jgoodies…$FormsLabel` — and navigation fires on `mouseReleased`. The library's
   `Click` dispatches a synthetic mouse event **directly to the located component**; when you
   locate the label, that event never reaches the card's listener (a real OS click works
   because AWT's `LightweightDispatcher` retargets the event up to the nearest ancestor that
   *has* mouse listeners — synthetic dispatch does not).

   Consequence: `Click FormsLabel[text='Input']` does nothing, but clicking the **card**
   works. Because the matcher supports geometry attributes (`x/y/width/height`), the tiles
   are reachable **today**:

   ```robotframework
   Click    JPanel[x='232'][y='38'][width='228'][height='112']     # Input tile → "Input Dialogs"
   Click    JPanel[x='232'][y='154'][width='228'][height='112']    # Selection tile → PivotBar + JList
   ```

   Verified live: the Input card opens the "Input Dialogs" page; the Selection card opens a
   `PivotBar` of `NavigationToggleButton`s plus a `JList` — all automatable with existing
   keywords from there. (An earlier note here wrongly called this "unreachable"; that was a
   detection artifact — the destination pages hold `ReadOnlyTextField`/sub-hubs, not the
   `JRadioButton`/`JTable` types being counted, so a successful navigation was misread.)

   **RESOLVED (by `click-retargeting-and-locator-fixes`).** The agent now retargets a
   synthetic click to the nearest listener-bearing ancestor (replicating
   `LightweightDispatcher`), so `Click FormsLabel[text='Input']` navigates directly — no
   geometry coordinates needed. `03_tile_navigation.robot` proves this live: the tile click
   reaches the "Input Dialogs" demo page, different tiles reach different pages, and the
   accompanying locator-engine fixes (capture filtering; deep `>` chains) resolve against the
   live app. Driving the individual radio/table/tree widgets that sit behind each demo page's
   deeper pivot sub-navigation is left as further showcase work; the navigation that unlocks
   them now works.

3. **Minor — `Element Text Should Be` and `Get Element Text` diverge on the search field.**
   `Get Element Text` returns the entered value; `Element Text Should Be` read empty. The
   suite uses `Get Element Text` for text validation. Worth reconciling in the library.
