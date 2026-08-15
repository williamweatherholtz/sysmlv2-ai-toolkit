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
        :>> id = "53f6749c-aea5-47d6-a809-7bb4db28d3b0";
        :>> title = "Sprint 6: sysmlv2 orient subcommand — cursor + ready/done/outstanding as JSON";
        :>> createdAt = "2026-06-15";
        :>> createdBy = "wweatherholtz";
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
        verification storyRustS6OrientDoD : Test { :>> id = "63cdd83b-004c-4a93-8a7f-72483f157f3c"; :>> method = VerificationMethod::test; :>> procedureText = "sysmlv2 orient [ROOT]: reads state cursor from .tracking/state.sysml; resolves ready/outstanding from backlog action defs + TestResult DoD outcomes; outputs JSON {cursor, ready, suspect, invalidEvidence, counts} matching query.py orient; 6 BDD scenarios, 21 steps green; clippy -D warnings clean."; }
        part storyRustS6OrientDoDR1 : TestResult { :>> id = "7b39bb06-6cfa-427a-af6a-2775f00645b9"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6f45a85"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }
    }

    // ====================================================================
    // Phase gates
    // ====================================================================
    verification rustS6RefineGate : Test {
        :>> id = "8268f1c5-df30-452a-8017-785f079f03df";
        :>> title = "Sprint 6 refine gate (DoR)";
        :>> createdAt = "2026-06-15";
        :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "orient subcommand approach agreed: lib.rs gains Cursor/OrientReport types + parse_cursor/compute_orient_state/orient_root; main.rs dispatch 'orient' → cmd_orient; 3 BDD orient scenarios (cursor extraction, done/ready computation, JSON format); fixture files tests/fixtures/orient/{state,backlog}.sysml; no new crate deps; DoD accepted.";
    }
    part rustS6RefineGateR1 : TestResult { :>> id = "fb1c5e69-9404-4e9a-8d73-3f9fef1c6e73"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "38609e7"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS6StandupGate : Test {
        :>> id = "52b7717d-6eac-4a00-a913-8ed11755ed26";
        :>> title = "Sprint 6 standup gate";
        :>> createdAt = "2026-06-15";
        :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation: dod_result_is_pass scans ActionDef.parts for {task}DoDR{n} Parts with outcome=VerdictKind::pass (latest n wins); compute_orient_state builds done_set + ready list from succession DAG; parse_cursor walks Package items for Part with activeWorkflow attribute; orient_root wires it together reading .tracking/; no blockers.";
    }
    part rustS6StandupGateR1 : TestResult { :>> id = "ce546632-5fef-4768-821c-9bc3107df480"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "38609e7"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS6ImplementGate : Test {
        :>> id = "788ae177-4846-4e55-8fbd-cabb0c027992";
        :>> title = "Sprint 6 implement gate";
        :>> createdAt = "2026-06-15";
        :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::test;
        :>> procedureText = "lib.rs: Cursor/OrientReport types, json_str/dod_result_is_pass/part_outcome_is_pass/str_value helpers, parse_cursor/compute_orient_state/orient_root public API; main.rs: cmd_orient + orient dispatch; cli_bdd.rs: 3 orient scenarios (cursor, done/ready, JSON); orient_bdd.rs: 3 orient_root scenarios; 9 total BDD scenarios 37 steps green; clippy -D warnings clean.";
    }
    part rustS6ImplementGateR1 : TestResult { :>> id = "fdb8440f-cf0f-4bda-b22e-da2b963d8c30"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6f45a85"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS6ReviewGate : Test {
        :>> id = "8c72a95f-59e5-4399-a8e2-5fc79831439c";
        :>> title = "Sprint 6 review gate";
        :>> createdAt = "2026-06-15";
        :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Diff reviewed: lib.rs uses is_none_or/is_some_and/clone_into per clippy::nursery/pedantic; map_or_else for cursor JSON; dod_result_is_pass finds latest-n DoDR Part correctly; compute_orient_state sorts ready list deterministically; orient_root delegates cleanly to parse_cursor + compute_orient_state; json_str escapes all JSON special chars; to_json matches query.py orient format.";
    }
    part rustS6ReviewGateR1 : TestResult { :>> id = "e37d97de-1622-4d3e-954d-26d08cdc279c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "6f45a85"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "claudeSonnet46"; }

    verification rustS6CloseOutGate : Test {
        :>> id = "858511b7-555f-4d56-b978-581c8b40657b";
        :>> title = "Sprint 6 closeOut gate";
        :>> createdAt = "2026-06-15";
        :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "storyRustS6OrientDoDR1 recorded (pass); rustS6OrientDoDR1 recorded in backlog (pass); validate_all green; user accepts Sprint 6 complete.";
    }
    part rustS6CloseOutGateR1 : TestResult { :>> id = "9751792e-48bf-4a80-b3d9-b62dcf56d328"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cbc4d0"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }

    verification rustS6RetroGate : Test {
        :>> id = "a3b4c5d6-e7f8-4a9b-0c1d-2e3f4a5b6c7d";
        :>> title = "Sprint 6 retro gate";
        :>> createdAt = "2026-06-15";
        :>> createdBy = "wweatherholtz";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "Sprint 6 retro held; process improvement items reviewed; any accepted changes recorded as Decisions with CR: commits; team lead confirms retro complete.";
    }
    part rustS6RetroGateR1 : TestResult { :>> id = "040a58d6-ee7c-443d-a86c-6c9d55fb53cb"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "0cbc4d0"; :>> judgedAt = "2026-06-17"; :>> judgedBy = "wweatherholtz"; }
}
