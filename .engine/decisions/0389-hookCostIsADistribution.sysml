// D0389 (status: the `status` field below is the truth) — hookCostIsADistribution: a hook that degrades writes a ledger event for it, and a hook's documented cost is the distribution the ledger holds, never a best case
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0389Accept event on sign-off.
package Decision0389 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: .keel/metrics/hooks.jsonl on this host, 2026-08-21 to 2026-09-08, 13,798 lines, read by scripts in the scratchpad and then by keel enforcement-report with the same numbers: user-prompt 941 fires, median 760 ms, p90 1,385 ms, p99 12,225 ms, max 33,503 ms; 23 fires over the 2,500 ms recall cap (2.44 percent, the skip proxy - the skip itself was never recorded), eight of them on 2026-09-06; post-edit 1,694 fires, median 0 ms, p90 0 ms, p99 2,323 ms, max 54,439 ms, against a documented cost of about 0.35 s; stop median 11.6 s, p99 92 s; subagent-stop median 16.5 s, p99 74 s.
    part d0389 : Decision {
        :>> id = "570fe759-b059-414a-97b8-1142d4e51594";
        :>> title = "hookCostIsADistribution: a hook that degrades writes a ledger event for it, and a hook's documented cost is the distribution the ledger holds, never a best case";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "issue402: the recall payload is dropped when its walk exceeds the cap, so the agent's memory channel is least available exactly when the machine is busiest, and nothing recorded which turns went without facts - the skip was a transcript line, and 808 fires later the count had to be reconstructed from a proxy. In the same ledger the per-edit tier documented at about 0.35 s had a median of 0 ms and a maximum of 54 s. dcHookDegradationIsCounted asks that the skip be counted as a fact, that the documented cost be a distribution, and that the cap question be decided from the ledger.";
        :>> decision = "NOT A FORK: this clause is the instrumentation the item asks for, and its shape follows from the ledger that already exists. A hook that degrades writes a ledger event for the degradation - the user-prompt hook writes recall-skipped, carrying the turn's session and the milliseconds the walk took, whenever it drops the facts past the cap - within the frozen six-field schema, since events are declared vocabulary and fields are not touched. keel enforcement-report reports, per event, median, p90, p99 and max latency by nearest rank beside fires and blocks, and a recall section with the skip count, the rate over user-prompt fires, and the turns that lost their facts; a documented hook cost anywhere in the doc surface is that distribution or a pointer to it, never a single figure.";
        :>> rationale = "A count that exists only in transcripts is reconstructed, when it is reconstructed at all, from a proxy - here a fire whose duration exceeds the cap - which cannot distinguish a skip from a slow turn that still pushed. The ledger is the single instrumentation path D0180 named, and an event there costs one line. Nearest-rank percentiles report values that occurred, so a reader can find the turn behind any number. The best-case figure in CLAUDE.md was true when written and misleading every time the machine was loaded, which is the condition under which a reader consults it.";
        :>> consequences = "main.rs recalled_facts writes recall-skipped; pm.rs gains latency and event_rows and the recall section, with tests binding the rate, the named turn and the percentile rule; CLAUDE.md replaces the 0.35 s figure with the pointer and the rule. Skips before 2026-09-08 exist only as transcript lines; the report says so and names the proxy. What the ledger says about the cap itself is a separate clause, D0390, because it changes what the hook does rather than what it records.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0389Accept : Test { :>> id = "29c692c1-311c-4488-96e6-d9f3e6d63783"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0389AcceptR1 : TestResult { :>> id = "5538410f-49c1-4f93-a1fd-883ab41e6666"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "d071820"; :>> judgedAt = "2026-09-08"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
