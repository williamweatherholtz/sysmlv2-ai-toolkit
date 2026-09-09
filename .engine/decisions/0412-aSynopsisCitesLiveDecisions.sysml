// D0412 (PROPOSED — NOT YET ACCEPTED) — aSynopsisCitesLiveDecisions: a CliCommand synopsis that cites a Decision cites one that exists and is not retired - guard cli-surface-declared holds the citation to the tree in both fact homes
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0412Accept event on sign-off.
package Decision0412 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0412 : Decision {
        :>> id = "45819570-e422-495f-8280-129dbbd8e9c1";
        :>> title = "aSynopsisCitesLiveDecisions: a CliCommand synopsis that cites a Decision cites one that exists and is not retired - guard cli-surface-declared holds the citation to the tree in both fact homes";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "issue423, tracked as dcSynopsisCitesLiveDecisions. The suite command's synopsis, in .engine/cli/commands.sysml and its mirror cli_facts.rs, cited D0353 - the push gate - after D0356 had retired it by a #Supersede edge (D0398). The cli-surface-declared guard held the two homes equal to each other and to the dispatch, so the two stale citations agreed and nothing failed: keel --help pointed every reader at authority the tree had withdrawn, and the drift was found by a person reading the help. The retired set already existed as a computed fact (supersede_targets, D0398) and the Decision files are the ids that exist; no reader of the CLI facts consulted either.";
        :>> decision = "NOT A FORK: the DoD names one property and the guard is its only home. Guard cli-surface-declared extracts every dNNNN/DNNNN token from every synopsis in both fact homes - the authored .sysml and the cli_facts.rs mirror - and resolves each against the Decision files under .engine/decisions and the #Supersede edge set. A citation of a Decision that is the target of a #Supersede edge is a violation naming the home, the command, the id and the Decision that retired it; a citation of an id with no Decision file is a violation naming the home, the command and the id. A token is four digits behind a d or D with no identifier character on either side, so item names such as dcOneClick and identifiers such as d03561 are not citations. The check is probed (D0388) by a synopsis fixture citing D0353 as the known-positive and the corrected suite synopsis citing D0356 as the known-negative, both named in the test; the live tree must be green under it.";
        :>> rationale = "A synopsis is the help's one sentence about a command; a Decision id in it is a claim that the authority named is in force. The tree already knows when that claim is false - retirement is one edge and existence is one file - so leaving the check to a reader is choosing to be told by a person what a guard could say at every commit. Holding both homes rather than the authored one alone costs nothing extra and closes the shape the defect had: the mirror drift check compares the homes to each other, so a stale citation copied faithfully into both was invisible to it. Naming the superseder in the violation makes the fix one edit rather than a search.";
        :>> consequences = "keel-cli/src/guards.rs gains decision_ids_cited, decision_ids_present and synopsis_citation_violations, wired into cli_surface_declared; keel-cli/src/lib.rs gains supersede_edges, of which supersede_targets is now the projection; three unit tests carry the probe pair, the token rule and the live tree; .engine/docs/guards.md says the rule in the same words. Guard source is the enforcement surface (D0209 clause 2), so this Decision is the co-committed change record and, as a process change, stays proposed for the human (D0337). STATED LIMIT: a synopsis that cites a Decision by title rather than id is not read; the token rule is the ids.";
    }
}
