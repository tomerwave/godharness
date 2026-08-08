---
id: test-data-builders
title: Test Data Builders Over Shared Fixtures
keywords: [test fixture, test data builder, shared fixture, test setup, fixture object]
paths: []
must-read: false
supersedes: []
relates-to: [test-independence, testing]
---

## Rule

Construct test objects with a builder or factory that fills in only the fields a test actually
cares about and defaults the rest, rather than reusing one shared fixture object across many
tests.

## Why

A shared fixture accretes fields over time until no single test's intent is visible in it -
readers can't tell which of the fixture's twelve fields the test in front of them actually
depends on. A builder makes that explicit: the fields a test sets are the fields that matter to
it, and changing the fixture for one test's needs can't silently break an unrelated test that
happened to share it.

## How to apply

When a test needs a data object, build it through a function that takes only the
test-relevant overrides and fills in reasonable defaults for everything else, instead of
importing a shared instance. If several tests need the same override, that's a signal to add a
named builder variant, not to widen a shared fixture everyone imports.
