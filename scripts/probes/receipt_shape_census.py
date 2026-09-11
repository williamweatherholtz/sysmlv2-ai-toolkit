#!/usr/bin/env python3
# ci-probe: --probe
"""Which `// RAN:` receipts in the tree does the D0444 replayable-demo rule admit? (dcReceiptShapeCensusIsASensor)

D0444 made a `method=demo` receipt REPLAYABLE when it IS a command and nothing else: one line, beginning with a
prefix `.engine/contracts/reverify.toml` declares under `[demo] replayable`, every space-separated token drawn
from the command alphabet `[A-Za-z0-9_./:=@-]` and none of them an English function word. The rule was MEASURED
before it was coded - a census over this tree's 283 prefixed receipts (2026-09-11) found that the prefix alone
admits 283 narratives, the alphabet alone admits 1, alphabet plus function words admits 0 - but the census lived
in a session scratchpad. This is that census as a committed Sensor: anyone loosening the rule re-runs the
measurement first, and the D0388 pair below says whether the rule as configured still tells a command from a
story.

The three rules are applied exactly as `keel-cli/src/reverify.rs` `is_replayable_with` applies them, over the
same universe (every receipt that begins with a declared prefix):

  prefix-only        the trimmed receipt is one line and begins with a declared prefix
  alphabet           prefix-only AND every `split(' ')` token is non-empty and in the alphabet
  alphabet+function  alphabet AND no token is one of PROSE_WORDS   <- the rule as implemented

The prefixes are read from `.engine/contracts/reverify.toml` and PROSE_WORDS is read VERBATIM from the Rust
source, so the census cannot drift from the binary: a word added to one is measured by the other. A `ci-run
id=` receipt (D0323) is replayable by CI itself and never begins with a declared prefix; it is counted apart.

D0388 - the known cases, run BEFORE the tree is read, under the rule as configured:

  known-positive: `keel show control-structure . --svg` (the DoD's case) and `keel version` - admitted by all
                  three rules.
  known-negative: `keel suite 571 passed, 0 failed` (the DoD's case; the comma is outside the alphabet) and
                  `keel show control-structure . and looked at the picture` (every character in the alphabet;
                  `and`, `at` and `the` are function words) - admitted by the prefix rule, rejected by the rule
                  as implemented. The second is the shape only the function-word layer catches, so a probe
                  that admits it says the layer is gone.

    python scripts/probes/receipt_shape_census.py --probe          # the known cases only; exit 1 on a disagreement
    python scripts/probes/receipt_shape_census.py                  # probe, then the census over .tracking
    python scripts/probes/receipt_shape_census.py --json           # the same census as one JSON object

The last line of the census, `demo latest-pass under the full rule`, is the count under attestation's semantics
(`keel show attestation . --json` -> `demoReplayable`: demo Tests whose LATEST result is a pass carrying a replayable
receipt, ci-run excluded) so the two instruments can be held against each other.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import sys
import tomllib

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
REVERIFY_RS = os.path.join(ROOT, "keel-cli", "src", "reverify.rs")
REVERIFY_TOML = os.path.join(ROOT, ".engine", "contracts", "reverify.toml")
TRACKING = os.path.join(ROOT, ".tracking")
ALPHABET_EXTRA = "_./:=@-"

KNOWN_POSITIVE = ["keel show control-structure . --svg", "keel version"]
KNOWN_NEGATIVE = ["keel suite 571 passed, 0 failed", "keel show control-structure . and looked at the picture"]


def prose_words() -> list[str]:
    """PROSE_WORDS as reverify.rs declares it - the census measures the binary's list, not a copy."""
    src = open(REVERIFY_RS, encoding="utf-8").read()
    m = re.search(r"const PROSE_WORDS: \[&str; (\d+)\] = \[(.*?)\];", src, re.S)
    if not m:
        sys.exit(f"PROSE_WORDS not found in {REVERIFY_RS}")
    words = re.findall(r'"([^"]*)"', m.group(2))
    if len(words) != int(m.group(1)):
        sys.exit(f"PROSE_WORDS declares {m.group(1)} words but lists {len(words)}")
    return words


def demo_prefixes() -> list[str]:
    """`[demo] replayable` from the contract; absent file or section declares NONE, as the binary reads it."""
    try:
        with open(REVERIFY_TOML, "rb") as f:
            cfg = tomllib.load(f)
    except (OSError, tomllib.TOMLDecodeError):
        return []
    return [p for p in cfg.get("demo", {}).get("replayable", []) if isinstance(p, str)]


