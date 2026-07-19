## ADDED Requirements

### Requirement: Stateless agentic CLI with structured, self-describing I/O
The spy SHALL expose a CLI optimized for LLM agents: stateless, idempotent, read-only verbs, always
`--json`, with a uniform envelope and **exit codes as control flow** for `validate`
(0 = unique, 2 = parse error, 3 = zero matches, 4 = ambiguous). Verbs SHALL include at least
`connect`/`launch`, `dump-tree`, `find`, `validate`, `suggest`, `describe`, `screenshot`, `schema`.

#### Scenario: An agent authors a working locator in five calls
- **WHEN** an agent runs `dump-tree --visible-only` → `find` → `suggest --node-id N` → `validate` → `screenshot --annotate`
- **THEN** each step returns deterministic JSON it can chain, and `validate` exit code 0 confirms a unique locator without human interpretation

#### Scenario: Errors are machine-correctable
- **WHEN** a malformed locator is validated
- **THEN** the tool returns `{code:"PARSE_ERROR", position, snippet, hint}` naming the offending token and the valid vocabulary, so an agent can self-correct

#### Scenario: Compact snapshots and durable artifacts
- **WHEN** `dump-tree` emits nodes
- **THEN** each node is one compact JSONL line (allowlisted fields ≈20 tokens) carrying both an
  ephemeral `node_id` for chaining and, on `suggest`, a durable resolved locator as the artifact;
  a `schema` verb returns the JSON schema of every verb plus a locator-grammar cheatsheet

### Requirement: One core, multiple surfaces
The CLI, the graphical UI, and a future MCP façade SHALL be thin clients of one SpyCore contract so
they cannot diverge; the GUI's API is the CLI's verb schemas.

#### Scenario: Same suggestion from CLI and UI
- **WHEN** the same node is inspected via the CLI `suggest` and via the UI candidate panel
- **THEN** both return the identical ranked candidate contract (locator/score/stability/flags)
