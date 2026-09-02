#!/usr/bin/env python3
"""Rewrite every `keel <lens>` call site to `keel show <lens>` — the D0273 clean break.

D0273: 35 read-only lens verbs collapse into one `keel show <lens>` router and the old spellings are
REMOVED, not aliased — the human's choice over an alias window. That leaves 141 call sites across the
engine's skills, processes, docs, contracts and views naming verbs the binary no longer dispatches,
and guard 39 (tool-reference) rightly fails each one. The rewrite and the removal must land in ONE
commit so no tree ever exists in which a skill names a command the binary lacks.

WHY THIS IS A D0067 MIGRATION AND NOT A SCRIPT. The first attempt did this rewrite with an ad-hoc
script in a scratchpad and hit two controls:
  - `ownership` (D0108): ten declared Viewpoints are owned by the human, and the rewrite edited their
    `renderer` field. A non-owner never overwrites in place.
  - The out-of-band-writes hook: the rewrite also touched 27 ACCEPTED Decision files, changing command
    names inside text a human had attested to.
Both are the shape D0067 exists for — a mechanical transform crossing every ownership boundary at
once — and D0067's own remedy is what resolves both: a COMMITTED transform (this file), a DRY RUN
that reconciles control totals, and the `ownership` guard's sanctioned exemption for a co-committed
transform under .engine/tools/migrations/. The transform is the record of why the boundary was
crossed, in the commit, where a reviewer can read it.

WHAT IT DELIBERATELY DOES NOT TOUCH.
  - `.engine/decisions/` and any `reference/decisions/`: a Decision names the commands that existed
    when it was signed. Keeping attested text "current" is not a goal, and tool-reference never scans
    decisions, so nothing there needs to change.
  - `.tracking/`: instance records are history. A retro that quoted the OLD spelling said that on the day.
  - `.engine/tools/migrations/`: this file's own examples, or the transform rewrites itself (it did, once).
  - Source and tests: those are code, not the authored surface, and were rewritten by hand with the
    router itself.

Idempotent by construction: `keel show show orphans` is not produced, because the pattern requires
`keel ` immediately followed by a lens name, and `show` is not a lens.

Usage:
    python .engine/tools/migrations/2026-09-01-lens-verbs-to-show-router.py --dry-run
    python .engine/tools/migrations/2026-09-01-lens-verbs-to-show-router.py --apply
"""
import io
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))

# The 35 lenses `cmd_show` routes — held equal to cli_surface::LENS_NAMES by test on the Rust side.
LENSES = """assumptions attestation-coverage authority-queue boundary boundary-sweep business
concern-coverage contentions controls coverage critique-coverage critique-policy
decision-follow-through decisions dispositions hardening indicators intake knowledge launchables ls
marker-census open-issues orphans outstanding recent rootedness sitting-coverage suspect
tier-satisfaction trace trace-need verification why workflows""".split()

# Longest first, so `trace-need` is not eaten by `trace` and `critique-coverage` not by `coverage`.
LENSES.sort(key=len, reverse=True)
PATTERN = re.compile(r"\bkeel (%s)\b" % "|".join(re.escape(l) for l in LENSES))

SCAN_ROOTS = [".engine", "CLAUDE.md", "README.md"]
EXTENSIONS = {"md", "sysml", "toml", "txt", "yml", "yaml", "sh", "py"}


def excluded(rel):
    """Attested and historical text is never rewritten — see the module docstring."""
    r = rel.replace("\\", "/")
    return "/decisions/" in r or r.startswith(".tracking/") or "/tools/migrations/" in r


def targets():
    for base in SCAN_ROOTS:
        full = os.path.join(ROOT, base)
        if os.path.isfile(full):
            yield base
            continue
        for dp, _, fs in os.walk(full):
            for f in fs:
                if f.rsplit(".", 1)[-1].lower() in EXTENSIONS:
                    rel = os.path.relpath(os.path.join(dp, f), ROOT)
                    if not excluded(rel):
                        yield rel


def plan():
    """(rel_path, hits, rewritten_text) for every file the transform would change."""
    out = []
    for rel in targets():
        try:
            text = io.open(os.path.join(ROOT, rel), encoding="utf-8").read()
        except (OSError, UnicodeDecodeError):
            continue
        hits = len(PATTERN.findall(text))
        if hits:
            out.append((rel, hits, PATTERN.sub(lambda m: "keel show " + m.group(1), text)))
    return out


def main(argv):
    if len(argv) != 2 or argv[1] not in ("--dry-run", "--apply"):
        print(__doc__)
        return 2
    p = plan()
    files = len(p)
    sites = sum(h for _, h, _ in p)
    print("lens-verbs-to-show-router: %d call site(s) across %d file(s)" % (sites, files))
    for rel, hits, _ in sorted(p, key=lambda x: -x[1]):
        print("  %4d  %s" % (hits, rel))
    if argv[1] == "--dry-run":
        print("dry run: nothing written. Control totals above; re-run with --apply.")
        return 0
    for rel, _, new in p:
        io.open(os.path.join(ROOT, rel), "w", encoding="utf-8", newline="").write(new)
    # Reconcile: a second plan must be EMPTY, or the transform is not idempotent.
    remaining = sum(h for _, h, _ in plan())
    print("applied. re-plan finds %d remaining call site(s)%s" % (remaining, "" if remaining == 0 else " — NOT IDEMPOTENT"))
    return 0 if remaining == 0 else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
