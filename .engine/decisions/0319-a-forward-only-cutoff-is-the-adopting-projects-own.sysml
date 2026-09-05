// D0319 (status: the `status` field below is the truth) — forwardOnlyCutoffIsTheProjectsOwn: a dated rule cutoff reads as the later of the engine's literal and the adopting project's declared adoption date
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0319Accept event on sign-off.
package Decision0319 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0319 : Decision {
        :>> id = "ec5bdd7d-f728-44f0-91f3-d5fdb31383ee";
        :>> title = "forwardOnlyCutoffIsTheProjectsOwn: a dated rule cutoff reads as the later of the engine's literal and the adopting project's declared adoption date";
        :>> createdAt = "2026-09-05";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "GH#47 (st082, issue352): delegatedAcceptanceSubstanceRule and delegatedConfirmationSubstanceRule ship predicate = ...(2026-08-22) and ...(2026-08-23) - the dates this repository adopted D0192 and D0198 - so a downstream project migrating into 0.3.1 was retro-failed on acceptances it recorded before it ever had the rule. The general form of issue068 (a new control must never retro-fail what was authored under the process in force): the literal is right for the engine's own history and wrong for everyone else's.";
        :>> decision = "The reading of a cutoff-bearing predicate term (acceptQuotesDelegatedWords, confirmationQuotesOrAttested) is raised to the LATER of its literal date and the project's `declaredAt` in `.engine/contracts/adoption-profile.toml` (checks::raise_cutoffs, applied in rule_violations for every ElementRule); the literal stays in rules.sysml as the engine's own value and rules.sysml is not edited per project; and `keel migrate`, when it ADDS an adoption profile to a project that never had one, stamps declaredAt with the migration date rather than shipping this repository's.";
        :>> rationale = "Why raise the reading and not parameterise the rule text: rules.sysml is engine-shipped and resynced, so a per-project literal would be overwritten on the next migrate, and a symbolic token would move the grandfather line out of the rule where a reader looks for it - the literal names when the rule came into force here, and the project's own contract names when it arrived there; the later of the two is the only date before which the project cannot have recorded against the rule. Why declaredAt: it is the one adoption date the project already declares (D0174), written by init and, from this Decision, by migrate when the profile is added - so no new fact is authored. Why this repository moves nowhere: its declaredAt (2026-08-21) precedes both literals. Arming (D0298): a unit test on raise_cutoffs (raised, unchanged, compound predicate, absent, malformed) and keel-cli/tests/forward_only_cutoff_is_the_projects_own.rs on a real scaffold - an unquoted delegated acceptance judged 2026-08-30 passes on a project declared today, and one judged today fails.";
        :>> consequences = "Every downstream project's substance rules now begin at its own adoption; nothing changes in this repository's gate. issue352 is resolved by dcForwardOnlyCutoffIsTheProjectsOwn. rules.sysml comments and guards.md's grandfathering paragraph say so. STATED RESIDUAL: a project whose adoption-profile.toml predates its actual adoption of the rule (a profile written at init, a rule that arrived by a later migrate) still reads the profile date; the rule's own arrival date is not recorded per project.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0319Accept : Test { :>> id = "d6fae7b6-d16a-4f09-bbf4-b65d0b47829f"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0319AcceptR1 : TestResult { :>> id = "f6c4217a-eea6-4a69-95bb-4711cae0d335"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d9e250e"; :>> judgedAt = "2026-09-05"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
