// The report skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryReport {
    private import EngineElement::*;
    private import EngineSkills::*;

    part reportSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a087-516273849616";
        :>> title = "report";
        :>> purpose = "Deploy the Computed Report / Scorecard process (D0087): generate human-digestible aggregate reports via `keel report <assurance|traceability|quality-debt|flow> [--html]` — coverage %, critique %, attestation %, findings, traceability completeness, debt, volatility, flow/velocity — rolled up from the per-element views and split into HEALTH vs OPPORTUNITY; never author a metric or commit a snapshot (computed #View, §2.1/D0015 — git-ignored). The deploying skill for .engine/processes/report.sysml.";
        :>> location = ".engine/skills/report/SKILL.md";
        :>> triggerCondition = "Asked for a status/health report, coverage or test totals/%, a scorecard, metrics/KPIs, model health, technical debt, velocity, or 'how are we doing / where can we improve'.";
        :>> writePolicy = WritePolicy::direct;
    }
}
