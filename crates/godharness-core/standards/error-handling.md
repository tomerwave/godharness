---
id: error-handling
title: Fail Fast, Handle Only Recoverable Errors
keywords: [error handling, try catch, exception, error, panic, unwrap]
paths: []
must-read: false
supersedes: []
relates-to: [runtime-validation]
---

## Rule

Let unrecoverable errors propagate and fail loudly. Only catch or handle an error at the place
that can actually do something about it - retry, fall back, or present a clear message - not
everywhere the error could theoretically occur.

## Why

A catch block with no recovery action just hides the failure and turns a clear crash into a
silent, harder-to-diagnose bug later. Centralizing genuine recovery logic keeps error handling
meaningful instead of decorative.

## How to apply

Before adding a try/catch or equivalent, name the specific recovery action it enables. If there
isn't one, remove the handler and let the error propagate to a boundary that can log or report
it clearly.
