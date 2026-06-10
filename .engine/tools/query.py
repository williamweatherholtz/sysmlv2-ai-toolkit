"""Engine query layer (v0) — small queries over the work-item model. This is the
general query core the user asked for; `whats-next` is one VIEW over it.

Subcommands (argv[1], default 'whats-next'):
  whats-next   -> READY outstanding tasks (all deps done) + done/blocked/suspect summary
  suspect      -> DONE tasks whose verification is stale vs an upstream (git-ancestry)
  item <name>  -> introspect one task (done?, method, commit, deps + their states)

Semantics:
  DONE        = the task's AcceptanceCriterion has a `verifiedAtCommit` (a pass result).
  OUTSTANDING = not done.
  READY       = outstanding AND every dependency (succession predecessor) is done.
  SUSPECT     = done, but a dependency was verified at a commit strictly DESCENDED
                from this task's verifiedAtCommit (git merge-base --is-ancestor) —
                i.e. the upstream moved after this task was verified (D0005 suspicion).

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
import json
import subprocess

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import _kernel  # noqa: E402
from whats_next import parse_show  # noqa: E402  (shared AST parser)

ENGINE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
REPO = os.path.dirname(ENGINE)
META = os.path.join(ENGINE, "workflows", "_meta.sysml")
BACKLOG = os.path.join(REPO, ".tracking", "backlog.sysml")
TARGET = ("EngineBacklog", "EngineBuild")

# A DoD line: `requirement <task>DoD : AcceptanceCriterion { ... k = "v"; ... }`.
_DOD_LINE = re.compile(r'requirement\s+(\w+)DoD\s*:')
_ASSIGN = re.compile(r'(\w+)\s*=\s*"([^"]*)"')


def read_dods(path):
    """DoD scalar values per task, parsed from .sysml text (kernel can't read them)."""
    dods = {}
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            m = _DOD_LINE.search(line)
            if not m:
                continue
            task = m.group(1)
            attrs = dict(_ASSIGN.findall(line))
            dods[task] = {'method': attrs.get('method'),
                          'statement': attrs.get('statement'),
                          'verifiedAtCommit': attrs.get('verifiedAtCommit'),
                          'verifiedBy': attrs.get('verifiedBy'),
                          'verifiedAt': attrs.get('verifiedAt')}
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
        info['done'] = bool(info['dod'].get('verifiedAtCommit'))
    return tasks


def _is_ancestor(a, b):
    """True if commit a is an ancestor of b."""
    return subprocess.run(["git", "-C", REPO, "merge-base", "--is-ancestor", a, b],
                          capture_output=True).returncode == 0


def classify(tasks):
    for info in tasks.values():
        info['ready'] = (not info['done']) and all(
            tasks[d]['done'] for d in info['deps'] if d in tasks)
        suspect = False
        if info['done']:
            ct = info['dod'].get('verifiedAtCommit')
            for d in info['deps']:
                cd = tasks.get(d, {}).get('dod', {}).get('verifiedAtCommit')
                if tasks.get(d, {}).get('done') and cd and ct and cd != ct and _is_ancestor(ct, cd):
                    suspect = True
                    break
        info['suspect'] = suspect
    return tasks


def main():
    sub = sys.argv[1] if len(sys.argv) > 1 else "whats-next"
    arg = sys.argv[2] if len(sys.argv) > 2 else None
    km, kc = _kernel.start()
    for f in (META, BACKLOG):
        with open(f, encoding="utf-8") as fh:
            _kernel.run_cell(kc, fh.read())
    _, text = _kernel.run_cell(kc, f"%show {TARGET[0]}::{TARGET[1]}")
    dods = read_dods(BACKLOG)
    tasks = classify(build_model(parse_show(text), dods))

    if sub == "item" and arg:
        out = tasks.get(arg, {"error": f"no task '{arg}'"})
    elif sub == "suspect":
        out = {"suspect": sorted(t for t, i in tasks.items() if i['suspect'])}
    else:  # whats-next
        out = {"ready": sorted(t for t, i in tasks.items() if i['ready']),
               "suspect": sorted(t for t, i in tasks.items() if i['suspect']),
               "blocked": sorted(t for t, i in tasks.items() if not i['done'] and not i['ready']),
               "done": sorted(t for t, i in tasks.items() if i['done'])}
    print(json.dumps(out, indent=2))
    _kernel.teardown_and_exit(km, 0)


if __name__ == "__main__":
    main()
