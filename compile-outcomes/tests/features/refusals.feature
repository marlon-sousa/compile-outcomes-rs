Feature: What keeps a compilation from its outcomes
  Problems with the description are reported together, before anything is
  written; a problem while compiling stops it, and is reported alone.

  Scenario: A compilation without a snippet is refused
    Given a compilation in edition 2021
    When it is compiled
    Then the compilation is refused with:
      """
      The compilation has no snippet. Add one with the snippet or snippets_in method.
      """
    And the compilation is refused with 1 problem

  Scenario: Every problem with the description is reported together
    Given a compilation in edition 2021
    And the snippet "nowhere.rs"
    And the snippets in "nowhere"
    And the snippet "folder"
    And the snippets in "invalid"
    And the snippets in "duplicates"
    And the lockfile "nowhere.lock"
    When it is compiled
    Then the compilation is refused with:
      """
      The snippet file tests/fixtures/nowhere.rs could not be read:
      The snippet folder tests/fixtures/nowhere could not be read:
      The snippet file tests/fixtures/folder could not be read:
      The snippet file tests/fixtures/invalid/2fields.rs has no valid name. A name is made of ASCII letters, digits, underscores and hyphens, and does not start with a digit.
      The snippet files tests/fixtures/duplicates/same-name.rs and tests/fixtures/duplicates/same_name.rs would both make the crate snippet_same_name.
      The lockfile tests/fixtures/nowhere.lock could not be read:
      """
    And the compilation is refused with 6 problems

  Scenario: A dependency that fails to compile stops the compilation, naming it
    Given a compilation in edition 2021
    And the dependency "broken" at "tests/fixtures/broken"
    And the snippet "snippets/unused.rs"
    When it is compiled
    Then the compilation is refused with:
      """
      The dependency broken failed to compile. The compiler said:
      expected expression, found `}`
      """
    And the compilation is refused with 1 problem

  Scenario: Cargo stopping before any snippet is reported with what it wrote
    Given a compilation in edition 2021
    And the dependency "missing" at "tests/fixtures/nowhere"
    And the snippet "snippets/unused.rs"
    When it is compiled
    Then the compilation is refused with:
      """
      Cargo stopped before compiling any snippet, and wrote:
      """
    And the compilation is refused with 1 problem

  Scenario: A project that cannot be written stops the compilation
    Given a compilation in edition 2021 whose work folder is a file
    And the snippet "snippets/unused.rs"
    When it is compiled
    Then the compilation is refused with:
      """
      The project could not be written at
      """
    And the compilation is refused with 1 problem

  Scenario: A project folder compile-outcomes did not write is left untouched
    Given a compilation in edition 2021 whose project folder holds a file of its own
    And the snippet "snippets/unused.rs"
    When it is compiled
    Then the compilation is refused with:
      """
      The folder
      """
    And the compilation is refused with 1 problem
    And the file of its own is still there

  Scenario: A project folder handed over to compile-outcomes is replaced
    Given a compilation in edition 2021 whose project folder holds a file of its own
    And the project folder is handed over to compile-outcomes
    And the snippet "snippets/unused.rs"
    When it is compiled
    Then the snippet "unused" compiled
    And the file of its own is gone
