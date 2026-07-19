# javagui-spy

Generate unique, verified Robot Framework locators for a running Java app — Swing, SWT, or Eclipse RCP. Point it at an instrumented app, ask it for a locator, and it hands you one that is guaranteed to parse and resolve, because it was checked through the exact same matcher your test will run.

---

## What it is, and why you want it

Writing keywords is easy. Writing *locators* is the part that hurts — especially against off-the-shelf apps you didn't build, where nothing has a helpful `name`, widgets nest fourteen levels deep, and the one attribute you keyed on turns out to match six other components.

The usual loop is: guess a locator, run the suite, watch it fail with `zero matches` or `ambiguous`, tweak, repeat. `javagui-spy` collapses that loop. It scans the live component tree, ranks candidate locators, and verifies each one through the production `find_elements` matcher *before* it suggests it. A locator it hands you cannot fail to parse, and it tells you up front whether the match is unique.

It ships inside the `robotframework-javagui` wheel. Installing the wheel gives you the `javagui-spy` command.

- **Stateless.** No daemon, no session. Every verb does connect → work → disconnect and prints one JSON envelope.
- **JSON-first.** Every command emits `{ok, command, data, meta}` on stdout.
- **Exit-code-driven.** `validate` returns codes an agent can branch on without parsing output.

---

## Install

```bash
pip install robotframework-javagui
```

That's it — the `javagui-spy` command is on your path:

```bash
javagui-spy schema
```

`schema` needs no app and no connection. If it prints a JSON document of verbs and a grammar cheatsheet, you're installed.

---

## Quick start

You need a Java app running with the javagui agent attached. Two ways in.

### Connect to an already-running app

If the app is already up with the agent listening (default ports: **5678** Swing, **5679** SWT/RCP):

```bash
javagui-spy dump-tree --toolkit swing --port 5678
```

### Or let spy launch it for you

Skip the agent wiring — `--launch JAR` starts the app with the bundled agent and connects:

```bash
javagui-spy dump-tree --launch /path/to/app.jar --toolkit swing
```

Connection flags are available on every verb except `schema`:

| Flag | Default | Meaning |
|------|---------|---------|
| `--host` | `localhost` | Agent host |
| `--port` | `5678` swing / `5679` swt·rcp | Agent port |
| `--toolkit` | `swing` | `swing` \| `swt` \| `rcp` |
| `--timeout` | `30` | Connect timeout, seconds |
| `--launch JAR` | — | Launch JAR with the bundled agent instead of connecting |

Every command prints one envelope:

```json
{
  "ok": true,
  "command": "dump-tree",
  "data": [ ... ],
  "meta": { "toolkit": "swing", "tree_timestamp": 1720000000000 }
}
```

---

## The five-call agent workflow

The intended path from "somewhere in this window" to "a locator I trust." Each step feeds the next.

### 1. `dump-tree` — orient

Get the visible component tree as compact rows. Invisible nodes are excluded by default; add `--all` to include them.

```bash
javagui-spy dump-tree
```

```json
{
  "ok": true,
  "command": "dump-tree",
  "data": [
    { "node_id": 3, "type": "JFrame",   "name": "main",           "text": null,  "depth": 0,
      "bounds": { "x": 0,   "y": 0,  "w": 900, "h": 640 } },
    { "node_id": 6, "type": "JToolBar", "name": "mainToolBar",    "text": null,  "depth": 3,
      "bounds": { "x": 0,   "y": 24, "w": 900, "h": 32  } },
    { "node_id": 7, "type": "JButton",  "name": "toolbarNewButton","text": "New", "depth": 4,
      "bounds": { "x": 4,   "y": 28, "w": 60,  "h": 24  } }
  ],
  "meta": { "toolkit": "swing", "tree_timestamp": 1720000000000 }
}
```

### 2. `find` — shortlist

Resolve a rough locator to see what it hits and how many. Great for narrowing by text before you commit.

```bash
javagui-spy find "text:New"
```

```json
{
  "ok": true,
  "command": "find",
  "data": {
    "locator": "text:New",
    "match_count": 1,
    "matches": [
      { "node_id": 7, "type": "JButton", "name": "toolbarNewButton", "text": "New", "depth": 4,
        "bounds": { "x": 4, "y": 28, "w": 60, "h": 24 } }
    ]
  },
  "meta": { "toolkit": "swing", "tree_timestamp": 1720000000000 }
}
```

