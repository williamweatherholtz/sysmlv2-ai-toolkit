Feature: sysmlv2-cli library

  Scenario: check a valid SysML file reports clean
    Given a SysML file with content "package ValidPkg {}"
    When I check the files
    Then the check report is clean

  Scenario: check an invalid SysML file reports an error
    Given a SysML file with content "THIS IS NOT SYSML @@@"
    When I check the files
    Then the check report has errors

  Scenario: collect_sysml finds .sysml files in a directory
    Given a temporary directory containing 2 SysML files
    When I collect SysML files from the directory
    Then 2 paths are collected

  Scenario: orient extracts cursor from state.sysml
    Given orient fixtures loaded from "tests/fixtures/orient"
    When I parse the orient cursor
    Then the cursor active workflow is "TestFlow"

  Scenario: orient computes done and ready tasks
    Given orient fixtures loaded from "tests/fixtures/orient"
    When I compute the orient state
    Then 1 task is done
    Then 1 task is outstanding
    Then "taskB" is in the ready list

  Scenario: orient JSON includes cursor and ready task
    Given orient fixtures loaded from "tests/fixtures/orient"
    When I compute the orient state
    Then the orient JSON contains "TestFlow"
    Then the orient JSON contains "taskB"
