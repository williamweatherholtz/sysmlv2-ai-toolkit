#!/usr/bin/env python3
"""
report.py — HTML project status dashboard (materialized VIEW, D0040 / D0015).

Tabs:
  1. Current Sprint   — cursor, backlog counts, ready frontier, velocity, recent commits
  2. Sprint History   — full metrics table + efficiency bars
  3. Process Health   — skills inventory, decision velocity, open issues, coverage
  4. Workflows        — visual delivery pipeline + all engine workflows
  5. All Decisions    — full decision log, filterable

Usage:
  python .engine/tools/report.py [--output PATH] [--stdout]
"""
import re, subprocess, sys, json, argparse
from pathlib import Path
from datetime import datetime

ROOT = Path(__file__).resolve().parent.parent.parent

# ---------------------------------------------------------------------------
# Data gathering — all subprocess calls use UTF-8 explicitly (Windows fix)
# ---------------------------------------------------------------------------

def _git(*args):
    r = subprocess.run(
        ["git"] + list(args),
        capture_output=True, encoding="utf-8", errors="replace", cwd=ROOT
    )
    return r.stdout.strip()

def head_sha():
    return _git("rev-parse", "--short", "HEAD")

def recent_commits(n=12):
    raw = _git("log", "--oneline", f"-{n}")
    return raw.splitlines() if raw else []

def parse_sprint_metrics():
    delivery = ROOT / ".tracking" / "delivery"
    sprints = []
    for f in sorted(delivery.glob("*.sysml")):
        text = f.read_text(encoding="utf-8", errors="replace")
        pts_m = re.search(r":>>\s+estimatedPoints\s*=\s*(\d+)", text)
        if not pts_m:
            continue
        title_m = re.search(r":>>\s+title\s*=\s*\"([^\"]{0,120})\"", text)
        hrs_m   = re.search(r":>>\s+actualHours\s*=\s*([\d.]+)", text)
        kind_m  = re.search(r":>>\s+kind\s*=\s*WorkKind::(\w+)", text)
        sprints.append({
            "file":   f.stem,
            "title":  title_m.group(1) if title_m else f.stem,
            "pts":    int(pts_m.group(1)),
            "hours":  float(hrs_m.group(1)) if hrs_m else None,
            "kind":   kind_m.group(1) if kind_m else "code",
        })
    return sprints

def orient_data():
    cli = ROOT / "target" / "release" / "sysmlv2.exe"
    default = {"cursor": {}, "ready": [], "suspect": [], "counts": {"done": "?", "outstanding": "?"},
               "_fallback": not cli.exists()}
    if not cli.exists():
        return default
    r = subprocess.run([str(cli), "orient", str(ROOT)],
                       capture_output=True, encoding="utf-8", errors="replace")
    try:
        return json.loads(r.stdout)
    except Exception:
        default["_fallback"] = True
        return default

def ready_names(o):
    items = []
    for item in o.get("ready", []):
        if ":>> procedureText" in item:
            continue
        name = item.split(" : ")[0].split("{")[0].strip()
        if name:
            items.append(name)
    return items

def all_decisions():
    dec_dir = ROOT / ".engine" / "decisions"
    results = []
    for f in sorted(dec_dir.glob("*.sysml")):
        text = f.read_text(encoding="utf-8", errors="replace")
        title_m = re.search(r":>>\s+title\s*=\s*\"([^\"]{0,200})\"", text)
        ctx_m   = re.search(r":>>\s+context\s*=\s*\"([^\"]{0,300})\"", text)
        dec_m   = re.search(r":>>\s+decisionText\s*=\s*\"([^\"]{0,100})", text)
        results.append({
            "num":     f.stem[:4],
            "file":    f.stem,
            "title":   title_m.group(1) if title_m else f.stem,
            "context": ctx_m.group(1)[:120] + "..." if ctx_m else "",
            "summary": (dec_m.group(1)[:80] + "...") if dec_m else "",
        })
    return results

