---
id: small-focused-units
title: Small, Focused Units
keywords: [large file, big function, god object, single responsibility, split file]
paths: []
must-read: false
supersedes: []
relates-to: [naming]
---

## Rule

Keep functions and files small and single-purpose. When a unit does more than one clearly
nameable thing, split it along that seam.

## Why

Small units are easier to hold in context, test in isolation, and change without breaking
something unrelated. A file or function that keeps growing is usually a sign that two
responsibilities are tangled together, not that the task is inherently large.

## How to apply

Before adding to a large function or file, ask whether the new code is really the same
responsibility. If not, extract a new unit with its own name rather than appending to the
existing one.
