## 0. Scope deviation — a real bug had to be fixed first

- [x] 0.1 Discovery revealed the Swing locator engine was capped at component-tree depth **10** (agent `ComponentInspector.getComponentTree()` / `RpcServer` defaulted `maxDepth` to 10), so `find`/`click` reached only ~44 of 279+ components — the showcase's deep widgets were unreachable. FIXED: the agent now returns the **full tree** when no depth is requested (also fixes the shallow `Get UI Tree` default). This crossed the proposal's "no code changes" non-goal — user approved. Files: `agent/.../ComponentInspector.java`, `agent/.../RpcServer.java`; agent rebuilt.

## 1. Discovery — ground-truth the 24.09.0 app

- [x] 1.1 Launched headless; confirmed agent/connect port **18080** (Swing toolkit auto-detected)
- [x] 1.2 Connected and dumped the start-page tree (hub of `FormsLabel` tiles + `JGSearchField` + `NavigationToggleButton` Start/Settings)
- [x] 1.3 Surveyed sections: the app is a *design library* showcase. `NavigationToggleButton` → Settings works (has a `JComboBox` + `JCheckBox`); tile navigation (custom unnamed `$FormsLabel` cards) does NOT activate via click — see finding 2.4
- [x] 1.4 Recorded working locators: `JGSearchField` (text), `NavigationToggleButton[text='Start'|'Settings']`, Settings `JComboBox`/`JCheckBox`

## 2. Showcase resource (launch / connect / navigate / helpers)

- [x] 2.1 `tests/robot/showcase/resources/showcase.resource` with variables + `Launch Showcase` / `Close Showcase` (Start Process + connect-with-retry; escaped the `=` in the javaagent arg)
- [x] 2.2 `Ensure Connected` reconnect guard + `Open Section` / `Navigate And Verify Section` (click toggle → refresh → assert selected)
- [x] 2.3 Opt-in self-skip in `Launch Showcase` (skips if jar/agent absent) — verified 5/5 skipped when jar missing
- [x] 2.4 Validated-action helpers: `Enter And Verify Text`, `Clear And Verify Empty`, `Navigate And Verify Section`, `Select Radio And Verify`, `Check And Verify`, `Uncheck And Verify`, `Select Combo And Verify` (radio/table helpers included for reuse though not reachable on this app)

## 3. Proof tests — validated per reachable action category

- [x] 3.1 Text entry: `Input Text` → `Get Element Text` equals typed; `Clear Text` → empty (on `JGSearchField`) — PASS
- [x] 3.2 Navigation: Start↔Settings via `NavigationToggleButton`; target toggle selected, other not — PASS
- [~] 3.3 Radio buttons: NOT reachable on this app (no radios on the reachable Start/Settings surfaces; demo pages unreachable — finding 2.4). Covered against the library's own apps in `tests/robot/swing/`
- [x] 3.4 Checkboxes: check → `Element Should Be Selected`; uncheck → `... Not Be Selected` (Settings checkbox) — PASS
- [x] 3.5 Combo: read value → re-select same value → `Element Text Should Be` unchanged (Settings combo) — PASS
- [~] 3.6 Table/tree: NOT reachable on this app (design-library pages unreachable). Covered in `tests/robot/swing/`
- [x] 3.7 Button effect: navigation IS a button-triggered visible change and is validated (3.2); a standalone status-label button was not present on reachable pages

## 4. Run, verify, document

- [x] 4.1 Ran the proof suite headless (Xvfb) against the live showcase — `02_validated_actions.robot` **5/5 pass**; `01_smoke.robot` **3/3 pass**
- [x] 4.2 Every reachable category has a passing validated test; the unreachable categories (radio/table/tree) are surfaced as findings, not hidden or faked
- [x] 4.3 Added `tests/robot/showcase/README.md` (how to obtain the jar, how to run, what is proven, and the 3 findings)
- [x] 4.4 Verified self-skip: 5 skipped / 0 failed with the jar absent