def parse_skills():
    reg = (ROOT / ".engine" / "skills" / "skills-registry.sysml").read_text(
        encoding="utf-8", errors="replace")
    skills = []
    for block in re.finditer(
        r"part\s+(\w+)\s*:\s*AISkill\s*\{([^}]+)\}", reg, re.DOTALL):
        body = block.group(2)
        t = re.search(r":>>\s+title\s*=\s*\"([^\"]+)\"", body)
        p = re.search(r":>>\s+purpose\s*=\s*\"([^\"]+)\"", body)
        l = re.search(r":>>\s+location\s*=\s*\"([^\"]+)\"", body)
        w = re.search(r":>>\s+writePolicy\s*=\s*WritePolicy::(\w+)", body)
        tr = re.search(r":>>\s+triggerCondition\s*=\s*\"([^\"]+)\"", body)
        skills.append({
            "name":    t.group(1) if t else block.group(1),
            "purpose": p.group(1) if p else "",
            "loc":     l.group(1) if l else "",
            "policy":  w.group(1) if w else "",
            "trigger": tr.group(1) if tr else "",
        })
    return skills

def parse_issues():
    iss_file = ROOT / ".tracking" / "issues.sysml"
    if not iss_file.exists():
        return []
    text = iss_file.read_text(encoding="utf-8", errors="replace")
    issues = []
    for block in re.finditer(r"part\s+\w+\s*:\s*Issue\s*\{([^}]+)\}", text, re.DOTALL):
        body = block.group(1)
        desc_m = re.search(r":>>\s+description\s*=\s*\"([^\"]+)\"", body)
        task_m = re.search(r":>>\s+relatedTask\s*=\s*\"([^\"]+)\"", body)
        title_m= re.search(r":>>\s+title\s*=\s*\"([^\"]+)\"", body)
        issues.append({
            "title":   title_m.group(1) if title_m else "Untitled",
            "desc":    desc_m.group(1)[:120] if desc_m else "",
            "task":    task_m.group(1) if task_m else "",
        })
    return issues

def velocity_stats(sprints, window=3):
    complete = [s for s in sprints if s["hours"] is not None]
    if not complete:
        return {"trailing_pts": None, "trailing_eff": None, "window": window}
    recent = complete[-window:]
    avg_pts = sum(s["pts"] for s in recent) / len(recent)
    eff_list = [s["pts"] / s["hours"] for s in recent if s["hours"] > 0]
    avg_eff = sum(eff_list) / len(eff_list) if eff_list else None
    return {
        "trailing_pts": round(avg_pts, 2),
        "trailing_eff": round(avg_eff, 3) if avg_eff else None,
        "window": window,
    }

# ---------------------------------------------------------------------------
# HTML rendering helpers
# ---------------------------------------------------------------------------

def esc(s):
    return (str(s)
            .replace("&", "&amp;").replace("<", "&lt;")
            .replace(">", "&gt;").replace('"', "&quot;"))

def badge(label, cls="gray"):
    return f'<span class="badge {cls}">{esc(label)}</span>'

def kv(key, val, dim=False):
    v_cls = ' class="val dim"' if dim else ' class="val"'
    return f'<div class="kv"><span class="key">{esc(key)}</span><span{v_cls}>{val}</span></div>'

def eff_cell(pts, hours):
    if hours is None or hours == 0:
        return '<td class="num dim">&mdash;</td>'
    e = pts / hours
    cls = "good" if e >= 0.4 else "warn"
    return f'<td class="num {cls}">{e:.2f}</td>'

def bar(value, max_val, width=80, cls="bar-blue"):
    if max_val == 0 or value is None:
        return ""
    pct = min(100, int(value / max_val * width))
    return f'<span class="{cls}" style="display:inline-block;width:{pct}px;height:8px;border-radius:2px;vertical-align:middle;margin-left:6px"></span>'

# ---------------------------------------------------------------------------
# Tab 1 — Current Sprint
# ---------------------------------------------------------------------------

