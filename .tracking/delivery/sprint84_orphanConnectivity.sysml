// =====================================================================
// ProjectDeliveryS84 — Sprint 84: orphanConnectivity (D0085) — the human asked why the diagram
// shows so many disconnected nodes. Diagnosis: it drew only the ~278 authored TYPED edges, but most
// model structure is CONTAINMENT (action/verification ∈ def, step ∈ process) and NAMING (TestResult
// <x>R<n> ∈ Test <x>) — neither a typed edge, so ~1900 nodes floated. Fix: DERIVE those edges in the
// computed Model (contains + resultof) so leaves connect to their structural parent; plus clear the
// one real gap the orphans view found (issue003.relatedTask -> a removed task).
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049); diagram-view enhancement under the accepted D0085.
// =====================================================================
package ProjectDeliveryS84 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from orphanConnectivityStory to d0085;

    part orphanConnectivityStory : Story {
        :>> id = "63940516-2c7d-4e8f-b084-51627384b501";
        :>> title = "Sprint 84: orphanConnectivity (3 pts) — derive contains + resultof structural edges so the diagram connects floaters to their parents; clear issue003 dangling relatedTask";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS84 {
        action storyOrphanConnectivity;
        verification storyOrphanConnectivityDoD : Test { :>> id = "63940516-2c7d-4e8f-b084-51627384b502"; :>> method = VerificationMethod::test; :>> procedureText = "The diagram's disconnected nodes were a RENDERING gap, not a model gap (orphans view was clean but for 1 dangling relatedTask). DERIVE the implicit structural edges in the computed Model (view.rs): `contains` (ActionDef -> its parts/verifications/actions via AST nesting; Process -> its ProcessSteps via same-package co-membership) and `resultof` (TestResult `<test>R<n>` -> Test `<test>` via naming, strip_result_suffix). New edge kinds are inert to coverage/critique/charter/assured (all filter by specific kind). Diagram edgeColors gain contains/resultof. VERIFY (live): `sysmlv2 diagram .` edges 278 -> 2160 (contains 909 + resultof 973), orphan nodes ~1900 -> 158 (residual = genuinely-unlinked early Decisions/grandfathered Stories/reference items); coverage UNCHANGED (110 elements, 0 gaps); clippy --workspace -D warnings + cargo test (33 lib) + rust validate + guard all (9) green."; }
        part storyOrphanConnectivityDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b084-51627384b503"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5f47bb9"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    }

    verification orphanConnectivityRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b084-51627384b504"; :>> title = "Sprint 84 refine gate (DoR)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the human asked why the diagram has 'lots of orphaned, disconnected items' and whether there should be relationships. Diagnosis: the diagram drew only the ~278 authored typed edges; ~1900 nodes float because their relationship is CONTAINMENT (action/verification ∈ def, step ∈ process) or NAMING (TestResult <x>R<n> ∈ Test <x>) — neither a typed edge. The relationships EXIST; the diagram should DERIVE + draw them (computed, nothing authored — §2.1). The orphans view confirmed the MODEL is sound (1 dangling relatedTask only). Human accepted: derive contains + resultof; fix issue003. estimatedPoints=3. #CharteredBy d0085. DoD accepted."; }
    part orphanConnectivityRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b084-51627384b505"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5f47bb9"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification orphanConnectivityStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b084-51627384b506"; :>> title = "Sprint 84 standup gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine + acceptance passed. Approach: (1) Model::ingest emits `contains` ad.name->each nested part/verification/action; a package-level pass links each Process to the ProcessSteps in its package. (2) Model::build post-pass adds `resultof` TestResult-><test> via strip_result_suffix (rfind 'R' + all-digits tail). (3) diagram edgeColors += contains/resultof (greys). New kinds NOT added to KNOWN_EDGES (that gates only view-referenced kinds; succession is precedent). (4) Repoint issue003.relatedTask trackedMetadata (removed) -> trackedMetadataReplan (surviving successor). Watch: keep new edges inert to assurance views; confirm coverage counts unchanged. No blockers."; }
    part orphanConnectivityStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b084-51627384b507"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5f47bb9"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification orphanConnectivityImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b084-51627384b508"; :>> title = "Sprint 84 implement gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented contains (ActionDef nesting + Process->step same-package) + resultof (strip_result_suffix) in view.rs; edgeColors += contains/resultof; issue003.relatedTask repointed to trackedMetadataReplan. VERIFY (live): `sysmlv2 diagram .` -> edges 278->2160 (contains 909, resultof 973, plus the authored kinds), orphan nodes ~1900->158; `sysmlv2 coverage .` UNCHANGED (110 elements / 0 gaps / 0 governed_gaps); `sysmlv2 orphans .` dangling_relatedTask now []; clippy --workspace --all-targets -D warnings green; cargo test 33 lib pass; rust validate clean; guard all (9) PASS. Diagram re-delivered to the human."; }
    part orphanConnectivityImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b084-51627384b509"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5f47bb9"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification orphanConnectivityReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b084-51627384b50a"; :>> title = "Sprint 84 review gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The derived contains/resultof edges are pure computed structure (the def OWNS its nested children; a TestResult IS a run of its Test) — nothing authored, regenerable, consistent with §2.1; committing no rendered output (/*.html gitignored). Blast radius checked: the new kinds are never referenced by coverage/critique/charter/assured (all filter by an explicit kind) — confirmed by coverage holding at 110/0 — and KNOWN_EDGES is untouched (it gates view-referenced kinds; succession sets the precedent for a Model-only kind). Orphans dropped ~1900->158; the residual 158 are HONEST no-edge items (early pre-charter Decisions, grandfathered Stories, reference Personas/Skills/Viewpoints) — not gaps to force-fit. issue003's relatedTask now points at the surviving task (trackedMetadataReplan), not a fabricated link. Honest scope: structural edges machine-verified; the in-browser render is the human's confirmation (D0051/the Sprint 83 lesson)."; }
    part orphanConnectivityReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b084-51627384b50b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5f47bb9"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification orphanConnectivityCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b084-51627384b50c"; :>> title = "Sprint 84 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyOrphanConnectivity DoD recorded pass; contains/resultof derived in the computed Model; diagram orphans ~1900->158; issue003 dangling relatedTask cleared; coverage unchanged (110/0); clippy + cargo test (33) + rust validate + guard all (9) green. Self-evidencing (D0051); browser render is the human's confirmation. The comprehensive diagram now reflects real model structure, not just authored edges."; }
    part orphanConnectivityCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b084-51627384b50d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5f47bb9"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification orphanConnectivityRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b084-51627384b50e"; :>> title = "Sprint 84 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the human's 'why so many orphans' question had a clean computed answer — the orphans VIEW already proved the model was sound (1 real gap), so the fix was rendering, not authoring; deriving contains/resultof connected ~1900 floaters without storing a thing, and the residual 158 self-classify as honest no-edge items. CAUGHT (avoidable-issue candidate): Sprint 82 shipped the diagram drawing ONLY authored typed edges, which made a structurally-sound model look broken — an avoidable presentation gap I could have caught by checking the edge:node ratio (278:2540) at build time. Lesson: a 'comprehensive' graph view must render derivable structural relationships (containment, naming-convention results), not only authored typed edges, or it misrepresents connectivity. Not worth a guard (one-off view-quality judgment, not a recurrable correctness defect). Estimate 3 pts held. The diagram is now a faithful structural lens."; }
    part orphanConnectivityRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b084-51627384b50f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5f47bb9"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
}
