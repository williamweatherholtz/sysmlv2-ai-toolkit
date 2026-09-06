#!/usr/bin/env python3
"""decision_facts.py - the COMPUTED facts file the standing executive summary is built from.

Run from the keel repository root, no required arguments. Prints ONE JSON object to stdout and
writes the same object beside this script as `decision-facts.json`.

Contract (why this exists): every number on the published summary must come from here, and every
value here carries its own provenance - `how` is the exact command, or the exact file + parsing
rule, that produced it. A fact that cannot be computed honestly is emitted with "value": null and
a `how` that says why. Nothing is ever guessed, and no answer is hardcoded.

Python 3 stdlib only; shells out to git, gh and ./target/release/keel.exe.
"""

import json
import os
import re
import subprocess
import sys
import time
from datetime import date, datetime, timedelta, timezone

# ---------------------------------------------------------------- infrastructure

REPO = os.getcwd()
KEEL = os.path.join(REPO, "target", "release", "keel.exe")
if not os.path.exists(KEEL):
    alt = os.path.join(REPO, "target", "release", "keel")
    KEEL = alt if os.path.exists(alt) else KEEL

TODAY = date.today()
NOW = datetime.now(timezone.utc)

FACTS = {}
NOTES = []


def fact(name, value, unit, how, as_of=None):
    """Record one fact. `value` may be None; `how` must then explain why."""
    FACTS[name] = {
        "value": value,
        "unit": unit,
        "as_of": as_of or TODAY.isoformat(),
        "how": how,
    }


def run(cmd, timeout=60):
    """Run a command, return (ok, stdout). Never raises; a failure becomes ok=False + the reason."""
    try:
        p = subprocess.run(cmd, cwd=REPO, capture_output=True, text=True,
                           timeout=timeout, encoding="utf-8", errors="replace")
        if p.returncode != 0 and not p.stdout.strip():
            return False, (p.stderr or "").strip()[:400] or ("exit %d" % p.returncode)
        return True, p.stdout
    except FileNotFoundError:
        return False, "executable not found: %s" % cmd[0]
    except subprocess.TimeoutExpired:
        return False, "timed out after %ss" % timeout
    except Exception as exc:                                        # pragma: no cover
        return False, "%s: %s" % (type(exc).__name__, exc)


def as_json(text):
    try:
        return json.loads(text)
    except Exception:
        return None


def read(path):
    try:
        with open(path, encoding="utf-8", errors="replace") as fh:
            return fh.read()
    except Exception:
        return None


def iso_min(a, b):
    """Minutes between two GitHub ISO timestamps."""
    fmt = "%Y-%m-%dT%H:%M:%SZ"
    return (datetime.strptime(b, fmt) - datetime.strptime(a, fmt)).total_seconds() / 60.0


# ---------------------------------------------------------------- tree identity

ok, out = run(["git", "rev-parse", "--short", "HEAD"])
TREE = out.strip() if ok else None

# ================================================================ 1. CLI SURFACE
# .engine/cli/commands.sysml is the authored CLI surface (D0271). One `part cli... : CliCommand`
# per command; a command whose `invocation` begins "show " is a lens rather than a top-level verb.

CLI_PATH = os.path.join(REPO, ".engine", "cli", "commands.sysml")
cli_src = read(CLI_PATH)
CLI_HOW = "parse .engine/cli/commands.sysml: "

if cli_src is None:
    for n in ("cliRecords", "cliShowLenses", "cliTopLevel", "cliDeprecated", "cliLive", "cliReadOnly",
              "cliLiveTopLevel", "cliLiveTopLevelReadOnly"):
        fact(n, None, "commands", CLI_HOW + "file not readable at %s" % CLI_PATH)