def tab_current(sprints, orient, commits):
    o = orient
    cursor = o.get("cursor", {})
    counts = o.get("counts", {})
    done = counts.get("done", "?")
    outstanding = counts.get("outstanding", "?")
    suspect_count = len(o.get("suspect", []))
    active_phase = cursor.get("activePhase", "—")
    active_workflow = cursor.get("activeWorkflow", "—")

    active = sprints[-1] if sprints else None
    v3 = velocity_stats(sprints, 3)
    v5 = velocity_stats(sprints, 5)
    ready_items = ready_names(o)

    def avg_row(label, v):
        if v["trailing_pts"] is None:
            return kv(label, '<span class="dim">no data yet — set actualHours at closeOut</span>')
        eff_str = f'{v["trailing_eff"]:.3f} pts/h' if v["trailing_eff"] else "&mdash;"
        return kv(label, f'{v["trailing_pts"]} pts avg &nbsp;&middot;&nbsp; {eff_str}')

    if active:
        hrs_disp = f'{active["hours"]:.1f}&thinsp;h' if active["hours"] is not None else '<span class="dim">not yet recorded</span>'
        eff_disp = (f'{active["pts"]/active["hours"]:.3f}&thinsp;pts/h' if active["hours"]
                    else '<span class="dim">&mdash;</span>')
        active_html = (kv("Sprint", esc(active["file"])) +
                       kv("Estimated points", str(active["pts"])) +
                       kv("Actual hours", hrs_disp) +
                       kv("Efficiency", eff_disp))
    else:
        active_html = kv("Sprint data", "none found", dim=True)

    ready_html = ("".join(f'<span class="chip">{esc(r)}</span>' for r in ready_items)
                  or '<span class="dim">none</span>')

    commit_rows = ""
    for c in commits:
        parts = c.split(" ", 1)
        sha_part = parts[0] if parts else ""
        msg_part = esc(parts[1]) if len(parts) > 1 else ""
        commit_rows += f'<div class="commit"><span class="sha">{sha_part}</span> {msg_part}</div>'

    fallback = ('<p class="warn-note">&#9888; sysmlv2.exe not found &mdash; counts unavailable</p>'
                if o.get("_fallback") else "")

    return f"""
<div class="grid3">
  <div class="card">
    <h2>Active Cursor</h2>
    {kv("Workflow", esc(active_workflow))}
    {kv("Phase", f'<span class="phase-chip">{esc(active_phase)}</span>')}
    {kv("Entered at", esc(cursor.get("enteredAt", "&mdash;")))}
    {kv("Entered by", esc(cursor.get("enteredBy", "&mdash;")))}
    {fallback}
  </div>
  <div class="card">
    <h2>Backlog Counts</h2>
    <div style="margin-bottom:10px">
      {badge(f"Done {done}", "green")}
      {badge(f"Outstanding {outstanding}", "blue")}
      {badge(f"Suspect {suspect_count}", "orange" if suspect_count else "gray")}
    </div>
    <h2>Ready Frontier</h2>
    <div style="margin-top:6px">{ready_html}</div>
  </div>
  <div class="card">
    <h2>Active Sprint</h2>
    {active_html}
  </div>
</div>
<div class="grid2" style="margin-top:16px">
  <div class="card">
    <h2>Velocity &amp; Efficiency Averages</h2>
    {avg_row("Trailing 3-sprint", v3)}
    {avg_row("Trailing 5-sprint", v5)}
    {kv("Formula", '<span class="dim">efficiency = estimatedPoints / actualHours</span>')}
  </div>
  <div class="card">
    <h2>Recent Activity ({len(commits)} commits)</h2>
    {commit_rows}
  </div>
</div>"""

# ---------------------------------------------------------------------------
# Tab 2 — Sprint History
# ---------------------------------------------------------------------------

def tab_history(sprints):
    if not sprints:
        return '<div class="card"><p class="dim">No sprint data found.</p></div>'
    max_pts  = max(s["pts"] for s in sprints)
    max_hrs  = max((s["hours"] or 0) for s in sprints) or 1

    rows = ""
    for s in sprints:
        hrs_str = f'{s["hours"]:.1f}' if s["hours"] is not None else '<span class="dim">&mdash;</span>'
        pts_bar = bar(s["pts"], max_pts, 60, "bar-blue")
        hrs_bar = bar(s["hours"], max_hrs, 60, "bar-green") if s["hours"] else ""
        rows += (f'<tr>'
                 f'<td style="font-family:monospace;font-size:12px">{esc(s["file"])}</td>'
                 f'<td><span class="badge {"blue" if s["kind"]=="code" else "gray"}">{esc(s["kind"])}</span></td>'
                 f'<td class="num">{s["pts"]}{pts_bar}</td>'
                 f'<td class="num">{hrs_str}{hrs_bar}</td>'
                 f'{eff_cell(s["pts"], s["hours"])}'
                 f'</tr>')

    complete = [s for s in sprints if s["hours"] is not None]
    total_pts = sum(s["pts"] for s in sprints)
    total_hrs = sum(s["hours"] for s in complete)
    overall_eff = total_pts / total_hrs if total_hrs else None
    eff_str = f'{overall_eff:.3f} pts/h' if overall_eff else "&mdash;"

    return f"""
<div class="card full">
  <h2>{len(sprints)} sprints &middot; {total_pts} total points &middot;
      {total_hrs:.1f}&thinsp;h recorded ({len(complete)}/{len(sprints)} sprints have actualHours) &middot;
      overall efficiency: {eff_str}</h2>
  <table style="margin-top:12px">
    <thead><tr>
      <th>Sprint File</th><th>Kind</th>
      <th style="text-align:right">Est Pts</th>
      <th style="text-align:right">Actual Hours</th>
      <th style="text-align:right">Efficiency (pts/h)</th>
    </tr></thead>
    <tbody>{rows}</tbody>
  </table>
</div>"""

