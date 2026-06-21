// =====================================================================
// D0084 — targeted element-content suspicion (the D0005 material-change disposition). A
// verify/critique of an assurance element goes SUSPECT when the element's primary semantic field
// changed since the verification's latest result commit (and it existed then). Closes the
// re-verify/re-critique-on-change gap; aligns computed-state.md to the real two-tier mechanism.
// Accepted 2026-06-21.
// =====================================================================
package Decision0084 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0084 : Decision {
        :>> id = "63940516-2c7d-4e8f-a084-5162738496b0";
        :>> title = "D0084: targeted element-content suspicion — re-suspect a verify/critique when its element's semantic field changes; align computed-state.md to the two-tier material-change proxy";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The D0001/D0005 batch-1 critique (Sprint 77) found that D0005 rule 2 (and computed-state.md) specify material-change as hashing every element's semantic field set, but the implementation only watched two proxies: a task DoD criterion text, and deliverable-manifest source paths. Nothing re-suspected a verification or critique when the ELEMENT IT VERIFIES changed its own content. Demonstrated live: amending sr11FastStart's statement (D0083) left sr11Verify falsely fresh, and an amended Decision would keep a stale critique — partially breaking D0080's re-critique-on-change promise. The human researched align-text vs full-hashing and chose option (b): targeted element-content suspicion.";
        :>> decision = "Add element-content staleness to the coverage/critique computation. For a Need / SystemRequirement / accepted Decision carrying a verify or critique edge, compute the element's PRIMARY SEMANTIC FIELD (statement for needs/requirements, decision for decisions) at the verification's LATEST result commit and compare to its current value; if they differ AND the element existed at that commit, mark that verify/critique Test SUSPECT. Effect: a stale explicit-test verifier no longer confers verified (the element drops below verified — to addressed if other coverage exists, else suspect/uncovered); a stale critique no longer covers its lens (re-critique needed). The element-existed-then guard prevents false flags on same-commit create+verify. Reuse orient's batched git cat-file (one spawn). ALIGN computed-state.md rule 2 to describe the real TWO-TIER material-change (criterion/source-path drift + element-content drift), with general per-field hashing recorded as the deferred aspirational superset. Scope: element-content drift watches the ONE primary semantic field per type, not all fields.";
        :>> rationale = "Option (b) fixes the genuine gap (re-verify/re-critique on element change, honoring D0080) at low, already-built cost (the orientPerf batched cat-file machinery), while avoiding the two costs of full field-hashing (option c): the fragile semantic-vs-cosmetic field classification per metaclass (the exact suspicion-storm risk D0005 warns about) and per-element historical reads across deep history. Watching the one primary semantic field per type is unambiguous (statement / decision are THE content) and scoped to elements that actually carry verification, so non-verified elements produce no noise. ALTERNATIVES: (a) align the text only, accept the blind spot — REJECTED (the gap demonstrably bites: sr11); (c) full per-field hashing — REJECTED for now (heaviest + most error-prone, marginal benefit over (b)); (b CHOSEN) targeted element-content suspicion + align the contract. The element-existed-then guard is the key correctness subtlety (judgedAgainst records the parent commit, so without it every same-sprint create+verify would falsely flag). Pairs with D0005 (material-change), D0050 (suspect), D0079/D0080/D0082 (assurance/critique/tiers), orientPerf (batched git).";
        :>> consequences = "view.rs: compute_stale_verifications (build_element_files + semantic_field + extract_field + latest_result, reusing orient::batch_cat_blobs) feeds direct_verifiers (explicit-test suspect) + compute_critique_coverage (a stale critique drops its lens); compute_coverage now takes task_suspect (charter-dod work staleness) AND stale (element-content) separately. computed-state.md rule 2 + the implementation-status header aligned. First application: sr11Verify flagged stale (statement amended after its commit) -> sr11 re-verified at HEAD (sr11VerifyR2) -> back to verified, demonstrating the loop. D0084 is dogfood-critiqued. KNOWN LIMITATION: watches only the primary semantic field per type (statement/decision) — a change to a secondary field (e.g. a decision's rationale) won't flag; acceptable (the primary field is the load-bearing content). Decisions stay immutable historical records; D0005's decision text is unchanged — only the living contract is aligned.";
    }

    verification d0084Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a084-5162738496b1";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0084 (targeted element-content suspicion: re-suspect a verify/critique when its element's primary semantic field changes since the verification's latest result commit, existed-then guarded; align computed-state.md to the two-tier proxy; defer full field-hashing) by selecting 'Build (b): targeted element-content suspicion' and then 'd0005 please' in the D0005-disposition dialogue on 2026-06-21.";
    }
    part d0084AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a084-5162738496b2";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "96584de";
        :>> judgedAt = "2026-06-21";
        :>> judgedBy = "wweatherholtz";
    }
}
