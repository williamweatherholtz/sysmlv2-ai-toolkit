// =====================================================================
// D0103 — guarantee the "strong why": a hard guard that every Decision carries a SUBSTANTIVE context +
// rationale, not just the schema-present (possibly blank) fields. The human's requirement (2026-06-25):
// "when we make decisions ... there should be a strong indication as to why ... so that we can improve
// decisions in the future and reevaluate when circumstances change." The Decision schema already MANDATES
// context/decision/rationale/consequences (required Strings, ADR/ISO-42010 shape; D0058/D0061) + the
// kernel checks PRESENCE, but nothing checks they are NON-BLANK — a blank rationale would pass. This adds
// the enforcement. (NOTE: the same review reconsidered retiring `#Supersede` — REJECTED: it is the typed
// Decision->Need scope-cut edge required by sr02DecisionRecord + the tracking-template + §2.4; only
// Decision->Decision supersession uses the status field, D0070. #Supersede stays.)
// Accepted 2026-06-25 by wweatherholtz (chose 'Add non-empty-rationale guard').
// =====================================================================
package Decision0103 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0103 : Decision {
        :>> id = "03a1b0c1-4103-42a3-ab5c-516273849703";
        :>> title = "D0103: decision-rationale hard guard — every Decision must carry a substantive context + rationale (the why)";
        :>> createdAt = "2026-06-25";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human set a requirement while reviewing the supersession/CHANGE model: every decision (supersede/modify/whatever) must carry a STRONG indication of WHY, so decisions can be improved + reevaluated when circumstances change. The engine already supports this well — the Decision schema (work.sysml) mandates context (Nygard ADR forces/situation), decision (the choice), rationale (justification + alternatives + criteria, ISO 42010), consequences (effects) as required Strings (D0058 decision-analysis + D0061 mandatory-rationale), superseded decisions are PRESERVED (status=superseded, not deleted) as the judgment history, and suspicion (D0050/D0084) + critique (D0080) re-flag decisions when their basis drifts (the reevaluation trigger). GAP: the kernel enforces the fields are PRESENT, but no guard enforces they are NON-BLANK/substantive — a Decision authored with an empty or trivial rationale ('', 'n/a', 'tbd') would pass every gate. The 'strong why' is convention, not a guarantee. A scan of all 102 existing decisions shows every one already has context + rationale >= 20 chars, so a hard guard binds cleanly with zero flood.";
        :>> decision = "Add a HARD honest-state guard `decision-rationale` (GUARD_NAMES 13 -> 14): a Decision whose `context` OR `rationale` is blank/trivial (trimmed length < 20) is a VIOLATION (blocks the commit). A recorded decision without its substantive why is dishonest/ill-formed state — exactly the honesty class that hard-gates (D0098), and it guarantees the human's requirement structurally rather than by vigilance (D0047). The 20-char floor rejects empty/'n/a'/'tbd' while passing all 102 current decisions; `decision` + `consequences` stay schema-required (present) but are not length-gated here (the why = context + rationale is the load-bearing pair). The fix is the cure for its own class: the rationale that justifies a decision is now a checked, not assumed, fact.";
        :>> rationale = "ALTERNATIVES: (i) rely on the schema's required-field presence + the D0061 convention — REJECTED (presence != substance; a blank rationale passes; vigilance is not a control, D0047). (ii) warning-level guard — REJECTED (the human wants the why GUARANTEED, 'not accidentally ignorable'; and unlike the noisy decision-requirement-link heuristic, blank-rationale is a precise, false-positive-free signal, so hard is safe). (iii) charter-time scope to new decisions only — UNNECESSARY (all 102 existing pass; no grandfathering needed). (iv) CHOSEN — a hard guard on non-blank context + rationale. CRITERIA: honesty (a decision states its why) x precision (blank-check has no false positives, unlike prose heuristics) x no-flood (all existing pass) x D0047 (automated control). The supersede-retirement also considered this round was REJECTED on new evidence (sr02 requires #Supersede for Decision->Need cuts).";
        :>> consequences = "New hard guard `decision-rationale` in GUARD_NAMES (13 -> 14 forward guards; 13 hard-blocking + the decision-requirement-link warning). A Decision with blank/<20-char context or rationale now fails `keel guard` + the pre-commit. Passes today (all 102 decisions compliant). Downstream `keel init` inherits via `keel guard`. `#Supersede` is UNCHANGED (kept — it is the Decision->Need scope-cut edge per sr02/§2.4/tracking-template; only Decision->Decision uses status, D0070). doc-sync: CLAUDE.md §5 (14 forward guards: 13 hard + 1 warning; the decision-rationale guard). No schema change (the guard reads existing required fields), so no schema-freeze sign-off beyond this Decision; kernel-validate the new decision instance.";
    }

    verification d0103Accept : Test { :>> id = "03a1b0c1-4103-42a3-bb5c-516273849703"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of d0103 (a hard decision-rationale guard requiring substantive non-blank context + rationale on every Decision) via the explicit choice 'Add non-empty-rationale guard' on 2026-06-25, stating decisions must carry a strong why for future improvement + reevaluation. (The same selection's 'retire #Supersede' was withdrawn on evidence that #Supersede is the required Decision->Need scope-cut edge, sr02.)"; }
    part d0103AcceptR1 : TestResult { :>> id = "03a1b0c1-4103-42a3-cb5c-516273849703"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "c2f3b82"; :>> judgedAt = "2026-06-25"; :>> judgedBy = "wweatherholtz"; }
}
