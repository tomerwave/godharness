---
id: dont-repeat-yourself
title: Don't Repeat Yourself
keywords: [duplicate, duplication, dry, copy-paste, repeated logic, same rule twice]
paths: []
must-read: false
supersedes: []
relates-to: [small-focused-units, prefer-existing-solutions]
---

## Rule

Every piece of knowledge or business rule should have exactly one authoritative representation
in the system. This is about duplicated *knowledge*, not duplicated shape - two blocks that
happen to look similar but encode unrelated facts should stay separate, not be merged.

## Why

When one rule is written twice, only one copy gets updated when the rule changes, and the other
quietly goes stale until it causes a bug nobody can explain. Merging two blocks that only look
alike creates the opposite problem: an unrelated change to one now has to work around logic that
serves a different rule.

## How to apply

Before duplicating a check, calculation, or constant, ask whether it's the same underlying rule
or coincidentally similar code. If it's the same rule, extract one shared definition and have
both call sites use it. If the resemblance is coincidental, leave them separate - a false merge
is worse than the duplication it was meant to remove.