class Rule:
    """The three rules of `is_replayable_with`, layered, minus the ci-run shortcut (counted apart)."""

    def __init__(self, prefixes: list[str], words: list[str]):
        self.prefixes = [p for p in prefixes if p]
        self.words = set(words)

    def prefixed(self, evidence: str) -> bool:
        e = evidence.strip()
        return bool(e) and len(e.splitlines()) == 1 and any(e.startswith(p) for p in self.prefixes)

    @staticmethod
    def _in_alphabet(token: str) -> bool:
        return bool(token) and all(c.isascii() and (c.isalnum() or c in ALPHABET_EXTRA) for c in token)

    def alphabet(self, evidence: str) -> bool:
        return self.prefixed(evidence) and all(self._in_alphabet(t) for t in evidence.strip().split(" "))

    def strict(self, evidence: str) -> bool:
        return self.alphabet(evidence) and not any(t in self.words for t in evidence.strip().split(" "))

    @staticmethod
    def ci_run(evidence: str) -> bool:
        return evidence.strip().startswith("ci-run id=")


def probe(rule: Rule) -> None:
    failures = []
    for e in KNOWN_POSITIVE:
        if not (rule.prefixed(e) and rule.alphabet(e) and rule.strict(e)):
            failures.append(f"known-positive `{e}` not admitted under all three rules")
    for e in KNOWN_NEGATIVE:
        if not rule.prefixed(e):
            failures.append(f"known-negative `{e}` is not even prefixed - the case no longer measures the layers")
        if rule.strict(e):
            failures.append(f"known-negative `{e}` ADMITTED under the rule as implemented")
    if len(rule.words) != 16:
        failures.append(f"PROSE_WORDS holds {len(rule.words)} words; D0444 sized the rule with sixteen - re-run the census before trusting the count")
    if failures:
        print("PROBE FAILED:")
        for f in failures:
            print("  " + f)
        sys.exit(1)
    print(f"probe: known-positive {', '.join(f'`{e}`' for e in KNOWN_POSITIVE)} admitted under prefix, alphabet and alphabet+function-words;")
    print(f"       known-negative {', '.join(f'`{e}`' for e in KNOWN_NEGATIVE)} prefixed, rejected under alphabet+function-words. Both hold.")
    print(f"       ({len(rule.prefixes)} declared prefix(es): {', '.join(repr(p) for p in rule.prefixes)}; {len(rule.words)} function words from reverify.rs)")


def sysml_files() -> list[str]:
    out = []
    for dirpath, _, files in os.walk(TRACKING):
        out.extend(os.path.join(dirpath, f) for f in files if f.endswith(".sysml"))
    return sorted(out)


def receipts_in(path: str, lines: list[str]) -> tuple[list[tuple[str, int, str]], int]:
    """Every `// RAN:` receipt of one file - a comment line that begins with the marker, or a TestResult line
    carrying it at the end (the two forms the binary reads) - as (relative path, 1-based line, trimmed text
    after the first marker); and the count of lines that merely MENTION the marker inside a string."""
    rel = os.path.relpath(path, ROOT).replace(os.sep, "/")
    out, mentions = [], 0
    for i, line in enumerate(lines, 1):
        if "// RAN:" not in line:
            continue
        s = line.lstrip()
        if s.startswith("// RAN:") or (s.startswith("part ") and " : TestResult {" in s):
            out.append((rel, i, line.split("// RAN:", 1)[1].strip()))
        else:
            mentions += 1
    return out, mentions


def demo_latest_pass_receipts(lines: list[str]) -> list[str]:
    """The receipts attestation counts as demoReplayable candidates: the LATEST result of each method=demo
    Test when it is a pass with a receipt on the line or directly above. Mirrors `demo_replays_in_text`."""
    text = "\n".join(lines)
    demos = set()
    for cap in text.split("verification ")[1:]:
        name = re.split(r"[ :]", cap, maxsplit=1)[0]
        if ":>> method = VerificationMethod::demo" in cap.split("}", 1)[0]:
            demos.add(name)
    latest: dict[str, tuple[int, str, str | None]] = {}
    for i, line in enumerate(lines):
        if " : TestResult {" not in line:
            continue
        head = line.split(" : TestResult", 1)[0]
        if "part " not in head:
            continue
        part = head.split("part ", 1)[1].strip()
        base, _, n = part.rpartition("R")
        if not n.isdigit() or base not in demos:
            continue
        outcome = re.split(r"[; }]", line.split("VerdictKind::", 1)[1], maxsplit=1)[0] if "VerdictKind::" in line else ""
        receipt = None
        if "// RAN:" in line:
            receipt = line.split("// RAN:", 1)[1].strip()
        elif i > 0 and lines[i - 1].lstrip().startswith("// RAN:"):
            receipt = lines[i - 1].lstrip()[len("// RAN:"):].strip()
        if int(n) >= latest.get(base, (0, "", None))[0]:
            latest[base] = (int(n), outcome, receipt)
    return [r for _, o, r in latest.values() if o == "pass" and r is not None]


