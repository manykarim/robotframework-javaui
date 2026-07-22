# agent-documentation Specification

## Purpose

Provide documentation purpose-built for AI coding agents in two roles — contributor-agents that
develop the library and automation-agents that use it — with a single source of truth, tool-agnostic
wiring, and CI anti-drift.

## ADDED Requirements

### Requirement: Canonical, tool-agnostic contributor-agent instructions

The repository SHALL provide a canonical `AGENTS.md` at the root as the single source of truth for
contributor-agent instructions, and every tool-specific instruction file SHALL be a thin pointer to
it (not an independent copy). Nested `AGENTS.md` files SHALL carry component-local specifics.

#### Scenario: A tool-specific file points to the canonical source
- **WHEN** `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursor/rules/agents.mdc`, or `GEMINI.md` is read
- **THEN** it references `AGENTS.md` as the source of truth (e.g. `See @AGENTS.md`) rather than duplicating build/test/gotcha facts, so the files cannot contradict each other

#### Scenario: The misleading swarm boilerplate is gone
- **WHEN** an agent reads `CLAUDE.md`
- **THEN** it no longer contains Claude-Flow/ruflo swarm-orchestration instructions (topologies, agent counts, hive-mind) that do not apply to building this library

#### Scenario: Nearest AGENTS.md wins for a subproject
- **WHEN** an agent works inside `src/`, `agent/`, `python/JavaGui/`, or `tests/apps/`
- **THEN** a nested `AGENTS.md` in that subtree provides the component-local build/test/gotcha context (e.g. the Java agent's `toolkit=swt`, Maven-at-`/tmp`, `xvfb`), and the root `AGENTS.md` stays concise

### Requirement: Executable, verifiable contributor guidance

`AGENTS.md` SHALL give runnable, deterministic commands rather than vague prose, SHALL state the
ordered test/verify pipeline with its expected results (verify-loop), SHALL record the repo's known
failure-modes/gotchas, and SHALL define what "done" means. It SHALL stay concise (deep detail behind
links).

#### Scenario: Commands are runnable and paired with expected results
- **WHEN** an agent follows the build or test section
- **THEN** each step is a copy-pasteable command, and the test steps state the expected outcome (e.g. "`uv run robot --dryrun …` → expect N passed / 0 failed / M skipped") so the agent can verify its own work

#### Scenario: Gotchas are documented, not inferred
- **WHEN** an agent could hit a known trap (self-killing `pkill`, apps not persisting across shell calls, `Broken pipe` flakiness, forcing `toolkit=swt`, needing `xvfb`)
- **THEN** `AGENTS.md` (or the relevant nested file) states the trap and the safe pattern explicitly

#### Scenario: Definition of done is explicit
- **WHEN** an agent finishes a change
- **THEN** `AGENTS.md` provides a done-checklist (build succeeds, the pipeline is at its baselines, lint deltas explained, no secrets committed)

### Requirement: Automation-agent usage channel backed by self-describing surfaces

The repository SHALL provide an automation-agent usage channel — an `llms.txt` index and a curated,
example-first cheatsheet — whose reference content points to the code's self-describing surfaces
(`javagui-spy schema`, the MCP server, generated libdoc) rather than re-specifying keyword/locator
details in prose.

#### Scenario: An llms.txt index exists in the documented format
- **WHEN** an agent fetches `docs/llms.txt`
- **THEN** it is a valid llmstxt.org document (H1 title, one-line summary, sectioned links) covering getting-started, attach-vs-connect, locators, keywords, the spy workflow, and gotchas, with an `## Optional` section for advanced material

#### Scenario: Reference points to the source of truth, not a copy
- **WHEN** the usage docs describe the keyword set or locator grammar
- **THEN** they direct the agent to the generated libdoc and `javagui-spy schema` / MCP as authoritative, so the prose cannot drift from the code

#### Scenario: The cheatsheet teaches the verify-loop
- **WHEN** an agent reads the usage cheatsheet
- **THEN** it shows the `dump-tree → find → validate` (exit codes 0/2/3/4) loop and the `suggest` fallback, plus the connect-vs-attach decision and the `JGSearchField` read-back divergence, each as a runnable example

### Requirement: Curated, navigable docs tree

The `docs/` tree SHALL be curated so agents can distinguish canonical from obsolete: each duplicate
cluster reduced to one canonical page, throwaway/status documents moved to `docs/archive/`, and an
index present.

#### Scenario: Duplicate clusters are consolidated
- **WHEN** an agent looks for component-tree, SWT, output-format, or quick-reference guidance
- **THEN** there is exactly one canonical page per topic in `docs/` (the redundant siblings moved to `docs/archive/`), reachable from a `docs/` index

#### Scenario: History is preserved
- **WHEN** documents are archived or consolidated
- **THEN** they are moved with `git mv` (history preserved), not deleted outright

### Requirement: CI keeps agent docs from drifting

CI SHALL treat the instruction/usage files as engineering artifacts: pointer files must resolve to
the canonical source, links must not be dead, and runnable command/example blocks must be exercised
by the existing checks.

#### Scenario: Broken pointers or dead links fail CI
- **WHEN** a pointer file stops referencing `AGENTS.md`, or a relative link in `AGENTS.md`/`llms.txt`/the cheatsheet points to a missing file
- **THEN** the documentation lint step fails

#### Scenario: A renamed command/keyword surfaces in CI
- **WHEN** a command or keyword used in a documented runnable block is renamed or removed
- **THEN** the doc-tested block (wired into the existing `robot --dryrun` / pytest run) fails, rather than the docs silently going stale
