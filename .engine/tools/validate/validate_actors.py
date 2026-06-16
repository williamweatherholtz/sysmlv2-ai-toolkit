"""Validate that authoredBy / judgedBy values in .tracking/ reference known ProjectActors.

Reads actor names from .tracking/actors.sysml (part <name> : Person/Actor entries).
Scans all .tracking/**/*.sysml for authoredBy/judgedBy attribute values.
Unknown values that are NOT in LEGACY_ACTORS exit with code 1.

Legacy actors (pre-authoredByRefs convention, 2026-06-16) are reported as WARN only.

Run:  python .engine/tools/validate/validate_actors.py
"""
import os
import re
import sys
import glob

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.dirname(os.path.dirname(os.path.dirname(HERE)))

ACTORS_FILE = os.path.join(REPO, ".tracking", "actors.sysml")
TRACKING_GLOB = os.path.join(REPO, ".tracking", "**", "*.sysml")

# Pre-convention values from early tracking entries (2026-06-10/11); reported as WARN
# Includes tool names (validate_*) used as judgedBy before the actor convention was set.
LEGACY_ACTORS = {
    "user", "demo", "inspect", "claudeOpus", "_test_suspect",
    "validate_schema", "validate_workflows", "validate_instances",
    "validate_tracking", "validate_all", "whats_next",
}

_PART_NAME_RE = re.compile(r'^\s*part\s+(\w+)\s*:\s*(?:Person|Actor)\b')
_ATTR_VAL_RE = re.compile(r':>>\s*(?:authoredBy|judgedBy)\s*=\s*"([^"]+)"')


def load_known_actors(actors_file: str) -> set:
    known = set()
    try:
        with open(actors_file, encoding="utf-8") as fh:
            for line in fh:
                m = _PART_NAME_RE.match(line)
                if m:
                    known.add(m.group(1))
    except FileNotFoundError:
        print(f"ERROR: actors.sysml not found at {actors_file}", file=sys.stderr)
        sys.exit(1)
    return known


def scan_tracking(known: set) -> tuple[int, int]:
    errors = 0
    warnings = 0
    files = sorted(glob.glob(TRACKING_GLOB, recursive=True))
    for path in files:
        relpath = os.path.relpath(path, REPO)
        with open(path, encoding="utf-8") as fh:
            for lineno, line in enumerate(fh, 1):
                for m in _ATTR_VAL_RE.finditer(line):
                    val = m.group(1)
                    if val in known:
                        continue
                    if val in LEGACY_ACTORS:
                        print(f"  WARN  {relpath}:{lineno}: legacy actor \"{val}\" (pre-convention)")
                        warnings += 1
                    else:
                        print(f"  ERROR {relpath}:{lineno}: unknown actor \"{val}\" not in ProjectActors")
                        errors += 1
    return errors, warnings


def main():
    known = load_known_actors(ACTORS_FILE)
    print(f"Known actors: {sorted(known)}")
    errors, warnings = scan_tracking(known)
    total_files = len(glob.glob(TRACKING_GLOB, recursive=True))
    print(f"\n{'=' * 56}")
    print(f"  {total_files} tracking files scanned")
    print(f"  {warnings} legacy-actor warnings (tolerated)")
    print(f"  {errors} unknown-actor errors")
    if errors:
        print("FAIL — unknown actor references must be fixed or added to actors.sysml")
        sys.exit(1)
    print("PASS")


if __name__ == "__main__":
    main()
