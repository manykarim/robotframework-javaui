#!/usr/bin/env python3
"""Doc-lint — keep agent-facing docs from drifting (see openspec agent-facing-documentation).

Checks, all machine-verifiable:
  1. Tool-specific instruction files are thin POINTERS to the canonical AGENTS.md
     (they must reference AGENTS.md and must NOT re-embed build/test facts).
  2. No DEAD relative links in the canonical agent docs (AGENTS.md, nested AGENTS.md,
     docs/llms.txt, docs/agent-usage-cheatsheet.md, docs/README.md).
  3. The generated libdoc keyword reference exists (the usage docs point to it as source of truth).

Exit 0 = clean, 1 = violations. Run: `python scripts/doc_lint.py` (or `uv run python scripts/doc_lint.py`).
"""
from __future__ import annotations

import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

# Committed tool pointers (must exist + point to AGENTS.md).
POINTERS = [
    "GEMINI.md",
    ".github/copilot-instructions.md",
    ".cursor/rules/agents.mdc",
]
# CLAUDE.md is git-ignored by repo policy (a per-dev local file), so it is optional —
# but if a checkout has one, it must be a pointer, not a stale copy.
OPTIONAL_POINTERS = ["CLAUDE.md"]
CANONICAL_DOCS = [
    "AGENTS.md",
    "src/AGENTS.md",
    "agent/AGENTS.md",
    "python/JavaGui/AGENTS.md",
    "tests/apps/AGENTS.md",
    "docs/llms.txt",
    "docs/agent-usage-cheatsheet.md",
    "docs/README.md",
]
KEYWORD_REFS = ["docs/keywords/Swing.html", "docs/keywords/Swt.html", "docs/keywords/Rcp.html"]

# A pointer that embeds these looks like a duplicated source of truth, not a pointer.
DRIFT_MARKERS = ("uv sync", "maturin develop", "cargo test", "invoke build", "mvn -f")

_LINK = re.compile(r"\]\(([^)]+)\)")


def _links(path: str):
    text = open(os.path.join(ROOT, path), encoding="utf-8").read()
    for m in _LINK.finditer(text):
        target = m.group(1).split("#", 1)[0].strip()
        if not target or target.startswith(("http://", "https://", "mailto:", "@")):
            continue
        yield target


def main() -> int:
    errors: list[str] = []

    # 1. pointers reference AGENTS.md and don't re-embed facts
    for p in POINTERS + OPTIONAL_POINTERS:
        fp = os.path.join(ROOT, p)
        if not os.path.exists(fp):
            if p in OPTIONAL_POINTERS:
                continue  # git-ignored / local-only: fine to be absent
            errors.append(f"missing pointer file: {p}")
            continue
        text = open(fp, encoding="utf-8").read()
        if "AGENTS.md" not in text:
            errors.append(f"{p}: does not reference AGENTS.md (pointer must point to the canonical file)")
        embedded = [m for m in DRIFT_MARKERS if m in text]
        if embedded:
            errors.append(f"{p}: re-embeds build/test facts {embedded} — keep it a thin pointer")

    # 2. no dead relative links in the canonical docs
    for d in CANONICAL_DOCS:
        if not os.path.exists(os.path.join(ROOT, d)):
            errors.append(f"missing canonical doc: {d}")
            continue
        base = os.path.dirname(d)
        for link in _links(d):
            cands = {
                os.path.normpath(os.path.join(ROOT, base, link)),
                os.path.normpath(os.path.join(ROOT, link)),
            }
            if not any(os.path.exists(c) for c in cands):
                errors.append(f"{d}: dead link -> {link}")

    # 3. keyword reference (source of truth for usage docs) exists
    for k in KEYWORD_REFS:
        if not os.path.exists(os.path.join(ROOT, k)):
            errors.append(f"missing generated keyword reference: {k} (run `uv run invoke docs`)")

    if errors:
        print("doc-lint: FAIL")
        for e in errors:
            print(f"  - {e}")
        return 1
    print(f"doc-lint: OK ({len(POINTERS)} pointers, {len(CANONICAL_DOCS)} docs, {len(KEYWORD_REFS)} refs)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
