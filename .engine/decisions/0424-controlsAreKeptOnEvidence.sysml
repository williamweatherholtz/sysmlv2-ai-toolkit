// D0424 (PROPOSED — NOT YET ACCEPTED) — controlsAreKeptOnEvidence: the stpa-self process reads the control census before it writes a UCA, and a control the census classes friction or hypothetical is surfaced as a removal candidate
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0424Accept event on sign-off.
package Decision0424 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0424 : Decision {
        :>> id = "88a2ed4e-f23b-4e60-80fa-6b04a117282f";
        :>> title = "controlsAreKeptOnEvidence: the stpa-self process reads the control census before it writes a UCA, and a control the census classes friction or hypothetical is surfaced as a removal candidate";
        :>> createdAt = "2026-09-10";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "The engine's controls were added one incident at a time (D0047) and never audited for whom they bind or whether the act they refuse has ever been observed. Asked on 2026-09-10 (st115) whether agents had attempted to accept Decisions under the human's authority, the record could not answer (issue445, issue446); what it could show is that every documented subversion is the agent's (issue267 scripts stamping verdicts, issue266 receipts, issue256/315/322 shell output in records, issue400 wrong first answers, issue434 green said over red CI) and every documented refusal of a person is of a correct act (issue397). The human's direction: 'build evidence around the ways processes are subverted today ... integrate into STPA and spring out implementation asking with upstream needs, use cases, and requirements.'";
        :>> decision = "NOT A FORK: this is the process the human asked for; the alternatives are what to do with its output, decided one Decision per control. The stpa-self process gains a step between stpa2 and stpa3: read keel show control-census (dcControlCensusByActorKind) - per control, whose act it binds (human, ai, either), its blocks by actor kind from the ledger (dcRefusalIsALedgerFact), and its motivating Issues classed observed or code-read - and write every UnsafeControlAction with its evidence class computed from that (dcUcaCarriesItsEvidenceClass). A control the census classes friction (its only blocks are of a human act, no observed incident) or hypothetical (code-read only, zero blocks) is listed in the run record as a REMOVAL CANDIDATE and reaches the human through decision-surfacing as one proposed Decision per control, each carrying the census row. A control classed evidenced is never a candidate on this path.";
        :>> rationale = "STPA finds hazards by reasoning, which is its strength and why it produced issue426: a UCA that was real in the code and never in the record, and a refusal that fell on the human instead. Reading the census first does not stop the reasoning - a hypothetical UCA is still written, labelled - it stops a hypothetical UCA from becoming a control that binds a person with no incident behind it, and it gives the human the question they asked as a computed row rather than my recollection. The direction that controls bind the AI and not the human is theirs (st115, and before it the human-is-not-a-process-component principle behind D0204); this makes it a step with a lens behind it rather than a memory. What would change it: a census that classes a human-binding control evidenced because an AI was blocked impersonating the human - that control stays, and the process is working.";
        :>> consequences = ".engine/processes/stpa-self.sysml and .claude/skills/stpa-self/SKILL.md gain the step and the candidate listing; nControlBindsTheSubvertingActor and nSubversionEvidenceGroundsTheAsks (.tracking/business/evidenced-controls.sysml, derived from us105/us106) are the Needs it satisfies; the requirements and use case follow in the Architecture workflow once the census exists. Order of work: dcRefusalIsALedgerFact (nothing can be counted until a refusal is a fact), dcControlCensusByActorKind, dcUcaCarriesItsEvidenceClass, then stpa-self run 3. Process-change (D0337): proposed until the human's word. What accepting binds: every future control-adding Decision names its UCA's evidence class. What it forecloses: nothing - a control can still be added on a code-read UCA, labelled as such.";
    }
    #DerivedFrom dependency from d0424 to us106;
}
