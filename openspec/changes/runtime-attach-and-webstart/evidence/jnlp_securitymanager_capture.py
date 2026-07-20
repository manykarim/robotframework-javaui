import os, subprocess, sys, time, glob, http.server, threading, functools, socket
IMG="/tmp/attach_exp/icedtea-web-image"; ROOT="/tmp/attach_exp/jnlp_root"; HTTP_PORT=8099; AP=5699
AGENT="/home/many/workspace/robotframework-javaui/python/JavaGui/jars/javagui-agent.jar"
CP=f"{IMG}/share/icedtea-web/javaws.jar:{IMG}/linux-deps-runtime/*:{IMG}/share/icedtea-web/jsobject.jar"
def serve():
    h=functools.partial(http.server.SimpleHTTPRequestHandler, directory=ROOT)
    http.server.HTTPServer(("127.0.0.1",HTTP_PORT), h).serve_forever()
threading.Thread(target=serve, daemon=True).start(); time.sleep(1)
OUT="/tmp/attach_exp/javaws2.out"
boot=subprocess.Popen(["java", f"@{IMG}/bin/itw-modularjdk.args","-cp",CP,
                       "net.sourceforge.jnlp.runtime.Boot", f"http://127.0.0.1:{HTTP_PORT}/app.jnlp"],
                      stdout=open(OUT,"w"), stderr=subprocess.STDOUT)
# wait until app started
for _ in range(40):
    time.sleep(1)
    if "Starting application" in open(OUT).read(): break
time.sleep(2)
print(f"[cap] attaching agent to in-process ITW JVM pid={boot.pid}")
r=subprocess.run(["java","--add-modules","jdk.attach","-cp","/tmp/attach_exp","Attacher",
                  str(boot.pid), AGENT, f"port={AP},toolkit=swing"],capture_output=True,text=True,timeout=60)
print("[attacher stdout]", r.stdout.strip())
print("[attacher stderr]", r.stderr.strip()[:500])
time.sleep(2)
print("=== agent / security lines in app JVM stdout ===")
for l in open(OUT):
    if any(k in l for k in ["UnifiedAgent","Security","Permission","Exception","denied","RpcServer","access"]):
        print("   ", l.rstrip()[:220])
try: os.kill(boot.pid,15)
except OSError: pass
