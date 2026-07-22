# Agent Usage Cheatsheet

Example-first reference for automation agents *using* `robotframework-javagui` to drive Java
Swing / SWT / Eclipse RCP apps. Every item leads with a runnable snippet — copy, adapt, run.

- Working **on** this repo (building it)? That contract is [`AGENTS.md`](../AGENTS.md) — the canonical
  source. This file does not repeat it.
- Full keyword reference (every keyword, every arg, with examples): generated libdoc at
  [`docs/keywords/Swing.html`](keywords/Swing.html) · [`Swt.html`](keywords/Swt.html) ·
  [`Rcp.html`](keywords/Rcp.html). When in doubt about a signature, look there.

---

## 1. Get connected — Connect vs Attach vs Web Start

**You launched the app yourself** (added `-javaagent`) → `Connect To Application`. Lowest overhead.

```robotframework
*** Settings ***
Library    JavaGui.Swing

*** Test Cases ***
Connect To A Launched App
    # App was started: java -javaagent:javagui-agent.jar=port=5678 -jar your-app.jar
    Connect To Application    main_class=com.example.MyApp    port=5678
    Click    JButton[text='Login']
    [Teardown]    Disconnect
```

**The app is already running** (no `-javaagent`, someone else launched it) → `Attach To Application`.
It injects the agent into the live JVM via the JDK Attach API. Needs a JDK (or `jattach`) on the test host.

```robotframework
Attach To A Running App
    Attach To Application    main_class=testapp.SwingTestApp    # or pid=... or title='*Editor*'
    Click    JButton[text='OK']
    [Teardown]    Disconnect
```

Not sure what is attachable? List first, then pick:

```robotframework
${apps}=    List Applications          # each is a dict: pid, main_class, command_line, display_name, ...
Attach To Application    pid=${apps}[0][pid]
```

**Java Web Start / JNLP** (launcher strips `-javaagent`) → `Launch Web Start Application`. Attaches at runtime.

```robotframework
Automate A Web Start App
    Launch Web Start Application    /path/to/app.jnlp    toolkit=auto
    Click    JButton[text='Start']
    [Teardown]    Disconnect
```

> SWT/RCP work identically — import `JavaGui.Swt` / `JavaGui.Rcp` (they default `toolkit=swt`; Swing
> defaults `toolkit=swing`). `toolkit=auto` on attach detects Swing vs SWT from loaded classes.
> Full model, JDK/launcher matrix, IcedTea-Web block, troubleshooting: [`docs/runtime-attach.md`](runtime-attach.md).

---

## 2. Locator grammar — one runnable example per form

The Rust matcher and `javagui-spy` speak the **same** grammar, so a locator the spy verifies cannot
fail to parse in a test. For SWT/RCP, swap the Swing type names (`Button`, `Text`, `Tree`, …).

| Form | Locator | Runnable line |
|------|---------|---------------|
| by `name` | `JButton[name='ok']` | `Click    JButton[name='ok']` |
| by `text` | `JButton[text='Save']` | `Click    JButton[text='Save']` |
| id shorthand (`name=`) | `#okButton` | `Click    #okButton` |
| text: strategy | `text:Login` | `Click    text:Login` |
| child (direct) `>` | `JPanel[name='form'] > JTextField` | `Input Text    JPanel[name='form'] > JTextField    hi` |
| cascaded / anchored `>>` | `JToolBar[name='main'] >> JButton[text='Save']` | `Click    JToolBar[name='main'] >> JButton[text='Save']` |
| capture `*` in cascade | `*JPanel >> JLabel[text='Total']` | `${n}=    Get Element Count    *JPanel >> JLabel[text='Total']` |
| XPath-style | `//JButton[@text='OK']` | `Click    //JButton[@text='OK']` |
| `:has(...)` | `JPanel:has(JLabel[text='Total']) >> JTextField` | `Input Text    JPanel:has(JLabel[text='Total']) >> JTextField    42` |
| `:nth-of-type(n)` | `JButton:nth-of-type(2)` | `Click    JButton:nth-of-type(2)` |
| geometry attrs | `JPanel[x='232'][y='38'][width='228'][height='112']` | `Click    JPanel[x='232'][y='38'][width='228'][height='112']` |

