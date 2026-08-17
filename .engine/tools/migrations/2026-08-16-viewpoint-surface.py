#!/usr/bin/env python
"""Populate `surface` on every declared Viewpoint (D0154).

WHY THIS IS A MIGRATION AND NOT AN EDIT
---------------------------------------
D0154 adds `attribute surface : String[0..1]` to the Viewpoint type. 28 viewpoints predate it and
are owned by `wweatherholtz`, so a non-owner adding the field to each is exactly what D0108 forbids
— the `ownership` guard blocks it, correctly. The two sanctioned ways out are to have the owner make
the change by hand across 28 items, or to supersede each with a corrected copy, which would double
the registry to add one attribute.

D0067's migration path is the third and the right one: a bulk transform that crosses every ownership
boundary at once, carrying a COMMITTED transform as the record of why. The `ownership` guard grants
its exemption only when a file under `.engine/tools/migrations/` is co-committed — it cannot be
claimed, it has to be in the commit where a reviewer can read it. This file is that record.

PROPERTIES (D0067)
------------------
* IDEMPOTENT — a viewpoint that already declares `surface` is left untouched, so re-running is a
  no-op rather than a double-application.
* RECONCILED — control totals are printed before and after, and the script REFUSES (exit 2) if the
  viewpoint count changes at all. This transform must only ever add attributes; if it has added or
  lost a viewpoint, something is wrong with it and not with the data.
* TOTAL, or it says so — a viewpoint whose title is not in the mapping is REPORTED and left with no
  surface, never guessed at. An unmapped viewpoint then shows up in the console under `unsurfaced`,
  which is the visible-gap behaviour N-C2 requires.

Usage:  python .engine/tools/migrations/2026-08-16-viewpoint-surface.py [--dry-run]
"""
import io
import re
import sys
from pathlib import Path

REGISTRY = Path(".engine/views/viewpoint-registry.sysml")

# title -> surface. Six surfaces, each answering a different question a supervisor has.
SURFACE = {
    "orient": "work", "whats-next": "work", "suspect": "work", "reprocess-candidates": "work",
    "traceability": "assurance", "requirements": "assurance", "safety": "assurance",
    "orphans": "assurance", "tier-satisfaction": "assurance", "rootedness": "assurance",
    "attestation-coverage": "assurance", "critique-policy": "assurance",
    "concern-coverage": "assurance", "verification-examined": "assurance",
    "verification-exercised": "assurance",
    "arch-elements": "architecture", "arch-criticality": "architecture",
    "arch-coupling": "architecture", "arch-drift": "architecture",
    "arch-stpa-inputs": "architecture", "arch-coverage": "architecture",
    "decisions": "record", "baselines": "record", "governing-version": "record",
    "diagram": "explore", "render": "explore", "report": "explore", "indicators": "explore",
    "acceptances-pending": "act", "findings-awaiting-disposition": "act",
    "authority-queue": "act", "sitting-review-due": "act",
}

VP = re.compile(r"part (\w+) : Viewpoint \{(.*?)\n    \}", re.S)


def main() -> int:
    dry = "--dry-run" in sys.argv
    if not REGISTRY.exists():
        print(f"error: {REGISTRY} not found — run from the repo root")
        return 2
    text = io.open(REGISTRY, encoding="utf-8").read()

    before = len(VP.findall(text))
    already = text.count(':>> surface = "')
    print(f"before: {before} viewpoint(s), {already} already carrying `surface`")

    unmapped = []
    added = [0]

    def fix(m):
        name, body = m.group(1), m.group(2)
        if ":>> surface" in body:
            return m.group(0)  # idempotent
        t = re.search(r'title = "([^"]*)"', body)
        title = t.group(1) if t else ""
        s = SURFACE.get(title)
        if not s:
            unmapped.append(f"{name} ({title})")
            return m.group(0)  # reported, never guessed
        added[0] += 1
        body2 = re.sub(r'(\n(\s*):>> renderer = "[^"]*";)',
                       r'\1\n\2:>> surface = "%s";' % s, body, count=1)
        return "part %s : Viewpoint {%s\n    }" % (name, body2)

    out = VP.sub(fix, text)

    after = len(VP.findall(out))
    if after != before:
        print(f"REFUSING: viewpoint count changed {before} -> {after}. This transform only ADDS "
              f"attributes; a changed count means the transform is wrong, not the data.")
        return 2

    print(f"after:  {after} viewpoint(s), {out.count(chr(58) + chr(62) + chr(62) + ' surface = ')} carrying `surface` "
          f"({added[0]} added this run)")
    if unmapped:
        print(f"UNMAPPED, left with no surface and visible as `unsurfaced` in the console: {unmapped}")

    if dry:
        print("dry run — nothing written")
        return 0
    io.open(REGISTRY, "w", encoding="utf-8", newline="\n").write(out)
    print("written")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