`match_count` is your signal: `1` is a keeper; more than one means keep narrowing.

### 3. `suggest` — get ranked, verified candidates

Hand it the `node_id` you care about. It returns candidates already run through the matcher, ranked by score, plus ready-to-paste RF snippets.

```bash
javagui-spy suggest --node-id 7
```

```json
{
  "ok": true,
  "command": "suggest",
  "data": {
    "target": { "node_id": 7, "type": "JButton", "name": "toolbarNewButton", "text": "New", "depth": 4,
                "bounds": { "x": 4, "y": 28, "w": 60, "h": 24 } },
    "candidates": [
      { "locator": "JButton[name='toolbarNewButton']",                          "strategy": "single",   "match_count": 1, "unique": true, "score": 0.91 },
      { "locator": "JToolBar[name='mainToolBar'] >> JButton[name='toolbarNewButton']", "strategy": "anchored", "match_count": 1, "unique": true, "score": 0.83 },
      { "locator": "JButton[text='New']",                                        "strategy": "single",   "match_count": 1, "unique": true, "score": 0.82 }
    ],
    "rf_snippets": {
      "click":              "Click    JButton[name='toolbarNewButton']",
      "get_text":           "${value}=    Get Element Text    JButton[name='toolbarNewButton']",
      "should_be_visible":  "Element Should Be Visible    JButton[name='toolbarNewButton']"
    }
  },
  "meta": { "toolkit": "swing", "tree_timestamp": 1720000000000 }
}
```

Two flags shape the output:

- `--top N` — return more (or fewer) candidates. Default `3`.
- `--strip-names` — pretend the widgets have no `name` attributes, forcing structural `>>` anchored chains. Use this to simulate an off-the-shelf app before you have to test one for real.

Each candidate carries the full contract: `locator`, `strategy`, `match_count`, `unique`, `stability`, `score`, `brittle_flags`, `preconditions`. Take the top one whose `unique` is `true`.

### 4. `validate` — confirm, via exit code

Prove the chosen locator resolves to exactly one node. This is the step an agent branches on — the exit code *is* the answer, no output parsing required.

```bash
javagui-spy validate "JButton[name='toolbarNewButton']"
echo $?   # 0 = unique, done
```

```json
{
  "ok": true,
  "command": "validate",
  "data": {
    "locator": "JButton[name='toolbarNewButton']",
    "match_count": 1,
    "unique": true,
    "matches_expected": null,
    "matches": [
      { "node_id": 7, "type": "JButton", "name": "toolbarNewButton", "text": "New", "depth": 4,
        "bounds": { "x": 4, "y": 28, "w": 60, "h": 24 } }
    ]
  },
  "meta": { "toolkit": "swing", "tree_timestamp": 1720000000000 }
}
```

Pin it to a specific node with `--expect-id` when you want "unique *and* the right one":

```bash
javagui-spy validate "JButton[text='New']" --expect-id 7
```

`matches_expected` reports whether the single match was the node you named.

### 5. `screenshot` — visual confirmation

Capture the app (or a widget region) to a PNG when you want eyes on it.

```bash
javagui-spy screenshot -o /tmp/app.png
```

```json
{
  "ok": true,
  "command": "screenshot",
  "data": { "path": "/tmp/app.png" },
  "meta": { "toolkit": "swing", "tree_timestamp": 1720000000000 }
}
```

### Bonus: `describe` — inspect one node

When `suggest` isn't enough and you want the raw truth about a node — full identity, geometry, state, and the ancestor breadcrumb up to the root:

```bash
javagui-spy describe --node-id 7
```

```json
{
  "ok": true,
  "command": "describe",
  "data": {
    "target": { "node_id": 7, "type": "JButton", "name": "toolbarNewButton", "text": "New", "depth": 4,
                "bounds": { "x": 4, "y": 28, "w": 60, "h": 24 } },
    "identity": { "name": "toolbarNewButton", "class": "javax.swing.JButton" },
    "geometry": { "x": 4, "y": 28, "width": 60, "height": 24 },
    "state": { "visible": true, "enabled": true, "focused": false },
    "component_type": "JButton",
    "ancestors": [
      { "node_id": 6, "type": "JToolBar", "name": "mainToolBar", "text": null },
      { "node_id": 3, "type": "JFrame",   "name": "main",        "text": null }
    ]
  },
  "meta": { "toolkit": "swing", "tree_timestamp": 1720000000000 }
}
```

