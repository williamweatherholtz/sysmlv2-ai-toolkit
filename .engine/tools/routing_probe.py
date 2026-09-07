#!/usr/bin/env python3
"""Does a skill's prose actually ROUTE the agent? Measured, not assumed (dcSkillsRouteBehaviourally).

THE GAP THIS CLOSES. This project has 64 guards over the model and none over the thing the model is
supposed to cause: an agent reading a process's prose and invoking the process. Every claim that a
skill "routes" has until now been an assumption about text nobody tested. This runs a real model
against one realistic request per process and reads the transcript for what it actually did.

WHAT IS MEASURED, per process:
  routed     the intended skill was invoked
  wrong      a DIFFERENT skill was invoked first - worse than silence, because the agent proceeded
             confidently under the wrong procedure
  none       no skill was invoked within the turn budget
  error      the run itself failed (reported, never counted as a verdict)

HOW IT IS RUN. `claude -p` with `--output-format stream-json`, so the tool_use blocks are readable
without trusting anything the model SAYS about itself. Writes are disallowed: the probe measures
routing, and a probe that can edit the repository it is measuring is not a probe. The turn budget is
small and explicit, because "would it have got there eventually" is a different question from "does
the situation reach the skill".

WHAT THIS COSTS, and why that matters here. Each run is a real model call - measured at roughly half a
dollar and twenty-five seconds on this host. A full sweep is therefore not free and is NOT wired to a
commit; the number belongs on a schedule, tracked as an indicator (D0088), because no defensible
threshold for "share of skills that route" exists yet and gating on one would invite writing prompts
that pass rather than prompts that are realistic.

Usage:
  python .engine/tools/routing_probe.py [--only NAME[,NAME...]] [--limit N] [--turns N] [--model M]
  python .engine/tools/routing_probe.py --dry-run      # what it would run, and the estimated cost
"""
import argparse
import concurrent.futures as cf
import json
import os
import re
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
PROMPTS = HERE / "routing_prompts.toml"
OUT = REPO / ".keel" / "metrics" / "routing.json"

# measured on this host, 2026-09-06: one probe run, one turn, no writes
COST_PER_RUN_USD = 0.52
SECONDS_PER_RUN = 25


# The Stop hook runs validate + every guard at each turn boundary - measured at 14-31s in the fire
# ledger. It cannot influence which skill the model chose (it fires after the turn), but it CAN block
# and force another turn, which would score a gate-driven skill call as a route. Silenced for the
# probe only, by a settings file the probe writes, never by touching the repository's own settings.
PROBE_SETTINGS = REPO / ".keel" / "metrics" / "probe-settings.json"


def probe_settings():
    """A settings file that empties the Stop hook for probe sessions only."""
    PROBE_SETTINGS.parent.mkdir(parents=True, exist_ok=True)
    PROBE_SETTINGS.write_text(json.dumps({"hooks": {"Stop": [], "SubagentStop": []}}), encoding="utf-8")
    return str(PROBE_SETTINGS)


def claude_exe():
    """The Claude Code entry point this host can actually spawn.

    `claude` on PATH is a shell shim on Windows: git-bash runs it, CreateProcess does not (WinError 2
    for a file that exists). Prefer the platform-executable form.
    """
    import shutil

    for candidate in ("claude.cmd", "claude.exe", "claude"):
        found = shutil.which(candidate)
        if found:
            return found
    sys.exit("claude CLI not found on PATH - the probe measures a real model and cannot proceed")


def load_cases():
    """The authored case set: name, prompt, and the skills that would each be a correct route.

    Hand-parsed rather than via a TOML library so the tool stays stdlib-only, like every other
    validation-path tool here (D0048).
    """
    text = PROMPTS.read_text(encoding="utf-8")
    cases, cur = [], None
    for line in text.splitlines():
        line = line.strip()
        if line.startswith("#"):
            continue
        if line == "[[case]]":
            if cur and "prompt" in cur:
                cases.append(cur)
            cur = {}
            continue
        if cur is None:
            continue
        m = re.match(r'^(name|prompt|expect)\s*=\s*"(.*)"$', line)
        if m:
            cur[m.group(1)] = m.group(2)
    if cur and "prompt" in cur:
        cases.append(cur)
    if not cases:
        sys.exit(f"no cases parsed from {PROMPTS}")
    for c in cases:
        expect = c.get("expect") or c["name"]
        c["accept"] = [x.strip() for x in expect.split(",") if x.strip()]
    return {c["name"]: c for c in cases}


def deployed_skills():
    d = REPO / ".claude" / "skills"
    return {p.name for p in d.iterdir() if p.is_dir()} if d.is_dir() else set()


