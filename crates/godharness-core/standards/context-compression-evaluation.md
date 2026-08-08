---
id: context-compression-evaluation
title: Evaluate Context Compression for High-Volume Tool Output
keywords: [context window, token budget, context compression, large tool output]
paths: []
must-read: false
supersedes: []
relates-to: [prefer-existing-solutions]
---

## Rule

When a repository's agent tooling routinely floods context with large tool outputs, logs, or
file dumps, evaluate a compression layer that sits between those outputs and the model - rather
than accepting the token cost, or hand-rolling truncation, as fixed facts of the setup.

## Why

Large, low-density tool output (verbose logs, full file reads, repetitive JSON) crowds out the
context an agent needs for the actual task, and naive truncation risks silently dropping the
one line that mattered. A dedicated compression layer can cut this substantially - compressing
verbose JSON aggressively and general tool output more modestly - while preserving the
information needed to answer correctly, which is a different and better trade than truncating
blind.

## How to apply

If tool-output volume is a recurring problem, evaluate an existing compression layer (for
example, a proxy or library that intercepts tool output before it reaches the model) rather than
building bespoke truncation. Before keeping it, verify empirically on real tasks from this repo
that answers stay correct after compression - a compression layer that changes an answer is a
regression, not a saving.
