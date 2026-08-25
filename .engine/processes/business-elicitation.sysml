// Business elicitation — pull a stakeholder's LATENT needs out of them, keeping their words and my
// reading separate. Governed by D0216. The procedure the business-architecture skill deploys.
//
// WHY THIS EXISTS (issue245). keel already ran an elicitation dialogue: the business-architecture
// skill deploys the Business workflow (Brief -> Personas -> Needs -> UseCases) and has authored 11
// Briefs, 15 Personas, 69 UseCases and 75 Needs. But it deployed WORKFLOWS and named no PROCESS, so
// (a) it appeared in no catalogue - not `process list`, not `activation`, not `hardening` - and could
// be exported by nothing, because the process-skill correspondence keys on a skill purpose naming
// `.engine/processes/<name>.sysml`; and (b) it authored Needs straight out of a conversation whose
// utterances were never recorded, which is both the mechanical cause of the unsourced population and
// the D0157 defect exactly: a Need with no cited utterance cannot be checked against what the human
// actually said, so nobody can tell a real demand from one written WIDER THAN THE DEMAND.
//
// This is NOT a fourth elicitation surface. Intake (D0166) is REACTIVE - it records direction a human
// volunteered. This is ACTIVE - it asks, for a stakeholder who cannot author a Need but can answer a
// question about their pain. The two share one substrate on purpose: every answer becomes a Statement,
// so an elicited need and a volunteered one are the same kind of fact and are counted by one view.
package ProcessBusinessElicitation {
    private import EngineElement::*;
    private import EngineProcess::*;

    action businessElicitation : Process {
        :>> id = "7e4a1c93-8d26-4f51-b0a7-3c9e51d8420f";
        :>> title = "Business elicitation";
        :>> purpose = "Draw out what a stakeholder needs but has not stated, recording their words verbatim before any Need is authored, so every Need can be checked against something they actually said.";
        :>> cadence = Cadence::perItem;
        // APPLIES-WHEN: the people who will live with the result cannot write requirements but can tell you what hurts; ALMOST ALWAYS TRUE, and the exception is a project with no stakeholder but its author
    }

    action beScope : ProcessStep {
        :>> id = "7e4a1c93-8d26-4f51-b0a7-000000000b01";
        :>> title = "Name whose needs these are";
        :>> triggerCondition = "A request means design, scope, figure-out X, what should we build, or a new capability - and the stakeholder has not authored Needs themselves.";
        :>> actionText = "Identify the stakeholder and what they are trying to accomplish, as a `Persona` and a one-line framing. Do not start from a solution the requester already named: the point of eliciting is that the stated request is usually a proposed remedy, and the need behind it is what must be recorded.";
        :>> producedArtifact = "A Persona and the framing question the dialogue will answer.";
        :>> owner = Owner::either;
    }

    action beAsk : ProcessStep {
        :>> id = "7e4a1c93-8d26-4f51-b0a7-000000000b02";
        :>> title = "Ask about pain, not features";
        :>> triggerCondition = "A stakeholder and framing exist.";
        :>> actionText = "Ask what is costly, slow, error-prone or frightening TODAY, and what they do instead when it fails. Ask for the last concrete instance, not the general case. NEVER offer a menu of features and ask which they want: a chosen option is evidence about the menu, not about the need, and it is the fastest route to the D0157 defect. Keep asking until an answer names a consequence rather than a wish.";
        :>> producedArtifact = "Answers in the stakeholder's own words, not yet interpreted.";
        :>> owner = Owner::either;
    }

    action beRecordVerbatim : ProcessStep {
        :>> id = "7e4a1c93-8d26-4f51-b0a7-000000000b03";
        :>> title = "Record each answer VERBATIM as a Statement, BEFORE interpreting it";
        :>> triggerCondition = "The stakeholder has answered.";
        :>> actionText = "Author a `Statement` per substantive answer with `text` character-for-character as they said it, plus saidBy, saidAt and channel - the same artifact intake authors, deliberately, so an elicited utterance and a volunteered one are one kind of fact. Do this BEFORE authoring any Brief, Persona detail or Need: once a Need is written the wording that justified it is gone, and every later reader is checking my paraphrase against my conclusion. This step is what makes the rest of the dialogue auditable rather than merely plausible.";
        :>> producedArtifact = "One Statement per answer, verbatim.";
        :>> owner = Owner::ai;
    }

    action beTranslate : ProcessStep {
        :>> id = "7e4a1c93-8d26-4f51-b0a7-000000000b04";
        :>> title = "Translate to UserStories and route them";
        :>> triggerCondition = "Statements are recorded.";
        :>> actionText = "Author a `UserStory` per implication with a `#DerivedFrom` edge to its Statement, then triage per the intake process - the same vocabulary, the same triageNote discipline, no parallel scheme. A compound answer becomes several stories. Where the stakeholder gave no outcome, leave soThat ABSENT rather than inventing one: an invented outcome is how a Need becomes wider than the demand.";
        :>> producedArtifact = "Triaged UserStories, each citing the words it came from.";
        :>> owner = Owner::ai;
    }

    action beAuthorNeeds : ProcessStep {
        :>> id = "7e4a1c93-8d26-4f51-b0a7-000000000b05";
        :>> title = "Author Needs that CITE their evidence";
        :>> triggerCondition = "UserStories are triaged as need-implicating.";
        :>> actionText = "Author each `Need` solution-free and MEASURABLE (not `it is slow` but `cold load under 1s for a 2,500-item set`), and give it a `#DerivedFrom` edge to the UserStory that implicated it. A Need with no such edge is my judgment presented as their demand - legitimate under D0064 as decision-driven work, but it must not be dressed as elicited. Where the measurable threshold was NOT stated by the stakeholder, say so in the Need rather than picking a number that will later be read as theirs.";
        :>> producedArtifact = "Needs traceable to specific recorded words.";
        :>> owner = Owner::ai;
    }

    action beConfirmNarrower : ProcessStep {
        :>> id = "7e4a1c93-8d26-4f51-b0a7-000000000b06";
        :>> title = "Read the Needs back and let them be CUT";
        :>> triggerCondition = "Needs are authored.";
        :>> actionText = "Read the Needs back to the stakeholder and ask specifically what is WIDER than they meant - not whether the set is good, which invites assent. D0157 records the failure this prevents: N-8 was not wrong, it was written wider than the demand, and a later reader decomposed the wide half. A Need the stakeholder narrows is the process working; a Need nobody narrowed is unconfirmed rather than confirmed. Record their acceptance as a `method=confirmation` verification ONLY on their explicit words, never inferred from their silence.";
        :>> producedArtifact = "A Need set the stakeholder has actually narrowed, with recorded acceptance.";
        :>> owner = Owner::human;
    }

    // Step order, native succession (D0143).
    first beScope then beAsk;
    first beAsk then beRecordVerbatim;
    first beRecordVerbatim then beTranslate;
    first beTranslate then beAuthorNeeds;
    first beAuthorNeeds then beConfirmNarrower;
}
