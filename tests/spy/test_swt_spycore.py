"""SpyCore over SWT — exercises the per-toolkit tree path (getComponentTree + normalizer) and the
runtime-attach entry point. Self-skips without java + agent + the SWT test app + DISPLAY.

Before the runtime-attach change, SpyCore('swt').refresh() called a nonexistent SwtLibrary
get_ui_tree and blew up; the spy's dump-tree/find/suggest were broken for SWT/RCP. This locks in
the fix (raw getComponentTree RPC normalized to the generator's node shape) end to end.
"""
import os
import shutil
import subprocess
import signal
import time
import pytest

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
AGENT = os.path.join(REPO, "python", "JavaGui", "jars", "javagui-agent.jar")
SWT_APP = os.path.join(REPO, "tests", "apps", "swt", "target", "swt-test-app-1.0.0-all.jar")

pytestmark = pytest.mark.skipif(
    not (shutil.which("java") and os.path.exists(AGENT) and os.path.exists(SWT_APP)
         and os.environ.get("DISPLAY")),
    reason="needs java + agent + SWT test app + DISPLAY (xvfb)",
)


@pytest.fixture(scope="module")
def swt_core():
    # Launch the SWT app WITHOUT -javaagent, then attach at runtime (exercises SpyCore.attach).
    proc = subprocess.Popen(["java", "-jar", SWT_APP],
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    time.sleep(7)
    from JavaGui.spy.core import SpyCore
    c = SpyCore("swt")
    try:
        c.attach(main_class="swt-test-app", timeout=30)
        yield c
    finally:
        try:
            c.disconnect()
        except Exception:
            pass
        proc.send_signal(signal.SIGTERM)


def test_swt_spycore_tree_and_suggest(swt_core):
    from JavaGui.spy import generator as G
    # The normalized SWT tree is non-trivial and dump-tree returns visible rows.
    assert len(swt_core._flat) > 0
    rows = swt_core.dump_tree()
    assert len(rows) > 0
    # Widget types come through the normalizer (Button/Label/Composite/... not "Component").
    types = {G.node_type(r["node"]) for r in swt_core._flat}
    assert types - {"Component", "Widget"}
    # A node with a name or text yields at least one verified locator candidate.
    named = [r for r in swt_core._flat if G.node_name(r["node"]) or G.node_text(r["node"])]
    assert named
    res = swt_core.suggest(named[0]["node_id"])
    assert res["candidates"], "SpyCore.suggest returned no candidates for a named SWT widget"
