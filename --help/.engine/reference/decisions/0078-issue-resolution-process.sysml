// =====================================================================
// D0078 — activate the Issue Resolution process + deploying skill (D0077 IRL-e; D0059).
// Codifies the loop IRL-a..d built: record -> triage (#Resolves to a resolving action OR a
// mooting Decision) -> resolve (computed) -> decision-moots-item (record #Resolves from the
// Decision for an Issue; supersede for a Need/Requirement). Issue-scoped (human-accepted scope).
// #ProspectiveChange: introduces a process-def (keystone co-commit, D0070). Accepted 2026-06-20.
// =====================================================================
package Decision0078 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;
    private import EngineRelationships::*;

    #ProspectiveChange part d0078 : Decision {
        :>> id = "63940516-2c7d-4e8f-a078-516273849668";
        :>> title = "D0078: activate the issue-resolution Process + deploying skill (IRL-e; D0077/D0059); issue-scoped decision-moots-item via #Resolves, supersede for needs";
        :>> createdAt = "2026-06-20";
        :>> createdBy = "wweatherholtz";
        :>> status = DecisionStatus::accepted;
        :>> context = "D0077 established the #Resolves edge + COMPUTED issue resolution, and IRL-a..d built the mechanism (edge, open-issues view + orient surfacing, the `issues` guard, and the backfill onto all 25 issues). But D0059 requires every process to have a DEPLOYING SKILL or it is inert (applied by vigilance, inconsistently) — and the issue-resolution loop was so far applied ad hoc, not codified. The human also asked to FORMALIZE 'when a Decision effectively makes an item moot' (issue003 was mooted by D0053; issue017 noted the same area was 'subsumed by D0053' as PROSE but never recorded a typed edge) — so the moot relation must become structural, not a description string.";
        :>> decision = "Add the issueResolution PROCESS (.engine/processes/issue-resolution.sysml, cadence perItem) with steps: (1) RECORD an Issue (defect/gap/correction, D0047) with description + relatedTask; (2) TRIAGE -> every Issue MUST carry a #Resolves edge from a resolver — a backlog ACTION that will fix it (create one if none) or an accepted DECISION that moots it — enforced by `sysmlv2 guard issues`; (3) RESOLVE (computed) -> the Issue is resolved when its resolver completes (action done / Decision accepted), surfaced by `sysmlv2 open-issues` + orient open_issues; prose 'RESOLVED' is never the source of truth; (4) DECISION-MOOTS-ITEM -> when a Decision makes an item moot, record the typed edge IN THE SAME CHANGE as the Decision: an ISSUE -> `#Resolves dependency from <decision> to <issueNNN>`; a Need/Requirement -> the existing `supersede` edge. Deploy the process via the issue-resolution SKILL (.engine/skills/issue-resolution, registered in skills-registry; D0059). SCOPE (human-accepted): issue-focused — needs/requirements use the existing supersede; backlog-action-moot is out of scope for now.";
        :>> rationale = "D0059: a process with no deploying skill is an orphan, applied inconsistently — the skill makes the loop reliable. Compute-don't-store (D0001/D0018): resolution stays computed from #Resolves + resolver-completeness, never an authored status. Reuses D0077's #Resolves (one mechanism). Codifying decision-moots-item turns the issue017-style 'subsumed by DXXXX' PROSE lapse into a structurally-recorded, computed edge — the kind of silent drift D0047 says to guard. Issue-scoped per the human's acceptance keeps the change tight; generalizing the moot relation to every item type (Needs/Requirements/actions under one edge) was considered and DEFERRED (Needs/Requirements already have supersede; a unified representation is a larger schema/semantic change, re-open if a concrete need arises). Pairs with D0077 (#Resolves), D0029 (relatedTask), D0047 (issue origin + corrections->guards), D0059 (deploying skill), D0004 (supersede scope for needs).";
        :>> consequences = "New process-def (.engine/processes/issue-resolution.sysml) + deploying skill (.engine/skills/issue-resolution/SKILL.md) + a skills-registry entry; #ProspectiveChange marks this as the process-change that introduces the process-def (D0070 keystone co-commit). The `issues` guard (IRL-c/d) is the forward enforcement of step 2; open-issues/orient are the computed surfacing of step 3. doc-sync: CLAUDE.md (§3c RECORD + the issue-resolution process/skill) + usage.md (issue rows). Completes the D0077 IRL milestone (a..e). The two open issues (014 -> suspectDiagnostics, 025 -> readdDroppedViews) remain the live resolving work the process now governs.";
    }

    verification d0078Accept : Test {
        :>> id = "63940516-2c7d-4e8f-a078-516273849669";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "wweatherholtz attests acceptance of d0078 (activate the issue-resolution process + deploying skill; issue-scoped decision-moots-item via #Resolves, supersede for needs) by selecting 'Issue-scoped now (reference supersede for needs)' + 'Accept — build IRL-e' in the IRL-e design dialogue on 2026-06-20.";
    }
    part d0078AcceptR1 : TestResult {
        :>> id = "63940516-2c7d-4e8f-a078-51627384966a";
        :>> outcome = VerdictKind::pass;
        :>> judgedAgainst = "9f7d0ec";
        :>> judgedAt = "2026-06-20";
        :>> judgedBy = "wweatherholtz";
    }
}
