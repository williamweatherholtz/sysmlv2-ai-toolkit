// =====================================================================
// ProjectCritiques — recorded antagonistic critiques (D0080): method=critique verifications
// #Verify-linked to their target element, by an INDEPENDENT critic (result judgedBy != target
// author). Findings (outcome=fail) become severity-carrying Issues via the #Resolves loop.
//
// First entries (D0081, the charter-time assurance-gate decision) — the dogfood that proves the
// pipeline: each required Decision lens (completeness/correctness/feasibility, Core-3) genuinely
// attacks d0081; all survived (outcome=pass). Critic: claudeOpus (independent of author wweatherholtz).
// =====================================================================
package ProjectCritiques {
    private import EngineElement::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // ── d0081 × completeness ──────────────────────────────────────────────────
    verification d0081CompletenessCritique : Test {
        :>> id = "63940516-2c7d-4e8f-c081-516273849701";
        :>> title = "Critique d0081 — completeness lens";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::completeness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> procedureText = "ATTACK (find uncovered cases): (1) renamed/moved elements — a rename changes the name, so a grandfathered element would drop out of the present-at-baseline set and re-enter scope. SURVIVED: that is correct behavior, not a gap — a renamed element is materially changed and SHOULD be re-examined; renames are migration-governed (D0067), deliberate and rare. (2) #SafetyChange that must force re-examination of old work — SURVIVED: d0081 explicitly preserves the D0062/D0068 safety override. (3) Architecture elements — SURVIVED: required_lenses returns empty for absent types; scoping applies automatically when Architecture targets are added. (4) git-unavailable — SURVIVED: explicit conservative fail-safe (grandfather all, never spuriously block). No completeness gap found.";
    }
    part d0081CompletenessCritiqueR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c081-516273849702"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5b90663"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    #Verify dependency from d0081CompletenessCritique to d0081;

    // ── d0081 × correctness ───────────────────────────────────────────────────
    verification d0081CorrectnessCritique : Test {
        :>> id = "63940516-2c7d-4e8f-c081-516273849703";
        :>> title = "Critique d0081 — correctness lens";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::correctness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> procedureText = "ATTACK (find unsound logic): (1) baseline choice — introduction commit vs acceptance judgedAgainst. SURVIVED: intro-commit is the sound freeze point — it grandfathers exactly what existed when the rule landed (incl the governing decision), matching D0068's 'in force as of creation' semantics; judgedAgainst (the parent) would wrongly place the governing decision itself in scope. (2) recursion — guard assured is in GUARD_NAMES and compute_readiness aggregates invariants. SURVIVED: readiness explicitly excludes assured (recursion) AND critique (double-count), verified by a clean run. (3) name extraction — names_present_at greps `(part|requirement) NAME :`, matching decisions (part dNNNN), needs/reqs (requirement nNN/srNN). SURVIVED: covers all assurance target forms. (4) independence — governed gap uses compute_critique_coverage which requires result.judgedBy != element author. SURVIVED. No correctness defect found.";
    }
    part d0081CorrectnessCritiqueR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c081-516273849704"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5b90663"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    #Verify dependency from d0081CorrectnessCritique to d0081;

    // ── d0081 × feasibility ───────────────────────────────────────────────────
    verification d0081FeasibilityCritique : Test {
        :>> id = "63940516-2c7d-4e8f-c081-516273849705";
        :>> title = "Critique d0081 — feasibility lens";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::feasibility;
        :>> critiquedBy = CriticKind::aiModel;
        :>> procedureText = "ATTACK (find what makes it impractical): (1) per-element git cost — SURVIVED: the grandfather set is computed with ONE `git grep` per governing decision (not per element), cheap enough for a pre-commit gate. (2) is it actually implementable + green? SURVIVED: built — guards.rs GUARD_NAMES 7->9, govern.rs grandfathered_under, view.rs scoping; clippy --workspace -D warnings clean, cargo test green, and the now-hard guard critique + guard assured PASS vacuously live. (3) does it block normal work? SURVIVED: grandfathered backlog stays committable; only new elements are held, and the dogfood (d0081 critiqued in-commit) shows the forward path is light (3 critiques per new decision). Feasible — demonstrated end to end.";
    }
    part d0081FeasibilityCritiqueR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c081-516273849706"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5b90663"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    #Verify dependency from d0081FeasibilityCritique to d0081;
}
