// D0445 (status: the `status` field below is the truth) — claudeMdIsRulesPlusPointers: CLAUDE.md carries a rule and the id of the Decision that holds its history, never the history itself, and a guard bounds its size
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0445Accept event on sign-off.
package Decision0445 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: git log over CLAUDE.md since 2026-08-14 - 12 KB to 53 KB, 1.5 KB/day; every added paragraph names a Decision or Issue that already carries the same words (D0105). The 2026-09-11 rewrite to 13 KB dropped no rule: each kept its Decision id.
    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0445 : Decision {
        :>> id = "b9b40daf-adb5-4214-87c8-0e649aca5c83";
        :>> title = "claudeMdIsRulesPlusPointers: CLAUDE.md carries a rule and the id of the Decision that holds its history, never the history itself, and a guard bounds its size";
        :>> createdAt = "2026-09-11";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "CLAUDE.md is injected into every turn. From 2026-08-14 to 2026-09-11 it grew 4.3x because each correction appended its history - the quote, the date, the issue numbers, the measured values - beside the rule it changed. Those histories already live in the Decisions and Issues the rule names (D0105: one canonical home per fact), so the growth was duplication paid on every turn. The human asked on 2026-09-11 to caveman it and tighten the docs (st118).";
        :>> decision = "CLAUDE.md states each rule as a terse imperative followed by the id of the Decision or Issue that holds its rationale and history, and nothing else: no verbatim quote, no dated narration, no measured value that a Decision carries. Its size is bounded by a byte budget declared in .engine/contracts/doc-budget.toml and enforced by guard claude-md-bounded (dcClaudeMdIsBounded), which also fails a rule line carrying a long quoted span or a dated parenthetical. A rule that needs more than a sentence gets a Decision, and the sentence points at it.";
        :>> rationale = "The response contract already left this file for the system prompt because context injected as ordinary text was demonstrably ignored (D0130); a 53 KB context is the same failure by volume. A pointer costs one id per turn and loses nothing, because `keel why d0424` and the decision file answer the history on demand. A budget the guard enforces is the only control that survives the next four weeks of corrections; a reminder to keep it short is the manual vigilance D0047 forbids.";
        :>> consequences = "Process-change (D0337): a new guard and a new contract file, so this Decision waits for the human's word and the guard is built under dcClaudeMdIsBounded after acceptance. Doc-sync: usage.md and the READMEs point at the sections that survive (1 invariants, 3 route, 4 rules, 5 validate). A future correction lands as: Issue, control, one sentence in CLAUDE.md naming the Decision. NOT A FORK: the human asked for the compression; the guard is the D0047 control on it.";
    }
    #DerivedFrom dependency from d0445 to st118;

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0445Accept : Test { :>> id = "860ba5bb-6887-45d1-8fc8-1927ca6bc953"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Selected on the decision brief copy-out (fifteenth publish), pasted into chat 2026-09-11: the set of 33 HELD process-change ratifications accepted with one word, this record being one member (D0406 named it in the table). No note added. - their words, verbatim: “Accept the set: each of the 33 rules named in the table is recorded accepted on its own, quoting these words (recommended)” WARN: read-back - the quoted words do not name d0445 (D0201 B); recorded as given (D0423)."; }
    part d0445AcceptR1 : TestResult { :>> id = "5ce84282-a0cf-4c4e-a457-5f97ce3ae6b6"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3e6c518"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
