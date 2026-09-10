// =====================================================================
// EvidencedControlsNeeds — a control binds the actor that has subverted the process, and is kept on
// the evidence that it did something.
//
// SOURCE. st115 (2026-09-10: "guards that handcuff me and do very little to AI agents ... are there
// indications that agents have attempted to accept decisions under my authority? if not, then this
// guard is moot and should be removed along with others like it ... users like me shouldn't be the
// entities strictly regulated here ... build evidence around the ways processes are subverted today"),
// translated as us105 (controls bind the AI, kept on evidence) and us106 (the evidence feeds STPA and
// comes out as needs, use cases and requirements). Every Need here cites the story that cites the words.
//
// WHAT THE RECORD SAID WHEN ASKED (issue445, issue446): no refusal by any control is recorded anywhere;
// the fire-ledger writes every in-loop block as an allow; the acceptance guards' origins are a vigilance
// gap (issue059), a code-read UCA (issue426) and eight refusals of the human's own correct acceptances
// (issue397). The subversions the record DOES hold are all the agent's: issue267, issue266, issue256,
// issue315, issue322, issue400, issue434.
// =====================================================================
package EvidencedControlsNeeds {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    part nControlBindsTheSubvertingActor : Need {
        :>> id = "af1bdb8d-bcb6-4a5a-a6df-0d5fce8c45e0";
        :>> title = "A control that refuses a human's act is kept only on evidence of the act it prevents";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "Every control in the engine can be read as binding a human act, an AI act, or either; each carries the incidents that motivated it, classified observed (the act happened) or code-read (found by inspection), and the blocks it has made, by the kind of actor stopped. A control that binds a human act and shows no observed incident and no block of an AI actor is a candidate for removal, surfaced to the human with the evidence, one Decision each. MEASURABLE: the census answers 'has an agent ever attempted X' for any control from the ledger, not from memory; the count of human-binding controls with no evidence trends to zero; no refusal anywhere leaves no trace. WHAT THIS EXCLUDES: it does not remove a control that binds the AI, however irritating - the record of AI-side subversion is the reason those exist.";
    }

    part nSubversionEvidenceGroundsTheAsks : Need {
        :>> id = "be7ebf50-8874-432d-a9ba-ec9d79d69e37";
        :>> title = "What the process asks the human to decide is grounded in how the process was actually subverted";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "The STPA run over this project's own control structure carries, per unsafe control action, whether the act has been observed or only reasoned about, and the implementation asks that come out of a run trace to a Need, a use case and a requirement before they reach the human. MEASURABLE: every UCA in the run record carries an evidence class computed from its Issue links; every proposed Decision that adds or removes a control names the UCA and the incident or its absence; a run that adds only code-read UCAs says so in its record. WHAT THIS EXCLUDES: a hypothetical UCA is still recorded - it is labelled, not dropped - because the analysis is where a control's absence of evidence becomes visible.";
    }

    #DerivedFrom dependency from nControlBindsTheSubvertingActor to us105;
    #DerivedFrom dependency from nSubversionEvidenceGroundsTheAsks to us106;

    // ── USE CASES: how the two Needs are exercised (Business workflow, us106: the asks come out
    // as needs, use cases and requirements). Actors are the human decider and the analyst running
    // stpa-self; the census and the ledger are the system. ─────────────────────────────────────
    use case ucAskWhetherAnAgentEverAttempted : UseCase {
        :>> id = "5888f77b-9d25-4115-959f-c8a1862ef500";
        :>> title = "Ask whether an agent has ever attempted the act a control refuses";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> description = "The human asks, of one control, 'has an agent ever attempted this?' (st115). The answer comes from the census - the ledger's blocks by actor kind, the Issues whose title names the control, each classed observed or code-read - never from the agent's memory of the session.";
        :>> mainSuccess = "keel show control-census . -> the control's row: whose act it binds, blocks by actor kind, its incidents with their class, and its evidence class (friction / hypothetical / evidenced). A friction or hypothetical row is already listed under removalCandidates.";
        :>> extensions = "The control is a hook rule the map never declared -> the ledger row is evidenced but nothing binds it; declare it with a `hook-rule:` title so the census and the map name one control (ctlHeredocBackslash was this case). The ledger predates 2026-09-10 -> the line is `unrecorded` and counts as no evidence either way.";
    }
    use case ucDissolveAFrictionControl : UseCase {
        :>> id = "9b0a1d28-f163-4180-af67-f4aa245ac0b1";
        :>> title = "Dissolve a control that has only ever refused the human";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> description = "A control the census classes friction (refusals of a human act, none observed subverting) is put to the human as one Decision with its evidence row quoted - the incident count, the blocks by actor kind, the constraint that stands on it - and the human keeps or dissolves it.";
        :>> mainSuccess = "One Decision per removal candidate (D0303), each naming the control, its census row and the UCA or constraint that stands on it; on acceptance the control's source, its map entry and its constraint are removed in one commit, and the census no longer lists it.";
        :>> extensions = "The control also binds the AI (Either) -> it is narrowed to the AI's act, not removed (D0426 kept gConfirmationAuthenticity's delegated-words check this way). The human declines -> the Decision stands rejected and the row stays listed; the ask is not repeated until its evidence changes.";
    }
    use case ucRunStpaOnTheCensus : UseCase {
        :>> id = "5c94d913-18c1-448b-90ba-e8ba47bf01de";
        :>> title = "Run STPA on the engine with the census read first";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> description = "The analyst (an agent under stpa-self) reads the control structure, then the control census, then walks the unanalysed actions. Each UCA written is classed from its edges: observed when it reaches an Issue whose incident happened, code-read otherwise. The run record carries the counts and the removal candidates copied from the lens, so a run that adds only code-read UCAs says so.";
        :>> mainSuccess = "stpa-self steps stpa1 -> stpa2 -> stpa2bCensus -> stpa3 -> stpa4 -> stpa5; the run's Test carries `EVIDENCE: <n> observed, <m> code-read` and `REMOVAL CANDIDATES: ...`; stpa-currency's uncovered set shrinks by the actions analysed.";
        :>> extensions = "A UCA's incident is a title reporting an act that happened while the Issue reads discoveredInField = false -> the flag is corrected in the run (issue360/372/374/375/398 were) and the class follows. No Issue names the act -> the UCA stays code-read; it is labelled, not dropped.";
    }
    use case ucGroundAControlAskInItsUca : UseCase {
        :>> id = "eba83ebf-0e1c-4d3d-b1bf-bb469b8fb70d";
        :>> title = "Ground a control change in the UCA and the incident it answers";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> description = "Before a Decision that adds or removes a control reaches the human, it names the UnsafeControlAction it answers and the incident (an Issue, observed) or the absence of one (code-read). The ask traces UCA -> Need -> use case -> requirement, so the human judges an empirical claim, not a hypothesis dressed as one.";
        :>> mainSuccess = "The Decision text names the UCA by name and its class; a `#DependsOn` or `#DerivedFrom` edge reaches the UCA, the ControllerConstraint or the Issue; the decision brief quotes the census row.";
        :>> extensions = "The Decision names no UCA -> it is a hypothesis; it is written as code-read and says so, or it waits for the run that analyses the action (stpa-currency names the tranche).";
    }

    // ── SYSTEM REQUIREMENTS. The first three are verified by the DoD Tests of the work that built
    // them (the #Verify edges below cite that evidence, D0108: the delivering thread's judgment);
    // the fourth is a requirement no work has satisfied - it reads as UNVERIFIED in tier-satisfaction
    // until a resolver lands, which is the honest state (D0098). ───────────────────────────────────
    requirement srControlCensusBindsByActorKind : SystemRequirement {
        :>> id = "70e61aea-f856-47f0-8b1e-f464f2936547";
        :>> title = "Every control is read as binding a human act, an AI act, or either, with its evidence class computed from the ledger and the Issues";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> statement = "The engine shall compute, for every control it holds - guard, ctl constraint, hook rule, write-path check, commit-gate tier - whose act it binds (human / ai / either) with the rule that decides it, the fire-ledger's blocks by actor kind, the Issues whose title names it classed observed (discoveredInField) or code-read, and an evidence class: friction (only refusals of a human act, none observed subverting), hypothetical (code-read only, zero blocks), evidenced. Friction and hypothetical controls shall be listed first as removal candidates.";
        :>> rationale = "st115: 'are there indications that agents have attempted to accept decisions under my authority? if not, then this guard is moot and should be removed along with others like it.' The question is answerable only if the census exists and reads the record, not memory (nControlBindsTheSubvertingActor MEASURABLE).";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srRefusalLeavesALedgerFact : SystemRequirement {
        :>> id = "2ea1dda2-4043-44bb-8608-3399e722b154";
        :>> title = "No refusal anywhere leaves no trace: every block, deny and refused write is a ledger line naming the control and the actor kind";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> statement = "Every hook fire shall record the verdict it EMITTED (allow / block / deny), a non-allow line shall name the control that refused and the kind of actor it fell on (human / ai / undeclared / unbound), every write path that refuses shall write a `refused` line naming `verb:check`, and each commit-gate tier shall write its own line; `keel enforcement-report` shall read them back per event, control and actor kind. A line from before the rule shall read `unrecorded`, never as zero refusals.";
        :>> rationale = "Before 2026-09-10 the ledger wrote every in-loop block as an allow, so the count of agent attempts against any control was unknowable (issue445, issue446) - the census in srControlCensusBindsByActorKind has nothing to read without this.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srUcaEvidenceClassIsComputed : SystemRequirement {
        :>> id = "a323b47a-76a1-4f60-b656-1398c6420b5f";
        :>> title = "A UCA's evidence class is computed from its edges, and the STPA run reads the census before it writes one";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> statement = "The engine shall class every UnsafeControlAction observed when a dependency edge from it, or from a ControllerConstraint bound to it, reaches an Issue with discoveredInField = true, and code-read otherwise; shall list the constraints standing on a friction or hypothetical control and the friction controls nothing binds as the run's removal candidates; shall name the evidenced controls no analysis stands on; and the stpa-self process shall read the census before any UCA is written and record `EVIDENCE: <n> observed, <m> code-read` with the candidates in the run's Test.";
        :>> rationale = "nSubversionEvidenceGroundsTheAsks MEASURABLE: every UCA carries an evidence class computed from its Issue links, and a run that adds only code-read UCAs says so. Without it a hypothetical hazard and an observed one read the same in the record.";
        :>> kind = RequirementKind::functional; :>> priority = Priority::must; :>> verificationMethod = VerificationMethod::test;
    }
    requirement srControlDecisionNamesItsUca : SystemRequirement {
        :>> id = "166902dc-6743-41a1-b6cf-bc9a05029ce5";
        :>> title = "A Decision that adds or removes a control names the UCA it answers and the incident or its absence";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> statement = "A Decision carrying the process-change or safety-change marker that adds, narrows or removes a control shall name, in its text or by a `#DependsOn` / `#DerivedFrom` edge, the UnsafeControlAction or ControllerConstraint it answers and the Issue that evidences it, or state that the UCA is code-read. A control Decision that names neither shall be reported by a warning-level guard before it reaches the decision brief.";
        :>> rationale = "nSubversionEvidenceGroundsTheAsks MEASURABLE: every proposed Decision that adds or removes a control names the UCA and the incident or its absence. This is the clause no work has yet built - the census and the UCA class exist; the check that a control ask cites them does not. It stands unverified here so the gap is computed, not remembered (D0098).";
        :>> kind = RequirementKind::functional; :>> priority = Priority::should; :>> verificationMethod = VerificationMethod::test;
    }

    // ── edges: requirement satisfies Need; use case exercises Need; DoD evidence verifies SR ───────
    satisfy nControlBindsTheSubvertingActor by srControlCensusBindsByActorKind;
    satisfy nControlBindsTheSubvertingActor by srRefusalLeavesALedgerFact;
    satisfy nSubversionEvidenceGroundsTheAsks by srUcaEvidenceClassIsComputed;
    satisfy nSubversionEvidenceGroundsTheAsks by srControlDecisionNamesItsUca;
    #DerivedFrom dependency from ucAskWhetherAnAgentEverAttempted to nControlBindsTheSubvertingActor;
    #DerivedFrom dependency from ucDissolveAFrictionControl to nControlBindsTheSubvertingActor;
    #DerivedFrom dependency from ucRunStpaOnTheCensus to nSubversionEvidenceGroundsTheAsks;
    #DerivedFrom dependency from ucGroundAControlAskInItsUca to nSubversionEvidenceGroundsTheAsks;
    #Verify dependency from dcControlCensusByActorKindDoD to srControlCensusBindsByActorKind;
    #Verify dependency from dcRefusalIsALedgerFactDoD to srRefusalLeavesALedgerFact;
    #Verify dependency from dcUcaCarriesItsEvidenceClassDoD to srUcaEvidenceClassIsComputed;
    #Verify dependency from dcControlDecisionNamesItsUcaDoD to srControlDecisionNamesItsUca;
}
