Feature: SysML v2 semantic validation — PackageRegistry

  Scenario: Unknown import namespace produces a diagnostic
    When I validate the package "package P { private import NoSuchNs::*; }"
    Then there are 1 diagnostics
    And diagnostic 1 message contains "NoSuchNs"

  Scenario: Unknown type reference produces a diagnostic
    Given the schema package "package Types { part def GoodType; }" is registered
    When I validate the package 'package P { private import Types::*; part x : BadType { :>> id = "1"; } }'
    Then there are 1 diagnostics
    And diagnostic 1 message contains "BadType"

  Scenario: Known type reference validates cleanly
    Given the schema package "package Types { part def GoodType; }" is registered
    When I validate the package 'package P { private import Types::*; part x : GoodType { :>> id = "1"; } }'
    Then there are no diagnostics

  Scenario: Unknown enum namespace produces a diagnostic
    Given the schema package 'package Enums { enum def Status { active; inactive; } }' is registered
    When I validate the package 'package P { private import Enums::*; part x { :>> s = Missing::active; } }'
    Then there are 1 diagnostics
    And diagnostic 1 message contains "Missing"

  Scenario: Unknown enum member produces a diagnostic
    Given the schema package 'package Enums { enum def Status { active; inactive; } }' is registered
    When I validate the package 'package P { private import Enums::*; part x { :>> s = Status::gone; } }'
    Then there are 1 diagnostics
    And diagnostic 1 message contains "gone"

  Scenario: Valid enum literal validates cleanly
    Given the schema package 'package Enums { enum def Status { active; inactive; } }' is registered
    When I validate the package 'package P { private import Enums::*; part x { :>> s = Status::active; } }'
    Then there are no diagnostics

  Scenario: ScalarValues import is always accepted
    When I validate the package "package P { private import ScalarValues::*; }"
    Then there are no diagnostics

  Scenario: Type ref inside action def body is validated
    Given the schema package "package Types { part def MyResult; }" is registered
    When I validate the package 'package P { private import Types::*; action def D { part r : BadResult { :>> id = "1"; } } }'
    Then there are 1 diagnostics
    And diagnostic 1 message contains "BadResult"
