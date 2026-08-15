// =====================================================================
// ProjectPanelCritiques — the three-critic panel the human commissioned on 2026-08-14, recorded as
// engine-native critique facts (D0080) instead of left in conversation history (D0018).
//
// PROVENANCE OF THIS FILE. Every finding below is reconstructed from COMMITTED records, not from
// memory: commit ec7d548 (the panel's verdict, with issue098 and issue099 stated in full) and
// D0142's `context`, which enumerates the seven non-marker findings (a),(b),(d),(f),(g) and whose
// siblings (c) and (e) became D0143 and issue106. Where a record does not state something — a
// per-finding severity the panel never assigned, or unanimity it never claimed — this file says so
// rather than supplying it.
//
// WHAT UNANIMITY IS ACTUALLY DOCUMENTED, and nothing beyond it:
//   - The CENTRAL question: all three critics reached the same verdict that D0139 clause (E) must
//     not be executed, by three independent routes (ec7d548, verbatim).
//   - The RANKING: all three rated the seven non-marker findings more consequential than the marker
//     count the human originally asked about (D0142 context, verbatim).
//   - Finding (c) specifically: "all three critics agreeing and the correctness adjudicator rating
//     it High confidence" (backlog.sysml).
// Per-finding unanimity is NOT documented for (a), (b), (d), (f) or (g). Those carry the raising
// critic where the record names one and no unanimity claim where it does not.
//
// THE INDEPENDENCE QUESTION, resolved explicitly rather than by convenience, as the item required.
// `critic-independence` requires an element carrying a Critical-severity finding to have a critique
// that is not merely aiModel. The panel CANNOT supply that: three subagent contexts of the same
// model as the author are a distinct framing, not a distinct judgement substrate, and recording
// them as independent would be the kind of self-assurance D0098 exists to forbid. So every panel
// critique below is `critiquedBy = aiModel` — it PASSES nothing by itself. The one critique that is
// genuinely independent is the kernel's: the `objective`/`subject` discriminator was run against the
// OMG pilot validators, whose verdict does not come from Claude, and it is recorded separately as
// `critiquedBy = tool`. The identity in `judgedBy` is the actor who ran it, per the schema's own
// rule that critic KIND and critic IDENTITY are different fields.
// =====================================================================
package ProjectPanelCritiques {
    private import EngineElement::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // ── The central finding: D0139(E) must not be executed ────────────────────

    verification panelVerifyShapeCritique : Test {
        :>> id = "c0a10001-0814-4a01-9a01-516273840001";
        :>> title = "Critique d0139 — correctness lens: the migration target of clause (E) is not a relationship";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::correctness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::Critical;
        :>> procedureText = "RAISED BY the technical-correctness critic; the antagonist and protagonist reached the same verdict on this question by their own routes, which is the panel's one documented unanimity. ATTACK: D0139(E) names `verification def V { subject s; objective r; }` as the base construct that #Verify should migrate onto. FINDING: `objective r;` DECLARES a nested member and carries no reference, so that form is not a relationship at all. Migrating onto it would leave every one of the 456 #Verify edges silently non-existent while both toolchains reported clean — worse than the syntax error issue097 caught, because this form is VALID and nothing would fail. SUPPORTING MEASUREMENT: the reference-bearing forms (`objective :>> r` / `:> r`) require a requirement-typed target, and 309 of the 456 edges point at `part Decision`, which cannot be a requirement objective. NOT SURVIVED — the finding stands. CONSEQUENCE: D0139's base-first RULE is vindicated and untouched; its two named migrations are refuted by exactly the standard the rule demands, and #Verify/#DerivedFrom become documented last resorts rather than unjustified dialect. Recorded as issue098 (Critical) and resolved by D0140, which supersedes clauses E and G only.";
    }
    part panelVerifyShapeCritiqueR1 : TestResult { :>> id = "c0a10002-0814-4a02-9a02-516273840002"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "ec7d548"; :>> judgedAt = "2026-08-14"; :>> judgedBy = "panelCorrectness"; }
    #Verify dependency from panelVerifyShapeCritique to d0139;

    verification kernelDiscriminatorCritique : Test {
        :>> id = "c0a10003-0814-4a03-9a03-516273840003";
        :>> title = "Critique d0139 — correctness lens, adjudicated by the OMG kernel (the panel's only non-Claude verdict)";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::correctness;
        :>> critiquedBy = CriticKind::tool;
        :>> severity = Severity::Critical;
        :>> procedureText = "THE INDEPENDENCE THE PANEL COULD NOT SUPPLY. The correctness critic's finding was checked against the OMG SysML v2 pilot kernel rather than accepted on an AI's say-so, using an undeclared-name discriminator: give a construct a name declared nowhere and see whether the kernel objects. `verification def V { subject s; objective zzz; }` with nothing named zzz PASSES, while the controls correctly FAIL — `part p : NoSuchTypeAnywhere` and `satisfy r by zzz3` both produce 'Couldn't resolve reference to Element'. So the kernel DOES resolve references, and `objective`/`subject` simply do not take one: they declare. VERDICT: the critic's finding is confirmed by an oracle that is not Claude. This is the only critique in the panel whose judgement does not originate from the same model as the author, and it is the reason the panel's central verdict is more than three restatements of one framing. `judgedBy` is the actor who ran the kernel, per the schema's separation of critic KIND from critic IDENTITY.";
    }
    part kernelDiscriminatorCritiqueR1 : TestResult { :>> id = "c0a10004-0814-4a04-9a04-516273840004"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "ec7d548"; :>> judgedAt = "2026-08-14"; :>> judgedBy = "claudeOpus5"; }
    #Verify dependency from kernelDiscriminatorCritique to d0139;

    verification panelCensusCritique : Test {
        :>> id = "c0a10005-0814-4a05-9a05-516273840005";
        :>> title = "Critique d0139 — correctness lens: the recorded marker control totals are inflated 35%";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::correctness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::High;
        :>> procedureText = "ATTACK: check the control totals an accepted Decision migrates against. FINDING: D0139 records 613/91/91 for #Verify/#DerivedFrom/#DependsOn; the real EDGE counts are 456/37/42. The inflation came from grepping the marker NAME, which also matches 129 prose mentions inside `description` and `procedureText` strings — a corpus containing prose about itself inflates its own census, and the error GROWS as more is written about the migration. Aggravating: guards.rs:936 and relationships.sysml:66 already carried the correct 456, and the author neither consulted them nor noticed the disagreement. NOT SURVIVED. Recorded as issue099 (High) and resolved by `keel marker-census`, which computes the figure instead of grepping it.";
    }
    part panelCensusCritiqueR1 : TestResult { :>> id = "c0a10006-0814-4a06-9a06-516273840006"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "ec7d548"; :>> judgedAt = "2026-08-14"; :>> judgedBy = "panelCorrectness"; }
    #Verify dependency from panelCensusCritique to d0139;

    // ── The seven non-marker findings, which all three rated more consequential ──
    // than the marker count the human originally asked about (D0142 context).

    verification panelFindingACritique : Test {
        :>> id = "c0a10007-0814-4a07-9a07-516273840007";
        :>> title = "Critique of schema/core — consistency lens, finding (a): two live Release types, and the Deploy workflow imported the wrong one";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::consistency;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::High;
        :>> procedureText = "FINDING (a), re-measured before any change as dcSchemaLeannessSweep required. TWO live `Release` types existed: `part def Release` in work.sysml and `occurrence def Release` in baseline.sysml. `.engine/workflows/deploy.sysml` imported the FIRST and threaded it through four action signatures, while all 7 authored releases bind the SECOND. The Deploy workflow therefore declared an artifact type nobody produced — a dual-truth defect that both toolchains reported clean because each type is individually well-formed. NOT SURVIVED. Resolved by D0142: the duplicate removed and Deploy retargeted. NOTE: the raising critic is not named in the committed record, and no per-finding unanimity was claimed — only that all three rated this class above the marker count.";
    }
    part panelFindingACritiqueR1 : TestResult { :>> id = "c0a10008-0814-4a08-9a08-516273840008"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "ec7d548"; :>> judgedAt = "2026-08-14"; :>> judgedBy = "panelAntagonist"; }
    #Verify dependency from panelFindingACritique to d0142;

    verification panelFindingBCritique : Test {
        :>> id = "c0a10009-0814-4a09-9a09-516273840009";
        :>> title = "Critique of schema/core — necessity lens, finding (b): nineteen types with zero declarations, against the repo's own recorded policy";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::necessity;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::Medium;
        :>> procedureText = "FINDING (b). NINETEEN types had zero DECLARATIONS anywhere in the model: Task, Epic, Assumption, ComponentRequirement, TestPlan, Risk, Mitigation, DesignElement, FunctionAllocation, InterfacePort, Agent, Role, WorkflowDefinition, MaintainerVP, OutstandingView, SuspectView, TraceView, RequirementCoverage, SuspicionSweep. THE FORCE OF THE FINDING is not that dead schema is untidy — it is that this contradicts the repo's OWN recorded policy at rules.sysml:63-66, where `OrderingRule` was removed as dead schema under the explicit rule 're-add it THEN, with an instance'. A rule the project wrote for itself and then stopped applying is worse than no rule, because it is cited as if enforced. NOT SURVIVED. Resolved by D0142's sweep. RESIDUE FOUND LATER, in sprint 281: the sweep itself left a hollow `risk.sysml`, an orphan `RiskLevel`, and two docs naming deleted types — recorded as issue108, which is this finding recurring one layer down.";
    }
    part panelFindingBCritiqueR1 : TestResult { :>> id = "c0a1000a-0814-4a0a-9a0a-51627384000a"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "ec7d548"; :>> judgedAt = "2026-08-14"; :>> judgedBy = "panelAntagonist"; }
    #Verify dependency from panelFindingBCritique to d0142;

    verification panelFindingCCritique : Test {
        :>> id = "c0a1000b-0814-4a0b-9a0b-51627384000b";
        :>> title = "Critique of schema/core — correctness lens, finding (c): a Process is behaviour and was modelled as a part";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::correctness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::High;
        :>> procedureText = "FINDING (c) — the one finding with documented per-finding agreement: all three critics agreed and the correctness adjudicator rated it High confidence (backlog.sysml, verbatim). ATTACK: `Process` and `ProcessStep` were `part def`s carrying an `order : Integer` attribute. A process is BEHAVIOUR, and SysML v2 has a native construct for it; modelling it as structure means a standard viewer renders the engine's own workflow definitions as parts, and sequence lives in an authored integer rather than in native succession — an authored field where the language has a relationship, which is the exact defect D0139's base-first rule exists to prevent. NOT SURVIVED. Resolved by D0143: both retyped to `action def :> TrackedAction`, `order` deleted, and 71 native `first..then` successions authored in its place. This is the finding all three critics ranked above the marker count the human originally asked about.";
    }
    part panelFindingCCritiqueR1 : TestResult { :>> id = "c0a1000c-0814-4a0c-9a0c-51627384000c"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "ec7d548"; :>> judgedAt = "2026-08-14"; :>> judgedBy = "panelCorrectness"; }
    #Verify dependency from panelFindingCCritique to d0143;

    verification panelFindingDCritique : Test {
        :>> id = "c0a1000d-0814-4a0d-9a0d-51627384000d";
        :>> title = "Critique of schema/core — consistency lens, finding (d): ports in schema/core contradict accepted D0100";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::consistency;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::Medium;
        :>> procedureText = "FINDING (d). `port def InterfacePort` and `port ifaces : InterfacePort[*]` sat in schema/core while ACCEPTED D0100 states plainly that no ports or interfaces enter schema/core. Compounding it, the `#View item def ICD` comment advertised a computation over ports and connections while the repo contains ZERO `connect` statements — a view documented as computed from inputs that do not exist. NOT SURVIVED for the ports. SURVIVED for the ICD TYPE itself, which is consumed by the Architecture workflow and stays; only the misleading comment was wrong. Resolved by D0142. NOTE the correction this finding needed: the sweep initially deleted `#View item def ICD` outright and the kernel caught that `architecture.sysml:19 out icd : ICD` still referenced it — the type was restored and D0142's text corrected. An accepted Decision citing a removal that did not happen would have been a worse defect than the one being fixed.";
    }
    part panelFindingDCritiqueR1 : TestResult { :>> id = "c0a1000e-0814-4a0e-9a0e-51627384000e"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "ec7d548"; :>> judgedAt = "2026-08-14"; :>> judgedBy = "panelCorrectness"; }
    #Verify dependency from panelFindingDCritique to d0142;

    verification panelFindingECritique : Test {
        :>> id = "c0a1000f-0814-4a0f-9a0f-51627384000f";
        :>> title = "Critique of schema/core — completeness lens, finding (e): provenance is unnavigable, carried in Strings beside a live Actor type";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::completeness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::Medium;
        :>> procedureText = "FINDING (e). `createdBy`, `owner` and `judgedBy` are Strings while `part def Actor` exists and is instantiated, so no viewer can select an actor and see what they authored or attested — and that is the one trace that survives a provenance audit, which matters directly under D0129 where provenance is never defaulted. THE FINDING IS CORRECT AND ITS PROPOSED FIX IS NOT. `ref createdBy : Actor` is structurally unavailable: `.engine` carries 137 of the 6132 provenance values, the registry is `ProjectActors` — per-project INSTANCE data — and `.engine` ships downstream via `keel init` where this project's actors do not exist, so those references would arrive unresolved. The panel assessed the type surface and not the packaging; that is the panel's one recorded MISS, and it is recorded here rather than only in the item that redirected the work. Resolved by delivering the navigability through computation (`keel actor-trace`) instead, with the residual stated: navigability now lives in keel rather than in the raw model. See issue106.";
    }
    part panelFindingECritiqueR1 : TestResult { :>> id = "c0a10010-0814-4a10-9a10-516273840010"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "ec7d548"; :>> judgedAt = "2026-08-14"; :>> judgedBy = "panelAntagonist"; }
    #Verify dependency from panelFindingECritique to d0142;

    verification panelFindingFCritique : Test {
        :>> id = "c0a10011-0814-4a11-9a11-516273840011";
        :>> title = "Critique of schema/core — correctness lens, finding (f): stale claims inside FROZEN schema";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::correctness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::Medium;
        :>> procedureText = "FINDING (f). Three false claims sitting inside frozen schema/core, which is the surface a reader trusts most: `query.py` cited 8 times though retired at D0074; rules.sysml:5 claiming '14 guards' against the real 28; and an `EdgeKind` comment labelling `verifyEdge` a typed native edge when verify is a metadata marker — the last one materially misleading, since it asserts the very thing findings (E)/(G) were refuted for assuming. Frozen means changes need sign-off; it does not mean the contents stop needing to be true. NOT SURVIVED. Corrected by D0142, and INCOMPLETELY: sprint 279 later found four surviving `query.py` citations outside schema/core, because the sweep fixed the consumer in front of it. That miss is what produced D0144.";
    }
    part panelFindingFCritiqueR1 : TestResult { :>> id = "c0a10012-0814-4a12-9a12-516273840012"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "ec7d548"; :>> judgedAt = "2026-08-14"; :>> judgedBy = "panelCorrectness"; }
    #Verify dependency from panelFindingFCritique to d0142;

    verification panelFindingGCritique : Test {
        :>> id = "c0a10013-0814-4a13-9a13-516273840013";
        :>> title = "Critique of schema/core — completeness lens, finding (g): a 17th marker hidden outside the vocabulary's declared home";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::completeness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::Low;
        :>> procedureText = "FINDING (g). The 17th marker, `metadata def View`, lived in workflows/_meta.sysml, whose own header states it holds workflow-local artifact types only. An audit performed from relationships.sysml — the vocabulary's declared home, and where anyone would look — therefore undercounted the custom vocabulary by one marker and 73 uses. Small in size and exactly the class that matters: the census the migration was scoped against was computed from a place that was not the whole truth, which is the same root as finding issue099 reached by a different route. NOT SURVIVED. Resolved by D0142, and by `keel marker-census` computing the figure from the model rather than from a chosen file.";
    }
    part panelFindingGCritiqueR1 : TestResult { :>> id = "c0a10014-0814-4a14-9a14-516273840014"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "ec7d548"; :>> judgedAt = "2026-08-14"; :>> judgedBy = "panelProtagonist"; }
    #Verify dependency from panelFindingGCritique to d0142;

    // ── The protagonist's brief: what the existing model already got right ────

    verification panelProtagonistCritique : Test {
        :>> id = "c0a10015-0814-4a15-9a15-516273840015";
        :>> title = "Critique d0139 — feasibility lens: the base-first RULE survives its own migrations being refuted";
        :>> createdAt = "2026-08-14";
        :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::feasibility;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::Low;
        :>> procedureText = "THE PROTAGONIST'S BRIEF was to argue for what the existing model already does well, and this is the one panel position that SURVIVED rather than being upheld as a defect. ATTACK ON THE ATTACKERS: if D0139's two named migrations are both refuted, is the base-first rule itself unsound? SURVIVED — the opposite. D0139(B) requires a custom marker to carry recorded justification that no base construct suffices. #Verify and #DerivedFrom now HAVE that justification, kernel-verified: 309 of 456 #Verify edges target `part Decision`, which cannot be a requirement objective, and all 37 #DerivedFrom edges are cross-metaclass, which D0055 already records as impossible for `:>`. The two markers stop being unjustified dialect and become documented last resorts. A rule that can refute its own proposed application, using its own evidentiary standard, is working. RECORDED BECAUSE IT IS EASY TO LOSE: a panel that produces only findings reads as a model with no strengths, and the human asked for a protagonist precisely so that conclusion could not be drawn by default.";
    }
    part panelProtagonistCritiqueR1 : TestResult { :>> id = "c0a10016-0814-4a16-9a16-516273840016"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ec7d548"; :>> judgedAt = "2026-08-14"; :>> judgedBy = "panelProtagonist"; }
    #Verify dependency from panelProtagonistCritique to d0139;
}
