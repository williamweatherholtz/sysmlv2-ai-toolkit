// D0259 (PROPOSED — NOT YET ACCEPTED) — Library stewardship becomes a process: publish deliberately, consume knowingly, update explicitly, unit-ify before folklore
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0259Accept event on sign-off.
package Decision0259 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0259 : Decision {
        :>> id = "5a120d81-27cc-4ff0-9577-8a58a9c8296f";
        :>> title = "Library stewardship becomes a process: publish deliberately, consume knowingly, update explicitly, unit-ify before folklore";
        :>> createdAt = "2026-08-29";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "The library's mechanics landed and were probe-proven (D0250-D0252), but the discipline around them - when to publish, what a staleness statement obliges, how a project takes an update, when a skill becomes a unit - existed only in conversation, which is the issue245 shape and the D0040 leak: publishing recurred four times on its first day with its method living in chat history. The human selected define-it-now on the program-summary page (st036).";
        :>> decision = "The libraryStewardship process (four steps: lsPublish, lsConsume, lsUpdate, lsStock) with its deploying skill, registered and shipped with the surface. Publish only from landed trees, one commit per changed unit, push as the deliberate fleet-visible act, no-op refusals read as information; sync any time but staleness statements and divergence refusals are obligations; a project updates via import --update plus its gate, committed under the keystone; any reusable skill becomes a UNIT (process + skill + registry + extras) before it spreads as folklore. Published to the library itself, alongside the units it governs.";
        :>> rationale = "The asymmetric shape (loud publish, silent consume) was already D0250's design; this process is where a NEW contributor learns it without reading four decisions. Making the process itself a library unit is the recursion that matters: the discipline for using the library travels IN the library, so a second machine gets the rules with the first sync.";
        :>> consequences = "The catalogue gains library-stewardship (guard-less, transferable). The library gains it as a unit. The 'not governed' column from the program summary - when to publish, when a project pulls, what the no-op obliges - is now governed. This Decision rides the keystone lock for the process-definition addition.";
    }
}
