---
id: pin-and-verify-dependencies
title: Pin and Verify Dependencies
keywords: [dependency pin, lockfile, unpinned dependency, supply chain, provenance, floating version]
paths: []
must-read: false
supersedes: []
relates-to: [prefer-existing-solutions, secrets-and-security]
---

## Rule

Lock dependencies to specific, hash-verified versions in a committed lockfile. Prefer packages
with verifiable build provenance over unpinned or unsigned artifacts.

## Why

A floating version range means the exact code that runs can change without a corresponding
commit in this repository - a compromised or accidentally-broken upstream release reaches every
build automatically. A committed, hash-verified lockfile makes "what code actually runs" a
reviewable fact instead of whatever happened to resolve at build time.

## How to apply

Commit the lockfile your package manager produces, and don't hand-edit a version range to
"just work" without regenerating it properly. When choosing between otherwise-similar packages,
prefer the one with verifiable provenance (signed releases, reproducible builds, a documented
supply chain) over one that offers neither.