else:
    # one record per `part <name> : CliCommand { ... }`, body captured to its closing brace
    records = re.findall(r"part\s+cli\w*\s*:\s*CliCommand\s*\{(.*?)\}\s*$",
                         cli_src, re.MULTILINE | re.DOTALL)
    if not records:                                   # single-line form (the shape in this tree)
        records = [m.group(1) for m in re.finditer(r"part\s+cli\w*\s*:\s*CliCommand\s*\{([^\n]*)",
                                                   cli_src)]
    lenses = [b for b in records if re.search(r'invocation\s*=\s*"show\s', b)]
    deprecated = [b for b in records if "CliStability::deprecated" in b]
    reads = [b for b in records if "CliEffect::reads" in b]

    fact("cliRecords", len(records), "CliCommand records",
         CLI_HOW + "count of `part cli... : CliCommand` blocks")
    fact("cliShowLenses", len(lenses), "show lenses",
         CLI_HOW + 'records whose `invocation` begins "show " (D0271: a show invocation makes it a lens). '
                   'CROSS-CHECK: `keel show <bad-name>` prints a "Lenses:" hint listing 35 - it omits '
                   '`priority` and `control-structure`, both of which DO dispatch; the authored facts (and '
                   '`keel --help`, which renders from them) are the authority, so 37 is the number to publish')
    fact("cliTopLevel", len(records) - len(lenses), "top-level commands",
         CLI_HOW + "records minus the show-lens records")
    fact("cliDeprecated", len(deprecated), "commands",
         CLI_HOW + "records carrying CliStability::deprecated")
    fact("cliLive", len(records) - len(deprecated), "commands",
         CLI_HOW + "records minus the CliStability::deprecated ones")
    fact("cliReadOnly", len(reads), "commands",
         CLI_HOW + "records carrying CliEffect::reads (writes/both/tooling excluded)")
    # SAME-SCOPE pair for the page: both drawn from the top-level, non-deprecated population, so
    # they can be compared in one sentence. Mixing scopes published "72 of 69" (issue384).
    live_top = [b for b in records if b not in lenses and "CliStability::deprecated" not in b]
    fact("cliLiveTopLevel", len(live_top), "live top-level commands",
         CLI_HOW + "records that are neither a show lens nor deprecated - the names a reader meets")
    fact("cliLiveTopLevelReadOnly", len([b for b in live_top if "CliEffect::reads" in b]),
         "of those, read-only",
         CLI_HOW + "of the live top-level records, those carrying CliEffect::reads. SCOPE MATTERS: "
                   "cliReadOnly counts the whole surface including the show lenses, so the two must "
                   "never appear in one sentence")

# ================================================================ 2. GATING CALL SITES
# Mentions of a gating verb invoked through the binary, across git-TRACKED files.
# .tracking/ is excluded from the headline: it is recorded history and must never be rewritten,
# so a call site there is not a maintenance surface. Its count is reported separately.

GATING_VERBS = ["validate", "check-engine", "check", "guard", "gate", "rules",
                "audit-history", "audit-adherence", "audit-ci-runs", "audit",
                "assured", "adoption-check", "suite"]
# longest-first so `check-engine` never matches as `check`, `audit-history` never as `audit`
GATING_RE = re.compile(r"(?<![\w.-])(?:keel\.exe|keelw|keel)[ \t]+(?:" +
                       "|".join(GATING_VERBS) + r")(?![\w-])")
ESCAPE_RE = re.compile(r"\\[nrt]")   # a literal \n in a source string is a line break, not a letter
# the naive form a reviewer would reach for first: no word boundaries at either end.
NAIVE_RE = re.compile(r"(?:keel\.exe|keelw|keel)[ \t]+(?:" + "|".join(GATING_VERBS) + r")")

GATING_HOW = ("git ls-files, then for each tracked TEXT file count regex "
              r"`(?<![\w.-])(keel\.exe|keelw|keel)[ \t]+<verb>(?![\w-])` over the gating verbs "
              "(" + ", ".join(sorted(GATING_VERBS)) + "); alternation is longest-first so "
              "check-engine/audit-history/audit-adherence/audit-ci-runs never collapse into "
              "check/audit. Counts OCCURRENCES, not lines - a line with two invocations is two "
              r"call sites. Literal \n/\r/\t escapes are normalised to a space first, so a call "
              r"site embedded in a source string (`\nkeel validate` in view/control_structure.rs) "
              "is counted. ")

ok, out = run(["git", "ls-files"])
if not ok:
    for n in ("gatingCallSites", "gatingCallFiles", "gatingCallSitesHistory"):
        fact(n, None, "call sites", GATING_HOW + "`git ls-files` failed: " + out)
