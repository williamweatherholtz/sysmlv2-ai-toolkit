// Intake for 2026-09-03 — their words, my translation, the triage verdict (D0166/D0216).
// Written by `keel record statement` / `keel record story` (D0236): a Statement's text is
// VERBATIM, so it is escaped for the literal and otherwise untouched.
package ProjectIntake20260903 {
    private import EngineElement::*;
    private import EngineIntake::*;
    private import EngineRelationships::*;

    part st071 : Statement {
        :>> id = "15e001df-611f-45c4-abdf-0cb4562b3749";
        :>> title = "0/0 is 0 percent, not a clean sweep of 100: nothing is done yet";
        :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5";
        :>> text = "this happens regularly - projects inheriting keel get a clean sweep of 100% when 0/0 should be considered 0% to indicate that nothing is done yet.  make those changes where relevant?";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-02"; :>> channel = StatementChannel::chat;
    }

    part us073 : UserStory {
        :>> id = "0fe84648-59ed-403e-b75e-febc0ab7718a";
        :>> title = "An empty population reads as nothing done, never as complete";
        :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5";
        :>> asA = "the owner of a project that just inherited keel";
        :>> iWant = "every ratio and readiness verdict to read 0 percent or 'nothing to assure' when there is nothing to measure";
        :>> soThat = "a fresh project does not open on a clean sweep of green that says everything is done when nothing has been authored";
        :>> implication = ImplicationKind::verifiedRequirement;
        :>> triageNote = "MEASURED on a fresh scaffold before the change: report assurance showed three 100 percent cards and READY; tier-satisfaction reported pct 100 per empty tier. Four sites defaulted an empty denominator to full (reports.rs pct, verification.rs pct, tier-satisfaction's missing-tier default, ReadinessBlockers::ready over an empty governed set) and one test pinned it as 'vacuously complete'. Fixed in the same sprint with a source-scan test so a fifth spelling cannot land.";
    }
    #DerivedFrom dependency from us073 to st071;

    part st072 : Statement {
        :>> id = "95407628-ca83-43b1-a770-679b1f7f0cd3";
        :>> title = "Colour passing vs failing, in keel itself so every project gets it";
        :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5";
        :>> text = "make it colored so i can tell passing vs failing. and push this back up to keel so other projects use";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-03"; :>> channel = StatementChannel::chat;
    }

    part us074 : UserStory {
        :>> id = "84314b4c-cabc-4f79-833f-705f9f9452da";
        :>> title = "A verdict is legible at a glance: pass green, fail red, warning yellow, in every project";
        :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5";
        :>> asA = "a human reading keel's gate output";
        :>> iWant = "PASS, FAIL, WARN, ERROR and the control-defect notes coloured in the terminal, in the binary every project runs";
        :>> soThat = "I can tell passing from failing without reading each line, and no project has to configure it";
        :>> implication = ImplicationKind::implementationChange;
        :>> triageNote = "Colour only when stdout is a terminal (NO_COLOR and KEEL_COLOR respected), so every text-parsing consumer - hooks, CI, tests, land - sees the same bytes; the words never change. Chat cannot carry colour, so the tool is where it lives.";
    }
    #DerivedFrom dependency from us074 to st072;

    part st073 : Statement {
        :>> id = "e4507698-fe31-4acc-9d14-421f56b5e487";
        :>> title = "Remove exec-summary as a process";
        :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5";
        :>> text = "remove exec-summary as a process";
        :>> saidBy = "wweatherholtz"; :>> saidAt = "2026-09-03"; :>> channel = StatementChannel::chat;
    }

    part us075 : UserStory {
        :>> id = "4f168112-eb13-47aa-8b74-04c0d00e8d9d";
        :>> title = "The engine does not carry exec-summary";
        :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5";
        :>> asA = "the owner of the engine";
        :>> iWant = "the exec-summary process, skill, template and unit record gone from the engine, with decision-surfacing rendering its own page";
        :>> soThat = "one process states how a decision reaches me, and the engine ships nothing it does not use";
        :>> implication = ImplicationKind::process;
        :>> triageNote = "Executed as D0288 the same hour: files deleted, unit record and id removed, five dependents rewritten with no tombstones; the library's published copy at v3 is untouched and said so.";
    }
    #DerivedFrom dependency from us075 to st073;
}
