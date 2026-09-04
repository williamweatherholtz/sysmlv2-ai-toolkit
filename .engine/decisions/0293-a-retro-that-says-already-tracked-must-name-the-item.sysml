// D0293 (status: the `status` field below is the truth) — A retro that says a finding is already tracked must name the item that tracks it; retro-backlog is registered as under-reporting until it checks
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0293Accept event on sign-off.
package Decision0293 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0293 : Decision {
        :>> id = "7097b9e8-eb25-4704-8417-6cf9ab3ed6a9";
        :>> title = "A retro that says a finding is already tracked must name the item that tracks it; retro-backlog is registered as under-reporting until it checks";
        :>> createdAt = "2026-09-03";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human asked whether findings were being recorded as they arose. Audit of this week: nineteen Issues recorded, and four findings that went into commit messages or retro prose instead - a sprint landed without the suite (CI red twenty minutes), six guards with no catalogue row, a scaffold shipping this repository's consent grants, and record decision unable to declare a supersession. Two of the four sat behind the words 'already tracked' or 'belongs to X' in a retro, and the retro-backlog guard accepted the words (guards.rs RETRO_NO_ITEM_JUSTIFICATIONS, a substring match consulting no item). D0131 says a retro's findings become tracked items or the retro records why not; the guard enforced the second half in a form that let 'why not' be unverifiable.";
        :>> decision = "The four findings are Issues now (issue360-issue363) with resolver tasks. The guard's gap is a control defect (issue364, D0278, direction under): registered in control-defects.toml so every retro-backlog verdict announces it until fixed, and the fix - a retro that claims 'already tracked' must name an item that resolves, and the other no-item justifications require the retro to also name the sprint's delivered item - is dcRetroAlreadyTrackedMustName, ranked ahead of the findings it would have caught. This Decision authorises the registry edit (a locked contract).";
        :>> rationale = "WHY THE GUARD AND NOT MY DISCIPLINE: the same discipline wrote 'already tracked' twice in two days about things that were not, in retros that passed; a justification nobody can verify is a justification that will be reached for when the item is inconvenient to write. Naming the item makes the claim checkable by the guard and by the reader. WHY RECORD THE LATE ISSUES AT ALL: three of the four are already fixed or chartered; recording them is the record half of D0047 and it is also what lets the frontier rank them - the suite-receipt gap is a Medium with a real control behind it, and it was invisible.";
        :>> consequences = "Every retro-backlog verdict carries the defect note until dcRetroAlreadyTrackedMustName lands; the frontier gains five items. REMOVAL PATH (D0163): the registry entry leaves with the fix, in the same commit, as the control-defect process requires.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0293Accept : Test { :>> id = "bca9abc8-6508-4bd9-8824-09275a28918f"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0293AcceptR1 : TestResult { :>> id = "1631bcd8-7676-467f-a22a-af7ac466735c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "7deb411"; :>> judgedAt = "2026-09-03"; :>> judgedBy = "wweatherholtz"; }
}
