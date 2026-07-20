"""Pure unit tests for the runtime-attach feature (JavaGui._attach + attach keywords).

No live JVM, no DISPLAY, no Java required — everything is monkeypatched. These must pass in a
plain ``uv run pytest tests/python/`` run.

Covers:
  * ``_main_class_from_cmdline`` parsing (``-jar`` and ``-cp ... MainClass``)
  * ``select_jvm`` raising ``AttachError`` on zero and on >1 matches
  * ``discover_jvms`` classifying a WebStart bootstrap launcher as ``is_launcher=True``
  * ``inject_agent`` surfacing a SecurityManager hint on attach rc=4 / AGENT_INIT_FAILED
  * ``SwingLibrary.list_applications`` returning a list of dicts
"""
import subprocess

import pytest

from JavaGui import _attach
from JavaGui._attach import AttachError, JvmProcess


# --------------------------------------------------------------------------------------------
# _main_class_from_cmdline
# --------------------------------------------------------------------------------------------

def test_main_class_from_cmdline_jar():
    """A ``-jar x.jar`` launch reports the jar's basename as the entry point."""
    assert _attach._main_class_from_cmdline("java -jar /opt/apps/x.jar") == "x.jar"
    # absolute java path + jar with a directory prefix -> just the file name
    assert _attach._main_class_from_cmdline(
        "/usr/lib/jvm/jdk17/bin/java -Xmx512m -jar /a/b/swing-test-app-1.0.0.jar arg1"
    ) == "swing-test-app-1.0.0.jar"


def test_main_class_from_cmdline_classpath():
    """A ``-cp <cp> com.Main`` launch reports the main class, skipping the classpath value."""
    assert _attach._main_class_from_cmdline(
        "java -cp /a/b:/c com.example.Main arg1"
    ) == "com.example.Main"
    # -classpath alias and an absolute java binary
    assert _attach._main_class_from_cmdline(
        "/usr/bin/java -classpath libs/*:. org.foo.App"
    ) == "org.foo.App"


def test_main_class_from_cmdline_none_when_no_entry():
    """Bare ``java`` with only flags has no discernible entry point."""
    assert _attach._main_class_from_cmdline("java -version") is None


# --------------------------------------------------------------------------------------------
# select_jvm — zero and ambiguous matches raise AttachError (never guesses)
# --------------------------------------------------------------------------------------------

def _jvm(pid, main_class, is_launcher=False):
    return JvmProcess(
        pid=pid, command_line=f"java -jar {main_class}", main_class=main_class,
        display_name=main_class, is_launcher=is_launcher,
    )


def test_select_jvm_zero_matches_raises(monkeypatch):
    monkeypatch.setattr(_attach, "discover_jvms", lambda include_launchers=True: [])
    with pytest.raises(AttachError) as exc:
        _attach.select_jvm(main_class="com.example.Nope")
    assert "no application JVM matched" in str(exc.value)


def test_select_jvm_multiple_matches_raises(monkeypatch):
    procs = [_jvm(101, "com.example.App"), _jvm(202, "com.example.AppTwo")]
    monkeypatch.setattr(_attach, "discover_jvms", lambda include_launchers=True: procs)
    with pytest.raises(AttachError) as exc:
        _attach.select_jvm(main_class="com.example")
    msg = str(exc.value)
    assert "ambiguous target" in msg
    assert "pid=101" in msg and "pid=202" in msg


def test_select_jvm_single_match_returns_it(monkeypatch):
    procs = [_jvm(101, "com.example.App"), _jvm(202, "com.other.Thing")]
    monkeypatch.setattr(_attach, "discover_jvms", lambda include_launchers=True: procs)
    chosen = _attach.select_jvm(main_class="example.App")
    assert chosen.pid == 101


def test_select_jvm_excludes_launchers(monkeypatch):
    """A launcher JVM is not an attach candidate even if its command line matches."""
    procs = [_jvm(303, "net.sourceforge.jnlp.runtime.Boot", is_launcher=True)]
    monkeypatch.setattr(_attach, "discover_jvms", lambda include_launchers=True: procs)
    with pytest.raises(AttachError):
        _attach.select_jvm(main_class="jnlp")


# --------------------------------------------------------------------------------------------
# discover_jvms — classifies a WebStart bootstrap launcher as is_launcher=True
# --------------------------------------------------------------------------------------------

_LAUNCHER_CMDLINE = (
    "/usr/lib/jvm/jdk17/bin/java @/opt/itw/bin/itw-modularjdk.args "
    "-cp /opt/itw/javaws.jar net.sourceforge.jnlp.runtime.Boot /tmp/app.jnlp"
)


def test_discover_jvms_classifies_webstart_launcher(monkeypatch):
    """A JVM whose command line contains the JNLP Boot marker is flagged is_launcher=True and
    is filtered out of the default (app-only) listing."""
    import os

    monkeypatch.setattr(_attach, "_own_pid_tree", lambda: set())
    monkeypatch.setattr(_attach, "_cmdline", lambda pid: _LAUNCHER_CMDLINE)
    real_listdir = os.listdir
    monkeypatch.setattr(
        os, "listdir",
        lambda p: ["9999"] if p == "/proc" else real_listdir(p),
    )

    with_launchers = _attach.discover_jvms(include_launchers=True)
    assert len(with_launchers) == 1
    proc = with_launchers[0]
    assert proc.pid == 9999
    assert proc.is_launcher is True
    assert "net.sourceforge.jnlp.runtime.Boot" in proc.markers

    # default listing (apps only) hides launchers
    assert _attach.discover_jvms(include_launchers=False) == []


