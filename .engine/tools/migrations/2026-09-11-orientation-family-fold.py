#!/usr/bin/env python3
"""Rewrite every call site of the eleven orientation verbs to its `keel show <word>` spelling — the
D0450 fold of the orientation family, the second of the five d0399 option A named.

D0450: the eleven read-only orientation verbs join `keel show` as lenses, EACH KEEPING ITS WORD and
its trailing arguments unchanged, and the top-level names are REMOVED from the dispatch in the sprint
512 shape (D0273): a committed transform, a dry run that reconciles control totals, call-site rewrites
in the SAME commit as the removal, no alias window. The mapping is the Decision's own text:

    keel orient [ROOT] [--html]        ->  keel show orient [ROOT] [--html]
    keel whats-next ...                ->  keel show whats-next ...
    keel status ...                    ->  keel show status ...
    keel view <name> ...               ->  keel show view <name> ...
    keel item <name>                   ->  keel show item <name>
    keel arch <sub> ...                ->  keel show arch <sub> ...
    keel attestation ...               ->  keel show attestation ...
    keel actor-trace <actor>           ->  keel show actor-trace <actor>
    keel governing-version <item>      ->  keel show governing-version <item>
    keel reprocess-candidates ...      ->  keel show reprocess-candidates ...
    keel enforcement-report ...        ->  keel show enforcement-report ...

Because every verb keeps its word and its arguments, ONE rule covers all eleven: `keel <verb>` becomes
`keel show <verb>` and whatever followed rides along. The verb must be followed by a character that is
neither a word character nor a hyphen, so `keel attestation-coverage` (already a lens) and
`keel view-something` are never touched.

WHAT IT DELIBERATELY DOES NOT TOUCH (the D0273 transform's rule, unchanged from the D0449 fold).
  - `.engine/decisions/` and any `reference/decisions/`: a Decision names the commands that existed
    when it was signed.
  - `.tracking/`: instance records are history.
  - `CHANGELOG.md` and `docs/reviews/`: dated records of what a release or a panel said that day.
  - `.engine/tools/migrations/`: this file's own examples.
  - `.claude/`: generated from `.engine/skills` by `keel sync-claude`, which the sprint runs after
    this — EXCEPT `.claude/output-styles/keel.md`, which is a SOURCE (embedded by claude_surface.rs)
    and is rewritten by hand in the same commit.
  - Source and tests under `keel-cli/`: code, rewritten by hand with the router itself — EXCEPT
    `keel-cli/assets/`, which is prose and a console `init` ships to a downstream project.
  - `keel-cli/tests/fixtures/`: a frozen input the SVG port is held byte-equal against.
  - `scripts/exec_brief/facts.py`: its line 480 is a regex over D0367's attested context string, which
    still reads `keel orient`; the file's eight prose sites are rewritten by hand in the same commit.

Idempotent by construction: the pattern requires `keel ` immediately followed by one of the eleven
verbs, and `show` is not one of them, so `keel show orient` is never rewritten again.

Usage:
    python .engine/tools/migrations/2026-09-11-orientation-family-fold.py --dry-run
    python .engine/tools/migrations/2026-09-11-orientation-family-fold.py --apply
"""
import io
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))

VERBS = ["orient", "whats-next", "status", "view", "item", "arch", "attestation", "actor-trace",
         "governing-version", "reprocess-candidates", "enforcement-report"]
_ALT = "|".join(re.escape(v) for v in VERBS)
# `keel <verb>` where the verb is a whole word: not followed by a word character or a hyphen.
CENSUS = re.compile(r"\bkeel (%s)(?![\w-])" % _ALT)
RULE = (CENSUS, r"keel show \1")

SCAN_ROOTS = [".engine", "CLAUDE.md", "README.md", "docs", "scripts", "keel-cli/assets"]
EXTENSIONS = {"md", "sysml", "toml", "txt", "yml", "yaml", "sh", "py", "html"}


def excluded(rel):
    """Attested, historical and generated text is never rewritten — see the module docstring."""
    r = rel.replace("\\", "/")
    return ("/decisions/" in r or r.startswith(".tracking/") or "/tools/migrations/" in r
            or r.startswith(".claude/") or r == "CHANGELOG.md" or r.startswith("docs/reviews/")
            or r == "scripts/exec_brief/facts.py")


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
    pat, rep = RULE
    return pat.sub(rep, text)


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
            # Conservation: every old call site becomes exactly one `keel show ` call site.
            gained = new.count("keel show ") - text.count("keel show ")
            if gained != hits:
                raise SystemExit("control total mismatch in %s: %d old site(s) but %d show site(s) gained" % (rel, hits, gained))
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
    print("orientation-family-fold: %d call site(s) across %d file(s)" % (sites, files))
    # Per-verb census, so the sprint's implement gate can read each word's count back.
    per_verb = {}
    for rel in [r for r, _, _ in p]:
        text = io.open(os.path.join(ROOT, rel), encoding="utf-8").read()
        for v in CENSUS.findall(text):
            per_verb[v] = per_verb.get(v, 0) + 1
    for v in VERBS:
        print("  %-22s %4d" % (v, per_verb.get(v, 0)))
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
