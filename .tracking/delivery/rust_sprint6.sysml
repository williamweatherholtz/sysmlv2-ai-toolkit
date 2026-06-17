// =====================================================================
// ProjectDeliveryRustS6 — Sprint 6 delivery run: sysmlv2 orient subcommand.
// Story: rustS6Orient — orient subcommand reads .tracking/ cursor + backlog;
//   computes ready/done/outstanding; outputs JSON matching query.py orient.
// Workflow: DeliveryWorkflow::Delivery (refine→standup→implement→review→closeOut+retro).
// =====================================================================
package ProjectDeliveryRustS6 {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // Sprint 6 Story
    // ====================================================================
    part rustS6OrientStory : Story {
        :>> id = "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d";
        :>> title = "Sprint 6: sysmlv2 orient subcommand — cursor + ready/done/outstanding as JSON";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeSonnet46";
        :>> estimatedPoints = 2;
    }

    // ====================================================================
    // Delivery backlog — DoD read by query.py.
    // ====================================================================
    action def DeliveryRunRustS6 {
        action storyRustS6Orient;
        verification storyRustS6OrientDoD : Test { :>> id = "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e"; :>> method = VerificationMethod::test; :>> procedureText = "sysmlv2 orient [ROOT]: reads state cursor from .tracking/state.sysml; resolves ready/outstanding from backlog action defs + TestResult DoD outcomes; outputs JSON {cursor, ready, suspect, invalidEvidence, counts} matching query.py orient; 6 BDD scenarios, 21 steps green; clippy -D warnings clean."; }
        part storyRustS6OrientDoDR1 : TestResult { :>> id = "c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6f45a85"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification rustS6RefineGate : Test {
        :>> id = "d4e5f6a7-b8c9-4d0e-1f2a-3b4c5d6e7f8a";
        :>> title = "Sprint 6 refine gate (DoR)";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "orient subcommand approach agreed: lib.rs gains Cursor/OrientReport types + parse_cursor/compute_orient_state/orient_root; main.rs dispatch 'orient' → cmd_orient; 3 BDD orient scenarios (cursor extraction, done/ready computation, JSON format); fixture files tests/fixtures/orient/{state,backlog}.sysml; no new crate deps; DoD accepted.";
    }
    part rustS6RefineGateR1 : TestResult { :>> id = "e5f6a7b8-c9d0-4e1f-2a3b-4c5d6e7f8a9b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "38609e7"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS6StandupGate : Test {
        :>> id = "f6a7b8c9-d0e1-4f2a-3b4c-5d6e7f8a9b0c";
        :>> title = "Sprint 6 standup gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation: dod_result_is_pass scans ActionDef.parts for {task}DoDR{n} Parts with outcome=VerdictKind::pass (latest n wins); compute_orient_state builds done_set + ready list from succession DAG; parse_cursor walks Package items for Part with activeWorkflow attribute; orient_root wires it together reading .tracking/; no blockers.";
    }
    part rustS6StandupGateR1 : TestResult { :>> id = "a7b8c9d0-e1f2-4a3b-4c5d-6e7f8a9b0c1d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "38609e7"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS6ImplementGate : Test {
        :>> id = "b8c9d0e1-f2a3-4b4c-5d6e-7f8a9b0c1d2e";
        :>> title = "Sprint 6 implement gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "lib.rs: Cursor/OrientReport types, json_str/dod_result_is_pass/part_outcome_is_pass/str_value helpers, parse_cursor/compute_orient_state/orient_root public API; main.rs: cmd_orient + orient dispatch; cli_bdd.rs: 3 orient scenarios (cursor, done/ready, JSON); orient_bdd.rs: 3 orient_root scenarios; 9 total BDD scenarios 37 steps green; clippy -D warnings clean.";
    }
    part rustS6ImplementGateR1 : TestResult { :>> id = "c9d0e1f2-a3b4-4c5d-6e7f-8a9b0c1d2e3f"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6f45a85"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS6ReviewGate : Test {
        :>> id = "d0e1f2a3-b4c5-4d6e-7f8a-9b0c1d2e3f4a";
        :>> title = "Sprint 6 review gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: lib.rs uses is_none_or/is_some_and/clone_into per clippy::nursery/pedantic; map_or_else for cursor JSON; dod_result_is_pass finds latest-n DoDR Part correctly; compute_orient_state sorts ready list deterministically; orient_root delegates cleanly to parse_cursor + compute_orient_state; json_str escapes all JSON special chars; to_json matches query.py orient format.";
    }
    part rustS6ReviewGateR1 : TestResult { :>> id = "e1f2a3b4-c5d6-4e7f-8a9b-0c1d2e3f4a5b"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6f45a85"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS6CloseOutGate : Test {
        :>> id = "f2a3b4c5-d6e7-4f8a-9b0c-1d2e3f4a5b6c";
        :>> title = "Sprint 6 closeOut gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyRustS6OrientDoDR1 recorded (pass); rustS6OrientDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 6 complete.";
    }
    part rustS6CloseOutGateR1 : TestResult { :>> id = "9751792e-48bf-4a80-b3d9-b62dcf56d328"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cbc4d0"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }

    verification rustS6RetroGate : Test {
        :>> id = "a3b4c5d6-e7f8-4a9b-0c1d-2e3f4a5b6c7d";
        :>> title = "Sprint 6 retro gate";
        :>> createdAt = "2026-06-15";
        :>> authoredBy = "wweatherholtz"; :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 6 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part rustS6RetroGateR1 : TestResult { :>> id = "040a58d6-ee7c-443d-a86c-6c9d55fb53cb"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cbc4d0"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
