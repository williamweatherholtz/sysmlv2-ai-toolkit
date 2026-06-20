// =====================================================================
// ProjectDeliveryS54 — Sprint 54: portGitTraversal (D0074/D0075 migration M2.2c).
// Story: port the git-traversal algorithmic views — governing-version + reprocess-candidates
//        (byte-parity vs query.py) + suspect (= orient's authoritative answer, a D0050 SUPERSET
//        of query.py's; parity-exception recorded as D0076). COMPLETES the portAlgorithmicViews
//        umbrella (M2.2). Precedes M3 (port guards) + M4 (retire query.py, close issue012).
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049) except D0076 (human-accepted CHANGE).
// =====================================================================
package ProjectDeliveryS54 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from portGitTraversalStory to d0074;

    part portGitTraversalStory : Story {
        :>> id = "63940516-2c7d-4e8f-b054-516273849701";
        :>> title = "Sprint 54: portGitTraversal (5 pts) — port governing-version/reprocess/suspect to Rust; completes portAlgorithmicViews; D0074/D0075 M2.2c";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS54 {
        action storyPortGitTraversal;
        verification storyPortGitTraversalDoD : Test { :>> id = "63940516-2c7d-4e8f-b054-516273849702"; :>> method = VerificationMethod::test; :>> procedureText = "portGitTraversalDoD: `sysmlv2 governing-version <item>` + `sysmlv2 reprocess-candidates` byte-identical to query.py (git-ancestry resolver: item intro-commit, process-def change-commits, is-ancestor, process-change Decisions in-force-vs-after, safety-reprocess). `sysmlv2 suspect` = orient's authoritative suspect (criterion-change + D0050 deliverable drift) — a SUPERSET of query.py suspect, NOT byte-parity, by D0076. Unit tests for the resolver. clippy --workspace --all-targets -D warnings + cargo test green. Completes the portAlgorithmicViews umbrella."; }
        part storyPortGitTraversalDoDR1 : TestResult { :>> id = "238228f6-fed6-4722-ba7f-a7d93162a106"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "967f018"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
    }

    verification portGitTraversalRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b054-516273849703"; :>> title = "Sprint 54 refine gate (DoR)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: M2.2c completes the portAlgorithmicViews umbrella. Port governing-version + reprocess-candidates (byte-parity vs query.py) and suspect (= orient authority; D0050 superset; parity-exception is a recorded CHANGE = D0076, human-accepted via the M2.2c suspect-semantics dialogue). governing-version is the bulk (git plumbing + process-change-decision resolver). estimatedPoints=5 (gv 3 + reprocess 1 + suspect 0.5 + D0076 0.5). #CharteredBy to d0074. DoD accepted."; }
    part portGitTraversalRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b054-516273849704"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "967f018"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification portGitTraversalStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b054-516273849705"; :>> title = "Sprint 54 standup gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: new src/govern.rs — git plumbing (git_lines, item_intro_commit `log -S --reverse`, def_change_commits, is_ancestor `merge-base --is-ancestor`; reuse orient::git_sha_valid made pub(crate)); proc_change_decisions (scan .engine/decisions #Prospective/#SafetyChange markers + acceptance judgedAgainst); govern_resolve(item)->struct feeding both governing_version (Json) and reprocess_candidates. suspect subcommand = orient::compute().suspect. D0076 records the suspect parity-exception (accepted via the suspect-semantics question). Wire governing-version/reprocess-candidates/suspect in main.rs. Verify: byte-parity governing-version (several items) + reprocess vs query.py; suspect == orient. .engine kernel validators run for the new Decision. No blockers."; }
    part portGitTraversalStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b054-516273849706"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "967f018"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification portGitTraversalImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b054-516273849707"; :>> title = "Sprint 54 implement gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented src/govern.rs (git plumbing + proc-change-decision resolver + govern_resolve) + Json::Null; orient::git_sha_valid + algo::{is_word,is_space,story_names} made pub(crate) for reuse; governing-version/reprocess-candidates/suspect wired in main.rs. PARITY: `sysmlv2 governing-version` byte-identical to query.py for portOrphansAuditStory/portAttestationCoverageStory/markerRetentionStory; `sysmlv2 reprocess-candidates` byte-identical ([]); `sysmlv2 suspect` == orient's authoritative suspect (13; D0050 superset of query.py's [], per D0076). D0076 recorded + accepted (validate_acceptance_events PASS). cargo test --workspace all pass (incl. 2 govern unit tests); clippy --workspace --all-targets -D warnings green; rust validate + validate_actors/ceremony/sprint_coverage/acceptance_events/parity_check + kernel validate_instances 85/85 (incl D0076) all PASS. usage.md doc-synced (governing-version/reprocess/suspect rows)."; }
    part portGitTraversalImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b054-516273849708"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "967f018"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification portGitTraversalReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b054-516273849709"; :>> title = "Sprint 54 review gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. govern.rs reuses orient::git_sha_valid + algo::{is_word,is_space,story_names} (no duplication); a single govern_resolve feeds BOTH governing_version and reprocess_candidates (query.py calls governing_version per story — same shape). query.py's unused process_change_decisions_full machinery (governed_defs via decision_intro_commit/commit_files) was correctly OMITTED — the resolver only needs each Decision's effective_commit + retroactivity. suspect-as-orient (D0076) preserves a SINGLE source of truth for suspect rather than a second divergent implementation. Byte-parity proven for the two parity-bound views; the suspect superset is documented + accepted. COMPLETES the portAlgorithmicViews umbrella; next is M3 (port the forward guards to Rust) then M4 (retire query.py, close issue012)."; }
    part portGitTraversalReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b054-51627384970a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "967f018"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification portGitTraversalCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b054-51627384970b"; :>> title = "Sprint 54 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyPortGitTraversal + backlog portAlgorithmicViews (the umbrella) DoDs recorded pass; cargo test + clippy + rust validate + all kernel-free validators + kernel validate_instances 85/85 green; governing-version + reprocess byte-parity vs query.py verified, suspect = orient (D0076). D0076 is human-accepted (its acceptance event passes); the rest is self-evidencing (D0051: tested port + verified parity). portAlgorithmicViews umbrella COMPLETE."; }
    part portGitTraversalCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b054-51627384970c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "967f018"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification portGitTraversalRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b054-51627384970d"; :>> title = "Sprint 54 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: one govern_resolve served both views (DRY), and reusing orient/algo helpers + the json primitive from M2.2b meant the hard view (governing-version) was mostly git plumbing. Omitting query.py's unused governed_defs machinery kept scope at 5 pts. The suspect semantic conflict was surfaced to the human (not force-fit) and resolved by D0076 — the triage flag-don't-force-fit rule working. CAUGHT: piped a kernel validator into `tail` and it HUNG (the documented CLAUDE.md §6 hazard); swept stale kernels + re-ran with file redirect. LESSON: kernel runs MUST redirect to a file, never pipe — already a documented guard (CLAUDE.md §6), so no avoidable issue warrants a new tracked item. Estimate 5 pts held. M2.2 (portAlgorithmicViews) COMPLETE. Next: M3 (port the forward guards — charter/ceremony/coverage/acceptance/process-change — to Rust), then M4 (retire query.py, close issue012)."; }
    part portGitTraversalRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b054-51627384970e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "967f018"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
}
