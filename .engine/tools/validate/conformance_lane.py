"""The CONFORMANCE lane: measure how much of this model the OMG SysML v2 kernel accepts.

WHY THIS IS A SEPARATE LANE AND NOT A GATE (issue097, D0048, D0132).

`keel validate` / `keel check` are the ENGINE's semantic authority: reference resolution, identity,
provenance, edge algebra. They are NOT a SysML v2 conformance check, and nothing said so — so
"validate is green" read as "this is valid SysML v2". It is not the same claim. The Rust parser
accepts `verify X by Y` at package level; the kernel rejects it. That divergence is how a
non-conformant construct reached an ACCEPTED Decision as a migration target (D0139 clause E).

The property the human actually cares about is whether this model opens in a standard viewer, and
that property was unmeasured. This lane measures it.

IT MUST NEVER BLOCK A COMMIT. D0132 demoted the per-file instance validator precisely because it
fails correct files, which forced an all-or-nothing bypass and thereby disabled every OTHER layer
(issue081). Repeating that would be the same mistake with a new name. So: this lane reports, and its
number is tracked as an INDICATOR. It exits 0 even when it finds non-conformance, unless you ask for
`--strict`, which exists for a human running it deliberately and is never wired into a hook.

READ THE NUMBER CORRECTLY. A non-conforming file is not necessarily a wrong file — the kernel is a
pilot implementation with its own gaps, and some rejections are the kernel's, not ours. The lane
reports the construct so a human can tell the two apart; it does not adjudicate.

Run (never pipe — the JVM holds the pipe and the shell hangs; redirect to a file):
  conda run -n sysml --no-capture-output python .engine/tools/validate/conformance_lane.py > out.txt 2>&1
  conda run -n sysml --no-capture-output python .engine/tools/validate/conformance_lane.py --construct snippet.sysml > out.txt 2>&1
"""
import argparse
import glob
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))  # .engine/tools
import _kernel  # noqa: E402
from _schema_files import SCHEMA_ORDER  # noqa: E402

ENGINE = os.path.dirname(os.path.dirname(HERE))
REPO = os.path.dirname(ENGINE)
PRELOAD = ([os.path.join(ENGINE, *rel.split("/")) for rel in SCHEMA_ORDER]
           + [os.path.join(ENGINE, "workflows", "_meta.sysml")])

# The kernel signals a problem in prose rather than a status code, so this is a substring set.
# Kept identical to validate_tracking.py's on purpose: two different answers to "did the kernel
# object" would be a second source of truth for the same fact.
ERR = ("error", "couldn't", "cannot", "unexpected", "mismatched",
       "no viable", "unresolved", "extraneous", "wasn't expected")


def instance_files():
    """Every INSTANCE file: `.tracking/` plus the `.engine` instance directories.

    Schema and workflow definitions are preloaded rather than measured — they are the vocabulary the
    instances are checked against, so a schema failure would surface as every instance failing and
    tell you nothing about which construct is at fault.
    """
    out = sorted(glob.glob(os.path.join(REPO, ".tracking", "**", "*.sysml"), recursive=True))
    for sub in ("decisions", "processes", "skills", "rules", "views"):
        out += sorted(glob.glob(os.path.join(ENGINE, sub, "**", "*.sysml"), recursive=True))
    return out


def first_diagnostic(text):
    """The kernel's first objection, trimmed to something a human can act on."""
    for line in (text or "").splitlines():
        low = line.lower()
        if any(w in low for w in ERR):
            return line.strip()[:300]
    return (text or "").strip().replace("\n", " ")[:300]


def construct_of(diagnostic, source):
    """Best-effort: the source line the kernel objected to.

    The pilot kernel's diagnostics do not carry a stable line number, so this greps the source for
    the quoted token when there is one. Returns None rather than guessing — an invented construct
    would be worse than no construct, because it is the field a reader would act on.
    """
    quoted = re.findall(r"'([^']{2,60})'", diagnostic or "")
    for q in quoted:
        for i, line in enumerate(source.splitlines(), 1):
            if q in line:
                return f"{i}: {line.strip()[:160]}"
    return None


DECL_RE = re.compile(
    r"^\s*(?:#\w+\s+)?(?:abstract\s+)?"
    r"(?:package|part|action|verification|requirement|occurrence|item|enum|metadata|port|connection|constraint|allocation|use\s+case)"
    r"\s+(?:def\s+)?(\w+)", re.M)


def declared_names():
    """Every name DECLARED anywhere in the model.

    Used to tell a lane artifact from a real finding. The kernel evaluates one file per cell in one
    session, so a file that references a name declared in a file not yet loaded reports
    "Couldn't resolve reference" — which says nothing about SysML v2 conformance and everything
    about the order this script happened to iterate in. Without this split the lane reported 190/470
    and would have published a 40% conformance rate that is entirely its own artifact. Whether a
    reference actually resolves is `keel validate`'s job and it already answers it.
    """
    names = set()
    for f in instance_files() + PRELOAD:
        try:
            names.update(DECL_RE.findall(open(f, encoding="utf-8").read()))
        except OSError:
            pass
    return names


