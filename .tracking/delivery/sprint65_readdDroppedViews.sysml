// =====================================================================
// ProjectDeliveryS65 — Sprint 65: readdDroppedViews (resolves issue025; D0074/D0077).
// Story: re-add the seven query.py view subcommands dropped at M4 — outstanding, item, trace
//        (incl. upstream/downstream), trace-need, workflows (Rust, src/queries.rs) + viewpoints
//        (TOML view). Resolver for the last open issue (issue025).
// Total: 5 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS65 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from readdDroppedViewsStory to d0077;

    part readdDroppedViewsStory : Story {
        :>> id = "63940516-2c7d-4e8f-b065-51627384a201";
        :>> title = "Sprint 65: readdDroppedViews (5 pts) — re-add the 7 dropped query.py views (Rust + TOML); resolves issue025";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 5;
    }

    action def DeliveryRunS65 {
        action storyReaddDroppedViews;
        verification storyReaddDroppedViewsDoD : Test { :>> id = "63940516-2c7d-4e8f-b065-51627384a202"; :>> method = VerificationMethod::test; :>> procedureText = "readdDroppedViewsDoD (resolves issue025): re-added the 7 dropped query.py subcommands. Rust (src/queries.rs): `sysmlv2 outstanding` (not-done tasks), `item <name>` (done/deps/DoD/results), `trace <name>` (transitive upstream + downstream — covers the dropped trace/upstream/downstream), `trace-need <need>` (satisfy/allocate closure), `workflows` (Kahn waves over workflow action defs). TOML: `sysmlv2 view viewpoints` (the viewpoint-registry listing). VERIFY (live): each returns sensible JSON — outstanding=[readdDroppedViews]; trace portGuardKeystone up=[ceremony,coverage] down=[retireQueryPy]; workflows = all 6 (Architecture/Business/ChangeMgmt/Delivery/Deploy/Operate) with waves; view viewpoints = 12. clippy --workspace -D warnings + cargo test (28) green; usage.md doc-synced."; }
        part storyReaddDroppedViewsDoDR1 : TestResult { :>> id = "b6d542c1-ee3b-491a-8386-64fc114e27ca"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bef381a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
    }

    verification readdDroppedViewsRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b065-51627384a203"; :>> title = "Sprint 65 refine gate (DoR)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: resolves issue025 — re-add the 7 dropped query.py views, each where it fits: Rust subcommands over the indexer DAG (outstanding/item/trace) + parser edges (trace-need) + workflow action defs (workflows); a TOML view for viewpoints. Parity bar = sensible output (query.py is gone, no byte-compare). estimatedPoints=5. #CharteredBy to d0077. DoD accepted."; }
    part readdDroppedViewsRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b065-51627384a204"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bef381a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification readdDroppedViewsStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b065-51627384a205"; :>> title = "Sprint 65 standup gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: new src/queries.rs (outstanding/item/trace via indexer + orient::done_names; trace-need via parser satisfy/allocate BFS; workflows via parser action defs + Kahn waves) wired through generic cmd_query0/cmd_query1 in main.rs; viewpoints.view.toml (select type=Viewpoint). downstream/upstream folded into trace. Verify each subcommand returns sensible JSON. No blockers."; }
    part readdDroppedViewsStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b065-51627384a206"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bef381a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification readdDroppedViewsImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b065-51627384a207"; :>> title = "Sprint 65 implement gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented src/queries.rs + cmd_query0/cmd_query1 dispatch + viewpoints.view.toml + usage.md rows. VERIFY (live): outstanding=[readdDroppedViews]; item readdDroppedViews {done:false,deps:[],results:0}; trace portGuardKeystone {up:[portGuardsCeremonyCharter,portGuardsCoverage], down:[retireQueryPy]}; trace-need returns valid JSON; workflows = 6 workflows with phaseCount+waves (Delivery 5/5, Business 4/4, ...); view viewpoints = 12 items. clippy --workspace --all-targets -D warnings + cargo test (28) green; rust validate clean; sysmlv2 guard all ALL PASS."; }
    part readdDroppedViewsImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b065-51627384a208"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bef381a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification readdDroppedViewsReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b065-51627384a209"; :>> title = "Sprint 65 review gate"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. queries.rs reuses existing data sources (indexer action DAG, orient::done_names, the parser for satisfy/allocate + workflow action defs) — no new parsing infra; the generic cmd_query0/cmd_query1 keep main.rs DRY; downstream+upstream folded into one `trace`. viewpoints is a TOML view (right tool — a pure type-select). All 7 dropped subcommands are restored. issue025 -> resolved once this lands (its #Resolves resolver readdDroppedViews done). The whole IRL arc + its two open-issue resolvers (suspectDiagnostics, readdDroppedViews) are now complete."; }
    part readdDroppedViewsReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b065-51627384a20a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bef381a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification readdDroppedViewsCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b065-51627384a20b"; :>> title = "Sprint 65 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyReaddDroppedViews + backlog readdDroppedViews DoDs recorded pass; 7 views re-added + smoke-tested; clippy + cargo test (28) + rust validate + sysmlv2 guard all (7) green. issue025 now computes RESOLVED (its #Resolves resolver readdDroppedViews is done) -> open-issues empty. Self-evidencing (D0051)."; }
    part readdDroppedViewsCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b065-51627384a20c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bef381a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }

    verification readdDroppedViewsRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b065-51627384a20d"; :>> title = "Sprint 65 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-20"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: re-adding 7 views was cheap because the data was already there (indexer DAG, parser edges, workflow action defs) — queries.rs is thin glue + the Json emitter; folding downstream/upstream into trace avoided three near-duplicate subcommands. This closes the LAST open issue (issue025) — open-issues now empty, the loop the human asked for has gone full circle: a gap was surfaced, tracked, linked to work, and resolved by computation. CAUGHT (avoidable): a flurry of clippy doc lints (long-first-paragraph, missing backticks, list-item-without-indent, match-vs-if-let) — all mechanical, all caught by the gate; the recurring doc-lint friction is real but the deny-config catches it every time, so no new tracked item (a doc-style note for new modules would help). Estimate 5 pts held. Backlog frontier now EMPTY — all of D0074 (full-rust) + D0077 (issue resolution) milestones complete."; }
    part readdDroppedViewsRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b065-51627384a20e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bef381a"; :>> judgedAt = "2026-06-20"; :>> judgedBy = "claudeOpus"; }
}
