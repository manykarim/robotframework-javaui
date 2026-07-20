import os, subprocess, signal, sys, time, glob, http.server, threading, functools, socket
sys.path.insert(0,"python")
AGENT=os.path.abspath("python/JavaGui/jars/javagui-agent.jar")
SW="tests/apps/swing/target/swing-test-app-1.0.0.jar"
IMG="/tmp/attach_exp/icedtea-web-image"; ROOT="/tmp/attach_exp/jnlp_root"
os.environ["JAVAGUI_JATTACH"]="/tmp/attach_exp/jattach"

# ---------- 0.4 jattach fallback ----------
print("=== SPIKE 0.4: jattach JRE-only fallback ===")
import JavaGui._attach as A
app=subprocess.Popen(["java","-jar",SW],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL); time.sleep(6)
orig=A._jdk_java
A._jdk_java=lambda: None   # simulate JRE-only host -> force jattach path
try:
    port=A.free_port()
    A.inject_agent(app.pid, AGENT, port, toolkit="swing", timeout=25)
    import JavaGui
    lib=JavaGui.Swing(); lib.connect_to_application(host="127.0.0.1",port=port,timeout=15)
    print(f"[0.4] jattach inject OK -> {len(lib.find_elements('JButton'))} buttons  RESULT: PASS")
    lib.disconnect()
except Exception as e:
    print(f"[0.4] RESULT: FAIL {type(e).__name__}: {str(e)[:180]}")
finally:
    A._jdk_java=orig; app.send_signal(signal.SIGTERM)

# ---------- 0.3 forked-child topology ----------
print("=== SPIKE 0.3: forked-child topology (JNLP heap args) ===")
def serve():
    h=functools.partial(http.server.SimpleHTTPRequestHandler, directory=ROOT)
    http.server.HTTPServer(("127.0.0.1",8099),h).serve_forever()
threading.Thread(target=serve,daemon=True).start(); time.sleep(1)
CP=f"{IMG}/share/icedtea-web/javaws.jar:{IMG}/linux-deps-runtime/*:{IMG}/share/icedtea-web/jsobject.jar"
boot=subprocess.Popen(["java",f"@{IMG}/bin/itw-modularjdk.args","-cp",CP,
                       "net.sourceforge.jnlp.runtime.Boot","http://127.0.0.1:8099/app_heap.jnlp"],
                      stdout=open("/tmp/attach_exp/heap.out","w"),stderr=subprocess.STDOUT)
def cl(pid):
    try: return open(f"/proc/{pid}/cmdline","rb").read().replace(b"\0",b" ").decode("utf-8","ignore")
    except OSError: return ""
forked=None
for i in range(30):
    time.sleep(1)
    kids=A._java_children(boot.pid)
    if kids:
        forked=kids[-1]
        print(f"[0.3] FORKED child JVM pid={forked} (parent Boot pid={boot.pid})")
        c=cl(forked)
        print(f"[0.3] child has -Xmx/heap marker: {'-Xmx' in c or 'Xmx192' in c or 'heap' in c.lower()}")
        print(f"[0.3] child cmdline (trunc): {c[:180]}")
        break
    if "Starting application" in open('/tmp/attach_exp/heap.out').read() and i>3 and not kids:
        print(f"[0.3] no forked child after app start -> IN-PROCESS even with heap args (app pid==boot {boot.pid})"); break
print("[0.3] RESULT:", "PASS forked-child detected+discriminated" if forked else "IN-PROCESS (ITW ran heap app in-process)")
try: boot.terminate()
except Exception: pass
