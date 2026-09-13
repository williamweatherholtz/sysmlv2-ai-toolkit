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

import glob
import json
import os
import re
import subprocess
import sys
import time
from datetime import date, datetime, timedelta, timezone

# ---------------------------------------------------------------- infrastructure

REPO = os.getcwd()
# The binary is a COPY, never the build image: a running target/release/keel.exe blocks its own relink
# (issue150), and this script ran it under a cargo build once (issue508). KEEL_BIN wins; then the
# serve copy; the build image only when nothing else exists.
def _keel_bin():
    env = os.environ.get("KEEL_BIN")
    if env and os.path.exists(env):
        return env
    rel = os.path.join(REPO, "target", "release")
    for name in ("keel-serve.exe", "keel-serve", "keel.exe", "keel"):
        cand = os.path.join(rel, name)
        if os.path.exists(cand):
            return cand
    return os.path.join(rel, "keel.exe")


KEEL = _keel_bin()

TODAY = date.today()
NOW = datetime.now(timezone.utc)

# The artefact is CLAIMED before anything is computed (D0387/issue399): the previous answer is replaced
# by a running stub now, an uncaught exception below rewrites it as failed, and only the last line
# writes `complete: true`. A reader that does not go through artefact.require_complete is the defect.
sys.path.insert(0, os.path.join(REPO, "scripts"))
from artefact import begin_json, finish_json  # noqa: E402
OUT_PATH = os.path.join(os.path.dirname(os.path.abspath(__file__)), "decision-facts.json")
begin_json(OUT_PATH, "scripts/exec_brief/facts.py is running - this file is not an answer until complete is true")

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

    # THE MEMBERS, not only the counts (D0406): every live top-level verb by family with its effect,
    # so a page that proposes folding a family can name what folds and compute what remains. The
    # family-to-router mapping is the Decision's text and stays in the builder, quoted; the members
    # are facts. `name` and `family` are the authored fields of the CliCommand record.
    fam_verbs = {}
    for b in live_top:
        nm = re.search(r'name\s*=\s*"([^"]+)"', b)
        fm = re.search(r'family\s*=\s*"([^"]+)"', b)
        ef = re.search(r"CliEffect::(\w+)", b)
        if not (nm and fm and ef):
            continue
        fam_verbs.setdefault(fm.group(1), []).append({"name": nm.group(1), "effect": ef.group(1)})
    for fm in fam_verbs:
        fam_verbs[fm].sort(key=lambda r: r["name"])
    fact("cliLiveTopLevelByFamily",
         fam_verbs if sum(len(v) for v in fam_verbs.values()) == len(live_top) else None,
         "live top-level verbs by family, each with its effect",
         CLI_HOW + "the live top-level records grouped by their `family` field, each row the record's `name` and "
                   "CliEffect; the groups sum to cliLiveTopLevel or the fact is null (a record with no name, family "
                   "or effect would otherwise vanish from a members table without a trace)")

# ================================================================ 2. GATING CALL SITES
# Mentions of a gating verb invoked through the binary, across git-TRACKED files.
# .tracking/ is excluded from the headline: it is recorded history and must never be rewritten,
# so a call site there is not a maintenance surface. Its count is reported separately.

# D0452: the gating family is three routers; `keel gate validate` is ONE site, counted at `gate`.
GATING_VERBS = ["gate", "audit", "suite"]
GATING_RE = re.compile(r"(?<![\w.-])(?:keel\.exe|keelw|keel)[ \t]+(?:" +
                       "|".join(GATING_VERBS) + r")(?![\w-])")
ESCAPE_RE = re.compile(r"\\[nrt]")   # a literal \n in a source string is a line break, not a letter
# the naive form a reviewer would reach for first: no word boundaries at either end.
NAIVE_RE = re.compile(r"(?:keel\.exe|keelw|keel)[ \t]+(?:" + "|".join(GATING_VERBS) + r")")

