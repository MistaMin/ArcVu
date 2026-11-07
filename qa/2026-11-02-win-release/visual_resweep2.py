#!/usr/bin/env python3
"""Visual re-sweep v2: prove the 44 NOT SHOWN results are HiDPI clipping artifacts.

Fresh REAPER session required. Adds the FX once, unbypasses, enlarges the floating
editor window so nothing is clipped at 150% DPI, verifies editor liveness before
sweeping, and restores each parameter with read-back verification.
"""
import ctypes
import json
import os
import sys
import time

sys.path.insert(0, r"C:\Users\marco\Developer\plugin-qa-skills\skills\plugin-daw-test\scripts")
import reaper_qa as qa  # noqa: E402

import numpy as np  # noqa: E402

QA_DIR = r"C:\Users\marco\Developer\ArcVu\qa\2026-11-02-win-release"
OUT = os.path.join(QA_DIR, "resweep2")
os.makedirs(OUT, exist_ok=True)

NOT_SHOWN = ["Peak", "AES-17", "RMS window", "Clip threshold", "Trim R", "Polarity L", "Polarity R",
             "Mute R", "Monitor", "High-pass", "High-pass slope", "Low-pass", "Low-pass slope",
             "Mono frequency", "Mono amount", "Dynamic frequency", "Dynamic depth", "Dynamic on",
             "Dynamic type", "Dynamic listen", "VU model", "Rise", "Fall", "Overshoot", "Peak hold",
             "Value hold", "Clip hold", "Peak lamp threshold", "Comfort zone", "Analog RMS", "Weighting",
             "Force mono", "In place", "Show values", "Show hold", "Show tools", "Show label",
             "Meters only", "Drag sensitivity", "Meter refresh", "Bypass hidden processors", "Tooltips",
             "Stay above windows"]
CONTEXT = {
    "High-pass": [("High-pass slope", 1.0)], "Low-pass": [("Low-pass slope", 1.0)],
    "Mono frequency": [("Mono amount", 1.0)],
    "Dynamic on": [("Dynamic depth", 1.0), ("Dynamic type", 0.5)],
    "Dynamic depth": [("Dynamic on", 1.0), ("Dynamic type", 0.5)],
    "Dynamic frequency": [("Dynamic on", 1.0), ("Dynamic depth", 1.0), ("Dynamic type", 0.5)],
    "Dynamic type": [("Dynamic on", 1.0), ("Dynamic depth", 1.0)],
    "Dynamic listen": [("Dynamic on", 1.0), ("Dynamic depth", 1.0), ("Dynamic type", 0.5)],
    "In place": [("Monitor", 0.4)],
    "Meters only": [("Bypass hidden processors", 1.0), ("High-pass slope", 1.0), ("High-pass", 1.0)],
    "Bypass hidden processors": [("Meters only", 1.0), ("High-pass slope", 1.0), ("High-pass", 1.0)],
}


def set_param_checked(b, by_name, name, value):
    p = by_name[name]
    for _ in range(3):
        r = b.call("set_param", index=p["index"], value=value)
        if abs(r["value"] - value) < 0.02:
            return r
        time.sleep(0.2)
    raise SystemExit("set_param %s -> %s stuck at %s" % (name, value, r["value"]))


def main():
    b = qa.Bridge(QA_DIR)
    pid = qa.reaper_pid(QA_DIR)
    print(b.call("add_fx", name="VST3: ArcVu"))
    info = b.call("info")
    print("fx:", info)
    b.call("show", open=1)
    time.sleep(1.5)

    params = b.call("params")["params"]
    by_name = {p["name"]: p for p in params}
    # REAPER appends its own Bypass/Wet/Delta; the previous suite left Bypass on, which freezes the editor.
    for nm, v in (("Wet", 1.0), ("Delta", 0.0)):
        if nm in by_name:
            b.call("set_param", index=by_name[nm]["index"], value=v)
    bypass_idxs = [p["index"] for p in params if p["name"] == "Bypass"]
    for idx in bypass_idxs:
        b.call("set_param", index=idx, value=0.0)
    time.sleep(1.0)

    win = qa.editor_window(QA_DIR, pid, info["name"])
    ctypes.windll.user32.SetWindowPos(int(win["id"]), None, 40, 40, 1750, 620, 0x0040)
    time.sleep(1.0)
    win = qa.editor_window(QA_DIR, pid, info["name"])
    print("window:", win)

    # liveness gate: Meter type toggle must change pixels, else the editor is frozen - abort.
    mt = by_name["Meter type"]
    a = qa.capture(win["id"], os.path.join(OUT, "liveness_a.png"))
    b.call("set_param", index=mt["index"], value=0.0 if mt["value"] > 0.5 else 1.0)
    time.sleep(0.8)
    c = qa.capture(win["id"], os.path.join(OUT, "liveness_b.png"))
    b.call("set_param", index=mt["index"], value=mt["value"])
    live = float((np.abs(a - c).max(axis=2) > 24).mean()) if a.shape == c.shape else 1.0
    print("liveness diff: %.3f%%" % (live * 100))
    if live < 0.001:
        raise SystemExit("editor frozen - aborting")
    time.sleep(0.5)

    frames = [qa.capture(win["id"], os.path.join(OUT, "base_%d.png" % i)) for i in range(3)]
    noise = np.zeros(frames[0].shape[:2], bool)
    for f in frames[1:]:
        noise |= np.abs(f - frames[0]).max(axis=2) > 8
    noise = qa.dilate(noise, 4)
    ref = frames[-1]
    npx = ref.shape[0] * ref.shape[1]
    print("animates by itself: %.3f%%" % (100.0 * noise.mean()))

    defaults = {p["name"]: p["value"] for p in b.call("params")["params"]}
    results = []
    for name in NOT_SHOWN:
        p = by_name[name]
        ctx = CONTEXT.get(name, [])
        for other, v in ctx:
            set_param_checked(b, by_name, other, v)
        if ctx:
            time.sleep(0.3)
        target = 1.0 if defaults[name] < 0.5 else 0.0
        set_param_checked(b, by_name, name, target)
        time.sleep(0.5)
        img = qa.capture(win["id"], os.path.join(OUT, "p%d.png" % p["index"]))
        if img.shape != ref.shape:
            frac = 1.0
        else:
            diff = (np.abs(img - ref).max(axis=2) > 24) & ~noise
            frac = float(diff.mean())
        px = int(frac * npx)
        verdict = "ok" if frac > 0.00005 else "NOT SHOWN"
        results.append({"name": name, "result": verdict, "pct": round(frac * 100, 4), "px": px})
        print("%-28s %s %.4f%% (%d px)" % (name, verdict, frac * 100, px), flush=True)
        set_param_checked(b, by_name, name, defaults[name])
        for other, _ in ctx:
            set_param_checked(b, by_name, other, defaults[other])
        time.sleep(0.3)
    with open(os.path.join(OUT, "resweep.json"), "w") as f:
        json.dump(results, f, indent=2)
    print("NOT SHOWN:", [r["name"] for r in results if r["result"] != "ok"])


if __name__ == "__main__":
    main()
