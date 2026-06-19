// =====================================================================
// ProjectDeliveryS51 — Sprint 51: portStructuralViews (D0074/D0075 migration M2.1).
// Story: subsume query.py's STRUCTURAL views as .view.toml declarations — add project.fields to
//        the executor (output selected attributes), then issues/decisions/process-changes as TOML
//        (parity-verified vs query.py). orphans stays algorithmic (per-type rules) -> M2.2.
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS51 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from portStructuralViewsStory to d0075;

    part portStructuralViewsStory : Story {
        :>> id = "63940516-2c7d-4e8f-b051-516273849501";
        :>> title = "Sprint 51: portStructuralViews (3 pts) — subsume issues/decisions/process-changes as TOML views; D0074/D0075 M2.1";
        :>> createdAt = "2026-06-19";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS51 {
        action storyPortStructuralViews;
        verification storyPortStructuralViewsDoD : Test { :>> id = "63940516-2c7d-4e8f-b051-516273849502"; :>> method = VerificationMethod::test; :>> procedureText = "portStructuralViewsDoD (pass): added project.fields to the view executor (emit selected item attributes; special `marker`). Subsumed as .view.toml: issues (type=Issue, fields title/relatedTask/discoveredInField), decisions (type=Decision, fields title/status), process-changes (sprint50). PARITY: `sysmlv2 view issues` issue-name set == `query.py issues` (diff identical); view decisions=75, view issues=24. orphans NOT subsumed — its per-type required-edge rules are logic (-> M2.2 algorithmic). usage.md updated to prefer the TOML views (query.py equivalents superseded, removed at M4). cargo test --workspace + clippy --workspace --all-targets -D warnings green."; }
        part storyPortStructuralViewsDoDR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b051-5162738495a2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a7b70db"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
    }

    verification portStructuralViewsRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b051-516273849503"; :>> title = "Sprint 51 refine gate (DoR)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: M2.1 (approved split: subsume structural as TOML). Found the executor outputs name+type only, so subsuming field-rendering views (issues/decisions) needs project.fields first. orphans encodes per-type required-edge RULES -> classified algorithmic (M2.2), not a single filter. Scope: project.fields + issues/decisions/process-changes TOML + parity. Command REMOVAL deferred to M4 (deprecate in docs now). estimatedPoints=3. DoD accepted."; }
    part portStructuralViewsRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b051-516273849504"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a7b70db"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification portStructuralViewsStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b051-516273849505"; :>> title = "Sprint 51 standup gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: Project.fields + emit_json renders a fields object from item attrs (+ marker); issues.view.toml + decisions.view.toml; parity-check view-issue-names vs query.py; usage.md doc-sync. orphans -> M2.2. No blockers."; }
    part portStructuralViewsStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b051-516273849506"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a7b70db"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification portStructuralViewsImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b051-516273849507"; :>> title = "Sprint 51 implement gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "project.fields implemented in the executor. issues.view.toml + decisions.view.toml created. PARITY: diff of view-issue-names vs query.py issue-names = identical. view decisions=75, issues=24, fields render (e.g. issue024 title/relatedTask/discoveredInField). clippy --workspace --all-targets -D warnings + cargo test view green. usage.md updated. portStructuralViews backlog DoD recorded pass. orphans deferred to M2.2 (algorithmic)."; }
    part portStructuralViewsImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b051-516273849508"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a7b70db"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification portStructuralViewsReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b051-516273849509"; :>> title = "Sprint 51 review gate"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed: project.fields makes the TOML views full replacements (set + key fields), parity-verified before deprecating the query.py path (no premature removal — that's M4). orphans honestly kept algorithmic (its per-type rules aren't a single filter) — not force-fit into TOML. The subsume-vs-port boundary (D0075) is being applied with judgment, not mechanically. Next: M2.2 (port the algorithmic views — governing-version/reprocess/suspect/attestation/audit + orphans — to Rust)."; }
    part portStructuralViewsReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b051-51627384950a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a7b70db"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification portStructuralViewsCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b051-51627384950b"; :>> title = "Sprint 51 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): story + portStructuralViews DoD recorded pass; cargo test + clippy + validate_all green; parity verified. No human gate (D0051: tested executor feature + parity-checked TOML views, self-evidencing)."; }
    part portStructuralViewsCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b051-51627384950c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a7b70db"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }

    verification portStructuralViewsRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b051-51627384950d"; :>> title = "Sprint 51 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-19"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: parity-before-deprecate (diff view-output vs query.py) is the right migration safety — prove equivalence, then deprecate, remove later (M4). project.fields was the small missing piece that made TOML views full replacements. Honest classification of orphans as algorithmic avoided force-fitting it into a filter. No avoidable issue. Next: M2.2 (algorithmic ports — the git-traversal + computation views) — the larger remaining piece. Estimate 3 pts held."; }
    part portStructuralViewsRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b051-51627384950e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a7b70db"; :>> judgedAt = "2026-06-19"; :>> judgedBy = "claudeOpus"; }
}
