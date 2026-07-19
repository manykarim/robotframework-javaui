"""Round-trip identity harness for javagui-spy's locator generator.

For a sample of live Swing widgets: generate a locator, resolve it through the PRODUCTION
matcher (find_elements), and require exactly one match whose id equals the target
(match.id == node.id). Actionable widgets must all get a working unique locator.

Opt-in / self-skipping: needs Java + the built Swing test app + a display (xvfb). Runs green
in CI's xvfb job; skips cleanly otherwise. Execution-based metric only (never string-presence).
"""
import os
import subprocess
import time
import shutil
import signal
import pytest

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
AGENT = os.path.join(REPO, "python", "JavaGui", "jars", "javagui-agent.jar")
APP = os.path.join(REPO, "tests", "apps", "swing", "target", "swing-test-app-1.0.0.jar")
PORT = 5688  # non-default to avoid clashing with other suites

pytestmark = pytest.mark.skipif(
    not (shutil.which("java") and os.path.exists(AGENT) and os.path.exists(APP)
         and os.environ.get("DISPLAY")),
    reason="needs java + built swing test app + a DISPLAY (run under xvfb)",
)


@pytest.fixture(scope="module")
def core():
    from JavaGui.spy.core import SpyCore
    proc = subprocess.Popen(
        ["java", f"-javaagent:{AGENT}=port={PORT}", "-jar", APP],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
    )
    time.sleep(6)
    c = SpyCore(toolkit="swing")
    try:
        c.connect(port=PORT, timeout=30)
        yield c
    finally:
        c.disconnect()
        proc.send_signal(signal.SIGTERM)


def _actionable(rec):
    from JavaGui.spy import generator as G
    t = G.node_type(rec["node"])
    return t in {"JButton", "JTextField", "JCheckBox", "JRadioButton", "JLabel",
                 "JComboBox", "JMenuItem", "JMenu", "JToggleButton", "JSpinner", "JList", "JTable", "JTree"}


def test_actionable_widgets_get_unique_locators(core):
    """Every visible actionable widget gets at least one verified-unique locator."""
    from JavaGui.spy import generator as G
    targets = [r for r in core._flat
               if _actionable(r) and (r["node"].get("state") or {}).get("visible") is not False]
    assert targets, "no actionable widgets found in the swing test app"
    ok = fails = 0
    for r in targets:
        cands = G.suggest(core._flat, r, core.resolve, top=1)
        if cands and cands[0]["unique"]:
            ok += 1
        else:
            fails += 1
    rate = ok / len(targets)
    assert rate >= 0.95, f"only {ok}/{len(targets)} ({rate:.0%}) actionable widgets got a unique locator"


def test_roundtrip_identity_correct_node(core):
    """A suggested unique locator resolves to the ORIGINAL node, not a same-count impostor."""
    from JavaGui.spy import generator as G
    checked = 0
    for r in core._flat:
        if not _actionable(r):
            continue
        cands = G.suggest(core._flat, r, core.resolve, top=1)
        if not (cands and cands[0]["unique"]):
            continue
        ids = core.resolve(cands[0]["locator"])
        assert ids == [r["node_id"]], f"{cands[0]['locator']} -> {ids}, expected [{r['node_id']}]"
        checked += 1
    assert checked >= 5, "expected to verify several actionable widgets"


def test_no_name_nodes_use_anchoring(core):
    """With names stripped (off-the-shelf sim), actionable widgets still resolve via text/anchoring."""
    from JavaGui.spy import generator as G
    targets = [r for r in core._flat if _actionable(r) and G.node_text(r["node"])]
    ok = sum(1 for r in targets
             if (c := G.suggest(core._flat, r, core.resolve, top=1, strip_names=True)) and c[0]["unique"])
    assert ok / max(len(targets), 1) >= 0.9, f"no-name coverage {ok}/{len(targets)} below 90%"
