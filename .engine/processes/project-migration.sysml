// projectMigration — how an EXISTING keel project takes an engine that moved underneath it.
//
// WHY THIS IS NOT THE MIGRATION PROCESS WE ALREADY HAD. D0067's `migration` process governs bulk
// changes to a project's OWN authored data - rename/split/drop a field across many sites, inside one
// tree, by a transform its author writes. This process governs the other direction entirely: the
// ENGINE changed, the project did not ask for it, and the project must arrive somewhere honest. The
// two share the word migration and nothing else - different actor, different artefact, different
// failure modes. Conflating them is why this one was never written down.
//
// WHY IT IS BEING WRITTEN NOW, and the evidence is not an impression. Seven defects live in this
// path - issue301, issue310, issue314, issue323, issue324, issue326, issue327 - and NOT ONE was
// found by a test. Three were found by a peer session running keel in a downstream project within a
// single day; the rest by someone hitting them while doing something else. Four still have no test
// at all. The cause is structural rather than negligent: `check_preconditions` refuses any tree
// holding `keel-cli/Cargo.toml` as a self-build (migrate.rs:664-666), so THE ENGINE CANNOT MIGRATE
// ITSELF. Every other keel discipline is dogfooded daily; this one is the single surface the
// self-build cannot exercise, and it has exactly the defect density that predicts.
//
// THE ORDERING PRINCIPLE, which is what makes this a process rather than a checklist: each step
// exists because a project can arrive at it in a state the NEXT step cannot recover from. Migrate
// refuses a dirty tree; a dirty tree cannot always be committed; so cleanliness is established
// before migration is attempted, not during it. That is the whole shape.
package ProcessProjectMigration {
    private import EngineElement::*;
    private import EngineProcess::*;

    action projectMigration : Process {
        :>> id = "d0275a01-0901-4c1e-9f10-1a2b3c4d5e6f";
        :>> title = "Project migration across an engine change";
        :>> purpose = "Bring an existing project from the engine it pinned to the engine it is being given, such that at every point the tree is either fully migrated and green or byte-identical to where it started - and such that the project's OWN choices (adoption, customisation, authored content) survive an upgrade it did not author.";
        :>> cadence = Cadence::eventDriven;
        // APPLIES-WHEN: a project's engine-version pin is behind the binary it is being asked to run
        // - an upgrade, a library unit requiring a newer capability, or a breaking rename like D0273.
        // NOT for changes to the project's own data: that is D0067's `migration`.
    }

    action pm0Preflight : ProcessStep {
        :>> id = "d0275a01-0901-4c1e-9f10-1a2b3c4d5e70";
        :>> title = "Establish a clean, green, RECORDED starting point - and prove you can still commit";
        :>> triggerCondition = "The project is about to be moved to a newer engine.";
        :>> actionText = "Three things, in this order, because the second can be impossible. (1) Gate the tree on the CURRENT engine and record the resulting SHA - it is the restore point, and a migration whose starting point was already red cannot tell you afterwards which failures it caused. (2) Make the tree clean. This is the step that can DEADLOCK and must be checked rather than assumed: migrate refuses a dirty tree, and under engine-version skew the pre-commit gate refuses too, so a project that is behind its pin AND holds an uncommitted file cannot clean itself (issue324). The file is often engine-authored - an obligation record the Stop hook wrote - so this is most likely exactly when the project is unhealthy. Escape without bypassing anything: `git stash push -u -- <path>`, migrate, `git stash pop`. NEVER --no-verify: it disables every other check to get past one. (3) Read `keel status` and write down what it says about library drift, so a unit that was already behind is not later blamed on the migration.";
        :>> producedArtifact = "A committed SHA on the old engine with a green gate, and a stated list of what was already behind.";
        :>> owner = Owner::either;
    }

    action pm1Capability : ProcessStep {
        :>> id = "d0275a01-0901-4c1e-9f10-1a2b3c4d5e71";
        :>> title = "Check what the new engine REMOVES, not just what it adds";
        :>> triggerCondition = "Before applying the migration, once the target engine version is known.";
        :>> actionText = "An upgrade that only adds is safe by construction; one that REMOVES or RENAMES a command, guard, schema type or rule breaks whatever named the old thing. D0252 clause A makes this checkable rather than a reading exercise: a unit declares the capabilities it invokes and an install REFUSES against an engine lacking one, naming it. Until that declaration exists for the capability class in question, this step is a manual grep of the project's own skills, processes and workflows for names the new engine no longer has - and the honest thing is to say which of the two you did. THE HALF-UPDATE IS THE TRAP: taking a newer library UNIT while the binary stays pinned is the case no refusal covers today, and it fails silently rather than loudly.";
        :>> producedArtifact = "Either a capability refusal that names what is missing, or a stated list of the project's own references to removed names.";
        :>> owner = Owner::either;
    }

    action pm2Apply : ProcessStep {
        :>> id = "d0275a01-0901-4c1e-9f10-1a2b3c4d5e72";
        :>> title = "Apply the migration and let it roll back rather than helping it";
        :>> triggerCondition = "Preflight is green and the removals are known.";
        :>> actionText = "`keel migrate .` - it detects the vintage by which steps still apply rather than by a stamped number, applies them, and re-stamps the pin. If it refuses, the refusal is the product working: outside version control there is no way back from a bad run, and a half-migrated tree mixed with uncommitted edits cannot be told apart afterwards. If it FAILS mid-apply it restores every file it wrote; if it is interrupted, the next run recovers from a pre-armed marker. Do not hand-repair a partial migration - re-run it. A rolled-back migration is a refused write at project scale, and a refused write is recoverable where a lost one is not.";
        :>> producedArtifact = "A migrated tree with the pin re-stamped, or a tree byte-identical to the preflight SHA. There is no third outcome.";
        :>> owner = Owner::ai;
    }

    action pm3Reconcile : ProcessStep {
        :>> id = "d0275a01-0901-4c1e-9f10-1a2b3c4d5e73";
        :>> title = "Reconcile what the project OWNED against what the engine resynced";
        :>> triggerCondition = "Migration reported success.";
        :>> actionText = "The migration resyncs engine files over the project's copies, and the project's own choices must be shown to have survived rather than assumed to have. Read back, each one specifically: the PIN is re-stamped and any comment the project added to it is still there; ADOPTION is unchanged - a process or viewpoint the project deactivated is still off, because silently re-arming a control the project turned off is a change nobody decided (issue314); the six project-owned contracts are untouched; and any engine file the project CUSTOMISED is either preserved or reported as a conflict, never silently replaced. Then gate. A migration that leaves a project un-gateable is the partial migration D0067 calls the most expensive outcome.";
        :>> producedArtifact = "A read-back showing pin, adoption and project-owned contracts intact, plus a green gate ON THE MIGRATED TREE.";
        :>> owner = Owner::either;
    }

    action pm4ProveItRemotely : ProcessStep {
        :>> id = "d0275a01-0901-4c1e-9f10-1a2b3c4d5e74";
        :>> title = "Make the project's OWN CI say it worked - a local green is not the claim";
        :>> triggerCondition = "The migrated tree gates green locally.";
        :>> actionText = "Push, and read the project's own gate workflow result. This step exists because a local gate proves the migration on ONE machine with ONE binary, and the whole point of the pin is that other machines exist. It is also the step most likely to be skipped and most likely to be broken: a scaffolded project's CI has never run at all (issue327 - the install step is three echoes), and a project that imported decision-channel acquired two workflows that cannot build (issue326). SO CHECK WHETHER THE PROJECT'S CI IS ACTUALLY RUNNING before treating its silence as success. A red CI that everyone has learned to ignore is worse than no CI, because it looks like coverage.";
        :>> producedArtifact = "A CI run on the migrated tree that actually executed the project's gate, with its result read.";
        :>> owner = Owner::either;
    }

    action pm5FeedBack : ProcessStep {
        :>> id = "d0275a01-0901-4c1e-9f10-1a2b3c4d5e75";
        :>> title = "Report what the migration cost UPSTREAM, because the engine cannot see it";
        :>> triggerCondition = "The migration is complete, or it failed and was rolled back.";
        :>> actionText = "File what happened as an Issue on the ENGINE, not only in the project: what refused, what needed a hand-repair, what the capability check missed, how long it took. This is not courtesy - it is the only channel that exists. The engine repo cannot migrate itself, so every defect in this path is invisible from upstream until a downstream project reports it, and the record is unambiguous: seven defects here, none found by a test, three found by one downstream session in a day. A migration that went fine is worth reporting too, and specifically: it is the only evidence that the path works, since no test can produce it.";
        :>> producedArtifact = "An Issue on the engine repository naming what this migration actually cost.";
        :>> owner = Owner::either;
    }
}
