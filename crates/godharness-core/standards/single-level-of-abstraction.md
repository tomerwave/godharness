---
id: single-level-of-abstraction
title: Single Level of Abstraction
keywords: [abstraction level, mixed abstraction, orchestration, low-level detail]
paths: []
must-read: false
supersedes: []
relates-to: [small-focused-units]
---

## Rule

A function should not mix high-level orchestration with low-level implementation detail in the
same block. If one line calls a named step and the next manually loops over bytes, they belong
at different levels and shouldn't share a function.

## Why

A reader scans a function expecting one level of detail throughout. Mixed levels force them to
context-switch line by line, and they usually mean a lower-level chunk is ready to be extracted
into its own named step - which also makes that chunk independently testable.

## How to apply

Read a function top to bottom and ask whether every line reads at the same altitude. Where the
altitude drops - a loop, a manual parse, a low-level API call sitting next to named,
high-level steps - extract it into its own function named for what it accomplishes.
