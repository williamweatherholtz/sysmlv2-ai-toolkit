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
        :>> title = "Notice that something is actually waiting on them";
        :>> triggerCondition = "A turn is ending.";
        :>> actionText = "A decision is PENDING when the work cannot honestly continue without their judgment, or when it continued under an assumption they have not confirmed. Concretely: a proposed Need or Decision with no acceptance, a fork where I picked a side and the other side is defensible, an accepted risk they have not seen, a priority order I chose. NOT pending: something I decided and can defend, or a question I can answer myself by reading the model - those get answered, not escalated. Manufacturing decisions to have something to surface is worse than burying one.";
        :>> producedArtifact = "A yes/no on whether anything is genuinely waiting.";
        :>> owner = Owner::ai;
    }

    action dsRender : ProcessStep {
        :>> id = "d0269a01-0831-4d7e-9b2c-3e4f5a6b7c8f";
        :>> title = "Put them in the exec-summary artifact, one tab each";
        :>> triggerCondition = "At least one decision is pending.";
        :>> actionText = "Deploy the `exec-summary` skill against the se-templates instance: one tab per decision, each with its stake, its options carrying a STEELMANNED case and an honest cost, my recommendation, and what would change my mind. Run the browser suite - the template's word budgets and contract checks are what keep the page readable, and they have rejected my drafts three times for good reason. Then publish it as an ARTIFACT so it has a durable URL. UPDATE the existing artifact by republishing the same file path rather than creating a second one: a decision page that forks into several links is buried again, differently.";
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
        :>> title = "Lead with the link - the top of the message, not the end";
        :>> triggerCondition = "The artifact is published or updated.";
        :>> actionText = "The URL goes in the FIRST lines of the response, before the report of what was done. Name what is waiting in one line each so they can decide whether to open it. The report follows the link; it never precedes it. THE ANTI-PATTERN THIS FORBIDS is the one that caused this process: a thorough account of the work with the pending decisions honestly described at the bottom, which is indistinguishable from not asking.";
        :>> producedArtifact = "A response whose first content is the decision link.";
        :>> owner = Owner::ai;
    }

    action dsCarry : ProcessStep {
        :>> id = "d0269a01-0831-4d7e-9b2c-3e4f5a6b7c91";
        :>> title = "Keep carrying it until it is answered or withdrawn";
        :>> triggerCondition = "A previously surfaced decision is still unanswered at the end of a later turn.";
        :>> actionText = "Re-surface the link every turn while anything remains pending. Silence is not an answer and it is not consent - it usually means the message scrolled past. A decision may leave the page two ways: they answer it, or I withdraw it because the work moved and it no longer matters, which is stated rather than dropped. An unanswered decision that quietly stops being mentioned has been decided by default, by me, invisibly.";
        :>> producedArtifact = "A pending set that shrinks only by answer or by stated withdrawal.";
        :>> owner = Owner::ai;
    }

    first dsDetect then dsRender;
    first dsRender then dsGround;
    first dsGround then dsLead;
    first dsLead then dsCarry;
}
