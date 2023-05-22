
Feature: User Interface

  Scenario: recording a transaction
    When    a transaction record is submitted
    Then    the transaction record contains the transaction

  Scenario: setting the date range
    When    the date range is set
    Then    the presentation contains only transactions occuring in that range

Feature: Webiew UI

  Scenario: submitting an income form
    Given   the user has entered information in the income form
    When    the user clicks the submit button
    Then    the transaction record contains an income entry with that information
    