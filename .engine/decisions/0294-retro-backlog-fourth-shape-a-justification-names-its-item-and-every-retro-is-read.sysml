// D0294 (status: the `status` field below is the truth) — retro-backlog, fourth shape: a no-item justification must name an existing item, and every retro gate is actually read - the extraction that skipped 27 of 510 is replaced
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0294Accept event on sign-off.
package Decision0294 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0294 : Decision {
        :>> id = "449b0ee3-9ddc-4ed1-b2a9-401c96adb252";
        :>> title = "retro-backlog, fourth shape: a no-item justification must name an existing item, and every retro gate is actually read - the extraction that skipped 27 of 510 is replaced";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0293 registered retro-backlog as under-reporting (issue364): 'already tracked' passed by substring with nothing checked. Building the fix's arming test exposed a second under-report in the same guard: retro_texts split the sprint file on the word 'verification ', so a retro whose text contained that word - the commonest word in this repository - was cut in half and never examined. Counted: 27 of 510 retro gates were never read by the guard, 20 of them carrying no justification at all. The guard passed retros it had not seen.";
        :>> decision = "Two changes to guards.rs under the keystone lock. (1) A justified retro ('no new item', 'no item needed', 'already tracked', 'no further item') must NAME an item that exists - an issueNNN, a dcTask, or a dNNNN Decision, added by the commit or already in the tree - or it is a violation quoting the phrase; a Decision counts because a 'won't do' IS a Decision (Invariant 4). Relevance stays the reader's judgment; that the claim points at something real is the guard's. (2) Blocks are extracted from lines that BEGIN with 'verification ', never from the word inside a string, so every retro gate is read. Twelve unit tests hold the shapes, including the two retros that motivated D0293 and a retro that mentions verification. The control-defects.toml entry for retro-backlog leaves in this commit, as the control-defect process requires; the 27 skipped retros are not re-checked and issue364 says so.";
        :>> rationale = "WHY NAME-AN-ITEM AND NOT A RELEVANCE CHECK: no guard can read whether issue336 covers a finding; a guard can read whether issue336 exists. The two retros that motivated this named real items and would pass the form check - and that is stated in a test rather than hidden, because the honest claim is that the guard now catches the empty justification, not the wrong one. WHY THE EXTRACTION BUG MATTERS MORE THAN THE JUSTIFICATION BUG: a justification loophole needs the writer to reach for it; the extraction bug needed only the word verification, and it silently excluded a retro from the one check that reads retros. Twenty unjustified retros passed that way. WHY IN ONE COMMIT: the second defect was found by arming the first fix; landing the first alone would have recorded a green guard that still did not read every retro.";
        :>> consequences = "retro-backlog reads every retro gate and refuses an unverifiable justification. The registry no longer names it. A retro that says 'already tracked' must now say tracked WHERE, which is the sentence that would have saved four findings this week. REMOVAL PATH (D0163): a superseding Decision; the guard reverts to the third shape and re-enters the registry, since the second shape's defect would return with it.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0294Accept : Test { :>> id = "80cfff96-472f-428f-9330-ca695c49f84b"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0294AcceptR1 : TestResult { :>> id = "7b676d30-9528-4ab6-9c13-c71599e29544"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4506a05"; :>> judgedAt = "2026-09-03"; :>> judgedBy = "wweatherholtz"; }
}
