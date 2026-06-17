package Decision0023 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    part d0023 : Decision {
        :>> id = "d1e2f3a4-b5c6-4d7e-8f90-9a0b1c2d3e4f";
        :>> title = "S7 CHANGE: align done-detection naming across Rust orient + query.py; add whats-next subcommand";
        :>> status = DecisionStatus::accepted;
        :>> context = "Sprint 6 introduced {task}DoDR{n} naming for DoD-linked TestResults. Legacy items (EngineBuild, dogfood) use {task}R{n} naming. Two bugs existed: (1) lib.rs dod_result_is_pass only matched DoDR{n} prefix — legacy tasks always appeared outstanding. (2) query.py _RESULT_LINE regex r'(\\w+)R(\\d+)' keyed DoDRn results under '<task>DoD' not '<task>' — sprints S0-S6 appeared not done in query.py. A third gap: no 'sysmlv2 whats-next' subcommand existed to output just the ready list as plain text (the orient subcommand outputs JSON). This change alters what tasks count as 'done' in both the Rust orient and query.py views — a computed-view semantics change (§3e → CHANGE §3a).";
        :>> decision = "Align done-detection naming across the Rust orient and query.py: (1) lib.rs dod_result_is_pass gains an or_else fallback to accept {task}R{n} legacy naming alongside {task}DoDR{n}; (2) orient.rs parse_result_header handles both naming styles via rfind('DoDR') + fallback R-scan; (3) query.py _RESULT_LINE changed to r'part\\s+(\\w+?)(?:DoD)?R(\\d+)\\s*:\\s*TestResult' (non-greedy capture + optional DoD infix) to key both styles under the base task name; (4) add a sysmlv2 whats-next [ROOT] subcommand printing the ready list one per line; (5) add 2 new orient.feature BDD scenarios (legacy naming, whats-next).";
        :>> rationale = "Two bugs made S0-S6 sprints appear not-done: lib.rs only matched the DoDR prefix and query.py's regex keyed DoDRn results under '<task>DoD'; plus no whats-next subcommand existed to emit the plain-text ready list (orient outputs JSON). Because this alters what tasks count as 'done' in both computed views, it is a computed-view semantics change (§3e → CHANGE §3a). The {task}DoDR{n} convention remains canonical for new work; legacy {task}R{n} is tolerated without migration. Accepted: wweatherholtz standing instruction (continue through sprints), 2026-06-15.";
        :>> consequences = "sysmlv2 orient and sysmlv2 whats-next correctly report all S0-S6 sprints as done and S7 as ready. query.py orient also shows correct done state for all sprints. The {task}DoDR{n} naming convention remains canonical for new work; legacy {task}R{n} naming in existing files is tolerated without migration. orient.feature has 6 BDD scenarios, cli.feature has 6 BDD scenarios, all green.";
    }

    verification d0023Accept : Test { :>> id = "e6343ac6-d925-405d-8c0a-6aff0bb8f282"; :>> method = VerificationMethod::confirmation; :>> procedureText = "wweatherholtz attests acceptance of decision d0023 (D0066 acceptance-events migration, attested 2026-06-17); prior record: status=accepted + file prose."; }
    part d0023AcceptR1 : TestResult { :>> id = "ba5dab64-0ad3-42dd-87a8-76c5daae52c3"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "e146621"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
