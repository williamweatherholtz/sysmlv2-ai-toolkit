// ProjectDeliveryS590 — Sprint 590: the facts the standing brief is built from stop naming a release and start deriving it. EXECUTE. 1 pts.
package ProjectDeliveryS590 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from factsNameNoVersionStory to d0359;

    part factsNameNoVersionStory : Story {
        :>> id = "41190a2e-bf5a-4d76-9d5b-36a75cb9435e";
        :>> title = "Sprint 590: the facts the standing brief is built from stop naming a release and start deriving it (1 pts)";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 1;
    }

    action def DeliveryRunS590 {
        action storyFactsNameNoVersion;
        verification storyFactsNameNoVersionDoD : Test { :>> id = "7b8f5c93-03dc-41f0-9ea2-d784613e4c21"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEM: dcFactsNameNoVersion, resolving issue389. DONE: facts.py derives the release it measures against with git describe rather than naming one in seven places, emits that tag as its own fact, carries it in the unit of every dependent fact, and REFUSES to run while its own source holds a version literal. VERIFIED: the script runs clean reporting releaseTag v0.4.1 with the commit, day and CI counts measured from it and no null values; an injected literal makes it exit naming the literal and the reason; removing it makes the run clean again; the check scans the whole file, which its first version did not."; }
        // RAN: python scripts/exec_brief/facts.py clean with releaseTag v0.4.1 derived; injected literal -> exits naming it; removed -> clean
        part storyFactsNameNoVersionDoDR1 : TestResult { :>> id = "b742e511-b5b1-4149-b4fb-e1655a5591cf"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "HEAD"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    }

    verification factsNameNoVersionRefineGate : Test { :>> id = "6b970708-380b-4713-8da5-fbdcf4c44833"; :>> title = "Sprint 590 refine gate (DoR)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: found while assembling the standing brief - the facts table still named the release before last, one hour after a new one shipped. The scope was set by the defect: derive the identifier, and make the rule that forbids typing it enforceable inside the script that runs on every publication."; }
    // RAN: facts.py clean-fail-clean probe
    part factsNameNoVersionRefineGateR1 : TestResult { :>> id = "9e2c0bbf-f9c3-48d1-adf2-95f5c63fc62d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "HEAD"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    verification factsNameNoVersionStandupGate : Test { :>> id = "ae37d578-beae-4fb9-836c-8e1fbc701320"; :>> title = "Sprint 590 standup gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "I ALMOST PUBLISHED IT. The numbers were computed correctly against the wrong release, and the provenance string named a real git command, so nothing on the page would have looked wrong. What caught it was reading the facts table rather than trusting it, which is not a control and is why the script now refuses."; }
    // RAN: facts.py clean-fail-clean probe
    part factsNameNoVersionStandupGateR1 : TestResult { :>> id = "12939edf-8346-4880-b3e4-873d0c1fc017"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "HEAD"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    verification factsNameNoVersionImplementGate : Test { :>> id = "f82b751f-5bd2-40d8-b405-cafb0ae1c5f0"; :>> title = "Sprint 590 implement gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: git describe wired in and the seven literal references removed; the script re-run and the tag read back as v0.4.1 with commitsSinceRelease 4, daysSinceRelease 0, ciRunsSinceRelease 5; the self-check installed, PROBED with an injected literal, found to be false, repaired to scan the whole file, and probed again clean-fail-clean."; }
    // RAN: facts.py clean-fail-clean probe
    part factsNameNoVersionImplementGateR1 : TestResult { :>> id = "459f69ca-c254-4b9b-83b5-abe151d91b66"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "HEAD"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    verification factsNameNoVersionReviewGate : Test { :>> id = "844aeb28-8785-4bde-a371-484d63bdc4b7"; :>> title = "Sprint 590 review gate — three passes per D0172"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) IS THE FIX COMPLETE: zero version literals remain in the file, checked by the control itself rather than by eye. (2) IS THE CONTROL REAL: no, not at first - it scanned only the source above itself, and the release facts sit below, so it would have passed over the defect it was written for. That is recorded in the issue rather than quietly repaired. (3) IS READING SOURCE TEXT DEFENSIBLE HERE: yes, and only here - the property IS about source text, which is exactly the distinction issue388 turns on."; }
    // RAN: facts.py clean-fail-clean probe
    part factsNameNoVersionReviewGateR1 : TestResult { :>> id = "e478adf8-c98f-4b65-a7fc-2fe50ec9dbb0"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "HEAD"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    verification factsNameNoVersionCloseOutGate : Test { :>> id = "e84daa85-a5e2-4543-85be-56a18f16a2cf"; :>> title = "Sprint 590 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. The brief's numbers name the release they are about, and the script cannot be edited back into naming one."; }
    // RAN: facts.py clean-fail-clean probe
    part factsNameNoVersionCloseOutGateR1 : TestResult { :>> id = "3903ebe8-e01c-43b5-a3f8-d8c093e0f39e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "HEAD"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    verification factsNameNoVersionRetroGate : Test { :>> id = "4ae2e5f7-6874-4420-a403-c3a30454fe8d"; :>> title = "Sprint 590 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "AVOIDABLE-ISSUE SCAN HELD. Already tracked, no new item: this is issue389 itself, and the control-that-did-not-control is recorded in its description. The lesson worth carrying is that a control must be probed against the defect it was written for, not merely run once and seen to pass - a check that passes on a clean tree proves nothing about what it catches, and mine passed while blind to the entire section it existed to police."; }
    // RAN: facts.py clean-fail-clean probe
    part factsNameNoVersionRetroGateR1 : TestResult { :>> id = "7eb5731a-c13b-47be-aa48-873d3932f2c2"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "HEAD"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
}
