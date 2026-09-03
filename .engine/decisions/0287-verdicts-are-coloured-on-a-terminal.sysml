// D0287 (status: the `status` field below is the truth) — keel's verdicts are coloured on a terminal - PASS green, FAIL and ERROR red, WARN yellow, a registered control defect magenta - and byte-identical to before when piped
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0287Accept event on sign-off.
package Decision0287 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a change to the enforcement surface: the guard
    // report's PRINT gains colour (words unchanged), which is guards.rs and therefore keystone-locked.
    #ProspectiveChange part d0287 : Decision {
        :>> id = "ad529b3e-3958-4d55-9c09-af4c7eba56ec";
        :>> title = "keel's verdicts are coloured on a terminal - PASS green, FAIL and ERROR red, WARN yellow, a registered control defect magenta - and byte-identical to before when piped";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human (st072): make it coloured so I can tell passing from failing, and push it into keel so other projects use it. Every gate, guard and validation verdict was plain text; on a screen of 58 guard lines with 27 warnings the reader hunted for the one FAIL. The chat cannot carry colour, so the only place the request can be met for every project is the binary they all run.";
        :>> decision = "A color module with one decision made once per process: colour is on when stdout is a terminal, off under NO_COLOR (any value), and forced on or off by KEEL_COLOR=1 or KEEL_COLOR=0. The verdict words are wrapped in ANSI codes, never replaced: PASS and ALL PASS and 'validated clean' bold green; FAIL, FAILED, ERROR and the validation failure line bold red; WARN yellow; the control-defect note beside a guard's verdict (D0278) magenta, because it is a qualification rather than a pass or a fail; NOTHING TO ASSURE and the empty tone dim. Applied to the guard runner, validate, check-engine and the per-guard report; JSON outputs (assured, the lenses) are untouched because they are machine contracts.";
        :>> rationale = "WHY TERMINAL-ONLY IS THE WHOLE DESIGN: keel's text is parsed by its own hooks, by CI, by land reading a gate, and by 447 tests that assert on the exact words. Colour on a terminal and bare bytes on a pipe means every one of those readers sees what it always saw - verified by piping guard and validate through cat -v before and after: identical. Forcing colour into a pipe is the operator's explicit call (KEEL_COLOR=1 for a pager). WHY WRAP AND NEVER REPLACE: a colour-blind reader and a log file must read the same verdict; the word carries the meaning and the colour only speeds finding it. WHY MAGENTA FOR A DEFECT NOTE: a control known to under-report prints its note beside a green PASS; colouring it red would say the guard failed, green would say it is fine - it is neither, and a third colour says so. WHY NO WINDOWS CONSOLE-MODE CALL: Windows Terminal and git-bash honour ANSI; legacy conhost does not enable it by default, and the fix without a new dependency is NO_COLOR - documented rather than papered over.";
        :>> consequences = "Every project on this engine vintage reads its gate at a glance; nothing that parses keel changes. Two environment variables enter the doc surface (CLAUDE.md section 5, guards.md). REMOVAL PATH (D0163): delete color.rs and the wrap calls; the words are already the fallback.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0287Accept : Test { :>> id = "30651764-6b9b-41d5-990d-1f4fe7915be3"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/29."; }
    part d0287AcceptR1 : TestResult { :>> id = "5dfe6c1c-a62c-4fcd-97f5-b78031230bae"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c3dd223"; :>> judgedAt = "2026-09-03"; :>> judgedBy = "wweatherholtz"; }
}
