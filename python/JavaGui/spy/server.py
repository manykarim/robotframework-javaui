"""javagui-spy web GUI server — a stdlib-only, localhost-only inspector over one SpyCore.

Design constraints (deliberate):
  * NO Flask/Electron/npm — just ``http.server.ThreadingHTTPServer`` from the stdlib.
  * BIND 127.0.0.1 ONLY. The Java agent's JSON-RPC channel has no authentication, so the
    web surface that fronts it must never be reachable off-host.
  * Hold exactly ONE ``SpyCore`` (already connected/launched) shared across request threads.

Routes (JSON envelope {ok, command, data, meta} where practical):
  GET  /                              -> static/spy.html
  GET  /api/tree?all=0                -> core.dump_tree(visible_only=not all)
  GET  /api/describe?node_id=N        -> core.describe(N)
  GET  /api/suggest?node_id=N&strip_names=0
  GET  /api/find?locator=...          -> core.find(locator)
  GET  /api/validate?locator=...&expect_id=N
  GET  /api/screenshot                -> image/png bytes (temp capture)
  POST /api/pick        {x,y}         -> core.hit_test(x,y)
  POST /api/highlight   {node_id}     -> core.highlight(node_id)
  GET  /events                        -> Server-Sent Events; emits on ui_generation() change

Public API: ``serve(core, host, port)`` and ``run(...)`` (constructs a SpyCore, then serves).
"""
from __future__ import annotations

import json
import tempfile
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from urllib.parse import parse_qs, urlparse

from .core import SpyCore, SpyError

_STATIC = Path(__file__).resolve().parent / "static"
_POLL_SECONDS = 0.4
_SSE_HEARTBEAT_SECONDS = 15.0


class _Handler(BaseHTTPRequestHandler):
    """One shared SpyCore is attached to the server (``server.core``); a lock serialises
    access because SpyCore's socket/library client is not thread-safe."""

    protocol_version = "HTTP/1.1"
    server_version = "javagui-spy"

    # ---- plumbing -------------------------------------------------------
    @property
    def core(self) -> SpyCore:
        return self.server.core  # type: ignore[attr-defined]

    @property
    def lock(self) -> threading.Lock:
        return self.server.core_lock  # type: ignore[attr-defined]

    def log_message(self, *_args):  # silence default stderr access log
        pass

    def _send_json(self, obj, status: int = 200) -> None:
        body = json.dumps(obj).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json; charset=utf-8")
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(body)

    def _send_bytes(self, data: bytes, content_type: str, status: int = 200) -> None:
        self.send_response(status)
        self.send_header("Content-Type", content_type)
        self.send_header("Content-Length", str(len(data)))
        self.send_header("Cache-Control", "no-store")
        self.end_headers()
        self.wfile.write(data)

    def _envelope(self, command: str, data, ok: bool = True):
        meta = {"toolkit": self.core.toolkit, "tree_timestamp": self.core._tree_ts}
        return {"ok": ok, "command": command, "data": data, "meta": meta}

    def _read_body(self) -> dict:
        length = int(self.headers.get("Content-Length") or 0)
        if length <= 0:
            return {}
        raw = self.rfile.read(length)
        try:
            return json.loads(raw.decode("utf-8")) or {}
        except (ValueError, UnicodeDecodeError):
            return {}

    # ---- routing --------------------------------------------------------
    def do_GET(self):  # noqa: N802 (http.server naming)
        parsed = urlparse(self.path)
        path = parsed.path
        q = parse_qs(parsed.query)
        try:
            if path == "/" or path == "/index.html":
                return self._serve_static("spy.html")
            if path == "/events":
                return self._serve_events()
            if path == "/api/tree":
                include_all = _flag(q, "all")
                with self.lock:
                    data = self.core.dump_tree(visible_only=not include_all)
                return self._send_json(self._envelope("tree", data))
            if path == "/api/describe":
                nid = _int(q, "node_id")
                with self.lock:
                    data = self.core.describe(nid)
                return self._send_json(self._envelope("describe", data))
            if path == "/api/suggest":
                nid = _int(q, "node_id")
                strip = _flag(q, "strip_names")
                with self.lock:
                    data = self.core.suggest(nid, strip_names=strip)
                return self._send_json(self._envelope("suggest", data))
            if path == "/api/find":
                loc = _str(q, "locator")
                with self.lock:
                    data = self.core.find(loc)
                return self._send_json(self._envelope("find", data))
            if path == "/api/validate":
                loc = _str(q, "locator")
                expect = _int(q, "expect_id", required=False)
                with self.lock:
                    data = self.core.validate(loc, expect_id=expect)
                ok = bool(data.get("unique"))
                return self._send_json(self._envelope("validate", data, ok=ok))
            if path == "/api/screenshot":
                return self._serve_screenshot()
            return self._send_json({"ok": False, "command": path,
                                    "error": {"code": "NOT_FOUND", "message": path}}, status=404)
        except SpyError as e:
            return self._send_json({"ok": False, "command": path,
                                    "error": {"code": "SPY_ERROR", "message": str(e)}}, status=400)
        except BrokenPipeError:
            return
        except Exception as e:  # transport / parse
            return self._send_json({"ok": False, "command": path,
                                    "error": {"code": type(e).__name__, "message": str(e)}}, status=500)

    def do_POST(self):  # noqa: N802
        parsed = urlparse(self.path)
        path = parsed.path
        try:
            body = self._read_body()
            if path == "/api/pick":
                x, y = int(body.get("x", 0)), int(body.get("y", 0))
                with self.lock:
                    hit = self.core.hit_test(x, y)
                ok = bool(hit.get("hit"))
                return self._send_json(self._envelope("pick", {"hit": hit}, ok=ok))
            if path == "/api/highlight":
                nid = int(body.get("node_id"))
                dur = int(body.get("duration_ms", 1500))
                with self.lock:
                    data = self.core.highlight(nid, dur)
                return self._send_json(self._envelope("highlight", data))
            return self._send_json({"ok": False, "command": path,
                                    "error": {"code": "NOT_FOUND", "message": path}}, status=404)
        except SpyError as e:
            return self._send_json({"ok": False, "command": path,
                                    "error": {"code": "SPY_ERROR", "message": str(e)}}, status=400)
        except BrokenPipeError:
            return
        except Exception as e:
            return self._send_json({"ok": False, "command": path,
                                    "error": {"code": type(e).__name__, "message": str(e)}}, status=500)

    # ---- static + binary ------------------------------------------------
    def _serve_static(self, name: str) -> None:
        # Guard against traversal: only a bare basename under _STATIC is served.
        safe = Path(name).name
        fp = _STATIC / safe
        if not fp.is_file():
            return self._send_json({"ok": False, "error": {"code": "NOT_FOUND",
                                                            "message": safe}}, status=404)
        data = fp.read_bytes()
        ctype = "text/html; charset=utf-8" if safe.endswith(".html") else "application/octet-stream"
        return self._send_bytes(data, ctype)

    def _serve_screenshot(self) -> None:
        with tempfile.NamedTemporaryFile(suffix=".png", delete=False) as tf:
            tmp = tf.name
        try:
            with self.lock:
                path = self.core.screenshot(tmp)
            data = Path(path).read_bytes()
            return self._send_bytes(data, "image/png")
        finally:
            try:
                Path(tmp).unlink()
            except OSError:
                pass

    # ---- Server-Sent Events --------------------------------------------
    def _serve_events(self) -> None:
        self.send_response(200)
        self.send_header("Content-Type", "text/event-stream")
        self.send_header("Cache-Control", "no-store")
        self.send_header("Connection", "keep-alive")
        self.end_headers()
        last_gen: int | None = None
        last_beat = time.monotonic()
        try:
            # Prime the client with the current generation so it renders immediately.
            while True:
                try:
                    with self.lock:
                        gen = self.core.ui_generation()
                except SpyError:
                    gen = last_gen if last_gen is not None else 0
                except Exception:
                    gen = last_gen if last_gen is not None else 0
                now = time.monotonic()
                if gen != last_gen:
                    last_gen = gen
                    payload = json.dumps({"generation": gen,
                                          "tree_timestamp": self.core._tree_ts})
                    self.wfile.write(f"event: change\ndata: {payload}\n\n".encode())
                    self.wfile.flush()
                    last_beat = now
                elif now - last_beat >= _SSE_HEARTBEAT_SECONDS:
                    # Comment line keeps the socket alive and surfaces client disconnects.
                    self.wfile.write(b": keep-alive\n\n")
                    self.wfile.flush()
                    last_beat = now
                time.sleep(_POLL_SECONDS)
        except (BrokenPipeError, ConnectionResetError, OSError):
            return  # client went away — end the handler cleanly


