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

    // ─────────────────────────────────────────────────────────────────────────────────────────────
    // CORRECTED REQUIREMENTS (2026-08-15) — the ACT disposition on issue121-125, the five findings of
    // the independent Core-3 critique (sprint 305). Authored by claudeOpus5, who is NOT the owner of
    // the originals, so each arrives as a SUPERSEDING item rather than an in-place edit: D0108 permits
    // a non-owner to ADD items and typed edges or to SUPERSEDE, never to overwrite another actor's
    // fields, and the `ownership` guard enforces it. The originals stay exactly as authored — they are
    // the record of what was asked and what the critique found, and rewriting them would erase it.
    // ─────────────────────────────────────────────────────────────────────────────────────────────

    requirement srServeLaunchContract : SystemRequirement {
        :>> id = "50932510-2f22-4d11-bf7b-490a3898c869";
        :>> title = "Launchable set AND output schema resolved from declared sources; no schema, no launch";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> statement = "keel serve shall compute, at request time, the set of launchable targets from the declared model (processes/skills/rules), and shall resolve each target's expected output schema from a DECLARED CONTRACT carried alongside those definitions. A target whose output schema is not declared shall NOT be offered for launch. There shall be no separately-authored launch list and no schema authored anywhere the model does not reach.";
        :>> rationale = "SUPERSEDES srServeModelDrivenRegistry, acting on issue121 (C1, High). The original required the output schema to be computed 'from the declared model' when no schema type can hold one: `action def Process` carries purpose and cadence, `ProcessStep` carries triggerCondition/actionText/producedArtifact/owner, and `producedArtifact` is a free-text sentence about the artifact CHANGE, not a validatable schema. So half of a Tier-1 contract was uncomputable and two Tier-2 requirements depended on it. THE FIX NAMES A MECHANISM THAT EXISTS: a declared contract file beside the process definitions, which is the pattern this engine already uses for critique-policy.toml and attestation-policy.toml — read at request time, downstream-overridable, and part of the declared model in this engine's idiom without requiring a frozen-core schema change. The added clause 'no schema, no launch' is what makes srServeOutputValidation always satisfiable: it removes the case where a target is launchable but its output cannot be validated, which is the hole the original left open.";
        :>> kind = RequirementKind::functional;
    }

    requirement srServeSingleSpawnPoint : SystemRequirement {
        :>> id = "53eac612-7e76-4148-9e92-f012cd75e6de";
        :>> title = "Exactly one spawn point, taking a validated target type";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> statement = "Exactly one function in the implementation shall spawn the agent, and it shall accept a validated launch-target VALUE rather than a string, so an un-routed or freeform launch is not expressible at the type level. A request naming no declared target shall be rejected with the valid set returned. An automated check shall assert the single-spawn-point property, so a second spawn path fails the gate rather than passing unnoticed.";
        :>> rationale = "SUPERSEDES srServeLauncherDefinedOnly, acting on issue122 (C2, Medium). The original said 'there shall be no code path that spawns the agent on a freeform, un-routed prompt' — a UNIVERSAL NEGATIVE over the whole codebase that no test can demonstrate. A test can show one entry point rejects an undeclared target; nothing can show no other path exists, and establishing it needs exhaustive review, which this engine classes as manual vigilance rather than a control (D0047). Restated as a STRUCTURAL property a check can enforce: one spawn point, taking a type that cannot hold an un-routed value. The intent of D0109 is unchanged — invalid actions unrepresentable at the boundary — but it is now falsifiable, and a new bypass path breaks a check instead of quietly satisfying the old wording.";
        :>> kind = RequirementKind::functional;
    }

    requirement srServeApproveGateHuman : SystemRequirement {
        :>> id = "9065edb3-2fb7-45af-ae5c-33484836dda1";
        :>> title = "Approval is a HUMAN attestation, checked by kind before role";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> statement = "Before spawning the agent, keel serve shall render the parsed input and the routed target, and shall require an approval attested by an actor whose KIND is human under the declared attestation policy — kind checked BEFORE role, so an AI actor holding any role is refused. The approval shall be recorded as an authored fact naming the approving actor, the time, and the exact routed target approved; execution shall not begin without it, and an approved route that is subsequently edited shall require a fresh approval.";
        :>> rationale = "SUPERSEDES srServeApproveGate, acting on issue123 (C3, High). The original required an 'explicit operator approve action' and never constrained the operator to a HUMAN — 'operator' is undefined across the Needs and requirements, and NeedSource::operator is a source category, not an actor kind. Nothing stopped an AI supplying the approval, which defeats the parent Need's own words: 'the agent never silently acts on its own interpretation.' An agent approving its own routing IS acting on its own interpretation, with a recorded approval to evidence it. The correction binds to machinery that now exists: D0146 added Actor.role, .engine/contracts/attestation-policy.toml declares per-class kinds, and the attestation-authority guard already checks kind BEFORE role so a right-role AI still fails a human-only class. Naming the recorded artefact also closes the original's minor gap, where 'a recorded approval' named nothing an auditor could read back.";
        :>> kind = RequirementKind::functional;
    }

    requirement srServeOutputValidation : SystemRequirement {
        :>> id = "824bfb9f-cf8b-489c-aeb7-2e2cc555c8ba";
        :>> title = "Output validated and repaired, with side effects accounted for on failure";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> statement = "keel serve shall validate agent output against the routed target's declared output schema and, on violation, return it for repair within a bounded retry count, never recording, committing, or surfacing as accepted a schema-invalid result. A launch that may take side effects shall execute with those effects ISOLATED until its output validates; where isolation is not possible, any side effects already taken by a failed attempt shall be RECORDED AS FACTS. A persistent failure shall end in an explicit error that names what was left behind, never silent acceptance and never silent discard.";
        :>> rationale = "SUPERSEDES srServeOutputSchemaValidate, acting on issue124 (C4, Medium). The original governed only the RESULT and was silent on side effects the failing attempt already caused: an agent that wrote files, appended records, or pushed during attempt 1 has already changed the world, and rejecting its output undoes none of it — after the retries are exhausted, the required 'explicit error' left those partial effects in place with nothing recorded to say they happened. For an engine whose premise is that the model must be truthful about state, silently orphaned partial writes are the worst residue a retry loop can leave. Isolation first, recording as the fallback, and an error that names the residue: all three keep the model honest about what actually happened.";
        :>> kind = RequirementKind::functional;
    }

    requirement srServeBoundedReachableContext : SystemRequirement {
        :>> id = "2b3d4092-f490-4ba5-97f5-ee42f4ed90e9";
        :>> title = "Both passed-in context and reachable surface are bounded per launch";
        :>> createdAt = "2026-08-15"; :>> createdBy = "claudeOpus5";
        :>> statement = "keel serve shall launch each routed target with only its declared inputs as context, accumulating nothing from prior launches, AND shall bound the surface the launched agent can REACH to that target's declared scope. Where the runtime cannot enforce a reachability bound, the residual shall be stated explicitly in the launch record rather than left implied, so the anti-drift property is never claimed more broadly than it is enforced.";
        :>> rationale = "SUPERSEDES srServeScopedContext, acting on issue125 (C5, Medium). The original bounded only what was PASSED IN, while a launched agent's dominant context channel is what it can READ: with file and shell access it reconstitutes arbitrary repository context however narrow its declared inputs were, so the stated goal — 'a task cannot drift on a diffuse long-running goal' — was not delivered by input scoping alone. A requirement that claims a property it cannot enforce is more dangerous than one that claims less, because the gap is invisible to anyone reading it. The correction bounds reachability too, and where that is not enforceable it requires the residual to be STATED rather than assumed away.";
        :>> kind = RequirementKind::functional;
    }

    // supersede: corrected -> original. The originals remain readable and remain the target of the
    // sprint 305 critiques, which is what keeps the finding and its correction legible together.
    #Supersede dependency from srServeLaunchContract to srServeModelDrivenRegistry;
    #Supersede dependency from srServeSingleSpawnPoint to srServeLauncherDefinedOnly;
    #Supersede dependency from srServeApproveGateHuman to srServeApproveGate;
    #Supersede dependency from srServeOutputValidation to srServeOutputSchemaValidate;
    #Supersede dependency from srServeBoundedReachableContext to srServeScopedContext;

    // satisfy: the corrected requirements carry the same Needs forward.
    satisfy nServeReuseModel by srServeLaunchContract;
    satisfy nServeLaunchDefinedOnly by srServeSingleSpawnPoint;
    satisfy nServeApproveBeforeExecute by srServeApproveGateHuman;
    satisfy nServeValidatedOutput by srServeOutputValidation;
    satisfy nServeBoundedContext by srServeBoundedReachableContext;

    // dependency TIERS, carried forward: the launch contract is still the foundation.
    #DependsOn dependency from srServeSingleSpawnPoint to srServeLaunchContract;
    #DependsOn dependency from srServeOutputValidation to srServeLaunchContract;
    #DependsOn dependency from srServeApproveGateHuman to srServeSingleSpawnPoint;
    #DependsOn dependency from srServeBoundedReachableContext to srServeSingleSpawnPoint;
}
