// =====================================================================
// ProjectPanelRigor — the five-critic panel on the ARCHITECTURE OF RIGOR, convened 2026-09-01 at the
// human's instruction: "why are we losing rigor? it feels like analyses of facts constantly slip, and
// while we do stop AI agents from going rogue sometimes, we lack the authority needed to really
// strongly drive custom processes. what's lacking?"
//
// METHOD (architectural-critique, D0046): each critic takes one lens, states the claim the engine
// makes about itself, and hunts REFUTING evidence in the source. Fifteen functions were read at
// file:line - the ones that decide what is enforced, what counts as done, who is acting, and what a
// finding becomes. Every number below was measured in this tree today, not recalled.
//
// THE FIFTEEN, by what they decide:
//   gates      hook_stop, hook_post_edit, hook_pre_write (main.rs) - the in-loop tiers
//   verdicts   guards::run_all + GuardReport::print - which guards run, how results are read
//   substance  guards::issues, guards::decision_scaffolding, guards::retro_backlog - what "checked" means
//   identity   guards::ownership + actor::resolve - who is acting
//   authority  guards::process_change / is_enforcement_surface, adherence.rs - the keystone and its audit
//   truth      orient::done_names, view::compute_issue_resolution - what done and open mean
//   writes     write::sanitize_field + the record_* paths, write::record_obligation - can a record lie
//   rules      view::rule_violations_opt - the declared-rule path (D0105)
//
// INDEPENDENCE, stated as every panel here states it: five framings of one model are five framings,
// not five judges. Every critique is critiquedBy = aiModel and PASSES nothing by itself. What makes
// them worth more than opinion is that each names a predicate at file:line the reader can re-run.
//
// THE VERDICT THE FIVE CONVERGED ON, and it is one sentence: keel enforces FORM rigorously and
// SUBSTANCE weakly, and the enforcement it has is advisory by failure mode. Everything below is a
// specific instance of that sentence.
// =====================================================================
package ProjectPanelRigor {
    private import EngineElement::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // ── Critic 1: enforcement topology — WHERE does authority actually live? ────────────────────

    verification rigorTopologyCritique : Test {
        :>> id = "d1a50001-0901-4e01-9c01-000000000101";
        :>> title = "Critique ceHookDispatch — correctness lens: the turn gate is advisory by failure mode";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::correctness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::Critical;
        :>> procedureText = "CLAIM ATTACKED: 'the Stop hook blocks the turn while the model is dishonest' (CLAUDE.md section 5). REFUTING EVIDENCE, from `keel hardening`'s own enforcementPoints table: of 12 enforcement points, TWO are fail-closed - the pre-write hook and the pre-commit git hook. The Stop hook, the post-edit hook, the subagent hook and the CI gate are all fail-OPEN: on error they print a loud warning and ALLOW, and on harness timeout every hook resolves to ALLOW. So the control the human is relying on to stop a rogue turn holds exactly when nothing goes wrong, and evaporates under the conditions - a slow guard, a locked binary, a crashed process - where a rogue turn is most likely to be happening. This is the mechanical answer to 'we stop AI agents from going rogue SOMETIMES': the gate is real, and its failure mode is permission. WHAT IS AND IS NOT FIXABLE: the harness's timeout-resolves-to-allow is the harness's behaviour and cannot be changed from inside keel; what CAN change is the hook's own error path, which currently chooses to allow. A gate whose error path denies is a gate; one whose error path allows is a suggestion with a green light attached. Functions read: hook_stop (main.rs:1011-1035), hook_post_edit, hook_pre_write, and the enforcementPoints computation that reports them.";
    }
    part rigorTopologyCritiqueR1 : TestResult { :>> id = "d1a50002-0901-4e02-9c02-000000000102"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "a52d81b"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "claudeOpus5"; }
    #Verify dependency from rigorTopologyCritique to ceHookDispatch;

    // The one non-Claude verdict on this panel, recorded separately as the independence rule requires:
    // `keel hardening`'s enforcementPoints table is COMPUTED from the live hook configuration, and it
    // classified the Stop hook fail-open before any critic wrote a word. The critic above interprets
    // that number; the number itself is the tool's.
    verification rigorTopologyToolCritique : Test {
        :>> id = "d1a5000b-0901-4e0b-9c0b-00000000010b";
        :>> title = "Critique ceHookDispatch — correctness lens, adjudicated by `keel hardening` (the panel's only tool verdict)";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::correctness;
        :>> critiquedBy = CriticKind::tool;
        :>> severity = Severity::Critical;
        :>> procedureText = "RUN: `keel hardening .` at a52d81b, before any change. The enforcementPoints table reported 12 points with failMode: 2 fail-closed (hook PreToolUse pre-write, pre-commit .githooks) and 10 fail-open or fail-open-announced, INCLUDING 'hook Stop (turn gate)' with error 'loud warning, allows' and timeout 'harness resolves to ALLOW'. The Stop hook's generated wrapper in .claude/settings.json read: `[ -n \"$K\" ] || { echo '[keel] keel binary NOT FOUND - this gate did not run. ...' >&2; exit 0; }` - exit 0 is ALLOW. That is the tool's finding and it is not an interpretation: the wrapper's exit code on a missing binary was zero. RE-RUN after the D0279 change: 3 fail-closed of 12, the Stop row now reading 'missing binary BLOCKS the turn'. The tool's verdict moved because the configuration moved, which is what makes it a verdict. CAVEAT the tool cannot state for itself and the panel states for it: failMode in that table is classified by keyword from each row's own prose (hardening.rs:522-531), so the table is a description kept true by hand, not a measurement of behaviour - the wrapper text quoted above is the measurement.";
    }
    part rigorTopologyToolCritiqueR1 : TestResult { :>> id = "d1a5000c-0901-4e0c-9c0c-00000000010c"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "a52d81b"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "claudeOpus5"; }
    #Verify dependency from rigorTopologyToolCritique to ceHookDispatch;

    // ── Critic 2: substance vs form — do controls test what they are TRUSTED to test? ────────────

    verification rigorSubstanceCritique : Test {
        :>> id = "d1a50003-0901-4e03-9c03-000000000103";
        :>> title = "Critique ceGuards — testability lens: the majority of governance guards check that a thing EXISTS, not that it is RIGHT";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::testability;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::High;
        :>> procedureText = "CLAIM ATTACKED: 'an Issue must be triaged; resolution is then computed' (CLAUDE.md section 3), and the analogous claims for retro findings and Decision delivery. REFUTING EVIDENCE, three predicates read at source. `issues` (guards.rs:970-986) sources the untriaged set from issuesTriagedRule and reports only Issues with NO #Resolves edge - an edge to an unrelated task is indistinguishable from a correct triage, observed when issue323 was filed against the CLI consolidation task and passed. `decision_scaffolding` requires SOME inbound tracked-item edge on an accepted #ProspectiveChange Decision - one edge satisfies a Decision with five clauses, observed when D0252 clause A sat unbuilt nine days behind clause B's edge. `retro_backlog` (guards.rs:1601-1628) fires only when the retro text contains the literal tokens AVOIDABLE-ISSUE or LESSON:, and is then satisfied by names_tracked_item, which returns true if ANY `dc`-prefixed name appears anywhere in the file - and every sprint file names the task it delivered. So a retro that says FINDING instead of LESSON is never examined, and one that says LESSON passes because it mentions the work it just did. THE PATTERN: existence is cheap to check and cheap to satisfy, and the three controls that stand between a finding and its loss all check existence. This is precisely why analyses of facts slip - the fact is recorded, the control confirms SOMETHING is there, and nobody asks whether the something is the right thing. NOT REFUTED, for fairness: the identity/marker/edge-endpoint family (identity-present, identity-well-formed, edge-endpoints, marker-vocabulary, duplicate-identity) checks FORM, and form is what it is trusted to check; those hold.";
    }
    part rigorSubstanceCritiqueR1 : TestResult { :>> id = "d1a50004-0901-4e04-9c04-000000000104"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "a52d81b"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "claudeOpus5"; }
    #Verify dependency from rigorSubstanceCritique to ceGuards;

    // ── Critic 3: fact lifecycle — WHERE does a fact leave the model? ────────────────────────────

    verification rigorFactLifecycleCritique : Test {
        :>> id = "d1a50005-0901-4e05-9c05-000000000105";
        :>> title = "Critique ceWriteApi — completeness lens: every analysis lands in a prose field, and no control can read prose";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::completeness;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::High;
        :>> procedureText = "CLAIM ATTACKED: 'text is truth; author only irreducible facts - atomic items, typed edges, test results, recorded judgments' (CLAUDE.md invariant 1). REFUTING EVIDENCE: the model's typed surface is items, edges and enums, and everything ANALYTICAL about them is a single String field - Decision.rationale, Decision.consequences, Issue.description, Test.procedureText, ProcessStep.actionText. A Decision's CLAUSES are prose (no view can list Decisions by clause, which is why decision_scaffolding cannot see partial delivery). A retro's FINDINGS are prose (retro_backlog has to grep for vocabulary). An Issue's RESOLVER FIT is prose (the DoD may or may not name the issue). A fix's ARMING is prose (the RAN: comment). The write API sanitizes these fields for SYNTAX - sanitize_field maps quotes to apostrophes and collapses whitespace, and the one path that skipped it broke the parse today (issue325) - but nothing types their CONTENT, so the moment an analysis is written it becomes invisible to every control at once. The human's phrase was exact: analyses of facts slip. They do not slip out of the repository - they are all there, committed - they slip out of the ENFORCEABLE model into fields that only a reader can check. Counted today: three control defects, two migration-path findings and one keystone-coverage gap were all written into retro procedureText and all went unrouted. The retro guard passed each one. WHAT WOULD NOT SLIP: a finding recorded as an Issue (typed, resolver-guarded, computed open/closed). The gap is that nothing forces the conversion, and the ceremony designed to (D0131) is enforced by a vocabulary grep.";
    }
    part rigorFactLifecycleCritiqueR1 : TestResult { :>> id = "d1a50006-0901-4e06-9c06-000000000106"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "a52d81b"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "claudeOpus5"; }
    #Verify dependency from rigorFactLifecycleCritique to ceWriteApi;

    // ── Critic 4: process authority — can a process DEFINITION drive behaviour? ──────────────────

    verification rigorProcessAuthorityCritique : Test {
        :>> id = "d1a50007-0901-4e07-9c07-000000000107";
        :>> title = "Critique ceGuards — feasibility lens: a process is a document with an optional guard, not a machine";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::feasibility;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::High;
        :>> procedureText = "CLAIM ATTACKED: 'every recurring task done without a skill leaks process knowledge into conversation history, where it cannot be enforced' - implying that a process WITH a skill IS enforced. REFUTING EVIDENCE, from `keel hardening` processEnforcement: 37 processes; 16 assert at least one guard; 10 are declared unenforceable as human judgments; 11 are UNDECLARED - no guard and no stated reason. Of the 11, five were authored in the last two days (controlDefect, controlPropriety, decisionSurfacing, execSummary, githubIntake), so process authoring is outrunning enforcement and nothing gates that ratio - it is an indicator by design (invariant 7), which is correct, and which means the ONLY thing that turns a new process into an enforced one is the author remembering to. WORSE THAN THE COUNT: 'enforced' means the process asserts a guard SOMEWHERE, not that each ProcessStep has a check. A process is six actionText paragraphs the AI reads; the guard, where one exists, checks one property of the RESULT. There is no binding from a step to a predicate, so a process cannot refuse to advance when a step was skipped - `keel advance --to` refuses only on the sprint CEREMONY steps, which are the one process (agile-workflow) that has per-step gates. This is the shape of 'we lack the authority to drive custom processes': the custom process exists, is well-written, ships as a unit, and is followed exactly as far as the reader chooses. The process-enforcement contract's own comment names the mechanism it is missing - 'the constraint IS checkable and no guard was written' - and cannot make anyone write one.";
    }
    part rigorProcessAuthorityCritiqueR1 : TestResult { :>> id = "d1a50008-0901-4e08-9c08-000000000108"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "a52d81b"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "claudeOpus5"; }
    #Verify dependency from rigorProcessAuthorityCritique to ceGuards;

    // ── Critic 5: self-reference — can the engine SEE its own controls failing? ──────────────────

    verification rigorSelfReferenceCritique : Test {
        :>> id = "d1a50009-0901-4e09-9c09-000000000109";
        :>> title = "Critique ceActorProvenance — consistency lens: the controls that watch the controls are the weakest ones, and identity does not reach the hooks";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::critique;
        :>> lens = CritiqueLens::consistency;
        :>> critiquedBy = CriticKind::aiModel;
        :>> severity = Severity::High;
        :>> procedureText = "CLAIM ATTACKED: 'corrections become permanent guards; manual vigilance is not a control' (D0047). REFUTING EVIDENCE in two parts. FIRST, the meta-controls are the weakest controls. The guards whose job is to catch OTHER things slipping - retro_backlog (findings become items), decision_scaffolding (promises get built), issues (defects get resolvers), decision_follow_through - are exactly the existence-checkers Critic 2 named. The controls on identity and edges are strong; the controls on whether the process is closing its own loops are vocabulary greps. Until today there was no registry of known control defects at all (D0278 added one), so a guard known to under-report kept issuing unqualified greens. SECOND, and found while checking rather than assumed: actor::resolve (actor.rs:97-112) correctly prefers explicit > KEEL_ACTOR > the machine binding, and the ownership guard correctly calls it (guards.rs:2498) - but the Stop hook is spawned by the harness with ITS environment, in which KEEL_ACTOR was never set, so resolve falls through to the binding. This machine is bound to claudeFable5; this session is claudeOpus5. The hook therefore attributed 22 edits to the wrong actor and flagged 12 items the session owns. The guard is correct; the SESSION'S IDENTITY HAS NO PATH TO THE HOOKS. That corrects issue332 as recorded, which blamed the guard's predicate and gave 'guard source under the keystone lock' as the reason for carrying it - the fix is not in guard source at all. A session-scoped actor binding that resolve reads before the machine's would close it, and it is not a keystone change. Functions read: actor::resolve, guards::ownership, hook_stop's environment, orient::done_names (which is sound - done requires a pass whose judgedAgainst SHA resolves in git, and that is the one place where a claim cannot be made without a receipt).";
    }
    part rigorSelfReferenceCritiqueR1 : TestResult { :>> id = "d1a5000a-0901-4e0a-9c0a-00000000010a"; :>> outcome = VerdictKind::fail; :>> judgedAgainst = "a52d81b"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "claudeOpus5"; }
    #Verify dependency from rigorSelfReferenceCritique to ceActorProvenance;
}
