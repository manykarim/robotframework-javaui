#!/usr/bin/env python3
"""Report end-to-end keyword coverage for robotframework-javaui.

Computes, per library (Swing/Swt/Rcp), how many public non-deprecated keywords
are exercised by at least one Robot Framework suite under ``tests/robot/``.

This makes the e2e-keyword-coverage contract measurable and enforceable:

    python scripts/keyword_coverage.py            # human report
    python scripts/keyword_coverage.py --json      # machine readable
    python scripts/keyword_coverage.py --min 90    # exit 1 if below threshold

Deprecated aliases (flagged via JavaGui.deprecation) and the ``SwingElement``
property accessors are excluded from the denominator, matching the release
coverage gate.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ROBOT_DIR = ROOT / "tests" / "robot"
LIBRARIES = ("Swing", "Swt", "Rcp")


def robot_keyword_form(method_name: str) -> str:
    """Convert a python method name to its spaced Robot Framework keyword form."""
    return " ".join(w.capitalize() for w in method_name.split("_"))


def library_keywords(lib_name: str) -> set[str]:
    sys.path.insert(0, str(ROOT / "python"))
    import JavaGui  # noqa: E402  (import after sys.path tweak)

    lib_cls = getattr(JavaGui, lib_name)
    names: set[str] = set()
    for attr in dir(lib_cls):
        if attr.startswith("_"):
            continue
        member = getattr(lib_cls, attr)
        if not callable(member):
            continue
        # Skip deprecated aliases (marked by the deprecation decorator).
        doc = (getattr(member, "__doc__", "") or "").lower()
        if getattr(member, "_javagui_deprecated", False) or "deprecated" in doc.split(".")[0]:
            continue
        names.add(attr)
    return names


def used_keywords() -> str:
    """Return the concatenated lowercased text of every robot suite/resource."""
    chunks = []
    for path in ROBOT_DIR.rglob("*.robot"):
        chunks.append(path.read_text(encoding="utf-8", errors="ignore"))
    for path in ROBOT_DIR.rglob("*.resource"):
        chunks.append(path.read_text(encoding="utf-8", errors="ignore"))
    return "\n".join(chunks).lower()


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--json", action="store_true", help="emit JSON")
    ap.add_argument("--min", type=float, default=None, help="fail if coverage %% below this")
    args = ap.parse_args()

    corpus = used_keywords()
    report: dict = {"libraries": {}, "total": {}}
    all_kw = 0
    all_cov = 0
    for lib in LIBRARIES:
        kws = library_keywords(lib)
        covered, uncovered = set(), set()
        for kw in kws:
            spaced = robot_keyword_form(kw).lower()
            # whole keyword-invocation match (word-boundary on both ends)
            if re.search(r"(^|\W)" + re.escape(spaced) + r"(\W|$)", corpus):
                covered.add(kw)
            else:
                uncovered.add(kw)
        report["libraries"][lib] = {
            "total": len(kws),
            "covered": len(covered),
            "uncovered": sorted(uncovered),
            "pct": round(100.0 * len(covered) / len(kws), 1) if kws else 100.0,
        }
        all_kw += len(kws)
        all_cov += len(covered)

    pct = round(100.0 * all_cov / all_kw, 1) if all_kw else 100.0
    report["total"] = {"total": all_kw, "covered": all_cov, "pct": pct}

    if args.json:
        print(json.dumps(report, indent=2))
    else:
        for lib, d in report["libraries"].items():
            print(f"{lib:6} {d['covered']:3}/{d['total']:<3} ({d['pct']:5.1f}%)")
            if d["uncovered"]:
                print("       uncovered: " + ", ".join(d["uncovered"]))
        print(f"TOTAL  {all_cov}/{all_kw} ({pct}%)")

    if args.min is not None and pct < args.min:
        print(f"FAIL: coverage {pct}% < required {args.min}%", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
