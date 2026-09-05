// D0327 (status: the `status` field below is the truth) — distinctivenessFloor: recall pushes only when the prompt carries an identifier or a word in fewer than a tenth of the model, so short domain words seed and a prompt of only function words pushes nothing
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0327Accept event on sign-off.
package Decision0327 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0327 : Decision {
        :>> id = "39d3b47f-4c80-4f44-9fa9-a7dd6dfefd10";
        :>> title = "distinctivenessFloor: recall pushes only when the prompt carries an identifier or a word in fewer than a tenth of the model, so short domain words seed and a prompt of only function words pushes nothing";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue294 / shortDomainWordsCanSeed: the DoD believed a token under five characters was dropped for length; on the binary the floor was already three, so gate, hook, land, push and view seeded - but so did that, this and with, and a prompt of only those pushed a row over 4,574 scored elements. The DoD asked for the trade to be MEASURED with the eight-case comparison, not guessed. Measured: naming the three- and four-letter function words as filler took the eight-case set from 5/8 to 3/8 - `who` is what d0219 (deciders) is about and `where`/`when` carry weight in this corpus - and 4/8 with the question words kept, because even `the` and `and` (IDF about 1.2 at 30 percent of the corpus) shift a marginal ranking.";
        :>> decision = "The filler list stays at its 55 English words; instead the push is gated on DISTINCTIVENESS: a prompt pushes only when at least one of its words has a document frequency below DISTINCTIVE_DF_SHARE = 0.10 of the model, or it names an identifier; otherwise the payload says every term is common and pushes nothing. Frequency decides, as BM25 already does for weighting: gate (45 percent) alone would not push, hook (3 percent) does, `that this with` (14-18 percent each) does not.";
        :>> rationale = "Why a floor and not a longer list: the list is language and is meant to stay language (D0243 rule 3); the words that broke the hand set are half domain word in this corpus, and no list can know that - frequency can. Why 10 percent: the four-letter domain words the DoD names sit between 2 and 8 percent except gate at 45, and the function words between 14 and 30; the floor sits in the gap, and the eight-case set reads 5/8 with it - unchanged from before the change, which is what the DoD required (not below 5/8). Read back on the binary: `that this with from and the been does` pushes nothing and says why; `gate hook land` pushes; the 50-case set is unchanged at 48/50 in 13 rows.";
        :>> consequences = "Prompt-path pushes on prompts made only of common words stop; `keel recall` and `keel why` say so instead of listing a common word's 5,000 hits. issue294 is resolved by shortDomainWordsCanSeed. The three rejected list variants and their hand-set scores are in the source beside the constant. NOT A FORK: this Decision chooses the floor; the list variants it names are the measurements that rejected them.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0327Accept : Test { :>> id = "6fa376d1-7302-4e8f-ae76-284b961165a1"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0327AcceptR1 : TestResult { :>> id = "88a8dbbe-84e7-4d0c-ac9e-05e29c5047d4"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "73ae2a2"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
