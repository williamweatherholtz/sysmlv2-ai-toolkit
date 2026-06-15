Feature: orient subcommand

  Scenario: single completed task has done=1 and empty ready
    Given a tracking dir with task "taskDone" and a passing result
    When I run orient
    Then done count is 1
    And outstanding count is 0
    And ready is empty

  Scenario: only the predecessor of a blocked task appears in ready
    Given a tracking dir with tasks "taskA" and "taskB" where "taskB" depends on "taskA"
    When I run orient
    Then done count is 0
    And outstanding count is 2
    And ready contains "taskA"

  Scenario: completed predecessor makes dependent task ready
    Given a tracking dir with task "taskA" and a passing result
    And task "taskB" depends on "taskA"
    When I run orient
    Then done count is 1
    And outstanding count is 1
    And ready contains "taskB"