# ---------------------------------------------------------------------------
# Tab 3 — Process Health
# ---------------------------------------------------------------------------

def tab_process(skills, decisions, issues, sprints):
    # Skill inventory
    skill_rows = ""
    for sk in skills:
        policy_cls = {"direct": "green", "prOnly": "orange", "readOnly": "gray"}.get(sk["policy"], "gray")
        skill_rows += (f'<tr>'
                       f'<td><code>{esc(sk["name"])}</code></td>'
                       f'<td>{badge(sk["policy"], policy_cls)}</td>'
                       f'<td style="font-size:12px;color:#8b949e">{esc(sk["purpose"][:90])}</td>'
                       f'</tr>')

    # Decision velocity by sprint (group by file prefix)
    dec_by_sprint: dict[str, int] = {}
    for d in decisions:
        num = int(d["num"]) if d["num"].isdigit() else 0
        # Map decision number to rough sprint bucket
        bracket = f"D{(num // 5)*5:04d}-D{min((num//5)*5+4, num):04d}"
        dec_by_sprint[bracket] = dec_by_sprint.get(bracket, 0) + 1
    vel_rows = "".join(
        f'<div class="kv"><span class="key">{esc(k)}</span>'
        f'<span class="val">{v} decision{"s" if v!=1 else ""}'
        f'{bar(v, max(dec_by_sprint.values()), 100, "bar-purple")}</span></div>'
        for k, v in sorted(dec_by_sprint.items()))

    # Issue list
    if issues:
        issue_rows = "".join(
            f'<div style="padding:6px 0;border-bottom:1px solid #21262d">'
            f'<div style="color:#d29922;font-size:12px;font-weight:600">{esc(i["title"])}</div>'
            f'<div style="color:#8b949e;font-size:11px;margin-top:2px">{esc(i["desc"])}</div>'
            f'{"<div style=\'font-size:11px;color:#58a6ff;margin-top:2px\'>&rarr; " + esc(i["task"]) + "</div>" if i["task"] else ""}'
            f'</div>'
            for i in issues)
    else:
        issue_rows = '<span class="dim">No open issues.</span>'

    # Coverage map — ceremony phases and their skills
    phases = [
        ("Refine",    ["sprint-planning", "backlog-refinement"], "inspect"),
        ("Standup",   ["sprint-standup"],                         "inspect"),
        ("Implement", ["test-design", "test-result", "repo-push", "traceability-audit"], "test/demo"),
        ("Review",    ["sprint-review"],                          "inspect"),
        ("CloseOut",  ["sprint-closeout"],                        "confirmation"),
        ("Retro",     ["sprint-retro"],                           "confirmation"),
    ]
    cov_rows = ""
    for phase, phase_skills, gate in phases:
        chips = " ".join(f'<span class="chip-small">{esc(s)}</span>' for s in phase_skills)
        cov_rows += (f'<div class="kv">'
                     f'<span class="key">{esc(phase)}</span>'
                     f'<span class="val">{chips} '
                     f'<span class="dim" style="font-size:11px">gate: {esc(gate)}</span></span></div>')

    # Other skill domains
    domain_map = {
        "Safety": ["stpa"],
        "Requirements": ["requirement-quality"],
        "Traceability": ["traceability-audit"],
        "Testing": ["test-design", "test-result"],
        "Views": ["status-report"],
        "VCS": ["repo-push"],
        "Routing": ["engine-triage"],
    }
    domain_rows = ""
    for domain, dskills in domain_map.items():
        chips = " ".join(f'<span class="chip-small">{esc(s)}</span>' for s in dskills)
        domain_rows += (f'<div class="kv"><span class="key">{esc(domain)}</span>'
                        f'<span class="val">{chips}</span></div>')

    return f"""
<div class="grid2">
  <div class="card full">
    <h2>Skills Inventory &mdash; {len(skills)} skills registered</h2>
    <table style="margin-top:10px">
      <thead><tr>
        <th>Skill Name</th><th>Write Policy</th><th>Purpose</th>
      </tr></thead>
      <tbody>{skill_rows}</tbody>
    </table>
  </div>
</div>
<div class="grid2" style="margin-top:16px">
  <div class="card">
    <h2>Delivery Ceremony Coverage</h2>
    {cov_rows}
    <div style="margin-top:10px">
      <h2>Other Skill Domains</h2>
      {domain_rows}
    </div>
  </div>
  <div class="card">
    <h2>Decision Velocity</h2>
    <p style="color:#8b949e;font-size:11px;margin-bottom:8px">{len(decisions)} total decisions</p>
    {vel_rows}
    <div style="margin-top:14px">
      <h2>Open Issues ({len(issues)})</h2>
      <div style="margin-top:6px">{issue_rows}</div>
    </div>
  </div>
</div>"""

