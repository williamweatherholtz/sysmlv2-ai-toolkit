// D0218 (PROPOSED — NOT YET ACCEPTED) — adherenceChecksTheFirstCommit: the baseline is the PARENT of the range, so no commit escapes the monotonicity gate
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0218Accept event on sign-off.
package Decision0218 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0218 : Decision {
        :>> id = "47236e7d-d639-4b9e-84a6-561cb797a06c";
        :>> title = "adherenceChecksTheFirstCommit: the baseline is the PARENT of the range, so no commit escapes the monotonicity gate";
        :>> createdAt = "2026-08-24";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::proposed;
        :>> context = "The chartered follow-on dcNoFalseCleanRange described audit-adherence printing 'fewer than 2 commits in range - nothing to compare. Clean.' as a false clean and proposed making it a usage error. Investigating it found the charter was wrong in a way that mattered twice over. FIRST, erroring would have broken CI on every single-commit push, because CI invokes --since github.event.before and a one-commit push lists exactly one commit - the issue089/issue090 trap of a new hard check failing the normal case on day one. SECOND, and this is the actual defect: the comparison seeded its baseline from shas[0] and then walked windows of two, so shas[0] was only ever a baseline and was NEVER ITSELF CHECKED. The first commit of every push escaped the gate, and a single-commit push escaped entirely. A weakening introduced in the first commit of a push has been invisible to this control since the day it shipped - the control built for exactly the issue236 self-modification class, with a hole at the most common push shape there is.";
        :>> decision = "The comparison baseline is the PARENT of the range's first commit, resolved to a real SHA, so every commit in the range - including the first - is compared against what preceded it. A range starting at a ROOT commit has no parent to weaken from, and that case is STATED in the output and uses the first commit as its own baseline rather than being silently guessed. Only a genuinely EMPTY range reports nothing to do; the 'fewer than 2 commits' branch is gone, because one commit is now fully checkable. The chartered usage-error remedy is explicitly REJECTED and the reason recorded, so the charter is not re-attempted.";
        :>> rationale = "Making the short-range case loud would have treated a symptom and broken the common path; extending the baseline removes the symptom by making the case checkable, which is the difference between refusing to answer and being able to answer. It is also strictly more coverage rather than more noise: no new failure class is introduced, an existing blind spot is closed. The root-commit branch is stated rather than defaulted because a baseline that silently becomes 'itself' would reproduce the very false-clean shape being fixed.";
        :>> consequences = "Every commit in a pushed range is now compared, including the first, so CI's single-commit-push case is checked for the first time. VERIFIED by probe: a rule weakened in a single commit with the hook bypassed (--no-verify, hooksPath overridden per-command) returns exit 1 naming capabilityRootednessRule and its parent SHA, where the identical probe previously reported Clean; real history PASSES at both --since HEAD~1 and --since HEAD~20, so the wider comparison introduces no false positive. A display defect found in the same pass is fixed: the parent was carried as the revision STRING 'X^' and shortened to seven characters, which ate the caret and printed a commit as its own parent. Resolves issue250 and closes the dcNoFalseCleanRange charter.";
    }
}
