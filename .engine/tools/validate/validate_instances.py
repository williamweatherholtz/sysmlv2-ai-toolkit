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
from _schema_files import SCHEMA_ORDER  # noqa: E402

ENGINE = os.path.dirname(os.path.dirname(HERE))

# SCHEMA: all schema files (canonical ordered list) + workflow meta.
SCHEMA = SCHEMA_ORDER + ["workflows/_meta.sysml"]


def lint_decision_imports(engine_root):
    """Decision files must import EngineWork — that is where the Decision type lives.
    Returns list of offending relative paths."""
    failures = []
    for f in sorted(glob.glob(os.path.join(engine_root, "decisions", "*.sysml"))):
        content = open(f, encoding="utf-8").read()
        if "import EngineWork" not in content:
            failures.append(os.path.relpath(f, engine_root).replace("\\", "/"))
    return failures

INSTANCES = (sorted(glob.glob(os.path.join(ENGINE, "decisions", "*.sysml")))
             + sorted(glob.glob(os.path.join(ENGINE, "processes", "*.sysml")))
             + sorted(glob.glob(os.path.join(ENGINE, "views", "*.sysml")))
             + [os.path.join(ENGINE, "skills", "skills-registry.sysml"),
                os.path.join(ENGINE, "docs", "tracking-template.sysml")])

_ID_TYPES = ("Decision", "AISkill", "Agent", "Process", "ProcessStep", "TestResult",
             "Brief", "Persona", "Need", "Issue", "Story", "Release", "ChangeRequest",
             "Component", "DesignElement", "Test", "Viewpoint")


def warn_missing_ids(path, text):
    """Identity invariant (§2.3): every tracked instance carries :>> id. WARN (not
    fail) during bootstrap when instances outnumber ids."""
    import re as _re
    inst = len(_re.findall(r"(?:part|verification|requirement)\s+\w+\s*:\s*(?:%s)"
                           % "|".join(_ID_TYPES), text))
    ids = text.count(":>> id =")
    if inst > ids:
        print(f"    WARN {path}: {inst - ids} tracked instance(s) missing :>> id")


ERR = ("error", "couldn't", "cannot", "unexpected", "mismatched",
       "no viable", "unresolved", "extraneous", "wasn't expected")


def main():
    lint_fails = lint_decision_imports(ENGINE)
    if lint_fails:
        for rel in lint_fails:
            print(f"[LINT-FAIL] {rel}: missing 'import EngineWork::*' (Decision type is in EngineWork)")
        print("Fix the import(s) above, then re-run.")
        sys.exit(2)

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
        warn_missing_ids(rel, open(f, encoding="utf-8").read())
        if bad:
            print("    " + (text or "").strip().replace("\n", "\n    ")[:400])
    print("=" * 56)
    print(f"  {sum(1 for _, p in results if p)}/{len(results)} instance files passed")
    _kernel.teardown_and_exit(km, 0 if all(p for _, p in results) else 1)


if __name__ == "__main__":
    main()
