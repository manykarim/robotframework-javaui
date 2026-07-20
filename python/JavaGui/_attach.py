"""Runtime dynamic attach — discover running JVMs and inject the agent by PID.

This is the toolkit-agnostic core behind `Attach To Application` / `List Applications` and the
Java Web Start flow. It lets the library drive a JVM that was started WITHOUT ``-javaagent`` by
loading the agent via the JDK Attach API (``agentmain``) after the app is already up.

Two injection vectors:
  * JDK Attach (default) — run the bundled agent jar's ``AttachMain`` under a JDK ``java``
    (``com.sun.tools.attach``). Requires a JDK on THIS host; the target only needs to be an
    attachable HotSpot JVM owned by the same user.
  * jattach (fallback) — a standalone binary for JRE-only hosts; used when ``JAVAGUI_JATTACH``
    points at a ``jattach`` executable (or one is on PATH) and no JDK ``java`` is usable.

Discovery reads ``/proc/<pid>/cmdline`` (full command line, avoids ``jps`` package truncation)
and falls back to ``jcmd -l`` / ``jps -l`` on non-Linux.
"""
from __future__ import annotations

import os
import re
import shutil
import socket
import subprocess
import sys
import time
from dataclasses import dataclass, field
from pathlib import Path
from typing import List, Optional


class AttachError(Exception):
    """Raised when discovery finds no/ambiguous target or injection fails."""


@dataclass
class JvmProcess:
    pid: int
    command_line: str = ""
    main_class: Optional[str] = None
    display_name: str = ""
    is_launcher: bool = False  # a WebStart/bootstrap launcher, not an application JVM
    markers: List[str] = field(default_factory=list)

    def to_dict(self) -> dict:
        return {
            "pid": self.pid,
            "main_class": self.main_class,
            "display_name": self.display_name,
            "command_line": self.command_line,
            "is_launcher": self.is_launcher,
            "markers": self.markers,
        }


# WebStart / bootstrap launcher markers — these JVMs are the launcher, not the app.
_LAUNCHER_MARKERS = (
    "net.sourceforge.jnlp.runtime.Boot",
    "com.sun.javaws.Main",
    "openwebstart.jar",
    "com.openwebstart.",
    "net.sourceforge.jnlp.runtime.Boot.basedir",
)
# our own tooling — never an attach target
_SELF_MARKERS = ("com.robotframework.attach.AttachMain", "sun.tools.jps", "jdk.jcmd")


def _own_pid_tree() -> set:
    """PIDs of this process and its ancestors — never attach to ourselves."""
    pids = set()
    pid = os.getpid()
    for _ in range(64):
        if pid <= 1 or pid in pids:
            break
        pids.add(pid)
        try:
            with open(f"/proc/{pid}/stat") as fh:
                pid = int(fh.read().split(") ", 1)[1].split()[1])  # PPID
        except (OSError, IndexError, ValueError):
            break
    return pids


def _cmdline(pid: int) -> Optional[str]:
    try:
        with open(f"/proc/{pid}/cmdline", "rb") as fh:
            return fh.read().replace(b"\0", b" ").decode("utf-8", "ignore").strip()
    except OSError:
        return None


def _main_class_from_cmdline(cl: str) -> Optional[str]:
    """Best-effort main class / entry jar from a full java command line."""
    toks = cl.split()
    for i, t in enumerate(toks):
        if t == "-jar" and i + 1 < len(toks):
            return Path(toks[i + 1]).name
    # first non-option token after java that isn't a flag value = main class
    seen_java = False
    skip_next = False
    for t in toks:
        if skip_next:
            skip_next = False
            continue
        base = t.rsplit("/", 1)[-1]
        if base == "java" or base.endswith("java"):
            seen_java = True
            continue
        if not seen_java:
            continue
        if t in ("-cp", "-classpath", "--class-path", "-p", "--module-path", "--add-modules"):
            skip_next = True
            continue
        if t.startswith("-") or t.startswith("@"):
            continue
        return t
    return None


