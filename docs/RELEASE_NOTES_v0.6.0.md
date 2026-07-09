# Release Notes — v0.6.0

Focused on **click and locator fidelity** on real-world Swing applications, proven by
automating a third-party design-library showcase end to end. Builds on v0.5.0.

## Highlights

### Locator engine reaches the whole component tree
The Swing locator previously fetched the component tree at the agent's default depth of 10,
so `Find Element`/`Click` could only reach shallow components — deeply nested widgets in real
applications were invisible to the library (only ~44 of 279+ components on the test app).
The agent now returns the **full tree** when no depth is requested, which also fixes the
surprisingly shallow default of `Get UI Tree`.

### Clicks behave like real user clicks (event retargeting)
`Click` dispatched a synthetic mouse event straight to the located component. Real OS clicks
are delivered via AWT's `LightweightDispatcher`, which routes the event to the nearest
component *that has mouse listeners*. So clicking a listener-less child — e.g. a label
rendered on a clickable "card" whose `MouseListener` performs the action — did nothing,
even though a user can click it. `Click`, `Double Click`, and `Right Click` now **retarget**
to the nearest listener-bearing ancestor (translating the point), so
`Click  Label[text='...']` activates the handler a user would trigger. Guiding invariant:
**if a user can steer the app by clicking, the library can too.**

### Locator chain fixes
- **Deep child chains:** `A > B > C` (three or more levels) now match. Previously every
  compound was checked against the target's immediate parent, so only two-level `>` chains
  worked.
- **Capture filtering:** `*A >> B` now returns only the captured `A` elements whose subtree
  actually contains a matching `B` — not every `A`.

### Proven on a real third-party app
An opt-in proof suite (`tests/robot/showcase/`, self-skips without the jar) automates the
JGoodies Smart Client Showcase and validates every action by reading the app state back:
text entry, navigation, checkbox, combobox, and tile-card navigation (which exercises the
retargeting fix). This is how all of the above defects were found.

## Compatibility

- No breaking keyword changes since v0.5.0. Existing suites are unaffected: the full Swing
  suite runs with 0 failures, Python unit tests 617 passed, Rust tests 245 passed.
- The keyword reference (`docs/keywords/*.html`) has been regenerated for this release
  (Swing ~108, SWT ~71, RCP ~75 keywords).

## Known issues (carried over)

- Connection stability under heavy load / on Windows can still produce transient
  `Broken pipe` / connection-timeout errors; wrap long flows with a reconnect guard
  (`Is Connected`).
- A few keywords remain backed by missing/limited agent support (see
  `RELEASE_NOTES_v0.5.0.md`): `Swt Tree Node Should Exist`, `Get Swt Tree Node Count`,
  list-selection getters, `Log Component Tree`. `List Applications` intentionally raises.
- Individual radio/table/tree widgets in the showcase sit behind deeper pivot
  sub-navigation; the navigation that reaches those pages now works, but driving those
  specific widgets end-to-end is future work.
