// =====================================================================
// ProjectDeliveryS87 — Sprint 87: reportTrends (D0087 Stage 2). Git-derived TREND sparklines for the
// report scorecards: `sysmlv2 report <name> --trend` recomputes each report's headline metric at ~12
// recent commits via a throwaway git worktree (full pipeline, incl. tier coverage) and renders a
// sparkline with first->last delta. Computed from git; never a stored metric history.
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049); chartered by D0087 (Stage 2 of the accepted report layer).
// =====================================================================
package ProjectDeliveryS87 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from reportTrendsStory to d0087;

    part reportTrendsStory : Story {
        :>> id = "63940516-2c7d-4e8f-b087-51627384b801";
        :>> title = "Sprint 87: reportTrends (3 pts) — git-derived trend sparklines for the report scorecards (`report <name> --trend`); full-pipeline tier-coverage trend via worktree-per-commit (D0087 Stage 2)";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p2;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS87 {
        action storyReportTrends;
        verification storyReportTrendsDoD : Test { :>> id = "63940516-2c7d-4e8f-b087-51627384b802"; :>> method = VerificationMethod::test; :>> procedureText = "`sysmlv2 report <name> --trend` (and `--html --trend`) adds a git-derived time-series for each report's headline metric (assurance -> verification coverage %, traceability -> requirements verified %, quality-debt -> supersede volatility, flow -> throughput). Implementation (view.rs): sampled_commits (recent ~12 touching .tracking/.engine, chronological); trend_series recomputes headline_at(root,name) at each commit via a THROWAWAY git worktree (git worktree add --detach -> run the FULL existing pipeline incl. compute_coverage tier coverage -> worktree remove), tolerating unparseable historical commits (skip); rendered as a unicode sparkline + first->last delta in the HTML scorecard. COMPUTED FROM GIT, never a stored metric history (compute-don't-store, S2.1/D0018). VERIFY (live): report assurance --trend -> 'Verification coverage %' series over 12 commits showing 73->...->100 (the assurance milestone trajectory); clippy --workspace --all-targets -D warnings + cargo test (38 lib, +headline_labels) + rust validate + guard all (9) green. No new decision/process (D0087 already covers Stage 2)."; }
        part storyReportTrendsDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b087-51627384b803"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c767f0d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    }

    verification reportTrendsRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b087-51627384b804"; :>> title = "Sprint 87 refine gate (DoR)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: D0087 Stage 2 — the human chose 'Build it now, full incl. tier-coverage trend'. Approach accepted: recompute the headline metric (incl. the heavy tier-coverage % requiring the full orient/coverage pipeline) at sampled recent commits via a throwaway git worktree, reusing every existing computation unchanged at that commit (no risky historical re-parse of in-memory blobs). estimatedPoints=3. #CharteredBy d0087. DoD accepted."; }
    part reportTrendsRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b087-51627384b805"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c767f0d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification reportTrendsStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b087-51627384b806"; :>> title = "Sprint 87 standup gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Approach: view.rs — git_out helper; sampled_commits; headline_at(root,name) (assurance/traceability run the full coverage pipeline at root; quality-debt = supersede count; flow = delivery-sprint count); trend_series = worktree add/remove per commit calling headline_at against the worktree path; trend_json attached to report()/report_html() behind a --trend flag; sparkline (unicode blocks) + delta in REPORT_TEMPLATE. main.rs cmd_report --trend. Watch: clippy str-slicing (use chars().take), worktree cleanup (remove+prune), per-commit cost. No blockers."; }
    part reportTrendsStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b087-51627384b807"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c767f0d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification reportTrendsImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b087-51627384b808"; :>> title = "Sprint 87 implement gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented sampled_commits + git_out + headline_at + trend_series (worktree-per-commit) + trend_json + --trend on report()/report_html() + sparkline in REPORT_TEMPLATE + cmd_report --trend; doc-sync (report skill + usage.md). VERIFY (live): `report assurance --trend` -> Verification coverage % over 12 commits = 73->73->73->73->98->...->100 (the true tier-coverage trajectory, the milestone jump visible), ~3.5min for 18 (trimmed to 12 for default). clippy --workspace --all-targets -D warnings green; cargo test 38 lib pass (incl headline_labels_are_stable); rust validate clean; guard all (9) PASS. No new decision/process-def (Stage 2 under D0087) so no keystone."; }
    part reportTrendsImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b087-51627384b809"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c767f0d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification reportTrendsReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b087-51627384b80a"; :>> title = "Sprint 87 review gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The worktree-per-commit design reuses the ENTIRE existing pipeline (orient/coverage/critique) unchanged at each historical commit, so the tier-coverage trend is the real metric, not an approximation — and it sidesteps the historical-parse-compatibility risk of rebuilding Models from old blobs (a recent commit either checks out + parses or is skipped). Trends are COMPUTED FROM GIT, never stored (consistent with §2.1/D0018); --trend is opt-in so the routine report stays fast. Honest cost: ~12 worktree builds = minutes; documented in the skill + usage as slow/opt-in. The 73->100 trajectory validates the whole assurance-milestone arc end-to-end. doc-sync done."; }
    part reportTrendsReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b087-51627384b80b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c767f0d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification reportTrendsCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b087-51627384b80c"; :>> title = "Sprint 87 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyReportTrends DoD recorded pass; --trend live for all four reports with a real git-derived sparkline (tier-coverage trajectory verified 73->100); clippy + cargo test (38) + rust validate + guard all (9) green; doc-sync done. Self-evidencing (D0051). D0087 is now fully delivered (Stage 1 scorecards + Stage 2 trends)."; }
    part reportTrendsCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b087-51627384b80d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c767f0d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification reportTrendsRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b087-51627384b80e"; :>> title = "Sprint 87 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: the worktree-per-commit insight made 'full pipeline at a past commit' a SMALL change — reuse the whole existing computation against a checked-out tree rather than refactoring orient/coverage to read git blobs (which would have been large + risky); this also dodged historical parse-incompatibility. The 73->98->100 coverage trajectory is a satisfying end-to-end validation that the assurance milestone actually moved the needle. Opt-in --trend keeps the routine report fast. CAUGHT (avoidable-issue candidate): the trend at 18 commits took ~3.6 min — an avoidable UX cost I reduced to 12 commits, but a per-commit full-pipeline trend is inherently minutes-scale; documented as slow/opt-in rather than hidden. Lesson: a git-history-recomputed metric is powerful but O(commits x full-pipeline) — sample modestly, make it opt-in, and label the cost; never put it on a hot path. Estimate 3 pts held. D0087 fully delivered; the report layer now shows both state and trajectory."; }
    part reportTrendsRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b087-51627384b80f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c767f0d"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
}
