// D0352 (status: the `status` field below is the truth) — recordDecisionDeclaresSupersedes: keel record decision takes --supersedes and --derived-from and authors the typed edges with the Decision, refusing a target that does not exist
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0352Accept event on sign-off.
package Decision0352 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0352 : Decision {
        :>> id = "66fff975-25cf-4c25-bc1f-936d9c724818";
        :>> title = "recordDecisionDeclaresSupersedes: keel record decision takes --supersedes and --derived-from and authors the typed edges with the Decision, refusing a target that does not exist";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "dcRecordDecisionDeclaresSupersedes. Found 2026-09-03: D0289 reversed D0178's third layer and carried no #Supersede edge to it; the human noticed, not a guard, because decision-requirement-link warns only when prose names an item by id and D0289's text did not. A reversal that lands edgeless is a supersession the model cannot compute - keel why, the scope views and the taint walk all read edges, not prose. The same gap holds for a Decision's source: a Decision that came from a Statement or UserStory (D0216) had to have its #DerivedFrom edge hand-authored afterwards, in the intake file, by a second write.";
        :>> decision = "keel record decision accepts --supersedes dNNNN[,dNNNN] and --derived-from stNNN|usNNN[,...], and a --from draft the header lines supersedes: and derived-from:; the Decision file is written with #Supersede dependency from dNEW to dNNNN and #DerivedFrom dependency from dNEW to stNNN lines inside its package, in the same atomic write; before anything is written every target is checked - a #Supersede target must be a Decision on disk, a #DerivedFrom target a declared Statement or UserStory - and a miss refuses naming the flag and the target; the record-time acceptance path is unchanged. NOT A FORK: the edge belongs with the fact it qualifies, and the write that creates the fact is the one place that can guarantee it.";
        :>> rationale = "Why refuse rather than warn on a missing target: an edge to nothing is worse than no edge, because every reader treats it as present (issue109's lesson for record issue). Why restrict derived-from to utterances and stories: D0216 makes a Decision's source a recorded word or the story that translates it; an edge to an arbitrary item would let the field say something the vocabulary does not mean. Arming (D0298): three binary tests on a scaffold - a reversal recorded with --supersedes carries the edge in its own file, validates, and keel why reads it back; a missing target refuses and writes nothing; a --from draft's two header lines author both edges and --derived-from to a Decision is refused.";
        :>> consequences = "A Decision recorded as a reversal carries its edge from birth; the intake file no longer needs a second write for a Decision's source. dcRecordDecisionDeclaresSupersedes is done on the tests above. Existing edgeless reversals are history and are not re-authored (D0129).";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0352Accept : Test { :>> id = "5313959c-006c-48e0-a859-0618034f521f"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0352AcceptR1 : TestResult { :>> id = "9c59b2ce-6e40-4ab9-ba3b-dfebe11c01fe"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a66324c"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