# ---------------------------------------------------------------------------
# Tab 4 — Workflows
# ---------------------------------------------------------------------------

def tab_workflows(orient):
    cursor = orient.get("cursor", {})
    active_phase = cursor.get("activePhase", "").lower()

    def phase_box(name, skills, gate_method, phase_key):
        is_active = phase_key in active_phase
        active_cls = " phase-active" if is_active else ""
        active_label = '<div class="active-badge">&#9654; ACTIVE</div>' if is_active else ""
        skill_chips = "".join(f'<div class="phase-skill">{esc(s)}</div>' for s in skills)
        return (f'<div class="phase-box{active_cls}">'
                f'{active_label}'
                f'<div class="phase-name">{esc(name)}</div>'
                f'<div class="phase-skills">{skill_chips}</div>'
                f'<div class="phase-gate">gate: {esc(gate_method)}</div>'
                f'</div>')

    delivery_phases = [
        ("Refine",    ["sprint-planning", "backlog-refinement"], "inspect",      "refine"),
        ("Standup",   ["sprint-standup"],                         "inspect",      "standup"),
        ("Implement", ["test-design", "test-result", "repo-push"],"test / demo",  "implement"),
        ("Review",    ["sprint-review"],                          "inspect",      "review"),
        ("CloseOut",  ["sprint-closeout"],                        "confirmation", "closeout"),
        ("Retro",     ["sprint-retro"],                           "confirmation", "retro"),
    ]

    delivery_boxes = ""
    for i, (name, skills, gate, key) in enumerate(delivery_phases):
        delivery_boxes += phase_box(name, skills, gate, key)
        if i < len(delivery_phases) - 1:
            delivery_boxes += '<div class="phase-arrow">&rsaquo;</div>'

    # Other workflows — simpler horizontal flows
    def simple_flow(phases):
        parts = []
        for i, p in enumerate(phases):
            parts.append(f'<span class="simple-phase">{esc(p)}</span>')
            if i < len(phases) - 1:
                parts.append('<span class="simple-arrow">&rarr;</span>')
        return "".join(parts)

    business_flow = simple_flow(["Persona Analysis", "Need Elicitation", "Need Prioritization", "Backlog"])
    arch_flow     = simple_flow(["Data Architecture", "Application Architecture", "Technology Architecture"])
    deploy_flow   = simple_flow(["Release Planning", "Configuration", "V&amp;V", "Operate"])
    operate_flow  = simple_flow(["Field Feedback", "Issue Triage", "Incorporate"])
    cr_flow       = simple_flow(["Propose Change", "Human Accept", "Apply + Validate", "Record Decision", "Commit CR:"])

    # Self-improvement loop diagram
    improvement_loop = simple_flow([
        "Sprint Review (transcript scan)",
        "Retro (triage findings)",
        "CHANGE (skill / CLAUDE.md / decision)",
        "Next Sprint (new skill in effect)",
    ])

    return f"""
<div class="card full" style="margin-bottom:16px">
  <h2>Delivery Workflow &mdash; <span style="color:#8b949e;font-weight:400">active cursor:
    <span class="phase-chip">{esc(cursor.get("activePhase","&mdash;"))}</span></span></h2>
  <div class="workflow-row" style="margin-top:14px">
    {delivery_boxes}
  </div>
  <p style="color:#484f58;font-size:11px;margin-top:10px">
    &#9654; highlighted box = current active phase &nbsp;&middot;&nbsp;
    Each phase has an associated gate (inspect / test / confirmation).
    Confirmation gates require explicit human sign-off.
  </p>
</div>

<div class="grid2">
  <div class="card">
    <h2>Change Request Workflow <span style="color:#8b949e;font-size:11px">(cross-cutting)</span></h2>
    <div class="simple-flow" style="margin-top:8px">{cr_flow}</div>
    <p style="color:#8b949e;font-size:11px;margin-top:8px">
      Triggered for any schema/process/skill change. Records a Decision file
      in <code>.engine/decisions/</code> before committing. Commit prefix: <code>CR:</code>.
    </p>
  </div>
  <div class="card">
    <h2>Self-Improvement Loop <span style="color:#8b949e;font-size:11px">(engine mission)</span></h2>
    <div class="simple-flow" style="margin-top:8px">{improvement_loop}</div>
    <p style="color:#8b949e;font-size:11px;margin-top:8px">
      Every sprint's transcript is reviewed for errors, inefficiencies, and gaps.
      Findings become typed improvement items, dispatched to skills / CLAUDE.md /
      decisions / backlog. The engine improves its own process each sprint.
    </p>
  </div>
  <div class="card">
    <h2>Business Workflow</h2>
    <div class="simple-flow" style="margin-top:8px">{business_flow}</div>
    <p style="color:#8b949e;font-size:11px;margin-top:8px">
      Captures <em>what</em> and <em>why</em>: Personas &rarr; Needs &rarr; prioritized backlog.
      Feeds into Architecture &amp; Delivery.
    </p>
  </div>
  <div class="card">
    <h2>Architecture Workflow</h2>
    <div class="simple-flow" style="margin-top:8px">{arch_flow}</div>
    <p style="color:#8b949e;font-size:11px;margin-top:8px">
      Captures <em>how</em>: data model &rarr; application structure &rarr; technology choices.
      Produces Requirements that satisfy Needs (traced by <code>satisfy</code> edge).
    </p>
  </div>
  <div class="card">
    <h2>Deploy Workflow</h2>
    <div class="simple-flow" style="margin-top:8px">{deploy_flow}</div>
  </div>
  <div class="card">
    <h2>Operate Workflow</h2>
    <div class="simple-flow" style="margin-top:8px">{operate_flow}</div>
  </div>
</div>"""

