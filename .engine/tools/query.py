"""Engine query layer (v0) — small queries over the work-item model. This is the
general query core the user asked for; `whats-next` is one VIEW over it.

Subcommands (argv[1], default 'whats-next'):
  whats-next      -> READY outstanding tasks (all deps done) + done/blocked/suspect summary
  outstanding     -> every not-done task
  suspect         -> DONE tasks whose verification is stale vs an upstream (git-ancestry)
  item <name>     -> introspect one task (done?, method, commit, deps + their states)
  downstream <n>  -> tasks transitively dependent on <n> (impact set)
  trace <name>    -> a task's full lineage: transitive upstream + downstream + DoD

INSTANCE-AWARE: discovers work-item backlogs by scanning .tracking/*.sysml for
`action def`s (an action def in .tracking IS a work backlog; workflows live in
.engine/workflows). All discovered backlogs are merged. `--target Pkg::Def`
focuses a single one. (No longer hardcoded to EngineBacklog::EngineBuild.)

Semantics (dialect v2, CR-3 — criteria + APPENDED results):
  DONE        = the task's LATEST appended TestResult (<task>R<n>, immutable) is a pass.
  OUTSTANDING = not done (no results, or latest is fail/inconclusive/error).
  READY       = outstanding AND every dependency (succession predecessor) is done.
  SUSPECT     = done, but a dependency's latest pass was judged at a commit strictly
                DESCENDED from this task's (git merge-base --is-ancestor) — the
                upstream moved after this task was verified (D0005 suspicion).

Reads the model in TWO complementary ways:
  - GRAPH (tasks + dependency successions) from the pilot kernel %show — the
    kernel is the validated semantic authority and renders SuccessionAsUsage
    edges reliably.
  - DoD SCALAR VALUES (method/statement/verifiedAtCommit) from the .sysml TEXT —
    this kernel build's %show renders a RequirementUsage as a bare leaf and will
    NOT surface its attribute values, so they are read from source (the values
    the real text/Rust parser will read directly anyway). See D0006.
Run:
  conda run -n sysml --no-capture-output python .engine/tools/query.py [subcommand] [arg]
"""
import os
import re
import sys
import glob
import json
import subprocess

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import _kernel  # noqa: E402
from whats_next import parse_show  # noqa: E402  (shared AST parser)

ENGINE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO = os.path.dirname(ENGINE)
# Full preload so .tracking instances may be typed by ANY engine type (CR-1):
# schema/core is the canonical instance vocabulary; _meta still backs the backlog dialect.
PRELOAD = [os.path.join(ENGINE, *rel.split("/")) for rel in (
    "schema/core/element.sysml", "schema/core/needs.sysml", "schema/core/requirements.sysml",
    "schema/core/verification.sysml", "schema/core/work.sysml", "schema/core/architecture.sysml",
    "schema/core/computed.sysml", "schema/core/relationships.sysml", "schema/core/workflow.sysml",
    "schema/core/process.sysml", "schema/core/skills.sysml", "schema/core/risk.sysml",
    "schema/safety/stpa.sysml", "workflows/_meta.sysml",
)]
TRACKING_DIR = os.path.join(REPO, ".tracking")

# Dialect v2 (CR-3): criterion = `verification <task>DoD : Test { ... }` (one line);
# results = APPENDED `part <task>R<n> : TestResult { ... }` (immutable; latest wins).
_DOD_LINE = re.compile(r'verification\s+(\w+)DoD\s*:\s*Test')
_RESULT_LINE = re.compile(r'part\s+(\w+)R(\d+)\s*:\s*TestResult')
_ASSIGN = re.compile(r'(\w+)\s*=\s*"([^"]*)"')
_ENUM = re.compile(r'(\w+)\s*=\s*\w+::(\w+)')
_PKG = re.compile(r'package\s+(\w+)')
_ACTION_DEF = re.compile(r'action\s+def\s+(\w+)')
# ordering-only successions: gate readiness but do NOT carry suspicion (D0005)
_ORDERING_ONLY = re.compile(r'#OrderingOnly\s+first\s+(\w+)\s+then\s+(\w+)\s*;')
# legacy (pre-CR-3) criterion line — used when reading OLD revisions for material change
_LEGACY_DOD = re.compile(r'requirement\s+(\w+)DoD\s*:\s*AcceptanceCriterion')


def tracking_files():
    return sorted(glob.glob(os.path.join(TRACKING_DIR, "**", "*.sysml"), recursive=True))


def read_all_dods():
    """DoD scalar values across every .tracking file, merged by task name."""
    merged = {}
    for f in tracking_files():
        merged.update(read_dods(f))
    return merged


