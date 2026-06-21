// =====================================================================
// ProjectDeliveryS74 — Sprint 74: orientPerf (serves sr11FastStart). Cut `orient` from ~13.6s to
// ~0.9s by collapsing per-task git spawns into batched `git cat-file`/`git diff` (the dominant cost
// on Windows, ~60ms/spawn). Surfaced by the sr11FastStart verification (D0082) — disposition: fix.
// Total: 3 pts  Ceremony: AUTONOMOUS (D0049).
// =====================================================================
package ProjectDeliveryS74 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from orientPerfStory to sr11FastStart;

    part orientPerfStory : Story {
        :>> id = "63940516-2c7d-4e8f-b074-51627384ab01";
        :>> title = "Sprint 74: orientPerf (3 pts) — batch git in orient (13.6s -> ~0.9s); serves sr11FastStart";
        :>> createdAt = "2026-06-21";
        :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 3;
    }

    action def DeliveryRunS74 {
        action storyOrientPerf;
        verification storyOrientPerfDoD : Test { :>> id = "63940516-2c7d-4e8f-b074-51627384ab02"; :>> method = VerificationMethod::test; :>> procedureText = "orient batched-git optimization: valid_commits (one cat-file --batch-check for all passing SHAs, was 1 cat-file/task), batch_cat_blobs + criterion_suspects (one cat-file --batch for all historical DoD blobs, was ls-tree + show-per-file per (task,dep)), apply_deliverable_suspicion (one git diff per distinct verified-commit, was one git log/task). VERIFY: orient 13.6s -> ~0.9s (>14x) live; suspect/ready/done identical; clippy -D warnings + cargo test (31 lib) + rust validate green."; }
        part storyOrientPerfDoDR1 : TestResult { :>> id = "3023f437-1024-4588-9b22-8bc8af953b80"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2a20503"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
    }

    verification orientPerfRefineGate : Test { :>> id = "63940516-2c7d-4e8f-b074-51627384ab03"; :>> title = "Sprint 74 refine gate (DoR)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "DoR: the sr11FastStart verification (Sprint 73, D0082) MEASURED orient at ~13.6s vs the <500ms requirement -> the human dispositioned the >=Medium gap as 'fix now'. Diagnosis: orient spawned a git process per task (cat-file sha-validity + per-(task,dep) criterion ls-tree/show + per-task deliverable-drift git log) — ~hundreds of ~60ms Windows spawns. Fix: batch into a handful of git invocations. estimatedPoints=3. #CharteredBy sr11FastStart. DoD accepted."; }
    part orientPerfRefineGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b074-51627384ab04"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2a20503"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification orientPerfStandupGate : Test { :>> id = "63940516-2c7d-4e8f-b074-51627384ab05"; :>> title = "Sprint 74 standup gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Refine passed. Approach: (1) valid_commits via cat-file --batch-check (all distinct passing SHAs, one spawn); (2) batch_cat_blobs via cat-file --batch (all <ct>:<dodfile> blobs, one spawn) feeding criterion_suspects (build_dod_files maps each DoD to its current file); (3) changed_paths_since via one git diff per distinct verified-commit for deliverable drift. Keep full-scan git_criterion_at as a correctness fallback. Measure after each. No blockers."; }
    part orientPerfStandupGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b074-51627384ab06"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2a20503"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification orientPerfImplementGate : Test { :>> id = "63940516-2c7d-4e8f-b074-51627384ab07"; :>> title = "Sprint 74 implement gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::test; :>> procedureText = "Implemented batched git in orient.rs (valid_commits, batch_cat_blobs, criterion_suspects, changed_paths_since + path_drifted; extracted propagate_transitive_suspect to keep compute_orient under the line cap). Staged measurements: 13.6s (baseline) -> 5.7s (known-file blob) -> 1.25s (batched cat-file) -> ~0.9s (batched deliverable-drift). VERIFY (live): 5-run orient ~0.85-1.0s; suspect=[rustToolchainFix], ready/done unchanged vs baseline; clippy --workspace --all-targets -D warnings green; cargo test 31 lib pass; rust validate clean. Correctness preserved (full-scan fallback for blobs the batch can't resolve)."; }
    part orientPerfImplementGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b074-51627384ab08"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2a20503"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification orientPerfReviewGate : Test { :>> id = "63940516-2c7d-4e8f-b074-51627384ab09"; :>> title = "Sprint 74 review gate"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Diff reviewed. The batched-git rewrite preserves semantics (suspect/ready/done outputs identical to baseline) while cutting wall-clock >14x. Batch parsing is bounded with .get() (no indexing panics; clippy indexing_slicing clean). Conservative fail-safes kept (git failure -> valid/no-drift, never spurious). The remaining ~0.9s (vs the <500ms sr11 bar) is git-spawn overhead (~5 calls) + orient parsing the repo ~twice (indexer + the open-issues Model) — closing that last gap needs a parse-once refactor of a shared path (separate decision; surfaced to the human). No doc-sync needed (orient's contract/behavior unchanged; only faster)."; }
    part orientPerfReviewGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b074-51627384ab0a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2a20503"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification orientPerfCloseOutGate : Test { :>> id = "63940516-2c7d-4e8f-b074-51627384ab0b"; :>> title = "Sprint 74 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::inspect; :>> procedureText = "Autonomous closeOut (D0049): storyOrientPerf + backlog orientPerf DoDs recorded pass; orient ~0.9s (>14x); clippy + cargo test + rust validate + guard all green. Self-evidencing (D0051). sr11FastStart hugely improved but still above the literal <500ms target -> the target decision (revise vs push the parse-once refactor) is surfaced to the human; sr11 disposition pending that."; }
    part orientPerfCloseOutGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b074-51627384ab0c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2a20503"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }

    verification orientPerfRetroGate : Test { :>> id = "63940516-2c7d-4e8f-b074-51627384ab0d"; :>> title = "Sprint 74 retro gate (autonomous, D0049)"; :>> createdAt = "2026-06-21"; :>> createdBy = "claudeOpus"; :>> method = VerificationMethod::analyze; :>> procedureText = "Autonomous retro (D0049). WELL: this is the assurance loop closing on itself — the three-tier coverage verification (Sprint 73) MEASURED a real requirement violation (orient 13.6s) that flat 'covered' had hidden, the human dispositioned 'fix', and the fix landed a >14x speedup with identical outputs. Staged measurement (13.6->5.7->1.25->0.9s) pinpointed git-spawn count as the cost on Windows. CAUGHT (avoidable-issue candidate): the first attempt (known-file blob, still per-call git) only got to 5.7s — the real lever was BATCHING spawns (cat-file --batch), not just narrowing them; the avoidable misstep was optimizing spawn DURATION before spawn COUNT. Lesson + process improvement: on Windows, minimize git PROCESS COUNT above all. Estimate 3 pts held. OUTSTANDING: the literal <500ms needs a parse-once refactor (orient parses the repo ~twice) — a separate, riskier optimization; surfaced for the human to weigh against revising the sr11 target to a realistic full-repo-with-suspicion bar."; }
    part orientPerfRetroGateR1 : TestResult { :>> id = "63940516-2c7d-4e8f-b074-51627384ab0e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "2a20503"; :>> judgedAt = "2026-06-21"; :>> judgedBy = "claudeOpus"; }
}
