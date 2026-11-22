Feature: Compiling with clippy
  A compilation given clippy lints its snippets with cargo clippy instead of a plain build, and reports clippy's lints with the compiler's messages, each with its lint as its code. Clippy keeps its own lint levels unless the compilation sets others, and lints only the snippets, never their dependencies.

  Scenario: Clippy's lints are reported as warnings, with their lint as their code
    Given a compilation in edition 2021
    And clippy
    And the snippet "clippy/returned.rs"
    When it is compiled
    Then the snippet "returned" compiled
    And the snippet "returned" has the warning "returning the result of a `let` binding from a block" with the code "clippy::let_and_return" at line 3, column 5

  Scenario: A plain build reports no lint of clippy
    Given a compilation in edition 2021
    And the snippet "clippy/returned.rs"
    When it is compiled
    Then the snippet "returned" compiled
    And the snippet "returned" has no warnings

  Scenario: A group set to warn reports the lints clippy leaves off by default
    Given a compilation in edition 2021
    And clippy with the levels:
      | warn | group | pedantic |
    And the snippet "clippy/literal.rs"
    When it is compiled
    Then the snippet "literal" compiled
    And the snippet "literal" has the warning "long literal lacking separators" with the code "clippy::unreadable_literal" at line 1, column 22

  Scenario Outline: A lint set to deny makes its snippet fail, named with or without clippy before it
    Given a compilation in edition 2021
    And clippy with the levels:
      | deny | lint | <name> |
    And the snippet "clippy/returned.rs"
    When it is compiled
    Then the snippet "returned" failed
    And the snippet "returned" has the error "returning the result of a `let` binding from a block" with the code "clippy::let_and_return" at line 3, column 5

    Examples:
      | name                   |
      | let_and_return         |
      | clippy::let_and_return |

  Scenario: A later level overrides an earlier one
    Given a compilation in edition 2021
    And clippy with the levels:
      | warn  | group | pedantic           |
      | allow | lint  | unreadable_literal |
    And the snippet "clippy/literal.rs"
    When it is compiled
    Then the snippet "literal" compiled
    And the snippet "literal" has no warnings

  Scenario: A lint clippy does not know is reported by clippy itself, in every snippet
    Given a compilation in edition 2021
    And clippy with the levels:
      | warn | lint | no_such_lint |
    And the snippet "clippy/literal.rs"
    And the snippet "clippy/returned.rs"
    When it is compiled
    Then the snippet "literal" has the warning "unknown lint: `clippy::no_such_lint`" with the code "E0602", pointing nowhere
    And the snippet "returned" has the warning "unknown lint: `clippy::no_such_lint`" with the code "E0602", pointing nowhere

  Scenario Outline: A group set to deny makes its snippets fail, and one set to allow silences them
    Given a compilation in edition 2021
    And clippy with the levels:
      | <level> | group | <group> |
    And the snippet "clippy/returned.rs"
    When it is compiled
    Then the snippet "returned" <outcome>

    Examples:
      | level | group | outcome         |
      | deny  | all   | failed          |
      | deny  | style | failed          |
      | allow | style | has no warnings |

  Scenario: A snippet's own attributes override the levels of the compilation
    Given a compilation in edition 2021
    And clippy with the levels:
      | deny | lint | let_and_return |
    And the snippet "clippy/allowed.rs"
    When it is compiled
    Then the snippet "allowed" compiled
    And the snippet "allowed" has no warnings

  Scenario: Clippy given again replaces the earlier levels
    Given a compilation in edition 2021
    And clippy with the levels:
      | warn | group | pedantic |
    And clippy
    And the snippet "clippy/literal.rs"
    When it is compiled
    Then the snippet "literal" compiled
    And the snippet "literal" has no warnings

  Scenario: Clippy lints the snippets, never their dependencies
    Given a compilation in edition 2021
    And clippy with the levels:
      | deny | lint | let_and_return |
    And the dependency greeting
    And the snippet "clippy/greets.rs"
    When it is compiled
    Then the snippet "greets" compiled
    And the snippet "greets" has no warnings

  Scenario: A clippy.toml above the work folder configures clippy, as for the user's own code
    Given a compilation in edition 2021 whose work folder is below a clippy.toml setting too-many-arguments-threshold to 1
    And clippy
    And the snippet "clippy/arguments.rs"
    When it is compiled
    Then the snippet "arguments" compiled
    And the snippet "arguments" has the warning "this function has too many arguments (2/1)" with the code "clippy::too_many_arguments" at line 1, column 1