def discover_jvms(include_launchers: bool = True) -> List[JvmProcess]:
    """Enumerate running Java processes owned by this user, classified app-vs-launcher."""
    own = _own_pid_tree()
    procs: List[JvmProcess] = []
    if sys.platform.startswith("linux") and os.path.isdir("/proc"):
        for entry in os.listdir("/proc"):
            if not entry.isdigit():
                continue
            pid = int(entry)
            if pid in own:
                continue
            cl = _cmdline(pid)
            if not cl:
                continue
            first = cl.split(" ", 1)[0].rsplit("/", 1)[-1]
            if first != "java" and not first.endswith("java") and " -jar " not in f" {cl} ":
                if "java" not in first:
                    continue
            if any(m in cl for m in _SELF_MARKERS):
                continue
            markers = [m for m in _LAUNCHER_MARKERS if m in cl]
            mc = _main_class_from_cmdline(cl)
            procs.append(JvmProcess(
                pid=pid, command_line=cl, main_class=mc,
                display_name=mc or f"pid {pid}",
                is_launcher=bool(markers), markers=markers,
            ))
    else:
        procs = _discover_via_jcmd(own)
    if not include_launchers:
        procs = [p for p in procs if not p.is_launcher]
    return procs


def _discover_via_jcmd(own: set) -> List[JvmProcess]:
    tool = shutil.which("jcmd") or shutil.which("jps")
    if not tool:
        return []
    flag = "-l" if tool.endswith("jps") else "-l"
    try:
        out = subprocess.run([tool, flag], capture_output=True, text=True, timeout=15).stdout
    except (OSError, subprocess.SubprocessError):
        return []
    procs = []
    for line in out.splitlines():
        parts = line.split(None, 1)
        if not parts or not parts[0].isdigit():
            continue
        pid = int(parts[0])
        if pid in own:
            continue
        rest = parts[1] if len(parts) > 1 else ""
        if any(m in rest for m in _SELF_MARKERS) or "Jcmd" in rest or rest.strip() == "jdk.jcmd/sun.tools.jcmd.JCmd":
            continue
        markers = [m for m in _LAUNCHER_MARKERS if m in rest]
        procs.append(JvmProcess(pid=pid, command_line=rest, main_class=rest or None,
                                display_name=rest or f"pid {pid}",
                                is_launcher=bool(markers), markers=markers))
    return procs


def _window_title_pids(pattern: str) -> set:
    """PIDs owning an X window whose title matches ``pattern`` (best effort, needs wmctrl)."""
    wmctrl = shutil.which("wmctrl")
    if not wmctrl:
        return set()
    rx = re.compile(pattern.replace("*", ".*"))
    pids = set()
    try:
        out = subprocess.run([wmctrl, "-lp"], capture_output=True, text=True, timeout=10).stdout
        for line in out.splitlines():
            cols = line.split(None, 4)
            if len(cols) >= 5 and rx.search(cols[4]):
                pids.add(int(cols[2]))
    except (OSError, subprocess.SubprocessError, ValueError):
        pass
    return pids


