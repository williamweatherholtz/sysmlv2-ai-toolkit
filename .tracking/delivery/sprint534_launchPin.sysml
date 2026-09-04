// ProjectDeliveryS534 — Sprint 534: the launch pins the hooks on above project scope. EXECUTE. 2 pts.
package ProjectDeliveryS534 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from launchPinStory to d0296;

    part launchPinStory : Story {
        :>> id = "bc7052a8-184e-4668-97f7-870454ffa6a3";
        :>> title = "Sprint 534: keel claude and the console spawn pass --plugin-dir and a launch-scope disableAllHooks:false - a key on disk at launch no longer kills the hooks (2 pts)";
        :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS534 {
        action storyLaunchPin;
        verification storyLaunchPinDoD : Test { :>> id = "4380e61a-94d7-49e4-9f34-b15fd9b6766f"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcLaunchPinsHooksOn. DONE: launcher::hook_pin_args writes .keel/launch-settings.json (disableAllHooks false) on every call and returns --plugin-dir <plugin rendering when present> --settings <that file>; serve's claude spawn appends it (a write failure is reported and the run still launches on the repo-scope hooks); keel claude [args...] is a new CliCommand (integration, tooling) that resolves the project root, writes the pin, sets KEEL_BIN to this binary, clears CLAUDECODE and passes everything through to claude, exiting with its code. VERIFY: unit test on the pin; cli-surface-declared 102 scanned; the live run over a poisoned project."; }
        // RAN: cargo test --release 461/0; clippy 0; keel guard cli-surface-declared PASS 102; keel --help lists claude; keel claude -p in a fixture whose .claude/settings.json = disableAllHooks:true -> hooks.jsonl user-prompt, pre-bash, post-edit, stop x2; the file stayed poisoned on disk (no change event) and the hooks fired anyway
        part storyLaunchPinDoDR1 : TestResult { :>> id = "0de36746-7873-4857-bc0b-c5a0616e75e0"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1652217"; :>> judgedAt = "2026-09-03"; :>> judgedBy = "claudeOpus5"; }
    }

    verification launchPinRefineGate : Test { :>> id = "89e4a22d-ad66-4188-81f1-401df14103fc"; :>> title = "Sprint 534 refine gate (DoR)"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY from D0296 run 6 and the documented precedence (managed > --settings > local > project > user); the spawn site in serve.rs and the existing KEEL_BIN hardening (D0182/P5.5) gave the pin a place to sit."; }
    verification launchPinStandupGate : Test { :>> id = "3ab4f53a-40a4-41e1-8bbd-24abc46e85e1"; :>> title = "Sprint 534 standup gate"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE CHOICE: --settings takes a FILE under .keel/, not inline JSON - the console spawn goes through cmd /C on Windows and inline JSON quoting there is the kind of thing that works until it does not. Second: keel claude REFUSES to launch when the pin cannot be written, because a launch without the pin is what plain claude already is; the console spawn, by contrast, reports and continues, because a human is watching it and killing their run over a machine-local file is worse."; }
    verification launchPinImplementGate : Test { :>> id = "b8321693-9ce3-41be-a0c1-92168f87d647"; :>> title = "Sprint 534 implement gate"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: script through the Write tool; two build errors (crate path inside the lib, the fact-array length), one clippy backtick. The three registries (COMMAND_NAMES, CLI_FACTS, commands.sysml) moved together and the two-way guard confirmed them equal on the first run."; }
    verification launchPinReviewGate : Test { :>> id = "d7d03fe9-8ec5-484a-9f69-20540d24bbdd"; :>> title = "Sprint 534 review gate — three passes per D0172"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) DOES THE PIN WIN: yes, by precedence, and the live run proves it against a key already on disk. (2) WHAT IS STILL OPEN: a session the human launches with plain claude gets no pin - that is a documented habit (CLAUDE.md), not a control, and dcStatusReportsHookHosts will at least SHOW when a project's settings carry the key. (3) IS THE PIN FILE ITSELF A HOST THE AGENT CAN EDIT: yes, under .keel/, but it is rewritten on every launch from the binary, so an edit lasts one session and the fire-ledger shows the silence."; }
    verification launchPinCloseOutGate : Test { :>> id = "19a17b4a-b9d8-4d7f-8910-2d73534cf58e"; :>> title = "Sprint 534 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. Three of D0296's four chartered items are landed; dcStatusReportsHookHosts remains."; }
    verification launchPinRetroGate : Test { :>> id = "65ac8332-ff01-4c4d-bcfc-59c7acb488fd"; :>> title = "Sprint 534 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: the plain-claude launch gap is d0296's stated limit and dcStatusReportsHookHosts is its detective half; nothing this sprint found is outside those two."; }
}
