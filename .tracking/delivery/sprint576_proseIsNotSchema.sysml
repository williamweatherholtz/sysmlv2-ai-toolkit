// ProjectDeliveryS576 — Sprint 576: migrate reads schema, not the prose inside it - a quoted foreign symbol stops blocking a healthy project. EXECUTE. 1 pts.
package ProjectDeliveryS576 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from proseIsNotSchemaStory to d0341;

    part proseIsNotSchemaStory : Story {
        :>> id = "8e6f9d01-2598-4d7b-8cb6-ad4d680a5af5";
        :>> title = "Sprint 576: migrate reads schema, not the prose inside it - a quoted foreign symbol stops blocking a healthy project (1 pts)";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS576 {
        action storyProseIsNotSchema;
        verification storyProseIsNotSchemaDoD : Test { :>> id = "e4222267-5ae9-4ce8-8f23-177072c3ad1a"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcRemovedTypeScannerReadsSchemaNotProse (D0341; resolves issue377, GH#59). DONE: without_string_literals blanks double-quoted literals (structure kept, escaped quotes honoured) and step_removed_types runs both the type and the enum scan on that form while reporting the original line. VERIFIED: the regression test's two arms - a description quoting ureq::Agent, std::task::Task and RiskLevel::high produces no blocker; a real part a : Agent and a real RiskLevel::high value outside quotes still block by name - plus the old-vintage arms unchanged."; }
        // RAN: dcRemovedTypeScannerReadsSchemaNotProse DoD pass recorded on suite577 543/0; the sprint's implement section names each run
        part storyProseIsNotSchemaDoDR1 : TestResult { :>> id = "f6db681d-4ee6-4165-a6b1-533a317bd5b2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5e3a8285c6ad296313624db9da86390071ae94ab"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    }

    verification proseIsNotSchemaRefineGate : Test { :>> id = "ac1d3e3c-8367-4c12-956d-6153ee04be58"; :>> title = "Sprint 576 refine gate (DoR)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: the reporter had located types_as and step_removed_types, implemented the blanking, and falsified it with a passthrough; the work was to land the same shape with its test and confirm the existing arms."; }
    verification proseIsNotSchemaStandupGate : Test { :>> id = "8976b298-0e5c-4277-918b-563d4f5a01f3"; :>> title = "Sprint 576 standup gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE READER, ONE SCOPE: the colon test is correct SysML; the defect was feeding it text that is not SysML. Fixing the reader's scope fixes Task, Risk, Role and every other removed name at once."; }
    verification proseIsNotSchemaImplementGate : Test { :>> id = "f47ef592-7c11-43b6-95a0-6d709196d76a"; :>> title = "Sprint 576 implement gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: cargo test --lib migrate 11 passed including a_removed_type_named_inside_a_string_literal_is_prose_not_schema; clippy -D warnings clean; the full suite in suite577.txt."; }
    verification proseIsNotSchemaReviewGate : Test { :>> id = "762af5ae-c261-40ac-bd5e-1b09dd0f0778"; :>> title = "Sprint 576 review gate — three passes per D0172"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "TWO PASSES. (1) CAN THE FIX PASS BY NEVER MATCHING: no - the second arm requires the real annotation and the real enum value to block. (2) IS ANYTHING LOST: a type annotation never appears inside a string literal, so the blanked form carries every declaration the scanners are entitled to see."; }
    verification proseIsNotSchemaCloseOutGate : Test { :>> id = "69672125-5830-41ba-90f4-484705b709b0"; :>> title = "Sprint 576 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. issue377 resolves on this pass; the reporter's project migrates past the blocker on the next main build."; }
    verification proseIsNotSchemaRetroGate : Test { :>> id = "e81a479a-06c1-4528-9871-daad7fb68155"; :>> title = "Sprint 576 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: issue380 records that fixes on main since v0.3.1 reach adopters only through a release, and this fix is one more of them. The lesson: a scanner over authored text needs a stated scope - schema or prose - before its first match rule."; }
}
