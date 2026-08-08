---
id: architecture-decisions
title: Record Architecture Decisions
keywords: [architecture decision, adr, decision record, why did we choose]
paths: []
must-read: false
supersedes: []
relates-to: []
---

## Rule

Write an architecture decision record when a choice is hard to reverse, affects multiple
components, or would otherwise get re-litigated from scratch by the next person who questions
it.

## Why

Without a written record, the reasoning behind a decision lives only in the heads of whoever was
in the room, and gets rebuilt from scratch - or silently reversed - the next time someone
questions it. A short record with real constraints and alternatives considered is far cheaper
than re-deriving the same decision twice.

## How to apply

Capture the constraint that mattered, the alternatives considered and why they were rejected,
and the decision itself. Keep it short enough that someone would actually read it before
proposing the same alternative again.
