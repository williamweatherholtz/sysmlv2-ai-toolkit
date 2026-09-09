#!/usr/bin/env python3
# not-an-instrument: a one-time D0067 transform over one tracking file; it measures nothing.
"""D0067 MIGRATE step for D0400 (2026-09-09): a Release carries the tag it names as its own field.

Before: the only binding between a git tag and its `Release` record was `title.contains(tag)`, first hit
(guard release-recorded). issue387: release040's title honestly says its payload shipped as v0.4.1, so it was
the first hit for the v0.4.1 tag, and two correct records were reported as a disagreement. Containment is
also unanchored: v0.4.1 would vouch for a v0.4.11 tag.

After (this transform, applied once): every `Release` block whose title OPENS with a version token
`vN.N.N` gains `:>> tag = "vN.N.N";` on the line after its `:>> commit` line. The token is taken from the
record's own title, first whitespace-delimited word only - a version named later in the prose (release040
mentions v0.4.1) is never read. Blocks whose title opens with anything else (three named milestones) are
left as they are: `tag` is [0..1], and a milestone that shipped no tag has none.

Control totals (gate 2 of the `migration` skill) - the run FAILS before writing when any does not balance:

  * conservation:  tagged-record count after == title-opens-with-version count before (expected 10: nine
                   distinct tags, v0.2.0 recorded twice - release020 retired by release020Shipped);
  * coverage:      every `git tag` matching vN.N.N is the tag of at least one record after (9 of 9);
  * no-leak:       tagged + untagged + already-tagged == Release blocks (13);
  * content hash:  SHA-256 over every line of the file EXCEPT the inserted ones is equal before and after.

Idempotent: a block already carrying `:>> tag =` is skipped and counted. Line endings preserved per file.
Dry run is the default; `--apply` writes.

Run from the repository root:

    python .engine/tools/migrations/2026-09-09-release-tag-field.py            # dry run + reconcile
    python .engine/tools/migrations/2026-09-09-release-tag-field.py --apply    # write, then re-reconcile
"""
import hashlib
import re
import subprocess
import sys
from pathlib import Path

FILE = Path(".tracking/baselines.sysml")
VERSION = re.compile(r"^v\d+\.\d+\.\d+$")


def blocks(lines):
    """Yield (start, end) index pairs of `part X : Release {` ... `}` blocks (indentation-matched close)."""
    i = 0
    while i < len(lines):
        l = lines[i]
        if l.lstrip().startswith("part ") and ": Release {" in l:
            indent = len(l) - len(l.lstrip())
            j = i + 1
            while j < len(lines) and not (lines[j].rstrip() == " " * indent + "}"):
                j += 1
            yield i, j
            i = j
        i += 1


def plan(lines):
    """Return (inserts, counts): inserts = [(index_after_which, text)], counts by category."""
    inserts, counts = [], {"tagged": 0, "untagged": 0, "already": 0, "blocks": 0}
    for s, e in blocks(lines):
        counts["blocks"] += 1
        body = lines[s : e + 1]
        if any(x.lstrip().startswith(":>> tag = ") for x in body):
            counts["already"] += 1
            continue
        title = next((x for x in body if x.lstrip().startswith(':>> title = "')), None)
        commit_i = next((k for k, x in enumerate(body) if x.lstrip().startswith(":>> commit = ")), None)
        if title is None or commit_i is None:
            counts["untagged"] += 1
            continue
        first = title.split('"', 1)[1].split()[0] if title.split('"', 1)[1].strip() else ""
        if not VERSION.match(first):
            counts["untagged"] += 1
            continue
        indent = body[commit_i][: len(body[commit_i]) - len(body[commit_i].lstrip())]
        inserts.append((s + commit_i, f'{indent}:>> tag = "{first}";'))
        counts["tagged"] += 1
    return inserts, counts


def content_hash(lines, skip_texts):
    h = hashlib.sha256()
    for l in lines:
        if l.strip() in skip_texts:
            continue
        h.update(l.encode("utf-8"))
        h.update(b"\n")
    return h.hexdigest()


def main(apply):
    raw = FILE.read_bytes()
    nl = "\r\n" if b"\r\n" in raw else "\n"
    lines = raw.decode("utf-8").split(nl)
    inserts, counts = plan(lines)
    tags = [
        t.strip()
        for t in subprocess.run(["git", "tag"], capture_output=True, text=True, check=True).stdout.splitlines()
        if VERSION.match(t.strip())
    ]
    new = list(lines)
    for idx, text in sorted(inserts, reverse=True):
        new.insert(idx + 1, text)
    tagged_after = [x.split('"')[1] for x in new if x.lstrip().startswith(':>> tag = "')]
    skip = {t.strip() for _, t in inserts}
    ok = True

    def check(name, cond, detail):
        nonlocal ok
        print(f"  {'ok  ' if cond else 'FAIL'} {name}: {detail}")
        ok = ok and cond

    print(f"{'APPLY' if apply else 'DRY RUN'} {FILE}")
    check("conservation", len(tagged_after) == counts["tagged"] + counts["already"], f"tagged after {len(tagged_after)} == opens-with-version {counts['tagged']} + already {counts['already']}")
    missing = [t for t in tags if t not in tagged_after]
    check("coverage", not missing, f"{len(tags) - len(missing)} of {len(tags)} git tags have a record naming them{'; missing ' + ', '.join(missing) if missing else ''}")
    check("no-leak", counts["tagged"] + counts["untagged"] + counts["already"] == counts["blocks"], f"{counts['tagged']} + {counts['untagged']} + {counts['already']} == {counts['blocks']} Release blocks")
    check("content", content_hash(lines, set()) == content_hash(new, skip), "SHA-256 over every non-inserted line equal")
    for idx, text in inserts:
        print(f"    line {idx + 2}: {text.strip()}")
    if not ok:
        print("totals do not balance: nothing written")
        return 2
    if apply:
        if inserts:
            FILE.write_bytes(nl.join(new).encode("utf-8"))
            print(f"wrote {len(inserts)} insert(s)")
            again, c2 = plan(nl.join(new).split(nl))
            print(f"re-reconcile: {len(again)} left to change, already {c2['already']}, untagged {c2['untagged']}")
            return 0 if not again else 2
        print("nothing to change")
    return 0


if __name__ == "__main__":
    sys.exit(main("--apply" in sys.argv[1:]))
