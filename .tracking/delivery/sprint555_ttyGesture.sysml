// ProjectDeliveryS555 — Sprint 555: the terminal is the gesture - keel accept typed at the human's own terminal never asks them to quote themself. EXECUTE. 3 pts.
package ProjectDeliveryS555 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from ttyGestureStory to d0315;

    part ttyGestureStory : Story {
        :>> id = "4f01e69e-4779-4015-84bd-21ee9e71f2ba";
        :>> title = "Sprint 555: the terminal is the gesture - keel accept typed at the human's own terminal never asks them to quote themself (3 pts)";
        :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 3;
    }

    action def DeliveryRunS555 {
        action storyTtyGesture;
        verification storyTtyGestureDoD : Test { :>> id = "cfab98b7-e857-4150-8b42-9b99262522f6"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcTtyAcceptCitesTheGesture (D0315; resolves issue359 / GH#53). DONE: with stdin an interactive terminal, keel accept appends a fixed citation - TTY gesture: typed at an interactive terminal by <judge>, <date> - to a note that carries no quote receipt; the substance predicate reads the mark as a cited human-surface gesture beside deck, console and GitHub comment; a note that already quotes is unchanged; the agent-session path is unchanged and still requires the human's words; the KEEL_TTY_GESTURE=1 stand-in is honoured but the record says asserted-not-observed. cmd_accept split into tty_gesture and accept_channel_refusal helpers under the 100-line limit. rules.sysml comment and CLAUDE.md §4 say so."; }
        // RAN: same suite receipt: 501/0 exit 0
        part storyTtyGestureDoDR1 : TestResult { :>> id = "c35c8cb4-994e-45b0-bf50-c2e79af5c2ed"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d697ae2927bde09c42b5a05d469da06e52e4cc29"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "claudeOpus5"; }
    }

    verification ttyGestureRefineGate : Test { :>> id = "2747149a-ac1e-45e6-b23c-7d6c5d76d2f0"; :>> title = "Sprint 555 refine gate (DoR)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: the rule already accepted cited gestures and D0299 already exempted the self-recorded case; the GH#53 shape is a machine bound to the agent with the human passing --by, where the recorder is the agent and the rule wanted a quote. The one design question was who writes the citation: the command, because it is the only writer that can observe a terminal, and because asking the human to quote themself binds the one party governance must not bind."; }
    verification ttyGestureStandupGate : Test { :>> id = "f33dfcae-a957-49ca-81f2-466d516c57ee"; :>> title = "Sprint 555 standup gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "ONE CHOICE WITH A VISIBLE COST: KEEL_TTY_GESTURE=1 is a bypass an agent could set. Rather than refuse the DoD's test stand-in, the record labels it - TTY gesture (asserted by KEEL_TTY_GESTURE, not observed) - so the fabricated-quote limit D0192 already states stays visible in the record instead of silent."; }
    verification ttyGestureImplementGate : Test { :>> id = "f135085b-121f-4c3f-9d2c-66ba537ae49a"; :>> title = "Sprint 555 implement gate"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: cargo test --test tty_accept_cites_the_gesture 2 passed (the GH#53 shape accepted with the citation; the recorded text passes note_quotes_human and the bare sentence fails it; the agent path refused naming the remedy); lib quotes tests 2 passed; clippy -D warnings clean; keel validate . clean; check-engine clean; the full suite in the background to completion (receipt on the DoD result)."; }
    verification ttyGestureReviewGate : Test { :>> id = "667988d9-eeb1-4c5a-b712-f82d18c214f2"; :>> title = "Sprint 555 review gate — three passes per D0172"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) IS A CITED TTY EVIDENCE: the same kind of evidence as a cited deck or console - a word the rule accepts - written by the process that observed the terminal; no stronger and no weaker. (2) DOES IT WEAKEN THE AGENT PATH: no, that arm is untouched and tested. (3) THE STAND-IN: labelled in the record; a reader can tell observed from asserted."; }
    verification ttyGestureCloseOutGate : Test { :>> id = "c4bced9d-e891-462c-a49b-5216c6059472"; :>> title = "Sprint 555 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. D0315 accepted under standing consent; issue359 resolved by computation."; }
    verification ttyGestureRetroGate : Test { :>> id = "289ee056-7f46-4a9f-8a40-41279dbc138c"; :>> title = "Sprint 555 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-05"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: the machine-bound-to-the-agent shape that made GH#53 possible is the standing D0201 question dcAttestationBindsToDevice carries; the recorder-versus-judge provenance D0299 added is what let the rule tell the two apart at all. The lesson: when a human hits a control from the documented path, the control is wrong, not the human."; }
}
