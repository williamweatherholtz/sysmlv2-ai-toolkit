// D0417 (status: the `status` field below is the truth) — recallMeasurementNamesItsBinary: every run of recall_ab.py and recall_bench.py names the build of the binary it interrogated, before the first case and again after the last, and prints no score over a binary it cannot identify
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0417Accept event on sign-off.
package Decision0417 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0417 : Decision {
        :>> id = "f0b422c6-f355-4fa6-baee-16a261a6f8de";
        :>> title = "recallMeasurementNamesItsBinary: every run of recall_ab.py and recall_bench.py names the build of the binary it interrogated, before the first case and again after the last, and prints no score over a binary it cannot identify";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "recall_ab.py was run while cargo was still relinking target/release/keel.exe and printed 6/8 at 5282 ms; three runs against the settled image read 7/8 at about 1050 ms. Nothing in the 6/8 output said which binary had answered, so the number looked exactly like a regression of the function that had just been refactored (issue406). This is the issue386 family - an instrument reporting over a file it does not control - reaching the measurement harnesses, whose numbers are what recall Decisions cite.";
        :>> decision = "NOT A FORK: one shared helper is added and both harnesses call it. recall_binary.py runs `<binary> version` and returns the build line (`keel 0.4.1 build df7a54e+dirty`) or raises naming the reason - absent, could not start, exited non-zero, no `build commit:` line. Each harness prints `binary : <path> = <line>` before its first case and aborts with `ABORT: no measurement - <reason>` and exit 2 when the line cannot be read; after its last case it re-reads the binary and prints `unchanged through the run`, or disowns the numbers and exits 3 when the build changed underneath them. KEEL_BENCH_BIN overrides the path so an absent binary is one environment variable away; `recall_binary.py --probe` runs the known-positive and known-negative cases (D0388).";
        :>> rationale = "A number that cannot name the image that produced it is not attributable, and an unattributable number on a Decision is worse than none. The header alone cannot see a relink that finishes mid-run, which is the case that produced 6/8; the footer is what catches it. Neither harness is on a gate, so this is legibility rather than blocking - a stale or mid-relink measurement is visible in its own output instead of indistinguishable from a settled one. The helper is shared so the two harnesses cannot drift apart on what they name.";
        :>> consequences = "Both harnesses gain two lines of output and two exit codes (2: no measurement, 3: the numbers above are not attributable). The next recall Decision that cites either harness carries the build line. The two Sensor records in engine-instruments.sysml name issue406 among their known failure modes. No guard, hook, skill or process changes; the knowledge-graph-memory skill row is untouched.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0417Accept : Test { :>> id = "c21b5f3d-034f-4cde-9a9a-a787a2dfa863"; :>> method = VerificationMethod::confirmation; :>> procedureText = "AUTO-ACCEPTED under standing consent (D0207). Their standing words, verbatim: 'issues raised are automatically accepted. they can be customizedly changed post-fact.' Not individually reviewed; override by a superseding Decision (D0290) or the human's quoted word in chat (D0289)."; }
    part d0417AcceptR1 : TestResult { :>> id = "c0eb5c2d-b57f-4395-a75f-0296d319deb4"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "872497f"; :>> judgedAt = "2026-09-09"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