def discover_targets():
    """(package, actionDef) work-item backlogs across .tracking/*.sysml. An
    `action def` in .tracking IS a work backlog (workflows live in .engine/workflows,
    not .tracking) — so the query is instance-aware, not hardcoded to EngineBuild."""
    targets = []
    for f in tracking_files():
        with open(f, encoding="utf-8") as fh:
            text = fh.read()
        pm = _PKG.search(text)
        if not pm:
            continue
        for m in _ACTION_DEF.finditer(text):
            targets.append((pm.group(1), m.group(1)))
    return targets


def read_dods(path):
    """Criteria + appended results per task, from .sysml text (the kernel can't
    render verification-usage attribute values — D0006). Done is NOT stored:
    it's computed from the LATEST appended TestResult (CR-3)."""
    dods, results = {}, {}
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            m = _DOD_LINE.search(line)
            if m:
                attrs = dict(_ASSIGN.findall(line))
                enums = dict(_ENUM.findall(line))
                dods[m.group(1)] = {'method': enums.get('method'),
                                    'statement': attrs.get('procedureText')}
                continue
            r = _RESULT_LINE.search(line)
            if r:
                attrs = dict(_ASSIGN.findall(line))
                enums = dict(_ENUM.findall(line))
                results.setdefault(r.group(1), []).append(
                    {'n': int(r.group(2)), 'outcome': enums.get('outcome'),
                     'judgedAgainst': attrs.get('judgedAgainst'),
                     'judgedAt': attrs.get('judgedAt'),
                     'judgedBy': attrs.get('judgedBy')})
    for task, rs in results.items():
        rs.sort(key=lambda x: x['n'])
        dods.setdefault(task, {})['results'] = rs
    return dods


def build_model(root, dods):
    """root = ActionDefinition node (graph); dods = text-read DoD values.
    -> {task: {name, deps, done, dod}}."""
    tasks, edges = {}, []
    for c in root['children']:
        if c['rel'] == 'FeatureMembership' and c['type'] == 'ActionUsage':
            tasks[c['name']] = {'name': c['name']}
        elif c['type'] == 'SuccessionAsUsage':
            ends = {}
            for e in c['children']:
                if e['rel'] == 'EndFeatureMembership':
                    act = next((g['name'] for g in e['children']
                                if g['rel'] == 'ReferenceSubsetting' and g['type'] == 'ActionUsage'), None)
                    ends[e['name']] = act
            a, b = ends.get('earlierOccurrence'), ends.get('laterOccurrence')
            if a and b:
                edges.append((a, b))
    for t, info in tasks.items():
        info['dod'] = dods.get(t, {})
        info['deps'] = sorted(a for a, b in edges if b == t)
        rs = info['dod'].get('results', [])
        last = rs[-1] if rs else None
        # done = the LATEST appended result is a pass (results are immutable; CR-3)
        info['done'] = bool(last and last.get('outcome') == 'pass')
        info['verifiedAtCommit'] = last.get('judgedAgainst') if info['done'] else None
    return tasks


def _is_ancestor(a, b):
    """True if commit a is an ancestor of b."""
    return subprocess.run(["git", "-C", REPO, "merge-base", "--is-ancestor", a, b],
                          capture_output=True).returncode == 0


def _sha_valid(sha):
    """True if sha resolves to a commit (evidence integrity — critique A9)."""
    r = subprocess.run(["git", "-C", REPO, "cat-file", "-t", sha], capture_output=True, text=True)
    return r.returncode == 0 and r.stdout.strip() == "commit"


_SHOW_CACHE = {}


def _file_at(sha, path):
    """File content at a commit (relative path, '/'-separated), or None."""
    key = (sha, path)
    if key not in _SHOW_CACHE:
        r = subprocess.run(["git", "-C", REPO, "show", f"{sha}:{path}"],
                          capture_output=True, text=True, encoding="utf-8", errors="replace")
        _SHOW_CACHE[key] = r.stdout if r.returncode == 0 else None
    return _SHOW_CACHE[key]


def _criterion_at(sha, task):
    """The task's criterion statement AS OF a commit (material-change detection,
    D0005 rule 3). Reads both dialects (v2 verification line / legacy requirement
    line) so pre-CR-3 revisions compare correctly. None = not determinable."""
    for f in tracking_files():
        rel = os.path.relpath(f, REPO).replace(os.sep, "/")
        text = _file_at(sha, rel)
        if text is None:
            continue
        for line in text.splitlines():
            m = _DOD_LINE.search(line)
            if m and m.group(1) == task:
                return dict(_ASSIGN.findall(line)).get('procedureText')
            m = _LEGACY_DOD.search(line)
            if m and m.group(1) == task:
                return dict(_ASSIGN.findall(line)).get('statement')
    return None


def read_ordering_only():
    """(earlier, later) succession pairs tagged #OrderingOnly — excluded from suspicion."""
    pairs = set()
    for f in tracking_files():
        with open(f, encoding="utf-8") as fh:
            for line in fh:
                m = _ORDERING_ONLY.search(line)
                if m:
                    pairs.add((m.group(1), m.group(2)))
    return pairs