else:
    tracked = [p for p in out.splitlines() if p.strip()]
    live_sites = live_files = hist_sites = 0
    naive_live = naive_hist = 0
    for rel in tracked:
        text = read(os.path.join(REPO, rel.replace("/", os.sep)))
        if text is None or "\0" in text[:4096]:
            continue
        flat = ESCAPE_RE.sub(" ", text)
        n = len(GATING_RE.findall(flat))
        nn = len(NAIVE_RE.findall(flat))
        if rel.startswith(".tracking/"):
            hist_sites += n
            naive_hist += nn
        else:
            live_sites += n
            naive_live += nn
            if n:
                live_files += 1
    # both numbers, and which to trust - a reviewer's obvious grep disagrees, on purpose
    BOTH = ("BOTH NUMBERS: the same sweep WITHOUT the trailing word-boundary reports %d live and %d "
            "history. The %d/%d extra are English prose, not invocations - overwhelmingly 'keel "
            "gates every project the commit touches'. TRUST the bounded number published here; the "
            "naive one over-counts. "
            % (naive_live, naive_hist, naive_live - live_sites, naive_hist - hist_sites))
    fact("gatingCallSites", live_sites, "invocations in live (non-.tracking) tracked files",
         GATING_HOW + BOTH + "EXCLUDES .tracking/ (recorded history, never rewritten).")
    fact("gatingCallFiles", live_files, "tracked files carrying at least one",
         GATING_HOW + "distinct non-.tracking tracked files with >=1 bounded match.")
    fact("gatingCallSitesHistory", hist_sites, "invocations inside .tracking/ (history)",
         GATING_HOW + BOTH + "the EXCLUDED half, reported so the exclusion is visible rather than "
                             "silent. These are past sprint records and test-result evidence; "
                             "rewriting them would orphan evidence (D0129).")

# ================================================================ 3. DECISIONS (from the files)
# .engine/decisions/NNNN-slug.sysml, one Decision part per file. Status and createdAt are read
# from the DECISION part's own `:>> ...` assignments, not from prose that mentions them.

DEC_DIR = os.path.join(REPO, ".engine", "decisions")
DEC_HOW = "parse .engine/decisions/*.sysml: "
dec_files = sorted(f for f in os.listdir(DEC_DIR)) if os.path.isdir(DEC_DIR) else []
dec_files = [f for f in dec_files if f.endswith(".sysml")]

decisions = []          # {slug, status, createdAt, marked, consequences}
for fn in dec_files:
    text = read(os.path.join(DEC_DIR, fn)) or ""
    # the Decision part and everything after it (the acceptance verification trails it)
    m = re.search(r"(#(?:ProspectiveChange|SafetyChange)\s+)?part\s+(d\d+)\s*:\s*Decision\s*\{",
                  text)
    if not m:
        continue
    body = text[m.end():]
    st = re.search(r":>>\s*status\s*=\s*DecisionStatus::(\w+)\s*;", body)
    ca = re.search(r':>>\s*createdAt\s*=\s*"(\d{4}-\d{2}-\d{2})"', body)
    cons = re.search(r':>>\s*consequences\s*=\s*"(.*?)"\s*;', body, re.DOTALL)
    decisions.append({
        "file": fn,
        "slug": m.group(2),
        "marked": bool(m.group(1)),
        "status": st.group(1) if st else None,
        "createdAt": ca.group(1) if ca else None,
        "consequences": cons.group(1) if cons else "",
    })

accepted = [d for d in decisions if d["status"] == "accepted"]
proposed = [d for d in decisions if d["status"] == "proposed"]

fact("decisionsTotal", len(decisions), "Decision records",
     DEC_HOW + "one `part dNNNN : Decision` per file; counted by file")
fact("decisionsAccepted", len(accepted), "Decisions",
     DEC_HOW + "`:>> status = DecisionStatus::accepted;` inside the Decision part (the `:>>` "
               "assignment only - a bare `DecisionStatus::` in prose is not counted, which is why "
               "this is lower than a naive grep)")
fact("decisionsProposed", len(proposed), "Decisions",
     DEC_HOW + "`:>> status = DecisionStatus::proposed;` inside the Decision part")

# --- the 7-day window: [today-6, today], i.e. seven calendar days including today
WIN_START = TODAY - timedelta(days=6)
recent = [d for d in decisions if d["createdAt"] and
          WIN_START.isoformat() <= d["createdAt"] <= TODAY.isoformat()]
