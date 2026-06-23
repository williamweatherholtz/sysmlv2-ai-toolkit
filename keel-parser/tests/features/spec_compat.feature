Feature: SysML v2 spec compatibility

  Scenario: Grammar version reflects offline mode baked in at compile time
    # .cargo/config.toml sets SYSML_V2_SPEC_OFFLINE=1 (force=false) so the
    # build script emits SYSML_V2_GRAMMAR_VERSION=offline by default.
    When I read the grammar version constant
    Then the grammar version is "offline"

  Scenario: SHA mismatch produces a descriptive error
    Given manifest bytes "hello world"
    And expected SHA "0000000000000000000000000000000000000000000000000000000000000000"
    When I verify the manifest SHA
    Then verification fails
    And the error message contains "SHA mismatch"

  Scenario: Correct SHA verifies cleanly
    Given manifest bytes "hello world"
    And the expected SHA is the SHA-256 of the manifest bytes
    When I verify the manifest SHA
    Then verification succeeds
