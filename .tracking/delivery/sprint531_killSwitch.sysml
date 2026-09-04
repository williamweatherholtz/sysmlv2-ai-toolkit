// ProjectDeliveryS531 — Sprint 531: the hook kill switch is seen, stripped and refused. EXECUTE. 1 pt.
package ProjectDeliveryS531 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from killSwitchStory to d0296;

    part killSwitchStory : Story {
        :>> id = "2acdf716-c3d0-472e-8523-d5d96c358862";
        :>> title = "Sprint 531: disableAllHooks in a repo-scope settings file is a named drift violation, sync-claude strips it, pre-write denies it in every profile (1 pt)";
        :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS531 {
        action storyKillSwitch;
        verification storyKillSwitchDoD : Test { :>> id = "839b25f5-640f-4b79-8857-2d9e55beb9ac"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcDriftGuardRefusesDisableAllHooks (resolves issue365). DONE: HOOK_KILL_SWITCH and REPO_SCOPE_SETTINGS in claude_surface.rs; hooks_silenced reads a settings document, text_sets_kill_switch reads a Write's content or an Edit's new_string; sync_claude --check names the file and the key as a violation for settings.json AND settings.local.json ahead of the generic drift line; merge_settings removes the key so sync-claude repairs it; hook pre-write DENIES the write in every profile naming issue365. VERIFY: two new tests, 457 pass, clippy clean, live plant-and-remove against this repository's own guard."; }
        // RAN: cargo test --release 457/0; cargo clippy --all-targets 0; plant .claude/settings.local.json = disableAllHooks:true -> guard claude-surface-drift FAIL 1 violation naming the file; rm -> PASS
        part storyKillSwitchDoDR1 : TestResult { :>> id = "ec19f05c-483a-43d3-85ee-c2c6611b60e3"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "cffe258"; :>> judgedAt = "2026-09-03"; :>> judgedBy = "claudeOpus5"; }
    }

    verification killSwitchRefineGate : Test { :>> id = "82de9b6f-22b1-411a-90e7-7ecfe27c3f69"; :>> title = "Sprint 531 refine gate (DoR)"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY from issue365 and D0296 runs 2 and 5: the key's effect was observed live, the mechanism (foreign top-level keys survive the owned-entry comparison) read at claude_surface.rs, the fix's shape in the DoD."; }
    verification killSwitchStandupGate : Test { :>> id = "5c79c105-fad9-43df-ab0e-658d6dad0164"; :>> title = "Sprint 531 standup gate"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE CHOICE: deny in EVERY profile, not only strict. Every other pre-write outcome is advisory in guided projects (D0176); this key is the exception because the advisory is the first thing it switches off - an advisory against a kill switch is a note taped to the breaker. Second choice: read settings.local.json too, though keel never writes it - the key works from there just the same."; }
    verification killSwitchImplementGate : Test { :>> id = "1d8459b1-60ef-4c5f-9b03-8a2c34e968ac"; :>> title = "Sprint 531 implement gate"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: the shell heredoc swallowed the edit script once (a quote inside the Rust char literal); rewritten through the Write tool as the memory says. A test-module import line, then three clippy first-paragraph lengths. The live proof planted the key in this repository's own settings.local.json and read the guard fail, then removed it and read the pass."; }
    verification killSwitchReviewGate : Test { :>> id = "e5577c44-598d-47f5-9e5d-71267498a7db"; :>> title = "Sprint 531 review gate — three passes per D0172"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) IS THE DENY TEXTUAL AND IS THAT SAFE: yes and yes - it matches the key followed by true; a false value, or the word in prose, is not denied (tested). (2) DOES THE GUARD NAME THE RIGHT FILE: settings.local.json in the test, settings.json in the live plant's sibling path; the message starts with the relative path. (3) WHAT IT DOES NOT DO: a key set in the USER's settings.json is out of scope by design - that is the user's own file, not the agent's, and D0296's launch override covers it."; }
    verification killSwitchCloseOutGate : Test { :>> id = "1c92f31b-f702-4ca3-a72a-fe86cc1eeba8"; :>> title = "Sprint 531 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. issue365 resolved; guards.md and CLAUDE.md carry the new claim in the same commit."; }
    verification killSwitchRetroGate : Test { :>> id = "1275684d-5ea8-4242-9d26-6884a6ae711d"; :>> title = "Sprint 531 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: this closes only the commit path for the kill switch; the live-session and next-launch paths are dcHookConfigChangeRestoresTheFile and dcLaunchPinsHooksOn, chartered by d0296 and next on the frontier. The heredoc failure recurred for the third time this week and was only a memory, not a control (D0047) - it is now dcPreBashSeesHeredocSource: the existing pre-bash advisory (issue094 lineage) learns to see a heredoc carrying source and points at the Write tool."; }
}
