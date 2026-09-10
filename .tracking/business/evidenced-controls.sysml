// =====================================================================
// EvidencedControlsNeeds — a control binds the actor that has subverted the process, and is kept on
// the evidence that it did something.
//
// SOURCE. st115 (2026-09-10: "guards that handcuff me and do very little to AI agents ... are there
// indications that agents have attempted to accept decisions under my authority? if not, then this
// guard is moot and should be removed along with others like it ... users like me shouldn't be the
// entities strictly regulated here ... build evidence around the ways processes are subverted today"),
// translated as us105 (controls bind the AI, kept on evidence) and us106 (the evidence feeds STPA and
// comes out as needs, use cases and requirements). Every Need here cites the story that cites the words.
//
// WHAT THE RECORD SAID WHEN ASKED (issue445, issue446): no refusal by any control is recorded anywhere;
// the fire-ledger writes every in-loop block as an allow; the acceptance guards' origins are a vigilance
// gap (issue059), a code-read UCA (issue426) and eight refusals of the human's own correct acceptances
// (issue397). The subversions the record DOES hold are all the agent's: issue267, issue266, issue256,
// issue315, issue322, issue400, issue434.
// =====================================================================
package EvidencedControlsNeeds {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    part nControlBindsTheSubvertingActor : Need {
        :>> id = "af1bdb8d-bcb6-4a5a-a6df-0d5fce8c45e0";
        :>> title = "A control that refuses a human's act is kept only on evidence of the act it prevents";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "Every control in the engine can be read as binding a human act, an AI act, or either; each carries the incidents that motivated it, classified observed (the act happened) or code-read (found by inspection), and the blocks it has made, by the kind of actor stopped. A control that binds a human act and shows no observed incident and no block of an AI actor is a candidate for removal, surfaced to the human with the evidence, one Decision each. MEASURABLE: the census answers 'has an agent ever attempted X' for any control from the ledger, not from memory; the count of human-binding controls with no evidence trends to zero; no refusal anywhere leaves no trace. WHAT THIS EXCLUDES: it does not remove a control that binds the AI, however irritating - the record of AI-side subversion is the reason those exist.";
    }

    part nSubversionEvidenceGroundsTheAsks : Need {
        :>> id = "be7ebf50-8874-432d-a9ba-ec9d79d69e37";
        :>> title = "What the process asks the human to decide is grounded in how the process was actually subverted";
        :>> createdAt = "2026-09-10"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "The STPA run over this project's own control structure carries, per unsafe control action, whether the act has been observed or only reasoned about, and the implementation asks that come out of a run trace to a Need, a use case and a requirement before they reach the human. MEASURABLE: every UCA in the run record carries an evidence class computed from its Issue links; every proposed Decision that adds or removes a control names the UCA and the incident or its absence; a run that adds only code-read UCAs says so in its record. WHAT THIS EXCLUDES: a hypothetical UCA is still recorded - it is labelled, not dropped - because the analysis is where a control's absence of evidence becomes visible.";
    }

    #DerivedFrom dependency from nControlBindsTheSubvertingActor to us105;
    #DerivedFrom dependency from nSubversionEvidenceGroundsTheAsks to us106;
}
