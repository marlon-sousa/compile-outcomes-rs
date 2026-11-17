Feature: What the compiler said about each snippet
  compile-outcomes compiles every snippet and reports, for each one,
  whether it compiled and every error and warning, in the compiler's words.

  Background:
    Given a compilation in edition 2021
    And the dependency greeting

  Scenario: A snippet that compiles has no errors and no warnings
    Given the snippet "snippets/hello.rs"
    When it is compiled
    Then the snippet "hello" compiled
    And the snippet "hello" has no warnings

  Scenario: A snippet that compiles with warnings reports them
    Given the snippet "snippets/unused.rs"
    When it is compiled
    Then the snippet "unused" compiled
    And the snippet "unused" has the warning "unused variable: `unused`" with the code "unused_variables" at line 2, column 9

  Scenario: A diagnostic's file is as the compiler names it, with the separators of the platform
    Given the snippet "snippets/unused.rs"
    When it is compiled
    Then the warning of "unused" points at its file as the compiler names it, with the separators of the platform

  Scenario: A snippet that fails reports its error with its code and where it points
    Given the snippet "snippets/shout.rs"
    When it is compiled
    Then the snippet "shout" failed
    And the snippet "shout" has the error "cannot find function `shout` in crate `greeting`" with the code "E0425" at line 2, column 15

  Scenario: Errors keep the order the compiler gave them, and its closing notes are left out
    Given the snippet "snippets/two_errors.rs"
    When it is compiled
    Then the errors of the snippet "two_errors" are, in order:
      """
      cannot find function `first_missing` in this scope
      cannot find function `second_missing` in this scope
      """

  Scenario: An outcome displays as the compiler rendered its errors, then its warnings
    Given the snippet "snippets/warned_and_failed.rs"
    When it is compiled
    Then the snippet "warned_and_failed" failed
    And the snippet "warned_and_failed" displays every error the compiler rendered, starting "error[E0308]: mismatched types", then every warning, starting "warning: unused import: `std::fmt`"

  Scenario: Every snippet of a folder has its own outcome, whatever the others did
    Given the snippets in "folder"
    When it is compiled
    Then the snippet "a" compiled
    And the snippet "b" failed
    And the compilation has the snippets "a, b"

  Scenario: A snippet file is compiled whatever its extension
    Given the snippet "snippets/plain.txt"
    When it is compiled
    Then the snippet "plain" compiled

  Scenario: A dependency's features can be enabled
    Given the dependency greeting with the feature "loud"
    And the snippet "snippets/shout.rs"
    When it is compiled
    Then the snippet "shout" compiled

  Scenario: A dependency's default features can be turned off
    Given the dependency greeting without its default features
    And the snippet "snippets/please.rs"
    When it is compiled
    Then the snippet "please" failed
