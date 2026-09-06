// ProjectDeliveryS583 — Sprint 583: a Decision that reverses another is recorded with its edge - keel record decision takes --supersedes and --derived-from and refuses a target that does not exist. EXECUTE. 2 pts.
package ProjectDeliveryS583 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from supersedesEdgeStory to d0352;

    part supersedesEdgeStory : Story {
        :>> id = "66a3e49b-92aa-431a-a57c-45ca23e656b9";
        :>> title = "Sprint 583: a Decision that reverses another is recorded with its edge - keel record decision takes --supersedes and --derived-from and refuses a target that does not exist (2 pts)";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS583 {
        action storySupersedesEdge;
        verification storySupersedesEdgeDoD : Test { :>> id = "136c1c3b-1007-4cea-8758-f35b30c1a8ea"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcRecordDecisionDeclaresSupersedes (D0352). DONE: write::DecisionLinks carries supersedes and derived_from; record_decision_with_links checks every target before writing - a #Supersede target must be a Decision on disk, a #DerivedFrom target a declared Statement or UserStory - and writes the edge lines inside the Decision's package in the same atomic write; keel record decision takes --supersedes dNNNN[,..] and --derived-from stNNN|usNNN and reads supersedes: / derived-from: header lines from a --from draft, printing each edge it authored. VERIFIED: record_decision_declares_supersedes 3/3 - a reversal carries the edge in its own file, validates, and keel show why reads it back from the superseded side; a missing Decision target refuses naming --supersedes and the id and writes nothing; a --from draft's two header lines author both edges and --derived-from to a Decision is refused."; }
        // RAN: DoD passes recorded on keel suite 562/0 (receipt af357bb03f1a); the implement section names each run
        part storySupersedesEdgeDoDR1 : TestResult { :>> id = "3fa9dc9f-7da9-41f5-81d6-e0638c9b5754"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a66324cffb59b843c32591c1a2e5f19ee1639036"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    }

    verification supersedesEdgeRefineGate : Test { :>> id = "0023985a-a8d7-4084-b8e4-050de14c2172"; :>> title = "Sprint 583 refine gate (DoR)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: the write path already composed the Decision file; the edges are two more lines of the same package, and the existence checks are two reads before the write. The one structural point was the lock-discipline test: an existence check inside an impl block was attributed to the enclosing pub fn, so it moved to a private free function."; }
    verification supersedesEdgeStandupGate : Test { :>> id = "b4ad38c5-266d-4941-9401-ba548f85cf38"; :>> title = "Sprint 583 standup gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THE EDGE BELONGS WITH THE FACT IT QUALIFIES. D0289 reversed D0178 and no edge said so; a guard could not see prose, and the human saw it instead."; }
    verification supersedesEdgeImplementGate : Test { :>> id = "ca2c14e1-5092-475e-ac06-02a6e771e6f6"; :>> title = "Sprint 583 implement gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: cargo test --test record_decision_declares_supersedes 3/3 (first run red on keel why versus keel show why - the lens spelling, fixed in the test); --lib no_public_write_reads_outside_the_lock green after moving the read into decision_on_disk; clippy -D warnings clean; keel suite (D0353) for the whole tree."; }
    verification supersedesEdgeReviewGate : Test { :>> id = "12733d6b-6fb7-4de9-95e3-2170e7af6652"; :>> title = "Sprint 583 review gate — three passes per D0172"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "TWO PASSES. (1) CAN AN EDGE TO NOTHING LAND: no - check runs before anything is written and names the target. (2) IS THE ACCEPTANCE PATH TOUCHED: no - record_decision delegates with empty links and the record-time consent arm is unchanged; D0352 itself auto-accepted at record time."; }
    verification supersedesEdgeCloseOutGate : Test { :>> id = "5d68dfce-5ea1-4be0-8b4a-af14145159a5"; :>> title = "Sprint 583 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. dcRecordDecisionDeclaresSupersedes done; existing edgeless reversals are history (D0129)."; }
    verification supersedesEdgeRetroGate : Test { :>> id = "5c6177a0-f5f0-4cf9-b1bf-44fd6585b1ed"; :>> title = "Sprint 583 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: dcDeliberateRankIsRecorded is the same shape - a prose claim (a deliberate inversion) with no typed edge to carry it - and is on the frontier. The lesson: when the write path creates a fact, it is the one place that can guarantee the fact's edges."; }
}