# ---- query helpers ------------------------------------------------------
def _str(q: dict, key: str, default: str = "") -> str:
    vals = q.get(key)
    return vals[0] if vals else default


def _int(q: dict, key: str, required: bool = True):
    vals = q.get(key)
    if not vals:
        if required:
            raise SpyError(f"missing required query parameter '{key}'")
        return None
    try:
        return int(vals[0])
    except (TypeError, ValueError):
        raise SpyError(f"query parameter '{key}' must be an integer")


def _flag(q: dict, key: str) -> bool:
    vals = q.get(key)
    if not vals:
        return False
    return vals[0] not in ("0", "false", "False", "", "no")


# ---- public entrypoints -------------------------------------------------
def serve(core: SpyCore, host: str = "127.0.0.1", port: int = 8123) -> ThreadingHTTPServer:
    """Bind a localhost-only web server fronting ``core`` and serve forever.

    Binds 127.0.0.1 unconditionally regardless of ``host`` unless the caller passes an
    explicit loopback alias; a non-loopback host is rejected because the agent RPC is unauthed.
    """
    if host not in ("127.0.0.1", "localhost", "::1"):
        raise SpyError(f"refusing to bind non-loopback host {host!r}; the agent RPC has no auth")
    bind = "127.0.0.1"
    httpd = ThreadingHTTPServer((bind, port), _Handler)
    httpd.daemon_threads = True
    httpd.core = core  # type: ignore[attr-defined]
    httpd.core_lock = threading.Lock()  # type: ignore[attr-defined]
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        httpd.shutdown()
        httpd.server_close()
    return httpd


def run(toolkit: str = "swing", host: str = "localhost", port: int | None = None,
        timeout: float = 30, launch: str | None = None,
        ui_host: str = "127.0.0.1", ui_port: int = 8123,
        open_browser: bool = True) -> None:
    """Construct a SpyCore (connect or launch), print the URL, then serve until interrupted."""
    core = SpyCore(toolkit=toolkit)
    if launch:
        core.launch(launch, port=port)
    else:
        core.connect(host=host, port=port, timeout=timeout)
    url = f"http://{ui_host}:{ui_port}/"
    print(f"javagui-spy UI serving at {url}  (toolkit={core.toolkit}, Ctrl-C to stop)")
    if open_browser:
        try:
            import webbrowser
            webbrowser.open(url)
        except Exception:
            pass
    try:
        serve(core, host=ui_host, port=ui_port)
    finally:
        core.disconnect()