# ---------------------------------------------------------------------------
# Tab 5 — All Decisions
# ---------------------------------------------------------------------------

def tab_decisions(decisions):
    rows = ""
    for d in decisions:
        rows += (f'<tr>'
                 f'<td style="font-family:monospace;font-weight:600;color:#58a6ff">{esc(d["num"])}</td>'
                 f'<td style="font-size:12px">{esc(d["title"][:110])}</td>'
                 f'<td style="font-size:11px;color:#8b949e">{esc(d["context"][:90])}</td>'
                 f'</tr>')
    return f"""
<div class="card full">
  <h2>{len(decisions)} Architecture Decisions Recorded</h2>
  <input id="dec-filter" type="text" placeholder="Filter decisions..."
    oninput="filterDecisions(this.value)"
    style="margin:10px 0;padding:6px 10px;background:#0d1117;border:1px solid #30363d;
           color:#c9d1d9;border-radius:4px;width:320px;font-size:13px">
  <table id="dec-table" style="margin-top:4px">
    <thead><tr><th>D#</th><th>Title</th><th>Context summary</th></tr></thead>
    <tbody id="dec-body">{rows}</tbody>
  </table>
</div>"""

# ---------------------------------------------------------------------------
# Page assembly
# ---------------------------------------------------------------------------

