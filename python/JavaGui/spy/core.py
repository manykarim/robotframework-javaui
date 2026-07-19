"""SpyCore — the single shared engine behind the javagui-spy CLI (and future GUI/MCP).

Owns ONE library connection to a running instrumented Java app, a cached widget tree, and the
``resolve`` oracle (production ``find_elements``). All surfaces are thin clients of this contract.
"""
from __future__ import annotations
import json
import subprocess
import time
from pathlib import Path

from . import generator as G


class SpyError(Exception):
    pass


class SpyCore:
    def __init__(self, toolkit: str = "swing"):
        import JavaGui
        self.toolkit = (toolkit or "swing").lower()
        if self.toolkit == "swing":
            self.lib = JavaGui.Swing()
        elif self.toolkit == "swt":
            self.lib = JavaGui.Swt()
        elif self.toolkit == "rcp":
            self.lib = JavaGui.Rcp()
        else:
            raise SpyError(f"unknown toolkit '{toolkit}' (use swing|swt|rcp)")
        self._proc: subprocess.Popen | None = None
        self._flat: list[dict] = []
        self._by_id: dict[int, dict] = {}
        self._tree_ts: int = 0

    # ---- lifecycle ------------------------------------------------------
    def connect(self, host: str = "localhost", port: int | None = None, timeout: float = 30) -> None:
        port = port or (5678 if self.toolkit == "swing" else 5679)
        if self.toolkit == "swing":
            self.lib.connect_to_application(host=host, port=port, timeout=timeout)
        else:
            self.lib.connect_to_swt_application(self.toolkit, host, port, timeout)
        self.refresh()

    def launch(self, jar: str, port: int | None = None, agent_jar: str | None = None,
               wait: float = 6.0) -> None:
        port = port or (5678 if self.toolkit == "swing" else 5679)
        agent_jar = agent_jar or str(Path(__file__).resolve().parents[1] / "jars" / "javagui-agent.jar")
        tk = "" if self.toolkit == "swing" else f",toolkit={self.toolkit}"
        self._proc = subprocess.Popen(
            ["java", f"-javaagent:{agent_jar}=port={port}{tk}", "-jar", jar],
            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
        )
        time.sleep(wait)
        self.connect(port=port)

    def disconnect(self) -> None:
        try:
            self.lib.disconnect()
        except Exception:
            pass
        if self._proc:
            try:
                self._proc.terminate()
            except Exception:
                pass

    # ---- tree + oracle --------------------------------------------------
    def refresh(self) -> None:
        raw = self.lib.get_ui_tree(format="json")
        tree = json.loads(raw)
        self._flat = G.flatten_forest(tree)
        self._by_id = {r["node_id"]: r for r in self._flat if r["node_id"] is not None}
        self._tree_ts = int(time.time() * 1000)

    def resolve(self, locator: str) -> list[int]:
        """Live uniqueness oracle: matched node ids via the production matcher."""
        ids: list[int] = []
        for e in self.lib.find_elements(locator):
            h = getattr(e, "hash_code", None)
            if h is None:
                try:
                    h = e.to_dict().get("hash_code")
                except Exception:
                    h = None
            if h is not None:
                ids.append(int(h))
        return ids

    def node_by_id(self, node_id: int) -> dict:
        rec = self._by_id.get(int(node_id))
        if rec is None:
            raise SpyError(f"NODE_GONE: node id {node_id} not in current tree; re-run dump-tree")
        return rec

    # ---- read verbs -----------------------------------------------------
    @staticmethod
    def _summary(rec: dict) -> dict:
        n = rec["node"]
        g = G.node_geometry(n) or (None, None, None, None)
        return {
            "node_id": rec["node_id"], "type": G.node_type(n), "name": G.node_name(n),
            "text": G.node_text(n), "depth": rec["depth"],
            "bounds": {"x": g[0], "y": g[1], "w": g[2], "h": g[3]},
        }

    def dump_tree(self, visible_only: bool = True) -> list[dict]:
        rows = []
        for r in self._flat:
            st = r["node"].get("state") or {}
            if visible_only and st.get("visible") is False:
                continue
            rows.append(self._summary(r))
        return rows

    def find(self, locator: str) -> dict:
        ids = self.resolve(locator)
        recs = [self._by_id[i] for i in ids if i in self._by_id]
        return {"locator": locator, "match_count": len(ids),
                "matches": [self._summary(r) for r in recs]}

    def validate(self, locator: str, expect_id: int | None = None) -> dict:
        ids = self.resolve(locator)
        unique = len(ids) == 1
        ok = unique and (expect_id is None or ids[0] == int(expect_id))
        recs = [self._by_id[i] for i in ids if i in self._by_id]
        return {"locator": locator, "match_count": len(ids), "unique": unique,
                "matches_expected": ok if expect_id is not None else None,
                "matches": [self._summary(r) for r in recs]}

    def describe(self, node_id: int) -> dict:
        rec = self.node_by_id(node_id)
        n = rec["node"]
        ancestors = []
        for a in rec["ancestors"]:
            an = a["node"]
            ancestors.append({"node_id": a["node_id"], "type": G.node_type(an),
                              "name": G.node_name(an), "text": G.node_text(an)})
        return {"target": self._summary(rec),
                "identity": n.get("identity"), "geometry": n.get("geometry"),
                "state": n.get("state"), "component_type": n.get("component_type"),
                "ancestors": ancestors}

    def suggest(self, node_id: int, top: int = 3, strip_names: bool = False) -> dict:
        rec = self.node_by_id(node_id)
        cands = G.suggest(self._flat, rec, self.resolve, strip_names=strip_names, top=top)
        best = cands[0]["locator"] if cands else None
        snippets = {}
        if best:
            snippets = {"click": f"Click    {best}",
                        "get_text": f"${{value}}=    Get Element Text    {best}",
                        "should_be_visible": f"Element Should Be Visible    {best}"}
        return {"target": self._summary(rec), "candidates": cands, "rf_snippets": snippets}

    def screenshot(self, path: str, annotate: str | None = None) -> str:
        return self.lib.capture_screenshot(path)
