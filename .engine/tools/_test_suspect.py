"""Unit test for query.classify suspicion (no kernel/git needed — stub ancestry)."""
import os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import query

# Synthetic: B depends on A. A was re-verified at a LATER commit (c2) than B (c1),
# i.e. B's verifiedAtCommit (c1) is an ancestor of A's (c2) -> B is SUSPECT.
ANCESTRY = {("c1", "c2"): True}  # c1 is ancestor of c2
query._is_ancestor = lambda a, b: ANCESTRY.get((a, b), False)

tasks = {
    "A": {"name": "A", "deps": [], "done": True, "dod": {"verifiedAtCommit": "c2"}},
    "B": {"name": "B", "deps": ["A"], "done": True, "dod": {"verifiedAtCommit": "c1"}},
    "C": {"name": "C", "deps": ["A"], "done": True, "dod": {"verifiedAtCommit": "c2"}},
}
query.classify(tasks)

assert tasks["B"]["suspect"] is True, "B should be suspect (upstream A moved after B verified)"
assert tasks["A"]["suspect"] is False, "A has no deps -> not suspect"
assert tasks["C"]["suspect"] is False, "C verified at same commit as A -> not suspect"
print("OK: suspicion fires on stale upstream; same-commit and root are clean.")
