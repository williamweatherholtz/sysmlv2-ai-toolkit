// Per-actor issues records for claudeFable5 (issue210/srDcPerActorWriteTargets): new records
// route here so concurrent contributors never conflict at a shared append anchor.
package IssuesClaudeFable5 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part issue214 : Issue {
        :>> id = "a6db1f9a-75cd-4d20-8ff8-c0030e6ca1fa";
        :>> title = "per-actor routing probe A";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "issue210's live verification probe, kept as the record of its own routing: recorded through keel record issue AFTER the per-actor change and required to land in issues-claudeFable5.sysml with the next globally-allocated number - which it did, making this Issue both the test and the evidence. Resolved by dcPerActorAppendFiles alongside issue210 itself.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcPerActorAppendFiles to issue214;

    part issue215 : Issue {
        :>> id = "67cedf7a-c673-485e-a85d-3e4fc0efc046";
        :>> title = "The allocation layer is stale and actively served: June components, 26 edges for 102 SRs, a retired tool described as the single source of state";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1, found independently by all five panelists. The allocIfaceGate confirmation was TRUE at abb033a (June) and is false at HEAD with nothing re-firing the obligation - the K10 reprocess gap applied to the engine's own architecture phase; trace-need serves the stale closure as current truth (EHZ8's open instance). Direction Decision D0194 queued (supersede + charter-chain/registry mechanism + coverage leg).";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from dcAllocationRefresh to issue215;

    part issue216 : Issue {
        :>> id = "93fb02ab-fa22-43df-ab67-5c9d10424323";
        :>> title = "pmMeasureDoD was stamped pass with its promotion-review criterion unmet - a hollow pass on the anti-hollow-pass invariant";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1, automotive finding 1 (their flip set). The DoD's text requires 'at least one advisory-to-blocking promotion or non-promotion decision cites >= 3 sprints of this evidence'; no such decision exists, and the ledger was 2 sessions old when R1 was stamped (judgedBy claudeFable5 - my stamp, the exact EHZ4 class). REOPENED honestly: pmMeasureDoDR2 fail at HEAD returns pmMeasure to the frontier, where it waits for the promotion/non-promotion Decision the guided profile's evidence bar (D0180) defines. The machinery clauses all hold; the item is open on the review clause alone.";
        :>> discoveredInField = false;
        :>> severity = Severity::High;
    }
    #Resolves dependency from pmMeasure to issue216;

    part issue217 : Issue {
        :>> id = "d216018a-8fee-40cf-8b63-cbbdc0af30f0";
        :>> title = "31 accepted decisions carried a PROPOSED header - prose restating the status field, gone stale";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1, automotive finding 5 (D0105 dual-truth class; it briefly misled the reviewer). FIXED in-sprint: keel accept now rewrites the scaffolded header on acceptance, and the 31 existing files were swept (the header now points at the status field instead of restating it).";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from storyPanelRound1 to issue217;

    part issue218 : Issue {
        :>> id = "d7736a07-a61a-4242-aa91-039e241368c7";
        :>> title = "The code registry omitted the human channel, the K15 backstop, the git spawn point, the hook dispatch, and the run lifecycle";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1 (SE finding 3, ARCH finding 6, AERO finding 4): against the registry's own criterion - units whose failure would matter - the AI write channel was risk-classified while serve.rs (human-judgment endpoints), history.rs (audit-history), gitx.rs, cmd_hook, and launcher.rs were not. FIXED in-sprint: five CodeElements added with riskClass/invariantSafety/stpaRole judgments (registry 12 -> 17).";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from storyPanelRound1 to issue218;

    part issue219 : Issue {
        :>> id = "f0a76d4a-0a1c-4df6-9674-37714ff0e106";
        :>> title = "The hook watchdog exited 0 with no ledger line - the one fail-SILENT branch inside the instrumented layer";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1, robotics finding 4: a wedged hook vanished without a trace, invisible to D0180's single instrumentation path and to K2's fail-loud inventory. FIXED in-sprint: the watchdog appends event=hook-watchdog-timeout before its mandated exit 0; declared in control-events.toml; guard 45's two-way diff covers it.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from storyPanelRound1 to issue219;

    part issue220 : Issue {
        :>> id = "0f6bddc4-7bb5-42b2-912e-459c5ce07d13";
        :>> title = "view.rs is a flagged god-module with no chartered follow-through";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R1, architecture finding 3: 8159 lines, ~235 definitions, depended on by guards/orient/claims/arch, marked #Restructure by the registry (Ca=4, I=0.20) with no backlog item or accept-risk Decision - a standing flag with no resolving edge, the untriaged-flag failure the engine forbids for Issues. Chartered: dcViewRsRestructure (split along lens seams, or record the accept-risk Decision).";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcViewRsRestructure to issue220;

    part issue221 : Issue {
        :>> id = "d0c6bfe8-365e-4f80-b88a-b9d3069b4eb4";
        :>> title = "EHZ9's controls trace to no standing requirement - removing the release gate would violate nothing recorded";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R2, aerospace finding 1 (MINOR, unanimous-accept residue): the ungated-outward-publish hazard's controls (release gate, pre-push check, land's merged-tree gate) have no invariant/SR anchor - the K-set has no outward-publish member. Resolver: D0195 clause 8 (added round 2) charters the SR anchor under dcControlMapView's delivery.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from dcControlMapView to issue221;

    part issue222 : Issue {
        :>> id = "a33e001f-489b-4a17-a3ba-b71705c4d9b5";
        :>> title = "GATE_ORDER hand-duplicated the declared ceremony successions - two homes for one fact";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "Panel R2, robotics finding 2 (MINOR): guards.rs GATE_ORDER is a compiled constant duplicating the succession chain declared in .engine/workflows/delivery.sysml; post-ingestion both live in the model and a CR editing the declared order would leave the guard silently enforcing the old one. FIXED in-sprint: gate_order_matches_the_declared_successions parses the declared chain at compile time and fails the build on divergence.";
        :>> discoveredInField = false;
        :>> severity = Severity::Low;
    }
    #Resolves dependency from storyPanelRound1 to issue222;

    part issue223 : Issue {
        :>> id = "77e4e057-9c16-4e55-8d37-e699c8a057af";
        :>> title = "The deck presents a fork Decision as a bare Sign card, soliciting an unbindable acceptance";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "d0192 enumerates OPTION A and OPTION B and states 'accepting this Decision means choosing ONE'; the deck rendered it as a single Sign button. wweatherholtz tapped it (inbox row uid 5b1d2b0e, At 2026-08-22T06:54:40Z, deck HEAD 3c9645e) and the tap could not be recorded - the gesture was solicited and then wasted, exactly the authoring-friction class D0054 names. The control (D0047): the deck detects OPTION-enumerating proposed decisions and renders one sign button per option, stamping the chosen option into the note so the recorded acceptance carries the choice.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcDeckForkOptions to issue223;

    part issue224 : Issue {
        :>> id = "1a148a03-fc88-4705-8bc9-f36c9d6a9477";
        :>> title = "The recording delegation covers acceptances only while the practice also records confirmation flips";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "srK06a completeness critique (srK06aCompl1, sprint 421): D0192 OPTION A declared the delegation and its quote-substance check for decisionAcceptance, but the identical practice - AI records the human's judgment on their verbatim words - also flips method=confirmation DoD results (sprint 420's dcHumanChannelDelegationRuleDoD flip is the live example), and no [confirmationRecord] policy class or substance demand exists there. Same asymmetry D0192 closed, one level down. Design note: a DoD result carries no note field, so where the quote lives is a design question, not a copy of the acceptance check.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcConfirmationRecordDelegation to issue224;

    part issue225 : Issue {
        :>> id = "9000ce80-8947-4391-9976-c4f5d76dcc0f";
        :>> title = "Parser proptest identifier strategy generates reserved words - latent CI flake";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "The sprint-421 push went CI-red on prop_part_string_attr: the [a-zA-Z_][a-zA-Z0-9_]{0,31} strategy generated 'doc', which the lexer keywords and the parser correctly refuses as a bare identifier. No commit in the push touched the parser - the random seed found a latent generator defect. Control (D0047): the lexer exposes is_reserved_word (single source: the keyword match itself) and the strategy filters through it, so the generator cannot drift from the grammar; a deterministic test pins the refusal.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcProptestReservedWords to issue225;

    part issue226 : Issue {
        :>> id = "b9aa7ad7-e1b0-40cb-9462-d629c5863ad8";
        :>> title = "The enforcement contract claimed D0108's ownership rule was unchecked - stale since sprint 293";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> description = "process-enforcement.toml [distributedCollaboration] said 'Nothing checks it today. This is the entry I would most expect a reviewer to act on' - but the ownership guard has existed since sprint 293 (1b2fd4f, 2026-08-15): staged-diff vs HEAD, item-level field comparison against the bound actor, D0067 migration keystone exemption. A dual truth (D0018): the hardening view faithfully reported a stale declaration, and sprint 421 chartered dcOwnershipDiffGuard to build a guard that already existed. The contract text was never swept when the guard landed - a doc-sync miss on a CONTRACT surface. Control: the unit now asserts the constraint (D0139(D) unit-ownership), so the enforcement map computes from the model instead of restating it in prose.";
        :>> discoveredInField = false;
        :>> severity = Severity::Medium;
    }
    #Resolves dependency from dcOwnershipDiffGuard to issue226;
}
