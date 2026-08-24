// D0206 (status: the `status` field below is the truth) — retireInbox: switch off the Smartsheet tap inbox once the GitHub channel has proven itself on this very decision
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0206Accept event on sign-off.
package Decision0206 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

// #ProspectiveChange (D0070): governs the inbox retirement - the deck-inbox.toml deletion and the
    // obligation-review skill re-point (sprint 437). Marker added at APPLICATION time; the signed
    // acceptance below is untouched.
    #ProspectiveChange part d0206 : Decision {
        :>> id = "6f87e488-a41c-4e30-8c77-81d30dc9b4b1";
        :>> title = "retireInbox: switch off the Smartsheet tap inbox once the GitHub channel has proven itself on this very decision";
        :>> createdAt = "2026-08-23";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "githubChannel (D0205, accepted) replaces the artifact-deck-to-Smartsheet tap path whose missing feedback loop caused the it-doesnt-seem-theyre-saving failure. This decision IS the dry run: it was proposed, an Action opened it as a GitHub issue, and if you are reading it there, the new channel carried it to your phone end to end.";
        :>> decision = "Retire the Smartsheet inbox as a decision channel: the deck's remote tap-save path and the inbox polling loop stand down; the sheet stays readable as history; keel deck survives as a computed view whose cards link to GitHub issues. Accepting FROM THE GITHUB ISSUE is both the approval and the proof - the receipt comment and auto-close you see are the feedback loop the old channel lacked. Rejecting keeps the Smartsheet path alongside GitHub until you say otherwise.";
        :>> rationale = "One channel with receipts beats two channels where one cannot acknowledge. The cutover is gated on its own successful use, so the old path is never removed on faith.";
        :>> consequences = "On acceptance: the obligation-review skill re-points at the issues board; the inbox sweep retires from session routine; deck tap buttons become issue links. The sheet is not deleted - history stays readable.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0206Accept : Test { :>> id = "c468635f-827f-4e14-8c44-0846e00a8578"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/2."; }
    part d0206AcceptR1 : TestResult { :>> id = "527b4c00-8396-4bc9-8e54-19bfcec94b74"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ae0197e"; :>> judgedAt = "2026-08-23"; :>> judgedBy = "wweatherholtz"; }
}
