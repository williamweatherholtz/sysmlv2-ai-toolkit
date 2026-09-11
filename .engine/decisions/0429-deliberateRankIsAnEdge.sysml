// D0429 (status: the `status` field below is the truth) — deliberateRankIsAnEdge: a deliberate priority inversion is recorded by a #PrioritizedBy edge from the item to the Statement or Decision that ranks it, and the priority-inversion guard honours the record
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0429Accept event on sign-off.
package Decision0429 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0429 : Decision {
        :>> id = "1d0f816c-2243-4382-8fcf-6b372ff5bdfd";
        :>> title = "deliberateRankIsAnEdge: a deliberate priority inversion is recorded by a #PrioritizedBy edge from the item to the Statement or Decision that ranks it, and the priority-inversion guard honours the record";
        :>> createdAt = "2026-09-10";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The priority-inversion guard warns when a ready item is declared above work whose computed class is High or Critical, and its text ends 'if that is deliberate say so'. Nothing it reads is a place anyone could say it: refineAssessPriority tells the refiner to 'say in the block comment why an inversion is deliberate', and a block comment reaches no guard. MEASURED 2026-09-02 (issue345): ranking the human's stated first step (st066) at the head of the frontier left 30 warnings standing although the reason was a recorded Statement; measured again 2026-09-10, 27 warnings stand over one High item. A warning nobody can discharge is the manual-vigilance shape D0047 rejects, and it trains blindness to the warnings that can be acted on.";
        :>> decision = "NOT A FORK: this gives the guard's own request a home and changes no severity. A deliberate inversion is recorded as a typed edge, `#PrioritizedBy dependency from <item> to <stNNN|dNNNN>;`, whose target is the Statement or Decision that ranks the item. The priority-inversion guard does not warn on an item carrying such an edge; `keel show priority` lists the item under `recorded` with its citation and carries `rankedBy` on the row. An edge whose target is any other type is a violation of the guard, because a citation to something that is not a recorded word or judgment records nothing. An item ranked above High or Critical work with no such edge still warns as before. `metadata def PrioritizedBy;` joins the marker vocabulary in schema/core/relationships.sysml, and refineAssessPriority names the edge in place of the block comment.";
        :>> rationale = "D0052 makes declaration order the priority and the guard makes disagreement with the computed signals visible; what was missing is the third leg - a recorded reason the guard can read. An edge is the one form that fits every invariant already in force: it is an irreducible fact (section 1.1), it is typed (1.2), it is a citation to a Statement or Decision so the reason is the human's words or a recorded judgment rather than the agent's prose, and its presence is queryable so 'is this inversion deliberate' stops being a question about a comment. Restricting the target to Statement or Decision is what keeps the record honest: an edge to a Task or an Issue would let an item rank itself.";
        :>> consequences = "Guard priority-inversion: an item carrying a #PrioritizedBy edge to a Statement or Decision is exempt from the outranks warning; an edge to any other type is a violation. keel show priority: rows carry rankedBy, and a recorded array lists every honoured record. Schema: PrioritizedBy is declared in relationships.sysml (frozen, so this Decision is the CR). Process: refineAssessPriority's actionText names the edge. Enforcement surface: guards.rs changes, so this Decision is the co-committed process-change record and stays proposed for the human under D0337; the code lands with it because the lock checks the marker's presence.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0429Accept : Test { :>> id = "dbdd6ccb-92a8-4547-962e-dd3eeb108645"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Selected on the decision brief copy-out (fifteenth publish), pasted into chat 2026-09-11: the set of 33 HELD process-change ratifications accepted with one word, this record being one member (D0406 named it in the table). No note added. - their words, verbatim: “Accept the set: each of the 33 rules named in the table is recorded accepted on its own, quoting these words (recommended)” WARN: read-back - the quoted words do not name d0429 (D0201 B); recorded as given (D0423)."; }
    part d0429AcceptR1 : TestResult { :>> id = "6c1b32e9-4cb9-4ff8-b877-4b5ad4549f9f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3e6c518"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
