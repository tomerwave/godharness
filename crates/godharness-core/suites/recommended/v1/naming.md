---
id: naming
title: Names Convey Responsibility
keywords: [naming, name, rename, variable name, function name]
paths: []
must-read: false
supersedes: []
relates-to: [no-comments, small-focused-units]
---

## Rule

A name must convey what it does, what it takes, and what it returns or produces. If a name
needs a comment or a teammate's explanation to be understood, it is the wrong name.

## Why

Precise names are the cheapest, most durable form of documentation: they never go stale because
they are the code. Vague names ("data", "handle", "process") push the burden of understanding
onto every future reader instead of the one person who wrote it.

## How to apply

Read a name in isolation, with no surrounding context. If you can't tell what it holds or does,
rename it. Prefer a longer, precise name over a short, ambiguous one.
