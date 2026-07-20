import os, subprocess, sys, time, glob, http.server, threading, functools
IMG="/tmp/attach_exp/icedtea-web-image"; ROOT="/tmp/attach_exp/jnlp_root"; HTTP_PORT=8099
CP=f"{IMG}/share/icedtea-web/javaws.jar:{IMG}/linux-deps-runtime/*:{IMG}/share/icedtea-web/jsobject.jar"
def serve():
    h=functools.partial(http.server.SimpleHTTPRequestHandler, directory=ROOT)
    http.server.HTTPServer(("127.0.0.1",HTTP_PORT), h).serve_forever()
threading.Thread(target=serve, daemon=True).start(); time.sleep(1)
def cmdline(pid):
    try: return open(f"/proc/{pid}/cmdline","rb").read().replace(b"\0",b" ").decode("utf-8","ignore").strip()
    except OSError: return None
def java_pids():
    o={}
    for p in glob.glob("/proc/[0-9]*"):
        pid=p.split("/")[-1]; cl=cmdline(pid)
        if cl and ("java" in cl.split(" ")[0]): o[int(pid)]=cl
    return o
before=set(java_pids())
boot=subprocess.Popen(["java", f"@{IMG}/bin/itw-modularjdk.args","-cp",CP,
                       "net.sourceforge.jnlp.runtime.Boot", f"http://127.0.0.1:{HTTP_PORT}/app.jnlp"],
                      stdout=open("/tmp/attach_exp/javaws.out","w"), stderr=subprocess.STDOUT)
print(f"[obs] launcher Boot pid={boot.pid}")
seen=set()
for i in range(30):
    time.sleep(1)
    now=java_pids()
    for pid in sorted(now):
        if pid not in before and pid not in seen:
            seen.add(pid)
            cl=now[pid]
            # mark distinguishing tokens
            marks=[m for m in ["SwingTestApp","-Xnofork","runtime.Boot","app.jar","-javaagent"] if m in cl]
            # SecurityManager token?
            sm = "SecurityManager" if "securitymanager" in cl.lower() else ""
            print(f"[obs] t={i}s NEW java pid={pid} marks={marks} {sm}")
            print(f"        {cl[:260]}")
    # note deaths
    for pid in list(seen):
        if pid not in now:
            print(f"[obs] t={i}s pid={pid} EXITED"); seen.discard(pid)
print("=== final java pids (non-launcher) ===")
for pid,cl in java_pids().items():
    if pid not in before: print(f"  pid={pid}: {cl[:260]}")
print("=== javaws.out tail ===")
print(open("/tmp/attach_exp/javaws.out").read()[-1800:])
for pid in list(seen)+[boot.pid]:
    try: os.kill(pid,15)
    except OSError: pass
