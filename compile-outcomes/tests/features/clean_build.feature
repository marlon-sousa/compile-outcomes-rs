Feature: A clean build
  A clean build removes the build output an earlier compilation left before building, so that every crate compiles again, as after cargo clean. It removes only a target folder Cargo marked as its own.

  Scenario: A clean build removes the build output an earlier build left
    Given a compilation in edition 2021 whose build output holds a file an earlier build left
    And a clean build
    And the snippet "snippets/unused.rs"
    When it is compiled
    Then the snippet "unused" compiled
    And the file the earlier build left is gone

  Scenario: A build that is not clean keeps the build output
    Given a compilation in edition 2021 whose build output holds a file an earlier build left
    And the snippet "snippets/unused.rs"
    When it is compiled
    Then the snippet "unused" compiled
    And the file the earlier build left is still there

  Scenario: A target folder Cargo did not make is left untouched
    Given a compilation in edition 2021 whose target folder holds a file of its own
    And a clean build
    And the snippet "snippets/unused.rs"
    When it is compiled
    Then the compilation is refused with:
      """
      The folder
      """
    And the refusal names the target folder and the file "CACHEDIR.TAG"
    And the compilation is refused with 1 problem
    And the target folder's file of its own is still there
