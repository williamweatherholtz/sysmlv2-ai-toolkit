// D0203 (status: the `status` field below is the truth) — shortNames: every item gets a plain short name, and every signature request carries its own deciding context
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0203Accept event on sign-off.
package Decision0203 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0203 : Decision {
        :>> id = "7d528f23-ab99-429e-a317-65fabf6e3e7a";
        :>> title = "shortNames: every item gets a plain short name, and every signature request carries its own deciding context";
        :>> createdAt = "2026-08-23";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Their words, 2026-08-23, verbatim: 'going forward, create a short name for each item. the shortname should be referenced (not the ID) and should be descriptive of the main value or pain point of an item/decision/issue. very often, it's not even clear what's really being proposed because there are so many references and obscure change or value implications' - and 'there is not sufficient context to make these decisions generally.' IDs and reference-dense prose made proposals unjudgeable; the process-value panel's rubber-stamp finding was partly this: the surfaces asked for judgment while withholding what judgment needs.";
        :>> decision = "THREE CLAUSES. (1) SHORT NAMES: every new Decision, Issue, and human-queued item gets a short name describing its VALUE or PAIN POINT in plain words (tap-to-sign, review sampling, blind-spot fence), leading its title; chat, reports, the deck and the dossiers reference the short name, with the ID at most parenthetical. (2) DECIDING CONTEXT TRAVELS: anything presented for a signature carries, on the presenting surface itself, plain-language answers to what changes, what it costs, and what rejection means - no unexplained cross-references; the deck's decision cards render the decision's own context and decision text in a collapsible panel (shipped with this decision). (3) EXISTING queue items are re-presented under this convention rather than re-authored - titles of signed history are never rewritten.";
        :>> rationale = "A name that carries the pain point does half the deciding-context work before a single sentence is read, and a card that carries its own why makes the signature a judgment instead of a trust gesture - the exact gap 4 of 9 panel lenses named. The convention costs nothing at authoring time and pays at every human touchpoint, which is D0054's friction economics applied to the supervisor instead of the author.";
        :>> consequences = "New titles lead with short names; the deck renders why-panels on decision cards (live); chat re-presentations use short names; the D0200 OPTION A work inherits clause 2 for sitting cards. Rejecting would leave the queue exactly as unjudgeable as they described.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0203Accept : Test { :>> id = "48b077c7-68f3-45ed-87d8-37fc4f31bf06"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Accepted on their explicit direction. Their words, verbatim: 'going forward, create a short name for each item. the shortname should be referenced (not the ID) and should be descriptive of the main value or pain point of an item/decision/issue' (chat, 2026-08-23). The decision records exactly that direction; the deck why-panels ship with it."; }
    part d0203AcceptR1 : TestResult { :>> id = "f6d05606-2e06-47f8-842a-34f9d3cbe48c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "71f6453"; :>> judgedAt = "2026-08-23"; :>> judgedBy = "wweatherholtz"; }
}
