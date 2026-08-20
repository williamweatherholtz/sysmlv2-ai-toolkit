// =====================================================================
// D0052 — backlog-prioritization: the backlog is an ORDERED list (declaration
// order = priority); orient/whats-next rank the ready frontier by it; the AI
// auto-follows the top ready item that needs no human sign-off — it never asks
// "which of these should I do?". Reorder the backlog to reprioritize.
// Accepted 2026-06-16 by wweatherholtz ("you should automatically be deciding
// which items to pursue next ... is there no backlog prioritization? ... we
// should be following that").
// =====================================================================
package Decision0052 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0052 : Decision {
        :>> id = "2d3e4f50-6172-423a-376f-1324a5b6c7e3";
        :>> title = "D0052: backlog is an ordered list (declaration order = priority); AI auto-follows the ranked frontier";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "orient/whats-next returned the ready frontier sorted ALPHABETICALLY, so there was no computed 'next' — and the AI fell back to asking the human which ready item to do. That is a process gap: a backlog should be PRIORITIZED and the AI should just follow it. User: 'you should automatically be deciding which items to pursue next. is there no backlog prioritization? if so, we should be following that.'";
        :>> decision = "Adopted agile product-backlog ordering: the order in which `action`s are DECLARED in the backlog IS their priority (top = highest); orient.rs ranks the ready frontier by declaration order (added TaskData.order; ready sorted by it, not alphabetically), query.py aligned (insertion order); the AI AUTO-FOLLOWS the top ready item and does NOT ask the human to choose among ready items. Reprioritizing = reordering the backlog.";
        :>> rationale = "orient/whats-next returned the ready frontier sorted ALPHABETICALLY, so there was no computed 'next' and the AI fell back to asking the human which ready item to do — a process gap; a prioritized backlog should just be followed. Two legitimate pauses remain (not 'asking which'): (1) the top item requires human sign-off — a frozen schema/core change (§2.5/§4) or a direction Decision (D0051) — then surface it for sign-off and meanwhile proceed with the top item that needs none; (2) the frontier is empty. User: 'you should automatically be deciding which items to pursue next. is there no backlog prioritization? if so, we should be following that.'";
        :>> consequences = "whats-next/orient ready is priority-ordered (top = next). The AI selects the next item itself. Items that change frozen schema/core or are direction Decisions still need human sign-off before execution (§2.5/§4/D0051) — that is a content gate, not a 'which item' question; the AI proceeds with the highest-priority sign-off-free item in the meantime. Backlog declaration order is now semantically significant — keep it curated; reorder to reprioritize.";
    }

    verification d0052Accept : Test { :>> id = "089cd791-3f9a-4ecd-a53c-e2874def1ca5"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0052 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0052AcceptR1 : TestResult { :>> id = "1a2f4616-1aab-4c7b-bed6-39d8589583db"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
