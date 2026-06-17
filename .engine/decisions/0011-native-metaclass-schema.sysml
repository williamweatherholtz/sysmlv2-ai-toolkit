package Decision0011 {
    private import EngineWork::*;

    part d0011 : Decision {
        :>> id = "75f09a66-6ab9-4247-a341-b822f9dbf623";
        :>> title = "Schema types use native SysML v2 metaclasses";
        :>> status = DecisionStatus::accepted;
        :>> context = "The original schema modeled requirements/use-cases/tests as part defs :> Element for a uniform identity base (recorded in requirements.sysml comments). The schemaAudit asked what native alignment should mean; %show value-elision (D0006-limit) and the Element-base loss were the costs.";
        :>> decision = "Types with a strong native metaclass go native via per-metaclass abstract tracked bases (Need + Requirement -> requirement def, UseCase -> use case def, Test -> verification def); engine concepts without a native (Brief, Persona, Decision, Issue, TestResult, Component, actors, risk) stay part def :> Element. Supersedes the part-based-everything choice.";
        :>> rationale = "Native elementId regenerates every parse, so the authored id remains identity; gains tool/visualizer interop and native satisfy/verification semantics. The schemaAudit weighed %show value-elision (D0006-limit) and the Element-base loss as costs. Accepted by user 2026-06-10 (commit 72bb02b).";
        :>> consequences = "Tool/visualizer interop and native satisfy/verification semantics; cost: tracking attributes duplicated per metaclass base (candidate fix: attributed metadata def, CR-12).";
    }
}
