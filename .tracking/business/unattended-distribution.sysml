// =====================================================================
// UnattendedDistributionNeeds — the project keeps itself current, and its autonomy stops exactly
// where proof stops.
//
// SOURCE. st047 (an issues pull+ingest pipeline; private acts, public plans; can it deploy globally;
// how do other projects pull the skill), st048 (is there a Django-migrations equivalent; antiquated
// data with no indication what process authored it), st049 (auto-update consent gated, IF correctness
// can really be guaranteed), translated as us059, us060 and us061. Elicited direction, not derivation:
// every Need here cites a UserStory that cites an utterance.
//
// CONSOLIDATED, applying the human's own stated principle to their own words: "having a use case for
// typing A, another for typing B, could be consolidated into the need for having a keyboard". Three
// statements spanning issue pulls, library sync, vintage archaeology and update consent produced TWO
// Needs, because they are two principles wearing several surfaces:
//
//   nUnattendedCurrency      <- us059 (pull pipeline, push/pull ergonomics), us060 (know what is
//                               antiquated and under what process it was authored)
//   nAutonomyBoundedByProof  <- us059 (private acts / public plans), us061 (auto-update only where
//                               correctness is guaranteed, consent where it is not)
//
// WHAT IS NOT HERE, deliberately. "See installed versus upstream at a glance" is a real gap I found
// and PROPOSED; the human has not asked for it. It is folded into nUnattendedCurrency rather than
// made its own Need, because a Need with no cited utterance is my judgment (D0216) and inventing one
// to hold my own proposal would dress my idea as their demand.
// =====================================================================
package UnattendedDistributionNeeds {
    private import EngineElement::*;
    private import EngineNeeds::*;
    private import EngineRelationships::*;

    part nUnattendedCurrency : Need {
        :>> id = "c4e4a2ea-9e23-4805-ad28-60cfd55be20f";
        :>> title = "A project stays current without the human driving each step, and can see where it is behind";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "Inbound reports, shared processes and the engine version reach a project without a person fetching each one by hand, and the project can state where it is behind: which issues are unrecorded, which units are older than the library's, and which engine version it is pinned to versus available. MEASURABLE: an unattended run records every open issue no Statement cites and records none twice; a drift report names every installed unit whose library version is higher; and the corpus-wide question 'what was authored under a process version since superseded' returns an answer rather than timing out. WHAT THIS EXCLUDES: currency is not conformance - being up to date says nothing about whether the project's own work is any good, which is what the gates are for.";
    }

    part nAutonomyBoundedByProof : Need {
        :>> id = "95bc2ab9-8538-4df6-a734-1859cd4bf194";
        :>> title = "Automatic action extends exactly as far as it can be proven safe, and asks beyond that";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "Where an action's outcome can be verified and reversed, the engine takes it unattended. Where it cannot, the engine PLANS it, records the plan, and waits for a human. The boundary is a property of the evidence available, not of how large the change looks. MEASURABLE: an input whose authority cannot be established is routed to a proposed Decision and never to an implementation; an engine update is retained only if the project's own gate passes after it and otherwise leaves the tree byte-identical to before; and every automatic action states which of the two it was before acting. WHAT THIS EXCLUDES: this is not a claim that the automatic actions are CORRECT - only that they are verified-or-reverted. A plan a human accepts can still be a bad plan; that judgment is theirs and this Need does not pretend to cover it.";
    }

    // ── the utterances these came from ───────────────────────────────────────────────────────────
    #DerivedFrom dependency from nUnattendedCurrency to us059;
    #DerivedFrom dependency from nUnattendedCurrency to us060;
    #DerivedFrom dependency from nAutonomyBoundedByProof to us059;
    #DerivedFrom dependency from nAutonomyBoundedByProof to us061;

    // ── UseCases: the concrete usage, where consolidation does NOT apply ─────────────────────────
    //
    // Needs are consolidated; use cases are deliberately not. Each below is a distinct interaction
    // someone actually performs, and three of them are FAILURE paths - because an autonomy boundary
    // that is only ever exercised on the happy path is a claim rather than a boundary.

    part ucScheduledPullBringsWorkIn : UseCase {
        :>> id = "4104bb05-19b8-4d11-9f3d-b37a8de72df0";
        :>> title = "An unattended pull records new issues at the authority they arrived with";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "On a schedule, the project pulls its repository's open issues. Each one no Statement already cites is recorded verbatim, attributed to the reporter's login, carrying its URL and its trust tier. A repeat run records nothing twice and says how many it skipped.";
    }

    part ucPublicIssuePlansOnly : UseCase {
        :>> id = "390fe918-3f1a-4e10-b173-3fd3f0921e10";
        :>> title = "An issue anyone could have filed becomes a proposed Decision, not a change";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "An issue from a public repository is triaged fully - reading is not obeying, and a stranger's report can be entirely correct - and anything that would change the project is written as a proposed Decision for a human to accept. Nothing is implemented from it unattended.";
    }

    part ucEngineUpdateAppliesItself : UseCase {
        :>> id = "788cc9c5-7797-43d1-8dc7-3897bbe64def";
        :>> title = "An engine update applies itself and the project gates green afterwards";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "A newer engine is available. The pre-update commit is recorded, the migration is applied, and the project's own gate runs. It passes, so the update is kept and the human is told what moved - including the pin.";
    }

    part ucEngineUpdateRevertsItself : UseCase {
        :>> id = "b3f8c07e-e649-4006-8120-d5046ba7e062";
        :>> title = "An engine update whose gate goes red reverts itself and says why";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "The same update, but the post-update gate fails - a new guard the project's existing tree violates, which is the issue089/issue090 class. The tree is restored to the recorded commit, the failure is reported with the gate's own output, and the update becomes a decision for the human rather than a broken project.";
    }

    part ucInterruptedMigrationLeavesNothing : UseCase {
        :>> id = "b03cf6a4-560c-49eb-9454-c4068f1f8846";
        :>> title = "A migration killed part-way leaves the tree exactly as it was";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "A migration is interrupted mid-apply - a timeout, a killed process, an evicted runner. Some files are already written. The tree is restored to its pre-migration state rather than left in a state that is neither vintage. Observed live during design: piping migrate's output to `head` closed the pipe, killed the command, and left one file written with the pin unstamped.";
    }

    part ucProjectSeesItsDrift : UseCase {
        :>> id = "b5e31bf7-0c6f-417b-bad8-771737dc8ef3";
        :>> title = "A project is told which of its units are behind the library";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "One command lists each installed unit with its version and the library's, flags the ones behind, and prints the update command. Today this requires checking each unit by hand against `keel library list`.";
    }

    part ucProcessTravelsBothWays : UseCase {
        :>> id = "008b5565-9787-4f8d-86e3-c2bfba629c44";
        :>> title = "A process authored in one project is adopted by another";
        :>> createdAt = "2026-08-30"; :>> createdBy = "claudeOpus5";
        :>> mainSuccess = "A process written for one project's needs is published to the library and imported by a sibling that lacked it. The receiving project's local adaptations survive an update, and a genuine conflict refuses rather than merging silently.";
    }

    satisfy nUnattendedCurrency by ucScheduledPullBringsWorkIn;
    satisfy nUnattendedCurrency by ucProjectSeesItsDrift;
    satisfy nUnattendedCurrency by ucProcessTravelsBothWays;
    satisfy nAutonomyBoundedByProof by ucPublicIssuePlansOnly;
    satisfy nAutonomyBoundedByProof by ucEngineUpdateAppliesItself;
    satisfy nAutonomyBoundedByProof by ucEngineUpdateRevertsItself;
    satisfy nAutonomyBoundedByProof by ucInterruptedMigrationLeavesNothing;

    // ── The human's adjudication (st062, 2026-08-31) ────────────────────────────────────────────
    //
    // Asked per D0157 what was WIDER than they meant rather than whether the set was good, and given
    // a concrete fork: one Need or two, or cut the size clause. They chose ONE, so the unification of
    // input-trust and update-consent under a single predicate is THEIRS now, not my judgment.
    //
    // SCOPE OF THIS ACCEPTANCE, stated because an attestation must not be read wider than it was
    // given: they adjudicated nAutonomyBoundedByProof, which is what the tab quoted and forked.
    // nUnattendedCurrency was never contested and is NOT covered here - it stays unaccepted and
    // keeps being carried until they answer or I withdraw it (D0269 dsCarry).
    private import EngineVerification::*;

    verification nAutonomyBoundedByProofAccept : Test {
        :>> id = "3851088a-a733-4d95-973c-85977413353d";
        :>> title = "nAutonomyBoundedByProof accepted as ONE Need (human adjudication)";
        :>> createdAt = "2026-08-31"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "The human selected 'Keep ONE Need - trust and update-consent are the same predicate' from a three-way fork that offered splitting it, or cutting the clause 'not of how large the change looks'. They were shown the statement verbatim, what it binds (guard 56's untrusted routing AND the auto-update verify-or-revert gate), what was already true in code, and what I do with each answer. So the unification is an adjudicated choice against named alternatives, not assent to a summary.";
    }
    part nAutonomyBoundedByProofAcceptR1 : TestResult { :>> id = "95edeb50-a603-45cb-bfa6-538a3c5ff7df"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "a16624c"; :>> judgedAt = "2026-08-31"; :>> judgedBy = "wweatherholtz"; }
    #Verify dependency from nAutonomyBoundedByProofAccept to nAutonomyBoundedByProof;

    // ── THE SPLIT (st063, 2026-09-01) — the human reversed yesterday's choice ────────────────────
    //
    // Yesterday they picked "keep ONE Need" from a three-way fork. Today, on the same fork, they
    // picked SPLIT. Both answers were given with the statement quoted in front of them, so this is a
    // change of mind and not a misread - and the trigger is the flip condition I wrote into the fork
    // myself: "wanting auto-act on trusted issues while never auto-updating. That is two Needs."
    //
    // THE EARLIER ACCEPTANCE IS SUPERSEDED, NOT DELETED. It records what they attested on 2026-08-31
    // and it stays visible; a recorded attestation is theirs (D0108), so a reversal is a new fact
    // rather than an edit to an old one. nAutonomyBoundedByProof is superseded by the two Needs
    // below, which divide it exactly along the seam the fork named: WHO is asking versus WHETHER THE
    // OUTCOME CAN BE UNDONE.

    part nUntrustedInputPlans : Need {
        :>> id = "7dcc03ca-cf9c-47e3-88cd-16ec91659306";
        :>> title = "Input whose authority cannot be established may plan, never act";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "An instruction reaching the project from a source whose authority is not established - an issue anyone could have filed, a comment, any unauthenticated channel - is triaged and may produce a PLAN, and may never produce a change nobody accepted. MEASURABLE: a story descending from an untrusted utterance is routed to a Decision and never straight to work; the tier is recorded ON the utterance so a later pass never re-derives it; and an undetermined authority resolves to untrusted. WHAT THIS EXCLUDES: it says nothing about whether the input is CORRECT - a stranger's bug report can be entirely right, and reading it is not obeying it.";
    }

    part nUnverifiableChangeReverts : Need {
        :>> id = "e6d12ee2-9275-4f88-abab-7f7c132e3da8";
        :>> title = "A change the engine makes to itself is kept only if it can be verified, and undone otherwise";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> priority = Priority::must;
        :>> statement = "When the engine changes a project on its own - an engine upgrade, a migration - the pre-change state is recorded first, the project's own gate runs after, and the change survives ONLY if that gate passes. MEASURABLE: after an unattended change the tree is either verified-green or byte-identical to before, with no third outcome; an interrupted change is restored by the next run; and a reverted change leaves a durable record naming the version and the gate that failed. WHAT THIS EXCLUDES: verified is not correct - a gate that passes says the tree is honest, not that the upgrade was wise.";
    }

    #DerivedFrom dependency from nUntrustedInputPlans to us066;
    #DerivedFrom dependency from nUnverifiableChangeReverts to us066;
    #Supersede dependency from nUntrustedInputPlans to nAutonomyBoundedByProof;
    #Supersede dependency from nUnverifiableChangeReverts to nAutonomyBoundedByProof;

    verification autonomySplitAccept : Test {
        :>> id = "c4c08b6b-526b-43e8-ab9f-d7323f5c0d8e";
        :>> title = "The autonomy Need is SPLIT in two (human adjudication, reversing 2026-08-31)";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "The human selected 'Split into two Needs - input trust and update consent diverge' from the same three-way fork on which they had previously selected 'keep ONE Need'. Both selections were made with the statement quoted in front of them and the alternatives costed, so the reversal is an adjudication and not a correction of a misunderstanding. Their reason is the one the fork's own flip condition named: the two surfaces may need different autonomy, and one contract cannot say that.";
    }
    part autonomySplitAcceptR1 : TestResult { :>> id = "ad2080f6-0d4d-4865-b023-2ef70fd1c109"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "5453550"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "wweatherholtz"; }
    #Verify dependency from autonomySplitAccept to nUntrustedInputPlans;
    #Verify dependency from autonomySplitAccept to nUnverifiableChangeReverts;

    // ── nUnattendedCurrency, read back and accepted (2026-09-01) ────────────────────────────────
    //
    // THE ONE NEED NOBODY HAD EVER BEEN ASKED ABOUT. Its sibling was forked and adjudicated twice
    // while this one was never contested, so it reached three shipped capabilities - the pull
    // pipeline, library sync and `keel status` - without a read-back. D0157 says to ask what is
    // WIDER than they meant, so the tab named the widest clause rather than asking whether the Need
    // was good, and offered narrowing and withdrawal beside acceptance.
    verification unattendedCurrencyAccept : Test {
        :>> id = "8c4b1f07-e2d6-4a95-b318-70fa5c92e6d1";
        :>> title = "nUnattendedCurrency accepted AS WRITTEN, including its autonomy clause (Business gate)";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz, 2026-09-01, via the Unattended Distribution decision page returned as a Copy-for-AI digest. ACCEPTS nUnattendedCurrency AS WRITTEN, over the two narrowing alternatives offered beside it. WHAT THIS SPECIFICALLY SETTLES, because it is the clause the tab flagged as widest: 'without a person fetching each one by hand' is an AUTONOMY claim, not merely a convenience one, and it stands - so the scheduled currency pass (dcScheduledCurrency) is licensed by a Need the human has now read, rather than by my inference. The narrowing they declined would have kept reporting automatic while leaving fetching a command they run. WHAT IT DOES NOT SETTLE: the Need excludes conformance explicitly and that exclusion is unchanged; and autonomy over untrusted input remains bounded by nUntrustedInputPlans and D0264, which this acceptance does not touch. Their verbatim selection is quoted in unattendedCurrencyAcceptAttest1.";
    }

    verification unattendedCurrencyAcceptAttest1 : Test {
        :>> id = "9d5c2a18-f3e7-4b06-a429-81f60ad37b2e";
        :>> title = "Quote receipt for unattendedCurrencyAccept (D0198)";
        :>> createdAt = "2026-09-01"; :>> createdBy = "claudeOpus5";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Their selection, verbatim from the returned digest. Tab 'The unaccepted Need', question 'Is anything in it wider than you meant?': [x] 'Accept as written - RECOMMENDED'. The two unselected options were 'Narrow it: currency is REPORTED automatically, but fetching stays a command I run' and 'Withdraw it - the shipped capabilities need no Need above them'. RECORDED RATHER THAN SMOOTHED OVER: this tab's Note field carried text about the CLI - 'remove cruft from CLI. tons of commands, when a smaller subset should do, and parentheticals embedded in descriptions. tighten it up - what core functional hooks do we need in the CLI? a lot of these seem to be variations of an idea cemented as separate cli hooks. and critique all descriptions to trim them down to the real root of what is being done and what a user needs to know to run it' - which is their earlier CLI instruction repeated, and belongs to the FIRST tab's subject, not this Need's. It is read as reinforcement of the CLI decision and NOT as a qualification on this acceptance; no clause of nUnattendedCurrency was altered on the strength of a note about a different tab.";
    }

    // judgedBy differs on purpose: the acceptance is the HUMAN's judgment, the quote receipt is MY
    // transcription of what they sent.
    part unattendedCurrencyAcceptR1 : TestResult { :>> id = "a1e93b7c-06d4-4f28-95b1-3ce8072f4d69"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ac8bdf6"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "wweatherholtz"; }
    part unattendedCurrencyAcceptAttest1R1 : TestResult { :>> id = "b2fa4c8d-17e5-4039-a6c2-4df1830e5b7a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "ac8bdf6"; :>> judgedAt = "2026-09-01"; :>> judgedBy = "claudeOpus5"; }

    #Verify dependency from unattendedCurrencyAccept to nUnattendedCurrency;
    #Verify dependency from unattendedCurrencyAcceptAttest1 to nUnattendedCurrency;
}