def run_one(accept, prompt, turns, model):
    """One headless run. Returns (verdict, skill_seen, tools, seconds, cost, note)."""
    cmd = [
        claude_exe(), "-p", prompt,
        "--output-format", "stream-json", "--verbose",
        "--max-turns", str(turns),
        "--settings", probe_settings(),
        "--disallowed-tools", "Write", "Edit", "NotebookEdit", "Bash",
    ]
    if model:
        cmd += ["--model", model]
    started = time.time()
    try:
        proc = subprocess.run(cmd, capture_output=True, text=True, timeout=240, cwd=str(REPO))
    except subprocess.TimeoutExpired:
        return "error", None, [], time.time() - started, 0.0, "timed out after 240s"

    skills, tools, cost = [], [], 0.0
    for line in proc.stdout.splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            ev = json.loads(line)
        except json.JSONDecodeError:
            continue
        if ev.get("type") == "assistant":
            for block in ev.get("message", {}).get("content", []):
                if block.get("type") == "tool_use":
                    tool = block.get("name", "")
                    tools.append(tool)
                    if tool == "Skill":
                        skills.append(str(block.get("input", {}).get("skill", "")))
        elif ev.get("type") == "result":
            cost = float(ev.get("total_cost_usd") or 0.0)

    seconds = time.time() - started
    if proc.returncode != 0 and not tools:
        return "error", None, tools, seconds, cost, (proc.stderr or "")[:160]
    if not skills:
        return "none", None, tools, seconds, cost, ""
    first = skills[0].split(":")[-1]
    if first in accept:
        return "routed", first, tools, seconds, cost, ""
    return "wrong", first, tools, seconds, cost, ""


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--only", default="", help="comma-separated process names")
    ap.add_argument("--limit", type=int, default=0, help="stop after N runs")
    ap.add_argument("--turns", type=int, default=3, help="turn budget per run")
    ap.add_argument("--model", default="", help="model to measure (default: the CLI's own)")
    ap.add_argument("--jobs", type=int, default=4, help="probe sessions to run at once")
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    cases = load_cases()
    deployed = deployed_skills()

    # A prompt for a skill that is not deployed measures nothing; a deployed skill with no prompt is
    # UNMEASURED and must be said out loud rather than omitted from the denominator.
    names = sorted(cases)
    if args.only:
        wanted = {n.strip() for n in args.only.split(",") if n.strip()}
        names = [n for n in names if n in wanted]
    if args.limit:
        names = names[: args.limit]

    missing_prompt = sorted(deployed - set(cases))
    stale_prompt = sorted(set(cases) - deployed)

    if args.dry_run:
        print(f"would run {len(names)} probe(s) at {args.turns} turn(s)")
        print(f"estimated cost ~${len(names) * COST_PER_RUN_USD:.2f}, "
              f"~{len(names) * SECONDS_PER_RUN // 60} min wall (measured per-run: "
              f"${COST_PER_RUN_USD}, {SECONDS_PER_RUN}s)")
        print(f"deployed skills with NO prompt (unmeasured): {len(missing_prompt)}")
        for n in missing_prompt:
            print(f"  - {n}")
        if stale_prompt:
            # An active process with no deployed skill cannot route at all. Reported, never hidden.
            print(f"cases naming a skill that is NOT deployed: {', '.join(stale_prompt)}")
        return 0

    rows, spend = [], 0.0
    probe_settings()  # written once, before any worker reads it
    done = 0
    with cf.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        futures = {
            pool.submit(run_one, cases[n]["accept"], cases[n]["prompt"], args.turns, args.model): n
            for n in names
        }
        for fut in cf.as_completed(futures):
            name = futures[fut]
            verdict, seen, tools, secs, cost, note = fut.result()
            spend += cost
            done += 1
            rows.append({"process": name, "verdict": verdict, "skillInvoked": seen,
                         "tools": tools[:8], "seconds": round(secs, 1),
                         "costUsd": round(cost, 4), "note": note})
            mark = {"routed": "ROUTED", "wrong": "WRONG ", "none": "none  ", "error": "ERROR "}[verdict]
            extra = f" -> {seen}" if verdict == "wrong" else (f"  {note}" if note else "")
            print(f"{done:>3}/{len(names)} {mark} {name:28s} {secs:5.1f}s ${cost:.3f}{extra}", flush=True)
    rows.sort(key=lambda r: r["process"])

    tally = {v: sum(1 for r in rows if r["verdict"] == v) for v in ("routed", "wrong", "none", "error")}
    measured = tally["routed"] + tally["wrong"] + tally["none"]
    print("-" * 78)
    print(f"routed {tally['routed']}/{measured} measured "
          f"(wrong {tally['wrong']}, silent {tally['none']}, errored {tally['error']})")
    print(f"UNMEASURED deployed skills (no prompt authored): {len(missing_prompt)}")
    print(f"spend this run: ${spend:.2f} over {len(names)} run(s) at {args.turns} turn(s)")

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps({
        "ranAt": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "model": args.model or "cli default",
        "turns": args.turns,
        "routed": tally["routed"],
        "wrong": tally["wrong"],
        "none": tally["none"],
        "errored": tally["error"],
        "measured": measured,
        "unmeasuredDeployedSkills": missing_prompt,
        "spendUsd": round(spend, 2),
        "rows": rows,
    }, indent=2), encoding="utf-8")
    print(f"wrote {OUT.relative_to(REPO)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
