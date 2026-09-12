// D0455 (status: the `status` field below is the truth) — gatingFoldHookAndWorkflowRewrites: the hooks, CI workflows and the gating-workflow-history needles are rewritten to the routed spellings in the D0452 fold commit
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0455Accept event on sign-off.
package Decision0455 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: Measured 2026-09-12 by the committed transform's dry run (.engine/tools/migrations/2026-09-12-gating-family-fold.py --dry-run) at 6821b5c: 302 call sites across 91 files, conservation reconciled per verb per file, re-plan 0 after --apply. On the enforcement surface the transform rewrote .githooks/pre-commit lines 89, 99, 104 and 109 (gate validate, gate guard, gate check-engine, gate rules --enforce), .githooks/post-commit line 93, .github/workflows/ci.yml (five step blocks) and release.yml lines 52-53. By hand: the gating-workflow-history guard's needles (keel-cli/src/guards.rs, the array under fn gating_workflow_history) read the routed spellings, with a D0388 pair in keel-cli/tests/gating_workflow_history_reads_routed_spelling.rs; hardening reads dispatched 39, namedInHelp 39.
    // #SafetyChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #SafetyChange part d0455 : Decision {
        :>> id = "7c253bf7-75e8-48d0-8926-97b00f4a0967";
        :>> title = "gatingFoldHookAndWorkflowRewrites: the hooks, CI workflows and the gating-workflow-history needles are rewritten to the routed spellings in the D0452 fold commit";
        :>> createdAt = "2026-09-12";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0452 folds the ten gating verbs under keel gate and keel audit and enumerates the hook and workflow rewrites as one step of the work, the step that sits on the enforcement surface the keystone lock covers (D0209 clause 2): .githooks, the CI workflows and the guard source. The human accepted D0452 by their own word on 2026-09-11. The lock reads a co-committed marked Decision from the commit that changes the surface, and D0452's acceptance landed a commit earlier, so the fold commit needs a marker Decision of its own naming exactly the step it carries out.";
        :>> decision = "The fold commit rewrites every hook and workflow line that names a gating verb to its routed spelling, by the committed transform where the line matches keel followed by the verb and by hand where the invocation is built in Rust: the pre-commit and post-commit hooks under .githooks, ci.yml and release.yml under .github/workflows, and the needles of the gating-workflow-history guard, which decide whether a workflow runs the gate and would otherwise fall silent over every rewritten workflow. Nothing else on the enforcement surface changes: no guard is added, removed or re-tiered, no hook event changes, and every rewritten line keeps its arguments. NOT A FORK: D0452 chose the course; this Decision names the one step of it that the keystone lock requires to be signed in its own commit.";
        :>> rationale = "A hook that names a verb the binary lacks blocks every commit including the fix, and a CI step that names one turns every push red; D0452 made the one-commit rule the point of the fold for this family. The guard needles are on the same surface for the opposite reason: a needle that still read keel validate would match nothing after the rewrite, and the guard would report zero scanned over a shallow checkout, which is a pass it did not verify. Recording this as a plan-covered step rather than re-asking the human keeps D0396's shape: they signed the plan, the plan enumerates the step, and the marker still lands on the commit so the lock has a Decision to read.";
        :>> consequences = "The fold commit carries this Decision beside the rewritten hooks, workflows and guard source and the keystone lock reads it. Every downstream project's hooks and CI that name a gating verb break by name on the next engine update, as D0452 states; the keel-gate.yml that keel init lays down is among the rewritten sources (its two steps run keel gate validate and keel gate guard), and the scaffolded pre-commit already ran keel gate --workspace, so a fresh project starts on the routed spellings. The gating-workflow-history guard's probe pair is the permanent control on the needles: a shallow workflow running keel gate validate fails it, a full checkout passes it.";
    }

    // D0303 layering: the enforcement-surface step of the fold D0452 fixed
    #DependsOn dependency from d0455 to d0452;

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0455Accept : Test { :>> id = "d8105c66-a327-4bea-b0f3-4d797a1f54b9"; :>> method = VerificationMethod::confirmation; :>> procedureText = "PLAN-COVERED under d0452 step 'the hook and workflow rewrites': the human signed the plan itself and this enumerated step does not re-ask (D0375 option C / D0396). The plan's judge: wweatherholtz."; }
    part d0455AcceptR1 : TestResult { :>> id = "8c5c6a6b-0c2c-45a6-a75b-b24b4eb557d9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6821b5c"; :>> judgedAt = "2026-09-12"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeFable5"; }
}