CSS = """
*{{box-sizing:border-box;margin:0;padding:0}}
body{{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;
     background:#0d1117;color:#c9d1d9;font-size:14px}}
header{{background:#161b22;border-bottom:1px solid #30363d;
        padding:14px 24px;display:flex;justify-content:space-between;align-items:center}}
header h1{{font-size:17px;color:#58a6ff;font-weight:600}}
header .meta{{color:#8b949e;font-size:12px}}
/* tabs */
.tabs{{background:#161b22;border-bottom:1px solid #30363d;padding:0 24px;display:flex;gap:2px}}
.tab-btn{{padding:10px 18px;background:none;border:none;color:#8b949e;cursor:pointer;
          font-size:13px;border-bottom:2px solid transparent;transition:.15s}}
.tab-btn:hover{{color:#c9d1d9}}
.tab-btn.active{{color:#58a6ff;border-bottom-color:#58a6ff}}
.tab-content{{display:none;padding:20px 24px}}
.tab-content.active{{display:block}}
/* layout */
.grid3{{display:grid;grid-template-columns:repeat(3,1fr);gap:14px}}
.grid2{{display:grid;grid-template-columns:repeat(2,1fr);gap:14px}}
.full{{grid-column:1/-1}}
.card{{background:#161b22;border:1px solid #30363d;border-radius:8px;padding:16px}}
.card h2{{font-size:11px;font-weight:600;color:#8b949e;text-transform:uppercase;
          letter-spacing:.08em;margin-bottom:10px}}
/* kv */
.kv{{display:flex;justify-content:space-between;padding:4px 0;
     border-bottom:1px solid #21262d;gap:12px}}
.kv:last-child{{border-bottom:none}}
.kv .key{{color:#8b949e;flex-shrink:0}}
.kv .val{{color:#e6edf3;font-weight:500;text-align:right}}
.kv .val.dim{{color:#8b949e;font-weight:400}}
.dim{{color:#8b949e}}
/* badges & chips */
.badge{{display:inline-block;padding:2px 7px;border-radius:10px;
        font-size:11px;font-weight:600;margin-right:4px}}
.badge.green {{background:#1a3a1a;color:#3fb950;border:1px solid #238636}}
.badge.blue  {{background:#0d2b4e;color:#58a6ff;border:1px solid #1f6feb}}
.badge.orange{{background:#3b2300;color:#d29922;border:1px solid #9e6a03}}
.badge.gray  {{background:#21262d;color:#8b949e;border:1px solid #30363d}}
.badge.purple{{background:#2d1f45;color:#bc8cff;border:1px solid #6e40c9}}
.chip{{display:inline-block;background:#21262d;border:1px solid #30363d;
       border-radius:4px;padding:2px 8px;margin:2px;font-size:12px;
       color:#58a6ff;font-family:monospace}}
.chip-small{{display:inline-block;background:#1a2d4e;border:1px solid #1f6feb;
             border-radius:3px;padding:1px 6px;margin:1px;font-size:11px;color:#79c0ff}}
/* phase-chip inline */
.phase-chip{{background:#1a2d4e;border:1px solid #1f6feb;border-radius:4px;
             padding:2px 10px;font-size:12px;color:#79c0ff;font-weight:500}}
/* tables */
table{{width:100%;border-collapse:collapse}}
th{{text-align:left;color:#8b949e;font-size:11px;font-weight:600;
    text-transform:uppercase;letter-spacing:.06em;padding:6px 8px;
    border-bottom:1px solid #30363d}}
td{{padding:6px 8px;border-bottom:1px solid #21262d;color:#c9d1d9}}
tr:last-child td{{border-bottom:none}}
.num{{font-family:monospace;text-align:right}}
.good{{color:#3fb950}}.warn{{color:#d29922}}
/* commits */
.commit{{font-family:monospace;font-size:12px;padding:3px 0;
         border-bottom:1px solid #21262d;white-space:nowrap;
         overflow:hidden;text-overflow:ellipsis}}
.commit:last-child{{border-bottom:none}}
.sha{{color:#58a6ff}}
/* bars */
.bar-blue  {{background:#1f6feb}}
.bar-green {{background:#238636}}
.bar-purple{{background:#6e40c9}}
/* workflow */
.workflow-row{{display:flex;align-items:stretch;gap:0;flex-wrap:wrap}}
.phase-box{{background:#161b22;border:1px solid #30363d;border-radius:6px;
            padding:12px 14px;min-width:120px;flex:1;text-align:center;
            transition:.15s}}
.phase-box.phase-active{{background:#0d2b4e;border-color:#58a6ff;
                          box-shadow:0 0 0 1px #1f6feb}}
.active-badge{{font-size:10px;color:#58a6ff;font-weight:700;margin-bottom:4px}}
.phase-name{{font-size:13px;font-weight:600;color:#e6edf3;margin-bottom:6px}}
.phase-skills{{display:flex;flex-direction:column;gap:2px;margin-bottom:6px}}
.phase-skill{{font-size:10px;background:#21262d;border-radius:3px;padding:1px 5px;
              color:#8b949e;font-family:monospace}}
.phase-gate{{font-size:10px;color:#484f58}}
.phase-arrow{{display:flex;align-items:center;padding:0 4px;
              color:#30363d;font-size:22px;font-weight:300}}
/* simple flow */
.simple-flow{{display:flex;flex-wrap:wrap;gap:4px;align-items:center}}
.simple-phase{{background:#21262d;border:1px solid #30363d;border-radius:4px;
               padding:4px 10px;font-size:12px;color:#c9d1d9}}
.simple-arrow{{color:#8b949e;font-size:16px;padding:0 2px}}
/* misc */
.warn-note{{color:#d29922;font-size:11px;margin-top:8px}}
footer{{text-align:center;padding:14px;color:#484f58;font-size:11px;
        border-top:1px solid #30363d;margin-top:8px}}
@media(max-width:900px){{.grid3,.grid2{{grid-template-columns:1fr}}}}
"""