Read `>>` as "then, inside" — the way to pin a widget that isn't unique on its own but *is* unique
under a stable ancestor. Attribute operators also work: `[text*='Sub']` (contains), `[name^='btn_']`
(starts), `[text$='...']` (ends), plus `:enabled` / `:visible` / `:first-child` / `:nth-child(n)`.

---

## 3. The VERIFY-LOOP — never guess a locator

Writing keywords is easy; writing *locators* is the part that hurts. Don't hand-roll and hope —
let `javagui-spy` hand you a locator it already ran through the production matcher. It ships in the
wheel (`pip install robotframework-javagui`), is stateless, and prints one JSON envelope per call.

**Bootstrap (no app needed)** — prints every verb + the locator grammar + the candidate contract:

```bash
javagui-spy schema
```

**The loop:** orient → find → (get suggestions) → validate on exit code.

```bash
# 1. ORIENT — dump the visible component tree as compact rows (node_id, type, name, text, bounds, depth)
javagui-spy dump-tree --toolkit swing --port 5678

# 2. FIND — resolve a rough locator, see match_count. 1 = keeper; >1 = keep narrowing
javagui-spy find "text:Save"

# 3. STUCK? SUGGEST — ranked, pre-verified candidates + ready-to-paste rf_snippets for a node_id
javagui-spy suggest --node-id 7            # add --top 1 for just the best, --strip-names to force >> chains

# 4. VALIDATE — the branch point. The EXIT CODE is the verdict, no output parsing:
javagui-spy validate "JButton[name='saveButton']"
echo $?     # 0 = unique (done) · 3 = zero matches · 4 = ambiguous · 2 = parse/usage/transport error
```

| `validate` exit | Meaning | Do |
|-----------------|---------|-----|
| `0` | **unique** — resolves to exactly one node | commit it |
| `3` | **zero matches** | locator resolves to nothing — loosen it |
| `4` | **ambiguous** — matches >1 node | narrow it (add a `>>` anchor, a `name`, a `:nth-of-type`) |
| `2` | parse / usage / transport error | fix the locator syntax or the connection |

Pin to a specific node with `--expect-id`: `javagui-spy validate "JButton[text='New']" --expect-id 7`.
Other useful verbs: `describe --node-id N` (full identity + geometry + state + ancestor breadcrumb),
`screenshot -o proof.png`, `pick --at X,Y` (in-JVM hit-test), `highlight`, `ui` (web inspector).

**Attach the spy to an already-running app** (same discover-and-inject path, no `-javaagent`):

```bash
javagui-spy dump-tree --attach-pid 48213 --toolkit auto
javagui-spy suggest  --attach-main-class 'testapp.SwingTestApp' --node-id 7
```

**Drive it from another agent:** `javagui-spy mcp` runs an MCP stdio server exposing every verb as a
tool — point your MCP client at it. Full guide: [`docs/spy.md`](spy.md).

---

## 4. Highest-value keywords — one line each

Grouped by what you're doing. Signatures & every option live in the libdoc HTML (linked at top).

### Connect / disconnect
```robotframework
Connect To Application    main_class=com.example.MyApp    port=5678
Attach To Application     main_class=testapp.SwingTestApp      # already-running JVM, no -javaagent
${connected}=    Is Connected
Disconnect
```

### Find
```robotframework
${el}=      Find Element      JButton[name='ok']
${count}=   Get Element Count    JButton                 # also asserts:  Get Element Count  JButton  >  3
Element Should Exist       JButton[text='Save']
```

### Click
```robotframework
Click              JButton[text='Save']
Double Click       TreeItem[text='Main.java']
Right Click        JTable[name='data']
```

### Type / clear text
```robotframework
Input Text     JTextField[name='username']    admin        # clears first by default (clear=False to append)
Clear Text     JTextField[name='username']
Type Text      JTextField[name='search']      query        # keystroke-style entry
```

