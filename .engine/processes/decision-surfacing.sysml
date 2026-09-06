// decisionSurfacing — a decision waiting on the human is surfaced with a LINK, at the top, every
// time (D0269).
//
// The failure this exists to end: I reported open decisions in prose, at the end of long turns,
// several times in one session. Each report was accurate and each one was buried under the work that
// produced it. The human said so directly: "Don't bury it under past messages."
//
// A decision the human cannot find is not a decision they have been asked to make. Surfacing is not
// courtesy - it is the last step of the work, because work that ends in an unmade decision has not
// reached them.
package ProcessDecisionSurfacing {
    private import EngineElement::*;
    private import EngineProcess::*;

    action decisionSurfacing : Process {
        :>> id = "d0269a01-0831-4d7e-9b2c-3e4f5a6b7c8d";
        :>> title = "Decision surfacing";
        :>> purpose = "Make every judgment that is waiting on the human reachable in one click, from the current message, without scrolling back - so a pending decision is something they can act on rather than something they must remember to look for.";
        :>> cadence = Cadence::continuous;
        // APPLIES-WHEN: any turn that ends with a judgment the human has not made - a Need awaiting
        // acceptance, a fork with costed options, a risk awaiting disposition, an ordering choice
    }

    action dsDetect : ProcessStep {
        :>> id = "d0269a01-0831-4d7e-9b2c-3e4f5a6b7c8e";
        :>> title = "Assess what is waiting, from the computed lens";
        :>> triggerCondition = "Substantive work has reached a stopping point, or a Decision was just recorded or accepted.";
        :>> actionText = "READ, do not remember: `keel show authority-queue ROOT` computes every obligation that only a human can discharge - each decisionAcceptance row carries the id, the short name of what it is about, and how long it has waited. That is the assessment; it is never re-derived in prose and never estimated from memory. Then apply the STATE TEST: compare that pending set with the set recorded in `.keel/decision-page.toml` from the last publish. Identical - the page already says everything true, so this process produces NOTHING this turn: no page, no link, no sentence. Different - continue. A decision belongs in the set when the work cannot honestly continue without their judgment, or continued under an assumption they never confirmed; it does NOT belong when I decided it and can defend it, or when the model answers it. Manufacturing an item to have something to publish is worse than burying one.";
        :>> producedArtifact = "The pending set, and whether it differs from what is published.";
        :>> owner = Owner::ai;
    }

    action dsRender : ProcessStep {
        :>> id = "d0269a01-0831-4d7e-9b2c-3e4f5a6b7c8f";
        :>> title = "Put them on one published page, one decision per section";
        :>> triggerCondition = "At least one decision is pending.";
        :>> actionText = "Author the page directly and publish it as an ARTIFACT so it has a durable URL: one section per pending decision, each with its stake, its options carrying a STEELMANNED case and an honest cost, my recommendation, and what would change my mind. Keep it short - a decision the human cannot read in a minute is one they will not judge. UPDATE the existing artifact by republishing the same file path rather than creating a second one: a decision page that forks into several is a queue nobody can find the head of. The console deck (keel deck) carries the same pending decisions for a human who prefers it.";
        :>> producedArtifact = "One artifact URL holding every open decision.";
        :>> owner = Owner::ai;
    }

    action dsGround : ProcessStep {
        :>> id = "d0269a01-0831-4d7e-9b2c-3e4f5a6b7c92";
        :>> title = "Give them what an INDEPENDENT judgment needs, not a request to bless my pick";
        :>> triggerCondition = "A decision tab is being written.";
        :>> actionText = "THE FAILURE THIS EXISTS TO END, in the human's words: 'I get some semblance of we need Need1 and Need2 judged, WHAT DECIDES IT: picking which ones are good.' That is circular - it restates the question as its own answer and leaves them to go find the item. A tab must carry, INLINE: (1) THE ITEM ITSELF - the actual statement, quoted or excerpted, never just its name, because a decision that requires opening another file to understand has not been surfaced; (2) WHAT ACCEPTING BINDS - what future work it legitimises and what it forecloses, since a Need is a contract future work will cite; (3) THE SPECIFIC FORK - the contested clause set against its alternative in their own words, not 'good versus bad'; (4) THE EVIDENCE - what is already TRUE in the model versus what is my assumption, labelled apart; (5) WHAT I DO WITH EACH ANSWER - the concrete next action per option, so the cost of each is visible before choosing. THE TEST FOR 'What decides it': it must name an OBSERVABLE, or a value only they hold. If it can be answered by reading the model, I have not finished my work and must answer it myself instead of asking.";
        :>> producedArtifact = "A tab someone could adjudicate without opening the repository.";
        :>> owner = Owner::ai;
    }

    action dsLead : ProcessStep {
        :>> id = "d0269a01-0831-4d7e-9b2c-3e4f5a6b7c90";
        :>> title = "Link it once, on the turn the page changed";
        :>> triggerCondition = "The page was just published or republished because the pending set changed.";
        :>> actionText = "Give the URL and one line naming what is new or answered - at the top, before the report of the work. Once. THE ANTI-PATTERN THIS ENDS is a per-turn restatement of an unchanged queue: for one day the turn boundary itself carried a count of outstanding decisions, and it was accurate every time, which is precisely why it stopped being read (D0359, the owner's instruction: 'I don't want stop says text anymore. remove'). A reader who has seen the link does not need it again to still have it; a reader who has answered part of it needs to know what changed, which is what this line says.";
        :>> producedArtifact = "One line carrying the URL and the delta, on the turn there was one.";
        :>> owner = Owner::ai;
    }

    action dsCarry : ProcessStep {
        :>> id = "d0269a01-0831-4d7e-9b2c-3e4f5a6b7c91";
        :>> title = "Say nothing while the page is current";
        :>> triggerCondition = "Items remain pending and the published page already carries exactly them.";
        :>> actionText = "Silence here is a computed state, not forgetfulness: it means the published set equals the pending set, which the state test established this turn. Do not restate the queue, do not re-link the page, do not append a count to the work. What keeps the channel honest is that the page is REFRESHED whenever the set moves - including when it shrinks, so an item that was answered stops appearing, and when I withdraw one, which is stated on the page rather than done by quietly ceasing to mention it. STATED RESIDUAL: nothing can force this process to run - no gate reads conversational output (D0151) - and `.keel/decision-page.toml` is machine-local, so a different clone republishes to its own URL and forks the queue until the two are reconciled by hand.";
        :>> producedArtifact = "A turn that adds no words about decisions, because the page already carries them.";
        :>> owner = Owner::ai;
    }

    first dsDetect then dsRender;
    first dsRender then dsGround;
    first dsGround then dsLead;
    first dsLead then dsCarry;
}
