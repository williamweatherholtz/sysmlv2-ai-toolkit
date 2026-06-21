// =====================================================================
// ProjectDeliveryS67 — Sprint 67: modelIngestRequirements (D0079 assurance milestone prereq).
// Story: teach the Rust view Model to ingest requirement/need USAGES (requirement X : T) as items
//        — the shared prerequisite for assurance-coverage (C) + the antagonistic critique, both of
//        which target needs/requirements (previously invisible to the Model).
// Total: 2 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS67 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from modelIngestRequirementsStory to d0079;

    part modelIngestRequirementsStory : Story {
        :>> id = "63940516-2c7d-4e8f-b067-51627384a401";
        :>> title = "Sprint 67: modelIngestRequirements (2 pts) — ingest requirement/need usages into the view Model; D0079 prereq";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    action def DeliveryRunS67 {
        action storyModelIngestRequirements;
        verification storyModelIngestRequirementsDoD : Test { :>> id = "63940516-2c7d-4e8f-b067-51627384a402"; :>> method = VerificationMethod::test; :>> procedureText = "modelIngestRequirementsDoD: parser.rs now emits Item::Part for a requirement USAGE (requirement X : T -> Part{type_name=T}), mirroring the part-usage arm; `requirement def` still -> TypeDef. So Need/Requirement usages are ingested as items (assurance-coverage + critique target them). VERIFY: the view Model sees 14 Need + 14 SystemRequirement items (was 0); rust validate 77 clean; cargo test --workspace all pass (incl parser BDD); clippy --workspace --all-targets -D warnings green."; }
        part storyModelIngestRequirementsDoDR1 : TestResult { :>> id = "d6adccb5-0a7f-4794-a128-1b730e3d2b10"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b7bb0b"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification modelIngestRequirementsRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b067-51627384a403"; :>> title = "Sprint 67 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: assurance-milestone prerequisite (surfaced in the assuranceCoverage refine investigation): the Model ingests only Parts/Decisions, not requirement/need usages (saw 0 Need items), so neither coverage nor critique could target them. Fix: parse requirement usages as Part. Small parser change. estimatedPoints=2. #CharteredBy to d0079. DoD accepted."; }
    part modelIngestRequirementsRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b067-51627384a404"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b7bb0b"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification modelIngestRequirementsStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b067-51627384a405"; :>> title = "Sprint 67 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: in parser.rs the TokenKind::Requirement arm, after the `def` check, parse the usage via parse_typed_item_body -> Item::Part{type_name} (mirrors the Part arm; respects had_abstract). Verify by a throwaway type=Need / type=SystemRequirement view + cargo test (parser BDD). No blockers."; }
    part modelIngestRequirementsStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b067-51627384a406"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b7bb0b"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification modelIngestRequirementsImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b067-51627384a407"; :>> title = "Sprint 67 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "parser.rs TokenKind::Requirement arm now parses usages -> Item::Part{type_name} (def still -> TypeDef; abstract skipped). VERIFY (live): throwaway views show 14 Need + 14 SystemRequirement items (was 0); rust validate 77 clean; cargo test --workspace all pass (28 lib + parser/cli BDD); clippy --workspace --all-targets -D warnings green."; }
    part modelIngestRequirementsImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b067-51627384a408"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b7bb0b"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification modelIngestRequirementsReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b067-51627384a409"; :>> title = "Sprint 67 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The change mirrors the existing part-usage arm exactly (parse_typed_item_body), so requirement usages become first-class items the view engine + coverage + critique can target — with zero new authoring (the needs/requirements already exist in business.sysml). `requirement def` still maps to TypeDef (unchanged). Unblocks assuranceCoverage (next). The actual coverage/critique semantics (charter-computed, addressed-vs-verified depth) come in the following sprints."; }
    part modelIngestRequirementsReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b067-51627384a40a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b7bb0b"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification modelIngestRequirementsCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b067-51627384a40b"; :>> title = "Sprint 67 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyModelIngestRequirements + backlog modelIngestRequirements DoDs recorded pass; Model ingests 14 Need + 14 SystemRequirement; cargo test + clippy + rust validate + sysmlv2 guard all green. Self-evidencing (D0051). Unblocks assuranceCoverage."; }
    part modelIngestRequirementsCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b067-51627384a40c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b7bb0b"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification modelIngestRequirementsRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b067-51627384a40d"; :>> title = "Sprint 67 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the assuranceCoverage refine investigation caught a real blocker BEFORE building on sand — the Model couldn't see requirements/needs at all (0 items) + there were no verify edges. Splitting out this 2pt ingestion prereq (vs forcing C onto an unsupported model) is exactly the right move, and it's tiny because it mirrors the part-usage arm. CAUGHT (the avoidable issue): nearly built assuranceCoverage assuming the Model saw requirements — investigating the data model first prevented it. No new tracked item (the refine-investigation discipline worked). Estimate 2 pts held. Next: assuranceCoverage (charter-computed coverage, addressed-vs-verified depth) on the now-visible needs/requirements."; }
    part modelIngestRequirementsRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b067-51627384a40e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8b7bb0b"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
