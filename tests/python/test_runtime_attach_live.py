"""Live runtime-attach coverage: drive a Swing JVM that was launched WITHOUT ``-javaagent``.

Self-skips (like tests/spy/test_phase45.py) unless java + the agent jar + the swing test app +
a DISPLAY (xvfb) are all present. Proves the JDK Attach-API path end to end:
  * launch ``java -jar <swing app>`` with NO ``-javaagent``
  * ``List Applications`` discovers the app pid
  * ``Attach To Application    main_class=swing-test-app`` injects the agent and connects
  * the connected library finds real JButtons in the running app
"""
import os
import shutil
import subprocess
import time

import pytest

REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
AGENT = os.path.join(REPO, "python", "JavaGui", "jars", "javagui-agent.jar")
APP = os.path.join(REPO, "tests", "apps", "swing", "target", "swing-test-app-1.0.0.jar")

pytestmark = pytest.mark.skipif(
    not (shutil.which("java") and os.path.exists(AGENT) and os.path.exists(APP)
         and os.environ.get("DISPLAY")),
    reason="needs java + agent jar + swing app + DISPLAY (xvfb)",
)


@pytest.fixture()
def running_app():
    """Launch the Swing test app as a plain JVM (no -javaagent) and yield its process."""
    proc = subprocess.Popen(
        ["java", "-jar", APP],
        stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
    )
    # give the JVM + Swing frame time to come up before we try to attach
    time.sleep(6)
    if proc.poll() is not None:
        pytest.skip(f"swing test app exited early (rc={proc.returncode}); cannot test attach")
    try:
        yield proc
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=10)
        except subprocess.TimeoutExpired:
            proc.kill()


def test_list_applications_finds_running_pid(running_app):
    from JavaGui import SwingLibrary

    lib = SwingLibrary()
    apps = lib.list_applications()
    assert isinstance(apps, list) and apps, "expected at least one discovered JVM"
    pids = {a["pid"] for a in apps}
    assert running_app.pid in pids, f"launched pid {running_app.pid} not in {sorted(pids)}"
    ours = next(a for a in apps if a["pid"] == running_app.pid)
    assert "swing-test-app" in (ours["main_class"] or ours["command_line"])
    assert ours["is_launcher"] is False


def test_attach_to_running_swing_app_and_find_buttons(running_app):
    from JavaGui import SwingLibrary

    lib = SwingLibrary()
    try:
        # Inject the agent at runtime into the already-running JVM and connect.
        lib.attach_to_application(main_class="swing-test-app")
        assert lib.is_connected() is True

        buttons = lib.find_elements("JButton")
        assert len(buttons) > 0, "attached but found no JButtons in the running app"
    finally:
        try:
            lib.disconnect()
        except Exception:
            pass


def test_attach_by_pid(running_app):
    """Attaching by explicit pid is the most deterministic path and also connects."""
    from JavaGui import SwingLibrary

    lib = SwingLibrary()
    try:
        lib.attach_to_application(pid=running_app.pid)
        assert lib.is_connected() is True
        assert len(lib.find_elements("JButton")) > 0
    finally:
        try:
            lib.disconnect()
        except Exception:
            pass
