// ProjectDeliveryS577 — Sprint 577: a binary in the wrapper cache is trusted only as far as it is verified - a hit is checked like a download, and init seeds nothing another machine cannot obtain. EXECUTE. 2 pts.
package ProjectDeliveryS577 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from seededCacheVerifiedStory to d0342;

    part seededCacheVerifiedStory : Story {
        :>> id = "1dec3802-a864-4479-8dda-ebc6fc79738c";
        :>> title = "Sprint 577: a binary in the wrapper cache is trusted only as far as it is verified - a hit is checked like a download, and init seeds nothing another machine cannot obtain (2 pts)";
        :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 2;
    }

    action def DeliveryRunS577 {
        action storySeededCacheVerified;
        verification storySeededCacheVerifiedDoD : Test { :>> id = "2e553857-ee1c-4bc3-808e-92dd524d4597"; :>> method = VerificationMethod::test; :>> procedureText = "DELIVERED BACKLOG ITEMS: dcSeededCacheIsVerified (D0342; resolves issue379, GH#54). DONE: keelw reads the committed checksum before the cache, verifies a hit against it when an entry exists (mismatch refuses naming both hashes and the path), and prints UNVERIFIED on stderr when no entry exists; keel init seeds the cache only from a reproducible build (a clean SHA - not +dirty, not unknown) and prints what it did either way, never writing a checksum. VERIFIED: a seeded binary whose hash differs from the committed release entry is refused naming both hashes; a hit with no entry runs and says UNVERIFIED; the five existing wrapper scenarios still hold; is_reproducible_build unit-tested; a live keel init from this +dirty tree printed wrapper cache NOT seeded naming build 5e3a828+dirty and left .keel/bin absent."; }
        // RAN: dcSeededCacheIsVerified DoD pass recorded on suite577 543/0; the sprint's implement section names each run
        part storySeededCacheVerifiedDoDR1 : TestResult { :>> id = "a099d4eb-aa70-4853-ad47-6e069805a8ad"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5e3a8285c6ad296313624db9da86390071ae94ab"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "claudeOpus5"; }
    }

    verification seededCacheVerifiedRefineGate : Test { :>> id = "481c19a1-4bfb-45bb-b2c7-365bc60edb26"; :>> title = "Sprint 577 refine gate (DoR)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: both of the reporter's candidates were taken because each closes a different half - verification catches a wrong file already in the cache, the seeding rule stops putting one there. The one design question was refuse-or-say when no entry exists; saying keeps a fresh project working and makes the trust visible on every run."; }
    verification seededCacheVerifiedStandupGate : Test { :>> id = "5637375c-0b1e-4aef-918f-7140a2bdacd5"; :>> title = "Sprint 577 standup gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THE FACT THAT DISTINGUISHED THE TWO BINARIES WAS THE BUILD COMMIT, not the version - both said 0.3.1. build.rs already baked it (D0251); init now reads what it baked."; }
    verification seededCacheVerifiedImplementGate : Test { :>> id = "e292af21-5a49-4689-b03f-7545a9bb1837"; :>> title = "Sprint 577 implement gate"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: cargo test wrapper_resolves_only_the_pin 7/7 (two new scenarios); cargo test --bin keel init_ and reproducible 3/3; clippy -D warnings clean; keel init C:/kt/initseed from this dirty build printed NOT seeded; the full suite in suite577.txt."; }
    verification seededCacheVerifiedReviewGate : Test { :>> id = "8b8520e0-dcc3-48cd-8669-cdfa3f07120b"; :>> title = "Sprint 577 review gate — three passes per D0172"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "TWO PASSES. (1) DOES THE HIT PATH REACH THE ORIGIN: no - the two new scenarios run with a nonexistent origin and the hit is decided from the cache and the committed entry alone. (2) IS ANY HASH INVENTED: init writes only the version header it always wrote; keelw writes nothing."; }
    verification seededCacheVerifiedCloseOutGate : Test { :>> id = "ffc83d26-28dd-4bec-953d-773ecf0c346d"; :>> title = "Sprint 577 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "CLOSED. issue379 resolves on this pass. A project seeded by an earlier init is refused by keelw the first time it runs after committing the release checksum - the moment the skew becomes known."; }
    verification seededCacheVerifiedRetroGate : Test { :>> id = "80c457dd-d44e-4113-9d74-50f5b2fc94ca"; :>> title = "Sprint 577 retro gate (autonomous, D0049)"; :>> createdAt = "2026-09-06"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "no new item - already tracked: issue378 (dcOneBinaryResolutionForEverySurface) is the other half of the same downstream day - the hooks that never looked in this cache at all. The lesson: every path that fills a cache is a trust decision, and the check belongs where the file is USED, not only where it arrives."; }
}
