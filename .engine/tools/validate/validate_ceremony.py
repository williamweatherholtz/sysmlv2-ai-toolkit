"""Guard the sprint-ceremony gate ordering invariant (issue010, D0047).

Invariant: within a `.tracking/delivery/*.sysml` file, no ceremony gate may have a
passing TestResult while an EARLIER-ordered gate that is DEFINED in the same file
is unpassed. A clean in-progress prefix (Refine..Review passed, CloseOut/Retro not
yet) is fine; a GAP (e.g. Implement passed while Standup is skipped) is a subversion.

Pure-Python, no kernel — safe in the pre-commit hook. Hard-fails (exit 1) on any
NEW violation; historical anomalies are grandfathered (WARN only), mirroring the
LEGACY_ACTORS pattern in validate_actors.py.

Why this exists: Sprint 16 recorded Refine->Implement->Review with Standup skipped;
it was caught only because orient happened to be run. Views surfaced it; nothing
PREVENTED it. This guard makes the prevention permanent (CLAUDE.md §5 discipline).

Run:  python .engine/tools/validate/validate_ceremony.py
"""
import os
import re
import sys
import glob

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(os.path.dirname(HERE)))
DELIVERY_GLOB = os.path.join(REPO, ".tracking", "delivery", "*.sysml")

GATE_ORDER = ["Refine", "Standup", "Implement", "Review", "CloseOut", "Retro"]

# Delivery files whose CURRENT gate record pre-dates this guard (issue010, 2026-06-16)
# and is accepted as historical. New files / new violations are NOT grandfathered.
GRANDFATHERED = {
    "sprint11_nativeSpikes",  # recorded only CloseOut+Retro; earlier gates never recorded
}

# A gate is "defined" if the file declares its verification; "passed" if a matching
# part <...>GateR<n> : TestResult has outcome = pass.
_GATE_DEF = re.compile(r"verification\s+\w*?(" + "|".join(GATE_ORDER) + r")Gate\b")
_GATE_PASS = re.compile(
    r"part\s+\w*?(" + "|".join(GATE_ORDER) + r")Gate\w*R\d+\s*:\s*TestResult"
    r"[^}]*?:>>\s*outcome\s*=\s*VerdictKind::pass",
    re.DOTALL,
)


def analyze(path):
    """Return (defined:set, passed:set) of gate names for one delivery file."""
    with open(path, encoding="utf-8") as fh:
        text = fh.read()
    defined = {m.group(1) for m in _GATE_DEF.finditer(text)}
    passed = {m.group(1) for m in _GATE_PASS.finditer(text)}
    defined |= passed  # a passed gate is implicitly defined
    return defined, passed


def violations(defined, passed):
    """List (passed_gate, missing_earlier_gate) ordering violations."""
    out = []
    for i, g in enumerate(GATE_ORDER):
        if g not in passed:
            continue
        for j in range(i):
            earlier = GATE_ORDER[j]
            if earlier in defined and earlier not in passed:
                out.append((g, earlier))
    return out


def main():
    files = sorted(glob.glob(DELIVERY_GLOB))
    errors = 0
    warnings = 0
    for path in files:
        stem = os.path.splitext(os.path.basename(path))[0]
        defined, passed = analyze(path)
        viols = violations(defined, passed)
        if not viols:
            continue
        detail = "; ".join(f"{g} passed but {e} (earlier) unpassed" for g, e in viols)
        if stem in GRANDFATHERED:
            print(f"  WARN  {stem}: {detail} (grandfathered, pre-issue010)")
            warnings += 1
        else:
            print(f"  ERROR {stem}: {detail}")
            errors += 1
    print(f"\n{'=' * 56}")
    print(f"  {len(files)} delivery files scanned")
    print(f"  {warnings} grandfathered warnings (tolerated)")
    print(f"  {errors} ceremony-ordering violations")
    if errors:
        print("FAIL — a gate was recorded out of order; record the skipped earlier "
              "gate(s) or fix the sequence before committing.")
        sys.exit(1)
    print("PASS")


if __name__ == "__main__":
    main()
