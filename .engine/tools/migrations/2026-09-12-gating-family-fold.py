#!/usr/bin/env python3
"""Rewrite every call site of the ten folded gating verbs to its `keel gate` / `keel audit` spelling —
the D0452 fold of the gating family, the fourth of the five d0399 option A named.

D0452: the ten top-level names are REMOVED from the dispatch and the two routers resolve them as
sub-verbs, in the sprint 512 shape (D0273): a committed transform, a dry run that reconciles control
totals, call-site rewrites in the SAME commit as the removal, no alias window. Each verb keeps its
word and its arguments, so every argument rides along unchanged and the sprint compares the pre-fold
and post-fold binaries byte for byte on stdout, stderr and exit code. The mapping is the Decision's:

    keel validate ...         ->  keel gate validate ...
    keel check ...            ->  keel gate check ...
    keel check-engine ...     ->  keel gate check-engine ...
    keel guard ...            ->  keel gate guard ...
    keel rules ...            ->  keel gate rules ...
    keel assured ...          ->  keel gate assured ...
    keel adoption-check ...   ->  keel gate adoption-check ...
    keel audit-history ...    ->  keel audit history ...
    keel audit-adherence ...  ->  keel audit adherence ...
    keel audit-ci-runs ...    ->  keel audit ci-runs ...

The binary is spelled several ways at the call sites this repository holds, and every spelling is a
site: `keel`, `keel.exe`, `./target/release/keel`, `keel-serve.exe`, the hooks' `"$KEEL"` / `$KEEL` /
`${KEEL}`, and the python list shape `[KEEL, "guard", ...]` the brief scripts use.

WHAT IT DELIBERATELY DOES NOT TOUCH (the D0273 transform's rule, unchanged).
  - `.engine/decisions/` and any `reference/decisions/`: a Decision names the commands that existed
    when it was signed.
  - `.tracking/`: instance records are history. The ledger ids `validate`, `check-engine` and `guard`
    that `.keel/metrics` and the enforcement report carry are ids, not invocations, and are kept.
  - `CHANGELOG.md` and `docs/reviews/`: dated text.
  - `.engine/tools/migrations/`: this file's own examples.
  - `.claude/`: generated from `.engine/skills` by `keel sync-claude`, which the sprint runs after this
    — EXCEPT `.claude/output-styles/keel.md`, which is SOURCE (D0130) and is rewritten.
  - Rust ARGV literals (`&["validate", ...]`): code, rewritten by hand with the router itself. The
    prose in `keel-cli/src` and `keel-cli/tests` — usage strings, doc comments, expected messages —
    IS rewritten, because it names the invocation a reader types.
  - `keel-cli/tests/fixtures/`: frozen inputs, snapshots of a computed lens on one day.

Idempotent by construction: the pattern requires the binary immediately followed by one of the ten
old verbs, and neither `gate` nor `audit` is one of them, so `keel gate validate` is never rewritten
again; the lookahead `(?![\\w-])` keeps `keel check` from matching `keel check-engine`.

Usage:
    python .engine/tools/migrations/2026-09-12-gating-family-fold.py --dry-run
    python .engine/tools/migrations/2026-09-12-gating-family-fold.py --apply
"""
import io
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))

TABLE = {
    "validate": ("gate", "validate"),
    "check": ("gate", "check"),
    "check-engine": ("gate", "check-engine"),
    "guard": ("gate", "guard"),
    "rules": ("gate", "rules"),
    "assured": ("gate", "assured"),
    "adoption-check": ("gate", "adoption-check"),
    "audit-history": ("audit", "history"),
    "audit-adherence": ("audit", "adherence"),
    "audit-ci-runs": ("audit", "ci-runs"),
}
VERBS = "|".join(sorted(TABLE, key=len, reverse=True))
# The binary, in every spelling a call site here uses, then ONE space, then the verb as a whole word.
BIN = r'(?P<bin>\bkeel(?:-serve|-prefold\d*)?(?:\.exe)?|"?\$\{?KEEL\}?"?)'
CALL = re.compile(BIN + r" (?P<verb>" + VERBS + r")(?![\w-])")
# The python list shape: `KEEL, "guard"` (the brief scripts build argv lists from a KEEL constant).
PYLIST = re.compile(r'(?P<bin>\bKEEL, *)"(?P<verb>' + VERBS + r')"')

SCAN_ROOTS = [
    ".engine", ".githooks", ".github/workflows", "CLAUDE.md", "README.md", "docs", "scripts",
    "keel-cli/assets", "keel-cli/src", "keel-cli/tests", ".claude/output-styles/keel.md",
]
EXTENSIONS = {"md", "sysml", "toml", "txt", "yml", "yaml", "sh", "py", "rs"}
SOURCE_UNDER_GENERATED = {".claude/output-styles/keel.md"}


