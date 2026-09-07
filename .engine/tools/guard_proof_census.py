"""Which enforced guards have ever been SHOWN to catch their own defect?

DECLARED (an edge says a control covers a hazard) and ARMED (a probe says it can fire) are both
computed by keel already. There is no third state for PROVEN - a test constructs the defect and this
control reports it - which is the only evidence separating a control from a control-shaped thing.

FIRST RUN, 2026-09-06: of 64 guards, 16 are named in a test body that asserts a failure, 21 are named
only around passing assertions, and 27 are named in no test body at all.

HOW THIS SCRIPT LIED TO ME FIRST, kept here because the lesson is the point: version one searched a
text window around each occurrence of a guard's name. Every name occurs in GUARD_NAMES inside
guards.rs, and that file is full of the words "violation" and "FAIL", so all 64 matched and the
answer came back 64/64 - a number produced by the instrument rather than by the tree, caught only
because it was too good. This version looks inside TEST FUNCTION BODIES only.

STATED LIMITATION: textual. A guard exercised by a fixture that never names it counts as unnamed, and
a guard whose test asserts a clean tree counts as named-but-unproven - which is the right way round
for this question, since a demonstrated PASS is not a demonstrated CATCH.
"""
import re
from pathlib import Path

REPO = Path("C:/Users/WilliamWeatherholtz/claude_code/sysmlv2-ai-toolkit")

names_block = re.search(
    r"pub const GUARD_NAMES: \[&str; \d+\] =\s*(\[[^\]]*\]);",
    (REPO / "keel-cli/src/guards.rs").read_text(encoding="utf-8"),
    re.S,
)
guards = re.findall(r'"([a-z0-9-]+)"', names_block.group(1))


def test_bodies(text):
    """Each #[test] function's body, by brace matching from its signature."""
    out = []
    for m in re.finditer(r"#\[test\][\s\S]{0,200}?fn\s+([a-z0-9_]+)\s*\([^)]*\)\s*\{", text):
        i = text.index("{", m.end() - 1)
        depth, j = 0, i
        while j < len(text):
            if text[j] == "{":
                depth += 1
            elif text[j] == "}":
                depth -= 1
                if depth == 0:
                    break
            j += 1
        out.append((m.group(1), text[i:j]))
    return out


bodies = []
for src in list((REPO / "keel-cli/tests").glob("*.rs")) + list((REPO / "keel-cli/src").rglob("*.rs")):
    text = src.read_text(encoding="utf-8", errors="ignore")
    for fn, body in test_bodies(text):
        bodies.append((src.name, fn, body))

FAIL_WORDS = ("violation", "FAIL", "fails", "refus", "denied", "0 violation", "is_empty()")
named, proven = {}, {}
for fname, fn, body in bodies:
    for g in guards:
        if g not in body:
            continue
        named.setdefault(g, []).append(f"{fname}::{fn}")
        # a constructed failure: the body asserts a non-empty violation set, or asserts the word FAIL
        if re.search(r"violations[^;]{0,80}(is_empty\(\)|len\(\)|!\s*=|>)", body) or "FAIL" in body \
           or re.search(r"assert!\([^;]{0,120}(violation|refus|denied)", body):
            proven.setdefault(g, []).append(f"{fname}::{fn}")

print(f"guards: {len(guards)}   test functions scanned: {len(bodies)}")
print(f"  named inside a test body            : {len(named)}")
print(f"  and that test asserts a FAILURE     : {len(proven)}")
print(f"  named nowhere in any test body      : {len(guards) - len(named)}")
print()
unproven = [g for g in guards if g in named and g not in proven]
unnamed = [g for g in guards if g not in named]
if unnamed:
    print("NO TEST BODY NAMES THESE (no demonstrated catch, no demonstrated pass):")
    for g in unnamed:
        print(f"  {g}")
print()
if unproven:
    print("NAMED IN A TEST, BUT NO CONSTRUCTED FAILURE IN THAT TEST:")
    for g in unproven:
        print(f"  {g:34s} ({', '.join(named[g][:2])})")
