// =====================================================================
// EngineIntake — what a human SAID, what it was translated into, and what it implicates.
// Statement -> UserStory -> triage. Authorized by D0166 (accepted 2026-08-20).
//
// WHY THIS EXISTS. Before this, a human's words survived only as MY transcription inside a
// procedureText field: the model held my reading and the conversation held the original. D0164 came
// out of a requirement buried in an acceptance note; issue137's record was corrected by a note in a
// review deck; D0161 was redesigned three times by one-line questions. Every one of those arrived as
// a statement and was captured by hand. D0018 says what a successor needs belongs in the model, and
// the thing most often needed is what was asked for.
//
// THE CHAIN IS THREE LINKS AND THE SEPARATION IS THE POINT:
//   Statement  - verbatim, no interpretation. Theirs.
//   UserStory  - the faithful translation. Mine, and falsifiable against the Statement.
//   triage     - what the story IMPLICATES, from a closed vocabulary.
//
// A Statement is NEVER edited to agree with what was derived from it. A misreading is corrected by a
// NEW UserStory citing the same Statement; the old one is superseded and stays visible. Their words
// and my reading are separately auditable, which is D0108 ownership and the response contract's
// no-fabricated-attestation rule expressed as data.
//
// TERMINOLOGY (their instruction, D0166): the concept is USER STORY in prose and `UserStory` in the
// model, never the bare word "story". `EngineWork::Story` is a DELIVERY work item (estimatedPoints,
// actualHours) used 356 times across the sprint records and is a different thing entirely.
// =====================================================================
package EngineIntake {
    private import ScalarValues::*;
    private import EngineElement::*;

    // The edge from a UserStory to whatever downstream item it produced. Declared here so `#Implicates`
    // is a legal marker and `implicates` a traversable edge kind, both derived from this declaration
    // rather than restated in code (D0136/issue119).
    metadata def Implicates;

    // WHAT A USER STORY IMPLICATES. The first nine members are the human's own enumeration, verbatim
    // from their message. The remaining five answer the "others?" they asked with — and they are not
    // invented, they are the categories THIS SESSION's own statements actually fell into, which is the
    // only evidence available about what a real intake stream contains.
    enum def ImplicationKind {
        need;                   // a stakeholder outcome -> EngineNeeds::Need
        useCase;                // an interaction -> EngineNeeds::UseCase
        // `constraint` and `requirement` are SysML KEYWORDS and the kernel rejects them as enum members
        // (the Rust parser accepts both - exactly the gap CLAUDE.md section 5 warns about, caught here by
        // validate_schema.py, which blocks on schema changes). Renamed, not dropped: their words map to
        // these members and the intake skill states the mapping so nothing is lost in translation.
        scopeConstraint;        // their `constraint` - a boundary on the solution -> a Decision that supersedes
        bug;                    // something is wrong -> Issue
        process;                // how work is done -> a Process/ProcessStep change (a CHANGE route)
        architecture;           // functional structure -> the CodeElement registry / an arch Decision
        verifiedRequirement;    // their `requirement` - a verified contract -> SystemRequirement + satisfy/verify
        designChange;           // how an implementation is SHAPED, without changing what it must do
        implementationChange;   // change the code directly; no new contract implied

        // --- answering "others?", each observed in this session -------------------------------------
        question;               // asks for an answer, implicates no change ("is that in our schema already?")
        priority;               // reorders existing work rather than adding any ("defer the hosted service")
        attestation;            // their sign-off or verdict; the single largest category by count here
        correction;             // retracts or corrects a PREVIOUS statement or record of mine
        convention;             // terminology or style with no functional content ("use user story, not story")
        none;                   // deliberately implicates nothing; recorded so silence is a decision
    }

    // WHERE A STATEMENT WAS MADE. Provenance of the channel, because a note typed into a review deck
    // and a line typed in a terminal carry different context and are recovered differently.
    // HOW MUCH AUTHORITY AN EXTERNALLY-SOURCED UTTERANCE CARRIES (D0264).
    //
    // `untrusted` is not an insult and does not mean the reporter is acting in bad faith - it means
    // ANYONE COULD HAVE WRITTEN IT. An issue on a public tracker is an instruction from an
    // unauthenticated stranger, so acting on it autonomously is prompt injection with a filing form.
    // Untrusted input may produce a PLAN and a proposed Decision; it may never produce an
    // implementation without a human accepting one.
    //
    // The tier DEFAULTS from repository visibility and is a PROXY, not a measurement: a private repo
    // with forty collaborators is not forty trusted people. So it is overridable by declaration, and
    // an UNDETERMINED visibility resolves to `untrusted` - fail closed, because the failure modes are
    // not symmetric (a wrongly-trusted stranger gets autonomous action; a wrongly-distrusted
    // maintainer gets asked).
    enum def SourceTrust { trusted; untrusted; }

    enum def StatementChannel { chat; console; deck; commitReview; github; other; }
    // `github` (D0263): an issue or comment on the repository is a channel in its own right, not
    // `other`. It differs from every other member in a way the model must keep: the speaker is a
    // GitHub login rather than an enrolled ProjectActor, and the utterance has a durable external
    // URL - which is what makes ingestion idempotent. Forcing it into `other` would have thrown
    // away both facts, which is the force-fit the intake process forbids.

    // A human's words, VERBATIM. No interpretation, no summary, no tidying.
    //
    // `text` is what they wrote, character for character. If it contains a typo, the typo is recorded:
    // this session's own evidence is that my paraphrases drift (a two-word phrase read as a
    // confirmation; three reports about a destination read as reports about a click) and the verbatim
    // text is the only thing that cannot.
    part def Statement :> Element {
        attribute text : String;                   // VERBATIM. Never paraphrased, never trimmed.
        attribute saidBy : String;                 // actor id — whose words these are
        attribute saidAt : Timestamp;
        attribute channel : StatementChannel;
        // The utterance's durable external address, where one exists (D0263). Present for a `github`
        // statement, absent for a spoken one - so it is [0..1] and never defaulted. This is what makes
        // ingestion IDEMPOTENT: re-ingesting an issue finds the Statement already carrying its URL and
        // refuses, rather than recording the same words twice under two ids. It is also the only way a
        // reader can go back to the source and check the transcription, which is the entire point of
        // recording verbatim.
        attribute sourceUrl : String [0..1];
        // WHAT AUTHORITY THESE WORDS CARRIED WHEN THEY ARRIVED (D0264). Recorded ON the utterance
        // rather than decided at act time, for two reasons: a repository's visibility can CHANGE
        // after an issue is filed, so re-deriving it later answers a different question than the one
        // that matters; and without it a Statement from a stranger on a public tracker is
        // indistinguishable from one from the maintainer, which is precisely the distinction an
        // autonomous pass must not have to guess. `[0..1]` because a spoken statement has no
        // external source to trust or distrust.
        attribute sourceTrust : SourceTrust [0..1];
        // Only DIRECTIVE statements are recorded: a request, a correction, a constraint, a priority, a
        // verdict. Not a transcript — a log of everything said is a second corpus with no questions
        // attached, which is the mistake D0161's first draft made.
    }

    // The faithful translation of a Statement into the form work is planned in.
    //
    // `asA` / `iWant` / `soThat` are the narrative; `implicates` is the triage verdict. Both are MINE,
    // and both are checkable against the Statement they cite — which is the whole reason the two are
    // separate items rather than fields on one.
    part def UserStory :> Element {
        attribute asA : String;                    // the role, from THEIR framing where they gave one
        attribute iWant : String;                  // the capability
        attribute soThat : String[0..1];           // the outcome; absent when they did not state one
        attribute implication : ImplicationKind;   // the triage verdict (the KIND)
        attribute triageNote : String[0..1];       // why THIS kind and not a neighbouring one
        // Edges carry the rest: `#DerivedFrom` to the Statement it translates (required — a UserStory
        // with no source is an invention), and `#Implicates` to whatever downstream item was created.
        // A story whose `implication` is not `none` and which reaches no downstream item is UNROUTED,
        // and `keel intake` reports it.
    }
}
