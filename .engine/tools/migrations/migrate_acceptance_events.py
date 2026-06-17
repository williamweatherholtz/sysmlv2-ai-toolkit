#!/usr/bin/env python3
"""One-time migration (D0066, attrModelAcceptanceEvents): give every accepted Decision a
confirmation acceptance EVENT (dNNNNAccept : Test + dNNNNAcceptR1 : TestResult), so tooling
reads acceptance uniformly from events (no status/event format split).

Attestation: judgedBy=wweatherholtz, judgedAt=2026-06-17, judgedAgainst=HEAD — the human's
explicit 'Migrate all; you attest the ~38 now' directive (2026-06-17). status=accepted + prose
remain as the prior historical record.

Idempotent: skips a decision that already has a `<dname>Accept : Test`. Only touches
status=accepted decisions. Adds EngineElement + EngineVerification imports if missing.

  python .engine/tools/migrations/migrate_acceptance_events.py          # dry-run
  python .engine/tools/migrations/migrate_acceptance_events.py --apply   # write
"""
from __future__ import annotations

import re
import sys
import uuid
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
DECISIONS = ROOT / ".engine" / "decisions"
SHA = "e146621"
WHEN = "2026-06-17"
WHO = "wweatherholtz"

_PART = re.compile(r'part\s+(d\w+)\s*:\s*Decision\b')


def migrate_file(text: str) -> tuple[str, str | None]:
    """Return (new_text, dname) or (text, None) if skipped."""
    if not re.search(r'status\s*=\s*DecisionStatus::accepted', text):
        return text, None
    m = _PART.search(text)
    if not m:
        return text, None
    dname = m.group(1)
    if f"{dname}Accept : Test" in text or f"{dname}Accept :Test" in text:
        return text, None  # already migrated

    # ensure imports
    if "EngineElement::*" not in text:
        text = text.replace("private import EngineWork::*;",
                            "private import EngineElement::*;\n    private import EngineWork::*;", 1)
    if "EngineVerification::*" not in text:
        text = text.replace("private import EngineWork::*;",
                            "private import EngineWork::*;\n    private import EngineVerification::*;", 1)

    block = (
        f'\n    verification {dname}Accept : Test {{ :>> id = "{uuid.uuid4()}"; '
        f':>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz '
        f'attests acceptance of decision {dname} (D0066 acceptance-events migration, attested '
        f'{WHEN}); prior record: status=accepted + file prose."; }}\n'
        f'    part {dname}AcceptR1 : TestResult {{ :>> id = "{uuid.uuid4()}"; '
        f':>> outcome = VerdictKind::pass; :>> judgedAgainst = "{SHA}"; :>> judgedAt = "{WHEN}"; '
        f':>> judgedBy = "{WHO}"; }}\n'
    )
    # insert before the final closing brace of the package
    idx = text.rstrip().rfind("}")
    text = text[:idx] + block + text[idx:]
    return text, dname


def main() -> int:
    apply = "--apply" in sys.argv
    done = []
    for path in sorted(DECISIONS.glob("*.sysml")):
        original = path.read_text(encoding="utf-8")
        new, dname = migrate_file(original)
        if dname:
            done.append(dname)
            if apply:
                path.write_text(new, encoding="utf-8")
    print(f"{'APPLY' if apply else 'DRY-RUN'}: acceptance event added to {len(done)} decision(s)")
    print("  " + ", ".join(done))
    return 0


if __name__ == "__main__":
    sys.exit(main())
