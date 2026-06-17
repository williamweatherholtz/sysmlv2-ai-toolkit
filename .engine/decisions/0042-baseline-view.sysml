// =====================================================================
// D0042 — baselineView: Release occurrence def + computed baseline view design.
// Sprint 14; accepted 2026-06-16 by wweatherholtz (sprint execution).
// =====================================================================
package Decision0042 {
    private import EngineWork::*;

    part d0042 : Decision {
        :>> id = "6a3b4c5d-7e8f-4a9b-0c1d-2e3f4a5b6c7d";
        :>> title = "D0042: Release occurrence def + .tracking/baselines.sysml pattern for computed baseline views";
        :>> createdAt = "2026-06-16";
        :>> createdBy = "claudeSonnet46";
        :>> status = DecisionStatus::accepted;
        :>> context = "baselineView (UC9) needs: (1) a Release type to capture a named snapshot (commit SHA + purpose); (2) a .tracking/baselines.sysml instance file; (3) a computed view in report.py rendering the baseline. The EngineWork package already has a Release part def (membershipRule String). The backlog UC9 item expected commit + member refs.";
        :>> decision = "Model Release as an occurrence def (a baseline cut at a point in time, analogous to TestResult D0032) carrying commit (git SHA) + purpose attributes in a new EngineBaseline package (schema/core/baseline.sysml); store instances in .tracking/baselines.sysml; render a computed Baselines tab in report.py. No explicit members attribute — the commit is the canonical reference.";
        :>> rationale = "baselineView (UC9) needs a Release type for a named snapshot (commit SHA + purpose), an instance file, and a computed view. EngineWork already has a Release part def (membershipRule String) and the backlog UC9 item expected commit + member refs. Member refs: the commit itself is canonical — any item with a passing TestResult judgedAgainst a commit at-or-before this SHA was in scope; an explicit members attribute would duplicate git-computable information, violating text-is-truth. report.py parses baselines.sysml and renders a Baselines tab (tab 5).";
        :>> consequences = "New EngineBaseline package in schema/core/. New .tracking/baselines.sysml with 3 Release instances. report.py gains parse_baselines() + tab_baselines(). validate_schema.py ORDER, validate_tracking.py PRELOAD, validate_instances.py SCHEMA all updated. EngineWork::Release (membershipRule) coexists as the 'future contract' for the full membership-rule pattern; EngineBaseline::Release is the simpler commit+purpose demo form. These may be merged in a future cleanup sprint.";
    }
}
