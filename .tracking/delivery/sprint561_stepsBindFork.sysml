// ProjectDeliveryS561 — Sprint 561: the process-authority fork written and surfaced - may a ProcessStep declare the guard that checks it. EXECUTE. 2 pts.
package ProjectDeliveryS561 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from stepsBindForkStory to d0321;

    part stepsBindForkStory : Story {
        :>> id = "152c809d-6f47-44d5-ba6c-0633740c0662";
        :>> title = "Sprint 561: the process-authority fork written and surfaced - may a ProcessStep declare the guard that checks it (2 pts)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS561 {
        action storyStepsBindFork;
        verification storyStepsBindForkDoD : Test { :>> id = "f806beb4-dca4-4e22-831b-d66fddf4f9c5"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcProcessStepsBindToChecks (the FORK D0321, proposed for the human). DONE: measured with keel show hardening (37 processes, 15 enforced, 10 declared judgments, 12 undeclared) and each undeclared process read against its steps - 7 bindable to an existing guard today, 4 joining the declared judgments, githubIntake and projectMigration mixed; three options costed (A checkedBy on the step, B A plus addressable analytical facts, C a contract with no schema change), recommendation A with what would change it; judgment-request-quality passes; published with D0312 on the one pending-forks page."; }
        // RAN: keel guard judgment-request-quality . PASS; keel validate . clean; no engine source changed in this sprint
        part storyStepsBindForkDoDR1 : TestResult { :>> id = "64b02757-1d79-47f3-a518-21a656d24ba5"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "18bbdc997f4aebe3eb740393643bda7cb4b11c58"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "claudeOpus5"; }
    }

    verification stepsBindForkRefineGate : Test { :>> id = "06297b7f-688a-4a79-bae9-bf562fd66ba8"; :>> title = "Sprint 561 refine gate (DoR)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: the panel's two findings named one root and the DoD asked for a proposal, not a build, because both halves are frozen-core questions. The measurement was the work - reading twelve process files for which step a check could attach to."; }
    verification stepsBindForkStandupGate : Test { :>> id = "a97713aa-b988-400b-9f98-32cbd4790529"; :>> title = "Sprint 561 standup gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE FINDING WORTH THE HUMAN'S EYE: the gap is a place to write the fact, not a missing check - 7 of 12 undeclared processes already have a guard that is their step's predicate. That is why the recommendation is the attribute (A) over the contract (C): the contract is the D0105 second-home shape process-enforcement.toml already is."; }
    verification stepsBindForkImplementGate : Test { :>> id = "25915375-a544-4ea5-8c70-e2f085d99766"; :>> title = "Sprint 561 implement gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: keel show hardening . (processEnforcement 37/15/10/12); keel record decision --from d0321 (fork, stays proposed); keel guard judgment-request-quality . PASS 2 scanned 0 violations; the page published as an Artifact and the URL leads the report."; }
    verification stepsBindForkReviewGate : Test { :>> id = "119b8a86-5106-4a78-be7b-c9384915354b"; :>> title = "Sprint 561 review gate — three passes per D0172"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) IS IT A REAL FORK: yes - each option is defensible and the human's cost differs (frozen-core change now, later, or never). (2) IS THE MEASUREMENT REPRODUCIBLE: the lens is computed and the per-process reading names its guard by number. (3) IS ANYTHING DECIDED BY DEFAULT: no - nothing is built until they answer; the fork is carried on the page every turn."; }
    verification stepsBindForkCloseOutGate : Test { :>> id = "047fe059-675a-4383-99df-c4225304f075"; :>> title = "Sprint 561 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. D0321 proposed; dcProcessStepsBindToChecks's DoD is the proposal and passes."; }
    verification stepsBindForkRetroGate : Test { :>> id = "8a2f5381-5644-4549-a7f1-1009ba8fd29a"; :>> title = "Sprint 561 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: dcDecisionClausesAreAddressable is option B's second half and stays on the backlog until the human answers; dcVerificationByAuthority (D0312) sits on the same page. The lesson: a proposal's measurement should be read off the computed lens and then off the files - the lens said 12, the files said which 7 could bind."; }
}
