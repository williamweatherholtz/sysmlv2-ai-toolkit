// D0211 (PROPOSED — NOT YET ACCEPTED) — freezeEnforcementSurface: the guard source, hook config and CI workflows join the keystone lock - a control changes only with a signed Decision (D0209 clause 2)
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0211Accept event on sign-off.
package Decision0211 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0211 : Decision {
        :>> id = "9563b7b5-1192-48f3-a72a-2b79df8a015e";
        :>> title = "freezeEnforcementSurface: the guard source, hook config and CI workflows join the keystone lock - a control changes only with a signed Decision (D0209 clause 2)";
        :>> createdAt = "2026-08-23";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::proposed;
        :>> context = "D0209 clause 2, chartered by the manager-inversion panel. issue236 (D0208 control eval, HIGH) proved an agent could disarm a blocking control by editing its DEFINITION, and dcRulesUnderKeystone closed only the DECLARED-rule leg (.engine/rules/). The rest of the enforcement surface was still freely editable: the guard SOURCE (keel-cli/src/guards.rs, adherence.rs), the local hook CONFIG (.githooks/), and the CI WORKFLOW files (.github/workflows/) that run the gates on infra the agent cannot touch. A guard whose logic is gutted while its name and severity stay put, a hook quietly disabled, or a CI gate deleted are all the same self-modification class the panel named - and none tripped the keystone.";
        :>> decision = "The keystone lock (process_change guard) now covers the ENFORCEMENT SURFACE in addition to process definitions: a staged change to guard source (the files defining any guard, currently keel-cli/src/guards.rs and keel-cli/src/adherence.rs), to .githooks/, or to .github/workflows/*.yml requires a co-committed #ProspectiveChange/#SafetyChange-marked Decision - a human-signed act - exactly as a process definition does. A distinct is_enforcement_surface predicate keeps the intent legible next to is_process_def. A coverage AUDIT (unit test enforcement_surface_covers_every_guard_source) scans the real src/ tree for and fails CI if any guard-defining file sits outside the lock, so a NEW guard file cannot be added to escape it. This does NOT verify a guard's logic is correct - only that changing it is a deliberate, signed, auditable act; correctness stays with code review and the recurring control evaluation.";
        :>> rationale = "Freezing the surface is what turns 'the agent should not disarm its own controls' from a hope into a mechanism. The friction is real and intended: every guard/hook/CI edit now costs a marked Decision - but that is the D0209 thesis (rigor around the AI) and D0054 friction is outweighed here because the surface changes rarely and each change is exactly the kind that must be deliberate. The lock is re-derived independently in CI by audit-adherence (clause 1) over the pushed range, so a bypassed local hook does not defeat it. The coverage audit makes the lock self-checking rather than a hand-list that silently rots.";
        :>> consequences = "Editing keel-cli/src/guards.rs, keel-cli/src/adherence.rs, any .githooks/ file, or any .github/workflows/*.yml now fails the process-change guard unless a marked Decision is co-committed - INCLUDING this very commit, which carries D0211 as that Decision. Adding a guard-defining file outside GUARD_SOURCE_FILES fails the coverage test in CI. The remaining residual (a guard kept blocking but rewritten to check nothing) is unchanged and stays owned by review + control-eval, stated not hidden. Doc-sync: CLAUDE.md section 5 and .engine/docs/guards.md gain the enforcement-surface note in this commit.";
    }
}
