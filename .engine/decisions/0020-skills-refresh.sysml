package Decision0020 {
    private import EngineWork::*;

    part d0020 : Decision {
        :>> id = "d1d2e3f4-5a6b-4c7d-8e9f-0a1b2c3d4e5f";
        :>> title = "skillsRefresh: rename agile-refinement to backlog-refinement; add test-design, test-result, traceability-audit skills";
        :>> status = DecisionStatus::accepted;
        :>> context = "The backlog item `skillsRefresh` (ready frontier) identified three changes: (1) the agile-refinement skill should be renamed backlog-refinement to match engine terminology (the process is called Backlog Refinement, not Agile Refinement); (2) no skill existed for designing Tests from a DoD procedureText; (3) no skill existed for recording TestResult instances with provenance; (4) no skill existed for auditing traceability coverage. These gaps were identified in the review phase of Sprint 5.";
        :>> decisionText = "Applied as part of Sprint 5 review phase (skillsRefresh backlog task). Actions taken: (1) .engine/skills/agile-refinement/ renamed to .engine/skills/backlog-refinement/ via git mv; SKILL.md name + version updated; skills-registry.sysml updated title + location + purpose. (2) .engine/skills/test-design/SKILL.md created: designs Test items from DoD procedureText with correct VerificationMethod, Gherkin scenarios, writePolicy=pr-only. (3) .engine/skills/test-result/SKILL.md created: records appended TestResult instances with full provenance triad, writePolicy=direct. (4) .engine/skills/traceability-audit/SKILL.md created: audits satisfy/verify coverage gaps + suspects + orphans, writePolicy=readOnly. All four registered in skills-registry.sysml. Accepted: wweatherholtz standing instruction (fix internal processes in each sprint review), 2026-06-15.";
        :>> consequences = "Terminology aligned: skills and processes now use the same vocabulary. Test-design, test-result, and traceability-audit skills are available for the next sprint and downstream use. The registry accurately reflects what exists on disk.";
    }
}
