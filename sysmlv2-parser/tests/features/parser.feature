Feature: SysML v2 recursive-descent parser

  Scenario: Empty package parses without error
    Given the parser source "package P {}"
    When I parse the source
    Then parsing succeeds
    And the package name is "P"
    And the package has 0 items

  Scenario: Part with a string attribute is parsed
    Given the parser source 'package P { part myPart : Story { :>> id = "abc"; } }'
    When I parse the source
    Then parsing succeeds
    And the package has 1 items

  Scenario: Concatenated string segments are merged into one value
    Given the parser source 'package P { part d : Decision { :>> ctx = "hello " + "world"; } }'
    When I parse the source
    Then parsing succeeds
    And the first attribute of the first part is "ctx" with value "hello world"

  Scenario: Block comment inside a package is ignored
    Given the parser source 'package P { /* a block comment */ part x : T { :>> id = "u"; } }'
    When I parse the source
    Then parsing succeeds
    And the package has 1 items

  Scenario: Part definition (type) is skipped without error
    Given the parser source "package P { part def MyType { attribute x : String; } }"
    When I parse the source
    Then parsing succeeds
    And the package has 0 items

  Scenario: Missing closing brace is a parse error
    Given the parser source "package P {"
    When I parse the source
    Then parsing fails
