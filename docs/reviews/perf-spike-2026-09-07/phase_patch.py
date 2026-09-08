"""Measurement-only: wrap each guard in a perf phase so KEEL_PERF=2 reports per-guard time in-process.

A patch that cannot find its anchor FAILS (sprint 592 retro). Reverse with `git checkout keel-cli/src/guards.rs`.
"""
import io
import sys

p = "keel-cli/src/guards.rs"
s = io.open(p, encoding="utf-8").read()
old = "            _ => run_one(n, root),\n        })\n        .collect()\n}"
new = "            _ => crate::perf::phase(&format!(\"guard:{n}\"), || run_one(n, root)),\n        })\n        .collect()\n}"
if s.count(old) != 1:
    sys.exit("anchor not found exactly once - refusing to patch")
io.open(p, "w", encoding="utf-8", newline="").write(s.replace(old, new, 1))
print("patched")
