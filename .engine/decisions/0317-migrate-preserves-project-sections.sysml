// D0317 (status: the `status` field below is the truth) — migratePreservesProjectSections: an engine resync merges a project's own [unit] sections of unit-extras.toml and blocks by name on a collision
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0317Accept event on sign-off.
package Decision0317 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0317 : Decision {
        :>> id = "4e73fdfd-8795-477f-823b-70bbaf3bcfe9";
        :>> title = "migratePreservesProjectSections: an engine resync merges a project's own [unit] sections of unit-extras.toml and blocks by name on a collision";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#44 (st085, issue349): a downstream project published a unit of its own and declared its mechanism files in the engine-shipped unit-extras.toml; keel migrate resynced the file wholesale and the published unit's payload was gone with nothing reported. The resync already protects six project-owned contracts by never overwriting them (issue314), but unit-extras.toml is neither wholly the engine's nor wholly the project's: the engine ships its sections and a project adds its own to the same file.";
        :>> decision = "For a sectioned contract (unit-extras.toml today), step engine-resync MERGES instead of overwriting: the engine's text leads, every [section] the engine's copy does not carry is appended verbatim under a header naming this rule, the run's detail line says how many project-authored sections were PRESERVED, and a section both copies carry with a different body (comments and blank lines aside) is a Blocker naming the section - the migration refuses and writes nothing, per issue314's shape - with the advice to rename the project's unit so the two do not collide.";
        :>> rationale = "Why merge and not add to the never-overwrite list: never-overwrite would freeze the engine's own sections at whatever vintage the project scaffolded on, so an engine unit that gains a mechanism would never reach the project; merge lets the engine's half move and the project's half stay. Why a collision blocks rather than picks a side: the engine cannot tell whether the project's differing body is a customisation of the engine's unit or a name clash with the project's own, and choosing wrong either loses a project's payload or ships the engine's under a project's name - the refusal is recoverable and names exactly what to do. Why the fixture suite gains it: the realistic-fixture suite (D0275) exists because seven migration defects reached a project before a test saw them; this is the fifth property, and the merge and the collision each have a unit test (migrate::section_merge_tests).";
        :>> consequences = "keel-cli/tests/migration_fixture_is_realistic.rs gains a_projects_own_unit_extras_section_survives_the_resync_and_is_reported; the project-migration skill's reconcile step and CLAUDE.md §4 name the sections among what must survive. issue349 is resolved by dcMigratePreservesProjectSections. The engine's own unit-extras.toml carries no section today, so the collision path is proven by unit test only until it does.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0317Accept : Test { :>> id = "164f9d8a-e5b6-460c-9386-435ca10f7719"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0317AcceptR1 : TestResult { :>> id = "89ca1d5e-69f1-4016-b3af-0f8c81941e13"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "8886405"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
