// D0208 (status: the `status` field below is the truth) — redTeamScope: how hard the guards get attacked before we claim they hold
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0208Accept event on sign-off.
package Decision0208 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: Panel dossier docs/reviews/process-value-panel-2026-08-22.md (AI-safety lens rec 1, empiricist rec 3, formal-methods differential oracle); Greenblatt et al. AI Control: Improving Safety Despite Intentional Subversion (Redwood, the catch-rate methodology and the sandboxing requirement); ControlConf 2026 (control evals approaching load-bearing for real deployments); keel precedent: the panel process's own refute-with-tree-evidence and the scratch-git-repo test pattern (sprint 427 claim-ancestry) are the containment mechanics reused here.
    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0208 : Decision {
        :>> id = "3083175c-fb6b-4b95-a976-c391a7f786ee";
        :>> title = "redTeamScope: how hard the guards get attacked before we claim they hold";
        :>> createdAt = "2026-08-23";
        :>> createdBy = "claudeFable5";
        :>> status = DecisionStatus::accepted;
        :>> context = "The process-value panel (2026-08-22) named this the cheapest high-value gap: keel measures whether controls FIRE on honest work, never whether they HOLD under attack, so its strongest claim ('controls exist') falls short of the AI-control field's required claim ('controls were attacked and held'). dcControlEvaluation is chartered and needs the human's go-ahead per its own DoD. What still needs deciding is SCOPE - how adversarial the first run is - because that trades safety-of-the-run against strength-of-the-evidence, and it is the human's call, not an engineering fact.";
        :>> decision = "PROPOSED, three scopes costed - accepting means choosing ONE. OPTION A (scripted probes, no agent): a shell harness in a throwaway git worktree plants each known attack (hollow pass, fabricated delegated quote, weakened guard, backdated claim, unfilled-scaffold land) and records per-tier catch/miss as TestResults; every miss becomes an issue plus a control (D0047). COST: covers only attacks WE ALREADY IMAGINED - it proves the known guards fire on the known evasions, not that a creative adversary is stopped; cheap, fully deterministic, safe. OPTION B (adversarial agent, sandboxed): a subagent is instructed to LAND a dishonest change past the gates by any means in a throwaway worktree with no push credentials; catch rate is measured at each tier and its attempts logged. COST: finds evasions we did not script (the real value), but an agent told to defeat controls is exactly the behavior the controls exist to stop - it must be truly sandboxed (no network, no push, disposable clone) and its transcript reviewed, and even then it may surface a real bypass that must be embargoed until patched. OPTION C (both, staged): A first to prove the floor, then B against the hardened result. COST: most evidence, roughly double the effort, and B still carries A-plus-B's containment burden. DRAFTER RECOMMENDS C with a hard containment contract (disposable worktree, no credentials, network off, transcript recorded, any found bypass fixed-then-disclosed before the run's results are published) - the panel's whole point is that unattacked controls are unproven, and A alone cannot make the field's claim.";
        :>> rationale = "Detection without an attack is not deterrence for a strategic adversary (the institutional-economist lens); the control literature's numbers (Redwood: trusted monitoring caught 62 percent of backdoors) exist only because someone ran the attack. The scope question is real and irreducibly the human's: option B deliberately runs the exact behavior keel is built to prevent, contained - a supervision-risk trade, not a technical one, so it is presented rather than assumed.";
        :>> consequences = "OPTION A: one harness, deterministic, no new risk surface. OPTION B: a sandboxed adversarial subagent with a containment contract and a transcript-review step; possible embargoed-bypass finding. OPTION C: both, staged, with B's containment. EVERY OPTION: catch-rate table recorded as TestResults, misses become D0047 controls, dcControlEvaluation's DoD flips on completion. REJECTED-AS-WRITTEN: the controls stay unattacked and the panel's finding stands open.";
    }

    // acceptance event (D0121 review-queue sign-off; D0066/D0106 — human-judged, not fabricated)
    verification d0208Accept : Test { :>> id = "d3a83963-3951-428f-ba4b-e18c202cd8d2"; :>> method = VerificationMethod::confirmation; :>> procedureText = "OPTION A - their words, verbatim: 'A' (GitHub comment https://github.com/williamweatherholtz/sysmlv2-ai-toolkit/issues/3#issuecomment-5389445822, id IC_kwDOSwT6BM8AAAABQTxqvg, authenticated login williamweatherholtz)"; }
    part d0208AcceptR1 : TestResult { :>> id = "85431a27-c072-4d1d-b749-c72e2afb7761"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "dc5aea0"; :>> judgedAt = "2026-08-24"; :>> judgedBy = "wweatherholtz"; }
}
