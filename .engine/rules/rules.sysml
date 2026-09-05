// =====================================================================
// EngineRuleSet — the engine's DECLARED controls (D0105 migration). These run via `keel check`
// ALONGSIDE the Rust guards during EXPAND; a guard retires only once its rule reaches PARITY
// (`keel check` == `keel guard` on the live repo — guardsToRulesMigration). Each rule carries its
// WHY as a typed #JustifiedBy edge to the Decision that justifies it (provenance, not prose).
// =====================================================================
package EngineRuleSet {
    private import EngineElement::*;
    private import EngineRules::*;
    private import EngineRelationships::*;

    // First migrated rule — mirrors guard:requirement-rootedness (D0099). Parity target:
    // view::capability_root_violations (a #Capability with no #DerivedFrom edge to a Need).
    part capabilityRootednessRule : EdgeRule {
        :>> id = "d0105r001-0001-4001-9001-516273841001";
        :>> title = "A user-facing capability must derive from a Need";
        :>> subjectType = "#Capability";
        :>> requiredEdge = EdgeKind::derivedFrom;
        :>> edgeDirection = EdgeDirection::outgoing;
        :>> objectType = "Need";
        :>> cardinality = RuleCardinality::atLeastOne;
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "all";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from capabilityRootednessRule to d0099;

    // Mirrors guard:issues (D0077/D0078). Parity target: view::untriaged_issues (an Issue with no
    // INCOMING #Resolves edge from a resolving action/Decision).
    part issuesTriagedRule : EdgeRule {
        :>> id = "d0105r002-0002-4002-9002-516273841002";
        :>> title = "Every Issue must be triaged (have a resolving edge)";
        :>> subjectType = "Issue";
        :>> requiredEdge = EdgeKind::resolves;
        :>> edgeDirection = EdgeDirection::incoming;
        :>> objectType = "*";
        :>> cardinality = RuleCardinality::atLeastOne;
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "all";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from issuesTriagedRule to d0078;

    // Mirrors guard:decision-rationale (D0103). Parity target: view::decisions_weak_rationale (a Decision
    // whose context OR rationale is blank/trivial — both must be a substantive >=20 chars).
    part decisionRationaleRule : ElementRule {
        :>> id = "d0105r003-0003-4003-9003-516273841003";
        :>> title = "A Decision must carry a substantive context + rationale (the why)";
        :>> subjectType = "Decision";
        :>> predicate = "minLength(context,20) and minLength(rationale,20)";
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "all";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from decisionRationaleRule to d0103;

    // Mirrors guard:acceptance-events (D0066). First rule using the SCOPE sub-language (whereStatus) + a
    // sibling-result predicate. Parity target: view::compute_attestation (a status=accepted Decision with
    // no passing <name>AcceptR1 event).
    part acceptanceEventRule : ElementRule {
        :>> id = "d0105r004-0004-4004-9004-516273841004";
        :>> title = "An accepted Decision must carry a passing acceptance event";
        :>> subjectType = "Decision";
        :>> predicate = "hasPassingResult(Accept)";
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "whereStatus(accepted)";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from acceptanceEventRule to d0066;

    // Mirrors guard:charter (D0068). First rule using the git-temporal `newlyAdded` scope. Parity target:
    // guards::charter (a Story in a newly-added .tracking/delivery file lacking a #CharteredBy edge).
    part charterRule : EdgeRule {
        :>> id = "d0105r005-0005-4005-9005-516273841005";
        :>> title = "A newly-added delivery Story must be chartered";
        :>> subjectType = "Story";
        :>> requiredEdge = EdgeKind::charteredBy;
        :>> edgeDirection = EdgeDirection::outgoing;
        :>> objectType = "*";
        :>> cardinality = RuleCardinality::atLeastOne;
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "newlyAdded";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from charterRule to d0068;

    // issue058 detecting rule: a Decision must NOT restate its acceptance verdict as PROSE (the dual-truth
    // the human caught — "ACCEPTED <date> by <who>" belongs in the discrete AcceptR1 TestResult, not a
    // field). Diagnostic (surfaced by `keel rules`, not gating); heuristic like decision-requirement-link.
    part decisionNoVerdictProseRule : ElementRule {
        :>> id = "d0105r006-0006-4006-9006-516273841006";
        :>> title = "A Decision must not restate its acceptance verdict as prose (one source of truth)";
        :>> subjectType = "Decision";
        :>> predicate = "notMatchesPatternCI(consequences,accepted 202) and notMatchesPatternCI(context,accepted 202) and notMatchesPatternCI(decision,accepted 202) and notMatchesPatternCI(rationale,accepted 202)";
        :>> severity = RuleSeverity::warning;
        :>> onViolation = ViolationKind::warn;
        :>> appliesWhen = "all";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from decisionNoVerdictProseRule to d0107;

    // issue059 (the enforceable slice of D0106): an accepted Decision's acceptance event must be judged by
    // a HUMAN (a Person), never AI-fabricated. GATING — the guard `confirmation-authenticity` sources from
    // this rule (CONTRACT pattern). Enforces "sign-off is a real human attestation" (D0016/D0106).
    part confirmationAuthenticityRule : ElementRule {
        :>> id = "d0105r007-0007-4007-9007-516273841007";
        :>> title = "An accepted Decision's acceptance must be human-judged (never AI-fabricated)";
        :>> subjectType = "Decision";
        :>> predicate = "resultJudgedByHuman(Accept)";
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "whereStatus(accepted)";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from confirmationAuthenticityRule to d0106;

    // D0192 OPTION A (the delegation clause, srK06aDelegatedRecording): an acceptance recorded under
    // the declared recording delegation must EVIDENCE its channel - quote the human's conversational
    // words verbatim (a single-quoted span) or cite a human surface gesture (deck/console/GitHub comment,
    // or the TTY itself - `keel accept` typed at an interactive terminal cites it, D0315/issue359). Forward-only
    // from D0192's acceptance date (issue068): earlier events are grandfathered - and the cutoff READS as the
    // later of this literal and the adopting project's adoption-profile declaredAt (D0319/issue352). GATING - the
    // `confirmation-authenticity` guard sources this rule alongside confirmationAuthenticityRule.
    // STATED LIMIT (from D0192 itself): an agent that fabricates a quote defeats this check; the
    // protections behind it are the human reading their own queue and the audit trail.
    part delegatedAcceptanceSubstanceRule : ElementRule {
        :>> id = "f20caebc-aef4-4487-83d4-b4a1be0d8900";
        :>> title = "A delegated acceptance record must quote the human's words (or cite their gesture)";
        :>> subjectType = "Decision";
        :>> predicate = "acceptQuotesDelegatedWords(2026-08-22)";
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "whereStatus(accepted)";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from delegatedAcceptanceSubstanceRule to d0192;

    // D0198 OPTION A (quote receipts): a human-judged confirmation FLIP recorded on/after the
    // delegation date must carry its evidence - its own text quotes the human's words or cites
    // their gesture, OR a companion <test>Attest<N> record does. Acceptance events keep their own
    // rule; companion records are the evidence, not subjects. Forward-only (issue068; the cutoff reads as the
    // later of the literal and the project's declaredAt, D0319). GATING via
    // the confirmation-authenticity guard. STATED LIMIT: a fabricated quote defeats it (D0192 family).
    part delegatedConfirmationSubstanceRule : ElementRule {
        :>> id = "7e348a2e-92f7-4bb0-a1c4-6d84f0b5c9e1";
        :>> title = "A delegated confirmation flip carries its quote receipt";
        :>> subjectType = "Test";
        :>> predicate = "confirmationQuotesOrAttested(2026-08-23)";
        :>> severity = RuleSeverity::blocking;
        :>> onViolation = ViolationKind::block;
        :>> appliesWhen = "all";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from delegatedConfirmationSubstanceRule to d0198;

    // issue055 (the enforceable slice of research-spike routing): a WorkKind::research Story must charter
    // to a LEGITIMATE governing source — its originating Decision/Need/SystemRequirement, or (as a spike)
    // an Issue/proposed-Decision (the union the charter guard sanctions, D0068/issue055) — never an
    // arbitrary element (a Story, a TestResult, ...). Converts issue055's triage-item-8 CONVENTION into a
    // declared control on the structurally-visible side — the "did analysis skip the spike?" judgment
    // stays reminder-enforced (a commit gate cannot see a conversation). WARNING first (the D0102
    // decision-requirement-link precedent — visible, non-gating; promote once proven low-noise). Uses the
    // whereKind scope + charterTargetType predicate (both added this sprint).
    part researchSpikeCharterRule : ElementRule {
        :>> id = "d0105r008-0008-4008-9008-516273841008";
        :>> title = "A research spike must charter to a legitimate governing source (Decision/Need/SystemRequirement/Issue)";
        :>> subjectType = "Story";
        :>> predicate = "charterTargetType(Issue,Decision,Need,SystemRequirement)";
        :>> severity = RuleSeverity::warning;
        :>> onViolation = ViolationKind::warn;
        :>> appliesWhen = "whereKind(research)";
        :>> exemptions = "";
    }
    #JustifiedBy dependency from researchSpikeCharterRule to d0111;
}
