#!/usr/bin/env python3
"""Visual re-sweep of the parameters the REAPER suite reported NOT SHOWN.

The suite's capture window (1115x315) clips the ArcVu editor at 150% DPI, so every
control below the top strip showed 0.000%. This driver enlarges the floating editor
window with SetWindowPos, rebuilds the noise mask, and re-runs the same comparison
for the clipped parameters only. Uses the same bridge and capture as reaper_qa.py.
"""
import ctypes
import json
import os
import subprocess
import sys
import time

sys.path.insert(0, r"C:\Users\marco\Developer\plugin-qa-skills\skills\plugin-daw-test\scripts")
import reaper_qa as qa  # noqa: E402

import numpy as np  # noqa: E402
from PIL import Image  # noqa: E402

QA_DIR = r"C:\Users\marco\Developer\ArcVu\qa\2026-11-02-win-release"
OUT = os.path.join(QA_DIR, "resweep")
os.makedirs(OUT, exist_ok=True)

NOT_SHOWN = ["Peak", "AES-17", "RMS window", "Clip threshold", "Trim R", "Polarity L", "Polarity R",
             "Mute R", "Monitor", "High-pass", "High-pass slope", "Low-pass", "Low-pass slope",
             "Mono frequency", "Mono amount", "Dynamic frequency", "Dynamic depth", "Dynamic on",
             "Dynamic type", "Dynamic listen", "VU model", "Rise", "Fall", "Overshoot", "Peak hold",
             "Value hold", "Clip hold", "Peak lamp threshold", "Comfort zone", "Analog RMS", "Weighting",
             "Force mono", "In place", "Show values", "Show hold", "Show tools", "Show label",
             "Meters only", "Drag sensitivity", "Meter refresh", "Bypass hidden processors", "Tooltips",
             "Stay above windows"]
CONTEXT = {  # from qa/2026-10-26-release-qa/exceptions.txt (conditional controls need their context)
    "Trim M": [("Mid side", 1.0)], "Trim S": [("Mid side", 1.0)], "Mid side": [("Trim M", 1.0)],
    "Trim link": [("Trim L", 1.0)], "High-pass": [("High-pass slope", 1.0)],
    "Low-pass": [("Low-pass slope", 1.0)], "Mono frequency": [("Mono amount", 1.0)],
    "Dynamic on": [("Dynamic depth", 1.0), ("Dynamic type", 0.5)],
    "Dynamic depth": [("Dynamic on", 1.0), ("Dynamic type", 0.5)],
    "Dynamic frequency": [("Dynamic on", 1.0), ("Dynamic depth", 1.0), ("Dynamic type", 0.5)],
    "Dynamic type": [("Dynamic on", 1.0), ("Dynamic depth", 1.0)],
    "Dynamic listen": [("Dynamic on", 1.0), ("Dynamic depth", 1.0), ("Dynamic type", 0.5)],
    "In place": [("Monitor", 0.4)],
    "Meters only": [("Bypass hidden processors", 1.0), ("High-pass slope", 1.0), ("High-pass", 1.0)],
    "Bypass hidden processors": [("Meters only", 1.0), ("High-pass slope", 1.0), ("High-pass", 1.0)],
}


def resize_window(win_id, w, h):
    hwnd = ctypes.windll.user32.FindWindowExW(None, None, None, None)  # placeholder
    # winlist_win.py ids are HWND values
    hwnd = int(win_id)
    ctypes.windll.user32.SetWindowPos(hwnd, None, 40, 40, w, h, 0x0040)  # SWP_SHOWWINDOW


def main():
    b = qa.Bridge(QA_DIR)
    pid = qa.reaper_pid(QA_DIR)
    info = b.call("info")
    print("fx:", info)
    b.call("show", open=1)
    time.sleep(1.0)
    win = qa.editor_window(QA_DIR, pid, info["name"])
    print("window before:", win)
    resize_window(win["id"], 1750, 560)
    time.sleep(1.0)
    win = qa.editor_window(QA_DIR, pid, info["name"])
    print("window after:", win)

    # noise mask: three captures of the untouched editor
    frames = [qa.capture(win["id"], os.path.join(OUT, "base_%d.png" % i)) for i in range(3)]
    noise = np.zeros(frames[0].shape[:2], bool)
    for f in frames[1:]:
        noise |= np.abs(f - frames[0]).max(axis=2) > 8
    noise = qa.dilate(noise, 4)
    ref = frames[-1]
    print("animates by itself: %.2f%%" % (100.0 * noise.mean()))

    params = b.call("params")["params"]
    by_name = {p["name"]: p for p in params}
    results = []
    for name in NOT_SHOWN:
        p = by_name.get(name)
        if p is None:
            results.append((name, "MISSING"))
            continue
        ctx = CONTEXT.get(name, [])
        for other, v in ctx:
            b.call("set_param", index=by_name[other]["index"], value=v)
            time.sleep(0.2)
        target = 1.0 if p["value"] < 0.5 else 0.0
        b.call("set_param", index=p["index"], value=target)
        time.sleep(0.5)
        img = qa.capture(win["id"], os.path.join(OUT, "p%d.png" % p["index"]))
        if img.shape != ref.shape:
            changed = 1.0
        else:
            diff = (np.abs(img - ref).max(axis=2) > 24) & ~noise
            changed = float(diff.mean())
        verdict = "ok" if changed > 0.00005 else "NOT SHOWN"
        results.append((name, "%s (%.3f%%)" % (verdict, changed * 100)))
        print("%-28s %s" % (name, results[-1][1]), flush=True)
        b.call("set_param", index=p["index"], value=p["value"])
        for other, _ in ctx:
            b.call("set_param", index=by_name[other]["index"], value=by_name[other]["value"])
        time.sleep(0.3)
    with open(os.path.join(OUT, "resweep.json"), "w") as f:
        json.dump([{"name": n, "result": r} for n, r in results], f, indent=2)
    bad = [n for n, r in results if not r.startswith("ok")]
    print("NOT SHOWN after resweep:", bad)


if __name__ == "__main__":
    main()
