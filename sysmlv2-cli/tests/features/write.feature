Feature: write API — append-result and add-task

  Scenario: append-result creates DoDR1 for a task with no existing results
    Given a tracking file with task "taskAlpha" and a DoD verification
    When I append a passing result for "taskAlpha" at SHA "abc1234"
    Then the file contains "taskAlphaDoDR1 : TestResult"
    And outcome is "VerdictKind::pass"
    And judgedAgainst is "abc1234"

  Scenario: append-result on a task with existing DoDR1 creates DoDR2
    Given a tracking file with task "taskBeta" and an existing DoDR1
    When I append a passing result for "taskBeta" at SHA "def5678"
    Then the file contains "taskBetaDoDR2 : TestResult"

  Scenario: append-result rejects an unknown task
    Given a tracking file with task "taskGamma"
    When I append a result for unknown task "noSuchTask" at SHA "aaa0000"
    Then the write fails with task-not-found

  Scenario: append-result rejects an invalid verdict
    Given a tracking file with task "taskDelta"
    When I append a "banana" result for "taskDelta" at SHA "bbb0001"
    Then the write fails with invalid-verdict

  Scenario: add-task inserts action and DoD verification
    Given a tracking file with action def "MyBuild"
    When I add task "taskNew" with DoD "new task passes tests" method "test" to def "MyBuild"
    Then the file contains "action taskNew;"
    And the file contains "verification taskNewDoD : Test"

  Scenario: add-task rejects a duplicate task name
    Given a tracking file with action def "DupBuild" containing task "taskExisting"
    When I add task "taskExisting" with DoD "dup" method "test" to def "DupBuild"
    Then the write fails with task-already-exists

  Scenario: append-result generates a UUID in the id field
    Given a tracking file with task "taskUuid" and a DoD verification
    When I append a passing result for "taskUuid" at SHA "ccc0002"
    Then the new result has a non-empty id field
