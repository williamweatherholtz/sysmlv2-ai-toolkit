package Decision0020 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0020 : Decision {
        :>> id = "d1d2e3f4-5a6b-4c7d-8e9f-0a1b2c3d4e5f";
        :>> title = "skillsRefresh: rename agile-refinement to backlog-refinement; add test-design, test-result, traceability-audit skills";
        :>> status = DecisionStatus::accepted;
        :>> context = "The backlog item `skillsRefresh` (ready frontier) identified three changes: (1) the agile-refinement skill should be renamed backlog-refinement to match engine terminology (the process is called Backlog Refinement, not Agile Refinement); (2) no skill existed for designing Tests from a DoD procedureText; (3) no skill existed for recording TestResult instances with provenance; (4) no skill existed for auditing traceability coverage. These gaps were identified in the review phase of Sprint 5.";
        :>> decision = "Rename agile-refinement skill to backlog-refinement (git mv, update SKILL.md name+version and skills-registry.sysml), and create three new registered skills: test-design (designs Test items from DoD procedureText, writePolicy=pr-only), test-result (records appended TestResult instances with full provenance triad, writePolicy=direct), and traceability-audit (audits satisfy/verify coverage gaps + suspects + orphans, writePolicy=readOnly).";
        :>> rationale = "Terminology should match: the process is called Backlog Refinement, not Agile Refinement. The three missing skills (designing Tests from a DoD, recording TestResults with provenance, auditing traceability) were gaps identified in the Sprint 5 review. Applied as part of Sprint 5 review phase (skillsRefresh backlog task); all four registered in skills-registry.sysml. Accepted: wweatherholtz standing instruction (fix internal processes in each sprint review), 2026-06-15.";
        :>> consequences = "Terminology aligned: skills and processes now use the same vocabulary. Test-design, test-result, and traceability-audit skills are available for the next sprint and downstream use. The registry accurately reflects what exists on disk.";
    }

    verification d0020Accept : Test { :>> id = "fa3cd5ac-d16e-459b-9dda-68241e9b9ea8"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0020 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0020AcceptR1 : TestResult { :>> id = "149a7236-d1cf-4ed9-ba9c-f892717fadc7"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