def classify(tasks, ordering_only=frozenset()):
    """D0005-honest classification (CR-4):
      - evidence: judgedAgainst SHAs must resolve (else INVALID-EVIDENCE, not done);
      - suspicion trigger: upstream's criterion text MATERIALLY CHANGED since this
        task's judgment commit (compared via git at the judgedAgainst revision).
        NOTE: mere re-attestation of an unchanged upstream does NOT flag — a
        re-verified-later trigger oscillates (every re-verification re-flags all
        downstreams) and is not what the contract demands;
      - suspicion travels only over SEMANTIC deps (ordering-only excluded);
      - suspicion is TRANSITIVE downstream."""
    for info in tasks.values():
        ct = info.get('verifiedAtCommit')
        info['invalidEvidence'] = bool(info['done'] and ct and not _sha_valid(ct))
        if info['invalidEvidence']:
            info['done'] = False           # unverifiable evidence is not done
            info['verifiedAtCommit'] = None
    for name, info in tasks.items():
        info['ready'] = (not info['done']) and not info['invalidEvidence'] and all(
            tasks[d]['done'] for d in info['deps'] if d in tasks)
        suspect = False
        if info['done']:
            ct = info['verifiedAtCommit']
            for d in info['deps']:
                if (d, name) in ordering_only or d not in tasks:
                    continue
                old = _criterion_at(ct, d) if ct else None
                cur = tasks[d].get('dod', {}).get('statement')
                if old is not None and cur is not None and old != cur:
                    suspect = True           # upstream's definition materially changed
                    break
        info['suspect'] = suspect
    # transitive: a done task whose SEMANTIC dep is suspect is itself suspect
    changed = True
    while changed:
        changed = False
        for name, info in tasks.items():
            if info['done'] and not info['suspect']:
                for d in info['deps']:
                    if (d, name) in ordering_only:
                        continue
                    if tasks.get(d, {}).get('suspect'):
                        info['suspect'] = True
                        changed = True
                        break
    return tasks


def _successors(tasks, name):
    return [t for t, i in tasks.items() if name in i.get('deps', [])]


def _reach(tasks, name, step):
    """Transitive closure of `step` (deps=upstream / successors=downstream) from name."""
    seen, stack = set(), [name]
    while stack:
        cur = stack.pop()
        for nxt in step(tasks, cur):
            if nxt not in seen:
                seen.add(nxt)
                stack.append(nxt)
    return sorted(seen)


def upstream(tasks, name):
    return _reach(tasks, name, lambda t, n: t.get(n, {}).get('deps', []))


def downstream(tasks, name):
    return _reach(tasks, name, _successors)


def main():
    argv = [a for a in sys.argv[1:] if not a.startswith("--")]
    sub = argv[0] if argv else "whats-next"
    arg = argv[1] if len(argv) > 1 else None
    # optional --target Pkg::Def to focus a single backlog (else discover all)
    override = None
    if "--target" in sys.argv:
        override = tuple(sys.argv[sys.argv.index("--target") + 1].split("::"))

    km, kc = _kernel.start()
    for f in PRELOAD:
        with open(f, encoding="utf-8") as fh:
            _kernel.run_cell(kc, fh.read())
    for f in tracking_files():
        with open(f, encoding="utf-8") as fh:
            _kernel.run_cell(kc, fh.read())

    targets = [override] if override else discover_targets()
    dods = read_all_dods()
    tasks = {}
    for pkg, adef in targets:
        _, text = _kernel.run_cell(kc, f"%show {pkg}::{adef}")
        tasks.update(build_model(parse_show(text), dods))
    classify(tasks, read_ordering_only())

    if sub == "item" and arg:
        out = tasks.get(arg, {"error": f"no task '{arg}'"})
    elif sub == "suspect":
        out = {"suspect": sorted(t for t, i in tasks.items() if i['suspect'])}
    elif sub == "outstanding":
        out = {"outstanding": sorted(t for t, i in tasks.items() if not i['done'])}
    elif sub == "downstream" and arg:
        out = {"name": arg, "downstream": downstream(tasks, arg)}
    elif sub == "trace" and arg:
        out = {"name": arg, "upstream": upstream(tasks, arg),
               "downstream": downstream(tasks, arg),
               "done": tasks.get(arg, {}).get('done'), "dod": tasks.get(arg, {}).get('dod', {})}
    else:  # whats-next
        out = {"ready": sorted(t for t, i in tasks.items() if i['ready']),
               "suspect": sorted(t for t, i in tasks.items() if i['suspect']),
               "invalidEvidence": sorted(t for t, i in tasks.items() if i['invalidEvidence']),
               "blocked": sorted(t for t, i in tasks.items()
                                 if not i['done'] and not i['ready'] and not i['invalidEvidence']),
               "done": sorted(t for t, i in tasks.items() if i['done'])}
    print(json.dumps(out, indent=2))
    _kernel.teardown_and_exit(km, 0)


if __name__ == "__main__":
    main()
