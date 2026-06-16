#!/usr/bin/env python3
"""
report.py — HTML project status dashboard (materialized VIEW, D0040 / D0015).

Usage:
  python .engine/tools/report.py [--output PATH] [--stdout]

Generates a self-contained HTML file from:
  - .tracking/delivery/*.sysml  (sprint metrics)
  - .engine/decisions/*.sysml   (recent decisions)
  - sysmlv2.exe orient           (done/ready/suspect counts)
  - git log                      (recent commits)

This is a VIEW — regenerable from authored facts + git. Do not commit the output
as authored content unless explicitly snapshotting to docs/ with a dated label.
"""
import re, subprocess, sys, json, argparse
from pathlib import Path
from datetime import datetime

ROOT = Path(__file__).resolve().parent.parent.parent


# ---------------------------------------------------------------------------
# Data gathering
# ---------------------------------------------------------------------------

def _git(*args):
    r = subprocess.run(["git"] + list(args), capture_output=True, text=True, cwd=ROOT)
    return r.stdout.strip()


def head_sha():
    return _git("rev-parse", "--short", "HEAD")


def recent_commits(n=10):
    raw = _git("log", f"--oneline", f"-{n}")
    return raw.splitlines() if raw else []


def parse_sprint_metrics():
    """Read delivery .sysml files and extract per-sprint estimatedPoints + actualHours."""
    delivery = ROOT / ".tracking" / "delivery"
    sprints = []
    for f in sorted(delivery.glob("*.sysml")):
        text = f.read_text(encoding="utf-8", errors="replace")
        # Only files that contain a Story (have estimatedPoints)
        pts_m = re.search(r":>>\s+estimatedPoints\s*=\s*(\d+)", text)
        if not pts_m:
            continue
        title_m = re.search(r":>>\s+title\s*=\s*\"([^\"]{0,100})\"", text)
        hrs_m = re.search(r":>>\s+actualHours\s*=\s*([\d.]+)", text)
        sprints.append({
            "file": f.stem,
            "title": title_m.group(1) if title_m else f.stem,
            "pts": int(pts_m.group(1)),
            "hours": float(hrs_m.group(1)) if hrs_m else None,
        })
    return sprints


def orient_data():
    """Run sysmlv2 orient and parse the JSON output."""
    cli = ROOT / "target" / "release" / "sysmlv2.exe"
    default = {"cursor": {}, "ready": [], "suspect": [], "counts": {"done": "?", "outstanding": "?"}}
    if not cli.exists():
        default["_fallback"] = True
        return default
    r = subprocess.run([str(cli), "orient", str(ROOT)], capture_output=True, text=True)
    try:
        return json.loads(r.stdout)
    except Exception:
        default["_fallback"] = True
        return default


def ready_names(orient_json):
    """Extract short names from ready list, filtering out inline SysML blobs."""
    items = []
    for item in orient_json.get("ready", []):
        # Skip verbose DoR inline definitions (contain ":>> procedureText")
        if ":>> procedureText" in item:
            continue
        # Take first token before any space or SysML syntax
        name = item.split(" : ")[0].split("{")[0].strip()
        if name:
            items.append(name)
    return items


def recent_decisions(n=10):
    dec_dir = ROOT / ".engine" / "decisions"
    results = []
    for f in sorted(dec_dir.glob("*.sysml"), reverse=True)[:n]:
        text = f.read_text(encoding="utf-8", errors="replace")
        title_m = re.search(r":>>\s+title\s*=\s*\"([^\"]{0,150})\"", text)
        results.append({
            "num": f.stem[:4],
            "title": title_m.group(1) if title_m else f.stem,
        })
    return results


