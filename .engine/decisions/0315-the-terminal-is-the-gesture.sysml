// D0315 (status: the `status` field below is the truth) — ttyGesture: keel accept typed at the human's own terminal cites the TTY as the gesture, so a plain sentence there is never refused
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0315Accept event on sign-off.
package Decision0315 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0315 : Decision {
        :>> id = "b5381942-ae18-42c1-8b63-5c0c0652aeb3";
        :>> title = "ttyGesture: keel accept typed at the human's own terminal cites the TTY as the gesture, so a plain sentence there is never refused";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#53 (st076, issue359): the human ran keel accept six times from their own terminal with a plain-sentence note, from the one path the docs told them to use; every write landed and the gate then reported six violations of delegatedAcceptanceSubstanceRule for want of a single-quoted span. The rule accepts a CITED gesture - deck, console, GitHub comment - and the command had not cited the one gesture it could see: an interactive stdin. D0299 already exempts a self-recorded acceptance (recorder equals judge), but on a machine bound to the agent the human's --by makes the agent the recorder, which is the GH#53 shape. This is the mirror of D0289, which handles the agent-session path with the human's quoted words.";
        :>> decision = "When stdin is an interactive terminal, keel accept appends a fixed citation to a note that carries no quote receipt of its own - `<note> - TTY gesture: typed at an interactive terminal by <judge>, <date>` - and the substance predicate (acceptQuotesDelegatedWords) reads `tty gesture` as a cited human-surface gesture alongside deck, console and GitHub comment; a note that already quotes or cites is written unchanged; the agent-session path (no TTY) is unchanged and still requires the human's quoted words; the test stand-in KEEL_TTY_GESTURE=1 is honoured but the record then says `TTY gesture (asserted by KEEL_TTY_GESTURE, not observed)`, so a reader can tell an observed terminal from a declared one.";
        :>> rationale = "Why the command cites and not the human: governance binds the AI, never the human (D0204); asking a person to quote themself at their own keyboard points the control at the one party it must not bind, and GH#53 is what that costs - six honest acceptances red. Why a fixed phrase written by the command: the deck and console citations are already accepted on the word `deck`/`console`; the TTY phrase is the same kind of citation, and the command is the only writer that can observe a terminal, so the phrase is evidence of the channel the same way. Why the env-var stand-in is labelled in the record: an agent could set it, so the citation must say it was asserted rather than observed - the fabricated-quote limit D0192 already states, kept visible instead of silent. Arming (D0298): keel-cli/tests/tty_accept_cites_the_gesture.rs - the GH#53 shape (agent-bound machine, --by the human) accepted with the citation, the recorded text passing the predicate and the bare sentence failing it; the agent path still refused naming the remedy.";
        :>> consequences = "A human at their terminal never sees the quote requirement again; the rule comment in rules.sysml and CLAUDE.md §4 say so. issue359 is resolved by dcTtyAcceptCitesTheGesture. STATED LIMIT: a real TTY is observed by the process, not proven to the reader; the audit trail and the labelled stand-in are what distinguish the cases.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0315Accept : Test { :>> id = "410f5555-86df-4caf-b0c1-3301bca58343"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0315AcceptR1 : TestResult { :>> id = "45754b9a-5bcb-4a73-a0ff-5703ef4a6d91"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d697ae2"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
