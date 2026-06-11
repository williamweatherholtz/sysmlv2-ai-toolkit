// =====================================================================
// ProjectDelivery — Delivery-workflow dogfood (dogfoodDelivery, 2026-06-11).
// One Story (foldWhatsNext) run through the full loop:
//   refine (DoR) → standup → implement → review → closeOut
// Phase gates authored here. DeliveryRun action def is read by query.py —
// `storyFoldWhatsNext` appears done once storyFoldWhatsNextR1 is appended.
// =====================================================================
package ProjectDelivery {
    private import EngineElement::*;
    private import EngineWork::*;
    private import EngineVerification::*;

    // ====================================================================
    // The Story being run through the loop
    // ====================================================================
    part foldWhatsNextStory : Story {
        :>> id = "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d";
        :>> title = "foldWhatsNext: merge whats_next.py into query.py workflows subcommand";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "claudeOpus";
        :>> kind = WorkKind::code;
        :>> priority = WorkPriority::p1;
        :>> owner = "claudeOpus";
        :>> estimatedPoints = 2;
    }

    // ====================================================================
    // Delivery-loop backlog — one action per phase; query.py reads this.
    // DoD: storyFoldWhatsNextDoD must pass; TestResult appended on closeOut.
    // ====================================================================
    action def DeliveryRun {
        action storyFoldWhatsNext;
        verification storyFoldWhatsNextDoD : Test { :>> id = "b2c3d4e5-f6a7-4b8c-9d0e-1f2a3b4c5d6e"; :>> method = VerificationMethod::test; :>> procedureText = "query.py workflows subcommand resolves the six workflow DAGs with parity to whats_next.py; whats_next.py retired; validate_all green; single query tool"; }
    }

    // ====================================================================
    // Phase gates (one per Delivery phase)
    // TestResults appended when each phase closes.
    // ====================================================================
    verification refineGate : Test {
        :>> id = "c3d4e5f6-a7b8-4c9d-0e1f-2a3b4c5d6e7f";
        :>> title = "refine phase gate (DoR)";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Story title clear; DoD criterion exists with method; no unmet dependencies; code I/O defined; DoR passes.";
    }
    part refineGateR1 : TestResult { :>> id = "d4e5f6a7-b8c9-4d0e-1f2a-3b4c5d6e7f8a"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "65a4f78"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "claudeOpus"; }

    verification standupGate : Test {
        :>> id = "e5f6a7b8-c9d0-4e1f-2a3b-4c5d6e7f8a9b";
        :>> title = "standup phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Story in-progress; implementation approach stated; no new blockers discovered at standup.";
    }
    part standupGateR1 : TestResult { :>> id = "f6a7b8c9-d0e1-4f2a-3b4c-5d6e7f8a9b0c"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "65a4f78"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "claudeOpus"; }

    verification implementGate : Test {
        :>> id = "07b8c9d0-e1f2-4a3b-4c5d-6e7f8a9b0c1d";
        :>> title = "implement phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::test;
        :>> procedureText = "query.py workflows outputs valid JSON for all six workflows; whats_next.py deleted; validate_all green; query.py orient still works.";
    }
    part implementGateR1 : TestResult { :>> id = "3a4b5c6d-7e8f-4a9b-0c1d-2e3f4a5b6c7d"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "65a4f78"; :>> judgedAt = "2026-06-11"; :>> judgedBy = "claudeOpus"; }

    verification reviewGate : Test {
        :>> id = "18c9d0e1-f2a3-4b4c-5d6e-7f8a9b0c1d2e";
        :>> title = "review phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::inspect;
        :>> procedureText = "Implementation diff reviewed; no regressions in orient/whats-next/suspect subcommands; code is clean.";
    }
    part reviewGateR1 : TestResult { :>> id = "4b5c6d7e-8f9a-4b0c-1d2e-3f4a5b6c7d8e"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "65a4f78"; :>> judgedBy = "claudeOpus"; :>> judgedAt = "2026-06-11"; }

    verification closeOutGate : Test {
        :>> id = "29d0e1f2-a3b4-4c5d-6e7f-8a9b0c1d2e3f";
        :>> title = "closeOut phase gate";
        :>> createdAt = "2026-06-11";
        :>> authoredBy = "claudeOpus";
        :>> method = VerificationMethod::confirmation;
        :>> procedureText = "foldWhatsNextR1 recorded in backlog (pass); storyFoldWhatsNextR1 recorded in DeliveryRun (pass); query.py orient shows foldWhatsNext as done; user accepts.";
    }
}
