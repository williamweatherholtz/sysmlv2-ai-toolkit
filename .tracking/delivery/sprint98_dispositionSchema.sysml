// =====================================================================
// ProjectDeliveryS98 — Sprint 98: dispositionSchema (D0092 milestone 1). Formalize the finding-
// disposition lifecycle — STEP 1, the frozen-core schema delta (human-accepted, re-frozen):
// `enum def DispositionKind { act; acceptRisk; dismiss; }` + optional `disposition : DispositionKind[0..1]`
// on verification def Test. A disposition becomes a method=confirmation verification carrying a typed
// verdict, #Dispositions-linked to the finding (the write path + computed view + backfill follow as
// dispositionWritePath / dispositionBackfill). Schema/core re-frozen after this one accepted change.
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS98 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from dispositionSchemaStory to d0092;

    part dispositionSchemaStory : Story {
        :>> id = "63940516-2c7d-4e8f-b098-51627384c001";
        :>> title = "Sprint 98: dispositionSchema (2 pts) — DispositionKind enum + optional Test.disposition attr (D0092 m1; frozen-core, human-accepted, re-frozen)";
        :>> createdAt = "2026-06-22";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS98 {
        action storyDispositionSchema;
        verification storyDispositionSchemaDoD : Test { :>> id = "63940516-2c7d-4e8f-b098-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "D0092 milestone 1 (the gated frozen-core change, human-accepted via the schema-unfreeze dialogue): add `enum def DispositionKind { act; acceptRisk; dismiss; }` to schema/core/element.sysml + optional `attribute disposition : DispositionKind[0..1]` to verification def Test in schema/core/verification.sysml (additive — every existing Test still parses). Author D0092 (status=accepted, #ProspectiveChange, acceptance event d0092Accept judgedBy=wweatherholtz) + its dogfood Core-3 critiques. schema/core RE-FROZEN after. VERIFY: kernel validate_schema 15/15; kernel validate_instances 107/107 (d0092 PASS); rust validate 111 clean; guard all (13) PASS (acceptance-events covers d0092Accept; critique covers d0092 Core-3). Write path + computed view + backfill are the follow-on milestones (dispositionWritePath / dispositionBackfill); doc-sync of the disposition mechanism rides dispositionWritePath (no current doc claim is invalidated by the additive enum alone)."; }
        part storyDispositionSchemaDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b098-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3126545"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }
    }

    verification dispositionSchemaRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b098-51627384c004"; :>> title = "Sprint 98 refine gate (DoR)"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0092 (formalize-disposition-lifecycle) was human-accepted ('Accept as proposed', incl. the schema-unfreeze sign-off). Milestone 1 = the schema delta. Resolver: dispositionSchema. estimatedPoints=2. #CharteredBy d0092. DoD accepted."; }
    part dispositionSchemaRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b098-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3126545"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification dispositionSchemaStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b098-51627384c006"; :>> title = "Sprint 98 standup gate"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: add the enum to element.sysml + the optional attr to verification def Test; author D0092 with #ProspectiveChange + acceptance event (import EngineRelationships for the marker); add d0092 dogfood Core-3 critiques so the critique guard stays green; kernel-validate schema + instances; re-freeze. No blockers."; }
    part dispositionSchemaStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b098-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3126545"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification dispositionSchemaImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b098-51627384c008"; :>> title = "Sprint 98 implement gate"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented the schema delta + D0092 + d0092 Core-3 critiques. VERIFY (live): kernel validate_schema 15/15 (element.sysml + verification.sysml PASS with DispositionKind + disposition attr); kernel validate_instances 107/107 (d0092 PASS after EngineRelationships import); rust validate 111 clean; guard all (13) PASS."; }
    part dispositionSchemaImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b098-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3126545"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification dispositionSchemaReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b098-51627384c00a"; :>> title = "Sprint 98 review gate"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The frozen-core touch is bounded + additive (1 enum + 1 optional attr; existing Tests parse unchanged), human-accepted, and re-frozen — honoring CLAUDE.md 2.5. The disposition-as-confirmation-verification model reuses the assurance substrate (the D0080 economy) so dispositions will inherit provenance + staleness. This is step 1 of 3; no behavior is wired yet, so no doc claim is invalidated (the mechanism doc-sync rides dispositionWritePath). Clean."; }
    part dispositionSchemaReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b098-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3126545"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification dispositionSchemaCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b098-51627384c00c"; :>> title = "Sprint 98 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyDispositionSchema DoD recorded pass; DispositionKind + Test.disposition live + kernel-clean; D0092 accepted + critiqued; schema re-frozen; rust validate 111 + guard all (13) green. Self-evidencing (D0051). The disposition lifecycle's foundation is in; write path + backfill are the next milestones."; }
    part dispositionSchemaCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b098-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3126545"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }

    verification dispositionSchemaRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b098-51627384c00e"; :>> title = "Sprint 98 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-22"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the human's two probing questions ('is ACT/ACCEPT-RISK/DISMISS formalized?' + 'are we straying into prose/bespoke data?') surfaced a REAL gap — the engine's most important recurring human act (judging a finding) was prose-in-an-issue-description — and steered batch-4's two findings (issue039/040) onto a sounder substrate (formalize the lifecycle first, then ride it) rather than bolting on bespoke markers; reusing method=confirmation kept the schema delta to 1 enum + 1 attr. CAUGHT (avoidable, fixed same-sprint): the D0092 file initially omitted `import EngineRelationships::*`, so the #ProspectiveChange marker failed kernel resolution (108.sysml line 21) — the SAME omission pattern seen before; lesson: a Decision carrying #ProspectiveChange/#SafetyChange MUST import EngineRelationships (candidate: a lint that a marker-bearing instance imports its marker's package). Estimate 2 pts held. Next: dispositionWritePath (apply-review + computed view), then dispositionBackfill, then issue039/040 ride the lifecycle."; }
    part dispositionSchemaRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b098-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3126545"; :>> judgedAt = "2026-06-22"; :>> judgedBy = "claudeOpus"; }
}
