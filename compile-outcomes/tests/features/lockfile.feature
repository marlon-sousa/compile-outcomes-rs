Feature: The lockfile the dependencies resolve with
  A compilation given a lockfile copies it into the project, so that the
  dependencies resolve to the versions it holds; the original is never written.

  Scenario: The lockfile is copied into the project and left as it was
    Cargo keeps the version of a lockfile it updates, and writes version 4
    when it makes one, so version 3 shows the copy was used.

    Given a compilation in edition 2021
    And the lockfile "lockfile.lock"
    And the snippet "snippets/unused.rs"
    When it is compiled
    Then the snippet "unused" compiled
    And the project's lockfile is of version 3, as the copy was
    And the lockfile "lockfile.lock" is left as it was

  Scenario: Dependencies resolve to the versions the lockfile holds
    The lockfile holds itoa 1.0.10, older than the newest itoa 1, which
    Cargo would choose without it.

    Given a compilation in edition 2021
    And the dependency "itoa" of version "1"
    And the lockfile "itoa.lock"
    And the snippet "snippets/formatted.rs"
    When it is compiled
    Then the snippet "formatted" compiled
    And the project resolved "itoa" to the version "1.0.10"
