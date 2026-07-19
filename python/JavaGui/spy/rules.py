"""Per-app recognition rules for the locator generator.

Off-the-shelf apps often set garbage default metadata on custom widget classes — but a
consistent, meaningful property elsewhere. A rules file tells the generator which attributes to
trust for which widget class, so generation stays semantic instead of degrading to nth/geometry.

Rules map a widget's SWT/Swing simple class name to an ordered list of attributes to prefer:

    { "recognition_rules": { "FormsLabel": ["text"], "GridRow": ["tooltip", "text"] },
      "data_widget_types": ["JTable", "JList", "JTree", "Table", "Tree"] }

Loaded from (first found): the path passed to ``load``, ``$JAVAGUI_SPY_RULES``, or
``./javagui-spy.rules.json``. Merged over the built-in defaults.
"""
from __future__ import annotations
import json
import os
from pathlib import Path

# Built-in defaults — the widget classes we already know are worth special-casing.
DEFAULT_RECOGNITION: dict[str, list[str]] = {
    # JGoodies showcase custom label whose only meaningful attribute is its text.
    "FormsLabel": ["text"],
}

# Types whose "cells" are stamped renderers, not addressable components — suggest data keywords.
DEFAULT_DATA_WIDGET_TYPES: set[str] = {
    "JTable", "JList", "JTree", "Table", "Tree", "List",
}


def load(path: str | None = None) -> tuple[dict[str, list[str]], set[str]]:
    """Return (recognition_rules, data_widget_types) merged over the defaults."""
    rec = dict(DEFAULT_RECOGNITION)
    data_types = set(DEFAULT_DATA_WIDGET_TYPES)
    candidates = [path, os.environ.get("JAVAGUI_SPY_RULES"), "javagui-spy.rules.json"]
    for cand in candidates:
        if not cand:
            continue
        p = Path(cand)
        if not p.is_file():
            continue
        try:
            cfg = json.loads(p.read_text())
        except Exception:
            continue
        rec.update(cfg.get("recognition_rules") or {})
        data_types |= set(cfg.get("data_widget_types") or [])
        break
    return rec, data_types
