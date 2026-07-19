"""MCP façade for javagui-spy — the same verbs, exposed as MCP tools over stdio.

A minimal Model Context Protocol server (newline-delimited JSON-RPC 2.0 on stdin/stdout) so an
MCP-capable agent can scan a Java app and author locators as tool calls. It is a fourth thin
client of one SpyCore — no new logic. Launch via `javagui-spy mcp --launch app.jar` (or --port).

Tools: spy_dump_tree, spy_find, spy_validate, spy_suggest, spy_describe, spy_pick, spy_schema.
"""
from __future__ import annotations
import json
import sys

from .core import SpyCore, SpyError

_TOOLS = [
    {"name": "spy_dump_tree", "description": "List visible widget nodes (id,type,name,text,bounds,depth).",
     "inputSchema": {"type": "object", "properties": {"all": {"type": "boolean"}}}},
    {"name": "spy_find", "description": "Resolve a locator; returns match_count + matching nodes.",
     "inputSchema": {"type": "object", "properties": {"locator": {"type": "string"}}, "required": ["locator"]}},
    {"name": "spy_validate", "description": "Check a locator is unique (optionally == expect_id).",
     "inputSchema": {"type": "object", "properties": {"locator": {"type": "string"}, "expect_id": {"type": "integer"}},
                     "required": ["locator"]}},
    {"name": "spy_suggest", "description": "Ranked, verified locator candidates for a node id.",
     "inputSchema": {"type": "object", "properties": {"node_id": {"type": "integer"}, "strip_names": {"type": "boolean"}},
                     "required": ["node_id"]}},
    {"name": "spy_describe", "description": "Properties + ancestor breadcrumb for a node id.",
     "inputSchema": {"type": "object", "properties": {"node_id": {"type": "integer"}}, "required": ["node_id"]}},
    {"name": "spy_pick", "description": "Deepest widget at screen point x,y + ancestor path.",
     "inputSchema": {"type": "object", "properties": {"x": {"type": "integer"}, "y": {"type": "integer"}},
                     "required": ["x", "y"]}},
    {"name": "spy_schema", "description": "Locator grammar cheatsheet + the candidate contract.",
     "inputSchema": {"type": "object", "properties": {}}},
]


def _call(core: SpyCore, name: str, a: dict):
    if name == "spy_dump_tree":
        return core.dump_tree(visible_only=not a.get("all", False))
    if name == "spy_find":
        return core.find(a["locator"])
    if name == "spy_validate":
        return core.validate(a["locator"], expect_id=a.get("expect_id"))
    if name == "spy_suggest":
        return core.suggest(int(a["node_id"]), strip_names=a.get("strip_names", False))
    if name == "spy_describe":
        return core.describe(int(a["node_id"]))
    if name == "spy_pick":
        return core.hit_test(int(a["x"]), int(a["y"]))
    if name == "spy_schema":
        from .cli import SCHEMA
        return SCHEMA
    raise SpyError(f"unknown tool {name}")


def serve(core: SpyCore, inp=sys.stdin, out=sys.stdout) -> None:
    def send(msg):
        out.write(json.dumps(msg) + "\n")
        out.flush()

    for line in inp:
        line = line.strip()
        if not line:
            continue
        try:
            req = json.loads(line)
        except Exception:
            continue
        mid, method, params = req.get("id"), req.get("method"), req.get("params") or {}
        if method == "initialize":
            send({"jsonrpc": "2.0", "id": mid, "result": {
                "protocolVersion": params.get("protocolVersion", "2024-11-05"),
                "capabilities": {"tools": {}},
                "serverInfo": {"name": "javagui-spy", "version": "0.7.0"}}})
        elif method in ("notifications/initialized", "initialized"):
            continue
        elif method == "tools/list":
            send({"jsonrpc": "2.0", "id": mid, "result": {"tools": _TOOLS}})
        elif method == "tools/call":
            try:
                result = _call(core, params.get("name"), params.get("arguments") or {})
                send({"jsonrpc": "2.0", "id": mid, "result": {
                    "content": [{"type": "text", "text": json.dumps(result, indent=2)}]}})
            except Exception as e:
                send({"jsonrpc": "2.0", "id": mid, "result": {
                    "content": [{"type": "text", "text": f"ERROR: {type(e).__name__}: {e}"}], "isError": True}})
        elif mid is not None:
            send({"jsonrpc": "2.0", "id": mid, "error": {"code": -32601, "message": f"method not found: {method}"}})


def run(toolkit="swing", host="localhost", port=None, timeout=30, launch=None) -> None:
    core = SpyCore(toolkit=toolkit)
    if launch:
        core.launch(launch, port=port)
    else:
        core.connect(host=host, port=port, timeout=timeout)
    try:
        serve(core)
    finally:
        core.disconnect()
