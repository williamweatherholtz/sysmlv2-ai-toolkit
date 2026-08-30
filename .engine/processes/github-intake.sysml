// githubIntake — an issue on the repository becomes a recorded utterance, then a judgment (D0263).
//
// This is the INBOUND half of the federation loop. The decision-channel carries decisions OUT and
// approval gestures IN; nothing carried an issue in, so a defect found by a downstream project or a
// collaborator lived in a browser tab until someone remembered it.
//
// It is deliberately intake, not a second tracker: a GitHub issue is SOMEONE'S WORDS, and what those
// words implicate is a judgment recorded separately (D0216). The same discipline, a different channel.
package ProcessGithubIntake {
    private import EngineElement::*;
    private import EngineProcess::*;

    action githubIntake : Process {
        :>> id = "d0263a01-0830-4c5e-9a1b-2c3d4e5f6a7b";
        :>> title = "GitHub intake";
        :>> purpose = "Take an issue raised on the repository into the model as a verbatim Statement with a durable source, then triage it through the ordinary intake vocabulary - so a report from outside the session becomes tracked work rather than a browser tab, and my reading of it stays checkable against their words.";
        :>> cadence = Cadence::eventDriven;
        // APPLIES-WHEN: the repository accepts issues from anyone the project does not sit beside -
        // a downstream keel project, a collaborator, a future maintainer, or the human on a phone
    }

    action giIngest : ProcessStep {
        :>> id = "d0263a01-0830-4c5e-9a1b-2c3d4e5f6a7c";
        :>> title = "Ingest the issue VERBATIM, attributed to the login that wrote it";
        :>> triggerCondition = "An open issue on the repository that no Statement already cites.";
        :>> actionText = "`keel github-ingest --repo O/N --issue N --by <recorder> --at <date>`. The body is stored character-for-character; `saidBy` is the GITHUB LOGIN, not an enrolled actor, because an outside reporter has no actor id here and inventing one would misattribute their words. `sourceUrl` carries the issue URL, which is what makes a re-ingest REFUSE rather than store the same words twice. Never retype an issue by hand: a paraphrase in a field labelled verbatim is the one defect this whole path exists to prevent.";
        :>> producedArtifact = "One Statement per issue, verbatim, with a URL a reader can check the transcription against.";
        :>> owner = Owner::ai;
    }

    action giTriage : ProcessStep {
        :>> id = "d0263a01-0830-4c5e-9a1b-2c3d4e5f6a7d";
        :>> title = "Translate and triage - the judgment is separate from the words";
        :>> triggerCondition = "A `github` Statement with no UserStory citing it.";
        :>> actionText = "`keel record story --from-statement <st> --implication <kind> --triage-note \"why THIS kind and not a neighbouring one\"`. A compound issue becomes SEVERAL stories - the triage verdict is singular by design. The note is the reviewable part; a bare verdict cannot be argued with. A kind the vocabulary cannot express is a CHANGE to the vocabulary through a Decision, never a forced fit: that rule is why this path needed `github` added to StatementChannel rather than filed under `other`.";
        :>> producedArtifact = "A UserStory per implication, each with its #DerivedFrom edge and a defensible triage note.";
        :>> owner = Owner::ai;
    }

    action giRoute : ProcessStep {
        :>> id = "d0263a01-0830-4c5e-9a1b-2c3d4e5f6a7e";
        :>> title = "Route to the item that carries it, and never invent a resolver";
        :>> triggerCondition = "A triaged story with no `#Implicates` edge.";
        :>> actionText = "Create the downstream item and author `#Implicates` from the story to it. A `bug` needs an Issue, and `record issue` needs a RESOLVER - if none exists, the honest resolver is a backlog task that does not exist yet, so AUTHOR IT rather than attaching the issue to the nearest plausible owner. An issue routed to a resolver that will not resolve it looks handled and is not.";
        :>> producedArtifact = "A tracked item, reachable from the reporter's own words by two edges.";
        :>> owner = Owner::ai;
    }

    action giAnswer : ProcessStep {
        :>> id = "d0263a01-0830-4c5e-9a1b-2c3d4e5f6a7f";
        :>> title = "Close the loop where the reporter is standing";
        :>> triggerCondition = "The routed item reaches a verdict - resolved, superseded, or declined.";
        :>> actionText = "Comment on the GitHub issue with the tracked id and the verdict, and close it there. The model is the truth, but the REPORTER is not reading the model - leaving the issue open forever teaches everyone that filing one achieves nothing, which is how an inbound channel dies. A declined issue gets the same courtesy and the reason.";
        :>> producedArtifact = "An issue whose reporter can see what became of it.";
        :>> owner = Owner::ai;
    }

    first giIngest then giTriage;
    first giTriage then giRoute;
    first giRoute then giAnswer;
}
