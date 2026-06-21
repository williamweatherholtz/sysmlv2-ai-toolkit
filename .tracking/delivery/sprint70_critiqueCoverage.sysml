// =====================================================================
// ProjectDeliveryS70 — Sprint 70: critiqueCoverage (D0080/D0079 — per-element x required-lens).
// Story: `sysmlv2 critique-coverage` + `sysmlv2 guard critique` — for each Need / SystemRequirement
//        / accepted Decision, which REQUIRED lenses (Core-3) have an INDEPENDENT method=critique
//        verification (#Verify-linked, result judgedBy != element author). Honest by construction.
//        Guard is runnable but NOT yet enforced (hard gate after a genuine critique pass).
// Total: 4 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS70 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from critiqueCoverageStory to d0080;

    part critiqueCoverageStory : Story {
        :>> id = "63940516-2c7d-4e8f-b070-51627384a701";
        :>> title = "Sprint 70: critiqueCoverage (4 pts) — `sysmlv2 critique-coverage` + `guard critique`, Core-3 required lenses (D0080)";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 4;
    }

    action def DeliveryRunS70 {
        action storyCritiqueCoverage;
        verification storyCritiqueCoverageDoD : Test { :>> id = "63940516-2c7d-4e8f-b070-51627384a702"; :>> method = VerificationMethod::test; :>> procedureText = "critiqueCoverageDoD: `sysmlv2 critique-coverage` computes per-element x required-lens (Core-3) critique coverage from independent method=critique verifications (#Verify marker-edge -> element; result judgedBy != element createdBy); per-type summary + gap set. `sysmlv2 guard critique` fails elements missing a required lens — runnable via run_one, NOT in enforced GUARD_NAMES (wired hard after a critique pass). VERIFY (live): 105 elements uncovered (0 critiques) -> guard critique FAIL 105; guard all PASS; clippy --workspace -D warnings + cargo test (33 lib) + rust validate green; doc-sync usage.md."; }
        part storyCritiqueCoverageDoDR1 : TestResult { :>> id = "7c2d6aa1-0b57-491d-880f-6a1f9ea699a7"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1f70e34"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification critiqueCoverageRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b070-51627384a703"; :>> title = "Sprint 70 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0080 critique-coverage. Two policy forks surfaced + human-resolved: required-lens policy = Core-3 (Need/Req = completeness/correctness/testability; Decision = completeness/correctness/feasibility); guard behavior = HARD pre-commit gate (the accepted target). Sequencing constraint identified: a hard gate with 0 critiques blocks every commit (incl the wiring commit), so build view+guard now, wire hard AFTER a genuine critique pass — never fabricate critiques (independence/anti-theater). estimatedPoints=4. #CharteredBy d0080. DoD accepted."; }
    part critiqueCoverageRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b070-51627384a704"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1f70e34"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification critiqueCoverageStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b070-51627384a705"; :>> title = "Sprint 70 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: view.rs compute_critique_coverage (targets = coverage scope; per required lens find a verify-edge critique Test with method=critique+lens, an independent result) + critique_coverage(root) JSON + critique_gaps(root); guards.rs critique() delegating to critique_gaps, added to run_one but NOT GUARD_NAMES; main.rs cmd_critique_coverage. Critiques link via the #Verify marker-dependency (parses as a 'verify' edge — no parser change). Unit test for independence + lens counting. No blockers."; }
    part critiqueCoverageStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b070-51627384a706"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1f70e34"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification critiqueCoverageImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b070-51627384a707"; :>> title = "Sprint 70 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented view.rs (compute_critique_coverage/critique_coverage/critique_gaps), guards.rs (critique() in run_one, NOT GUARD_NAMES), main.rs (cmd_critique_coverage + dispatch + doc). Independence enforced (result judgedBy != element createdBy; a self-critique does NOT count — unit-tested). VERIFY (live): critique-coverage = 14 Need + 14 SR + 77 Decision, all uncovered (0 critiques), 105 gaps; `guard critique` FAIL 105; `guard all` PASS (critique not enforced -> repo still committable); clippy --workspace --all-targets -D warnings green; cargo test 33 lib pass; rust validate clean."; }
    part critiqueCoverageImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b070-51627384a708"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1f70e34"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification critiqueCoverageReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b070-51627384a709"; :>> title = "Sprint 70 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. critique_coverage reuses the coverage target scope + the Model verify-edge + the same honest-by-construction shape (no stored verdict). Independence is the key adversarial invariant and it is ENFORCED in code (not just prose) + unit-tested. The guard is deliberately runnable-but-unenforced: the human chose a hard gate, but wiring it with 0 critiques would block all commits — so it ships available, to be enforced after a genuine critique pass (recorded constraint, not a silent cap). DEFERRED (recorded): full git-temporal critique-staleness (reuse suspect) lands with assuranceReadiness, which also wires the hard gate once the model is critiqued. Architecture as a target type is not yet in the model (no Architecture elements) — required_lenses returns empty for it, ready when added. doc-sync: usage.md critique-coverage row + guard note."; }
    part critiqueCoverageReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b070-51627384a70a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1f70e34"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification critiqueCoverageCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b070-51627384a70b"; :>> title = "Sprint 70 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyCritiqueCoverage + backlog critiqueCoverage DoDs recorded pass; critique-coverage view + guard critique built (runnable, unenforced); clippy + cargo test + rust validate + guard all green. Self-evidencing (D0051). Unblocks assuranceReadiness (the composite gate; also where the hard critique gate gets wired after a genuine critique pass)."; }
    part critiqueCoverageCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b070-51627384a70c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1f70e34"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification critiqueCoverageRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b070-51627384a70d"; :>> title = "Sprint 70 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: surfacing the two policy forks (required-lens breadth; guard-blocking behavior) to the human BEFORE coding avoided baking in a heavy default — and caught the hard-gate recursion (a blocking guard would block its own wiring commit). The #Verify marker-dependency reuse meant critique linkage needed NO parser change. Independence enforced in code + tested, not left to prose. CAUGHT (avoidable-issue candidate): the user chose a HARD gate, but it cannot be honestly enforced until the model is genuinely critiqued (105 elements x Core-3 = 315 critiques) — fabricating them would be critique-theater (the exact anti-pattern the skill forbids). So the gate ships unenforced with the wiring explicitly conditioned on a genuine critique pass. This is the honest reading of 'hard gate now'; the genuine critique campaign is the real next effort (surfaced to the user). Estimate 4 pts held. Next: assuranceReadiness (composite gate) — and the critique campaign that lets the hard gate go live."; }
    part critiqueCoverageRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b070-51627384a70e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1f70e34"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
