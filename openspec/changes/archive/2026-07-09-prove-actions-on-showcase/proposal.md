## Why

The library's own test apps are purpose-built for it. A skeptic can reasonably ask: does it actually drive a *real, third-party* Java Swing application, and does each keyword truly change the app's state — or does it just "not error"? We have a real-world target on hand (the JGoodies **Smart Client Showcase 24.09.0**, `example-apps/smart-client-showcase-24.09.0.jar`). This change builds a **validation-first proof suite** against it where **every action is independently confirmed by reading the app back** — so a green run is evidence the action truly happened on the UI, not merely that the call returned.

## What Changes

- Add a Robot Framework proof suite (under `tests/robot/showcase/`) that launches the Smart Client Showcase with the agent, connects, and exercises real user actions with an assertion after **each** one:
  - **Text entry** → after `Input Text`, read the field back (`Get Element Text` / `Element Text Should Be`) and assert it equals what was typed; `Clear Text` → assert empty.
  - **Navigation** → after navigating to a section, assert the new section/page is showing (section title via `Element Text Should Be` and/or `Element Should Be Visible` on a page-specific element), and that the previous section's marker is no longer showing.
  - **Radio buttons** → after `Select Radio Button`, assert `Element Should Be Selected`; assert the sibling radios are `Element Should Not Be Selected`.
  - **Checkboxes** → after `Check Checkbox` assert selected; after `Uncheck Checkbox` assert not selected.
  - **Combo boxes / lists** → after selecting an item, assert the selected value/index matches.
  - **Tables / trees** → after selecting a row/node or expanding a node, assert the selection/expansion state via getters.
  - **Buttons** → after a click that changes visible state, assert the resulting change (e.g. a status label, dialog, or enabled/disabled transition).
- Add a launcher/resource (`tests/robot/showcase/resources/showcase.resource`) with robust connect (agent port, panel-expansion waits, `Refresh Ui Tree` after navigation) and a self-skip guard when the showcase jar is absent (it is gitignored), so the suite is opt-in and never breaks default CI.
- Add a short doc/README describing how to run the proof suite and what each test proves.
- **Non-goal:** no library/agent code changes are expected; if a genuine defect is found while proving actions, it is reported as a finding (and fixed under a separate change), not silently worked around.

## Capabilities

### New Capabilities
- `showcase-action-proof`: A proof suite that automates a real third-party Swing app and validates every performed action by reading the app's state back; defines the assert-after-every-action contract, the categories of action that must be covered, and the opt-in/self-skip behavior.

### Modified Capabilities
<!-- None — this is an additive test/demonstration suite; no existing spec's requirements change. -->

## Impact

- **Tests:** new `tests/robot/showcase/` suite(s) + `resources/showcase.resource`; opt-in (self-skips without the jar).
- **Docs:** a short runner/README for the proof suite; optionally referenced from the main README as evidence.
- **Build/CI:** no change to the default gate (suite self-skips); optionally a dedicated opt-in job that supplies the showcase jar.
- **Runtime deps:** the showcase jar is a third-party artifact kept out of git (`example-apps/*.jar` is gitignored); the suite documents how to obtain/point at it.
- **Code:** none expected in `python/`, `src/`, or `agent/` (findings, if any, are raised separately).
