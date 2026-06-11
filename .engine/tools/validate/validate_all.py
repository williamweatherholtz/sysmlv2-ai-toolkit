"""Validate EVERY layer on ONE kernel (CR-11): schema -> workflows -> instances ->
tracking, in dependency order. One ~20s JVM startup instead of four.

Run:  conda run -n sysml --no-capture-output python .engine/tools/validate/validate_all.py
"""
import os
import sys
import glob

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))
import _kernel  # noqa: E402

ENGINE = os.path.dirname(os.path.dirname(HERE))
REPO = os.path.dirname(ENGINE)

SCHEMA = ["schema/core/element.sysml", "schema/core/needs.sysml", "schema/core/requirements.sysml",
          "schema/core/verification.sysml", "schema/core/work.sysml", "schema/core/architecture.sysml",
          "schema/core/computed.sysml", "schema/core/relationships.sysml", "schema/core/workflow.sysml",
          "schema/core/process.sysml", "schema/core/skills.sysml", "schema/core/risk.sysml",
          "schema/safety/stpa.sysml"]
WORKFLOWS = ["workflows/_meta.sysml", "workflows/business.sysml", "workflows/architecture.sysml",
             "workflows/delivery.sysml", "workflows/deploy.sysml", "workflows/operate.sysml",
             "workflows/change-request.sysml"]
INSTANCES = ([os.path.relpath(p, ENGINE) for p in sorted(glob.glob(os.path.join(ENGINE, "decisions", "*.sysml")))]
             + [os.path.relpath(p, ENGINE) for p in sorted(glob.glob(os.path.join(ENGINE, "processes", "*.sysml")))]
             + ["skills/skills-registry.sysml", "docs/tracking-template.sysml"])
TRACKING = sorted(glob.glob(os.path.join(REPO, ".tracking", "**", "*.sysml"), recursive=True))

ERR = ("error", "couldn't", "cannot", "unexpected", "mismatched",
       "no viable", "unresolved", "extraneous", "wasn't expected")


def main():
    km, kc = _kernel.start()
    results = []
    for layer, files in (("schema", [os.path.join(ENGINE, *r.split("/")) for r in SCHEMA]),
                         ("workflows", [os.path.join(ENGINE, *r.split("/")) for r in WORKFLOWS]),
                         ("instances", [os.path.join(ENGINE, *r.split("/")) for r in INSTANCES]),
                         ("tracking", TRACKING)):
        for f in files:
            status, text = _kernel.run_cell(kc, open(f, encoding="utf-8").read())
            bad = any(w in (text or "").lower() for w in ERR)
            results.append((layer, os.path.basename(f), not bad))
            if bad:
                print(f"[FAIL] {layer}/{os.path.basename(f)}")
                print("    " + (text or "").strip().replace("\n", "\n    ")[:400])
    by_layer = {}
    for layer, _, p in results:
        ok, tot = by_layer.get(layer, (0, 0))
        by_layer[layer] = (ok + (1 if p else 0), tot + 1)
    print("=" * 48)
    for layer, (ok, tot) in by_layer.items():
        print(f"  {layer:10s} {ok}/{tot}")
    allpass = all(p for _, _, p in results)
    print(f"  ALL {'GREEN' if allpass else 'FAILED'}")
    _kernel.teardown_and_exit(km, 0 if allpass else 1)


if __name__ == "__main__":
    main()
