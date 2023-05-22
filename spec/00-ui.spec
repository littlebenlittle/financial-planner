
Agents:

- User
- App

Feature: User Interface

  Scenario: recording a transaction
    When    a transaction record is submitted
    Then    the transaction record contains the transaction

  Scenario: setting the date range
    When    the date range is set
    Then    the presentation contains only transactions occuring in that range

Feature: Timeline

  Scenario: Timeline
    When    a new transaction record is received
    Then    the timeline
    