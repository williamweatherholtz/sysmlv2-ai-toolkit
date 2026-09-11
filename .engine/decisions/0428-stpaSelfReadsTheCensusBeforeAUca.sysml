// D0428 (status: the `status` field below is the truth) — stpaSelfReadsTheCensusBeforeAUca: the stpa-self process gains step stpa2bCensus - the control census is read before any UCA is written, and a UCA's evidence class is computed from its edges
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0428Accept event on sign-off.
package Decision0428 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: keel-cli/src/guards.rs keystone_violations (line 1165) reads the process-change marker only from a decision file STAGED in the same commit, so the step D0424 accepted on 2026-09-10 cannot ride D0424 itself once D0424 is committed; the last stpa-self process edit (7c5d6e5) carried D0410 the same way, recorded proposed beside the edit. The census join was probed before the tree was read (D0388): a fixture UCA reaching an in-field Issue classes observed, one reaching no Issue classes code-read, one reaching a code-read Issue stays code-read - keel-cli/src/view/census.rs a_uca_reaching_an_in_field_issue_is_observed_and_one_reaching_none_is_code_read.
    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0428 : Decision {
        :>> id = "4ee33271-3df3-4820-a692-2ca76145f703";
        :>> title = "stpaSelfReadsTheCensusBeforeAUca: the stpa-self process gains step stpa2bCensus - the control census is read before any UCA is written, and a UCA's evidence class is computed from its edges";
        :>> createdAt = "2026-09-10";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0424 (accepted) decided that the stpa-self process reads the control census before it writes an UnsafeControlAction and that a control the census classes friction or hypothetical is surfaced as a removal candidate. This Decision is the clause that lands the step itself, in the form the process definition and the run record carry, so the process file changes under a Decision that names exactly what changed (D0070 keystone; D0303 one clause per Decision). NOT A FORK: the direction is D0424's and is the human's; this is its implementing step.";
        :>> decision = ".engine/processes/stpa-self.sysml gains ProcessStep stpa2bCensus between stpa2Structure and stpa3Ucas: run `keel show control-census`, read per control whose act it binds and its evidence class, and read its `ucaSummary`, which classes every recorded UnsafeControlAction `observed` when a dependency edge from the UCA or from a ControllerConstraint bound to it reaches an Issue whose incident happened (discoveredInField = true) and `code-read` otherwise, and lists the constraints standing on a friction or hypothetical control as REMOVAL CANDIDATES. stpa5's run record carries `EVIDENCE: <n> observed, <m> code-read` and `REMOVAL CANDIDATES: ...` copied from that lens at the same tree. The class is never written in a UCA's text; the edges decide it.";
        :>> rationale = "A UCA found by reading code is still worth recording - STPA's strength is reasoning - but issue426 showed what happens when a reasoned UCA becomes a control with no incident behind it: the refusal fell on the human. Computing the class from the edges means a run cannot claim an observed hazard without pointing at the Issue where it happened, and a run that adds only code-read UCAs says so in its own record. Reading the census first puts the human's question of 2026-09-10 - has any agent actually done this - in front of the analyst as a computed row before a constraint is written.";
        :>> consequences = "Process-change (D0337): proposed until the human's word; the step text is on disk with this Decision beside it, as D0410 was. stpa-self run 3 is the first run under the step. dcUcaCarriesItsEvidenceClass delivers the lens half (`ucaSummary` and `ucas` in keel show control-census) and this step. What accepting binds: every future stpa-self run record reports its evidence counts and candidates. What it forecloses: nothing - a code-read UCA is still written.";
    }
    #DerivedFrom dependency from d0428 to st115;
    #DependsOn dependency from d0428 to d0424;

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0428Accept : Test { :>> id = "ef985d72-97c1-4100-83e4-00413ee9d269"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Selected on the decision brief copy-out (fifteenth publish), pasted into chat 2026-09-11: the set of 33 HELD process-change ratifications accepted with one word, this record being one member (D0406 named it in the table). No note added. - their words, verbatim: “Accept the set: each of the 33 rules named in the table is recorded accepted on its own, quoting these words (recommended)” WARN: read-back - the quoted words do not name d0428 (D0201 B); recorded as given (D0423)."; }
    part d0428AcceptR1 : TestResult { :>> id = "dbeff1f7-234c-48a3-ac40-43cbe95f0711"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3e6c518"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
