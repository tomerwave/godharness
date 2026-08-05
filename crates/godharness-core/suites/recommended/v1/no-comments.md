---
id: no-comments
title: No Code Comments
keywords: [comment, comments, self-documenting, redundant comment]
paths: []
must-read: true
supersedes: []
relates-to: [naming]
---

## Rule

Don't write comments explaining what code does or why. If code needs a comment to be
understood, rename or restructure it instead.

## Why

A comment explaining "why" is usually a sign that a variable, function, or module could carry
that meaning in its name instead. Comments rot: they stop being updated when the code changes
and then actively mislead. Evidence and history belong in commit messages and PR descriptions,
not in the file.

## How to apply

Before adding a comment, ask whether a better name would carry the same meaning. If a fallback
or workaround isn't backed by evidence, remove it or verify it and record the reasoning in a
decision record instead of a caveat comment.
