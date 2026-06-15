package TestBacklog {
    action def TestWork {
        action taskA;
        action taskB;
        verification taskADoD : Test { :>> id = "aaaa1111-0000-4000-8000-000000000001"; :>> method = VerificationMethod::test; :>> procedureText = "taskA criterion"; }
        part taskADoDR1 : TestResult { :>> id = "aaaa2222-0000-4000-8000-000000000002"; :>> outcome = VerdictKind::pass; :>> judgedAgainst = "abc1234"; :>> judgedAt = "2026-06-15"; :>> judgedBy = "test"; }
        verification taskBDoD : Test { :>> id = "bbbb1111-0000-4000-8000-000000000003"; :>> method = VerificationMethod::test; :>> procedureText = "taskB criterion"; }
        first taskA then taskB;
    }
}
