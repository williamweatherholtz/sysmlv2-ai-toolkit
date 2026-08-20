// =====================================================================
// D0121 (PROPOSED — NOT YET ACCEPTED) — the HUMAN REVIEW QUEUE + acceptance surface in keel serve.
// A computed review-queue view lists user-gated items (proposed Decisions + pending confirmation
// gates, ceremony-filtered); keel serve renders it at a URL (a "Review" tab) and RECORDS the human's
// acceptance as fact via the write API. Realises the D0093 dual-surface human-oversight loop.
// D0106-compliant: the human's UI action + note IS the attestation (judgedBy = a Person), never
// AI-fabricated. Human-directed 2026-07-15 ("launch keel serve, give me a URL... my acceptance
// recorded as fact... then prompt me to resume"). Acceptance = sitting review.
// =====================================================================
package Decision0121 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    part d0121 : Decision {
        :>> id = "10a1b0c1-4121-42a3-ab5c-516273849721";
        :>> title = "D0121: human review queue + UI acceptance surface in keel serve (records confirmations as fact, D0106-compliant)";
        :>> createdAt = "2026-07-15";
        :>> createdBy = "claudeOpus";
        :>> status = DecisionStatus::accepted;
        :>> context = "The human asked (2026-07-15) for a review loop: when the AI needs confirmation, temporarily launch keel serve, give a URL to observe OUTSTANDING process tasks including verification — filtered to USER-GATED items — so the human can quickly review + give feedback via the UI; the input + acceptance is RECORDED AS FACT via keel serve; then the human prompts the AI to resume. keel serve already has the write substrate (D0093/D0094: /api/testresult, /api/disposition, /api/decision) but no view of pending user-gated items, no UI to observe them, and no way to accept a proposed Decision.";
        :>> decision = "Add a HUMAN REVIEW QUEUE to keel serve. (1) COMPUTE a review-queue view (view::review_queue_json): user-gated items = proposed Decisions (status=proposed) + pending confirmation gates (method=confirmation Tests with no passing result); each gate is tagged ceremony=true|false so legacy pre-D0049/D0051 sprint DoD/retro confirmations (autonomous now) are filtered from the ACTIONABLE default (heuristic by structural name suffix). Exposed at GET /api/review-queue. (2) RENDER it in a 'Review' console tab: list the actionable items with their context (a Decision's context/decision/rationale/consequences; a gate's procedureText), a feedback-note field, and an Accept control; a toggle reveals ceremony/all. (3) RECORD acceptance as fact via the write API: accept a pending confirmation gate via append_gate_result (POST /api/gate-result); accept a proposed Decision via a NEW write op accept_decision (flip status proposed->accepted + append a dNNNNAccept confirmation verification + a passing TestResult) via POST /api/decision/accept. In BOTH, judgedBy = a human Person and the note is the attestation — D0106/confirmation-authenticity: the human's explicit UI action IS the sign-off, never inferred or AI-fabricated; nothing auto-commits (the human/AI commits after).";
        :>> rationale = "ALTERNATIVES: (i) keep confirmations to the CLI / hand-editing — REJECTED: high friction, and the human wants an ergonomic oversight lens (D0093: HTML is the human's surface). (ii) let the AI record acceptance on the human's behalf from a chat 'ok' — REJECTED: violates D0106/confirmation-authenticity (acceptance must be a human-judged event, never AI-fabricated); the whole point is that the UI action IS the attestation. (iii) show ALL pending confirmation gates unfiltered — REJECTED as the default: legacy ceremony/DoD confirmations are noise (autonomous now), burying the few genuine user gates; keep them behind an 'all' toggle for honesty. CHOSEN: a computed queue + a serve Review tab + write-back that turns the human's click+note into a recorded TestResult / decision acceptance. This closes the D0093 loop (the human orients + accepts in the UI; the AI resumes from recorded fact) and reuses the existing write API (append_gate_result) plus one new op (accept_decision).";
        :>> consequences = "Recorded PROPOSED; acceptance = sitting review — and, fittingly, this very queue can carry that acceptance once built (dogfood). Built as sprints 226 (view + /api/review-queue), 227 (write-backs: accept_decision + /api/decision/accept + /api/gate-result), 228 (the Review console tab). The AI's confirmation protocol becomes: launch keel serve, hand the human /#review (or the endpoint), pause; the human accepts in the UI (recorded as fact); the human prompts resume. The ceremony filter is a heuristic (name suffix) — promotable to a structural signal later. NOTE: accepting a Decision flips status in .engine/decisions/<file> and appends the accept event; it does NOT auto-commit."; }

    #DependsOn dependency from d0121 to d0093;

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0121Accept : Test { :>> id = "784726f5-f06e-4584-ab24-59856cfffd4e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "accepted"; }
    part d0121AcceptR1 : TestResult { :>> id = "e4a94e23-f829-46c6-bca1-1b3c94221c33"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "db485b5"; :>> judgedAt = "2026-07-15"; :>> judgedBy = "wweatherholtz"; }
}