GATING_HOW = ("git ls-files, then for each tracked TEXT file count regex "
              r"`(?<![\w.-])(keel\.exe|keelw|keel)[ \t]+<verb>(?![\w-])` over the gating verbs "
              "(" + ", ".join(sorted(GATING_VERBS)) + "); since D0452 the family is routed, so "
              "`keel gate validate` and `keel audit history` each count once, at the router. "
              "Counts OCCURRENCES, not lines - a line with two invocations is two "
              r"call sites. Literal \n/\r/\t escapes are normalised to a space first, so a call "
              r"site embedded in a source string (`\nkeel gate validate` in view/control_structure.rs) "
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

# Standing is the status MINUS the edge (D0398): a `#Supersede` edge retires its target whole and the
# target keeps the status it had, so a proposed-or-accepted Decision that is an edge target is neither
# pending nor in force. `#SupersedeClause` reverses one clause and leaves its target standing. Read from
# every non-comment line under .tracking/ and .engine/, the way section 11 reads DerivedFrom.
SUP_WHOLE_RE = re.compile(r"#Supersede\s+dependency\s+from\s+(\w+)\s+to\s+([\w, ]+);")
SUP_CLAUSE_RE = re.compile(r"#SupersedeClause\s+dependency\s+from\s+(\w+)\s+to\s+([\w, ]+);")
_sup_files = []
for _base in (".tracking", ".engine"):
    for _dp, _dn, _fns in os.walk(os.path.join(REPO, _base)):
        _sup_files.extend(os.path.join(_dp, f) for f in _fns if f.endswith(".sysml"))
retired = set()
sup_whole_edges = 0
sup_clause_edges = 0
clause_targets = set()
for _p in _sup_files:
    for _line in (read(_p) or "").splitlines():
        if _line.lstrip().startswith("//"):
            continue
        for _m in SUP_WHOLE_RE.finditer(_line):
            sup_whole_edges += 1
            retired.update(x.strip() for x in _m.group(2).split(","))
        for _m in SUP_CLAUSE_RE.finditer(_line):
            sup_clause_edges += 1
            clause_targets.update(x.strip() for x in _m.group(2).split(","))
_slugs = {d["slug"] for d in decisions}
retired_decisions = sorted(s for s in _slugs if s in retired)
clause_targets = {t for t in clause_targets if t in _slugs}
sup_clause_edges = len(clause_targets)
STANDING = (" and NOT the target of a `#Supersede` edge (D0398: retirement is the edge, the status is "
            "what the record read when retired; %d Decisions are retired this way)" % len(retired_decisions))

accepted = [d for d in decisions if d["status"] == "accepted" and d["slug"] not in retired]
proposed = [d for d in decisions if d["status"] == "proposed" and d["slug"] not in retired]

fact("decisionsTotal", len(decisions), "Decision records",
     DEC_HOW + "one `part dNNNN : Decision` per file; counted by file")
fact("decisionsAccepted", len(accepted), "Decisions in force",
     DEC_HOW + "`:>> status = DecisionStatus::accepted;` inside the Decision part (the `:>>` "
               "assignment only - a bare `DecisionStatus::` in prose is not counted, which is why "
               "this is lower than a naive grep)" + STANDING)
fact("decisionsProposed", len(proposed), "Decisions",
     DEC_HOW + "`:>> status = DecisionStatus::proposed;` inside the Decision part" + STANDING)
fact("decisionsRetired", len(retired_decisions), "Decisions retired by a #Supersede edge",
     "distinct Decision targets of a non-comment `#Supersede dependency from X to Y;` line under "
     ".tracking/ or .engine/: " + (", ".join(retired_decisions) or "none") + ".")
fact("supersedeEdges", sup_whole_edges, "#Supersede edges in the model (retire the target whole)",
     "non-comment lines matching `#Supersede dependency from X to Y;` - Y may be a comma list; "
     "targets are Decisions, Needs and requirements alike.")
fact("supersedeClauseEdges", sup_clause_edges, "#SupersedeClause edges (reverse one clause, target stands)",
     "non-comment lines matching `#SupersedeClause dependency from X to Y;` whose target is a declared "
     "Decision (a Decision's text quoting the grammar is not an edge): targets "
     + (", ".join(sorted(clause_targets)) or "none") + ".")

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
    if d["slug"] in retired or d["status"] == "rejected":
        continue                       # retired by a #Supersede edge (D0398) or rejected (D0122): decided, not open
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

# --- the pending set itself, member by member (D0406: a page that asks for a word on a SET names its members).
# name = the file's slug after the number (the Decision's own title token, not a record id); fork = the
# decision text opens with an OPTION marker (D0322: a weighed alternative is a fork; a ratification is not).
_name_re = re.compile(r"^\d{4}-(.+)\.sysml$")
pending_members = []
for d in proposed:
    _nm = _name_re.match(d["file"])
    _txt = read(os.path.join(DEC_DIR, d["file"])) or ""
    _dec = re.search(r':>>\s*decision\s*=\s*"(.*?)"\s*;', _txt, re.DOTALL)
    pending_members.append({
        "slug": d["slug"],
        "name": _nm.group(1) if _nm else d["file"],
        "fork": bool(_dec and _dec.group(1).lstrip().startswith("OPTION")),
        "inTree": d["slug"] in in_tree_slugs,
    })
PEND_HOW = (DEC_HOW + "every PROPOSED, non-retired Decision (the same set decisionsProposed counts), one row each: "
            "name = the file name between the number and .sysml; fork = the `decision` field begins with the "
            "literal OPTION (D0322's marker); inTree = the slug is in pendingInTreeList. File order.")
fact("pendingMembers", pending_members or None, "one row per pending Decision", PEND_HOW)
fact("pendingForks", sum(1 for p in pending_members if p["fork"]),
     "pending Decisions whose text opens a weighed fork", PEND_HOW)

# ================================================================ 3b. performance (D0367 asks)
# The baseline is whatever D0367's context RECORDS - read by regex from the Decision file, so a
# retyped number here cannot drift from the record. The current numbers are timed now, against the
# binary this script already shells out to, so the page states what THIS tree costs today.
_d0367 = ""
for _fn in os.listdir(os.path.join(REPO, ".engine", "decisions")):
    if _fn.startswith("0367-"):
        _d0367 = read(os.path.join(REPO, ".engine", "decisions", _fn))
_base = re.search(r"`keel orient` costs ([0-9.]+) s and `keel gate guard` ([0-9.]+) s", _d0367)
BASE_HOW = ("regex `keel orient` costs N s and `keel gate guard` N s over the context string of "
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
fact("perfGuardFullMs", _timed_ms([KEEL, "gate", "guard", "--no-receipt"], runs=2),
     "milliseconds",
     "wall time of `keel gate guard --no-receipt` in this tree, median of 2 - every enforced guard runs, "
     "the receipt is neither read nor written")
fact("perfOrientMs", _timed_ms([KEEL, "show", "orient", "."], runs=3),
     "milliseconds", "wall time of `keel show orient .` in this tree, median of 3")

# ================================================================ 4. keel show orient (one call)
# `keel show orient .` already emits JSON on stdout - there is no `--json` flag (it errors), so the
# plain invocation IS the JSON lens.

ok, out = run([KEEL, "show", "orient", "."], timeout=90)
orient = as_json(out) if ok else None
O_HOW = ("`./target/release/keel.exe show orient .` (D0450: orientation folded under show) - it prints JSON "
         "with no flag, so the bare command is the JSON lens. ")

if orient is None:
    reason = O_HOW + ("command failed: " + (out if not ok else "stdout was not JSON"))
    for n in ("pendingAcceptances", "suspectElements", "openIssues", "readyItems"):
        fact(n, None, "items", reason)
else:
    _pending = len(orient.get("pendingAcceptances", []))
    fact("pendingAcceptances", _pending,
         "proposed Decisions awaiting a human's acceptance",
         O_HOW + "len(.pendingAcceptances). Cross-checked against the file-derived decisionsProposed.")
    if _pending != len(proposed):
        # The file reader has drifted from the engine's standing rule (the D0398 shape: this script
        # counted two retired-while-proposed Decisions as pending for a day). Publish no number.
        fact("decisionsProposed", None, "Decisions",
             "REFUSED: the file-derived count (%d: %s) disagrees with orient's pendingAcceptances (%d) - "
             "this script's reading of a Decision's standing is stale against the engine's; fix facts.py "
             "before a brief carries either number." % (len(proposed), ", ".join(d["slug"] for d in proposed), _pending))
    fact("suspectElements", len(orient.get("suspect", [])),
         "done items whose evidence drifted from the tree",
         O_HOW + "len(.suspect) - identical to `keel show suspect .` .suspect, verified by hand; "
                 "orient is used so the whole set costs one process start. NOTE `show suspect` "
                 "also reports critique_suspect (a different, larger set) - this is NOT that.")
    fact("openIssues", len(orient.get("open_issues", [])), "open Issues",
         O_HOW + "len(.open_issues) - equals `keel show open-issues .` .open, verified by hand.")
    fact("readyItems", len(orient.get("ready", [])), "items on the ready frontier",
         O_HOW + "len(.ready) - equals the line count of `keel show whats-next .` and the 'N ready' in "
                 "`keel show status .`, verified by hand.")

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

# ================================================================ 7. keel show status (guards)
ok, out = run([KEEL, "show", "status", "."], timeout=120)
S_HOW = "`./target/release/keel.exe show status .` (D0450) - TEXT; parsed from its `model` section. "
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
         S_HOW + "'N violations, M warning(s)'. Same numbers `keel gate guard .` prints in its ALL PASS "
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
    running = rget("outcome") == "running"
    if running:
        # the stub `keel suite` writes at its START (D0387): a run in progress, or one that was killed -
        # either way the counts are not an answer, and the `how` says which file said so
        passed = failed = None
    RUNNING_HOW = (RC_HOW + "`outcome = \"running\"` - the stub keel suite writes before cargo starts (D0387): "
                  "a run in progress, or one that was killed at %s. No count until a run completes." % at)
    fact("suiteTests", int(passed) + int(failed) if passed and failed else None, "tests run",
         RUNNING_HOW if running else
         RC_HOW + "`passed` + `failed`. The receipt counts the whole `cargo test --release "
                  "--no-fail-fast` run, unit + integration + doc tests.")
    fact("suiteFailed", int(failed) if failed else None, "failing tests",
         RUNNING_HOW if running else RC_HOW + "`failed`.")
    fact("suiteHead", head, "short SHA the suite last ran against", RC_HOW + "`head`.")

    # wall time: since issue472 the receipt carries `seconds` (the run's wall clock) and `at` is
    # the moment the receipt was WRITTEN - the end of the run, the touched receipt's meaning. A
    # receipt from before that has no `seconds` and its `at` was the START, so for it alone the log's
    # mtime (written as the run streamed) minus `at` is the finish.
    seconds = rget("seconds")
    logpath = os.path.join(REPO, (logrel or "").replace("./", "").replace("/", os.sep))
    if at and seconds is not None and not running:
        wall = int(seconds)
        fact("suiteWallMinutes", round(wall / 60.0, 1) if wall > 0 else None,
             "wall minutes of the most recent suite run",
             RC_HOW + "`seconds`, stamped by keel-cli/src/suite.rs as the write moment minus the launch "
                      "(issue472). WALL time: compilation and the gaps between binaries included, not the "
                      "sum of the per-binary 'finished in' values."
             if wall > 0 else RC_HOW + "`seconds` is 0; nothing honest to derive.",
             as_of=datetime.fromtimestamp(int(at)).date().isoformat())
    elif at and logrel and os.path.exists(logpath):
        wall = os.path.getmtime(logpath) - int(at)
        fact("suiteWallMinutes", round(wall / 60.0, 1) if wall > 0 else None,
             "wall minutes of the most recent suite run",
             RC_HOW + "mtime(%s) minus `at` - a receipt from before issue472, whose `at` was the run's "
                      "START and which carries no `seconds`; the log is written as the run streams, so "
                      "its mtime is the finish. This "
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

# ================================================================ 10. the routing probe's price (D0378)
# The numbers the routingNumber fork rests on. The prompt count is read from the rig's own table; the
# costs are read from issue392's text - the one place they were recorded - never retyped here.
_rp = read(os.path.join(REPO, ".engine", "tools", "routing_prompts.toml")) or ""
fact("routingPrompts", _rp.count("[[case]]") or None, "prompts in the routing table",
     "count of `[[case]]` tables in .engine/tools/routing_prompts.toml - one per deployed process.")
_i392 = ""
for _p in glob.glob(os.path.join(REPO, ".tracking", "issues-*.sysml")):
    _t = read(_p) or ""
    _m = re.search(r"part issue392 : Issue \{(.*?)\n    \}", _t, re.S)
    if _m:
        _i392 = _m.group(1)
        break
I392_HOW = "read from the description of issue392 (the probe's cost, measured on this host 2026-09-06): "
_m = re.search(r"\$(\d+\.\d+)-(\d+\.\d+) and (\d+)-(\d+)s per run sequentially", _i392)
fact("routingRunCostLowUsd", float(_m.group(1)) if _m else None, "USD per probe run (low)",
     I392_HOW + "the `$a-b ... per run` span.")
fact("routingRunCostHighUsd", float(_m.group(2)) if _m else None, "USD per probe run (high)",
     I392_HOW + "the `$a-b ... per run` span.")
fact("routingRunSecLow", int(_m.group(3)) if _m else None, "seconds per probe run (low)",
     I392_HOW + "the `a-bs per run sequentially` span.")
fact("routingRunSecHigh", int(_m.group(4)) if _m else None, "seconds per probe run (high)",
     I392_HOW + "the `a-bs per run sequentially` span.")
_m = re.search(r"One sample per skill over \d+ skills is ~\$(\d+)", _i392)
fact("routingOneSampleUsd", int(_m.group(1)) if _m else None, "USD, one sample of every prompt",
     I392_HOW + "the `One sample per skill ... is ~$N` sentence.")
_m = re.search(r"Three samples[^$]*~\$(\d+) and around an hour", _i392)
fact("routingThreeSamplesUsd", int(_m.group(1)) if _m else None, "USD, three samples of every prompt",
     I392_HOW + "the `Three samples ... ~$N and around an hour` sentence.")
fact("routingThreeSamplesMinutes", 60 if _m else None, "minutes, three samples of every prompt",
     I392_HOW + "the same sentence's `around an hour`.")
_m = re.search(r"(\w+) of the (\w+) cases common to both runs changed verdict", _i392)
_words = {"one": 1, "two": 2, "three": 3, "four": 4}
fact("routingVerdictsFlipped", _words.get((_m.group(1) if _m else "").lower()),
     "verdicts that changed between two identical runs",
     I392_HOW + "the `N of the M cases ... changed verdict` sentence.")
fact("routingVerdictsCompared", _words.get((_m.group(2) if _m else "").lower()),
     "verdicts compared across two identical runs", I392_HOW + "the same sentence.")
_low = FACTS["routingRunCostLowUsd"]["value"]
_high = FACTS["routingRunCostHighUsd"]["value"]
fact("routingSubsetThreeSamplesUsd",
     round(16 * 3 * (_low + _high) / 2.0) if _low and _high else None,
     "USD, three samples of a 16-prompt subset",
     "16 prompts x 3 samples x the midpoint of routingRunCostLowUsd..HighUsd. The subset size (12 fixed + 4 "
     "rotated) is the shape proposed in D0378 option B, not a measured fact; the per-run cost is.")
fact("routingSubsetThreeSamplesMinutes",
     round(16 * 3 * 31 / 60.0) if "31s per run at four-way parallelism" in _i392 else None,
     "minutes, three samples of a 16-prompt subset at four-way parallelism",
     I392_HOW + "`31s per run at four-way parallelism`, times 48 runs. The subset size is D0378 option B's proposal.")

# ================================================================ 11. the STPA step-2 gate (D0363 / sprint610 / D0383)
# The gate the SOP used to state in prose is a row set now; `cs` was read above in section 9.
_g = cs.get("stepTwoGate") or []
fact("stepTwoClauses", len(_g) or None, "clauses of the SOP's step-2 gate the view decides",
     "`keel show control-structure`: length of the stepTwoGate array - one row per clause of the stpa SOP's "
     "step-2 gate (sprint610). Zero before 2026-09-08: the gate was a paragraph.")
fact("stepTwoHolds", sum(1 for r in _g if r.get("holds") is True) if _g else None, "of those clauses that hold",
     "`keel show control-structure`: stepTwoGate rows with holds == true. Each row carries the evidence that "
     "decided it; this counts, it does not re-judge.")
fact("controllersPresent", len(cs.get("controllers") or []) or None, "controller roles this project wires",
     "`keel show control-structure`: length of the controllers array (present roles only since sprint610).")
fact("rolesAbsent", len(cs.get("absentRoles") or []), "controller roles nothing wires",
     "`keel show control-structure`: length of absentRoles - " +
     (", ".join(r.get("role", "?") for r in (cs.get("absentRoles") or [])) or "none") +
     ". A role here is drawn nowhere; the diagram footnotes it with what would wire it.")
fact("actuatorsComputed", len(cs.get("actuators") or []) or None, "actuators derived from the actions",
     "`keel show control-structure`: length of the actuators array. Zero before sprint610.")
fact("otherInputsOutputs", len(cs.get("otherInputsOutputs") or []) or None,
     "authored OtherInputOutput items (the fifth element type, D0363)",
     "`keel show control-structure`: length of otherInputsOutputs, joined from engine-control-structure.sysml.")
fact("responsibilitiesComputed", len(cs.get("responsibilities") or []) or None,
     "controller -> hazard responsibilities, one hierarchical level deep",
     "`keel show control-structure`: length of the responsibilities array (actions x hazardsByProcess).")
_ok, _gd = run([KEEL, "gate", "guard", "."], timeout=180)
_m = re.search(r"stpa-currency: (\d+) of (\d+) computed control action", _gd or "")
fact("stpaActionsUnanalysed", int(_m.group(1)) if _m else None, "computed control actions no stpa-self run has analysed",
     "`keel gate guard .`: the stpa-currency WARN line's first number" + (" of %s" % _m.group(2) if _m else " - line not found") +
     ". sprint610 added agentEditsDeliverable to the action set, which is the designed re-run trigger (D0313).")

# ================================================================ 13. the recall cap (D0389 / D0390)
# The hook latency distribution keel show enforcement-report computes from the fire-ledger, and the over-cap
# proxy for skips before recall-skipped existed. Every number is read from the report or the ledger.
_er_ok, _er_raw = run([KEEL, "show", "enforcement-report", REPO])
ER_HOW = "keel show enforcement-report (D0389): per-event nearest-rank latency over .keel/metrics/hooks.jsonl, machine-local; "
try:
    _er = json.loads(_er_raw) if _er_ok and _er_raw else {}
except ValueError:
    _er = {}
_rows = {e.get("event"): e for e in _er.get("perEvent", [])}
for _ev, _key in (("user-prompt", "userPrompt"), ("post-edit", "postEdit")):
    _r = _rows.get(_ev) or {}
    fact(_key + "Fires", _r.get("fires"), "fires", ER_HOW + "`perEvent[%s].fires`." % _ev)
    for _f in ("msMedian", "msP90", "msP99", "msMax"):
        fact(_key + _f[2:], _r.get(_f), "ms", ER_HOW + "`perEvent[%s].%s`." % (_ev, _f))
_cap = 2500
_over = None
_ledger = read(os.path.join(REPO, ".keel", "metrics", "hooks.jsonl"))
if _ledger:
    _over = 0
    for _l in _ledger.splitlines():
        try:
            _o = json.loads(_l)
        except ValueError:
            continue
        if _o.get("event") == "user-prompt" and _o.get("ms", 0) > _cap:
            _over += 1
fact("userPromptOverCap", _over, "fires over the 2,500 ms recall cap",
     "count of user-prompt lines in .keel/metrics/hooks.jsonl with ms > 2500 - the PROXY for a dropped payload before "
     "recall-skipped existed (D0389); the check runs after the walk (main.rs recalled_facts), so every one of these paid "
     "the walk and received nothing.")
fact("recallSkippedCounted", (_er.get("recall") or {}).get("skipped"), "recall-skipped events since D0389",
     ER_HOW + "`recall.skipped`.")

# ================================================================ 14. script probes in CI (D0394)
# How many scripts under scripts/ ship a known-answer entry point (--probe / --self-test), and whether
# CI runs them. Read from the tree.
_probe_scripts = []
for _p in glob.glob(os.path.join(REPO, "scripts", "**", "*.py"), recursive=True):
    for _line in (read(_p) or "").splitlines():
        if _line.strip().startswith("# ci-probe:"):
            _probe_scripts.append(os.path.relpath(_p, REPO).replace(os.sep, "/"))
            break
fact("scriptsWithProbes", len(_probe_scripts) or None, "scripts under scripts/ shipping a known-answer entry point",
     "count of scripts/**/*.py carrying a `# ci-probe:` marker line: " + (", ".join(sorted(_probe_scripts)) or "none") + ".")
_ci = read(os.path.join(REPO, ".github", "workflows", "ci.yml")) or ""
fact("ciRunsProbes", ("--probe" in _ci or "--self-test" in _ci), "does CI run the script probes",
     ".github/workflows/ci.yml mentions `--probe` or `--self-test`: today it does not, so these checks run only by hand.")

# The fit sensor's own probe set and where it runs (D0402/D0403): the number of constructed known cases,
# read from the module, and whether the brief builders end in it. A builder whose last statement is not
# assert_fits publishes unmeasured.
_fit_src = read(os.path.join(REPO, "scripts", "exec_brief", "fit_check.py")) or ""
_fit_cases = re.search(r"PROBE_CASES\s*=\s*\{([\s\S]*?)\n\}", _fit_src)
fact("fitProbeCases", len(re.findall(r'^\s{4}"[^"]+":', _fit_cases.group(1), re.M)) if _fit_cases else None,
     "constructed known cases fit_check runs before measuring a page",
     "count of top-level keys in PROBE_CASES in scripts/exec_brief/fit_check.py (each is a page built to show one "
     "finding kind, or the fitting figure that must show none); null if the dict is not found.")
_builders = sorted(glob.glob(os.path.join(REPO, "scripts", "exec_brief", "build_*.py")))
_ending = [b for b in _builders if (read(b) or "").rstrip().splitlines()[-1].lstrip().startswith("assert_fits(")]
fact("briefBuilders", len(_builders) or None, "brief builder scripts", "count of scripts/exec_brief/build_*.py.")
fact("briefBuildersEndingInFit", len(_ending), "builders whose last statement is assert_fits",
     "of those, the ones whose last non-blank line begins `assert_fits(`: " + (", ".join(os.path.basename(b) for b in _ending) or "none") + ".")

# ================================================================ 15. releases bound to tags (D0400)
# How a git tag finds its Release record. Read from `git tag` and .tracking/baselines.sysml; the
# old rule (title contains the tag, first hit) is re-run here over the SAME records so its miscount is a
# number, not a story.
_tags = [t.strip() for t in (run(["git", "-C", REPO, "tag"])[1] or "").splitlines() if re.match(r"^v\d+\.\d+\.\d+$", t.strip())]
_rel_blocks = []
_cur = None
for _line in (read(os.path.join(REPO, ".tracking", "baselines.sysml")) or "").splitlines():
    _s = _line.strip()
    if _s.startswith("part ") and ": Release {" in _s:
        _cur = {"name": _s.split()[1], "title": "", "tag": ""}
    elif _cur is not None and _s.startswith(':>> title = "'):
        _cur["title"] = _s.split('"')[1]
    elif _cur is not None and _s.startswith(':>> tag = "'):
        _cur["tag"] = _s.split('"')[1]
    elif _cur is not None and _s == "}":
        _rel_blocks.append(_cur)
        _cur = None
fact("versionTags", len(_tags) or None, "git tags of the form vN.N.N", "`git tag` filtered to vN.N.N: " + ", ".join(_tags) + ".")
fact("releaseRecords", len(_rel_blocks) or None, "Release blocks in .tracking/baselines.sysml", "count of `part X : Release {` blocks.")
fact("releaseRecordsWithTagField", sum(1 for b in _rel_blocks if b["tag"]), "Release blocks carrying `:>> tag`",
     "count of blocks with a `:>> tag = ` line (D0400 migration 2026-09-09-release-tag-field.py).")
_title_hits = {t: [b["name"] for b in _rel_blocks if t in b["title"]] for t in _tags}
_title_wrong = [t for t in _tags if _title_hits[t] and next(b["tag"] for b in _rel_blocks if b["name"] == _title_hits[t][0]) != t]
_title_multi = [t for t in _tags if len(_title_hits[t]) > 1]
fact("tagsWithSeveralTitleMatches", len(_title_multi), "tags whose string occurs in more than one Release title",
     "re-running the pre-D0400 rule (title contains tag) over the same records: " + (", ".join(f"{t} -> {', '.join(_title_hits[t])}" for t in _title_multi) or "none") + ".")
fact("tagsMisboundByTitle", len(_title_wrong), "tags the title rule would bind to a record whose `tag` field says otherwise",
     "for each tag, the FIRST title hit compared with the record whose `tag` field equals it: " + (", ".join(f"{t} -> first title hit {_title_hits[t][0]}" for t in _title_wrong) or "none") + ".")
fact("releaseGuardWarnings", None, "release-recorded warnings on this tree", "not run here; keel gate guard release-recorded . prints it.")
_rr_ok, _rr_raw = run([KEEL, "gate", "guard", "release-recorded", REPO, "--no-receipt"])
_m = re.search(r"release-recorded\] \w+ [^0-9]*(\d+) scanned, (\d+) warning", _rr_raw or "")
if _m:
    fact("releaseGuardWarnings", int(_m.group(2)), "release-recorded warnings on this tree",
         "`keel gate guard release-recorded . --no-receipt` summary line: %s scanned, %s warning(s)." % (_m.group(1), _m.group(2)))

# ================================================================ 16. tests bound to source (D0401)
# Which tests in keel-cli read program source, and how many assert a code-shaped literal is PRESENT in
# it. A text scan, coarser than the Rust control (tests_bind_to_properties.rs) but run over two trees:
# the parent of the commit that added the control (where the offenders still stood) and HEAD. Probed
# before either number is stated (D0388): the parent tree is the known positive, HEAD the known
# negative - its own control is green - and a scan that disagrees with either refuses.
_CODE_TOKENS = (";", "{", "}", "==", "!=", "return ", "let ", "if ", "fn ", "=>", "()")
_BIND_RE = re.compile(r'(?:const|static|let(?:\s+mut)?)\s+([A-Za-z_]\w*)\b[^;]*?(?:include_str!|read_to_string)\s*\([^)]*\.rs"')
_NEEDLE_RE = re.compile(r'(!?)\s*([A-Za-z_]\w*)(?:\[[^\]]*\])?\.(?:contains|matches)\(\s*"((?:[^"\\]|\\.)*)"\s*\)')
_LET_RE = re.compile(r'\blet\s+(?:mut\s+)?([A-Za-z_]\w*)\b[^=]*=([^;]*)')
_CONTROL_FILE = "keel-cli/tests/tests_bind_to_properties.rs"


def _surface_at(rev):
    """(path, text) for every test region: src files from their first #[cfg(test)], tests/*.rs whole."""
    ok, listing = run(["git", "-C", REPO, "ls-tree", "-r", "--name-only", rev, "keel-cli/src", "keel-cli/tests"])
    out = []
    for p in (listing or "").splitlines():
        p = p.strip()
        if not p.endswith(".rs") or p == _CONTROL_FILE:
            continue
        if p.startswith("keel-cli/tests/") and p.count("/") != 2:
            continue
        ok, text = run(["git", "-C", REPO, "show", "%s:%s" % (rev, p)])
        if not ok or text is None:
            continue
        if p.startswith("keel-cli/src/"):
            at = text.find("#[cfg(test)]")
            if at < 0:
                continue
            text = text[at:]
        out.append((p, text))
    return out


def _test_fns(text):
    """Test function bodies, split at test attributes; the region's module-level text is index 0."""
    parts = re.split(r"(?=#\[(?:test|tokio::test))", text)
    return parts[0], parts[1:]


def _census(rev):
    """(source-reading tests, code-shaped positive asserts, tests carrying one)."""
    reading, asserts, offenders = 0, 0, 0
    for path, text in _surface_at(rev):
        module, fns = _test_fns(text)
        module_bound = set(_BIND_RE.findall(module))
        for body in fns:
            bound = set(_BIND_RE.findall(body)) | {b for b in module_bound if re.search(r"\b%s\b" % re.escape(b), body)}
            if not bound:
                continue
            grown = True                       # a `let` whose right side names a bound name binds its left side too
            while grown:
                grown = False
                for name, rhs in _LET_RE.findall(body):
                    if name not in bound and any(re.search(r"\b%s\b" % re.escape(b), rhs) for b in bound):
                        bound.add(name)
                        grown = True
            reading += 1
            hits = 0
            for neg, recv, lit in _NEEDLE_RE.findall(body):
                if neg == "!" or not any(t in lit for t in _CODE_TOKENS):
                    continue
                if any(re.search(r"\b%s\b" % re.escape(b), recv) for b in bound):
                    hits += 1
            asserts += hits
            offenders += 1 if hits else 0
    return reading, asserts, offenders


_ok_added, _added_raw = run(["git", "-C", REPO, "log", "--format=%H", "--diff-filter=A", "--", _CONTROL_FILE])
_added = (_added_raw or "").split()
_added_sha = _added[-1] if _added else None
_before = _census(_added_sha + "^") if _added_sha else (None, None, None)
_now = _census("HEAD")
# The known positive is not "some": the control's first real-tree run named 3 tests carrying 5 asserts
# (sprint 627), so a scan that sees fewer has missed a shape and its numbers are withheld.
_PROBE_OK = bool(_added_sha) and _before[2] == 3 and _before[1] == 5 and _now[2] == 0
_HOW = ("text scan over keel-cli test regions (src from the first #[cfg(test)], tests/*.rs, the control's own file "
        "excluded): a test READS SOURCE when its body, or a module const it names, binds include_str!/read_to_string "
        "of a .rs path, or a `let` whose right side names such a binding (to a fixpoint); an assert is CODE-SHAPED when a non-negated .contains/.matches on a bound name carries a literal "
        "holding one of ; { } == != return let if fn => (). Probe (D0388): the parent of the commit adding the control "
        "must show the 3 tests / 5 asserts its first run named, and HEAD none - " + ("held" if _PROBE_OK else "FAILED, numbers withheld") + ".")
if not _PROBE_OK:
    _before, _now = (None, None, None), (None, None, None)
fact("sourceReadingTestsBefore", _before[0], "tests reading program source, before the rebinding", _HOW + " Tree: %s^." % (_added_sha or "?")[:7])
fact("codeShapedAssertsBefore", _before[1], "positive code-shaped asserts on bound source, before", _HOW)
fact("sourceBoundOffendersBefore", _before[2], "tests carrying such an assert, before", _HOW)
fact("sourceReadingTestsNow", _now[0], "tests reading program source at HEAD", _HOW)
fact("sourceBoundOffendersNow", _now[2], "tests carrying a code-shaped assert at HEAD", _HOW)
_probe_fns = len(re.findall(r"\bfn probe_", read(os.path.join(REPO, _CONTROL_FILE)) or ""))
fact("testsBindProbes", _probe_fns or None, "known-answer probes shipped with the control",
     "count of `fn probe_` in %s: the fixtures the discriminator is run against before the tree is read (D0388)." % _CONTROL_FILE)

# ================================================================ 17. what this range changed (D0282 / dcCommitDeltaView)
# `keel show commit-delta` is the model delta over a git range - items ADDED by type, items RETIRED by a
# #Supersede edge, Issues RESOLVED - reconciled against the diff's declaration count. The brief's "what this
# range changed" section is built from THIS fact, never typed: the range runs from the tree the page was last
# published against (`publishedAgainst` in .keel/decision-page.toml) to HEAD, so a reader of the refreshed page
# sees what the model gained since they last read it. The decision channel the item's DoD named as a second
# surface is disconnected (D0291); the page is the surface.
_page = read(os.path.join(REPO, ".keel", "decision-page.toml")) or ""
_m = re.search(r'^publishedAgainst\s*=\s*"([0-9a-fA-F]+)"', _page, re.M)
_delta_from = _m.group(1) if _m else None
_DELTA_HOW = ("`keel show commit-delta . --range %s..HEAD` (D0282): items of a delta type - Need, SystemRequirement, "
              "Requirement, Decision, Issue, action - present at HEAD and absent at the range start, #Supersede and "
              "#Resolves edges new in the range; `reconciled` is the view's per-type count against the "
              "`+part <name> : <Type>` / `+action <name>;` lines git diff adds NET of the same name removed (a move)."
              % (_delta_from or "?"))
if _delta_from is None:
    fact("commitDelta", None, "model delta since the last publish",
         "no `publishedAgainst` in .keel/decision-page.toml, so the range has no start - " + _DELTA_HOW)
else:
    ok, out = run([KEEL, "show", "commit-delta", ".", "--range", "%s..HEAD" % _delta_from], timeout=180)
    _delta = None
    if ok:
        try:
            _delta = json.loads(out)
        except ValueError:
            ok, out = False, "commit-delta emitted non-JSON: %s" % out[:200]
    if _delta is None:
        fact("commitDelta", None, "model delta since the last publish", "commit-delta failed: %s - %s" % (out, _DELTA_HOW))
    else:
        fact("commitDelta", {
            "range": _delta.get("range"),
            "empty": _delta.get("empty"),
            "added": _delta.get("added", []),
            "superseded": _delta.get("superseded", []),
            "resolved": _delta.get("resolved", []),
            "reconciled": (_delta.get("reconciliation") or {}).get("matches"),
        }, "model delta since the last publish", _DELTA_HOW)
        fact("commitDeltaAdded", len(_delta.get("added", [])), "items added since the last publish", _DELTA_HOW)
        fact("commitDeltaSuperseded", len(_delta.get("superseded", [])), "items retired since the last publish", _DELTA_HOW)
        fact("commitDeltaResolved", len(_delta.get("resolved", [])), "Issues resolved since the last publish", _DELTA_HOW)

# ================================================================ 18. family call-site census (D0452 / D0453)
# The two safety Decisions fold the gating and channel families into routers and rewrite every call site in
# the SAME commit that removes the names. A page that says how many call sites move, and where, computes the
# number here with the census method each Decision's research comment names: the family's MEMBERS come from
# the CliCommand facts in .engine/cli/commands.sysml, the CALL SITES from the bounded regex of section 2
# restricted to those members over `git ls-files` (target/ is untracked, so excluded), every occurrence
# assigned to exactly ONE area by its path. Which members move and which stay is the Decision's text and
# stays in the builder; this section only counts.
_AREAS = [   # (area, predicate) - first match wins, so .engine/decisions is split out before .engine
    ("hooksAndCi", lambda p: p.startswith(".githooks/") or p.startswith(".github/workflows/")),
    ("decisions", lambda p: p.startswith(".engine/decisions/")),
    ("engine", lambda p: p.startswith(".engine/")),
    ("claude", lambda p: p.startswith(".claude/")),
    ("rustSource", lambda p: p.startswith("keel-cli/src/")),
    ("rustTests", lambda p: p.startswith("keel-cli/tests/")),
    ("rootDocs", lambda p: "/" not in p and p.endswith(".md")),
    ("tracking", lambda p: p.startswith(".tracking/")),
    ("other", lambda p: True),
]
_NOT_REWRITTEN = ("decisions", "tracking")   # recorded history: never rewritten (D0129), reported apart
_FAMILY_HOW = ("members: the `name` of EVERY non-lens CliCommand fact in .engine/cli/commands.sysml whose `family` "
               "is \"%s\", deprecated ones included - cliLiveTopLevelByFamily (live only) would drop a deprecated "
               "verb from a members table without a trace; memberFacts carries each name with its CliEffect and "
               "CliStability. sites: git ls-files, then for each tracked TEXT file count regex "
               r"`(?<![\w.-])(keel\.exe|keelw|keel)[ \t]+<member>(?![\w-])` (longest member first, so a hyphenated "
               "name never collapses into its prefix; literal \\n/\\r/\\t normalised to a space, as in section 2), "
               "OCCURRENCES not lines, each file assigned to ONE area by path, first match wins: hooksAndCi = "
               ".githooks/ + .github/workflows/; decisions = .engine/decisions/; engine = the rest of .engine/; "
               "claude = .claude/; rustSource = keel-cli/src/; rustTests = keel-cli/tests/; rootDocs = top-level "
               "*.md; tracking = .tracking/; other = everything else tracked. `sites[area][member]` and "
               "`files[area][member]` (distinct files) are the grid; `rewritten` sums every area except decisions "
               "and tracking, which the Decision does not rewrite and `notRewritten` sums. target/ is untracked "
               "and therefore absent. The verb-to-router mapping is NOT here: it is the Decision's text.")


def _family_census(family, members, tracked):
    verbs = sorted(members, key=len, reverse=True)
    rx = re.compile(r"(?<![\w.-])(?:keel\.exe|keelw|keel)[ \t]+(" + "|".join(re.escape(v) for v in verbs) + r")(?![\w-])")
    sites = {a: {v: 0 for v in members} for a, _ in _AREAS}
    files = {a: {v: 0 for v in members} for a, _ in _AREAS}
    for rel in tracked:
        text = read(os.path.join(REPO, rel.replace("/", os.sep)))
        if text is None or "\0" in text[:4096]:
            continue
        hits = rx.findall(ESCAPE_RE.sub(" ", text))
        if not hits:
            continue
        area = next(a for a, pred in _AREAS if pred(rel))
        for v in set(hits):
            files[area][v] += 1
        for v in hits:
            sites[area][v] += 1
    total_by_area = {a: sum(sites[a].values()) for a, _ in _AREAS}
    return {
        "family": family,
        "members": sorted(members),
        "sites": sites,
        "files": files,
        "totalByArea": total_by_area,
        "rewritten": sum(n for a, n in total_by_area.items() if a not in _NOT_REWRITTEN),
        "notRewritten": sum(n for a, n in total_by_area.items() if a in _NOT_REWRITTEN),
    }


# members are EVERY CliCommand fact of the family, deprecated ones included: the Decision moves the names
# the facts file holds. Each member carries its effect and stability so the page can say which of the
# moving names the surface already calls deprecated.
_by_family = None
if cli_src is not None:
    _by_family = {}
    for _b in re.finditer(r"part\s+cli\w*\s*:\s*CliCommand\s*\{([^\n]*)", cli_src):
        _b = _b.group(1)
        _nm = re.search(r'name\s*=\s*"([^"]+)"', _b)
        _fm = re.search(r'family\s*=\s*"([^"]+)"', _b)
        _ef = re.search(r"CliEffect::(\w+)", _b)
        _st = re.search(r"CliStability::(\w+)", _b)
        if _nm and _fm and _ef and _st and not re.search(r'invocation\s*=\s*"show\s', _b):
            _by_family.setdefault(_fm.group(1), []).append(
                {"name": _nm.group(1), "effect": _ef.group(1), "stability": _st.group(1)})
ok, out = run(["git", "ls-files"])
for _fam, _fact_name in (("gating", "familyCensusGating"), ("channel", "familyCensusChannel")):
    if not ok:
        fact(_fact_name, None, "call sites by area and member", _FAMILY_HOW % _fam + " `git ls-files` failed: " + out)
    elif not _by_family or _fam not in _by_family:
        fact(_fact_name, None, "call sites by area and member",
             _FAMILY_HOW % _fam + " commands.sysml is unreadable or has no %s family, so the members are unknown" % _fam)
    else:
        _members = [r["name"] for r in _by_family[_fam]]
        _c = _family_census(_fam, _members, [p for p in out.splitlines() if p.strip()])
        _c["memberFacts"] = sorted(_by_family[_fam], key=lambda r: r["name"])
        fact(_fact_name, _c, "call sites by area and member", _FAMILY_HOW % _fam)

# the dispatch count the Decisions say falls (by ten, by four) and the implement gate reads back
_HARD_HOW = ("`keel show hardening .` (D0169/D0434), field helpCoverage.dispatched: the number of match arms "
             "in keel-cli/src/main.rs that dispatch a top-level command; the read-back the two Decisions' "
             "implement gates name.")
ok, out = run([KEEL, "show", "hardening", "."], timeout=120)
_hard = as_json(out) if ok else None
_hc = (_hard or {}).get("helpCoverage") or {}
fact("cliDispatchArms", _hc.get("dispatched") if _hc.get("available") else None, "dispatch arms",
     _HARD_HOW if _hc.get("available") else _HARD_HOW + " hardening did not answer: " + out[:200])


# the count d0457 puts to the human: every remaining top-level family by member, effect and stability, the
# distance to D0273's under-25 clause, and the fold arithmetic against d0399's baseline commit - each from the
# facts file at the two commits, none typed from a Decision.
_D0399_BASE = "649b433"
_CENSUS_HOW = ("keel-cli/src/cli_facts.rs, every `CliFact {{ name, family, effect, stability }}` whose family is not "
               "`lens` (the 51 lenses sit under show), grouped by family with the member list; byEffect counts the "
               "same set; topLevel is the set's size and namesToUnder25 = topLevel - 24, the names that must go (armsToUnder25 the same from the help count, which has `help`) "
               "for D0273's clause to hold; baseline* comes from `git show {base}:keel-cli/src/cli_facts.rs` read the "
               "same way (d0399's counts were taken at {base}), removed = baseline - current, added = current - "
               "baseline; dispatchArms is cliDispatchArms (hardening), which is topLevel + `help`.")
_FACT_RX = re.compile(r'CliFact \{ name: "([^"]+)", family: "([^"]+)", effect: "([^"]+)", stability: "([^"]+)"')


def _top_level_facts(src):
    return [(n, f, e, s) for n, f, e, s in _FACT_RX.findall(src) if f != "lens"]


try:
    _cur_src = open(os.path.join(REPO, "keel-cli", "src", "cli_facts.rs"), encoding="utf-8").read()
except OSError as _e:
    _cur_src = None
    fact("cliFamilyCensus", None, "top-level verbs by family", _CENSUS_HOW.format(base=_D0399_BASE) +
         " cli_facts.rs unreadable: %s" % _e)
if _cur_src is not None:
    _cur = _top_level_facts(_cur_src)
    _fams = {}
    _eff = {}
    for _n, _f, _e, _s in _cur:
        _fams.setdefault(_f, []).append({"name": _n, "effect": _e, "stability": _s})
        _eff[_e] = _eff.get(_e, 0) + 1
    ok, _base_src = run(["git", "show", "%s:keel-cli/src/cli_facts.rs" % _D0399_BASE])
    _base = _top_level_facts(_base_src) if ok else None
    _cur_names = {n for n, _, _, _ in _cur}
    _base_names = {n for n, _, _, _ in _base} if _base is not None else None
    fact("cliFamilyCensus", {
        "topLevel": len(_cur),
        "namesToUnder25": len(_cur) - 24,
        "armsToUnder25": (_hc.get("dispatched") - 24) if _hc.get("available") else None,
        "dispatchArms": _hc.get("dispatched") if _hc.get("available") else None,
        "families": [{"family": f, "count": len(ms), "members": sorted(ms, key=lambda r: r["name"])}
                     for f, ms in sorted(_fams.items(), key=lambda kv: (-len(kv[1]), kv[0]))],
        "byEffect": _eff,
        "baselineCommit": _D0399_BASE,
        "baselineTopLevel": len(_base) if _base is not None else None,
        "removedSinceBaseline": sorted(_base_names - _cur_names) if _base_names is not None else None,
        "addedSinceBaseline": sorted(_cur_names - _base_names) if _base_names is not None else None,
    }, "top-level verbs by family", _CENSUS_HOW.format(base=_D0399_BASE) +
        ("" if ok else " (`git show` failed, baseline rows null: %s)" % _base_src[:120]))


# ================================================================ 19. sub-verb control actions (D0454 / sprint681)
# D0454 derives one control action per sub-verb of a routing write command from the fact's own invocation.
# The page that asks for a word on it says how many actions the rule adds, which routers it opens, and how
# much of the new set the analysis has covered - all read from `cs` (section 9, this binary's structure)
# and the census view, never from the Decision's text. "Sub-verb of" is the data prefix the derivation
# writes (control_structure.rs), so an action is a sub-verb action iff its data carries it; the router it
# belongs to is the backticked command in that phrase.
_SUB_HOW = ("`keel show control-structure .`: actions whose `data` contains `sub-verb of` are the D0454 sub-verb "
            "actions; the router is the `keel <command>` inside the backticks of that phrase. subVerbActions = their "
            "count; routers = distinct routers with their per-router counts; actionsTotal = every action; "
            "actionsBeforeRule = actionsTotal - subVerbActions + len(routers), i.e. what the structure listed when "
            "each router was one action (the sprint-680 read-back of 42 is the check); bareRoutersPresent = router "
            "action names (cmd + CamelCase command) that still appear - the rule says none may.")
_acts = cs.get("actions") or []
_sub_rx = re.compile(r"sub-verb of `keel ([a-z][a-z-]*)`")
_routers = {}
for _a in _acts:
    _mm = _sub_rx.search(_a.get("data") or "")
    if _mm:
        _routers[_mm.group(1)] = _routers.get(_mm.group(1), 0) + 1
_sub_n = sum(_routers.values())
_camel = lambda s: "cmd" + "".join(w[:1].upper() + w[1:] for w in s.split("-"))
_bare = sorted(_camel(r) for r in _routers if any(_a.get("name") == _camel(r) for _a in _acts))
fact("subVerbActions", {
    "actionsTotal": len(_acts) or None,
    "subVerbActions": _sub_n,
    "routers": dict(sorted(_routers.items())),
    "actionsBeforeRule": (len(_acts) - _sub_n + len(_routers)) if _acts else None,
    "bareRoutersPresent": _bare,
}, "control actions derived per sub-verb", _SUB_HOW)

# how much of the sub-verb set the analysis has covered: the ANALYSED: lines of every recorded stpa-self run
# hold the names; the stpa-currency guard's open count is section 11's stpaActionsUnanalysed.
_UCAS_PATH = os.path.join(REPO, ".tracking", "architecture", "engine-ucas.sysml")
_ucas_src = read(_UCAS_PATH) or ""
_analysed = set()
for _line in re.findall(r"ANALYSED:([^\"]*)", _ucas_src):
    _analysed.update(re.findall(r"\bcmd[A-Z]\w*", _line))
_sub_names = [_a.get("name") for _a in _acts if _sub_rx.search(_a.get("data") or "")]
fact("subVerbActionsAnalysed", {
    "runsRecorded": len(re.findall(r"verification stpaRun\d+ : Test", _ucas_src)),
    "analysed": sorted(n for n in _sub_names if n in _analysed),
    "open": sorted(n for n in _sub_names if n not in _analysed),
}, "sub-verb actions a recorded stpa-self run names",
     "engine-ucas.sysml: every `ANALYSED:` procedureText of a `verification stpaRunN : Test`, its cmd* names collected; "
     "a sub-verb action (subVerbActions) is analysed iff one of those lines names it. runsRecorded counts the run "
     "records. The guard stpa-currency computes the same set the other way round (section 11).")

# the UCA census after the run, from the view that holds it (D0428): how many UCAs exist, how many stand on a
# control, how many on an Issue that names the gap.
ok, out = run([KEEL, "show", "control-census", "."], timeout=120)
_cc = as_json(out) if ok else None
_us = (_cc or {}).get("ucaSummary") or {}
_ul = (_cc or {}).get("ucas") or []
fact("ucaCensus", {
    "total": _us.get("ucas"),
    "observed": _us.get("observed"),
    "codeRead": _us.get("codeRead"),
    "onAnIssue": sum(1 for u in _ul if u.get("issues")),
    "onAControl": sum(1 for u in _ul if u.get("boundControls")),
} if _us else None, "UCAs and what each stands on",
     "`keel show control-census .` (D0426/D0428): ucaSummary.ucas / observed / codeRead as the view computes them; "
     "onAnIssue = ucas rows whose `issues` list is non-empty, onAControl = rows whose `boundControls` is non-empty "
     "(a row can be both)." + ("" if _us else " The view did not answer: " + (out or "")[:200]))


# ================================================================ 20. the nineteenth publish's four asks
# Three process-change ratifications and one fork. Each number here is read from the record or the tree
# that holds it - the guard's own print, the skill files, the Decision's RESEARCH line - never retyped.

# --- D0463: the direction-cited guard's own line at HEAD (scanned / violations / the counted history)
ok, out = run([KEEL, "gate", "guard", "direction-cited", "."], timeout=120)
_dc = re.search(r"\[guard:direction-cited\] (PASS|FAIL|WARN)[^\d]*(\d+) scanned, (\d+) warning\(s\) \+ (\d+) counted-history line\(s\), (\d+) violation\(s\)", out or "")
_dh = re.search(r"HISTORY\s+(\d+) citing Decision\(s\) recorded before (\d{4}-\d{2}-\d{2})[^\d]*(\d+) citing DoD\(s\) carry no createdAt", out or "")
DC_HOW = ("`keel gate guard direction-cited .` (D0463, guard 73): the verdict line's scanned / warning / counted-history / "
          "violation counts, and the HISTORY line's two counts (citing Decisions before the cutoff, citing DoDs with no "
          "createdAt) - the guard's own print, quoted." + ("" if _dc else " The guard did not print a verdict line: " + (out or "")[-200:]))
fact("directionCited", {
    "verdict": _dc.group(1), "scanned": int(_dc.group(2)), "violations": int(_dc.group(5)),
    "historyDecisions": int(_dh.group(1)) if _dh else None, "cutoff": _dh.group(2) if _dh else None,
    "historyDoDs": int(_dh.group(3)) if _dh else None,
} if _dc else None, "the guard's counts at HEAD", DC_HOW)

# --- D0460: what the two skills say today about judgedAgainst and HEAD - the lines the Decision rewrites
_sk = {}
for _rel in ("test-result", "sprint-standup"):
    _t = read(os.path.join(REPO, ".engine", "skills", _rel, "SKILL.md")) or ""
    _sk[_rel] = [ln.strip() for ln in _t.splitlines() if "judgedAgainst" in ln and ("HEAD" in ln or "≠" in ln or "!=" in ln)]
fact("skillHeadEqualityLines", {k: len(v) for k, v in _sk.items()},
     "skill lines that tie judgedAgainst to HEAD",
     "lines of .engine/skills/test-result/SKILL.md and .engine/skills/sprint-standup/SKILL.md containing `judgedAgainst` "
     "and one of `HEAD`, `≠`, `!=` - the wording D0460 replaces (the recording instruction, which keeps HEAD, is one of them).")

# --- D0461: retros in the tree that name a Decision as written (D0NNN) - the class the widened needle reads
_retro_upper = 0
_retro_total = 0
for _fn in os.listdir(os.path.join(REPO, ".tracking", "delivery")):
    if not _fn.endswith(".sysml"):
        continue
    _t = read(os.path.join(REPO, ".tracking", "delivery", _fn)) or ""
    # a record starts at a LINE beginning `verification` or `part` (the guard's own reading); the retro gates are
    # the verifications whose name says Retro; the chunk runs to the next record's line
    _chunks = re.split(r"\n(?=\s*(?:verification|part)\s)", _t)
    for _c in _chunks:
        if re.match(r"\s*verification\s+\w*Retro\w*\s*:\s*Test", _c):
            _retro_total += 1
            if re.search(r"(?<![A-Za-z0-9])D0\d{3}(?![A-Za-z0-9])", _c):
                _retro_upper += 1
fact("retrosNamingDecisionUpper", {"retros": _retro_total, "namingD0NNN": _retro_upper},
     "retro gate Tests, and how many name a Decision as D0NNN",
     "over .tracking/delivery/*.sysml: every record starting at a line `verification <name>Retro<...> : Test` (the "
     "retro gate, read to the next `verification`/`part` line as guards.rs retro_texts does), and those whose text "
     "carries `D0` + three digits at a word boundary - the form named_items did not read before D0461.")

# --- D0464: the sweep the fork stands on, quoted from the Decision's own RESEARCH line; the constant from the source
_d0464 = ""
for _fn in os.listdir(DEC_DIR):
    if _fn.startswith("0464-"):
        _d0464 = read(os.path.join(DEC_DIR, _fn))
_rl = re.search(r"// RESEARCH: (.*)", _d0464)
_research = _rl.group(1) if _rl else ""
_arm1 = re.search(r"One-hop arm, 50 cases: (.*?)\. Two-hop arm", _research)
_arm2 = re.search(r"Two-hop arm, 50 cases: (.*?)\. Verdict lines", _research)


def _arm(text):
    rows = {}
    if not text:
        return rows
    # "DOMINANCE=0 hits 45/50 median 2 top-3 28/45 mean rows 12; 1.1 45/50 median 4 top-3 21/45; ..."
    for seg in text.split(";"):
        m = re.search(r"(?:DOMINANCE=)?([0-9.]+) (?:hits )?(\d+)/50 median (\d+) top-3 (\d+)/(\d+)", seg.strip())
        if m:
            rows[m.group(1)] = {"hits": int(m.group(2)), "median": int(m.group(3)), "top3": int(m.group(4))}
    # "1.3, 1.35, 1.4, 1.45 each 45/50 median 3 top-3 27/45"
    for m in re.finditer(r"((?:[0-9.]+, )+[0-9.]+) each (\d+)/50 median (\d+) top-3 (\d+)/(\d+)", text):
        for s in m.group(1).split(", "):
            rows[s] = {"hits": int(m.group(2)), "median": int(m.group(3)), "top3": int(m.group(4))}
    return rows


_hop1, _hop2 = _arm(_arm1.group(1) if _arm1 else ""), _arm(_arm2.group(1) if _arm2 else "")
_hand = {}
for m in re.finditer(r"at ([0-9.]+)(?: and at ([0-9.]+))? (?:the rebase question's d0129 arrives at position (\d+), )?(?:it does not arrive, )?injection ON (\d)/8, bar (MET|NOT MET) \((\d)/7", _research):
    for s in (m.group(1), m.group(2)):
        if s:
            _hand[s] = {"on": int(m.group(4)), "reachable": int(m.group(6)), "bar": m.group(5), "d0129Position": int(m.group(3)) if m.group(3) else None}
_ties = len(re.findall(r"COULD NOT CHOOSE on KEEL_RECALL_DOMINANCE", _research))
_kn = read(os.path.join(REPO, "keel-cli", "src", "view", "knowledge.rs")) or ""
_const = re.search(r"const DOMINANCE: f64 = ([0-9.]+);", _kn)
_bar = re.search(r'recallRanksLinkedRecordsAsWellAsGrepDoesDoD : Test \{[^}]*?procedureText = "(.*?)"', read(os.path.join(REPO, ".tracking", "backlog.sysml")) or "", re.DOTALL)
SW_HOW = ("regex over the `// RESEARCH:` line of .engine/decisions/0464-*.sysml - the sweep that Decision records, one row per "
          "DOMINANCE setting per arm (hits/50, median position, top-3), the hand-set readings (injection ON n/8, bar MET or "
          "NOT MET, n/7 reachable, d0129's position where it arrived), and the count of COULD NOT CHOOSE tie lines; "
          "`constant` = regex `const DOMINANCE: f64 = N;` over keel-cli/src/view/knowledge.rs (the value in force); "
          "`barText` = the procedureText of recallRanksLinkedRecordsAsWellAsGrepDoesDoD in .tracking/backlog.sysml. "
          "Quoted from the record and the source, never retyped; re-runnable with .engine/tools/recall_bench.py --sweep.")
fact("dominanceSweep", {
    "hop1": _hop1, "hop2": _hop2, "handSet": _hand, "tieLines": _ties,
    "constant": float(_const.group(1)) if _const else None,
    "barText": _bar.group(1) if _bar else None,
} if _hop1 and _hop2 and _hand else None, "the D0464 sweep, the constant in force, the bar's text", SW_HOW)

# every fact above reads the WORKING TREE while `tree` names HEAD; when the two differ the page must say so
_DIRTY_HOW = ("`git status --porcelain --untracked-files=all`: lines beginning with a change code other than `??` are "
              "tracked files with uncommitted edits, `??` lines are untracked files. Every file-reading fact in this "
              "run (the CLI surface, the censuses, hardening) reads the working tree, so when `modified` is not zero "
              "the numbers describe HEAD plus these edits and the page's provenance names the count.")
ok, out = run(["git", "status", "--porcelain", "--untracked-files=all"])
if ok:
    _lines = [l for l in out.splitlines() if l.strip()]
    fact("treeUncommitted", {"modified": len([l for l in _lines if not l.startswith("??")]),
                             "untracked": len([l for l in _lines if l.startswith("??")])},
         "files differing from HEAD", _DIRTY_HOW)
else:
    fact("treeUncommitted", None, "files differing from HEAD", _DIRTY_HOW + " git status failed: " + out)

# ================================================================ emit
DOC = {
    "generatedAt": NOW.replace(microsecond=0).isoformat(),
    "tree": TREE,
    "facts": FACTS,
}

# A FAILED RUN MUST NOT LEAVE A CURRENT-LOOKING FILE (D0387/issue399). The file was claimed with a running
# stub before section 1 ran; a raise above rewrote it as failed; this is the one place `complete` becomes
# true, and the stdout copy carries the same field so a redirected copy is checked the same way.
DOC["complete"] = True
finish_json(OUT_PATH, DOC)
print(json.dumps(DOC, indent=2, sort_keys=False))

