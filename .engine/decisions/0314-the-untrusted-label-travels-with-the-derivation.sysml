// D0314 (status: the `status` field below is the truth) — untrustedTaint: the untrusted label follows derivation to every sink, and only a human's acceptance - or a declared decider speaking - takes it off
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0314Accept event on sign-off.
package Decision0314 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0314 : Decision {
        :>> id = "de2772f7-c134-496e-ab4a-b4a640923f1d";
        :>> title = "untrustedTaint: the untrusted label follows derivation to every sink, and only a human's acceptance - or a declared decider speaking - takes it off";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0264 made autonomy follow repository visibility: a public issue is an instruction from an unauthenticated stranger, so it may PLAN and a human accepts before anything is built; guard 56 enforces that routing for the first hop, an untrusted story to a Decision. issue347 (from the landscape spike's CaMeL finding, D0282) named the laundering: record-time standing consent (D0291) auto-accepts a non-fork Decision, so untrusted story -> proposed Decision -> auto-accepted -> chartered task -> built, and nobody read the stranger's instruction. Three hops and the label is gone. The DoD dcUntrustedTaintClosure asked for the closure to be computed and refused.";
        :>> decision = "Guard 63 `untrusted-taint` (hard) computes the taint closure from every `SourceTrust::untrusted` utterance over `#DerivedFrom`, `#Implicates`, `#CharteredBy` and `#Resolves` edges and fails the gate when a tainted item is a Decision whose acceptance was AUTOMATIC (standing consent) or a task that is DONE, unless the path crosses a Decision a HUMAN accepted (their quoted words, the console, a terminal - where the walk stops because trust is conferred there) or the utterance's `saidBy` is a GitHub login that `github-actors.toml` maps to a declared decider (their own issue on their own public repository is the human's direction, not a stranger's instruction; the `untrusted` label on the record stays as recorded, since it is a visibility proxy and D0264 keeps it exactly as written).";
        :>> rationale = "Why a closure and not a wider guard 56: the first hop is already policed; the defect is that the label does not travel, so the control must be the traversal itself, and it must name the whole path so the reader sees where trust should have been conferred. Why AUTO-accepted is the violation and proposed is not: planning is the tier untrusted input is allowed (D0264), and a proposed Decision is a plan; the failure is an acceptance nobody gave. Why a declared decider's own words confer trust at the source: `github-actors.toml` is the committed login-to-human table the decision channel already trusts to attribute an acceptance (D0219), and GitHub authenticated the login; the first live run found exactly three violations - d0268, d0292 and dcUnknownFlagIsRefused - every one descended from an issue williamweatherholtz filed themself (st052, st076, st058; the human said of that batch 'all work. these are essentially from me', st089), so refusing them would police the human, which governance never does. Measured after that rule: 26 untrusted utterances, 0 violations. Arming (D0298): a fixture with the DoD's exact chain is red naming the path and green with a human acceptance or a decider speaker - on the binary, in keel-cli/tests/untrusted_taint.rs.";
        :>> consequences = "GUARD_NAMES is 63; constraint def untrustedTaint, guards.md row, control-map anchor gUntrustedTaint (ehz2, ehz3) land with this Decision. A stranger's issue that reaches an auto-accepted Decision now fails the commit until a human accepts it in their words (D0289). STATED RESIDUAL: a guard, contract or hook edit is not a model item and carries no edge; a tainted task that changes the enforcement surface is caught only as a done task, and the keystone lock (D0209) is what requires its Decision. issue347 is resolved by dcUntrustedTaintClosure.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0314Accept : Test { :>> id = "8249f84d-1eb7-4d4d-8b95-40cb918feee7"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0314AcceptR1 : TestResult { :>> id = "274c4619-8b43-411f-8532-08c2537d17f7"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "9549309"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
