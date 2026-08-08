---
id: structured-logging-over-printf
title: Structured Logging Over printf
keywords: [logging, log message, structured logging, printf, log line, free-text log]
paths: []
must-read: false
supersedes: []
relates-to: [error-handling]
---

## Rule

Emit logs as structured key-value events, not free-text strings interpolated together. A log
line should be queryable and joinable with traces later, not just readable in the moment it was
written.

## Why

A free-text log line like `"failed for user " + id + " after " + attempts + " tries"` is
readable once, in isolation, but useless for asking "show me every failure for this user" or
"what's the distribution of attempt counts across failures" without fragile string parsing.
Structured fields (`user_id`, `attempts`, `outcome`) turn the same event into something a log
aggregator can filter, group, and correlate.

## How to apply

When logging an event, pass the variable parts as named fields through the logging library's
structured API, not string-formatted into the message. Keep the message itself a short,
constant description of what happened; let the fields carry the specifics.
