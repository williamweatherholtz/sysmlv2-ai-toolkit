// =====================================================================
// EngineDerivedNeeds — the twelve Needs recovered from the code, not from a conversation.
//
// PROVENANCE. These were DERIVED BOTTOM-UP from the artifacts (41,203 LOC, the 89-entry CLI help
// surface, 52 guards, 6 hook events, 50 /api routes, 25 processes, 38 skills, 16 contract files,
// 4 schema packages, 4 CI workflows) and then diffed against the 64 authored Needs. A keyword scan
// over all 64 statements returned ZERO for intake and verbatim elicitation, ZERO for migration and
// version skew, ZERO for a foreign or downstream tree, ZERO for friction, ZERO for promise and
// follow-through, and one incidental hit each for indicator and recall. Each Need below therefore
// names a capability that has schema, a write path or a command, a process, and a guard or contract
// behind it, and that nothing in the model ever asked for.
//
// EVERY ONE CARRIES #DerivedFrom -> us029. D0216: a Need with no cited utterance is MY JUDGMENT and
// must say so. The human's instruction, recorded verbatim as st023 and translated as us029, was:
// "So sure, create them as needs." The derivation is mine; the decision to make them Needs is theirs,
// and the edge is what keeps those two facts distinguishable.
//
// NO SystemRequirement IS AUTHORED HERE. D0246 clause C: authoring these does not retroactively root
// the 415 decision-chartered stories, and no story is to be re-parented onto them to move the
// rootedness number. `tier-satisfaction` will report twelve Need-tier gaps as a RESULT of this file.
// That fall is the finding becoming visible, and closing it with requirements that do not exist would
// destroy the only signal this pass produced.
//
// Chartered by D0246 (keel's subject is AI work management).
// =====================================================================
package EngineDerivedNeeds {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    // ── U1: the intake layer, which has a schema, a write path, a process and a view ─────────────

    requirement nIntakeBeforeInterpretation : Need {
        :>> id = "23fa79d2-8481-4552-bc15-4f27a2625c1c";
        :>> title = "A stakeholder's words are recorded verbatim before anything interprets them";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::customer;
        :>> statement = "What a person actually said is an authored fact in its own right, recorded character-for-character and attributed, BEFORE any Need, Brief, Story or work item is derived from it. Every interpretation is a separate item that cites the utterance it translates, so a reader can always tell the speaker's words from the reader's reading of them, and a derived item with no cited source is visibly an invention rather than silently indistinguishable from a sourced one.";
        :>> priority = Priority::must;
    }

    // ── U2: `keel recall`, the user-prompt hook, and the A/B and benchmark harnesses ─────────────

    requirement nTurnStartRecall : Need {
        :>> id = "65af25f9-aee2-4ab1-96c5-05fd86b21792";
        :>> title = "An agent begins each turn with the model's relevant facts, not with its memory";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "The facts an agent needs to act correctly are pushed to it from the model at the start of a turn, seeded from what was actually asked, with the retrieval visible in the turn. An agent's recollection is not a source: the model is. Retrieval that cannot complete says so and pushes nothing, rather than silently degrading the turn.";
        :>> priority = Priority::must;
    }

    // ── U3: migrate.rs, engine-version.toml, installed-units.toml, the version parity warning ────

    requirement nEnginePinAndEvolution : Need {
        :>> id = "401adbc6-3c4b-4c8e-93c9-875e8e4e6870";
        :>> title = "A project pins the engine version that gates it, and survives engine evolution";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "A project declares which engine version its checks are defined against, and that declaration is what runs — not whatever binary happens to be present on the machine. An engine that has moved on does not strand the project: there is a path that brings an existing tree up to a stated vintage without fabricating history. A project that never upgrades keeps working, and a project that does upgrade does so as a visible, dated act in its own tree.";
        :>> priority = Priority::must;
    }

    // ── U4: workspace.rs, `keel projects`, gate --workspace, the qualified decision id ───────────

    requirement nRepoHoldsSeveralProjects : Need {
        :>> id = "de354ce4-58dc-4019-b759-6cab2d923a64";
        :>> title = "One repository may hold several projects";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "Several tracked projects can live in one git repository without any of them losing gate coverage, identity or attribution. Where git forces a single answer for the whole repository — one hooks path, one push, one history — the engine gates every project the act touches and names the ones it skipped, rather than gating one and appearing to gate all. Where a name is unique only within a project, it is qualified by project wherever it crosses the repository boundary. A single-project repository is unaffected throughout.";
        :>> priority = Priority::must;
    }

    // ── U5: adoption_check.rs — 0 of 51 guards read another tree; 11 of 20 issues found there ────

    requirement nForeignTreeVerification : Need {
        :>> id = "68b2dde0-6305-4577-93d6-f46d758510d0";
        :>> title = "Verification runs against trees the engine did not author";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "The engine is verified against projects other than the one that built it, because that is where its defects actually are: measured on this project's own record, no guard read another project's tree while defects clustered at exactly that boundary. A project that has not adopted a control is in a legitimate state and is never reported as violating it.";
        :>> priority = Priority::must;
    }

    // ── U6: attestation.rs, attestation-policy.toml, the --evidence refusal ──────────────────────

    requirement nPassDisclosesItsSource : Need {
        :>> id = "8940a76f-1f09-4298-8f23-e317c39e56c2";
        :>> title = "A pass discloses what produced it";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "A recorded pass says what kind of thing produced it — a command that ran, a human's word, a reading — so a reader can tell a receipt from a testimony without trusting the recorder. A claim that something was EXERCISED records what exercised it, and a machine actor cannot supply a human's acceptance for any claim.";
        :>> priority = Priority::must;
    }

    // ── U7: the Indicator/Measurement schema, `indicators`, invariant 7's anti-Goodhart rule ─────

    requirement nMonitoredNotGated : Need {
        :>> id = "35356401-38eb-4ebd-9efc-fe7e980cf7b2";
        :>> title = "A quantity worth watching but not defensibly boundable is monitored, never gated";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "A measurement whose good-enough boundary cannot be justified is tracked as a monitored series with no enforced threshold, and is promoted to an enforced contract only when a defensible boundary emerges. Gating a number nobody can justify converts it into the target and destroys it as evidence; leaving it unmeasured loses the signal entirely. Both failures are avoided by making monitored-without-a-threshold a first-class, visible state rather than an absence.";
        :>> priority = Priority::should;
    }

    // ── U8: decision-artifacts.toml, `decision-follow-through` — D0161 accepted, artifact absent ─

    requirement nDecisionPromisesTracked : Need {
        :>> id = "49684d92-0d77-44d0-b798-5349a79d571b";
        :>> title = "What an accepted decision promised is tracked until it exists";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::operator;
        :>> statement = "When a decision is accepted on the promise that something will exist, that promise is tracked against the tree until the thing is there. Acceptance alone must not make the model read as complete: a decision whose artifact was never built is reported as an open promise rather than as done work. This is the mechanism behind half-finished work appearing settled, and it is checked against what is actually present, not against what was recorded.";
        :>> priority = Priority::must;
    }

    // ── U9: D0054, named the #1 project risk in CLAUDE.md and absent from all 64 prior Needs ─────

    requirement nAuthoringIsLowFriction : Need {
        :>> id = "a8847d7e-2ced-4750-b942-1e567986b49a";
        :>> title = "Recording a fact is low-friction by construction";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::customer;
        :>> statement = "Identity, dates and provenance are supplied by the engine rather than typed; the scaffolding of a routine record is generated rather than hand-written; and a recurring friction that has been met more than once is removed by tooling rather than by remembering. The dominant failure of disciplined modelling is adoption friction rather than bad structure: if recording is harder than the informal alternative, the alternative wins and the model stops being true.";
        :>> priority = Priority::must;
    }

    // ── U10: the parser-coverage guard + its ratchet baseline; incidents 102, 109, 119 ───────────

    requirement nParserIsTheTrustedBase : Need {
        :>> id = "ac6f9aef-05cb-49a0-9497-4e0df755b84b";
        :>> title = "A construct the parser skips is invisible to every check built on it";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "The parser is the trusted base of every guard, view and report: a statement class it does not read is not merely unvalidated, it is invisible to the entire enforcement stack, and a clean gate over unread text is a false negative rather than a pass. The set of constructs the parser skips is therefore counted and visible, new invisibility is a failure, and existing invisibility stays a standing warning rather than being written off.";
        :>> priority = Priority::must;
    }

    // ── U11: control-arming.toml + `controls` controlArming — the issue240 false all-clear ───────

    requirement nDeclaredControlDistinguishableFromArmed : Need {
        :>> id = "28bb3259-924d-4e2a-9730-2bcc178c5695";
        :>> title = "A control that cannot fire is distinguishable from one with nothing to catch";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "Declaring a control against a failure condition is not the same as that control being able to fire, and any surface reporting coverage states which of the two it means. A silent control and a satisfied one produce the same absence of findings, so an all-clear that cannot distinguish them is misleading while being literally true — the condition under which this engine once reported no uncovered hazard while its commit gate could not run at all.";
        :>> priority = Priority::must;
    }

    // ── U12: the doc-sync guard, doc-guard-count, and the doc-sync process ───────────────────────

    requirement nDocSurfaceCannotOutliveTheChange : Need {
        :>> id = "84d09ea5-54ce-4301-9c37-abe6817a3fde";
        :>> title = "The documented surface cannot outlive the change that invalidated it";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> source = NeedSource::internal;
        :>> statement = "A change that invalidates a documented claim is not complete until that claim is corrected in the same act, because prose describing a surface that has moved is worse than no prose: it is read as current and cannot be distinguished from it.";
        :>> priority = Priority::should;
    }

    // ── The derivation edge: mine to derive, theirs to make Needs (D0216) ────────────────────────

    #DerivedFrom dependency from nIntakeBeforeInterpretation to us029;
    #DerivedFrom dependency from nTurnStartRecall to us029;
    #DerivedFrom dependency from nEnginePinAndEvolution to us029;
    #DerivedFrom dependency from nRepoHoldsSeveralProjects to us029;
    #DerivedFrom dependency from nForeignTreeVerification to us029;
    #DerivedFrom dependency from nPassDisclosesItsSource to us029;
    #DerivedFrom dependency from nMonitoredNotGated to us029;
    #DerivedFrom dependency from nDecisionPromisesTracked to us029;
    #DerivedFrom dependency from nAuthoringIsLowFriction to us029;
    #DerivedFrom dependency from nParserIsTheTrustedBase to us029;
    #DerivedFrom dependency from nDeclaredControlDistinguishableFromArmed to us029;
    #DerivedFrom dependency from nDocSurfaceCannotOutliveTheChange to us029;
}