def velocity_stats(sprints, window=3):
    complete = [s for s in sprints if s["hours"] is not None]
    if not complete:
        return {"trailing_pts": None, "trailing_eff": None, "window": window}
    recent = complete[-window:]
    avg_pts = sum(s["pts"] for s in recent) / len(recent)
    eff_list = [s["pts"] / s["hours"] for s in recent if s["hours"] > 0]
    avg_eff = sum(eff_list) / len(eff_list) if eff_list else None
    return {"trailing_pts": round(avg_pts, 2), "trailing_eff": round(avg_eff, 3) if avg_eff else None, "window": window}


# ---------------------------------------------------------------------------
# HTML rendering
# ---------------------------------------------------------------------------

CSS = """
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
       background: #0d1117; color: #c9d1d9; font-size: 14px; }
header { background: #161b22; border-bottom: 1px solid #30363d;
         padding: 16px 24px; display: flex; justify-content: space-between; align-items: center; }
header h1 { font-size: 18px; color: #58a6ff; font-weight: 600; }
header .meta { color: #8b949e; font-size: 12px; }
.grid { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; padding: 20px 24px; }
.card { background: #161b22; border: 1px solid #30363d; border-radius: 8px;
        padding: 16px; }
.card.full { grid-column: 1 / -1; }
.card h2 { font-size: 13px; font-weight: 600; color: #8b949e;
           text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: 12px; }
.badge { display: inline-block; padding: 2px 8px; border-radius: 12px;
         font-size: 11px; font-weight: 600; margin-right: 6px; }
.badge.green  { background: #1a3a1a; color: #3fb950; border: 1px solid #238636; }
.badge.blue   { background: #0d2b4e; color: #58a6ff; border: 1px solid #1f6feb; }
.badge.orange { background: #3b2300; color: #d29922; border: 1px solid #9e6a03; }
.badge.gray   { background: #21262d; color: #8b949e; border: 1px solid #30363d; }
.kv { display: flex; justify-content: space-between; padding: 4px 0;
      border-bottom: 1px solid #21262d; }
.kv:last-child { border-bottom: none; }
.kv .key { color: #8b949e; }
.kv .val { color: #e6edf3; font-weight: 500; }
.kv .val.dim { color: #8b949e; }
table { width: 100%; border-collapse: collapse; }
th { text-align: left; color: #8b949e; font-size: 11px; font-weight: 600;
     text-transform: uppercase; letter-spacing: 0.06em; padding: 6px 8px;
     border-bottom: 1px solid #30363d; }
td { padding: 6px 8px; border-bottom: 1px solid #21262d; color: #c9d1d9; }
tr:last-child td { border-bottom: none; }
td.num { font-family: monospace; text-align: right; }
td.dim { color: #8b949e; }
td.good { color: #3fb950; }
td.warn { color: #d29922; }
.commit { font-family: monospace; font-size: 12px; padding: 3px 0;
          border-bottom: 1px solid #21262d; white-space: nowrap; overflow: hidden;
          text-overflow: ellipsis; }
.commit:last-child { border-bottom: none; }
.commit .sha { color: #58a6ff; }
.dec-row { display: flex; gap: 10px; padding: 4px 0; border-bottom: 1px solid #21262d; }
.dec-row:last-child { border-bottom: none; }
.dec-num { color: #58a6ff; font-family: monospace; font-weight: 600; min-width: 48px; }
.dec-title { color: #c9d1d9; font-size: 12px; white-space: nowrap;
             overflow: hidden; text-overflow: ellipsis; }
.ready-item { display: inline-block; background: #21262d; border: 1px solid #30363d;
              border-radius: 4px; padding: 3px 8px; margin: 3px 3px 3px 0;
              font-size: 12px; color: #58a6ff; font-family: monospace; }
footer { text-align: center; padding: 16px; color: #484f58; font-size: 11px;
         border-top: 1px solid #30363d; }
.phase-chip { background: #1a2d4e; border: 1px solid #1f6feb; border-radius: 4px;
              padding: 2px 10px; font-size: 12px; color: #79c0ff; font-weight: 500; }
"""

def esc(s):
    return str(s).replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;").replace('"', "&quot;")