UNRESOLVED_RE = re.compile(r"Couldn't resolve reference to \w+ '([^']+)'")


def classify(diagnostic, known):
    """`conformance` (the kernel rejected the CONSTRUCT), `ordering` (a lane artifact), or
    `unresolved` (a genuinely undeclared name, which validate also reports)."""
    m = UNRESOLVED_RE.search(diagnostic or "")
    if not m:
        return "conformance"
    return "ordering" if m.group(1) in known else "unresolved"


def sweep(kc):
    files = instance_files()
    known = declared_names()
    results = []
    for f in files:
        src = open(f, encoding="utf-8").read()
        _status, text = _kernel.run_cell(kc, src)
        bad = any(w in (text or "").lower() for w in ERR)
        rel = os.path.relpath(f, REPO).replace("\\", "/")
        if bad:
            diag = first_diagnostic(text)
            kind = classify(diag, known)
            results.append({"file": rel, "conforms": kind != "conformance", "kind": kind,
                            "diagnostic": diag, "construct": construct_of(diag, src)})
        else:
            results.append({"file": rel, "conforms": True, "kind": "clean"})
    return results


def check_construct(kc, path):
    """Rule (b): a NEW base construct is kernel-validated BEFORE adoption, never on the Rust parser
    alone. This is that check, runnable — the Rust parser accepting a construct means only that keel
    can read it, which is a strictly weaker claim than a standard tool being able to."""
    src = open(path, encoding="utf-8").read()
    _status, text = _kernel.run_cell(kc, src)
    bad = any(w in (text or "").lower() for w in ERR)
    diag = first_diagnostic(text) if bad else None
    print(f"construct: {path}")
    print(f"  kernel verdict: {'REJECTED' if bad else 'ACCEPTED'}")
    if bad:
        print(f"  diagnostic: {diag}")
        print("  DO NOT ADOPT this construct on the Rust parser's acceptance alone.")
    return 0 if not bad else 1


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--construct", metavar="FILE", help="kernel-check ONE snippet before adopting it")
    ap.add_argument("--json", metavar="OUT", help="write the full result set as JSON")
    ap.add_argument("--strict", action="store_true", help="exit non-zero on non-conformance (never wire this into a hook)")
    args = ap.parse_args()

    km, kc = _kernel.start()
    for f in PRELOAD:
        _kernel.run_cell(kc, open(f, encoding="utf-8").read())

    if args.construct:
        code = check_construct(kc, args.construct)
        _kernel.teardown_and_exit(km, code if args.strict else 0)

    results = sweep(kc)
    total = len(results)
    bad = [r for r in results if r.get("kind") == "conformance"]
    ordering = [r for r in results if r.get("kind") == "ordering"]
    unresolved = [r for r in results if r.get("kind") == "unresolved"]
    print("=" * 72)
    print(f"CONFORMANCE LANE — {total - len(bad)}/{total} instance file(s) accepted by the OMG kernel")
    print("=" * 72)
    for r in bad:
        print(f"  NON-CONFORMING  {r['file']}")
        print(f"      {r['diagnostic']}")
        if r["construct"]:
            print(f"      at {r['construct']}")
    if not bad:
        print("  no CONSTRUCT was rejected by the kernel.")
    print()
    print(f"  {len(ordering)} file(s) reported an unresolved reference to a name that IS declared")
    print("  elsewhere in the model — a LANE artifact (one file per cell, in iteration order), not")
    print("  a conformance result. Excluded deliberately: counting them reported 190/470 and would")
    print("  have published a 40% conformance rate that is entirely this script's own doing.")
    if unresolved:
        print(f"  {len(unresolved)} file(s) reference a name declared NOWHERE — a real defect, but")
        print("  `keel validate` is the authority on that and already reports it:")
        for r in unresolved[:10]:
            print(f"      {r['file']}: {r['diagnostic'][:140]}")
    print()
    print("  This lane REPORTS; it never blocks (D0132/issue081). A rejection may be the kernel's")
    print("  gap rather than the model's — the construct is printed so a human can tell which.")
    print(f"  indicator value: conformanceRate = {round(100.0 * (total - len(bad)) / total, 1) if total else 0.0}%")

    if args.json:
        with open(args.json, "w", encoding="utf-8") as fh:
            json.dump({"total": total, "conforming": total - len(bad), "nonConforming": bad,
                       "orderingArtifacts": len(ordering), "unresolvedNames": unresolved}, fh, indent=2)
        print(f"  wrote {args.json}")

    _kernel.teardown_and_exit(km, 1 if (bad and args.strict) else 0)


if __name__ == "__main__":
    main()
