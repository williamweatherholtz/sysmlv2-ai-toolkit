package TestState {
    part cursor : StateCursor {
        :>> activeWorkflow = "TestFlow";
        :>> activePhase = "test/run";
        :>> enteredAt = "2026-06-15";
        :>> enteredBy = "testActor";
    }
}
