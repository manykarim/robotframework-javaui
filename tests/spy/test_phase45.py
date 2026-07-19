"""Phase 4-5 coverage: CLI schema, MCP tools, recognition rules, data-widget types (no app),
plus a live hit-test round-trip (self-skips without java + swing app + DISPLAY)."""
import os
import shutil
import subprocess
import signal
import time
import pytest

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
AGENT = os.path.join(REPO, "python", "JavaGui", "jars", "javagui-agent.jar")
APP = os.path.join(REPO, "tests", "apps", "swing", "target", "swing-test-app-1.0.0.jar")
PORT = 5689

# ---- no-app unit coverage ----

def test_mcp_tools_and_schema():
    from JavaGui.spy import mcp
    from JavaGui.spy.cli import SCHEMA
    names = {t["name"] for t in mcp._TOOLS}
    assert {"spy_dump_tree", "spy_find", "spy_validate", "spy_suggest", "spy_pick"} <= names
    assert "pick" in SCHEMA["verbs"] and "mcp" in SCHEMA["verbs"] and "ui" in SCHEMA["verbs"]


def test_recognition_rules_and_data_types():
    from JavaGui.spy import generator as G
    assert "FormsLabel" in G.RECOGNITION_RULES
    assert {"JTable", "JTree", "JList"} <= G.DATA_WIDGET_TYPES


def test_recognition_rule_reorders_qualifiers():
    """A rule for a class makes its ordered attribute win over name."""
    from JavaGui.spy import generator as G
    node = {"component_type": {"simple_name": "FormsLabel"},
            "identity": {"name": "x1", "text": "Input"}}
    quals = G._qualifiers(node, strip_names=False)
    assert quals and quals[0] == ("text", "Input")  # rule says text-first for FormsLabel


# ---- live coverage ----

pytestmark_live = pytest.mark.skipif(
    not (shutil.which("java") and os.path.exists(AGENT) and os.path.exists(APP) and os.environ.get("DISPLAY")),
    reason="needs java + swing app + DISPLAY (xvfb)",
)


@pytest.fixture(scope="module")
def core():
    from JavaGui.spy.core import SpyCore
    proc = subprocess.Popen(["java", f"-javaagent:{AGENT}=port={PORT}", "-jar", APP],
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    time.sleep(6)
    c = SpyCore("swing")
    try:
        c.connect(port=PORT, timeout=30)
        yield c
    finally:
        c.disconnect()
        proc.send_signal(signal.SIGTERM)


@pytestmark_live
def test_hittest_and_generation_live(core):
    from JavaGui.spy import generator as G
    # generation is stable across two reads with no UI change
    assert core.ui_generation() == core.ui_generation()
    # hit-test the center of a known button (screen pos = sum of parent-relative bounds)
    btn = next(r for r in core._flat if G.node_type(r["node"]) == "JButton" and G.node_name(r["node"]))
    chain = btn["ancestors"] + [btn]
    sx = sum(G.node_geometry(r["node"])[0] for r in chain if G.node_geometry(r["node"]))
    sy = sum(G.node_geometry(r["node"])[1] for r in chain if G.node_geometry(r["node"]))
    bg = G.node_geometry(btn["node"])
    hit = core.hit_test(sx + bg[2] // 2, sy + bg[3] // 2)
    assert hit.get("hit") is True
    assert btn["node_id"] == hit.get("id") or btn["node_id"] in (hit.get("ancestor_path") or [])
    assert core.highlight(btn["node_id"], 200).get("ok") is True


@pytestmark_live
def test_arm_pick_times_out_cleanly(core):
    r = core.arm_pick(1500)
    assert r.get("hit") is False and r.get("timeout") is True
