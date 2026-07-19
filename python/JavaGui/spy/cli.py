"""javagui-spy CLI — stateless, JSON-first, exit-code-driven verbs for humans and AI agents.

Because the tool holds no daemon state between invocations, each command that needs the live app
takes connection flags (--host/--port/--toolkit or --launch JAR) and does connect → work →
disconnect. Every command prints one JSON envelope: {ok, command, data, meta}.

validate exit codes (control flow for agents):  0 unique · 2 parse/usage error · 3 zero matches · 4 ambiguous
"""
from __future__ import annotations
import argparse
import json
import sys

from . import generator as G
from .core import SpyCore, SpyError

SCHEMA = {
    "verbs": {
        "dump-tree": "compact node rows (node_id,type,name,text,bounds,depth); --visible-only default",
        "find": "resolve LOCATOR -> matching nodes (match_count)",
        "validate": "resolve LOCATOR; exit 0=unique 3=zero 4=ambiguous 2=error; --expect-id N",
        "suggest": "ranked verified locator candidates for --node-id N (--strip-names to sim no-name)",
        "describe": "properties + ancestor breadcrumb for --node-id N",
        "screenshot": "capture app/widget PNG to -o FILE",
        "pick": "deepest widget at --at X,Y (in-JVM hit-test) + ancestor path; --suggest to add locators, --flash to highlight",
        "highlight": "flash a hollow border around --node-id N for --duration ms",
        "ui": "serve the localhost-only web inspector (--ui-port, --no-browser)",
        "mcp": "run as an MCP server (stdio) exposing the verbs as tools for AI agents",
        "schema": "this document + a locator grammar cheatsheet",
    },
    "grammar_cheatsheet": [
        "JButton[name='ok']", "JButton[text='Save']", "#okButton", "text:Login",
        "JToolBar[name='main'] >> JButton[text='Save']", "//JButton[@text='OK']",
        "JPanel:has(JLabel[text='Total']) >> JTextField", "JButton:nth-of-type(2)",
    ],
    "candidate_contract": ["locator", "strategy", "match_count", "unique", "stability",
                            "score", "brittle_flags", "preconditions"],
}


def _emit(command: str, data, core: SpyCore | None = None, ok: bool = True) -> None:
    meta = {}
    if core is not None:
        meta = {"toolkit": core.toolkit, "tree_timestamp": core._tree_ts}
    print(json.dumps({"ok": ok, "command": command, "data": data, "meta": meta}, indent=2))


def _err(command: str, code: str, message: str, **extra) -> None:
    print(json.dumps({"ok": False, "command": command,
                      "error": {"code": code, "message": message, **extra}}, indent=2))


def _connect(args) -> SpyCore:
    core = SpyCore(toolkit=args.toolkit)
    if args.launch:
        core.launch(args.launch, port=args.port)
    else:
        core.connect(host=args.host, port=args.port, timeout=args.timeout)
    return core


