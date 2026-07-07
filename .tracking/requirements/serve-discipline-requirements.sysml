// =====================================================================
// ProjectServeDisciplineRequirements — Architecture-phase SystemRequirements for the keel-serve
// AI-discipline tooling (D0109), derived from the accepted Needs (serve-discipline.sysml, Business gate
// passed 2026-07-07). Each SR satisfies a Need + carries measurable/verifiable criteria. #DependsOn edges
// encode the dependency TIERS (build highest-tier first, per the human's build-order rule).
// =====================================================================
package ProjectServeDisciplineRequirements {
    private import EngineElement::*;
    private import EngineRequirements::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    // ── Tier 1 (foundation): the model-driven launch/output CONTRACT (launcher + validator read it). ──
    requirement srServeModelDrivenRegistry : SystemRequirement {
        :>> id = "5e400109-5201-4201-9201-516273845201";
        :>> title = "Launchable set + output schemas computed from the declared model";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz";
        :>> statement = "keel serve shall compute, at request time from the declared model (processes/skills/rules), the set of launchable processes and each one's expected output schema — with zero separately-authored launch list or schema (a launch target absent from the model is not offered).";
        :>> rationale = "One source of truth (nServeReuseModel): a duplicated launch/schema list drifts from the model — the exact dual-truth the engine forbids. The launcher (Tier 2) + validator (Tier 2) both read this contract, so it is the foundation.";
        :>> kind = RequirementKind::functional;
    }

    // ── Tier 2 (on the contract): the process-launcher + approve-gate + output-validator. ──
    requirement srServeLauncherDefinedOnly : SystemRequirement {
        :>> id = "5e400109-5202-4202-9202-516273845202";
        :>> title = "Only declared processes are launchable; no freeform path";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz";
        :>> statement = "keel serve shall expose an AI-launch action ONLY for targets in the Tier-1 launchable set; a request naming no declared process shall be rejected (with the valid set returned) — there shall be no code path that spawns the agent on a freeform, un-routed prompt.";
        :>> rationale = "Makes un-routed actions unrepresentable at the boundary (nServeLaunchDefinedOnly) — the action-space constraint (D0109), the compiler-not-linter lever.";
        :>> kind = RequirementKind::functional;
    }
    requirement srServeApproveGate : SystemRequirement {
        :>> id = "5e400109-5203-4203-9203-516273845203";
        :>> title = "Parsed routing shown + explicitly approved before execution";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz";
        :>> statement = "Before spawning the agent, keel serve shall render the parsed input + the routed process and require an explicit operator approve action; execution shall not begin without a recorded approval (an unapproved or edited-away route never runs).";
        :>> rationale = "Approve-before-execute (nServeApproveBeforeExecute) — this is where D0106's conversational parse-first residual becomes STRUCTURALLY enforceable (issue059), which a commit gate cannot do.";
        :>> kind = RequirementKind::functional;
    }
    requirement srServeOutputSchemaValidate : SystemRequirement {
        :>> id = "5e400109-5204-4204-9204-516273845204";
        :>> title = "Agent output schema-validated + repaired before it lands";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz";
        :>> statement = "keel serve shall validate agent output against the routed process's Tier-1 output schema; on violation it shall return the output for repair within a bounded retry count, and shall NEVER record/commit or surface as accepted a schema-invalid result (a persistent failure ends in an explicit error, not silent acceptance).";
        :>> rationale = "Generation-time validation + repair (nServeValidatedOutput) — catches malformed output earlier than keel's commit-time validate/guard, closing the fail-fast gap.";
        :>> kind = RequirementKind::functional;
    }

    // ── Tier 3 (policy on launches): bounded single-purpose context. ──
    requirement srServeScopedContext : SystemRequirement {
        :>> id = "5e400109-5205-4205-9205-516273845205";
        :>> title = "Each launch runs in bounded, single-purpose context";
        :>> createdAt = "2026-07-07"; :>> createdBy = "wweatherholtz";
        :>> statement = "keel serve shall launch each routed process with only its declared inputs as context (no accumulation of unrelated prior-task context across launches), so a task cannot drift on a diffuse long-running goal.";
        :>> rationale = "Bounded single-purpose context (nServeBoundedContext) — the anti-drift structural lever; sits on the launcher (Tier 2).";
        :>> kind = RequirementKind::functional;
    }

    // ── satisfy: SR -> Need ──
    satisfy nServeReuseModel by srServeModelDrivenRegistry;
    satisfy nServeLaunchDefinedOnly by srServeLauncherDefinedOnly;
    satisfy nServeApproveBeforeExecute by srServeApproveGate;
    satisfy nServeValidatedOutput by srServeOutputSchemaValidate;
    satisfy nServeBoundedContext by srServeScopedContext;

    // ── #DependsOn: dependency TIERS (build the depended-on first; no downstream churn). ──
    #DependsOn dependency from srServeLauncherDefinedOnly to srServeModelDrivenRegistry;
    #DependsOn dependency from srServeOutputSchemaValidate to srServeModelDrivenRegistry;
    #DependsOn dependency from srServeApproveGate to srServeLauncherDefinedOnly;
    #DependsOn dependency from srServeScopedContext to srServeLauncherDefinedOnly;
}
