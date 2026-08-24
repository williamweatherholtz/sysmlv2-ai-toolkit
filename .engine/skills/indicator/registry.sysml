// The indicator skill's own registry declaration, beside the skill it describes (D0222).
//
// MOVED verbatim out of the shared skills-registry by a committed migration transform (D0222).
// The shared registry
// could not travel with a unit, so adopting any unit left the receiving project's
// `process-skill` guard failing on arrival (issue252). Moved, never copied: the id is
// unchanged and `duplicate-identity` is what proves it.
package SkillsRegistryIndicator {
    private import EngineElement::*;
    private import EngineSkills::*;

    part indicatorSkill : AISkill {
        :>> id = "63940516-2c7d-4e8f-a089-516273849617";
        :>> title = "indicator";
        :>> purpose = "Deploy the Indicator Monitoring process (D0089): declare monitored measures (Indicators) that inform by DIRECTION not threshold (D0088), and collect data by method — computed (repo-derived, `keel indicators`), pulled (external programmatic) or manual (subjective), recorded via `keel record-measurement`. A Measurement is one datapoint; never fabricate an external/manual observation. ISO-15939/PSM/SMM-grounded. The deploying skill for .engine/processes/indicator.sysml.";
        :>> location = ".engine/skills/indicator/SKILL.md";
        :>> triggerCondition = "Asked to track/watch/monitor a metric or KPI over time, add an indicator, record a measurement, pull an external signal (market/regulation/social), or see how a measure is trending.";
        :>> writePolicy = WritePolicy::direct;
    }
}
