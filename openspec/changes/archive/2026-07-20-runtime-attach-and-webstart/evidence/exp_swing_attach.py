import os, subprocess, signal, socket, sys, time
sys.path.insert(0, "python")
REPO = os.getcwd()
APP  = os.path.join(REPO, "tests/apps/swing/target/swing-test-app-1.0.0.jar")
AGENT= os.path.join(REPO, "python/JavaGui/jars/javagui-agent.jar")
PORT = 5695

def port_up(p):
    try:
        with socket.create_connection(("127.0.0.1", p), timeout=1): return True
    except OSError: return False

# 1. Launch the Swing app with NO -javaagent (a "found in the wild" JVM)
app = subprocess.Popen(["java", "-jar", APP], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
print(f"[exp] launched plain Swing app (NO agent), pid={app.pid}")
time.sleep(6)
print(f"[exp] port {PORT} before attach: up={port_up(PORT)}  (expect False)")

# 2. Dynamically attach the agent by PID (agentmain path)
t0 = time.time()
r = subprocess.run(["java", "--add-modules", "jdk.attach", "-cp", "/tmp/attach_exp",
                    "Attacher", str(app.pid), AGENT, f"port={PORT},toolkit=swing"],
                   capture_output=True, text=True, timeout=60)
print(r.stdout.strip()); print(r.stderr.strip()[:400])

# 3. Wait for the RPC port to open post-attach
for _ in range(20):
    if port_up(PORT): break
    time.sleep(0.5)
dt = time.time() - t0
print(f"[exp] port {PORT} after attach: up={port_up(PORT)}  (attach+boot took {dt:.1f}s)")

# 4. Connect via the real library and introspect
ok = False
try:
    import JavaGui
    lib = JavaGui.Swing()
    lib.connect_to_application(host="127.0.0.1", port=PORT, timeout=20)
    tree = lib.get_ui_tree(format="json")
    import json; roots = json.loads(tree).get("roots", [])
    def count(n): return 1 + sum(count(c) for c in (n.get("children") or []))
    total = sum(count(r) for r in roots)
    print(f"[exp] CONNECTED post-attach; get_ui_tree -> {total} nodes across {len(roots)} roots")
    els = lib.find_elements("JButton")
    print(f"[exp] find_elements('JButton') -> {len(els)} buttons")
    ok = total > 0 and len(els) > 0
    lib.disconnect()
except Exception as e:
    print(f"[exp] connect/introspect FAILED: {type(e).__name__}: {e}")
finally:
    app.send_signal(signal.SIGTERM)
print("RESULT:", "PASS runtime-attach works" if ok else "FAIL")
sys.exit(0 if ok else 1)
