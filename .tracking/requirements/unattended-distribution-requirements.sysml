// =====================================================================
// UnattendedDistributionRequirements — the two Needs decomposed into contracts a test can fail.
//
// Every threshold below is stated so that a run either meets it or does not. Where a requirement
// could only be checked by inspection it is written as the OBSERVABLE that stands in for it, and
// says so - an unmeasurable requirement is an indicator wearing a contract's clothes (D0088).
//
// ORDER IS DELIBERATE. srMigrationIsReversible comes first and everything about auto-update depends
// on it: without a rollback, "verified-or-reverted" has no second half, and an auto-update would be
// the failure I triggered accidentally during design (a truncating pipe killed migrate mid-run and
// left one file written with the pin unstamped).
// =====================================================================
package UnattendedDistributionRequirements {
    private import EngineElement::*;
    private import EngineRequirements::*;
    private import EngineRelationships::*;

    part srMigrationIsReversible : SystemRequirement {
        :>> id = "755826e1-7af5-41bf-9601-601f85e41888";
        :>> title = "A migration that does not complete leaves the tree byte-identical to its pre-migration commit";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> statement = "`keel migrate` records the pre-migration commit before writing anything, and on ANY non-completion - a blocker found mid-apply, an I/O failure, an interrupted process - restores every file it wrote. THRESHOLD: after an interrupted migration, `git status --porcelain` over .engine/ and .tracking/ is EMPTY. The recovery must be automatic: today the only recovery is an error message advising the human to run `git checkout -- .`, which is a hope rather than a mechanism. Held by a FAULT-INJECTED test - a rollback never observed to run is a claim (D0253).";
    }

    part srUpdateIsVerifiedOrReverted : SystemRequirement {
        :>> id = "26ee9e01-97c0-4a50-9f7b-527f14026015";
        :>> title = "An automatically applied engine update is kept only if the project's own gate passes after it";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> statement = "After an unattended update the project's gate runs - validate, every enforced guard, check-engine. THRESHOLD: on green the update is retained and what moved is reported including the pin; on ANY red the tree is restored to the pre-update commit and the gate's own output is reported verbatim. There is no third outcome: the tree after an unattended update is either verified-green or byte-identical to before. NOTE ON WHAT THIS DOES NOT SAY: it does not claim the update is correct, only that an unverifiable one does not survive.";
    }

    part srUnprovenUpdateAsksRatherThanActs : SystemRequirement {
        :>> id = "d6059af5-7554-46b9-866c-d7745dde675a";
        :>> title = "An update that cannot be auto-verified becomes a recorded proposal, not a silent skip";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> statement = "When an update is reverted, or when its preconditions refuse it (a dirty tree, no version control), the outcome is RECORDED where the human will see it rather than logged and forgotten. THRESHOLD: a reverted update leaves a durable artefact naming the version attempted, the gate that failed and its output; and a subsequent run does not silently retry the same version without saying it is retrying. The failure this prevents is the one that just cost seven hours: an automatic process whose failures are only visible to whoever reads the console at the right moment.";
    }

    part srUnattendedPullIsIdempotent : SystemRequirement {
        :>> id = "03727fef-4827-4a12-9249-2980235d7c1c";
        :>> title = "A repeated pull records nothing twice and reports what it skipped";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> statement = "THRESHOLD: running the pull twice over the same open issues produces exactly the Statements the first run produced - no duplicates under two ids - and the second run REPORTS the skip count rather than exiting silently. Idempotency is enforced on the issue's durable URL, checked inside the write lock so two concurrent pulls cannot both find an issue absent. Already met by the ingestion path; stated here because a scheduled caller makes it load-bearing rather than convenient.";
    }

    part srDriftIsReportedPerUnit : SystemRequirement {
        :>> id = "131c1e9b-fc2e-4256-a60e-88edac983a37";
        :>> title = "One command names every installed unit that is behind the library";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> statement = "THRESHOLD: for each unit in the project's install record, the report states the installed version, the library's version, and whether it is behind - and for each behind unit prints the exact update command. A project with no library configured says so rather than reporting zero drift, because 'nothing is behind' and 'I cannot tell' must not render identically. Today this requires reading `keel library list` and `keel process show` and comparing by hand.";
    }

    // Added when the Need split (st063): the untrusted-input half had a MECHANISM in code (guard 56)
    // and no contract citing it, which is how a Need becomes orphaned while looking covered.
    part srUntrustedInputRoutesToADecision : SystemRequirement {
        :>> id = "d08380a5-d9bb-4319-b259-88906c6b1864";
        :>> title = "A story descending from an unauthenticated utterance cannot be routed to work without a Decision";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> statement = "THRESHOLD: for every UserStory whose #DerivedFrom chain reaches a Statement recorded at sourceTrust=untrusted AND which carries an #Implicates edge, at least one target of that edge is a Decision. An UNROUTED story is not a violation - untriaged is not misrouted, and a check that fired at ingest would make ingestion itself feel like a fault. The tier is read from the utterance rather than re-derived, so a repository whose visibility changes after an issue was filed does not retroactively alter what that issue was permitted to do.";
    }

    // ── which Need each contract serves ──────────────────────────────────────────────────────────
    // Re-pointed after the split (st063): reversibility is about undoing a change the engine
    // made to itself, not about who asked for it.
    satisfy nUnverifiableChangeReverts by srMigrationIsReversible;
    satisfy nUnverifiableChangeReverts by srUpdateIsVerifiedOrReverted;
    satisfy nUnverifiableChangeReverts by srUnprovenUpdateAsksRatherThanActs;
    satisfy nUnattendedCurrency by srUnattendedPullIsIdempotent;
    satisfy nUnattendedCurrency by srDriftIsReportedPerUnit;
    satisfy nUntrustedInputPlans by srUntrustedInputRoutesToADecision;
}
