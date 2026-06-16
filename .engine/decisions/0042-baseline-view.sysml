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
        :>> authoredBy = "claudeSonnet46";
        :>> status = DecisionStatus::accepted;
        :>> context = "baselineView (UC9) needs: (1) a Release type to capture a named snapshot (commit SHA + purpose); (2) a .tracking/baselines.sysml instance file; (3) a computed view in report.py rendering the baseline. The EngineWork package already has a Release part def (membershipRule String). The backlog UC9 item expected commit + member refs.";
        :>> decisionText = "Design: Release is an occurrence def (a Release IS an event — a baseline cut at a point in time, analogous to TestResult D0032). Added EngineBaseline package in schema/core/baseline.sysml with Release occurrence def carrying commit (git SHA) + purpose attributes. Instances in .tracking/baselines.sysml. View: report.py parses baselines.sysml and renders a Baselines tab (tab 5). Member refs: the commit itself is the canonical reference — any item with a passing TestResult judgedAgainst a commit at-or-before this SHA was in scope. No explicit members attribute (would duplicate git-computable information, violating text-is-truth). Accepted: wweatherholtz 2026-06-16 at commit aba9e6f.";
        :>> consequences = "New EngineBaseline package in schema/core/. New .tracking/baselines.sysml with 3 Release instances. report.py gains parse_baselines() + tab_baselines(). validate_schema.py ORDER, validate_tracking.py PRELOAD, validate_instances.py SCHEMA all updated. EngineWork::Release (membershipRule) coexists as the 'future contract' for the full membership-rule pattern; EngineBaseline::Release is the simpler commit+purpose demo form. These may be merged in a future cleanup sprint.";
    }
}
