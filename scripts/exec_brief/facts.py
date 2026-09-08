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

# --- the consent scope since D0337: who has been accepting, and how long each waited
# D0337 (2026-09-05) scoped standing consent to the processes it was promulgated under, so a
# process/safety-change Decision waits for the human. Read from each file's acceptance records:
# AUTO if the acceptance Test's procedureText carries the AUTO-ACCEPTED token, HUMAN if a passing
# AcceptR result exists without it, else still proposed. Wait = judgedAt of the first AcceptR minus
# the Decision's createdAt, in days.
SCOPE_FROM = "d0337"
scope_auto, scope_human, scope_open, scope_waits = [], [], [], []
for d in decisions:
    if d["slug"] < SCOPE_FROM:
        continue
    text = read(os.path.join(DEC_DIR, d["file"])) or ""
    is_auto = "AUTO-ACCEPTED" in text
    r1 = re.search(r'part\s+' + d["slug"] + r'AcceptR1\s*:\s*TestResult\s*\{.*?judgedAt\s*=\s*"(\d{4}-\d{2}-\d{2})"', text, re.DOTALL)
    if d["status"] != "accepted" or not r1:
        scope_open.append(d["slug"])
    elif is_auto:
        scope_auto.append(d["slug"])
    else:
        scope_human.append(d["slug"])
        if d["createdAt"]:
            scope_waits.append((date.fromisoformat(r1.group(1)) - date.fromisoformat(d["createdAt"])).days)
SCOPE_HOW = (DEC_HOW + "Decisions with slug >= %s (D0337, the consent-scope rule, 2026-09-05); AUTO if the "
             "file carries the AUTO-ACCEPTED token, HUMAN if `<slug>AcceptR1` exists without it, OPEN otherwise. "
             % SCOPE_FROM)
fact("scopeDecisionsHumanAccepted", len(scope_human), "Decisions accepted on the human's own word since the consent-scope rule", SCOPE_HOW)
fact("scopeDecisionsAutoAccepted", len(scope_auto), "Decisions auto-accepted under standing consent since the consent-scope rule", SCOPE_HOW)
fact("scopeDecisionsOpen", len(scope_open), "Decisions since the consent-scope rule still proposed", SCOPE_HOW)
fact("scopeDaysSinceRule", (TODAY - date(2026, 9, 5)).days, "calendar days since D0337 was recorded",
     "today minus 2026-09-05, D0337's own createdAt.")
fact("scopeHumanWaitMaxDays", max(scope_waits) if scope_waits else None, "days",
     SCOPE_HOW + "for the HUMAN set, AcceptR1.judgedAt minus the Decision's createdAt; the maximum.")
fact("scopeHumanWaitMeanDays", round(sum(scope_waits) / len(scope_waits), 1) if scope_waits else None, "days",
     SCOPE_HOW + "for the HUMAN set, AcceptR1.judgedAt minus the Decision's createdAt; the mean.")

# --- the consent-defect census (D0375). Two HAND-CLASSIFIED lists, fixed here so the number on the
# page is the length of a list anyone can re-read, not a judgment re-made each run. Classified
# 2026-09-08 from each Issue's title and description; the census script that surfaced the candidates
# is textual (keyword over titles) and its class counts are NOT emitted as facts for that reason.
CONSENT_TOO_WIDE = ["issue256", "issue373", "issue371", "issue341", "issue342", "issue347",
                    "issue298", "issue238", "issue254"]
CONSENT_REFUSED_REAL = ["issue287", "issue359", "issue397", "issue223", "issue234", "issue383",
                       "issue396", "issue217"]
CENSUS_HOW = ("hand-classified 2026-09-08 from .tracking/issues-*.sysml titles and descriptions; the ids are "
              "listed in this script (CONSENT_TOO_WIDE / CONSENT_REFUSED_REAL) so the count is re-readable. ")
fact("issuesConsentTooWide", len(CONSENT_TOO_WIDE), "Issues where an acceptance was recorded or kept that was not what was given",
     CENSUS_HOW + ", ".join(CONSENT_TOO_WIDE))
fact("issuesConsentRefusedReal", len(CONSENT_REFUSED_REAL), "Issues where a control refused or mis-framed a real acceptance",
     CENSUS_HOW + ", ".join(CONSENT_REFUSED_REAL))
_iss_total = 0
_iss_dir = os.path.join(REPO, ".tracking")
for fn in os.listdir(_iss_dir) if os.path.isdir(_iss_dir) else []:
    if fn.startswith("issues-") and fn.endswith(".sysml"):
        _iss_total += len(re.findall(r"part\s+issue\d+\s*:\s*Issue\s*\{", read(os.path.join(_iss_dir, fn)) or ""))
