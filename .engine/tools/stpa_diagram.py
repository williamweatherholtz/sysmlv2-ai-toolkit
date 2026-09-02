#!/usr/bin/env python3
"""stpa_diagram.py - render `keel show control-structure` as an STPA control-structure diagram (D0285).

THE CONVENTIONS, set by the human on 2026-09-02 and enforced here by construction, not by taste:
  1. AUTHORITY is the vertical axis, highest at the top; controlled processes sit at the bottom.
  2. CONTROL ACTIONS go DOWN (solid; leave an issuer's bottom-left, enter a process's top-left).
     FEEDBACK goes UP (dashed; leaves a process's top-right, enters a receiver's bottom-right).
  3. Edges are ORTHOLINEAR: vertical / horizontal / vertical, 90 degrees only, never diagonal or curved.
  4. NO TWO SEGMENTS OVERLAP: every edge owns its horizontal channel (a unique y) and its vertical
     trunk and drop (unique x per slot); controllers sit in the gaps between process columns so no
     line passes through a box.
  5. EVERY EDGE IS LABELLED WITH WHAT PASSES along it - the label sits ON the channel and is placed
     where no other edge's vertical runs behind it; a channel too short for its label is extended to
     it as a leader.
  6. A vertical crossing another edge's horizontal HOPS it with a semicircle; a crossing is never
     drawable as a junction.

INPUT: the JSON of `keel show control-structure` (stdin, or --from FILE, or run live with --root DIR).
OUTPUT: an <svg> fragment (stdout or --out FILE) styled through CSS variables --ctl --fb --proc
--ctl-bg --proc-bg --panel --ink --muted, so the host page owns the palette. Nothing here is authored
about a specific project: roles, processes and edges come from the JSON.

Kernel-free, dependency-free (stdlib only). Deterministic: the same JSON draws the same picture.
"""
import json, io, html, re, sys, subprocess
from collections import defaultdict

def load(argv):
    if "--root" in argv:
        root = argv[argv.index("--root") + 1]
        out = subprocess.run(["keel", "show", "control-structure", root], capture_output=True, text=True, encoding="utf-8")
        if out.returncode != 0:
            sys.stderr.write(out.stderr); sys.exit(out.returncode)
        return json.loads(out.stdout)
    if "--from" in argv:
        return json.load(io.open(argv[argv.index("--from") + 1], encoding="utf-8"))
    return json.load(sys.stdin)

