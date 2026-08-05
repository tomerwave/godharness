---
id: testing
title: Test Behavior, Not Structure
keywords: [test, testing, unit test, integration test, coverage]
paths: []
must-read: false
supersedes: []
relates-to: [error-handling]
---

## Rule

Write tests against observable behavior - inputs and outputs, not internal structure or
implementation details. A test should still pass after a refactor that doesn't change behavior.

## Why

Tests coupled to implementation details break on every refactor regardless of whether behavior
changed, which trains people to ignore test failures instead of trusting them. Behavior-focused
tests are the only kind that catch real regressions without also punishing safe changes.

## How to apply

Before asserting on an internal detail (a private field, a call count, an implementation-only
helper), ask whether the same assertion could be made against the public input/output contract
instead. If so, test that instead.
