# AGENTS.md — python/JavaGui (RF keyword layer)

The Robot Framework library: the three RF classes, the runtime-attach helper, the `javagui-spy`
tool, and the bundled agent jar. **Read the root `/AGENTS.md` first — it is canonical** for build,
test baselines, gotchas, and style. This file covers ONLY what is specific to this package.

## What lives here
- `__init__.py` (~4.4k lines) — the god-file. Holds `SwingLibrary`, `SwtLibrary`, `RcpLibrary`
  (the RF library classes) and `SwingElement`. These are **thin Python wrappers**: each keyword
  validates args (`validate_locator`, `_parse_timeout`) then delegates to a Rust object
  `self._lib = _SwingLibrary(...)` imported from `JavaGui._core` (the PyO3 extension `_core.abi3.so`
  built from `src/`). All real locator/RPC work is in Rust — keep Python bodies tiny.
  The wrappers exist because RF introspects `__init__` signatures, which PyO3 classes don't expose.
- `keywords/` — mixin classes composed into the library classes (`GetterKeywords`, `TableKeywords`,
  `TreeKeywords`, `ListKeywords`, `Rcp*`, `Swt*`). Assertion-engine keywords land here.
  `class SwingLibrary(GetterKeywords, TableKeywords, TreeKeywords, ListKeywords)` — group new
  keywords into the right mixin, don't just pile them into `__init__.py`.
- `assertions/` — AssertionEngine glue (`AssertionOperator`, `ElementState`, formatters, security).
- `validation.py` / `deprecation.py` — locator validation + `@deprecated` keyword aliases.
- `_attach.py` — runtime dynamic attach (below).
- `spy/` — the `javagui-spy` tool (below).
- `jars/javagui-agent.jar` — the bundled Java agent, loaded via `-javaagent` or attach.
  **Rebuilt from `agent/`; you must copy it here after any Java change** (`invoke build-dev` does it).

## Where new keywords go
Add the RF-facing method to the matching class (`SwingLibrary`/`SwtLibrary`/`RcpLibrary`) or a
`keywords/` mixin; keep the body a validate-then-`self._lib.<rust_method>(...)` delegation. If the
behavior is new logic (not a new RF surface for existing logic), it likely belongs in `src/` (Rust).
The ≤500-line rule is aspirational for `__init__.py` — keep additions cohesive, prefer the mixins.

## `_attach.py` — runtime attach (no `-javaagent`)
Backs `Attach To Application` / `List Applications` / Web Start. `discover_jvms` reads
`/proc/<pid>/cmdline` (falls back to `jcmd`/`jps`), `select_jvm` picks exactly one (raises on
zero/ambiguous — never guesses), `inject_agent` loads the jar via JDK Attach (`AttachMain`) or
`jattach`. Needs a JDK with `jdk.attach` on THIS host (set `JAVAGUI_JAVA`), or `JAVAGUI_JATTACH`.
At attach (`agentmain`) `toolkit=auto` works; at `-javaagent` (`premain`) force `toolkit=swt` for SWT/RCP.

## `spy/` — the `javagui-spy` tool  (console entry `JavaGui.spy.cli:main`, docs/spy.md)
- `core.py` `SpyCore` — the single shared engine: owns ONE library connection, a cached tree, and
  the `resolve` oracle (production `find_elements`). Every surface is a thin client of it.
- `cli.py` — argparse verbs: `schema dump-tree find validate suggest describe screenshot pick
  highlight ui mcp`. `validate` exit codes: 0 unique / 2 error / 3 zero / 4 ambiguous.
- `generator.py` — locator-candidate generation (the `suggest` contract). `mcp.py` — MCP stdio
  server exposing the verbs as tools. `server.py` + `static/` — the local web inspector (`ui`).
Add a verb in `cli.py` + a `SpyCore` method + (if agent-facing) update `schema`; keep logic in core.

## Test & docs for this layer
- pytest for this package lives in `tests/python/` and `tests/unit/` (not here).
- Regenerate the keyword reference after any keyword/docstring change: `uv run invoke docs`
  (libdoc → `docs/keywords/{Swing,Swt,Rcp}.html`).
- Known divergence: on `JGSearchField`, `Element Text Should Be` reads `''` — use `Get Element Text`.
