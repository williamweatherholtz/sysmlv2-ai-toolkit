// D0397 (status: the `status` field below is the truth) — ciProbeMarkerDiscovery: the CI script-probe step discovers a probe by a dedicated ci-probe marker line, not by the raw flag string, so the sensor is never a false positive
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0397Accept event on sign-off.
package Decision0397 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0397 : Decision {
        :>> id = "bfba5663-1646-4f03-984b-638fff547673";
        :>> title = "ciProbeMarkerDiscovery: the CI script-probe step discovers a probe by a dedicated ci-probe marker line, not by the raw flag string, so the sensor is never a false positive";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0394 accepted the CI script-probe gate and described discovery as 'by the flag string in the file'. Wiring it revealed that the raw flag string is too loose: scripts/exec_brief/facts.py mentions the flag in its section-14 prose (it COUNTS the probe scripts) and would be run as a probe, but it is the sensor, not a probe, and running it needs the keel binary and git. This Decision records the refinement that keeps D0394's intent - discovered from the tree, gated - while fixing the false positive. It is co-committed with the ci.yml change because the enforcement surface changes only with a marked Decision (D0209 clause 2), and it is covered by the human's signature on D0394, whose consequences name 'ci.yml carries the step'.";
        :>> decision = "NOT A FORK: a refinement of the mechanism D0394 chose, not a new direction. A script opts into the CI script-probe gate by carrying a `# ci-probe: <flag>` line, discovered by a line that starts with that marker so the discovery code itself never self-matches; the step runs each marked script with its flag and fails the build on any non-zero exit. A script that only mentions a flag in prose - the sensor facts.py - is not run, because it carries no marker. This is the stricter reading of D0394's 'discovered from the tree rather than listed': the opt-in is a declared marker, not an inferred string.";
        :>> rationale = "D0394's raw-flag discovery would have run the facts sensor as a probe, which is both a false positive and a real failure mode on the CI host (no keel binary bound the way a session binds it). A dedicated marker is unambiguous, matches D0361's 'declared' philosophy for instruments, and is self-reference-safe by construction (a line-start check cannot match the search code). The human signed the gate on D0394; this is the enumerated step that carries it, so it does not re-ask (D0396).";
        :>> consequences = "ci.yml scans each scripts/*.py for a `# ci-probe:` line and runs the marked ones as a gate; the four probes (artefact, check_templates, issue400_reproduction, textpatch) carry the marker; facts.py section 14 counts the marker rather than a prose mention. A future probe joins by carrying the marker. This Decision is accepted at record time as PLAN-COVERED by d0394 step 'ci.yml carries the step'.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0397Accept : Test { :>> id = "e25d22fa-4fc3-47a3-a9f8-98522ae310f3"; :>> method = VerificationMethod::confirmation; :>> procedureText = "PLAN-COVERED under d0394 step 'ci.yml carries the step': the human signed the plan itself and this enumerated step does not re-ask (D0375 option C / D0396). The plan's judge: wweatherholtz."; }
    part d0397AcceptR1 : TestResult { :>> id = "a79f2d86-7263-4364-b8d8-330e9c3f3b2d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "639d8d3"; :>> judgedAt = "2026-09-09"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
