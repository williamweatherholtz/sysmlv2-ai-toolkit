// =====================================================================
// D0104 — sharpen the decision-requirement-link guard (D0102) from "names a requirement at all" to
// "GOVERNS a requirement": flag a (Decision, requirement) prose mention only when a governance verb
// (amend/supersede/descope/revise/cancel/replace/retire/rescope/moot) sits within a window of the
// requirement name AND no OTHER decision id is cited in that window. Reviewing the 6 D0102 warnings, 1
// was genuine governance (d0083 amended n11/sr11 — now linked) and 4 were CONTEXTUAL: a measured-gap
// example (d0082), and three that describe ANOTHER decision's action (d0084/d0088 cite D0083; d0102 cites
// D0100). The refinement clears the contextual false-positives + keeps genuine governance, moving the
// guard toward hard-gate-able precision (D0102's stated promotion path). Accepted 2026-06-29 by wweatherholtz.
// =====================================================================
package Decision0104 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0104 : Decision {
        :>> id = "04a1b0c1-4104-42a3-ab5c-516273849704";
        :>> title = "D0104: decision-requirement-link precision — flag GOVERNANCE mentions, not every mention";
        :>> createdAt = "2026-06-29";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0102's decision-requirement-link guard (warning-level) flagged 6 prose-only decision->requirement references for human review. The review found the guard's exact-name heuristic is too broad: only 1 of 6 was a GOVERNANCE relationship (d0083 amended n11FastStart + sr11FastStart statements — now typed-linked), while 4 were CONTEXTUAL mentions that should NOT carry a governance edge: d0082 names sr11FastStart only as a measured-GAP example; d0084 + d0088 describe ANOTHER decision's action ('amending sr11's statement (D0083)'; '(D0083): sr11FastStart HAD a threshold'); d0102 names sr19 as the issue052 case study, citing D0100's descope. Typed-linking these would FALSELY assert governance. The warning-level design anticipated this residual (promote to hard once low-noise); this is the precision step.";
        :>> decision = "Refine view::decision_requirement_prose_links (the guard's source) to flag a (Decision D, requirement R) pair only when: (1) D names R by exact identifier, AND (2) D has NO typed edge to R, AND (3) a GOVERNANCE VERB (amend/supersede/descope/revise/cancel/replace/retire/rescope/moot, matched as a token substring so inflections count) occurs within a ~60-char window of an occurrence of R, AND (4) NO foreign decision id (`d`/`D` + 4 digits, other than D's own number) occurs in that window. (3) drops pure examples (no governance verb); (4) drops descriptions of another decision's governance (the verb is attributed to a cited decision, which should be linked THERE). The guard stays WARNING-level (D0102) but is now precise enough to be promotable to a hard gate.";
        :>> rationale = "ALTERNATIVES: (i) accept all 4 as warning-level residual (no change) — REJECTED (the human chose precision; perpetual contextual noise erodes the signal + blocks ever hard-gating). (ii) verb-near-only (no foreign-id check) — REJECTED (d0084/d0102 contain a governance verb describing ANOTHER decision's action, so they'd still false-positive). (iii) hard-gate the broad guard — REJECTED (would block on contextual mentions). (iv) CHOSEN — governance-verb-near AND no-foreign-decision-id-near: clears all 4 contextual, keeps genuine (d0083-style 'this decision amends R'), and is testable. CRITERIA: precision (no contextual false-positives) x keeps-genuine (real governance still flagged) x testability (token + window, deterministic) x promotion-path (toward hard, D0102).";
        :>> consequences = "view::decision_requirement_prose_links gains the governance-verb + no-foreign-id window filter; guards::decision_requirement_link is unchanged in shape (still warning-level in GUARD_NAMES). The 4 contextual warnings clear (decision-requirement-link -> 0 warnings); a future decision that GOVERNS a requirement in prose without a typed edge still warns. No new guard, no count change (still 14 forward guards: 13 hard + 1 warning). doc-sync: CLAUDE.md (the guard's description gains 'governance mention' precision). Refines D0102 (same guard, sharper signal); kernel-validate the new decision instance.";
    }

    verification d0104Accept : Test { :>> id = "04a1b0c1-4104-42a3-bb5c-516273849704"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of d0104 (sharpen decision-requirement-link to flag governance mentions — governance-verb-near AND no-foreign-decision-id-near — not every mention) via the explicit choice 'Refine the guard for precision' on 2026-06-29, after reviewing the 6 D0102 warnings (1 genuine, 4 contextual)."; }
    part d0104AcceptR1 : TestResult { :>> id = "04a1b0c1-4104-42a3-cb5c-516273849704"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "1c581fb"; :>> judgedAt = "2026-06-29"; :>> judgedBy = "wweatherholtz"; }
}
