package Decision0040 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0040 : Decision {
        :>> id = "be9e4fd3-1b2c-4d3e-efa4-5c6d7e8f9a0b";
        :>> title = "recurring-task-skill-rule: check recurring vs one-time before executing; skill-before-execute for recurring tasks; status-report skill + report.py added";
        :>> status = DecisionStatus::accepted;
        :>> context = "Recurring tasks (code review, report generation, deployment, environment setup, estimation) were being executed ad-hoc without checking if a skill existed or should be created. This allowed process knowledge to escape into conversation history rather than being encoded in durable, reusable skills. The engine-triage classification step had no gate for this. The result: the same guidance was re-derived each time, and process improvements were not captured systematically.";
        :>> decision = "Add a mandatory 'recurring or one-time?' pre-execution check to the interaction loop (CLAUDE.md §3 and engine-triage): for any EXECUTE or VIEW, if it will recur and no skill exists -> CHANGE (create/update a skill first, then execute using it); if clearly one-time -> execute directly; if ambiguous -> ask. Also add the status-report skill and report.py.";
        :>> rationale = "Recurring tasks (code review, report generation, deployment, environment setup, estimation) were being executed ad-hoc without checking whether a skill existed or should be created, letting process knowledge escape into conversation history rather than durable reusable skills; the engine-triage classification step had no gate for this, so the same guidance was re-derived each time and improvements were not captured systematically. Examples: 'I am on Windows' = recurring environment fact -> CLAUDE.md §6; 'make an HTML report' = recurring -> ask + create status-report skill; 'review sprint transcript' = recurring -> sprint-review skill (already exists). status-report is a VIEW skill generating the HTML project dashboard; report.py generates a self-contained HTML dashboard from tracking files + git + sysmlv2 CLI orient JSON.";
        :>> consequences = "Every execution of a recurring task is preceded by a skill check. New skills are created when recurring tasks lack them. Engine-triage and CLAUDE.md §3 are both updated. Status report is a materialized VIEW (regenerable, marked as derived). Skills registry grows by one (status-report). Report.py is a BOOTSTRAP tool tracked in the backlog.";
    }

    verification d0040Accept : Test { :>> id = "9d7699f1-1a30-488f-beca-98afc51001e9"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0040 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0040AcceptR1 : TestResult { :>> id = "a5996b99-15cc-461a-a113-1507dd829624"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
