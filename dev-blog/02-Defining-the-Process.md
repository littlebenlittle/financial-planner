---
id: tREmDsdp9vlHGda65q
date: 2023-05-20
title: Defining-the-Process
---

# {{ meta.title}}

### Quick Summary

> At this stage we know what we *might* build. It's time to start defining that
  thing in a language that is amenable to the hardware we have available.
  
This is much like the measurement phase of painting. Certain rules must be met
for the painting to be compatible with our intuition about how 3D shapes work.
  
## How can we specify state machines?

A state machine can be specified as a relation between predicates over input
streams and predicates over output streams.

### UI Specification

In this case the input stream carries *UI Actions* and the output stream carries
*UI States*. When a user inputs a sequence of UI actions that satisfies some
predicate, the UI will produce a sequence of states that satisfy some other
predicate.

For example, in the Finance Application we expect the Transaction List to update
whenver the user submits a new transaction form. This can expressed as

```feature
Scenario: submitting an income entry form
    Given   the page has loaded
    When    the user submits an Income Entry
    Then    the Transactions List contains the Income Entry
```

### Notes

- Underlying semantics is the log of input events
- Do changes propogate immediately or on sync?
- ~~Each component has a cached copy of the state~~
- Each component owns a log of events from other components or environment
- This might be too many logs for applications; current state is a "compressed"
  version of log entries
- Each component is specified by a state machine that ingests events,
  updates its state, and emits other events
