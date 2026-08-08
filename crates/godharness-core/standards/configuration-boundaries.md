---
id: configuration-boundaries
title: Centralize Configuration at a Validated Boundary
keywords: [environment variable, config, configuration, env var, feature flag]
paths: []
must-read: false
supersedes: []
relates-to: [secrets-and-security, runtime-validation]
---

## Rule

Keep environment variables and external configuration minimal, read them in one place, validate
them there, and pass typed, validated values to the rest of the codebase - don't read raw
environment variables scattered through business logic.

## Why

Configuration read ad hoc throughout a codebase is impossible to audit: nobody can answer "what
does this service actually depend on" without grepping the whole tree. A single validated
boundary makes every dependency visible and every invalid value fail at startup, not at runtime
deep in unrelated code.

## How to apply

Add new configuration to the existing central boundary, not inline where it's used. If no such
boundary exists yet, that's the first thing to add before a second configuration value shows up.
