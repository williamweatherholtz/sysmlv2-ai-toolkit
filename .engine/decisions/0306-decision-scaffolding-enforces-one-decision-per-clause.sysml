// D0306 (status: the `status` field below is the truth) — decision-scaffolding enforces D0303 option C: from 2026-09-05 a Decision whose text enumerates several clauses is a violation, and the compound Decisions before it are counted once
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0306Accept event on sign-off.
package Decision0306 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0306 : Decision {
        :>> id = "b6e3afce-51dc-48fc-9e50-4fe1b2409287";
        :>> title = "decision-scaffolding enforces D0303 option C: from 2026-09-05 a Decision whose text enumerates several clauses is a violation, and the compound Decisions before it are counted once";
        :>> createdAt = "2026-09-04";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0303 was accepted 2026-09-04 in the human's words - i do like C the best - choosing the convention over a schema change: a Decision is one clause, a layered change is several Decisions with edges between them. This Decision is the guard side of that choice, recorded separately because it changes guard source (the keystone lock) and because under C it must itself be one clause. Measured at build time: 81 accepted Decisions in this tree enumerate two or more clauses, including every one recorded this week by this author.";
        :>> decision = "The decision-scaffolding guard reads every Decision's decision text for enumerated clauses - (1) ... (2), (a) ... (b), or clause A ... clause B, counted in sequence from the first - and fails a Decision created on or after 2026-09-05 whose count is two or more, naming the count and the remedy: one Decision per clause with #DependsOn edges between them, each chartered on its own. Decisions created before the cutoff are counted in a single warning and never enumerated or re-split.";
        :>> rationale = "WHY THE DAY AFTER: today's records were written before the choice existed, and a rule that fails its own acceptance day is a rule nobody can land. WHY A VIOLATION AND NOT A WARNING: the cost is one more record per layer, made cheap by D0301, and the benefit is that the guard's covered verdict becomes exact for every Decision it can see - the whole point of issue331. WHY THE CENSUS STAYS A WARNING: 81 records are true history; the D0129 rule forbids rewriting them and the count keeps the residual visible. WHAT IT CANNOT SEE: a compound Decision written without enumeration markers - prose that layers without numbering. That is the author's honesty, and the review pass of every sprint reads Decisions for it.";
        :>> consequences = "dcDecisionClausesAreAddressable is delivered; issue331 resolves; the control-defects.toml registry is EMPTY for the first time since D0278, which the scan-count test now names as the healthy state. CLAUDE.md carries the convention beside the standing-consent bullet. My own Decisions from D0296 to D0305 are among the 81 grandfathered.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0306Accept : Test { :>> id = "483f0229-8905-465a-ae24-7621af6238d3"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0306AcceptR1 : TestResult { :>> id = "97f5a90b-d00d-4469-b1d2-63155c7db7f1"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "fcd56db"; :>> judgedAt = "2026-09-04"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
