---
id: secrets-and-security
title: Never Commit Secrets
keywords: [secret, secrets, api key, credential, credentials, token, password]
paths: []
must-read: true
supersedes: []
relates-to: [configuration-boundaries]
---

## Rule

Never commit secrets, credentials, or tokens to the repository, including in tests, fixtures,
or example configuration. Load them from a secrets manager or environment at runtime instead.

## Why

A secret committed to Git history is compromised the moment it's pushed, even if the file is
later deleted - history retains it, and forks or clones can carry it further. Detection after
the fact is a mitigation, not a fix.

## How to apply

Use placeholder values in examples and fixtures. Load real secrets through environment
variables or a secrets manager, validated at the configuration boundary, never hardcoded.
