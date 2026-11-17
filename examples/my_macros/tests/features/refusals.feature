Feature: What my macros refuse

  Scenario: A user without a name is refused
    Given the snippet "missing_field"
    When it is compiled
    Then it fails with an error that contains "missing field `name`"

  Scenario: A user without an age is refused
    Given the snippet "missing_age"
    When it is compiled
    Then it fails with an error that contains "missing field `age`"