def render(d):
    E = html.escape

    # ---------- geometry
    PROCS = ["work", "model", "enforcement-surface", "main-ref", "agent-turn", "deliverable"]
    PW, PH = 200, 58                     # process box
    CW, CH = 180, 58                     # controller box
    COL = 400                            # process column pitch
    X0 = 120
    proc_x = {p: X0 + i * COL for i, p in enumerate(PROCS)}
    gap_x = [X0 + PW + i * COL + (COL - PW - CW) / 2 for i in range(len(PROCS) - 1)]   # controller x in gap i
    W = X0 + (len(PROCS) - 1) * COL + PW + 120

    LEVELS = [["human"], ["remote", "ci", "channel"], ["commit-gate"], ["hooks"], ["console", "agent"]]
    LEVEL_NAMES = ["human authority", "delegated and independent controls", "the local commit gate", "the turn boundary", "the operators: the human's console · the agent"]
    GAP_OF = {"human": 1, "channel": 0, "remote": 3, "ci": 4, "commit-gate": 3, "hooks": 4, "console": 0, "agent": 1}
    LH, CHAR_W = 12, 5.55

    ctl_info = {c["role"]: c for c in d["controllers"]}
    proc_info = {p["role"]: p for p in d["processes"]}

    # ---------- what is passed
    def cmd_name(a):
        m = re.match(r"keel ([a-z-]+)", a["title"]); return m.group(1) if m else None

    ctl_edges = defaultdict(list)
    for a in d["actions"]:
        key = (a["issuedBy"], a["actsOn"]); n = a["name"]
        if n.startswith("cmd"): ctl_edges[key].append(("cmd", cmd_name(a)))
        elif n.startswith("hook"):
            kind = a["title"].split(": ", 1)[1] if ": " in a["title"] else ""
            ctl_edges[key].append(("hook", f"{n[4:]} → {kind}"))
        elif n.startswith("githook"): ctl_edges[key].append(("githook", n[7:] + ": " + a["data"].replace("keel ", "")))
        elif n.startswith("workflow"):
            steps = a["data"].split("steps: ", 1)[1].split("; runs:")[0]
            steps = [s.split(" (")[0].strip() for s in steps.split(" | ")]
            ctl_edges[key].append(("workflow", n[8:] + ": " + ", ".join(steps)))
        else: ctl_edges[key].append(("other", n))

    fb_edges = defaultdict(list)
    for f in d["feedback"]:
        key = (f["sensedFrom"], f["reportsTo"]); n = f["name"]
        if n.startswith("read"):
            m = re.match(r"keel (show )?([a-z-]+)", f["title"])
            fb_edges[key].append(("read", ((m.group(1) or "") + m.group(2)) if m else n))
        elif n.startswith("status"): fb_edges[key].append(("status", n[6:]))
        else: fb_edges[key].append(("other", n))

    def wrap(items, width=46, indent="  "):
        lines, row = [], indent
        for c in items:
            if len(row) + len(c) + 2 > width: lines.append(row.rstrip(", ")); row = indent
            row += c + ", "
        lines.append(row.rstrip(", ")); return lines

    def ctl_label(issuer, proc, frags):
        k = defaultdict(list)
        for kind, v in frags: k[kind].append(v)
        if issuer == "human":
            if proc == "work": return ["DIRECTION, as prose: chat · a Statement recorded verbatim ·", "a direction Decision  (no receiving control parses it)"]
            if proc == "model":
                L = ["ACCEPTANCE {decision, by, date, note, SHA}", "CONFIRMATION + quote receipt · OVERRIDE 'reject <why>'"]
                if "humanDecidesOnChannel" in k.get("other", []): L.append("CHANNEL JUDGMENT: a comment by a declared login")
                return L
        L = []
        if k.get("hook"):
            L.append("VERDICT JSON {block | deny | allow, reason}, per event:"); L += ["  " + v for v in k["hook"]]
        if k.get("githook"):
            L.append("REFUSAL (exit ≠ 0) unless green — what each hook runs:"); L += ["  " + v[:60] for v in k["githook"]]
        if k.get("workflow"):
            L.append("RED / GREEN CHECK on the pushed range — steps:" if issuer == "ci" else "ACCEPTANCE EVENT by delegation (auto-accept, D0207);")
            if issuer != "ci": L.append("accept / reject recorded for a declared login — steps:")
            for v in k["workflow"]:
                name, rest = v.split(": ", 1)
                L += wrap(rest.split(", "), width=58, indent="  " + name + ": ")[:2]
        if k.get("cmd"):
            head = {"model": "AUTHORED FACTS · typed edges · AI-judged verdicts, via:", "main-ref": "COMMITS + PUSHES (merge, never rebase), via:",
                    "enforcement-surface": "UNIT + SURFACE CHANGES (signed Decision required), via:", "work": "A CLAIM on an item, via:"}.get(proc, "WRITES via:")
            L.append(head); L += wrap(k["cmd"])
        for v in k.get("other", []):
            if v == "remoteRefusesRewrite": L.append("REJECTED PUSH: force-push and deletion refused")
            elif v == "consoleApprovesWrite": L += ["APPROVAL of an ask-tier write", "{path, requesting run, approver} → an obligation record"]
        return L

    def fb_label(proc, recv, frags):
        k = defaultdict(list)
        for kind, v in frags: k[kind].append(v)
        L = []
        if k.get("read"):
            lenses = [r[5:] for r in k["read"] if r.startswith("show ")]
            verbs = [r for r in k["read"] if not r.startswith("show ")]
            L.append("COMPUTED STATE, via read commands:"); L += wrap(verbs)
            if lenses: L.append(f"  + show <lens> ×{len(lenses)}: " + ", ".join(lenses[:4]) + ", …")
        if k.get("status"):
            L.append("CI RUN STATUS red / green: " + ", ".join(k["status"])); L.append("  by email, or `gh run list` if the agent looks (D0266)")
        for v in k.get("other", []):
            L.append({"consoleLenses": "CONSOLE LENSES + the approve queue (127.0.0.1:7777)", "deliverableDrift": "MANIFEST DRIFT → done work computes as suspect"}.get(v, v))
        return L

    def label_size(lines): return max(len(l) for l in lines) * CHAR_W + 14, LH * len(lines) + 10

    # ---------- edges as records
    ctl = []   # dict(issuer, proc, lines)
    for (i, p), frags in ctl_edges.items():
        lines = ctl_label(i, p, frags)
        if lines: ctl.append(dict(issuer=i, proc=p, lines=lines))
    fb = []
    for (p, r), frags in fb_edges.items():
        lines = fb_label(p, r, frags)
        if lines: fb.append(dict(proc=p, recv=r, lines=lines))

    # ---------- vertical layout: row of boxes, then that row's channel band (one channel per edge)
    pos, row_y, band = {}, [], {}
    y = 60
    for li, roles in enumerate(LEVELS):
        row_y.append(y)
        for r in roles: pos[r] = (gap_x[GAP_OF[r]], y, CW, CH)
        y += CH + 26
        # channels for this row: control edges (sorted by target column, left to right) then feedback edges
        es = [e for e in ctl if e["issuer"] in roles]; es.sort(key=lambda e: PROCS.index(e["proc"]))
        fs = [f for f in fb if f["recv"] in roles]; fs.sort(key=lambda f: PROCS.index(f["proc"]))
        for e in es + fs:
            _, h = label_size(e["lines"]); y += h / 2 + 4; e["cy"] = y; y += h / 2 + 10
        y += 30
    PROC_Y = y + 20
    for p in PROCS: pos[p] = (proc_x[p], PROC_Y, PW, PH)
    H = PROC_Y + PH + 40

    # ---------- horizontal x assignment: trunk x per issuer slot, drop x per process slot (unique everywhere)
    out_ct = defaultdict(int); in_ct = defaultdict(int); fb_in_ct = defaultdict(int); fb_up_ct = defaultdict(int)
    n_out = defaultdict(int); n_in = defaultdict(int); n_fb_in = defaultdict(int); n_fb_up = defaultdict(int)
    for e in ctl: n_out[e["issuer"]] += 1; n_in[e["proc"]] += 1
    for f in fb: n_fb_up[f["proc"]] += 1; n_fb_in[f["recv"]] += 1
    for e in sorted(ctl, key=lambda e: e["cy"]):
        x, yy, w, h = pos[e["issuer"]]
        k = out_ct[e["issuer"]]; out_ct[e["issuer"]] += 1
        e["tx"] = x + 14 + k * 14                                   # trunk x: left part of the issuer's bottom
        px = proc_x[e["proc"]]; j = in_ct[e["proc"]]; in_ct[e["proc"]] += 1
        e["dx"] = px + 14 + j * (PW * 0.45 / max(1, n_in[e["proc"]]))   # drop x: left half of the process top
    for f in sorted(fb, key=lambda f: f["cy"]):
        x, yy, w, h = pos[f["recv"]]
        k = fb_in_ct[f["recv"]]; fb_in_ct[f["recv"]] += 1
        f["rx"] = x + w - 14 - k * 14                                # riser x into the receiver's bottom-right
        px = proc_x[f["proc"]]; j = fb_up_ct[f["proc"]]; fb_up_ct[f["proc"]] += 1
        f["ux"] = px + PW - 14 - j * (PW * 0.4 / max(1, n_fb_up[f["proc"]]))  # rise x: right half of the process top

    # ---------- segments
    segs = []   # (kind, cls, x1,y1,x2,y2, edge)   kind: v|h
    for e in ctl:
        x, yy, w, h = pos[e["issuer"]]; py = PROC_Y
        segs.append(("v", "ctl", e["tx"], yy + h, e["tx"], e["cy"], e))
        segs.append(("h", "ctl", e["tx"], e["cy"], e["dx"], e["cy"], e))
        segs.append(("v", "ctl", e["dx"], e["cy"], e["dx"], py, e))
    for f in fb:
        x, yy, w, h = pos[f["recv"]]; py = PROC_Y
        segs.append(("v", "fb", f["ux"], py, f["ux"], f["cy"], f))
        segs.append(("h", "fb", f["ux"], f["cy"], f["rx"], f["cy"], f))
        segs.append(("v", "fb", f["rx"], f["cy"], f["rx"], yy + h, f))

    hsegs = [s for s in segs if s[0] == "h"]
    def crossings(vx, y1, y2, own):
        lo, hi = min(y1, y2), max(y1, y2)
        out = []
        for _, _, hx1, hy, hx2, _, e in hsegs:
            if e is own: continue
            if min(hx1, hx2) < vx < max(hx1, hx2) and lo < hy < hi: out.append(hy)
        return sorted(out)

    R = 6
    def vpath(x, y1, y2, own):
        """Vertical from y1 to y2 with a semicircular hop over every horizontal it crosses."""
        down = y2 > y1
        ys = crossings(x, y1, y2, own)
        if not down: ys = ys[::-1]
        p = [f"M{x},{y1}"]
        for cy in ys:
            a, b = (cy - R, cy + R) if down else (cy + R, cy - R)
            p.append(f"L{x},{a} A{R},{R} 0 0 1 {x},{b}" if down else f"L{x},{a} A{R},{R} 0 0 0 {x},{b}")
        p.append(f"L{x},{y2}")
        return " ".join(p)

    svg = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" width="{W}" height="{H}" style="font-family:IBM Plex Sans,Segoe UI,Helvetica,Arial,sans-serif">',
           '<defs><marker id="down" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="var(--ctl)"/></marker>'
           '<marker id="up" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto"><path d="M0,0 L10,5 L0,10 z" fill="var(--fb)"/></marker></defs>']
    svg.append(f'<line x1="30" y1="{row_y[0]}" x2="30" y2="{PROC_Y}" stroke="var(--muted)" stroke-width="1" stroke-dasharray="2 5"/>')
    svg.append(f'<polygon points="30,{row_y[0] - 10} 25,{row_y[0] + 2} 35,{row_y[0] + 2}" fill="var(--muted)"/>')
    svg.append(f'<text transform="translate(16,{(row_y[0] + PROC_Y) / 2}) rotate(-90)" text-anchor="middle" font-size="11" letter-spacing="2" fill="var(--muted)">AUTHORITY</text>')
    svg.append(f'<text x="{W - 30}" y="{row_y[0]}" text-anchor="end" font-size="11" fill="var(--ctl)">▼  control action — solid; leaves the issuer bottom-left, enters the process top-left</text>')
    svg.append(f'<text x="{W - 30}" y="{row_y[0] + 16}" text-anchor="end" font-size="11" fill="var(--fb)">▲  feedback — dashed; leaves the process top-right, enters the receiver bottom-right</text>')
    svg.append(f'<text x="{W - 30}" y="{row_y[0] + 32}" text-anchor="end" font-size="11" fill="var(--muted)">a semicircle is a crossing, not a junction · the label on a line is what passes along it</text>')
    for yy, nm in zip(row_y, LEVEL_NAMES):
        svg.append(f'<text x="{W - 30}" y="{yy + CH - 4}" text-anchor="end" font-size="10" fill="var(--muted)" font-style="italic">{E(nm)}</text>')

    # lines (verticals with hops), then boxes, then labels on top
    for kind, cls, x1, y1, x2, y2, e in segs:
        dash = ' stroke-dasharray="6 4"' if cls == "fb" else ""
        if kind == "h":
            svg.append(f'<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="var(--{cls})" stroke-width="1.4"{dash}/>')
        else:
            end = ""
            if cls == "ctl" and y2 == PROC_Y: end = ' marker-end="url(#down)"'
            if cls == "fb" and y2 != e["cy"]: end = ' marker-end="url(#up)"'
            svg.append(f'<path d="{vpath(x1, y1, y2, e)}" fill="none" stroke="var(--{cls})" stroke-width="1.4"{dash}{end}/>')

    def box(role, info, is_proc):
        x, y, w, h = pos[role]
        fill = "var(--proc-bg)" if is_proc else "var(--ctl-bg)"; stroke = "var(--proc)" if is_proc else "var(--ctl)"
        out = [f'<rect x="{x}" y="{y}" width="{w}" height="{h}" rx="4" fill="{fill}" stroke="{stroke}" stroke-width="1.6"/>',
               f'<text x="{x + w / 2}" y="{y + 23}" text-anchor="middle" font-size="14" font-weight="600" fill="var(--ink)">{E(role)}</text>',
               f'<text x="{x + w / 2}" y="{y + 41}" text-anchor="middle" font-size="10" fill="var(--muted)">{E(info["what"][:40])}</text>']
        if info.get("anchor"): out.append(f'<text x="{x + w - 6}" y="{y + h - 5}" text-anchor="end" font-size="8.5" fill="var(--muted)" font-family="IBM Plex Mono,Consolas,monospace">{E(info["anchor"])}</text>')
        return "\n".join(out)
    for r in GAP_OF: svg.append(box(r, ctl_info[r], False))
    for p in PROCS: svg.append(box(p, proc_info[p], True))

    vsegs = [s for s in segs if s[0] == "v"]
    def verticals_through(lx, w, cy, h, own):
        """Other edges' vertical segments that would pass behind a label box at (lx, cy-h/2, w, h)."""
        n = 0
        for _, _, vx, y1, _, y2, e in vsegs:
            if e is own: continue
            if lx - 3 < vx < lx + w + 3 and min(y1, y2) < cy + h / 2 and max(y1, y2) > cy - h / 2: n += 1
        return n

    def label_on(e, x1, x2, cls):
        lines = e["lines"]; w, h = label_size(lines)
        lo, hi = min(x1, x2), max(x1, x2)
        # Sit ON the channel at the first position along it that no other edge's vertical passes behind;
        # the channel may be extended past its far end when the span is shorter than the label.
        inside = list(range(int(lo) + 14, max(int(lo) + 15, int(hi) - int(w) - 8), 8))
        beyond = list(range(max(int(lo) + 14, int(hi) - int(w) - 8), min(int(hi) + 260, int(W - w - 20)), 8))
        cands = inside + beyond
        # fewest verticals behind the box; then nearest the channel's own span; a crossing behind a label
        # near its line beats a clean label floating far from it
        best = min(cands, key=lambda cx: (verticals_through(cx, w, e["cy"], h, e) + (0 if cx <= hi - w else 1), abs(cx - (lo + 14))))
        lx = best
        ly = e["cy"] - h / 2
        out = []
        if lx > hi:   # the label sits past the channel's far end: extend the channel to it as a leader
            out.append(f'<line x1="{hi}" y1="{e["cy"]}" x2="{lx}" y2="{e["cy"]}" stroke="var(--{cls})" stroke-width="1.4"{" stroke-dasharray=\"6 4\"" if cls == "fb" else ""}/>')
        out += [f'<rect x="{lx}" y="{ly}" width="{w}" height="{h}" rx="3" fill="var(--panel)" stroke="var(--{cls})" stroke-width="0.9"{" stroke-dasharray=\"4 3\"" if cls == "fb" else ""}/>']
        for i, l in enumerate(lines):
            out.append(f'<text x="{lx + 7}" y="{ly + 13 + i * LH}" font-size="9.6" font-family="IBM Plex Mono,Consolas,monospace" font-weight="{"600" if i == 0 else "400"}" fill="var(--ink)" xml:space="preserve">{E(l)}</text>')
        return "\n".join(out)
    for e in ctl: svg.append(label_on(e, e["tx"], e["dx"], "ctl"))
    for f in fb: svg.append(label_on(f, f["ux"], f["rx"], "fb"))
    svg.append("</svg>")
    return "\n".join(svg)

if __name__ == "__main__":
    svg = render(load(sys.argv))
    if "--out" in sys.argv:
        io.open(sys.argv[sys.argv.index("--out") + 1], "w", encoding="utf-8").write(svg)
    else:
        sys.stdout.reconfigure(encoding="utf-8")
        sys.stdout.write(svg)
