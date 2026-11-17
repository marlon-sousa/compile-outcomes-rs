Feature: The edition every snippet is compiled with

  Scenario Outline: A snippet is compiled with the compilation's edition
    Given a compilation in edition <edition>
    And the snippet "snippets/keyword.rs"
    When it is compiled
    Then the snippet "keyword" <outcome>

    Examples:
      | edition | outcome  |
      | 2015    | compiled |
      | 2018    | failed   |
      | 2021    | failed   |
      | 2024    | failed   |
