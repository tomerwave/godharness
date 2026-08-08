---
id: liskov-substitutability
title: Liskov Substitutability
keywords: [subtype, subclass, inheritance, substitution, override, implements]
paths: []
must-read: false
supersedes: []
relates-to: [testing]
---

## Rule

A subtype must be usable anywhere its supertype is expected, without surprising the caller or
breaking its contract - no narrowed inputs, no widened preconditions, no silently different
behavior for the same call.

## Why

Callers write code against the supertype's contract, not against every concrete
implementation. A subtype that throws on inputs the supertype accepted, or returns something
the contract didn't promise, breaks every caller that trusted the contract instead of checking
which concrete type it got.

## How to apply

Before overriding or implementing a method, check it against the contract the base type or
interface promises: same accepted inputs (or wider), same or stronger guarantees, no new
exceptions a caller couldn't have expected. If a subtype genuinely can't honor the contract, it
shouldn't be a subtype of it - reach for composition instead.
