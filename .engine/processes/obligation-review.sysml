// Obligation Review — what is waiting on the HUMAN, as a page they operate away from the terminal.
// Governed by D0171. A PROCESS rather than a script so it is a movable unit (D0128): a downstream
// project adopts it with `keel activate obligation-review` and gets the skill, the generator and this
// definition together.
package ProcessObligationReview {
    private import EngineElement::*;
    private import EngineProcess::*;

    action obligationReview : Process {
        :>> id = "8f21c4d0-6b53-4e97-a018-53ca27be9f16";
        :>> title = "Obligation Review";
        :>> purpose = "Put what is waiting on the human in front of them where they actually are, and take their judgment back as recorded facts with no copy step. Deploys `keel deck` (keel-cli/src/deck.rs), .engine/tools/test_deck_e2e.py, and .engine/tools/deck_inbox_record.py.";
        :>> cadence = Cadence::eventDriven;  // a sitting opening, or the turn-boundary hook noticing a queue
    }

    action orCompute : ProcessStep {
        :>> id = "8f21c4d0-6b53-4e97-a018-000000000o01";
        :>> title = "Compute the obligation set — never enumerate it";
        :>> triggerCondition = "The human asks what needs them, or a sitting opens, or the turn-boundary hook reports obligations with no console up.";
        :>> actionText = "Run the generator, which reads the COMPUTED views (`keel orient`, `authority-queue`, `dispositions`, `sitting-coverage`) through the binary. Never hand-list obligations: a hand-listed queue is a second truth that drifts, and the whole point of the deck is that a new obligation class appears because a viewpoint declared it, not because someone remembered to add it.";
        :>> producedArtifact = "One self-contained HTML file, generated.";
        :>> owner = Owner::ai;
    }

    action orDelineate : ProcessStep {
        :>> id = "8f21c4d0-6b53-4e97-a018-000000000o02";
        :>> title = "Separate what needs a SIGNATURE from what needs a judgment";
        :>> triggerCondition = "The obligation set is computed.";
        :>> actionText = "Decisions awaiting acceptance are rendered FIRST and structurally distinct - a heavier rail, a `needs your signature` ribbon, and the verb `Sign` rather than `Accept`. Every other class asks for a judgment an AI could in principle have formed and is merely deferring; an acceptance asks for something no AI may EVER supply (section 4). Rendering the two identically is how an instruction gets mistaken for a signature, which is the defect issue144 recorded.";
        :>> producedArtifact = "A deck where the signature obligations cannot be confused with the rest.";
        :>> owner = Owner::ai;
    }

    action orPublish : ProcessStep {
        :>> id = "8f21c4d0-6b53-4e97-a018-000000000o03";
        :>> title = "Serve locally, or publish with the connector inbox declared";
        :>> triggerCondition = "The deck is generated.";
        :>> actionText = "At their machine: `keel serve`, point them at /deck - every tap saves through the TESTED endpoints and earns `saved to keel`. Away from it: publish `keel deck --out` with the artifact `mcp` capability declaring the Smartsheet `add_rows` tool; each tap delivers one ROW to the keel-deck-inbox sheet (.engine/contracts/deck-inbox.toml) and shows the STORE'S row id as its receipt - `delivered to inbox`, never `saved`, because keel has the verdict only after the pull (issue194: the live-doc mechanism is retired; its appends silently dropped and no receiver-side acknowledgment existed). Never publish a connector call whose request/response pair was not observed in-session.";
        :>> producedArtifact = "A URL the human can open on a phone, whose save labels claim exactly what each transport can verify.";
        :>> owner = Owner::ai;
    }

    action orRecord : ProcessStep {
        :>> id = "8f21c4d0-6b53-4e97-a018-000000000o04";
        :>> title = "Turn each judgment into a recorded fact through the write API";
        :>> triggerCondition = "The human has judged one or more items.";
        :>> actionText = "Local taps record themselves. Inbox rows are pulled by the SESSION (only it holds connector access): get_sheet_summary with Processed=false, then .engine/tools/deck_inbox_record.py, which records each row through the write API and VERIFIES it from the computed view; only verified rows are marked Processed, and duplicate deliveries dedup by latest-At per (uid, kind). Their note is recorded verbatim, never paraphrased. NEVER infer a verdict from silence, and never record a signature the deck did not carry - an untouched card is an untouched card.";
        :>> producedArtifact = "Recorded facts with provenance, and a queue that shrinks.";
        :>> owner = Owner::ai;
    }

    action orReadBack : ProcessStep {
        :>> id = "8f21c4d0-6b53-4e97-a018-000000000o05";
        :>> title = "Read the queue back from the computed view";
        :>> triggerCondition = "After recording.";
        :>> actionText = "Re-run the obligation view and report the new total from IT, never from the count of judgments processed. The two disagree whenever a recording failed, and the transform's own report is exactly the thing that cannot notice its own failure.";
        :>> producedArtifact = "The obligation total, computed.";
        :>> owner = Owner::ai;
    }
}