undated = [d for d in decisions if not d["createdAt"]]
WIN_HOW = (DEC_HOW + "the Decision part's own `:>> createdAt` in [%s, %s] - seven calendar days "
           "including today. %d of %d Decisions carry NO createdAt (the earliest records predate "
           "the field) and can never fall in the window; they are all far older than 7 days, so "
           "the count is unaffected. "
           % (WIN_START.isoformat(), TODAY.isoformat(), len(undated), len(decisions)))

fact("decisions7d", len(recent), "Decisions recorded in the last 7 days", WIN_HOW)
fact("decisionsMarked7d", sum(1 for d in recent if d["marked"]),
     "of those carrying #ProspectiveChange or #SafetyChange",
     WIN_HOW + "of those, the ones whose Decision part is prefixed "
               "`#ProspectiveChange` or `#SafetyChange` (D0070) - i.e. process/safety change, which "
               "under D0337 falls outside standing consent and waits for the human.")
fact("decisionsPerDay7d", round(len(recent) / 7.0, 1), "Decisions per day (7-day mean)",
     WIN_HOW + "divided by 7.")

# --- proposed Decisions that say, in their own consequences, that the change already ships
SHIPPED_PHRASES = ["the code ships", "already ship", "ships now", "is built"]
already = [d for d in proposed
           if any(p in d["consequences"].lower() for p in SHIPPED_PHRASES)]
SHIP_HOW = (DEC_HOW + "of the PROPOSED Decisions, those whose own `consequences` string contains one "
            "of the literal phrases " + repr(SHIPPED_PHRASES) + " (case-insensitive). This is a "
            "PHRASE RULE over authored prose, not a reading, and it is imperfect in two named ways - "
            "publish the number only with this caveat. FALSE POSITIVE: d0355 matches on 'once their "
            "state chip says the change already ships', which is about the OTHER pending items, not "
            "about itself. FALSE NEGATIVE: d0345 says 'the retirement itself is pushed now', which "
            "means the same thing and is outside the phrase list. The two cancel in the COUNT (5 "
            "either way) but not in the LIST. Fix by widening the phrase list, never by re-reading "
            "prose into a different answer.")
fact("pendingAlreadyShipped", len(already), "proposed Decisions whose code already ships", SHIP_HOW)
fact("pendingAlreadyShippedList", ", ".join(d["slug"] for d in already) or None,
     "decision slugs", SHIP_HOW + " Slugs listed in file order.")

# ================================================================ 4. keel orient (one call)
# `keel orient .` already emits JSON on stdout - there is no `--json` flag (it errors), so the
# plain invocation IS the JSON lens.

ok, out = run([KEEL, "orient", "."], timeout=90)
orient = as_json(out) if ok else None
O_HOW = ("`./target/release/keel.exe orient .` - it prints JSON with no flag; `orient --json` is "
         "rejected as an unknown flag, so the bare command is the JSON lens. ")

if orient is None:
    reason = O_HOW + ("command failed: " + (out if not ok else "stdout was not JSON"))
    for n in ("pendingAcceptances", "suspectElements", "openIssues", "readyItems"):
        fact(n, None, "items", reason)
