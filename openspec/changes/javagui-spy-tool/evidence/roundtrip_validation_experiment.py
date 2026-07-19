#!/usr/bin/env python3
"""Spy-tool locator-generation VALIDATION (explore-mode experiment).
Generate a locator per node, round-trip through the PRODUCTION find_elements matcher,
verify EXACTLY ONE match whose bounds == the target. Two modes: full, and names stripped
(simulates off-the-shelf no-name deep trees — the hard case)."""
import sys, time, subprocess, json, signal
REPO="/home/many/workspace/robotframework-javaui"; sys.path.insert(0, f"{REPO}/python")
proc=subprocess.Popen(["java", f"-javaagent:{REPO}/python/JavaGui/jars/javagui-agent.jar=port=5678",
  "-jar", f"{REPO}/tests/apps/swing/target/swing-test-app-1.0.0.jar"],
  stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
time.sleep(6)
import JavaGui; lib=JavaGui.Swing(); lib.connect_to_application(host="localhost", port=5678, timeout=30); time.sleep(1)
data=json.loads(lib.get_ui_tree(format="json"))
root=data["roots"][0]

# ---- nested accessors ----
def typ(n): return (n.get("component_type") or {}).get("simple_name") or "Component"
def nm(n):  return (n.get("identity") or {}).get("name")
def txt(n):
    idy=n.get("identity") or {}; t=idy.get("text") or idy.get("title")
    if not t: return None
    return " ".join(t.replace("&","").split()) or None
def tip(n): return (n.get("identity") or {}).get("tooltip")
def bnds(n):
    g=n.get("geometry") or {}
    try: return (int(g["x"]),int(g["y"]),int(g["width"]),int(g["height"]))
    except Exception: return None
def kids(n): return n.get("children") or []

# ---- flatten with ancestor chains + type index ----
nodes=[]
def walk(n,parent,anc,dep):
    sibs=kids(parent) if parent else [n]
    same=[c for c in sibs if typ(c)==typ(n)]
    rec={"n":n,"anc":anc,"depth":dep,
         "tix":(same.index(n) if n in same else 0)}
    nodes.append(rec)
    for c in kids(n): walk(c,n,anc+[rec],dep+1)
walk(root,None,[],0)
ALL=[r["n"] for r in nodes]

# ---- offline uniqueness (candidate picking) ----
def has(m,attr,val):
    if attr=="name": return nm(m)==val
    if attr=="text": return txt(m)==val
    if attr=="tooltip": return tip(m)==val
    return False
def gcount(T,attr,val): return sum(1 for m in ALL if typ(m)==T and has(m,attr,val))

# ---- candidate generation: ordered list, VERIFIED LIVE (design: parity by construction) ----
def quals(n,strip):
    qs=[]
    if not strip and nm(n): qs.append(("name",nm(n)))
    if txt(n): qs.append(("text",txt(n)))
    if tip(n): qs.append(("tooltip",tip(n)))
    return qs
def candidates(rec,strip):
    """ranked (locator, tier) list; caller verifies each live and takes the first that passes."""
    n=rec["n"]; T=typ(n); out=[]
    # tier P1 — global single segment
    for a,v in quals(n,strip): out.append((f"{T}[{a}='{v}']","P1-single"))
    # tier P2 — nearest-stable-ancestor anchored >> chain (nearest first)
    tqs=[f"[{a}='{v}']" for a,v in quals(n,strip)] or []
    for a in reversed(rec["anc"]):
        an=a["n"]; AT=typ(an)
        for aa,av in quals(an,strip):
            anchor=f"{AT}[{aa}='{av}']"
            for tq in tqs: out.append((f"{anchor} >> {T}{tq}","P2-anchored"))
            out.append((f"{anchor} >> {T}","P2-anchored-bare"))          # anchor + type only
            out.append((f"{anchor} >> {T}:nth-of-type({rec['tix']+1})","P2-anchored-nth"))
        if any(quals(an,strip)): break   # stop at nearest stable anchor
    # tier P4 — geometry last resort
    b=bnds(n)
    if b: out.append((f"{T}[width='{b[2]}'][height='{b[3]}'][x='{b[0]}'][y='{b[1]}']","P4-geometry"))
    return out

# ---- live round-trip ----
def ebounds(e):
    try:
        b=e.bounds
        if isinstance(b,(list,tuple)) and len(b)==4: return tuple(int(x) for x in b)
    except Exception: pass
    try:
        d=e.to_dict(); return (int(d["x"]),int(d["y"]),int(d["width"]),int(d["height"]))
    except Exception: return None
def ehash(e):
    for a in ("hash_code","id"):
        try:
            v=getattr(e,a);  v=v() if callable(v) else v
            if isinstance(v,int): return v
        except Exception: pass
    try: return int(e.to_dict().get("hash_code"))
    except Exception: return None
def check(loc,target):
    tid=(target.get("id") or {}).get("hash_code")
    try: f=lib.find_elements(loc)
    except Exception as ex: return "error"
    if len(f)!=1: return f"count={len(f)}"
    return "OK" if (tid is not None and ehash(f[0])==tid) else "wrong-node"

def run(strip):
    S={"tot":0,"ok":0,"geom":0,"nn":0,"nnok":0,"tier":{},"lens":[],"fail":[]}
    for r in nodes:
        if r["depth"]<1: continue
        S["tot"]+=1
        noname = strip or not nm(r["n"])
        if noname: S["nn"]+=1
        won=None
        for loc,tier in candidates(r,strip):
            if check(loc,r["n"])=="OK": won=(loc,tier); break
        if won:
            loc,tier=won; S["ok"]+=1; S["lens"].append(len(loc))
            if tier=="P4-geometry": S["geom"]+=1
            if noname: S["nnok"]+=1
            S["tier"][tier]=S["tier"].get(tier,0)+1
        elif len(S["fail"])<8:
            S["fail"].append((typ(r["n"]),r["depth"],"no working locator"))
    return S

print(f"\n=== Swing tree: {len(nodes)} nodes, max depth {max(r['depth'] for r in nodes)}, "
      f"named={sum(1 for n in ALL if nm(n))}/{len(ALL)} ===")
for label,strip in (("FULL (all attributes)",False),("NO-NAME (names stripped = off-the-shelf sim)",True)):
    S=run(strip); tot=S["tot"]
    med=sorted(S["lens"])[len(S["lens"])//2] if S["lens"] else 0
    print(f"\n### {label}")
    print(f"  got a WORKING unique locator : {S['ok']}/{tot} = {100*S['ok']/max(tot,1):.1f}%")
    print(f"  no-name / deep nodes covered : {S['nnok']}/{S['nn']} = {100*S['nnok']/max(S['nn'],1):.1f}%")
    print(f"  geometry fallback share      : {S['geom']}/{tot} = {100*S['geom']/max(tot,1):.1f}%  (KPI: keep low)")
    print(f"  median locator length        : {med} chars")
    print(f"  winning tier                 : " + ", ".join(f"{t}:{c}" for t,c in sorted(S['tier'].items())))
    for f in S["fail"]: print("     no-locator:", f)
lib.disconnect(); proc.send_signal(signal.SIGTERM); print("\n=== done ===")
