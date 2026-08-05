---
id: verify-through-real-path
title: Verify Through the Real Path
keywords: [verify, verified, hard evidence, tested, confirm, confidence]
paths: []
must-read: true
supersedes: []
relates-to: []
---

## Rule

A "verified" claim only counts if the test exercised the actual mechanism being claimed, not a
hand-built proxy for it.

## Why

Testing a component's internal logic through a simulated input is not the same as testing that
it fires correctly through the real integration path - the real invocation, the real
environment, the real caller. A proxy test can pass while the real path is broken, and reporting
it as verified then hides exactly the failure the verification was supposed to catch.

## How to apply

Before claiming something is verified, name the exact mechanism that triggers it in production,
and confirm the test used that mechanism. If the real mechanism can't be exercised - no access,
too costly, needs another person - say so explicitly and state a lower confidence instead of
implying full verification by omission. When a bug report contradicts a "verified" claim, the
verification method was wrong; fix the test method, not just the bug.
