#!/usr/bin/env python3
"""Move every AISkill declaration out of the shared registry, next to the skill it describes.

D0222, after issue252. `keel process audit` reports 23 of 24 units LANDS-RED: a unit carries its
process definition and its SKILL.md, but the declaration BINDING them lives in the single shared
`.engine/skills/skills-registry.sysml`, which cannot travel with one unit — so adopting any unit
into a project that lacks it leaves that project's `process-skill` guard failing on arrival. The
guard and `activation::deploying_skills` already read every `.sysml` under `.engine/skills/`
(D0220), so a per-skill file is a valid home today; this migration moves the other 35 declarations
into one.

D0067 bulk-migration discipline: a COMMITTED transform, a DRY RUN that reconciles control totals,
and green at every step. Nothing is fabricated — each declaration is MOVED verbatim, byte for byte,
so the ids and provenance are unchanged and `duplicate-identity` proves the move rather than a copy.

Usage:
    python .engine/tools/migrations/2026-08-24-skill-registry-beside-skill.py --dry-run
    python .engine/tools/migrations/2026-08-24-skill-registry-beside-skill.py --apply
"""
import io
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", ".."))
CENTRAL = os.path.join(ROOT, ".engine", "skills", "skills-registry.sysml")


def blocks(text):
    """Every `part <name> : AISkill { ... }` block, with its brace-matched extent."""
    out = []
    for m in re.finditer(r"^    part (\w+) : AISkill \{", text, re.M):
        name = m.group(1)
        i = text.index("{", m.start())
        depth = 0
        for j in range(i, len(text)):
            if text[j] == "{":
                depth += 1
            elif text[j] == "}":
                depth -= 1
                if depth == 0:
                    out.append((name, m.start(), j + 1, text[m.start():j + 1]))
                    break
    return out


def skill_dir(block):
    """The skill DIRECTORY, from its own declared location — never guessed from the part name."""
    m = re.search(r':>> location = "\.engine/skills/([^/"]+)/', block)
    return m.group(1) if m else None


def pascal(dirname):
    return "".join(p.capitalize() for p in re.split(r"[-_]", dirname))


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "--dry-run"
    if mode not in ("--dry-run", "--apply"):
        print(__doc__)
        return 2
    text = io.open(CENTRAL, encoding="utf-8").read()
    found = blocks(text)

    planned, skipped = [], []
    for name, start, end, body in found:
        d = skill_dir(body)
        if not d:
            skipped.append((name, "no declared location - cannot place it beside a skill"))
            continue
        target = os.path.join(ROOT, ".engine", "skills", d, "registry.sysml")
        if os.path.exists(target):
            skipped.append((name, "already declared beside its skill (" + d + ")"))
            continue
        if not os.path.isdir(os.path.join(ROOT, ".engine", "skills", d)):
            skipped.append((name, "skill directory " + d + " does not exist"))
            continue
        planned.append((name, d, target, body))

    # ---- control totals, reconciled BEFORE anything is written (D0067) ----
    print("skill-registry migration (D0222)")
    print("  AISkill declarations in the central registry : %d" % len(found))
    print("  to MOVE beside their skill                   : %d" % len(planned))
    print("  skipped (with reason)                        : %d" % len(skipped))
    for n, why in skipped:
        print("      - %s: %s" % (n, why))
    if len(planned) + len(skipped) != len(found):
        print("  RECONCILIATION FAILED: %d + %d != %d - refusing" % (len(planned), len(skipped), len(found)))
        return 1
    print("  reconciled: moved + skipped == found")

    if mode == "--dry-run":
        for n, d, _, _ in planned:
            print("      would write .engine/skills/%s/registry.sysml  (%s)" % (d, n))
        print("  DRY RUN - nothing written.")
        return 0

    # ---- apply: write each per-skill file, then remove the blocks from central ----
    for name, d, target, body in planned:
        pkg = "SkillsRegistry" + pascal(d)
        content = (
            "// The %s skill's own registry declaration, beside the skill it describes (D0222).\n"
            "//\n"
            "// MOVED verbatim out of the shared skills-registry by a committed migration transform\n"
            "// (D0222). A file that TRAVELS must not cite a repo-local tool path: the adopter hits a\n"
            "// dead reference and tool-reference fails in THEIR tree. The shared registry\n"
            "// could not travel with a unit, so adopting any unit left the receiving project's\n"
            "// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is\n"
            "// unchanged and `duplicate-identity` is what proves it.\n"
            "package %s {\n"
            "    private import EngineElement::*;\n"
            "    private import EngineSkills::*;\n\n"
            "%s\n"
            "}\n"
        ) % (d, pkg, body)
        io.open(target, "w", encoding="utf-8", newline="\n").write(content)

    # Remove moved blocks from central, last-first so earlier offsets stay valid.
    moved_names = {n for n, _, _, _ in planned}
    for name, start, end, _ in sorted(found, key=lambda b: -b[1]):
        if name in moved_names:
            text = text[:start] + text[end:]
    # collapse the blank lines the removals leave behind
    text = re.sub(r"\n{3,}", "\n\n", text)
    io.open(CENTRAL, "w", encoding="utf-8", newline="\n").write(text)
    print("  APPLIED: %d file(s) written, %d block(s) removed from the central registry." % (len(planned), len(planned)))
    print("  Now run: keel validate . && keel guard && keel check-engine . && keel process audit")
    return 0


if __name__ == "__main__":
    sys.exit(main())
