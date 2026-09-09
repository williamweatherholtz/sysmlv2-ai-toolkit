#!/usr/bin/env python3
"""Does a skill's prose actually ROUTE the agent? An INVESTIGATION rig, not a measurement (D0378 C).

THE GAP THIS CLOSES. This project has 64 guards over the model and none over the thing the model is
supposed to cause: an agent reading a process's prose and invoking the process. Every claim that a
skill "routes" was an assumption about text nobody tested. This runs a real model against one
realistic request for a NAMED process, several times, and reads the transcripts for what it did.

WHAT IS READ, per sample - the WHOLE turn, not its first Skill call (issue415):
  routed     a skill in the case's accept list was invoked within the turn budget, and every Skill
             call before it was intake (recorded as `prefix`, printed beside the verdict)
  wrong      a skill that is neither accepted nor intake was invoked - worse than silence, because the
             agent proceeded confidently under the wrong procedure, whether or not the intended skill
             followed it
  none       the intended skill never appeared (an intake-only turn is `none` with its prefix shown)
  error      the run itself failed (reported, never read as a verdict)

WHY INTAKE IS THE ONE DECLARED PREFIX. The rig's first investigation (2026-09-08) read the release case
as 1 routed / 2 wrong, and both wrong samples had invoked intake first and release second. Intake's own
deployed description declares that position - `.claude/skills/intake/SKILL.md:3`: "Use at the START of
any turn where the human states direction" - and every prompt in routing_prompts.toml is phrased as a
person stating direction. An agent that runs intake and then the intended skill is following the
process exactly as written, so scoring its first call was scoring the process as a wrong route. No
other skill declares a start-of-turn position, so no other skill is a prefix: a different skill first
stays wrong. `--probe` runs the pure scorer on the four transcripts this rule must read correctly
(D0388) before any model is paid for.

WHAT THIS IS NOT. There is no standing coverage number and no whole-set sweep. On 2026-09-06 two of
four verdicts flipped between two runs of identical prompts (issue392), so one sample per skill was
never a measurement, and the human declined to buy more samples to average the noise away: a check
that disagrees with itself is a defect of the CHECK - the prompt, the skill's description - to record
as an Issue against it, never a rate (D0381). What survives is the re-run as an INVESTIGATION (D0382,
their words: "repeated sampling should be an option though ... it's a different process - ultimately
they can't really verify"): started when a skill is suspected of not routing, run three times before
and after any rewrite, its output FINDINGS recorded as evidence on the item that motivated it. It
writes no TestResult, no indicator and no percentage, and a single sample is reported as a
suspicion, never a finding.

WHAT IT COSTS. Each sample is a real model call. The cost is read from the model's own result event
and recorded beside the samples; the dry-run estimate quotes the last measured run, or the constant
below with the date it was measured.

Usage:
  python .engine/tools/routing_probe.py --investigate NAME[,NAME...] [--samples N] [--turns N] [--model M]
  python .engine/tools/routing_probe.py --investigate NAME --dry-run     # what it would run, and the estimate
  python .engine/tools/routing_probe.py --probe                          # the scorer on its known transcripts
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
OUT = REPO / ".keel" / "metrics" / "routing-investigation.json"

# measured on this host, 2026-09-06: one probe run, one turn, no writes - superseded by the last
# investigation's own receipt whenever one exists
COST_PER_RUN_USD = 0.52
SECONDS_PER_RUN = 25
MEASURED_ON = "2026-09-06"

# fewer than this many samples cannot ground a finding: one sample flipped on identical inputs (issue392)
MIN_SAMPLES_FOR_FINDING = 2

# The skills whose own description declares they run at the START of a turn where the human states
# direction (.claude/skills/intake/SKILL.md:3). A call to one of these before the intended skill is a
# prefix, not a wrong route. Nothing else declares that position.
DECLARED_PREFIX = ("intake",)


def short(skill):
    """`plugin:name` and `name` both score as `name`."""
    return skill.split(":")[-1]


def score(skills, accept):
    """The verdict over EVERY Skill call of a turn, in order -> (verdict, skill_seen, prefix).

    routed: an accepted skill appears and everything before it is a declared prefix; skill_seen is the
    accepted name, prefix the calls before it. wrong: a skill that is neither accepted nor a declared
    prefix appears anywhere; skill_seen is the first such name. none: no accepted skill and nothing
    wrong - an empty turn, or an intake-only turn (prefix shows it). Pure, so `--probe` can read it.
    """
    prefix = []
    for name in (short(s) for s in skills):
        if name in accept:
            return "routed", name, prefix
        if name in DECLARED_PREFIX:
            prefix.append(name)
            continue
        return "wrong", name, prefix
    return "none", None, prefix


def probe():
    """The scorer against the transcripts its rule must read correctly - known before any run (D0388)."""
    cases = [
        ("intake then release = routed with prefix", ["intake", "release"], ["release"], ("routed", "release", ["intake"])),
        ("release alone = routed", ["release"], ["release"], ("routed", "release", [])),
        ("stpa then release = wrong (stpa declares no prefix position)", ["stpa", "release"], ["release"], ("wrong", "stpa", [])),
        ("nothing = none", [], ["release"], ("none", None, [])),
        ("intake alone = none, prefix shown", ["intake"], ["release"], ("none", None, ["intake"])),
        ("intake then stpa, release never = wrong", ["intake", "stpa"], ["release"], ("wrong", "stpa", ["intake"])),
        ("a namespaced call scores by its short name", ["projectSettings:intake", "projectSettings:release"], ["release"], ("routed", "release", ["intake"])),
    ]
    ok = True
    for what, skills, accept, want in cases:
        got = score(skills, accept)
        held = got == want
        ok &= held
        print(f"[probe] {'PASS' if held else 'FAIL'} {what}\n        -> {got}" + ("" if held else f" wanted {want}"))
    print(f"[probe] {'all %d cases hold' % len(cases) if ok else 'A CASE FAILED - the verdict is not trusted'}")
    return ok


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
    """One headless run. Returns (verdict, skill_seen, prefix, skills, tools, seconds, cost, note).

    `skills` is EVERY Skill call in order and the verdict reads all of them (`score`): intake before the
    intended skill is a prefix, a different skill anywhere is wrong (issue415).
    """
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
        return "error", None, [], [], [], time.time() - started, 0.0, "timed out after 240s"

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
        return "error", None, [], skills, tools, seconds, cost, (proc.stderr or "")[:160]
    verdict, seen, prefix = score(skills, accept)
    return verdict, seen, prefix, skills, tools, seconds, cost, ""


def last_measured():
    """The per-run cost and seconds the last investigation actually paid, if one was recorded."""
    try:
        prev = json.loads(OUT.read_text(encoding="utf-8"))
        m = prev.get("measured") or {}
        if m.get("perRunUsd") and m.get("perRunSeconds"):
            return float(m["perRunUsd"]), float(m["perRunSeconds"]), prev.get("ranAt", "")[:10]
    except (OSError, ValueError):
        pass
    return COST_PER_RUN_USD, SECONDS_PER_RUN, MEASURED_ON


def finding(name, verdicts, samples):
    """One line saying what the samples of a case established - and, when they cannot, saying so.

    A single sample is a suspicion (issue392: one flipped on identical inputs). Samples that disagree
    are a defect of the check, never averaged (D0381). Only unanimous samples ground a finding.
    """
    read = [v for v in verdicts if v != "error"]
    errored = len(verdicts) - len(read)
    if not read:
        return f"ERROR      {name}: every sample errored - nothing established"
    if len(read) < MIN_SAMPLES_FOR_FINDING:
        v = read[0]
        if v == "routed":
            return f"1 sample   {name}: routed once - a single sample; run --samples {samples if samples > 1 else 3} to establish it"
        return (f"SUSPICION  {name}: {v} on ONE sample - not a finding of not-routing (issue392); "
                f"re-run with --samples 3")
    counts = {v: read.count(v) for v in ("routed", "wrong", "none") if read.count(v)}
    tail = f" ({errored} errored)" if errored else ""
    if len(counts) == 1:
        v, n = next(iter(counts.items()))
        if v == "routed":
            return f"ROUTED     {name}: {n}/{n} samples agree{tail}"
        return (f"NOT ROUTING {name}: {n}/{n} samples {v}{tail} - a finding: record an Issue "
                f"carrying the prompt that missed (dcSkillsRouteBehaviourally VERIFY)")
    spread = ", ".join(f"{n} {v}" for v, n in counts.items())
    return (f"DISAGREES  {name}: {spread}{tail} - the CHECK is the defect (D0381): record an Issue "
            f"against this case's prompt or the skill's description; never average these")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--investigate", default="",
                    help="comma-separated case names to investigate (required: there is no whole-set sweep, D0378 C)")
    ap.add_argument("--samples", type=int, default=3, help="runs per case (default 3; 1 is a suspicion, never a finding)")
    ap.add_argument("--turns", type=int, default=3, help="turn budget per run")
    ap.add_argument("--model", default="", help="model to run (default: the CLI's own)")
    ap.add_argument("--jobs", type=int, default=4, help="probe sessions to run at once")
    ap.add_argument("--dry-run", action="store_true")
    ap.add_argument("--probe", action="store_true", help="run the scorer on its known transcripts and exit (D0388)")
    args = ap.parse_args()
    if args.probe:
        return 0 if probe() else 1

    if not args.investigate:
        sys.exit("refusing: name the case(s) to investigate with --investigate NAME[,NAME...]. "
                 "There is no whole-set sweep and no standing routing number (D0378 C, the human's "
                 "choice 2026-09-08): one sample per skill flipped on identical inputs (issue392), and "
                 "a check that disagrees with itself is a defect to critique, not a rate to buy (D0381).")
    if args.samples < 1:
        sys.exit("refusing: --samples must be at least 1")

    cases = load_cases()
    deployed = deployed_skills()
    names = [n.strip() for n in args.investigate.split(",") if n.strip()]
    unknown = [n for n in names if n not in cases]
    if unknown:
        sys.exit(f"no authored case named {', '.join(unknown)} in {PROMPTS.relative_to(REPO)}; "
                 f"known: {', '.join(sorted(cases))}")
    undeployed = [n for n in names if n not in deployed]

    per_usd, per_sec, measured_on = last_measured()
    runs = len(names) * args.samples
    if args.dry_run:
        print(f"would run {len(names)} case(s) x {args.samples} sample(s) = {runs} run(s) at {args.turns} turn(s)")
        print(f"estimated cost ~${runs * per_usd:.2f}, ~{runs * per_sec / 60:.0f} min wall "
              f"(per run ${per_usd:.2f}, {per_sec:.0f}s, measured {measured_on})")
        if undeployed:
            # An active process with no deployed skill cannot route at all. Reported, never hidden.
            print(f"cases naming a skill that is NOT deployed: {', '.join(undeployed)}")
        return 0
    if undeployed:
        print(f"note: {', '.join(undeployed)} name(s) a skill that is not deployed - it cannot route; "
              f"the samples will say so", flush=True)

    if not probe():
        sys.exit(2)   # the scorer failed a transcript it must read correctly; no model is paid for
    print(f"investigating {', '.join(names)}: {args.samples} sample(s) each at {args.turns} turn(s); "
          f"estimate ~${runs * per_usd:.2f} from the {measured_on} measurement", flush=True)
    if OUT.exists():
        # D0387: the previous investigation is gone before this one starts, so a dead run leaves nothing.
        # Removed HERE, after the estimate read it and after a dry run has returned: unlinking first made
        # every estimate fall back to the constant and every dry run erase the last receipt (issue431).
        OUT.unlink()
    probe_settings()  # written once, before any worker reads it
    rows, spend, wall_started, done = {n: [] for n in names}, 0.0, time.time(), 0
    with cf.ThreadPoolExecutor(max_workers=args.jobs) as pool:
        futures = {}
        for n in names:
            for i in range(args.samples):
                fut = pool.submit(run_one, cases[n]["accept"], cases[n]["prompt"], args.turns, args.model)
                futures[fut] = (n, i + 1)
        for fut in cf.as_completed(futures):
            name, i = futures[fut]
            verdict, seen, prefix, skills, tools, secs, cost, note = fut.result()
            spend += cost
            done += 1
            rows[name].append({"sample": i, "verdict": verdict, "skillInvoked": seen, "prefix": prefix,
                               "skillsInvoked": skills, "tools": tools[:8], "seconds": round(secs, 1),
                               "costUsd": round(cost, 4), "note": note})
            mark = {"routed": "ROUTED", "wrong": "WRONG ", "none": "none  ", "error": "ERROR "}[verdict]
            extra = f" -> {' then '.join(skills)}" if verdict == "wrong" else (f"  {note}" if note else "")
            if prefix:
                extra = f" prefix {' then '.join(prefix)}" + extra
            print(f"{done:>3}/{runs} {mark} {name:28s} #{i} {secs:5.1f}s ${cost:.3f}{extra}", flush=True)
    wall = time.time() - wall_started

    print("-" * 78)
    findings = {}
    for n in names:
        rows[n].sort(key=lambda r: r["sample"])
        verdicts = [r["verdict"] for r in rows[n]]
        findings[n] = finding(n, verdicts, args.samples)
        print(findings[n])
    paid = [r["costUsd"] for rs in rows.values() for r in rs if r["costUsd"] > 0]
    per_run_usd = round(spend / len(paid), 4) if paid else None
    per_run_sec = round(sum(r["seconds"] for rs in rows.values() for r in rs) / max(runs, 1), 1)
    print(f"spend this investigation: ${spend:.2f} over {runs} run(s), {wall:.0f}s wall "
          f"(measured per run: ${per_run_usd if per_run_usd is not None else 'n/a'}, {per_run_sec}s)")
    print("this is an investigation (D0382): findings above, no rate, no TestResult - record the receipt "
          "on the item that motivated it")

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps({
        "kind": "investigation",
        "why": "D0378 C / D0382: a re-run of named cases, findings only - no coverage rate, no standing number",
        "ranAt": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "model": args.model or "cli default",
        "turns": args.turns,
        "samplesPerCase": args.samples,
        "cases": {n: {"samples": rows[n], "finding": findings[n]} for n in names},
        "measured": {"spendUsd": round(spend, 2), "runs": runs, "wallSeconds": round(wall),
                     "perRunUsd": per_run_usd, "perRunSeconds": per_run_sec},
    }, indent=2), encoding="utf-8")
    print(f"wrote {OUT.relative_to(REPO)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
