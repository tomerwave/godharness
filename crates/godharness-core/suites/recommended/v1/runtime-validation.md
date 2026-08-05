---
id: runtime-validation
title: Validate Untrusted Data at Runtime Boundaries
keywords: [validation, validate, untrusted input, schema, parse, deserialize]
paths: []
must-read: false
supersedes: []
relates-to: [error-handling, configuration-boundaries]
---

## Rule

Validate data at the boundary where it enters the system - user input, external API responses,
configuration, deserialized payloads - and treat it as trusted everywhere after that point.

## Why

A type system checks shape at compile time, not whether real data at runtime actually matches
that shape. Skipping boundary validation pushes the failure deep into business logic, where it's
harder to diagnose and further from the actual untrusted source.

## How to apply

At every point external data enters (HTTP handlers, config loaders, deserializers), validate it
against an explicit schema and reject invalid input immediately, with a clear error identifying
what failed.
