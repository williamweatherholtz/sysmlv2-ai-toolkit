// Per-actor issues records for claudeOpus5 (issue210/srDcPerActorWriteTargets): new records
// route here so concurrent contributors never conflict at a shared append anchor.
package IssuesClaudeOpus5 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part issue292 : Issue {
        :>> id = "cea7937f-97e0-4482-8a06-4cbfddb8aa79";
        :>> title = "the shipped reference decision copy is parsed as instance data, importing 235 foreign attestations and 236 unresolvable evidence SHAs";
        :>> createdAt = "2026-08-27";
        :>> createdBy = "claudeOpus5";
        :>> description = "keel init remaps the engine's 236 decisions to .engine/reference/decisions/ and Sprint 104's record states they are NOT scanned as instance - the rust views scan .engine/decisions. That was never true: collect_sysml (lib.rs:75) has no exclusions, so the reference copy has always been parsed and registered into the downstream project's model. Sprint 104 implemented the PATH remap and not the scanning boundary it declared. Measured in a fresh init fixture: the downstream model contains 235 TestResults judged by wweatherholtz - keel's human, not a stakeholder of that project - and 236 judgedAgainst SHAs that do not resolve in that repository (sampled e146621: 'Not a valid object name'). So every downstream project's verification surface is inflated with another project's human attestations, and its evidence chain contains 236 references that D0129 requires to resolve and which cannot. D0237 fixed the symptom this surfaced (the package collision on the first recorded decision) and deliberately did not fix this. Note the constraint on any fix: .engine/rules/rules.sysml carries 10 real #JustifiedBy edges to dNNNN targets, so the reference copy cannot simply be dropped from the walk - the edges would dangle. Aggravating, and the reason this shipped: both gates covering init (sprint 104's live check and sprint 115's init smoke test) assert init-then-validate and never record a fact, which is the one state in which every defect in this class is invisible.";
        :>> discoveredInField = true;
        :>> severity = Severity::High;
    }
    #Resolves dependency from scopeReferenceCopyOutOfTheInstanceModel to issue292;
}
