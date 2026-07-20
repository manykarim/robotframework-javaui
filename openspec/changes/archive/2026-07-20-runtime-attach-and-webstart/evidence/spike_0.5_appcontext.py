import os, subprocess, signal, sys, time, socket, json
sys.path.insert(0,"python")
AGENT=os.path.abspath("python/JavaGui/jars/javagui-agent.jar")
PORT=5721
# launch the multi-AppContext app WITHOUT agent (needs the add-exports for its own reflection)
app=subprocess.Popen(["java","--add-exports","java.desktop/sun.awt=ALL-UNNAMED","-cp","/tmp/attach_exp","MultiCtx"],
                     stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
print(f"[0.5] multi-AppContext app pid={app.pid}"); time.sleep(6)
subprocess.run(["java","--add-modules","jdk.attach","-cp","/tmp/attach_exp","Attacher",str(app.pid),AGENT,f"port={PORT},toolkit=swing"],capture_output=True,timeout=60)
for _ in range(20):
    try: socket.create_connection(("127.0.0.1",PORT),timeout=1).close(); break
    except OSError: time.sleep(0.5)
try:
    import JavaGui
    lib=JavaGui.Swing(); lib.connect_to_application(host="127.0.0.1",port=PORT,timeout=15)
    tree=json.loads(lib.get_ui_tree(format="json")); roots=tree.get("roots",[])
    # Which frames/windows are visible?
    titles=[]
    def walk(n):
        t=(n.get("identity") or {}).get("title") or (n.get("identity") or {}).get("name")
        ct=(n.get("component_type") or {}).get("simple_name")
        if ct in ("JFrame","Frame"): titles.append(t)
        for c in (n.get("children") or []): walk(c)
    for r in roots: walk(r)
    frames=lib.find_elements("JFrame")
    print(f"[0.5] roots={len(roots)} JFrame elements={len(frames)} frame_titles={titles}")
    main_seen = any("Main" in (t or "") for t in titles)
    second_seen = any("Second" in (t or "") for t in titles)
    print(f"[0.5] main-context frame seen={main_seen}  second-context frame seen={second_seen}")
    if main_seen and second_seen:
        print("[0.5] RESULT: BOTH contexts visible -> agent already sees all AppContexts; task 5.1 NOT needed")
    elif main_seen and not second_seen:
        print("[0.5] RESULT: ONLY caller context visible -> 5.1 AppContext-scoped discovery IS needed")
    else:
        print("[0.5] RESULT: inconclusive")
    lib.disconnect()
except Exception as e:
    import traceback; traceback.print_exc()
finally:
    app.send_signal(signal.SIGTERM)
