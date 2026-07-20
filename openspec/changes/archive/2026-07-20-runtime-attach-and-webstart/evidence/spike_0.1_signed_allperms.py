import os, sys, time, threading, functools, http.server, subprocess, socket
sys.path.insert(0,"python")
IMG="/tmp/attach_exp/icedtea-web-image"; ROOT="/tmp/attach_exp/jnlp_signed"; HTTP=8097; PORT=5713
AGENT=os.path.abspath("python/JavaGui/jars/javagui-agent.jar")
CP=f"{IMG}/share/icedtea-web/javaws.jar:{IMG}/linux-deps-runtime/*:{IMG}/share/icedtea-web/jsobject.jar"
def serve():
    h=functools.partial(http.server.SimpleHTTPRequestHandler, directory=ROOT)
    http.server.HTTPServer(("127.0.0.1",HTTP), h).serve_forever()
threading.Thread(target=serve,daemon=True).start(); time.sleep(1)
# launch ITW Boot with -Xtrustall so the self-signed cert is accepted headless
boot=subprocess.Popen(["java",f"@{IMG}/bin/itw-modularjdk.args","-cp",CP,
                       "net.sourceforge.jnlp.runtime.Boot","-Xtrustall",
                       f"http://127.0.0.1:{HTTP}/app.jnlp"],
                      stdout=open("/tmp/attach_exp/signed.out","w"),stderr=subprocess.STDOUT)
print(f"[spike] launched signed all-perms JNLP via ITW pid={boot.pid}")
# wait for app to start
for _ in range(40):
    time.sleep(1)
    if "Starting application" in open("/tmp/attach_exp/signed.out").read(): break
time.sleep(3)
from JavaGui import _attach
try:
    _attach.inject_agent(boot.pid, AGENT, PORT, toolkit="auto", timeout=25)
    import JavaGui
    lib=JavaGui.Swing(); lib.connect_to_application(host="127.0.0.1",port=PORT,timeout=15)
    print(f"[spike] ALL-PERMISSIONS ATTACH SUCCEEDED: buttons={len(lib.find_elements('JButton'))}")
    lib.disconnect(); print("RESULT: PASS attach works under all-permissions signed JNLP")
except Exception as e:
    print(f"[spike] attach raised: {type(e).__name__}: {str(e)[:200]}")
    # show whether SM still recursed
    log=open("/tmp/attach_exp/signed.out").read()
    print("[spike] setSecurityManager present:", "setSecurityManager" in log, "| all-perms granted:", "all-permissions" in log.lower() or "AllPermission" in log)
    print("RESULT: FAIL (attach blocked even under all-permissions)")
finally:
    try: boot.terminate()
    except Exception: pass
