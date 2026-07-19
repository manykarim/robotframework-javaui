"""Locator generator for javagui-spy.

Given a fetched widget tree and a target node, synthesize ranked, human-readable Robot
Framework locators. Every candidate is verified through a ``resolve`` oracle — the production
matcher (``find_elements``) — so a candidate is only ever emitted if it resolves to exactly one
node whose id equals the target (parity by construction; see the ``javagui-spy-tool`` design).

Tiers (best first):
  1. global single segment      Type[name='..'] / Type[text='..'] / Type[tooltip='..']
  2. nearest-stable-ancestor >>  Anchor[..] >> Type[..]  (workhorse for deep, no-name nodes)
  3. geometry fallback           Type[width][height][x][y]   (always flagged brittle)

Validated in Phase 0: single-segment name/text is 100% unique+correct; anchored ``>>`` chains
resolve deep no-name nodes; every uncovered node is an anonymous structural container.
"""
from __future__ import annotations
from typing import Callable, Iterable

from . import rules as _rules

# Per-app recognition rules (widget class -> preferred attribute order) + data-widget types.
# Loaded from defaults + an optional javagui-spy.rules.json; re-load via set_rules().
RECOGNITION_RULES, DATA_WIDGET_TYPES = _rules.load()


def set_rules(path: str | None = None) -> None:
    global RECOGNITION_RULES, DATA_WIDGET_TYPES
    RECOGNITION_RULES, DATA_WIDGET_TYPES = _rules.load(path)

# ---------------------------------------------------------------------------
# Nested tree accessors (get_ui_tree format=json:
#   {roots:[node], ...}; node.id.hash_code, node.component_type.simple_name,
#   node.identity.{name,text,title,tooltip}, node.geometry.{x,y,width,height}, node.children)
# ---------------------------------------------------------------------------
def node_type(n: dict) -> str:
    return (n.get("component_type") or {}).get("simple_name") or "Component"

def node_name(n: dict):
    return (n.get("identity") or {}).get("name")

def node_text(n: dict):
    idy = n.get("identity") or {}
    t = idy.get("text") or idy.get("title")
    if not t:
        return None
    return " ".join(t.replace("&", "").split()) or None

def node_tooltip(n: dict):
    return (n.get("identity") or {}).get("tooltip")

def node_geometry(n: dict):
    # tree geometry is nested: geometry.bounds.{x,y,width,height} (parent-relative)
    g = n.get("geometry") or {}
    b = g.get("bounds") or g  # tolerate flat or nested
    try:
        return (int(b["x"]), int(b["y"]), int(b["width"]), int(b["height"]))
    except Exception:
        return None

def node_id(n: dict):
    return (n.get("id") or {}).get("hash_code")

def node_children(n: dict) -> list:
    return n.get("children") or []


# ---------------------------------------------------------------------------
# Flatten the tree into records carrying ancestor chains + type index
# ---------------------------------------------------------------------------
def flatten(root: dict) -> list[dict]:
    """Return a list of records: {node, ancestors:[rec...], depth, type_index, node_id}."""
    flat: list[dict] = []

    def walk(n, parent, ancestors, depth):
        siblings = node_children(parent) if parent else [n]
        same_type = [c for c in siblings if node_type(c) == node_type(n)]
        rec = {
            "node": n,
            "ancestors": ancestors,
            "depth": depth,
            "type_index": (same_type.index(n) if n in same_type else 0),
            "node_id": node_id(n),
        }
        flat.append(rec)
        for c in node_children(n):
            walk(c, n, ancestors + [rec], depth + 1)

    walk(root, None, [], 0)
    return flat


def flatten_forest(tree_json: dict) -> list[dict]:
    """Flatten a get_ui_tree JSON payload ({roots:[...]}) into records."""
    flat: list[dict] = []
    for root in (tree_json.get("roots") or []):
        flat.extend(flatten(root))
    return flat


# ---------------------------------------------------------------------------
# Qualifiers + stability weights (the attribute-priority ladder)
# ---------------------------------------------------------------------------
_STABILITY = {"name": 1.00, "accessiblename": 0.90, "text": 0.75, "tooltip": 0.65,
              "nth-of-type": 0.40, "index": 0.25, "geometry": 0.15}


def _accessible_name(n: dict):
    return (n.get("accessibility") or {}).get("accessible_name")


def _qualifiers(n: dict, strip_names: bool) -> list[tuple[str, str]]:
    getters = {
        "name": lambda: (None if strip_names else node_name(n)),
        "accessiblename": _lambda(_accessible_name, n),
        "text": _lambda(node_text, n),
        "tooltip": _lambda(node_tooltip, n),
    }
    # A per-app recognition rule for this widget class overrides the default order.
    order = RECOGNITION_RULES.get(node_type(n)) or ["name", "accessiblename", "text", "tooltip"]
    qs: list[tuple[str, str]] = []
    for attr in order:
        get = getters.get(attr)
        if get is None:
            continue
        v = get()
        if v:
            qs.append((attr, v))
    return qs


