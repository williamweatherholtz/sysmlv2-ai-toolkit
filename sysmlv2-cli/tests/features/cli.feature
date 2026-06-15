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
