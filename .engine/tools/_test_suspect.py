"""Unit test for query.classify D0005 suspicion (CR-4) — no kernel/git needed.
Covers: re-verified-later trigger, material-change trigger, ordering-only exclusion,
transitive propagation, invalid-evidence demotion."""
import os
import sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import query

ANCESTRY = {("c1", "c2"): True}                     # c1 is ancestor of c2
CRITERIA_AT = {("c1", "M"): "old definition"}        # M's criterion text as of c1
query._is_ancestor = lambda a, b: ANCESTRY.get((a, b), False)
query._sha_valid = lambda sha: sha != "BOGUS"
query._criterion_at = lambda sha, task: CRITERIA_AT.get((sha, task))


def t(name, deps, done, commit, stmt="s"):
    return {"name": name, "deps": deps, "done": done,
            "verifiedAtCommit": commit, "dod": {"statement": stmt}}


tasks = {
    # A re-verified at c2 AFTER B was judged at c1 -> B suspect (trigger 1)
    "A": t("A", [], True, "c2"),
    "B": t("B", ["A"], True, "c1"),
    # C depends on A only via an ORDERING-ONLY edge -> NOT suspect
    "C": t("C", ["A"], True, "c1"),
    # D depends on B (suspect) -> transitively suspect
    "D": t("D", ["B"], True, "c1"),
    # M's criterion text changed since N's judgment commit -> N suspect (trigger 2)
    "M": t("M", [], True, "c1", stmt="new definition"),
    "N": t("N", ["M"], True, "c1"),
    # E claims a commit that doesn't resolve -> invalid evidence, demoted from done
    "E": t("E", [], True, "BOGUS"),
}
query.classify(tasks, ordering_only={("A", "C")})

assert tasks["B"]["suspect"], "B: upstream re-verified later must fire"
assert not tasks["C"]["suspect"], "C: ordering-only edge must NOT carry suspicion"
assert tasks["D"]["suspect"], "D: suspicion must propagate transitively"
assert tasks["N"]["suspect"], "N: upstream material change must fire"
assert not tasks["M"]["suspect"] and not tasks["A"]["suspect"], "roots clean"
assert tasks["E"]["invalidEvidence"] and not tasks["E"]["done"], "E: bogus SHA demoted"
print("OK: re-verified-later, material-change, ordering-only exclusion, "
      "transitivity, invalid-evidence all behave per D0005.")
