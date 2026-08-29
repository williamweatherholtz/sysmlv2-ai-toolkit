// ProjectDeliveryS484 — Sprint 484: the unit manifest becomes portable before anything publishes it. EXECUTE. 5 pts.
package ProjectDeliveryS484 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #CharteredBy dependency from libraryManifestIntegrityStory to d0250;

    part libraryManifestIntegrityStory : Story {
        :>> id = "a2b7bc09-3d2c-49a3-b8b3-63bb5a6d5f2c";
        :>> title = "Sprint 484: the unit manifest is portable and its version means something (5 pts)";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5"; :>> kind = WorkKind::code; :>> priority = WorkPriority::p0; :>> owner = "claudeOpus5"; :>> estimatedPoints = 5;
    }

    action def DeliveryRunS484 {
        action storyLibraryManifestIntegrity;
        verification storyLibraryManifestIntegrityDoD : Test { :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f01"; :>> method = VerificationMethod::test; :>> procedureText = "RESOLVES issue301 (machine-absolute keys in installed-units.toml) and issue302 (version bumps with no content change). DO: (1) every recorded file key is repository-relative; a unit-extras path outside the project root is REFUSED at export rather than absolutised, naming the path and why. (2) The version advances only when at least one recorded hash changes; a re-stamp finding identical hashes leaves the file byte-identical. (3) A check rejects a manifest containing an absolute key, so this cannot silently return. VERIFY by the Gherkin below, plus: keel validate, check-engine, all guards, clippy -D warnings, cargo test --workspace --release."; }
        // RAN: // RAN: cargo clippy --workspace --all-targets -- -D warnings (clean); cargo test --workspace --release (299 lib + 51 integration, 0 failed); keel validate . (544 clean); keel check-engine .; keel guard (53 guards, 0 FAIL); arming probe: injected absolute key -> manifest-key-portability FAIL 84/1, removed -> PASS 83/0
        part storyLibraryManifestIntegrityDoDR1 : TestResult { :>> id = "eecaa0a8-0a5e-48ab-a232-b2a6a43fb065"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "7c5b416"; :>> judgedAt = "2026-08-28"; :>> judgedBy = "claudeOpus5"; }
    }

    // ── Gherkin (backlog-refinement, D0172): each scenario is a Test the DoD is discharged by ────
    // Written BEFORE the implementation, and each names a failure that can actually occur rather
    // than restating the requirement in Given/When/Then costume.

    verification s484ScenarioRelativeKeys : Test {
        :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f02";
        :>> title = "Scenario: a unit whose extras live outside the project root exports with relative keys";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::test;
        :>> procedureText = "GIVEN a unit whose declared extras include a file at the repository root, outside the .engine tree WHEN the unit is exported THEN every file key in the manifest is repository-relative AND no key contains a drive letter, a home directory, or the exporting machine's user name. This is the scenario issue301 was found by: four keys under the decision-channel unit name this machine's home directory.";
    }

    verification s484ScenarioForeignImport : Test {
        :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f03";
        :>> title = "Scenario: a unit exported on one machine reconciles by hash on another";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::test;
        :>> procedureText = "GIVEN a unit exported from a project at one filesystem location WHEN it is imported into a project at a DIFFERENT location THEN every recorded hash resolves to a real file AND the three-way merge base is found without rewriting any path. The second location is the whole test - an import into the same path would pass while absolute keys were still broken, which is why this scenario moves the tree rather than reusing it.";
    }

    verification s484ScenarioAbsoluteKeyRefused : Test {
        :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f04";
        :>> title = "Scenario: a manifest carrying an absolute key is refused, not tolerated";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::test;
        :>> procedureText = "GIVEN a manifest hand-edited to contain one absolute file key WHEN the check runs THEN it fails and names the offending key. Without this the fix is a one-time cleanup that regresses the next time an extras path is added - D0047: a defect that can recur becomes a control, never a corrected file.";
    }

    verification s484ScenarioNoOpRestampIsByteIdentical : Test {
        :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f05";
        :>> title = "Scenario: re-stamping with no edits leaves the manifest byte-identical";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::test;
        :>> procedureText = "GIVEN a project whose units are all current WHEN the stamping path runs twice with no file edited between runs THEN installed-units.toml is byte-identical after both AND no unit version advanced. Byte-identical rather than semantically-equal, because the churn itself is half the defect: a manifest rewritten every commit is useless as a review surface for what actually moved.";
    }

    verification s484ScenarioEditAdvancesExactlyOne : Test {
        :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f06";
        :>> title = "Scenario: editing one unit's file advances that unit's version and no other";
        :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::test;
        :>> procedureText = "GIVEN a project with several installed units WHEN exactly one file of one unit is edited and the manifest is re-stamped THEN that unit's version advances by one AND every other unit's version and hashes are unchanged. This is the property that makes the version answerable to should I update - the question a project asks once the library is a repository other machines pull (D0250).";
    }

    #Verify dependency from s484ScenarioRelativeKeys to srUnitDeclaresEngineRange;
    #Verify dependency from s484ScenarioForeignImport to srUnitDeclaresEngineRange;
    #Verify dependency from s484ScenarioNoOpRestampIsByteIdentical to srActivatedUnitVersionIsPinned;
    #Verify dependency from s484ScenarioEditAdvancesExactlyOne to srActivatedUnitVersionIsPinned;

    verification libraryManifestIntegrityRefineGate : Test { :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f07"; :>> title = "Sprint 484 refine gate (DoR)"; :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "READY: chartered by D0250, which settles the fork this work depends on - a manifest's portability only becomes testable once it is known the library has a remote other machines pull. Both defects are recorded Issues (issue301 High, issue302 Medium) with this story as their resolver, so resolution is computed rather than asserted. Five Gherkin scenarios exist and were written before implementation; two of them (foreign import, absolute-key refusal) fail against today's code, which is what makes them worth running."; }
    verification libraryManifestIntegrityStandupGate : Test { :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f08"; :>> title = "Sprint 484 standup gate"; :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "No blocker. The dependency was D0250 (fork 1: the library is a git repository), recorded this session on the human's own words, which is what turned two latent manifest defects into urgent ones. The scaffold-placeholder guard also shaped the sequencing: it REFUSED a sprint record carrying unfilled future gates, so this sprint was executed rather than planned-and-shelved - the record and the work land together or not at all."; }
    verification libraryManifestIntegrityImplementGate : Test { :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f09"; :>> title = "Sprint 484 implement gate"; :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::test; :>> procedureText = "RAN: cargo build --release; cargo clippy --workspace --all-targets -D warnings; cargo test --workspace --release (299 lib + 51 across the BDD/integration binaries, 0 failed); keel guard (53 guards). ROOT CAUSE, and it was one line: unit_files puts a unit's declared EXTRAS at root/<extra>, while the manifest key builder stripped only the .engine prefix and fell through via unwrap_or(f) to the ABSOLUTE path - so every extra outside .engine wrote this machine's home directory into a committed contract. FIXED: manifest_key strips .engine then root and RETURNS Err rather than absolutising; next_unit_version advances only when a recorded hash actually changed; the four existing absolute keys were rewritten repository-relative. CLIPPY CAUGHT THE FIX: cmd_export went to 108/100 lines, which was extracted into manifest_hashes and next_unit_version rather than suppressed - and the extraction made hashes_match unit-testable without a filesystem, which is the part that was actually wrong."; }
    verification libraryManifestIntegrityReviewGate : Test { :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f0a"; :>> title = "Sprint 484 review gate — three passes per D0172"; :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "THREE PASSES. (1) CORRECTNESS: the guard detects by SHAPE - drive letter, leading slash, leading tilde - never by this machine's user name, because a guard looking for the current user would pass on every machine except the one that already got it right. (2) ARMING, the D0217 lesson applied to my own control: the guard was PROVEN to fire, not assumed. An absolute key was injected and it went FAIL 84 scanned / 1 violation; the key was removed and it returned PASS 83 / 0. A control that has never been observed failing is declared, not armed. (3) COMPLETENESS: the export path is fixed and the two import paths still call write_install_record with hashes built by their own local logic - stated as a KNOWN RESIDUAL rather than claimed clean, because the guard now catches any absolute key those paths produce at the next gate, which bounds the exposure without my asserting a fix I did not make."; }
    verification libraryManifestIntegrityCloseOutGate : Test { :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f0b"; :>> title = "Sprint 484 closeOut gate (autonomous, D0049)"; :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::inspect; :>> procedureText = "Both issues resolved by this story and computed as such (#Resolves edges authored on arrival). The control is new guard 53, manifest-key-portability, hard-blocking. THE RESIDUAL IS NAMED: the two import paths build their hashes independently of manifest_key and were not refactored - the guard covers them at the gate, but the refusal-at-source that the export path now has does not exist there. Recorded here rather than left for a reader to discover."; }
    verification libraryManifestIntegrityRetroGate : Test { :>> id = "1f6ad2a0-2f7c-4a9e-9c22-2c4c1d2f8f0c"; :>> title = "Sprint 484 retro gate (autonomous, D0049)"; :>> createdAt = "2026-08-28"; :>> createdBy = "claudeOpus5"; :>> method = VerificationMethod::analyze; :>> procedureText = "FINDING: this defect was INVISIBLE to every existing control while sitting in a committed contract file for months, and what finally surfaced it was not a guard but a human's architecture decision - D0250 made the library a repository other machines clone, which turned an ugliness into a break. The general shape is that a latent defect's severity is a function of a decision not yet made, so a control inventory computed against today's architecture will always understate. BECOMES TRACKED WORK: nothing new is filed, because the existing latent-need derivation process is the right home for this class - it re-reads the model when direction moves - and inventing a second mechanism for the same observation would be the duplication D0105 forbids. SECOND FINDING: the scaffold-placeholder guard turned a planning habit into an execution one by refusing a sprint record with unfilled gates. That is a control changing behaviour rather than reporting on it, and it is worth noticing because it is the outcome nAiGovernanceInLoop asks for."; }
}