else:
    fact("pendingAcceptances", len(orient.get("pendingAcceptances", [])),
         "proposed Decisions awaiting a human's acceptance",
         O_HOW + "len(.pendingAcceptances). Cross-checks with the file-derived decisionsProposed.")
    fact("suspectElements", len(orient.get("suspect", [])),
         "done items whose evidence drifted from the tree",
         O_HOW + "len(.suspect) - identical to `keel show suspect .` .suspect, verified by hand; "
                 "orient is used so the whole set costs one process start. NOTE `show suspect` "
                 "also reports critique_suspect (a different, larger set) - this is NOT that.")
    fact("openIssues", len(orient.get("open_issues", [])), "open Issues",
         O_HOW + "len(.open_issues) - equals `keel show open-issues .` .open, verified by hand.")
    fact("readyItems", len(orient.get("ready", [])), "items on the ready frontier",
         O_HOW + "len(.ready) - equals the line count of `keel whats-next .` and the 'N ready' in "
                 "`keel status .`, verified by hand.")

    # the triggered indicator lives in orient's burndown AND in `show indicators .`
    trig = None
    for t in (orient.get("burndown") or {}).get("triggers", []):
        if "ungrounded" in (t.get("indicator") or "").lower():
            trig = t
    bd = orient.get("burndown") or {}
    if trig is not None or "ungrounded_ratio_pct" in bd:
        val = float(trig["latest"]) if trig and trig.get("latest") else bd.get("ungrounded_ratio_pct")
        fact("ungroundedRatio", val, "% of Decision-chartered Delivery Stories reaching no Need",
             O_HOW + ".burndown.triggers[ungroundedRatioIndicator].latest (same number as "
                     ".burndown.ungrounded_ratio_pct, and as `keel show indicators .` "
                     "-> .triggered[].latest). D0333: an indicator surfaces work, it never gates.")
        fact("ungroundedRatioThreshold",
             (trig or {}).get("threshold"), "declared trigger threshold",
             O_HOW + ".burndown.triggers[].threshold - declared in "
                     ".engine/contracts/indicator-triggers.toml (D0333), not computed.")
    else:
        for n in ("ungroundedRatio", "ungroundedRatioThreshold"):
            fact(n, None, "%", O_HOW + "no ungrounded indicator present in .burndown.triggers")

# ================================================================ 5. keel show coverage
ok, out = run([KEEL, "show", "coverage", "."], timeout=90)
cov = as_json(out) if ok else None
C_HOW = "`./target/release/keel.exe show coverage .` -> .summary[] "

if cov is None:
    reason = C_HOW + ("failed: " + (out if not ok else "stdout was not JSON"))
    for n in ("needsTotal", "needsVerified", "needsUncovered",
              "reqsTotal", "reqsVerified", "reqsUncovered"):
        fact(n, None, "items", reason)
else:
    rows = {r.get("type"): r for r in cov.get("summary", [])}
    for prefix, typ in (("needs", "Need"), ("reqs", "SystemRequirement")):
        r = rows.get(typ) or {}
        label = "Needs" if typ == "Need" else "SystemRequirements"
        fact(prefix + "Total", r.get("total"), label,
             C_HOW + 'row type="%s" .total' % typ)
        fact(prefix + "Verified", r.get("verified"), label + " with reproducible verify evidence",
             C_HOW + 'row type="%s" .verified (D0082 top tier: reproducible verify-edge evidence; '
                     'Needs count transitively via a verified requirement)' % typ)
        fact(prefix + "Uncovered", r.get("uncovered"), label + " with no coverage at all",
             C_HOW + 'row type="%s" .uncovered (neither verified, attested, nor addressed)' % typ)

# ================================================================ 6. keel show verification
ok, out = run([KEEL, "show", "verification", "."], timeout=90)
V_HOW = ("`./target/release/keel.exe show verification .` - TEXT, not JSON. Parsed from the three "
         "labelled lines 'exercised but NEVER examined', 'examined but NEVER exercised', 'neither'. "
         "D0083: EXAMINED (a judgment was formed about the requirement) and EXERCISED (the system "
         "was run against it) are two dimensions - never publish their union as one 'verified %'. ")


def vgrab(text, label):
    m = re.search(re.escape(label) + r"\s*:?\s*(\d+)", text)
    return int(m.group(1)) if m else None


if not ok:
    for n in ("reqExercisedNeverExamined", "reqExaminedNeverExercised", "reqNeither"):
        fact(n, None, "SystemRequirements", V_HOW + "command failed: " + out)
else:
    fact("reqExercisedNeverExamined", vgrab(out, "exercised but NEVER examined"),
         "live SystemRequirements run against but never adversarially read",
         V_HOW + "line 'exercised but NEVER examined'.")
    fact("reqExaminedNeverExercised", vgrab(out, "examined but NEVER exercised"),
         "live SystemRequirements judged but never run against",
         V_HOW + "line 'examined but NEVER exercised'.")
    fact("reqNeither", vgrab(out, "neither"),
         "live SystemRequirements neither examined nor exercised",
         V_HOW + "line 'neither'.")