def excluded(rel):
    """Attested, historical and generated text is never rewritten — see the module docstring."""
    r = rel.replace("\\", "/")
    if r in SOURCE_UNDER_GENERATED:
        return False
    return (
        "/decisions/" in r
        or r.startswith(".tracking/")
        or "/tools/migrations/" in r
        or r.startswith(".claude/")
        or r == "CHANGELOG.md"
        or r.startswith("docs/reviews/")
        or r.startswith("keel-cli/tests/fixtures/")
    )


def targets():
    for base in SCAN_ROOTS:
        full = os.path.join(ROOT, base)
        if os.path.isfile(full):
            if not excluded(base):
                yield base
            continue
        for dp, _, fs in os.walk(full):
            if "__pycache__" in dp or os.sep + "target" + os.sep in dp + os.sep:
                continue
            for f in fs:
                # hooks under .githooks carry no extension; everything else is typed
                typed = f.rsplit(".", 1)[-1].lower() in EXTENSIONS if "." in f else base == ".githooks"
                if typed:
                    rel = os.path.relpath(os.path.join(dp, f), ROOT)
                    if not excluded(rel):
                        yield rel


def rewrite(text):
    def call(m):
        router, sub = TABLE[m.group("verb")]
        return "%s %s %s" % (m.group("bin"), router, sub)

    def pylist(m):
        router, sub = TABLE[m.group("verb")]
        return '%s"%s", "%s"' % (m.group("bin"), router, sub)

    return PYLIST.sub(pylist, CALL.sub(call, text))


def census(text):
    """{old verb: count} of call sites in a text, both shapes."""
    out = {}
    for m in CALL.finditer(text):
        out[m.group("verb")] = out.get(m.group("verb"), 0) + 1
    for m in PYLIST.finditer(text):
        out[m.group("verb")] = out.get(m.group("verb"), 0) + 1
    return out


def routed_count(text, router, sub):
    return len(re.findall(r"\b%s %s(?![\w-])" % (router, sub), text)) + len(re.findall(r'"%s", "%s"' % (router, sub), text))


def plan():
    """(rel_path, {verb: hits}, rewritten_text) for every file the transform would change."""
    out = []
    for rel in targets():
        try:
            text = io.open(os.path.join(ROOT, rel), encoding="utf-8", newline="").read()
        except (OSError, UnicodeDecodeError):
            continue
        hits = census(text)
        if not hits:
            continue
        new = rewrite(text)
        # Conservation, PER VERB: every old call site becomes exactly one routed call site of ITS verb.
        for verb, n in hits.items():
            router, sub = TABLE[verb]
            gained = routed_count(new, router, sub) - routed_count(text, router, sub)
            if gained != n:
                raise SystemExit("control total mismatch in %s for %s: %d old site(s) but %d `%s %s` site(s) gained" % (rel, verb, n, gained, router, sub))
        if census(new):
            raise SystemExit("old spelling survives the rewrite in %s: %s" % (rel, census(new)))
        out.append((rel, hits, new))
    return out


def main(argv):
    if len(argv) != 2 or argv[1] not in ("--dry-run", "--apply"):
        print(__doc__)
        return 2
    p = plan()
    files = len(p)
    per_verb = {}
    for _, hits, _ in p:
        for v, n in hits.items():
            per_verb[v] = per_verb.get(v, 0) + n
    sites = sum(per_verb.values())
    print("gating-family-fold: %d call site(s) across %d file(s)" % (sites, files))
    for v in TABLE:
        if per_verb.get(v):
            print("  %4d  keel %s -> keel %s %s" % (per_verb[v], v, TABLE[v][0], TABLE[v][1]))
    for rel, hits, _ in sorted(p, key=lambda x: -sum(x[1].values())):
        print("  %4d  %s" % (sum(hits.values()), rel))
    if argv[1] == "--dry-run":
        print("dry run: nothing written. Control totals above (conservation checked per verb per file); re-run with --apply.")
        return 0
    for rel, _, new in p:
        io.open(os.path.join(ROOT, rel), "w", encoding="utf-8", newline="").write(new)
    # Reconcile: a second plan must be EMPTY, or the transform is not idempotent.
    remaining = sum(sum(h.values()) for _, h, _ in plan())
    print("applied. re-plan finds %d remaining call site(s)%s" % (remaining, "" if remaining == 0 else " — NOT IDEMPOTENT"))
    return 0 if remaining == 0 else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
