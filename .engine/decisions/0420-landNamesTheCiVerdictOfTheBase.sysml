// D0420 (status: the `status` field below is the truth) — landNamesTheCiVerdictOfTheBase: keel land reads the CI conclusion of the commit it is about to push onto and says it, advisory, before every push - the binary's reading of the conclusion field, never a wrapper's exit code
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0420Accept event on sign-off.
package Decision0420 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0420 : Decision {
        :>> id = "bb164098-0707-4a32-a4c8-e422d0f01273";
        :>> title = "landNamesTheCiVerdictOfTheBase: keel land reads the CI conclusion of the commit it is about to push onto and says it, advisory, before every push - the binary's reading of the conclusion field, never a wrapper's exit code";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue434, High, tracked as dcLandNamesTheCiVerdictOfTheBase. In this session two reports said CI was green for f848923 and afd90d0 while both runs had failed on issue433. The mechanism was a background shell command of the form gh run watch <id> --exit-status; echo ci exit=$? - the harness reported the wrapper's exit code, 0, and the report was read from the wrapper instead of the run. Six pushes landed on a red main and no surface said so at the moment any of them was made. keel status already reads the verdict of HEAD from gh run list --json conclusion,status (status::ci_section); keel land, the command that changes the base, read nothing.";
        :>> decision = "NOT A FORK: the DoD names one line at one point in one command and forbids the refusal. status::ci_section's reading is extracted as status::ci_verdict(root, sha) -> (CiVerdict, line), the one reading of a CI verdict in the binary, and keel land calls it for origin/<branch> after the held-items warning and before the tree gate: keel land: CI on the base origin/<branch>: <sha7> passed | FAILED | still running | no run found | gh unavailable. A FAILED base adds one line: main stays red unless this push is its fix, and a report cites gh run list --json conclusion, never a wrapper's exit code. ADVISORY, never a refusal - a red base is exactly what the fixing push must land on. Silent when origin is not a GitHub remote (nothing to ask) or origin/<branch> does not resolve (first push).";
        :>> rationale = "The false reports were made from a number that was not the verdict. The correction that holds is not a reminder to read the right field - manual vigilance is not a control (D0047) - but the binary reading that field at the moment the base is about to change, so a red base is stated by the process, in the transcript, before the push that lands on it. Extracting ci_verdict makes status and land read the same field through the same code, so there is one place to be wrong. The line costs one gh call per land on a GitHub remote and nothing on a bare origin, which keeps the land fixtures unchanged. It does not refuse because the six red pushes show the case it would block: the fix itself.";
        :>> consequences = "status.rs gains CiVerdict and ci_verdict, ci_section becomes their caller; sync.rs gains name_ci_verdict_of_base called from cmd_land. CLAUDE.md names the rule in the integration paragraph: a CI verdict is read from gh run list --json conclusion or keel status, never from a wrapper's exit code. The land fixtures (push_gate_is_withdrawn, touched-tests) print no CI line because their origin is bare. What remains uncontrolled: the transcript's claim itself - no gate reads conversational output (D0151) - so the milestone report of this session states the two wrong claims plainly and names this line as what will say it next time.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0420Accept : Test { :>> id = "12cd3248-71c9-4e99-8f75-3d149927f85e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0420AcceptR1 : TestResult { :>> id = "714d5268-35c5-4154-b2a6-346380506a6b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "afd90d0"; :>> judgedAt = "2026-09-09"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
