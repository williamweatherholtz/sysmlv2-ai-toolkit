#!/usr/bin/env python3
"""Rewrite every `keel diagram` / `keel report` / `keel decision-card` call site to its `keel render`
spelling — the D0449 fold of the rendering family, one of the five d0399 option A named.

D0449: the three top-level names are REMOVED from the dispatch and `keel render` resolves them as
sub-verbs, in the sprint 512 shape (D0273): a committed transform, a dry run that reconciles control
totals, call-site rewrites in the SAME commit as the removal, no alias window. The mapping is the
Decision's own text:

    keel diagram [ROOT]                    ->  keel render model [--root ROOT]
    keel report <kind> [--html] [--trend]  ->  keel render report <kind> [--html] [--trend]
    keel decision-card [NAME] [--proposed] ->  keel render decision-card [NAME] [--proposed]

`keel diagram` took its root as a POSITIONAL; `keel render` takes it as `--root`, so the one argument
shape that changes is rewritten explicitly: `keel diagram [ROOT]` becomes `keel render model [--root
ROOT]` and `keel diagram .` becomes `keel render model --root .`. Every other argument rides along
unchanged, which is what lets the sprint compare the two binaries' output byte for byte.

WHAT IT DELIBERATELY DOES NOT TOUCH (the D0273 transform's rule, unchanged).
  - `.engine/decisions/` and any `reference/decisions/`: a Decision names the commands that existed
    when it was signed.
  - `.tracking/`: instance records are history.
  - `.engine/tools/migrations/`: this file's own examples.
  - `.claude/`: generated from `.engine/skills` by `keel sync-claude`, which the sprint runs after this.
  - Source and tests under `keel-cli/`: code, rewritten by hand with the router itself — EXCEPT
    `keel-cli/assets/claude-md-template.md`, which is prose `init` ships to a downstream project.
  - `keel-cli/tests/fixtures/`: a frozen input the SVG port is held byte-equal against; it is a
    snapshot of a computed lens on one day, not a call site.

Idempotent by construction: the pattern requires `keel ` immediately followed by one of the three
old verbs, and `render` is not one of them, so `keel render report` is never rewritten again.

Usage:
    python .engine/tools/migrations/2026-09-11-render-family-fold.py --dry-run
    python .engine/tools/migrations/2026-09-11-render-family-fold.py --apply
"""
import io
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))

# Order matters only for `diagram`: the argument-shape rules run before the bare rule.
RULES = [
    # `keel diagram [ROOT]` (the usage spelling) -> `keel render model [--root ROOT]`
    (re.compile(r"\bkeel diagram \[ROOT\]"), "keel render model [--root ROOT]"),
    # `keel diagram .` as a whole token (followed by space, `>`, quote or end) -> `--root .`
    (re.compile(r"\bkeel diagram \.(?=[\s>`'\"]|$)"), "keel render model --root ."),
    # any other `keel diagram` -> `keel render model` (the next token, if any, rides along)
    (re.compile(r"\bkeel diagram\b"), "keel render model"),
    (re.compile(r"\bkeel report\b"), "keel render report"),
    (re.compile(r"\bkeel decision-card\b"), "keel render decision-card"),
]
# For the census: the three old verbs, matched exactly as a call site.
CENSUS = re.compile(r"\bkeel (diagram|report|decision-card)\b")

SCAN_ROOTS = [".engine", "CLAUDE.md", "README.md", "docs", "scripts", "keel-cli/assets"]
EXTENSIONS = {"md", "sysml", "toml", "txt", "yml", "yaml", "sh", "py"}


def excluded(rel):
    """Attested, historical and generated text is never rewritten — see the module docstring."""
    r = rel.replace("\\", "/")
    return "/decisions/" in r or r.startswith(".tracking/") or "/tools/migrations/" in r or r.startswith(".claude/")


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
    for pat, rep in RULES:
        text = pat.sub(rep, text)
    return text


def plan():
    """(rel_path, hits, rewritten_text) for every file the transform would change."""
    out = []
    for rel in targets():
        try:
            text = io.open(os.path.join(ROOT, rel), encoding="utf-8").read()
        except (OSError, UnicodeDecodeError):
            continue
        hits = len(CENSUS.findall(text))
        if hits:
            new = rewrite(text)
            # Conservation: every old call site becomes exactly one `keel render` call site.
            gained = new.count("keel render ") - text.count("keel render ")
            if gained != hits:
                raise SystemExit("control total mismatch in %s: %d old site(s) but %d render site(s) gained" % (rel, hits, gained))
            if CENSUS.search(new):
                raise SystemExit("old spelling survives the rewrite in %s" % rel)
            out.append((rel, hits, new))
    return out


def main(argv):
    if len(argv) != 2 or argv[1] not in ("--dry-run", "--apply"):
        print(__doc__)
        return 2
    p = plan()
    files = len(p)
    sites = sum(h for _, h, _ in p)
    print("render-family-fold: %d call site(s) across %d file(s)" % (sites, files))
    for rel, hits, _ in sorted(p, key=lambda x: -x[1]):
        print("  %4d  %s" % (hits, rel))
    if argv[1] == "--dry-run":
        print("dry run: nothing written. Control totals above (conservation checked per file); re-run with --apply.")
        return 0
    for rel, _, new in p:
        io.open(os.path.join(ROOT, rel), "w", encoding="utf-8", newline="").write(new)
    # Reconcile: a second plan must be EMPTY, or the transform is not idempotent.
    remaining = sum(h for _, h, _ in plan())
    print("applied. re-plan finds %d remaining call site(s)%s" % (remaining, "" if remaining == 0 else " — NOT IDEMPOTENT"))
    return 0 if remaining == 0 else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
