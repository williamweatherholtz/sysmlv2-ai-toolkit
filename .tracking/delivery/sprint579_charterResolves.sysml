// ProjectDeliveryS579 — Sprint 579: a charter names a Decision the project holds, or the guard and onboard say so - a provenance claim the tree cannot back is never printed as fact. EXECUTE. 2 pts.
package ProjectDeliveryS579 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from charterResolvesStory to d0344;

    part charterResolvesStory : Story {
        :>> id = "0a22ec40-f7b8-48da-868b-33fdae2253e7";
        :>> title = "Sprint 579: a charter names a Decision the project holds, or the guard and onboard say so - a provenance claim the tree cannot back is never printed as fact (2 pts)";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS579 {
        action storyCharterResolves;
        verification storyCharterResolvesDoD : Test { :>> id = "7726e562-c7d0-4643-9ffc-ce363b8b6827"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcCharterMustResolve (D0344 proposed, safety-change - waits for the human under D0337; resolves issue380, GH#56). DONE: onboard::charter_resolves checks charteredBy against this project's .engine/decisions/NNNN-*.sysml; the activation-manifest guard fails a dangling charter naming the Decision and the path it looked for; keel onboard prints UNRESOLVED CHARTER with the remedy and its JSON carries charterResolves with chartered false. VERIFIED: a scaffold chartered by d0226 fails the guard and onboard prints UNRESOLVED CHARTER d0226; the same scaffold chartered by its own D0001 passes and reads CHARTERED; this repository's charter d0226 (held here) stays green. STATED, not fixed: fixes on main since v0.3.1 - issue314's overwrite among them - reach adopters only through a release."; }
        // RAN: dcCharterMustResolve DoD pass on suite579 546/0; the implement section names each run
        part storyCharterResolvesDoDR1 : TestResult { :>> id = "06cad0ed-dd37-437e-a6d4-3b78ddf35565"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0d70e27634589958a3de92351c61df149e0d6bb4"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    }

    verification charterResolvesRefineGate : Test { :>> id = "33818db2-b23f-485f-a453-2a428ab6b5a3"; :>> title = "Sprint 579 refine gate (DoR)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: the overwrite half was already fixed (issue314); the residual was the read side - a charter is read by name and never resolved. One predicate, used by the guard and by onboard, so the two cannot disagree."; }
    verification charterResolvesStandupGate : Test { :>> id = "397c5ea0-8391-4440-a4a0-0a40a9507efe"; :>> title = "Sprint 579 standup gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THE COMMAND WHOSE JOB IS PROVENANCE PRINTED A CLAIM IT COULD NOT READ. The check is a file's existence; the failure it catches is a recorded human decision replaced by nobody's."; }
    verification charterResolvesImplementGate : Test { :>> id = "f7602c7a-8824-4b0c-af04-9c8ba7e75888"; :>> title = "Sprint 579 implement gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: cargo test --test charter_must_resolve 2/2; keel guard activation-manifest . PASS 32 scanned on this repository; clippy -D warnings clean after is_ok_and; guards.md row updated; full suite in suite579.txt."; }
    verification charterResolvesReviewGate : Test { :>> id = "141d292c-b477-4159-811a-ed23d921e29c"; :>> title = "Sprint 579 review gate — three passes per D0172"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "TWO PASSES. (1) IS THE CHECK EXACT: a dNNNN with four digits and a matching NNNN-*.sysml under .engine/decisions - nothing heuristic. (2) DOES A HEALTHY PROJECT STAY GREEN: the second test and this repository's own guard run."; }
    verification charterResolvesCloseOutGate : Test { :>> id = "360b5914-9ef5-4faf-b83b-477e671d8b2a"; :>> title = "Sprint 579 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. issue380 resolves on this pass; D0344 is on the human's queue with D0340 and D0343; the release gap is on the same page as a fact for the human."; }
    verification charterResolvesRetroGate : Test { :>> id = "d129136f-c532-4385-886b-21425e88ffbd"; :>> title = "Sprint 579 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: issue381 (dcLibraryUnitRetirement) is the last open resolver from GH#54-59. The lesson: a reference field read by name is a claim; every surface that prints it owes a resolution first."; }
}
