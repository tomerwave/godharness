---
id: test-independence
title: Test Independence
keywords: [test order, shared fixture, test isolation, test state, depends on another test]
paths: []
must-read: false
supersedes: []
relates-to: [testing, flaky-test-is-signal]
---

## Rule

Every test builds its own fresh fixture and must be runnable alone or in any order. A test that
only passes because another test ran first, or left state behind, is broken even if it's
currently green.

## Why

Order-dependent tests work by accident, not by design - reorder the suite, run one test in
isolation for debugging, or parallelize the run, and green tests start failing for reasons that
have nothing to do with the code they claim to test. The dependency itself is the bug, whether
or not it's currently causing a visible failure.

## How to apply

Give each test its own setup rather than relying on shared mutable state left by another test.
If a fixture is expensive to build, share the *creation* logic, not a mutable instance - each
test still gets its own fresh copy. Run a suspect test alone before trusting it.
