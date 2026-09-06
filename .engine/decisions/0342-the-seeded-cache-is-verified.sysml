// D0342 (status: the `status` field below is the truth) — seededCacheIsVerified: keelw verifies a cache hit against the committed checksum whenever one exists, and init seeds the cache only from a reproducible build, saying what it did
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0342Accept event on sign-off.
package Decision0342 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0342 : Decision {
        :>> id = "d67403c6-5ce0-4f91-be20-a425f6bcda19";
        :>> title = "seededCacheIsVerified: keelw verifies a cache hit against the committed checksum whenever one exists, and init seeds the cache only from a reproducible build, saying what it did";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#54 (st097, us096, issue379). keel init copied whatever binary ran it into .keel/bin/<version>/<asset>, so a +dirty local build stood as the pinned 0.3.1 for a project's whole life; keelw execed a cache hit without comparing it to keel-wrapper.toml, so the release's correct checksum sat committed in the tree while nothing ever checked the cached file against it. Measured downstream: two binaries both answering 0.3.1, 900 KB apart, spelling the lenses differently, one passing and one failing 14 guard violations on the same tree - and the project's CI, which downloads by tag, was the only honest signal. The wrapper's own header says trust-on-first-use is no verification; a seeded cache was trust-on-first-use by another route.";
        :>> decision = "keelw reads the committed checksum before consulting the cache and, on a hit, compares the cached file's SHA-256 to the committed entry for that version and asset whenever one exists - a mismatch refuses naming both hashes and the path, exactly as a download does; with no entry the hit runs and prints on stderr that it is UNVERIFIED and which entry would verify it. keel init seeds the cache only when the running binary's baked build commit is reproducible - a clean SHA, not +dirty and not unknown - and prints either what it seeded, from which build, and that it stays unverified until the release checksum is committed, or that it did not seed and why; it never writes a checksum it made up. NOT A FORK: both of the reporter's candidates are taken, because each closes a different half - verification catches a wrong file already there, the seeding rule stops putting one there.";
        :>> rationale = "Why verify on hit and not only on download: the cache is the thing that runs; the download is one of two ways it gets filled, and the other way had no check at all. Why say UNVERIFIED rather than refuse when no entry exists: a project that has not yet committed the release checksum was running on trust before and still is - the change is that it now says so on every run, which is what makes the skew findable; refusing would break every fresh project until its human pastes a hash. Why the build commit and not a version compare: both binaries answered the same version, which is the whole defect - the baked commit is the fact that distinguished them. Arming (D0298): two keelw tests (a seeded binary whose hash differs from the committed entry refuses naming both; a hit with no entry runs and says UNVERIFIED), a unit test on the reproducibility predicate, and a live init from this +dirty tree printing that it did not seed.";
        :>> consequences = "A project initialised by a dev build has an empty cache and keelw fetches the release on first use; a project whose cache was seeded by an earlier init is refused by keelw the first time it runs after committing the release checksum, which is the moment the skew becomes known rather than lived with. issue379 resolves on dcSeededCacheIsVerified's pass. The hooks' own resolution of the binary is issue378 and is not changed here.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0342Accept : Test { :>> id = "39eb406f-9950-41ce-9d7f-64ea5831c251"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0342AcceptR1 : TestResult { :>> id = "24ed3702-145f-4f1c-8243-4cd495d8d2ba"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5e3a828"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
