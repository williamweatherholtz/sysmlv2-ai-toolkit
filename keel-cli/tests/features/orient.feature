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

  Scenario: task with unresolvable SHA appears in invalidEvidence
    Given a tracking dir with task "taskEvil" and an invalid SHA result
    When I run orient on the prepared dir
    Then done count is 0
    And invalidEvidence contains "taskEvil"

  Scenario: legacy R naming (without DoDR infix) is detected as done
    Given a tracking dir with a legacy-named result for "taskOld"
    When I run orient on the prepared dir
    Then done count is 1
    And outstanding count is 0
    And ready is empty

  Scenario: whats-next returns tasks with completed predecessors
    Given a tracking dir with task "taskA" and a passing result
    And task "taskB" depends on "taskA"
    When I run whats-next
    Then ready contains "taskB"

  Scenario: ordering-only succession does not block ready computation
    Given a tracking dir with an ordering-only edge from "taskA" to "taskB"
    When I run orient on the prepared dir
    Then done count is 0
    And outstanding count is 2
    And ready contains "taskA"
    And ready contains "taskB"

  Scenario: tasks and results at package level (outside action def) are indexed
    Given a tracking dir with package-level task "pkgTask" and result
    When I run orient on the prepared dir
    Then done count is 1
    And outstanding count is 0
    And ready is empty
