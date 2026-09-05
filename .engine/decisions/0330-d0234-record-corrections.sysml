// D0330 (status: the `status` field below is the truth) — d0234Corrections: the six refuted claims of D0234's record, corrected as measured, by a Decision the author of the corrections owns
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0330Accept event on sign-off.
package Decision0330 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0330 : Decision {
        :>> id = "409868c1-e9c9-4da6-afe7-52fe60550d52";
        :>> title = "d0234Corrections: the six refuted claims of D0234's record, corrected as measured, by a Decision the author of the corrections owns";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue283 (the facts pedant on D0234, the workspace Decision): six claims refuted. D0234 is owned by claudeFable5; D0108 says a non-owner ADDS items and typed edges or SUPERSEDES, never overwrites in place, and the ownership guard held to it when the corrections were first written into D0234's own text - so they are recorded here, by their author, with a #DependsOn edge to d0234, and D0234's text stands as written with this Decision beside it.";
        :>> decision = "The record of D0234 reads with these corrections. The claim that a single-project repository behaves byte-identically is withdrawn - the multi-project predicate gates only message wording, and three of the four changes alter single-project behaviour too, namely the keystone lock's ADDED-versus-MODIFIED filter, validate's refusal of a non-project, and staged-path re-basing for a project nested in a larger git repository. Discovery was verified against two of the eight keel repositories on this machine, not against both real repositories. The no-nested-false-positives claim was vacuous, because the walk short-circuits at a root project and so examined no subdirectory in any repository. The two RAN receipts in sprint469 that cited commits 96e680e and 98456c1 in scratch repositories since deleted did not resolve and now name the suite tests that prove the same claims - land_gate.rs, init_smoke.rs, no_green_over_nothing.rs. The refusal message for a non-project now names the directory that is actually missing (workspace::missing_project_dirs). The installed-units.toml version bump 10 to 12 that rode commit f356770 was adoption_check_can_fail exporting from the live repository root (issue339, fixed later), not part of that sprint's work.";
        :>> rationale = "Why a Decision beside D0234 and not an edit: the ownership rule exists so that a record cannot be changed under its author, and a correction is the case where that matters most - the pedant's findings are recorded by the actor who made them, the original text stays legible as what was claimed, and a reader can hold the two side by side. Why every correction names its measurement: the six were refuted because they were counts nobody had made (both, none, byte-identical); each replacement names the mechanism, the machine count, the code path or the test that a reader can check. Why sprint469's receipts are edited in place: a `// RAN:` receipt is a comment, not an owned field, and a receipt that does not resolve is a D0129 violation inside the receipt mechanism itself; the corrected receipt says what it replaced and why. NOT A FORK: one course, the corrections; the six sentences are the pedant's six findings, not alternatives.";
        :>> consequences = "issue283 is resolved by dcPanelRecordCorrections. D0234's text is unchanged and its acceptance stands. The guard fix that let a correction of an OWNED record land in one commit (D0329) stands for the owner's own corrections. STATED: the model has no #Corrects edge; #DependsOn from this Decision to d0234 is the typed link, and the title says what it is.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0330Accept : Test { :>> id = "16877ff6-e8fb-445d-b80f-09e4542ed834"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0330AcceptR1 : TestResult { :>> id = "02513945-abe2-4636-acc5-1906ebbb8458"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "f84ff68"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
    // the record this Decision corrects (issue283); D0234 stays as written, owned by its author (D0108)
    #DependsOn dependency from d0330 to d0234;
}
