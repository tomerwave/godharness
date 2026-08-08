---
id: test-pyramid-shape
title: Test Pyramid Shape
keywords: [test pyramid, e2e test, integration test, unit test, too many end-to-end]
paths: []
must-read: false
supersedes: []
relates-to: [testing]
---

## Rule

Keep far more small, fast tests than integration tests, and far more integration tests than
end-to-end tests. Invert that ratio and the suite becomes slow, flaky, and expensive to
maintain.

## Why

A test that spins up real infrastructure catches things a unit test can't, but it's slow and
more likely to fail for reasons unrelated to the code under test. A suite top-heavy with those
tests takes long enough to run that people stop running it locally, and failures get treated as
noise instead of signal.

## How to apply

When adding coverage for new logic, default to the smallest test that can prove it - no
process boundary, no real network or database, unless the thing actually being tested is that
integration. Reserve end-to-end tests for the few paths where nothing less proves the system
actually works end to end.
