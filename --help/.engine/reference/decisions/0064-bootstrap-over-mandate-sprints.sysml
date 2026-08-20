// =====================================================================
// D0064 — bootstrap-over-mandate-sprints: the tracking engine exists, so the
// §3e bootstrap sprint-exemption is retired. Substantive work goes through a
// sprint; triage-first is mandatory. (Sprint 29.)
// Authored in the post-migrate Decision pattern: decision + context + rationale
// (+ decisionText kept until attrModelContract drops it).
// Accepted 2026-06-17 by wweatherholtz ("there's no bootstrap work going forward
// - the tracking tool is created... hard transition to the sprint as indicated,
// but also update claude.md to prevent process subversion").
// =====================================================================
package Decision0064 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // D0070: this Decision changed the Delivery process (mandated sprints; prospective, D0062 default). Grandfathered for resolution (pre-guard).
    #ProspectiveChange part d0064 : Decision {
        :>> id = "63940516-2c7d-4e8f-b064-516273849564";
        :>> title = "D0064: bootstrap is over — retire the §3e sprint-exemption; substantive work goes through a sprint; mandatory triage-first";
        :>> createdAt = "2026-06-17";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "CLAUDE.md §3e ('building the engine's own runtime/tooling is exempt from the full workflow') and the 'until the runtime exists' framings assumed the tracking engine did not yet exist. It now does: the Rust authority (D0048) — write API (append-result/add-task/append-gate-result), the validators, and computed orient/whats-next/suspect — is in daily use. Recent practice already sprinted engine work (sprints 25-28). issue020: the attribution migration was implemented and committed with NO sprint, slipping through pre-commit precisely because §3e exempted it. So the exemption's premise is false and the exempt-vs-sprint-everything contradiction actively caused process subversion.";
        :>> decision = "Bootstrap as a sprint-exemption regime is OVER. Retire §3e's 'engine tooling is exempt from the full workflow' and the 'until the runtime exists' framings. ALL substantive work — CHANGE (schema/process), delivery, and significant engine work — goes through a sprint (refine->standup->implement->review->closeOut->retro). Only TRIVIAL one-off edits (a typo, a single rename, one doc line) are sprint-exempt; the test is 'could a reviewer want gates on this?'. Triage/classification is a MANDATORY first move on every request: route each part to its process and FLAG anything that does not cleanly map. Keep the durable §4 rules unchanged (validate-green-before-commit, CR: commits for schema/process, doc-sync rides every change, D0047 corrections->guards, main-only). The meta-process freeze (CR-workflow self-modification) is OUT OF SCOPE and stays frozen.";
        :>> rationale = "The §3e exemption existed only because the engine could not yet track its own construction; now it can, so keeping the exemption just invites the subversion issue020 demonstrated. Sprinting substantive work gives every change the gated, reviewable, ceremony-wrapped trail the engine exists to provide. ALTERNATIVES: (A) keep §3e (status quo) — rejected: its premise (no runtime) is now false and it directly caused issue020. (B) retire §3e AND unfreeze the meta-process — rejected: the meta-process freeze is a separate concern; descope it, leave it frozen. (C CHOSEN) retire the sprint-exemption + 'until runtime exists' framings + make triage-first mandatory, keeping the durable §4 rules and the meta-freeze. CRITERIA: premise-validity x prevents-recurrence (issue020) x scope-discipline.";
        :>> consequences = "Resolves bootstrapExitEval. CLAUDE.md edited: header BOOTSTRAP status updated; §3 triage-first made mandatory + flag-non-mapping; §3e block retired; §4 'until runtime exists' framings updated; durable rules kept. ENFORCEMENT is owed and queued as sprint30 (issue020): (1) a no-sprint guard (flag substantive .tracking/schema commits not covered by a sprint, D0047); (2) the per-interaction triage skill (engine-triage upgrade); (3) a UserPromptSubmit hook that fires triage every turn. Until the guard exists, the no-sprint rule binds by the now-mandatory triage discipline. doc-sync ran on this change.";
    }

    verification d0064Accept : Test { :>> id = "e6c1b390-bddb-4c13-b3b5-fc7ed0201f1e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0064 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0064AcceptR1 : TestResult { :>> id = "492b6881-1a7d-4613-9cb9-ca1bfacab801"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
