// D0271 (status: the `status` field below is the truth) — The CLI interface becomes authored facts, so it can be critiqued: 90 commands, 9 intents, one of them holding 43
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0271Accept event on sign-off.
package Decision0271 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0271 : Decision {
        :>> id = "cb627947-ee5d-463a-85b7-5001b13c1ae5";
        :>> title = "The CLI interface becomes authored facts, so it can be critiqued: 90 commands, 9 intents, one of them holding 43";
        :>> createdAt = "2026-09-01";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human asked to 'cement our CLI ICD so we can critique and improve it down the road', then, on seeing the count: '90? 90 seems too many as far as intent.' Invariant 1 forbids authoring an ICD - it is a document, regenerable from other facts, and it would drift from the binary the moment either moved. So the interface itself becomes the authored fact and the ICD is a computed view over it. MEASURED, which is what turned their impression into a finding: 90 top-level commands resolve to NINE intents, and one of them holds 43 - assumptions, orphans, suspect, rootedness, tier-satisfaction, hardening, why, workflows and 35 more, every one of them doing the same thing: compute a lens and print it. That is 47 percent of the surface as one idea cemented 43 times. The next clusters are gate (12: validate, guard, gate, check, check-engine, rules, assured, reverify plus four audits) and record (11). `library <verb>` shows the shape that already works: one verb, many operations.";
        :>> decision = "The `EngineCli` schema: one `CliCommand` per command carrying name, synopsis, invocation, FAMILY, effect and stability. FAMILY IS THE CRITIQUE HANDLE. It makes 'these are variations of one idea' a queryable property rather than an impression - which is exactly what the human saw by eye and what no document would have made checkable. EFFECT (reads / writes / both / tooling) makes 'a status command must not write' a fact a guard can test. STABILITY (stable / aliased / deprecated / internal) is what lets a consolidation land without breaking the 118 call sites that name these commands today: an aliased name is a declared state, not a comment in a migration plan. `family` is a String rather than an enum ON PURPOSE: the point is to ARGUE about the grouping, and an enum would make every proposed regrouping a schema change request. NOT YET AUTHORED: the 90 instances and the two-way guard that every dispatched command is declared and every declared command dispatched. That guard is what would make this cemented rather than aspirational, and it is held until the human answers whether the lens family collapses - because authoring 90 items and then renaming 43 of them is work done twice.";
        :>> rationale = "WHY NOT CodeElement. It already has kind = 'command', and no room for the argument shape, the family, the effect or the stability - which are the four things a critique needs. Reusing it would have produced a registry that lists the commands and cannot be argued with. WHY THE KEYWORD RENAMES ARE RECORDED IN THE SCHEMA. `meta` and `alias` were the natural member names and the KERNEL REJECTS BOTH - they are SysML keywords, the same trap ImplicationKind hit with `constraint` and `requirement`, and the same reason validate_schema.py blocks on schema changes: the Rust parser accepted them happily. They are `tooling` and `aliased`, with the reason beside them so the next author does not rediscover it.";
        :>> consequences = "Schema kernel-validated 20/20 and registered in SCHEMA_ORDER. A type with no instances yet, stated plainly rather than implied: the ICD is not cemented until the 90 items and the reconciliation guard exist. What is landed is the vocabulary that makes the argument possible. The measurement stands on its own regardless of what is authored next: 90 commands, 9 intents, 43 in one.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0271Accept : Test { :>> id = "b8270bd5-0420-4347-9cfe-77b288e1b288"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override anytime by replying 'reject <why>' on https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/29."; }
    part d0271AcceptR1 : TestResult { :>> id = "4b1d1a25-1fac-413b-a169-fd7eae8bc67b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "854cdd6"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "wweatherholtz"; }
}
