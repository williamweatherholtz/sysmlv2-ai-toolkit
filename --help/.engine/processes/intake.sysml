// Intake — a human statement becomes a tracked item. Governed by D0166.
package ProcessIntake {
    private import EngineElement::*;
    private import EngineProcess::*;

    action intake : Process {
        :>> id = "4b78e0c1-3f52-4d96-a814-27ec5b0da963";
        :>> title = "Intake";
        :>> purpose = "Turn what a human said into tracked items, keeping their words and my reading as separate, separately auditable facts.";
        :>> cadence = Cadence::perItem;
    }

    action inRecord : ProcessStep {
        :>> id = "4b78e0c1-3f52-4d96-a814-000000000i01";
        :>> title = "Record the statement VERBATIM";
        :>> triggerCondition = "A human says something DIRECTIVE - a request, correction, constraint, priority, verdict or convention. Not every utterance: a transcript is a corpus with no questions attached.";
        :>> actionText = "Author a `Statement` with `text` character-for-character as they wrote it, including typos, plus saidBy, saidAt and channel. Do NOT tidy, summarise or fix it. The verbatim text is the only artifact that cannot drift, and this session's record shows paraphrase drifting twice - a two-word phrase read as a confirmation, three reports about a destination read as reports about a click.";
        :>> producedArtifact = "A Statement item.";
        :>> owner = Owner::ai;
    }

    action inTranslate : ProcessStep {
        :>> id = "4b78e0c1-3f52-4d96-a814-000000000i02";
        :>> title = "Translate into one or more UserStories";
        :>> triggerCondition = "A Statement is recorded.";
        :>> actionText = "Author a `UserStory` with asA / iWant / soThat, and a `#DerivedFrom` edge to the Statement. A COMPOUND statement becomes SEVERAL stories: the triage verdict is singular by design, so a statement implicating three things gets three stories. Use their framing where they gave one; leave soThat absent rather than inventing an outcome they did not state.";
        :>> producedArtifact = "One or more UserStory items, each citing its Statement.";
        :>> owner = Owner::ai;
    }

    action inTriage : ProcessStep {
        :>> id = "4b78e0c1-3f52-4d96-a814-000000000i03";
        :>> title = "Triage: what does this story IMPLICATE?";
        :>> triggerCondition = "A UserStory exists.";
        :>> actionText = "Set `implication` from ImplicationKind - need, useCase, scopeConstraint (their `constraint`; the word is a SysML keyword), bug, process, architecture, verifiedRequirement (their `requirement`, same reason), designChange, implementationChange, question, priority, attestation, correction, convention, or none - and write `triageNote` saying why THIS kind and not a neighbouring one. The note is the part that gets reviewed: the verdict alone is unfalsifiable, the reasoning is arguable. A kind the vocabulary cannot express is a CHANGE to the vocabulary, never a forced fit.";
        :>> producedArtifact = "A triaged UserStory whose reasoning can be disagreed with.";
        :>> owner = Owner::ai;
    }

    action inRoute : ProcessStep {
        :>> id = "4b78e0c1-3f52-4d96-a814-000000000i04";
        :>> title = "Create the downstream item and link it";
        :>> triggerCondition = "The story is triaged to anything other than `none`.";
        :>> actionText = "Create the item the implication names - Need, UseCase, Decision, Issue, SystemRequirement, a process change, an architecture record, or code - and author `#Implicates` from the story to it. A story triaged and then dropped is WORSE than an untriaged one, because it looks handled; `keel intake` reports it as unrouted.";
        :>> producedArtifact = "A downstream item, reachable from the statement that asked for it.";
        :>> owner = Owner::ai;
    }

    action inAudit : ProcessStep {
        :>> id = "4b78e0c1-3f52-4d96-a814-000000000i05";
        :>> title = "Read the three gaps back";
        :>> triggerCondition = "After any intake, and at every sitting.";
        :>> actionText = "Run `keel intake`. UNPARSED means direction nothing translated. UNROUTED means triaged and dropped. UNSOURCED means work with no recorded statement behind it - the ratio of what was asked for to what the AI invented. Treat unsourced as a FLOOR on the record's completeness, never as a claim about the human: nothing can force a statement to be written down.";
        :>> producedArtifact = "Three numbers, and a decision about each gap.";
        :>> owner = Owner::either;
    }
}