JS = """
function showTab(id) {
  document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
  document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
  document.getElementById('tab-' + id).classList.add('active');
  document.querySelector('[data-tab="' + id + '"]').classList.add('active');
}
function filterDecisions(q) {
  const rows = document.querySelectorAll('#dec-body tr');
  q = q.toLowerCase();
  rows.forEach(r => {
    r.style.display = r.textContent.toLowerCase().includes(q) ? '' : 'none';
  });
}
"""

def render_html(sprints, orient, commits, decisions, skills, issues, generated_at, sha):
    t1 = tab_current(sprints, orient, commits)
    t2 = tab_history(sprints)
    t3 = tab_process(skills, decisions, issues, sprints)
    t4 = tab_workflows(orient)
    t5 = tab_decisions(decisions)

    return f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>SysML v2 AI Toolkit &mdash; Status Dashboard</title>
<style>{CSS}</style>
</head>
<body>
<header>
  <h1>SysML v2 AI Toolkit &mdash; Status Dashboard</h1>
  <div class="meta">Generated {esc(generated_at)} &nbsp;&middot;&nbsp;
    HEAD <code style="color:#58a6ff">{esc(sha)}</code></div>
</header>
<nav class="tabs">
  <button class="tab-btn active" data-tab="sprint"    onclick="showTab('sprint')">Current Sprint</button>
  <button class="tab-btn"        data-tab="history"   onclick="showTab('history')">Sprint History</button>
  <button class="tab-btn"        data-tab="process"   onclick="showTab('process')">Process Health</button>
  <button class="tab-btn"        data-tab="workflows" onclick="showTab('workflows')">Workflows</button>
  <button class="tab-btn"        data-tab="decisions" onclick="showTab('decisions')">All Decisions ({len(decisions)})</button>
</nav>
<div id="tab-sprint"    class="tab-content active">{t1}</div>
<div id="tab-history"   class="tab-content">{t2}</div>
<div id="tab-process"   class="tab-content">{t3}</div>
<div id="tab-workflows" class="tab-content">{t4}</div>
<div id="tab-decisions" class="tab-content">{t5}</div>
<footer>
  Generated by <code>.engine/tools/report.py</code> &mdash;
  <strong>derived VIEW</strong>, not authored content (D0015, D0040).
  Regenerate at any time: <code>python .engine/tools/report.py</code>
</footer>
<script>{JS}</script>
</body>
</html>"""

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="Generate HTML project status dashboard")
    parser.add_argument("--output", "-o", default=None)
    parser.add_argument("--stdout", action="store_true")
    args = parser.parse_args()

    generated_at = datetime.now().strftime("%Y-%m-%d %H:%M")
    sha       = head_sha()
    sprints   = parse_sprint_metrics()
    orient    = orient_data()
    commits   = recent_commits(12)
    decisions = all_decisions()
    skills    = parse_skills()
    issues    = parse_issues()

    html = render_html(sprints, orient, commits, decisions, skills, issues, generated_at, sha)

    if args.stdout:
        print(html)
        return

    out_path = Path(args.output) if args.output else ROOT / "status.html"
    out_path.write_text(html, encoding="utf-8")
    print(f"[report.py] Dashboard written to: {out_path}")
    print(f"            {len(sprints)} sprints, {len(decisions)} decisions, "
          f"{len(skills)} skills -- HEAD {sha}")

    try:
        import webbrowser
        webbrowser.open(out_path.as_uri())
    except Exception:
        pass

if __name__ == "__main__":
    main()