def _lambda(fn, n):
    return lambda: fn(n)


def _esc(v: str) -> str:
    return str(v).replace("'", r"\'")


def _score(locator: str, stability: float, has_semantic: bool, depth_gap: int) -> float:
    readability = 1.0 if has_semantic else 0.4
    brevity = 1.0 / (1.0 + len(locator) / 40.0)
    anchor_locality = 1.0 / (1.0 + max(depth_gap, 0))
    return round(0.45 * stability + 0.25 * readability + 0.20 * brevity + 0.10 * anchor_locality, 4)


def _candidate(locator, strategy, stability, brittle, has_semantic, depth_gap, preconditions=None):
    return {
        "locator": locator,
        "strategy": strategy,
        "match_count": None,          # filled in by verification
        "unique": None,
        "stability": round(stability, 3),
        "score": _score(locator, stability, has_semantic, depth_gap),
        "brittle_flags": list(brittle or []),
        "preconditions": list(preconditions or []),
    }


# ---------------------------------------------------------------------------
# Candidate enumeration (unverified) — ordered best-first
# ---------------------------------------------------------------------------
def _enumerate(target: dict, flat: list[dict], strip_names: bool) -> list[dict]:
    n = target["node"]
    T = node_type(n)
    out: list[dict] = []

    # Tier 1 — global single segment
    for attr, val in _qualifiers(n, strip_names):
        out.append(_candidate(f"{T}[{attr}='{_esc(val)}']", "single",
                               _STABILITY[attr], [], True, 0))

    # Tier 2 — nearest-stable-ancestor anchored >> chain
    target_quals = [f"[{a}='{_esc(v)}']" for a, v in _qualifiers(n, strip_names)]
    for depth_gap, anc in enumerate(reversed(target["ancestors"]), start=1):
        an = anc["node"]
        AT = node_type(an)
        anc_quals = _qualifiers(an, strip_names)
        if not anc_quals:
            continue
        for aa, av in anc_quals:
            anchor = f"{AT}[{aa}='{_esc(av)}']"
            anc_stab = _STABILITY[aa]
            # anchored + a semantic target qualifier (preferred)
            for tq, (ta, _tv) in zip(target_quals, _qualifiers(n, strip_names)):
                stab = min(anc_stab, _STABILITY[ta])
                out.append(_candidate(f"{anchor} >> {T}{tq}", "anchored", stab, [], True, depth_gap))
            # anchor + bare type (unique when target is the only such type under the anchor)
            out.append(_candidate(f"{anchor} >> {T}", "anchored-bare",
                                   min(anc_stab, 0.55), [], bool(target_quals), depth_gap))
            # anchor + nth-of-type (structural; brittle)
            out.append(_candidate(f"{anchor} >> {T}:nth-of-type({target['type_index'] + 1})",
                                   "anchored-nth", min(anc_stab, _STABILITY["nth-of-type"]),
                                   ["sibling-index"], False, depth_gap))
        break  # nearest stable anchor only

    # Tier 3 — geometry fallback (always flagged brittle)
    g = node_geometry(n)
    if g:
        out.append(_candidate(
            f"{T}[width='{g[2]}'][height='{g[3]}'][x='{g[0]}'][y='{g[1]}']",
            "geometry", _STABILITY["geometry"],
            ["resize/relayout will break this"], False, 0))
    return out


# ---------------------------------------------------------------------------
# Public API
# ---------------------------------------------------------------------------
def suggest(flat: list[dict], target: dict, resolve: Callable[[str], Iterable[int]],
            *, strip_names: bool = False, top: int = 3) -> list[dict]:
    """Return up to ``top`` verified-unique candidates for ``target``, best score first.

    ``resolve(locator)`` -> iterable of matched node ids (the live oracle). A candidate is
    accepted only if it resolves to exactly one id equal to the target id. If none are unique,
    the best-scoring *non-unique* candidate is returned (with ``unique=False``) so callers can
    still show the closest attempt.
    """
    tid = target["node_id"]
    unique_ok: list[dict] = []
    best_effort: dict | None = None
    for cand in _enumerate(target, flat, strip_names):
        try:
            ids = list(resolve(cand["locator"]))
        except Exception:
            continue
        cand["match_count"] = len(ids)
        cand["unique"] = (len(ids) == 1 and ids[0] == tid)
        if cand["unique"]:
            unique_ok.append(cand)
            if len(unique_ok) >= top * 2:
                break
        elif best_effort is None or cand["score"] > best_effort["score"]:
            if not cand["brittle_flags"] or "ambiguous" not in cand["brittle_flags"]:
                be = dict(cand); be["brittle_flags"] = cand["brittle_flags"] + ["not-unique"]
                best_effort = be
    unique_ok.sort(key=lambda c: (-c["score"], len(c["locator"])))
    if unique_ok:
        return unique_ok[:top]
    return [best_effort] if best_effort else []
