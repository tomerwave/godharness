---
id: simplify-before-done
title: Simplify Before Calling It Done
keywords: [can this be simpler, simplify, reduce complexity, is this done]
paths: []
must-read: true
supersedes: []
relates-to: [small-focused-units, design-for-extension]
---

## Rule

Before marking work complete, do one deliberate pass asking: can this be simpler? Is there a
fundamentally different, less complex way to get the same result - not just line-level cleanup,
but is the whole approach more complicated than it needs to be?

## Why

First-draft solutions carry accidental complexity from how they were arrived at - false starts,
working around an earlier wrong assumption - not from what the problem actually needs. Without a
deliberate pass, that complexity calcifies into "how things are done" and gets built on top of
instead of removed.

## How to apply

Before calling something done, reread it fresh: what could be removed, merged, or replaced
without losing behavior? Ask whether a different approach to the whole problem would be less
complex, not just whether the current one can be tidied. Optimize for whoever reads this next
understanding and extending it easily, not for however it was easiest to write first.
