---
id: tREmDsdp9vlHGda65q
date: 2023-05-20
title: Defining the Process
template: post
---

# Defining the Process

### Quick Summary

> At this stage we know what we *might* build. It's time to start defining that
  thing in a language that is amenable to the hardware we have available.

## How can we specify UIs?

One model for UIs is that the user provides a sequence of input events. The
events then form a log and queries about the state of the application reduce to
queries about the state of the log.

For example, a user might input:

```
ReportTransaction:
  date: 2023-01-01
  value: 150
  kind: income
  id: 1
ReportTransaction:
  date: 2023-01-05
  value: 100
  kind: Expense
  id: 2
DeleteTransaction:
  id: 2
ReportTransaction:
  date: 2023-01-05
  value: 120
  kind: Expense
  id: 2
```

We might then ask: What are the entries in the list of transactions after this
sequence of events?

Initially the user reports an income of 150 on 1st of January and then an
expense of 100 on the 5th of Januay. Then delete the expense transaction and
report another expense. So the transaction log *should* contain the first
transaction and the last transaction, but not the one that was deleted.

```
Transactions:
- date: 2023-01-01
  value: 150
  kind: income
  id: 1
- date: 2023-01-05
  value: 120
  kind: Expense
  id: 2
```

This is a specific instance of a general rule:

> The transaction list should contain every non-deleted transaction and no
  deleted transaction.

So how do we formally specify this intuitive rule?

## Semantics of Event-Driven Architectures

The semantics of a specification tell us the *meaning*, that is how the 
properties of a system's input relate to properties of the system's output.
In this case the input is an ordered sequence of user events that form a log
and the output is the sequence of UI states that the application goes through
as these inputs are processed. As specification engineers, our job is to relate
logical facts about the log to logical facts about the sequence UI states.

So how do we transalate our above example into a language about sequences of
user input and sequences of UI states? Consider the following definitions:

- Deleted transaction: A "report transaction" event that is followed by
  a delete transaction event with the same transaction id
- Non-deleted transaction: A "report transaction" event that is NOT followed by
  a delete transaction event with the same transaction id
  
So our specification looks like:

- For every "report transaction" event with "id=X", if there is NO later
  "delete transaction" event with "id=X" then the transaction with "id=X"
  is in transaction list
- For every "report transaction" event with "id=X", if there is SOME later
  "delete transaction" event with "id=X" then the transaction with "id=X"
  is NOT in transaction list

This is an example of a specification for how the Transaction List should
behave. We can then generate sequences of user actions, run them through our
Transaction List generation logic, and test if our implementation matches
the specification.

This is not a complete specifcation by any means. What if a user tries to create
two transactions with same id? What if a user deletes a transaction with an id
that hasn't been created yet? The more questions like these that we can answer,
the more constrained our implementation will be and the fewer "unexpected"
results we will get!

### Notes

- For responsive UI design, we are often interested in "immediate" next states,
  which is "next" or X in LTL