def test_discover_jvms_classifies_plain_app_not_launcher(monkeypatch):
    """A plain ``java -jar app.jar`` JVM is an application, not a launcher."""
    import os

    monkeypatch.setattr(_attach, "_own_pid_tree", lambda: set())
    monkeypatch.setattr(_attach, "_cmdline", lambda pid: "/usr/bin/java -jar /opt/my-app.jar")
    real_listdir = os.listdir
    monkeypatch.setattr(
        os, "listdir",
        lambda p: ["8888"] if p == "/proc" else real_listdir(p),
    )
    procs = _attach.discover_jvms(include_launchers=False)
    assert len(procs) == 1
    assert procs[0].pid == 8888
    assert procs[0].is_launcher is False
    assert procs[0].main_class == "my-app.jar"


# --------------------------------------------------------------------------------------------
# inject_agent — rc=4 / AGENT_INIT_FAILED surfaces the SecurityManager hint
# --------------------------------------------------------------------------------------------

def test_inject_agent_security_manager_hint_on_rc4(monkeypatch):
    monkeypatch.setattr(_attach, "_jdk_java", lambda: "/usr/bin/java")

    def fake_run(cmd, **kwargs):
        return subprocess.CompletedProcess(
            cmd, returncode=4, stdout="",
            stderr="AGENT_INIT_FAILED: java.security.AccessControlException denied by JNLPSecurityManager",
        )

    monkeypatch.setattr(_attach.subprocess, "run", fake_run)
    with pytest.raises(AttachError) as exc:
        _attach.inject_agent(1234, "/x/agent.jar", 5000)
    msg = str(exc.value)
    assert "SecurityManager" in msg
    assert "failed to initialize" in msg


def test_inject_agent_agent_init_failed_marker_on_nonzero(monkeypatch):
    """The SecurityManager hint also fires when stderr carries AGENT_INIT_FAILED (any rc)."""
    monkeypatch.setattr(_attach, "_jdk_java", lambda: "/usr/bin/java")

    def fake_run(cmd, **kwargs):
        return subprocess.CompletedProcess(
            cmd, returncode=1, stdout="", stderr="boom AGENT_INIT_FAILED trace",
        )

    monkeypatch.setattr(_attach.subprocess, "run", fake_run)
    with pytest.raises(AttachError) as exc:
        _attach.inject_agent(1234, "/x/agent.jar", 5000)
    assert "SecurityManager" in str(exc.value)


def test_inject_agent_generic_failure_message(monkeypatch):
    """A non-SecurityManager failure reports the raw rc, not the JNLP hint."""
    monkeypatch.setattr(_attach, "_jdk_java", lambda: "/usr/bin/java")

    def fake_run(cmd, **kwargs):
        return subprocess.CompletedProcess(cmd, returncode=2, stdout="", stderr="no such pid")

    monkeypatch.setattr(_attach.subprocess, "run", fake_run)
    with pytest.raises(AttachError) as exc:
        _attach.inject_agent(1234, "/x/agent.jar", 5000)
    msg = str(exc.value)
    assert "rc=2" in msg
    assert "SecurityManager" not in msg


def test_inject_agent_no_injector_available(monkeypatch):
    """Neither a JDK java nor jattach available -> a clear, actionable AttachError."""
    monkeypatch.setattr(_attach, "_jdk_java", lambda: None)
    monkeypatch.setenv("JAVAGUI_JATTACH", "")
    monkeypatch.setattr(_attach.shutil, "which", lambda name: None)
    with pytest.raises(AttachError) as exc:
        _attach.inject_agent(1234, "/x/agent.jar", 5000)
    assert "jattach" in str(exc.value)


# --------------------------------------------------------------------------------------------
# free_port — returns a usable ephemeral port
# --------------------------------------------------------------------------------------------

def test_free_port_returns_int():
    port = _attach.free_port()
    assert isinstance(port, int)
    assert 1024 <= port <= 65535


# --------------------------------------------------------------------------------------------
# SwingLibrary.list_applications -> list of dicts (keyword layer, monkeypatched discovery)
# --------------------------------------------------------------------------------------------

def test_swing_list_applications_returns_dicts(monkeypatch):
    from JavaGui import SwingLibrary

    procs = [
        JvmProcess(pid=42, command_line="java -jar a.jar", main_class="a.jar", display_name="a.jar"),
        JvmProcess(pid=43, command_line="java -cp . com.B", main_class="com.B", display_name="com.B"),
    ]
    monkeypatch.setattr(_attach, "discover_jvms", lambda include_launchers=False: procs)

    lib = SwingLibrary()
    apps = lib.list_applications()
    assert isinstance(apps, list)
    assert len(apps) == 2
    assert all(isinstance(a, dict) for a in apps)
    first = apps[0]
    for key in ("pid", "main_class", "display_name", "command_line", "is_launcher", "markers"):
        assert key in first
    assert first["pid"] == 42
    assert first["main_class"] == "a.jar"
    assert first["is_launcher"] is False


def test_swing_list_applications_forwards_include_launchers(monkeypatch):
    """The keyword passes include_launchers through to discovery."""
    from JavaGui import SwingLibrary

    seen = {}

    def fake_discover(include_launchers=False):
        seen["include_launchers"] = include_launchers
        return []

    monkeypatch.setattr(_attach, "discover_jvms", fake_discover)
    lib = SwingLibrary()
    assert lib.list_applications(include_launchers=True) == []
    assert seen["include_launchers"] is True
