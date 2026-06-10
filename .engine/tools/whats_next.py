"""whats-next (v1): resolve each workflow's phase dependency-DAG into ordered,
parallel-grouped waves, emitted as JSON.

Reads the model via the pilot kernel's `%show` (the standard-JSON `%export` is a
no-op in this kernel build; `%show` dumps the full typed AST over iopub). v1
resolves the workflow DEFINITIONS (the process *shape*): order + parallelism are
COMPUTED from each phase's consumes/produces (the DAG). Instance-aware
"what's next to DO" (reading .tracking/ work-items + done-state) is v2.

KNOWN LIMITATION (v1): a multi-valued feature value `(a, b, c)` is rendered by
`%show` as an opaque `OperatorExpression ","` whose operands are NOT printed, so
phases with multiple produces/consumes currently parse as empty (Architecture,
Delivery, Deploy under-resolve). Fix pending: model produces/consumes as discrete
typed edges (one per phase<->artifact) so `%show` renders each parseably.

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
WF_DIR = os.path.join(ENGINE, "workflows")

LOAD = ["_meta.sysml", "business.sysml", "architecture.sysml", "delivery.sysml",
        "deploy.sysml", "operate.sysml", "change-request.sysml"]
WORKFLOW_PKGS = ["BusinessWorkflow", "ArchitectureWorkflow", "DeliveryWorkflow",
                 "DeployWorkflow", "OperateWorkflow", "ChangeRequestWorkflow"]

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
        (stack[-1][1]['children'].append(node) if stack else None)
        if not stack:
            root = node
        stack.append((indent, node))
    return root


def _member(node, fname):
    for c in node['children']:
        if c['rel'] == 'FeatureMembership' and c['name'] == fname:
            return c
    return None


def _collect(node, etype):
    out = []
    def walk(n):
        if n['type'] == etype:
            out.append(n['name'])
        for ch in n['children']:
            walk(ch)
    walk(node)
    return out


def _literal(node, fname):
    fm = _member(node, fname)
    vals = _collect(fm, 'LiteralString') if fm else []
    return vals[0] if vals else None


def _refs(node, fname):
    fm = _member(node, fname)
    return _collect(fm, 'FeatureReferenceExpression') if fm else []


def extract(root):
    """Workflow package node -> list of phase dicts."""
    phases = []
    for c in root['children']:
        if c['type'] != 'PartUsage':
            continue
        typed = next((x['name'] for x in c['children'] if x['rel'] == 'FeatureTyping'), None)
        if typed == 'Phase':
            phases.append({'name': c['name'], 'title': _literal(c, 'title'),
                           'produces': _refs(c, 'produces'), 'consumes': _refs(c, 'consumes')})
    return phases


def waves(phases):
    """Kahn waves. Phase B depends on phase A iff B.consumes ∩ A.produces != {}
    (matched by ArtifactType name). Returns (waves, cycle_remainder)."""
    produced_by = {}
    for p in phases:
        for a in p['produces']:
            produced_by.setdefault(a, set()).add(p['name'])
    deps = {}
    for p in phases:
        d = set()
        for a in p['consumes']:
            d |= produced_by.get(a, set())
        d.discard(p['name'])
        deps[p['name']] = d
    by_name = {p['name']: p for p in phases}
    out, done, remaining = [], set(), set(by_name)
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
    result = {"schema": "whats-next.v1", "workflows": []}
    for pkg in WORKFLOW_PKGS:
        _, text = _kernel.run_cell(kc, f"%show {pkg}")
        phases = extract(parse_show(text))
        w, cycle = waves(phases)
        wf = {"workflow": pkg, "phaseCount": len(phases)}
        if cycle:
            wf["error"], wf["cycle"] = "dependency cycle", cycle
        else:
            wf["waves"] = [[{"phase": p['name'], "title": p['title'],
                             "consumes": p['consumes'], "produces": p['produces']}
                            for p in wave] for wave in w]
        result["workflows"].append(wf)
    print(json.dumps(result, indent=2))
    _kernel.teardown_and_exit(km, 0)


if __name__ == "__main__":
    main()
