// =====================================================================
// D0111 — declare researchSpikeCharterRule: the enforceable slice of research-spike routing (issue055),
// converting engine-triage item 8's CONVENTION into a declared control on the structurally-visible side.
// Part 2 of proveProcessDeployment (issue061). The D0070 keystone governing this rule addition
// (#ProspectiveChange). Human-accepted 2026-07-09.
// =====================================================================
package Decision0111 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #ProspectiveChange part d0111 : Decision {
        :>> id = "10a1b0c1-4111-42a3-ab5c-516273849711";
        :>> title = "D0111: declare researchSpikeCharterRule (research spike must charter to an Issue or Decision)";
        :>> createdAt = "2026-07-09";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue055's fix routed substantial analysis to a chartered WorkKind::research spike, but that routing lives ONLY as engine-triage checklist item 8 + the UserPromptSubmit reminder hook — vigilance, not a control. issue059 already converted the ATTESTATION half of D0106 into a real gate (confirmation-authenticity); proveProcessDeployment part 2 (issue061) asks for the parallel move on the spike-routing half. issue055's own fix (3) named the shape: 'if target-type validation is wanted, express it as a declared EdgeRule under the declarative-controls direction' (D0105/D0107). The honest limit (stated in both issue055 and issue059): a commit-time gate cannot see a conversation, so it cannot force 'did substantial analysis skip the spike?' — only the STRUCTURALLY-VISIBLE side is gatable.";
        :>> decision = "Declare researchSpikeCharterRule (.engine/rules/rules.sysml): a WorkKind::research Story must charter to a LEGITIMATE governing source — its originating Decision/Need/SystemRequirement, or (as a spike) an Issue/proposed-Decision — i.e. charterTargetType(Issue,Decision,Need,SystemRequirement), the exact union the charter guard message sanctions (guards.rs:376, D0068/issue055) — never an arbitrary element (a Story, TestResult, ...). ALLOW-LIST WIDENED during implementation: the first cut (Issue|Decision only) spuriously flagged sr19SpikeStory, a legitimate spike chartering to a SystemRequirement — corrected to match D0068, not a scope change. WARNING-level (RuleSeverity::warning / ViolationKind::warn) — runs in `keel rules`, visible, non-gating — following the D0102 decision-requirement-link precedent; promote to a hard guard once proven low-noise. Realized via two small, unit-tested evaluator additions: a whereKind(v) scope (mirrors whereStatus) and a charterTargetType(T1,...) predicate term (every outgoing #CharteredBy edge targets an allow-listed type). #JustifiedBy this Decision.";
        :>> rationale = "ALTERNATIVES: (i) a hard gate now — DEFERRED: charter-target-type is arguably a well-formedness property and 0-violation today (safe to gate), but a new control earns trust before it blocks (D0102 precedent); the human chose warning-first. (ii) a bespoke Rust guard — REJECTED: issue055 fix (3) explicitly routes target-type validation through the declared-rules path, not a new guard (D0105 direction). (iii) force 'analysis must become a spike' as a commit control — IMPOSSIBLE: un-gatable at commit (a conversation is invisible to a gate); stays reminder-enforced, exactly as issue059 left D0106's parse-first part. CHOSEN — the declared warning rule on the visible side + the existing reminder on the conversational side: the honest, precedented decomposition. CRITERIA: enforceable-not-guidance x declared-not-bespoke x honest-scope x trust-before-gating.";
        :>> consequences = "researchSpikeCharterRule runs in `keel rules` (warning-level; 0 violations on the live repo — every existing research spike already charters to a legitimate governing source). The evaluator gains a reusable whereKind scope + charterTargetType predicate. Guard count UNCHANGED (15) — this is not a new blocking guard. This is PART 2 of proveProcessDeployment (issue061); part 1 (dogfooding the business-architecture skill on a real design session) remains, so issue061 stays OPEN — not overclaimed. Governs the rule addition (D0070 keystone; this Decision carries #ProspectiveChange). doc-sync: CLAUDE.md Sec 5 (declarative-controls paragraph). Acceptance verdict/provenance are the discrete d0111Accept TestResult (not prose, D0058).";
    }

    verification d0111Accept : Test { :>> id = "10a1b0c1-4111-42a3-bb5c-516273849711"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of D0111 (declare researchSpikeCharterRule, warning-level) via the explicit choice 'Warning-level first' in response to the CHANGE proposal on 2026-07-09 — accepting the change and selecting the warning-not-gating level."; }
    part d0111AcceptR1 : TestResult { :>> id = "10a1b0c1-4111-42a3-cb5c-516273849711"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "633b689"; :>> judgedAt = "2026-07-09"; :>> judgedBy = "wweatherholtz"; }
}
