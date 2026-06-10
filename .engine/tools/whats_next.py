"""whats-next (v2): resolve each workflow's action DAG into ordered, parallel-
grouped waves, as JSON.

Workflows are native SysML `action def`s. This reads the model via the pilot
kernel's `%show <Pkg>::<ActionDef>` and parses the typed AST:
  * phases        = `ActionUsage` members (with the item types they touch)
  * dependencies  = `SuccessionAsUsage` (earlierOccurrence -> laterOccurrence)
The DAG is the successions; Kahn layering gives the parallel waves. (Order +
parallelism are native SysML, not a custom produces/consumes attribute.)

v2 resolves the workflow DEFINITIONS (process shape). Instance-aware "what's next
to DO" (reading .tracking/ work-items + done-state) is the next step.

Run (sandbox disabled; kernel calls bare java -> go through conda run):
  conda run -n sysml --no-capture-output python .engine/tools/whats_next.py
"""
import os
import sys
import re
import json

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import _kernel  # noqa: E402

ENGINE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))  # .engine
REPO = os.path.dirname(ENGINE)
WF_DIR = os.path.join(ENGINE, "workflows")
BACKLOG = os.path.join(REPO, ".tracking", "backlog.sysml")  # self-build plan (dogfood)

LOAD = ["_meta.sysml", "business.sysml", "architecture.sysml", "delivery.sysml",
        "deploy.sysml", "operate.sysml", "change-request.sysml"]
# (package, action def) per workflow
WORKFLOWS = [("BusinessWorkflow", "Business"), ("ArchitectureWorkflow", "Architecture"),
             ("DeliveryWorkflow", "Delivery"), ("DeployWorkflow", "Deploy"),
             ("OperateWorkflow", "Operate"), ("ChangeRequestWorkflow", "ChangeMgmt")]

_UUID = re.compile(r'^(.*?)\s*\([0-9a-fA-F-]{36}\)\s*$')


def parse_show(text):
    """Parse a `%show` indented AST into a tree of {rel,type,name,children}."""
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


def phases(root):
    """ActionUsage members of the action def -> [{name, artifacts:[item types]}]."""
    out = []
    for c in root['children']:
        if c['rel'] == 'FeatureMembership' and c['type'] == 'ActionUsage':
            arts = []
            for p in c['children']:
                if p['rel'] == 'FeatureMembership' and p['type'] == 'ReferenceUsage':
                    it = next((g['name'] for g in p['children'] if g['type'] == 'ItemDefinition'), None)
                    if it:
                        arts.append(it)
            out.append({'name': c['name'], 'artifacts': sorted(set(arts))})
    return out


def succession_edges(root):
    """SuccessionAsUsage members -> [(earlier, later)] action-name pairs."""
    edges = []
    for c in root['children']:
        if c['type'] == 'SuccessionAsUsage':
            ends = {}
            for e in c['children']:
                if e['rel'] == 'EndFeatureMembership':
                    act = next((g['name'] for g in e['children']
                                if g['rel'] == 'ReferenceSubsetting' and g['type'] == 'ActionUsage'), None)
                    ends[e['name']] = act
            a, b = ends.get('earlierOccurrence'), ends.get('laterOccurrence')
            if a and b:
                edges.append((a, b))
    return edges


def waves(ph, edges):
    """Kahn layering. Action Y depends on X iff there is a succession X -> Y."""
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


def main():
    km, kc = _kernel.start()
    for fn in LOAD:
        with open(os.path.join(WF_DIR, fn), encoding="utf-8") as fh:
            _kernel.run_cell(kc, fh.read())
    resolve = list(WORKFLOWS)
    if os.path.exists(BACKLOG):
        with open(BACKLOG, encoding="utf-8") as fh:
            _kernel.run_cell(kc, fh.read())
        resolve.append(("EngineBacklog", "EngineBuild"))
    result = {"schema": "whats-next.v2", "workflows": []}
    for pkg, act in resolve:
        _, text = _kernel.run_cell(kc, f"%show {pkg}::{act}")
        root = parse_show(text)
        ph = phases(root)
        w, cycle = waves(ph, succession_edges(root))
        wf = {"workflow": act, "package": pkg, "phaseCount": len(ph)}
        if cycle:
            wf["error"], wf["cycle"] = "dependency cycle", cycle
        else:
            wf["waves"] = [[{"action": p['name'], "artifacts": p['artifacts']} for p in wave]
                           for wave in w]
        result["workflows"].append(wf)
    print(json.dumps(result, indent=2))
    _kernel.teardown_and_exit(km, 0)


if __name__ == "__main__":
    main()
