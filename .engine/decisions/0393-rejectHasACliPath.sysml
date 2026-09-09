// D0393 (status: the `status` field below is the truth) — rejectHasACliPath: a human's rejection of a proposed Decision is recorded from an agent session through keel reject, under exactly the channel rules keel accept applies, with who recorded stamped on the result
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0393Accept event on sign-off.
package Decision0393 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: issue414, 2026-09-08: the human answered a brief ask with the reverse of D0376 and there was no keel reject - write::reject_decision existed only behind the console's POST /api/decision/reject (serve.rs) - so the first rejection in the project's history (d0376, recorded as d0380) was a hand edit mirroring the writer's output, which no guard distinguishes from a fabricated one. keel accept's channel layer (accept_channel_refusal in main.rs) already encodes every rule a verdict needs: the declared recording delegation, the typographic quote receipt, read-back ratification, the TTY gesture, and refusal when the recorder is unbound; reject_decision stamped judgedBy but not createdBy, so the console's rejections never said who recorded them.
    part d0393 : Decision {
        :>> id = "78183e7e-5871-4c59-b517-3a69ad3f624b";
        :>> title = "rejectHasACliPath: a human's rejection of a proposed Decision is recorded from an agent session through keel reject, under exactly the channel rules keel accept applies, with who recorded stamped on the result";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "dcRejectHasACliPath: a rejection is the same act as an acceptance with the opposite verdict, and the engine's write API had one path for the first and none for the second outside the console. The CLI surface is an authored fact (D0271): a new command is a CliCommand in .engine/cli/commands.sysml, mirrored in cli_facts.rs and dispatched, or guard cli-surface-declared fails.";
        :>> decision = "NOT A FORK: the shape is the existing acceptance path with the verdict inverted, and nothing in the item admits an alternative. keel reject <d> (--words TEXT | --note TEXT) --by <person> --date YYYY-MM-DD is a governance-family CliCommand beside accept; it shares accept's channel layer through one function taking the verb - the same delegation key (decisionAcceptance covers a human's verdict on a Decision either way), the same quote receipt, the same read-back message in its own verb, the same TTY gesture at the human's terminal, the same refusal when --by names the judge and the session actor is unbound - and it dispatches to write::reject_decision, which now stamps createdBy on the RejectR1 result (D0299) for the CLI and the console alike.";
        :>> rationale = "Two channel layers for one act would drift, so there is one, parameterised by the verb. Reusing the acceptance delegation rather than declaring a second is a judgment stated here: the human delegated the recording of their verdict on Decisions, and a verdict has two values; a project that wants rejections undelegated while acceptances are delegated has no such wish today and can split the key when it does. createdBy on the rejection result is what lets a reader tell a human's own console tap from an agent's delegated record, which is the distinction D0299 exists for.";
        :>> consequences = "cmd_reject in main.rs, the reject arm in the dispatch, COMMAND_NAMES 69 to 70, CLI_FACTS 106 to 107, cliReject in commands.sysml; write::reject_decision gains recorded_by and the console passes the judge as recorder; keel-cli/tests/reject_has_a_cli_path.rs holds the recorded, refused-bare-no, refused-unbound and surface cases. d0380's hand-edited rejection of d0376 stands as history; the next rejection goes through the command.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0393Accept : Test { :>> id = "3315260f-3494-47c2-b9de-1c8be0bfcc40"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0393AcceptR1 : TestResult { :>> id = "35b0caaa-c310-4b27-8e5a-4bd91208605e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4b75101"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