# ================================================================ 7. keel status (guards)
ok, out = run([KEEL, "status", "."], timeout=120)
S_HOW = "`./target/release/keel.exe status .` - TEXT; parsed from its `model` section. "
if not ok:
    for n in ("guardWarnings", "guardViolations"):
        fact(n, None, "guard findings", S_HOW + "command failed: " + out)
else:
    mv = re.search(r"(\d+)\s+violations?,\s+(\d+)\s+warning", out)
    fact("guardViolations", int(mv.group(1)) if mv else None,
         "guard violations (blocking)",
         S_HOW + "'N violations, M warning(s)'. A violation blocks the gate; this repo's contract is "
                 "that it is zero.")
    fact("guardWarnings", int(mv.group(2)) if mv else None,
         "guard warnings (non-blocking, unread until someone reads them)",
         S_HOW + "'N violations, M warning(s)'. Same numbers `keel guard .` prints in its ALL PASS "
                 "line; status is used because one process start yields both.")
    mg = re.search(r"(\d+)\s+guards", out)
    fact("guardsEnforced", int(mg.group(1)) if mg else None, "enforced forward guards",
         S_HOW + "'N guards' in the model line.")

# ================================================================ 8. release + git
ok, tagiso = run(["git", "log", "-1", "--format=%cI", "v0.3.1"])
tagiso = tagiso.strip() if ok else None
G_HOW = "`git log -1 --format=%cI v0.3.1` "
if not tagiso:
    fact("releaseTagDate", None, "ISO date", G_HOW + "failed (tag missing?): " + str(tagiso))
    tag_dt = None
else:
    tag_dt = datetime.fromisoformat(tagiso)
    fact("releaseTagDate", tagiso, "ISO-8601 commit date of tag v0.3.1",
         G_HOW + "- the commit date of the tagged commit, not the tag object's own date.")

ok, out = run(["git", "rev-list", "--count", "v0.3.1..HEAD"])
fact("commitsSinceRelease", int(out.strip()) if ok and out.strip().isdigit() else None,
     "commits on this branch since v0.3.1",
     "`git rev-list --count v0.3.1..HEAD`" + ("" if ok else " failed: " + out))

if tag_dt:
    fact("daysSinceRelease", (NOW - tag_dt.astimezone(timezone.utc)).days,
         "whole days since v0.3.1 was committed",
         G_HOW + "differenced against now (UTC), floored to whole days.")
else:
    fact("daysSinceRelease", None, "days", G_HOW + "no tag date, so nothing to difference.")

# ================================================================ 9. CI (gh - 2 calls, the slow part)
CI_CMD = ["gh", "run", "list", "--workflow=ci.yml", "--branch=main", "--limit", "200",
          "--json", "conclusion,createdAt,updatedAt"]
ok, out = run(CI_CMD, timeout=60)
runs = as_json(out) if ok else None
CI_HOW = "`" + " ".join(CI_CMD) + "`, filtered to runs whose createdAt >= the v0.3.1 tag date. "

if runs is None or tag_dt is None:
    reason = CI_HOW + ("gh failed: " + str(out)[:200] if runs is None
                       else "no v0.3.1 tag date to filter against")
    for n in ("ciRunsSinceRelease", "ciFailuresSinceRelease", "ciMeanMinutes"):
        fact(n, None, "runs", reason)
