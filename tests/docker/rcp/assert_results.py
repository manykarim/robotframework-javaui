#!/usr/bin/env python3
"""Fail (exit 1) if the harness Robot run had any failed test, or ran none.

The harness entrypoint always exits 0 ("success == we collected evidence"), so CI
uses this to turn the collected results into a real pass/fail gate.
"""
import glob
import sys
import xml.etree.ElementTree as ET

paths = sorted(glob.glob("results/dbeaver/output.xml"))
if not paths:
    sys.exit("no Robot output produced — the harness did not run the suite")

root = ET.parse(paths[0]).getroot()
fails = [t.get("name") for t in root.iter("test") if t.find("status").get("status") == "FAIL"]
npass = sum(1 for t in root.iter("test") if t.find("status").get("status") == "PASS")
print(f"real RCP suite: {npass} passed, {len(fails)} failed" + (f" -> {fails}" if fails else ""))

if not npass:
    sys.exit("no tests ran against real DBeaver")
if fails:
    sys.exit(1)
