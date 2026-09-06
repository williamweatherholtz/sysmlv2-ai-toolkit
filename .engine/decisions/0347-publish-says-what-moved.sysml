// D0347 (status: the `status` field below is the truth) — publishSaysWhatMoved: a manifest-only publish commits once with a subject that says so, at the unchanged version
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0347Accept event on sign-off.
package Decision0347 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0347 : Decision {
        :>> id = "103441d1-b424-4fd1-844c-3d8b6f286d06";
        :>> title = "publishSaysWhatMoved: a manifest-only publish commits once with a subject that says so, at the unchanged version";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue340. publish's no-op detection (issue302) compares the unit's FILE hashes against the install record and holds the version when they match - correct - but it did not look at the MANIFEST it writes, so a manifest-only change (every unit gained a commands line under D0252 clause A) produced a library commit titled publish X vN at an unchanged N, and the log read as if the same version had been published twice. The library log is a review surface; a subject that misstates what moved is noise in the one place a consumer reads what changed.";
        :>> decision = "publish reads what git says changed under the unit in the clone after the export: nothing changed is the stated no-op with no commit, as before; only unit.toml changed commits once with the subject publish X vN (manifest only) and prints MANIFEST ONLY with why the version did not move; any file changed commits publish X vN as before. NOT A FORK: the version rule is issue302's and is unchanged - the fix is to say which of the two things moved.";
        :>> rationale = "Why read git rather than hash the manifest ourselves: the clone already holds the last published manifest and git already diffs it; a second hash of the same fact is a second place to be wrong. Why not bump the version on a manifest change: consumers compare versions to learn whether the unit's CONTENT moved (issue323), and a manifest line is not content - moving the number would tell them to update for nothing. Arming (D0298): a test that publishes, replaces the clone's manifest with an older exporter's copy (one line fewer, same files), publishes again and asserts one commit with the qualified subject and an unmoved version line, then a second run as the stated no-op; the three existing publish scenarios unchanged.";
        :>> consequences = "The library log distinguishes a version from a manifest refresh. issue340 resolves on dcPublishSaysWhatMoved's pass. No process-surface change; the library-stewardship text already says a no-op is information.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0347Accept : Test { :>> id = "302dbed3-27eb-4fad-adb6-4524e46610cd"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0347AcceptR1 : TestResult { :>> id = "e2dac73d-2fb8-4a22-88c1-60ab197c3859"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "515cf80"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
