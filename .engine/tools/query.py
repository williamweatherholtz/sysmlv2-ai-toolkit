"""Engine query layer — small queries over the work-item model.

Subcommands (argv[1], default 'whats-next'):
  orient             -> in-progress sprint ceremony status (computed) + ready/suspect frontier
  whats-next         -> READY outstanding tasks (all deps done) + done/blocked/suspect summary
  outstanding        -> every not-done task
  suspect            -> DONE tasks whose verification is stale vs an upstream (git-ancestry)
  item <name>        -> introspect one task (done?, method, commit, deps + their states)
  downstream <n>     -> tasks transitively dependent on <n> (impact set)
  trace <name>       -> a task's full lineage: transitive upstream + downstream + DoD
  trace-need <name>  -> trace a Need/SR/Component over satisfy+allocate edges (text-read)
  workflows          -> the six workflow DAGs as Kahn-layered parallel waves (JSON)
  issues             -> Issue instances from all .tracking files (name/title/description/relatedTask)

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

ENGINE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO = os.path.dirname(ENGINE)
WF_DIR = os.path.join(ENGINE, "workflows")
# (package, action def) for each of the six workflows
WORKFLOWS = [
    ("BusinessWorkflow",      "Business"),
    ("ArchitectureWorkflow",  "Architecture"),
    ("DeliveryWorkflow",      "Delivery"),
    ("DeployWorkflow",        "Deploy"),
    ("OperateWorkflow",       "Operate"),
    ("ChangeRequestWorkflow", "ChangeMgmt"),
]
# Full preload: schema/core (canonical instance vocabulary) + all six workflow defs
# so `workflows` subcommand can %show any action def without a second kernel start.
PRELOAD = [os.path.join(ENGINE, *rel.split("/")) for rel in (
    "schema/core/element.sysml", "schema/core/needs.sysml", "schema/core/requirements.sysml",
    "schema/core/verification.sysml", "schema/core/work.sysml", "schema/core/architecture.sysml",
    "schema/core/computed.sysml", "schema/core/relationships.sysml", "schema/core/workflow.sysml",
    "schema/core/process.sysml", "schema/core/skills.sysml", "schema/core/risk.sysml",
    "schema/safety/stpa.sysml",
    "workflows/_meta.sysml", "workflows/business.sysml", "workflows/architecture.sysml",
    "workflows/delivery.sysml", "workflows/deploy.sysml", "workflows/operate.sysml",
    "workflows/change-request.sysml",
)]
TRACKING_DIR = os.path.join(REPO, ".tracking")

# UUID suffix pattern used by parse_show to strip kernel-appended elementIds
_UUID = re.compile(r'^(.*?)\s*\([0-9a-fA-F-]{36}\)\s*$')

# Dialect v2 (CR-3): criterion = `verification <task>DoD : Test { ... }` (one line);
# results = APPENDED `part <task>R<n> : TestResult { ... }` (immutable; latest wins).
_DOD_LINE = re.compile(r'verification\s+(\w+)DoD\s*:\s*Test')
_RESULT_LINE = re.compile(r'part\s+(\w+?)(?:DoD)?R(\d+)\s*:\s*TestResult')
_ASSIGN = re.compile(r'(\w+)\s*=\s*"([^"]*)"')
_ENUM = re.compile(r'(\w+)\s*=\s*\w+::(\w+)')
_PKG = re.compile(r'package\s+(\w+)')
_ACTION_DEF = re.compile(r'action\s+def\s+(\w+)')
# ordering-only successions: gate readiness but do NOT carry suspicion (D0005)
_ORDERING_ONLY = re.compile(r'#OrderingOnly\s+first\s+(\w+)\s+then\s+(\w+)\s*;')
# legacy (pre-CR-3) criterion line — used when reading OLD revisions for material change
_LEGACY_DOD = re.compile(r'requirement\s+(\w+)DoD\s*:\s*AcceptanceCriterion')
# satisfy/allocate edge patterns for trace-need subcommand
_SATISFY = re.compile(r'\bsatisfy\s+(\w+)\s+by\s+(\w+)\s*;')
_ALLOCATE = re.compile(r'\ballocate\s+(\w+)\s+to\s+(\w+)\s*;')
# Issue instance blocks — capture to the LINE-ANCHORED closing brace (\n + indent + }),
# NOT the first }: field values can contain } (e.g. a description quoting "Test {...}"),
# which [^}]* would truncate on (latent bug exposed by the orphans view, 2026-06-17).
_ISSUE_BLOCK = re.compile(r'part\s+(\w+)\s*:\s*Issue\s*\{(.*?)\n\s*\}', re.DOTALL)
# Boolean attribute assignments (not captured by _ASSIGN which only matches quoted strings)
_BOOL_ASSIGN = re.compile(r'(\w+)\s*=\s*(true|false)')


def parse_show(text):
    """Parse a `%show` indented AST into a tree of {rel,type,name,children}.
    Inlined from whats_next.py (foldWhatsNext 2026-06-11)."""
    root, stack = None, []
    for raw in text.splitlines():
        if not raw.strip():
            continue
        indent = len(raw) - len(raw.lstrip(' '))
        content = raw.strip()
        rel = None
        if content.startswith('['):
            rel, content = content[1:].split(']', 1)
            content = content.strip()
        m = _UUID.match(content)
        if m:
            content = m.group(1).strip()
        bits = content.split(' ', 1)
        node = {'rel': rel, 'type': bits[0],
                'name': bits[1].strip() if len(bits) > 1 else '', 'children': []}
        while stack and stack[-1][0] >= indent:
            stack.pop()
        if stack:
            stack[-1][1]['children'].append(node)
        else:
            root = node
        stack.append((indent, node))
    return root


def _wf_phases(root):
    """ActionUsage members of a workflow action def -> [{name, artifacts}]."""
    out = []
    for c in root['children']:
        if c['rel'] == 'FeatureMembership' and c['type'] == 'ActionUsage':
            arts = []
            for p in c['children']:
                if p['rel'] == 'FeatureMembership' and p['type'] == 'ReferenceUsage':
                    it = next((g['name'] for g in p['children']
                               if g['type'] == 'ItemDefinition'), None)
                    if it:
                        arts.append(it)
            out.append({'name': c['name'], 'artifacts': sorted(set(arts))})
    return out


def _wf_edges(root):
    """SuccessionAsUsage edges in a workflow action def -> [(earlier, later)]."""
    edges = []
    for c in root['children']:
        if c['type'] == 'SuccessionAsUsage':
            ends = {}
            for e in c['children']:
                if e['rel'] == 'EndFeatureMembership':
                    act = next((g['name'] for g in e['children']
                                if g['rel'] == 'ReferenceSubsetting'
                                and g['type'] == 'ActionUsage'), None)
                    ends[e['name']] = act
            a, b = ends.get('earlierOccurrence'), ends.get('laterOccurrence')
            if a and b:
                edges.append((a, b))
    return edges


def _kahn_waves(ph, edges):
    """Kahn layering of workflow phases into ordered parallel-ready waves."""
    names = [p['name'] for p in ph]
    deps = {n: set() for n in names}
    for a, b in edges:
        if a in deps and b in deps:
            deps[b].add(a)
    by_name = {p['name']: p for p in ph}
    out, done, remaining = [], set(), set(names)
    while remaining:
        ready = sorted(n for n in remaining if deps[n] <= done)
        if not ready:
            return None, sorted(remaining)
        out.append([by_name[n] for n in ready])
        done |= set(ready)
        remaining -= set(ready)
    return out, []


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


_GATE_PASS = re.compile(
    r'part\s+\w+(?P<gate>Refine|Standup|Implement|Review|CloseOut|Retro)Gate\w*\s*:\s*TestResult'
    r'[^}]*:>>\s+outcome\s*=\s*VerdictKind::pass',
    re.DOTALL,
)
_GATE_ORDER = ["Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"]


def read_sprint_ceremony_status():
    """Compute in-progress sprints from delivery file TestResults (D0045 — replaces StateCursor)."""
    delivery = os.path.join(TRACKING_DIR, "delivery")
    if not os.path.isdir(delivery):
        return []
    results = []
    for fname in sorted(os.listdir(delivery)):
        if not fname.endswith(".sysml"):
            continue
        fpath = os.path.join(delivery, fname)
        with open(fpath, encoding="utf-8") as fh:
            text = fh.read()
        passed = {m.group("gate") for m in _GATE_PASS.finditer(text)}
        if not passed:
            continue
        if "Retro" in passed:
            continue  # Retro passed = ceremony complete regardless of earlier gaps
        pending = next((g for g in _GATE_ORDER if g not in passed), None)
        results.append({
            "sprint": fname[:-6],
            "passed": [g for g in _GATE_ORDER if g in passed],
            "pending": pending,
        })
    return results


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


def read_satisfy_edges():
    """(need, sr) pairs from `satisfy <need> by <sr>;` in all tracking files."""
    edges = []
    for f in tracking_files():
        with open(f, encoding="utf-8") as fh:
            for line in fh:
                m = _SATISFY.search(line)
                if m:
                    edges.append((m.group(1), m.group(2)))
    return edges


def read_allocate_edges():
    """(sr, component) pairs from `allocate <sr> to <comp>;` in all tracking files."""
    edges = []
    for f in tracking_files():
        with open(f, encoding="utf-8") as fh:
            for line in fh:
                m = _ALLOCATE.search(line)
                if m:
                    edges.append((m.group(1), m.group(2)))
    return edges


def trace_need(name, satisfy_edges, allocate_edges):
    """Trace a Need/SR/Component over satisfy and allocate edges.

    Need  -> satisfiedBy (SRs) -> allocatedTo (Components)
    SR    -> satisfies (Needs) + allocatedTo (Components)
    Comp  -> allocatedFrom (SRs) -> satisfies (Needs)
    """
    satisfied_by = {}    # need -> [sr]
    satisfies_needs = {} # sr   -> [need]
    for need, sr in satisfy_edges:
        satisfied_by.setdefault(need, []).append(sr)
        satisfies_needs.setdefault(sr, []).append(need)

    allocated_to = {}   # sr   -> [comp]
    allocated_from = {} # comp -> [sr]
    for sr, comp in allocate_edges:
        allocated_to.setdefault(sr, []).append(comp)
        allocated_from.setdefault(comp, []).append(sr)

    out = {"name": name}
    is_need = name in satisfied_by
    is_sr   = name in satisfies_needs or name in allocated_to
    is_comp = name in allocated_from

    if is_need:
        out["kind"] = "need"
        srs = sorted(set(satisfied_by[name]))
        out["satisfiedBy"] = srs
        comps = sorted(set(c for s in srs for c in allocated_to.get(s, [])))
        if comps:
            out["allocatedTo"] = comps
    elif is_sr:
        out["kind"] = "systemRequirement"
        needs = sorted(set(satisfies_needs.get(name, [])))
        if needs:
            out["satisfies"] = needs
        alloc = sorted(set(allocated_to.get(name, [])))
        if alloc:
            out["allocatedTo"] = alloc
    elif is_comp:
        out["kind"] = "component"
        srs = sorted(set(allocated_from[name]))
        out["allocatedFrom"] = srs
        needs = sorted(set(n for s in srs for n in satisfies_needs.get(s, [])))
        if needs:
            out["satisfies"] = needs
    else:
        out["error"] = f"no satisfy or allocate edge found for '{name}'"
    return out


def read_issues():
    """Issue instances from all .tracking files (issueLoop UC10)."""
    issues = []
    for f in tracking_files():
        with open(f, encoding='utf-8') as fh:
            text = fh.read()
        for m in _ISSUE_BLOCK.finditer(text):
            body = m.group(2)
            attrs = dict(_ASSIGN.findall(body))
            bools = dict(_BOOL_ASSIGN.findall(body))
            issues.append({
                'name': m.group(1),
                'title': attrs.get('title', ''),
                'description': attrs.get('description', ''),
                'discoveredInField': bools.get('discoveredInField', 'false') == 'true',
                'relatedTask': attrs.get('relatedTask', ''),
                'createdAt': attrs.get('createdAt', ''),
                'authoredBy': attrs.get('createdBy') or attrs.get('authoredBy', ''),
            })
    return issues


_VIEWPOINT_BLOCK = re.compile(r'part\s+(\w+)\s*:\s*Viewpoint\s*\{(.*?)\n\s*\}', re.DOTALL)


def read_viewpoints():
    """Declared viewpoints from .engine/views/ (D0056/D0057) — the 'viewpoints' view:
    a view OF the declared lenses + render status (the concern-coverage audit)."""
    vps = []
    for f in sorted(glob.glob(os.path.join(ENGINE, "views", "*.sysml"))):
        with open(f, encoding="utf-8") as fh:
            text = fh.read()
        for m in _VIEWPOINT_BLOCK.finditer(text):
            attrs = dict(_ASSIGN.findall(m.group(2)))
            renderer = attrs.get("renderer", "")
            vps.append({
                "name": m.group(1),
                "title": attrs.get("title", ""),
                "concern": attrs.get("concernText", ""),
                "audience": attrs.get("audience", ""),
                "sources": attrs.get("sources", ""),
                "renderer": renderer,
                "rendered": not renderer.strip().startswith("(increment"),
            })
    return vps


def compute_orphans():
    """Orphaned / dangling elements (orphansVP, D0056) — kernel-free text read.
    A task with no DoD, an Issue with no/dangling relatedTask = broken traceability."""
    text_all = "\n".join(open(f, encoding="utf-8").read() for f in tracking_files())
    actions = set(re.findall(r'\baction\s+(\w+)\s*;', text_all))
    dods = set(re.findall(r'\bverification\s+(\w+)DoD\b', text_all))
    tasks_without_dod = sorted(a for a in actions if a not in dods)
    issues_no_rel, issues_dangling = [], []
    for iss in read_issues():
        rt = iss.get("relatedTask", "")
        if not rt:
            issues_no_rel.append(iss["name"])
        elif rt not in actions:
            issues_dangling.append({"issue": iss["name"], "relatedTask": rt})
    return {
        "tasks_without_dod": tasks_without_dod,
        "issues_without_relatedTask": issues_no_rel,
        "issues_dangling_relatedTask": issues_dangling,
    }


def compute_attestation_coverage():
    """Process-required-attestation coverage (attrModelCoverageView, D0066) — kernel-free.
    Lists items whose governing process requires an attestation they lack. Current rule:
    every status=accepted Decision must carry a passing acceptance event
    (`dNNNNAccept` confirmation TestResult). Extend with more (attestation, finder) pairs."""
    missing = []
    total = 0
    for f in sorted(glob.glob(os.path.join(ENGINE, "decisions", "*.sysml"))):
        with open(f, encoding="utf-8") as fh:
            text = fh.read()
        m = re.search(r'\bpart\s+(d\w+)\s*:\s*Decision\b', text)
        if not m or "DecisionStatus::accepted" not in text:
            continue
        dname = m.group(1)
        total += 1
        if not re.search(rf'\b{re.escape(dname)}AcceptR1\b[^}}]*VerdictKind::pass', text):
            missing.append(dname)
    return {
        "attestation": "accepted Decision -> acceptance event (dNNNNAccept, D0066)",
        "total_accepted": total,
        "covered": total - len(missing),
        "missing": sorted(missing),
    }


_CHARTERED = re.compile(r'#CharteredBy\s+dependency\s+from\s+(\w+)\s+to\s+(\w+)')


def read_charter_edges():
    """#CharteredBy edges (D0068, pglCharterEdge) — kernel-free: a work item -> the item
    (Decision/Need/Requirement) that chartered it, by name. The authored charter LINEAGE;
    the governing process VERSION is computed from it as-of the charter (pglViews, Inc 3)."""
    edges = []
    for f in tracking_files():
        with open(f, encoding="utf-8") as fh:
            for m in _CHARTERED.finditer(fh.read()):
                edges.append({"work": m.group(1), "charteredBy": m.group(2)})
    return edges


# Prefix-marker form `#ProspectiveChange part dNNNN : Decision` (the rust parser rejects the
# `{ @Marker; }` member form). Anchored at line-start (MULTILINE) so a real declaration matches
# but an EXAMPLE inside a quoted string (prose) does not.
_PROC_CHANGE = re.compile(
    r'^[ \t]*#(ProspectiveChange|SafetyChange)\s+part\s+(\w+)\s*:\s*Decision\b',
    re.MULTILINE)


def read_process_change_decisions():
    """Process-change Decisions (D0068/D0070) — kernel-free: a Decision carrying a
    #ProspectiveChange / #SafetyChange PREFIX MARKER on its part is a process change, with that
    retroactivity class. NO process linkage is stored — which process(es) it governs + when are
    COMPUTED from git (the process-def file(s) changed in that Decision's commit), the Inc-3
    resolver (pglViews). retroactivity: 'prospective' (then-process outputs stay valid, D0062
    default) or 'safety' (downstream items are mandatory reprocess candidates)."""
    out = []
    for f in sorted(glob.glob(os.path.join(ENGINE, "decisions", "*.sysml"))):
        with open(f, encoding="utf-8") as fh:
            for m in _PROC_CHANGE.finditer(fh.read()):
                out.append({
                    "decision": m.group(2),
                    "retroactivity": "safety" if m.group(1) == "SafetyChange" else "prospective",
                })
    return out


# --- pglViews: the git-traversal process-governance RESOLVER (D0068/D0069/D0070, Inc 3) ----------
# Reads the now-guarded authored inputs (process-change markers + charter edges) and COMPUTES, by
# git ancestry, the process version that governed any work item AS-OF its charter — never a stored
# per-item stamp (D0070). work->process is by CONVENTION/kind (D0069).

# Convention: which process-def file governs an item of a given declared type (D0069).
_GOVERNING_PROCESS = {
    "Story": ".engine/workflows/delivery.sysml",  # a sprint Story is governed by Delivery
}


def _git_lines(*args):
    r = subprocess.run(["git", "-C", REPO, *args], capture_output=True, text=True,
                       encoding="utf-8", errors="replace")
    return [ln.strip() for ln in r.stdout.splitlines() if ln.strip()] if r.returncode == 0 else []


def _item_intro_commit(name):
    """The commit that INTRODUCED a named item into .tracking/delivery (charter-time anchor,
    D0068 charter-time freeze). Earliest commit that changed occurrences of the name."""
    commits = _git_lines("log", "--format=%H", "--reverse", "-S", name, "--", ".tracking/delivery")
    return commits[0] if commits else None


def _def_change_commits(path):
    """Commits that changed a process-def file, newest-first."""
    return _git_lines("log", "--format=%H", "--", path)


def _decision_intro_commit(decision_file_rel):
    """The commit that added a decision file (its effective introduction)."""
    commits = _git_lines("log", "--diff-filter=A", "--format=%H", "--", decision_file_rel)
    return commits[-1] if commits else None  # --reverse-equivalent: oldest add


def _commit_files(sha):
    return _git_lines("show", "--name-only", "--format=", sha)


def process_change_decisions_full():
    """Process-change Decisions + effective commit (acceptance-event judgedAgainst, D0069/D0070)
    + git-DERIVED governed process-def files (the process-defs changed in the Decision's intro
    commit). Grandfathered decisions (committed before the keystone guard) typically have no
    co-committed process-def, so governed_defs == [] — honest, not fabricated (D0067)."""
    out = []
    for f in sorted(glob.glob(os.path.join(ENGINE, "decisions", "*.sysml"))):
        rel = os.path.relpath(f, REPO).replace(os.sep, "/")
        text = open(f, encoding="utf-8").read()
        for m in _PROC_CHANGE.finditer(text):
            dec = m.group(2)
            jm = re.search(rf'\b{re.escape(dec)}AcceptR1\b.*?judgedAgainst\s*=\s*"(\w+)"', text, re.DOTALL)
            eff = jm.group(1) if jm else None
            intro = _decision_intro_commit(rel)
            governed = [p for p in (_commit_files(intro) if intro else [])
                        if p.endswith(".sysml") and (p.startswith(".engine/processes/")
                                                     or p.startswith(".engine/workflows/"))]
            out.append({
                "decision": dec,
                "retroactivity": "safety" if m.group(1) == "SafetyChange" else "prospective",
                "effective_commit": eff,
                "governed_defs": governed,
            })
    return out


def governing_version(item):
    """The process version that governed `item` AS-OF its charter (D0068 charter-time freeze).
    Pure git: the process-def state at the latest change-commit that is an ancestor of the item's
    introduction commit, correlated with the process-change Decisions in force then vs. after."""
    item_commit = _item_intro_commit(item)
    if not item_commit:
        return {"item": item, "error": "no introduction commit found in .tracking/delivery"}
    proc_def = _GOVERNING_PROCESS["Story"]  # convention (only declared kind so far)

    def_commits = _def_change_commits(proc_def)
    governing = next((c for c in def_commits if _is_ancestor(c, item_commit)), None)
    later = [c for c in def_commits if not _is_ancestor(c, item_commit)]

    pcs = process_change_decisions_full()
    in_force, after = [], []
    for d in pcs:
        ec = d["effective_commit"]
        if not ec or not _sha_valid(ec):
            continue
        (in_force if _is_ancestor(ec, item_commit) else after).append(d)
    reprocess = sorted(d["decision"] for d in after if d["retroactivity"] == "safety")

    return {
        "item": item,
        "process": "Delivery",
        "process_def": proc_def,
        "convention": "a sprint Story is governed by Delivery (D0069 work->process by kind)",
        "item_commit": item_commit,
        "governing_version_commit": governing,
        "process_as_it_was": f"git show {governing}:{proc_def}" if governing else None,
        "later_version_count": len(later),
        "decisions_in_force_at_charter": sorted(d["decision"] for d in in_force),
        "process_changes_after_charter": [
            {"decision": d["decision"], "retroactivity": d["retroactivity"]} for d in after],
        "reprocess_required": bool(reprocess),
        "reprocess_due_to": reprocess,
        "valid_then": "asserted by the item's own ceremony gates (they encode the process it followed)",
    }


def _all_delivery_stories():
    out = []
    for f in sorted(glob.glob(os.path.join(TRACKING_DIR, "delivery", "*.sysml"))):
        for m in re.finditer(r'^[ \t]*part\s+(\w+)\s*:\s*Story\b', open(f, encoding="utf-8").read(),
                             re.MULTILINE):
            out.append(m.group(1))
    return out


def reprocess_candidates():
    """Items chartered under a process version later superseded by a SAFETY change (mandatory
    reprocess, D0069); prospective changes do NOT flag (D0062 — then-process outputs stay valid).
    Empty while no safety change exists — correctly demonstrating the prospective default."""
    out = []
    for s in _all_delivery_stories():
        gv = governing_version(s)
        if gv.get("reprocess_required"):
            out.append({"item": s, "due_to": gv["reprocess_due_to"]})
    return out


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

    # issues / viewpoints / orphans are pure text-reads — skip the kernel entirely
    if sub == "issues":
        print(json.dumps({"issues": read_issues()}, indent=2))
        return
    if sub == "viewpoints":
        print(json.dumps({"viewpoints": read_viewpoints()}, indent=2))
        return
    if sub == "orphans":
        print(json.dumps(compute_orphans(), indent=2))
        return
    if sub == "attestation-coverage":
        print(json.dumps(compute_attestation_coverage(), indent=2))
        return
    if sub == "charter":
        edges = read_charter_edges()
        if arg:
            ch = next((e["charteredBy"] for e in edges if e["work"] == arg), None)
            print(json.dumps({"item": arg, "charteredBy": ch}, indent=2))
        else:
            print(json.dumps({"charter_edges": edges}, indent=2))
        return
    if sub == "process-changes":
        # The process-change Decisions + their retroactivity class (D0070). Which process each
        # governs is git-derived (the Inc-3 resolver, pglViews) — not filtered here.
        print(json.dumps({"process_change_decisions": read_process_change_decisions()}, indent=2))
        return
    if sub == "governing-version":
        # pglViews resolver (D0068/D0069/D0070): the process version governing an item as-of its
        # charter, by git ancestry. Usage: query.py governing-version <storyName>
        if not arg:
            print(json.dumps({"error": "usage: governing-version <delivery Story name>"}, indent=2))
            return
        print(json.dumps(governing_version(arg), indent=2))
        return
    if sub == "reprocess-candidates":
        print(json.dumps({"reprocess_candidates": reprocess_candidates()}, indent=2))
        return

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
        part = build_model(parse_show(text), dods)
        # items must never collide on name (§2.3): REFUSE silent merge across backlogs
        clash = set(tasks) & set(part)
        if clash:
            print(json.dumps({"error": "task-name collision across backlogs — "
                              "qualify with --target", "collisions": sorted(clash),
                              "backlog": f"{pkg}::{adef}"}, indent=2))
            _kernel.teardown_and_exit(km, 2)
        tasks.update(part)
    classify(tasks, read_ordering_only())

    if sub == "orient":
        out = {"in_progress_sprints": read_sprint_ceremony_status(),
               # ready ranked by declaration (insertion) order = backlog priority (D0052)
               "ready": [t for t, i in tasks.items() if i['ready']],
               "suspect": sorted(t for t, i in tasks.items() if i['suspect']),
               "invalidEvidence": sorted(t for t, i in tasks.items() if i['invalidEvidence']),
               "counts": {"done": sum(1 for i in tasks.values() if i['done']),
                          "outstanding": sum(1 for i in tasks.values() if not i['done'])}}
    elif sub == "item" and arg:
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
    elif sub == "trace-need" and arg:
        out = trace_need(arg, read_satisfy_edges(), read_allocate_edges())
    elif sub == "workflows":
        result = {"workflows": []}
        for pkg, act in WORKFLOWS:
            _, text = _kernel.run_cell(kc, f"%show {pkg}::{act}")
            root = parse_show(text)
            ph = _wf_phases(root)
            w, cycle = _kahn_waves(ph, _wf_edges(root))
            wf = {"workflow": act, "package": pkg, "phaseCount": len(ph)}
            if cycle:
                wf["error"], wf["cycle"] = "dependency cycle", cycle
            else:
                wf["waves"] = [[{"action": p['name'], "artifacts": p['artifacts']}
                                for p in wave] for wave in w]
            result["workflows"].append(wf)
        out = result
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
