// =====================================================================
// D0113 — convert the doc-sync discipline from VIGILANCE into a CONTROL: a warning-level `doc-sync` guard.
// Human-directed 2026-07-14 (process-audit: "are our processes respected?" -> the vigilance-only ones
// are the weak point; convert the highest-value one). The D0070 keystone governing the guard addition
// (#ProspectiveChange). Accepted.
// =====================================================================
package Decision0113 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #ProspectiveChange part d0113 : Decision {
        :>> id = "10a1b0c1-4113-42a3-ab5c-516273849713";
        :>> title = "D0113: doc-sync as a CONTROL — a warning-level guard flags a definitional change with no co-committed doc update";
        :>> createdAt = "2026-07-14";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "A process-audit (2026-07-14) asked whether our MANY processes are actually respected. Answer: the guard-backed ones are (respected by construction — 16 guards held all session); the VIGILANCE-only ones are not reliably — doc-sync/migration/parse-first have no enforcing control, and even this session the AI drifted from parse-first + bypassed a gate via SKIP_VALIDATE. doc-sync was the standout gap: documentation drift was a recorded HIGH critique finding, yet doc-sync had only a skill (run-by-choice), no guard. The human directed converting the highest-value vigilance process to a control; doc-sync was chosen.";
        :>> decision = "Add a WARNING-level `doc-sync` guard (guards.rs, GUARD_NAMES 16->17): a staged change to a DEFINITIONAL artifact (.engine/schema/, .engine/processes/, .engine/workflows/) with NO co-committed doc update (CLAUDE.md / .engine/docs/ / any README) emits a warning. Reuses the process_change git-staged mechanism. WARNING-level (the D0102 promote-once-low-noise pattern, like decision-requirement-link): a definitional change MIGHT legitimately need no doc, so it NUDGES, never blocks — promotable to a hard gate once proven low-noise. SCOPE (deliberately NARROW to stay low-noise): tool `.rs` code + skills are EXCLUDED for now; widen once proven. #JustifiedBy this Decision.";
        :>> rationale = "ALTERNATIVES: (i) keep doc-sync a skill (status quo) — REJECTED: it's the vigilance the audit flagged; the discipline drifts precisely because nothing checks it. (ii) a HARD gate now — DEFERRED: doc-sync is heuristic (a mechanical check can't know a doc CLAIM is stale, only that a definitional change shipped with no doc touch), so a hard gate would false-block; warning-first earns trust (D0102 precedent) before it blocks. (iii) enforce broadly (all .rs = tool changes) — REJECTED for now as too noisy; start with the definitional trio (schema/process/workflow) where doc implications are near-certain. CHOSEN — narrow warning-level heuristic on the git-staged set: catches the real forgot-to-doc-sync case, never false-blocks, promotable. CRITERIA: converts-vigilance-to-control x low-noise x reversible/warning-first x reuses-existing-mechanism.";
        :>> consequences = "ACCEPTED by the human 2026-07-14 (the discrete d0113Accept TestResult, human-judged — not prose, D0058/D0106): they selected 'doc-sync guard' from the audit's convert-a-vigilance-gap options. doc-sync joins GUARD_NAMES as a second warning-only member (with decision-requirement-link); guard count 16->17, hard-blocking unchanged at 15. Governs the guard addition (D0070 keystone; carries #ProspectiveChange). Does NOT eliminate the doc-sync SKILL (still run for the actual fixing); the guard is the safety net that the skill was run. doc-sync CLAUDE.md §5 (17 guards). Promotion to hard is a later Decision once low-noise is shown. Remaining vigilance gaps (migration, parse-first) stay noted, not yet controlled."; }

    verification d0113Accept : Test { :>> id = "10a1b0c1-4113-42a3-bb5c-516273849713"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of D0113 (convert doc-sync from vigilance to a warning-level control guard) via selecting 'doc-sync guard' from the process-audit's convert-a-vigilance-gap-to-a-control options on 2026-07-14 — an explicit choice of this specific control, in direct response to the presented design (warning-level, definitional-trio scope)."; }
    part d0113AcceptR1 : TestResult { :>> id = "10a1b0c1-4113-42a3-cb5c-516273849713"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e0f1361"; :>> judgedAt = "2026-07-14"; :>> judgedBy = "wweatherholtz"; }
}
