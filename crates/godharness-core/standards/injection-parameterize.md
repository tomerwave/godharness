---
id: injection-parameterize
title: Parameterize, Don't Concatenate
keywords: [sql injection, command injection, parameterize, concatenate query, template injection, shell injection]
paths: []
must-read: false
supersedes: []
relates-to: [runtime-validation, secrets-and-security]
---

## Rule

Build SQL, shell commands, and template/expression strings through parameterized APIs or safe
builders. Never concatenate or interpolate untrusted input directly into a query, command, or
template string.

## Why

A parameterized API keeps data and code separate at the point where the underlying engine
parses them - an attacker's input can never be reinterpreted as SQL syntax, a shell operator, or
a template directive, because it's never in a position to be. String concatenation collapses
that separation; a validation check upstream doesn't restore it, because the danger is the
mechanism, not the specific value.

## How to apply

When building a query, shell command, or template output from a variable, use the
parameterized/prepared form your library provides instead of string formatting. If no
parameterized API exists for a given sink, use an allowlist of safe values or a purpose-built
escaping function for that exact context - not a generic sanitizer applied everywhere.
