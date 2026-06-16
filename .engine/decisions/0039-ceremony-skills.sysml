package Decision0039 {
    private import EngineWork::*;

    part d0039 : Decision {
        :>> id = "ad8d3ec2-0a1b-4c2d-def3-4b5c6d7e8f9a";
        :>> title = "ceremony-skills: sprint-planning/standup/review/closeout/retro skills added; transcript review in sprint-review; engine-triage ceremony routing table added";
        :>> status = DecisionStatus::accepted;
        :>> context = "All delivery workflow phases (refine/standup/implement/review/closeOut/retro) had no dedicated skills. The sprint-review phase had no mechanism to review the session transcript for errors, inefficiencies, or process violations. Engine-triage had no ceremony routing table. Task.estimatedHours had no comment clarifying that [0..1] is SysML multiplicity (not a 0-to-1 hour value bound). Improvement items surfaced mid-sprint had no prescribed capture path.";
        :>> decisionText = "Adopted: (1) sprint-planning skill — orient, Fibonacci sizing, DoR; (2) sprint-standup skill — 3-question orient, blocker check, phase confirmation; (3) sprint-review skill — DoD verify, actualHours record, metrics (velocity/efficiency/accuracy), mandatory transcript review producing typed improvement items; (4) sprint-closeout skill — DoD complete check, actualHours gate, explicit confirmation protocol; (5) sprint-retro skill — triage improvement items, dispatch to skill-update/decision/backlog/retro-note, continuous improvement capture via issues.sysml; (6) engine-triage updated with ceremony routing table + mid-sprint issue capture rule; (7) Task.estimatedHours comment clarifies [0..1] is multiplicity (no value bound); (8) skills-registry updated with all five new skills. Accepted: wweatherholtz 2026-06-16.";
        :>> consequences = "All six delivery ceremonies now have dedicated skills. Sprint-review transcript analysis is mandatory (not optional). Mid-sprint improvement items flow to issues.sysml. Task estimatedHours is explicitly unbounded. Skills-registry grows from 8 to 13 entries. Validate_instances must pass with the updated skills-registry.sysml.";
    }
}
