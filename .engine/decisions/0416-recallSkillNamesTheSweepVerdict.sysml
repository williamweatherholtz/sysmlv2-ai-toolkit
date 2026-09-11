// D0416 (status: the `status` field below is the truth) — recallSkillNamesTheSweepVerdict: the knowledge-graph-memory skill's step-7 row tells a project sweeping a ranker knob to read the sweep's verdict line, not the per-value table
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0416Accept event on sign-off.
package Decision0416 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0416 : Decision {
        :>> id = "7c5efd1e-1c84-40b6-8562-fcacafe81b00";
        :>> title = "recallSkillNamesTheSweepVerdict: the knowledge-graph-memory skill's step-7 row tells a project sweeping a ranker knob to read the sweep's verdict line, not the per-value table";
        :>> createdAt = "2026-09-09";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0366 made the step-7 row of the knowledge-graph-memory skill tell an adopting project to measure on both benchmark arms. D0415 gave recall_bench.py a --sweep whose last line states whether the set could choose. The row is the surface an adopting project reads when it changes the ranker, and without this sentence it would learn about both arms and still read a per-value table of equal numbers as a verdict - the issue405 reading, which the verdict line exists to prevent.";
        :>> decision = "NOT A FORK: one sentence is added to one row. The step-7 row says: sweep a knob with --sweep KNOB=v1,v2,... and read the last line, which is derived from the numbers - DISCRIMINATED with the spread, or COULD NOT CHOOSE naming what never moved and whether the set held any case the knob touched at all - and cites D0415 and issue405 so the reader can follow the measurement. The row's existing guidance from D0366 is unchanged.";
        :>> rationale = "The skill is where the finding reaches a project that is not this one; a verdict line nobody is told to read is a per-value table with one more row. One sentence rather than a section, for the same reason D0366 gave: the failure mode is specific and the remedy is a flag.";
        :>> consequences = "The skill file changes, so this Decision is co-committed with it under D0209 clause 2 and stays PROPOSED - a process-change marker is outside standing consent (D0337). The change is in the tree because the lock is on the commit, not the edit; keel sync-claude regenerates the .claude copy. D0415 stands on its own: the sweep and its verdict are in force whether or not this row is accepted, and if this Decision is refused the row reverts to D0366's text.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0416Accept : Test { :>> id = "d91d4951-6771-4fce-82d2-8bc9b0545f11"; :>> method = VerificationMethod::confirmation; :>> procedureText = "Selected on the decision brief copy-out (fifteenth publish), pasted into chat 2026-09-11: the set of 33 HELD process-change ratifications accepted with one word, this record being one member (D0406 named it in the table). No note added. - their words, verbatim: “Accept the set: each of the 33 rules named in the table is recorded accepted on its own, quoting these words (recommended)” WARN: read-back - the quoted words do not name d0416 (D0201 B); recorded as given (D0423)."; }
    part d0416AcceptR1 : TestResult { :>> id = "dc2b911a-e728-42c2-a302-4fabebf77673"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "3e6c518"; :>> judgedAt = "2026-09-11"; :>> judgedBy = "wweatherholtz"; :>> createdBy = "claudeOpus5"; }
}
