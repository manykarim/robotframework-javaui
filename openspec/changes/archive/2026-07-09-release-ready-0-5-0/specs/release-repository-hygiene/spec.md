## ADDED Requirements

### Requirement: Public repository contains no throwaway process documentation
The shipped repository SHALL NOT contain internal process/status/mission/phase/delivery reports. Only user-facing docs, contributor docs (ADRs, guides), and release notes SHALL remain under `docs/`.

#### Scenario: Doc set reduced to essentials
- **WHEN** `docs/` is reviewed before release
- **THEN** throwaway files (matching `PHASE_*`, `MISSION_*`, `*_DELIVERY`, `*_REPORT`, `*_SUMMARY`, `*COMPLETE*`, `FIXES_*`, `*_VERIFICATION`, `*_OLD`, internal `research/` and `test-plans/` artifacts) are removed, leaving roughly 25–40 curated docs

### Requirement: No runtime or tooling artifacts are tracked
The repository SHALL NOT track editor/agent/tooling runtime state or large third-party binaries.

#### Scenario: Tooling runtime untracked
- **WHEN** `git ls-files` is inspected
- **THEN** the previously tracked `.claude-flow/` runtime files (daemon.pid, daemon-state.json, metrics/*) are removed from tracking, and `.gitignore` prevents re-adding them

#### Scenario: Third-party jars ignored
- **WHEN** the working tree is checked
- **THEN** `example-apps/` (and any bundled third-party showcase jar) is covered by `.gitignore` and not committed

### Requirement: Keyword documentation is single-source and current
The project SHALL maintain one canonical, up-to-date keyword reference generated from the current library.

#### Scenario: Libdoc regenerated and consistent
- **WHEN** the keyword reference is produced for release
- **THEN** it is regenerated from the current `python/JavaGui/` source (reflecting the v0.4.1 API cleanup: renamed/deprecated keywords), and there is no stale or drifting parallel reference contradicting it
