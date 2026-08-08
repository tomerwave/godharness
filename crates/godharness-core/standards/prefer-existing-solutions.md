---
id: prefer-existing-solutions
title: Prefer Existing Solutions
keywords: [existing solution, reinvent the wheel, hand rolled, dependency, library]
paths: []
must-read: false
supersedes: []
relates-to: [architecture-decisions]
---

## Rule

Before writing custom code for a problem that's likely already solved (parsing, validation,
retries, date handling, and the like), look for a maintained existing library first. Only build
custom when no adequate solution exists, the requirement is genuinely unique, or the dependency
cost outweighs the build cost.

## Why

Most problems aren't novel, and reinventing them wastes time and re-introduces bugs already
fixed elsewhere. But this isn't a blanket rule - every dependency adds maintenance burden,
security surface, and weight, so it's a real tradeoff, not an automatic "always use a library."

## How to apply

Before implementing, search for a maintained library solving this exact problem, and weigh its
maintenance activity, adoption, and security surface against the cost of building and
maintaining the equivalent yourself. Trivial scope - a five-line utility - doesn't need a
dependency. When picking between candidates, prefer actively maintained, widely adopted ones
over abandoned or obscure ones, even if the obscure one fits slightly better on paper.
