"""Live probe of the SWT/RCP javagui-spy RPCs (hitTest / highlight / getUiGeneration)
against DBeaver. Run inside the harness container after DBeaver + agent are up on :5682."""
import sys, json, socket, time
sys.path.insert(0, "/work/python")


def rpc(method, params=None, port=5682):
    s = socket.create_connection(("127.0.0.1", port), timeout=10)
    s.sendall((json.dumps({"jsonrpc": "2.0", "id": 1, "method": method, "params": params or {}}) + "\n").encode())
    dec = json.JSONDecoder(); buf = b""
    while True:
        ch = s.recv(65536)
        if not ch:
            break
        buf += ch
        try:
            obj, _ = dec.raw_decode(buf.decode("utf-8", "ignore").lstrip())
            s.close(); return obj
        except json.JSONDecodeError:
            continue
    s.close(); return {}


ok = True
# getUiGeneration (easy path)
g = rpc("getUiGeneration").get("result", {})
gen = g.get("generation")
print(f"[swt getUiGeneration] generation={gen} stable={gen == rpc('getUiGeneration').get('result', {}).get('generation')}")
ok &= gen is not None

# hitTest at the center of the DBeaver window (1600x1000 xvfb -> ~ workbench area)
for (x, y) in [(150, 140), (800, 400), (100, 300)]:
    h = rpc("hitTest", {"x": x, "y": y}).get("result", {})
    print(f"[swt hitTest {x},{y}] hit={h.get('hit')} type={h.get('type')} name={h.get('name')} path_len={len(h.get('ancestor_path') or [])}")
    if h.get("hit"):
        # highlight the picked widget
        hl = rpc("highlight", {"componentId": h.get("id"), "durationMs": 300}).get("result", {})
        print(f"[swt highlight id={h.get('id')}] ok={hl.get('ok')}")
        break

# armPick timeout path (no click -> graceful timeout; validates the SWT reflection listener install/remove)
t0 = time.time()
ap = rpc("armPick", {"timeoutMs": 2000}).get("result", {})
print(f"[swt armPick] returned in {time.time()-t0:.1f}s hit={ap.get('hit')} timeout={ap.get('timeout')} err={ap.get('error')}")
ok &= (ap.get("hit") is False)

print("=== swt spy probe done ===")
sys.exit(0 if ok else 1)
