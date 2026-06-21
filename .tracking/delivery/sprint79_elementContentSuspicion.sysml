// =====================================================================
// ProjectDeliveryS79 — Sprint 79: elementContentSuspicion (D0084; the D0005 batch-1-critique
// disposition). A verify/critique of an assurance element goes SUSPECT when the element's primary
// semantic field changes since the verifying commit — closing the re-verify/re-critique-on-change
// gap. Aligns computed-state.md. Human-accepted (D0084).
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049); the suspicion-meaning CHANGE was human-accepted (D0084).
// =====================================================================
package ProjectDeliveryS79 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from elementContentSuspicionStory to d0084;

    part elementContentSuspicionStory : Story {
        :>> id = "63940516-2c7d-4e8f-b079-51627384b001";
        :>> title = "Sprint 79: elementContentSuspicion (5 pts) — re-suspect verify/critique on element-content change (D0084)";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS79 {
        action storyElementContentSuspicion;
        verification storyElementContentSuspicionDoD : Test { :>> id = "63940516-2c7d-4e8f-b079-51627384b002"; :>> method = VerificationMethod::test; :>> procedureText = "D0084: compute_stale_verifications (view.rs) flags a verify/critique Test suspect when its target element's primary semantic field (statement for needs/reqs, decision for decisions) changed since the Test's latest result commit AND the element existed then (existed-then guard avoids same-commit create+verify false flags). Reuses orient::batch_cat_blobs (one git spawn). direct_verifiers explicit-test suspect = element-stale; charter-dod suspect = task-suspect (separate sets); compute_critique_coverage drops a stale critique's lens. computed-state.md rule 2 + status header aligned. VERIFY (live): sr11 verified->addressed (sr11Verify stale: statement amended D0083) -> re-verified R2@HEAD -> verified; d0081-83 critiques NOT flagged; coverage 14/14+14/14 verified, 80 attested; clippy -D warnings + cargo test (32 lib) + rust validate + guard all (9) green; D0084 critiqued."; }
        part storyElementContentSuspicionDoDR1 : TestResult { :>> id = "f675a41b-a0fd-4ab7-b514-383f0bf621f9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "96584de"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    }

    verification elementContentSuspicionRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b079-51627384b003"; :>> title = "Sprint 79 refine gate (DoR)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the batch-1 critique of D0005 found a real gap — nothing re-suspected a verify/critique when its element's content changed (sr11 proof). Researched align vs targeted vs full-hashing; the human chose option (b) targeted element-content suspicion. CHANGE to suspicion/coverage meaning -> D0084 + human acceptance. estimatedPoints=5. #CharteredBy d0084. DoD accepted."; }
    part elementContentSuspicionRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b079-51627384b004"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "96584de"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification elementContentSuspicionStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b079-51627384b005"; :>> title = "Sprint 79 standup gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine + acceptance passed. Approach: view.rs — latest_result (max Rn), build_element_files, semantic_field, extract_field (block-scoped field read), compute_stale_verifications (batched git via orient::batch_cat_blobs, made pub(crate)); thread a separate `stale` set through direct_verifiers (explicit-test) + compute_coverage + compute_critique_coverage, keeping task_suspect (charter-dod) distinct; align computed-state.md; D0084 + critiques; re-verify sr11 (R2). Watch: existed-then guard; un-conflating the two suspect sets. No blockers."; }
    part elementContentSuspicionStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b079-51627384b006"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "96584de"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification elementContentSuspicionImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b079-51627384b007"; :>> title = "Sprint 79 implement gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented compute_stale_verifications + threading; orient::batch_cat_blobs made pub(crate). Caught + fixed in implement: first cut MERGED the two suspect sets (charter-dod work-staleness vs element-content), breaking charter-dod — un-conflated into task_suspect + stale params. VERIFY (live): sr11 verified->addressed when sr11Verify stale (statement amended after R1@324ab15); re-verified R2@96584de -> verified; existed-then guard confirmed (d0081/d0082/d0083 critiques not flagged — they didn't exist at their parent judgedAgainst); coverage 14/14 needs + 14/14 reqs verified + 80 decisions attested + zero gaps; clippy --workspace --all-targets -D warnings green; cargo test 32 lib pass; rust validate 91 clean; guard all (9) PASS; D0084 critique-covered."; }
    part elementContentSuspicionImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b079-51627384b008"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "96584de"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification elementContentSuspicionReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b079-51627384b009"; :>> title = "Sprint 79 review gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. Option (b) implemented exactly: targeted (primary semantic field only), scoped (elements with verify/critique edges), cheap (one batched cat-file, reusing orientPerf), with the existed-then guard for correctness. The two suspect axes (charter-dod work-staleness via orient task-suspect; element-content drift via the new stale set) are correctly kept separate. The fix is proven by the sr11 round-trip (verified->addressed->re-verify->verified) and by NOT false-flagging same-sprint create+verify. computed-state.md aligned to reality (two-tier proxy) with full field-hashing recorded as the deferred superset — closing the D0005 doc/impl divergence honestly. D0084 dogfood-critiqued (correctness lens explicitly attacked the existed-then guard + the set un-conflation — survived). No doc-sync beyond computed-state.md (no other doc claimed the old mechanism)."; }
    part elementContentSuspicionReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b079-51627384b00a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "96584de"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification elementContentSuspicionCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b079-51627384b00b"; :>> title = "Sprint 79 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyElementContentSuspicion + backlog elementContentSuspicion DoDs recorded pass; D0084 accepted + critiqued; computed-state.md aligned; sr11 re-verified; coverage 14/14+14/14 verified + 80 attested, zero gaps; clippy + cargo test + rust validate + guard all (9) green. Self-evidencing (D0051). The D0005 batch-1-critique finding is now resolved with a real mechanism, not just a doc note."; }
    part elementContentSuspicionCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b079-51627384b00c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "96584de"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification elementContentSuspicionRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b079-51627384b00d"; :>> title = "Sprint 79 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the critique campaign closed a full loop on itself — batch-1 critiquing D0005 found a real staleness gap, the human picked the targeted fix, and it landed proven end to end (sr11 round-trip). The existed-then guard was the crux insight (judgedAgainst = parent commit, so same-sprint create+verify would otherwise false-flag everything) — caught at design, validated live (d0081-83 not flagged). CAUGHT (avoidable-issue candidate): the first cut MERGED charter-dod work-staleness with element-content staleness into one `stale` param, which would have wrongly driven charter-dod off element drift — caught during implement by re-reading the charter-dod branch, un-conflated into two params. Lesson: when adding a new suspicion axis, keep it ORTHOGONAL to existing ones. Estimate 5 pts held. The D0005 doc/impl divergence is resolved; computed-state.md now describes what is actually built. Remaining campaign: critique batch 2 (D0018+D0070) when the human is ready."; }
    part elementContentSuspicionRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b079-51627384b00e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "96584de"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
}
