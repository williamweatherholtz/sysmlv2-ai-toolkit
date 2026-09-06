#!/usr/bin/env python3
"""brand-watch manifest diff (D0233).

Usage: python check_manifest.py BASELINE.json FRESH.json

Compares two brand-watch snapshots (same schema as brand/manifest.json) and
prints drift as one line per finding, keyed by entry `id` (SharePoint item ids
are stable across renames; paths are display-only):

    ADDED   <path>
    REMOVED <path>
    CHANGED <path>  (size 123 -> 456; lastModified a -> b; textSha256 drifted)

A field that is null on EITHER side is skipped for that comparison (an
unobserved value is not evidence of change). Exit code: 0 = no drift,
1 = drift found, 2 = usage/parse error.
"""
import json
import sys


def load(path):
    with open(path, encoding="utf-8") as f:
        return {e["id"]: e for e in json.load(f)["entries"]}


def main():
    if len(sys.argv) != 3:
        print(__doc__.strip(), file=sys.stderr)
        return 2
    try:
        base, fresh = load(sys.argv[1]), load(sys.argv[2])
    except (OSError, KeyError, json.JSONDecodeError) as e:
        print(f"error: {e}", file=sys.stderr)
        return 2

    drift = False
    for iid, e in fresh.items():
        if iid not in base:
            print(f"ADDED   {e['path']}")
            drift = True
    for iid, e in base.items():
        if iid not in fresh:
            print(f"REMOVED {e['path']}")
            drift = True
    for iid, b in base.items():
        f = fresh.get(iid)
        if f is None:
            continue
        changes = []
        for field in ("size", "lastModified", "textSha256"):
            bv, fv = b.get(field), f.get(field)
            if bv is not None and fv is not None and bv != fv:
                changes.append(f"{field} {bv} -> {fv}")
        if changes:
            print(f"CHANGED {b['path']}  ({'; '.join(changes)})")
            drift = True

    if not drift:
        print("no drift: fresh snapshot matches baseline on every observed field")
    return 1 if drift else 0


if __name__ == "__main__":
    sys.exit(main())
