// D0202 (PROPOSED — NOT YET ACCEPTED) — D0202: the parser-coverage census gains a committed baseline that only ever tightens
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0202Accept event on sign-off.
package Decision0202 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0202 : Decision {
        :>> id = "2ef83bc1-dad6-4624-914d-2e48b8c5b192";
        :>> title = "D0202: the parser-coverage census gains a committed baseline that only ever tightens";
        :>> createdAt = "2026-08-22";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::proposed;
        :>> context = "issue231 (process-value panel, formal-methods lens): the parser is the trusted computing base of every guard and view - a statement class it skips is invisible to the whole enforcement stack (incident lineage issue102/109/119 plus the bare-dependency skip) - and parser-coverage was a WARNING whose count could grow silently, which is a control people learn to scroll past.";
        :>> decision = "A committed baseline contract (.engine/contracts/parser-coverage-baseline.toml, currently 7 - the known legacy successions) turns the census into a RATCHET: the live count EXCEEDING baseline is a guard VIOLATION (new invisibility goes red); any nonzero count still warns (legacy invisibility stays visible); a count below baseline warns to ratchet the baseline DOWN; a malformed contract fails loud; an absent contract is stated non-adoption (D0136). Raising the baseline is a visible one-line diff made in the same commit as its cause, with the reason in the contract - never silent growth. Companion prevention: keel-parser's proptest suite generates random compositions of every supported statement form and asserts zero skips, so a regression on the SUPPORTED side is caught by generation.";
        :>> rationale = "Exceed-goes-red / presence-warns / shrink-prompts-tightening: each behavior is chosen against a named prior failure - the D0132 all-or-nothing bypass (never gate on mere presence), the scroll-past warning class (never leave growth ungated), and silent bound decay (never let a gained margin evaporate). The panel's ShardStore citation is the pattern: lightweight formal methods at the TCB boundary, not heavyweight proof.";
        :>> consequences = "One contract file, ~25 guard lines, one proptest; the guard was negative-probed red-then-green with a planted connect statement before landing. Raising the baseline now requires a co-committed reason; teaching the parser a construct earns a baseline decrease the guard itself prompts for. issue231 resolves.";
    }
}