def render_html(sprints, orient, commits, decisions, generated_at, sha):
    o = orient
    cursor = o.get("cursor", {})
    counts = o.get("counts", {})
    done = counts.get("done", "?")
    outstanding = counts.get("outstanding", "?")
    suspect_count = len(o.get("suspect", []))
    active_workflow = cursor.get("activeWorkflow", "—")
    active_phase = cursor.get("activePhase", "—")
    ready_items = ready_names(o)

    # Active sprint (last sprint file with estimatedPoints)
    active = sprints[-1] if sprints else None
    v3 = velocity_stats(sprints, 3)
    v5 = velocity_stats(sprints, 5)

    def eff_fmt(pts, hours):
        if hours is None or hours == 0:
            return '<td class="num dim">—</td>'
        e = pts / hours
        cls = "good" if e >= 0.4 else "warn"
        return f'<td class="num {cls}">{e:.2f}</td>'

    # Sprint table rows
    sprint_rows = ""
    for s in sprints[-15:]:
        hrs_cell = f'{s["hours"]:.1f}' if s["hours"] is not None else '<span style="color:#484f58">—</span>'
        sprint_rows += (
            f'<tr><td style="font-family:monospace;font-size:12px">{esc(s["file"][:28])}</td>'
            f'<td class="num">{s["pts"]}</td>'
            f'<td class="num">{hrs_cell}</td>'
            f'{eff_fmt(s["pts"], s["hours"])}</tr>'
        )

    # Velocity trailing averages
    def avg_row(label, v):
        if v["trailing_pts"] is None:
            return f'<div class="kv"><span class="key">{label}</span><span class="val dim">no data</span></div>'
        eff_str = f'{v["trailing_eff"]:.3f} pts/h' if v["trailing_eff"] else "—"
        return (
            f'<div class="kv"><span class="key">{label}</span>'
            f'<span class="val">{v["trailing_pts"]} pts avg &nbsp;·&nbsp; {eff_str}</span></div>'
        )

    # Active sprint card content
    if active:
        hrs_disp = f'{active["hours"]:.1f} h' if active["hours"] is not None else '<span class="dim">not yet recorded</span>'
        eff_disp = f'{active["pts"]/active["hours"]:.3f} pts/h' if active["hours"] else '<span class="dim">—</span>'
        active_html = (
            f'<div class="kv"><span class="key">Sprint</span><span class="val">{esc(active["file"])}</span></div>'
            f'<div class="kv"><span class="key">Estimated points</span><span class="val">{active["pts"]}</span></div>'
            f'<div class="kv"><span class="key">Actual hours</span><span class="val">{hrs_disp}</span></div>'
            f'<div class="kv"><span class="key">Efficiency</span><span class="val">{eff_disp}</span></div>'
        )
    else:
        active_html = '<div class="kv"><span class="key">No sprint data found</span></div>'

    # Decision rows
    dec_rows = "".join(
        f'<div class="dec-row"><span class="dec-num">D{d["num"]}</span>'
        f'<span class="dec-title">{esc(d["title"][:100])}</span></div>'
        for d in decisions
    )

    # Ready items
    ready_html = "".join(f'<span class="ready-item">{esc(r)}</span>' for r in ready_items) or '<span class="dim">none</span>'

    # Commit rows
    commit_rows = ""
    for c in commits:
        parts = c.split(" ", 1)
        sha_part = parts[0] if parts else ""
        msg_part = esc(parts[1]) if len(parts) > 1 else ""
        commit_rows += f'<div class="commit"><span class="sha">{sha_part}</span> {msg_part}</div>'

    fallback_note = '<p style="color:#d29922;font-size:11px;margin-top:8px">⚠ sysmlv2.exe not found — counts from direct file parsing</p>' if o.get("_fallback") else ""

    return f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>SysML v2 AI Toolkit — Status Dashboard</title>
<style>{CSS}</style>
</head>
<body>
<header>
  <h1>SysML v2 AI Toolkit &mdash; Status Dashboard</h1>
  <div class="meta">Generated {esc(generated_at)} &nbsp;·&nbsp; HEAD <code style="color:#58a6ff">{esc(sha)}</code></div>
