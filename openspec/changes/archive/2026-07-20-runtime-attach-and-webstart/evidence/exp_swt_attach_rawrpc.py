import os, subprocess, signal, socket, sys, time, json
APP="tests/apps/swt/target/swt-test-app-1.0.0-all.jar"
AGENT="python/JavaGui/jars/javagui-agent.jar"
PORT=5698
def rpc(method,params=None):
    s=socket.create_connection(("127.0.0.1",PORT),timeout=10)
    s.sendall((json.dumps({"jsonrpc":"2.0","id":1,"method":method,"params":params or {}})+"\n").encode())
    dec=json.JSONDecoder(); buf=b""
    while True:
        ch=s.recv(65536)
        if not ch: break
        buf+=ch
        try: o,_=dec.raw_decode(buf.decode("utf-8","ignore").lstrip()); s.close(); return o
        except json.JSONDecodeError: continue
    s.close(); return {}
def up(p):
    try:
        with socket.create_connection(("127.0.0.1",p),timeout=1): return True
    except OSError: return False
app=subprocess.Popen(["java","-jar",APP],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
print(f"[exp] SWT app pid={app.pid}"); time.sleep(7)
subprocess.run(["java","--add-modules","jdk.attach","-cp","/tmp/attach_exp","Attacher",
                str(app.pid),AGENT,f"port={PORT},toolkit=auto"],capture_output=True,text=True,timeout=60)
for _ in range(30):
    if up(PORT): break
    time.sleep(0.5)
try:
    for m in ("getUiGeneration","getComponentTree","getWorkbenchInfo"):
        r=rpc(m)
        res=r.get("result"); err=r.get("error")
        if isinstance(res,(dict,list)):
            n=len(json.dumps(res))
            print(f"[rpc {m}] ok, {n} bytes; keys={list(res)[:6] if isinstance(res,dict) else 'list len '+str(len(res))}")
        else:
            print(f"[rpc {m}] result={res} err={err}")
finally:
    app.send_signal(signal.SIGTERM)
