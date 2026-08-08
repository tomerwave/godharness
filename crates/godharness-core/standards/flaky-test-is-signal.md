---
id: flaky-test-is-signal
title: A Flaky Test Is a Signal
keywords: [flaky test, intermittent failure, nondeterministic test, quarantine test, retry test]
paths: []
must-read: false
supersedes: []
relates-to: [testing, test-independence]
---

## Rule

Treat a nondeterministic test as evidence of a real race condition, timing dependency, or
shared-state leak until proven otherwise. Don't quarantine it, wrap it in a retry loop, or
delete it without first identifying the cause.

## Why

A test doesn't usually fail intermittently by accident - something underneath it actually is
nondeterministic, and the test is the only thing currently telling you so. Suppressing the
symptom removes the one signal pointing at a real bug, which often surfaces in production later
with far less information to debug it than the failing test already gave you.

## How to apply

When a test fails intermittently, run it in isolation and under load before assuming it's "just
flaky." Look for shared mutable state, unawaited async work, real time or randomness left
unmocked, or ordering dependencies. Only add a retry or skip after the cause is understood and
the retry is a deliberate decision, not a first resort.