</header>

<div class="grid">

  <!-- Cursor -->
  <div class="card">
    <h2>Active Cursor</h2>
    <div class="kv"><span class="key">Workflow</span><span class="val">{esc(active_workflow)}</span></div>
    <div class="kv"><span class="key">Phase</span><span class="val"><span class="phase-chip">{esc(active_phase)}</span></span></div>
    <div class="kv"><span class="key">Entered at</span><span class="val">{esc(cursor.get("enteredAt","—"))}</span></div>
    <div class="kv"><span class="key">Entered by</span><span class="val">{esc(cursor.get("enteredBy","—"))}</span></div>
    {fallback_note}
  </div>

  <!-- Backlog counts -->
  <div class="card">
    <h2>Backlog Status</h2>
    <div style="margin-bottom:12px">
      <span class="badge green">Done {done}</span>
      <span class="badge blue">Outstanding {outstanding}</span>
      <span class="badge {'orange' if suspect_count else 'gray'}">Suspect {suspect_count}</span>
    </div>
    <h2 style="margin-top:4px">Ready frontier</h2>
    <div style="margin-top:6px">{ready_html}</div>
  </div>

  <!-- Active sprint -->
  <div class="card">
    <h2>Active Sprint</h2>
    {active_html}
  </div>

  <!-- Velocity averages -->
  <div class="card">
    <h2>Velocity &amp; Efficiency</h2>
    {avg_row("Trailing 3-sprint avg", v3)}
    {avg_row("Trailing 5-sprint avg", v5)}
    <div class="kv" style="margin-top:8px"><span class="key" style="font-size:11px">
      Efficiency = estimatedPoints / actualHours<br>
      <span style="color:#484f58">Higher = more points per hour</span>
    </span></div>
  </div>

  <!-- Sprint history table -->
  <div class="card full">
    <h2>Sprint History ({len(sprints)} sprints with point estimates)</h2>
    <table>
      <thead><tr>
        <th>Sprint</th><th style="text-align:right">Est Pts</th>
        <th style="text-align:right">Actual Hours</th>
        <th style="text-align:right">Efficiency (pts/h)</th>
      </tr></thead>
      <tbody>{sprint_rows}</tbody>
    </table>
  </div>

  <!-- Recent decisions -->
  <div class="card">
    <h2>Recent Decisions (last 10)</h2>
    {dec_rows}
  </div>

  <!-- Recent commits -->
  <div class="card">
    <h2>Recent Activity (last 10 commits)</h2>
    {commit_rows}
  </div>

</div>
<footer>
  Generated by <code>.engine/tools/report.py</code> &mdash;
  <strong>derived VIEW</strong>, not authored content (D0015, D0040).
  Regenerate at any time; do not commit as truth.
</footer>
</body>
</html>"""


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="Generate HTML project status dashboard")
    parser.add_argument("--output", "-o", default=None, help="Write HTML to this file")
    parser.add_argument("--stdout", action="store_true", help="Print HTML to stdout")
    args = parser.parse_args()

    generated_at = datetime.now().strftime("%Y-%m-%d %H:%M")
    sha = head_sha()
    sprints = parse_sprint_metrics()
    orient = orient_data()
    commits = recent_commits(10)
    decisions = recent_decisions(10)

    html = render_html(sprints, orient, commits, decisions, generated_at, sha)

    if args.stdout:
        print(html)
        return

    out_path = Path(args.output) if args.output else ROOT / "status.html"
    out_path.write_text(html, encoding="utf-8")
    print(f"[report.py] Dashboard written to: {out_path}")
    print(f"            {len(sprints)} sprints · HEAD {sha}")

    # Try to open in browser
    try:
        import webbrowser
        webbrowser.open(out_path.as_uri())
    except Exception:
        pass


if __name__ == "__main__":
    main()
