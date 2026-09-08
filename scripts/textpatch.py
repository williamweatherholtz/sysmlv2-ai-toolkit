#!/usr/bin/env python3
# not-an-instrument: it edits text files at a stated anchor; it measures nothing.
"""textpatch - a patch that cannot find its anchor FAILS, and never lands where it does no work (D0386, issue398).

The shape this replaces: a scratch script did `s.replace(anchor, new)` or "find the anchor, else append",
the anchor did not match, the block landed after the line that serialises the output, the script exited
zero and wrote its file, and every fact the block computed was silently absent. Three times in one
session. An append is a plausible-looking success; a refusal names the problem in one line, at the moment.

So here the assertion is the only available shape. Every operation:

  * refuses (SystemExit 1, nothing written) when the anchor occurs 0 times - a miss;
  * refuses when it occurs more than once - an ambiguity, never "the first one";
  * writes through a temp file then rename, so a refusal leaves the target byte-for-byte as it was;
  * and `append` is a SEPARATE, named operation: a deliberate end-of-file addition reads as one in the
    caller's own text, and is never what a failed search falls through to.

Import it from a scratch script run at the repository root:

    import sys; sys.path.insert(0, "scripts")
    from textpatch import replace_once, insert_after, insert_before, append

    replace_once("keel-cli/src/guards.rs", 'pub const GUARD_NAMES: [&str; 65] =', 'pub const GUARD_NAMES: [&str; 67] =')
    insert_after("scripts/exec_brief/facts.py", "# ===== 11. control structure\n", NEW_SECTION)   # before the emit
    append("CHANGELOG.md", "\n## 0.4.2\n")                                                    # stated, not fallen into

Or from the shell, with the texts in files so backslashes and newlines survive the harness (D0309):

    python scripts/textpatch.py replace FILE --old OLD.txt --new NEW.txt
    python scripts/textpatch.py insert-after FILE --anchor ANCHOR.txt --new NEW.txt
    python scripts/textpatch.py append FILE --new NEW.txt
    python scripts/textpatch.py --probe        # the known-positive / known-negative cases (dcAdHocChecksAreProbedFirst)

Every write is CRLF-free: files here are LF, and a patch that flipped line endings would show every line
changed in the diff.
"""
from __future__ import annotations

import io
import os
import sys
import tempfile


class PatchRefused(SystemExit):
    """Raised (as a non-zero exit) when a patch cannot be applied exactly once. Nothing has been written."""

    def __init__(self, message: str):
        super().__init__(1)
        self.message = message
        print(f"textpatch: REFUSED - {message}", file=sys.stderr)


def _read(path: str) -> str:
    with io.open(path, encoding="utf-8", newline="") as f:
        return f.read()


def _write_atomically(path: str, text: str) -> None:
    d = os.path.dirname(os.path.abspath(path)) or "."
    fd, tmp = tempfile.mkstemp(prefix=".textpatch-", dir=d)
    try:
        with io.open(fd, "w", encoding="utf-8", newline="") as f:
            f.write(text)
        os.replace(tmp, path)
    except BaseException:
        try:
            os.unlink(tmp)
        except OSError:
            pass
        raise


def _locate(text: str, anchor: str, path: str, what: str) -> int:
    """The index of the anchor's single occurrence, or a refusal naming the count."""
    if anchor == "":
        raise PatchRefused(f"{path}: the {what} is empty - an empty anchor matches everywhere, which is nowhere in particular")
    n = text.count(anchor)
    if n == 0:
        preview = anchor.strip().splitlines()[0][:70] if anchor.strip() else repr(anchor)
        raise PatchRefused(f"{path}: {what} not found ({preview!r}) - the file has moved on, or the anchor was misquoted; nothing written")
    if n > 1:
        raise PatchRefused(f"{path}: {what} occurs {n} times - refusing to pick one; widen the anchor until it is unique")
    return text.index(anchor)


def replace_once(path: str, old: str, new: str) -> None:
    """Replace the single occurrence of `old` with `new`. Refuses on 0 or >1 occurrences."""
    text = _read(path)
    i = _locate(text, old, path, "text to replace")
    _write_atomically(path, text[:i] + new + text[i + len(old):])


def insert_after(path: str, anchor: str, new: str) -> None:
    """Insert `new` immediately after the single occurrence of `anchor`."""
    text = _read(path)
    i = _locate(text, anchor, path, "anchor") + len(anchor)
    _write_atomically(path, text[:i] + new + text[i:])


def insert_before(path: str, anchor: str, new: str) -> None:
    """Insert `new` immediately before the single occurrence of `anchor`."""
    text = _read(path)
    i = _locate(text, anchor, path, "anchor")
    _write_atomically(path, text[:i] + new + text[i:])


