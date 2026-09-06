// D0349 (status: the `status` field below is the truth) — surfaceStampNamesTheBuild: the .claude surface sidecar stamps version and build commit, and a drift report names the build that stamped the surface and the build running
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0349Accept event on sign-off.
package Decision0349 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0349 : Decision {
        :>> id = "38841a73-dd72-4265-af28-dc4e161e1564";
        :>> title = "surfaceStampNamesTheBuild: the .claude surface sidecar stamps version and build commit, and a drift report names the build that stamped the surface and the build running";
        :>> createdAt = "2026-09-06";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#51 (st078, us087, issue357, routed under D0292; dcSurfaceStampNamesTheBuild). Two 0.3.1 builds - cf985ec+dirty and 6eafa86+dirty - generated different .claude surfaces, the claude-surface-drift guard failed, and its report named no binary: the sidecar carried the version alone, and both binaries were 0.3.1. The reader could see that the surface differed and not which generator had written what was on disk.";
        :>> decision = "The sidecar .claude/.keel-surface stamps one line, version then build commit (build.rs's KEEL_BUILD_COMMIT, +dirty from a modified tree); a pre-existing version-only sidecar reads back with an unknown build; the version-skew obligation is unchanged and reads the first token; when the check finds any drift and the stamped build differs from the running one, the report adds a line naming both - surface stamped by build X; this binary is build Y; sync-claude's own report prints the stamp it wrote. NOT A FORK: the stamp gains the one fact the report lacked.";
        :>> rationale = "Why only on drift: two builds of one version that generate the same surface are not a problem, and a line on every check would be noise on the common case; the build matters exactly when the content differs, because then it is the explanation. Why not make a build difference itself drift: this repository rebuilds many times a day and the surface is regenerated on sync, so a build-only difference would keep the guard red between rebuild and sync over content that is identical. Arming (D0298): a binary test asserts the scaffold's sidecar has two tokens and the second is the build keel version prints, forges the stamp to another build and mutates a skill, and asserts the check report names both builds; regenerating restores this build's stamp and a clean check.";
        :>> consequences = "A drift report on a mixed-build machine says which binary wrote the surface. This repository's committed sidecar now carries the generating build (a +dirty commit when synced from a working tree, which is the truth about the generator). dcSurfaceStampNamesTheBuild is done on the test above.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0349Accept : Test { :>> id = "7b27a037-0eef-4a70-aa05-9f95e18ecdd2"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0349AcceptR1 : TestResult { :>> id = "88f43bdb-dba8-41e6-9944-0c1448c5280a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a66324c"; :>> judgedAt = "2026-09-06"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