else:
    since = [r for r in runs
             if datetime.strptime(r["createdAt"], "%Y-%m-%dT%H:%M:%SZ")
             .replace(tzinfo=timezone.utc) >= tag_dt.astimezone(timezone.utc)]
    concluded = [r for r in since if r.get("conclusion")]
    fails = [r for r in concluded if r["conclusion"] != "success"]
    # is the 200-run window wide enough to reach back past the tag?
    oldest = min((r["createdAt"] for r in runs), default=None)
    saturated = (len(runs) >= 200 and oldest and
                 datetime.strptime(oldest, "%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=timezone.utc)
                 > tag_dt.astimezone(timezone.utc))
    window = ("The 200-run window reaches back to %s, which PREDATES the tag, so the count is "
              "complete." % oldest) if not saturated else \
             ("WARNING: the 200-run window's oldest run is %s, AFTER the tag - the window is "
              "saturated and these are LOWER BOUNDS." % oldest)
    fact("ciRunsSinceRelease", len(since), "ci.yml runs on main since v0.3.1",
         CI_HOW + window, as_of=NOW.date().isoformat())
    fact("ciFailuresSinceRelease", len(fails),
         "of those, concluded non-success (failure/cancelled/timed_out)",
         CI_HOW + "counts concluded runs whose conclusion != 'success'; %d run(s) in the window "
                  "had no conclusion yet and are excluded from both this and the mean. %s"
                  % (len(since) - len(concluded), window), as_of=NOW.date().isoformat())
    mins = [iso_min(r["createdAt"], r["updatedAt"]) for r in concluded]
    fact("ciMeanMinutes", round(sum(mins) / len(mins), 1) if mins else None,
         "mean wall minutes per concluded ci.yml run since v0.3.1",
         CI_HOW + "mean of (updatedAt - createdAt) over the %d CONCLUDED runs. This is queue+run "
                  "time as GitHub records it, not billable compute." % len(concluded),
         as_of=NOW.date().isoformat())

REL_CMD = ["gh", "run", "list", "--workflow=release.yml", "--limit", "5",
           "--json", "conclusion,createdAt,updatedAt,status,displayTitle"]
ok, out = run(REL_CMD, timeout=60)
rels = as_json(out) if ok else None
R_HOW = "`" + " ".join(REL_CMD) + "` -> the most recent entry (gh returns newest first). "

if not rels:
    reason = R_HOW + ("gh failed: " + str(out)[:200] if rels is None else "no release runs returned")
    for n in ("releaseLastRunDate", "releaseLastRunConclusion", "releaseLastRunMinutes"):
        fact(n, None, "release run", reason)
else:
    r = rels[0]
    fact("releaseLastRunDate", r.get("createdAt"), "ISO-8601 start of the last release.yml run",
         R_HOW + "its .createdAt. Title: %r" % r.get("displayTitle"),
         as_of=NOW.date().isoformat())
    fact("releaseLastRunConclusion", r.get("conclusion") or r.get("status"),
         "GitHub conclusion of the last release.yml run",
         R_HOW + "its .conclusion (falling back to .status while a run is still in flight).",
         as_of=NOW.date().isoformat())
    fact("releaseLastRunMinutes",
         round(iso_min(r["createdAt"], r["updatedAt"]), 1) if r.get("conclusion") else None,
         "wall minutes of the last release.yml run",
         R_HOW + ("(updatedAt - createdAt)." if r.get("conclusion")
                  else "the run has not concluded, so its updatedAt is not an end time."),
         as_of=NOW.date().isoformat())

# ================================================================ 10. suite receipt (D0353)
RCP = os.path.join(REPO, ".keel", "metrics", "suite-receipt.toml")
rcp_src = read(RCP)
RC_HOW = ".keel/metrics/suite-receipt.toml (D0353, machine-local): "

if rcp_src is None:
    for n in ("suiteTests", "suiteFailed", "suiteHead", "suiteWallMinutes"):
        fact(n, None, "tests", RC_HOW + "no receipt on this machine - run `keel suite`.")
else:
    def rget(key):
        m = re.search(r"^\s*%s\s*=\s*\"?([^\"\n]+)\"?\s*$" % key, rcp_src, re.MULTILINE)
        return m.group(1).strip() if m else None

    passed, failed = rget("passed"), rget("failed")
    at, head, logrel = rget("at"), rget("head"), rget("log")
    fact("suiteTests", int(passed) + int(failed) if passed and failed else None, "tests run",
         RC_HOW + "`passed` + `failed`. The receipt counts the whole `cargo test --release "
                  "--no-fail-fast` run, unit + integration + doc tests.")
    fact("suiteFailed", int(failed) if failed else None, "failing tests",
         RC_HOW + "`failed`.")
    fact("suiteHead", head, "short SHA the suite last ran against", RC_HOW + "`head`.")

    # wall time: `at` is the run's START (suite.rs: `let started = now_secs(); ... at: started`)
    # and the receipt names the log it streamed into, so the log's mtime is the finish.
    logpath = os.path.join(REPO, (logrel or "").replace("./", "").replace("/", os.sep))
    if at and logrel and os.path.exists(logpath):
        wall = os.path.getmtime(logpath) - int(at)
        fact("suiteWallMinutes", round(wall / 60.0, 1) if wall > 0 else None,
             "wall minutes of the most recent suite run",
             RC_HOW + "mtime(%s) minus `at`. `at` is the run's START - keel-cli/src/suite.rs does "
                      "`let started = now_secs(); let log = ...suite-{started}.log; ... at: started` "
                      "- and the log is written as the run streams, so its mtime is the finish. This "
                      "is WALL time. It is deliberately NOT the sum of the per-binary 'finished in' "
                      "values in the log, which is test time and excludes compilation and the gaps "
                      "between binaries." % logrel
             if wall > 0 else RC_HOW + "log mtime is not after `at`; nothing honest to derive.",
             as_of=datetime.fromtimestamp(int(at)).date().isoformat())
    else:
        fact("suiteWallMinutes", None, "minutes",
             RC_HOW + "the receipt's `log` (%r) is not on disk, so there is no finish timestamp to "
                      "difference against `at`. Refusing to substitute the summed per-binary "
                      "'finished in' values: that is test time, not wall time." % logrel)

# ================================================================ 11. Decision -> Need/UC/SR edges
# Typed edge lines look like `#DerivedFrom dependency from <src> to <dst>;`. A destination's TYPE
# comes from its declaration `<name> : Need|UseCase|SystemRequirement {` anywhere in the model.

DECL_RE = re.compile(r"\b([A-Za-z][A-Za-z0-9_]*)\s*:\s*(Need|UseCase|SystemRequirement)\s*\{")
EDGE_RE = re.compile(r"#([A-Za-z]+)\s+(?:dependency|connection)\s+from\s+"
                     r"([A-Za-z0-9_]+)\s+to\s+([A-Za-z0-9_]+)\s*;")
DEC_SRC = re.compile(r"^d\d{4}$")

sysml_files = []
for base in (".tracking", ".engine"):
    for dp, _dn, fns in os.walk(os.path.join(REPO, base)):
        for f in fns:
            if f.endswith(".sysml"):
                sysml_files.append(os.path.join(dp, f))

types = {}
for p in sysml_files:
    t = read(p) or ""
    for m in DECL_RE.finditer(t):
        types[m.group(1)] = m.group(2)

hits = []
for p in sysml_files:
    t = read(p) or ""
    for line in t.splitlines():
        if line.lstrip().startswith("//"):
            continue                       # a commented-out edge is not an edge
        for m in EDGE_RE.finditer(line):
            _mk, src, dst = m.groups()
            if DEC_SRC.match(src) and dst in types:
                hits.append((src, dst, types[dst]))

by_type = {}
for _s, _d, t in hits:
    by_type[t] = by_type.get(t, 0) + 1
fact("decisionNeedEdges", len(hits),
     "typed edges from a Decision to a Need / UseCase / SystemRequirement",
     "walk every .sysml under .tracking/ and .engine/. Build name -> type from declarations "
     r"`<name> : (Need|UseCase|SystemRequirement) {` (the keyword varies - `requirement n... : Need`, "
     "`use case uc... : UseCase`), then count non-comment lines matching "
     r"`#<Marker> dependency from d<NNNN> to <name>;` whose destination is in that map. "
     "Breakdown: " + (", ".join("%s=%d" % (k, v) for k, v in sorted(by_type.items())) or "none") +
     ". FINDING: d0355's own consequences state 'no Decision in this repository is connected by any "
     "edge to a Need or UseCase' - this count contradicts that. The edges exist but are concentrated "
     "in two old clusters (d0094 -> the serve Needs, and the keel-viewer Needs); the claim is right "
     "in spirit for RECENT Decisions and wrong as written.")

# ================================================================ emit
DOC = {
    "generatedAt": NOW.replace(microsecond=0).isoformat(),
    "tree": TREE,
    "facts": FACTS,
}

blob = json.dumps(DOC, indent=2, sort_keys=False)
out_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "decision-facts.json")
try:
    tmp = out_path + ".tmp"
    with open(tmp, "w", encoding="utf-8") as fh:
        fh.write(blob + "\n")
    os.replace(tmp, out_path)                  # atomic, so a re-run never leaves a half file
except Exception as exc:
    print("WARN: could not write %s: %s" % (out_path, exc), file=sys.stderr)

print(blob)
