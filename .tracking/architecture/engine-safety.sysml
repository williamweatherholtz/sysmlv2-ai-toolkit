// EngineSafety instances for the ENGINE ITSELF (panel R1: aerospace finding 1, SE finding 4,
// robotics Q3a - the riskClass judgments and K-invariants had no severity-ordered failure-condition
// list above them). STPA step 1 only, calibrated to a work-tracking engine's risk class: losses and
// hazards as authored facts, NO control-structure diagrams or FHA tables (the panel's own
// cargo-cult line). Losses are SEVERITY-ORDERED by declaration (EL1 worst). Each hazard names its
// worst-case loss, its standing controls (guards/mechanisms by name - the computable control-map
// view is chartered under D0195), and the incident history that makes it real. The K-invariants
// that constrain each hazard are linked with #DerivedFrom edges (the invariants were derived from
// exactly these conditions' recorded incidents).
package EngineSafetyInstances {
    private import EngineElement::*;
    private import EngineSafety::*;

    // ---- Losses (severity-ordered: EL1 worst) ----
    part el1 : Loss {
        :>> id = "8dcea740-48fd-4ba1-8362-4eefe4caefa5";
        :>> title = "EL1 - recorded truth corrupted or lost";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "The model's authored facts are destroyed, orphaned, or forked beyond recovery - the one unacceptable outcome for a system whose entire premise is that the text IS the truth. Everything else can be recomputed; this cannot.";
    }
    part el2 : Loss {
        :>> id = "7158f3d3-5045-4026-8e51-2e534a39d615";
        :>> title = "EL2 - a falsified attestation stands as truth";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "A human judgment (acceptance, confirmation, disposition) is fabricated or misattributed and consumed as genuine - the trust the whole supervision model rests on is spent without the human ever judging.";
    }
    part el3 : Loss {
        :>> id = "d257ac1c-92d5-4c6a-90cf-ba70d12deb04";
        :>> title = "EL3 - work falsely reported done or a verdict unearned";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "A consumer (the human, a downstream project, a release) acts on completion or greenness the evidence does not support - the hollow-pass class.";
    }
    part el4 : Loss {
        :>> id = "3ce384ed-3cb7-4def-9897-416aa0d7a32b";
        :>> title = "EL4 - enforcement silently absent";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "Controls are believed active while off, weakened, or unreachable - not a loss in itself but the enabler that lets EL1-EL3 happen unnoticed, so it is managed as one.";
    }

    // ---- Hazards (each: worst-case loss, standing controls, incident lineage) ----
    part ehz1 : Hazard {
        :>> id = "6a00d821-4e4b-4b02-84ad-db1c4bfc55bb";
        :>> title = "EHZ1 - history rewritten on a shared ref";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "Worst case EL1: a rebase/squash/force-push orphans judgedAgainst SHAs, making evidence machine-dependent (green on one clone, not-done on every other). CONTROLS: remote branch protection + disabled squash/rebase (probed live, sprint 401); local hook refusals; CI audit-history over every pushed range (K15/D0179). INCIDENT: issue071.";
    }
    part ehz2 : Hazard {
        :>> id = "690e3c6f-0f2e-41c2-9549-e5f23c00b371";
        :>> title = "EHZ2 - a write lands outside a validated channel";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "Worst case EL1/EL3: tracked-model state transitions unvalidated or is lost to a write race. CONTROLS: the single write API + .keel-write-lock serialisation; pre-write tiers on protected surfaces; the commit gate (validate + 45 guards); recorded overrides (single-use, path-bound). INCIDENTS: issue184/issue185 (four concurrent writes, two lost, all exit 0).";
    }
    part ehz3 : Hazard {
        :>> id = "0bb52128-6740-452c-9f48-4fcba1620ff5";
        :>> title = "EHZ3 - AI records human judgment, or an actor is misattributed";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "Worst case EL2: the agent supplies or inherits a human identity on a judgment record. CONTROLS: refuse_ai_judgment at the write layer; registered-Person checks on sitting/answer/approval; judged_by REQUIRED on judgment endpoints (API 2.0.0); confirmation-authenticity + attestation-authority guards; actor-rebind ledger record. INCIDENTS: issue072, issue197, issue199; OPEN RESIDUAL: issue204 - the delegation channel is name-spoofable by design, direction Decision D0192 awaits the human.";
    }
    part ehz4 : Hazard {
        :>> id = "82e2a723-f069-4bbc-baa7-0a3575189071";
        :>> title = "EHZ4 - completion computed from absent or invalid evidence";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "Worst case EL3: a DoD or gate result whose anchor does not resolve, predates its own evidence, or was stamped past its criterion. CONTROLS: invalidEvidence/unsynchronizedEvidence computed classes; impossible-evidence-date, ceremony, stale-gate-prose, scaffold-placeholder guards; keel reverify at HEAD. INCIDENTS: the D0050/issue092 hollow-pass era; panel R1 automotive finding 1 (pmMeasureDoD stamped past its promotion-review criterion - re-opened this sprint as the honest state).";
    }
    part ehz5 : Hazard {
        :>> id = "0cfdfe2a-1886-4f20-afee-f58689a4fc7c";
        :>> title = "EHZ5 - an enforcement point is silently dead";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "Worst case EL4: a control exists in belief only - unloaded hooks, a sentinel pin, an uninventoried check, a wedged hook exiting silently. CONTROLS: the enforcementPoints inventory (hardening lens); the fire-ledger + enforcement-report (D0180); control-event-coverage guard 45; the watchdog's own ledger line (panel R1 robotics finding 4, fixed this sprint); CI re-derivation trusting no hook (K15). INCIDENTS: issue202 (a gate whose only client could not reach it), issue203, the sr13 dead sentinel.";
    }
    part ehz6 : Hazard {
        :>> id = "ab783361-a562-4a17-9696-438674c0c034";
        :>> title = "EHZ6 - duplicate identity forks the truth";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "Worst case EL1: two items share an id/name/number and views silently merge or split facts. CONTROLS: duplicate-identity guard (hard, whole-model, cross-file); CSPRNG v4 ids (refuses on weak entropy); identity-present/identity-well-formed guards. INCIDENT: the D0174 decision-number collision with a parallel session, caught at authoring time.";
    }
    part ehz7 : Hazard {
        :>> id = "6aee20ad-e31f-4a7d-9568-0ef9d97e7fd2";
        :>> title = "EHZ7 - a control is weakened without a record";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "Worst case EL4: deactivation, exemption, hook/config edit, or rebinding leaves no durable, surfaced trace. CONTROLS: activation.toml as committed data; claude-surface-drift guard; hooksPath warning in orient; single-use path-bound overrides whose consumption records an obligation; the actor-rebind ledger line. INCIDENT: issue205 (rebinds were invisible until sprint 407).";
    }
    part ehz8 : Hazard {
        :>> id = "38e0f96c-3094-4505-833a-1602e46fd4fc";
        :>> title = "EHZ8 - a stale computed layer is served as current truth";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "Worst case EL3: a view computes over authored facts that reality outgrew, and the reader cannot tell. CONTROLS: suspect-on-drift + reverify for evidence; doc-sync discipline for prose. OPEN INSTANCE (panel R1, all five panelists): trace-need serves the June-era Component allocations - including a component describing a tool retired at D0074 as the single source of project state - as live truth; tracked this sprint, direction Decision D0194 queued.";
    }
    part ehz9 : Hazard {
        :>> id = "c8c74613-78c4-4abf-b394-79e05a581541";
        :>> title = "EHZ9 - an ungated artifact ships outward";
        :>> createdAt = "2026-08-22"; :>> createdBy = "claudeFable5";
        :>> statement = "Worst case EL3, outward-facing: a publish (release binary, shared artifact) leaves the boundary unverified. CONTROLS: the release gate job the publish requires (proven both ways on the live pipeline, sprint 409); the pre-push behind-check; keel land's gate-on-merged-tree. INCIDENT: issue211 (the pipeline published with zero gate steps until sprint 409).";
    }

    // ---- The invariants that constrain these conditions (derived from their incidents) ----
    #DerivedFrom dependency from srK15TreeDerivable to ehz1;
    #DerivedFrom dependency from srK01ValidatedChannel to ehz2;
    #DerivedFrom dependency from srK06HumanChannel to ehz3;
    #DerivedFrom dependency from srK05ComputedCompletion to ehz4;
    #DerivedFrom dependency from srK12VerdictTraceability to ehz4;
    #DerivedFrom dependency from srK02FailLoud to ehz5;
    #DerivedFrom dependency from srK03EnforcementParity to ehz5;
    #DerivedFrom dependency from srK14MeasuredEffect to ehz5;
    #DerivedFrom dependency from srDcUniqueIdentityGuard to ehz6;
    #DerivedFrom dependency from srK07RecordedWeakening to ehz7;
    #DerivedFrom dependency from srK10Reprocess to ehz8;
}
