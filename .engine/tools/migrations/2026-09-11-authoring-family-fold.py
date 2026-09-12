#!/usr/bin/env python3
"""Rewrite every call site of the nine authoring verbs to its `keel record <fact>` spelling — the D0451
fold of the authoring family, the third of the five d0399 option A named.

D0451: the nine authoring verbs become sub-verbs of `keel record`, EACH NAMED FOR THE FACT IT WRITES
and each keeping its flags unchanged; the top-level names are REMOVED from the dispatch in the sprint
512 shape (D0273): a committed transform, a dry run that reconciles control totals, call-site rewrites
in the SAME commit as the removal, no alias window. The mapping is the Decision's own text:

    keel add-task ...                  ->  keel record task ...
    keel new sprint <N> <slug> ...     ->  keel record sprint <N> <slug> ...
    keel append-result ...             ->  keel record result ...
    keel append-gate-result ...        ->  keel record gate-result ...
    keel apply-review ...              ->  keel record review ...
    keel record-measurement ...        ->  keel record measurement ...
    keel snapshot-indicators ...       ->  keel record indicator-snapshot ...
    keel reverify ...                  ->  keel record reverify ...
    keel mint [N]                      ->  keel record mint [N]

Unlike the D0449 and D0450 folds this is a RENAME TABLE, not one prefix rule: `append-result` becomes
`result`, not `append-result`. So the control total is checked PER VERB — each old spelling's count is
the count its new spelling gains — and not as one sum. The verb must be followed by a character that
is neither a word character nor a hyphen, so `keel record-measurement` is matched by its own row and
never as a prefix of something longer.

WHAT IT DELIBERATELY DOES NOT TOUCH (the D0273 transform's rule, unchanged from the two earlier folds).
  - `.engine/decisions/` and any `reference/decisions/`: a Decision names the commands that existed
    when it was signed.
  - `.tracking/`: instance records are history. The eight `// RAN:` receipts that name a folded verb
    are narratives, not replayable commands (`keel reverify --demos` at ceaabdd: "no demo pass carries
    a replayable receipt"), so D0451's receipt clause has nothing to rewrite.
  - `CHANGELOG.md` and `docs/reviews/`: dated records of what a release or a panel said that day.
  - `.engine/tools/migrations/`: this file's own examples and the earlier folds' docstrings.
  - `.claude/`: generated from `.engine/skills` by `keel sync-claude`, which the sprint runs after
    this — EXCEPT `.claude/output-styles/keel.md`, which is a SOURCE (embedded by claude_surface.rs)
    and is rewritten by hand in the same commit.
  - Source and tests under `keel-cli/`: code, rewritten by hand with the router itself — EXCEPT
    `keel-cli/assets/`, which is prose and a console `init` ships to a downstream project.
  - `keel-cli/tests/fixtures/`: a frozen input the SVG port is held byte-equal against.

Idempotent by construction: no row's new spelling begins with `keel <old verb>`, and `record` is not a
row, so `keel record result` is never rewritten again.

Usage:
    python .engine/tools/migrations/2026-09-11-authoring-family-fold.py --dry-run
    python .engine/tools/migrations/2026-09-11-authoring-family-fold.py --apply
"""
import io
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))

# (old spelling after `keel `, new spelling after `keel `) — D0451's own table.
TABLE = [
    ("add-task", "record task"),
    ("new sprint", "record sprint"),
    ("append-result", "record result"),
    ("append-gate-result", "record gate-result"),
    ("apply-review", "record review"),
    ("record-measurement", "record measurement"),
    ("snapshot-indicators", "record indicator-snapshot"),
    ("reverify", "record reverify"),
    ("mint", "record mint"),
]
NEW = dict(TABLE)
_ALT = "|".join(re.escape(old) for old, _ in TABLE)
# `keel <verb>` where the verb is a whole word: not followed by a word character or a hyphen.
CENSUS = re.compile(r"\bkeel (%s)(?![\w-])" % _ALT)

SCAN_ROOTS = [".engine", "CLAUDE.md", "README.md", "docs", "scripts", "keel-cli/assets"]
EXTENSIONS = {"md", "sysml", "toml", "txt", "yml", "yaml", "sh", "py", "html"}


def excluded(rel):
    """Attested, historical and generated text is never rewritten — see the module docstring."""
    r = rel.replace("\\", "/")
    return ("/decisions/" in r or r.startswith(".tracking/") or "/tools/migrations/" in r
            or r.startswith(".claude/") or r == "CHANGELOG.md" or r.startswith("docs/reviews/"))


def targets():
    for base in SCAN_ROOTS:
        full = os.path.join(ROOT, base)
        if os.path.isfile(full):
            yield base
            continue
        for dp, _, fs in os.walk(full):
            if "__pycache__" in dp:
                continue
            for f in fs:
                if f.rsplit(".", 1)[-1].lower() in EXTENSIONS:
                    rel = os.path.relpath(os.path.join(dp, f), ROOT)
                    if not excluded(rel):
                        yield rel


def rewrite(text):
    return CENSUS.sub(lambda m: "keel " + NEW[m.group(1)], text)


def per_verb(text):
    counts = {}
    for v in CENSUS.findall(text):
        counts[v] = counts.get(v, 0) + 1
    return counts


def plan():
    """(rel_path, hits, per-verb counts, rewritten_text) for every file the transform would change."""
    out = []
    for rel in targets():
        try:
            text = io.open(os.path.join(ROOT, rel), encoding="utf-8").read()
        except (OSError, UnicodeDecodeError):
            continue
        counts = per_verb(text)
        if counts:
            new = rewrite(text)
            # Conservation PER VERB: each old spelling's sites become exactly that many new-spelling sites.
            for old, n in counts.items():
                needle = "keel " + NEW[old]
                gained = len(re.findall(re.escape(needle) + r"(?![\w-])", new)) - len(re.findall(re.escape(needle) + r"(?![\w-])", text))
                if gained != n:
                    raise SystemExit("control total mismatch in %s: %d `keel %s` site(s) but %d `%s` site(s) gained" % (rel, n, old, gained, needle))
            if CENSUS.search(new):
                raise SystemExit("old spelling survives the rewrite in %s" % rel)
            out.append((rel, sum(counts.values()), counts, new))
    return out


def main(argv):
    if len(argv) != 2 or argv[1] not in ("--dry-run", "--apply"):
        print(__doc__)
        return 2
    p = plan()
    files = len(p)
    sites = sum(h for _, h, _, _ in p)
    print("authoring-family-fold: %d call site(s) across %d file(s)" % (sites, files))
    # Per-verb census, so the sprint's implement gate can read each word's count back.
    totals = {}
    for _, _, counts, _ in p:
        for v, n in counts.items():
            totals[v] = totals.get(v, 0) + n
    for old, new in TABLE:
        print("  %-22s -> %-28s %4d" % (old, new, totals.get(old, 0)))
    for rel, hits, _, _ in sorted(p, key=lambda x: -x[1]):
        print("  %4d  %s" % (hits, rel))
    if argv[1] == "--dry-run":
        print("dry run: nothing written. Control totals above (conservation checked per verb per file); re-run with --apply.")
        return 0
    for rel, _, _, new in p:
        io.open(os.path.join(ROOT, rel), "w", encoding="utf-8", newline="").write(new)
    # Reconcile: a second plan must be EMPTY, or the transform is not idempotent.
    remaining = sum(h for _, h, _, _ in plan())
    print("applied. re-plan finds %d remaining call site(s)%s" % (remaining, "" if remaining == 0 else " — NOT IDEMPOTENT"))
    return 0 if remaining == 0 else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
