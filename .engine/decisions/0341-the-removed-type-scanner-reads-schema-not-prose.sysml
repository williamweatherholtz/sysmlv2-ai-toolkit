// D0341 (status: the `status` field below is the truth) — removedTypeScannerReadsSchemaNotProse: migrate's removed-type and removed-enum scans run on the line with its string literals blanked, so a quoted foreign symbol never blocks a migration
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0341Accept event on sign-off.
package Decision0341 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0341 : Decision {
        :>> id = "4236ae62-7925-4beb-8b10-1876ac361343";
        :>> title = "removedTypeScannerReadsSchemaNotProse: migrate's removed-type and removed-enum scans run on the line with its string literals blanked, so a quoted foreign symbol never blocks a migration";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#59 (st092, us091, issue377). A downstream project's authored Issue quoted a critic's finding - 'get() uses a concrete ureq::Agent with no injection point' - and keel migrate refused the whole project: types_as treats any text ending in a colon before the name as a typing colon, so Rust's path separator passed, and step_removed_types scanned every line in full, prose included; the REMOVED_ENUMS scan used line.contains the same way. No override existed, and each workaround corrupted something - the critic's words, the schema, or the project's ability to move vintage. Every REMOVED_TYPES name that is an ordinary word (Task, Risk, Role) was exposed identically. The reporter implemented and falsified the fix locally against d82c512.";
        :>> decision = "step_removed_types scans each line with its double-quoted string literals blanked (structure preserved, escaped quotes honoured) and reports the original line; both the type and the enum scans read that blanked form. A type annotation never appears inside a string literal, so nothing the scanners are entitled to see is lost, and a qualified reference outside quotes still matches. NOT A FORK: the reporter's shape is the only one that reads schema without reading prose.";
        :>> rationale = "Why blank rather than teach types_as about ::: the colon test is right for SysML and the failure is reading text that is not SysML; fixing the reader's scope fixes every removed name at once, where a :: special case fixes one spelling. Why in step_removed_types only: the other steps rewrite declarations they positively match and were never fooled by prose. Arming (D0298): a unit test with both arms - the prose form must not block and the same names as real references outside quotes must still block, so a passthrough helper fails it with the reason it names - plus the existing old-vintage arms (the Task annotation and the RiskLevel::high value still block).";
        :>> consequences = "The reporter's project migrates past the blocker. issue377 resolves on dcRemovedTypeScannerReadsSchemaNotProse's pass. Unchanged: a removed type actually declared outside quotes still has no mechanical transform and still blocks by name.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0341Accept : Test { :>> id = "af45ae27-d8e4-4f3e-8d64-413fc516b393"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0341AcceptR1 : TestResult { :>> id = "5e68fd79-92f3-4cd9-83bf-eb777cc48871"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5e3a828"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