def census(rule: Rule) -> dict:
    total, mentions, ci_run, prefixed = 0, 0, 0, []
    per_prefix = {p: 0 for p in rule.prefixes}
    demo_latest = []
    for path in sysml_files():
        lines = open(path, encoding="utf-8", errors="replace").read().split("\n")
        found, mentioned = receipts_in(path, lines)
        mentions += mentioned
        for rel, ln, text in found:
            total += 1
            if rule.ci_run(text):
                ci_run += 1
            elif rule.prefixed(text):
                prefixed.append((rel, ln, text))
                per_prefix[next(p for p in rule.prefixes if text.startswith(p))] += 1
        demo_latest.extend(demo_latest_pass_receipts(lines))
    alphabet = [r for r in prefixed if rule.alphabet(r[2])]
    strict = [r for r in alphabet if rule.strict(r[2])]
    alphabet_only = [r for r in alphabet if not rule.strict(r[2])]
    demo_replayable = [r for r in demo_latest if rule.strict(r) and not rule.ci_run(r)]
    return {
        "ranReceipts": total,
        "markerMentionsInStrings": mentions,
        "ciRunReceipts": ci_run,
        "prefixed": len(prefixed),
        "perPrefix": per_prefix,
        "admitted": {"prefixOnly": len(prefixed), "alphabet": len(alphabet), "alphabetPlusFunctionWords": len(strict)},
        "narrativesAdmittedByPrefixOnly": len(prefixed) - len(strict),
        "alphabetFalsePositives": [{"at": f"{r}:{l}", "text": t} for r, l, t in alphabet_only],
        "admittedUnderFullRule": [{"at": f"{r}:{l}", "text": t} for r, l, t in strict],
        "demoLatestPassUnderFullRule": len(demo_replayable),
        "prefixes": rule.prefixes,
        "functionWords": sorted(rule.words),
    }


def report(c: dict) -> None:
    print(f"census: {c['ranReceipts']} `// RAN:` receipts under .tracking ({c['markerMentionsInStrings']} more lines only mention the marker inside a string); "
          f"{c['ciRunReceipts']} are ci-run (D0323, counted apart); "
          f"{c['prefixed']} begin with a declared prefix ({', '.join(f'{p!r} {n}' for p, n in c['perPrefix'].items())})")
    a = c["admitted"]
    print(f"  prefix-only admits             {a['prefixOnly']:>4}   ({c['narrativesAdmittedByPrefixOnly']} of them narratives the full rule rejects)")
    print(f"  alphabet admits                {a['alphabet']:>4}   ({len(c['alphabetFalsePositives'])} narrative(s) the function words then catch)")
    print(f"  alphabet+function-words admits {a['alphabetPlusFunctionWords']:>4}   <- the rule as implemented")
    if c["alphabetFalsePositives"]:
        print("  alphabet-only false positives (prose in the alphabet, caught by a function word):")
        for fp in c["alphabetFalsePositives"]:
            print(f"    {fp['at']}: {fp['text']}")
    if c["admittedUnderFullRule"]:
        print("  admitted under the full rule (each IS a command `keel reverify --demos` may run):")
        for r in c["admittedUnderFullRule"]:
            print(f"    {r['at']}: {r['text']}")
    else:
        print("  admitted under the full rule: none")
    print(f"  demo latest-pass under the full rule: {c['demoLatestPassUnderFullRule']}   (compare `keel show attestation . --json` -> demoReplayable)")


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--probe", action="store_true", help="run the known cases and exit")
    ap.add_argument("--json", action="store_true", help="print the census as one JSON object (the probe still runs first, to stderr)")
    a = ap.parse_args()
    rule = Rule(demo_prefixes(), prose_words())
    if a.json:
        out, sys.stdout = sys.stdout, sys.stderr
        probe(rule)
        sys.stdout = out
        print(json.dumps(census(rule), indent=2))
        return
    probe(rule)
    if a.probe:
        return
    report(census(rule))


if __name__ == "__main__":
    main()
