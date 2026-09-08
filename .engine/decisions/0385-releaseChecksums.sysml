// D0385 (status: the `status` field below is the truth) — releaseChecksums: every release asset is published with its SHA-256 by the run that built it, a normal part of the release process
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0385Accept event on sign-off.
package Decision0385 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: gh release download v0.4.1 on 2026-09-08 returned three binaries and no checksum file; .github/workflows/release.yml has no hashing step and its publish step lists only the binary; keel-wrapper.toml carries entries for 0.3.1 only while .engine/contracts/engine-version.toml pins 0.4.1 (moved 2026-09-06, b171cd7), and keelw line 69 refuses a download with no entry - so a fresh clone on the current pin has been refused for two days. The existing entries were made by downloading each asset and hashing it locally, a hand step no process text names.
    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0385 : Decision {
        :>> id = "b90d90f5-70ab-4f4a-b0d1-e19ed67440e4";
        :>> title = "releaseChecksums: every release asset is published with its SHA-256 by the run that built it, a normal part of the release process";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The wrapper contract promises verification against the release page's published checksums (keel-cli/src/main.rs:3733, keel-wrapper.toml header) and the release page has never published one. The release skill (D0135) names four steps - reconcile, gate, tag and run a downloaded asset, record - and none of them produces or consumes a checksum, so the only integrity evidence a downstream project has is a hash it computed itself from the same download it is meant to verify. The human, 2026-09-08: 'looks like keel release doesn't have a checksum? it needs one' and 'make a normal part of process'.";
        :>> decision = "NOT A FORK: the human named the change and the shape follows from how the assets are built. Every asset a release publishes is published with its SHA-256, computed in the same workflow run and job that built the asset - a sha256sum-format `<asset>.sha256` beside each binary and one `SHA256SUMS` over all of them, attached only after the run has downloaded the published binaries and verified them against it.";
        :>> rationale = "A checksum computed by the consumer from the download it verifies proves nothing; one computed by the build and published beside the asset lets keelw, the release skill and any downstream project check that the bytes they hold are the bytes that were built. Doing it in the build job costs one step per platform and no new tooling; the aggregate job's verify step makes the release run itself the first consumer, so a stale or partial upload fails the run instead of the next downstream clone. The release is the one artifact this project cannot re-verify after the fact (D0135), so its integrity evidence has to travel with it.";
        :>> consequences = "release.yml gains the per-job hashing step and the checksums job; guard release-checksums-published joins the enforced set and fails any workflow that attaches release assets without hashing and publishing them; guard wrapper-pin-checksummed (warning) reports a pinned engine version with no keel-wrapper.toml entry for an asset the wrapper can download - warning, because a project fresh from keel init legitimately has none until it copies them from the release. The release skill's step 3 verifies the downloaded asset against SHA256SUMS and copies the entries into keel-wrapper.toml before step 4 records the Release; keel-wrapper.toml gains the 0.4.1 entries computed from the published assets on 2026-09-08. The next tag exercises the workflow change; until then the guard test is the evidence.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0385Accept : Test { :>> id = "db4f60a6-6a13-41eb-85e5-4ee59b72bca2"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Direction given in chat 2026-09-08, two messages while the brief was being built; the second, also verbatim: “make a normal part of process”. The Decision records the shape that follows - the checksum published by the run that built the asset, the release skill and two guards making it a step of the process, not a hand action. - their words, verbatim: “looks like keel release doesn't have a checksum? it needs one”"; }
    part d0385AcceptR1 : TestResult { :>> id = "95ad6bb9-7a55-49ac-aea9-1871c435e99f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "b54cf82"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
