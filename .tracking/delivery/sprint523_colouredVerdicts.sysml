// ProjectDeliveryS523 — Sprint 523: verdicts are coloured on a terminal, in the binary every project runs. EXECUTE. 1 pt.
package ProjectDeliveryS523 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from colouredVerdictsStory to d0287;

    part colouredVerdictsStory : Story {
        :>> id = "2a0ba910-c9f1-41c1-85ce-09058494862d";
        :>> title = "Sprint 523: PASS green, FAIL red, WARN yellow, DEFECT magenta on a terminal; bare bytes when piped (1 pt)";
        :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p1; :>> owner = "claudeOpus5"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS523 {
        action storyColouredVerdicts;
        verification storyColouredVerdictsDoD : Test { :>> id = "a95116f0-e205-4192-95a5-9635a0c6eb80"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcVerdictsAreColoured. The human (st072): colour passing versus failing, and put it in keel so other projects get it. DONE: color.rs decides once per process - terminal and not NO_COLOR, or KEEL_COLOR forcing - and wraps the verdict words without replacing them; applied to the guard runner (per-guard PASS/FAIL, WARN, ERROR, the control-defect note in magenta, ALL PASS / FAILED), validate and check-engine (clean in green, failure in red). JSON outputs untouched. VERIFY: piped output byte-identical to before (cat -v); KEEL_COLOR=1 emits the codes; NO_COLOR wins; two unit tests; the full suite unchanged because capture is not a terminal; the env vars documented."; }
        // RAN: keel guard cli-surface-declared . | cat -v -> bare; KEEL_COLOR=1 -> ^[[1;32mPASS^[[0m; KEEL_COLOR=1 keel validate -> green clean line; NO_COLOR=1 -> bare; cargo test color::tests 2/2; full suite 447/0; clippy clean after nesting one or-pattern
        part storyColouredVerdictsDoDR1 : TestResult { :>> id = "243469d4-101a-4c32-b013-50036ffadbf9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a88ae5b"; :>> judgedAt = "2026-09-03"; :>> judgedBy = "claudeOpus5"; }
    }

    verification colouredVerdictsRefineGate : Test { :>> id = "bcbf9e5b-20c4-4ada-9fda-199da2f4dae7"; :>> title = "Sprint 523 refine gate (DoR)"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY because the constraint was obvious and total: keel's text is parsed by its own hooks, CI, land and 447 tests, so colour must be invisible to every non-terminal reader. That fixed the design - terminal-only, wrap-never-replace - before any code."; }
    verification colouredVerdictsStandupGate : Test { :>> id = "e70bb958-5ab8-45fe-9228-1a21d4f15c4c"; :>> title = "Sprint 523 standup gate"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "TWO CHOICES. (1) No crate: std::io::IsTerminal and a OnceLock are enough, and a colour dependency would be the largest change in the diff for the smallest reason. (2) No Windows console-mode call: Windows Terminal and git-bash honour ANSI, legacy conhost does not by default, and the honest fix without a dependency is NO_COLOR, documented. Magenta for the defect note because it is neither a pass nor a fail."; }
    verification colouredVerdictsImplementGate : Test { :>> id = "35a41349-5f44-44f1-bfc5-7bd0a54e18d4"; :>> title = "Sprint 523 implement gate"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: the same two commands piped through cat -v in three environments - default (bare), KEEL_COLOR=1 (codes), NO_COLOR=1 (bare); unit tests for off-under-capture and words-survive-stripping; full suite unchanged at 447/0."; }
    verification colouredVerdictsReviewGate : Test { :>> id = "0288f5a8-77b6-4c55-b386-74f504120e5a"; :>> title = "Sprint 523 review gate — three passes per D0172"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) CAN ANY PARSER BREAK: only if it runs keel on a terminal and parses the result, which none of keel's own do; a human doing so by hand has NO_COLOR. (2) DID ANY WORD CHANGE: no - the codes wrap, the test strips them and gets the word back. (3) IS IT EVERYWHERE THE HUMAN LOOKS: guard, gate (validate plus guards), check-engine; assured and the lenses are JSON contracts and stay bare on purpose."; }
    verification colouredVerdictsCloseOutGate : Test { :>> id = "1b275eff-6db3-4141-a170-f3b8afd008b1"; :>> title = "Sprint 523 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. Ships to every project with the next engine vintage; no project configures anything."; }
    verification colouredVerdictsRetroGate : Test { :>> id = "937fc439-8f96-42fa-b596-409809e92147"; :>> title = "Sprint 523 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-03"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: the request came from reading my chat summary ('445/0'), which cannot carry colour; the fix went to the surface that can. The reporting habit - write the numbers out in words - is a chat convention, not a control, and is noted as such."; }
}