def select_jvm(pid: Optional[int] = None, main_class: Optional[str] = None,
               title: Optional[str] = None) -> JvmProcess:
    """Pick exactly one application JVM by pid / main_class regex / window title.

    Raises AttachError on zero or multiple matches (never guesses). Launcher/bootstrap JVMs are
    excluded unless targeted explicitly by pid.
    """
    if pid is not None:
        cl = _cmdline(int(pid)) or ""
        return JvmProcess(pid=int(pid), command_line=cl,
                          main_class=_main_class_from_cmdline(cl),
                          display_name=_main_class_from_cmdline(cl) or f"pid {pid}",
                          markers=[m for m in _LAUNCHER_MARKERS if m in cl],
                          is_launcher=any(m in cl for m in _LAUNCHER_MARKERS))
    candidates = [p for p in discover_jvms(include_launchers=True) if not p.is_launcher]
    if main_class is not None:
        rx = re.compile(main_class)
        candidates = [p for p in candidates
                      if (p.main_class and rx.search(p.main_class)) or rx.search(p.command_line)]
    if title is not None:
        want = _window_title_pids(title)
        if not want:
            raise AttachError(
                f"no window title matched {title!r} (needs 'wmctrl'); select by pid= or main_class= instead")
        candidates = [p for p in candidates if p.pid in want]
    if not candidates:
        raise AttachError(
            f"no application JVM matched (pid={pid}, main_class={main_class!r}, title={title!r}). "
            f"Run 'List Applications' to see candidates.")
    if len(candidates) > 1:
        listing = "; ".join(f"pid={p.pid} {p.display_name}" for p in candidates)
        raise AttachError(
            f"ambiguous target: {len(candidates)} JVMs matched — {listing}. "
            f"Disambiguate with pid= or a more specific main_class=.")
    return candidates[0]


def free_port(host: str = "127.0.0.1") -> int:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind((host, 0))
        return s.getsockname()[1]


def _jdk_java() -> Optional[str]:
    """A ``java`` that has the jdk.attach module (needed to run AttachMain)."""
    for cand in (os.environ.get("JAVAGUI_JAVA"), shutil.which("java")):
        if not cand:
            continue
        try:
            out = subprocess.run([cand, "--list-modules"], capture_output=True, text=True, timeout=15).stdout
            if "jdk.attach" in out:
                return cand
        except (OSError, subprocess.SubprocessError):
            continue
    return None


def inject_agent(pid: int, agent_jar: str, port: int, toolkit: str = "auto",
                 host: str = "127.0.0.1", timeout: float = 30.0) -> None:
    """Load the agent into the running JVM ``pid`` and wait for its RPC port to open.

    Uses the JDK Attach API via the agent jar's AttachMain (default), or ``jattach`` when only a
    JRE is present. Raises AttachError with an actionable message on failure (incl. the common
    SecurityManager-denied case for sandboxed JNLP apps).
    """
    agent_jar = str(Path(agent_jar).resolve())
    agent_args = f"port={port},host={host},toolkit={toolkit}"
    java = _jdk_java()
    jattach = os.environ.get("JAVAGUI_JATTACH") or shutil.which("jattach")

    if java:
        cmd = [java, "--add-modules", "jdk.attach", "-cp", agent_jar,
               "com.robotframework.attach.AttachMain", str(pid), agent_jar, agent_args]
    elif jattach:
        # jattach load <jar> false <options> ; boolean=false => absolute path, not from tmp
        cmd = [jattach, str(pid), "load", "instrument", "false", f"{agent_jar}={agent_args}"]
    else:
        raise AttachError(
            "no JDK 'java' (with jdk.attach) and no 'jattach' found to inject the agent. "
            "Install a JDK, set JAVAGUI_JAVA, or provide jattach via JAVAGUI_JATTACH.")

    try:
        res = subprocess.run(cmd, capture_output=True, text=True, timeout=max(timeout, 10))
    except (OSError, subprocess.SubprocessError) as e:
        raise AttachError(f"attach process failed to run: {e}") from e
    if res.returncode == 4 or "AGENT_INIT_FAILED" in res.stderr:
        raise AttachError(
            "agent loaded but failed to initialize in the target JVM — a restrictive "
            "SecurityManager (e.g. a sandboxed IcedTea-Web JNLP app) most likely denied it. "
            "All-permissions apps and modern OpenWebStart / JDK 24+ do not have this limit. "
            f"[details] {res.stderr.strip()[:300]}")
    if res.returncode != 0:
        raise AttachError(f"attach failed (rc={res.returncode}): {res.stderr.strip()[:300] or res.stdout.strip()[:300]}")

    # Wait for the agent's RPC port to accept connections.
    deadline = time.time() + timeout
    while time.time() < deadline:
        try:
            with socket.create_connection((host, port), timeout=1):
                return
        except OSError:
            time.sleep(0.3)
    raise AttachError(f"agent attached to pid {pid} but RPC port {host}:{port} never opened within {timeout}s")


