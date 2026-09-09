// D0394 (PROPOSED — NOT YET ACCEPTED) — scriptProbesRunInCi: every python check under scripts/ that ships its own known cases has them run by CI as a gate, discovered from the tree rather than listed
// Recorded via `keel record decision` (D0105 RMWX axis; issue054). Acceptance is a separate explicit
// human gate (method=confirmation, D0106): flip status + add the d0394Accept event on sign-off.
package Decision0394 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    // RESEARCH: On 2026-09-08 three python checks under scripts/ carry known-answer cases - check_templates.py --self-test (the brief contract, D0377), textpatch.py --probe (D0386) and artefact.py --probe (D0387) - and scripts/probes/issue400_reproduction.py is a fourth, a probe by construction. .github/workflows/ci.yml runs no python step: grep python ci.yml finds nothing. Three consecutive retros (sprints 612, 613, 614) named the same gap, which is what made dcScriptProbesRunInCi High by recurrence (D0311) and moved it to position 5 of the frontier.
    // #ProspectiveChange (D0070) - this Decision governs a process/schema change; the co-committed
    // change rides the process-change guard's hard lock.
    #ProspectiveChange part d0394 : Decision {
        :>> id = "1c99bbb6-4cda-4efb-9298-2bbb1c46e4ae";
        :>> title = "scriptProbesRunInCi: every python check under scripts/ that ships its own known cases has them run by CI as a gate, discovered from the tree rather than listed";
        :>> createdAt = "2026-09-08";
        :>> createdBy = "claudeOpus5";
        :>> status = DecisionStatus::proposed;
        :>> context = "dcScriptProbesRunInCi, a retro item: a check whose correctness is verified only by the hand that happens to run its probe is found broken by the next session, not the next push. The item asks for one CI step that discovers every script declaring a probe, runs each as a gate, and picks up a new one without editing the workflow. It changes a CI workflow, which is enforcement surface under the keystone lock (D0209 clause 2), so it waits for the human under D0337.";
        :>> decision = "NOT A FORK: the item states the shape and the only judgment left is whether the step gates or reports, and a known-answer check of a control that fails is a broken control, which is what a gate is for. ci.yml gains one step after the keel gates: it lists every .py under scripts/ whose text declares a --probe or --self-test entry point, runs each with that flag against the built binary, and fails the build on any non-zero exit, printing the script and its output; a tree with no such script passes with a line saying so. Discovery is by the flag string in the file, so a fourth probe joins by existing.";
        :>> rationale = "The probes exist to catch a regression in a control before the control is trusted again; a probe that runs only by hand catches it only when someone remembers, which is the failure mode this project keeps recording. Discovery from the tree rather than a list is the same rule the guards apply to instruments (D0361): a list is a second home that drifts. Gating rather than reporting because each probe is a known-answer check with no interpretation left - a red one is a fact.";
        :>> consequences = "ci.yml carries the step; issue400_reproduction.py gains a --probe entry point and resolves the binary as keel or keel.exe so it runs on the CI host; the retro finding named in sprints 612 through 614 is closed by the same commit's DoD result. A script that wants a probe declares one and is run; one that declares none is not accused - the not-an-instrument line already argues its exclusion from measurement, and this step reads only the flag.";
    }
}