fact("issuesTotal", _iss_total, "Issue records", "count of `part issueNNN : Issue {` across .tracking/issues-*.sysml")

# --- proposed Decisions that say, in their own consequences, that the change already ships
SHIPPED_PHRASES = ["the code ships", "already ship", "ships now", "is built",
                   # WIDENED 2026-09-07, by the rule this fact's own `how` states: the list is fixed
                   # by adding phrases, never by re-reading prose into a different answer. It read 0
                   # against a queue where three of five had shipped - d0361 says "IMPLEMENTED
                   # 2026-09-06", d0366 says "already in the tree", d0363 describes the move in the
                   # present tense. A false ZERO here is the issue383 defect exactly: a brief that
                   # frames already-shipped work as an open choice.
                   "already in the tree", "implemented", "in the commit that carries",
                   "landed", "is in the tree"]
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

# --- proposed Decisions whose chartered sprint is DONE - a structural reading, not a phrase rule
# A sprint record names the Decision that chartered it (`#CharteredBy dependency from <story> to
# dNNNN;`, D0068) and records its story's DoD verdict as a TestResult. Both are edges/results the
# guards already validate, so this fact cannot be moved by rewording a consequences string.
DELIV_HOW = ("walk .tracking/delivery/*.sysml; a proposed Decision is DELIVERED when one file holds "
             "`#CharteredBy dependency from <x> to <its id>;` and the same file holds a "
             "`story<Slug>DoDR<n> : TestResult` whose `outcome = VerdictKind::pass`. Reads typed "
             "edges and recorded verdicts, never prose; a Decision built OUTSIDE a sprint is not "
             "seen here, which is why the phrase rule above is kept beside it.")
_charter_re = re.compile(r"#CharteredBy\s+dependency\s+from\s+\w+\s+to\s+(d\d{4})\s*;")
_dod_pass_re = re.compile(r"part\s+story\w*DoDR\d+\s*:\s*TestResult\s*\{[^}]*VerdictKind::pass")
delivered_ids = set()
_deliv_dir = os.path.join(REPO, ".tracking", "delivery")
if os.path.isdir(_deliv_dir):
    for _fn in sorted(os.listdir(_deliv_dir)):
        if not _fn.endswith(".sysml"):
            continue
        _txt = read(os.path.join(_deliv_dir, _fn))
        if _dod_pass_re.search(_txt):
            delivered_ids.update(_charter_re.findall(_txt))
delivered = [d for d in proposed if d["slug"] in delivered_ids]
fact("pendingDelivered", len(delivered),
     "proposed Decisions whose chartered sprint records a passing DoD", DELIV_HOW)
fact("pendingDeliveredList", ", ".join(d["slug"] for d in delivered) or None,
     "decision slugs", DELIV_HOW + " Slugs listed in file order.")
in_tree_slugs = [d["slug"] for d in proposed if d["slug"] in delivered_ids or d in already]
IN_TREE_HOW = ("Union of pendingDeliveredList (structural: a finished sprint charters the Decision and its DoD "
               "passed) and pendingAlreadyShippedList (the phrase rule, for a Decision built outside any sprint). "
               "This is the ONE number the brief states as 'already in the tree'; the page builder reads it and "
               "refuses if its own cross-check of the two lists disagrees.")
fact("pendingInTree", len(in_tree_slugs), "proposed Decisions whose change is already in the tree", IN_TREE_HOW)
fact("pendingInTreeList", ", ".join(in_tree_slugs) or None, "decision slugs", IN_TREE_HOW + " Slugs listed in file order.")

# ================================================================ 3b. performance (D0367 asks)
# The baseline is whatever D0367's context RECORDS - read by regex from the Decision file, so a
# retyped number here cannot drift from the record. The current numbers are timed now, against the
# binary this script already shells out to, so the page states what THIS tree costs today.
_d0367 = ""
for _fn in os.listdir(os.path.join(REPO, ".engine", "decisions")):
    if _fn.startswith("0367-"):
        _d0367 = read(os.path.join(REPO, ".engine", "decisions", _fn))
_base = re.search(r"`keel orient` costs ([0-9.]+) s and `keel guard` ([0-9.]+) s", _d0367)
BASE_HOW = ("regex `keel orient` costs N s and `keel guard` N s over the context string of "
            ".engine/decisions/0367-*.sysml - the spike's own measured baseline (2026-09-07, this host), "
            "quoted from the record, never retyped.")
