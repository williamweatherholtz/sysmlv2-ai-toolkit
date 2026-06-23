Feature: SysML v2 lexer

  Scenario: Package keyword is recognised
    Given the source text "package"
    When I tokenize the source
    Then the first token kind is Package

  Scenario: ColonColon operator is recognised
    Given the source text "::"
    When I tokenize the source
    Then the first token kind is ColonColon

  Scenario: ColonGtGt operator is recognised
    Given the source text ":>>"
    When I tokenize the source
    Then the first token kind is ColonGtGt

  Scenario: ColonGt operator is recognised
    Given the source text ":>"
    When I tokenize the source
    Then the first token kind is ColonGt

  Scenario: String literal is recognised
    Given the source text '"hello"'
    When I tokenize the source
    Then the first token kind is a string

  Scenario: Integer literal is recognised
    Given the source text "42"
    When I tokenize the source
    Then the first token kind is an integer

  Scenario: Identifier is recognised
    Given the source text "myIdent"
    When I tokenize the source
    Then the first token kind is an identifier

  Scenario: Line comments are skipped
    Given the source text "// skip\npackage"
    When I tokenize the source
    Then the first token kind is Package

  Scenario: Hash marker is recognised
    Given the source text "#DependsOn"
    When I tokenize the source
    Then the first token kind is Hash

  Scenario: Unexpected character returns an error
    Given the source text "@"
    When I tokenize the source
    Then tokenization fails
