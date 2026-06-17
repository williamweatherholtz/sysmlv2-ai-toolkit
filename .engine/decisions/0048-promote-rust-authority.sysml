// =====================================================================
// D0048 — promote-rust-authority: the Rust sysmlv2 toolchain becomes the
// canonical orient/validate path; the Jupyter pilot kernel + query.py are
// demoted to secondary (deep-check / cross-reference). Ends the JVM leak.
// Accepted 2026-06-16 by wweatherholtz ("yes promote rust ... just do so now").
// =====================================================================
package Decision0048 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0048 : Decision {
        :>> id = "fc012d3e-4f50-4126-376f-21324a5b6c7d";
        :>> title = "D0048: promote the Rust sysmlv2 toolchain to the canonical orient/validate authority";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 17 (rustToolchainFix, W1) repaired the Rust CLI: `sysmlv2 validate .` exits clean (occurrence-def parse fix) and `sysmlv2 orient .` matches query.py exactly on in_progress_sprints/ready/suspect/invalidEvidence. The parity check further showed the Rust orient counts 92 tasks — exactly the structural number of `action <name>;` declarations — while query.py counts 91 (a latent query.py undercount, issue012). So Rust is now AT LEAST as correct as the Python+kernel path, and faster (no ~20s JVM startup; no orphaned-kernel leak, issue004). The kernel path's JVM leak was the original motivation for W1. User directive: 'yes promote rust after parity is achieved. in fact, just do so now. we'll reach parity soon enough.'";
        :>> decision = "PROMOTED: the Rust `sysmlv2` binary is the canonical (authority) path for orient and for .tracking validation — (1) the pre-commit hook validates `.tracking/` with `sysmlv2 validate .` (no kernel) while kernel validators remain for `.engine/` schema/workflow/instance + as the on-demand/CI SysML oracle; (2) orientation computed with `sysmlv2 orient`/`whats-next`, query.py orient retained as secondary cross-check; (3) CLAUDE.md §1/§5 updated to name the Rust path first; (4) parity_check.py cross-checks and fails on qualitative-set divergence (tolerating the known query.py count delta, issue012); query.py is NOT deleted.";
        :>> rationale = "Sprint 17 (W1) repaired the Rust CLI so `sysmlv2 validate .` exits clean and `sysmlv2 orient .` matches query.py on in_progress_sprints/ready/suspect/invalidEvidence — Rust is now AT LEAST as correct as the Python+kernel path (it counts 92 `action` declarations vs query.py's 91, a latent query.py undercount, issue012) and faster (no ~20s JVM startup; no orphaned-kernel leak, issue004, the original motivation for W1). User directive: 'yes promote rust after parity is achieved. in fact, just do so now. we'll reach parity soon enough.'";
        :>> consequences = "Routine commits (tracking edits) no longer start a JVM — the kernel-leak (issue004) is eliminated for the common path. .engine schema/process/workflow changes still run the kernel validators (deeper SysML semantics than the Rust validator). The Rust toolchain now carries authority obligations: its correctness must be guarded (parity_check.py + the existing BDD/cargo tests + W4 deliverableSuspicion, which ties Rust source changes to the parity TestResults). issue012 (query.py undercount) tracked for later reconciliation; non-blocking since Rust is the oracle. Supersedes the CLAUDE.md §5 'each starts the pilot kernel' assumption for the tracking layer.";
    }

    verification d0048Accept : Test { :>> id = "26bc2b9e-811d-45d8-8e69-b0763f6eb2df"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0048 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0048AcceptR1 : TestResult { :>> id = "5b004b14-712e-4652-8c30-b745eddee881"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