def append(path: str, new: str) -> None:
    """Add `new` at the END of the file. A deliberate operation with its own name - never a fallback."""
    text = _read(path)
    if new == "":
        raise PatchRefused(f"{path}: nothing to append")
    _write_atomically(path, text + new)


# ------------------------------------------------------------------------------------------- probe
def probe() -> int:
    """Known-positive and known-negative cases, run BEFORE trusting the helper on a real tree.

    Each case states what it expects; a helper that passed only the positive cases would be the
    issue400 shape (a check that matches its own registry). Returns the number of failures.
    """
    import shutil

    failures = 0
    d = tempfile.mkdtemp(prefix="textpatch-probe-")

    def case(name: str, ok: bool, detail: str = "") -> None:
        nonlocal failures
        print(f"  {'pass' if ok else 'FAIL'}  {name}{(' - ' + detail) if detail and not ok else ''}")
        if not ok:
            failures += 1

    def fresh(body: str) -> str:
        p = os.path.join(d, "t.txt")
        with io.open(p, "w", encoding="utf-8", newline="") as f:
            f.write(body)
        return p

    def refused(fn, *args) -> bool:
        try:
            fn(*args)
        except PatchRefused:
            return True
        return False

    body = "alpha\n# anchor\nbeta\nEMIT\n"
    try:
        # positive: a unique anchor is patched exactly there
        p = fresh(body)
        insert_after(p, "# anchor\n", "gamma\n")
        case("insert_after lands after its unique anchor", _read(p) == "alpha\n# anchor\ngamma\nbeta\nEMIT\n")
        p = fresh(body)
        replace_once(p, "beta", "BETA")
        case("replace_once replaces the one occurrence", _read(p) == "alpha\n# anchor\nBETA\nEMIT\n")
        p = fresh(body)
        insert_before(p, "EMIT\n", "delta\n")
        case("insert_before lands before its anchor", _read(p) == "alpha\n# anchor\nbeta\ndelta\nEMIT\n")
        p = fresh(body)
        append(p, "omega\n")
        case("append is explicit and lands at the end", _read(p) == body + "omega\n")

        # negative: the issue398 case - an absent anchor is a refusal and the file is untouched
        p = fresh(body)
        r = refused(insert_after, p, "# anchr\n", "gamma\n")
        case("an ABSENT anchor is refused", r)
        case("...and nothing was written", _read(p) == body)
        case("...in particular, nothing was APPENDED", "gamma" not in _read(p))

        # negative: an ambiguous anchor is a refusal, never 'the first one'
        p = fresh("x\nSAME\ny\nSAME\nz\n")
        r = refused(replace_once, p, "SAME", "DIFF")
        case("an anchor that occurs twice is refused", r)
        case("...and neither occurrence was touched", _read(p) == "x\nSAME\ny\nSAME\nz\n")

        # negative: an empty anchor
        p = fresh(body)
        case("an empty anchor is refused", refused(insert_after, p, "", "q"))

        # line endings survive
        crlf = "a\r\n# anchor\r\nb\r\n"
        p = fresh(crlf)
        insert_after(p, "# anchor\r\n", "c\r\n")
        case("CRLF text is patched without being rewritten", _read(p) == "a\r\n# anchor\r\nc\r\nb\r\n")
    finally:
        shutil.rmtree(d, ignore_errors=True)
    print(f"textpatch probe: {failures} failure(s)")
    return failures


# --------------------------------------------------------------------------------------------- cli
def _main(argv: list[str]) -> int:
    import argparse

    if argv == ["--probe"]:
        return 1 if probe() else 0
    ap = argparse.ArgumentParser(prog="textpatch", description=__doc__.splitlines()[0])
    sub = ap.add_subparsers(dest="op", required=True)
    for op in ("replace", "insert-after", "insert-before", "append"):
        s = sub.add_parser(op)
        s.add_argument("file")
        if op == "replace":
            s.add_argument("--old", required=True, help="file holding the text to replace")
        elif op != "append":
            s.add_argument("--anchor", required=True, help="file holding the anchor text")
        s.add_argument("--new", required=True, help="file holding the new text")
    a = ap.parse_args(argv)
    new = _read(a.new)
    if a.op == "replace":
        replace_once(a.file, _read(a.old), new)
    elif a.op == "insert-after":
        insert_after(a.file, _read(a.anchor), new)
    elif a.op == "insert-before":
        insert_before(a.file, _read(a.anchor), new)
    else:
        append(a.file, new)
    print(f"textpatch: {a.op} applied to {a.file}")
    return 0


if __name__ == "__main__":
    sys.exit(_main(sys.argv[1:]))
