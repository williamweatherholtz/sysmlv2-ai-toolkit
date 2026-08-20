package Decision0039 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // D0070: this Decision changed the Delivery ceremony (prospective; D0062 default). Grandfathered for resolution (pre-guard).
    #ProspectiveChange part d0039 : Decision {
        :>> id = "ad8d3ec2-0a1b-4c2d-def3-4b5c6d7e8f9a";
        :>> title = "ceremony-skills: sprint-planning/standup/review/closeout/retro skills added; transcript review in sprint-review; engine-triage ceremony routing table added";
        :>> status = DecisionStatus::accepted;
        :>> context = "All delivery workflow phases (refine/standup/implement/review/closeOut/retro) had no dedicated skills. The sprint-review phase had no mechanism to review the session transcript for errors, inefficiencies, or process violations. Engine-triage had no ceremony routing table. Task.estimatedHours had no comment clarifying that [0..1] is SysML multiplicity (not a 0-to-1 hour value bound). Improvement items surfaced mid-sprint had no prescribed capture path.";
        :>> decision = "Add dedicated skills for all six delivery ceremonies (sprint-planning, sprint-standup, sprint-review, sprint-closeout, sprint-retro), with mandatory transcript review in sprint-review; add a ceremony routing table + mid-sprint issue-capture rule to engine-triage; clarify Task.estimatedHours [0..1] is multiplicity; register all five new skills.";
        :>> rationale = "All delivery workflow phases (refine/standup/implement/review/closeOut/retro) had no dedicated skills, the sprint-review phase had no mechanism to review the session transcript for errors/inefficiencies/process violations, engine-triage had no ceremony routing table, Task.estimatedHours lacked a comment clarifying [0..1] is SysML multiplicity (not an hour-value bound), and mid-sprint improvement items had no prescribed capture path. The skills cover: planning (orient, Fibonacci sizing, DoR); standup (3-question orient, blocker check, phase confirmation); review (DoD verify, actualHours record, metrics, mandatory transcript review producing typed items); closeout (DoD complete check, actualHours gate, explicit confirmation protocol); retro (triage improvement items, dispatch to skill-update/decision/backlog/retro-note via issues.sysml). Skills-registry grows from 8 to 13 entries.";
        :>> consequences = "All six delivery ceremonies now have dedicated skills. Sprint-review transcript analysis is mandatory (not optional). Mid-sprint improvement items flow to issues.sysml. Task estimatedHours is explicitly unbounded. Skills-registry grows from 8 to 13 entries. Validate_instances must pass with the updated skills-registry.sysml.";
    }

    verification d0039Accept : Test { :>> id = "6b5bf307-e0ce-4e7f-a706-10cf2597146a"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0039 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0039AcceptR1 : TestResult { :>> id = "a57864c7-fcae-48a1-9a88-06670665728b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
