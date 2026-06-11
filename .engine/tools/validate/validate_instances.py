"""Validate the .engine instance files (decisions / processes / skills-registry)
against the new flat schema. Loads schema/core + _meta first, then each instance
file. Replaces the legacy validate_sysml.py for these files.

Run:  conda run -n sysml --no-capture-output python .engine/tools/validate/validate_instances.py
"""
import os
import sys
import glob

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))
import _kernel  # noqa: E402

ENGINE = os.path.dirname(os.path.dirname(HERE))

SCHEMA = [
    "schema/core/element.sysml", "schema/core/needs.sysml", "schema/core/requirements.sysml",
    "schema/core/verification.sysml", "schema/core/work.sysml", "schema/core/architecture.sysml",
    "schema/core/computed.sysml", "schema/core/relationships.sysml", "schema/core/workflow.sysml",
    "schema/core/process.sysml", "schema/core/skills.sysml", "schema/core/risk.sysml",
    "schema/safety/stpa.sysml", "workflows/_meta.sysml",
]

INSTANCES = (sorted(glob.glob(os.path.join(ENGINE, "decisions", "*.sysml")))
             + sorted(glob.glob(os.path.join(ENGINE, "processes", "*.sysml")))
             + [os.path.join(ENGINE, "skills", "skills-registry.sysml"),
                os.path.join(ENGINE, "docs", "tracking-template.sysml")])

ERR = ("error", "couldn't", "cannot", "unexpected", "mismatched",
       "no viable", "unresolved", "extraneous", "wasn't expected")


def main():
    km, kc = _kernel.start()
    for rel in SCHEMA:
        _kernel.run_cell(kc, open(os.path.join(ENGINE, rel), encoding="utf-8").read())
    results = []
    for f in INSTANCES:
        status, text = _kernel.run_cell(kc, open(f, encoding="utf-8").read())
        bad = any(w in (text or "").lower() for w in ERR)
        rel = os.path.relpath(f, ENGINE)
        results.append((rel, not bad))
        print(f"[{'PASS' if not bad else 'FAIL'}] {rel}")
        if bad:
            print("    " + (text or "").strip().replace("\n", "\n    ")[:400])
    print("=" * 56)
    print(f"  {sum(1 for _, p in results if p)}/{len(results)} instance files passed")
    _kernel.teardown_and_exit(km, 0 if all(p for _, p in results) else 1)


if __name__ == "__main__":
    main()