### Read text / value (assertion-enabled — append operator + expected to assert with auto-retry)
```robotframework
${text}=    Get Element Text    JLabel[name='status']
Get Text    JLabel[name='status']    ==    Welcome              # inline assert, retries ~5s
Get Text    JLabel[name='status']    contains    admin    timeout=10
```

### Waits
```robotframework
Wait For Element             JLabel[name='status']       timeout=10
Wait Until Element Visible   JPanel[name='results']      timeout=10
Wait Until Element Contains  JLabel[name='status']    Done    timeout=15
```

### Assertions / verification
```robotframework
Element Should Be Visible    JButton[name='submit']
Element Should Be Enabled    JButton[name='submit']
Element Text Should Contain  JLabel[name='status']    Welcome
Get Element States    JButton[name='submit']    contains    enabled     # visible/enabled/focused/selected/...
```

### Tree
```robotframework
Expand Tree Node      JTree[name='fileTree']    Project Root
Select Tree Node      JTree[name='fileTree']    Project Root/src
Get Tree Node Count   JTree[name='fileTree']    Project Root    >    0
```

### Table
```robotframework
Get Table Row Count    JTable[name='dataTable']    >=    5
${cell}=    Get Table Cell Value    JTable[name='dataTable']    0    1
${data}=    Get Table Data          JTable[name='dataTable']            # all rows at once
Select Table Row       JTable[name='dataTable']    0
```

### List / form controls
```robotframework
${items}=    Get List Items        JList[name='itemList']
Select From List         JList[name='itemList']    Item 3
Select From Combobox     JComboBox[name='theme']    CLASSIC
Check Checkbox           JCheckBox[name='enabled']
Select Tab               JTabbedPane                Data View
```

### Inspect the tree (when you're not using the spy)
```robotframework
${tree}=    Get Component Tree    format=json    max_depth=5
Log Component Tree    format=text
${buttons}=    Get Component Tree    types=J*Button    visible_only=${True}
```

### Screenshot
```robotframework
Capture Screenshot    filename=login.png
```

---

## 5. Gotchas (bite here first)

- **`Element Text Should Be` reads `''` on `JGSearchField`** (and some custom fields) while
  **`Get Element Text` returns the real value** — a known divergence. When a text assertion mysteriously
  sees empty, read with `Get Element Text` and assert on the returned variable:
  ```robotframework
  ${v}=    Get Element Text    JGSearchField[name='search']
  Should Be Equal    ${v}    hello
  ```
- **The agent must be forced `toolkit=swt` at launch-time** (`-javaagent:…=port=NNNN,toolkit=swt`) for
  SWT/RCP — premain runs before SWT loads, so auto-detect fails. At runtime *attach* (`agentmain`),
  `toolkit=auto` works.
- **Launched apps don't persist across tool calls.** Launch the app AND drive it in ONE command
  (`Suite Setup` in the suite, or `subprocess.Popen` inside one `xvfb-run uv run python` call).
- **`pkill -f <pattern>` can self-kill your shell** if the pattern appears in your own command line
  (e.g. `pkill -f smart-client` while your command mentions "smart-client"). Kill by a pattern that
  is NOT in your invocation.
- **`Broken pipe (os error 32)` is flaky under load / large responses**, not a real failure — re-run,
  reduce concurrency, or cap tree depth (`Get Component Tree    max_depth=…`).
- **Synthetic clicks retarget to the nearest listener-bearing ancestor** (LightweightDispatcher-style),
  so clicking a label whose handler lives on a parent card still fires. If a `Click` seems to no-op,
  `javagui-spy suggest --node-id N` will point you at a locator that resolves to a component that reacts.
- **Headless?** Live keywords need a display — run under `xvfb-run -a`. Tests self-skip when no app or
  `DISPLAY` is present.

---

## See also
- [`docs/spy.md`](spy.md) — full `javagui-spy` workflow (verbs, JSON envelope, MCP server, exit codes).
- [`docs/runtime-attach.md`](runtime-attach.md) — attach model, JDK/launcher matrix, Web Start, troubleshooting.
- [`docs/keywords/`](keywords/) — generated libdoc: the complete, authoritative keyword reference.
- [`AGENTS.md`](../AGENTS.md) — canonical contract for working *on* the repo (build, test, verify-loop).