def _java_children(pid: int) -> List[int]:
    """PIDs of Java child processes of ``pid`` (the forked-app-JVM case)."""
    kids = []
    if not (sys.platform.startswith("linux") and os.path.isdir("/proc")):
        return kids
    for entry in os.listdir("/proc"):
        if not entry.isdigit():
            continue
        cpid = int(entry)
        try:
            with open(f"/proc/{cpid}/stat") as fh:
                ppid = int(fh.read().split(") ", 1)[1].split()[1])
        except (OSError, IndexError, ValueError):
            continue
        if ppid == pid:
            cl = _cmdline(cpid) or ""
            if "java" in cl.split(" ", 1)[0].rsplit("/", 1)[-1]:
                kids.append(cpid)
    return kids


def resolve_webstart_launcher(launcher: Optional[str] = None) -> List[str]:
    """Return a command prefix that launches a .jnlp. Accepts an explicit binary/dir, else
    JAVAGUI_JAVAWS, else `javaws` on PATH. A directory is treated as an IcedTea-Web image."""
    cand = launcher or os.environ.get("JAVAGUI_JAVAWS") or shutil.which("javaws")
    if not cand:
        raise AttachError(
            "no Web Start launcher found. Install OpenWebStart/IcedTea-Web (`javaws` on PATH), "
            "or set JAVAGUI_JAVAWS to the launcher binary or an IcedTea-Web image directory.")
    p = Path(cand)
    if p.is_dir():
        jar = p / "share" / "icedtea-web" / "javaws.jar"
        args = p / "bin" / "itw-modularjdk.args"
        if jar.exists():
            java = _jdk_java() or shutil.which("java") or "java"
            cp = f"{jar}:{p/'linux-deps-runtime'/'*'}:{p/'share'/'icedtea-web'/'jsobject.jar'}"
            base = [java]
            if args.exists():
                base.append(f"@{args}")
            return base + ["-cp", cp, "net.sourceforge.jnlp.runtime.Boot"]
        raise AttachError(f"{p} is not an IcedTea-Web image (no share/icedtea-web/javaws.jar)")
    return [str(cand)]


def launch_webstart(jnlp: str, launcher: Optional[str] = None, settle: float = 8.0,
                    timeout: float = 60.0):
    """Launch a .jnlp and return (launcher_process, app_pid). Handles both topologies: a forked
    child app JVM (preferred) or the launcher JVM running the app in-process."""
    cmd = resolve_webstart_launcher(launcher) + [jnlp]
    proc = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    deadline = time.time() + timeout
    # Prefer a forked child JVM; fall back to the launcher pid itself (in-process case).
    while time.time() < deadline:
        kids = _java_children(proc.pid)
        if kids:
            return proc, kids[-1]
        if proc.poll() is not None:
            raise AttachError(f"Web Start launcher exited early (rc={proc.returncode}) launching {jnlp}")
        if time.time() - (deadline - timeout) >= settle:
            return proc, proc.pid  # in-process: the launcher JVM IS the app JVM
        time.sleep(0.5)
    return proc, proc.pid


def attach_and_connect(connect, agent_jar, toolkit, *, pid=None, main_class=None,
                       title=None, host="127.0.0.1", port=None, timeout=30.0) -> JvmProcess:
    """Discover -> inject -> connect. ``connect(host, port, timeout)`` runs the library's own
    connect once the agent's RPC port is open. Returns the selected JvmProcess."""
    jvm = select_jvm(pid=pid, main_class=main_class, title=title)
    use_port = int(port) if port else free_port(host)
    inject_agent(jvm.pid, agent_jar, use_port, toolkit=toolkit, host=host, timeout=timeout)
    connect(host, use_port, timeout)
    return jvm
