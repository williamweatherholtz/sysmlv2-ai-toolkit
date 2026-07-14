// ProjectDeliveryS207 — Sprint 207: docSyncGuard (D0113) — convert doc-sync from VIGILANCE to a CONTROL.
// A warning-level `doc-sync` guard (17th): a staged schema/process/workflow change with no co-committed
// doc update is flagged. From the process-audit ("are our processes respected?"). CHANGE (new guard)
// governed by D0113. 2 pt. AUTONOMOUS (D0049).
package ProjectDeliveryS207 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from docSyncGuardStory to d0113;

    part docSyncGuardStory : Story {
        :>> id = "63940516-2c7d-4e8f-c207-51627384c001";
        :>> title = "Sprint 207: docSyncGuard (2 pt) - doc-sync as a warning-level control (D0113, from the process-audit)";
        :>> createdAt = "2026-07-14"; :>> createdBy = "wweatherholtz"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p2; :>> owner = "claudeOpus"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS207 {
        action storyDocSyncGuard;
        verification storyDocSyncGuardDoD : Test { :>> id = "63940516-2c7d-4e8f-c207-51627384c002"; :>> method = VerificationMethod::test; :>> procedureText = "Delivers D0113 (accepted 2026-07-14). Converts the doc-sync discipline from vigilance to a CONTROL. guards.rs `doc_sync` (+ pure core doc_sync_warnings + is_doc_governed_def/is_doc_file): a staged DEFINITIONAL change (.engine/schema|processes|workflows/) with NO co-committed doc update (CLAUDE.md / .engine/docs/ / any README) emits a WARNING. Reuses the process_change staged_files git mechanism. WARNING-level (D0102 pattern) — nudges, never blocks; promotable to hard once low-noise. GUARD_NAMES 16->17 (second warning-only member, with decision-requirement-link); hard-blocking unchanged at 15. Scope deliberately NARROW (definitional trio; tool .rs + skills excluded) to stay low-noise. D0047 control: unit test doc_sync_flags_undocumented_definitional_change (governed change + no doc = warn; + a doc = clean; non-definitional = clean). doc-sync: CLAUDE.md §5 (17 guards). VERIFY: `keel guard doc-sync` present + PASS on the live repo; clippy --workspace --all-targets clean; cargo test (83) green; keel validate + guard all (17) + check-engine green."; }
        part storyDocSyncGuardDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c207-51627384c003"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e0f1361"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    }

    #Verify dependency from storyDocSyncGuardDoD to d0113;

    verification dsgRefineGate : Test { :>> id = "63940516-2c7d-4e8f-c207-51627384c004"; :>> title = "Sprint 207 refine gate (DoR)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0113 accepted (human selected 'doc-sync guard' from the audit). Scope = a warning-level heuristic on the git-staged definitional trio; reuses process_change's mechanism. CHANGE (new guard) governed by D0113 #ProspectiveChange. estimatedPoints=2. #CharteredBy d0113. DoR met."; }
    part dsgRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c207-51627384c005"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e0f1361"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification dsgStandupGate : Test { :>> id = "63940516-2c7d-4e8f-c207-51627384c006"; :>> title = "Sprint 207 standup gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: is_doc_governed_def/is_doc_file + doc_sync_warnings pure core + doc_sync guard (reuse staged_files); GUARD_NAMES 17 + run_one; unit test; doc-sync CLAUDE.md §5. No blockers."; }
    part dsgStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c207-51627384c007"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e0f1361"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification dsgImplementGate : Test { :>> id = "63940516-2c7d-4e8f-c207-51627384c008"; :>> title = "Sprint 207 implement gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "doc_sync guard + test + GUARD_NAMES 17 + doc-sync landed. `keel guard doc-sync` PASS. clippy --workspace --all-targets clean; cargo test (83) green; keel validate + guard (17) + check-engine green."; }
    part dsgImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c207-51627384c009"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e0f1361"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification dsgReviewGate : Test { :>> id = "63940516-2c7d-4e8f-c207-51627384c00a"; :>> title = "Sprint 207 review gate"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Reviewed. This directly answers the process-audit: the #1 vigilance gap (doc-sync, a HIGH historical drift) is now a visible control, not a convention. Warning-first is the honest posture for a heuristic that can't KNOW a doc claim is stale (only that a definitional change shipped with no doc touch) — it nudges without false-blocking, promotable per D0102. Narrow scope keeps it low-noise. Governed by D0113 #ProspectiveChange. Honest residual: migration + parse-first remain vigilance (noted, not yet controlled). Clean."; }
    part dsgReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c207-51627384c00b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e0f1361"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification dsgCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-c207-51627384c00c"; :>> title = "Sprint 207 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): DoD pass; doc-sync is the 17th enforced guard (warning-level), converting the audit's #1 vigilance gap into a control. clippy + cargo test + keel validate + guard (17) + check-engine green. Self-evidencing (D0051). NEXT: integrate the keel-viewer handoff doc + create items; migration/parse-first controls remain optional follow-ons."; }
    part dsgCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c207-51627384c00d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e0f1361"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
    verification dsgRetroGate : Test { :>> id = "63940516-2c7d-4e8f-c207-51627384c00e"; :>> title = "Sprint 207 retro gate (autonomous, D0049)"; :>> createdAt = "2026-07-14"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the process-audit didn't end in talk — it produced a permanent CONTROL. The human's question ('are processes respected?') got an honest evidence-based answer (enforced=yes, vigilance=no, with THIS session's own drift as proof), and the highest-value vigilance gap (doc-sync) is now guarded. Reused process_change's mechanism (small, safe). Ran the AUTHORITATIVE clippy first (the a642269 lesson holds). AVOIDABLE-ISSUE SCAN: none new. LESSON: convert vigilance->control is the durable answer to 'is X respected?', per D0047. Estimate 2 pt held. NEXT: migration + parse-first are the remaining vigilance gaps if the human wants them controlled."; }
    part dsgRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-c207-51627384c00f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e0f1361"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "claudeOpus"; }
}