def _add_conn(p):
    p.add_argument("--host", default="localhost")
    p.add_argument("--port", type=int, default=None)
    p.add_argument("--toolkit", default="swing", choices=["swing", "swt", "rcp"])
    p.add_argument("--timeout", type=float, default=30)
    p.add_argument("--launch", metavar="JAR", default=None, help="launch JAR with the agent instead of connecting")


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(prog="javagui-spy",
                                 description="Scan Java GUIs and generate unique Robot Framework locators.")
    sub = ap.add_subparsers(dest="cmd", required=True)

    ps = sub.add_parser("schema", help="print the verb + grammar schema (agent bootstrap)")

    p = sub.add_parser("dump-tree"); _add_conn(p); p.add_argument("--all", action="store_true", help="include invisible")
    p = sub.add_parser("find"); _add_conn(p); p.add_argument("locator")
    p = sub.add_parser("validate"); _add_conn(p); p.add_argument("locator"); p.add_argument("--expect-id", type=int, default=None)
    p = sub.add_parser("suggest"); _add_conn(p); p.add_argument("--node-id", type=int, required=True)
    p.add_argument("--top", type=int, default=3); p.add_argument("--strip-names", action="store_true")
    p = sub.add_parser("describe"); _add_conn(p); p.add_argument("--node-id", type=int, required=True)
    p = sub.add_parser("screenshot"); _add_conn(p); p.add_argument("-o", "--out", required=True); p.add_argument("--annotate", default=None)
    p = sub.add_parser("pick"); _add_conn(p); p.add_argument("--at", metavar="X,Y", default=None)
    p.add_argument("--arm", action="store_true", help="wait for a Ctrl+Shift+click in the app instead of --at")
    p.add_argument("--arm-timeout", type=int, default=15000, metavar="MS")
    p.add_argument("--suggest", action="store_true"); p.add_argument("--flash", action="store_true")
    p = sub.add_parser("highlight"); _add_conn(p); p.add_argument("--node-id", type=int, required=True); p.add_argument("--duration", type=int, default=1500)
    p = sub.add_parser("ui", help="serve the local web inspector (localhost only)")
    _add_conn(p); p.add_argument("--ui-port", type=int, default=8123); p.add_argument("--no-browser", action="store_true")
    p = sub.add_parser("mcp", help="run an MCP server (stdio) exposing the verbs as tools")
    _add_conn(p)

    args = ap.parse_args(argv)

    if args.cmd == "schema":
        _emit("schema", SCHEMA)
        return 0

    if args.cmd == "mcp":
        from . import mcp  # lazy
        try:
            mcp.run(toolkit=args.toolkit, host=args.host, port=args.port,
                    timeout=args.timeout, launch=args.launch)
        except KeyboardInterrupt:
            pass
        except Exception as e:
            _err("mcp", type(e).__name__, str(e))
            return 2
        return 0

    if args.cmd == "ui":
        from . import server  # lazy: only the UI verb needs the http stack
        try:
            server.run(toolkit=args.toolkit, host=args.host, port=args.port,
                       timeout=args.timeout, launch=args.launch,
                       ui_port=args.ui_port, open_browser=not args.no_browser)
        except KeyboardInterrupt:
            pass
        except SpyError as e:
            _err("ui", "SPY_ERROR", str(e))
            return 2
        except Exception as e:
            _err("ui", type(e).__name__, str(e))
            return 2
        return 0

    # A locator syntax error is independent of the running app: surface a machine-readable
    # parse error (byte position + valid-vocabulary hint) up front, before we even connect.
    if args.cmd == "validate":
        pe = G.explain_locator(args.locator)
        if not pe.get("ok"):
            _err("validate", "PARSE_ERROR", pe.get("message") or "invalid locator syntax",
                 position=pe.get("position"), hint=pe.get("hint"))
            return 2

    core = None
    try:
        core = _connect(args)
        if args.cmd == "dump-tree":
            _emit("dump-tree", core.dump_tree(visible_only=not args.all), core)
        elif args.cmd == "find":
            _emit("find", core.find(args.locator), core)
        elif args.cmd == "validate":
            v = core.validate(args.locator, expect_id=args.expect_id)
            _emit("validate", v, core, ok=v["unique"])
            return 0 if v["unique"] else (3 if v["match_count"] == 0 else 4)
        elif args.cmd == "suggest":
            _emit("suggest", core.suggest(args.node_id, top=args.top, strip_names=args.strip_names), core)
        elif args.cmd == "describe":
            _emit("describe", core.describe(args.node_id), core)
        elif args.cmd == "screenshot":
            _emit("screenshot", {"path": core.screenshot(args.out, annotate=args.annotate)}, core)
        elif args.cmd == "pick":
            if args.arm:
                hit = core.arm_pick(args.arm_timeout)
            else:
                if not args.at:
                    _err("pick", "USAGE", "pass --at X,Y or --arm")
                    return 2
                try:
                    x, y = (int(v) for v in args.at.split(","))
                except Exception:
                    _err("pick", "USAGE", "--at expects X,Y (e.g. --at 232,38)")
                    return 2
                hit = core.hit_test(x, y)
            data = {"hit": hit}
            if hit.get("hit"):
                if args.flash:
                    core.highlight(hit["id"])
                if args.suggest:
                    data["suggest"] = core.suggest(hit["id"])
            _emit("pick", data, core, ok=bool(hit.get("hit")))
        elif args.cmd == "highlight":
            _emit("highlight", core.highlight(args.node_id, args.duration), core)
        return 0
    except SpyError as e:
        _err(args.cmd, "SPY_ERROR", str(e))
        return 2
    except Exception as e:  # transport / parse / connection
        _err(args.cmd, type(e).__name__, str(e))
        return 2
    finally:
        if core is not None:
            core.disconnect()


if __name__ == "__main__":
    sys.exit(main())
