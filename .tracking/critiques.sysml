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

    // ── d0082 × completeness ──────────────────────────────────────────────────
    verification d0082CompletenessCritique : Test {
        :>> id = "63940516-2c7d-4e8f-c082-516273849711";
        :>> title = "Critique d0082 — completeness lens";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::completeness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> procedureText = "ATTACK (uncovered cases): (1) a FAILING verify test (outcome=fail) — the five tiers fold it into 'uncovered', losing the 'tested-and-failed' vs 'never-tested' distinction. SURVIVED as a deliberate, low-risk simplification: a failing verify surfaces immediately in cargo test/CI, and none exist today; documented limitation, not a silent gap. (2) element with BOTH verify + charter — SURVIVED: strongest-wins ordering resolves it (verified). (3) non-testable requirements that also aren't decisions — SURVIVED: they land in addressed/uncovered honestly (no false 'verified'). (4) transitive depth >1 (need->req->sub-req) — SURVIVED for the current model (needs satisfy requirements directly; deeper chains would extend the same rule). No material completeness gap; the failing-fold is an accepted scope note.";
    }
    part d0082CompletenessCritiqueR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c082-516273849712"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0941e97"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    #Verify dependency from d0082CompletenessCritique to d0082;

    // ── d0082 × correctness ───────────────────────────────────────────────────
    verification d0082CorrectnessCritique : Test {
        :>> id = "63940516-2c7d-4e8f-c082-516273849713";
        :>> title = "Critique d0082 — correctness lens";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::correctness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> procedureText = "ATTACK (unsound logic): (1) transitive-satisfy conferring 'verified' — could it over-credit? SURVIVED: it confers verified ONLY when the satisfied requirement is itself verified (req_tier==verified), so a need can't be verified through an unverified/addressed requirement (confirmed live: n11 stays uncovered because sr11 is the gap). (2) critique edges polluting coverage — SURVIVED: method=critique verify-edges are explicitly excluded (confirmed: d0081 reverted from verified to attested after the fix). (3) completeness oracle — explicit-test reads the Test's own R1 outcome, not the action `done` set; correct for standalone verification Tests (the done-set bug that initially showed 0 verified). CAVEAT (accepted): tier reads R1 only, so a re-verified Test (R1 fail, R2 pass) would misread — same single-result assumption as accept-event; all current results are R1. No correctness defect for the current model.";
    }
    part d0082CorrectnessCritiqueR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c082-516273849714"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0941e97"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    #Verify dependency from d0082CorrectnessCritique to d0082;

    // ── d0082 × feasibility ───────────────────────────────────────────────────
    verification d0082FeasibilityCritique : Test {
        :>> id = "63940516-2c7d-4e8f-c082-516273849715";
        :>> title = "Critique d0082 — feasibility lens";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "claudeOpus";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::feasibility;
        :>> critiquedBy = CriticKind::aiModel;
        :>> procedureText = "ATTACK (impractical?): (1) does the tier refactor build + stay green? SURVIVED: clippy --workspace -D warnings clean, cargo test 31 lib pass (incl coverage_tiers_and_transitive_verification), live `sysmlv2 coverage` shows 13 verified reqs + 13 verified needs + 78 attested decisions + the sr11/n11 gap. (2) authoring cost for verification — SURVIVED: 13 requirement verify-chains pointing at EXISTING re-runnable evidence (clippy/cargo test/build.rs/validate), needs verified transitively for free; the one true gap (sr11 perf) is an Issue, not fabricated. (3) does it block work? SURVIVED: gate scoped to governed elements (D0081); the 28 are grandfathered. Feasible — demonstrated end to end in-sprint.";
    }
    part d0082FeasibilityCritiqueR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c082-516273849716"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0941e97"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    #Verify dependency from d0082FeasibilityCritique to d0082;
}