fact("perfBaselineOrientSec", float(_base.group(1)) if _base else None, "seconds", BASE_HOW)
fact("perfBaselineGuardSec", float(_base.group(2)) if _base else None, "seconds", BASE_HOW)


def _timed_ms(cmd, runs, stdin_text=None, warm=0):
    """Median wall ms of `runs` runs after `warm` untimed warm-up runs; None if any run fails."""
    samples = []
    for i in range(warm + runs):
        t0 = time.perf_counter()
        try:
            p = subprocess.run(cmd, cwd=REPO, capture_output=True, text=True, timeout=300,
                               input=stdin_text, encoding="utf-8", errors="replace")
        except Exception:
            return None
        ms = int((time.perf_counter() - t0) * 1000)
        if i >= warm:
            if p.returncode != 0 and cmd[1] != "hook":
                return None
            samples.append(ms)
    samples.sort()
    return samples[len(samples) // 2]


_env_note = " (KEEL_NO_RECEIPT unset; a receipt from an earlier green run is honoured when its key is equal)"
fact("perfTurnBoundaryIdleMs",
     _timed_ms([KEEL, "hook", "stop"], runs=3, warm=1,
               stdin_text='{"session_id":"exec-brief","stop_hook_active":false}'),
     "milliseconds",
     "wall time of `echo {hook json} | keel hook stop` in this tree, median of 3 after one untimed "
     "warm-up run (the warm-up writes the receipt when the build changed)" + _env_note)
fact("perfGuardFullMs", _timed_ms([KEEL, "guard", "--no-receipt"], runs=2),
     "milliseconds",
     "wall time of `keel guard --no-receipt` in this tree, median of 2 - every enforced guard runs, "
     "the receipt is neither read nor written")
fact("perfOrientMs", _timed_ms([KEEL, "orient", "."], runs=3),
     "milliseconds", "wall time of `keel orient .` in this tree, median of 3")

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

# ---------------------------------------------------------------- the control (issue389)
# This file measures "since the last release" and once TYPED the tag it measured against, so every
# such fact silently described a superseded release after a newer one shipped - with a `how` string
# that still read as authoritative because it named a real command. The rule is about SOURCE TEXT,
# which is why this check reads source text: nothing else can express "no version is typed here".
#
# It scans the WHOLE file, with no self-exclusion. The first version of this check scanned only what
# sat above it - and the release facts sit below, so it passed over exactly the defect it was written
# for. A deliberate probe caught that; the rule is now phrased without naming any version, so there
# is nothing to exclude and nothing to get wrong.
def _no_version_is_typed_here():
    src = open(__file__, encoding="utf-8").read()
    typed = sorted(set(re.findall(r"v\d+\.\d+\.\d+", src)))
    if typed:
        sys.exit(
            "facts.py names " + ", ".join(typed) + " literally. The release these facts measure against is "
            "DERIVED (`git describe --tags --abbrev=0 --match v*`); typing one makes every since-the-release "
            "fact describe that release forever, including after a newer one ships (issue389)."
        )


_no_version_is_typed_here()

# ================================================================ 8. release + git
# WHICH release: derived, never typed (issue389). Naming a version here makes every fact below
# describe that version forever, including after a newer one ships - and the `how` string still
# reads as authoritative because it names a real command.
ok, tag = run(["git", "describe", "--tags", "--abbrev=0", "--match", "v*"])
TAG = tag.strip() if ok and tag.strip() else None
fact("releaseTag", TAG, "the newest v* tag reachable from HEAD",
     "`git describe --tags --abbrev=0 --match v*` - the release these since-the-release facts are measured against."
     + ("" if TAG else " FAILED: " + str(tag)))

if TAG:
    ok, tagiso = run(["git", "log", "-1", "--format=%cI", TAG])
    tagiso = tagiso.strip() if ok else None
else:
    tagiso = None
G_HOW = f"`git log -1 --format=%cI {TAG}` " if TAG else "no v* tag found, so "
if not tagiso:
    fact("releaseTagDate", None, "ISO date", G_HOW + "produced no date: " + str(tagiso))
    tag_dt = None
else:
    tag_dt = datetime.fromisoformat(tagiso)
    fact("releaseTagDate", tagiso, f"ISO-8601 commit date of tag {TAG}",
         G_HOW + "- the commit date of the tagged commit, not the tag object's own date.")

if TAG:
    ok, out = run(["git", "rev-list", "--count", f"{TAG}..HEAD"])
    fact("commitsSinceRelease", int(out.strip()) if ok and out.strip().isdigit() else None,
         f"commits on this branch since {TAG}",
         f"`git rev-list --count {TAG}..HEAD`" + ("" if ok else " failed: " + out))
else:
    fact("commitsSinceRelease", None, "commits", "no v* tag to count from.")

if tag_dt:
    fact("daysSinceRelease", (NOW - tag_dt.astimezone(timezone.utc)).days,
         f"whole days since {TAG} was committed",
         G_HOW + "differenced against now (UTC), floored to whole days.")
else:
    fact("daysSinceRelease", None, "days", G_HOW + "no tag date, so nothing to difference.")

# ================================================================ 9. CI (gh - 2 calls, the slow part)
CI_CMD = ["gh", "run", "list", "--workflow=ci.yml", "--branch=main", "--limit", "200",
          "--json", "conclusion,createdAt,updatedAt"]
ok, out = run(CI_CMD, timeout=60)
runs = as_json(out) if ok else None
CI_HOW = "`" + " ".join(CI_CMD) + f"`, filtered to runs whose createdAt >= the {TAG} tag date. "

if runs is None or tag_dt is None:
    reason = CI_HOW + ("gh failed: " + str(out)[:200] if runs is None
                       else f"no {TAG} tag date to filter against")
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
    fact("ciRunsSinceRelease", len(since), "ci.yml runs on main since " + str(TAG),
         CI_HOW + window, as_of=NOW.date().isoformat())
    fact("ciFailuresSinceRelease", len(fails),
         "of those, concluded non-success (failure/cancelled/timed_out)",
         CI_HOW + "counts concluded runs whose conclusion != 'success'; %d run(s) in the window "
                  "had no conclusion yet and are excluded from both this and the mean. %s"
                  % (len(since) - len(concluded), window), as_of=NOW.date().isoformat())
    mins = [iso_min(r["createdAt"], r["updatedAt"]) for r in concluded]
    fact("ciMeanMinutes", round(sum(mins) / len(mins), 1) if mins else None,
         f"mean wall minutes per concluded ci.yml run since {TAG}",
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

# ================================================================ 9. instruments and control proofs
# The measures, and whether the controls have ever been shown to catch anything (D0360/D0361).
cs = as_json(run([KEEL, "show", "control-structure", "."])[1]) or {}
fact("instrumentsDeclared", len(cs.get("sensors") or []) or None, "declared measures (Sensor items)",
     "`keel show control-structure`: length of the sensors array. The measures were briefly declared in a "
     "contract file; they are model items now (D0363), so this counts what the model holds rather than what "
     "a manifest claimed - one source, and a stale entry cannot survive `edge-endpoints`.")
fact("feedbackChannels", len(cs.get("feedback") or []) or None, "computed feedback channels",
     "`keel show control-structure`: length of the feedback array - the upward half of the control structure.")
sensors = cs.get("sensors") or []
fact("sensorsComputed", len(sensors) or None, "computed sensors",
     "`keel show control-structure`: length of the sensors array. Zero before 2026-09-06: none had ever existed.")
assessed = [x for x in sensors if not str(x.get("propriety", "")).startswith("UNASSESSED")]
fact("instrumentsAssessed", len(assessed), "instruments with a propriety finding",
     "`keel show control-structure`: sensors whose computed `propriety` is not UNASSESSED - a JOIN against "
     "ProprietyFinding targets, never a stored flag (the owner's correction, 2026-09-07).")

ok, census = run(["python", ".engine/tools/guard_proof_census.py"])
proven = unnamed = None
for line in (census or "").splitlines():
    if "asserts a FAILURE" in line:
        proven = int(line.split(":")[-1].strip())
    elif "named nowhere in any test body" in line:
        unnamed = int(line.split(":")[-1].strip())
fact("guardsProven", proven, "guards named in a test that asserts a failure",
     "`python .engine/tools/guard_proof_census.py`: a DEMONSTRATED CATCH, distinct from declared and armed "
     "(D0360)." + ("" if ok else " census failed: " + str(census)[:80]))
fact("guardsUnnamed", unnamed, "guards named in no test body at all",
     "`python .engine/tools/guard_proof_census.py`: neither a demonstrated catch nor a demonstrated pass.")

# ================================================================ emit
DOC = {
    "generatedAt": NOW.replace(microsecond=0).isoformat(),
    "tree": TREE,
    "facts": FACTS,
}

# A FAILED RUN MUST NOT LEAVE A CURRENT-LOOKING FILE. This script wrote decision-facts.json on an
# earlier run, then raised on a later one, and the stale file sat there reading as fresh - the only
# thing that noticed was a reader checking for keys the new section should have added. A missing file
# is an honest answer; a stale one is scenario S-F4 with a timestamp.
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

