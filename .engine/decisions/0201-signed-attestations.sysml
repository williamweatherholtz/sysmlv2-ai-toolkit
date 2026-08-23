// D0201 (status: the `status` field below is the truth) — D0201: authenticate human attestations cryptographically - the panel's most convergent import
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0201Accept event on sign-off.
package Decision0201 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0201 : Decision {
        :>> id = "553d2b58-ca67-4c6a-80e2-23eb94e5d0e5";
        :>> title = "D0201: authenticate human attestations cryptographically - the panel's most convergent import";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The process-value panel's most convergent recommendation (AI-safety, SE-research, formal-methods, economist lenses independently): keel's evidence chain is a well-designed but UNSIGNED provenance scheme - roughly SLSA L1 (structured, checkable) with L0 tamper-resistance, because any actor who can write files can author a plausible TestResult with a plausible judgedBy. The D0192 quote-substance check verifies format, not origin, and states so ('a fabricated quote defeats the check'); EHZ3 records the channel as name-spoofable by design; D0047 itself declares human vigilance is not a control, yet human queue-reading is currently the trust anchor for every human-judgment record. Authentication, not text-checking, is the literature's consensus close for spoofable channels - a conclusion D0192's own context paragraph already conceded.";
        :>> decision = "PROPOSED, three options costed - accepting means choosing ONE. OPTION A (git-native signing): the human signs their attestation-bearing commits with gitsign/SSH key; a guard verifies that every commit introducing a method=confirmation result judged by a Person carries that Person's signature. COST: the human must commit (or countersign) their own attestations - the chat-delegation workflow gains a periodic countersigning step (e.g. at each sitting, one signed commit ratifies the batch since the last). OPTION B (deck-tap signature binding): the deck/inbox channel gains an HMAC per registered human (key held outside the repo, provisioned once); a tap's row carries the MAC over (item, verdict, timestamp); the recording write stores it and a guard verifies MACs on post-cutoff human-judgment records. Chat-delegated records remain quote-based but must CITE a MACed gesture within N days (the read-back ratification). COST: key management on the human's devices; the serve/inbox path grows a crypto dependency. OPTION C (defer with a named revisit): record that signing is the accepted end-state, revisit at the first second-human enrollment or first external audit need - the single-supervisor threat model (their own queue, their own repo) may not justify key management today. COST: the fabrication residual stays open exactly as EHZ3 records it. DRAFTER RECOMMENDS B: it authenticates the channel the human already uses (taps), keeps chat delegation alive via ratification, and is the smallest step that converts quotes from claims into evidence.";
        :>> rationale = "Every panel lens that touched the trust anchor reached the same conclusion by a different route: the audit trail is excellent and its root is unauthenticated. The three options span the honest range - pay the signing cost in git, pay it in the deck channel, or explicitly accept the residual with a revisit trigger. Which cost fits this human's workflow is a supervision preference, not an engineering fact - the D0192 precedent.";
        :>> consequences = "OPTION A: guard + signing setup, chat workflow gains countersigning. OPTION B: HMAC provisioning, inbox row MAC, verification guard, read-back ratification surface; EHZ3's residual closes for gesture-recorded judgments and narrows for chat-delegated ones. OPTION C: one revisit condition recorded beside EHZ3; nothing else changes. EITHER A or B moves keel from SLSA-L1-equivalent provenance toward L2 (statement plus signature); the panel dossier is the citation trail.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0201Accept : Test { :>> id = "fa156037-57f2-4c6c-b95a-38de7fad277e"; :>> method = VerificationMethod::confirmation; :>> procedureText = "OPTION B chosen (deck-tap signature binding: device HMAC on taps, read-back ratification for chat approvals). Deck taps: a contradictory cluster at 13:05-13:06Z on the pre-fix rendering, then a SINGLE clean OPTION B tap at 2026-08-23T19:54:34Z on the fixed single-highlight deck (HEAD d6c3417) - the latest, uncontradicted gesture binds. uid 553d2b58."; }
    part d0201AcceptR1 : TestResult { :>> id = "992fa944-a76d-4a2a-888d-b37236dcb554"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6b3f6fd"; :>> judgedAt = "2026-08-23"; :>> judgedBy = "wweatherholtz"; }
}