---

## Verbs reference

Every verb except `schema` accepts the connection flags above and prints the `{ok, command, data, meta}` envelope.

| Verb | Key args | Does | Notable exit codes |
|------|----------|------|--------------------|
| `schema` | — | Print all verbs + the locator grammar cheatsheet. No app needed. Bootstrap for agents. | `0` |
| `dump-tree` | `--all` | Compact node rows: `node_id, type, name, text, bounds, depth`. Visible-only unless `--all`. | `0` / `2` |
| `find` | `LOCATOR` | Resolve a locator → matching nodes + `match_count`. | `0` / `2` |
| `validate` | `LOCATOR` `--expect-id N` | Resolve and judge uniqueness. Exit code is the verdict. | `0` unique · `3` zero · `4` ambiguous · `2` error |
| `suggest` | `--node-id N` `--top 3` `--strip-names` | Ranked, verified locator candidates + RF snippets for a node. | `0` / `2` |
| `describe` | `--node-id N` | Properties, geometry, state + ancestor breadcrumb for a node. | `0` / `2` |
| `screenshot` | `-o FILE` | Capture a PNG of the app/widget. | `0` / `2` |

Deep, unnamed widgets are handled automatically: `suggest` falls back to nearest-stable-ancestor `>>` anchored chains when a node has nothing unique of its own.

---

## Locator grammar cheatsheet

The same grammar Robot Framework's `find_elements` speaks — which is exactly why a verified suggestion can't fail in a test. Straight from `javagui-spy schema`:

| Locator | Matches |
|---------|---------|
| `JButton[name='ok']` | By component type + `name` attribute |
| `JButton[text='Save']` | By type + visible text |
| `#okButton` | Shorthand for `name='okButton'` |
| `text:Login` | Any component whose text is `Login` |
| `JToolBar[name='main'] >> JButton[text='Save']` | Anchored chain: Save button *inside* the main toolbar |
| `//JButton[@text='OK']` | XPath-style attribute match |
| `JPanel:has(JLabel[text='Total']) >> JTextField` | The text field in the panel that contains a "Total" label |
| `JButton:nth-of-type(2)` | The second matching button among siblings |

`>>` is the anchoring combinator — read it as "then, inside." Chains are how you pin down a widget that isn't unique on its own but *is* unique under a stable ancestor. For SWT/RCP, swap the type names (`Button`, `Text`, `Tree`, …); the grammar is identical.

---

## Exit codes

`validate` is the branch point; other verbs use `0`/`2`.

| Code | Meaning |
|------|---------|
| `0` | Success. For `validate`: the locator is **unique** (and matched `--expect-id` if given). |
| `2` | Parse / usage / transport error. Something was malformed or the connection failed. See the `error` object in the envelope. |
| `3` | `validate` only: **zero matches**. The locator resolves to nothing. |
| `4` | `validate` only: **ambiguous**. The locator matches more than one node. |

An error envelope looks like:

```json
{ "ok": false, "command": "validate",
  "error": { "code": "SPY_ERROR", "message": "NODE_GONE: node id 42 not in current tree; re-run dump-tree" } }
```

---

## For AI agents

This tool was built to be driven by a program, not just a person.

- **It's stateless.** No session to keep alive, no cursor to manage. Each call reconnects, does its work, and disconnects. Treat every invocation as independent — and re-run `dump-tree` if you suspect the UI changed, since `node_id`s are bound to the tree snapshot in `meta.tree_timestamp` (a `NODE_GONE` error means the tree moved under you).
- **Everything is JSON.** Read `ok`, `command`, `data`, `meta` off stdout. Bootstrap yourself with `javagui-spy schema` — it needs no app and returns the full verb list and grammar in one shot.
- **Branch on exit codes, not prose.** `validate` gives you `0`/`3`/`4`/`2` directly. Loop it: `suggest` → take the top `unique` candidate → `validate` → if `0`, you're done; if `3` or `4`, narrow and retry. No natural-language parsing required to know whether you succeeded.

A tight agent loop:

```bash
javagui-spy find "text:Save"                 # shortlist → grab node_id
javagui-spy suggest --node-id 12 --top 1     # ranked, pre-verified
javagui-spy validate "JButton[name='saveButton']"   # exit 0 → commit it
```
