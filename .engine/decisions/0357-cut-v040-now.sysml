// D0357 (status: the `status` field below is the truth) — releaseV040: v0.4.0 is cut now, carrying the field fixes that have sat unshipped, and the command names are unchanged
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0357Accept event on sign-off.
package Decision0357 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0357 : Decision {
        :>> id = "eecc3125-165d-49c1-84d0-306a321e5987";
        :>> title = "releaseV040: v0.4.0 is cut now, carrying the field fixes that have sat unshipped, and the command names are unchanged";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "Seven days of fixes sat between this repository and anything anyone could install: an upgrade silently put back controls a project had switched off, a configuration naming a decision the project did not hold read as governed, three gating surfaces resolved different binaries, a cached binary under a pinned name ran unverified, and a migration refused a healthy project because a scanner read prose as schema. The owner's answer on the published brief, verbatim: 'Release now'. On the same brief they chose 'Leave them' for the command names, so this release renames nothing.";
        :>> decision = "The crate and the engine pin move to 0.4.0 and an annotated tag is pushed, after the full gate has run on exactly the tree that will be tagged; the Release is recorded as an authored fact carrying the version, the tagged commit and what it ships. NOT BREAKING: no command is renamed or removed, so a project on the previous version meets no rename at its next upgrade; the three deprecated channel verbs stay until a release that is breaking for other reasons. NOT A FORK: the owner instructed the release.";
        :>> rationale = "Why 0.4.0 rather than a patch: the surface gained commands (the unattended currency pass, the suite runner, unit retirement) which is a minor bump under semantic versioning, while nothing that existed was renamed or removed. Why now rather than with the next change: the defect that motivated it is silent, it lands on a project the owner is actively migrating, and every day it stays unshipped is a day that project can lose its own configuration without being told. Arming (D0298): the release gate is the deploy process's own step - validate, guard, check-engine, clippy and the full suite on the tagged tree - and it earned its keep here by catching a stale plugin rendering that the version bump itself created.";
        :>> consequences = "Downstream projects can install the fixes. The release also ships the withdrawal of the push gate and the reworked decision surface, so an adopter's next upgrade brings a shorter path to a push and a different report. A defect found after this tag is fixed in the next release rather than by moving the tag, because a tag that moves makes every recorded judgement against it unreadable.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0357Accept : Test { :>> id = "b789dc5d-cb89-4ae4-a8e8-01766b2bc7cf"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0357AcceptR1 : TestResult { :>> id = "21ac0a24-91dd-4bbc-b047-1cc0bc2dbadc"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8dc7c77"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
