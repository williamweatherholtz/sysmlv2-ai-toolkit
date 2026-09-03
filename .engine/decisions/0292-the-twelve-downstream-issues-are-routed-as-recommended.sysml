// D0292 (status: the `status` field below is the truth) — The twelve downstream issues (GH#42-#53) are routed as recommended on the owner's word: eleven become tracked Issues with resolver tasks, one is verified already fixed
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0292Accept event on sign-off.
package Decision0292 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0292 : Decision {
        :>> id = "0d69ff92-7f02-45ae-b180-378e397fbe77";
        :>> title = "The twelve downstream issues (GH#42-#53) are routed as recommended on the owner's word: eleven become tracked Issues with resolver tasks, one is verified already fixed";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Twelve open issues on the public repository, all filed by the owner from downstream projects (Penumbra, se-templates), were pulled as verbatim untrusted Statements (st076-st087, D0264) and triaged into stories with a recommendation each (us078-us089). The tier says a human accepts before anything becomes work. The owner's word (st089): 'all work. these are essentially from me'. They are - every one was filed by the declared decider - which is the visibility-is-a-proxy case D0264 names: a public repository whose only filer is its owner.";
        :>> decision = "Route as recommended. Eleven Issues with resolver tasks: hooks resolve the pin before PATH (issue348, dcHooksResolveThePinFirst, High); migrate preserves project-authored contract sections (issue349, High); adoption-check reports the real verdict (issue350, High); claude-surface-drift ignores line endings (issue351, High); forward-only cutoffs are the adopting project's own (issue352, High); append-result finds the closing brace (issue353, High, joined by issue354 found while routing: add-task's def lookup breaks on a brace inside a DoD string, same brace scan); a deactivated process's skill is deployed stating its state, option B (issue355, Medium); the red-yield obligation keeps its diagnostic (issue356, Low); the surface stamp names the build (issue357, Medium); the Stop advisory becomes one line (issue358, Low); a TTY accept cites the gesture (issue359, High). GH#42 is verified fixed at this vintage - keel deactivate knowledge-graph-memory succeeds on a fresh scaffold and reads back INACTIVE - and is closed without an Issue. Each story carries #Implicates to its Issue and to this Decision; the GitHub issues are closed with their tracked ids, as the owner, so nothing notifies.";
        :>> rationale = "WHY A DECISION AND NOT ELEVEN ACCEPTANCES: D0264 asks that an untrusted story reach work only through a Decision a human accepts; one Decision over the batch, carrying the owner's word as its receipt, is that acceptance for all twelve, and guard 56 sees it among every story's targets. WHY THE ORDER OF SEVERITY: six of the eleven are High because each is a control or write path that is silently wrong downstream - a gate always red (D0214), a write that corrupts its file (issue325 class), a rule that retro-fails an adopter (issue068 class), a pin the turn does not honour (D0251 class), a human's own acceptance going red on the path the docs prescribe. WHAT WAS FOUND WHILE ROUTING: the brace-scan defect (issue354) - the write path locates blocks without honouring string literals - bit the routing itself when a DoD quoted the very text GH#48 describes; the DoD was reworded and the defect recorded rather than hidden.";
        :>> consequences = "Eleven items enter the frontier; their ranking is by declaration and should be read back. issue347 (record-time consent could launder an untrusted story) is untouched by this: these twelve were accepted by the human's word, not by standing consent. The reporter reads the tracked ids where they filed. REMOVAL PATH (D0163): a superseding Decision; the Issues then need a mooting resolver each.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0292Accept : Test { :>> id = "f759af7f-9910-48aa-90d4-c37f101e00f8"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0292AcceptR1 : TestResult { :>> id = "5f9432d3-e5b7-4b23-8ad4-6176b12a6eb1"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4466ab9"; :>> judgedAt = "2026-09-03"; :>> judgedBy = "wweatherholtz"; }

    // QUOTE RECEIPT (D0198 shape): the owner's word that routes the twelve.
    verification d0292AcceptAttest1 : Test { :>> id = "4aa32bda-2f3a-4429-8cab-ea1c3d9a1339"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz, 2026-09-03, in chat, verbatim: 'all work. these are essentially from me' (st089) - given against the twelve-row triage table with a recommendation per issue. The acceptance above is standing consent at record time; this receipt is their word on the same decision."; }
    part d0292AcceptAttest1R1 : TestResult { :>> id = "1fcd99b0-0e56-42cc-ab62-67b4b799da4f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "4466ab9"; :>> judgedAt = "2026-09-03"; :>> judgedBy = "claudeOpus5"; }
    #DerivedFrom dependency from d0292 to st089;
}
