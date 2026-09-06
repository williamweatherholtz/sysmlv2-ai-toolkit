// D0351 (status: the `status` field below is the truth) — stopAdvisoryIsOneLine: the Stop hook's green advisory is one line in the owner's format - how many decisions are outstanding, their range, how to accept, and their short names - and silence when none
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0351Accept event on sign-off.
package Decision0351 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0351 : Decision {
        :>> id = "b076543d-3553-462a-8273-46a7cd89a986";
        :>> title = "stopAdvisoryIsOneLine: the Stop hook's green advisory is one line in the owner's format - how many decisions are outstanding, their range, how to accept, and their short names - and silence when none";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#52 (st077, routed under D0292; dcStopAdvisoryIsOneLine). The owner's words, quoted in the report: 'a single number indicating how many decisions are outstanding followed by an exec summary', with the wanted shape - 17 decisions outstanding (d9001..d9017) - accept from your terminal: keel accept dNNNN --by will. Covering: landscapeProcessCharter, designBasisSupplyFlow, ... The oversight nag that prescribed a console or a deck every turn was removed under D0269, which left the green turn boundary saying nothing at all - not the one line the owner asked for. Partly moot after D0289 and D0291, so Low; after D0337 marker Decisions stay proposed for the human, so the count is live again - seven outstanding on this tree when this was built.";
        :>> decision = "When the Stop hook finds the model green, it emits exactly one systemMessage line when any Decision is proposed - the count, the id range, how to accept (the quoted word in chat, or keel accept dNNNN --by <you> at their terminal), and the short names (the title's prefix before the colon) whole, with an ellipsis and a count past a 160-character budget - and emits nothing when none is proposed; the blocking path is unchanged. NOT A FORK: the format is the owner's, taken from their words.";
        :>> rationale = "Why in the binary and not a wrapper script: the reporter tried a wrapper and claude-surface-drift refused it, correctly - the advisory's content is an engine decision. Why silence on none: a green tree with nothing waiting owes the human no line. Arming (D0298): two binary tests on a scaffold that grants no consent - two proposed Decisions produce exactly one JSON line starting 2 decisions outstanding (d0001..d0002), naming how to accept and ending Covering: probeOne, probeTwo, with no console or deck words; a scaffold with none produces an empty stdout and exit 0; and the live run on this tree printed 7 decisions outstanding (d0337..d0346) with six names and (+1).";
        :>> consequences = "The turn boundary reports the human's queue as a count and names, once, in their words' shape. dcStopAdvisoryIsOneLine is done on the tests above.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0351Accept : Test { :>> id = "9ecf6a64-870d-48e2-9c9f-82616a125c1e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0351AcceptR1 : TestResult { :>> id = "16cee93d-abb6-4dc0-b896-4d28ae006d6d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a66324c"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
